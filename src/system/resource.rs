use std::fmt;
use std::error::Error;
use std::sync::{Arc, Mutex};

/// Resource error
#[derive(Debug)]
pub enum ResourceError {
    /// Initialization error
    InitializationError(String),
    /// Allocation error
    AllocationError(String),
    /// Deallocation error
    DeallocationError(String),
    /// Other error
    Other(String),
}

impl Error for ResourceError {}

impl fmt::Display for ResourceError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            ResourceError::InitializationError(msg) => write!(f, "Initialization error: {}", msg),
            ResourceError::AllocationError(msg) => write!(f, "Allocation error: {}", msg),
            ResourceError::DeallocationError(msg) => write!(f, "Deallocation error: {}", msg),
            ResourceError::Other(msg) => write!(f, "Other error: {}", msg),
        }
    }
}

/// Resource type
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum ResourceType {
    /// CPU
    CPU,
    /// Memory
    Memory,
    /// Disk
    Disk,
    /// Network
    Network,
    /// GPU
    GPU,
    /// Other
    Other,
}

impl fmt::Display for ResourceType {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            ResourceType::CPU => write!(f, "CPU"),
            ResourceType::Memory => write!(f, "Memory"),
            ResourceType::Disk => write!(f, "Disk"),
            ResourceType::Network => write!(f, "Network"),
            ResourceType::GPU => write!(f, "GPU"),
            ResourceType::Other => write!(f, "Other"),
        }
    }
}

/// Resource
#[derive(Debug, Clone)]
pub struct Resource {
    /// Resource ID
    pub id: String,
    /// Resource name
    pub name: String,
    /// Resource type
    pub resource_type: ResourceType,
    /// Resource capacity
    pub capacity: f64,
    /// Resource used
    pub used: f64,
    /// Resource unit
    pub unit: String,
    /// Resource owner
    pub owner: Option<String>,
    /// Resource allocation timestamp
    pub allocation_timestamp: std::time::SystemTime,
}

impl Resource {
    /// Create a new resource
    pub fn new(name: &str, resource_type: ResourceType, capacity: f64, unit: &str) -> Self {
        Self {
            id: uuid::Uuid::new_v4().to_string(),
            name: name.to_string(),
            resource_type,
            capacity,
            used: 0.0,
            unit: unit.to_string(),
            owner: None,
            allocation_timestamp: std::time::SystemTime::now(),
        }
    }
    
    /// Set owner
    pub fn set_owner(&mut self, owner: &str) {
        self.owner = Some(owner.to_string());
    }
    
    /// Allocate resource
    pub fn allocate(&mut self, amount: f64) -> Result<(), ResourceError> {
        if self.used + amount > self.capacity {
            return Err(ResourceError::AllocationError(format!(
                "Not enough capacity: used={}, amount={}, capacity={}",
                self.used, amount, self.capacity
            )));
        }
        
        self.used += amount;
        Ok(())
    }
    
    /// Deallocate resource
    pub fn deallocate(&mut self, amount: f64) -> Result<(), ResourceError> {
        if self.used < amount {
            return Err(ResourceError::DeallocationError(format!(
                "Cannot deallocate more than used: used={}, amount={}",
                self.used, amount
            )));
        }
        
        self.used -= amount;
        Ok(())
    }
    
    /// Get usage
    pub fn get_usage(&self) -> f64 {
        self.used / self.capacity
    }
}

/// Resource manager
pub struct ResourceManager {
    /// Resources
    pub resources: std::collections::HashMap<String, Resource>,
    /// Resource handlers
    pub resource_handlers: Vec<Box<dyn Fn(&Resource) -> Result<(), ResourceError> + Send + Sync>>,
}

impl ResourceManager {
    /// Create a new resource manager
    pub fn new() -> Result<Self, ResourceError> {
        Ok(Self {
            resources: std::collections::HashMap::new(),
            resource_handlers: Vec::new(),
        })
    }
    
    /// Add resource
    pub fn add_resource(&mut self, resource: Resource) -> Result<(), ResourceError> {
        // Notify resource handlers
        for handler in &self.resource_handlers {
            if let Err(e) = handler(&resource) {
                return Err(e);
            }
        }
        
        self.resources.insert(resource.id.clone(), resource);
        Ok(())
    }
    
    /// Get resource
    pub fn get_resource(&self, id: &str) -> Option<&Resource> {
        self.resources.get(id)
    }
    
    /// Get resource (mutable)
    pub fn get_resource_mut(&mut self, id: &str) -> Option<&mut Resource> {
        self.resources.get_mut(id)
    }
    
    /// Allocate resource
    pub fn allocate_resource(&mut self, id: &str, amount: f64) -> Result<(), ResourceError> {
        let resource = self.resources.get_mut(id).ok_or_else(|| {
            ResourceError::AllocationError(format!("Resource not found: id={}", id))
        })?;
        
        resource.allocate(amount)?;
        
        // Notify resource handlers
        for handler in &self.resource_handlers {
            if let Err(e) = handler(resource) {
                return Err(e);
            }
        }
        
        Ok(())
    }
    
    /// Deallocate resource
    pub fn deallocate_resource(&mut self, id: &str, amount: f64) -> Result<(), ResourceError> {
        let resource = self.resources.get_mut(id).ok_or_else(|| {
            ResourceError::DeallocationError(format!("Resource not found: id={}", id))
        })?;
        
        resource.deallocate(amount)?;
        
        // Notify resource handlers
        for handler in &self.resource_handlers {
            if let Err(e) = handler(resource) {
                return Err(e);
            }
        }
        
        Ok(())
    }
    
    /// Get resources by type
    pub fn get_resources_by_type(&self, resource_type: ResourceType) -> Vec<&Resource> {
        self.resources.values()
            .filter(|r| r.resource_type == resource_type)
            .collect()
    }
    
    /// Get resources by owner
    pub fn get_resources_by_owner(&self, owner: &str) -> Vec<&Resource> {
        self.resources.values()
            .filter(|r| r.owner.as_ref().map_or(false, |o| o == owner))
            .collect()
    }
    
    /// Add resource handler
    pub fn add_resource_handler<F>(&mut self, handler: F)
    where
        F: Fn(&Resource) -> Result<(), ResourceError> + Send + Sync + 'static,
    {
        self.resource_handlers.push(Box::new(handler));
    }
}

/// Initialize resource module
pub fn init() -> Result<(), ResourceError> {
    // Initialize resource module
    Ok(())
}

/// Start resource module
pub fn start() -> Result<(), ResourceError> {
    // Start resource module
    Ok(())
}

/// Stop resource module
pub fn stop() -> Result<(), ResourceError> {
    // Stop resource module
    Ok(())
}
