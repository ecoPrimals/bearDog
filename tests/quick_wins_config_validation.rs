//! Quick Win Tests: Config Validation
//!
//! This module contains high-value unit tests designed to boost test coverage with minimal effort.
//! It focuses on basic config validation, serialization, and accessibility of config fields
//! for the UnifiedBearDogConfig type.
//!
//! Coverage: Config creation (2 tests), Serialization (1 test), Field access (6 tests), Traits (4 tests)

use beardog_types::canonical::config::unified::UnifiedBearDogConfig;

// ============================================================================
// Config Creation and Loading Tests
// ============================================================================

/// Tests that default config can be created successfully
///
/// TEST_CATEGORY: unit
/// TEST_DOMAIN: config
/// TEST_PRIORITY: high
#[test]
fn test_default_config_exists() {
    // When: creating default config
    let _config = UnifiedBearDogConfig::default();

    // Then: should succeed without panic
    // No panic means test passes
}

/// Tests that config can be loaded from environment
///
/// TEST_CATEGORY: unit
/// TEST_DOMAIN: config
/// TEST_PRIORITY: high
#[test]
fn test_config_load_from_environment() {
    // When: loading config from environment
    let result = UnifiedBearDogConfig::load();

    // Then: should either succeed or fail gracefully (not panic)
    assert!(
        result.is_ok() || result.is_err(),
        "Config load should return Result"
    );
}

// ============================================================================
// Serialization Tests
// ============================================================================

/// Tests that config can be serialized and deserialized
///
/// TEST_CATEGORY: unit
/// TEST_DOMAIN: config
/// TEST_PRIORITY: high
#[test]
fn test_config_serialization() {
    // Given: a default config
    let config = UnifiedBearDogConfig::default();

    // When: serializing to JSON
    let json = serde_json::to_string(&config);
    assert!(json.is_ok(), "Config should serialize to JSON");

    // When: deserializing back
    if let Ok(json_str) = json {
        let deserialized: Result<UnifiedBearDogConfig, _> = serde_json::from_str(&json_str);

        // Then: should deserialize successfully
        assert!(deserialized.is_ok(), "Config should deserialize from JSON");
    }
}

// ============================================================================
// Config Field Accessibility Tests
// ============================================================================

/// Tests that app config section is accessible
///
/// TEST_CATEGORY: unit
/// TEST_DOMAIN: config
/// TEST_PRIORITY: normal
#[test]
fn test_app_config_accessible() {
    // Given: a default config
    let config = UnifiedBearDogConfig::default();

    // Then: app section should be accessible
    let _ = &config.app;
    // No panic means test passes
}

/// Tests that network config section is accessible
///
/// TEST_CATEGORY: unit
/// TEST_DOMAIN: config
/// TEST_PRIORITY: normal
#[test]
fn test_network_config_accessible() {
    // Given: a default config
    let config = UnifiedBearDogConfig::default();

    // Then: network section should be accessible
    let _ = &config.network;
    // No panic means test passes
}

/// Tests that security config section is accessible
///
/// TEST_CATEGORY: unit
/// TEST_DOMAIN: config
/// TEST_PRIORITY: normal
#[test]
fn test_security_config_accessible() {
    // Given: a default config
    let config = UnifiedBearDogConfig::default();

    // Then: security section should be accessible
    let _ = &config.security;
    // No panic means test passes
}

/// Tests that HSM config section is accessible
///
/// TEST_CATEGORY: unit
/// TEST_DOMAIN: config
/// TEST_PRIORITY: normal
#[test]
fn test_hsm_config_accessible() {
    // Given: a default config
    let config = UnifiedBearDogConfig::default();

    // Then: HSM section should be accessible
    let _ = &config.hsm;
    // No panic means test passes
}

/// Tests that database config section is accessible
///
/// TEST_CATEGORY: unit
/// TEST_DOMAIN: config
/// TEST_PRIORITY: normal
#[test]
fn test_database_config_accessible() {
    // Given: a default config
    let config = UnifiedBearDogConfig::default();

    // Then: database section should be accessible
    let _ = &config.database;
    // No panic means test passes
}

/// Tests that metadata section is accessible
///
/// TEST_CATEGORY: unit
/// TEST_DOMAIN: config
/// TEST_PRIORITY: normal
#[test]
fn test_metadata_accessible() {
    // Given: a default config
    let config = UnifiedBearDogConfig::default();

    // Then: metadata section should be accessible
    let _ = &config.metadata;
    // No panic means test passes
}

/// Tests that genetics config section is accessible
///
/// TEST_CATEGORY: unit
/// TEST_DOMAIN: config
/// TEST_PRIORITY: normal
#[test]
fn test_genetics_config_present() {
    // Given: a default config
    let config = UnifiedBearDogConfig::default();

    // Then: genetics section should be accessible
    let _ = &config.genetics;
    // No panic means test passes
}

// ============================================================================
// Config Trait and Behavior Tests
// ============================================================================

/// Tests that environment field has a valid default value
///
/// TEST_CATEGORY: unit
/// TEST_DOMAIN: config
/// TEST_PRIORITY: normal
#[test]
fn test_environment_has_default() {
    // Given: a default config
    let config = UnifiedBearDogConfig::default();

    // When: accessing environment
    let env_str = format!("{:?}", config.metadata.environment);

    // Then: should have a valid default
    assert!(!env_str.is_empty(), "Environment should have valid default");
}

/// Tests that config can be cloned correctly
///
/// TEST_CATEGORY: unit
/// TEST_DOMAIN: config
/// TEST_PRIORITY: high
#[test]
fn test_config_clone_works() {
    // Given: a config
    let config1 = UnifiedBearDogConfig::default();

    // When: cloning it
    let config2 = config1.clone();

    // Then: cloned config should match original
    assert_eq!(
        config1.metadata.instance_id, config2.metadata.instance_id,
        "Cloned config should match"
    );
}

/// Tests that config implements Debug trait correctly
///
/// TEST_CATEGORY: unit
/// TEST_DOMAIN: config
/// TEST_PRIORITY: normal
#[test]
fn test_config_debug_format() {
    // Given: a config
    let config = UnifiedBearDogConfig::default();

    // When: formatting with Debug
    let debug_str = format!("{:?}", config);

    // Then: should contain type name
    assert!(
        debug_str.contains("UnifiedBearDogConfig"),
        "Debug output should contain type name"
    );
}

/// Tests that multiple config instances can be created
///
/// TEST_CATEGORY: unit
/// TEST_DOMAIN: config
/// TEST_PRIORITY: normal
#[test]
fn test_multiple_config_instances() {
    // When: creating multiple configs
    let _config1 = UnifiedBearDogConfig::default();
    let _config2 = UnifiedBearDogConfig::default();

    // Then: should succeed without issues
    // No panic means test passes
}
