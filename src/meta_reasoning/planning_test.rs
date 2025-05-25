#[cfg(test)]
mod tests {
    use super::*;
    use crate::meta_reasoning::planning::{PlanningSystem, PlanningStatus, PlanningError};
    use std::time::Duration;

    #[test]
    fn test_planning_status() {
        assert_eq!(format!("{}", PlanningStatus::NotStarted), "not_started");
        assert_eq!(format!("{}", PlanningStatus::InProgress), "in_progress");
        assert_eq!(format!("{}", PlanningStatus::Completed), "completed");
        assert_eq!(format!("{}", PlanningStatus::Failed), "failed");
    }

    #[test]
    fn test_planning_error() {
        let goal_error = PlanningError::GoalError("Invalid goal".to_string());
        let constraint_error = PlanningError::ConstraintError("Constraint violation".to_string());
        let timeout_error = PlanningError::TimeoutError("Planning timed out".to_string());
        let other_error = PlanningError::Other("Unknown error".to_string());
        
        assert_eq!(
            format!("{}", goal_error),
            "Goal error: Invalid goal"
        );
        assert_eq!(
            format!("{}", constraint_error),
            "Constraint error: Constraint violation"
        );
        assert_eq!(
            format!("{}", timeout_error),
            "Timeout error: Planning timed out"
        );
        assert_eq!(
            format!("{}", other_error),
            "Other error: Unknown error"
        );
    }

    #[test]
    fn test_planning_system() {
        let mut planner = PlanningSystem::new("test_planner");
        
        // Test initial state
        assert_eq!(planner.name, "test_planner");
        assert_eq!(planner.status, PlanningStatus::NotStarted);
        assert_eq!(planner.goals.len(), 0);
        assert_eq!(planner.constraints.len(), 0);
        assert_eq!(planner.steps.len(), 0);
        
        // Test add_goal
        planner.add_goal("reach_destination");
        planner.add_goal("minimize_cost");
        
        assert_eq!(planner.goals.len(), 2);
        
        // Test add_constraint
        planner.add_constraint("time_limit", "24h");
        planner.add_constraint("max_cost", "1000");
        
        assert_eq!(planner.constraints.len(), 2);
        
        // Test plan generation
        assert!(planner.generate_plan(Duration::from_secs(10)).is_ok());
        assert_eq!(planner.status, PlanningStatus::Completed);
        assert!(planner.steps.len() > 0);
        
        // Test plan execution
        assert!(planner.execute_step(0).is_ok());
        
        // Test plan reset
        planner.reset();
        assert_eq!(planner.status, PlanningStatus::NotStarted);
        assert_eq!(planner.steps.len(), 0);
    }

    #[test]
    fn test_module_functions() {
        // Test initialization functions
        assert!(init().is_ok());
        assert!(start().is_ok());
        assert!(stop().is_ok());
    }
}
