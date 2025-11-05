//! Startup and Error Handling Tests for `BearDog` Core
//!
//! Tests critical error scenarios during startup and lifecycle operations

use crate::core::system::BearDogCore;
use beardog_types::canonical::config::unified::UnifiedBearDogConfig;
use beardog_types::canonical::{ComponentStatus, HealthStatus};

#[cfg(test)]
mod startup_error_handling {
    use super::*;

    #[tokio::test]
    async fn test_startup_sets_components_running() {
        // Verify startup properly sets component statuses
        let config = UnifiedBearDogConfig::development();
        let core = BearDogCore::new(config);

        let result = core.startup().await;
        assert!(result.is_ok(), "Startup should succeed");

        let state = core.state.read().await;

        // Verify components are in Running state
        // TEST_CATEGORY: integration
        // TEST_DOMAIN: core
        // TEST_PRIORITY: normal
        if let Some(status) = state.components.get("security") {
            assert_eq!(
                *status,
                ComponentStatus::Running,
                "Security component should be running"
            );
        }

        // Verify overall health is healthy
        assert_eq!(
            state.overall_health,
            HealthStatus::Healthy,
            "Overall health should be healthy after startup"
        );
        drop(state);
    }

    #[tokio::test]
    async fn test_shutdown_sets_components_inactive() {
        // Verify shutdown properly transitions components
        let config = UnifiedBearDogConfig::development();
        let core = BearDogCore::new(config);

        // Start first
        core.startup().await.expect("Startup should succeed");

        // TEST_CATEGORY: integration
        // TEST_DOMAIN: core
        // TEST_PRIORITY: normal
        // Then shutdown
        let result = core.shutdown().await;
        assert!(result.is_ok(), "Shutdown should succeed");

        let state = core.state.read().await;

        // Verify all components are inactive
        for (name, status) in &state.components {
            assert_eq!(
                *status,
                ComponentStatus::Inactive,
                "Component {name} should be inactive after shutdown"
            );
        }

        // Verify health is unhealthy after shutdown
        assert_eq!(
            state.overall_health,
            HealthStatus::Unhealthy,
            "Health should be unhealthy after shutdown"
        );
        drop(state);
    }

    #[tokio::test]
    async fn test_health_check_after_creation() {
        // Verify health check works immediately after creation
        let config = UnifiedBearDogConfig::development();
        let core = BearDogCore::new(config);

        // TEST_CATEGORY: integration
        // TEST_DOMAIN: core
        // TEST_PRIORITY: normal
        let health = core.health_check().await;
        assert!(health.is_ok(), "Health check should succeed");

        let health_result = health.unwrap();
        // Health check returns ComponentStatus for components
        // Just verify we got a valid result
        assert!(
            !health_result.component_name.is_empty(),
            "Health check should return valid component information"
        );
    }

    #[tokio::test]
    async fn test_state_consistency_across_operations() {
        // Verify state remains consistent through lifecycle
        // TEST_CATEGORY: integration
        // TEST_DOMAIN: core
        // TEST_PRIORITY: normal
        let config = UnifiedBearDogConfig::development();
        let core = BearDogCore::new(config);

        // Check initial state
        let state = core.state.read().await;
        assert_eq!(state.overall_health, HealthStatus::Healthy);
        drop(state);

        // Startup
        core.startup().await.expect("Startup should succeed");

        // Check state after startup
        let state = core.state.read().await;
        assert_eq!(state.overall_health, HealthStatus::Healthy);
        assert!(
            !state.components.is_empty(),
            "Should have components after startup"
        );
        drop(state);

        // Shutdown
        core.shutdown().await.expect("Shutdown should succeed");

        // Check state after shutdown
        let state = core.state.read().await;
        assert_eq!(state.overall_health, HealthStatus::Unhealthy);
        drop(state);
    }

    // TEST_CATEGORY: integration
    // TEST_DOMAIN: core
    // TEST_PRIORITY: normal
    #[tokio::test]
    async fn test_concurrent_state_access() {
        // Verify multiple concurrent reads work correctly
        let config = UnifiedBearDogConfig::development();
        let core = std::sync::Arc::new(BearDogCore::new(config));

        let core1 = core.clone();
        let core2 = core.clone();
        let core3 = core.clone();

        let handle1 = tokio::spawn(async move {
            let state = core1.state.read().await;
            state.overall_health
        });

        let handle2 = tokio::spawn(async move {
            let state = core2.state.read().await;
            state.overall_health
        });

        let handle3 = tokio::spawn(async move {
            let state = core3.state.read().await;
            state.overall_health
        });

        let health1 = handle1.await.expect("Task 1 should complete");
        let health2 = handle2.await.expect("Task 2 should complete");
        let health3 = handle3.await.expect("Task 3 should complete");

        // All should read the same initial healthy state
        assert_eq!(health1, HealthStatus::Healthy);
        assert_eq!(health2, HealthStatus::Healthy);
        assert_eq!(health3, HealthStatus::Healthy);
    }
}
