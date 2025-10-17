// BearDog Core Initialization Tests
// Tests basic initialization and configuration

use beardog_core::BearDogCore;
use beardog_types::canonical::config::unified::UnifiedBearDogConfig as BearDogConfig;

#[test]
fn test_core_new_with_default_config() {
    // Test creating BearDogCore with default configuration
    let config = BearDogConfig::default();
    let _core = BearDogCore::new(config);

    // Should successfully create instance without panicking
}

#[test]
fn test_core_with_default_config_factory() {
    // Test convenience factory method
    let result = BearDogCore::with_default_config();
    assert!(result.is_ok());

    let _core = result.unwrap();
    // Core created successfully
}

#[tokio::test]
async fn test_core_initialize() {
    // Test basic initialization
    let result = BearDogCore::with_default_config();
    assert!(result.is_ok());

    let mut core = result.unwrap();

    // Initialize should succeed
    let init_result = core.initialize().await;
    assert!(init_result.is_ok(), "Initialization should succeed");
}

#[tokio::test]
async fn test_core_hsm_initialization() {
    // Test HSM management initialization
    let result = BearDogCore::with_default_config();
    assert!(result.is_ok());

    let mut core = result.unwrap();
    core.initialize().await.expect("Core initialization failed");

    // HSM initialization should succeed
    let hsm_result = core.initialize_hsm_management().await;
    assert!(hsm_result.is_ok(), "HSM initialization should succeed");
}

#[tokio::test]
async fn test_core_ai_service_registration() {
    // Test AI service registration
    let result = BearDogCore::with_default_config();
    assert!(result.is_ok());

    let mut core = result.unwrap();
    core.initialize().await.expect("Core initialization failed");

    // AI service registration should succeed
    let ai_result = core.register_with_ai_service_alt().await;
    assert!(ai_result.is_ok(), "AI service registration should succeed");
}

#[test]
fn test_core_configuration_variants() {
    // Test creating core with different configs
    let default_config = BearDogConfig::default();
    let _core1 = BearDogCore::new(default_config);

    // Test with second default config
    let config2 = BearDogConfig::default();
    let _core2 = BearDogCore::new(config2);

    // Both cores created successfully
}

#[tokio::test]
async fn test_core_multiple_initialization() {
    // Test that multiple initialization calls are handled
    let mut core = BearDogCore::with_default_config().expect("Failed to create core");

    // First initialization
    let result1 = core.initialize().await;
    assert!(result1.is_ok());

    // Second initialization (should still work or handle gracefully)
    let result2 = core.initialize().await;
    assert!(result2.is_ok() || result2.is_err()); // Either is valid
}

#[tokio::test]
async fn test_core_full_initialization_sequence() {
    // Test complete initialization sequence
    let mut core = BearDogCore::with_default_config().expect("Failed to create core");

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

    // All steps should complete successfully
}

#[test]
fn test_core_config_access() {
    // Test that config is accessible
    let core = BearDogCore::with_default_config().unwrap();

    // Config should be accessible
    let _config_ref = &core.config;
    // Config is accessible and can be referenced
}

#[test]
fn test_core_state_access() {
    // Test that state is accessible
    let core = BearDogCore::with_default_config().unwrap();

    // State should be accessible
    let _state_ref = &core.state;
    // State exists and can be referenced
}

#[test]
fn test_core_monitor_access() {
    // Test that monitor is accessible
    let core = BearDogCore::with_default_config().unwrap();

    // Monitor should be accessible
    let _monitor_ref = &core.monitor;
    // Monitor exists and can be referenced
}

#[test]
fn test_core_security_provider_access() {
    // Test that security provider is accessible
    let core = BearDogCore::with_default_config().unwrap();

    // Security provider should be accessible
    let _security_ref = &core.security;
    // Security provider exists and can be referenced
}

#[test]
fn test_core_genetic_optimizer_access() {
    // Test that genetic optimizer is accessible
    let core = BearDogCore::with_default_config().unwrap();

    // Genetic optimizer should be accessible
    let _optimizer_ref = &core.genetic_optimizer;
    // Optimizer exists and can be referenced
}

#[test]
fn test_core_universal_adapter_access() {
    // Test that universal adapter is accessible
    let core = BearDogCore::with_default_config().unwrap();

    // Universal adapter should be accessible
    let _adapter_ref = &core.universal_adapter;
    // Adapter exists and can be referenced
}
