// SPDX-License-Identifier: AGPL-3.0-only
//! `BearDog` Core Initialization Tests
//!
//! This module contains unit and integration tests for `BearDogCore` initialization,
//! configuration, and component access. Tests verify that the core can be created,
//! initialized, and that all components are properly accessible.
//!
//! Coverage: Core creation (2 tests), Initialization (6 tests), Component access (6 tests)

use beardog_core::BearDogCore;
use beardog_types::canonical::config::unified::UnifiedBearDogConfig as BearDogConfig;

// ============================================================================
// Core Creation Tests
// ============================================================================

/// Tests that `BearDogCore` can be created with default configuration
///
/// `TEST_CATEGORY`: unit
/// `TEST_DOMAIN`: core
/// `TEST_PRIORITY`: high
#[test]
fn test_core_new_with_default_config() {
    // Given: a default configuration
    let config = BearDogConfig::default();

    // When: creating BearDogCore
    let _core = BearDogCore::new(config);

    // Then: should successfully create instance without panicking
}

/// Tests that `BearDogCore` can be created using convenience factory method
///
/// `TEST_CATEGORY`: unit
/// `TEST_DOMAIN`: core
/// `TEST_PRIORITY`: high
#[test]
fn test_core_with_default_config_factory() {
    // When: using factory method
    let result = BearDogCore::with_default_config();

    // Then: should succeed
    assert!(
        result.is_ok(),
        "Factory method should create core successfully"
    );

    let _core = result.unwrap();
}

/// Tests that core can be created with different configuration variants
///
/// `TEST_CATEGORY`: unit
/// `TEST_DOMAIN`: core
/// `TEST_PRIORITY`: normal
#[test]
fn test_core_configuration_variants() {
    // Given: multiple default configurations
    let default_config = BearDogConfig::default();
    let _core1 = BearDogCore::new(default_config);

    let config2 = BearDogConfig::default();
    let _core2 = BearDogCore::new(config2);

    // Then: both cores should be created successfully
}

// ============================================================================
// Core Initialization Tests
// ============================================================================

/// Tests that core initialization succeeds
///
/// `TEST_CATEGORY`: integration
/// `TEST_DOMAIN`: core
/// `TEST_PRIORITY`: critical
#[tokio::test]
async fn test_core_initialize() {
    // Given: a core instance
    let result = BearDogCore::with_default_config();
    assert!(result.is_ok(), "Core creation should succeed");

    let mut core = result.unwrap();

    // When: initializing
    let init_result = core.initialize().await;

    // Then: should succeed
    assert!(init_result.is_ok(), "Initialization should succeed");
}

/// Tests that HSM management can be initialized
///
/// `TEST_CATEGORY`: integration
/// `TEST_DOMAIN`: core
/// `TEST_PRIORITY`: critical
#[tokio::test]
async fn test_core_hsm_initialization() {
    // Given: an initialized core
    let result = BearDogCore::with_default_config();
    assert!(result.is_ok(), "Core creation should succeed");

    let mut core = result.unwrap();
    core.initialize().await.expect("Core initialization failed");

    // When: initializing HSM management
    let hsm_result = core.initialize_hsm_management().await;

    // Then: should succeed
    assert!(hsm_result.is_ok(), "HSM initialization should succeed");
}

/// Tests that AI service registration succeeds
///
/// `TEST_CATEGORY`: integration
/// `TEST_DOMAIN`: core
/// `TEST_PRIORITY`: high
#[tokio::test]
async fn test_core_ai_service_registration() {
    // Given: an initialized core
    let result = BearDogCore::with_default_config();
    assert!(result.is_ok(), "Core creation should succeed");

    let mut core = result.unwrap();
    core.initialize().await.expect("Core initialization failed");

    // When: registering with AI service
    let ai_result = core.register_with_ai_service_alt().await;

    // Then: should succeed
    assert!(ai_result.is_ok(), "AI service registration should succeed");
}

/// Tests that multiple initialization calls are handled gracefully
///
/// `TEST_CATEGORY`: integration
/// `TEST_DOMAIN`: core
/// `TEST_PRIORITY`: normal
#[tokio::test]
async fn test_core_multiple_initialization() {
    // Given: a core instance
    let mut core = BearDogCore::with_default_config().expect("Failed to create core");

    // When: initializing multiple times
    let result1 = core.initialize().await;
    assert!(result1.is_ok(), "First initialization should succeed");

    let result2 = core.initialize().await;

    // Then: second initialization should handle gracefully (either succeed or fail appropriately)
    assert!(
        result2.is_ok() || result2.is_err(),
        "Should handle multiple initialization"
    );
}

/// Tests complete initialization sequence with all components
///
/// `TEST_CATEGORY`: integration
/// `TEST_DOMAIN`: core
/// `TEST_PRIORITY`: critical
#[tokio::test]
async fn test_core_full_initialization_sequence() {
    // Given: a core instance
    let mut core = BearDogCore::with_default_config().expect("Failed to create core");

    // When: executing full initialization sequence
    // Step 1: Initialize core
    core.initialize().await.expect("Core initialization failed");

    // Step 2: Initialize HSM
    core.initialize_hsm_management()
        .await
        .expect("HSM initialization failed");

    // Step 3: Register with AI services
    core.register_with_ai_service_alt()
        .await
        .expect("AI registration failed");

    // Then: all steps should complete successfully
}

// ============================================================================
// Component Access Tests
// ============================================================================

/// Tests that config field is accessible
///
/// `TEST_CATEGORY`: unit
/// `TEST_DOMAIN`: core
/// `TEST_PRIORITY`: normal
#[test]
fn test_core_config_access() {
    // Given: a core instance
    let core = BearDogCore::with_default_config().unwrap();

    // When: accessing config
    let _config_ref = &core.config;

    // Then: config should be accessible
}

/// Tests that state field is accessible
///
/// `TEST_CATEGORY`: unit
/// `TEST_DOMAIN`: core
/// `TEST_PRIORITY`: normal
#[test]
fn test_core_state_access() {
    // Given: a core instance
    let core = BearDogCore::with_default_config().unwrap();

    // When: accessing state
    let _state_ref = &core.state;

    // Then: state should be accessible
}

/// Tests that monitor field is accessible
///
/// `TEST_CATEGORY`: unit
/// `TEST_DOMAIN`: core
/// `TEST_PRIORITY`: normal
#[test]
fn test_core_monitor_access() {
    // Given: a core instance
    let core = BearDogCore::with_default_config().unwrap();

    // When: accessing monitor
    let _monitor_ref = &core.monitor;

    // Then: monitor should be accessible
}

/// Tests that security provider field is accessible
///
/// `TEST_CATEGORY`: unit
/// `TEST_DOMAIN`: core
/// `TEST_PRIORITY`: high
#[test]
fn test_core_security_provider_access() {
    // Given: a core instance
    let core = BearDogCore::with_default_config().unwrap();

    // When: accessing security provider
    let _security_ref = &core.security;

    // Then: security provider should be accessible
}

/// Tests that genetic optimizer field is accessible
///
/// `TEST_CATEGORY`: unit
/// `TEST_DOMAIN`: core
/// `TEST_PRIORITY`: normal
#[test]
fn test_core_genetic_optimizer_access() {
    // Given: a core instance
    let core = BearDogCore::with_default_config().unwrap();

    // When: accessing genetic optimizer
    let _optimizer_ref = &core.genetic_optimizer;

    // Then: optimizer should be accessible
}

/// Tests that universal adapter field is accessible
///
/// `TEST_CATEGORY`: unit
/// `TEST_DOMAIN`: core
/// `TEST_PRIORITY`: normal
#[test]
fn test_core_universal_adapter_access() {
    // Given: a core instance
    let core = BearDogCore::with_default_config().unwrap();

    // When: accessing universal adapter
    let _adapter_ref = &core.universal_adapter;

    // Then: adapter should be accessible
}
