// SPDX-License-Identifier: AGPL-3.0-only

//! Configuration validation tests for beardog-types
//!
//! Tests validation logic for various configuration types

#[cfg(test)]
mod tests {
    use crate::canonical::config::runtime_config::RuntimeConfig;

    #[test]
    fn test_port_validation_ranges() {
        let config = RuntimeConfig::default();

        // All ports should be in valid range (1-65535)
        // Note: u16 max is 65535, so upper bound check is unnecessary
        assert!(config.network.api_port > 0);
        assert!(config.network.metrics_port > 0);
        assert!(config.network.health_port > 0);
        assert!(config.network.ws_port > 0);
        assert!(config.network.grpc_port > 0);
    }

    #[test]
    fn test_port_uniqueness() {
        let config = RuntimeConfig::default();

        let ports = [
            config.network.api_port,
            config.network.metrics_port,
            config.network.health_port,
            config.network.ws_port,
            config.network.grpc_port,
        ];

        // Create a set and verify no duplicates
        let unique_ports: std::collections::HashSet<_> = ports.iter().collect();
        // TEST_CATEGORY: integration
        // TEST_DOMAIN: types
        // TEST_PRIORITY: normal
        assert_eq!(
            unique_ports.len(),
            ports.len(),
            "All ports must be unique to avoid conflicts"
        );
    }

    #[test]
    fn test_timeout_reasonable_bounds() {
        let config = RuntimeConfig::default();
        // TEST_CATEGORY: integration
        // TEST_DOMAIN: types
        // TEST_PRIORITY: normal

        // Timeout should be reasonable
        assert!(
            config.network.timeout_seconds >= 1,
            "Timeout must be at least 1 second"
        );
        assert!(
            config.network.timeout_seconds <= 600,
            "Timeout should not exceed 10 minutes"
        );
    }

    #[test]
    fn test_max_connections_reasonable() {
        let config = RuntimeConfig::default();

        // Max connections should be reasonable
        assert!(
            // TEST_CATEGORY: integration
            // TEST_DOMAIN: types
            // TEST_PRIORITY: normal
            config.network.max_connections >= 10,
            "Should support at least 10 connections"
        );
        assert!(
            config.network.max_connections <= 100_000,
            "Should not exceed 100,000 connections"
        );
    }

    #[test]
    fn test_api_host_not_empty() {
        let config = RuntimeConfig::default();
        // TEST_CATEGORY: integration
        // TEST_DOMAIN: types
        // TEST_PRIORITY: normal

        assert!(
            !config.network.api_host.is_empty(),
            "API host must not be empty"
        );
    }

    #[test]
    fn test_discovery_endpoint_format() {
        let config = RuntimeConfig::default();

        // Should be a valid URL format
        // TEST_CATEGORY: integration
        // TEST_DOMAIN: types
        // TEST_PRIORITY: normal
        assert!(
            config.network.discovery_endpoint.starts_with("http://")
                || config.network.discovery_endpoint.starts_with("https://"),
            "Discovery endpoint should be a valid HTTP(S) URL"
        );
    }

    // TEST_CATEGORY: integration
    // TEST_DOMAIN: types
    // TEST_PRIORITY: normal
    #[test]
    fn test_tls_enabled_by_default() {
        let config = RuntimeConfig::default();

        // For production readiness, TLS should be on by default
        assert!(
            config.network.enable_tls,
            "TLS should be enabled by default for security"
        );
        // TEST_CATEGORY: integration
        // TEST_DOMAIN: types
        // TEST_PRIORITY: normal
    }

    #[test]
    fn test_well_known_port_avoidance() {
        let config = RuntimeConfig::default();

        // Should avoid well-known ports (0-1023) to prevent permission issues
        let ports = vec![
            // TEST_CATEGORY: integration
            // TEST_DOMAIN: types
            // TEST_PRIORITY: normal
            config.network.api_port,
            config.network.metrics_port,
            config.network.health_port,
            config.network.ws_port,
            config.network.grpc_port,
        ];

        for port in ports {
            assert!(
                port > 1023,
                "Port {port} should be above 1023 to avoid well-known port range"
            );
        }
    }

    #[test]
    fn test_config_cloneable() {
        let config = RuntimeConfig::default();
        // TEST_CATEGORY: integration
        // TEST_DOMAIN: types
        // TEST_PRIORITY: normal
        let cloned = config.clone();

        // Verify clone produces equivalent config
        assert_eq!(config.network.api_port, cloned.network.api_port);
        assert_eq!(config.network.api_host, cloned.network.api_host);
        assert_eq!(
            config.network.timeout_seconds,
            cloned.network.timeout_seconds
        );
        assert_eq!(config.network.enable_tls, cloned.network.enable_tls);
    }

    // TEST_CATEGORY: integration
    // TEST_DOMAIN: types
    // TEST_PRIORITY: normal
    #[test]
    fn test_config_debug_output() {
        let config = RuntimeConfig::default();

        // Should be able to debug print (useful for logging)
        let debug_str = format!("{config:?}");
        assert!(!debug_str.is_empty());
        assert!(debug_str.contains("RuntimeConfig"));
    }
}
