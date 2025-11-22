//! Health Monitoring Comprehensive Tests
//!
//! Tests for system health tracking and monitoring

use crate::core::system::BearDogCore;
use beardog_types::canonical::config::unified::UnifiedBearDogConfig;
use beardog_types::canonical::HealthStatus;

#[tokio::test]
async fn test_initial_health_status() {
    let core = BearDogCore::with_default_config().unwrap();
    let state = core.state.read().await;

    assert_eq!(
        state.overall_health,
        HealthStatus::Healthy,
        "Initial health should be Healthy"
    );
    drop(state);
}

#[tokio::test]
async fn test_health_status_after_init() {
    let mut core = BearDogCore::with_default_config().unwrap();
    core.initialize().await.unwrap();

    let state = core.state.read().await;
    assert_eq!(
        state.overall_health,
        HealthStatus::Healthy,
        "Health should remain Healthy after initialization"
    );
    // TEST_CATEGORY: integration
    // TEST_DOMAIN: core
    // TEST_PRIORITY: normal
    drop(state);
}

#[tokio::test]
async fn test_development_config_health() {
    let config = UnifiedBearDogConfig::development();
    let core = BearDogCore::new(config);

    let state = core.state.read().await;
    assert_eq!(
        // TEST_CATEGORY: integration
        // TEST_DOMAIN: core
        // TEST_PRIORITY: normal
        state.overall_health,
        HealthStatus::Healthy,
        "Development config should have Healthy status"
    );
    drop(state);
}

#[tokio::test]
async fn test_production_config_health() {
    let config = UnifiedBearDogConfig::production();
    let core = BearDogCore::new(config);
    // TEST_CATEGORY: integration
    // TEST_DOMAIN: core
    // TEST_PRIORITY: normal

    let state = core.state.read().await;
    assert_eq!(
        state.overall_health,
        HealthStatus::Healthy,
        "Production config should have Healthy status"
    );
    drop(state);
}

#[tokio::test]
// TEST_CATEGORY: integration
// TEST_DOMAIN: core
// TEST_PRIORITY: normal
async fn test_components_registry_accessible() {
    let core = BearDogCore::with_default_config().unwrap();
    let state = core.state.read().await;

    // Components registry should be accessible
    let _components = &state.components;
    // Verify components registry is accessible
}

#[tokio::test]
async fn test_multiple_health_checks() {
    // TEST_CATEGORY: integration
    // TEST_DOMAIN: core
    // TEST_PRIORITY: normal
    let core = BearDogCore::with_default_config().unwrap();

    // Check health multiple times
    for _ in 0..5 {
        let state = core.state.read().await;
        assert_eq!(state.overall_health, HealthStatus::Healthy);
        drop(state); // Release lock
                     // TEST_CATEGORY: integration
                     // TEST_DOMAIN: core
                     // TEST_PRIORITY: normal
    }
}

#[tokio::test]
async fn test_concurrent_health_checks() {
    let core = std::sync::Arc::new(BearDogCore::with_default_config().unwrap());

    let mut handles = vec![];

    // TEST_CATEGORY: integration
    // TEST_DOMAIN: core
    // TEST_PRIORITY: normal
    for _ in 0..10 {
        let core_clone = std::sync::Arc::clone(&core);
        let handle = tokio::spawn(async move {
            let state = core_clone.state.read().await;
            assert_eq!(state.overall_health, HealthStatus::Healthy);
            drop(state);
        });
        handles.push(handle);
    }

    // Wait for all checks to complete
    for handle in handles {
        handle.await.unwrap();
    }
}

#[tokio::test]
async fn test_health_status_consistency() {
    let core = BearDogCore::with_default_config().unwrap();
    // TEST_CATEGORY: integration
    // TEST_DOMAIN: core
    // TEST_PRIORITY: normal

    let state1 = core.state.read().await;
    let health1 = state1.overall_health;
    drop(state1);

    let state2 = core.state.read().await;
    let health2 = state2.overall_health;
    drop(state2);

    assert_eq!(health1, health2, "Health status should be consistent");
}

// TEST_CATEGORY: integration
// TEST_DOMAIN: core
// TEST_PRIORITY: normal
#[tokio::test]
async fn test_initialized_core_health() {
    let config = UnifiedBearDogConfig::default();
    let mut core = BearDogCore::new(config);

    // Before initialization
    let state_before = core.state.read().await;
    let health_before = state_before.overall_health;
    drop(state_before);

    // Initialize
    core.initialize().await.unwrap();

    // After initialization
    let state_after = core.state.read().await;
    let health_after = state_after.overall_health;

    assert_eq!(
        health_before, health_after,
        "Health should remain consistent through initialization"
    );
    drop(state_after);
}
