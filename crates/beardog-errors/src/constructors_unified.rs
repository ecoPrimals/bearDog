// Unified error constructors for BearDog
// Provides consistent error creation patterns across the ecosystem

use crate::categories::{BusinessErrorCategory, SecurityErrorCategory, SystemErrorCategory};
use crate::core::BearDogError;

/// Create a security error with proper categorization
#[must_use]
pub fn security_error(message: &str, category: SecurityErrorCategory) -> BearDogError {
    BearDogError::Security {
        message: message.to_string(),
        category,
    }
}

/// Create a system error with proper categorization
#[must_use]
pub fn system_error(message: &str, category: SystemErrorCategory) -> BearDogError {
    BearDogError::System {
        message: message.to_string(),
        category,
    }
}

/// Create a business error with proper categorization
#[must_use]
pub fn business_error(message: &str, category: BusinessErrorCategory) -> BearDogError {
    BearDogError::Business {
        message: message.to_string(),
        category,
    }
}

/// Create a validation error (business logic validation)
#[must_use]
pub fn validation_error(field: &str, message: &str) -> BearDogError {
    BearDogError::Business {
        message: format!("Validation failed for {field}: {message}"),
        category: BusinessErrorCategory::Validation, // Fixed to use correct variant
    }
}

/// Create a network error (system-level networking)
#[must_use]
pub fn network_error(operation: &str, cause: &str) -> BearDogError {
    BearDogError::System {
        message: format!("Network operation '{operation}' failed: {cause}"),
        category: SystemErrorCategory::General, // Using General since Network doesn't exist
    }
}

/// Create a configuration error
#[must_use]
pub fn configuration_error(component: &str, issue: &str) -> BearDogError {
    BearDogError::System {
        message: format!("Configuration error in {component}: {issue}"),
        category: SystemErrorCategory::General, // Using General since Configuration doesn't exist
    }
}

/// Create an authentication error
#[must_use]
pub fn authentication_error(reason: &str) -> BearDogError {
    BearDogError::Security {
        message: format!("Authentication failed: {reason}"),
        category: SecurityErrorCategory::Authentication,
    }
}

/// Create an authorization error
#[must_use]
pub fn authorization_error(resource: &str, action: &str) -> BearDogError {
    BearDogError::Security {
        message: format!("Access denied for action '{action}' on resource '{resource}'"),
        category: SecurityErrorCategory::Authorization,
    }
}

/// Create a cryptographic error
#[must_use]
pub fn crypto_error(operation: &str, details: &str) -> BearDogError {
    BearDogError::Security {
        message: format!("Cryptographic operation '{operation}' failed: {details}"),
        category: SecurityErrorCategory::Encryption, // Using Encryption since Cryptographic doesn't exist
    }
}

/// Create an unsupported operation error
#[must_use]
pub fn unsupported_operation(operation: &str) -> BearDogError {
    BearDogError::Business {
        message: format!("Unsupported operation: {operation}"),
        category: BusinessErrorCategory::Validation,
    }
}

/// Create a not implemented error
#[must_use]
pub fn not_implemented(feature: &str) -> BearDogError {
    BearDogError::Business {
        message: format!("Not yet implemented: {feature}"),
        category: BusinessErrorCategory::Validation,
    }
}

/// Create an I/O error
#[must_use]
pub fn io_error(operation: &str, details: &str) -> BearDogError {
    BearDogError::System {
        message: format!("I/O operation '{operation}' failed: {details}"),
        category: SystemErrorCategory::FileSystem,
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    /// Test security error creation functionality.
    ///
    /// # Panics
    /// Panics if the created error is not a Security variant or has incorrect values.
    #[test]
    fn test_security_error_creation() {
        let error = security_error("Test error", SecurityErrorCategory::Authentication);
        match error {
            BearDogError::Security { message, category } => {
                assert_eq!(message, "Test error");
                // TEST_CATEGORY: unit
                // TEST_DOMAIN: errors
                // TEST_PRIORITY: important
                assert_eq!(category, SecurityErrorCategory::Authentication);
            }
            _ => panic!("Expected Security error"),
        }
    }

    /// Test validation error creation functionality.
    ///
    /// # Panics
    /// Panics if the created error is not a Business variant with Validation category,
    /// or if the message doesn't contain expected content.
    #[test]
    fn test_validation_error_creation() {
        let error = validation_error("username", "cannot be empty");
        // TEST_CATEGORY: unit
        // TEST_DOMAIN: errors
        // TEST_PRIORITY: important
        match error {
            BearDogError::Business { message, category } => {
                assert!(message.contains("username"));
                assert!(message.contains("cannot be empty"));
                assert_eq!(category, BusinessErrorCategory::Validation);
            }
            _ => panic!("Expected Business error"),
        }
    }

    /// Test network error creation functionality.
    ///
    /// # Panics
    /// Panics if the created error is not a System variant or if the message
    /// doesn't contain expected network-related content.
    // TEST_CATEGORY: unit
    // TEST_DOMAIN: errors
    // TEST_PRIORITY: important
    #[test]
    fn test_network_error_creation() {
        let error = network_error("connect", "timeout ");
        match error {
            BearDogError::System { message, category } => {
                assert!(message.contains("connect"));
                assert!(message.contains("timeout "));
                assert_eq!(category, SystemErrorCategory::General);
            }
            _ => panic!("Expected System error"),
        }
    }
}
