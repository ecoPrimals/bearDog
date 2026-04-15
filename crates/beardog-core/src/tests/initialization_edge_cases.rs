// SPDX-License-Identifier: AGPL-3.0-or-later

//! Initialization edge case tests for beardog-core
//!
//! Tests edge cases and error conditions during system initialization

#[cfg(test)]
mod tests {
    use beardog_types::canonical::config::runtime_config::RuntimeConfig;

    #[test]
    fn test_runtime_config_default_values() {
        // Test that default config has sensible values
        let config = RuntimeConfig::default();

        // Verify ports are valid (non-zero, u16 type ensures they're in range)
        assert!(config.network.api_port > 0);
        assert!(config.network.metrics_port > 0);
        assert!(config.network.health_port > 0);

        // Verify ports are different
        assert_ne!(config.network.api_port, config.network.metrics_port);
        assert_ne!(config.network.api_port, config.network.health_port);
        assert_ne!(config.network.metrics_port, config.network.health_port);

        // Verify reasonable timeout
        assert!(config.network.timeout_seconds > 0);
        assert!(config.network.timeout_seconds < 3600); // Less than 1 hour

        // Verify max connections is reasonable
        assert!(config.network.max_connections > 0);
        assert!(config.network.max_connections < 100_000);
    }

    #[test]
    fn test_runtime_config_network_discovery() {
        let config = RuntimeConfig::default();
        // TEST_CATEGORY: integration
        // TEST_DOMAIN: core
        // TEST_PRIORITY: normal

        // Should have network configuration
        assert!(!config.network.api_host.is_empty());

        // Network config should be valid (non-zero, u16 type ensures valid range)
        assert!(config.network.api_port > 0);
    }

    #[test]
    fn test_runtime_config_host_not_empty() {
        let config = RuntimeConfig::default();

        // API host should not be empty
        assert!(!config.network.api_host.is_empty());

        // Should be a valid format (either IP or hostname)
        assert!(
            config.network.api_host.parse::<std::net::IpAddr>().is_ok()
                || !config.network.api_host.contains(' ') // Basic hostname validation
        );
    }
    // TEST_CATEGORY: integration
    // TEST_DOMAIN: core
    // TEST_PRIORITY: normal

    #[test]
    fn test_runtime_config_tls_defaults() {
        let config = RuntimeConfig::default();

        // TLS should be enabled by default for production readiness
        assert!(config.network.enable_tls);
    }
    // TEST_CATEGORY: integration
    // TEST_DOMAIN: core
    // TEST_PRIORITY: normal

    #[test]
    fn test_runtime_config_reasonable_connections() {
        let config = RuntimeConfig::default();

        // Max connections should be reasonable for a server
        assert!(
            config.network.max_connections >= 100,
            "Should support at least 100 connections"
        );
        assert!(
            // TEST_CATEGORY: integration
            // TEST_DOMAIN: core
            // TEST_PRIORITY: normal
            config.network.max_connections <= 10_000,
            "Should not exceed 10,000 connections"
        );
    }

    // TEST_CATEGORY: integration
    // TEST_DOMAIN: core
    // TEST_PRIORITY: normal
    #[test]
    fn test_runtime_config_timeout_reasonable() {
        let config = RuntimeConfig::default();

        // Timeout should be reasonable (not too short, not too long)
        assert!(
            config.network.timeout_seconds >= 5,
            "Timeout should be at least 5 seconds"
        );
        assert!(
            config.network.timeout_seconds <= 300,
            "Timeout should not exceed 5 minutes" // TEST_CATEGORY: integration
                                                  // TEST_DOMAIN: core
                                                  // TEST_PRIORITY: normal
        );
    }

    #[test]
    fn test_runtime_config_ws_port_differs_from_api() {
        let config = RuntimeConfig::default();

        // WebSocket port should differ from API port to avoid conflicts
        assert_ne!(config.network.ws_port, config.network.api_port);
    }

    #[test]
    // TEST_CATEGORY: integration
    // TEST_DOMAIN: core
    // TEST_PRIORITY: normal
    fn test_runtime_config_grpc_port_differs() {
        let config = RuntimeConfig::default();

        // gRPC port should differ from other ports
        assert_ne!(config.network.grpc_port, config.network.api_port);
        // TEST_CATEGORY: integration
        // TEST_DOMAIN: core
        // TEST_PRIORITY: normal
        assert_ne!(config.network.grpc_port, config.network.ws_port);
        assert_ne!(config.network.grpc_port, config.network.metrics_port);
    }

    #[test]
    fn test_runtime_config_all_ports_unique() {
        let config = RuntimeConfig::default();
        // TEST_CATEGORY: integration
        // TEST_DOMAIN: core
        // TEST_PRIORITY: normal

        // Collect all ports
        let ports = [
            config.network.api_port,
            config.network.metrics_port,
            config.network.health_port,
            config.network.ws_port,
            config.network.grpc_port,
        ];

        // Verify all are unique
        let unique_ports: std::collections::HashSet<_> = ports.iter().collect();
        assert_eq!(
            unique_ports.len(),
            ports.len(),
            "All ports should be unique"
        );
    }

    // TEST_CATEGORY: integration
    // TEST_DOMAIN: core
    // TEST_PRIORITY: normal
    #[test]
    fn test_runtime_config_endpoints_not_empty() {
        let config = RuntimeConfig::default();

        // All endpoint strings should be non-empty
        assert!(!config.network.discovery_endpoint.is_empty());
        assert!(!config.network.api_host.is_empty());
    }
}
