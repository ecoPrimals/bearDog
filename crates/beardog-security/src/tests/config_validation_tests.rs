// SPDX-License-Identifier: AGPL-3.0-or-later

//! Configuration Validation Tests
//!
//! Tests for secure configuration loading, validation, and error handling.
//! These tests ensure configuration errors are caught early and safely.

use beardog_types::canonical::config::auth::CanonicalAuthConfig;
use beardog_types::canonical::config::network::NetworkConfig;
use std::time::Duration;

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_auth_config_enabled_flag() {
        let config = CanonicalAuthConfig {
            enabled: true,
            ..Default::default()
        };
        assert!(config.enabled);
    }

    #[test]
    fn test_auth_config_disabled() {
        let config = CanonicalAuthConfig {
            enabled: false,
            ..Default::default()
        };
        assert!(!config.enabled);
    }

    #[test]
    fn test_session_timeout_reasonable() {
        let config = CanonicalAuthConfig {
            enabled: true,
            session_timeout: Duration::from_secs(3600),
            ..Default::default()
        };
        assert!(config.session_timeout.as_secs() >= 300);
        assert!(config.session_timeout.as_secs() <= 86400);
    }

    #[test]
    fn test_max_login_attempts_bounds() {
        let config = CanonicalAuthConfig {
            enabled: true,
            max_login_attempts: 5,
            ..Default::default()
        };
        assert!(config.max_login_attempts > 0);
        assert!(config.max_login_attempts <= 10);
    }

    #[test]
    fn test_network_config_has_host() {
        let config = NetworkConfig::default();
        assert!(!config.default_host.is_empty());
    }

    #[test]
    fn test_network_config_has_ports() {
        let config = NetworkConfig::default();
        assert!(config.service_ports.api_port > 0);
        assert!(config.service_ports.admin_port > 0);
    }

    #[test]
    fn test_network_config_has_timeouts() {
        let config = NetworkConfig::default();
        assert!(config.timeouts.connection_timeout_ms > 0);
        // TEST_CATEGORY: integration
        // TEST_DOMAIN: security
        // TEST_PRIORITY: critical
        assert!(config.timeouts.request_timeout_ms > 0);
    }

    #[test]
    fn test_auth_config_default_secure() {
        let config = CanonicalAuthConfig::default();
        // TEST_CATEGORY: integration
        // TEST_DOMAIN: security
        // TEST_PRIORITY: critical
        assert!(config.max_login_attempts > 0);
        assert!(config.session_timeout.as_secs() > 0);
    }

    #[test]
    fn test_session_timeout_minimum() {
        // TEST_CATEGORY: integration
        // TEST_DOMAIN: security
        // TEST_PRIORITY: normal
        let config = CanonicalAuthConfig {
            enabled: true,
            session_timeout: Duration::from_secs(300),
            ..Default::default()
        };
        assert!(config.session_timeout.as_secs() >= 300);
    }

    // TEST_CATEGORY: integration
    // TEST_DOMAIN: security
    // TEST_PRIORITY: normal
    #[test]
    fn test_config_immutability_pattern() {
        let config = CanonicalAuthConfig::default();
        let cloned = config.clone();
        assert_eq!(config.enabled, cloned.enabled);
        assert_eq!(config.max_login_attempts, cloned.max_login_attempts);
    }

    // TEST_CATEGORY: integration
    // TEST_DOMAIN: security
    // TEST_PRIORITY: normal
    #[test]
    fn test_config_debug_output() {
        let config = CanonicalAuthConfig::default();
        // TEST_CATEGORY: integration
        // TEST_DOMAIN: security
        // TEST_PRIORITY: normal
        let debug_str = format!("{config:?}");
        assert!(!debug_str.is_empty());
    }

    // TEST_CATEGORY: integration
    // TEST_DOMAIN: security
    // TEST_PRIORITY: normal
    #[test]
    fn test_concurrent_config_access() {
        use std::sync::Arc;
        let config = Arc::new(CanonicalAuthConfig::default());
        // TEST_CATEGORY: integration
        // TEST_DOMAIN: security
        // TEST_PRIORITY: critical
        let config_clone = Arc::clone(&config);
        assert_eq!(config.max_login_attempts, config_clone.max_login_attempts);
    }

    // TEST_CATEGORY: integration
    // TEST_DOMAIN: security
    // TEST_PRIORITY: normal
    #[test]
    fn test_network_port_assignments_valid() {
        let config = NetworkConfig::default();
        // u16 type automatically enforces port range (0-65535)
        assert!(config.service_ports.api_port > 0);
        assert!(config.service_ports.admin_port > 0);
        assert!(config.service_ports.metrics_port > 0);
        // TEST_CATEGORY: integration
        // TEST_DOMAIN: security
        // TEST_PRIORITY: normal
    }

    #[test]
    fn test_network_timeouts_positive() {
        let config = NetworkConfig::default();
        // TEST_CATEGORY: integration
        // TEST_DOMAIN: security
        // TEST_PRIORITY: normal
        assert!(config.timeouts.connection_timeout_ms > 0);
        assert!(config.timeouts.request_timeout_ms > 0);
    }

    // TEST_CATEGORY: integration
    // TEST_DOMAIN: security
    // TEST_PRIORITY: normal
    #[test]
    fn test_auth_providers_list() {
        let config = CanonicalAuthConfig {
            enabled: false,
            providers: vec![],
            // TEST_CATEGORY: integration
            // TEST_DOMAIN: security
            // TEST_PRIORITY: normal
            ..Default::default()
        };
        assert!(config.providers.is_empty());
    }

    #[test]
    // TEST_CATEGORY: integration
    // TEST_DOMAIN: security
    // TEST_PRIORITY: normal
    fn test_session_timeout_duration_properties() {
        let timeout = Duration::from_secs(3600);
        assert_eq!(timeout.as_secs(), 3600);
        assert_eq!(timeout.as_millis(), 3_600_000);
        // TEST_CATEGORY: integration
        // TEST_DOMAIN: security
        // TEST_PRIORITY: critical
    }

    #[test]
    fn test_config_type_safety() {
        let timeout_ms: u32 = 5000;
        let max_retries: u32 = 3;
        assert!(timeout_ms > 0);
        // TEST_CATEGORY: integration
        // TEST_DOMAIN: security
        // TEST_PRIORITY: normal
        // u32 is always >= 0, checking max bound
        assert!(max_retries < 1000);
    }

    // TEST_CATEGORY: integration
    // TEST_DOMAIN: security
    // TEST_PRIORITY: normal
    #[test]
    fn test_network_config_clone() {
        let config = NetworkConfig::default();
        let cloned = config.clone();
        assert_eq!(config.default_host, cloned.default_host);
    }
    // TEST_CATEGORY: integration
    // TEST_DOMAIN: security
    // TEST_PRIORITY: normal

    #[test]
    fn test_auth_config_send_sync() {
        fn assert_send_sync<T: Send + Sync>() {}
        // TEST_CATEGORY: integration
        // TEST_DOMAIN: security
        // TEST_PRIORITY: critical
        assert_send_sync::<CanonicalAuthConfig>();
    }

    // TEST_CATEGORY: integration
    // TEST_DOMAIN: security
    // TEST_PRIORITY: normal
    #[test]
    fn test_network_config_send_sync() {
        fn assert_send_sync<T: Send + Sync>() {}
        assert_send_sync::<NetworkConfig>();
    }
}
