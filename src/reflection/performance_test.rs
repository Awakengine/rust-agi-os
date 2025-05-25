#[cfg(test)]
mod tests {
    use super::*;
    use crate::reflection::performance::{PerformanceMetric, PerformanceMonitor, PerformanceError};
    use std::time::Duration;

    #[test]
    fn test_performance_metric() {
        let metric = PerformanceMetric::new("response_time", 150.0, "ms");
        
        assert_eq!(metric.name, "response_time");
        assert_eq!(metric.value, 150.0);
        assert_eq!(metric.unit, "ms");
    }

    #[test]
    fn test_performance_monitor() {
        let mut monitor = PerformanceMonitor::new("test_monitor");
        
        // Test initial state
        assert_eq!(monitor.name, "test_monitor");
        assert_eq!(monitor.metrics.len(), 0);
        
        // Test add_metric
        monitor.add_metric("cpu_usage", 25.0, "%");
        monitor.add_metric("memory_usage", 512.0, "MB");
        
        assert_eq!(monitor.metrics.len(), 2);
        
        // Test get_metric
        let cpu_metric = monitor.get_metric("cpu_usage").unwrap();
        assert_eq!(cpu_metric.value, 25.0);
        assert_eq!(cpu_metric.unit, "%");
        
        // Test update_metric
        monitor.update_metric("cpu_usage", 30.0).unwrap();
        let updated_cpu_metric = monitor.get_metric("cpu_usage").unwrap();
        assert_eq!(updated_cpu_metric.value, 30.0);
        
        // Test get_metric for non-existent metric
        assert!(monitor.get_metric("non_existent").is_none());
        
        // Test update_metric for non-existent metric
        assert!(monitor.update_metric("non_existent", 10.0).is_err());
    }

    #[test]
    fn test_performance_error() {
        let metric_error = PerformanceError::MetricError("Invalid metric".to_string());
        let monitor_error = PerformanceError::MonitorError("Monitor failed".to_string());
        let threshold_error = PerformanceError::ThresholdError("Threshold exceeded".to_string());
        let other_error = PerformanceError::Other("Unknown error".to_string());
        
        assert_eq!(
            format!("{}", metric_error),
            "Metric error: Invalid metric"
        );
        assert_eq!(
            format!("{}", monitor_error),
            "Monitor error: Monitor failed"
        );
        assert_eq!(
            format!("{}", threshold_error),
            "Threshold error: Threshold exceeded"
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
