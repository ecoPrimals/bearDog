// SPDX-License-Identifier: AGPL-3.0-or-later
#![allow(clippy::expect_used, clippy::unwrap_used)]

//! Comprehensive Tests for `BearDog` Error Types
//!
//! Coverage expansion for beardog-errors crate

use beardog_errors::{
    BearDogError, BusinessErrorCategory, SecurityErrorCategory, SystemErrorCategory,
};

type Result<T> = std::result::Result<T, BearDogError>;

#[test]
fn test_error_not_found() {
    let err = BearDogError::not_found("Resource not found".to_string());

    match err {
        BearDogError::Business { message, category } => {
            assert!(message.contains("Resource not found"));
            assert_eq!(category, BusinessErrorCategory::General);
        }
        _ => panic!("Expected Business error"),
    }
}

#[test]
fn test_error_invalid_input() {
    let err = BearDogError::invalid_input("Bad input");

    match err {
        BearDogError::Business { message, category } => {
            assert!(message.contains("Bad input"));
            assert_eq!(category, BusinessErrorCategory::Validation);
        }
        _ => panic!("Expected Business error"),
    }
}

#[test]
fn test_error_unauthorized() {
    let err = BearDogError::unauthorized("Access denied".to_string());

    match err {
        BearDogError::Security { message, category } => {
            assert!(message.contains("Access denied"));
            assert_eq!(category, SecurityErrorCategory::Authorization);
        }
        _ => panic!("Expected Security error"),
    }
}

#[test]
fn test_error_configuration() {
    let err = BearDogError::configuration("Config missing");

    match err {
        BearDogError::System { message, category } => {
            assert!(message.contains("Config missing"));
            assert_eq!(category, SystemErrorCategory::General);
        }
        _ => panic!("Expected System error"),
    }
}

#[test]
fn test_error_serialization() {
    let err = BearDogError::serialization("JSON error");

    match err {
        BearDogError::System { message, category } => {
            assert!(message.contains("JSON error"));
            assert_eq!(category, SystemErrorCategory::General);
        }
        _ => panic!("Expected System error"),
    }
}

#[test]
fn test_error_io_error() {
    let err = BearDogError::io_error("File not accessible");

    match err {
        BearDogError::System { message, category } => {
            assert!(message.contains("File not accessible"));
            assert_eq!(category, SystemErrorCategory::FileSystem);
        }
        _ => panic!("Expected System error"),
    }
}

#[test]
fn test_error_network() {
    let err = BearDogError::network("Connection failed".to_string());

    match err {
        BearDogError::System { message, category } => {
            assert!(message.contains("Connection failed"));
            assert_eq!(category, SystemErrorCategory::General);
        }
        _ => panic!("Expected System error"),
    }
}

#[test]
fn test_error_system() {
    let err = BearDogError::system("System failure".to_string());

    match err {
        BearDogError::System { message, category } => {
            assert!(message.contains("System failure"));
            assert_eq!(category, SystemErrorCategory::General);
        }
        _ => panic!("Expected System error"),
    }
}

#[test]
fn test_error_display() {
    let err = BearDogError::not_found("test".to_string());
    let display = format!("{err}");

    assert!(display.contains("test"));
}

#[test]
fn test_error_debug() {
    let err = BearDogError::invalid_input("debug test");
    let debug = format!("{err:?}");

    assert!(debug.contains("debug test"));
}

#[test]
fn test_error_clone() {
    let err = BearDogError::unauthorized("clone test".to_string());
    let cloned = err.clone();

    assert_eq!(format!("{err}"), format!("{cloned}"));
}

#[test]
fn test_error_category_equality() {
    assert_eq!(
        BusinessErrorCategory::General,
        BusinessErrorCategory::General
    );
    assert_ne!(
        BusinessErrorCategory::General,
        BusinessErrorCategory::Validation
    );
}

#[test]
fn test_error_category_debug() {
    let category = SecurityErrorCategory::Authorization;
    let debug = format!("{category:?}");

    assert!(debug.contains("Authorization"));
}

#[test]
fn test_error_category_clone() {
    let category = SystemErrorCategory::General;
    let cloned = category.clone();

    assert_eq!(category, cloned);
}

#[test]
fn test_result_type_ok() {
    let result: Result<i32> = Ok(42);
    assert!(result.is_ok());
    if let Ok(value) = result {
        assert_eq!(value, 42);
    }
}

#[test]
fn test_result_type_err() {
    let result: Result<i32> = Err(BearDogError::not_found("test".to_string()));
    assert!(result.is_err());
}

#[test]
fn test_error_propagation() {
    fn inner() -> Result<i32> {
        Err(BearDogError::invalid_input("inner error"))
    }

    fn outer() -> Result<i32> {
        inner()?;
        Ok(42)
    }

    let result = outer();
    assert!(result.is_err());
}

#[test]
fn test_error_with_context() {
    let err = BearDogError::not_found("User not found: user_id=123".to_string());

    match err {
        BearDogError::Business { message, .. } => {
            assert!(message.contains("user_id=123"));
        }
        _ => panic!("Expected Business error"),
    }
}

#[test]
fn test_error_chaining() {
    fn operation1() -> Result<()> {
        Err(BearDogError::invalid_input("op1 failed"))
    }

    fn operation2() -> Result<()> {
        operation1()?;
        Ok(())
    }

    fn operation3() -> Result<()> {
        operation2()?;
        Ok(())
    }

    assert!(operation3().is_err());
}

#[test]
fn test_error_message_formatting() {
    let err = BearDogError::configuration(&format!("Port {} is invalid", 65536));

    match err {
        BearDogError::System { message, .. } => {
            assert!(message.contains("65536"));
        }
        _ => panic!("Expected System error"),
    }
}

#[test]
fn test_multiple_error_types() {
    let errors = [
        BearDogError::not_found("resource1".to_string()),
        BearDogError::unauthorized("resource2".to_string()),
        BearDogError::configuration("resource3"),
    ];

    assert_eq!(errors.len(), 3);
}

#[test]
fn test_error_in_collection() {
    let results: Vec<Result<i32>> = vec![Ok(1), Err(BearDogError::invalid_input("bad")), Ok(3)];

    assert_eq!(results.len(), 3);
    assert!(results[0].is_ok());
    assert!(results[1].is_err());
    assert!(results[2].is_ok());
}

#[test]
fn test_error_category_serialization() {
    let category = SystemErrorCategory::Internal;
    let serialized = format!("{category:?}");
    assert!(serialized.contains("Internal"));
}

#[test]
fn test_error_with_empty_message() {
    let err = BearDogError::not_found(String::new());

    match err {
        BearDogError::Business { message, .. } => {
            assert_eq!(message, "");
        }
        _ => panic!("Expected Business error"),
    }
}

#[test]
fn test_error_with_special_characters() {
    let err = BearDogError::invalid_input("Error: <>&\"'");

    match err {
        BearDogError::Business { message, .. } => {
            assert!(message.contains("<>&\"'"));
        }
        _ => panic!("Expected Business error"),
    }
}

#[test]
fn test_error_with_unicode() {
    let err = BearDogError::not_found("用户未找到".to_string());

    match err {
        BearDogError::Business { message, .. } => {
            assert!(message.contains("用户未找到"));
        }
        _ => panic!("Expected Business error"),
    }
}

#[test]
fn test_error_long_message() {
    let long_msg = "Error: ".to_string() + &"x".repeat(1000);
    let err = BearDogError::system(long_msg.clone());

    match err {
        BearDogError::System { message, .. } => {
            assert_eq!(message.len(), long_msg.len());
        }
        _ => panic!("Expected System error"),
    }
}
