//! Comprehensive tests for HSM health monitoring types

#[cfg(test)]
mod tests {
    use crate::hsm::health::*;
    use std::time::Duration;

    #[test]
    fn test_provider_health_new() {
        let health = ProviderHealth::new(
            true,
            "operational".to_string(),
            Duration::from_millis(50),
        );
        
        assert!(health.is_available);
        assert_eq!(health.status_message, "operational");
        assert_eq!(health.response_time, Duration::from_millis(50));
    }

    #[test]
    fn test_provider_health_default() {
        let health = ProviderHealth::default();
        
        assert!(!health.is_available);
        assert!(!health.status_message.is_empty());
        assert_eq!(health.response_time, Duration::from_secs(0));
    }

    #[test]
    fn test_provider_health_healthy() {
        let health = ProviderHealth::healthy();
        
        assert!(health.is_available);
        assert!(health.is_healthy());
        assert!(health.status_message.contains("healthy") || health.status_message.contains("available"));
    }

    #[test]
    fn test_provider_health_unhealthy() {
        // TEST_CATEGORY: unit
        // TEST_DOMAIN: types
        // TEST_PRIORITY: normal
        let health = ProviderHealth::unhealthy("connection failed".to_string());
        
        assert!(!health.is_available);
        assert!(!health.is_healthy());
        assert_eq!(health.status_message, "connection failed");
    }

    #[test]
    fn test_provider_health_is_healthy() {
        let healthy = ProviderHealth::new(
            // TEST_CATEGORY: unit
            // TEST_DOMAIN: types
            // TEST_PRIORITY: normal
            true,
            "ok".to_string(),
            Duration::from_millis(10),
        );
        assert!(healthy.is_healthy());
        
        // TEST_CATEGORY: unit
        // TEST_DOMAIN: types
        // TEST_PRIORITY: normal
        let unhealthy = ProviderHealth::new(
            false,
            "error".to_string(),
            Duration::from_secs(1),
        );
        assert!(!unhealthy.is_healthy());
    // TEST_CATEGORY: unit
    // TEST_DOMAIN: types
    // TEST_PRIORITY: normal
    }

    #[test]
    fn test_provider_health_with_fast_response() {
        let health = ProviderHealth::new(
            true,
            // TEST_CATEGORY: unit
            // TEST_DOMAIN: types
            // TEST_PRIORITY: normal
            "fast".to_string(),
            Duration::from_millis(5),
        );
        
        assert!(health.is_available);
        assert!(health.response_time < Duration::from_millis(100));
    }

    #[test]
    fn test_provider_health_with_slow_response() {
        let health = ProviderHealth::new(
            true,
            "slow".to_string(),
            Duration::from_secs(2),
        // TEST_CATEGORY: unit
        // TEST_DOMAIN: types
        // TEST_PRIORITY: normal
        );
        
        assert!(health.is_available);
        assert!(health.response_time > Duration::from_secs(1));
    }

    #[test]
    fn test_provider_health_update_status() {
        let mut health = ProviderHealth::healthy();
         // TEST_CATEGORY: unit
         // TEST_DOMAIN: types
         // TEST_PRIORITY: normal
        
        health.update_status(false, "service down".to_string());
        
        assert!(!health.is_available);
        assert_eq!(health.status_message, "service down");
    }

    #[test]
    fn test_provider_health_update_response_time() {
        // TEST_CATEGORY: unit
        // TEST_DOMAIN: types
        // TEST_PRIORITY: normal
        let mut health = ProviderHealth::healthy();
        
        health.update_response_time(Duration::from_millis(75));
        
        assert_eq!(health.response_time, Duration::from_millis(75));
    }

    // TEST_CATEGORY: unit
    // TEST_DOMAIN: types
    // TEST_PRIORITY: normal
    #[test]
    fn test_provider_health_clone() {
        let health1 = ProviderHealth::new(
            true,
            "active".to_string(),
            Duration::from_millis(25),
        // TEST_CATEGORY: unit
        // TEST_DOMAIN: types
        // TEST_PRIORITY: normal
        );
        let health2 = health1.clone();
        
        assert_eq!(health1.is_available, health2.is_available);
        assert_eq!(health1.status_message, health2.status_message);
        assert_eq!(health1.response_time, health2.response_time);
    }

    #[test]
    fn test_provider_health_debug_format() {
        let health = ProviderHealth::new(
            // TEST_CATEGORY: unit
            // TEST_DOMAIN: types
            // TEST_PRIORITY: normal
            true,
            "testing".to_string(),
            Duration::from_millis(30),
        );
        
        let debug_str = format!("{:?}", health);
        assert!(debug_str.contains("ProviderHealth"));
    }

    // TEST_CATEGORY: unit
    // TEST_DOMAIN: types
    // TEST_PRIORITY: normal
    #[test]
    fn test_provider_health_transitions() {
        let mut health = ProviderHealth::healthy();
        assert!(health.is_healthy());
        
        // Transition to unhealthy
        health.update_status(false, "error occurred".to_string());
        assert!(!health.is_healthy());
        
        // Transition back to healthy
        health.update_status(true, "recovered".to_string());
        assert!(health.is_healthy());
    }
}

