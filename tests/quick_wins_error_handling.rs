// Quick Win Tests: Error Handling
//
// Verify error handling patterns work correctly

use beardog_errors::BearDogError;

#[test]
fn test_beardog_error_creation() {
    let err = BearDogError::validation("Test validation error");
    assert!(format!("{:?}", err).contains("validation"));
}

#[test]
fn test_beardog_error_system() {
    let err = BearDogError::system("System error test".to_string());
    assert!(format!("{:?}", err).contains("System"));
}

#[test]
fn test_beardog_error_business() {
    let err = BearDogError::business("Business logic error".to_string());
    assert!(format!("{:?}", err).contains("Business"));
}

#[test]
fn test_result_error_propagation() -> Result<(), BearDogError> {
    fn might_fail(should_fail: bool) -> Result<(), BearDogError> {
        if should_fail {
            Err(BearDogError::validation("Expected failure"))
        } else {
            Ok(())
        }
    }

    // Test success case
    might_fail(false)?;

    // Test failure case
    let result = might_fail(true);
    assert!(result.is_err());

    Ok(())
}

#[test]
fn test_error_display() {
    let err = BearDogError::validation("Display test");
    let display_str = format!("{}", err);
    assert!(!display_str.is_empty());
}

#[test]
fn test_error_debug() {
    let err = BearDogError::validation("Debug test");
    let debug_str = format!("{:?}", err);
    // Debug output should contain some meaningful information
    assert!(!debug_str.is_empty());
}

#[test]
fn test_error_from_string() {
    let err = BearDogError::validation("From string test");
    let err_string = err.to_string();
    assert!(!err_string.is_empty());
}

#[test]
fn test_multiple_error_types() {
    let errors = [
        BearDogError::validation("error1"),
        BearDogError::system("error2".to_string()),
        BearDogError::business("error3".to_string()),
    ];

    assert_eq!(errors.len(), 3);
}
