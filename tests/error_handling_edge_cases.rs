//! Error Handling Edge Cases Tests
//!
//! This module contains edge case tests for BearDog's error handling system,
//! focusing on boundary conditions, unusual inputs, and extreme scenarios that
//! might not be covered in standard tests.

use beardog_errors::BearDogError;

// ============================================================================
// Empty and Null-Like Input Edge Cases
// ============================================================================

/// Tests error construction with empty message strings
///
/// `TEST_CATEGORY`: unit
/// `TEST_DOMAIN`: errors
/// `TEST_PRIORITY`: high
#[test]
fn test_empty_error_message() {
    // When: creating an error with empty message
    let error = BearDogError::validation("");

    // Then: should still create valid error
    let error_str = format!("{}", error);
    assert!(
        !error_str.is_empty(),
        "Error string should not be empty even with empty message"
    );
}

/// Tests error construction with whitespace-only messages
///
/// `TEST_CATEGORY`: unit
/// `TEST_DOMAIN`: errors
/// `TEST_PRIORITY`: normal
#[test]
fn test_whitespace_only_message() {
    // When: creating errors with various whitespace patterns
    let error1 = BearDogError::validation("   ");
    let error2 = BearDogError::validation("\t\n\r");
    let error3 = BearDogError::configuration("     ");

    // Then: all should format without panicking
    let _ = format!("{}", error1);
    let _ = format!("{}", error2);
    let _ = format!("{}", error3);
}

/// Tests error construction with very long messages
///
/// `TEST_CATEGORY`: unit
/// `TEST_DOMAIN`: errors
/// `TEST_PRIORITY`: normal
#[test]
fn test_very_long_error_message() {
    // Given: a very long error message (10KB)
    let long_message = "Error! ".repeat(1500); // ~10KB

    // When: creating error with long message
    let error = BearDogError::internal(long_message.clone());

    // Then: should handle it without truncation or panic
    let error_str = format!("{}", error);
    assert!(error_str.len() > 5000, "Long message should be preserved");
}

/// Tests error construction with special characters
///
/// `TEST_CATEGORY`: unit
/// `TEST_DOMAIN`: errors
/// `TEST_PRIORITY`: normal
#[test]
fn test_special_characters_in_message() {
    // Given: messages with special characters
    let special_chars = vec![
        "Error: <script>alert('xss')</script>",
        "Error with emoji: 🚀🔥💥",
        "Error with unicode: ąčęėįšųū",
        "Error with null byte:\0in middle",
        "Error with quotes: \"quoted\" and 'single'",
    ];

    // When: creating errors with special characters
    for msg in special_chars {
        let error = BearDogError::validation(msg);

        // Then: should format without panicking
        let _ = format!("{}", error);
        let _ = format!("{:?}", error);
    }
}

// ============================================================================
// Result Chain Edge Cases
// ============================================================================

/// Tests deeply nested error propagation
///
/// `TEST_CATEGORY`: unit
/// `TEST_DOMAIN`: errors
/// `TEST_PRIORITY`: normal
#[test]
fn test_deeply_nested_error_propagation() {
    // Helper function with 20 levels of nesting
    fn level_20() -> Result<i32, BearDogError> {
        Err(BearDogError::internal("Deep error at level 20".to_string()))
    }

    fn level_19() -> Result<i32, BearDogError> {
        level_20()
    }
    fn level_18() -> Result<i32, BearDogError> {
        level_19()
    }
    fn level_17() -> Result<i32, BearDogError> {
        level_18()
    }
    fn level_16() -> Result<i32, BearDogError> {
        level_17()
    }
    fn level_15() -> Result<i32, BearDogError> {
        level_16()
    }
    fn level_14() -> Result<i32, BearDogError> {
        level_15()
    }
    fn level_13() -> Result<i32, BearDogError> {
        level_14()
    }
    fn level_12() -> Result<i32, BearDogError> {
        level_13()
    }
    fn level_11() -> Result<i32, BearDogError> {
        level_12()
    }
    fn level_10() -> Result<i32, BearDogError> {
        level_11()
    }
    fn level_9() -> Result<i32, BearDogError> {
        level_10()
    }
    fn level_8() -> Result<i32, BearDogError> {
        level_9()
    }
    fn level_7() -> Result<i32, BearDogError> {
        level_8()
    }
    fn level_6() -> Result<i32, BearDogError> {
        level_7()
    }
    fn level_5() -> Result<i32, BearDogError> {
        level_6()
    }
    fn level_4() -> Result<i32, BearDogError> {
        level_5()
    }
    fn level_3() -> Result<i32, BearDogError> {
        level_4()
    }
    fn level_2() -> Result<i32, BearDogError> {
        level_3()
    }
    fn level_1() -> Result<i32, BearDogError> {
        level_2()
    }

    // When: propagating error through 20 levels
    let result = level_1();

    // Then: error should propagate successfully
    assert!(result.is_err(), "Error should propagate through all levels");
}

/// Tests error handling with zero-sized types
///
/// `TEST_CATEGORY`: unit
/// `TEST_DOMAIN`: errors
/// `TEST_PRIORITY`: normal
#[test]
fn test_result_with_unit_type() {
    // When: using Result with unit type
    let result: Result<(), BearDogError> = Ok(());
    assert!(result.is_ok());

    let result: Result<(), BearDogError> = Err(BearDogError::validation("Unit error"));
    assert!(result.is_err());
}

// ============================================================================
// Concurrent Error Handling Edge Cases
// ============================================================================

/// Tests error creation in tight loop (stress test)
///
/// `TEST_CATEGORY`: unit
/// `TEST_DOMAIN`: errors
/// `TEST_PRIORITY`: normal
#[test]
fn test_rapid_error_creation() {
    // When: creating thousands of errors rapidly
    let errors: Vec<BearDogError> = (0..10000)
        .map(|i| BearDogError::internal(format!("Error {}", i)))
        .collect();

    // Then: all should be created successfully
    assert_eq!(errors.len(), 10000);
}

/// Tests error handling with maximum recursion depth
///
/// `TEST_CATEGORY`: unit
/// `TEST_DOMAIN`: errors
/// `TEST_PRIORITY`: normal
#[test]
fn test_error_with_recursive_validation() {
    // Helper function that validates recursively
    fn validate_recursive(depth: u32) -> Result<(), BearDogError> {
        if depth == 0 {
            return Err(BearDogError::validation("Max depth reached"));
        }
        if depth > 100 {
            return Err(BearDogError::validation("Depth too high"));
        }
        validate_recursive(depth - 1)?;
        Ok(())
    }

    // When: validating with recursion
    let result = validate_recursive(50);

    // Then: should handle recursion properly
    assert!(result.is_err());
}

// ============================================================================
// Boundary Value Edge Cases
// ============================================================================

/// Tests error handling with maximum usize value
///
/// `TEST_CATEGORY`: unit
/// `TEST_DOMAIN`: errors
/// `TEST_PRIORITY`: normal
#[test]
fn test_error_with_max_numeric_values() {
    // When: creating errors with boundary numeric values
    let error1 = BearDogError::validation(&format!("Index out of bounds: {}", usize::MAX));
    let error2 = BearDogError::validation(&format!("Negative value: {}", i64::MIN));
    let error3 = BearDogError::validation(&format!("Maximum value: {}", i64::MAX));

    // Then: all should format correctly
    let _ = format!("{}", error1);
    let _ = format!("{}", error2);
    let _ = format!("{}", error3);
}

/// Tests error with zero-length string operations
///
/// `TEST_CATEGORY`: unit
/// `TEST_DOMAIN`: errors
/// `TEST_PRIORITY`: normal
#[test]
fn test_error_with_empty_strings() {
    // Helper function that processes strings
    fn process_string(s: &str) -> Result<usize, BearDogError> {
        if s.is_empty() {
            return Err(BearDogError::validation("String cannot be empty"));
        }
        Ok(s.len())
    }

    // When: processing empty string
    let result = process_string("");

    // Then: should return appropriate error
    assert!(result.is_err());

    // When: processing string with only null byte
    let result = process_string("\0");

    // Then: should succeed (non-empty)
    assert!(result.is_ok());
}

// ============================================================================
// Error Transformation Edge Cases
// ============================================================================

/// Tests chaining multiple error transformations
///
/// `TEST_CATEGORY`: unit
/// `TEST_DOMAIN`: errors
/// `TEST_PRIORITY`: normal
#[test]
fn test_multiple_error_transformations() {
    // Helper function chain
    fn step_1() -> Result<i32, BearDogError> {
        Err(BearDogError::internal("Step 1 failed".to_string()))
    }

    fn step_2() -> Result<i32, BearDogError> {
        step_1().map_err(|_| BearDogError::validation("Step 2 fallback failed"))
    }

    fn step_3() -> Result<i32, BearDogError> {
        step_2().map_err(|_| BearDogError::network("Step 3 fallback failed".to_string()))
    }

    // When: chaining multiple error transformations
    let result = step_3();

    // Then: should end with final transformation
    assert!(result.is_err());
}

/// Tests error handling with unusual type conversions
///
/// `TEST_CATEGORY`: unit
/// `TEST_DOMAIN`: errors
/// `TEST_PRIORITY`: normal
#[test]
fn test_error_with_type_conversions() {
    // When: converting between different error-returning types
    let result1: Result<String, BearDogError> = Err(BearDogError::validation("Type error"));
    let result2: Result<i32, BearDogError> = result1.map(|_| 42);

    // Then: error should be preserved through transformation
    assert!(result2.is_err());
}

// ============================================================================
// Async Error Handling Edge Cases
// ============================================================================

/// Tests async error handling with immediate return
///
/// `TEST_CATEGORY`: unit
/// `TEST_DOMAIN`: errors
/// `TEST_PRIORITY`: normal
#[tokio::test]
async fn test_async_error_immediate() {
    // Helper async function
    async fn async_operation() -> Result<(), BearDogError> {
        Err(BearDogError::internal("Async error".to_string()))
    }

    // When: calling async operation
    let result = async_operation().await;

    // Then: should handle error correctly
    assert!(result.is_err());
}

/// Tests async error propagation through multiple awaits
///
/// `TEST_CATEGORY`: unit
/// `TEST_DOMAIN`: errors
/// `TEST_PRIORITY`: normal
#[tokio::test]
async fn test_async_error_propagation() {
    // Helper async functions
    async fn async_step_3() -> Result<i32, BearDogError> {
        Err(BearDogError::network("Network error".to_string()))
    }

    async fn async_step_2() -> Result<i32, BearDogError> {
        async_step_3().await
    }

    async fn async_step_1() -> Result<i32, BearDogError> {
        async_step_2().await
    }

    // When: propagating error through async chain
    let result = async_step_1().await;

    // Then: error should propagate correctly
    assert!(result.is_err());
}

/// Tests concurrent async error handling
///
/// `TEST_CATEGORY`: unit
/// `TEST_DOMAIN`: errors
/// `TEST_PRIORITY`: normal
#[tokio::test]
async fn test_concurrent_async_errors() {
    // Helper async function that may fail
    async fn async_task(id: u32) -> Result<u32, BearDogError> {
        if id % 2 == 0 {
            Ok(id)
        } else {
            Err(BearDogError::validation(&format!("Task {} failed", id)))
        }
    }

    // When: running multiple async tasks
    let tasks = vec![
        tokio::spawn(async_task(1)),
        tokio::spawn(async_task(2)),
        tokio::spawn(async_task(3)),
        tokio::spawn(async_task(4)),
    ];

    // Then: some should succeed, some should fail
    let mut success_count = 0;
    let mut error_count = 0;

    for task in tasks {
        match task.await.unwrap() {
            Ok(_) => success_count += 1,
            Err(_) => error_count += 1,
        }
    }

    assert_eq!(success_count, 2, "Even-numbered tasks should succeed");
    assert_eq!(error_count, 2, "Odd-numbered tasks should fail");
}

// ============================================================================
// Memory and Resource Edge Cases
// ============================================================================

/// Tests error handling doesn't leak memory with repeated operations
///
/// `TEST_CATEGORY`: unit
/// `TEST_DOMAIN`: errors
/// `TEST_PRIORITY`: normal
#[test]
fn test_no_memory_leak_on_error_creation() {
    // When: creating and dropping many errors
    for _ in 0..1000 {
        let _error = BearDogError::internal("Memory test error".to_string());
        // Error dropped here
    }

    // Then: should complete without issue (memory leak would cause slowdown)
}

/// Tests error with very nested Result/Option combinations
///
/// `TEST_CATEGORY`: unit
/// `TEST_DOMAIN`: errors
/// `TEST_PRIORITY`: normal
#[test]
fn test_nested_result_option_edge_case() {
    // When: dealing with nested Result<Option<Result<T>>>
    fn complex_operation() -> Result<Option<Result<i32, BearDogError>>, BearDogError> {
        Ok(Some(Err(BearDogError::validation("Nested error"))))
    }

    // Then: should handle complex nesting
    let result = complex_operation();
    assert!(result.is_ok());

    let inner = result.unwrap();
    assert!(inner.is_some());

    let inner_result = inner.unwrap();
    assert!(inner_result.is_err());
}

// ============================================================================
// Edge Cases Summary
// ============================================================================

/// Tests that all error variants can be created and formatted
///
/// `TEST_CATEGORY`: unit
/// `TEST_DOMAIN`: errors
/// `TEST_PRIORITY`: high
#[test]
fn test_all_error_variants_edge_cases() {
    // When: creating all error types with edge case inputs
    let errors = vec![
        BearDogError::validation(""),
        BearDogError::configuration(""),
        BearDogError::invalid_input(""),
        BearDogError::not_found("".to_string()),
        BearDogError::network("".to_string()),
        BearDogError::internal("".to_string()),
    ];

    // Then: all should be valid
    for error in errors {
        let _ = format!("{}", error);
        let _ = format!("{:?}", error);
    }
}
