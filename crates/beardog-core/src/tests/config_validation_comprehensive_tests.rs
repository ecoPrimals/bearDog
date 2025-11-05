//! Comprehensive Configuration Validation Tests
//!
//! Tests for configuration validation, defaults, and error cases.

use beardog_types::canonical::config::RuntimeConfig;

#[cfg(test)]
mod config_validation_tests {
    use super::*;

    #[test]
    fn test_default_config_is_valid() {
        let config = RuntimeConfig::default();

        // Verify basic validity
        assert!(config.network.api_port > 0, "API port should be positive");
        assert!(
            config.network.health_port > 0,
            "Health port should be positive"
        );
        assert!(
            config.network.metrics_port > 0,
            "Metrics port should be positive"
        );
    }

    #[test]
    fn test_network_config_defaults() {
        let config = RuntimeConfig::default();

        // Verify network defaults
        assert!(
            config.network.max_connections > 0,
            "Max connections should be positive"
        );
        assert!(
            config.network.timeout_seconds > 0,
            "Timeout should be positive"
        );
        assert!(
            !config.network.api_host.is_empty(),
            "API host should not be empty"
        );
    }

    #[test]
    fn test_ports_are_different() {
        let config = RuntimeConfig::default();

        // All ports should be different to avoid conflicts
        assert_ne!(
            config.network.api_port, config.network.health_port,
            "API and health ports should differ"
        );
        assert_ne!(
            // TEST_CATEGORY: integration
            // TEST_DOMAIN: core
            // TEST_PRIORITY: normal
            config.network.api_port,
            config.network.metrics_port,
            "API and metrics ports should differ"
        );
        assert_ne!(
            config.network.health_port, config.network.metrics_port,
            "Health and metrics ports should differ"
        );
    }

    #[test]
    fn test_timeout_is_reasonable() {
        let config = RuntimeConfig::default();

        // TEST_CATEGORY: integration
        // TEST_DOMAIN: core
        // TEST_PRIORITY: normal
        // Timeout should be reasonable (not too short, not too long)
        assert!(
            config.network.timeout_seconds >= 1,
            "Timeout should be at least 1 second"
        );
        assert!(
            config.network.timeout_seconds <= 3600,
            "Timeout should not exceed 1 hour"
        );
    }

    #[test]
    fn test_max_connections_is_reasonable() {
        let config = RuntimeConfig::default();

        // Max connections should be reasonable
        // TEST_CATEGORY: integration
        // TEST_DOMAIN: core
        // TEST_PRIORITY: normal
        assert!(
            config.network.max_connections >= 10,
            "Should support at least 10 connections"
        );
        assert!(
            config.network.max_connections <= 100_000,
            "Should not exceed 100,000 connections"
        );
    }

    #[test]
    fn test_config_is_cloneable() {
        let config = RuntimeConfig::default();
        let cloned = config.clone();

        // Verify clone produces equivalent config
        // TEST_CATEGORY: integration
        // TEST_DOMAIN: core
        // TEST_PRIORITY: normal
        assert_eq!(config.network.api_port, cloned.network.api_port);
        assert_eq!(config.network.health_port, cloned.network.health_port);
        assert_eq!(config.network.metrics_port, cloned.network.metrics_port);
        assert_eq!(config.network.api_host, cloned.network.api_host);
    }

    #[test]
    fn test_config_is_debuggable() {
        let config = RuntimeConfig::default();

        // Should be able to debug print config
        let debug_output = format!("{config:?}");
        // TEST_CATEGORY: integration
        // TEST_DOMAIN: core
        // TEST_PRIORITY: normal
        assert!(!debug_output.is_empty(), "Debug output should not be empty");
        assert!(
            debug_output.contains("RuntimeConfig"),
            "Debug output should contain type name"
        );
    }

    #[test]
    fn test_tls_enabled_by_default() {
        let config = RuntimeConfig::default();

        // TLS should be enabled by default for security
        // TEST_CATEGORY: integration
        // TEST_DOMAIN: core
        // TEST_PRIORITY: normal
        assert!(
            config.network.enable_tls,
            "TLS should be enabled by default"
        );
    }

    #[test]
    fn test_environment_is_set() {
        let config = RuntimeConfig::default();
        // TEST_CATEGORY: integration
        // TEST_DOMAIN: core
        // TEST_PRIORITY: normal

        // Environment should be set
        assert!(
            !config.environment.is_empty(),
            "Environment should be set to a default value"
        );
    }

    #[test]
    fn test_api_host_not_empty() {
        // TEST_CATEGORY: integration
        // TEST_DOMAIN: core
        // TEST_PRIORITY: normal
        let config = RuntimeConfig::default();

        assert!(
            !config.network.api_host.is_empty(),
            "API host should have a default value"
        );
    }

    // TEST_CATEGORY: integration
    // TEST_DOMAIN: core
    // TEST_PRIORITY: normal
    #[test]
    fn test_port_ranges_valid() {
        let config = RuntimeConfig::default();

        // All ports should be in valid range (> 0 since they're u16)
        assert!(config.network.api_port > 0);
        assert!(config.network.health_port > 0);
        assert!(config.network.metrics_port > 0);
        // TEST_CATEGORY: integration
        // TEST_DOMAIN: core
        // TEST_PRIORITY: normal
    }

    #[test]
    fn test_config_has_sensible_defaults() {
        let config = RuntimeConfig::default();

        // Check various sensible defaults
        // TEST_CATEGORY: integration
        // TEST_DOMAIN: core
        // TEST_PRIORITY: normal
        assert!(
            config.network.api_port > 1024,
            "API port should be above privileged range"
        );
        assert!(
            config.network.health_port > 1024,
            "Health port should be above privileged range" // TEST_CATEGORY: integration
                                                           // TEST_DOMAIN: core
                                                           // TEST_PRIORITY: normal
        );
        assert!(
            config.network.metrics_port > 1024,
            "Metrics port should be above privileged range"
        );
    }

    #[test]
    fn test_network_timeout_not_zero() {
        let config = RuntimeConfig::default();

        assert!(
            config.network.timeout_seconds > 0,
            "Network timeout should be greater than zero"
        );
    }
    // TEST_CATEGORY: integration
    // TEST_DOMAIN: core
    // TEST_PRIORITY: normal

    #[test]
    fn test_network_max_connections_not_zero() {
        let config = RuntimeConfig::default();

        assert!(
            config.network.max_connections > 0,
            // TEST_CATEGORY: integration
            // TEST_DOMAIN: core
            // TEST_PRIORITY: normal
            "Max connections should be greater than zero"
        );
    }

    #[test]
    fn test_config_enables_security_by_default() {
        let config = RuntimeConfig::default();
        // TEST_CATEGORY: integration
        // TEST_DOMAIN: core
        // TEST_PRIORITY: normal

        // Security features should be enabled by default
        assert!(config.network.enable_tls, "TLS should be enabled");
        // Add more security-related checks as needed
    }

    // TEST_CATEGORY: integration
    // TEST_DOMAIN: core
    // TEST_PRIORITY: normal
    #[test]
    fn test_config_struct_size_reasonable() {
        use std::mem::size_of;

        let size = size_of::<RuntimeConfig>();

        // Config should not be excessively large (< 1KB is reasonable)
        assert!(
            size < 1024,
            "RuntimeConfig size should be reasonable (<1KB), actual: {size} bytes"
        );
    }
}
