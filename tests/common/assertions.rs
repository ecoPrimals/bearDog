// BearDog - Enterprise Security Ecosystem
// Copyright (C) 2025 EcoPrimals
//
// This program is free software: you can redistribute it and/or modify
// it under the terms of the GNU Affero General Public License as published by
// the Free Software Foundation, either version 3 of the License, or
// (at your option) any later version.
//
// This program is distributed in the hope that it will be useful,
// but WITHOUT ANY WARRANTY; without even the implied warranty of
// MERCHANTABILITY or FITNESS FOR A PARTICULAR PURPOSE. See the
// GNU Affero General Public License for more details.
//
// You should have received a copy of the GNU Affero General Public License
// along with this program. If not, see <https://www.gnu.org/licenses/>.


//! Modern Test Assertions with Unified Error Handling
//!
//! **Safe, Robust Test Assertions for BearDog**
//!
//! This module provides comprehensive test assertion utilities that integrate
//! with BearDog's unified error system, replacing panic-prone patterns with
//! sophisticated error handling and rich debugging information.

use beardog_errors::{BearDogError, BearDogResult};
use serde_json::Value as JsonValue;
use std::{
    collections::HashMap,
    fmt::Debug,
    time::Duration,
};

/// Test assertion result type
// Moved to beardog-types/src/aliases.rs for centralization
pub use beardog_types::aliases::AssertionResult;

/// Comprehensive assertion context with debugging information
#[derive(Debug, Clone)]
pub struct AssertionContext {
    pub assertion_type: String,
    pub expected: Option<String>,
    pub actual: Option<String>,
    pub additional_info: HashMap<String, JsonValue>,
    pub file: String,
    pub line: u32,
}

impl AssertionContext {
    /// Create new assertion context
    pub fn new(assertion_type: impl Into<String>) -> Self {
        Self {
            assertion_type: assertion_type.into(),
            expected: None,
            actual: None,
            additional_info: HashMap::new(),
            file: String::new(),
            line: 0,
        }
    }

    /// Add expected value information
    pub fn with_expected(mut self, expected: impl Into<String>) -> Self {
        self.expected = Some(expected.into());
        self
    }

    /// Add actual value information
    pub fn with_actual(mut self, actual: impl Into<String>) -> Self {
        self.actual = Some(actual.into());
        self
    }

    /// Add additional debugging information
    pub fn with_info(mut self, key: impl Into<String>, value: JsonValue) -> Self {
        self.additional_info.insert(key.into(), value);
        self
    }

    /// Set source location information
    pub fn with_location(mut self, file: impl Into<String>, line: u32) -> Self {
        self.file = file.into();
        self.line = line;
        self
    }
}

/// Assert that a result is successful with rich error context
pub fn assert_success<T: Debug>(
    result: &BearDogResult<T>,
    context: Option<AssertionContext>,
) -> AssertionResult<()> {
    match result {
        Ok(_) => Ok(()),
        Err(error) => {
            let ctx = context.unwrap_or_else(|| AssertionContext::new("assert_success"));
            
            Err(BearDogError::enhanced(
                "ASSERTION_FAILED",
                format!(
                    "Expected success but got error: {}. Context: {}",
                    error,
                    ctx.assertion_type
                ),
                beardog_errors::ErrorSeverity::High,
                beardog_errors::ErrorCategory::Validation,
                "test_assertions",
                "assert_success",
                vec![
                    "Review the operation that was expected to succeed".to_string(),
                    format!("Original error: {}", error),
                    "Check test setup and preconditions".to_string(),
                ],
            ))
        }
    }
}

/// Assert that a result contains a specific error pattern
pub fn assert_error_contains<T: Debug>(
    result: &BearDogResult<T>,
    expected_error: &str,
    context: Option<AssertionContext>,
) -> AssertionResult<()> {
    match result {
        Ok(value) => {
            let ctx = context.unwrap_or_else(|| AssertionContext::new("assert_error_contains"));
            
            Err(BearDogError::enhanced(
                "ASSERTION_FAILED",
                format!(
                    "Expected error containing '{}' but got success: {:?}. Context: {}",
                    expected_error,
                    value,
                    ctx.assertion_type
                ),
                beardog_errors::ErrorSeverity::High,
                beardog_errors::ErrorCategory::Validation,
                "test_assertions",
                "assert_error_contains",
                vec![
                    format!("Expected error pattern: '{}'", expected_error),
                    "Verify that the operation should actually fail".to_string(),
                    "Check test logic and error conditions".to_string(),
                ],
            ))
        }
        Err(error) => {
            let error_str = format!("{:?}", error);
            if error_str.contains(expected_error) {
                Ok(())
            } else {
                let ctx = context.unwrap_or_else(|| AssertionContext::new("assert_error_contains"));
                
                Err(BearDogError::enhanced(
                    "ASSERTION_FAILED",
                    format!(
                        "Expected error containing '{}' but got different error. Context: {}",
                        expected_error,
                        ctx.assertion_type
                    ),
                    beardog_errors::ErrorSeverity::High,
                    beardog_errors::ErrorCategory::Validation,
                    "test_assertions",
                    "assert_error_contains",
                    vec![
                        format!("Expected error pattern: '{}'", expected_error),
                        format!("Actual error: {}", error_str),
                        "Verify the expected error pattern is correct".to_string(),
                    ],
                ))
            }
        }
    }
}

/// Assert equality with rich debugging information
pub fn assert_eq<T: Debug + PartialEq>(
    left: &T,
    right: &T,
    context: Option<AssertionContext>,
) -> AssertionResult<()> {
    if left == right {
        Ok(())
    } else {
        let ctx = context.unwrap_or_else(|| AssertionContext::new("assert_eq"));
        
        Err(BearDogError::enhanced(
            "ASSERTION_FAILED",
            format!("Values are not equal. Context: {}", ctx.assertion_type),
            beardog_errors::ErrorSeverity::High,
            beardog_errors::ErrorCategory::Validation,
            "test_assertions",
            "assert_eq",
            vec![
                format!("Left value: {:?}", left),
                format!("Right value: {:?}", right),
                "Review the values being compared".to_string(),
            ],
        ))
    }
}

/// Assert inequality with rich debugging information
pub fn assert_ne<T: Debug + PartialEq>(
    left: &T,
    right: &T,
    context: Option<AssertionContext>,
) -> AssertionResult<()> {
    if left != right {
        Ok(())
    } else {
        let ctx = context.unwrap_or_else(|| AssertionContext::new("assert_ne"));
        
        Err(BearDogError::enhanced(
            "ASSERTION_FAILED",
            format!("Values should not be equal but they are. Context: {}", ctx.assertion_type),
            beardog_errors::ErrorSeverity::High,
            beardog_errors::ErrorCategory::Validation,
            "test_assertions",
            "assert_ne",
            vec![
                format!("Both values: {:?}", left),
                "Verify that the values should actually be different".to_string(),
            ],
        ))
    }
}

/// Assert that a condition is true with context
pub fn assert_true(
    condition: bool,
    message: impl Into<String>,
    context: Option<AssertionContext>,
) -> AssertionResult<()> {
    if condition {
        Ok(())
    } else {
        let ctx = context.unwrap_or_else(|| AssertionContext::new("assert_true"));
        let msg = message.into();
        
        Err(BearDogError::enhanced(
            "ASSERTION_FAILED",
            format!("Condition is false: {}. Context: {}", msg, ctx.assertion_type),
            beardog_errors::ErrorSeverity::High,
            beardog_errors::ErrorCategory::Validation,
            "test_assertions",
            "assert_true",
            vec![
                format!("Failed condition: {}", msg),
                "Review the condition logic".to_string(),
            ],
        ))
    }
}

/// Assert that a condition is false with context
pub fn assert_false(
    condition: bool,
    message: impl Into<String>,
    context: Option<AssertionContext>,
) -> AssertionResult<()> {
    if !condition {
        Ok(())
    } else {
        let ctx = context.unwrap_or_else(|| AssertionContext::new("assert_false"));
        let msg = message.into();
        
        Err(BearDogError::enhanced(
            "ASSERTION_FAILED",
            format!("Condition is true but should be false: {}. Context: {}", msg, ctx.assertion_type),
            beardog_errors::ErrorSeverity::High,
            beardog_errors::ErrorCategory::Validation,
            "test_assertions",
            "assert_false",
            vec![
                format!("Expected false condition: {}", msg),
                "Review the condition logic".to_string(),
            ],
        ))
    }
}

/// Assert that a collection contains an item
pub fn assert_contains<T: Debug + PartialEq>(
    collection: &[T],
    item: &T,
    context: Option<AssertionContext>,
) -> AssertionResult<()> {
    if collection.contains(item) {
        Ok(())
    } else {
        let ctx = context.unwrap_or_else(|| AssertionContext::new("assert_contains"));
        
        Err(BearDogError::enhanced(
            "ASSERTION_FAILED",
            format!("Collection does not contain expected item. Context: {}", ctx.assertion_type),
            beardog_errors::ErrorSeverity::High,
            beardog_errors::ErrorCategory::Validation,
            "test_assertions",
            "assert_contains",
            vec![
                format!("Expected item: {:?}", item),
                format!("Collection: {:?}", collection),
                "Verify the item should be in the collection".to_string(),
            ],
        ))
    }
}

/// Assert that a collection does not contain an item
pub fn assert_not_contains<T: Debug + PartialEq>(
    collection: &[T],
    item: &T,
    context: Option<AssertionContext>,
) -> AssertionResult<()> {
    if !collection.contains(item) {
        Ok(())
    } else {
        let ctx = context.unwrap_or_else(|| AssertionContext::new("assert_not_contains"));
        
        Err(BearDogError::enhanced(
            "ASSERTION_FAILED",
            format!("Collection contains item that should not be present. Context: {}", ctx.assertion_type),
            beardog_errors::ErrorSeverity::High,
            beardog_errors::ErrorCategory::Validation,
            "test_assertions",
            "assert_not_contains",
            vec![
                format!("Unexpected item: {:?}", item),
                format!("Collection: {:?}", collection),
                "Verify the item should not be in the collection".to_string(),
            ],
        ))
    }
}

/// Assert that a collection is empty
pub fn assert_empty<T: Debug>(
    collection: &[T],
    context: Option<AssertionContext>,
) -> AssertionResult<()> {
    if collection.is_empty() {
        Ok(())
    } else {
        let ctx = context.unwrap_or_else(|| AssertionContext::new("assert_empty"));
        
        Err(BearDogError::enhanced(
            "ASSERTION_FAILED",
            format!("Collection should be empty but contains {} items. Context: {}", 
                   collection.len(), ctx.assertion_type),
            beardog_errors::ErrorSeverity::High,
            beardog_errors::ErrorCategory::Validation,
            "test_assertions",
            "assert_empty",
            vec![
                format!("Collection size: {}", collection.len()),
                format!("Collection contents: {:?}", collection),
                "Verify the collection should actually be empty".to_string(),
            ],
        ))
    }
}

/// Assert that a collection has a specific length
pub fn assert_len<T: Debug>(
    collection: &[T],
    expected_len: usize,
    context: Option<AssertionContext>,
) -> AssertionResult<()> {
    if collection.len() == expected_len {
        Ok(())
    } else {
        let ctx = context.unwrap_or_else(|| AssertionContext::new("assert_len"));
        
        Err(BearDogError::enhanced(
            "ASSERTION_FAILED",
            format!("Collection length mismatch. Context: {}", ctx.assertion_type),
            beardog_errors::ErrorSeverity::High,
            beardog_errors::ErrorCategory::Validation,
            "test_assertions",
            "assert_len",
            vec![
                format!("Expected length: {}", expected_len),
                format!("Actual length: {}", collection.len()),
                format!("Collection contents: {:?}", collection),
            ],
        ))
    }
}

/// Assert that a duration is within acceptable bounds
pub fn assert_duration_within(
    actual: Duration,
    expected: Duration,
    tolerance: Duration,
    context: Option<AssertionContext>,
) -> AssertionResult<()> {
    let diff = if actual > expected { actual - expected } else { expected - actual };
    
    if diff <= tolerance {
        Ok(())
    } else {
        let ctx = context.unwrap_or_else(|| AssertionContext::new("assert_duration_within"));
        
        Err(BearDogError::enhanced(
            "ASSERTION_FAILED",
            format!("Duration outside acceptable tolerance. Context: {}", ctx.assertion_type),
            beardog_errors::ErrorSeverity::High,
            beardog_errors::ErrorCategory::Validation,
            "test_assertions",
            "assert_duration_within",
            vec![
                format!("Expected duration: {:?}", expected),
                format!("Actual duration: {:?}", actual),
                format!("Tolerance: {:?}", tolerance),
                format!("Difference: {:?}", diff),
            ],
        ))
    }
}

/// Assert that a value is within a numeric range
pub fn assert_in_range<T: Debug + PartialOrd + Copy>(
    value: T,
    min: T,
    max: T,
    context: Option<AssertionContext>,
) -> AssertionResult<()> {
    if value >= min && value <= max {
        Ok(())
    } else {
        let ctx = context.unwrap_or_else(|| AssertionContext::new("assert_in_range"));
        
        Err(BearDogError::enhanced(
            "ASSERTION_FAILED",
            format!("Value outside expected range. Context: {}", ctx.assertion_type),
            beardog_errors::ErrorSeverity::High,
            beardog_errors::ErrorCategory::Validation,
            "test_assertions",
            "assert_in_range",
            vec![
                format!("Value: {:?}", value),
                format!("Expected range: {:?} to {:?}", min, max),
                "Verify the expected range is correct".to_string(),
            ],
        ))
    }
}

/// Macro for convenient assertion with automatic context
#[macro_export]
macro_rules! assert_beardog {
    ($assertion:expr, $message:expr) => {
        $assertion.map_err(|e| {
            beardog_errors::BearDogError::enhanced(
                "ASSERTION_FAILED",
                format!("Assertion failed: {} - {}", $message, e),
                beardog_errors::ErrorSeverity::High,
                beardog_errors::ErrorCategory::Validation,
                "test_assertions",
                "assert_beardog_macro",
                vec![
                    format!("Assertion: {}", $message),
                    format!("Location: {}:{}", file!(), line!()),
                ],
            )
        })?;
    };
}

/// Convenience macro for success assertions
#[macro_export]
macro_rules! assert_ok {
    ($result:expr) => {
        $crate::tests::common::assertions::assert_success(
            &$result,
            Some($crate::tests::common::assertions::AssertionContext::new("assert_ok")
                .with_location(file!(), line!()))
        )?;
    };
    ($result:expr, $context:expr) => {
        $crate::tests::common::assertions::assert_success(&$result, Some($context))?;
    };
}

/// Convenience macro for error assertions
#[macro_export]
macro_rules! assert_err_contains {
    ($result:expr, $expected:expr) => {
        $crate::tests::common::assertions::assert_error_contains(
            &$result,
            $expected,
            Some($crate::tests::common::assertions::AssertionContext::new("assert_err_contains")
                .with_location(file!(), line!()))
        )?;
    };
    ($result:expr, $expected:expr, $context:expr) => {
        $crate::tests::common::assertions::assert_error_contains(&$result, $expected, Some($context))?;
    };
} 