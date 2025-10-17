//! Comprehensive Error Tests
//!
//! Tests for BearDogError construction, conversion, and handling

use crate::*;

#[cfg(test)]
mod error_construction_tests {
    use super::*;

    #[test]
    fn test_system_error_creation() {
        let error = BearDogError::system("System error".to_string());
        assert!(matches!(error, BearDogError::System { .. }));
    }

    #[test]
    fn test_security_error_creation() {
        let error = BearDogError::security("Security error".to_string());
        assert!(matches!(error, BearDogError::Security { .. }));
    }

    #[test]
    fn test_configuration_error_creation() {
        let error = BearDogError::configuration("Config error");
        assert!(matches!(error, BearDogError::System { .. }));
    }

    #[test]
    fn test_network_error_creation() {
        let error = BearDogError::network("Network error".to_string());
        assert!(matches!(error, BearDogError::System { .. }));
    }

    #[test]
    fn test_internal_error_creation() {
        let error = BearDogError::internal("Internal error".to_string());
        assert!(matches!(error, BearDogError::System { .. }));
    }

    #[test]
    fn test_not_found_error_creation() {
        let error = BearDogError::not_found("Resource not found".to_string());
        assert!(matches!(error, BearDogError::Business { .. }));
    }

    #[test]
    fn test_unauthorized_error_creation() {
        let error = BearDogError::unauthorized("Unauthorized access".to_string());
        assert!(matches!(error, BearDogError::Security { .. }));
    }

    #[test]
    fn test_unavailable_error_creation() {
        let error = BearDogError::unavailable("Service unavailable".to_string());
        assert!(matches!(error, BearDogError::System { .. }));
    }

    #[test]
    fn test_business_error_creation() {
        let error = BearDogError::business("Business error".to_string());
        assert!(matches!(error, BearDogError::Business { .. }));
    }

    #[test]
    fn test_validation_error_creation() {
        let error = BearDogError::validation("Validation failed");
        assert!(matches!(error, BearDogError::Business { .. }));
    }

    #[test]
    fn test_invalid_input_error_creation() {
        let error = BearDogError::invalid_input("Invalid input");
        assert!(matches!(error, BearDogError::Business { .. }));
    }
}

#[cfg(test)]
mod error_display_tests {
    use super::*;

    #[test]
    fn test_error_display() {
        let error = BearDogError::internal("Test error".to_string());
        let display = format!("{}", error);
        assert!(display.contains("Test error"));
    }

    #[test]
    fn test_error_debug() {
        let error = BearDogError::internal("Debug test".to_string());
        let debug = format!("{:?}", error);
        assert!(debug.contains("System"));
    }

    #[test]
    fn test_system_error_display() {
        let error = BearDogError::system("System failure".to_string());
        let display = format!("{}", error);
        assert!(display.contains("System"));
    }

    #[test]
    fn test_security_error_display() {
        let error = BearDogError::security("Security violation".to_string());
        let display = format!("{}", error);
        assert!(display.contains("Security"));
    }
}

#[cfg(test)]
mod error_conversion_tests {
    use super::*;

    #[test]
    fn test_result_type() {
        let result: BearDogResult<i32> = Ok(42);
        assert!(result.is_ok());
        assert_eq!(result.ok(), Some(42));
    }

    #[test]
    fn test_result_error() {
        let result: BearDogResult<i32> = Err(BearDogError::internal("Error".to_string()));
        assert!(result.is_err());
    }

    #[test]
    fn test_result_propagation() -> BearDogResult<()> {
        fn inner() -> BearDogResult<i32> {
            Ok(42)
        }

        let value = inner()?;
        assert_eq!(value, 42);
        Ok(())
    }

    #[test]
    fn test_error_propagation() {
        fn inner() -> BearDogResult<i32> {
            Err(BearDogError::internal("Inner error".to_string()))
        }

        let result = inner();
        assert!(result.is_err());
    }
}

#[cfg(test)]
mod error_matching_tests {
    use super::*;

    #[test]
    fn test_match_system_error() {
        let error = BearDogError::system("Test".to_string());
        match error {
            BearDogError::System { .. } => { /* Expected */ }
            _ => panic!("Wrong error variant"),
        }
    }

    #[test]
    fn test_match_security_error() {
        let error = BearDogError::security("Test".to_string());
        match error {
            BearDogError::Security { .. } => { /* Expected */ }
            _ => panic!("Wrong error variant"),
        }
    }

    #[test]
    fn test_match_business_error() {
        let error = BearDogError::business("Test".to_string());
        match error {
            BearDogError::Business { .. } => { /* Expected */ }
            _ => panic!("Wrong error variant"),
        }
    }
}

#[cfg(test)]
mod error_edge_cases {
    use super::*;

    #[test]
    fn test_empty_message() {
        let error = BearDogError::internal(String::new());
        let display = format!("{}", error);
        assert!(!display.is_empty());
    }

    #[test]
    fn test_long_message() {
        let long_msg = "a".repeat(1000);
        let error = BearDogError::internal(long_msg);
        let display = format!("{}", error);
        assert!(display.len() > 900);
    }

    #[test]
    fn test_unicode_message() {
        let error = BearDogError::internal("Error: 测试错误 🔥".to_string());
        let display = format!("{}", error);
        assert!(display.contains("测试错误"));
    }

    #[test]
    fn test_newline_in_message() {
        let error = BearDogError::internal("Line 1\nLine 2".to_string());
        let display = format!("{}", error);
        assert!(display.contains("Line 1"));
    }
}

#[cfg(test)]
mod error_helper_functions {
    use super::*;

    #[test]
    fn test_system_error_helper() {
        let error = BearDogError::system("System".to_string());
        assert!(format!("{:?}", error).contains("System"));
    }

    #[test]
    fn test_security_error_helper() {
        let error = BearDogError::security("Security".to_string());
        assert!(format!("{:?}", error).contains("Security"));
    }

    #[test]
    fn test_internal_error_helper() {
        let error = BearDogError::internal("Internal".to_string());
        assert!(format!("{:?}", error).contains("System"));
    }

    #[test]
    fn test_not_found_error_helper() {
        let error = BearDogError::not_found("Not found".to_string());
        assert!(format!("{:?}", error).contains("Business"));
    }

    #[test]
    fn test_unauthorized_error_helper() {
        let error = BearDogError::unauthorized("Unauthorized".to_string());
        assert!(format!("{:?}", error).contains("Security"));
    }

    #[test]
    fn test_configuration_error_helper() {
        let error = BearDogError::configuration("Config error");
        assert!(format!("{:?}", error).contains("System"));
    }
}

#[cfg(test)]
mod error_std_error_trait {
    use super::*;
    use std::error::Error;

    #[test]
    fn test_error_trait_implemented() {
        let error: Box<dyn Error> = Box::new(BearDogError::internal("Test".to_string()));
        assert!(!error.to_string().is_empty());
    }

    #[test]
    fn test_error_as_trait_object() {
        let error = BearDogError::system("System error".to_string());
        let _: &dyn Error = &error;
    }
}

#[cfg(test)]
mod error_clone_tests {
    use super::*;

    #[test]
    fn test_error_clone() {
        let error1 = BearDogError::internal("Original".to_string());
        let error2 = error1.clone();
        assert_eq!(format!("{:?}", error1), format!("{:?}", error2));
    }

    #[test]
    fn test_error_clone_multiple() {
        let error = BearDogError::security("Security".to_string());
        let _clone1 = error.clone();
        let _clone2 = error.clone();
        // Should not panic
    }
}

#[cfg(test)]
mod error_send_sync_tests {
    use super::*;

    #[test]
    fn test_error_is_send() {
        fn assert_send<T: Send>() {}
        assert_send::<BearDogError>();
    }

    #[test]
    fn test_error_is_sync() {
        fn assert_sync<T: Sync>() {}
        assert_sync::<BearDogError>();
    }

    #[test]
    fn test_result_is_send() {
        fn assert_send<T: Send>() {}
        assert_send::<BearDogResult<i32>>();
    }

    #[test]
    fn test_result_is_sync() {
        fn assert_sync<T: Sync>() {}
        assert_sync::<BearDogResult<i32>>();
    }
}

#[cfg(test)]
mod error_category_tests {
    use super::*;
    use crate::categories::*;

    #[test]
    fn test_system_with_category() {
        let error = BearDogError::system_with_category("IO error", SystemErrorCategory::FileSystem);
        assert!(matches!(error, BearDogError::System { .. }));
    }

    #[test]
    fn test_security_with_category() {
        let error =
            BearDogError::security_with_category("Crypto error", SecurityErrorCategory::Encryption);
        assert!(matches!(error, BearDogError::Security { .. }));
    }

    #[test]
    fn test_business_with_category() {
        let error =
            BearDogError::business_with_category("State error", BusinessErrorCategory::State);
        assert!(matches!(error, BearDogError::Business { .. }));
    }
}
