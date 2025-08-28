

use beardog_errors::BearDogError;
use serde_json::Value as JsonValue;
use std::{
    collections::HashMap,
    fmt::Debug,
    time::Duration,
};

pub use beardog_types::aliases::AssertionResult;

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

    pub fn new(assertion_type: impl Into<&str>) -> Self {
        Self {
            assertion_type: assertion_type.into(),
            expected: None,
            actual: None,
            additional_info: HashMap::with_capacity(16),
            file: String::with_capacity(64),
            line: 0,
        }
    }

    pub fn with_expected(mut self, expected: impl Into<&str>) -> Self {
        self.expected = Some(expected.into());
        self
    }

    pub fn with_actual(mut self, actual: impl Into<&str>) -> Self {
        self.actual = Some(actual.into());
        self
    }

    pub fn with_info(mut self, key: impl Into<&str>, value: JsonValue) -> Self {
        self.additional_info.insert(key.into(), value);
        self
    }

    pub fn with_location(mut self, file: impl Into<&str>, line: u32) -> Self {
        self.file = file.into();
        self.line = line;
        self
    }
}

pub fn assert_success<T: Debug>(
    result: &Result<T, BearDogError>,
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
                    format_args!("Original error: {}", error).to_string(),
                    "Check test setup and preconditions".to_string(),
                ],
            ))
        }
    }
}

pub fn assert_error_contains<T: Debug>(
    result: &Result<T, BearDogError>,
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
                    format_args!("Expected error pattern: '{}'", expected_error).to_string(),
                    "Verify that the operation should actually fail".to_string(),
                    "Check test logic and error conditions".to_string(),
                ],
            ))
        }
        Err(error) => {
            let error_str = format_args!("{:?}", error).to_string();
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
                        format_args!("Expected error pattern: '{}'", expected_error).to_string(),
                        format_args!("Actual error: {}", error_str).to_string(),
                        "Verify the expected error pattern is correct".to_string(),
                    ],
                ))
            }
        }
    }
}

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
            format_args!("Values are not equal. Context: {}", ctx.assertion_type).to_string(),
            beardog_errors::ErrorSeverity::High,
            beardog_errors::ErrorCategory::Validation,
            "test_assertions",
            "assert_eq",
            vec![
                format_args!("Left value: {:?}", left).to_string(),
                format_args!("Right value: {:?}", right).to_string(),
                "Review the values being compared".to_string(),
            ],
        ))
    }
}

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
            format_args!("Values should not be equal but they are. Context: {}", ctx.assertion_type).to_string(),
            beardog_errors::ErrorSeverity::High,
            beardog_errors::ErrorCategory::Validation,
            "test_assertions",
            "assert_ne",
            vec![
                format_args!("Both values: {:?}", left).to_string(),
                "Verify that the values should actually be different".to_string(),
            ],
        ))
    }
}

pub fn assert_true(
    condition: bool,
    message: impl Into<&str>,
    context: Option<AssertionContext>,
) -> AssertionResult<()> {
    if condition {
        Ok(())
    } else {
        let ctx = context.unwrap_or_else(|| AssertionContext::new("assert_true"));
        let msg = message.into();
        
        Err(BearDogError::enhanced(
            "ASSERTION_FAILED",
            format_args!("Condition is false: {}. Context: {}", msg, ctx.assertion_type).to_string(),
            beardog_errors::ErrorSeverity::High,
            beardog_errors::ErrorCategory::Validation,
            "test_assertions",
            "assert_true",
            vec![
                format_args!("Failed condition: {}", msg).to_string(),
                "Review the condition logic".to_string(),
            ],
        ))
    }
}

pub fn assert_false(
    condition: bool,
    message: impl Into<&str>,
    context: Option<AssertionContext>,
) -> AssertionResult<()> {
    if !condition {
        Ok(())
    } else {
        let ctx = context.unwrap_or_else(|| AssertionContext::new("assert_false"));
        let msg = message.into();
        
        Err(BearDogError::enhanced(
            "ASSERTION_FAILED",
            format_args!("Condition is true but should be false: {}. Context: {}", msg, ctx.assertion_type).to_string(),
            beardog_errors::ErrorSeverity::High,
            beardog_errors::ErrorCategory::Validation,
            "test_assertions",
            "assert_false",
            vec![
                format_args!("Expected false condition: {}", msg).to_string(),
                "Review the condition logic".to_string(),
            ],
        ))
    }
}

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
            format_args!("Collection does not contain expected item. Context: {}", ctx.assertion_type).to_string(),
            beardog_errors::ErrorSeverity::High,
            beardog_errors::ErrorCategory::Validation,
            "test_assertions",
            "assert_contains",
            vec![
                format_args!("Expected item: {:?}", item).to_string(),
                format_args!("Collection: {:?}", collection).to_string(),
                "Verify the item should be in the collection".to_string(),
            ],
        ))
    }
}

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
            format_args!("Collection contains item that should not be present. Context: {}", ctx.assertion_type).to_string(),
            beardog_errors::ErrorSeverity::High,
            beardog_errors::ErrorCategory::Validation,
            "test_assertions",
            "assert_not_contains",
            vec![
                format_args!("Unexpected item: {:?}", item).to_string(),
                format_args!("Collection: {:?}", collection).to_string(),
                "Verify the item should not be in the collection".to_string(),
            ],
        ))
    }
}

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
            format_args!("Collection should be empty but contains {} items. Context: {}", 
                   collection.len().to_string(), ctx.assertion_type),
            beardog_errors::ErrorSeverity::High,
            beardog_errors::ErrorCategory::Validation,
            "test_assertions",
            "assert_empty",
            vec![
                format_args!("Collection size: {}", collection.len().to_string()),
                format_args!("Collection contents: {:?}", collection).to_string(),
                "Verify the collection should actually be empty".to_string(),
            ],
        ))
    }
}

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
            format_args!("Collection length mismatch. Context: {}", ctx.assertion_type).to_string(),
            beardog_errors::ErrorSeverity::High,
            beardog_errors::ErrorCategory::Validation,
            "test_assertions",
            "assert_len",
            vec![
                format_args!("Expected length: {}", expected_len).to_string(),
                format_args!("Actual length: {}", collection.len().to_string()),
                format_args!("Collection contents: {:?}", collection).to_string(),
            ],
        ))
    }
}

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
            format_args!("Duration outside acceptable tolerance. Context: {}", ctx.assertion_type).to_string(),
            beardog_errors::ErrorSeverity::High,
            beardog_errors::ErrorCategory::Validation,
            "test_assertions",
            "assert_duration_within",
            vec![
                format_args!("Expected duration: {:?}", expected).to_string(),
                format_args!("Actual duration: {:?}", actual).to_string(),
                format_args!("Tolerance: {:?}", tolerance).to_string(),
                format_args!("Difference: {:?}", diff).to_string(),
            ],
        ))
    }
}

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
            format_args!("Value outside expected range. Context: {}", ctx.assertion_type).to_string(),
            beardog_errors::ErrorSeverity::High,
            beardog_errors::ErrorCategory::Validation,
            "test_assertions",
            "assert_in_range",
            vec![
                format_args!("Value: {:?}", value).to_string(),
                format_args!("Expected range: {:?} to {:?}", min, max).to_string(),
                "Verify the expected range is correct".to_string(),
            ],
        ))
    }
}

#[macro_export]
macro_rules! assert_beardog {
    ($assertion:expr, $message:expr) => {
        $assertion.map_err(|e| {
            beardog_errors::BearDogError::enhanced(
                "ASSERTION_FAILED",
                format_args!("Assertion failed: {} - {}", $message, e).to_string(),
                beardog_errors::ErrorSeverity::High,
                beardog_errors::ErrorCategory::Validation,
                "test_assertions",
                "assert_beardog_macro",
                vec![
                    format_args!("Assertion: {}", $message).to_string(),
                    format_args!("Location: {}:{}", file!().to_string(), line!()),
                ],
            )
        })?;
    };
}

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