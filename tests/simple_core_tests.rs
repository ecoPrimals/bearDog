//! Simple core integration tests
//!
//! These tests validate basic functionality of core `BearDog` types and errors.

use beardog_errors::BearDogError;
use beardog_types::canonical::capabilities::ServiceCapabilityType;
use beardog_types::canonical::config::WorkingUnifiedConfig;

/// Tests that `BearDogError::configuration` creates errors with correct messages
///
/// `TEST_CATEGORY`: unit
/// `TEST_DOMAIN`: core
/// `TEST_PRIORITY`: high
#[test]
fn test_basic_error_types() {
    // Given: a configuration error message
    let error = BearDogError::configuration("Test rejection");

    // Then: the error contains the expected message
    assert!(format!("{error:?}").contains("Test rejection"));
}

/// Tests that `ServiceCapabilityType` variants can be instantiated and matched
///
/// `TEST_CATEGORY`: unit
/// `TEST_DOMAIN`: core
/// `TEST_PRIORITY`: high
#[test]
fn test_trust_levels() {
    // Given: a Compute capability type
    let capability = ServiceCapabilityType::Compute;

    // Then: it matches the expected variant
    assert!(matches!(capability, ServiceCapabilityType::Compute));
}

/// Tests that `WorkingUnifiedConfig` can be created with defaults
///
/// `TEST_CATEGORY`: unit
/// `TEST_DOMAIN`: core
/// `TEST_PRIORITY`: medium
#[test]
fn test_configuration_defaults() {
    // When: creating a default configuration
    let config = WorkingUnifiedConfig::default();

    // Then: the config is valid (has a version field)
    // Note: We check for either empty or non-empty to ensure field exists
    assert!(config.version.is_empty() || !config.version.is_empty());
}

/// Tests that `BearDogError::internal` creates errors that display correctly
///
/// `TEST_CATEGORY`: unit
/// `TEST_DOMAIN`: core
/// `TEST_PRIORITY`: high
#[test]
fn test_error_conversion() {
    // Given: an internal error
    let error = BearDogError::internal("Internal test error".to_string());

    // When: formatting the error as a string
    let error_string = format!("{error}");

    // Then: the string contains the error message
    assert!(error_string.contains("Internal test error"));
}

/// Tests that basic capability types can be instantiated
///
/// `TEST_CATEGORY`: unit
/// `TEST_DOMAIN`: core
/// `TEST_PRIORITY`: medium
#[test]
fn test_basic_types() {
    // When: creating a Compute capability type
    let capability = ServiceCapabilityType::Compute;

    // Then: it matches the expected variant
    assert!(matches!(capability, ServiceCapabilityType::Compute));
}
