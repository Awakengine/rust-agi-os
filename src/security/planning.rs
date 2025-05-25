//! # Planning Module
//! 
//! This module provides meta-planning capabilities for the AGI operating system,
//! enabling higher-order planning and reasoning about plans.

use std::sync::{Arc, Mutex, Once};
use std::collections::{HashMap, HashSet};
use std::time::Instant;
use std::fmt;
use std::error::Error;

static INIT: Once = Once::new();

/// Initialize the planning subsystem
pub fn init() -> Result<(), PlanningError> {
    let result = Ok(());
    
    INIT.call_once(|| {
        // Initialize planning components
        // In a real implementation, this would initialize planners,
        // plan evaluators, etc.
    });
    
    result
}

/// Error type for planning operations
#[derive(Debug)]
pub enum PlanningError {
    /// Plan generation error
    PlanGenerationError(String),
    /// Plan execution error
    PlanExecutionError(String),
    /// Resource allocation error
    ResourceAllocationError(String),
    /// General error
    General(&'static str),
}

// 实现Display trait，解决E0277错误
impl fmt::Display for PlanningError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            PlanningError::PlanGenerationError(msg) => write!(f, "Plan generation error: {}", msg),
            PlanningError::PlanExecutionError(msg) => write!(f, "Plan execution error: {}", msg),
            PlanningError::ResourceAllocationError(msg) => write!(f, "Resource allocation error: {}", msg),
            PlanningError::General(msg) => write!(f, "General planning error: {}", msg),
        }
    }
}

// 实现Error trait，解决?操作符错误转换问题
impl Error for PlanningError {}

/// Planning configuration
#[derive(Debug, Clone)]
pub struct PlanningConfig {
    /// Enable hierarchical planning
    pub enable_hierarchical_planning: bool,
    /// Enable contingency planning
    pub enable_contingency_planning: bool,
    /// Enable multi-agent planning
    pub enable_multi_agent_planning: bool,
    /// Enable plan optimization
    pub enable_plan_optimization: bool,
    /// Maximum planning depth
    pub max_planning_depth: usize,
}

impl Default for PlanningConfig {
    fn default() -> Self {
        Self {
            enable_hierarchical_planning: true,
            enable_contingency_planning: true,
            enable_multi_agent_planning: true,
            enable_plan_optimization: true,
            max_planning_depth: 10,
        }
    }
}

/// Planning status
#[derive(Debug, Clone)]
pub struct PlanningStatus {
    /// Is hierarchical planning enabled
    pub hierarchical_planning_enabled: bool,
    /// Is contingency planning enabled
    pub contingency_planning_enabled: bool,
    /// Is multi-agent planning enabled
    pub multi_agent_planning_enabled: bool,
    /// Is plan optimization enabled
    pub plan_optimization_enabled: bool,
    /// Number of active plans
    pub active_plans_count: usize,
    /// Memory usage (bytes)
    pub memory_usage: usize,
}

/// Plan
#[derive(Debug, Clone)]
pub struct Plan {
    /// Plan ID
    pub id: String,
    /// Plan name
    pub name: String,
    /// Plan description
    pub description: Option<String>,
    /// Goal
    pub goal: String,
    /// Initial state
    pub initial_state: HashMap<String, String>,
    /// Steps
    pub steps: Vec<PlanStep>,
    /// Dependencies
    pub dependencies: HashMap<String, HashSet<String>>, // Step ID -> Set of step IDs it depends on
    /// Creation time
    pub creation_time: Instant,
    /// Last modified time
    pub last_modified_time: Instant,
    /// Status
    pub status: PlanStatus,
}

/// Plan step
#[derive(Debug, Clone)]
pub struct PlanStep {
    /// Step ID
    pub id: String,
    /// Step description
    pub description: String,
    /// Action
    pub action: PlanAction,
    /// Preconditions
    pub preconditions: HashMap<String, String>,
    /// Effects
    pub effects: HashMap<String, String>,
    /// Estimated duration (seconds)
    pub estimated_duration: f64,
    /// Status
    pub status: PlanStepStatus,
    /// Start time
    pub start_time: Option<Instant>,
    /// End time
    pub end_time: Option<Instant>,
}

/// Plan action
#[derive(Debug, Clone)]
pub struct PlanAction {
    /// Action type
    pub action_type: String,
    /// Parameters
    pub parameters: HashMap<String, String>,
    /// Sub-plan ID (for hierarchical planning)
    pub sub_plan_id: Option<String>,
}

/// Plan status
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum PlanStatus {
    /// Created
    Created,
    /// Ready
    Ready,
    /// In progress
    InProgress,
    /// Completed
    Completed,
    /// Failed
    Failed,
    /// Cancelled
    Cancelled,
}

/// Plan step status
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum PlanStepStatus {
    /// Pending
    Pending,
    /// Ready
    Ready,
    /// In progress
    InProgress,
    /// Completed
    Completed,
    /// Failed
    Failed,
    /// Skipped
    Skipped,
}

/// Meta-planner
#[derive(Debug)]
pub struct MetaPlanner {
    /// Planner ID
    id: String,
    /// Plans
    plans: HashMap<String, Plan>,
    /// Configuration
    config: PlanningConfig,
}

impl MetaPlanner {
    /// Create a new meta-planner
    pub fn new(id: &str, config: PlanningConfig) -> Self {
        Self {
            id: id.to_string(),
            plans: HashMap::new(),
            config,
        }
    }
    
    /// Create a new plan
    pub fn create_plan(&mut self, name: &str, description: Option<&str>, goal: &str, initial_state: HashMap<String, String>) 
        -> Result<String, PlanningError> {
        let plan_id = format!("plan_{}", self.plans.len());
        
        let plan = Plan {
            id: plan_id.clone(),
            name: name.to_string(),
            description: description.map(|s| s.to_string()),
            goal: goal.to_string(),
            initial_state,
            steps: Vec::new(),
            dependencies: HashMap::new(),
            creation_time: Instant::now(),
            last_modified_time: Instant::now(),
            status: PlanStatus::Created,
        };
        
        self.plans.insert(plan_id.clone(), plan);
        
        Ok(plan_id)
    }
    
    /// Add step to plan
    pub fn add_step(&mut self, plan_id: &str, description: &str, action: PlanAction, 
                   preconditions: HashMap<String, String>, effects: HashMap<String, String>,
                   estimated_duration: f64, dependencies: HashSet<String>) 
        -> Result<String, PlanningError> {
        let plan = self.plans.get_mut(plan_id)
            .ok_or_else(|| PlanningError::General("Plan not found"))?;
        
        let step_id = format!("step_{}", plan.steps.len());
        
        let step = PlanStep {
            id: step_id.clone(),
            description: description.to_string(),
            action,
            preconditions,
            effects,
            estimated_duration,
            status: PlanStepStatus::Pending,
            start_time: None,
            end_time: None,
        };
        
        plan.steps.push(step);
        plan.dependencies.insert(step_id.clone(), dependencies);
        plan.last_modified_time = Instant::now();
        
        Ok(step_id)
    }
    
    /// Generate plan
    pub fn generate_plan(&mut self, plan_id: &str) -> Result<(), PlanningError> {
        let plan = self.plans.get_mut(plan_id)
            .ok_or_else(|| PlanningError::General("Plan not found"))?;
        
        if plan.status != PlanStatus::Created {
            return Err(PlanningError::General("Plan is not in Created status"));
        }
        
        // In a real implementation, this would generate a plan based on the goal and initial state
        // For this prototype, we just set the status to Ready
        
        plan.status = PlanStatus::Ready;
        plan.last_modified_time = Instant::now();
        
        Ok(())
    }
    
    /// Start plan execution
    pub fn start_plan(&mut self, plan_id: &str) -> Result<(), PlanningError> {
        let plan = self.plans.get_mut(plan_id)
            .ok_or_else(|| PlanningError::General("Plan not found"))?;
        
        if plan.status != PlanStatus::Ready {
            return Err(PlanningError::General("Plan is not in Ready status"));
        }
        
        // Update plan status
        plan.status = PlanStatus::InProgress;
        plan.last_modified_time = Instant::now();
        
        // Update step statuses
        for step in &mut plan.steps {
            let dependencies = plan.dependencies.get(&step.id).cloned().unwrap_or_default();
            
            if dependencies.is_empty() {
                step.status = PlanStepStatus::Ready;
            }
        }
        
        Ok(())
    }
    
    /// Execute step
    pub fn execute_step(&mut self, plan_id: &str, step_id: &str) -> Result<(), PlanningError> {
        let plan = self.plans.get_mut(plan_id)
            .ok_or_else(|| PlanningError::General("Plan not found"))?;
        
        if plan.status != PlanStatus::InProgress {
            return Err(PlanningError::General("Plan is not in InProgress status"));
        }
        
        // Find step
        let step_index = plan.steps.iter().position(|s| s.id == step_id)
            .ok_or_else(|| PlanningError::General("Step not found"))?;
        
        let step = &mut plan.steps[step_index];
        
        if step.status != PlanStepStatus::Ready {
            return Err(PlanningError::General("Step is not in Ready status"));
        }
        
        // Execute step
        step.status = PlanStepStatus::InProgress;
        step.start_time = Some(Instant::now());
        
        // In a real implementation, this would execute the action
        // For this prototype, we just simulate execution
        
        // Complete step
        step.status = PlanStepStatus::Completed;
        step.end_time = Some(Instant::now());
        
        // Update dependent steps
        for (other_step_id, dependencies) in &mut plan.dependencies {
            if dependencies.contains(step_id) {
                dependencies.remove(step_id);
                
                if dependencies.is_empty() {
                    if let Some(other_step) = plan.steps.iter_mut().find(|s| s.id == *other_step_id) {
                        if other_step.status == PlanStepStatus::Pending {
                            other_step.status = PlanStepStatus::Ready;
                        }
                    }
                }
            }
        }
        
        // Check if plan is completed
        if plan.steps.iter().all(|s| s.status == PlanStepStatus::Completed || s.status == PlanStepStatus::Skipped) {
            plan.status = PlanStatus::Completed;
        }
        
        plan.last_modified_time = Instant::now();
        
        Ok(())
    }
    
    /// Get plan
    pub fn get_plan(&self, plan_id: &str) -> Option<&Plan> {
        self.plans.get(plan_id)
    }
    
    /// Get planner ID
    pub fn id(&self) -> &str {
        &self.id
    }
    
    /// Get planner status
    pub fn status(&self) -> PlanningStatus {
        PlanningStatus {
            hierarchical_planning_enabled: self.config.enable_hierarchical_planning,
            contingency_planning_enabled: self.config.enable_contingency_planning,
            multi_agent_planning_enabled: self.config.enable_multi_agent_planning,
            plan_optimization_enabled: self.config.enable_plan_optimization,
            active_plans_count: self.plans.values()
                .filter(|plan| plan.status == PlanStatus::InProgress)
                .count(),
            memory_usage: 0, // In a real implementation, this would be calculated
        }
    }
}

/// Set planning configuration
pub fn set_config(_config: PlanningConfig) -> Result<(), PlanningError> {
    // In a real implementation, this would update a global planning manager
    // For this prototype, we just return Ok
    Ok(())
}

/// Get planning status
pub fn get_status() -> Result<PlanningStatus, PlanningError> {
    // In a real implementation, this would get status from a global planning manager
    // For this prototype, we just return a dummy status
    Ok(PlanningStatus {
        hierarchical_planning_enabled: true,
        contingency_planning_enabled: true,
        multi_agent_planning_enabled: true,
        plan_optimization_enabled: true,
        active_plans_count: 0,
        memory_usage: 0,
    })
}

#[cfg(test)]
mod tests {
    use super::*;
    
    #[test]
    fn test_meta_planner() {
        let config = PlanningConfig::default();
        let mut planner = MetaPlanner::new("test", config);
        
        // Create plan
        let mut initial_state = HashMap::new();
        initial_state.insert("location".to_string(), "A".to_string());
        
        let plan_id = planner.create_plan(
            "Test Plan",
            Some("Test description"),
            "Move from A to C",
            initial_state
        ).unwrap();
        
        // Add steps
        let action1 = PlanAction {
            action_type: "move".to_string(),
            parameters: {
                let mut params = HashMap::new();
                params.insert("from".to_string(), "A".to_string());
                params.insert("to".to_string(), "B".to_string());
                params
            },
            sub_plan_id: None,
        };
        
        let mut preconditions1 = HashMap::new();
        preconditions1.insert("location".to_string(), "A".to_string());
        
        let mut effects1 = HashMap::new();
        effects1.insert("location".to_string(), "B".to_string());
        
        let step1_id = planner.add_step(
            &plan_id,
            "Move from A to B",
            action1,
            preconditions1,
            effects1,
            10.0,
            HashSet::new()
        ).unwrap();
        
        let action2 = PlanAction {
            action_type: "move".to_string(),
            parameters: {
                let mut params = HashMap::new();
                params.insert("from".to_string(), "B".to_string());
                params.insert("to".to_string(), "C".to_string());
                params
            },
            sub_plan_id: None,
        };
        
        let mut preconditions2 = HashMap::new();
        preconditions2.insert("location".to_string(), "B".to_string());
        
        let mut effects2 = HashMap::new();
        effects2.insert("location".to_string(), "C".to_string());
        
        let mut dependencies2 = HashSet::new();
        dependencies2.insert(step1_id.clone());
        
        let step2_id = planner.add_step(
            &plan_id,
            "Move from B to C",
            action2,
            preconditions2,
            effects2,
            15.0,
            dependencies2
        ).unwrap();
        
        // Generate plan
        assert!(planner.generate_plan(&plan_id).is_ok());
        
        // Get plan
        let plan = planner.get_plan(&plan_id).unwrap();
        assert_eq!(plan.status, PlanStatus::Ready);
        assert_eq!(plan.steps.len(), 2);
        
        // Start plan
        assert!(planner.start_plan(&plan_id).is_ok());
        
        let plan = planner.get_plan(&plan_id).unwrap();
        assert_eq!(plan.status, PlanStatus::InProgress);
        assert_eq!(plan.steps[0].status, PlanStepStatus::Ready);
        assert_eq!(plan.steps[1].status, PlanStepStatus::Pending);
        
        // Execute first step
        assert!(planner.execute_step(&plan_id, &step1_id).is_ok());
        
        let plan = planner.get_plan(&plan_id).unwrap();
        assert_eq!(plan.steps[0].status, PlanStepStatus::Completed);
        assert_eq!(plan.steps[1].status, PlanStepStatus::Ready);
        
        // Execute second step
        assert!(planner.execute_step(&plan_id, &step2_id).is_ok());
        
        let plan = planner.get_plan(&plan_id).unwrap();
        assert_eq!(plan.steps[1].status, PlanStepStatus::Completed);
        assert_eq!(plan.status, PlanStatus::Completed);
   
(Content truncated due to size limit. Use line ranges to read in chunks)