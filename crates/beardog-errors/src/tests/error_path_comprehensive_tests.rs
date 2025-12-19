//! Comprehensive Error Path Tests
//!
//! Tests for error construction, conversion, and propagation scenarios.
//! Added October 29, 2025 - Test coverage improvement initiative

#[cfg(test)]
mod error_path_tests {
    use crate::*;

    #[test]
    fn test_invalid_input_error() {
        let error = BearDogError::invalid_input("Test error");
        let display = format!("{}", error);
        assert!(!display.is_empty());
    }

    #[test]
    fn test_result_ok_handling() {
        let result: Result<i32, BearDogError> = Ok(42);
        assert!(result.is_ok());
        // Extract value without unwrap on literal
        if let Ok(value) = result {
            assert_eq!(value, 42);
        } else {
            panic!("Expected Ok, got Err");
        }
    }

    #[test]
    fn test_result_err_handling() {
        let result: Result<i32, BearDogError> = Err(BearDogError::invalid_input("Test"));
        assert!(result.is_err());
        // Test error fallback without unwrap_or on literal
        match result {
            Ok(_) => panic!("Expected Err, got Ok"),
            Err(e) => {
                // Business error with validation category
                assert_eq!(e.to_string(), "Business error: Test");
            }
        }
    }

    #[test]
    fn test_error_propagation() {
        fn inner() -> Result<(), BearDogError> {
            Err(BearDogError::invalid_input("Inner error"))
        }

        fn outer() -> Result<(), BearDogError> {
            inner()?;
            Ok(())
        }

        assert!(outer().is_err());
    }

    #[test]
    fn test_result_map() {
        let result: Result<i32, BearDogError> = Ok(21);
        let mapped = result.map(|x| x * 2);
        // TEST_CATEGORY: integration
        // TEST_DOMAIN: errors
        // TEST_PRIORITY: important
        assert_eq!(mapped.unwrap(), 42);
    }

    #[test]
    // TEST_CATEGORY: integration
    // TEST_DOMAIN: errors
    // TEST_PRIORITY: normal
    fn test_result_and_then() {
        let result: Result<i32, BearDogError> = Ok(21);
        let chained = result.map(|x| x * 2);
        assert_eq!(chained.unwrap(), 42);
        // TEST_CATEGORY: integration
        // TEST_DOMAIN: errors
        // TEST_PRIORITY: normal
    }

    #[test]
    fn test_option_to_result() {
        // TEST_CATEGORY: integration
        // TEST_DOMAIN: errors
        // TEST_PRIORITY: important
        let some: Option<i32> = Some(42);
        let result = some.ok_or_else(|| BearDogError::invalid_input("Not found"));
        assert_eq!(result.unwrap(), 42);
    }

    #[test]
    fn test_option_none_to_result() {
        let none: Option<i32> = None;
        let result = none.ok_or_else(|| BearDogError::invalid_input("Not found"));
        assert!(result.is_err());
    }
    // TEST_CATEGORY: integration
    // TEST_DOMAIN: errors
    // TEST_PRIORITY: normal

    #[test]
    fn test_collect_results_success() {
        let results: Vec<Result<i32, BearDogError>> = vec![Ok(1), Ok(2), Ok(3)];
        // TEST_CATEGORY: integration
        // TEST_DOMAIN: errors
        // TEST_PRIORITY: normal
        let collected: Result<Vec<i32>, BearDogError> = results.into_iter().collect();
        assert_eq!(collected.unwrap(), vec![1, 2, 3]);
    }

    // TEST_CATEGORY: integration
    // TEST_DOMAIN: errors
    // TEST_PRIORITY: normal
    #[test]
    fn test_collect_results_with_error() {
        let results: Vec<Result<i32, BearDogError>> =
            vec![Ok(1), Err(BearDogError::invalid_input("Error")), Ok(3)];
        // TEST_CATEGORY: integration
        // TEST_DOMAIN: errors
        // TEST_PRIORITY: normal
        let collected: Result<Vec<i32>, BearDogError> = results.into_iter().collect();
        assert!(collected.is_err());
    }

    // TEST_CATEGORY: integration
    // TEST_DOMAIN: errors
    // TEST_PRIORITY: normal
    #[test]
    fn test_error_with_empty_message() {
        let error = BearDogError::invalid_input("");
        let display = format!("{}", error);
        // TEST_CATEGORY: integration
        // TEST_DOMAIN: errors
        // TEST_PRIORITY: important
        assert!(!display.is_empty());
    }

    #[test]
    fn test_error_with_long_message() {
        // TEST_CATEGORY: integration
        // TEST_DOMAIN: errors
        // TEST_PRIORITY: important
        let long_msg = "error ".repeat(100);
        let error = BearDogError::invalid_input(&long_msg);
        let display = format!("{}", error);
        assert!(!display.is_empty());
        // TEST_CATEGORY: integration
        // TEST_DOMAIN: errors
        // TEST_PRIORITY: important
    }

    #[test]
    fn test_error_debug_format() {
        let error = BearDogError::invalid_input("Test");
        // TEST_CATEGORY: integration
        // TEST_DOMAIN: errors
        // TEST_PRIORITY: important
        let debug = format!("{:?}", error);
        assert!(!debug.is_empty());
    }

    // TEST_CATEGORY: integration
    // TEST_DOMAIN: errors
    // TEST_PRIORITY: normal
    #[test]
    fn test_early_return_pattern() {
        fn step1() -> Result<(), BearDogError> {
            Ok(())
        }

        fn step2() -> Result<(), BearDogError> {
            Err(BearDogError::invalid_input("Step 2 failed"))
        }

        fn step3() -> Result<(), BearDogError> {
            Ok(())
        }

        fn process() -> Result<(), BearDogError> {
            step1()?;
            step2()?;
            step3()?;
            Ok(())
        }

        assert!(process().is_err());
    }
}
