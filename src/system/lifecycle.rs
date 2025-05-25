use std::fmt;
use std::error::Error;

/// Lifecycle error
#[derive(Debug)]
pub enum LifecycleError {
    /// Initialization error
    InitializationError(String),
    /// Start error
    StartError(String),
    /// Stop error
    StopError(String),
    /// Other error
    Other(String),
}

impl Error for LifecycleError {}

impl fmt::Display for LifecycleError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            LifecycleError::InitializationError(msg) => write!(f, "Initialization error: {}", msg),
            LifecycleError::StartError(msg) => write!(f, "Start error: {}", msg),
            LifecycleError::StopError(msg) => write!(f, "Stop error: {}", msg),
            LifecycleError::Other(msg) => write!(f, "Other error: {}", msg),
        }
    }
}

/// Lifecycle state
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum LifecycleState {
    /// Uninitialized
    Uninitialized,
    /// Initializing
    Initializing,
    /// Initialized
    Initialized,
    /// Starting
    Starting,
    /// Running
    Running,
    /// Stopping
    Stopping,
    /// Stopped
    Stopped,
    /// Error
    Error,
}

impl fmt::Display for LifecycleState {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            LifecycleState::Uninitialized => write!(f, "Uninitialized"),
            LifecycleState::Initializing => write!(f, "Initializing"),
            LifecycleState::Initialized => write!(f, "Initialized"),
            LifecycleState::Starting => write!(f, "Starting"),
            LifecycleState::Running => write!(f, "Running"),
            LifecycleState::Stopping => write!(f, "Stopping"),
            LifecycleState::Stopped => write!(f, "Stopped"),
            LifecycleState::Error => write!(f, "Error"),
        }
    }
}

/// Lifecycle component
pub trait LifecycleComponent: Send + Sync {
    /// Get component name
    fn name(&self) -> &str;
    
    /// Initialize component
    fn initialize(&mut self) -> Result<(), LifecycleError>;
    
    /// Start component
    fn start(&mut self) -> Result<(), LifecycleError>;
    
    /// Stop component
    fn stop(&mut self) -> Result<(), LifecycleError>;
    
    /// Get component state
    fn state(&self) -> LifecycleState;
}

/// Component reference wrapper
pub struct ComponentRef<'a> {
    component: &'a mut Box<dyn LifecycleComponent>,
}

impl<'a> ComponentRef<'a> {
    /// Get component reference
    pub fn get(&mut self) -> &mut dyn LifecycleComponent {
        self.component.as_mut()
    }
}

/// Lifecycle manager
pub struct LifecycleManager {
    /// Components
    pub components: Vec<Box<dyn LifecycleComponent>>,
    /// State
    pub state: LifecycleState,
    /// State change handlers
    pub state_change_handlers: Vec<Box<dyn Fn(LifecycleState, LifecycleState) -> Result<(), LifecycleError> + Send + Sync>>,
}

impl LifecycleManager {
    /// Create a new lifecycle manager
    pub fn new() -> Result<Self, LifecycleError> {
        Ok(Self {
            components: Vec::new(),
            state: LifecycleState::Uninitialized,
            state_change_handlers: Vec::new(),
        })
    }
    
    /// Add component
    pub fn add_component<T>(&mut self, component: T) -> Result<(), LifecycleError>
    where
        T: LifecycleComponent + 'static,
    {
        self.components.push(Box::new(component));
        Ok(())
    }
    
    /// Initialize
    pub fn initialize(&mut self) -> Result<(), LifecycleError> {
        self.set_state(LifecycleState::Initializing)?;
        
        // Collect errors during initialization
        let mut init_error = None;
        let mut failed_component = String::new();
        
        for component in &mut self.components {
            if let Err(e) = component.initialize() {
                init_error = Some(e);
                failed_component = component.name().to_string();
                break;
            }
        }
        
        // Handle errors after the loop
        if let Some(e) = init_error {
            self.set_state(LifecycleState::Error)?;
            return Err(LifecycleError::InitializationError(format!(
                "Failed to initialize component {}: {}",
                failed_component,
                e
            )));
        }
        
        self.set_state(LifecycleState::Initialized)?;
        Ok(())
    }
    
    /// Start
    pub fn start(&mut self) -> Result<(), LifecycleError> {
        if self.state != LifecycleState::Initialized && self.state != LifecycleState::Stopped {
            return Err(LifecycleError::StartError(format!(
                "Cannot start from state {}",
                self.state
            )));
        }
        
        self.set_state(LifecycleState::Starting)?;
        
        // Collect errors during start
        let mut start_error = None;
        let mut failed_component = String::new();
        
        for component in &mut self.components {
            if let Err(e) = component.start() {
                start_error = Some(e);
                failed_component = component.name().to_string();
                break;
            }
        }
        
        // Handle errors after the loop
        if let Some(e) = start_error {
            self.set_state(LifecycleState::Error)?;
            return Err(LifecycleError::StartError(format!(
                "Failed to start component {}: {}",
                failed_component,
                e
            )));
        }
        
        self.set_state(LifecycleState::Running)?;
        Ok(())
    }
    
    /// Stop
    pub fn stop(&mut self) -> Result<(), LifecycleError> {
        if self.state != LifecycleState::Running {
            return Err(LifecycleError::StopError(format!(
                "Cannot stop from state {}",
                self.state
            )));
        }
        
        self.set_state(LifecycleState::Stopping)?;
        
        // Collect errors during stop
        let mut stop_error = None;
        let mut failed_component = String::new();
        
        // Stop components in reverse order
        for component in self.components.iter_mut().rev() {
            if let Err(e) = component.stop() {
                stop_error = Some(e);
                failed_component = component.name().to_string();
                break;
            }
        }
        
        // Handle errors after the loop
        if let Some(e) = stop_error {
            self.set_state(LifecycleState::Error)?;
            return Err(LifecycleError::StopError(format!(
                "Failed to stop component {}: {}",
                failed_component,
                e
            )));
        }
        
        self.set_state(LifecycleState::Stopped)?;
        Ok(())
    }
    
    /// Set state
    fn set_state(&mut self, new_state: LifecycleState) -> Result<(), LifecycleError> {
        let old_state = self.state;
        self.state = new_state;
        
        // Notify state change handlers
        for handler in &self.state_change_handlers {
            if let Err(e) = handler(old_state, new_state) {
                return Err(e);
            }
        }
        
        Ok(())
    }
    
    /// Add state change handler
    pub fn add_state_change_handler<F>(&mut self, handler: F)
    where
        F: Fn(LifecycleState, LifecycleState) -> Result<(), LifecycleError> + Send + Sync + 'static,
    {
        self.state_change_handlers.push(Box::new(handler));
    }
    
    /// Get component
    pub fn get_component(&self, name: &str) -> Option<&dyn LifecycleComponent> {
        self.components.iter()
            .find(|c| c.name() == name)
            .map(|c| c.as_ref())
    }
    
    /// Get component wrapper (mutable)
    pub fn get_component_mut<'a>(&'a mut self, name: &str) -> Option<ComponentRef<'a>> {
        let component = self.components.iter_mut().find(|c| c.name() == name)?;
        Some(ComponentRef { component })
    }
}

/// Initialize lifecycle module
pub fn init() -> Result<(), LifecycleError> {
    // Initialize lifecycle module
    Ok(())
}

/// Start lifecycle module
pub fn start() -> Result<(), LifecycleError> {
    // Start lifecycle module
    Ok(())
}

/// Stop lifecycle module
pub fn stop() -> Result<(), LifecycleError> {
    // Stop lifecycle module
    Ok(())
}
