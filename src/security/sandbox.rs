//! # Sandbox Module
//! 
//! This module provides secure sandboxing capabilities for the AGI operating system,
//! enabling safe execution of untrusted code and isolation of system components.

use std::sync::{Arc, Mutex, Once};
use std::collections::HashMap;
use std::path::PathBuf;
use std::fmt;

use crate::kernel::memory::{IsolatedMemoryRegion, ProtectionFlags};
use crate::kernel::process::{ProcessId, ProcessError};
use crate::security::access_control::{AccessControlList, Permission};

// 使用lazy_static替代LazyLock，因为LazyLock在Rust 1.87中可能不可用
use lazy_static::lazy_static;

static INIT: Once = Once::new();

// 全局沙箱注册表
lazy_static! {
    static ref SANDBOX_REGISTRY: Mutex<SandboxRegistry> = Mutex::new(SandboxRegistry::new());
}

/// Initialize the sandbox subsystem
pub fn init() -> Result<(), SandboxError> {
    let mut result = Ok(());
    
    INIT.call_once(|| {
        // Initialize the sandbox registry
        let _unused = SANDBOX_REGISTRY.lock().unwrap();
    });
    
    result
}

/// Error type for sandbox operations
#[derive(Debug)]
pub enum SandboxError {
    /// Memory error
    MemoryError(String),
    /// Process error
    ProcessError(ProcessError),
    /// Resource limit exceeded
    ResourceLimitExceeded(String),
    /// Permission denied
    PermissionDenied(String),
    /// Sandbox creation error
    CreationError(String),
    /// Execution error
    ExecutionError(String),
    /// General error
    General(&'static str),
}

impl std::error::Error for SandboxError {}

impl fmt::Display for SandboxError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            SandboxError::MemoryError(msg) => write!(f, "Memory error: {}", msg),
            SandboxError::ProcessError(err) => write!(f, "Process error: {}", err),
            SandboxError::ResourceLimitExceeded(msg) => write!(f, "Resource limit exceeded: {}", msg),
            SandboxError::PermissionDenied(msg) => write!(f, "Permission denied: {}", msg),
            SandboxError::CreationError(msg) => write!(f, "Sandbox creation error: {}", msg),
            SandboxError::ExecutionError(msg) => write!(f, "Execution error: {}", msg),
            SandboxError::General(msg) => write!(f, "General error: {}", msg),
        }
    }
}

/// Sandbox configuration
#[derive(Debug, Clone)]
pub struct SandboxConfig {
    /// Enable memory isolation
    pub enable_memory_isolation: bool,
    /// Enable process isolation
    pub enable_process_isolation: bool,
    /// Enable resource limits
    pub enable_resource_limits: bool,
    /// Enable capability-based security
    pub enable_capabilities: bool,
    /// Default memory limit (in bytes)
    pub default_memory_limit: usize,
    /// Default CPU limit (in percentage)
    pub default_cpu_limit: f32,
    /// Default network access
    pub default_network_access: bool,
    /// Default file system access
    pub default_filesystem_access: bool,
}

impl Default for SandboxConfig {
    fn default() -> Self {
        Self {
            enable_memory_isolation: true,
            enable_process_isolation: true,
            enable_resource_limits: true,
            enable_capabilities: true,
            default_memory_limit: 100 * 1024 * 1024, // 100 MB
            default_cpu_limit: 10.0, // 10%
            default_network_access: false,
            default_filesystem_access: false,
        }
    }
}

/// Sandbox status
#[derive(Debug, Clone)]
pub struct SandboxStatus {
    /// Number of active sandboxes
    pub active_sandbox_count: usize,
    /// Number of terminated sandboxes
    pub terminated_sandbox_count: usize,
    /// Total memory used by all sandboxes (in bytes)
    pub total_memory_usage: usize,
    /// Total CPU usage by all sandboxes (in percentage)
    pub total_cpu_usage: f32,
}

/// Resource limits
#[derive(Debug, Clone)]
pub struct ResourceLimits {
    /// Memory limit (in bytes)
    pub memory_limit: usize,
    /// CPU limit (in percentage)
    pub cpu_limit: f32,
    /// Network bandwidth limit (in bytes per second)
    pub network_bandwidth_limit: Option<usize>,
    /// File system space limit (in bytes)
    pub filesystem_space_limit: Option<usize>,
    /// Maximum execution time (in seconds)
    pub max_execution_time: Option<f64>,
}

impl Default for ResourceLimits {
    fn default() -> Self {
        Self {
            memory_limit: 100 * 1024 * 1024, // 100 MB
            cpu_limit: 10.0, // 10%
            network_bandwidth_limit: None,
            filesystem_space_limit: None,
            max_execution_time: None,
        }
    }
}

/// Sandbox capabilities
#[derive(Debug, Clone)]
pub struct Capabilities {
    /// Network access
    pub network_access: bool,
    /// File system access
    pub filesystem_access: bool,
    /// Process creation
    pub process_creation: bool,
    /// System calls
    pub system_calls: Vec<String>,
    /// Device access
    pub device_access: HashMap<String, Permission>,
}

impl Default for Capabilities {
    fn default() -> Self {
        Self {
            network_access: false,
            filesystem_access: false,
            process_creation: false,
            system_calls: Vec::new(),
            device_access: HashMap::new(),
        }
    }
}

/// Sandbox ID
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub struct SandboxId(pub u64);

/// Sandbox state
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum SandboxState {
    /// Created but not started
    Created,
    /// Running
    Running,
    /// Paused
    Paused,
    /// Terminated
    Terminated,
}

/// Sandbox
#[derive(Debug)]
pub struct Sandbox {
    /// Sandbox ID
    id: SandboxId,
    /// Sandbox name
    name: String,
    /// Sandbox state
    state: SandboxState,
    /// Resource limits
    resource_limits: ResourceLimits,
    /// Capabilities
    capabilities: Capabilities,
    /// Memory regions
    memory_regions: Vec<Arc<Mutex<IsolatedMemoryRegion>>>,
    /// Processes
    processes: Vec<ProcessId>,
    /// Access control list
    acl: AccessControlList,
    /// Working directory
    working_dir: PathBuf,
}

impl Sandbox {
    /// Create a new sandbox
    pub fn new(
        id: SandboxId,
        name: &str,
        resource_limits: ResourceLimits,
        capabilities: Capabilities,
        working_dir: PathBuf,
    ) -> Self {
        Self {
            id,
            name: name.to_string(),
            state: SandboxState::Created,
            resource_limits,
            capabilities,
            memory_regions: Vec::new(),
            processes: Vec::new(),
            acl: AccessControlList::new(),
            working_dir,
        }
    }
    
    /// Get sandbox ID
    pub fn id(&self) -> SandboxId {
        self.id
    }
    
    /// Get sandbox name
    pub fn name(&self) -> &str {
        &self.name
    }
    
    /// Get sandbox state
    pub fn state(&self) -> SandboxState {
        self.state
    }
    
    /// Get resource limits
    pub fn resource_limits(&self) -> &ResourceLimits {
        &self.resource_limits
    }
    
    /// Get capabilities
    pub fn capabilities(&self) -> &Capabilities {
        &self.capabilities
    }
    
    /// Get working directory
    pub fn working_dir(&self) -> &PathBuf {
        &self.working_dir
    }
    
    /// Start the sandbox
    pub fn start(&mut self) -> Result<(), SandboxError> {
        if self.state != SandboxState::Created && self.state != SandboxState::Paused {
            return Err(SandboxError::General("Sandbox is not in a startable state"));
        }
        
        // Allocate memory regions
        if self.memory_regions.is_empty() {
            let region = crate::kernel::memory::create_isolated_region(
                1024 * 1024, // 1 MB
                ProtectionFlags::read_write(),
                Some(format!("sandbox_{}_main", self.id.0)),
            ).map_err(|e| SandboxError::MemoryError(format!("{:?}", e)))?;
            
            self.memory_regions.push(Arc::new(Mutex::new(region)));
        }
        
        // Create initial process if none exists
        if self.processes.is_empty() {
            // In a real implementation, this would create a process
            // For this prototype, we just simulate it
            self.processes.push(ProcessId(1));
        }
        
        self.state = SandboxState::Running;
        
        Ok(())
    }
    
    /// Pause the sandbox
    pub fn pause(&mut self) -> Result<(), SandboxError> {
        if self.state != SandboxState::Running {
            return Err(SandboxError::General("Sandbox is not running"));
        }
        
        // Pause all processes
        // In a real implementation, this would pause all processes
        // For this prototype, we just update the state
        
        self.state = SandboxState::Paused;
        
        Ok(())
    }
    
    /// Resume the sandbox
    pub fn resume(&mut self) -> Result<(), SandboxError> {
        if self.state != SandboxState::Paused {
            return Err(SandboxError::General("Sandbox is not paused"));
        }
        
        // Resume all processes
        // In a real implementation, this would resume all processes
        // For this prototype, we just update the state
        
        self.state = SandboxState::Running;
        
        Ok(())
    }
    
    /// Terminate the sandbox
    pub fn terminate(&mut self) -> Result<(), SandboxError> {
        if self.state == SandboxState::Terminated {
            return Err(SandboxError::General("Sandbox is already terminated"));
        }
        
        // Terminate all processes
        // In a real implementation, this would terminate all processes
        // For this prototype, we just update the state
        
        // Free memory regions
        self.memory_regions.clear();
        
        self.state = SandboxState::Terminated;
        
        Ok(())
    }
    
    /// Execute code in the sandbox
    pub fn execute(&mut self, code: &str) -> Result<String, SandboxError> {
        if self.state != SandboxState::Running {
            return Err(SandboxError::General("Sandbox is not running"));
        }
        
        // Check resource limits
        // In a real implementation, this would check if executing the code
        // would exceed resource limits
        // For this prototype, we just simulate it
        
        // Check capabilities
        // In a real implementation, this would check if the code requires
        // capabilities that the sandbox doesn't have
        // For this prototype, we just simulate it
        
        // Execute the code
        // In a real implementation, this would actually execute the code
        // For this prototype, we just return a dummy result
        
        Ok(format!("Executed code in sandbox {}: {}", self.id.0, code))
    }
    
    /// Check if the sandbox has a capability
    pub fn has_capability(&self, capability: &str) -> bool {
        match capability {
            "network_access" => self.capabilities.network_access,
            "filesystem_access" => self.capabilities.filesystem_access,
            "process_creation" => self.capabilities.process_creation,
            _ => self.capabilities.system_calls.contains(&capability.to_string()),
        }
    }
    
    /// Get memory usage (in bytes)
    pub fn memory_usage(&self) -> usize {
        self.memory_regions.iter()
            .map(|region| {
                if let Ok(locked_region) = region.lock() {
                    locked_region.region.size
                } else {
                    0
                }
            })
            .sum()
    }
    
    /// Get CPU usage (in percentage)
    pub fn cpu_usage(&self) -> f32 {
        // In a real implementation, this would calculate actual CPU usage
        // For this prototype, we just return a dummy value
        5.0
    }
}

/// Sandbox registry
#[derive(Debug)]
pub struct SandboxRegistry {
    /// Registered sandboxes
    sandboxes: HashMap<SandboxId, Arc<Mutex<Sandbox>>>,
    /// Next sandbox ID
    next_id: u64,
    /// Configuration
    config: SandboxConfig,
    /// Terminated sandbox count
    terminated_count: usize,
}

impl SandboxRegistry {
    /// Create a new sandbox registry
    pub fn new() -> Self {
        Self {
            sandboxes: HashMap::new(),
            next_id: 1,
            config: SandboxConfig::default(),
            terminated_count: 0,
        }
    }
    
    /// Create a new sandbox
    pub fn create_sandbox(&mut self, name: &str, resource_limits: Option<ResourceLimits>, capabilities: Option<Capabilities>, working_dir: Option<PathBuf>) -> Result<SandboxId, SandboxError> {
        let id = SandboxId(self.next_id);
        self.next_id += 1;
        
        let resource_limits = resource_limits.unwrap_or_else(|| ResourceLimits {
            memory_limit: self.config.default_memory_limit,
            cpu_limit: self.config.default_cpu_limit,
            ..ResourceLimits::default()
        });
        
        let mut capabilities = capabilities.unwrap_or_else(Capabilities::default);
        capabilities.network_access = capabilities.network_access || self.config.default_network_access;
        capabilities.filesystem_access = capabilities.filesystem_access || self.config.default_filesystem_access;
        
        let working_dir = working_dir.unwrap_or_else(|| PathBuf::from("/tmp"));
        
        let sandbox = Sandbox::new(id, name, resource_limits, capabilities, working_dir);
        
        self.sandboxes.insert(id, Arc::new(Mutex::new(sandbox)));
        
        Ok(id)
    }
    
    /// Get a sandbox
    pub fn get_sandbox(&self, id: SandboxId) -> Result<Arc<Mutex<Sandbox>>, SandboxError> {
        self.sandboxes.get(&id)
            .cloned()
            .ok_or_else(|| SandboxError::General("Sandbox not found"))
    }
    
    /// Remove a sandbox
    pub fn remove_sandbox(&mut self, id: SandboxId) -> Result<(), SandboxError> {
        let sandbox = self.get_sandbox(id)?;
        let mut sandbox = sandbox.lock().map_err(|_| 
            SandboxError::General("Failed to lock sandbox"))?;
        
        if sandbox.state() != SandboxState::Terminated {
            sandbox.terminate()?;
        }
        
        self.sandboxes.remove(&id);
        self.terminated_count += 1;
        
        Ok(())
    }
    
    /// Set configuration
    pub fn set_config(&mut self, config: SandboxConfig) {
        self.config = config;
    }
    
    /// Get status
    pub fn get_status(&self) -> SandboxStatus {
        let active_count = self.sandboxes.len();
        let total_memory_usage = self.sandboxes.values()
            .map(|sandbox| {
                if let Ok(sandbox) = sandbox.lock() {
                    sandbox.memory_usage()
                } else {
                    0
                }
            })
            .sum();
        
        let total_cpu_usage = self.sandboxes.values()
            .map(|sandbox| {
                if let Ok(sandbox) = sandbox.lock() {
                    sandbox.cpu_usage()
                } else {
                    0.0
                }
            })
            .sum();
        
        SandboxStatus {
            active_sandbox_count: active_count,
            terminated_sandbox_count: self.terminated_count,
            total_memory_usage,
            total_cpu_usage,
        }
    }
}

/// Create a new sandbox
pub fn create_sandbox(name: &str, resource_limits: Option<ResourceLimits>, capabilities: Option<Capabilities>, working_dir: Option<PathBuf>) -> Result<SandboxId, SandboxError> {
    let mut registry = SANDBOX_REGISTRY.lock().map_err(|_| 
        SandboxError::General("Failed to lock registry"))?;
    registry.create_sandbox(name, resource_limits, capabilities, working_dir)
}

/// Start a sandbox
pub fn start_sandbox(id: SandboxId) -> Result<(), SandboxError> {
    let registry = SANDBOX_REGISTRY.lock().map_err(|_| 
        SandboxError::General("Failed to lock registry"))?;
    let sandbox = registry.get_sandb
(Content truncated due to size limit. Use line ranges to read in chunks)