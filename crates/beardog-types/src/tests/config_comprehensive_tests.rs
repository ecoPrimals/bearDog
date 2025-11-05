use crate::canonical::config::network::NetworkConfig;
use crate::canonical::config::security::SecurityConfig;
use crate::canonical::config::performance::PerformanceConfig;
use crate::canonical::HealthStatus;
use std::time::Duration;

#[cfg(test)]
mod config_comprehensive_tests {
    use super::*;

    // ============================================================================
    // NetworkConfig Tests
    // ============================================================================

    #[test]
    fn test_network_config_default() {
        let config = NetworkConfig::default();
        assert!(!config.host.is_empty(), "Default host should not be empty");
        assert!(config.port > 0, "Default port should be valid");
        assert!(config.timeout_ms > 0, "Default timeout should be positive");
    }

    #[test]
    fn test_network_config_with_custom_values() {
        let config = NetworkConfig {
            host: "192.168.1.100".to_string(),
            port: 9090,
            timeout_ms: 5000,
            max_connections: 200,
            enable_tls: true,
            ..Default::default()
        };

        assert_eq!(config.host, "192.168.1.100");
        assert_eq!(config.port, 9090);
        assert_eq!(config.timeout_ms, 5000);
        assert_eq!(config.max_connections, 200);
        assert!(config.enable_tls);
    }

    #[test]
    fn test_network_config_serialization() {
        let config = NetworkConfig::default();
        let json = serde_json::to_string(&config).expect("Failed to serialize");
        let deserialized: NetworkConfig = serde_json::from_str(&json).expect("Failed to deserialize");
        assert_eq!(config.host, deserialized.host);
        assert_eq!(config.port, deserialized.port);
    }

    #[test]
    fn test_network_config_clone() {
        let config = NetworkConfig::default();
        let cloned = config.clone();
        assert_eq!(config.host, cloned.host);
        assert_eq!(config.port, cloned.port);
        assert_eq!(config.timeout_ms, cloned.timeout_ms);
    }

    #[test]
    fn test_network_config_debug() {
        let config = NetworkConfig::default();
        let debug_str = format!("{:?}", config);
        assert!(debug_str.contains("NetworkConfig"));
    }

    // ============================================================================
    // SecurityConfig Tests
    // ============================================================================

    #[test]
    fn test_security_config_default() {
        let config = SecurityConfig::default();
        assert!(config.enable_encryption, "Encryption should be enabled by default");
    }

    #[test]
    fn test_security_config_with_custom_values() {
        let config = SecurityConfig {
            enable_encryption: true,
            enable_mfa: true,
            session_timeout_minutes: 60,
            max_failed_attempts: 5,
            ..Default::default()
        };

        assert!(config.enable_encryption);
        assert!(config.enable_mfa);
        assert_eq!(config.session_timeout_minutes, 60);
        assert_eq!(config.max_failed_attempts, 5);
    // TEST_CATEGORY: integration
    // TEST_DOMAIN: types
    // TEST_PRIORITY: normal
    }

    #[test]
    fn test_security_config_serialization() {
        let config = SecurityConfig::default();
        // TEST_CATEGORY: integration
        // TEST_DOMAIN: types
        // TEST_PRIORITY: normal
        let json = serde_json::to_string(&config).expect("Failed to serialize");
        let deserialized: SecurityConfig = serde_json::from_str(&json).expect("Failed to deserialize");
        assert_eq!(config.enable_encryption, deserialized.enable_encryption);
    }

    #[test]
    fn test_security_config_timeout_range() {
        let config = SecurityConfig {
            session_timeout_minutes: 30,
            ..Default::default()
        };
        assert!(config.session_timeout_minutes >= 1, "Timeout should be at least 1 minute");
        assert!(config.session_timeout_minutes <= 1440, "Timeout should be at most 24 hours");
    }

    // TEST_CATEGORY: integration
    // TEST_DOMAIN: types
    // TEST_PRIORITY: normal
    // ============================================================================
    // PerformanceConfig Tests
    // ============================================================================

    #[test]
    fn test_performance_config_default() {
        // TEST_CATEGORY: integration
        // TEST_DOMAIN: types
        // TEST_PRIORITY: normal
        let config = PerformanceConfig::default();
        assert!(config.thread_pool_size > 0, "Thread pool should have at least 1 thread");
        assert!(config.queue_size > 0, "Queue size should be positive");
    }

    #[test]
    // TEST_CATEGORY: integration
    // TEST_DOMAIN: types
    // TEST_PRIORITY: normal
    fn test_performance_config_with_custom_values() {
        let config = PerformanceConfig {
            thread_pool_size: 16,
            queue_size: 10000,
            enable_batching: true,
            batch_size: 100,
            ..Default::default()
        };
 // TEST_CATEGORY: integration
 // TEST_DOMAIN: types
 // TEST_PRIORITY: normal

        assert_eq!(config.thread_pool_size, 16);
        assert_eq!(config.queue_size, 10000);
        // TEST_CATEGORY: integration
        // TEST_DOMAIN: types
        // TEST_PRIORITY: normal
        assert!(config.enable_batching);
        assert_eq!(config.batch_size, 100);
    }

    #[test]
    fn test_performance_config_serialization() {
        let config = PerformanceConfig::default();
        let json = serde_json::to_string(&config).expect("Failed to serialize");
        let deserialized: PerformanceConfig = serde_json::from_str(&json).expect("Failed to deserialize");
        assert_eq!(config.thread_pool_size, deserialized.thread_pool_size);
    }

    #[test]
    // TEST_CATEGORY: integration
    // TEST_DOMAIN: types
    // TEST_PRIORITY: normal
    fn test_performance_config_reasonable_values() {
        let config = PerformanceConfig::default();
        assert!(config.thread_pool_size <= 1000, "Thread pool should be reasonable");
        assert!(config.queue_size <= 1_000_000, "Queue size should be reasonable");
    }
 // TEST_CATEGORY: integration
 // TEST_DOMAIN: types
 // TEST_PRIORITY: normal

    // ============================================================================
    // HealthStatus Tests
    // ============================================================================

    #[test]
    fn test_health_status_healthy() {
        let status = HealthStatus::Healthy;
        assert_eq!(format!("{:?}", status), "Healthy");
    }

    // TEST_CATEGORY: integration
    // TEST_DOMAIN: types
    // TEST_PRIORITY: normal
    #[test]
    fn test_health_status_degraded() {
        let status = HealthStatus::Degraded;
        assert_eq!(format!("{:?}", status), "Degraded");
    // TEST_CATEGORY: integration
    // TEST_DOMAIN: types
    // TEST_PRIORITY: normal
    }

    #[test]
    fn test_health_status_unhealthy() {
        let status = HealthStatus::Unhealthy;
        assert_eq!(format!("{:?}", status), "Unhealthy");
    }

    #[test]
    fn test_health_status_equality() {
        assert_eq!(HealthStatus::Healthy, HealthStatus::Healthy);
        assert_ne!(HealthStatus::Healthy, HealthStatus::Degraded);
        assert_ne!(HealthStatus::Healthy, HealthStatus::Unhealthy);
    // TEST_CATEGORY: integration
    // TEST_DOMAIN: types
    // TEST_PRIORITY: normal
    }

    #[test]
    fn test_health_status_clone() {
        let status = HealthStatus::Healthy;
        // TEST_CATEGORY: integration
        // TEST_DOMAIN: types
        // TEST_PRIORITY: normal
        let cloned = status.clone();
        assert_eq!(status, cloned);
    }

    #[test]
    fn test_health_status_serialization() {
        for status in [HealthStatus::Healthy, HealthStatus::Degraded, HealthStatus::Unhealthy] {
            let json = serde_json::to_string(&status).expect("Failed to serialize");
            // TEST_CATEGORY: integration
            // TEST_DOMAIN: types
            // TEST_PRIORITY: normal
            let deserialized: HealthStatus = serde_json::from_str(&json).expect("Failed to deserialize");
            assert_eq!(status, deserialized);
        }
    // TEST_CATEGORY: integration
    // TEST_DOMAIN: types
    // TEST_PRIORITY: normal
    }

    // ============================================================================
    // TEST_CATEGORY: integration
    // TEST_DOMAIN: types
    // TEST_PRIORITY: normal
    // Config Integration Tests
    // ============================================================================

    // TEST_CATEGORY: integration
    // TEST_DOMAIN: types
    // TEST_PRIORITY: normal
    #[test]
    fn test_all_configs_serializable() {
        let network = NetworkConfig::default();
        let security = SecurityConfig::default();
        // TEST_CATEGORY: integration
        // TEST_DOMAIN: types
        // TEST_PRIORITY: normal
        let performance = PerformanceConfig::default();

        assert!(serde_json::to_string(&network).is_ok());
        assert!(serde_json::to_string(&security).is_ok());
        // TEST_CATEGORY: integration
        // TEST_DOMAIN: types
        // TEST_PRIORITY: normal
        assert!(serde_json::to_string(&performance).is_ok());
    }

    #[test]
    fn test_config_round_trip() {
        let network = NetworkConfig::default();
        let json = serde_json::to_string(&network).unwrap();
        let restored: NetworkConfig = serde_json::from_str(&json).unwrap();
        
        let json2 = serde_json::to_string(&restored).unwrap();
        // TEST_CATEGORY: integration
        // TEST_DOMAIN: types
        // TEST_PRIORITY: normal
        assert_eq!(json, json2, "Round trip should be stable");
    }

    // ============================================================================
    // Edge Cases
    // ============================================================================

    #[test]
    // TEST_CATEGORY: integration
    // TEST_DOMAIN: types
    // TEST_PRIORITY: normal
    fn test_network_config_extreme_values() {
        let config = NetworkConfig {
            port: 65535, // Max port
            timeout_ms: u64::MAX,
            max_connections: usize::MAX,
            ..Default::default()
        };

        assert_eq!(config.port, 65535);
        assert_eq!(config.timeout_ms, u64::MAX);
    }
 // TEST_CATEGORY: integration
 // TEST_DOMAIN: types
 // TEST_PRIORITY: normal

    #[test]
    fn test_security_config_zero_timeout() {
        let config = SecurityConfig {
            session_timeout_minutes: 0, // Edge case: no timeout
            ..Default::default()
        };
        assert_eq!(config.session_timeout_minutes, 0);
    }

    // TEST_CATEGORY: integration
    // TEST_DOMAIN: types
    // TEST_PRIORITY: normal
    #[test]
    fn test_performance_config_single_thread() {
        let config = PerformanceConfig {
            thread_pool_size: 1, // Minimum threads
            queue_size: 1, // Minimum queue
            ..Default::default()
        // TEST_CATEGORY: integration
        // TEST_DOMAIN: types
        // TEST_PRIORITY: normal
        };
        assert_eq!(config.thread_pool_size, 1);
        assert_eq!(config.queue_size, 1);
    }

    #[test]
    fn test_network_config_ipv6() {
        let config = NetworkConfig {
            // TEST_CATEGORY: integration
            // TEST_DOMAIN: types
            // TEST_PRIORITY: normal
            host: "::1".to_string(), // IPv6 localhost
            ..Default::default()
        };
        assert_eq!(config.host, "::1");
    }

    // TEST_CATEGORY: integration
    // TEST_DOMAIN: types
    // TEST_PRIORITY: normal
    #[test]
    fn test_network_config_domain_name() {
        let config = NetworkConfig {
            host: "example.com".to_string(),
            ..Default::default()
        };
        assert_eq!(config.host, "example.com");
    }
}

