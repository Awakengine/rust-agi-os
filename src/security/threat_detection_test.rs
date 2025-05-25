#[cfg(test)]
mod tests {
    use super::*;
    use crate::security::threat_detection::{ThreatDetectionError};

    #[test]
    fn test_threat_detection_error() {
        let detection_error = ThreatDetectionError::DetectionError("Failed to detect threats".to_string());
        let analysis_error = ThreatDetectionError::AnalysisError("Failed to analyze threat data".to_string());
        let rule_error = ThreatDetectionError::RuleError("Invalid rule format".to_string());
        let other_error = ThreatDetectionError::Other("Unknown error".to_string());
        
        assert_eq!(
            format!("{}", detection_error),
            "Detection error: Failed to detect threats"
        );
        assert_eq!(
            format!("{}", analysis_error),
            "Analysis error: Failed to analyze threat data"
        );
        assert_eq!(
            format!("{}", rule_error),
            "Rule error: Invalid rule format"
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
