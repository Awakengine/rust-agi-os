//! # System Monitoring Module
//! 
//! This module provides system monitoring capabilities for the AGI operating system,
//! enabling performance tracking, resource usage monitoring, and anomaly detection.

use std::sync::{Arc, Mutex};
use std::collections::{HashMap, VecDeque};
use std::time::Instant;
use std::fmt;

/// Initialize the monitoring subsystem
pub fn init() -> Result<(), MonitoringError> {
    // Initialize monitoring components
    Ok(())
}

/// Error type for monitoring operations
#[derive(Debug)]
pub enum MonitoringError {
    /// Metric error
    MetricError(String),
    /// Collector error
    CollectorError(String),
    /// Alert error
    AlertError(String),
    /// General error
    General(&'static str),
}

impl std::fmt::Display for MonitoringError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            MonitoringError::MetricError(msg) => write!(f, "Metric error: {}", msg),
            MonitoringError::CollectorError(msg) => write!(f, "Collector error: {}", msg),
            MonitoringError::AlertError(msg) => write!(f, "Alert error: {}", msg),
            MonitoringError::General(msg) => write!(f, "General monitoring error: {}", msg),
        }
    }
}

impl std::error::Error for MonitoringError {}

/// Monitoring configuration
#[derive(Debug, Clone)]
pub struct MonitoringConfig {
    /// Enable system monitoring
    pub enable_monitoring: bool,
    /// Sampling interval (in milliseconds)
    pub sampling_interval: u64,
    /// History size
    pub history_size: usize,
    /// Enable anomaly detection
    pub enable_anomaly_detection: bool,
    /// Enable alerts
    pub enable_alerts: bool,
    /// Alert threshold
    pub alert_threshold: f64,
}

impl Default for MonitoringConfig {
    fn default() -> Self {
        Self {
            enable_monitoring: true,
            sampling_interval: 1000,
            history_size: 1000,
            enable_anomaly_detection: true,
            enable_alerts: true,
            alert_threshold: 0.9,
        }
    }
}

/// Monitoring status
#[derive(Debug, Clone)]
pub struct MonitoringStatus {
    /// Is monitoring active
    pub is_active: bool,
    /// Number of registered metrics
    pub metric_count: usize,
    /// Number of registered collectors
    pub collector_count: usize,
    /// Number of registered alerts
    pub alert_count: usize,
    /// Number of active alerts
    pub active_alert_count: usize,
    /// Last sampling time
    pub last_sampling_time: Option<Instant>,
}

/// Metric type
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum MetricType {
    /// Counter (monotonically increasing)
    Counter,
    /// Gauge (can go up and down)
    Gauge,
    /// Histogram (distribution of values)
    Histogram,
}

/// Metric value
#[derive(Debug, Clone)]
pub enum MetricValue {
    /// Integer value
    Integer(i64),
    /// Float value
    Float(f64),
    /// Boolean value
    Boolean(bool),
    /// String value
    String(String),
}

impl MetricValue {
    /// Convert to float
    pub fn as_float(&self) -> Option<f64> {
        match self {
            MetricValue::Integer(i) => Some(*i as f64),
            MetricValue::Float(f) => Some(*f),
            MetricValue::Boolean(b) => Some(if *b { 1.0 } else { 0.0 }),
            MetricValue::String(_) => None,
        }
    }
    
    /// Convert to integer
    pub fn as_integer(&self) -> Option<i64> {
        match self {
            MetricValue::Integer(i) => Some(*i),
            MetricValue::Float(f) => Some(*f as i64),
            MetricValue::Boolean(b) => Some(if *b { 1 } else { 0 }),
            MetricValue::String(_) => None,
        }
    }
    
    /// Convert to boolean
    pub fn as_boolean(&self) -> Option<bool> {
        match self {
            MetricValue::Integer(i) => Some(*i != 0),
            MetricValue::Float(f) => Some(*f != 0.0),
            MetricValue::Boolean(b) => Some(*b),
            MetricValue::String(s) => {
                match s.to_lowercase().as_str() {
                    "true" | "yes" | "1" => Some(true),
                    "false" | "no" | "0" => Some(false),
                    _ => None,
                }
            }
        }
    }
    
    /// Convert to string
    pub fn as_string(&self) -> String {
        match self {
            MetricValue::Integer(i) => i.to_string(),
            MetricValue::Float(f) => f.to_string(),
            MetricValue::Boolean(b) => b.to_string(),
            MetricValue::String(s) => s.clone(),
        }
    }
}

/// Metric
#[derive(Debug)]
pub struct Metric {
    /// Metric ID
    id: String,
    /// Metric name
    name: String,
    /// Metric description
    description: String,
    /// Metric type
    metric_type: MetricType,
    /// Metric unit
    unit: Option<String>,
    /// Current value
    current_value: MetricValue,
    /// History
    history: VecDeque<(Instant, MetricValue)>,
    /// History size
    history_size: usize,
    /// Tags
    tags: HashMap<String, String>,
}

impl Metric {
    /// Create a new metric
    pub fn new(
        id: &str,
        name: &str,
        description: &str,
        metric_type: MetricType,
        unit: Option<&str>,
        initial_value: MetricValue,
        history_size: usize,
    ) -> Self {
        let mut history = VecDeque::with_capacity(history_size);
        history.push_back((Instant::now(), initial_value.clone()));
        
        Self {
            id: id.to_string(),
            name: name.to_string(),
            description: description.to_string(),
            metric_type,
            unit: unit.map(|s| s.to_string()),
            current_value: initial_value,
            history,
            history_size,
            tags: HashMap::new(),
        }
    }
    
    /// Get metric ID
    pub fn id(&self) -> &str {
        &self.id
    }
    
    /// Get metric name
    pub fn name(&self) -> &str {
        &self.name
    }
    
    /// Get metric description
    pub fn description(&self) -> &str {
        &self.description
    }
    
    /// Get metric type
    pub fn metric_type(&self) -> MetricType {
        self.metric_type
    }
    
    /// Get metric unit
    pub fn unit(&self) -> Option<&str> {
        self.unit.as_deref()
    }
    
    /// Get current value
    pub fn current_value(&self) -> &MetricValue {
        &self.current_value
    }
    
    /// Get history
    pub fn history(&self) -> &VecDeque<(Instant, MetricValue)> {
        &self.history
    }
    
    /// Get tags
    pub fn tags(&self) -> &HashMap<String, String> {
        &self.tags
    }
    
    /// Add tag
    pub fn add_tag(&mut self, key: &str, value: &str) {
        self.tags.insert(key.to_string(), value.to_string());
    }
    
    /// Remove tag
    pub fn remove_tag(&mut self, key: &str) {
        self.tags.remove(key);
    }
    
    /// Update value
    pub fn update_value(&mut self, value: MetricValue) {
        self.current_value = value.clone();
        
        // Add to history
        self.history.push_back((Instant::now(), value));
        
        // Trim history if needed
        while self.history.len() > self.history_size {
            self.history.pop_front();
        }
    }
    
    /// Increment counter
    pub fn increment(&mut self, amount: i64) -> Result<(), String> {
        if self.metric_type != MetricType::Counter {
            return Err(format!("Metric {} is not a counter", self.id));
        }
        
        match &self.current_value {
            MetricValue::Integer(i) => {
                self.update_value(MetricValue::Integer(i + amount));
                Ok(())
            }
            MetricValue::Float(f) => {
                self.update_value(MetricValue::Float(f + amount as f64));
                Ok(())
            }
            _ => Err(format!("Cannot increment non-numeric metric {}", self.id)),
        }
    }
    
    /// Set gauge value
    pub fn set_gauge(&mut self, value: MetricValue) -> Result<(), String> {
        if self.metric_type != MetricType::Gauge {
            return Err(format!("Metric {} is not a gauge", self.id));
        }
        
        self.update_value(value);
        Ok(())
    }
    
    /// Add histogram value
    pub fn add_histogram_value(&mut self, value: f64) -> Result<(), String> {
        if self.metric_type != MetricType::Histogram {
            return Err(format!("Metric {} is not a histogram", self.id));
        }
        
        // In a real implementation, this would update histogram statistics
        // For this prototype, we just store the latest value
        self.update_value(MetricValue::Float(value));
        Ok(())
    }
    
    /// Get average value over time window
    pub fn average(&self, window_seconds: u64) -> Option<f64> {
        let now = Instant::now();
        let window = std::time::Duration::from_secs(window_seconds);
        
        let values: Vec<f64> = self.history.iter()
            .filter(|(time, _)| now.duration_since(*time) <= window)
            .filter_map(|(_, value)| value.as_float())
            .collect();
        
        if values.is_empty() {
            None
        } else {
            Some(values.iter().sum::<f64>() / values.len() as f64)
        }
    }
    
    /// Get minimum value over time window
    pub fn minimum(&self, window_seconds: u64) -> Option<f64> {
        let now = Instant::now();
        let window = std::time::Duration::from_secs(window_seconds);
        
        self.history.iter()
            .filter(|(time, _)| now.duration_since(*time) <= window)
            .filter_map(|(_, value)| value.as_float())
            .min_by(|a, b| a.partial_cmp(b).unwrap_or(std::cmp::Ordering::Equal))
    }
    
    /// Get maximum value over time window
    pub fn maximum(&self, window_seconds: u64) -> Option<f64> {
        let now = Instant::now();
        let window = std::time::Duration::from_secs(window_seconds);
        
        self.history.iter()
            .filter(|(time, _)| now.duration_since(*time) <= window)
            .filter_map(|(_, value)| value.as_float())
            .max_by(|a, b| a.partial_cmp(b).unwrap_or(std::cmp::Ordering::Equal))
    }
    
    /// Get rate of change
    pub fn rate_of_change(&self, window_seconds: u64) -> Option<f64> {
        let now = Instant::now();
        let window = std::time::Duration::from_secs(window_seconds);
        
        let values: Vec<(Instant, f64)> = self.history.iter()
            .filter(|(time, _)| now.duration_since(*time) <= window)
            .filter_map(|(time, value)| value.as_float().map(|v| (*time, v)))
            .collect();
        
        if values.len() < 2 {
            None
        } else {
            let (oldest_time, oldest_value) = values.first().unwrap();
            let (newest_time, newest_value) = values.last().unwrap();
            
            let time_diff = newest_time.duration_since(*oldest_time).as_secs_f64();
            if time_diff == 0.0 {
                None
            } else {
                Some((newest_value - oldest_value) / time_diff)
            }
        }
    }
}

/// Collector trait
pub trait Collector: Send + Sync + fmt::Debug {
    /// Collect metrics
    fn collect(&self) -> Result<HashMap<String, MetricValue>, String>;
    
    /// Get collector name
    fn name(&self) -> &str;
    
    /// Get collector description
    fn description(&self) -> &str;
}

/// Debug wrapper for Collector trait objects
#[derive(Debug)]
pub struct CollectorWrapper {
    /// Collector name
    name: String,
    /// Collector description
    description: String,
    /// Collector implementation
    #[allow(dead_code)]
    collector: Arc<dyn Collector>,
}

impl CollectorWrapper {
    /// Create a new collector wrapper
    pub fn new(collector: Arc<dyn Collector>) -> Self {
        let name = collector.name().to_string();
        let description = collector.description().to_string();
        
        Self {
            name,
            description,
            collector,
        }
    }
    
    /// Get collector
    pub fn collector(&self) -> &Arc<dyn Collector> {
        &self.collector
    }
}

/// Alert severity
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord)]
pub enum AlertSeverity {
    /// Info
    Info,
    /// Warning
    Warning,
    /// Error
    Error,
    /// Critical
    Critical,
}

/// Alert state
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum AlertState {
    /// Inactive
    Inactive,
    /// Active
    Active,
    /// Acknowledged
    Acknowledged,
    /// Resolved
    Resolved,
}

/// Alert
#[derive(Debug)]
pub struct Alert {
    /// Alert ID
    id: String,
    /// Alert name
    name: String,
    /// Alert description
    description: String,
    /// Alert severity
    severity: AlertSeverity,
    /// Alert state
    state: AlertState,
    /// Metric ID
    metric_id: String,
    /// Threshold
    threshold: f64,
    /// Comparison operator
    operator: String,
    /// Activation time
    activation_time: Option<Instant>,
    /// Acknowledgement time
    acknowledgement_time: Option<Instant>,
    /// Resolution time
    resolution_time: Option<Instant>,
}

impl Alert {
    /// Create a new alert
    pub fn new(
        id: &str,
        name: &str,
        description: &str,
        severity: AlertSeverity,
        metric_id: &str,
        threshold: f64,
        operator: &str,
    ) -> Self {
        Self {
            id: id.to_string(),
            name: name.to_string(),
            description: description.to_string(),
            severity,
            state: AlertState::Inactive,
            metric_id: metric_id.to_string(),
            threshold,
            operator: operator.to_string(),
            activation_time: None,
            acknowledgement_time: None,
            resolution_time: None,
        }
    }
    
    /// Get alert ID
    pub fn id(&self) -> &str {
        &self.id
    }
    
    /// Get alert name
    pub fn name(&self) -> &str {
        &self.name
    }
    
    /// Get alert description
    pub fn description(&self) -> &str {
        &self.description
    }
    
    /// Get alert severity
    pub fn severity(&self) -> AlertSeverity {
        self.severity
    }
    
    /// Get alert state
    pub fn state(&self) -> AlertState {
        self.state
    }
    
    /// Get metric ID
    pub fn metric_id(&self) -> &str {
        &self.metric_id
    }
    
    /// Get threshold
    pub fn threshold(&self) -> f64 {
        self.threshold
    }
    
    /// Get operator
    pub fn operator(&self) -> &str {
        &self.operator
    }
    
    /// Get activation time
    pub fn activation_time(&self) -> Option<Instant> {
        self.activation_time
    }
    
    /// Get acknowledgement time
    pub fn acknowledgement_time(&self) -> Option<Instant> {
        self.acknowledgement_time
    }
    
    /// Get resolution time
    pub fn resolution_time(&self) -> Option<Instant> {
        self.resolution_time
    }
    
    /// Check if alert is triggered
    pub fn check(&mut self, value: f64) -> bool {
        let triggered = match self.operator.as_str() {
            ">" => value > self.threshold,
            ">=" => value >= self.threshold,
            "<" => value < self.threshold,
            "<=" => value <= self.threshold,
            "==" => (value - self.threshold).abs() < f64::EPSILON,
            "!=" => (value - self.threshold).abs() >= f64::EPSILON,
            _ => false,
        };
        
        if triggered && self.state == AlertState::Inactive {
            self.state = AlertState::Active;
            self.activation_time = Some(Instant::now());
            true
        } else if !triggered && (self.state == AlertState::Active || self.state == AlertState::Acknowledged) {
            self.state = AlertState::Resolved;
            self.resolution_time = Some(Instant::now());
            true
        } else {
            false
        }
    }
    
    /// Acknowledge alert
    pub fn acknowledge(&mut
(Content truncated due to size limit. Use line ranges to read in chunks)