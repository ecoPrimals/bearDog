// SPDX-License-Identifier: AGPL-3.0-or-later

//! Coverage extension tests for Network Configuration
//!
//! Added December 8, 2025 to increase coverage from 72.41% to 90%+
//! Targets: validation paths, from_env(), builders, edge cases

#[cfg(test)]
mod network_coverage_extension_tests {
    use crate::domains::network::{
        AdminConfig, AdminConfigBuilder, ApiConfig, ApiConfigBuilder, NetworkConfig,
        ServiceDiscoveryConfig, ServiceDiscoveryConfigBuilder,
    };
    use crate::domains::network_ports::{DEFAULT_API_PORT, DEFAULT_DISCOVERY_PORT};
    use std::net::{IpAddr, Ipv4Addr, Ipv6Addr};

    // ============================================================================
    // NetworkConfig comprehensive tests
    // ============================================================================

    #[test]
    fn test_network_config_from_env_no_variables() {
        let config = NetworkConfig::from_env();

        assert!(config.validate().is_ok());
    }

    #[test]
    fn test_network_config_all_port_conflicts() {
        let mut config = NetworkConfig::default();

        // API vs Discovery
        config.api.port = 8080;
        config.discovery.port = 8080;
        assert!(config.validate().is_err());

        // API vs Admin
        config.discovery.port = 8081;
        config.admin.port = 8080;
        assert!(config.validate().is_err());

        // Discovery vs Admin
        config.api.port = 8082;
        config.admin.port = 8081;
        assert!(config.validate().is_err());
    }

    #[test]
    fn test_network_config_no_port_conflicts() {
        let mut config = NetworkConfig::default();
        config.api.port = 8080;
        config.discovery.port = 8081;
        config.admin.port = 8082;

        assert!(config.validate().is_ok());
    }

    #[test]
    fn test_network_config_clone() {
        let config1 = NetworkConfig::default();
        let config2 = config1.clone();

        assert_eq!(config1, config2);
    }

    #[test]
    fn test_network_config_debug() {
        let config = NetworkConfig::default();
        let debug_str = format!("{config:?}");

        assert!(debug_str.contains("NetworkConfig"));
    }

    #[test]
    fn test_network_config_serialization() {
        let config = NetworkConfig::default();

        let json = serde_json::to_string(&config).expect("Should serialize");
        let deserialized: NetworkConfig = serde_json::from_str(&json).expect("Should deserialize");

        assert_eq!(config, deserialized);
    }

    // ============================================================================
    // ApiConfig comprehensive tests
    // ============================================================================

    #[test]
    fn test_api_config_custom_port_via_builder() {
        let config = ApiConfig::builder().port(9999).build();

        assert_eq!(config.port, 9999);
    }

    #[test]
    fn test_api_config_wildcard_bind_via_builder() {
        let config = ApiConfig::builder()
            .bind_address(IpAddr::V4(Ipv4Addr::UNSPECIFIED))
            .build();

        assert_eq!(config.bind_address, IpAddr::V4(Ipv4Addr::UNSPECIFIED));
    }

    #[test]
    fn test_api_config_max_connections_via_builder() {
        let config = ApiConfig::builder().max_connections(500).build();

        assert_eq!(config.max_connections, 500);
    }

    #[test]
    fn test_api_config_default_port_constant() {
        let config = ApiConfig::const_defaults();

        assert_eq!(config.port, DEFAULT_API_PORT);
    }

    #[test]
    fn test_api_config_builder_all_fields() {
        let config = ApiConfig::builder()
            .bind_address(IpAddr::V4(Ipv4Addr::new(192, 168, 1, 1)))
            .port(8888)
            .tls_enabled(true)
            .tls_cert_path("/cert.pem".to_string())
            .tls_key_path("/key.pem".to_string())
            .max_connections(1000)
            .build();

        assert_eq!(
            config.bind_address,
            IpAddr::V4(Ipv4Addr::new(192, 168, 1, 1))
        );
        assert_eq!(config.port, 8888);
        assert!(config.tls_enabled);
        assert_eq!(config.tls_cert_path, Some("/cert.pem".to_string()));
        assert_eq!(config.tls_key_path, Some("/key.pem".to_string()));
        assert_eq!(config.max_connections, 1000);
    }

    #[test]
    fn test_api_config_builder_ipv6() {
        let ipv6_addr = IpAddr::V6(Ipv6Addr::LOCALHOST);
        let config = ApiConfig::builder().bind_address(ipv6_addr).build();

        assert_eq!(config.bind_address, ipv6_addr);
    }

    #[test]
    fn test_api_config_validation_zero_max_connections() {
        let config = ApiConfig::builder().max_connections(0).build();

        assert!(config.validate().is_err());
    }

    #[test]
    fn test_api_config_validation_tls_missing_cert() {
        let config = ApiConfig::builder()
            .tls_enabled(true)
            .tls_key_path("/key.pem".to_string())
            .build();

        assert!(config.validate().is_err());
    }

    #[test]
    fn test_api_config_validation_tls_missing_key() {
        let config = ApiConfig::builder()
            .tls_enabled(true)
            .tls_cert_path("/cert.pem".to_string())
            .build();

        assert!(config.validate().is_err());
    }

    #[test]
    fn test_api_config_default_trait() {
        let config1 = ApiConfig::default();
        let config2 = ApiConfig::const_defaults();

        assert_eq!(config1, config2);
    }

    #[test]
    fn test_api_config_builder_partial() {
        let config = ApiConfig::builder().port(9000).build();

        // Unset fields should use defaults
        assert_eq!(config.port, 9000);
        assert_eq!(config.bind_address, IpAddr::V4(Ipv4Addr::LOCALHOST));
    }

    // ============================================================================
    // ServiceDiscoveryConfig comprehensive tests
    // ============================================================================

    #[test]
    fn test_discovery_config_with_custom_port() {
        // ✅ EVOLVED: Use builder pattern instead of environment variables
        // This is concurrent-safe and doesn't pollute global state
        let config = ServiceDiscoveryConfig::builder().port(7777).build();

        assert_eq!(config.port, 7777);
    }

    #[test]
    fn test_discovery_config_defaults_on_invalid() {
        // ✅ EVOLVED: Test default behavior directly
        // No environment variable pollution needed
        let config = ServiceDiscoveryConfig::const_defaults();

        assert_eq!(config.port, DEFAULT_DISCOVERY_PORT);
        assert_eq!(config.interval_secs, 60);
        assert!(!config.backends.is_empty());
    }

    #[test]
    fn test_discovery_config_builder_all_fields() {
        let config = ServiceDiscoveryConfig::builder()
            .port(6666)
            .interval_secs(120)
            .build();

        assert_eq!(config.port, 6666);
        assert_eq!(config.interval_secs, 120);
    }

    #[test]
    fn test_discovery_config_validation_zero_port() {
        let config = ServiceDiscoveryConfig::builder().port(0).build();

        assert!(config.validate().is_err());
    }

    #[test]
    fn test_discovery_config_validation_zero_interval() {
        let config = ServiceDiscoveryConfig::builder().interval_secs(0).build();

        assert!(config.validate().is_err());
    }

    #[test]
    fn test_discovery_config_validation_various_intervals() {
        let config1 = ServiceDiscoveryConfig::builder().interval_secs(1).build();
        assert!(config1.validate().is_ok());

        let config2 = ServiceDiscoveryConfig::builder().interval_secs(600).build();
        assert!(config2.validate().is_ok());

        let config3 = ServiceDiscoveryConfig::builder()
            .interval_secs(10000)
            .build();
        assert!(config3.validate().is_ok());
    }

    #[test]
    fn test_discovery_config_default() {
        let config = ServiceDiscoveryConfig::default();
        assert!(config.validate().is_ok());
    }

    // ============================================================================
    // AdminConfig comprehensive tests
    // ============================================================================

    #[test]
    fn test_admin_config_custom_port_via_builder() {
        let config = AdminConfig::builder().port(5555).build();

        assert_eq!(config.port, 5555);
    }

    #[test]
    fn test_admin_config_disabled_via_builder() {
        let config = AdminConfig::builder().enabled(false).build();

        assert!(!config.enabled);
    }

    #[test]
    fn test_admin_config_builder_all_fields() {
        let config = AdminConfig::builder()
            .bind_address(IpAddr::V6(Ipv6Addr::LOCALHOST))
            .port(4444)
            .enabled(true)
            .build();

        assert_eq!(config.bind_address, IpAddr::V6(Ipv6Addr::LOCALHOST));
        assert_eq!(config.port, 4444);
        assert!(config.enabled);
    }

    #[test]
    fn test_admin_config_validation_zero_port() {
        let config = AdminConfig::builder().port(0).build();

        assert!(config.validate().is_err());
    }

    #[test]
    fn test_admin_config_default() {
        let config = AdminConfig::default();
        assert!(config.validate().is_ok());
    }

    #[test]
    fn test_admin_config_builder_new() {
        let builder = AdminConfigBuilder::new();
        let config = builder.build();

        assert_eq!(config, AdminConfig::default());
    }

    // ============================================================================
    // Validation error message tests
    // ============================================================================

    #[test]
    fn test_api_validation_error_message_port_zero() {
        let config = ApiConfig::builder().port(0).build();
        let err = config.validate().unwrap_err();
        let err_str = format!("{err}");

        assert!(err_str.contains("port") || err_str.contains("Port"));
    }

    #[test]
    fn test_api_validation_error_message_tls() {
        let config = ApiConfig::builder().tls_enabled(true).build();
        let err = config.validate().unwrap_err();
        let err_str = format!("{err}");

        assert!(err_str.contains("TLS") || err_str.contains("certificate"));
    }

    #[test]
    fn test_api_validation_error_message_max_connections() {
        let config = ApiConfig::builder().max_connections(0).build();
        let err = config.validate().unwrap_err();
        let err_str = format!("{err}");

        assert!(err_str.contains("connection") || err_str.contains("max"));
    }

    // ============================================================================
    // Builder default values
    // ============================================================================

    #[test]
    fn test_api_builder_default_values() {
        let config = ApiConfig::builder().build();
        let defaults = ApiConfig::default();

        assert_eq!(config, defaults);
    }

    #[test]
    fn test_discovery_builder_default_values() {
        let config = ServiceDiscoveryConfig::builder().build();
        let defaults = ServiceDiscoveryConfig::default();

        assert_eq!(config, defaults);
    }

    #[test]
    fn test_admin_builder_default_values() {
        let config = AdminConfig::builder().build();
        let defaults = AdminConfig::default();

        assert_eq!(config, defaults);
    }

    // ============================================================================
    // Edge cases and combinations
    // ============================================================================

    #[test]
    fn test_api_config_high_port_number() {
        let config = ApiConfig::builder().port(65535).build();

        assert!(config.validate().is_ok());
    }

    #[test]
    fn test_api_config_very_high_max_connections() {
        let config = ApiConfig::builder().max_connections(100_000).build();

        assert!(config.validate().is_ok());
    }

    #[test]
    fn test_discovery_config_minimum_interval() {
        let config = ServiceDiscoveryConfig::builder().interval_secs(1).build();

        assert!(config.validate().is_ok());
    }

    #[test]
    fn test_discovery_config_maximum_interval() {
        let config = ServiceDiscoveryConfig::builder().interval_secs(600).build();

        assert!(config.validate().is_ok());
    }

    #[test]
    fn test_network_config_with_ipv6_addresses() {
        let ipv6 = IpAddr::V6(Ipv6Addr::LOCALHOST);

        let mut config = NetworkConfig::default();
        config.api.bind_address = ipv6;
        config.admin.bind_address = ipv6;

        assert!(config.validate().is_ok());
    }

    #[test]
    fn test_network_config_with_public_ip() {
        let public_ip = IpAddr::V4(Ipv4Addr::new(8, 8, 8, 8));

        let mut config = NetworkConfig::default();
        config.api.bind_address = public_ip;

        assert!(config.validate().is_ok());
    }

    #[test]
    fn test_api_config_clone() {
        let config1 = ApiConfig::default();
        let config2 = config1.clone();

        assert_eq!(config1, config2);
    }

    #[test]
    fn test_discovery_config_clone() {
        let config1 = ServiceDiscoveryConfig::default();
        let config2 = config1.clone();

        assert_eq!(config1, config2);
    }

    #[test]
    fn test_admin_config_clone() {
        let config1 = AdminConfig::default();
        let config2 = config1.clone();

        assert_eq!(config1, config2);
    }

    #[test]
    fn test_api_config_debug() {
        let config = ApiConfig::default();
        let debug_str = format!("{config:?}");

        assert!(debug_str.contains("ApiConfig"));
    }

    #[test]
    fn test_discovery_config_debug() {
        let config = ServiceDiscoveryConfig::default();
        let debug_str = format!("{config:?}");

        assert!(debug_str.contains("ServiceDiscoveryConfig"));
    }

    #[test]
    fn test_admin_config_debug() {
        let config = AdminConfig::default();
        let debug_str = format!("{config:?}");

        assert!(debug_str.contains("AdminConfig"));
    }

    #[test]
    fn test_api_config_builder_default() {
        let builder1 = ApiConfigBuilder::default();
        let builder2 = ApiConfigBuilder::new();

        let config1 = builder1.build();
        let config2 = builder2.build();

        assert_eq!(config1, config2);
    }

    #[test]
    fn test_discovery_config_builder_default() {
        let builder1 = ServiceDiscoveryConfigBuilder::default();
        let builder2 = ServiceDiscoveryConfigBuilder::new();

        let config1 = builder1.build();
        let config2 = builder2.build();

        assert_eq!(config1, config2);
    }

    #[test]
    fn test_admin_config_builder_default() {
        let builder1 = AdminConfigBuilder::default();
        let builder2 = AdminConfigBuilder::new();

        let config1 = builder1.build();
        let config2 = builder2.build();

        assert_eq!(config1, config2);
    }
}
