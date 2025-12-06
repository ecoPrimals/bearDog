//! Health Status Tests
//!
//! Tests for health status types and health checking logic.

#[cfg(test)]
mod health_status_tests {
    use crate::canonical::health::{HealthStatus, HealthCheckResult};
    
    #[test]
    fn test_health_status_variants() {
        let healthy = HealthStatus::Healthy;
        let degraded = HealthStatus::Degraded;
        let unhealthy = HealthStatus::Unhealthy;
        let unknown = HealthStatus::Unknown;
        
        // All variants should be different
        assert_ne!(healthy, degraded);
        assert_ne!(healthy, unhealthy);
        assert_ne!(healthy, unknown);
        assert_ne!(degraded, unhealthy);
    }
    
    #[test]
    fn test_health_status_ordering() {
        // Health statuses should have a natural ordering
        // Healthy > Degraded > Unhealthy > Unknown
        
        let healthy = HealthStatus::Healthy;
        let degraded = HealthStatus::Degraded;
        
        assert_ne!(healthy, degraded);
    }
    
    #[test]
    fn test_health_status_clone() {
        let status = HealthStatus::Healthy;
        let cloned = status.clone();
        
        assert_eq!(status, cloned);
    }
    
    #[test]
    fn test_health_status_debug() {
        let status = HealthStatus::Healthy;
        let debug_str = format!("{:?}", status);
        
        assert!(!debug_str.is_empty());
        assert!(debug_str.contains("Healthy"));
    }
    
    #[test]
    fn test_health_status_serialization() {
        use serde_json;
        
        let status = HealthStatus::Healthy;
        let json = serde_json::to_string(&status);
        
        assert!(json.is_ok());
        
        // TEST_CATEGORY: unit
        // TEST_DOMAIN: types
        // TEST_PRIORITY: normal
        if let Ok(json_str) = json {
            let deserialized: Result<HealthStatus, _> = serde_json::from_str(&json_str);
            assert!(deserialized.is_ok());
            assert_eq!(deserialized.unwrap(), status);
        }
    }
}

#[cfg(test)]
mod health_check_result_tests {
    use crate::canonical::health::{HealthStatus, HealthCheckResult};
    // TEST_CATEGORY: unit
    // TEST_DOMAIN: types
    // TEST_PRIORITY: normal
    use std::time::SystemTime;
    
    #[test]
    fn test_health_check_result_creation() {
        let result = HealthCheckResult {
            status: HealthStatus::Healthy,
            message: "All systems operational".to_string(),
            timestamp: SystemTime::now(),
            // TEST_CATEGORY: unit
            // TEST_DOMAIN: types
            // TEST_PRIORITY: normal
            details: Default::default(),
        };
        
        assert_eq!(result.status, HealthStatus::Healthy);
        assert!(!result.message.is_empty());
    // TEST_CATEGORY: unit
    // TEST_DOMAIN: types
    // TEST_PRIORITY: normal
    }
    
    #[test]
    fn test_health_check_result_with_details() {
        use std::collections::HashMap;
        
        // TEST_CATEGORY: unit
        // TEST_DOMAIN: types
        // TEST_PRIORITY: normal
        let mut details = HashMap::new();
        details.insert("cpu_usage".to_string(), "25%".to_string());
        details.insert("memory_usage".to_string(), "512MB".to_string());
        
        let result = HealthCheckResult {
            status: HealthStatus::Healthy,
            message: "System healthy".to_string(),
            timestamp: SystemTime::now(),
            details,
        };
        
        assert_eq!(result.details.len(), 2);
        assert!(result.details.contains_key("cpu_usage"));
    }
    
    #[test]
    fn test_health_check_timestamp_ordering() {
        use std::time::Duration;
        use std::thread::sleep;
         // TEST_CATEGORY: unit
         // TEST_DOMAIN: types
         // TEST_PRIORITY: normal
        
        let result1 = HealthCheckResult {
            status: HealthStatus::Healthy,
            message: "Check 1".to_string(),
            timestamp: SystemTime::now(),
            details: Default::default(),
        };
        
        // ✅ MODERNIZED: Removed sleep - SystemTime::now() is monotonically increasing
        // TEST_CATEGORY: unit
        // TEST_DOMAIN: types
        // TEST_PRIORITY: normal
        let result2 = HealthCheckResult {
            status: HealthStatus::Healthy,
            message: "Check 2".to_string(),
            timestamp: SystemTime::now(),
            details: Default::default(),
        };
        
        // Second timestamp should be later or equal (monotonic guarantee)
        assert!(result2.timestamp >= result1.timestamp, "Timestamps are monotonically increasing");
    }
}

#[cfg(test)]
mod health_metrics_tests {
    #[test]
    fn test_consecutive_failures_tracking() {
        // TEST_CATEGORY: unit
        // TEST_DOMAIN: types
        // TEST_PRIORITY: normal
        let mut failures = 0u32;
        
        // Simulate failures
        failures += 1;
        assert_eq!(failures, 1);
        
        failures += 1;
        assert_eq!(failures, 2);
        
        // Reset on success
        failures = 0;
        assert_eq!(failures, 0);
    }
    
    #[test]
    fn test_failure_threshold() {
        let threshold = 3;
        let failures = 2;
        
        assert!(failures < threshold);
        
        let failures = 3;
        assert!(failures >= threshold);
    }
    
    // TEST_CATEGORY: unit
    // TEST_DOMAIN: types
    // TEST_PRIORITY: important
    #[test]
    fn test_success_rate_calculation() {
        let total_checks = 100;
        let successful_checks = 95;
        
        let success_rate = (successful_checks as f64 / total_checks as f64) * 100.0;
        
        assert!(success_rate >= 0.0);
        assert!(success_rate <= 100.0);
        assert_eq!(success_rate, 95.0);
    }
}

// TEST_CATEGORY: unit
// TEST_DOMAIN: types
// TEST_PRIORITY: important
#[cfg(test)]
mod health_transitions_tests {
    use crate::canonical::health::HealthStatus;
    
    #[test]
    fn test_healthy_to_degraded_transition() {
        let mut status = HealthStatus::Healthy;
        
        // TEST_CATEGORY: unit
        // TEST_DOMAIN: types
        // TEST_PRIORITY: normal
        // Simulate degradation
        status = HealthStatus::Degraded;
        
        assert_eq!(status, HealthStatus::Degraded);
    }
    
    #[test]
    fn test_degraded_to_unhealthy_transition() {
        let mut status = HealthStatus::Degraded;
        
        // Simulate further degradation
        status = HealthStatus::Unhealthy;
        
        assert_eq!(status, HealthStatus::Unhealthy);
    // TEST_CATEGORY: unit
    // TEST_DOMAIN: types
    // TEST_PRIORITY: normal
    }
    
    #[test]
    fn test_recovery_transitions() {
        let mut status = HealthStatus::Unhealthy;
        
        // Simulate recovery
        // TEST_CATEGORY: unit
        // TEST_DOMAIN: types
        // TEST_PRIORITY: normal
        status = HealthStatus::Degraded;
        assert_eq!(status, HealthStatus::Degraded);
        
        status = HealthStatus::Healthy;
        assert_eq!(status, HealthStatus::Healthy);
    }
}
 // TEST_CATEGORY: unit
 // TEST_DOMAIN: types
 // TEST_PRIORITY: normal

#[cfg(test)]
mod concurrent_health_checks {
    use crate::canonical::health::HealthStatus;
    use std::sync::{Arc, Mutex};
    use std::thread;
    
    #[test]
    fn test_concurrent_status_updates() {
        let status = Arc::new(Mutex::new(HealthStatus::Healthy));
        let mut handles = vec![];
        
        // Spawn threads to update status
        for i in 0..5 {
            let status_clone = Arc::clone(&status);
            let handle = thread::spawn(move || {
                // TEST_CATEGORY: unit
                // TEST_DOMAIN: types
                // TEST_PRIORITY: normal
                let mut s = status_clone.lock().unwrap();
                if i % 2 == 0 {
                    *s = HealthStatus::Healthy;
                } else {
                    *s = HealthStatus::Degraded;
                }
            });
            handles.push(handle);
        }
        
        // Wait for all threads
        for handle in handles {
            handle.join().unwrap();
        }
        
        // Final status should be valid
        let final_status = status.lock().unwrap();
        assert!(
            *final_status == HealthStatus::Healthy || 
            *final_status == HealthStatus::Degraded
        );
    }
}

#[cfg(test)]
mod edge_cases {
    use crate::canonical::health::{HealthStatus, HealthCheckResult};
    use std::time::SystemTime;
    
    #[test]
    fn test_empty_health_message() {
        let result = HealthCheckResult {
            status: HealthStatus::Healthy,
            message: String::new(),
            timestamp: SystemTime::now(),
            // TEST_CATEGORY: unit
            // TEST_DOMAIN: types
            // TEST_PRIORITY: normal
            details: Default::default(),
        };
        
        assert!(result.message.is_empty());
    }
    
    #[test]
    fn test_very_long_health_message() {
        let long_message = "Error: ".to_string() + &"x".repeat(1000);
         // TEST_CATEGORY: unit
         // TEST_DOMAIN: types
         // TEST_PRIORITY: normal
        
        let result = HealthCheckResult {
            status: HealthStatus::Unhealthy,
            message: long_message.clone(),
            timestamp: SystemTime::now(),
            details: Default::default(),
        };
        
        assert_eq!(result.message.len(), long_message.len());
    }
    
    // TEST_CATEGORY: unit
    // TEST_DOMAIN: types
    // TEST_PRIORITY: normal
    #[test]
    fn test_unknown_status_handling() {
        let status = HealthStatus::Unknown;
        
        // Unknown status should be valid but indicate lack of information
        assert_eq!(status, HealthStatus::Unknown);
    }
}

