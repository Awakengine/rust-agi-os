#[cfg(test)]
mod tests {
    use super::*;
    use crate::kernel::process::{Process, ProcessState, ProcessError};
    use std::sync::{Arc, Mutex};

    #[test]
    fn test_process_state() {
        assert_eq!(format!("{}", ProcessState::Created), "created");
        assert_eq!(format!("{}", ProcessState::Ready), "ready");
        assert_eq!(format!("{}", ProcessState::Running), "running");
        assert_eq!(format!("{}", ProcessState::Waiting), "waiting");
        assert_eq!(format!("{}", ProcessState::Terminated), "terminated");
    }

    #[test]
    fn test_process_error() {
        let creation_error = ProcessError::CreationError("Failed to create process".to_string());
        let execution_error = ProcessError::ExecutionError("Failed to execute process".to_string());
        let termination_error = ProcessError::TerminationError("Failed to terminate process".to_string());
        let other_error = ProcessError::Other("Unknown error".to_string());
        
        assert_eq!(
            format!("{}", creation_error),
            "Creation error: Failed to create process"
        );
        assert_eq!(
            format!("{}", execution_error),
            "Execution error: Failed to execute process"
        );
        assert_eq!(
            format!("{}", termination_error),
            "Termination error: Failed to terminate process"
        );
        assert_eq!(
            format!("{}", other_error),
            "Other error: Unknown error"
        );
    }

    // Mock process for testing
    #[derive(Debug)]
    struct MockProcess {
        id: String,
        name: String,
        state: ProcessState,
    }

    impl MockProcess {
        fn new(id: &str, name: &str) -> Self {
            Self {
                id: id.to_string(),
                name: name.to_string(),
                state: ProcessState::Created,
            }
        }
    }

    impl Process for MockProcess {
        fn id(&self) -> &str {
            &self.id
        }
        
        fn name(&self) -> &str {
            &self.name
        }
        
        fn state(&self) -> ProcessState {
            self.state
        }
        
        fn start(&mut self) -> Result<(), ProcessError> {
            self.state = ProcessState::Running;
            Ok(())
        }
        
        fn pause(&mut self) -> Result<(), ProcessError> {
            self.state = ProcessState::Waiting;
            Ok(())
        }
        
        fn resume(&mut self) -> Result<(), ProcessError> {
            self.state = ProcessState::Running;
            Ok(())
        }
        
        fn terminate(&mut self) -> Result<(), ProcessError> {
            self.state = ProcessState::Terminated;
            Ok(())
        }
    }

    #[test]
    fn test_process_lifecycle() {
        let mut process = MockProcess::new("proc-1", "test-process");
        
        // Test initial state
        assert_eq!(process.id(), "proc-1");
        assert_eq!(process.name(), "test-process");
        assert_eq!(process.state(), ProcessState::Created);
        
        // Test start
        assert!(process.start().is_ok());
        assert_eq!(process.state(), ProcessState::Running);
        
        // Test pause
        assert!(process.pause().is_ok());
        assert_eq!(process.state(), ProcessState::Waiting);
        
        // Test resume
        assert!(process.resume().is_ok());
        assert_eq!(process.state(), ProcessState::Running);
        
        // Test terminate
        assert!(process.terminate().is_ok());
        assert_eq!(process.state(), ProcessState::Terminated);
    }

    #[test]
    fn test_module_functions() {
        // Test initialization functions
        assert!(init().is_ok());
        assert!(start().is_ok());
        assert!(stop().is_ok());
    }
}
