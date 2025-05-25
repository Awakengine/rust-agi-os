#[cfg(test)]
mod tests {
    use super::*;
    use crate::system::config::{SystemConfig, ConfigError};
    use std::collections::HashMap;

    #[test]
    fn test_system_config_creation() {
        let config = SystemConfig::new("test_system", "1.0.0", "Test system configuration");
        
        assert_eq!(config.name, "test_system");
        assert_eq!(config.version, "1.0.0");
        assert_eq!(config.description, "Test system configuration");
        assert_eq!(config.properties.len(), 0);
    }

    #[test]
    fn test_system_config_properties() {
        let mut config = SystemConfig::new("test_system", "1.0.0", "Test system configuration");
        
        // Test setting properties
        config.set_property("max_memory", "1024MB");
        config.set_property("max_cpu", "4");
        
        // Test getting properties
        assert_eq!(config.get_property("max_memory").unwrap(), "1024MB");
        assert_eq!(config.get_property("max_cpu").unwrap(), "4");
        assert_eq!(config.get_property("non_existent"), None);
        
        // Test overwriting property
        config.set_property("max_memory", "2048MB");
        assert_eq!(config.get_property("max_memory").unwrap(), "2048MB");
    }

    #[test]
    fn test_system_config_default() {
        let config = SystemConfig::default();
        
        assert_eq!(config.name, "default");
        assert_eq!(config.version, "0.1.0");
        assert_eq!(config.description, "Default system config");
    }

    #[test]
    fn test_config_error() {
        let parse_error = ConfigError::ParseError("Invalid JSON".to_string());
        let validation_error = ConfigError::ValidationError("Missing required field".to_string());
        let io_error = ConfigError::IoError("Failed to read file".to_string());
        let other_error = ConfigError::Other("Unknown error".to_string());
        
        assert_eq!(
            format!("{}", parse_error),
            "Parse error: Invalid JSON"
        );
        assert_eq!(
            format!("{}", validation_error),
            "Validation error: Missing required field"
        );
        assert_eq!(
            format!("{}", io_error),
            "IO error: Failed to read file"
        );
        assert_eq!(
            format!("{}", other_error),
            "Other error: Unknown error"
        );
    }

    #[test]
    fn test_load_save_operations() {
        // Test load_from_file
        let config_result = SystemConfig::load_from_file("/non/existent/path");
        assert!(config_result.is_ok()); // In our simplified implementation, this always succeeds
        
        // Test save_to_file
        let config = SystemConfig::default();
        let save_result = config.save_to_file("/tmp/config.json");
        assert!(save_result.is_ok());
    }

    #[test]
    fn test_json_operations() {
        // Test from_json
        let config_result = SystemConfig::from_json("{}");
        assert!(config_result.is_ok());
        
        // Test to_json
        let config = SystemConfig::default();
        let json_result = config.to_json();
        assert!(json_result.is_ok());
    }

    #[test]
    fn test_module_functions() {
        // Test initialization functions
        assert!(init().is_ok());
        assert!(start().is_ok());
        assert!(stop().is_ok());
    }
}
