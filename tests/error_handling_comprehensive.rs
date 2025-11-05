//! Comprehensive Error Handling Tests
//!
//! This module contains extensive tests for BearDog's error handling system,
//! including error construction, propagation, recovery, and transformation patterns.
//! Tests cover validation, configuration, network, internal, and not-found error types.

use beardog_errors::{BearDogError, BearDogResult};

// ============================================================================
// Basic Error Construction Tests
// ============================================================================

/// Tests that BearDogError::validation creates errors with correct messages
///
/// TEST_CATEGORY: unit
/// TEST_DOMAIN: errors
/// TEST_PRIORITY: high
#[test]
fn test_error_validation() {
    // Given: a validation error message
    let error = BearDogError::validation("Field cannot be empty");

    // Then: the error should contain the expected message
    let error_str = format!("{}", error);
    assert!(error_str.contains("Field cannot be empty"));
}

/// Tests that BearDogError::configuration creates errors with correct messages
///
/// TEST_CATEGORY: unit
/// TEST_DOMAIN: errors
/// TEST_PRIORITY: high
#[test]
fn test_error_configuration() {
    // Given: a configuration error message
    let error = BearDogError::configuration("Invalid port number");

    // Then: the error should contain the expected message
    let error_str = format!("{}", error);
    assert!(error_str.contains("Invalid port number"));
}

/// Tests that BearDogError::not_found creates errors with resource identifiers
///
/// TEST_CATEGORY: unit
/// TEST_DOMAIN: errors
/// TEST_PRIORITY: high
#[test]
fn test_error_not_found() {
    // Given: a not found error with resource ID
    let error = BearDogError::not_found("Resource 'test-id-123' not found".to_string());

    // Then: the error should contain the resource ID
    let error_str = format!("{}", error);
    assert!(error_str.contains("test-id-123"));
}

/// Tests that BearDogError::network creates errors for network failures
///
/// TEST_CATEGORY: unit
/// TEST_DOMAIN: errors
/// TEST_PRIORITY: high
#[test]
fn test_error_network() {
    // Given: a network error message
    let error = BearDogError::network("Connection timeout".to_string());

    // Then: the error should contain the network issue description
    let error_str = format!("{}", error);
    assert!(error_str.contains("Connection timeout"));
}

/// Tests that BearDogError::internal creates errors for internal failures
///
/// TEST_CATEGORY: unit
/// TEST_DOMAIN: errors
/// TEST_PRIORITY: high
#[test]
fn test_error_internal() {
    // Given: an internal error message
    let error = BearDogError::internal("Unexpected state".to_string());

    // Then: the error should contain the state description
    let error_str = format!("{}", error);
    assert!(error_str.contains("Unexpected state"));
}

/// Tests that BearDogError::invalid_input creates errors for input validation
///
/// TEST_CATEGORY: unit
/// TEST_DOMAIN: errors
/// TEST_PRIORITY: high
#[test]
fn test_error_invalid_input() {
    // Given: an invalid input error message
    let error = BearDogError::invalid_input("Input must be a positive number");

    // Then: the error should contain the validation requirement
    let error_str = format!("{}", error);
    assert!(error_str.contains("Input must be a positive number"));
}

// ============================================================================
// Error Chain and Propagation Tests
// ============================================================================

/// Tests error chaining through validation logic
///
/// TEST_CATEGORY: unit
/// TEST_DOMAIN: errors
/// TEST_PRIORITY: high
#[test]
fn test_error_chain_validation() {
    // Helper function that validates input
    fn validate_input(input: &str) -> BearDogResult<()> {
        if input.is_empty() {
            return Err(BearDogError::validation("Input cannot be empty"));
        }
        Ok(())
    }

    // When: validating empty input
    let result = validate_input("");
    // Then: should return an error
    assert!(result.is_err());

    // When: validating valid input
    let result = validate_input("valid");
    // Then: should succeed
    assert!(result.is_ok());
}

/// Tests error chaining with not_found errors
///
/// TEST_CATEGORY: unit
/// TEST_DOMAIN: errors
/// TEST_PRIORITY: high
#[test]
fn test_error_chain_not_found() {
    // Helper function that finds resources
    fn find_resource(id: &str) -> BearDogResult<String> {
        if id == "missing" {
            return Err(BearDogError::not_found(format!(
                "Resource '{}' not found",
                id
            )));
        }
        Ok(format!("Resource: {}", id))
    }

    // When: finding a missing resource
    let result = find_resource("missing");
    // Then: should return not_found error
    assert!(result.is_err());

    // When: finding an existing resource
    let result = find_resource("exists");
    // Then: should succeed
    assert!(result.is_ok());
}

/// Tests error propagation through multiple function levels
///
/// TEST_CATEGORY: unit
/// TEST_DOMAIN: errors
/// TEST_PRIORITY: high
#[test]
fn test_error_propagation() {
    // Helper functions simulating a 3-level call chain
    fn level_3() -> BearDogResult<()> {
        Err(BearDogError::internal("Deep error".to_string()))
    }

    fn level_2() -> BearDogResult<()> {
        level_3()?;
        Ok(())
    }

    fn level_1() -> BearDogResult<()> {
        level_2()?;
        Ok(())
    }

    // When: error originates at level 3
    let result = level_1();
    // Then: should propagate through all levels
    assert!(result.is_err());
}

// ============================================================================
// Error Recovery and Transformation Tests
// ============================================================================

/// Tests error recovery patterns with unwrap_or
///
/// TEST_CATEGORY: unit
/// TEST_DOMAIN: errors
/// TEST_PRIORITY: normal
#[test]
fn test_error_recovery() {
    // Helper function that may fail
    fn risky_operation(should_fail: bool) -> BearDogResult<i32> {
        if should_fail {
            Err(BearDogError::internal("Operation failed".to_string()))
        } else {
            Ok(42)
        }
    }

    // When: operation fails
    let result = risky_operation(true);
    // Then: should return error
    assert!(result.is_err());

    // When: recovering with default value
    let value = risky_operation(true).unwrap_or(0);
    // Then: should use fallback value
    assert_eq!(value, 0);

    // When: operation succeeds
    let result = risky_operation(false);
    // Then: should return success value
    assert!(result.is_ok());
    assert_eq!(result.unwrap(), 42);
}

/// Tests error mapping and transformation patterns
///
/// TEST_CATEGORY: unit
/// TEST_DOMAIN: errors
/// TEST_PRIORITY: normal
#[test]
fn test_error_map() {
    // Helper function that fails
    fn operation() -> BearDogResult<i32> {
        Err(BearDogError::internal("Failed".to_string()))
    }

    // When: operation fails
    let result = operation();
    // Then: should be an error
    assert!(result.is_err());

    // When: mapping error to default value
    let value = result.unwrap_or(100);
    // Then: should use the mapped value
    assert_eq!(value, 100);
}

/// Tests error recovery with or_else for fallback logic
///
/// TEST_CATEGORY: unit
/// TEST_DOMAIN: errors
/// TEST_PRIORITY: normal
#[test]
fn test_error_or_else() {
    // Helper functions for primary and fallback sources
    fn primary_source() -> BearDogResult<String> {
        Err(BearDogError::not_found(
            "Primary source not found".to_string(),
        ))
    }

    fn fallback_source() -> BearDogResult<String> {
        Ok("Fallback data".to_string())
    }

    // When: primary fails but fallback succeeds
    let result = primary_source().or_else(|_| fallback_source());
    // Then: should use fallback data
    assert!(result.is_ok());
    assert_eq!(result.unwrap(), "Fallback data");
}

// ============================================================================
// Error Context and Validation Tests
// ============================================================================

/// Tests that error context is preserved through formatting
///
/// TEST_CATEGORY: unit
/// TEST_DOMAIN: errors
/// TEST_PRIORITY: normal
#[test]
fn test_error_context_preservation() {
    // Given: an error with specific context
    let error = BearDogError::validation("Invalid email format for field 'email'");
    let error_string = format!("{}", error);

    // Then: context should be preserved in the error message
    assert!(error_string.contains("email"));
    assert!(error_string.contains("Invalid email format"));
}

/// Tests handling multiple validation errors for complex input
///
/// TEST_CATEGORY: unit
/// TEST_DOMAIN: errors
/// TEST_PRIORITY: high
#[test]
fn test_multiple_validation_errors() {
    // Helper function that validates user input with multiple rules
    fn validate_user_input(name: &str, email: &str, age: i32) -> BearDogResult<()> {
        if name.is_empty() {
            return Err(BearDogError::validation("Name is required"));
        }
        if !email.contains('@') {
            return Err(BearDogError::validation("Invalid email format"));
        }
        if !(0..=150).contains(&age) {
            return Err(BearDogError::validation("Age must be between 0 and 150"));
        }
        Ok(())
    }

    // Test each validation rule
    assert!(
        validate_user_input("", "test@example.com", 25).is_err(),
        "Empty name should fail"
    );
    assert!(
        validate_user_input("John", "invalid", 25).is_err(),
        "Invalid email should fail"
    );
    assert!(
        validate_user_input("John", "test@example.com", -1).is_err(),
        "Negative age should fail"
    );
    assert!(
        validate_user_input("John", "test@example.com", 200).is_err(),
        "Age > 150 should fail"
    );

    // Test valid input
    assert!(
        validate_user_input("John", "test@example.com", 25).is_ok(),
        "Valid input should succeed"
    );
}

/// Tests debug formatting of errors
///
/// TEST_CATEGORY: unit
/// TEST_DOMAIN: errors
/// TEST_PRIORITY: normal
#[test]
fn test_error_debug_format() {
    // Given: an internal error
    let error = BearDogError::internal("Test error".to_string());

    // When: formatting with debug
    let debug_str = format!("{:?}", error);

    // Then: debug format should contain information
    assert!(!debug_str.is_empty());
}

// ============================================================================
// BearDogResult Type Tests
// ============================================================================

/// Tests that BearDogResult type works correctly for success and failure cases
///
/// TEST_CATEGORY: unit
/// TEST_DOMAIN: errors
/// TEST_PRIORITY: high
#[test]
fn test_result_type_basic() {
    // Helper function that returns BearDogResult
    fn returns_result(succeed: bool) -> BearDogResult<String> {
        if succeed {
            Ok("Success".to_string())
        } else {
            Err(BearDogError::internal("Failed".to_string()))
        }
    }

    // When: succeeding
    let success = returns_result(true);
    // Then: should be Ok with correct value
    assert!(success.is_ok());
    assert_eq!(success.unwrap(), "Success");

    // When: failing
    let failure = returns_result(false);
    // Then: should be Err
    assert!(failure.is_err());
}

/// Tests all error construction variants
///
/// TEST_CATEGORY: unit
/// TEST_DOMAIN: errors
/// TEST_PRIORITY: high
#[test]
fn test_error_construction_variants() {
    // When: constructing all error variants
    let _err1 = BearDogError::validation("Validation message");
    let _err2 = BearDogError::not_found("Item not found".to_string());
    let _err3 = BearDogError::invalid_input("Invalid input");
    let _err4 = BearDogError::configuration("Config error");
    let _err5 = BearDogError::network("Network error".to_string());
    let _err6 = BearDogError::internal("Internal error".to_string());

    // Then: all constructions should succeed without panicking
}

// ============================================================================
// Complex Error Chain Tests
// ============================================================================

/// Tests complex error chains with multiple steps and validation
///
/// TEST_CATEGORY: unit
/// TEST_DOMAIN: errors
/// TEST_PRIORITY: normal
#[test]
fn test_error_chain_complex() {
    // Multi-step chain with validation at each step
    fn step_1() -> BearDogResult<i32> {
        Ok(1)
    }

    fn step_2(val: i32) -> BearDogResult<i32> {
        if val > 0 {
            Ok(val * 2)
        } else {
            Err(BearDogError::validation("Value must be positive"))
        }
    }

    fn step_3(val: i32) -> BearDogResult<i32> {
        if val < 100 {
            Ok(val + 10)
        } else {
            Err(BearDogError::validation("Value must be less than 100"))
        }
    }

    fn full_chain() -> BearDogResult<i32> {
        let v1 = step_1()?;
        let v2 = step_2(v1)?;
        let v3 = step_3(v2)?;
        Ok(v3)
    }

    // When: running full successful chain
    let result = full_chain();
    // Then: should compute correctly through all steps
    assert!(result.is_ok());
    assert_eq!(result.unwrap(), 12); // (1 * 2) + 10
}

/// Tests converting Option to Result with proper error handling
///
/// TEST_CATEGORY: unit
/// TEST_DOMAIN: errors
/// TEST_PRIORITY: normal
#[test]
fn test_option_to_result_conversion() {
    // Helper function returning Option
    fn find_item(id: u32) -> Option<String> {
        if id == 42 {
            Some("Item 42".to_string())
        } else {
            None
        }
    }

    // Helper function converting Option to Result
    fn get_item(id: u32) -> BearDogResult<String> {
        find_item(id).ok_or_else(|| BearDogError::not_found(format!("Item '{}' not found", id)))
    }

    // When: item exists
    let result = get_item(42);
    // Then: should return Ok
    assert!(result.is_ok());

    // When: item doesn't exist
    let result = get_item(99);
    // Then: should return Err
    assert!(result.is_err());
}

/// Tests early return pattern with the ? operator
///
/// TEST_CATEGORY: unit
/// TEST_DOMAIN: errors
/// TEST_PRIORITY: normal
#[test]
fn test_error_early_return() {
    // Helper function with early returns
    fn process_data(data: &str) -> BearDogResult<usize> {
        if data.is_empty() {
            return Err(BearDogError::validation("Data cannot be empty"));
        }

        if data.len() > 1000 {
            return Err(BearDogError::validation("Data too large (max 1000 chars)"));
        }

        Ok(data.len())
    }

    // Test validation rules
    assert!(process_data("").is_err(), "Empty data should fail");
    assert!(
        process_data(&"x".repeat(1001)).is_err(),
        "Large data should fail"
    );
    assert!(process_data("valid").is_ok(), "Valid data should succeed");
}

/// Tests error handling in iterator chains and loops
///
/// TEST_CATEGORY: unit
/// TEST_DOMAIN: errors
/// TEST_PRIORITY: normal
#[test]
fn test_error_in_iterator() {
    // Helper function that parses multiple numbers
    fn parse_numbers(strings: &[&str]) -> BearDogResult<Vec<i32>> {
        let mut numbers = Vec::new();
        for s in strings {
            let num = s
                .parse::<i32>()
                .map_err(|_| BearDogError::validation("Invalid number format"))?;
            numbers.push(num);
        }
        Ok(numbers)
    }

    // When: all strings are valid numbers
    let valid = ["1", "2", "3"];
    // Then: should parse successfully
    assert!(parse_numbers(&valid).is_ok());

    // When: one string is invalid
    let invalid = ["1", "not-a-number", "3"];
    // Then: should fail with validation error
    assert!(parse_numbers(&invalid).is_err());
}
