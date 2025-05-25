#[cfg(test)]
mod tests {
    use super::*;
    use crate::security::sandbox::{Sandbox, MemoryRegion, ProtectionFlags, SandboxError};

    #[test]
    fn test_protection_flags() {
        let read_only = ProtectionFlags::read_only();
        assert!(read_only.read);
        assert!(!read_only.write);
        assert!(!read_only.execute);

        let read_write = ProtectionFlags::read_write();
        assert!(read_write.read);
        assert!(read_write.write);
        assert!(!read_write.execute);

        let read_execute = ProtectionFlags::read_execute();
        assert!(read_execute.read);
        assert!(!read_execute.write);
        assert!(read_execute.execute);

        let no_access = ProtectionFlags::no_access();
        assert!(!no_access.read);
        assert!(!no_access.write);
        assert!(!no_access.execute);

        let custom = ProtectionFlags::new(true, true, true);
        assert!(custom.read);
        assert!(custom.write);
        assert!(custom.execute);
    }

    #[test]
    fn test_memory_region() {
        let region = MemoryRegion::new(
            0x1000,
            0x1000,
            ProtectionFlags::read_write(),
            "test_region"
        );

        assert_eq!(region.base, 0x1000);
        assert_eq!(region.size, 0x1000);
        assert!(region.protection.read);
        assert!(region.protection.write);
        assert!(!region.protection.execute);
        assert_eq!(region.name, "test_region");

        // Test contains
        assert!(region.contains(0x1000));
        assert!(region.contains(0x1FFF));
        assert!(!region.contains(0x0FFF));
        assert!(!region.contains(0x2000));

        // Test permissions
        assert!(region.is_readable());
        assert!(region.is_writable());
        assert!(!region.is_executable());
    }

    #[test]
    fn test_sandbox() {
        let mut sandbox = Sandbox::new("test_sandbox", "Test Sandbox");
        
        assert_eq!(sandbox.id, "test_sandbox");
        assert_eq!(sandbox.name, "Test Sandbox");
        assert_eq!(sandbox.memory_regions.len(), 0);
        assert_eq!(sandbox.resource_limits.len(), 0);

        // Add memory region
        let region = MemoryRegion::new(
            0x1000,
            0x1000,
            ProtectionFlags::read_write(),
            "test_region"
        );
        sandbox.add_memory_region(region);
        assert_eq!(sandbox.memory_regions.len(), 1);

        // Test memory access checks
        assert!(sandbox.check_memory_access(0x1500, false, false).is_ok()); // Read access
        assert!(sandbox.check_memory_access(0x1500, true, false).is_ok());  // Write access
        assert!(sandbox.check_memory_access(0x1500, false, true).is_err()); // Execute access (should fail)
        assert!(sandbox.check_memory_access(0x2500, false, false).is_err()); // Outside region (should fail)
    }

    #[test]
    fn test_sandbox_error() {
        let init_error = SandboxError::InitializationError("Failed to initialize".to_string());
        let exec_error = SandboxError::ExecutionError("Failed to execute".to_string());
        let resource_error = SandboxError::ResourceError("Resource limit exceeded".to_string());
        let permission_error = SandboxError::PermissionError("Access denied".to_string());
        let other_error = SandboxError::Other("Unknown error".to_string());

        assert_eq!(
            format!("{}", init_error),
            "Initialization error: Failed to initialize"
        );
        assert_eq!(
            format!("{}", exec_error),
            "Execution error: Failed to execute"
        );
        assert_eq!(
            format!("{}", resource_error),
            "Resource error: Resource limit exceeded"
        );
        assert_eq!(
            format!("{}", permission_error),
            "Permission error: Access denied"
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
