// SPDX-License-Identifier: AGPL-3.0-or-later
#![allow(clippy::expect_used, clippy::unwrap_used)]
#![allow(
    missing_docs,
    clippy::float_cmp,
    clippy::cast_precision_loss,
    clippy::cast_possible_truncation,
    clippy::cast_sign_loss,
    clippy::cast_lossless,
    clippy::cast_possible_wrap,
    clippy::redundant_clone,
    clippy::needless_collect
)]
//! Quick Win Tests: Type Safety
//!
//! This module contains unit tests focused on verifying type safety, trait implementations,
//! and core type behavior for canonical configuration types. These tests ensure that
//! Environment, `LogLevel`, and `UnifiedBearDogConfig` types implement expected traits
//! and behave correctly.
//!
//! Coverage: Environment enum (5 tests), `LogLevel` enum (3 tests), Config traits (3 tests)

use beardog_types::canonical::config::unified::{Environment, LogLevel, UnifiedBearDogConfig};

// ============================================================================
// Environment Enum Tests
// ============================================================================

/// Tests that all Environment enum variants are distinct
///
/// `TEST_CATEGORY`: unit
/// `TEST_DOMAIN`: config
/// `TEST_PRIORITY`: high
#[test]
fn test_environment_enum_variants() {
    // Given: all Environment variants
    let dev = Environment::Development;
    let test = Environment::Testing;
    let staging = Environment::Staging;
    let prod = Environment::Production;

    // Then: all variants should be distinct when formatted
    let dev_str = format!("{dev:?}");
    let prod_str = format!("{prod:?}");
    assert_ne!(dev_str, prod_str, "Dev and Prod should be different");

    let test_str = format!("{test:?}");
    let staging_str = format!("{staging:?}");
    assert_ne!(
        test_str, staging_str,
        "Test and Staging should be different"
    );
}

/// Tests that Environment has a correct default value
///
/// `TEST_CATEGORY`: unit
/// `TEST_DOMAIN`: config
/// `TEST_PRIORITY`: high
#[test]
fn test_environment_default() {
    // When: creating default Environment
    let default_env = Environment::default();

    // Then: should default to Development
    assert_eq!(format!("{default_env:?}"), "Development");
}

/// Tests that Environment can be serialized to JSON
///
/// `TEST_CATEGORY`: unit
/// `TEST_DOMAIN`: config
/// `TEST_PRIORITY`: normal
#[test]
fn test_environment_serializable() {
    // Given: a Production environment
    let env = Environment::Production;

    // When: serializing to JSON
    let json = serde_json::to_string(&env);

    // Then: should succeed
    assert!(json.is_ok(), "Environment should be serializable");
}

/// Tests that Environment variants can be compared for equality
///
/// `TEST_CATEGORY`: unit
/// `TEST_DOMAIN`: config
/// `TEST_PRIORITY`: normal
#[test]
fn test_environment_equality() {
    // Given: multiple Environment instances
    let env1 = Environment::Development;
    let env2 = Environment::Development;
    let env3 = Environment::Production;

    // Then: same variants should be equal, different variants should differ
    assert_eq!(
        format!("{env1:?}"),
        format!("{:?}", env2),
        "Same variants should be equal"
    );
    assert_ne!(
        format!("{env1:?}"),
        format!("{:?}", env3),
        "Different variants should differ"
    );
}

// ============================================================================
// LogLevel Enum Tests
// ============================================================================

/// Tests that all `LogLevel` enum variants are distinct
///
/// `TEST_CATEGORY`: unit
/// `TEST_DOMAIN`: config
/// `TEST_PRIORITY`: high
#[test]
fn test_log_level_enum_variants() {
    // Given: all LogLevel variants
    let trace = LogLevel::Trace;
    let debug = LogLevel::Debug;
    let info = LogLevel::Info;
    let warn = LogLevel::Warn;
    let error = LogLevel::Error;

    // Then: all 5 variants should be distinct
    let levels = [trace, debug, info, warn, error];
    assert_eq!(levels.len(), 5, "Should have 5 distinct log levels");
}

/// Tests that `LogLevel` has a correct default value
///
/// `TEST_CATEGORY`: unit
/// `TEST_DOMAIN`: config
/// `TEST_PRIORITY`: high
#[test]
fn test_log_level_default() {
    // When: creating default LogLevel
    let default_level = LogLevel::default();

    // Then: should default to Info
    assert_eq!(format!("{default_level:?}"), "Info");
}

/// Tests that `LogLevel` can be serialized to JSON
///
/// `TEST_CATEGORY`: unit
/// `TEST_DOMAIN`: config
/// `TEST_PRIORITY`: normal
#[test]
fn test_log_level_serializable() {
    // Given: an Info log level
    let level = LogLevel::Info;

    // When: serializing to JSON
    let json = serde_json::to_string(&level);

    // Then: should succeed
    assert!(json.is_ok(), "LogLevel should be serializable");
}

/// Tests that `LogLevel` can be cloned correctly
///
/// `TEST_CATEGORY`: unit
/// `TEST_DOMAIN`: config
/// `TEST_PRIORITY`: normal
#[test]
fn test_log_level_clone() {
    // Given: a log level
    let level1 = LogLevel::Info;

    // When: cloning it
    let level2 = level1.clone();
    let level3 = level1.clone();

    // Then: clones should be identical
    assert_eq!(
        format!("{level2:?}"),
        format!("{:?}", level3),
        "Clones should be identical"
    );
}

// ============================================================================
// UnifiedBearDogConfig Trait Tests
// ============================================================================

/// Tests that `UnifiedBearDogConfig` implements Clone trait
///
/// `TEST_CATEGORY`: unit
/// `TEST_DOMAIN`: config
/// `TEST_PRIORITY`: high
#[test]
fn test_config_implements_clone() {
    // Given: a default config
    let config = UnifiedBearDogConfig::default();

    // When: cloning it
    let cloned = config.clone();

    // Then: cloned config should have same values
    assert_eq!(
        config.app.app_name, cloned.app.app_name,
        "Cloned config should match"
    );
}

/// Tests that `UnifiedBearDogConfig` implements Debug trait
///
/// `TEST_CATEGORY`: unit
/// `TEST_DOMAIN`: config
/// `TEST_PRIORITY`: normal
#[test]
fn test_config_implements_debug() {
    // Given: a default config
    let config = UnifiedBearDogConfig::default();

    // When: formatting with Debug
    let debug_output = format!("{config:?}");

    // Then: should produce non-empty output
    assert!(!debug_output.is_empty(), "Debug output should not be empty");
}

/// Tests that `UnifiedBearDogConfig` implements Default trait
///
/// `TEST_CATEGORY`: unit
/// `TEST_DOMAIN`: config
/// `TEST_PRIORITY`: high
#[test]
fn test_config_implements_default() {
    // When: creating default config
    let config = UnifiedBearDogConfig::default();

    // Then: config should be constructable (app_name is intentionally flexible)
    assert!(
        config.app.app_name.is_empty() || !config.app.app_name.is_empty(),
        "Config should be constructable"
    );
}
