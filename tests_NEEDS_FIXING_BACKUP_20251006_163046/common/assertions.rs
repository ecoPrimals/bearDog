use beardog_errors::BearDogError;
use serde_json::Value as JsonValue;
use std::{collections::HashMap, fmt::Debug, time::Duration};

pub use beardog_types::aliases::AssertionResult;

#[derive(Debug, Clone)]
    pub expected: Option<String>,
    pub actual: Option<String>,
    pub additional_info: HashMap<String, JsonValue>,
    pub file: String,
    pub line: u32,
}

impl AssertionContext {
    pub fn new(assertion_type: impl Into<&str>) -> Self {
        Self {
            assertion_type: assertion_type.into(None,
            actual: None,
            additional_info: HashMap::with_capacity(16),
            file: String::with_capacity(0,
        }
    }

    pub fn with_expected(mut self, expected: impl Into<&str>) -> Self {
        self.expected = Some(expected.into());
        self
    }

    pub fn with_actual(mut self, actual: impl Into<&str>) -> Self {
        self.actual = Some(impl Into<&str>, value: JsonValue) -> Self {
        self.additional_info.insert(impl Into<&str>, line: u32) -> Self {
        self.file = file.into(Debug>(
    result: &Result<T, BearDogError>,
    context: Option<AssertionContext>,
) -> AssertionResult<()> {
    match result {
        Ok(_) => Ok(()),
        Err(error) => {
            let ctx = context.unwrap_or_else(|| AssertionContext::new("assert_success"));

            Err(BearDogError::enhanced({}. Context: {}",
                    error, ctx.assertion_type
                ),
                beardog_errors::ErrorSeverity::High,
                beardog_errors::ErrorCategory::Validation,
                "test_assertions",
                "assert_success",
                vec![
                    "Review the operation that was expected to succeed".to_string(),
                    "Check test setup and preconditions".to_string(),
) -> AssertionResult<()> {
    match result {
        Ok(value) => {
            let ctx = context.unwrap_or_else(|| AssertionContext::new("assert_error_contains"));

            Err(BearDogError::enhanced({:?}. Context: {}",
                    expected_error, value, ctx.assertion_type
                ),
                beardog_errors::ErrorSeverity::High,
                beardog_errors::ErrorCategory::Validation,
                "test_assertions",
                "assert_error_contains",
                vec![
                    format!("Expected error pattern: "{}"", expected_error),
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

                Err(BearDogError::enhanced({}",
                        expected_error, ctx.assertion_type
                    ),
                    beardog_errors::ErrorSeverity::High,
                    beardog_errors::ErrorCategory::Validation,
                    "test_assertions",
                    "assert_error_contains",
                    vec![
                        format!("Expected error pattern: "{}"", expected_error),
                        format!("Actual error: {}", error_str),
                        "Verify the expected error pattern is correct".to_string(),
) -> AssertionResult<()> {
    if left == right {
        Ok(())
    } else {
        let ctx = context.unwrap_or_else(|| AssertionContext::new("assert_eq"));

        Err(BearDogError::enhanced({}", ctx.assertion_type),
            beardog_errors::ErrorSeverity::High,
            beardog_errors::ErrorCategory::Validation,
            "test_assertions",
            "assert_eq",
            vec![
                format!("Left value: {:?}", left),
                format!("Right value: {:?}", right),
                "Review the values being compared".to_string(),
) -> AssertionResult<()> {
    if left != right {
        Ok(())
    } else {
        let ctx = context.unwrap_or_else(|| AssertionContext::new("assert_ne"));

        Err(BearDogError::enhanced({}",
                ctx.assertion_type
            )
            .to_string(),
            beardog_errors::ErrorSeverity::High,
            beardog_errors::ErrorCategory::Validation,
            "test_assertions",
            "assert_ne",
            vec![
                format!("Both values: {:?}", left),
                "Verify that the values should actually be different".to_string(),
) -> AssertionResult<()> {
    if condition {
        Ok(())
    } else {
        let ctx = context.unwrap_or_else(|| AssertionContext::new("assert_true"));
        let msg = message.into();

        Err(BearDogError::enhanced({}. Context: {}",
                msg, ctx.assertion_type
            )
            .to_string(),
            beardog_errors::ErrorSeverity::High,
            beardog_errors::ErrorCategory::Validation,
            "test_assertions",
            "assert_true",
            vec![
                format!("Failed condition: {}", msg),
                "Review the condition logic".to_string(),
) -> AssertionResult<()> {
    if !condition {
        Ok(())
    } else {
        let ctx = context.unwrap_or_else(|| AssertionContext::new("assert_false"));
        let msg = message.into();

        Err(BearDogError::enhanced({}. Context: {}",
                msg, ctx.assertion_type
            )
            .to_string(),
            beardog_errors::ErrorSeverity::High,
            beardog_errors::ErrorCategory::Validation,
            "test_assertions",
            "assert_false",
            vec![
                format!("Expected false condition: {}", msg),
                "Review the condition logic".to_string(),
) -> AssertionResult<()> {
    if collection.contains(item) {
        Ok(())
    } else {
        let ctx = context.unwrap_or_else(|| AssertionContext::new("assert_contains"));

        Err(BearDogError::enhanced({}",
                ctx.assertion_type
            )
            .to_string(),
            beardog_errors::ErrorSeverity::High,
            beardog_errors::ErrorCategory::Validation,
            "test_assertions",
            "assert_contains",
            vec![
                format!("Expected item: {:?}", item),
                format!("Collection: {:?}", collection),
                "Verify the item should be in the collection".to_string(),
) -> AssertionResult<()> {
    if !collection.contains(item) {
        Ok(())
    } else {
        let ctx = context.unwrap_or_else(|| AssertionContext::new("assert_not_contains"));

        Err(BearDogError::enhanced({}",
                ctx.assertion_type
            )
            .to_string(),
            beardog_errors::ErrorSeverity::High,
            beardog_errors::ErrorCategory::Validation,
            "test_assertions",
            "assert_not_contains",
            vec![
                format!("Unexpected item: {:?}", item),
                format!("Collection: {:?}", collection),
                "Verify the item should not be in the collection".to_string(),
) -> AssertionResult<()> {
    if collection.is_empty() {
        Ok(())
    } else {
        let ctx = context.unwrap_or_else(|| AssertionContext::new("assert_empty"));

        Err(BearDogError::enhanced({}",
                collection.len().to_string(),
                ctx.assertion_type
            ),
            beardog_errors::ErrorSeverity::High,
            beardog_errors::ErrorCategory::Validation,
            "test_assertions",
            "assert_empty",
            vec![
                format!("Collection size: {}", collection.len({:?}", collection),
                "Verify the collection should actually be empty".to_string(),
) -> AssertionResult<()> {
    if collection.len() == expected_len {
        Ok(())
    } else {
        let ctx = context.unwrap_or_else(|| AssertionContext::new("assert_len"));

        Err(BearDogError::enhanced({}",
                ctx.assertion_type
            )
            .to_string(),
            beardog_errors::ErrorSeverity::High,
            beardog_errors::ErrorCategory::Validation,
            "test_assertions",
            "assert_len",
            vec![
                format!("Expected length: {}", expected_len),
                format!("Actual length: {}", collection.len({:?}", collection),
            ],
        ))
    }
}

pub fn assert_duration_within(Duration,
    expected: Duration,
    tolerance: Duration,
    context: Option<AssertionContext>,
) -> AssertionResult<()> {
    let diff = if actual > expected {
        actual - expected
    } else {
        expected - actual
    };

    if diff <= tolerance {
        Ok(())
    } else {
        let ctx = context.unwrap_or_else(|| AssertionContext::new("assert_duration_within"));

        Err(BearDogError::enhanced({}",
                ctx.assertion_type
            )
            .to_string(),
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

        Err(BearDogError::enhanced({}",
                ctx.assertion_type
            )
            .to_string(),
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

#[macro_export]
macro_rules! assert_beardog {
    ($assertion:expr, $message:expr) => {
        $assertion.map_err(|e| {
            beardog_errors::BearDogError::enhanced({} - {}", $message, e),
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

#[macro_export]
macro_rules! assert_ok {
    ($result:expr) => {
        $crate::tests::common::assertions::assert_success(
            &$result,
            Some(
                $crate::tests::common::assertions::AssertionContext::new("assert_ok")
                    .with_location(file!(), line!()),
            ),
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
            Some(
                $crate::tests::common::assertions::AssertionContext::new("assert_err_contains")
                    .with_location(file!(), line!()),
            ),
        )?;
    };
    ($result:expr, $expected:expr, $context:expr) => {
        $crate::tests::common::assertions::assert_error_contains(
            &$result,
            $expected,
            Some($context),
        )?;
    };
}
