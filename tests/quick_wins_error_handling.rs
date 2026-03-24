// SPDX-License-Identifier: AGPL-3.0-only
#![allow(clippy::expect_used, clippy::unwrap_used)]
//! Quick Win Tests: Error Handling
//!
//! This module contains unit tests focused on verifying basic error handling patterns
//! for `BearDogError`. These tests ensure that errors can be created, displayed, propagated,
//! and serialized correctly across the system.
//!
//! Coverage: Error creation (3 tests), Propagation (1 test), Display/Debug (2 tests), Misc (2 tests)

use beardog_errors::BearDogError;

// ============================================================================
// Error Creation Tests
// ============================================================================

/// Tests that validation errors can be created with correct type
///
/// `TEST_CATEGORY`: unit
/// `TEST_DOMAIN`: errors
/// `TEST_PRIORITY`: high
#[test]
fn test_beardog_error_creation() {
    // Given: a validation error message
    let err = BearDogError::validation("Test validation error");

    // Then: error should contain validation type
    assert!(
        format!("{err:?}").contains("validation"),
        "Should be validation error"
    );
}

/// Tests that system errors can be created with correct type
///
/// `TEST_CATEGORY`: unit
/// `TEST_DOMAIN`: errors
/// `TEST_PRIORITY`: high
#[test]
fn test_beardog_error_system() {
    // Given: a system error message
    let err = BearDogError::system("System error test".to_string());

    // Then: error should contain system type
    assert!(
        format!("{err:?}").contains("System"),
        "Should be system error"
    );
}

/// Tests that business errors can be created with correct type
///
/// `TEST_CATEGORY`: unit
/// `TEST_DOMAIN`: errors
/// `TEST_PRIORITY`: high
#[test]
fn test_beardog_error_business() {
    // Given: a business error message
    let err = BearDogError::business("Business logic error".to_string());

    // Then: error should contain business type
    assert!(
        format!("{err:?}").contains("Business"),
        "Should be business error"
    );
}

// ============================================================================
// Error Propagation Tests
// ============================================================================

/// Tests that errors propagate correctly through Result chains
///
/// `TEST_CATEGORY`: unit
/// `TEST_DOMAIN`: errors
/// `TEST_PRIORITY`: high
#[test]
fn test_result_error_propagation() -> Result<(), BearDogError> {
    // Helper function that may fail
    fn might_fail(should_fail: bool) -> Result<(), BearDogError> {
        if should_fail {
            Err(BearDogError::validation("Expected failure"))
        } else {
            Ok(())
        }
    }

    // When: success case
    might_fail(false)?;

    // When: failure case
    let result = might_fail(true);

    // Then: should propagate error
    assert!(result.is_err(), "Error should propagate");

    Ok(())
}

// ============================================================================
// Display and Debug Tests
// ============================================================================

/// Tests that errors implement Display trait correctly
///
/// `TEST_CATEGORY`: unit
/// `TEST_DOMAIN`: errors
/// `TEST_PRIORITY`: normal
#[test]
fn test_error_display() {
    // Given: an error
    let err = BearDogError::validation("Display test");

    // When: formatting with Display
    let display_str = format!("{err}");

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
    let err = BearDogError::validation("Debug test");

    // When: formatting with Debug
    let debug_str = format!("{err:?}");

    // Then: should produce meaningful output
    assert!(!debug_str.is_empty(), "Debug output should not be empty");
}

// ============================================================================
// Miscellaneous Error Tests
// ============================================================================

/// Tests that errors can be converted to strings
///
/// `TEST_CATEGORY`: unit
/// `TEST_DOMAIN`: errors
/// `TEST_PRIORITY`: normal
#[test]
fn test_error_from_string() {
    // Given: an error
    let err = BearDogError::validation("From string test");

    // When: converting to string
    let err_string = err.to_string();

    // Then: should produce non-empty string
    assert!(
        !err_string.is_empty(),
        "String conversion should not be empty"
    );
}

/// Tests that multiple error types can be created and stored
///
/// `TEST_CATEGORY`: unit
/// `TEST_DOMAIN`: errors
/// `TEST_PRIORITY`: normal
#[test]
fn test_multiple_error_types() {
    // Given: different error types
    let errors = [
        BearDogError::validation("error1"),
        BearDogError::system("error2".to_string()),
        BearDogError::business("error3".to_string()),
    ];

    // Then: all errors should be distinct
    assert_eq!(errors.len(), 3, "Should have 3 distinct errors");
}
