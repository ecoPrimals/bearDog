// SPDX-License-Identifier: AGPL-3.0-only

//! Comprehensive tests for BearDogError construction and handling
//!
//! Tests all error constructor methods and basic error handling.

#![allow(clippy::disallowed_methods)] // unwrap/unwrap_err acceptable in test code

use crate::BearDogError;

#[test]
fn test_security_error_construction() {
    let error = BearDogError::security("Authentication failed".to_string());

    match error {
        BearDogError::Security { message, .. } => {
            assert_eq!(message, "Authentication failed");
        }
        _ => panic!("Expected Security error variant"),
    }
}

#[test]
fn test_system_error_construction() {
    let error = BearDogError::system("Database connection lost".to_string());

    match error {
        BearDogError::System { message, .. } => {
            assert_eq!(message, "Database connection lost");
        }
        _ => panic!("Expected System error variant"),
    }
}

#[test]
fn test_business_error_construction() {
    let error = BearDogError::business("Invalid email format".to_string());

    match error {
        BearDogError::Business { message, .. } => {
            assert_eq!(message, "Invalid email format");
        }
        _ => panic!("Expected Business error variant"),
    }
}

#[test]
fn test_network_error_construction() {
    let error = BearDogError::network("Connection timeout".to_string());

    // network() returns System variant
    match error {
        BearDogError::System { message, .. } => {
            assert_eq!(message, "Connection timeout");
        }
        _ => panic!("Expected System error variant from network constructor"),
    }
}

#[test]
fn test_configuration_error_construction() {
    let error = BearDogError::configuration("Invalid port number");

    // configuration() returns System variant
    match error {
        BearDogError::System { message, .. } => {
            assert_eq!(message, "Invalid port number");
        }
        _ => panic!("Expected System error variant from configuration constructor"),
    }
}

#[test]
fn test_api_error_construction() {
    let error = BearDogError::api("Rate limit exceeded".to_string());

    match error {
        BearDogError::Api { message, .. } => {
            assert_eq!(message, "Rate limit exceeded");
        }
        _ => panic!("Expected Api error variant"),
    }
}

#[test]
fn test_hsm_error_construction() {
    let error = BearDogError::hsm("HSM not available".to_string());

    // TEST_CATEGORY: integration
    // TEST_DOMAIN: errors
    // TEST_PRIORITY: important
    match error {
        BearDogError::Cryptographic { message, .. } => {
            assert_eq!(message, "HSM not available");
        }
        _ => panic!("Expected Cryptographic error variant"),
    }
}

#[test]
// TEST_CATEGORY: integration
// TEST_DOMAIN: errors
// TEST_PRIORITY: important
fn test_workflow_error_construction() {
    let error = BearDogError::workflow("Step execution failed".to_string());

    match error {
        BearDogError::Workflow { message, .. } => {
            assert_eq!(message, "Step execution failed");
        }
        _ => panic!("Expected Workflow error variant"),
    }
    // TEST_CATEGORY: integration
    // TEST_DOMAIN: errors
    // TEST_PRIORITY: important
}

#[test]
fn test_genetics_error_construction() {
    let error = BearDogError::genetics("Entropy generation failed".to_string());

    match error {
        BearDogError::Genetics { message } => {
            assert_eq!(message, "Entropy generation failed");
            // TEST_CATEGORY: integration
            // TEST_DOMAIN: errors
            // TEST_PRIORITY: important
        }
        _ => panic!("Expected Genetics error variant"),
    }
}

#[test]
fn test_initialization_error_construction() {
    let error = BearDogError::initialization("Core initialization failed".to_string());

    // initialization() returns System variant
    // TEST_CATEGORY: integration
    // TEST_DOMAIN: errors
    // TEST_PRIORITY: important
    match error {
        BearDogError::System { message, .. } => {
            assert_eq!(message, "Core initialization failed");
        }
        _ => panic!("Expected System error variant from initialization constructor"),
    }
}

#[test]
fn test_error_clone() {
    // TEST_CATEGORY: integration
    // TEST_DOMAIN: errors
    // TEST_PRIORITY: important
    let error = BearDogError::security("Test error".to_string());
    let cloned = error.clone();

    assert_eq!(error, cloned);
}

#[test]
fn test_error_debug() {
    let error = BearDogError::security("Test error".to_string());
    // TEST_CATEGORY: integration
    // TEST_DOMAIN: errors
    // TEST_PRIORITY: important
    let debug_str = format!("{error:?}");

    assert!(debug_str.contains("Security"));
    assert!(debug_str.contains("Test error"));
}

#[test]
fn test_error_display() {
    let error = BearDogError::security("Authentication failed".to_string());
    // TEST_CATEGORY: integration
    // TEST_DOMAIN: errors
    // TEST_PRIORITY: important
    let display_str = format!("{error}");

    assert!(display_str.contains("Authentication failed"));
}

#[test]
fn test_error_serialization() {
    let error = BearDogError::security("Test error".to_string());
    let json = serde_json::to_string(&error).expect("Should serialize");
    // TEST_CATEGORY: integration
    // TEST_DOMAIN: errors
    // TEST_PRIORITY: important

    assert!(!json.is_empty());

    let deserialized: BearDogError = serde_json::from_str(&json).expect("Should deserialize");

    assert_eq!(error, deserialized);
}

#[test]
// TEST_CATEGORY: integration
// TEST_DOMAIN: errors
// TEST_PRIORITY: important
fn test_result_type_ok() {
    fn returns_ok() -> Result<String, BearDogError> {
        Ok("success".to_string())
    }

    let result = returns_ok();
    assert!(result.is_ok());
    assert_eq!(result.unwrap(), "success");
}

// TEST_CATEGORY: integration
// TEST_DOMAIN: errors
// TEST_PRIORITY: important
#[test]
fn test_result_type_err() {
    fn returns_err() -> Result<String, BearDogError> {
        Err(BearDogError::security("Failed".to_string()))
    }
    // TEST_CATEGORY: integration
    // TEST_DOMAIN: errors
    // TEST_PRIORITY: important

    let result = returns_err();
    assert!(result.is_err());
}

#[test]
// TEST_CATEGORY: integration
// TEST_DOMAIN: errors
// TEST_PRIORITY: important
fn test_error_propagation() {
    fn inner_function() -> Result<i32, BearDogError> {
        Err(BearDogError::security("Inner error".to_string()))
    }

    // TEST_CATEGORY: integration
    // TEST_DOMAIN: errors
    // TEST_PRIORITY: important
    fn outer_function() -> Result<String, BearDogError> {
        let _value = inner_function()?;
        Ok("success".to_string())
    }

    let result = outer_function();
    assert!(result.is_err());
}

// TEST_CATEGORY: integration
// TEST_DOMAIN: errors
// TEST_PRIORITY: normal
#[test]
fn test_multiple_error_types() {
    let errors = [
        BearDogError::security("Security issue".to_string()),
        BearDogError::system("System issue".to_string()),
        BearDogError::network("Network issue".to_string()), // Returns System
        BearDogError::business("Business issue".to_string()),
    ];
    // TEST_CATEGORY: integration
    // TEST_DOMAIN: errors
    // TEST_PRIORITY: normal

    assert_eq!(errors.len(), 4);

    // Verify error types (note: network() returns System variant)
    for (i, error) in errors.iter().enumerate() {
        match (i, error) {
            (0, BearDogError::Security { .. }) => {}
            // TEST_CATEGORY: integration
            // TEST_DOMAIN: errors
            // TEST_PRIORITY: important
            (1, BearDogError::System { .. }) => {}
            (2, BearDogError::System { .. }) => {} // network() returns System
            (3, BearDogError::Business { .. }) => {}
            _ => panic!("Unexpected error variant at index {i}"),
        }
    }
}

#[test]
fn test_error_equality() {
    let error1 = BearDogError::security("Same error".to_string());
    let error2 = BearDogError::security("Same error".to_string());
    // TEST_CATEGORY: integration
    // TEST_DOMAIN: errors
    // TEST_PRIORITY: important
    let error3 = BearDogError::security("Different error".to_string());

    assert_eq!(error1, error2);
    assert_ne!(error1, error3);
}

#[test]
fn test_error_with_empty_message() {
    let error = BearDogError::security(String::new());

    match error {
        BearDogError::Security { message, .. } => {
            assert_eq!(message, "");
        }
        _ => panic!("Expected Security error variant"),
    }
}

#[test]
fn test_error_with_long_message() {
    // TEST_CATEGORY: integration
    // TEST_DOMAIN: errors
    // TEST_PRIORITY: important
    let long_message = "a".repeat(1000);
    let error = BearDogError::security(long_message.clone());

    match error {
        BearDogError::Security { message, .. } => {
            assert_eq!(message.len(), 1000);
            assert_eq!(message, long_message);
            // TEST_CATEGORY: integration
            // TEST_DOMAIN: errors
            // TEST_PRIORITY: important
        }
        _ => panic!("Expected Security error variant"),
    }
}

#[test]
fn test_error_with_unicode() {
    let unicode_message = "🔒 Authentication failed 认证失败 المصادقة فشلت";
    let error = BearDogError::security(unicode_message.to_string());
    // TEST_CATEGORY: integration
    // TEST_DOMAIN: errors
    // TEST_PRIORITY: important

    match error {
        BearDogError::Security { message, .. } => {
            assert_eq!(message, unicode_message);
        }
        _ => panic!("Expected Security error variant"),
    }
}

#[test]
fn test_error_in_option_chain() {
    // TEST_CATEGORY: integration
    // TEST_DOMAIN: errors
    // TEST_PRIORITY: important
    fn may_fail(should_fail: bool) -> Result<Option<String>, BearDogError> {
        if should_fail {
            return Err(BearDogError::security("Failed".to_string()));
        }
        Ok(Some("value".to_string()))
    }

    assert!(may_fail(true).is_err());
    assert!(may_fail(false).is_ok());
}
// TEST_CATEGORY: integration
// TEST_DOMAIN: errors
// TEST_PRIORITY: important

#[test]
fn test_error_categories_distinct() {
    let security = BearDogError::security("test".to_string());
    let system = BearDogError::system("test".to_string());
    let business = BearDogError::business("test".to_string());

    // Even with same message, different variants should not be equal
    assert_ne!(security, system);
    assert_ne!(system, business);
    // TEST_CATEGORY: integration
    // TEST_DOMAIN: errors
    // TEST_PRIORITY: important
    assert_ne!(security, business);
}

#[test]
fn test_all_constructor_methods_exist() {
    // Verify all main constructors can be called
    let _security = BearDogError::security("test".to_string());
    let _system = BearDogError::system("test".to_string());
    let _business = BearDogError::business("test".to_string());
    // TEST_CATEGORY: integration
    // TEST_DOMAIN: errors
    // TEST_PRIORITY: normal
    let _network = BearDogError::network("test".to_string());
    let _config = BearDogError::configuration("test");
    let _init = BearDogError::initialization("test".to_string());
    let _hsm = BearDogError::hsm("test".to_string());
    let _api = BearDogError::api("test".to_string());
    let _workflow = BearDogError::workflow("test".to_string());
    let _genetics = BearDogError::genetics("test".to_string());
}

#[test]
fn test_error_from_result_chain() {
    fn step1() -> Result<i32, BearDogError> {
        // TEST_CATEGORY: integration
        // TEST_DOMAIN: errors
        // TEST_PRIORITY: important
        Ok(42)
    }

    fn step2(value: i32) -> Result<String, BearDogError> {
        if value == 42 {
            Ok("success".to_string())
        } else {
            Err(BearDogError::business("Invalid value".to_string()))
        }
    }

    fn pipeline() -> Result<String, BearDogError> {
        let value = step1()?;
        step2(value)
    }

    let result = pipeline();
    assert!(result.is_ok());
    assert_eq!(result.unwrap(), "success");
}

// TEST_CATEGORY: integration
// TEST_DOMAIN: errors
// TEST_PRIORITY: important
#[test]
fn test_error_message_content_preservation() {
    let original_message = "Critical: Database connection lost at 127.0.0.1:5432";
    let error = BearDogError::system(original_message.to_string());

    match error {
        BearDogError::System { message, .. } => {
            assert_eq!(message, original_message);
            // Ensure exact preservation, not truncation or modification
            assert!(message.contains("127.0.0.1:5432"));
        }
        _ => panic!("Expected System error variant"),
    }
}
