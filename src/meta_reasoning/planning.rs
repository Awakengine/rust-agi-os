use std::fmt;
use std::error::Error;
use std::collections::HashMap;

/// Planning error
#[derive(Debug)]
pub enum PlanningError {
    /// Initialization error
    InitializationError(String),
    /// Processing error
    ProcessingError(String),
    /// Other error
    Other(String),
}

impl Error for PlanningError {}

impl fmt::Display for PlanningError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            PlanningError::InitializationError(msg) => write!(f, "Initialization error: {}", msg),
            PlanningError::ProcessingError(msg) => write!(f, "Processing error: {}", msg),
            PlanningError::Other(msg) => write!(f, "Other error: {}", msg),
        }
    }
}

/// Plan step
#[derive(Debug, Clone)]
pub struct PlanStep {
    /// Step ID
    pub id: String,
    /// Step description
    pub description: String,
    /// Step dependencies
    pub dependencies: Vec<String>,
    /// Step status
    pub status: PlanStepStatus,
    /// Step metadata
    pub metadata: HashMap<String, String>,
}

/// Plan step status
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum PlanStepStatus {
    /// Not started
    NotStarted,
    /// In progress
    InProgress,
    /// Completed
    Completed,
    /// Failed
    Failed(String),
}

impl fmt::Display for PlanStepStatus {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            PlanStepStatus::NotStarted => write!(f, "Not Started"),
            PlanStepStatus::InProgress => write!(f, "In Progress"),
            PlanStepStatus::Completed => write!(f, "Completed"),
            PlanStepStatus::Failed(reason) => write!(f, "Failed: {}", reason),
        }
    }
}

impl PlanStep {
    /// Create a new plan step
    pub fn new(description: &str) -> Self {
        Self {
            id: uuid::Uuid::new_v4().to_string(),
            description: description.to_string(),
            dependencies: Vec::new(),
            status: PlanStepStatus::NotStarted,
            metadata: HashMap::new(),
        }
    }
    
    /// Add dependency
    pub fn add_dependency(&mut self, step_id: &str) {
        self.dependencies.push(step_id.to_string());
    }
    
    /// Set status
    pub fn set_status(&mut self, status: PlanStepStatus) {
        self.status = status;
    }
    
    /// Add metadata
    pub fn add_metadata(&mut self, key: &str, value: &str) {
        self.metadata.insert(key.to_string(), value.to_string());
    }
    
    /// Get metadata
    pub fn get_metadata(&self, key: &str) -> Option<&String> {
        self.metadata.get(key)
    }
}

/// Plan
#[derive(Debug, Clone)]
pub struct Plan {
    /// Plan ID
    pub id: String,
    /// Plan name
    pub name: String,
    /// Plan description
    pub description: String,
    /// Plan steps
    pub steps: HashMap<String, PlanStep>,
    /// Plan metadata
    pub metadata: HashMap<String, String>,
}

impl Plan {
    /// Create a new plan
    pub fn new(name: &str, description: &str) -> Self {
        Self {
            id: uuid::Uuid::new_v4().to_string(),
            name: name.to_string(),
            description: description.to_string(),
            steps: HashMap::new(),
            metadata: HashMap::new(),
        }
    }
    
    /// Add step
    pub fn add_step(&mut self, step: PlanStep) -> Result<(), PlanningError> {
        self.steps.insert(step.id.clone(), step);
        Ok(())
    }
    
    /// Get step
    pub fn get_step(&self, id: &str) -> Option<&PlanStep> {
        self.steps.get(id)
    }
    
    /// Get step mut
    pub fn get_step_mut(&mut self, id: &str) -> Option<&mut PlanStep> {
        self.steps.get_mut(id)
    }
    
    /// Get next steps
    pub fn get_next_steps(&self) -> Vec<&PlanStep> {
        let mut next_steps = Vec::new();
        
        for step in self.steps.values() {
            if step.status == PlanStepStatus::NotStarted {
                let mut can_execute = true;
                
                for dep_id in &step.dependencies {
                    if let Some(dep_step) = self.get_step(dep_id) {
                        if dep_step.status != PlanStepStatus::Completed {
                            can_execute = false;
                            break;
                        }
                    } else {
                        can_execute = false;
                        break;
                    }
                }
                
                if can_execute {
                    next_steps.push(step);
                }
            }
        }
        
        next_steps
    }
    
    /// Add metadata
    pub fn add_metadata(&mut self, key: &str, value: &str) {
        self.metadata.insert(key.to_string(), value.to_string());
    }
    
    /// Get metadata
    pub fn get_metadata(&self, key: &str) -> Option<&String> {
        self.metadata.get(key)
    }
}

/// Planning system
pub struct PlanningSystem {
    /// Plans
    pub plans: HashMap<String, Plan>,
    /// Current plan ID
    pub current_plan_id: Option<String>,
}

impl PlanningSystem {
    /// Create a new planning system
    pub fn new() -> Result<Self, PlanningError> {
        Ok(Self {
            plans: HashMap::new(),
            current_plan_id: None,
        })
    }
    
    /// Add plan
    pub fn add_plan(&mut self, plan: Plan) -> Result<(), PlanningError> {
        self.plans.insert(plan.id.clone(), plan);
        Ok(())
    }
    
    /// Get plan
    pub fn get_plan(&self, id: &str) -> Option<&Plan> {
        self.plans.get(id)
    }
    
    /// Get plan mut
    pub fn get_plan_mut(&mut self, id: &str) -> Option<&mut Plan> {
        self.plans.get_mut(id)
    }
    
    /// Set current plan
    pub fn set_current_plan(&mut self, id: &str) -> Result<(), PlanningError> {
        if self.plans.contains_key(id) {
            self.current_plan_id = Some(id.to_string());
            Ok(())
        } else {
            Err(PlanningError::ProcessingError(format!("Plan with ID {} not found", id)))
        }
    }
    
    /// Get current plan
    pub fn get_current_plan(&self) -> Option<&Plan> {
        if let Some(id) = &self.current_plan_id {
            self.get_plan(id)
        } else {
            None
        }
    }
    
    /// Get current plan mut
    pub fn get_current_plan_mut(&mut self) -> Option<&mut Plan> {
        // 修复借用冲突：先克隆ID，避免同时借用self
        let current_id = self.current_plan_id.clone();
        if let Some(id) = current_id {
            self.get_plan_mut(&id)
        } else {
            None
        }
    }
    
    /// Execute next step
    pub fn execute_next_step(&mut self) -> Result<Option<PlanStep>, PlanningError> {
        // 获取当前计划ID
        let current_id = match &self.current_plan_id {
            Some(id) => id.clone(),
            None => return Err(PlanningError::ProcessingError("No current plan set".to_string()))
        };
        
        // 获取当前计划
        let plan = match self.plans.get(&current_id) {
            Some(p) => p,
            None => return Err(PlanningError::ProcessingError("Current plan not found".to_string()))
        };
        
        // 获取下一步
        let next_steps = plan.get_next_steps();
        if next_steps.is_empty() {
            return Ok(None);
        }
        
        // 克隆第一个步骤
        let first_step = next_steps[0].clone();
        let step_id = first_step.id.clone();
        
        // 更新步骤状态
        if let Some(plan) = self.plans.get_mut(&current_id) {
            if let Some(step) = plan.steps.get_mut(&step_id) {
                step.set_status(PlanStepStatus::InProgress);
                return Ok(Some(first_step));
            }
        }
        
        Ok(None)
    }
    
    /// Complete step
    pub fn complete_step(&mut self, id: &str) -> Result<(), PlanningError> {
        // 获取当前计划ID
        let current_id = match &self.current_plan_id {
            Some(id) => id.clone(),
            None => return Err(PlanningError::ProcessingError("No current plan set".to_string()))
        };
        
        // 更新步骤状态
        if let Some(plan) = self.plans.get_mut(&current_id) {
            if let Some(step) = plan.steps.get_mut(id) {
                step.set_status(PlanStepStatus::Completed);
                return Ok(());
            }
            return Err(PlanningError::ProcessingError(format!("Step with ID {} not found", id)));
        }
        
        Err(PlanningError::ProcessingError("Current plan not found".to_string()))
    }
    
    /// Fail step
    pub fn fail_step(&mut self, id: &str, reason: &str) -> Result<(), PlanningError> {
        // 获取当前计划ID
        let current_id = match &self.current_plan_id {
            Some(id) => id.clone(),
            None => return Err(PlanningError::ProcessingError("No current plan set".to_string()))
        };
        
        // 更新步骤状态
        if let Some(plan) = self.plans.get_mut(&current_id) {
            if let Some(step) = plan.steps.get_mut(id) {
                step.set_status(PlanStepStatus::Failed(reason.to_string()));
                return Ok(());
            }
            return Err(PlanningError::ProcessingError(format!("Step with ID {} not found", id)));
        }
        
        Err(PlanningError::ProcessingError("Current plan not found".to_string()))
    }
    
    /// Is plan completed
    pub fn is_plan_completed(&self) -> Result<bool, PlanningError> {
        if let Some(plan) = self.get_current_plan() {
            for step in plan.steps.values() {
                match step.status {
                    PlanStepStatus::NotStarted | PlanStepStatus::InProgress => return Ok(false),
                    PlanStepStatus::Failed(_) => return Ok(false),
                    PlanStepStatus::Completed => {}
                }
            }
            
            Ok(true)
        } else {
            Err(PlanningError::ProcessingError("No current plan set".to_string()))
        }
    }
}

/// Initialize planning module
pub fn init() -> Result<(), PlanningError> {
    // Initialize planning module
    Ok(())
}

/// Start planning module
pub fn start() -> Result<(), PlanningError> {
    // Start planning module
    Ok(())
}

/// Stop planning module
pub fn stop() -> Result<(), PlanningError> {
    // Stop planning module
    Ok(())
}
