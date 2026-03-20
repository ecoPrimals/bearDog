// SPDX-License-Identifier: AGPL-3.0-only
//! Configuration Validation Tests
//!
//! This module contains unit tests for `BearDog` configuration validation,
//! including default values, custom settings, and cloning behavior.
//! Also includes tests for `HealthStatus` and `ServiceCapabilityType` enums.

#![allow(clippy::unwrap_used, clippy::expect_used)]
#![allow(
    clippy::field_reassign_with_default,
    clippy::needless_borrows_for_generic_args
)]

use beardog_core::BearDogConfig;
use beardog_types::canonical::HealthStatus;
use beardog_types::canonical::capabilities::ServiceCapabilityType;

// ============================================================================
// Configuration Default Tests
// ============================================================================

/// Tests that `BearDogConfig::default()` creates a valid configuration with non-empty fields
///
/// `TEST_CATEGORY`: unit
/// `TEST_DOMAIN`: config
/// `TEST_PRIORITY`: high
#[test]
fn test_config_default() {
    // Given: nothing (testing default constructor)

    // When: creating a default config
    let config = BearDogConfig::default();

    // Then: all required fields should be populated
    assert!(
        !config.environment.is_empty(),
        "environment should not be empty"
    );
    assert!(!config.version.is_empty(), "version should not be empty");
    assert!(!config.node_id.is_empty(), "node_id should not be empty");
}

/// Tests that the default environment is set to "development"
///
/// `TEST_CATEGORY`: unit
/// `TEST_DOMAIN`: config
/// `TEST_PRIORITY`: high
#[test]
fn test_config_environment_default() {
    // When: creating a default config
    let config = BearDogConfig::default();

    // Then: environment should be "development"
    assert_eq!(config.environment, "development");
}

/// Tests that the default version is "3.0.0"
///
/// `TEST_CATEGORY`: unit
/// `TEST_DOMAIN`: config
/// `TEST_PRIORITY`: high
#[test]
fn test_config_version_default() {
    // When: creating a default config
    let config = BearDogConfig::default();

    // Then: version should be "3.0.0"
    assert_eq!(config.version, "3.0.0");
}

/// Tests that each config instance gets a unique `node_id`
///
/// `TEST_CATEGORY`: unit
/// `TEST_DOMAIN`: config
/// `TEST_PRIORITY`: high
#[test]
fn test_config_node_id_unique() {
    // When: creating two separate config instances
    let config1 = BearDogConfig::default();
    let config2 = BearDogConfig::default();

    // Then: their node_ids should be different
    assert_ne!(
        config1.node_id, config2.node_id,
        "node_ids should be unique per instance"
    );
}

/// Tests that the `node_id` follows UUID v4 format (36 chars with dashes)
///
/// `TEST_CATEGORY`: unit
/// `TEST_DOMAIN`: config
/// `TEST_PRIORITY`: normal
#[test]
fn test_config_node_id_format() {
    // When: creating a default config
    let config = BearDogConfig::default();

    // Then: node_id should be a valid UUID format
    assert!(config.node_id.contains('-'), "UUID should contain dashes");
    assert_eq!(config.node_id.len(), 36, "UUID should be 36 characters");
}

// ============================================================================
// Configuration Customization Tests
// ============================================================================

/// Tests that environment can be set to a custom value
///
/// `TEST_CATEGORY`: unit
/// `TEST_DOMAIN`: config
/// `TEST_PRIORITY`: normal
#[test]
fn test_config_custom_environment() {
    // Given: a default config
    let mut config = BearDogConfig::default();

    // When: setting a custom environment
    config.environment = "production".to_string();

    // Then: environment should reflect the new value
    assert_eq!(config.environment, "production");
}

/// Tests that version can be set to a custom value
///
/// `TEST_CATEGORY`: unit
/// `TEST_DOMAIN`: config
/// `TEST_PRIORITY`: normal
#[test]
fn test_config_custom_version() {
    // Given: a default config
    let mut config = BearDogConfig::default();

    // When: setting a custom version
    config.version = "4.0.0".to_string();

    // Then: version should reflect the new value
    assert_eq!(config.version, "4.0.0");
}

/// Tests that `BearDogConfig` can be cloned and maintains all field values
///
/// `TEST_CATEGORY`: unit
/// `TEST_DOMAIN`: config
/// `TEST_PRIORITY`: normal
#[test]
fn test_config_clone() {
    // Given: a default config
    let config1 = BearDogConfig::default();

    // When: cloning the config
    let config2 = config1.clone();

    // Then: all fields should be identical
    assert_eq!(
        config1.environment, config2.environment,
        "environment should match"
    );
    assert_eq!(config1.version, config2.version, "version should match");
    assert_eq!(config1.node_id, config2.node_id, "node_id should match");
}

// ============================================================================
// HealthStatus Enum Tests
// ============================================================================

/// Tests that `HealthStatus::Healthy` can be instantiated and compared
///
/// `TEST_CATEGORY`: unit
/// `TEST_DOMAIN`: types
/// `TEST_PRIORITY`: normal
#[test]
fn test_health_status_healthy() {
    let status = HealthStatus::Healthy;
    assert_eq!(status, HealthStatus::Healthy);
}

/// Tests that `HealthStatus::Degraded` can be instantiated and compared
///
/// `TEST_CATEGORY`: unit
/// `TEST_DOMAIN`: types
/// `TEST_PRIORITY`: normal
#[test]
fn test_health_status_degraded() {
    let status = HealthStatus::Degraded;
    assert_eq!(status, HealthStatus::Degraded);
}

/// Tests that `HealthStatus::Unhealthy` can be instantiated and compared
///
/// `TEST_CATEGORY`: unit
/// `TEST_DOMAIN`: types
/// `TEST_PRIORITY`: normal
#[test]
fn test_health_status_unhealthy() {
    let status = HealthStatus::Unhealthy;
    assert_eq!(status, HealthStatus::Unhealthy);
}

// ============================================================================
// ServiceCapabilityType Enum Tests
// ============================================================================

/// Tests that `ServiceCapabilityType::KeyManagement` can be instantiated and compared
///
/// `TEST_CATEGORY`: unit
/// `TEST_DOMAIN`: capabilities
/// `TEST_PRIORITY`: normal
#[test]
fn test_service_capability_key_management() {
    let cap = ServiceCapabilityType::KeyManagement;
    assert_eq!(cap, ServiceCapabilityType::KeyManagement);
}

/// Tests that `ServiceCapabilityType::HardwareSecurityModule` can be instantiated and compared
///
/// `TEST_CATEGORY`: unit
/// `TEST_DOMAIN`: capabilities
/// `TEST_PRIORITY`: normal
#[test]
fn test_service_capability_hsm() {
    let cap = ServiceCapabilityType::HardwareSecurityModule;
    assert_eq!(cap, ServiceCapabilityType::HardwareSecurityModule);
}

/// Tests that `ServiceCapabilityType::DatabaseService` can be instantiated and compared
///
/// `TEST_CATEGORY`: unit
/// `TEST_DOMAIN`: capabilities
/// `TEST_PRIORITY`: normal
#[test]
fn test_service_capability_database() {
    let cap = ServiceCapabilityType::DatabaseService;
    assert_eq!(cap, ServiceCapabilityType::DatabaseService);
}

/// Tests that `ServiceCapabilityType::Authentication` can be instantiated and compared
///
/// `TEST_CATEGORY`: unit
/// `TEST_DOMAIN`: capabilities
/// `TEST_PRIORITY`: normal
#[test]
fn test_service_capability_authentication() {
    let cap = ServiceCapabilityType::Authentication;
    assert_eq!(cap, ServiceCapabilityType::Authentication);
}

/// Tests that `ServiceCapabilityType::CloudStorage` can be instantiated and compared
///
/// `TEST_CATEGORY`: unit
/// `TEST_DOMAIN`: capabilities
/// `TEST_PRIORITY`: normal
#[test]
fn test_service_capability_cloud_storage() {
    let cap = ServiceCapabilityType::CloudStorage;
    assert_eq!(cap, ServiceCapabilityType::CloudStorage);
}
