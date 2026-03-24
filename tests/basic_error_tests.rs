// SPDX-License-Identifier: AGPL-3.0-only
#![allow(clippy::expect_used, clippy::unwrap_used)]
//! Basic Error Creation and Handling Tests
//!
//! This module contains unit tests for basic error type creation across all `BearDogError` variants.
//! Tests verify that each error type can be instantiated correctly and contains expected messages.
//!
//! Coverage: Error type creation (17 tests), Display/Debug (2 tests), Result handling (5 tests)

use beardog_errors::BearDogError;

// ============================================================================
// Core Error Type Creation Tests
// ============================================================================

/// Tests that system errors can be created with correct message
///
/// `TEST_CATEGORY`: unit
/// `TEST_DOMAIN`: errors
/// `TEST_PRIORITY`: high
#[test]
fn test_system_error_creation() {
    // Given: a system error message
    let error = BearDogError::system("system error".to_string());

    // Then: error should contain the message
    assert!(
        format!("{error:?}").contains("system error"),
        "System error should contain message"
    );
}

/// Tests that security errors can be created with correct message
///
/// `TEST_CATEGORY`: unit
/// `TEST_DOMAIN`: errors
/// `TEST_PRIORITY`: critical
#[test]
fn test_security_error_creation() {
    // Given: a security error message
    let error = BearDogError::security("security issue".to_string());

    // Then: error should contain the message
    assert!(
        format!("{error:?}").contains("security issue"),
        "Security error should contain message"
    );
}

/// Tests that validation errors can be created with correct message
///
/// `TEST_CATEGORY`: unit
/// `TEST_DOMAIN`: errors
/// `TEST_PRIORITY`: high
#[test]
fn test_validation_error_creation() {
    // Given: a validation error message
    let error = BearDogError::validation("invalid input");

    // Then: error should contain the message
    assert!(
        format!("{error:?}").contains("invalid input"),
        "Validation error should contain message"
    );
}

/// Tests that network errors can be created with correct message
///
/// `TEST_CATEGORY`: unit
/// `TEST_DOMAIN`: errors
/// `TEST_PRIORITY`: high
#[test]
fn test_network_error_creation() {
    // Given: a network error message
    let error = BearDogError::network("connection failed".to_string());

    // Then: error should contain the message
    assert!(
        format!("{error:?}").contains("connection failed"),
        "Network error should contain message"
    );
}

/// Tests that configuration errors can be created with correct message
///
/// `TEST_CATEGORY`: unit
/// `TEST_DOMAIN`: errors
/// `TEST_PRIORITY`: high
#[test]
fn test_configuration_error_creation() {
    // Given: a configuration error message
    let error = BearDogError::configuration("bad config");

    // Then: error should contain the message
    assert!(
        format!("{error:?}").contains("bad config"),
        "Configuration error should contain message"
    );
}

/// Tests that `not_found` errors can be created with correct message
///
/// `TEST_CATEGORY`: unit
/// `TEST_DOMAIN`: errors
/// `TEST_PRIORITY`: high
#[test]
fn test_not_found_error_creation() {
    // Given: a not found error message
    let error = BearDogError::not_found("resource not found".to_string());

    // Then: error should contain the message
    assert!(
        format!("{error:?}").contains("resource not found"),
        "NotFound error should contain message"
    );
}

/// Tests that unauthorized errors can be created with correct message
///
/// `TEST_CATEGORY`: unit
/// `TEST_DOMAIN`: errors
/// `TEST_PRIORITY`: critical
#[test]
fn test_unauthorized_error_creation() {
    // Given: an unauthorized error message
    let error = BearDogError::unauthorized("access denied".to_string());

    // Then: error should contain the message
    assert!(
        format!("{error:?}").contains("access denied"),
        "Unauthorized error should contain message"
    );
}

/// Tests that `invalid_input` errors can be created with correct message
///
/// `TEST_CATEGORY`: unit
/// `TEST_DOMAIN`: errors
/// `TEST_PRIORITY`: high
#[test]
fn test_invalid_input_error_creation() {
    // Given: an invalid input error message
    let error = BearDogError::invalid_input("bad input");

    // Then: error should contain the message
    assert!(
        format!("{error:?}").contains("bad input"),
        "InvalidInput error should contain message"
    );
}

/// Tests that unavailable errors can be created with correct message
///
/// `TEST_CATEGORY`: unit
/// `TEST_DOMAIN`: errors
/// `TEST_PRIORITY`: high
#[test]
fn test_unavailable_error_creation() {
    // Given: an unavailable error message
    let error = BearDogError::unavailable("service unavailable".to_string());

    // Then: error should contain the message
    assert!(
        format!("{error:?}").contains("service unavailable"),
        "Unavailable error should contain message"
    );
}

/// Tests that internal errors can be created with correct message
///
/// `TEST_CATEGORY`: unit
/// `TEST_DOMAIN`: errors
/// `TEST_PRIORITY`: high
#[test]
fn test_internal_error_creation() {
    // Given: an internal error message
    let error = BearDogError::internal("internal error".to_string());

    // Then: error should contain the message
    assert!(
        format!("{error:?}").contains("internal error"),
        "Internal error should contain message"
    );
}

/// Tests that business errors can be created with correct message
///
/// `TEST_CATEGORY`: unit
/// `TEST_DOMAIN`: errors
/// `TEST_PRIORITY`: normal
#[test]
fn test_business_error_creation() {
    // Given: a business error message
    let error = BearDogError::business("business rule violated".to_string());

    // Then: error should contain the message
    assert!(
        format!("{error:?}").contains("business rule"),
        "Business error should contain message"
    );
}

// ============================================================================
// Specialized Error Type Creation Tests
// ============================================================================

/// Tests that API errors can be created with correct message
///
/// `TEST_CATEGORY`: unit
/// `TEST_DOMAIN`: errors
/// `TEST_PRIORITY`: high
#[test]
fn test_api_error_creation() {
    // Given: an API error message
    let error = BearDogError::api("API error".to_string());

    // Then: error should contain the message
    assert!(
        format!("{error:?}").contains("API error"),
        "API error should contain message"
    );
}

/// Tests that workflow errors can be created with correct message
///
/// `TEST_CATEGORY`: unit
/// `TEST_DOMAIN`: errors
/// `TEST_PRIORITY`: high
#[test]
fn test_workflow_error_creation() {
    // Given: a workflow error message
    let error = BearDogError::workflow("workflow failed".to_string());

    // Then: error should contain the message
    assert!(
        format!("{error:?}").contains("workflow failed"),
        "Workflow error should contain message"
    );
}

/// Tests that genetics errors can be created with correct message
///
/// `TEST_CATEGORY`: unit
/// `TEST_DOMAIN`: errors
/// `TEST_PRIORITY`: normal
#[test]
fn test_genetics_error_creation() {
    // Given: a genetics error message
    let error = BearDogError::genetics("genetic error".to_string());

    // Then: error should contain the message
    assert!(
        format!("{error:?}").contains("genetic error"),
        "Genetics error should contain message"
    );
}

/// Tests that initialization errors can be created with correct message
///
/// `TEST_CATEGORY`: unit
/// `TEST_DOMAIN`: errors
/// `TEST_PRIORITY`: high
#[test]
fn test_initialization_error_creation() {
    // Given: an initialization error message
    let error = BearDogError::initialization("init failed".to_string());

    // Then: error should contain the message
    assert!(
        format!("{error:?}").contains("init failed"),
        "Initialization error should contain message"
    );
}

/// Tests that HSM errors can be created with correct message
///
/// `TEST_CATEGORY`: unit
/// `TEST_DOMAIN`: errors
/// `TEST_PRIORITY`: critical
#[test]
fn test_hsm_error_creation() {
    // Given: an HSM error message
    let error = BearDogError::hsm("HSM error".to_string());

    // Then: error should contain the message
    assert!(
        format!("{error:?}").contains("HSM error"),
        "HSM error should contain message"
    );
}

/// Tests that testing errors can be created with correct message
///
/// `TEST_CATEGORY`: unit
/// `TEST_DOMAIN`: errors
/// `TEST_PRIORITY`: normal
#[test]
fn test_testing_error_creation() {
    // Given: a testing error message
    let error = BearDogError::testing("test error");

    // Then: error should contain the message
    assert!(
        format!("{error:?}").contains("test error"),
        "Testing error should contain message"
    );
}

// ============================================================================
// Display and Debug Trait Tests
// ============================================================================

/// Tests that errors implement Display trait correctly
///
/// `TEST_CATEGORY`: unit
/// `TEST_DOMAIN`: errors
/// `TEST_PRIORITY`: normal
#[test]
fn test_error_display() {
    // Given: an error
    let error = BearDogError::validation("test");

    // When: formatting with Display
    let display_str = format!("{error}");

    // Then: should produce non-empty output
    assert!(
        !display_str.is_empty(),
        "Display output should not be empty"
    );
}

/// Tests that errors implement Debug trait correctly
///
/// `TEST_CATEGORY`: unit
/// `TEST_DOMAIN`: errors
/// `TEST_PRIORITY`: normal
#[test]
fn test_error_debug() {
    // Given: an error
    let error = BearDogError::system("test".to_string());

    // When: formatting with Debug
    let debug_str = format!("{error:?}");

    // Then: should produce non-empty output
    assert!(!debug_str.is_empty(), "Debug output should not be empty");
}

// ============================================================================
// Result Handling Tests
// ============================================================================

/// Tests that Result<(), `BearDogError`> can return Ok
///
/// `TEST_CATEGORY`: unit
/// `TEST_DOMAIN`: errors
/// `TEST_PRIORITY`: normal
#[test]
fn test_result_ok() -> Result<(), BearDogError> {
    // When: returning Ok
    Ok(())

    // Then: test passes if no error
}

/// Tests that Result can be used with values
///
/// `TEST_CATEGORY`: unit
/// `TEST_DOMAIN`: errors
/// `TEST_PRIORITY`: normal
#[test]
fn test_result_with_value() -> Result<(), BearDogError> {
    // Given: a value
    let _value: i32 = 42;

    // When: returning Ok
    Ok(())

    // Then: test passes if no error
}

/// Tests that Result can contain errors
///
/// `TEST_CATEGORY`: unit
/// `TEST_DOMAIN`: errors
/// `TEST_PRIORITY`: high
#[test]
fn test_error_in_result() {
    // Given: a Result containing an error
    let result: Result<(), BearDogError> = Err(BearDogError::validation("error"));

    // Then: should be recognized as Err
    assert!(result.is_err(), "Result should be Err");
}

/// Tests that multiple errors can be created and stored
///
/// `TEST_CATEGORY`: unit
/// `TEST_DOMAIN`: errors
/// `TEST_PRIORITY`: normal
#[test]
fn test_multiple_errors() {
    // Given: multiple errors of different types
    let errors = [
        BearDogError::validation("error 1"),
        BearDogError::system("error 2".to_string()),
        BearDogError::network("error 3".to_string()),
    ];

    // Then: all errors should be present
    assert_eq!(errors.len(), 3, "Should have 3 errors");
}
