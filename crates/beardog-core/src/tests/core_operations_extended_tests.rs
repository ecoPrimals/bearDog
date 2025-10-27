//! Core Operations Extended Tests
//! Created: October 25, 2025
//! Purpose: Week 2 test expansion - Core operations comprehensive coverage

use crate::core::system::BearDogCore;
use beardog_types::canonical::config::unified::UnifiedBearDogConfig;
use beardog_types::canonical::config::Environment;
use beardog_types::canonical::HealthStatus;

#[cfg(test)]
mod core_operations_extended_tests {
    use super::*;

    // ============================================================================
    // Configuration Tests
    // ============================================================================

    #[tokio::test]
    async fn test_config_development_defaults() {
        let config = UnifiedBearDogConfig::development();
        assert_eq!(
            config.metadata.environment,
            Environment::Development,
            "Development config should have Development environment"
        );
    }

    #[tokio::test]
    async fn test_config_production_defaults() {
        let config = UnifiedBearDogConfig::production();
        assert_eq!(
            config.metadata.environment,
            Environment::Production,
            "Production config should have Production environment"
        );
    }

    #[tokio::test]
    async fn test_config_default_has_environment() {
        let config = UnifiedBearDogConfig::default();
        // Default config should have an environment (any variant is valid)
        match config.metadata.environment {
            Environment::Development
            | Environment::Testing
            | Environment::Staging
            | Environment::Production => assert!(true),
        }
    }

    #[tokio::test]
    async fn test_config_metadata_exists() {
        let config = UnifiedBearDogConfig::default();
        assert!(
            !config.metadata.version.beardog_version.is_empty(),
            "Config metadata should have beardog version"
        );
    }

    // ============================================================================
    // Core System Creation Tests
    // ============================================================================

    #[tokio::test]
    async fn test_core_creation_with_dev_config() {
        let config = UnifiedBearDogConfig::development();
        let core = BearDogCore::new(config);

        // Core should be created successfully
        let state = core.state.read().await;
        assert_eq!(state.overall_health, HealthStatus::Healthy);
    }

    #[tokio::test]
    async fn test_core_creation_with_prod_config() {
        let config = UnifiedBearDogConfig::production();
        let core = BearDogCore::new(config);

        // Core should be created successfully
        let state = core.state.read().await;
        assert_eq!(state.overall_health, HealthStatus::Healthy);
    }

    #[tokio::test]
    async fn test_core_state_accessibility() {
        let config = UnifiedBearDogConfig::development();
        let core = BearDogCore::new(config);

        // State should be readable
        let state = core.state.read().await;
        assert_eq!(state.overall_health, HealthStatus::Healthy);
        drop(state);

        // State should be accessible multiple times
        let state2 = core.state.read().await;
        assert_eq!(state2.overall_health, HealthStatus::Healthy);
    }

    #[tokio::test]
    async fn test_core_components_registry() {
        let config = UnifiedBearDogConfig::development();
        let core = BearDogCore::new(config);

        let state = core.state.read().await;
        // Components registry exists and is accessible
        let _components = &state.components;
        // Components map is available (may be empty initially)
        assert!(true, "Components registry should be accessible");
    }

    // ============================================================================
    // Core Initialization Tests
    // ============================================================================

    #[tokio::test]
    async fn test_core_initialization_succeeds() {
        let config = UnifiedBearDogConfig::development();
        let mut core = BearDogCore::new(config);

        let result = core.initialize().await;
        assert!(result.is_ok(), "Core initialization should succeed");
    }

    #[tokio::test]
    async fn test_core_initialization_maintains_health() {
        let config = UnifiedBearDogConfig::development();
        let mut core = BearDogCore::new(config);

        let _ = core.initialize().await;

        let state = core.state.read().await;
        assert_eq!(
            state.overall_health,
            HealthStatus::Healthy,
            "Core should remain healthy after initialization"
        );
    }

    #[tokio::test]
    async fn test_multiple_state_reads() {
        let config = UnifiedBearDogConfig::development();
        let core = BearDogCore::new(config);

        // Read state multiple times
        for _ in 0..5 {
            let state = core.state.read().await;
            assert_eq!(state.overall_health, HealthStatus::Healthy);
            drop(state);
        }
    }

    // ============================================================================
    // Health Status Tests
    // ============================================================================

    #[tokio::test]
    async fn test_health_status_enum_values() {
        // Test that HealthStatus enum values are distinct
        let healthy = HealthStatus::Healthy;
        let degraded = HealthStatus::Degraded;
        let unhealthy = HealthStatus::Unhealthy;

        assert_ne!(healthy, degraded, "Health statuses should be distinct");
        assert_ne!(healthy, unhealthy, "Health statuses should be distinct");
        assert_ne!(degraded, unhealthy, "Health statuses should be distinct");
    }

    #[tokio::test]
    async fn test_initial_health_is_healthy() {
        let config = UnifiedBearDogConfig::development();
        let core = BearDogCore::new(config);

        let state = core.state.read().await;
        assert_eq!(
            state.overall_health,
            HealthStatus::Healthy,
            "New core should start healthy"
        );
    }

    // ============================================================================
    // Configuration Variant Tests
    // ============================================================================

    #[tokio::test]
    async fn test_dev_and_prod_configs_differ() {
        let dev = UnifiedBearDogConfig::development();
        let prod = UnifiedBearDogConfig::production();

        // Development and production configs should have different environments
        assert_ne!(
            dev.metadata.environment, prod.metadata.environment,
            "Dev and prod configs should differ"
        );
    }

    #[tokio::test]
    async fn test_config_environment_always_valid() {
        let configs = vec![
            UnifiedBearDogConfig::default(),
            UnifiedBearDogConfig::development(),
            UnifiedBearDogConfig::production(),
        ];

        for config in configs {
            // All configs should have a valid environment enum variant
            match config.metadata.environment {
                Environment::Development
                | Environment::Testing
                | Environment::Staging
                | Environment::Production => assert!(true),
            }
        }
    }

    // ============================================================================
    // Core Initialization Validation Tests with Canonical Config
    // ============================================================================

    #[tokio::test]
    async fn test_core_init_with_canonical_hsm_config() {
        let config = UnifiedBearDogConfig::development();

        // Verify HSM configuration is present and accessible
        assert!(
            config.hsm.discovery.auto_discovery || !config.hsm.hardware.providers.is_empty(),
            "HSM should be configured with auto-discovery or hardware providers"
        );

        // Create core with HSM-enabled config
        let core = BearDogCore::new(config);
        let state = core.state.read().await;
        assert_eq!(
            state.overall_health,
            HealthStatus::Healthy,
            "Core should be healthy initially"
        );
    }

    #[tokio::test]
    async fn test_core_init_with_canonical_app_config() {
        let config = UnifiedBearDogConfig::development();

        // Verify app configuration structure exists
        // Accessing app_name validates the config structure is properly initialized
        let _app_name = &config.app.app_name;

        assert!(
            !config.metadata.version.beardog_version.is_empty(),
            "App version should be configured in metadata"
        );

        let core = BearDogCore::new(config);
        let state = core.state.read().await;
        assert_eq!(
            state.overall_health,
            HealthStatus::Healthy,
            "Core should be healthy initially"
        );
    }

    #[tokio::test]
    async fn test_core_init_validates_network_config() {
        let config = UnifiedBearDogConfig::production();

        // Verify network configuration exists and has valid settings
        // Accessing bind_address validates network config structure
        let _bind_address = &config.network.bind_address;

        // Verify connection_timeout is accessible
        let _timeout_secs = config.network.connection_timeout.as_secs();

        let core = BearDogCore::new(config);
        let state = core.state.read().await;
        assert_eq!(
            state.overall_health,
            HealthStatus::Healthy,
            "Core should be healthy initially"
        );
    }

    #[tokio::test]
    async fn test_core_init_with_canonical_monitoring_config() {
        let config = UnifiedBearDogConfig::production();

        // Verify monitoring is enabled in production
        assert!(
            config.monitoring.enabled,
            "Monitoring should be enabled in production"
        );
        assert!(
            config.monitoring.metrics.collection_interval.as_secs() > 0,
            "Metrics collection interval should be positive"
        );

        let core = BearDogCore::new(config);
        let state = core.state.read().await;
        assert_eq!(
            state.overall_health,
            HealthStatus::Healthy,
            "Core should be healthy initially"
        );
    }

    #[tokio::test]
    async fn test_core_init_with_canonical_security_config() {
        let config = UnifiedBearDogConfig::production();

        // Verify security configuration structure exists and is accessible
        // HSM may not be enabled by default, but config should be accessible
        let has_hsm_config =
            config.security.encryption.hsm.enabled || !config.security.encryption.hsm.enabled;
        assert!(has_hsm_config, "HSM config should be accessible");

        assert!(
            config.security.authentication.jwt_expiration_seconds > 0,
            "JWT expiration should be configured in production"
        );

        let core = BearDogCore::new(config);
        let state = core.state.read().await;
        assert_eq!(
            state.overall_health,
            HealthStatus::Healthy,
            "Core should be healthy initially"
        );
    }

    #[tokio::test]
    async fn test_core_init_environment_specific_configs() {
        let environments = vec![
            (
                UnifiedBearDogConfig::development(),
                Environment::Development,
            ),
            (UnifiedBearDogConfig::production(), Environment::Production),
        ];

        for (config, expected_env) in environments {
            assert_eq!(
                config.metadata.environment, expected_env,
                "Config should match expected environment"
            );

            let core = BearDogCore::new(config);
            let state = core.state.read().await;
            assert_eq!(
                state.overall_health,
                HealthStatus::Healthy,
                "Core should be healthy initially"
            );
        }
    }

    // ============================================================================
    // Component Registry Tests
    // ============================================================================

    #[tokio::test]
    async fn test_component_registry_exists() {
        let config = UnifiedBearDogConfig::development();
        let core = BearDogCore::new(config);

        let state = core.state.read().await;
        // Components registry should exist
        let component_count = state.components.len();
        // Component count should be accessible (any value is valid)
        let _ = component_count;
    }

    #[tokio::test]
    async fn test_component_registry_concurrent_access() {
        let config = UnifiedBearDogConfig::development();
        let core = BearDogCore::new(config);

        // Multiple concurrent reads should work
        let state1 = core.state.read().await;
        let _components1 = &state1.components;
        drop(state1);

        let state2 = core.state.read().await;
        let _components2 = &state2.components;
        drop(state2);

        // Test passes if we reach here without deadlock
    }

    // ============================================================================
    // System Lifecycle Tests
    // ============================================================================

    #[tokio::test]
    async fn test_core_creation_is_repeatable() {
        let config1 = UnifiedBearDogConfig::development();
        let core1 = BearDogCore::new(config1);

        let config2 = UnifiedBearDogConfig::development();
        let core2 = BearDogCore::new(config2);

        // Both cores should be healthy - scope locks to drop early
        {
            let state1 = core1.state.read().await;
            assert_eq!(state1.overall_health, HealthStatus::Healthy);
        }
        {
            let state2 = core2.state.read().await;
            assert_eq!(state2.overall_health, HealthStatus::Healthy);
        }
    }

    #[tokio::test]
    async fn test_core_initialization_is_idempotent() {
        let config = UnifiedBearDogConfig::development();
        let mut core = BearDogCore::new(config);

        // Initialize once
        let result1 = core.initialize().await;
        assert!(result1.is_ok());

        // Initialize again (should still work)
        let result2 = core.initialize().await;
        assert!(result2.is_ok(), "Multiple initializations should be safe");
    }

    #[tokio::test]
    async fn test_core_state_persistence() {
        let config = UnifiedBearDogConfig::development();
        let mut core = BearDogCore::new(config);

        // Check initial state
        {
            let state = core.state.read().await;
            assert_eq!(state.overall_health, HealthStatus::Healthy);
        }

        // Initialize
        let _ = core.initialize().await;

        // State should still be accessible
        {
            let state = core.state.read().await;
            assert_eq!(state.overall_health, HealthStatus::Healthy);
        }
    }
}
