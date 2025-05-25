//! # Threat Detection Module
//! 
//! This module provides threat detection capabilities for the AGI operating system,
//! enabling identification of security threats, anomalies, and potential attacks.

use std::sync::{Arc, Mutex, Once, LazyLock};
use std::collections::{HashMap, VecDeque};
use std::time::{Duration, Instant};
use std::fmt;
use std::error::Error;

use lazy_static::lazy_static;

static INIT: Once = Once::new();

// 全局威胁检测注册表
lazy_static! {
    static ref THREAT_DETECTION_REGISTRY: Mutex<ThreatDetectionRegistry> = 
        Mutex::new(ThreatDetectionRegistry::new());
}

/// 威胁检测注册表，管理所有检测引擎
#[derive(Debug)]
pub struct ThreatDetectionRegistry {
    /// 注册的检测引擎
    engines: HashMap<String, Arc<Mutex<DetectionEngine>>>,
    /// 全局配置
    config: ThreatDetectionConfig,
    /// 全局状态
    status: ThreatDetectionStatus,
}

impl ThreatDetectionRegistry {
    /// 创建新的威胁检测注册表
    pub fn new() -> Self {
        Self {
            engines: HashMap::new(),
            config: ThreatDetectionConfig::default(),
            status: ThreatDetectionStatus {
                engine_count: 0,
                rule_count: 0,
                threat_count: 0,
                alert_count: 0,
                last_detection_time: None,
            },
        }
    }
    
    /// 注册检测引擎
    pub fn register_engine(&mut self, engine: DetectionEngine) -> Result<(), ThreatDetectionError> {
        let id = engine.id().to_string();
        
        if self.engines.contains_key(&id) {
            return Err(ThreatDetectionError::General("Engine already registered"));
        }
        
        self.engines.insert(id, Arc::new(Mutex::new(engine)));
        self.update_status();
        Ok(())
    }
    
    /// 注销检测引擎
    pub fn unregister_engine(&mut self, engine_id: &str) -> Result<(), ThreatDetectionError> {
        if !self.engines.contains_key(engine_id) {
            return Err(ThreatDetectionError::General("Engine not found"));
        }
        
        self.engines.remove(engine_id);
        self.update_status();
        Ok(())
    }
    
    /// 获取检测引擎
    pub fn get_engine(&self, engine_id: &str) -> Result<Arc<Mutex<DetectionEngine>>, ThreatDetectionError> {
        self.engines.get(engine_id)
            .cloned()
            .ok_or_else(|| ThreatDetectionError::General("Engine not found"))
    }
    
    /// 处理系统事件
    pub fn process_event(&mut self, event: SystemEvent) -> Result<Vec<ThreatAlert>, ThreatDetectionError> {
        if !self.config.enable_threat_detection {
            return Ok(Vec::new());
        }
        
        let mut all_alerts = Vec::new();
        
        for engine in self.engines.values() {
            let mut engine = engine.lock().map_err(|_| 
                ThreatDetectionError::General("Failed to lock engine"))?;
            
            let alerts = engine.process_event(event.clone())?;
            all_alerts.extend(alerts);
        }
        
        self.update_status();
        Ok(all_alerts)
    }
    
    /// 设置全局配置
    pub fn set_config(&mut self, config: ThreatDetectionConfig) {
        self.config = config.clone();
        
        // 更新所有引擎的配置
        for engine in self.engines.values() {
            if let Ok(mut engine) = engine.lock() {
                engine.set_config(config.clone());
            }
        }
    }
    
    /// 获取全局配置
    pub fn get_config(&self) -> ThreatDetectionConfig {
        self.config.clone()
    }
    
    /// 获取全局状态
    pub fn get_status(&self) -> ThreatDetectionStatus {
        self.status.clone()
    }
    
    /// 更新全局状态
    fn update_status(&mut self) {
        let mut rule_count = 0;
        let mut threat_count = 0;
        let mut alert_count = 0;
        let mut last_detection_time = None;
        
        for engine in self.engines.values() {
            if let Ok(engine) = engine.lock() {
                let status = engine.get_status();
                rule_count += status.rule_count;
                threat_count += status.threat_count;
                alert_count += status.alert_count;
                
                if let Some(time) = status.last_detection_time {
                    if last_detection_time.is_none() || 
                       last_detection_time.unwrap() < time {
                        last_detection_time = Some(time);
                    }
                }
            }
        }
        
        self.status = ThreatDetectionStatus {
            engine_count: self.engines.len(),
            rule_count,
            threat_count,
            alert_count,
            last_detection_time,
        };
    }
}

/// Initialize the threat detection subsystem
pub fn init() -> Result<(), ThreatDetectionError> {
    let mut result = Ok(());
    
    INIT.call_once(|| {
        // Initialize the threat detection registry
        let _unused = THREAT_DETECTION_REGISTRY.lock().unwrap();
    });
    
    result
}

/// Error type for threat detection operations
#[derive(Debug)]
pub enum ThreatDetectionError {
    /// Detection engine error
    DetectionEngineError(String),
    /// Rule error
    RuleError(String),
    /// Analysis error
    AnalysisError(String),
    /// General error
    General(&'static str),
}

// 实现Display trait，解决E0277错误
impl fmt::Display for ThreatDetectionError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            ThreatDetectionError::DetectionEngineError(msg) => write!(f, "Detection engine error: {}", msg),
            ThreatDetectionError::RuleError(msg) => write!(f, "Rule error: {}", msg),
            ThreatDetectionError::AnalysisError(msg) => write!(f, "Analysis error: {}", msg),
            ThreatDetectionError::General(msg) => write!(f, "General error: {}", msg),
        }
    }
}

// 实现Error trait，解决?操作符错误转换问题
impl Error for ThreatDetectionError {}

/// Threat detection configuration
#[derive(Debug, Clone)]
pub struct ThreatDetectionConfig {
    /// Enable threat detection
    pub enable_threat_detection: bool,
    /// Enable anomaly detection
    pub enable_anomaly_detection: bool,
    /// Enable behavioral analysis
    pub enable_behavioral_analysis: bool,
    /// Enable signature-based detection
    pub enable_signature_detection: bool,
    /// Detection interval (in seconds)
    pub detection_interval: u64,
    /// Event history size
    pub event_history_size: usize,
    /// Alert threshold
    pub alert_threshold: f32,
}

impl Default for ThreatDetectionConfig {
    fn default() -> Self {
        Self {
            enable_threat_detection: true,
            enable_anomaly_detection: true,
            enable_behavioral_analysis: true,
            enable_signature_detection: true,
            detection_interval: 60,
            event_history_size: 1000,
            alert_threshold: 0.7,
        }
    }
}

/// Threat detection status
#[derive(Debug, Clone)]
pub struct ThreatDetectionStatus {
    /// Number of registered detection engines
    pub engine_count: usize,
    /// Number of registered rules
    pub rule_count: usize,
    /// Number of detected threats
    pub threat_count: usize,
    /// Number of alerts
    pub alert_count: usize,
    /// Last detection time
    pub last_detection_time: Option<Instant>,
}

/// Threat severity
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord)]
pub enum ThreatSeverity {
    /// Low severity
    Low,
    /// Medium severity
    Medium,
    /// High severity
    High,
    /// Critical severity
    Critical,
}

/// Threat type
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum ThreatType {
    /// Unauthorized access
    UnauthorizedAccess,
    /// Privilege escalation
    PrivilegeEscalation,
    /// Data exfiltration
    DataExfiltration,
    /// Code injection
    CodeInjection,
    /// Denial of service
    DenialOfService,
    /// Anomalous behavior
    AnomalousBehavior,
    /// Resource abuse
    ResourceAbuse,
}

/// System event
#[derive(Debug, Clone)]
pub struct SystemEvent {
    /// Event ID
    pub id: String,
    /// Event type
    pub event_type: String,
    /// Event source
    pub source: String,
    /// Event timestamp
    pub timestamp: Instant,
    /// Event data
    pub data: HashMap<String, String>,
}

impl SystemEvent {
    /// Create a new system event
    pub fn new(id: &str, event_type: &str, source: &str) -> Self {
        Self {
            id: id.to_string(),
            event_type: event_type.to_string(),
            source: source.to_string(),
            timestamp: Instant::now(),
            data: HashMap::new(),
        }
    }
    
    /// Add event data
    pub fn add_data(&mut self, key: &str, value: &str) {
        self.data.insert(key.to_string(), value.to_string());
    }
}

/// Threat alert
#[derive(Debug, Clone)]
pub struct ThreatAlert {
    /// Alert ID
    pub id: String,
    /// Threat type
    pub threat_type: ThreatType,
    /// Threat severity
    pub severity: ThreatSeverity,
    /// Alert timestamp
    pub timestamp: Instant,
    /// Alert description
    pub description: String,
    /// Related events
    pub related_events: Vec<String>,
    /// Recommended actions
    pub recommended_actions: Vec<String>,
}

impl ThreatAlert {
    /// Create a new threat alert
    pub fn new(id: &str, threat_type: ThreatType, severity: ThreatSeverity, description: &str) -> Self {
        Self {
            id: id.to_string(),
            threat_type,
            severity,
            timestamp: Instant::now(),
            description: description.to_string(),
            related_events: Vec::new(),
            recommended_actions: Vec::new(),
        }
    }
    
    /// Add related event
    pub fn add_related_event(&mut self, event_id: &str) {
        self.related_events.push(event_id.to_string());
    }
    
    /// Add recommended action
    pub fn add_recommended_action(&mut self, action: &str) {
        self.recommended_actions.push(action.to_string());
    }
}

// 为闭包类型实现Debug trait的包装器
#[derive(Clone)]
struct DebugFnWrapper<F> {
    inner: F,
}

impl<F> fmt::Debug for DebugFnWrapper<F> {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "Function<{}>", std::any::type_name::<F>())
    }
}

/// Detection rule
#[derive(Debug)]
pub struct DetectionRule {
    /// Rule ID
    id: String,
    /// Rule name
    name: String,
    /// Rule description
    description: String,
    /// Threat type
    threat_type: ThreatType,
    /// Threat severity
    severity: ThreatSeverity,
    /// Rule condition (as a closure)
    #[allow(clippy::type_complexity)]
    condition: DebugFnWrapper<Box<dyn Fn(&SystemEvent, &[SystemEvent]) -> bool + Send + Sync>>,
    /// Rule enabled
    enabled: bool,
}

impl DetectionRule {
    /// Create a new detection rule
    pub fn new<F>(id: &str, name: &str, description: &str, threat_type: ThreatType, severity: ThreatSeverity, condition: F) -> Self
    where
        F: Fn(&SystemEvent, &[SystemEvent]) -> bool + Send + Sync + 'static,
    {
        Self {
            id: id.to_string(),
            name: name.to_string(),
            description: description.to_string(),
            threat_type,
            severity,
            condition: DebugFnWrapper { inner: Box::new(condition) },
            enabled: true,
        }
    }
    
    /// Get rule ID
    pub fn id(&self) -> &str {
        &self.id
    }
    
    /// Get rule name
    pub fn name(&self) -> &str {
        &self.name
    }
    
    /// Get rule description
    pub fn description(&self) -> &str {
        &self.description
    }
    
    /// Get threat type
    pub fn threat_type(&self) -> ThreatType {
        self.threat_type
    }
    
    /// Get threat severity
    pub fn severity(&self) -> ThreatSeverity {
        self.severity
    }
    
    /// Check if rule is enabled
    pub fn is_enabled(&self) -> bool {
        self.enabled
    }
    
    /// Enable rule
    pub fn enable(&mut self) {
        self.enabled = true;
    }
    
    /// Disable rule
    pub fn disable(&mut self) {
        self.enabled = false;
    }
    
    /// Evaluate rule against an event and event history
    pub fn evaluate(&self, event: &SystemEvent, history: &[SystemEvent]) -> bool {
        if !self.enabled {
            return false;
        }
        
        (self.condition.inner)(event, history)
    }
}

/// Detection engine
#[derive(Debug)]
pub struct DetectionEngine {
    /// Engine ID
    id: String,
    /// Engine name
    name: String,
    /// Engine description
    description: String,
    /// Detection rules
    rules: HashMap<String, Arc<Mutex<DetectionRule>>>,
    /// Event history
    event_history: VecDeque<SystemEvent>,
    /// Alert history
    alert_history: Vec<ThreatAlert>,
    /// Configuration
    config: ThreatDetectionConfig,
    /// Last detection time
    last_detection_time: Option<Instant>,
}

impl DetectionEngine {
    /// Create a new detection engine
    pub fn new(id: &str, name: &str, description: &str, config: ThreatDetectionConfig) -> Self {
        Self {
            id: id.to_string(),
            name: name.to_string(),
            description: description.to_string(),
            rules: HashMap::new(),
            event_history: VecDeque::with_capacity(config.event_history_size),
            alert_history: Vec::new(),
            config,
            last_detection_time: None,
        }
    }
    
    /// Get engine ID
    pub fn id(&self) -> &str {
        &self.id
    }
    
    /// Get engine name
    pub fn name(&self) -> &str {
        &self.name
    }
    
    /// Get engine description
    pub fn description(&self) -> &str {
        &self.description
    }
    
    /// Add a detection rule
    pub fn add_rule(&mut self, rule: DetectionRule) -> Result<(), ThreatDetectionError> {
        let id = rule.id().to_string();
        
        if self.rules.contains_key(&id) {
            return Err(ThreatDetectionError::General("Rule already exists"));
        }
        
        self.rules.insert(id, Arc::new(Mutex::new(rule)));
        Ok(())
    }
    
    /// Remove a detection rule
    pub fn remove_rule(&mut self, rule_id: &str) -> Result<(), ThreatDetectionError> {
        if !self.rules.contains_key(rule_id) {
            return Err(ThreatDetectionError::General("Rule not found"));
        }
        
        self.rules.remove(rule_id);
        Ok(())
    }
    
    /// Get a detection rule
    pub fn get_rule(&self, rule_id: &str) -> Result<Arc<Mutex<DetectionRule>>, ThreatDetectionError> {
        self.rules.get(rule_id)
            .cloned()
            .ok_or_else(|| ThreatDetectionError::General("Rule not found"))
    }
    
    /// Process a system event
    pub fn process_event(&mut self, event: SystemEvent) -> Result<Vec<ThreatAlert>, ThreatDetectionError> {
        if !self.config.enable_threat_detection {
            return Ok(Vec::new());
        }
        
        // Add event to history
        self.event_history.push_back(event.clone());
        
        // Trim history if needed
        while self.event_history.len() > self.config.event_history_size {
            self.event_history.pop_front();
        }
        
        // Check if it's time to run detection
        let should_detect = if let Some(last_time) = self.last_detection_time {
            last_time.elapsed() >= Duration::from_secs(self.config.detection_interval)
        } else {
            true
        };
        
        if !should_detect {
            return Ok(Vec::new());
        }
        
        // Run detection
        let alerts = self.detect_threats()?;
        
        // Update last detection time
        self.last_detection_time = Some(Instant::now());
        
        Ok(alerts)
    }
    
    /// Detect threats based on current event history
    pub fn detect_threats(&mut self) -> Result<Vec<ThreatAlert>, ThreatDetectionError> {
        if !self.config.enable_threat_detection {
            return Ok(Vec::new());
        }
        
        let mut alerts = Vec::new();
        
        // Convert event history to a slice f
(Content truncated due to size limit. Use line ranges to read in chunks)