//! # System Resource Module
//! 
//! This module provides resource management capabilities for the AGI operating system,
//! enabling allocation, tracking, and optimization of system resources.

use std::sync::{Arc, Mutex};
use std::collections::HashMap;
use std::time::Instant;
use std::fmt;
use std::error::Error;

/// Initialize the resource management subsystem
pub fn init() -> Result<(), ResourceError> {
    // Initialize resource management components
    Ok(())
}

/// Error type for resource operations
#[derive(Debug)]
pub enum ResourceError {
    /// Allocation error
    AllocationError(String),
    /// Limit error
    LimitError(String),
    /// Resource not found
    NotFound(String),
    /// Permission error
    PermissionError(String),
    /// Manager not initialized
    ManagerNotInitialized,
    /// General error
    General(&'static str),
}

// 实现Display trait，解决E0277错误
impl fmt::Display for ResourceError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            ResourceError::AllocationError(msg) => write!(f, "Allocation error: {}", msg),
            ResourceError::LimitError(msg) => write!(f, "Limit error: {}", msg),
            ResourceError::NotFound(msg) => write!(f, "Resource not found: {}", msg),
            ResourceError::PermissionError(msg) => write!(f, "Permission error: {}", msg),
            ResourceError::ManagerNotInitialized => write!(f, "Resource manager not initialized"),
            ResourceError::General(msg) => write!(f, "General resource error: {}", msg),
        }
    }
}

// 实现Error trait，解决?操作符错误转换问题
impl Error for ResourceError {}

/// Resource type
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum ResourceType {
    /// CPU
    CPU,
    /// Memory
    Memory,
    /// Storage
    Storage,
    /// Network
    Network,
    /// GPU
    GPU,
    /// Custom
    Custom,
}

/// Resource configuration
#[derive(Debug, Clone)]
pub struct ResourceConfig {
    /// Enable resource limits
    pub enable_limits: bool,
    /// CPU limit (percentage)
    pub cpu_limit: f64,
    /// Memory limit (bytes)
    pub memory_limit: u64,
    /// Storage limit (bytes)
    pub storage_limit: u64,
    /// Network bandwidth limit (bytes per second)
    pub network_limit: u64,
    /// GPU memory limit (bytes)
    pub gpu_memory_limit: u64,
    /// Enable resource optimization
    pub enable_optimization: bool,
    /// Resource allocation strategy
    pub allocation_strategy: String,
}

impl Default for ResourceConfig {
    fn default() -> Self {
        Self {
            enable_limits: true,
            cpu_limit: 90.0,
            memory_limit: 8 * 1024 * 1024 * 1024, // 8 GB
            storage_limit: 100 * 1024 * 1024 * 1024, // 100 GB
            network_limit: 100 * 1024 * 1024, // 100 MB/s
            gpu_memory_limit: 4 * 1024 * 1024 * 1024, // 4 GB
            enable_optimization: true,
            allocation_strategy: "balanced".to_string(),
        }
    }
}

/// Resource status
#[derive(Debug, Clone)]
pub struct ResourceStatus {
    /// CPU usage (percentage)
    pub cpu_usage: f64,
    /// Memory usage (bytes)
    pub memory_usage: u64,
    /// Storage usage (bytes)
    pub storage_usage: u64,
    /// Network usage (bytes per second)
    pub network_usage: u64,
    /// GPU memory usage (bytes)
    pub gpu_memory_usage: u64,
    /// Number of resource allocations
    pub allocation_count: usize,
    /// Number of resource limits
    pub limit_count: usize,
}

/// Resource allocation
#[derive(Debug)]
pub struct ResourceAllocation {
    /// Allocation ID
    id: String,
    /// Resource type
    resource_type: ResourceType,
    /// Amount
    amount: u64,
    /// Owner
    owner: String,
    /// Creation time
    creation_time: Instant,
    /// Expiration time (optional)
    expiration_time: Option<Instant>,
    /// Priority
    priority: u32,
    /// Tags
    tags: HashMap<String, String>,
}

impl ResourceAllocation {
    /// Create a new resource allocation
    pub fn new(
        id: &str,
        resource_type: ResourceType,
        amount: u64,
        owner: &str,
        priority: u32,
    ) -> Self {
        Self {
            id: id.to_string(),
            resource_type,
            amount,
            owner: owner.to_string(),
            creation_time: Instant::now(),
            expiration_time: None,
            priority,
            tags: HashMap::new(),
        }
    }
    
    /// Get allocation ID
    pub fn id(&self) -> &str {
        &self.id
    }
    
    /// Get resource type
    pub fn resource_type(&self) -> ResourceType {
        self.resource_type
    }
    
    /// Get amount
    pub fn amount(&self) -> u64 {
        self.amount
    }
    
    /// Get owner
    pub fn owner(&self) -> &str {
        &self.owner
    }
    
    /// Get creation time
    pub fn creation_time(&self) -> Instant {
        self.creation_time
    }
    
    /// Get expiration time
    pub fn expiration_time(&self) -> Option<Instant> {
        self.expiration_time
    }
    
    /// Set expiration time
    pub fn set_expiration_time(&mut self, expiration_time: Option<Instant>) {
        self.expiration_time = expiration_time;
    }
    
    /// Get priority
    pub fn priority(&self) -> u32 {
        self.priority
    }
    
    /// Set priority
    pub fn set_priority(&mut self, priority: u32) {
        self.priority = priority;
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
    
    /// Is expired
    pub fn is_expired(&self) -> bool {
        if let Some(expiration_time) = self.expiration_time {
            Instant::now() >= expiration_time
        } else {
            false
        }
    }
}

/// Resource limit
#[derive(Debug)]
pub struct ResourceLimit {
    /// Limit ID
    id: String,
    /// Resource type
    resource_type: ResourceType,
    /// Maximum amount
    max_amount: u64,
    /// Current usage
    current_usage: u64,
    /// Owner
    owner: String,
    /// Creation time
    creation_time: Instant,
    /// Expiration time (optional)
    expiration_time: Option<Instant>,
    /// Tags
    tags: HashMap<String, String>,
}

impl ResourceLimit {
    /// Create a new resource limit
    pub fn new(
        id: &str,
        resource_type: ResourceType,
        max_amount: u64,
        owner: &str,
    ) -> Self {
        Self {
            id: id.to_string(),
            resource_type,
            max_amount,
            current_usage: 0,
            owner: owner.to_string(),
            creation_time: Instant::now(),
            expiration_time: None,
            tags: HashMap::new(),
        }
    }
    
    /// Get limit ID
    pub fn id(&self) -> &str {
        &self.id
    }
    
    /// Get resource type
    pub fn resource_type(&self) -> ResourceType {
        self.resource_type
    }
    
    /// Get maximum amount
    pub fn max_amount(&self) -> u64 {
        self.max_amount
    }
    
    /// Set maximum amount
    pub fn set_max_amount(&mut self, max_amount: u64) {
        self.max_amount = max_amount;
    }
    
    /// Get current usage
    pub fn current_usage(&self) -> u64 {
        self.current_usage
    }
    
    /// Update current usage
    pub fn update_usage(&mut self, usage: u64) {
        self.current_usage = usage;
    }
    
    /// Get owner
    pub fn owner(&self) -> &str {
        &self.owner
    }
    
    /// Get creation time
    pub fn creation_time(&self) -> Instant {
        self.creation_time
    }
    
    /// Get expiration time
    pub fn expiration_time(&self) -> Option<Instant> {
        self.expiration_time
    }
    
    /// Set expiration time
    pub fn set_expiration_time(&mut self, expiration_time: Option<Instant>) {
        self.expiration_time = expiration_time;
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
    
    /// Is expired
    pub fn is_expired(&self) -> bool {
        if let Some(expiration_time) = self.expiration_time {
            Instant::now() >= expiration_time
        } else {
            false
        }
    }
    
    /// Can allocate
    pub fn can_allocate(&self, amount: u64) -> bool {
        self.current_usage + amount <= self.max_amount
    }
    
    /// Allocate
    pub fn allocate(&mut self, amount: u64) -> Result<(), ResourceError> {
        if !self.can_allocate(amount) {
            return Err(ResourceError::LimitError(
                format!("Resource limit exceeded: {} + {} > {}", 
                    self.current_usage, amount, self.max_amount)
            ));
        }
        
        self.current_usage += amount;
        Ok(())
    }
    
    /// Deallocate
    pub fn deallocate(&mut self, amount: u64) -> Result<(), ResourceError> {
        if amount > self.current_usage {
            return Err(ResourceError::LimitError(
                format!("Cannot deallocate more than current usage: {} > {}", 
                    amount, self.current_usage)
            ));
        }
        
        self.current_usage -= amount;
        Ok(())
    }
}

/// Resource manager
#[derive(Debug)]
pub struct ResourceManager {
    /// Allocations
    allocations: HashMap<String, Arc<Mutex<ResourceAllocation>>>,
    /// Limits
    limits: HashMap<String, Arc<Mutex<ResourceLimit>>>,
    /// Configuration
    config: ResourceConfig,
    /// Status
    status: ResourceStatus,
}

impl ResourceManager {
    /// Create a new resource manager
    pub fn new(config: ResourceConfig) -> Self {
        Self {
            allocations: HashMap::new(),
            limits: HashMap::new(),
            config,
            status: ResourceStatus {
                cpu_usage: 0.0,
                memory_usage: 0,
                storage_usage: 0,
                network_usage: 0,
                gpu_memory_usage: 0,
                allocation_count: 0,
                limit_count: 0,
            },
        }
    }
    
    /// Allocate resource
    pub fn allocate_resource(
        &mut self,
        resource_type: ResourceType,
        amount: u64,
        owner: &str,
        priority: u32,
    ) -> Result<String, ResourceError> {
        if self.config.enable_limits {
            // Check global limits
            match resource_type {
                ResourceType::CPU => {
                    let cpu_usage = self.status.cpu_usage + (amount as f64 / 100.0);
                    if cpu_usage > self.config.cpu_limit {
                        return Err(ResourceError::LimitError(
                            format!("CPU limit exceeded: {} > {}", 
                                cpu_usage, self.config.cpu_limit)
                        ));
                    }
                }
                ResourceType::Memory => {
                    let memory_usage = self.status.memory_usage + amount;
                    if memory_usage > self.config.memory_limit {
                        return Err(ResourceError::LimitError(
                            format!("Memory limit exceeded: {} > {}", 
                                memory_usage, self.config.memory_limit)
                        ));
                    }
                }
                ResourceType::Storage => {
                    let storage_usage = self.status.storage_usage + amount;
                    if storage_usage > self.config.storage_limit {
                        return Err(ResourceError::LimitError(
                            format!("Storage limit exceeded: {} > {}", 
                                storage_usage, self.config.storage_limit)
                        ));
                    }
                }
                ResourceType::Network => {
                    let network_usage = self.status.network_usage + amount;
                    if network_usage > self.config.network_limit {
                        return Err(ResourceError::LimitError(
                            format!("Network limit exceeded: {} > {}", 
                                network_usage, self.config.network_limit)
                        ));
                    }
                }
                ResourceType::GPU => {
                    let gpu_memory_usage = self.status.gpu_memory_usage + amount;
                    if gpu_memory_usage > self.config.gpu_memory_limit {
                        return Err(ResourceError::LimitError(
                            format!("GPU memory limit exceeded: {} > {}", 
                                gpu_memory_usage, self.config.gpu_memory_limit)
                        ));
                    }
                }
                ResourceType::Custom => {
                    // No global limit for custom resources
                }
            }
            
            // Check owner-specific limits
            for limit in self.limits.values() {
                let limit = limit.lock().map_err(|_| 
                    ResourceError::General("Failed to lock limit"))?;
                
                if limit.owner() == owner && limit.resource_type() == resource_type {
                    if !limit.can_allocate(amount) {
                        return Err(ResourceError::LimitError(
                            format!("Owner-specific limit exceeded: {} + {} > {}", 
                                limit.current_usage(), amount, limit.max_amount())
                        ));
                    }
                }
            }
        }
        
        // Generate allocation ID
        let id = format!("alloc-{}-{}-{}", 
            resource_type as u32,
            owner,
            self.allocations.len()
        );
        
        // Create allocation
        let allocation = ResourceAllocation::new(
            &id,
            resource_type,
            amount,
            owner,
            priority,
        );
        
        // Update owner-specific limits
        if self.config.enable_limits {
            for limit in self.limits.values() {
                let mut limit = limit.lock().map_err(|_| 
                    ResourceError::General("Failed to lock limit"))?;
                
                if limit.owner() == owner && limit.resource_type() == resource_type {
                    limit.allocate(amount)?;
                }
            }
        }
        
        // Update global usage
        match resource_type {
            ResourceType::CPU => {
                self.status.cpu_usage += amount as f64 / 100.0;
            }
            ResourceType::Memory => {
                self.status.memory_usage += amount;
            }
            ResourceType::Storage => {
                self.status.storage_usage += amount;
            }
            ResourceType::Network => {
                self.status.network_usage += amount;
            }
            ResourceType::GPU => {
                self.status.gpu_memory_usage += amount;
            }
            ResourceType::Custom => {
                // No global usage for custom resources
            }
        }
        
        // Store allocation
        self.allocations.insert(id.clone(), Arc::new(Mutex::new(allocation)));
        self.status.allocation_count = self.allocations.len();
        
        Ok(id)
    }
    
    /// Deallocate resource
    pub fn deallocate_resource(&mut self, allocation_id: &str) -> Result<(), ResourceError> {
        let allocation = self.allocations.get(allocation_id).ok_or_else(|| 
            ResourceError::NotFound(format!("Allocation not found: {}", allocation_id)))?;
        
        let
(Content truncated due to size limit. Use line ranges to read in chunks)