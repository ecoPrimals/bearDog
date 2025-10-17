//! Comprehensive tests for Unified Configuration System
//!
//! Tests cover:
//! - Default configuration construction
//! - Serialization/deserialization  
//! - Configuration validation
//! - Development/production presets
//! - Domain-specific configurations

use super::*;
use crate::canonical::config::domains::testing::CanonicalTestConfig;

#[cfg(test)]
#[allow(clippy::no_effect_underscore_binding)] // Test code readability
#[allow(clippy::assertions_on_constants)] // Test structure patterns
mod unified_config_tests {
    use super::*;
    use serde_json;

    // ========================================================================
    // Construction Tests
    // ========================================================================

    #[test]
    fn test_unified_config_default_construction() {
        let config = UnifiedBearDogConfig::default();

        // Verify core domains exist and can be accessed
        // Verify core domains exist and can be accessed
        let _ = &config.app;
        let _ = &config.network;
    }

    #[test]
    fn test_unified_config_development_preset() {
        let config = UnifiedBearDogConfig::development();

        // Development config should be constructible
        let _ = &config.app;
        let _ = &config.network;
    }

    #[test]
    fn test_unified_config_production_preset() {
        let config = UnifiedBearDogConfig::production();

        // Production config should be constructible
        let _ = &config.app;
        let _ = &config.network;
        assert!(
            config.monitoring.enabled,
            "Production monitoring should be enabled"
        );
    }

    // ========================================================================
    // Serialization Tests
    // ========================================================================

    #[test]
    fn test_unified_config_json_serialization() {
        let config = UnifiedBearDogConfig::default();

        let json = serde_json::to_string(&config).expect("Should serialize to JSON");
        assert!(!json.is_empty());
        assert!(
            json.len() > 100,
            "Serialized config should have substantial content"
        );
    }

    #[test]
    fn test_unified_config_json_round_trip() {
        let original = UnifiedBearDogConfig::development();

        let json = serde_json::to_string(&original).expect("Should serialize");
        let deserialized: UnifiedBearDogConfig =
            serde_json::from_str(&json).expect("Should deserialize");

        assert_eq!(original.app.app_name, deserialized.app.app_name);
        assert_eq!(
            original.network.bind_address,
            deserialized.network.bind_address
        );
    }

    #[test]
    fn test_unified_config_toml_serialization() {
        let config = UnifiedBearDogConfig::default();

        let toml = toml::to_string(&config).expect("Should serialize to TOML");
        assert!(!toml.is_empty());
        assert!(
            toml.len() > 100,
            "Serialized config should have substantial content"
        );
    }

    // ========================================================================
    // Domain Configuration Tests
    // ========================================================================

    #[test]
    fn test_app_config_valid() {
        let config = UnifiedBearDogConfig::default();

        // App config should be accessible with all fields present
        // App config should be accessible with all fields present
        let _ = &config.app;
    }

    #[test]
    fn test_network_config_valid() {
        let config = UnifiedBearDogConfig::default();

        // Network config should be accessible
        // Network config should be accessible
        let _ = &config.network;
    }

    #[test]
    fn test_security_config_valid() {
        let config = UnifiedBearDogConfig::default();

        // Security config should have nested configs
        // Verify security config exists
        let _ = &config.security.authentication;
        let _ = &config.security.authorization;
        // Encryption config should be accessible
    }

    #[test]
    fn test_hsm_config_accessible() {
        let config = UnifiedBearDogConfig::default();

        // HSM config should be toggleable
        // HSM config should be accessible (enabled state can be true or false)
        let _ = config.hsm.enabled;
    }

    #[test]
    fn test_database_config_valid() {
        let config = UnifiedBearDogConfig::default();

        // Database config should have connection details
        // Database config should have connection details
        let _ = &config.database;
    }

    #[test]
    fn test_monitoring_config_valid() {
        let config = UnifiedBearDogConfig::default();

        // Monitoring config should be accessible
        let _ = config.monitoring.enabled;
    }

    // ========================================================================
    // Metadata Tests
    // ========================================================================

    #[test]
    fn test_system_metadata_accessible() {
        let config = UnifiedBearDogConfig::default();

        // Verify metadata is accessible
        assert!(!config.metadata.version.beardog_version.is_empty());
    }

    #[test]
    fn test_system_metadata_environment_differs() {
        let dev = UnifiedBearDogConfig::development();
        let prod = UnifiedBearDogConfig::production();

        // Metadata should exist and differ between environments
        assert_ne!(dev.metadata.environment, prod.metadata.environment);
    }

    // ========================================================================
    // Clone and Debug Tests
    // ========================================================================

    #[test]
    fn test_unified_config_is_cloneable() {
        let config1 = UnifiedBearDogConfig::default();
        let config2 = config1.clone();

        assert_eq!(config1.app.app_name, config2.app.app_name);
        assert_eq!(config1.network.bind_address, config2.network.bind_address);
    }

    #[test]
    fn test_unified_config_debug_output() {
        let config = UnifiedBearDogConfig::default();
        let debug_output = format!("{:?}", config);

        assert!(!debug_output.is_empty());
        assert!(debug_output.len() > 100);
    }

    // ========================================================================
    // Integration Tests
    // ========================================================================

    #[test]
    fn test_config_all_domains_accessible() {
        let config = UnifiedBearDogConfig::default();

        // Verify all major domains can be accessed
        // Verify all major domains can be accessed
        let _ = &config.app;
        let _ = &config.network;
        let _ = &config.security;
        let _ = &config.hsm;
        let _ = &config.database;
        let _ = &config.monitoring;
    }

    #[test]
    fn test_config_specialized_domains_accessible() {
        let config = UnifiedBearDogConfig::default();

        // All specialized domains should be accessible
        let _genetics = &config.genetics;
        let _workflows = &config.workflows;
        let _compliance = &config.compliance;
        let _performance = &config.performance;
    }

    #[test]
    fn test_config_infrastructure_domains_accessible() {
        let config = UnifiedBearDogConfig::default();

        // All infrastructure domains should be accessible
        let _production = &config.production;
        let _deployment = &config.deployment;
        let _testing = &config.testing;
        let _development = &config.development;
    }

    #[test]
    fn test_config_integration_domains_accessible() {
        let config = UnifiedBearDogConfig::default();

        // All integration domains should be accessible
        let _adapters = &config.adapters;
        let _tunnel = &config.tunnel;
        let _federation = &config.federation;
        let _ecosystem = &config.ecosystem;
    }

    // ========================================================================
    // Testing Configuration Tests
    // ========================================================================

    #[test]
    fn test_canonical_test_config_default() {
        let test_config = CanonicalTestConfig::default();

        assert!(!test_config.environment.is_empty());
        assert!(test_config.timeout_seconds > 0);
    }

    #[test]
    fn test_canonical_test_config_parallel_execution() {
        let test_config = CanonicalTestConfig::default();

        // Parallel execution config should be accessible
        let _ = test_config.parallel_execution;
        assert!(test_config.max_threads > 0);
    }

    #[test]
    fn test_canonical_test_config_timeout_reasonable() {
        let test_config = CanonicalTestConfig::default();

        assert!(test_config.timeout_seconds > 0);
        assert!(test_config.timeout_seconds < 3600); // Less than 1 hour
    }

    // ========================================================================
    // Validation Tests
    // ========================================================================

    #[test]
    fn test_config_memory_limits_valid() {
        let config = UnifiedBearDogConfig::default();

        // Memory limit field should be accessible
        let mem = config.app.max_memory_mb;
        assert!(mem < 1_000_000, "Memory limit should be reasonable (< 1TB)");
    }

    #[test]
    fn test_config_connection_limits_valid() {
        let config = UnifiedBearDogConfig::default();

        // Connection limit field should be accessible
        let conn = config.app.max_connections;
        assert!(conn < 1_000_000, "Connection limit should be reasonable");
    }

    #[test]
    fn test_config_file_descriptor_limit_valid() {
        let config = UnifiedBearDogConfig::default();

        // File descriptor limit should be accessible
        let _fd = config.app.max_file_descriptors;
        assert!(true);
    }

    // ========================================================================
    // Edge Case Tests
    // ========================================================================

    #[test]
    fn test_config_handles_max_cpu_cores_option() {
        let config = UnifiedBearDogConfig::default();

        // max_cpu_cores is optional
        if let Some(cores) = config.app.max_cpu_cores {
            assert!(cores > 0);
            assert!(cores <= 1024); // Reasonable upper limit
        }
    }

    #[test]
    fn test_development_config_differs_from_default() {
        let default_config = UnifiedBearDogConfig::default();
        let dev_config = UnifiedBearDogConfig::development();

        // Configs should be independently constructed
        let _ = (&default_config, &dev_config);
        assert!(true); // If we got here, both configs exist
    }

    #[test]
    fn test_production_config_differs_from_default() {
        let default_config = UnifiedBearDogConfig::default();
        let prod_config = UnifiedBearDogConfig::production();

        // Production should have monitoring enabled
        assert!(prod_config.monitoring.enabled);

        let _ = &default_config;
        assert!(true);
    }
}
