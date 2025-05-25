#[cfg(test)]
mod tests {
    use super::*;
    use crate::meta_reasoning::reasoning::{ReasoningConfig, ReasoningResult};
    use std::collections::HashMap;
    use std::time::Duration;

    #[test]
    fn test_reasoning_config() {
        let config = ReasoningConfig::new(
            100,
            Duration::from_secs(30),
            0.8
        );
        
        assert_eq!(config.max_iterations, 100);
        assert_eq!(config.timeout, Duration::from_secs(30));
        assert_eq!(config.confidence_threshold, 0.8);
    }

    #[test]
    fn test_reasoning_config_default() {
        let config = ReasoningConfig::default();
        
        assert_eq!(config.max_iterations, 100);
        assert_eq!(config.timeout, Duration::from_secs(30));
        assert_eq!(config.confidence_threshold, 0.8);
    }

    #[test]
    fn test_reasoning_result() {
        let mut results = HashMap::new();
        results.insert("conclusion".to_string(), "valid".to_string());
        results.insert("confidence".to_string(), "high".to_string());
        
        let result = ReasoningResult::new(
            true,
            0.95,
            50,
            Duration::from_millis(500),
            results
        );
        
        assert!(result.success);
        assert_eq!(result.confidence, 0.95);
        assert_eq!(result.iterations, 50);
        assert_eq!(result.duration, Duration::from_millis(500));
        assert_eq!(result.results.len(), 2);
        assert_eq!(result.results.get("conclusion").unwrap(), "valid");
        assert_eq!(result.results.get("confidence").unwrap(), "high");
    }

    #[test]
    fn test_module_functions() {
        // Test initialization functions
        assert!(init().is_ok());
        assert!(start().is_ok());
        assert!(stop().is_ok());
    }
}
