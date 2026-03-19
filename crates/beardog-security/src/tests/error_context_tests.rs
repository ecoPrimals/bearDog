// SPDX-License-Identifier: AGPL-3.0-only

//! Error Context Tests
//!
//! Tests for error handling, context, and propagation.

#[cfg(test)]
mod tests {
    #[test]
    fn test_error_display_impl() {
        // Test that errors have Display implementation
        use beardog_errors::BearDogError;

        let error = BearDogError::internal("test error".to_string());
        let display = format!("{}", error);

        assert!(!display.is_empty(), "Error display should not be empty");
    }

    #[test]
    fn test_error_debug_impl() {
        // Test that errors have Debug implementation
        use beardog_errors::BearDogError;

        let error = BearDogError::internal("test error".to_string());
        let debug = format!("{:?}", error);

        assert!(!debug.is_empty(), "Error debug should not be empty");
    }

    #[test]
    fn test_error_send_sync() {
        // Test that errors are Send + Sync
        fn assert_send_sync<T: Send + Sync>() {}
        assert_send_sync::<beardog_errors::BearDogError>();
    }

    #[test]
    fn test_result_ok_propagation() {
        // Test that Ok results propagate correctly
        fn returns_ok() -> Result<i32, beardog_errors::BearDogError> {
            Ok(42)
        }

        let result = returns_ok();
        assert!(result.is_ok());
        assert_eq!(result.unwrap(), 42);
    }

    #[test]
    // TEST_CATEGORY: integration
    // TEST_DOMAIN: security
    // TEST_PRIORITY: important
    fn test_result_err_propagation() {
        // Test that Err results propagate correctly
        fn returns_err() -> Result<i32, beardog_errors::BearDogError> {
            Err(beardog_errors::BearDogError::internal(
                "test error".to_string(),
            ))
        }

        // TEST_CATEGORY: integration
        // TEST_DOMAIN: security
        // TEST_PRIORITY: important
        let result = returns_err();
        assert!(result.is_err());
    }

    #[test]
    fn test_result_question_mark_operator() {
        // Test that ? operator works with our errors
        fn inner() -> Result<i32, beardog_errors::BearDogError> {
            // TEST_CATEGORY: integration
            // TEST_DOMAIN: security
            // TEST_PRIORITY: important
            Err(beardog_errors::BearDogError::internal(
                "inner error".to_string(),
            ))
        }
        // TEST_CATEGORY: integration
        // TEST_DOMAIN: security
        // TEST_PRIORITY: normal

        fn outer() -> Result<i32, beardog_errors::BearDogError> {
            let _value = inner()?;
            Ok(42)
        }

        let result = outer();
        assert!(result.is_err());
    }
    // TEST_CATEGORY: integration
    // TEST_DOMAIN: security
    // TEST_PRIORITY: normal

    #[test]
    fn test_error_chain() {
        // Test error chaining
        use beardog_errors::BearDogError;

        let error1 = BearDogError::internal("root cause".to_string());
        let error2 = BearDogError::internal(format!("wrapped: {}", error1));

        // Test passes if error chaining compiles and runs
        // TEST_CATEGORY: integration
        // TEST_DOMAIN: security
        // TEST_PRIORITY: normal
        assert!(error2.to_string().contains("wrapped"));
    }

    #[test]
    fn test_error_type_conversion() {
        // Test that errors can be created from different types
        use beardog_errors::BearDogError;

        let error = BearDogError::internal("string error".to_string());
        let error2 = BearDogError::internal(String::from("owned string error"));

        // Test passes if type conversions compile and create valid errors
        assert!(!error.to_string().is_empty());
        assert!(!error2.to_string().is_empty());
    }
    // TEST_CATEGORY: integration
    // TEST_DOMAIN: security
    // TEST_PRIORITY: important

    #[test]
    fn test_result_map() {
        // Test Result::map operations
        let result: Result<i32, beardog_errors::BearDogError> = Ok(21);
        let doubled = result.map(|x| x * 2);

        assert_eq!(doubled.unwrap(), 42);
    }
    // TEST_CATEGORY: integration
    // TEST_DOMAIN: security
    // TEST_PRIORITY: important

    #[test]
    fn test_result_map_err() {
        // Test Result::map_err operations
        use beardog_errors::BearDogError;

        let result: Result<i32, &str> = Err("string error");
        let converted = result.map_err(|e| BearDogError::internal(e.to_string()));

        assert!(converted.is_err());
        // TEST_CATEGORY: integration
        // TEST_DOMAIN: security
        // TEST_PRIORITY: normal
    }

    #[test]
    fn test_result_and_then() {
        // Test Result::map chaining (more idiomatic than and_then with Ok)
        let result: Result<i32, beardog_errors::BearDogError> = Ok(21);
        // TEST_CATEGORY: integration
        // TEST_DOMAIN: security
        // TEST_PRIORITY: normal
        let chained = result.map(|x| x * 2);

        assert_eq!(chained.unwrap(), 42);
    }

    #[test]
    #[allow(clippy::unnecessary_lazy_evaluations)]
    fn test_result_or_else() {
        // TEST_CATEGORY: integration
        // TEST_DOMAIN: security
        // TEST_PRIORITY: normal
        // Test Result::or_else fallback
        let result: Result<i32, beardog_errors::BearDogError> =
            Err(beardog_errors::BearDogError::internal("error".to_string()));
        let fallback: Result<i32, beardog_errors::BearDogError> = result.or_else(|_| Ok(42));

        assert_eq!(fallback.unwrap(), 42);
        // TEST_CATEGORY: integration
        // TEST_DOMAIN: security
        // TEST_PRIORITY: normal
    }

    #[test]
    #[allow(clippy::unwrap_or_default)]
    #[allow(clippy::manual_unwrap_or_default)]
    #[allow(clippy::unnecessary_literal_unwrap)]
    fn test_result_unwrap_or() {
        // Test Result::unwrap_or with default
        // TEST_CATEGORY: integration
        // TEST_DOMAIN: security
        // TEST_PRIORITY: normal
        let result: Result<i32, beardog_errors::BearDogError> = Ok(42);
        let value = result.unwrap_or(0);

        assert_eq!(value, 42);
    }

    #[test]
    #[allow(clippy::unnecessary_lazy_evaluations)]
    #[allow(clippy::unwrap_or_default)]
    #[allow(clippy::manual_unwrap_or_default)]
    #[allow(clippy::unnecessary_literal_unwrap)]
    fn test_result_unwrap_or_else() {
        // Test Result::unwrap_or_else with closure
        let result: Result<i32, beardog_errors::BearDogError> = Ok(42);
        let value = result.unwrap_or_else(|_| 0);

        assert_eq!(value, 42);
    }

    #[test]
    fn test_result_ok_method() {
        // Test Result::ok conversion to Option
        // TEST_CATEGORY: integration
        // TEST_DOMAIN: security
        // TEST_PRIORITY: normal
        let result: Result<i32, beardog_errors::BearDogError> = Ok(42);
        let option = result.ok();

        assert_eq!(option, Some(42));
    }

    // TEST_CATEGORY: integration
    // TEST_DOMAIN: security
    // TEST_PRIORITY: normal
    #[test]
    fn test_result_err_method() {
        // Test Result::err conversion to Option
        use beardog_errors::BearDogError;

        let result: Result<i32, BearDogError> = Err(BearDogError::internal("error".to_string()));
        let option = result.err();

        assert!(option.is_some());
    }
}
