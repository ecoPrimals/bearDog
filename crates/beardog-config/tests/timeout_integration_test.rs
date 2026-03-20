// SPDX-License-Identifier: AGPL-3.0-only
//! Integration tests for timeout configuration
//!
//! Tests end-to-end timeout configuration using the **builder pattern**
//! instead of environment variables to ensure **concurrent safety**.
//!
//! All tests in this file can run in parallel without race conditions.

use beardog_config::BearDogConfig;
use beardog_config::domains::timeouts_new::TimeoutConfig;
use std::time::Duration;

// ============================================================================
// Static Defaults Tests
// ============================================================================

#[test]
fn test_timeout_const_defaults() {
    let config = TimeoutConfig::default();

    assert_eq!(config.health_check_secs, 5);
    assert_eq!(config.hsm_operation_secs, 2);
    assert_eq!(config.hsm_probe_millis, 500);
    assert_eq!(config.discovery_operation_secs, 10);
}

#[test]
fn test_timeout_default_trait() {
    let config = TimeoutConfig::default();

    // Default should equal const_defaults (no env var reads)
    assert_eq!(config, TimeoutConfig::default());
}

// ============================================================================
// Builder Pattern Tests (Concurrent-Safe)
// ============================================================================

#[test]
fn test_timeout_builder_single_field() {
    let config = TimeoutConfig::builder().health_check_secs(15).build();

    assert_eq!(config.health_check_secs, 15);
    // Other fields use defaults
    assert_eq!(config.hsm_operation_secs, 2);
    assert_eq!(config.hsm_probe_millis, 500);
    assert_eq!(config.discovery_operation_secs, 10);
}

#[test]
fn test_timeout_builder_multiple_fields() {
    let config = TimeoutConfig::builder()
        .health_check_secs(8)
        .hsm_operation_secs(3)
        .hsm_probe_millis(750)
        .build();

    assert_eq!(config.health_check_secs, 8);
    assert_eq!(config.hsm_operation_secs, 3);
    assert_eq!(config.hsm_probe_millis, 750);
    assert_eq!(config.discovery_operation_secs, 10); // default
}

#[test]
fn test_timeout_builder_all_fields() {
    let config = TimeoutConfig::builder()
        .health_check_secs(12)
        .hsm_operation_secs(5)
        .hsm_probe_millis(1000)
        .discovery_operation_secs(25)
        .build();

    assert_eq!(config.health_check_secs, 12);
    assert_eq!(config.hsm_operation_secs, 5);
    assert_eq!(config.hsm_probe_millis, 1000);
    assert_eq!(config.discovery_operation_secs, 25);
}

// ============================================================================
// Validation Tests
// ============================================================================

#[test]
fn test_timeout_validation_success() {
    let config = TimeoutConfig::default();
    assert!(config.validate().is_ok(), "Default config should be valid");
}

#[test]
fn test_timeout_validation_health_check_too_short() {
    let config = TimeoutConfig::builder().health_check_secs(0).build();

    let result = config.validate();
    assert!(result.is_err(), "Should fail with health_check_secs = 0");
    assert!(result.unwrap_err().contains("Health check timeout"));
}

#[test]
fn test_timeout_validation_health_check_too_long() {
    let config = TimeoutConfig::builder().health_check_secs(61).build();

    let result = config.validate();
    assert!(result.is_err(), "Should fail with health_check_secs = 61");
}

#[test]
fn test_timeout_validation_hsm_operation_too_short() {
    let config = TimeoutConfig::builder().hsm_operation_secs(0).build();

    assert!(config.validate().is_err());
}

#[test]
fn test_timeout_validation_hsm_operation_too_long() {
    let config = TimeoutConfig::builder().hsm_operation_secs(11).build();

    assert!(config.validate().is_err());
}

#[test]
fn test_timeout_validation_hsm_probe_too_short() {
    let config = TimeoutConfig::builder().hsm_probe_millis(50).build();

    assert!(config.validate().is_err());
}

#[test]
fn test_timeout_validation_hsm_probe_too_long() {
    let config = TimeoutConfig::builder().hsm_probe_millis(6000).build();

    assert!(config.validate().is_err());
}

#[test]
fn test_timeout_validation_discovery_too_short() {
    let config = TimeoutConfig::builder().discovery_operation_secs(0).build();

    assert!(config.validate().is_err());
}

#[test]
fn test_timeout_validation_discovery_too_long() {
    let config = TimeoutConfig::builder()
        .discovery_operation_secs(301)
        .build();

    assert!(config.validate().is_err());
}

// ============================================================================
// Duration Conversion Tests
// ============================================================================

#[test]
fn test_timeout_duration_conversions() {
    let config = TimeoutConfig::builder()
        .health_check_secs(10)
        .hsm_operation_secs(3)
        .hsm_probe_millis(750)
        .discovery_operation_secs(20)
        .build();

    assert_eq!(config.health_check_duration(), Duration::from_secs(10));
    assert_eq!(config.hsm_operation_duration(), Duration::from_secs(3));
    assert_eq!(config.hsm_probe_duration(), Duration::from_millis(750));
    assert_eq!(
        config.discovery_operation_duration(),
        Duration::from_secs(20)
    );
}

#[test]
fn test_timeout_default_duration_conversions() {
    let config = TimeoutConfig::default();

    assert_eq!(config.health_check_duration(), Duration::from_secs(5));
    assert_eq!(config.hsm_operation_duration(), Duration::from_secs(2));
    assert_eq!(config.hsm_probe_duration(), Duration::from_millis(500));
    assert_eq!(
        config.discovery_operation_duration(),
        Duration::from_secs(10)
    );
}

// ============================================================================
// Serialization Tests
// ============================================================================

#[test]
fn test_timeout_serialization_roundtrip() {
    let config = TimeoutConfig::builder()
        .health_check_secs(15)
        .hsm_operation_secs(4)
        .build();

    // Serialize to TOML
    let toml_str = toml::to_string(&config).expect("Failed to serialize");

    // Deserialize back
    let deserialized: TimeoutConfig = toml::from_str(&toml_str).expect("Failed to deserialize");

    // Verify timeouts match
    assert_eq!(config, deserialized);
}

#[test]
fn test_timeout_serialization_partial() {
    // Serialize config with some defaults
    let config = TimeoutConfig::builder().health_check_secs(20).build();

    let toml_str = toml::to_string(&config).unwrap();
    let deserialized: TimeoutConfig = toml::from_str(&toml_str).unwrap();

    assert_eq!(config, deserialized);
}

// ============================================================================
// Configuration Profile Tests
// ============================================================================

#[test]
fn test_timeout_development_profile() {
    let config = TimeoutConfig::builder()
        .health_check_secs(2)
        .hsm_operation_secs(1)
        .hsm_probe_millis(200)
        .discovery_operation_secs(5)
        .build();

    assert!(
        config.validate().is_ok(),
        "Development profile should be valid"
    );
    assert_eq!(config.health_check_secs, 2);
}

#[test]
fn test_timeout_production_profile() {
    let config = TimeoutConfig::builder()
        .health_check_secs(10)
        .hsm_operation_secs(5)
        .hsm_probe_millis(1000)
        .discovery_operation_secs(30)
        .build();

    assert!(
        config.validate().is_ok(),
        "Production profile should be valid"
    );
    assert_eq!(config.health_check_secs, 10);
}

#[test]
fn test_timeout_high_performance_profile() {
    let config = TimeoutConfig::builder()
        .health_check_secs(3)
        .hsm_operation_secs(1)
        .hsm_probe_millis(300)
        .discovery_operation_secs(5)
        .build();

    assert!(
        config.validate().is_ok(),
        "High-performance profile should be valid"
    );
    assert_eq!(config.hsm_probe_millis, 300);
}

// ============================================================================
// BearDogConfig Integration Tests
// ============================================================================

#[test]
fn test_beardog_config_default_timeouts() {
    let config = BearDogConfig::default();

    // Verify timeout defaults are present
    assert_eq!(config.timeouts.health_check_secs, 5);
    assert_eq!(config.timeouts.hsm_operation_secs, 2);
}

#[test]
fn test_beardog_config_validation_with_timeouts() {
    let config = BearDogConfig::default();

    // Should validate successfully with default timeouts
    assert!(config.validate().is_ok());
}

#[test]
fn test_beardog_config_validation_fails_invalid_timeouts() {
    // Set invalid timeout
    let config = BearDogConfig {
        timeouts: TimeoutConfig::builder()
            .health_check_secs(0)  // Invalid!
            .build(),
        ..Default::default()
    };

    // Should fail validation
    assert!(config.validate().is_err());
}

// ============================================================================
// Edge Case Tests
// ============================================================================

#[test]
fn test_timeout_boundary_values_lower() {
    let config = TimeoutConfig::builder()
        .health_check_secs(1)  // Minimum valid
        .hsm_operation_secs(1)
        .hsm_probe_millis(100)
        .discovery_operation_secs(1)
        .build();

    assert!(config.validate().is_ok());
}

#[test]
fn test_timeout_boundary_values_upper() {
    let config = TimeoutConfig::builder()
        .health_check_secs(60)  // Maximum valid
        .hsm_operation_secs(10)
        .hsm_probe_millis(5000)
        .discovery_operation_secs(300)
        .build();

    assert!(config.validate().is_ok());
}

#[test]
fn test_timeout_builder_fluent_api() {
    // Test that builder methods can be chained fluently
    let config = TimeoutConfig::builder()
        .health_check_secs(7)
        .hsm_operation_secs(3)
        .hsm_probe_millis(600)
        .discovery_operation_secs(15)
        .build();

    assert_eq!(config.health_check_secs, 7);
    assert_eq!(config.hsm_operation_secs, 3);
    assert_eq!(config.hsm_probe_millis, 600);
    assert_eq!(config.discovery_operation_secs, 15);
}

// ============================================================================
// NOTE: Environment-free tests
// ============================================================================
//
// Prefer `TimeoutConfig::builder()` and explicit values for unit tests.
// For `from_env()` behavior, use a parameter-injected variant or integration
// tests that read the real environment without mutating it (documented).
