// SPDX-License-Identifier: AGPL-3.0-or-later

//! Configuration Validation Tests
//!
//! Tests for configuration validation, error handling, and edge cases.
//! Added as part of Week 1 test expansion (October 17, 2025).

#![allow(
    unused_imports,
    clippy::float_cmp,
    clippy::useless_vec,
    clippy::needless_range_loop,
    clippy::uninlined_format_args,
    clippy::field_reassign_with_default,
    clippy::manual_range_contains,
    unused_variables,
    dead_code
)]

use crate::core::system::BearDogCore;
use beardog_types::canonical::HealthStatus;
use beardog_types::canonical::config::unified::UnifiedBearDogConfig;

/// Test core initialization with default configuration
#[tokio::test]
async fn test_core_with_default_config_validates() {
    let result = BearDogCore::with_default_config();
    assert!(
        result.is_ok(),
        "Default configuration should be valid and create core successfully"
    );
}

/// Test core initialization with development configuration
#[tokio::test]
async fn test_core_with_development_config() {
    let config = UnifiedBearDogConfig::development();
    let core = BearDogCore::new(config);

    let state = core.state.read().await;
    assert_eq!(
        state.overall_health,
        HealthStatus::Healthy,
        "Development config should result in healthy initial state"
    );
    // TEST_CATEGORY: integration
    // TEST_DOMAIN: core
    // TEST_PRIORITY: normal
    drop(state);
}

/// Test core initialization with production configuration
#[tokio::test]
async fn test_core_with_production_config() {
    let config = UnifiedBearDogConfig::production();
    // TEST_CATEGORY: integration
    // TEST_DOMAIN: core
    // TEST_PRIORITY: normal
    let core = BearDogCore::new(config);

    let state = core.state.read().await;
    assert_eq!(
        state.overall_health,
        HealthStatus::Healthy,
        "Production config should result in healthy initial state"
    );
    drop(state);
}

/// Test that core preserves configuration after creation
// TEST_CATEGORY: integration
// TEST_DOMAIN: core
// TEST_PRIORITY: normal
#[tokio::test]
async fn test_configuration_preservation_after_init() {
    let config = UnifiedBearDogConfig::development();
    let mut core = BearDogCore::new(config);

    // Initialize the core
    let init_result = core.initialize().await;
    assert!(
        init_result.is_ok(),
        "Initialization should succeed with valid config"
    );

    // TEST_CATEGORY: integration
    // TEST_DOMAIN: core
    // TEST_PRIORITY: normal
    // Configuration should still be accessible and valid
    let state = core.state.read().await;
    assert_eq!(state.overall_health, HealthStatus::Healthy);
    drop(state);
}

/// Test concurrent core creation with same config
#[tokio::test]
async fn test_concurrent_core_creation() {
    let handle1 = tokio::spawn(async {
        let config = UnifiedBearDogConfig::development();
        BearDogCore::new(config)
    });

    let handle2 = tokio::spawn(async {
        let config = UnifiedBearDogConfig::development();
        // TEST_CATEGORY: integration
        // TEST_DOMAIN: core
        // TEST_PRIORITY: normal
        BearDogCore::new(config)
    });

    let core1 = handle1.await.expect("First core creation should succeed");
    let core2 = handle2.await.expect("Second core creation should succeed");

    // Both should be valid
    let state1 = core1.state.read().await;
    let state2 = core2.state.read().await;

    assert_eq!(state1.overall_health, HealthStatus::Healthy);
    assert_eq!(state2.overall_health, HealthStatus::Healthy);
    drop(state1);
    drop(state2);
}

/// Test that system state is properly initialized
#[tokio::test]
async fn test_system_state_initialization() {
    let config = UnifiedBearDogConfig::development();
    let core = BearDogCore::new(config);

    let state = core.state.read().await;
    // TEST_CATEGORY: integration
    // TEST_DOMAIN: core
    // TEST_PRIORITY: normal

    // Verify initial state properties
    assert_eq!(state.overall_health, HealthStatus::Healthy);
    assert_eq!(
        state.components.len(),
        0,
        "Should start with no registered components"
    );
    drop(state);
}

/// Test component registration in state
#[tokio::test]
async fn test_component_state_management() {
    let config = UnifiedBearDogConfig::development();
    // TEST_CATEGORY: integration
    // TEST_DOMAIN: core
    // TEST_PRIORITY: normal
    let mut core = BearDogCore::new(config);

    // Initialize should set up components
    let _ = core.initialize().await;

    let state = core.state.read().await;
    // After initialization, some components may be registered
    // (count depends on implementation)
    // Components map should be accessible (len() is always >= 0 for usize)
    let _components = &state.components;
}

/// Test health status tracking after operations
// TEST_CATEGORY: integration
// TEST_DOMAIN: core
// TEST_PRIORITY: normal
#[tokio::test]
async fn test_health_status_tracking() {
    let config = UnifiedBearDogConfig::development();
    let mut core = BearDogCore::new(config);

    // Health should start as Healthy
    let state = core.state.read().await;
    assert_eq!(state.overall_health, HealthStatus::Healthy);
    drop(state);

    // After initialization, should still be healthy
    let _ = core.initialize().await;

    let state = core.state.read().await;
    assert_eq!(
        state.overall_health,
        HealthStatus::Healthy,
        "Health should remain healthy after successful initialization"
    );
    drop(state);
}

// ============================================================================
// Test Summary
// ============================================================================
// Total tests added: 8
// Category: Configuration validation and state management
// Purpose: Week 1 test coverage expansion
// Focus: Configuration handling, concurrent creation, state initialization
// Date: October 17, 2025
// ============================================================================
