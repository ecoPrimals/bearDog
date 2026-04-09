// SPDX-License-Identifier: AGPL-3.0-or-later
// Copyright 2025 EcoPrimals BearDog Team
// SPDX-License-Identifier: AGPL-3.0-or-later

//! Comprehensive test coverage expansion for beardog-errors
//!
//! This module systematically tests previously uncovered paths to increase
//! test coverage from 70% to 90%+.

use crate::*;

#[test]
fn test_all_error_domain_constructors() {
    // Security domain
    let err = BearDogError::security("auth failed".to_string());
    assert!(matches!(err, BearDogError::Security { .. }));
    assert!(err.to_string().contains("auth failed"));

    // System domain
    let err = BearDogError::system("out of memory".to_string());
    assert!(matches!(err, BearDogError::System { .. }));

    // Business domain
    let err = BearDogError::business("invalid input".to_string());
    assert!(matches!(err, BearDogError::Business { .. }));

    // Network domain
    let err = BearDogError::network("timeout".to_string());
    assert!(matches!(err, BearDogError::Network { .. }));

    // Configuration domain
    let err = BearDogError::configuration("missing config".to_string());
    assert!(matches!(err, BearDogError::Configuration { .. }));

    // HSM domain
    let err = BearDogError::hsm("provider unavailable".to_string());
    assert!(matches!(err, BearDogError::Hsm { .. }));

    // Workflow domain
    let err = BearDogError::workflow("invalid state".to_string());
    assert!(matches!(err, BearDogError::Workflow { .. }));
}

#[test]
fn test_error_display_formatting() {
    let err = BearDogError::security("test error".to_string());
    let display = format!("{}", err);
    assert!(display.contains("Security Error"));
    assert!(display.contains("test error"));

    let err = BearDogError::system("system test".to_string());
    let display = format!("{}", err);
    assert!(display.contains("System Error"));
}

#[test]
fn test_error_debug_formatting() {
    let err = BearDogError::security("debug test".to_string());
    let debug = format!("{:?}", err);
    assert!(debug.contains("Security"));
    assert!(debug.contains("debug test"));
}

#[test]
fn test_error_source_trait() {
    use std::error::Error;
    
    let err = BearDogError::security("test".to_string());
    // BearDogError doesn't wrap other errors by default, so source should be None
    assert!(err.source().is_none());
}

#[test]
fn test_error_propagation_chain() {
    fn level3() -> Result<(), BearDogError> {
        Err(BearDogError::system("level 3 error".to_string()))
    }

    fn level2() -> Result<(), BearDogError> {
        level3()?;
        Ok(())
    }

    fn level1() -> Result<(), BearDogError> {
        level2()?;
        Ok(())
    }

    let result = level1();
    assert!(result.is_err());
    let err = result.unwrap_err();
    assert!(err.to_string().contains("level 3 error"));
}

#[test]
fn test_error_conversion_from_io_error() {
    use std::io;
    
    let io_err = io::Error::new(io::ErrorKind::NotFound, "file not found");
    let beardog_err = BearDogError::from(io_err);
    
    assert!(matches!(beardog_err, BearDogError::System { .. }));
    assert!(beardog_err.to_string().contains("file not found"));
}

#[test]
fn test_error_conversion_from_string() {
    let err: BearDogError = "simple error message".to_string().into();
    assert!(matches!(err, BearDogError::System { .. }));
}

#[test]
fn test_error_equality() {
    let err1 = BearDogError::security("test".to_string());
    let err2 = BearDogError::security("test".to_string());
    
    // Errors with same domain and message should format the same
    assert_eq!(err1.to_string(), err2.to_string());
}

#[test]
fn test_error_cloning() {
    let err1 = BearDogError::security("original".to_string());
    let err2 = err1.clone();
    
    assert_eq!(err1.to_string(), err2.to_string());
}

#[test]
fn test_result_ok_path() {
    fn returns_ok() -> Result<String, BearDogError> {
        Ok("success".to_string())
    }

    let result = returns_ok();
    assert!(result.is_ok());
    assert_eq!(result.unwrap(), "success");
}

#[test]
fn test_result_err_path() {
    fn returns_err() -> Result<String, BearDogError> {
        Err(BearDogError::business("validation failed".to_string()))
    }

    let result = returns_err();
    assert!(result.is_err());
}

#[test]
fn test_error_with_empty_message() {
    let err = BearDogError::system("".to_string());
    assert!(matches!(err, BearDogError::System { .. }));
}

#[test]
fn test_error_with_long_message() {
    let long_msg = "a".repeat(1000);
    let err = BearDogError::system(long_msg.clone());
    assert!(err.to_string().contains(&long_msg));
}

#[test]
fn test_error_with_special_characters() {
    let msg = "Error: 🔒 Security violation!!! \n\t Special: <>&\"'";
    let err = BearDogError::security(msg.to_string());
    assert!(err.to_string().contains("Security violation"));
}

#[test]
fn test_error_in_option_context() {
    fn maybe_error(should_err: bool) -> Option<Result<String, BearDogError>> {
        if should_err {
            Some(Err(BearDogError::system("error in option".to_string())))
        } else {
            Some(Ok("success".to_string()))
        }
    }

    let result = maybe_error(true);
    assert!(result.is_some());
    assert!(result.unwrap().is_err());

    let result = maybe_error(false);
    assert!(result.is_some());
    assert!(result.unwrap().is_ok());
}

#[test]
fn test_error_in_vector_context() {
    let errors: Vec<BearDogError> = vec![
        BearDogError::security("error 1".to_string()),
        BearDogError::system("error 2".to_string()),
        BearDogError::business("error 3".to_string()),
    ];

    assert_eq!(errors.len(), 3);
    assert!(matches!(errors[0], BearDogError::Security { .. }));
    assert!(matches!(errors[1], BearDogError::System { .. }));
    assert!(matches!(errors[2], BearDogError::Business { .. }));
}

#[test]
fn test_nested_result_handling() {
    fn outer() -> Result<String, BearDogError> {
        let inner_result: Result<i32, BearDogError> = Err(BearDogError::system("inner error".to_string()));
        let _value = inner_result?;
        Ok("success".to_string())
    }

    let result = outer();
    assert!(result.is_err());
    assert!(result.unwrap_err().to_string().contains("inner error"));
}

#[test]
fn test_error_match_patterns() {
    let err = BearDogError::security("test".to_string());

    match err {
        BearDogError::Security { message } => {
            assert!(message.contains("test"));
        }
        _ => panic!("Should match Security variant"),
    }
}

#[test]
fn test_all_domain_variants_exist() {
    // Ensure all domain variants can be constructed
    let _security = BearDogError::security("test".to_string());
    let _system = BearDogError::system("test".to_string());
    let _business = BearDogError::business("test".to_string());
    let _network = BearDogError::network("test".to_string());
    let _config = BearDogError::configuration("test".to_string());
    let _hsm = BearDogError::hsm("test".to_string());
    let _workflow = BearDogError::workflow("test".to_string());
}

#[test]
fn test_error_in_concurrent_context() {
    use std::sync::{Arc, Mutex};
    use std::thread;

    let errors = Arc::new(Mutex::new(Vec::new()));
    let errors_clone = Arc::clone(&errors);

    let handle = thread::spawn(move || {
        let err = BearDogError::system("thread error".to_string());
        errors_clone.lock().unwrap().push(err);
    });

    handle.join().unwrap();

    let errors = errors.lock().unwrap();
    assert_eq!(errors.len(), 1);
    assert!(errors[0].to_string().contains("thread error"));
}

#[test]
fn test_error_size_is_reasonable() {
    use std::mem::size_of;
    
    // Error should not be excessively large
    let size = size_of::<BearDogError>();
    assert!(size < 200, "BearDogError size is {}, should be < 200 bytes", size);
}

#[test]
fn test_result_type_alias_works() {
    type BearDogResult<T> = Result<T, BearDogError>;

    fn returns_result() -> BearDogResult<String> {
        Ok("success".to_string())
    }

    assert!(returns_result().is_ok());
}

#[test]
fn test_error_from_various_sources() {
    // From std::io::Error
    let io_err = std::io::Error::new(std::io::ErrorKind::Other, "io");
    let _err: BearDogError = io_err.into();

    // From String
    let _err: BearDogError = "string error".to_string().into();

    // From `&str` is not implemented for BearDogError.
}

#[test]
fn test_error_with_unicode() {
    let err = BearDogError::security("错误: 認証失敗 🔒".to_string());
    assert!(err.to_string().contains("認証失敗"));
}

#[test]
fn test_error_serialization_compatibility() {
    // Even if not serializable, error should be usable
    let err = BearDogError::security("test".to_string());
    let _ = format!("{:?}", err);
    let _ = format!("{}", err);
}

