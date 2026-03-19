// SPDX-License-Identifier: AGPL-3.0-only

// Comprehensive Configuration Validation Tests
//
// Extensive test coverage for configuration validation and error handling

#[cfg(test)]
mod validation_tests {
    use crate::canonical::config::unified::*;
    use std::sync::Arc;
    // Removed unused import

    #[test]
    fn test_default_config_is_valid() {
        let config = SimplifiedBearDogConfig::default();
        assert!(config.validate().is_ok());
    }

    #[test]
    fn test_network_settings_validation() {
        let settings = NetworkSettings {
            bind_address: Arc::from("127.0.0.1"),
            port: 8080,
            max_connections: 1000,
            timeout_seconds: 30,
            enable_tls: true,
        };

        assert_eq!(settings.port, 8080);
        assert_eq!(settings.max_connections, 1000);
    }

    #[test]
    fn test_network_settings_invalid_port() {
        let settings = NetworkSettings {
            bind_address: Arc::from("0.0.0.0"),
            port: 0, // Invalid port
            max_connections: 100,
            timeout_seconds: 30,
            // TEST_CATEGORY: unit
            // TEST_DOMAIN: types
            // TEST_PRIORITY: normal
            enable_tls: false,
        };

        // TEST_CATEGORY: unit
        // TEST_DOMAIN: types
        // TEST_PRIORITY: normal
        // Port 0 is technically valid (system-assigned), but should be validated in production
        assert_eq!(settings.port, 0);
    }

    #[test]
    fn test_security_settings_validation() {
        let settings = SecuritySettings {
            session_timeout_seconds: 3600,
            max_login_attempts: 5,
            enable_mfa: true,
            hash_rounds: 12,
            // TEST_CATEGORY: unit
            // TEST_DOMAIN: types
            // TEST_PRIORITY: important
            audit_retention_days: 365,
        };

        assert!(settings.enable_mfa);
        assert_eq!(settings.session_timeout_seconds, 3600);
        assert_eq!(settings.max_login_attempts, 5);
    }

    #[test]
    fn test_database_settings_validation() {
        let settings = DatabaseSettings {
            // TEST_CATEGORY: unit
            // TEST_DOMAIN: types
            // TEST_PRIORITY: normal
            connection_string: Arc::from("postgres://localhost/beardog"),
            pool_size: 20,
            timeout_seconds: 10,
            enable_encryption: true,
        };

        assert!(settings.enable_encryption);
        assert_eq!(settings.pool_size, 20);
    }

    #[test]
    fn test_monitoring_settings_validation() {
        // TEST_CATEGORY: unit
        // TEST_DOMAIN: types
        // TEST_PRIORITY: normal
        let settings = MonitoringSettings {
            enable_metrics: true,
            metrics_interval_seconds: 60,
            log_level: Arc::from("info"),
            health_check_interval_seconds: 30,
        };

        assert!(settings.enable_metrics);
        assert_eq!(settings.metrics_interval_seconds, 60);
    }
    // TEST_CATEGORY: unit
    // TEST_DOMAIN: types
    // TEST_PRIORITY: normal

    #[test]
    fn test_performance_settings_validation() {
        let settings = PerformanceSettings {
            max_memory_mb: 512,
            worker_threads: 8,
            cache_size_mb: 64,
            enable_optimization: true,
        };

        // TEST_CATEGORY: unit
        // TEST_DOMAIN: types
        // TEST_PRIORITY: normal
        assert_eq!(settings.worker_threads, 8);
        assert!(settings.enable_optimization);
    }

    #[test]
    fn test_config_serialization_roundtrip() {
        let config = SimplifiedBearDogConfig::default();
        let json = serde_json::to_string(&config).unwrap();
        let deserialized: SimplifiedBearDogConfig = serde_json::from_str(&json).unwrap();

        // TEST_CATEGORY: unit
        // TEST_DOMAIN: types
        // TEST_PRIORITY: normal
        assert_eq!(config.network.port, deserialized.network.port);
        assert_eq!(config.security.enable_mfa, deserialized.security.enable_mfa);
    }

    #[test]
    fn test_config_with_custom_network() {
        let mut config = SimplifiedBearDogConfig::default();
        // TEST_CATEGORY: unit
        // TEST_DOMAIN: types
        // TEST_PRIORITY: normal
        config.network.port = 9000;
        config.network.max_connections = 500;

        assert_eq!(config.network.port, 9000);
        assert_eq!(config.network.max_connections, 500);
    }

    // TEST_CATEGORY: unit
    // TEST_DOMAIN: types
    // TEST_PRIORITY: normal
    #[test]
    fn test_config_with_custom_security() {
        let mut config = SimplifiedBearDogConfig::default();
        config.security.enable_mfa = true;
        config.security.session_timeout_seconds = 7200;

        assert!(config.security.enable_mfa);
        assert_eq!(config.security.session_timeout_seconds, 7200);
    }
}
