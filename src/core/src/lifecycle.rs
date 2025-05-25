//! # System Lifecycle Module
//! 
//! This module provides lifecycle management capabilities for the AGI operating system,
//! enabling controlled startup, shutdown, and state transitions.

use std::sync::{Arc, Mutex};
use std::collections::HashMap;
use std::time::{Duration, Instant};
use std::fmt;

/// Initialize the lifecycle subsystem
pub fn init() -> Result<(), LifecycleError> {
    // Initialize lifecycle components
    Ok(())
}

/// Error type for lifecycle operations
#[derive(Debug)]
pub enum LifecycleError {
    /// Initialization error
    InitializationError(String),
    /// Transition error
    TransitionError(String),
    /// Component error
    ComponentError(String),
    /// General error
    General(&'static str),
}

impl std::fmt::Display for LifecycleError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            LifecycleError::InitializationError(msg) => write!(f, "Initialization error: {}", msg),
            LifecycleError::TransitionError(msg) => write!(f, "Transition error: {}", msg),
            LifecycleError::ComponentError(msg) => write!(f, "Component error: {}", msg),
            LifecycleError::General(msg) => write!(f, "General lifecycle error: {}", msg),
        }
    }
}

impl std::error::Error for LifecycleError {}

/// System state
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum SystemState {
    /// Uninitialized
    Uninitialized,
    /// Initializing
    Initializing,
    /// Running
    Running,
    /// Paused
    Paused,
    /// Shutting down
    ShuttingDown,
    /// Terminated
    Terminated,
    /// Error
    Error,
}

/// Component state
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum ComponentState {
    /// Uninitialized
    Uninitialized,
    /// Initializing
    Initializing,
    /// Running
    Running,
    /// Paused
    Paused,
    /// Shutting down
    ShuttingDown,
    /// Terminated
    Terminated,
    /// Error
    Error,
}

/// Lifecycle configuration
#[derive(Debug, Clone)]
pub struct LifecycleConfig {
    /// Startup timeout (in seconds)
    pub startup_timeout: u64,
    /// Shutdown timeout (in seconds)
    pub shutdown_timeout: u64,
    /// Enable graceful shutdown
    pub enable_graceful_shutdown: bool,
    /// Enable automatic recovery
    pub enable_automatic_recovery: bool,
    /// Maximum recovery attempts
    pub max_recovery_attempts: u32,
}

impl Default for LifecycleConfig {
    fn default() -> Self {
        Self {
            startup_timeout: 60,
            shutdown_timeout: 30,
            enable_graceful_shutdown: true,
            enable_automatic_recovery: true,
            max_recovery_attempts: 3,
        }
    }
}

/// Lifecycle status
#[derive(Debug, Clone)]
pub struct LifecycleStatus {
    /// Current system state
    pub system_state: SystemState,
    /// Number of registered components
    pub component_count: usize,
    /// Number of running components
    pub running_component_count: usize,
    /// Number of failed components
    pub failed_component_count: usize,
    /// System uptime (in seconds)
    pub uptime: u64,
    /// Last state transition time
    pub last_transition_time: Option<Instant>,
    /// Recovery attempts
    pub recovery_attempts: u32,
}

/// Function wrapper with Debug implementation for lifecycle functions
pub struct LifecycleFn {
    /// The actual function
    func: Box<dyn Fn() -> Result<(), String> + Send + Sync>,
    /// Function description for debug purposes
    description: String,
}

impl LifecycleFn {
    /// Create a new lifecycle function wrapper
    pub fn new<F>(func: F, description: &str) -> Self
    where
        F: Fn() -> Result<(), String> + Send + Sync + 'static,
    {
        Self {
            func: Box::new(func),
            description: description.to_string(),
        }
    }
    
    /// Call the function
    pub fn call(&self) -> Result<(), String> {
        (self.func)()
    }
}

// Implement Debug for LifecycleFn
impl fmt::Debug for LifecycleFn {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.debug_struct("LifecycleFn")
            .field("description", &self.description)
            .field("func", &"<function>")
            .finish()
    }
}

/// Component
#[derive(Debug)]
pub struct Component {
    /// Component ID
    id: String,
    /// Component name
    name: String,
    /// Component state
    state: ComponentState,
    /// Dependencies
    dependencies: Vec<String>,
    /// Initialization function
    init_fn: LifecycleFn,
    /// Shutdown function
    shutdown_fn: LifecycleFn,
    /// Pause function
    pause_fn: Option<LifecycleFn>,
    /// Resume function
    resume_fn: Option<LifecycleFn>,
    /// Last state transition time
    last_transition_time: Option<Instant>,
    /// Error message
    error_message: Option<String>,
}

impl Component {
    /// Create a new component
    pub fn new<F1, F2>(
        id: &str,
        name: &str,
        dependencies: Vec<String>,
        init_fn: F1,
        shutdown_fn: F2,
    ) -> Self
    where
        F1: Fn() -> Result<(), String> + Send + Sync + 'static,
        F2: Fn() -> Result<(), String> + Send + Sync + 'static,
    {
        Self {
            id: id.to_string(),
            name: name.to_string(),
            state: ComponentState::Uninitialized,
            dependencies,
            init_fn: LifecycleFn::new(init_fn, &format!("{}_init", id)),
            shutdown_fn: LifecycleFn::new(shutdown_fn, &format!("{}_shutdown", id)),
            pause_fn: None,
            resume_fn: None,
            last_transition_time: None,
            error_message: None,
        }
    }
    
    /// Set pause function
    pub fn set_pause_fn<F>(&mut self, pause_fn: F)
    where
        F: Fn() -> Result<(), String> + Send + Sync + 'static,
    {
        self.pause_fn = Some(LifecycleFn::new(pause_fn, &format!("{}_pause", self.id)));
    }
    
    /// Set resume function
    pub fn set_resume_fn<F>(&mut self, resume_fn: F)
    where
        F: Fn() -> Result<(), String> + Send + Sync + 'static,
    {
        self.resume_fn = Some(LifecycleFn::new(resume_fn, &format!("{}_resume", self.id)));
    }
    
    /// Get component ID
    pub fn id(&self) -> &str {
        &self.id
    }
    
    /// Get component name
    pub fn name(&self) -> &str {
        &self.name
    }
    
    /// Get component state
    pub fn state(&self) -> ComponentState {
        self.state
    }
    
    /// Get dependencies
    pub fn dependencies(&self) -> &[String] {
        &self.dependencies
    }
    
    /// Get last state transition time
    pub fn last_transition_time(&self) -> Option<Instant> {
        self.last_transition_time
    }
    
    /// Get error message
    pub fn error_message(&self) -> Option<&str> {
        self.error_message.as_deref()
    }
    
    /// Initialize the component
    pub fn initialize(&mut self) -> Result<(), String> {
        if self.state != ComponentState::Uninitialized {
            return Err(format!("Component {} is not in uninitialized state", self.id));
        }
        
        self.state = ComponentState::Initializing;
        self.last_transition_time = Some(Instant::now());
        
        match self.init_fn.call() {
            Ok(()) => {
                self.state = ComponentState::Running;
                self.last_transition_time = Some(Instant::now());
                Ok(())
            }
            Err(e) => {
                self.state = ComponentState::Error;
                self.error_message = Some(e.clone());
                self.last_transition_time = Some(Instant::now());
                Err(e)
            }
        }
    }
    
    /// Shutdown the component
    pub fn shutdown(&mut self) -> Result<(), String> {
        if self.state == ComponentState::Uninitialized || self.state == ComponentState::Terminated {
            return Ok(());
        }
        
        self.state = ComponentState::ShuttingDown;
        self.last_transition_time = Some(Instant::now());
        
        match self.shutdown_fn.call() {
            Ok(()) => {
                self.state = ComponentState::Terminated;
                self.last_transition_time = Some(Instant::now());
                Ok(())
            }
            Err(e) => {
                self.state = ComponentState::Error;
                self.error_message = Some(e.clone());
                self.last_transition_time = Some(Instant::now());
                Err(e)
            }
        }
    }
    
    /// Pause the component
    pub fn pause(&mut self) -> Result<(), String> {
        if self.state != ComponentState::Running {
            return Err(format!("Component {} is not in running state", self.id));
        }
        
        if let Some(pause_fn) = &self.pause_fn {
            match pause_fn.call() {
                Ok(()) => {
                    self.state = ComponentState::Paused;
                    self.last_transition_time = Some(Instant::now());
                    Ok(())
                }
                Err(e) => {
                    self.error_message = Some(e.clone());
                    Err(e)
                }
            }
        } else {
            Err(format!("Component {} does not support pause", self.id))
        }
    }
    
    /// Resume the component
    pub fn resume(&mut self) -> Result<(), String> {
        if self.state != ComponentState::Paused {
            return Err(format!("Component {} is not in paused state", self.id));
        }
        
        if let Some(resume_fn) = &self.resume_fn {
            match resume_fn.call() {
                Ok(()) => {
                    self.state = ComponentState::Running;
                    self.last_transition_time = Some(Instant::now());
                    Ok(())
                }
                Err(e) => {
                    self.error_message = Some(e.clone());
                    Err(e)
                }
            }
        } else {
            Err(format!("Component {} does not support resume", self.id))
        }
    }
    
    /// Reset error state
    pub fn reset_error(&mut self) {
        if self.state == ComponentState::Error {
            self.state = ComponentState::Uninitialized;
            self.error_message = None;
            self.last_transition_time = Some(Instant::now());
        }
    }
}

/// Lifecycle manager
#[derive(Debug)]
pub struct LifecycleManager {
    /// Components
    components: HashMap<String, Arc<Mutex<Component>>>,
    /// System state
    system_state: SystemState,
    /// Configuration
    config: LifecycleConfig,
    /// Status
    status: LifecycleStatus,
    /// Start time
    start_time: Option<Instant>,
}

impl LifecycleManager {
    /// Create a new lifecycle manager
    pub fn new(config: LifecycleConfig) -> Self {
        Self {
            components: HashMap::new(),
            system_state: SystemState::Uninitialized,
            config,
            status: LifecycleStatus {
                system_state: SystemState::Uninitialized,
                component_count: 0,
                running_component_count: 0,
                failed_component_count: 0,
                uptime: 0,
                last_transition_time: None,
                recovery_attempts: 0,
            },
            start_time: None,
        }
    }
    
    /// Register a component
    pub fn register_component(&mut self, component: Component) -> Result<(), LifecycleError> {
        let id = component.id().to_string();
        
        if self.components.contains_key(&id) {
            return Err(LifecycleError::General("Component already registered"));
        }
        
        self.components.insert(id, Arc::new(Mutex::new(component)));
        self.update_status();
        
        Ok(())
    }
    
    /// Unregister a component
    pub fn unregister_component(&mut self, component_id: &str) -> Result<(), LifecycleError> {
        if !self.components.contains_key(component_id) {
            return Err(LifecycleError::General("Component not found"));
        }
        
        // Shutdown the component if it's running
        {
            let component = self.components.get(component_id).unwrap();
            let mut component = component.lock().map_err(|_| 
                LifecycleError::General("Failed to lock component"))?;
            
            if component.state() != ComponentState::Uninitialized && 
               component.state() != ComponentState::Terminated {
                component.shutdown().map_err(|e| 
                    LifecycleError::ComponentError(e))?;
            }
        }
        
        // Now that we've released the lock, we can safely remove the component and update status
        self.components.remove(component_id);
        self.update_status();
        
        Ok(())
    }
    
    /// Get a component
    pub fn get_component(&self, component_id: &str) -> Result<Arc<Mutex<Component>>, LifecycleError> {
        self.components.get(component_id)
            .cloned()
            .ok_or_else(|| LifecycleError::General("Component not found"))
    }
    
    /// Start the system
    pub fn start_system(&mut self) -> Result<(), LifecycleError> {
        if self.system_state != SystemState::Uninitialized && 
           self.system_state != SystemState::Terminated {
            return Err(LifecycleError::TransitionError(
                format!("System is not in uninitialized or terminated state: {:?}", self.system_state)
            ));
        }
        
        self.system_state = SystemState::Initializing;
        self.status.last_transition_time = Some(Instant::now());
        self.update_status();
        
        // Build dependency graph and initialize components in order
        let mut initialized_components = std::collections::HashSet::new();
        let mut remaining_components: Vec<String> = self.components.keys().cloned().collect();
        
        let start_time = Instant::now();
        let timeout = Duration::from_secs(self.config.startup_timeout);
        
        while !remaining_components.is_empty() {
            if start_time.elapsed() > timeout {
                self.system_state = SystemState::Error;
                self.update_status();
                return Err(LifecycleError::InitializationError(
                    format!("Startup timeout after {} seconds", self.config.startup_timeout)
                ));
            }
            
            let mut progress = false;
            
            // Find components whose dependencies are all initialized
            let mut i = 0;
            while i < remaining_components.len() {
                let component_id = &remaining_components[i];
                
                // Check dependencies in a separate scope to limit the borrow
                let all_deps_initialized = {
                    let component = self.components.get(component_id).unwrap();
                    let component = component.lock().map_err(|_| 
                        LifecycleError::General("Failed to lock component"))?;
                    
                    let dependencies = component.dependencies();
                    dependencies.iter().all(|dep| initialized_components.contains(dep))
                };
                
                if all_deps_initialized {
                    // Initialize this component
                    let init_result = {
                        let component = self.components.get(component_id).unwrap();
                        let mut component = component.lock().map_err(|_| 
                            LifecycleError::General("Failed to lock component"))?;
                        
                        component.initialize()
                    };
                    
                    // Handle initialization result outside the lock scope
                    if let Err(e) = init_result {
                        self.system_state = SystemState::Error;
                        self.update_status();
        
(Content truncated due to size limit. Use line ranges to read in chunks)