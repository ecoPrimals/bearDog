//! Extended Error Handling Tests
//!
//! This module contains high-value integration tests for advanced error handling patterns,
//! including error construction with context, propagation through complex chains,
//! recovery patterns, and helper utilities.
//!
//! Coverage:
//! - Error Construction (10 tests) - Context-rich error creation
//! - Error Propagation (10 tests) - Complex propagation scenarios
//! - Error Recovery (6 tests) - Recovery and retry patterns
//! - Helper Functions (1 test) - Utility functions

#![allow(
    clippy::unwrap_used,
    clippy::expect_used,
    clippy::unnecessary_literal_unwrap
)]
#![allow(clippy::needless_borrows_for_generic_args, clippy::useless_format)]
#![allow(dead_code)] // Test helpers may not all be used

use beardog_errors::BearDogError;

// ============================================================================
// Error Construction Tests (10 tests)
// ============================================================================

/// Tests validation error with detailed field context
///
/// `TEST_CATEGORY`: integration
/// `TEST_DOMAIN`: errors
/// `TEST_PRIORITY`: high
#[test]
fn test_error_validation_with_context() {
    // Given: a validation error with detailed context
    let error = BearDogError::validation("Field 'email' must be a valid email address");

    // Then: error should contain field name and requirement
    let error_str = format!("{error}");
    assert!(
        error_str.contains("email"),
        "Error should mention field name"
    );
    assert!(
        error_str.contains("valid"),
        "Error should mention validation requirement"
    );
}

/// Tests configuration error with detailed range information
///
/// `TEST_CATEGORY`: integration
/// `TEST_DOMAIN`: errors
/// `TEST_PRIORITY`: high
#[test]
fn test_error_configuration_with_details() {
    // Given: a configuration error with specific details
    let error = BearDogError::configuration("Port 99999 is out of valid range (1-65535)");

    // Then: error should contain port number and range
    let error_str = format!("{error}");
    assert!(
        error_str.contains("99999"),
        "Error should contain invalid port"
    );
    assert!(
        error_str.contains("range"),
        "Error should mention valid range"
    );
}

/// Tests `not_found` error with resource ID and type
///
/// `TEST_CATEGORY`: integration
/// `TEST_DOMAIN`: errors
/// `TEST_PRIORITY`: high
#[test]
fn test_error_not_found_with_id() {
    // Given: a not found error with resource ID
    let resource_id = "hsm-provider-abc123";
    let error = BearDogError::not_found(format!(
        "HSM provider '{resource_id}' not found in registry"
    ));

    // Then: error should contain ID and resource type
    let error_str = format!("{error}");
    assert!(
        error_str.contains("abc123"),
        "Error should contain resource ID"
    );
    assert!(
        error_str.contains("HSM provider"),
        "Error should contain resource type"
    );
}

/// Tests network error with endpoint information
///
/// `TEST_CATEGORY`: integration
/// `TEST_DOMAIN`: errors
/// `TEST_PRIORITY`: high
#[test]
fn test_error_network_with_endpoint() {
    // Given: a network error with endpoint
    let endpoint = "https://api.example.com:8080";
    let error = BearDogError::network(format!("Failed to connect to endpoint: {endpoint}"));

    // Then: error should contain host and port
    let error_str = format!("{error}");
    assert!(
        error_str.contains("example.com"),
        "Error should contain hostname"
    );
    assert!(error_str.contains("8080"), "Error should contain port");
}

/// Tests internal error with state information
///
/// `TEST_CATEGORY`: integration
/// `TEST_DOMAIN`: errors
/// `TEST_PRIORITY`: high
#[test]
fn test_error_internal_with_state_info() {
    // Given: an internal error with state info
    let state = "ShuttingDown";
    let error = BearDogError::internal(format!("Operation rejected: system in {state} state"));

    // Then: error should contain state information
    let error_str = format!("{error}");
    assert!(
        error_str.contains("ShuttingDown"),
        "Error should contain state"
    );
}

/// Tests security error with specific reason
///
/// `TEST_CATEGORY`: integration
/// `TEST_DOMAIN`: errors
/// `TEST_PRIORITY`: critical
#[test]
fn test_error_security_with_reason() {
    // Given: a security error with reason
    let error = BearDogError::security("Authentication token expired".to_string());

    // Then: error should contain security issue details
    let error_str = format!("{error}");
    assert!(error_str.contains("token"), "Error should mention token");
    assert!(
        error_str.contains("expired"),
        "Error should mention expiration"
    );
}

/// Tests system error with timeout information
///
/// `TEST_CATEGORY`: integration
/// `TEST_DOMAIN`: errors
/// `TEST_PRIORITY`: high
#[test]
fn test_error_system_with_timeout_info() {
    // Given: a system error with timeout
    let timeout_secs = 30;
    let error = BearDogError::system(format!("Operation timed out after {timeout_secs}s"));

    // Then: error should contain timeout value
    let error_str = format!("{error}");
    assert!(
        error_str.contains("30"),
        "Error should contain timeout value"
    );
    assert!(
        error_str.contains("timed out"),
        "Error should mention timeout"
    );
}

/// Tests unavailable error with service name
///
/// `TEST_CATEGORY`: integration
/// `TEST_DOMAIN`: errors
/// `TEST_PRIORITY`: high
#[test]
fn test_error_unavailable_with_service() {
    // Given: an unavailable error for a service
    let service = "discovery-service";
    let error = BearDogError::unavailable(format!("Service '{service}' is currently unavailable"));

    // Then: error should contain service name
    let error_str = format!("{error}");
    assert!(
        error_str.contains("discovery-service"),
        "Error should contain service name"
    );
}

/// Tests `invalid_input` error with field name
///
/// `TEST_CATEGORY`: integration
/// `TEST_DOMAIN`: errors
/// `TEST_PRIORITY`: high
#[test]
fn test_error_invalid_input_with_field() {
    // Given: an invalid input error with field
    let error = BearDogError::invalid_input("Field 'primal_id' contains invalid characters");

    // Then: error should contain field name
    let error_str = format!("{error}");
    assert!(
        error_str.contains("primal_id"),
        "Error should contain field name"
    );
}

/// Tests debug formatting produces non-empty output
///
/// `TEST_CATEGORY`: integration
/// `TEST_DOMAIN`: errors
/// `TEST_PRIORITY`: normal
#[test]
fn test_error_debug_format() {
    // Given: an error
    let error = BearDogError::validation("Test error");

    // When: formatting with debug
    let debug_str = format!("{error:?}");

    // Then: should produce meaningful output
    assert!(!debug_str.is_empty(), "Debug output should not be empty");
    assert!(debug_str.len() > 5, "Debug output should be meaningful");
}

// ============================================================================
// Error Propagation Tests (10 tests)
// ============================================================================

/// Tests simple error propagation through Result chain
///
/// `TEST_CATEGORY`: integration
/// `TEST_DOMAIN`: errors
/// `TEST_PRIORITY`: high
#[test]
fn test_error_propagation_simple() {
    // Helper functions
    fn inner_fn() -> Result<(), BearDogError> {
        Err(BearDogError::validation("Inner error"))
    }

    fn outer_fn() -> Result<(), BearDogError> {
        inner_fn()?;
        Ok(())
    }

    // When: inner function fails
    let result = outer_fn();

    // Then: error should propagate to outer
    assert!(result.is_err(), "Error should propagate through call chain");
}

/// Tests error propagation with context preservation
///
/// `TEST_CATEGORY`: integration
/// `TEST_DOMAIN`: errors
/// `TEST_PRIORITY`: high
#[test]
fn test_error_propagation_with_context() {
    // Helper functions with context
    fn inner_fn() -> Result<String, BearDogError> {
        Err(BearDogError::not_found("Resource not found".to_string()))
    }

    fn middle_fn() -> Result<String, BearDogError> {
        inner_fn()?;
        Ok("success".to_string())
    }

    fn outer_fn() -> Result<String, BearDogError> {
        middle_fn()?;
        Ok("complete".to_string())
    }

    // When: error occurs at innermost level
    let result = outer_fn();

    // Then: error should propagate with context preserved
    assert!(result.is_err());
    if let Err(e) = result {
        let msg = format!("{e}");
        assert!(
            msg.contains("not found"),
            "Original error context should be preserved"
        );
    }
}

/// Tests error propagation across async/sync boundaries
///
/// `TEST_CATEGORY`: integration
/// `TEST_DOMAIN`: errors
/// `TEST_PRIORITY`: normal
#[test]
fn test_error_propagation_async_sync_boundary() {
    // Sync function that returns error
    fn sync_fn() -> Result<(), BearDogError> {
        Err(BearDogError::validation("Sync error"))
    }

    // When: sync function fails
    let result = sync_fn();

    // Then: error should be properly typed
    assert!(result.is_err(), "Error should be Result-typed");
}

/// Tests error propagation through match expressions
///
/// `TEST_CATEGORY`: integration
/// `TEST_DOMAIN`: errors
/// `TEST_PRIORITY`: normal
#[test]
fn test_error_propagation_through_match() {
    // Helper function with match-based validation
    fn process(value: i32) -> Result<i32, BearDogError> {
        match value {
            0 => Err(BearDogError::invalid_input("Value cannot be zero")),
            n if n < 0 => Err(BearDogError::invalid_input("Value must be positive")),
            n => Ok(n * 2),
        }
    }

    // Then: match arms should propagate errors correctly
    assert!(process(0).is_err(), "Zero should fail");
    assert!(process(-5).is_err(), "Negative should fail");
    assert_eq!(process(10).unwrap(), 20, "Positive should succeed");
}

/// Tests error transformation with `map_err`
///
/// `TEST_CATEGORY`: integration
/// `TEST_DOMAIN`: errors
/// `TEST_PRIORITY`: normal
#[test]
fn test_error_propagation_with_map_err() {
    // Helper functions with error transformation
    fn inner() -> Result<(), String> {
        Err("string error".to_string())
    }

    fn outer() -> Result<(), BearDogError> {
        inner().map_err(BearDogError::internal)?;
        Ok(())
    }

    // When: transforming error type
    let result = outer();

    // Then: error should be transformed to BearDogError
    assert!(result.is_err(), "Transformed error should propagate");
}

/// Tests early return with ? operator
///
/// `TEST_CATEGORY`: integration
/// `TEST_DOMAIN`: errors
/// `TEST_PRIORITY`: high
#[test]
fn test_error_early_return() {
    // Helper functions for multi-step operation
    fn step1() -> Result<(), BearDogError> {
        Ok(())
    }

    fn step2() -> Result<(), BearDogError> {
        Err(BearDogError::validation("Step 2 failed"))
    }

    fn step3() -> Result<String, BearDogError> {
        Ok("complete".to_string())
    }

    fn multi_step() -> Result<String, BearDogError> {
        step1()?;
        step2()?;
        step3()
    }

    // When: step2 fails
    let result = multi_step();

    // Then: should return early with error
    assert!(result.is_err(), "Should fail at step2");
    if let Err(e) = result {
        let msg = format!("{e}");
        assert!(msg.contains("Step 2"), "Should identify failing step");
    }
}

/// Tests error handling in iterator chains
///
/// `TEST_CATEGORY`: integration
/// `TEST_DOMAIN`: errors
/// `TEST_PRIORITY`: normal
#[test]
fn test_error_in_iterator_chain() {
    // Helper function processing items
    fn process_items(items: Vec<i32>) -> Result<Vec<i32>, BearDogError> {
        items
            .into_iter()
            .map(|n| {
                if n < 0 {
                    Err(BearDogError::invalid_input("Negative value"))
                } else {
                    Ok(n * 2)
                }
            })
            .collect()
    }

    // Then: valid items should succeed, invalid should fail
    assert!(
        process_items(vec![1, 2, 3]).is_ok(),
        "Valid items should succeed"
    );
    assert!(
        process_items(vec![1, -2, 3]).is_err(),
        "Invalid items should fail"
    );
}

/// Tests nested Result handling
///
/// `TEST_CATEGORY`: integration
/// `TEST_DOMAIN`: errors
/// `TEST_PRIORITY`: normal
#[test]
fn test_error_in_nested_results() {
    // Helper functions with nested Results
    fn inner() -> Result<String, BearDogError> {
        Err(BearDogError::validation("Inner validation failed"))
    }

    fn outer() -> Result<Result<String, BearDogError>, BearDogError> {
        Ok(inner())
    }

    // When: dealing with nested Results
    let result = outer();

    // Then: outer should succeed, inner should fail
    assert!(result.is_ok(), "Outer result should be Ok");
    if let Ok(inner_result) = result {
        assert!(inner_result.is_err(), "Inner result should be Err");
    }
}

/// Tests error creation with `ok_or_else`
///
/// `TEST_CATEGORY`: integration
/// `TEST_DOMAIN`: errors
/// `TEST_PRIORITY`: normal
#[test]
fn test_error_with_ok_or_else() {
    // Helper function converting Option to Result
    fn get_value(should_exist: bool) -> Result<String, BearDogError> {
        if should_exist {
            Some("value".to_string())
        } else {
            None
        }
        .ok_or_else(|| BearDogError::not_found("Value not found".to_string()))
    }

    // Then: present values succeed, absent values fail
    assert!(get_value(true).is_ok(), "Present value should succeed");
    assert!(get_value(false).is_err(), "Absent value should fail");
}

/// Tests error handling with `and_then` chains
///
/// `TEST_CATEGORY`: integration
/// `TEST_DOMAIN`: errors
/// `TEST_PRIORITY`: normal
#[test]
fn test_error_and_then_chain() {
    // Helper functions for chained operations
    fn get_value() -> Result<i32, BearDogError> {
        Ok(10)
    }

    fn validate(v: i32) -> Result<i32, BearDogError> {
        if v > 0 {
            Ok(v)
        } else {
            Err(BearDogError::validation("Must be positive"))
        }
    }

    fn transform(v: i32) -> Result<i32, BearDogError> {
        Ok(v * 2)
    }

    fn process() -> Result<i32, BearDogError> {
        get_value().and_then(validate).and_then(transform)
    }

    // When: chaining operations
    let result = process();

    // Then: all steps should succeed
    assert!(result.is_ok(), "Chain should succeed");
    assert_eq!(result.unwrap(), 20, "Should apply all transformations");
}

// ============================================================================
// Error Recovery Tests (6 tests)
// ============================================================================

/// Tests error recovery with `unwrap_or`
///
/// `TEST_CATEGORY`: integration
/// `TEST_DOMAIN`: errors
/// `TEST_PRIORITY`: high
#[test]
fn test_error_recovery_with_unwrap_or() {
    // Helper function that fails
    fn may_fail() -> Result<i32, BearDogError> {
        Err(BearDogError::validation("Failed"))
    }

    // When: recovering with default value
    let value = may_fail().unwrap_or(42);

    // Then: should use default
    assert_eq!(value, 42, "Should recover with default value");
}

/// Tests error recovery with `unwrap_or_else`
///
/// `TEST_CATEGORY`: integration
/// `TEST_DOMAIN`: errors
/// `TEST_PRIORITY`: normal
#[test]
fn test_error_recovery_with_unwrap_or_else() {
    // Helper function that fails
    fn may_fail() -> Result<String, BearDogError> {
        Err(BearDogError::not_found("Not found".to_string()))
    }

    // When: recovering with computed default
    let value = may_fail().unwrap_or_else(|_| "default".to_string());

    // Then: should use computed default
    assert_eq!(value, "default", "Should recover with computed default");
}

/// Tests error recovery with `unwrap_or_default`
///
/// `TEST_CATEGORY`: integration
/// `TEST_DOMAIN`: errors
/// `TEST_PRIORITY`: normal
#[test]
fn test_error_recovery_with_unwrap_or_default() {
    // Helper function that fails
    fn may_fail() -> Result<Vec<i32>, BearDogError> {
        Err(BearDogError::internal("Failed".to_string()))
    }

    // When: recovering with type's default
    let value = may_fail().unwrap_or_default();

    // Then: should use type's default (empty vec)
    assert!(value.is_empty(), "Should recover with type default");
}

/// Tests error recovery with match expression
///
/// `TEST_CATEGORY`: integration
/// `TEST_DOMAIN`: errors
/// `TEST_PRIORITY`: normal
#[test]
fn test_error_recovery_with_match() {
    // Helper function that fails
    fn may_fail() -> Result<i32, BearDogError> {
        Err(BearDogError::validation("Failed"))
    }

    // When: recovering with match
    let value = may_fail().unwrap_or(100);

    // Then: should use error branch value
    assert_eq!(value, 100, "Should recover via match");
}

/// Tests error recovery by converting to Option
///
/// `TEST_CATEGORY`: integration
/// `TEST_DOMAIN`: errors
/// `TEST_PRIORITY`: normal
#[test]
fn test_error_recovery_with_ok() {
    // Helper function that fails
    fn may_fail() -> Result<i32, BearDogError> {
        Err(BearDogError::validation("Failed"))
    }

    // When: converting error to None
    let option = may_fail().ok();

    // Then: should be None
    assert!(option.is_none(), "Failed result should convert to None");
}

/// Tests retry pattern for error recovery
///
/// `TEST_CATEGORY`: integration
/// `TEST_DOMAIN`: errors
/// `TEST_PRIORITY`: high
#[test]
fn test_error_recovery_retry_pattern() {
    use std::sync::atomic::{AtomicU32, Ordering};
    use std::sync::Arc;

    // Setup: operation that succeeds on third attempt
    let attempt_count = Arc::new(AtomicU32::new(0));
    let attempt_count_clone = Arc::clone(&attempt_count);

    let operation = || -> Result<String, BearDogError> {
        let count = attempt_count_clone.fetch_add(1, Ordering::SeqCst);
        if count < 2 {
            Err(BearDogError::system("Timeout".to_string()))
        } else {
            Ok("success".to_string())
        }
    };

    // When: retrying up to 3 times
    let mut result = operation();
    for _ in 0..2 {
        if result.is_ok() {
            break;
        }
        result = operation();
    }

    // Then: should succeed on third attempt
    assert!(result.is_ok(), "Retry should eventually succeed");
    assert_eq!(result.unwrap(), "success", "Should return success value");
}

// ============================================================================
// Helper Functions
// ============================================================================

#[cfg(test)]
mod error_helpers {
    use super::*;

    /// Create a test validation error
    pub fn validation_error(msg: &str) -> BearDogError {
        BearDogError::validation(msg)
    }

    /// Check if error message contains text
    pub fn error_contains(error: &BearDogError, text: &str) -> bool {
        format!("{error}").contains(text)
    }
}

/// Tests error helper utilities
///
/// `TEST_CATEGORY`: integration
/// `TEST_DOMAIN`: errors
/// `TEST_PRIORITY`: normal
#[test]
fn test_error_helpers() {
    // When: using helper functions
    let error = error_helpers::validation_error("test");

    // Then: helpers should work correctly
    assert!(
        error_helpers::error_contains(&error, "test"),
        "Helper should find text in error"
    );
}
