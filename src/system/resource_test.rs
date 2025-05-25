#[cfg(test)]
mod tests {
    use super::*;
    use crate::system::resource::{ResourceManager, ResourceLimit, ResourceType, ResourceError};

    #[test]
    fn test_resource_type() {
        assert_eq!(format!("{}", ResourceType::Cpu), "cpu");
        assert_eq!(format!("{}", ResourceType::Memory), "memory");
        assert_eq!(format!("{}", ResourceType::Disk), "disk");
        assert_eq!(format!("{}", ResourceType::Network), "network");
    }

    #[test]
    fn test_resource_limit() {
        let limit = ResourceLimit::new(ResourceType::Memory, 1024, "MB");
        
        assert_eq!(limit.resource_type, ResourceType::Memory);
        assert_eq!(limit.hard_limit, 1024);
        assert_eq!(limit.limit_unit, "MB");
    }

    #[test]
    fn test_resource_manager() {
        let mut manager = ResourceManager::new();
        
        // Test initial state
        assert_eq!(manager.resource_limits.len(), 0);
        assert_eq!(manager.resource_usage.len(), 0);
        
        // Add resource limits
        let memory_limit = ResourceLimit::new(ResourceType::Memory, 1024, "MB");
        let cpu_limit = ResourceLimit::new(ResourceType::Cpu, 4, "cores");
        
        manager.add_resource_limit(memory_limit);
        manager.add_resource_limit(cpu_limit);
        
        assert_eq!(manager.resource_limits.len(), 2);
        
        // Test get_resource_limit
        let retrieved_limit = manager.get_resource_limit(ResourceType::Memory).unwrap();
        assert_eq!(retrieved_limit.hard_limit, 1024);
        assert_eq!(retrieved_limit.limit_unit, "MB");
        
        // Test set_resource_usage and get_resource_usage
        manager.set_resource_usage(ResourceType::Memory, 512);
        assert_eq!(*manager.get_resource_usage(ResourceType::Memory).unwrap(), 512);
        
        // Test check_resource_limits (within limits)
        assert!(manager.check_resource_limits().is_ok());
        
        // Test check_resource_limits (exceeding limits)
        manager.set_resource_usage(ResourceType::Memory, 2048);
        assert!(manager.check_resource_limits().is_err());
        
        // Test apply_resource_limits
        assert!(manager.apply_resource_limits().is_ok());
    }

    #[test]
    fn test_resource_error() {
        let allocation_error = ResourceError::AllocationError("Failed to allocate memory".to_string());
        let limit_error = ResourceError::LimitError("Resource limit exceeded".to_string());
        let io_error = ResourceError::IoError("Failed to read resource file".to_string());
        let other_error = ResourceError::Other("Unknown error".to_string());
        
        assert_eq!(
            format!("{}", allocation_error),
            "Allocation error: Failed to allocate memory"
        );
        assert_eq!(
            format!("{}", limit_error),
            "Limit error: Resource limit exceeded"
        );
        assert_eq!(
            format!("{}", io_error),
            "IO error: Failed to read resource file"
        );
        assert_eq!(
            format!("{}", other_error),
            "Other error: Unknown error"
        );
    }

    #[test]
    fn test_module_functions() {
        // Test initialization functions
        assert!(init().is_ok());
        assert!(start().is_ok());
        assert!(stop().is_ok());
    }
}
