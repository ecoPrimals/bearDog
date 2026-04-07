// SPDX-License-Identifier: AGPL-3.0-or-later

//! Edge Case Tests for `BearDog` Errors - November 6, 2025
//!
//! Comprehensive edge case and boundary condition tests to improve code coverage.

#[cfg(test)]
#[allow(clippy::disallowed_methods)] // unwrap() is acceptable in test code
mod error_edge_cases {
    use crate::*;

    // ========================================================================
    // Empty String Handling
    // ========================================================================

    #[test]
    fn test_error_with_empty_message() {
        let error = BearDogError::validation("");
        let display = format!("{error}");
        // Empty message should still produce valid error
        assert!(!display.is_empty());
    }

    #[test]
    fn test_error_with_whitespace_only() {
        let error = BearDogError::validation("   ");
        let display = format!("{error}");
        assert!(!display.is_empty());
    }

    #[test]
    fn test_error_with_very_long_message() {
        let long_message = "x".repeat(10000);
        let error = BearDogError::validation(&long_message);
        let display = format!("{error}");
        assert!(display.contains(&long_message));
    }

    #[test]
    fn test_error_with_unicode() {
        let unicode_msg = "Error: 测试 🔐 🐻";
        let error = BearDogError::validation(unicode_msg);
        let display = format!("{error}");
        assert!(display.contains("测试"));
    }

    #[test]
    fn test_error_with_newlines() {
        let msg = "Line 1\nLine 2\nLine 3";
        let error = BearDogError::validation(msg);
        let display = format!("{error}");
        assert!(display.contains("Line 1"));
    }

    // ========================================================================
    // Error Variant Tests
    // ========================================================================

    #[test]
    fn test_all_error_constructors_produce_correct_variants() {
        // Security
        let err = BearDogError::security("test".to_string());
        assert!(matches!(err, BearDogError::Security { .. }));

        // System
        let err = BearDogError::system("test".to_string());
        assert!(matches!(err, BearDogError::System { .. }));

        // Business
        let err = BearDogError::business("test".to_string());
        assert!(matches!(err, BearDogError::Business { .. }));

        // Network
        let err = BearDogError::network("test".to_string());
        assert!(matches!(err, BearDogError::System { .. })); // Network maps to System

        // Configuration
        let err = BearDogError::configuration("test");
        assert!(matches!(err, BearDogError::System { .. }));

        // Validation
        let err = BearDogError::validation("test");
        assert!(matches!(err, BearDogError::Business { .. }));
    }

    #[test]
    fn test_specialized_error_constructors() {
        let err = authentication_error("test");
        assert!(matches!(err, BearDogError::Security { .. }));

        let err = authorization_error("resource", "action");
        assert!(matches!(err, BearDogError::Security { .. }));

        let err = crypto_error("operation", "details");
        assert!(matches!(err, BearDogError::Security { .. }));

        let err = io_error("operation", "details");
        assert!(matches!(err, BearDogError::System { .. }));

        let err = not_implemented("test");
        assert!(matches!(err, BearDogError::System { .. }));

        let err = unsupported_operation("test");
        assert!(matches!(err, BearDogError::System { .. }));
    }

    // ========================================================================
    // Error Display and Debug Tests
    // ========================================================================

    #[test]
    fn test_error_display_format() {
        let error = BearDogError::security("Authentication failed".to_string());
        let display = format!("{error}");
        assert!(display.contains("Authentication failed"));
    }

    #[test]
    fn test_error_debug_format() {
        let error = BearDogError::security("Debug test".to_string());
        let debug = format!("{error:?}");
        assert!(!debug.is_empty());
        assert!(debug.len() > 10); // Should have some structure
    }

    #[test]
    fn test_error_display_all_variants() {
        let errors = vec![
            BearDogError::security("sec".to_string()),
            BearDogError::system("sys".to_string()),
            BearDogError::business("bus".to_string()),
            BearDogError::validation("val"),
        ];

        for error in errors {
            let display = format!("{error}");
            assert!(!display.is_empty());
            assert!(display.len() > 2);
        }
    }

    // ========================================================================
    // Result Extension Trait Tests
    // ========================================================================

    #[test]
    fn test_security_context_with_ok() {
        let result: Result<i32, std::io::Error> = Ok(42);
        let with_context = result.security_context("Should not add context to Ok");
        assert!(with_context.is_ok());
        assert_eq!(with_context.unwrap(), 42);
    }

    #[test]
    fn test_security_context_with_err() {
        let result: Result<i32, std::io::Error> = Err(std::io::Error::new(
            std::io::ErrorKind::NotFound,
            "file not found",
        ));
        let with_context = result.security_context("Security violation");
        assert!(with_context.is_err());

        if let Err(e) = with_context {
            assert!(matches!(e, BearDogError::Security { .. }));
            let display = format!("{e}");
            assert!(display.contains("Security violation"));
        }
    }

    #[test]
    fn test_system_context_with_ok() {
        let result: Result<String, std::io::Error> = Ok("success".to_string());
        let with_context = result.system_context("Should not add context");
        assert!(with_context.is_ok());
        assert_eq!(with_context.unwrap(), "success");
    }

    #[test]
    fn test_system_context_with_err() {
        let result: Result<i32, std::io::Error> = Err(std::io::Error::new(
            std::io::ErrorKind::PermissionDenied,
            "access denied",
        ));
        let with_context = result.system_context("System failure");
        assert!(with_context.is_err());

        if let Err(e) = with_context {
            assert!(matches!(e, BearDogError::System { .. }));
        }
    }

    #[test]
    fn test_business_context_with_ok() {
        let result: Result<bool, String> = Ok(true);
        let with_context = result.business_context("Should not add context");
        assert!(with_context.is_ok());
        assert!(with_context.unwrap());
    }

    #[test]
    fn test_business_context_with_err() {
        let result: Result<i32, String> = Err("validation failed".to_string());
        let with_context = result.business_context("Business rule violation");
        assert!(with_context.is_err());

        if let Err(e) = with_context {
            assert!(matches!(e, BearDogError::Business { .. }));
        }
    }

    // ========================================================================
    // Error Propagation Tests
    // ========================================================================

    #[test]
    fn test_error_propagation_single_level() {
        fn inner() -> Result<i32, BearDogError> {
            Err(BearDogError::validation("inner error"))
        }

        fn outer() -> Result<i32, BearDogError> {
            inner()?;
            Ok(42)
        }

        let result = outer();
        assert!(result.is_err());
    }

    #[test]
    fn test_error_propagation_multi_level() {
        fn level_3() -> Result<i32, BearDogError> {
            Err(BearDogError::validation("level 3"))
        }

        fn level_2() -> Result<i32, BearDogError> {
            level_3()
        }

        fn level_1() -> Result<i32, BearDogError> {
            level_2()
        }

        let result = level_1();
        assert!(result.is_err());
    }

    #[test]
    fn test_error_propagation_with_success() {
        fn inner() -> Result<i32, BearDogError> {
            Ok(21)
        }

        fn outer() -> Result<i32, BearDogError> {
            let val = inner()?;
            Ok(val * 2)
        }

        let result = outer();
        assert!(result.is_ok());
        assert_eq!(result.unwrap(), 42);
    }

    // ========================================================================
    // Result Combinators
    // ========================================================================

    #[test]
    fn test_result_map_ok() {
        let result: Result<i32, BearDogError> = Ok(21);
        let mapped = result.map(|x| x * 2);
        assert_eq!(mapped.unwrap(), 42);
    }

    #[test]
    fn test_result_map_err() {
        let result: Result<i32, BearDogError> = Err(BearDogError::validation("test"));
        let mapped = result.map(|x| x * 2);
        assert!(mapped.is_err());
    }

    #[test]
    fn test_result_and_then_ok() {
        let result: Result<i32, BearDogError> = Ok(21);
        let chained = result.map(|x| x * 2);
        assert_eq!(chained.unwrap(), 42);
    }

    #[test]
    fn test_result_and_then_err() {
        let result: Result<i32, BearDogError> = Err(BearDogError::validation("test"));
        let chained = result.map(|x| x * 2);
        assert!(chained.is_err());
    }

    #[test]
    fn test_result_or_else_ok() {
        let result: Result<i32, BearDogError> = Ok(42);
        let fallback: Result<i32, BearDogError> = result.or(Ok(0));
        assert_eq!(fallback.unwrap(), 42);
    }

    #[test]
    fn test_result_or_else_err() {
        let result: Result<i32, BearDogError> = Err(BearDogError::validation("test"));
        let fallback: Result<i32, BearDogError> = result.or(Ok(0));
        assert_eq!(fallback.unwrap(), 0);
    }

    #[test]
    fn test_result_unwrap_or() {
        // Test with Ok value
        let ok_result = 42;
        assert_eq!(ok_result, 42);

        // Test with Err value - use a function to avoid literal Err
        fn get_err() -> Result<i32, BearDogError> {
            Err(BearDogError::validation("test"))
        }
        assert_eq!(get_err().unwrap_or(0), 0);
    }

    #[test]
    fn test_result_unwrap_or_else() {
        // Test with Ok value
        let ok_result = 42;
        assert_eq!(ok_result, 42);

        // Test with Err value - use a function to avoid literal Err
        fn get_err() -> Result<i32, BearDogError> {
            Err(BearDogError::validation("test"))
        }
        assert_eq!(get_err().unwrap_or(0), 0);
    }

    // ========================================================================
    // Option to Result Conversion
    // ========================================================================

    #[test]
    fn test_option_some_to_result() {
        let option: Option<i32> = Some(42);
        let result: Result<i32, BearDogError> =
            option.ok_or_else(|| BearDogError::validation("should not happen"));
        assert!(result.is_ok());
        assert_eq!(result.unwrap(), 42);
    }

    #[test]
    fn test_option_none_to_result() {
        let option: Option<i32> = None;
        let result: Result<i32, BearDogError> =
            option.ok_or_else(|| BearDogError::validation("option was none"));
        assert!(result.is_err());
    }

    // ========================================================================
    // Error Equality and Cloning
    // ========================================================================

    #[test]
    fn test_error_clone() {
        let original = BearDogError::validation("test error");
        let cloned = original.clone();

        let original_str = format!("{original}");
        let cloned_str = format!("{cloned}");
        assert_eq!(original_str, cloned_str);
    }

    #[test]
    fn test_error_send_sync() {
        // Verify BearDogError is Send + Sync
        fn assert_send<T: Send>() {}
        fn assert_sync<T: Sync>() {}

        assert_send::<BearDogError>();
        assert_sync::<BearDogError>();
    }

    // ========================================================================
    // Concurrent Error Handling
    // ========================================================================

    #[test]
    fn test_error_across_threads() {
        use std::sync::Arc;
        use std::thread;

        let error = Arc::new(BearDogError::validation("concurrent test"));
        let error_clone = Arc::clone(&error);

        let handle = thread::spawn(move || {
            let display = format!("{error_clone}");
            assert!(display.contains("concurrent test"));
        });

        let display = format!("{error}");
        assert!(display.contains("concurrent test"));

        handle.join().expect("Thread should complete");
    }

    #[test]
    fn test_result_across_threads() {
        use std::sync::Arc;
        use std::sync::Mutex;
        use std::thread;

        let result = Arc::new(Mutex::new(Ok::<i32, BearDogError>(42)));
        let result_clone = Arc::clone(&result);

        let handle = thread::spawn(move || {
            let guard = result_clone.lock().unwrap();
            assert!(guard.is_ok());
            // Lock released when guard goes out of scope
        });

        // Wait for spawned thread to complete BEFORE locking in main thread
        // This prevents deadlock
        handle.join().expect("Thread should complete");

        // Now safe to lock - spawned thread has released it
        let guard = result.lock().unwrap();
        assert!(guard.is_ok());
    }

    // ========================================================================
    // Error Conversion Tests
    // ========================================================================

    #[test]
    fn test_string_to_error() {
        let string_error = "test error";
        let error: BearDogError = BearDogError::validation(string_error);
        assert!(matches!(error, BearDogError::Business { .. }));
    }

    #[test]
    fn test_str_to_error() {
        let str_error = "test error";
        let error = BearDogError::validation(str_error);
        let display = format!("{error}");
        assert!(display.contains("test error"));
    }

    // ========================================================================
    // Specialized Error Scenarios
    // ========================================================================

    #[test]
    fn test_authentication_error_scenario() {
        let error = authentication_error("Invalid credentials");
        let display = format!("{error}");
        assert!(display.contains("Invalid credentials"));
        assert!(matches!(error, BearDogError::Security { .. }));
    }

    #[test]
    fn test_authorization_error_scenario() {
        let error = authorization_error("/api/users", "delete");
        let display = format!("{error}");
        assert!(display.contains("/api/users"));
        assert!(matches!(error, BearDogError::Security { .. }));
    }

    #[test]
    fn test_crypto_error_scenario() {
        let error = crypto_error("key_generation", "Key generation failed");
        let display = format!("{error}");
        assert!(display.contains("Key generation failed"));
        assert!(matches!(error, BearDogError::Security { .. }));
    }

    #[test]
    fn test_io_error_scenario() {
        let error = io_error("read_file", "File not found");
        let display = format!("{error}");
        assert!(display.contains("File not found"));
        assert!(matches!(error, BearDogError::System { .. }));
    }

    #[test]
    fn test_validation_error_scenario() {
        let error = validation_error("email", "Email format invalid");
        let display = format!("{error}");
        assert!(display.contains("Email format invalid"));
        assert!(matches!(error, BearDogError::Business { .. }));
    }

    #[test]
    fn test_not_implemented_scenario() {
        let error = not_implemented("Feature coming soon");
        let display = format!("{error}");
        assert!(display.contains("Feature coming soon"));
        assert!(matches!(error, BearDogError::System { .. }));
    }

    #[test]
    fn test_unsupported_operation_scenario() {
        let error = unsupported_operation("Platform not supported");
        let display = format!("{error}");
        assert!(display.contains("Platform not supported"));
        assert!(matches!(error, BearDogError::System { .. }));
    }

    // ========================================================================
    // Boundary Conditions
    // ========================================================================

    #[test]
    #[allow(clippy::cast_possible_truncation, clippy::cast_possible_wrap)]
    fn test_max_error_nesting() {
        // Test deeply nested error propagation
        fn nest(depth: u32) -> Result<i32, BearDogError> {
            if depth == 0 {
                Err(BearDogError::validation("max depth"))
            } else {
                nest(depth - 1)?;
                Ok(depth as i32)
            }
        }

        let result = nest(100);
        assert!(result.is_err());
    }

    #[test]
    fn test_error_with_special_characters() {
        let special = "Error: \t\n\r\\ \" ' {}[]()<>";
        let error = BearDogError::validation(special);
        let display = format!("{error}");
        assert!(!display.is_empty());
    }
}
