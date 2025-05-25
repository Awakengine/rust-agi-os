use std::fmt;
use std::error::Error;
use std::sync::{Arc, Mutex};

/// Process error
#[derive(Debug)]
pub enum ProcessError {
    /// Creation error
    CreationError(String),
    /// Execution error
    ExecutionError(String),
    /// Termination error
    TerminationError(String),
    /// Other error
    Other(String),
}

impl Error for ProcessError {}

impl fmt::Display for ProcessError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            ProcessError::CreationError(msg) => write!(f, "Creation error: {}", msg),
            ProcessError::ExecutionError(msg) => write!(f, "Execution error: {}", msg),
            ProcessError::TerminationError(msg) => write!(f, "Termination error: {}", msg),
            ProcessError::Other(msg) => write!(f, "Other error: {}", msg),
        }
    }
}

/// Process state
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum ProcessState {
    /// Created
    Created,
    /// Running
    Running,
    /// Waiting
    Waiting,
    /// Terminated
    Terminated,
    /// Error
    Error,
}

impl fmt::Display for ProcessState {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            ProcessState::Created => write!(f, "Created"),
            ProcessState::Running => write!(f, "Running"),
            ProcessState::Waiting => write!(f, "Waiting"),
            ProcessState::Terminated => write!(f, "Terminated"),
            ProcessState::Error => write!(f, "Error"),
        }
    }
}

/// Process priority
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord)]
pub enum ProcessPriority {
    /// Low
    Low,
    /// Normal
    Normal,
    /// High
    High,
    /// Critical
    Critical,
}

impl fmt::Display for ProcessPriority {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            ProcessPriority::Low => write!(f, "Low"),
            ProcessPriority::Normal => write!(f, "Normal"),
            ProcessPriority::High => write!(f, "High"),
            ProcessPriority::Critical => write!(f, "Critical"),
        }
    }
}

/// Process
pub struct Process {
    /// Process ID
    pub id: String,
    /// Process name
    pub name: String,
    /// Process state
    pub state: ProcessState,
    /// Process priority
    pub priority: ProcessPriority,
    /// Process parent ID
    pub parent_id: Option<String>,
    /// Process memory blocks
    pub memory_blocks: Vec<String>,
    /// Process creation timestamp
    pub created_at: std::time::SystemTime,
    /// Process start timestamp
    pub started_at: Option<std::time::SystemTime>,
    /// Process termination timestamp
    pub terminated_at: Option<std::time::SystemTime>,
    /// Process error
    pub error: Option<String>,
}

impl Process {
    /// Create a new process
    pub fn new(name: &str, priority: ProcessPriority) -> Self {
        Self {
            id: uuid::Uuid::new_v4().to_string(),
            name: name.to_string(),
            state: ProcessState::Created,
            priority,
            parent_id: None,
            memory_blocks: Vec::new(),
            created_at: std::time::SystemTime::now(),
            started_at: None,
            terminated_at: None,
            error: None,
        }
    }
    
    /// Set parent
    pub fn set_parent(&mut self, parent_id: &str) {
        self.parent_id = Some(parent_id.to_string());
    }
    
    /// Add memory block
    pub fn add_memory_block(&mut self, block_id: &str) {
        self.memory_blocks.push(block_id.to_string());
    }
    
    /// Set state
    pub fn set_state(&mut self, state: ProcessState) {
        self.state = state;
        
        match state {
            ProcessState::Running => {
                self.started_at = Some(std::time::SystemTime::now());
            },
            ProcessState::Terminated => {
                self.terminated_at = Some(std::time::SystemTime::now());
            },
            _ => {},
        }
    }
    
    /// Set error
    pub fn set_error(&mut self, error: &str) {
        self.error = Some(error.to_string());
        self.state = ProcessState::Error;
    }
}

/// Process manager
pub struct ProcessManager {
    /// Processes
    pub processes: std::collections::HashMap<String, Process>,
    /// Process execution handlers
    pub execution_handlers: std::collections::HashMap<String, Box<dyn Fn() -> Result<(), ProcessError> + Send + Sync>>,
}

impl ProcessManager {
    /// Create a new process manager
    pub fn new() -> Result<Self, ProcessError> {
        Ok(Self {
            processes: std::collections::HashMap::new(),
            execution_handlers: std::collections::HashMap::new(),
        })
    }
    
    /// Create process
    pub fn create_process(&mut self, name: &str, priority: ProcessPriority) -> Result<String, ProcessError> {
        let process = Process::new(name, priority);
        let process_id = process.id.clone();
        
        self.processes.insert(process_id.clone(), process);
        
        Ok(process_id)
    }
    
    /// Get process
    pub fn get_process(&self, id: &str) -> Option<&Process> {
        self.processes.get(id)
    }
    
    /// Get process (mutable)
    pub fn get_process_mut(&mut self, id: &str) -> Option<&mut Process> {
        self.processes.get_mut(id)
    }
    
    /// Register execution handler
    pub fn register_execution_handler<F>(&mut self, process_id: &str, handler: F) -> Result<(), ProcessError>
    where
        F: Fn() -> Result<(), ProcessError> + Send + Sync + 'static,
    {
        if !self.processes.contains_key(process_id) {
            return Err(ProcessError::Other(format!(
                "Process not found: id={}",
                process_id
            )));
        }
        
        self.execution_handlers.insert(process_id.to_string(), Box::new(handler));
        Ok(())
    }
    
    /// Start process
    pub fn start_process(&mut self, id: &str) -> Result<(), ProcessError> {
        let process = self.processes.get_mut(id).ok_or_else(|| {
            ProcessError::ExecutionError(format!("Process not found: id={}", id))
        })?;
        
        if process.state != ProcessState::Created && process.state != ProcessState::Waiting {
            return Err(ProcessError::ExecutionError(format!(
                "Process is not in a startable state: id={}, state={:?}",
                id, process.state
            )));
        }
        
        process.set_state(ProcessState::Running);
        
        if let Some(handler) = self.execution_handlers.get(id) {
            if let Err(e) = handler() {
                process.set_error(&format!("Execution error: {}", e));
                return Err(e);
            }
        }
        
        Ok(())
    }
    
    /// Terminate process
    pub fn terminate_process(&mut self, id: &str) -> Result<(), ProcessError> {
        let process = self.processes.get_mut(id).ok_or_else(|| {
            ProcessError::TerminationError(format!("Process not found: id={}", id))
        })?;
        
        if process.state != ProcessState::Running && process.state != ProcessState::Waiting {
            return Err(ProcessError::TerminationError(format!(
                "Process is not in a terminable state: id={}, state={:?}",
                id, process.state
            )));
        }
        
        process.set_state(ProcessState::Terminated);
        
        Ok(())
    }
    
    /// Get processes by state
    pub fn get_processes_by_state(&self, state: ProcessState) -> Vec<&Process> {
        self.processes.values()
            .filter(|p| p.state == state)
            .collect()
    }
    
    /// Get processes by priority
    pub fn get_processes_by_priority(&self, priority: ProcessPriority) -> Vec<&Process> {
        self.processes.values()
            .filter(|p| p.priority == priority)
            .collect()
    }
    
    /// Get child processes
    pub fn get_child_processes(&self, parent_id: &str) -> Vec<&Process> {
        self.processes.values()
            .filter(|p| p.parent_id.as_ref().map_or(false, |id| id == parent_id))
            .collect()
    }
}

/// Initialize process module
pub fn init() -> Result<(), ProcessError> {
    // Initialize process module
    Ok(())
}

/// Start process module
pub fn start() -> Result<(), ProcessError> {
    // Start process module
    Ok(())
}

/// Stop process module
pub fn stop() -> Result<(), ProcessError> {
    // Stop process module
    Ok(())
}
