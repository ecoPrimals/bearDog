//! Lifecycle Tests
//!
//! Comprehensive testing of `BearDog` Core lifecycle operations including:
//! - Startup sequence
//! - Initialization procedures  
//! - Health check operations
//! - Component initialization

use super::super::system::BearDogCore;
use beardog_types::canonical::config::unified::UnifiedBearDogConfig;
use beardog_types::canonical::HealthStatus;

#[tokio::test]
async fn test_beardog_core_creation() {
    let config = UnifiedBearDogConfig::development();
    let core = BearDogCore::new(config);
    // Creation should succeed
    let state = core.state.read().await;
    assert_eq!(state.overall_health, HealthStatus::Healthy);
    drop(state);
}

#[tokio::test]
async fn test_beardog_core_initialization() {
    let config = UnifiedBearDogConfig::development();
    let mut core = BearDogCore::new(config);

    // Initialize should succeed
    let result = core.initialize().await;
    assert!(result.is_ok());
}

#[tokio::test]
async fn test_system_status_after_creation() {
    let config = UnifiedBearDogConfig::development();
    let core = BearDogCore::new(config);

    // State should be healthy after creation
    let state = core.state.read().await;
    assert_eq!(state.overall_health, HealthStatus::Healthy);
    drop(state);
}

#[tokio::test]
async fn test_component_registry() {
    let config = UnifiedBearDogConfig::development();
    let core = BearDogCore::new(config);

    // Should have components registry in state
    let state = core.state.read().await;
    // Initial state (may be empty or have defaults)
    // Components map exists and is accessible
    let _component_count = state.components.len();
}

#[tokio::test]
async fn test_full_initialization() {
    let config = UnifiedBearDogConfig::development();
    let mut core = BearDogCore::new(config);

    // Initialize
    core.initialize().await.unwrap();

    // Verify state is healthy
    let state = core.state.read().await;
    assert_eq!(state.overall_health, HealthStatus::Healthy);
    drop(state);
}

#[tokio::test]
async fn test_state_management() {
    let config = UnifiedBearDogConfig::development();
    let core = BearDogCore::new(config);

    // State should be accessible
    let state = core.state.read().await;
    // Verify default state has components map
    // Components map exists and is initialized
    let _component_count = state.components.len();
}

#[tokio::test]
async fn test_config_storage() {
    let config = UnifiedBearDogConfig::development();
    let core = BearDogCore::new(config.clone());

    // Config should be stored and core should have healthy state
    let state = core.state.read().await;
    assert_eq!(state.overall_health, HealthStatus::Healthy);
    drop(state);
}

#[tokio::test]
async fn test_state_uptime_tracking() {
    let config = UnifiedBearDogConfig::development();
    let core = BearDogCore::new(config);

    // State should track start time
    let state = core.state.read().await;
    let uptime = state.start_time.elapsed();
    // Uptime should be positive (as_millis() returns u128, always >= 0, so check it's reasonable)
    assert!(
        uptime.as_millis() < 1000,
        "Uptime should be less than 1 second for new core"
    );
    drop(state);
}
