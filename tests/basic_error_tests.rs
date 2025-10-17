// Basic error creation and handling tests

use beardog_errors::BearDogError;

#[test]
fn test_system_error_creation() {
    let error = BearDogError::system("system error".to_string());
    assert!(format!("{:?}", error).contains("system error"));
}

#[test]
fn test_security_error_creation() {
    let error = BearDogError::security("security issue".to_string());
    assert!(format!("{:?}", error).contains("security issue"));
}

#[test]
fn test_validation_error_creation() {
    let error = BearDogError::validation("invalid input");
    assert!(format!("{:?}", error).contains("invalid input"));
}

#[test]
fn test_network_error_creation() {
    let error = BearDogError::network("connection failed".to_string());
    assert!(format!("{:?}", error).contains("connection failed"));
}

#[test]
fn test_configuration_error_creation() {
    let error = BearDogError::configuration("bad config");
    assert!(format!("{:?}", error).contains("bad config"));
}

#[test]
fn test_not_found_error_creation() {
    let error = BearDogError::not_found("resource not found".to_string());
    assert!(format!("{:?}", error).contains("resource not found"));
}

#[test]
fn test_unauthorized_error_creation() {
    let error = BearDogError::unauthorized("access denied".to_string());
    assert!(format!("{:?}", error).contains("access denied"));
}

#[test]
fn test_invalid_input_error_creation() {
    let error = BearDogError::invalid_input("bad input");
    assert!(format!("{:?}", error).contains("bad input"));
}

#[test]
fn test_unavailable_error_creation() {
    let error = BearDogError::unavailable("service unavailable".to_string());
    assert!(format!("{:?}", error).contains("service unavailable"));
}

#[test]
fn test_internal_error_creation() {
    let error = BearDogError::internal("internal error".to_string());
    assert!(format!("{:?}", error).contains("internal error"));
}

#[test]
fn test_business_error_creation() {
    let error = BearDogError::business("business rule violated".to_string());
    assert!(format!("{:?}", error).contains("business rule"));
}

#[test]
fn test_api_error_creation() {
    let error = BearDogError::api("API error".to_string());
    assert!(format!("{:?}", error).contains("API error"));
}

#[test]
fn test_workflow_error_creation() {
    let error = BearDogError::workflow("workflow failed".to_string());
    assert!(format!("{:?}", error).contains("workflow failed"));
}

#[test]
fn test_genetics_error_creation() {
    let error = BearDogError::genetics("genetic error".to_string());
    assert!(format!("{:?}", error).contains("genetic error"));
}

#[test]
fn test_initialization_error_creation() {
    let error = BearDogError::initialization("init failed".to_string());
    assert!(format!("{:?}", error).contains("init failed"));
}

#[test]
fn test_hsm_error_creation() {
    let error = BearDogError::hsm("HSM error".to_string());
    assert!(format!("{:?}", error).contains("HSM error"));
}

#[test]
fn test_testing_error_creation() {
    let error = BearDogError::testing("test error");
    assert!(format!("{:?}", error).contains("test error"));
}

#[test]
fn test_error_display() {
    let error = BearDogError::validation("test");
    let display_str = format!("{}", error);
    assert!(!display_str.is_empty());
}

#[test]
fn test_error_debug() {
    let error = BearDogError::system("test".to_string());
    let debug_str = format!("{:?}", error);
    assert!(!debug_str.is_empty());
}

#[test]
fn test_result_ok() -> Result<(), BearDogError> {
    Ok(())
}

#[test]
fn test_result_with_value() -> Result<(), BearDogError> {
    let _value: i32 = 42;
    Ok(())
}

#[test]
fn test_error_in_result() {
    let result: Result<(), BearDogError> = Err(BearDogError::validation("error"));
    assert!(result.is_err());
}

#[test]
fn test_multiple_errors() {
    let errors = [
        BearDogError::validation("error 1"),
        BearDogError::system("error 2".to_string()),
        BearDogError::network("error 3".to_string()),
    ];
    assert_eq!(errors.len(), 3);
}
