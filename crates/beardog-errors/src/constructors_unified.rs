// SPDX-License-Identifier: AGPL-3.0-or-later

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
    BearDogError::System {
        message: format!("Unsupported operation: {operation}"),
        category: SystemErrorCategory::NotSupported,
    }
}

/// Create a not implemented error
#[must_use]
pub fn not_implemented(feature: &str) -> BearDogError {
    BearDogError::System {
        message: format!("Not yet implemented: {feature}"),
        category: SystemErrorCategory::NotImplemented,
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

// ═══════════════════════════════════════════════════════════════════════════
// ENHANCED CONSTRUCTORS WITH REMEDIATION HINTS
// ═══════════════════════════════════════════════════════════════════════════

/// Enhanced authentication error with remediation hint
///
/// Use this constructor when authentication fails and you want to provide
/// actionable guidance to help resolve the issue.
///
/// # Examples
///
/// ```
/// use beardog_errors::authentication_error_with_hint;
///
/// let error = authentication_error_with_hint(
///     "JWT token signature verification failed",
///     "Verify the token was signed with the correct key. Check configuration: beardog.auth.jwt_secret"
/// );
/// ```
#[must_use]
pub fn authentication_error_with_hint(reason: &str, hint: &str) -> BearDogError {
    BearDogError::Security {
        message: format!("Authentication failed: {reason}\n💡 Hint: {hint}"),
        category: SecurityErrorCategory::Authentication,
    }
}

/// Enhanced authorization error with remediation hint
///
/// Use this constructor when access is denied and you want to provide
/// guidance on what permissions or roles are required.
///
/// # Examples
///
/// ```
/// use beardog_errors::authorization_error_with_hint;
///
/// let error = authorization_error_with_hint(
///     "/api/admin/users",
///     "DELETE",
///     "Requires 'admin' role or 'users:delete' permission"
/// );
/// ```
#[must_use]
pub fn authorization_error_with_hint(resource: &str, action: &str, hint: &str) -> BearDogError {
    BearDogError::Security {
        message: format!(
            "Access denied for action '{action}' on resource '{resource}'\n💡 Hint: {hint}"
        ),
        category: SecurityErrorCategory::Authorization,
    }
}

/// Enhanced validation error with suggestion
///
/// Use this constructor when validation fails and you want to provide
/// a helpful suggestion for fixing the input.
///
/// # Examples
///
/// ```
/// use beardog_errors::validation_error_with_suggestion;
///
/// let error = validation_error_with_suggestion(
///     "email",
///     "Must be a valid email address",
///     "Use format: user@domain.com or check for typos"
/// );
/// ```
#[must_use]
pub fn validation_error_with_suggestion(
    field: &str,
    issue: &str,
    suggestion: &str,
) -> BearDogError {
    BearDogError::Business {
        message: format!("Validation failed for '{field}': {issue}\n💡 Suggestion: {suggestion}"),
        category: BusinessErrorCategory::Validation,
    }
}

/// Network error with full context
///
/// Use this constructor when network operations fail and you want to provide
/// complete context for debugging, including the endpoint and operation.
///
/// # Examples
///
/// ```
/// use beardog_errors::network_error_with_context;
///
/// let error = network_error_with_context(
///     "Connection timeout after 30s",
///     "tcp://consul.service.local:8500",
///     "service_discovery_init"
/// );
/// ```
#[must_use]
pub fn network_error_with_context(message: &str, endpoint: &str, operation: &str) -> BearDogError {
    BearDogError::System {
        message: format!(
            "Network operation '{operation}' failed: {message}\n\
             🌐 Endpoint: {endpoint}\n\
             💡 Check network connectivity and firewall rules"
        ),
        category: SystemErrorCategory::General,
    }
}

/// Configuration error with documentation link
///
/// Use this constructor when configuration is invalid and you want to point
/// users to relevant documentation for troubleshooting.
///
/// # Examples
///
/// ```
/// use beardog_errors::configuration_error_with_docs;
///
/// let error = configuration_error_with_docs(
///     "HSM provider 'yubico' not found in configuration",
///     "hsm",
///     "docs/hsm/providers.md#supported-providers"
/// );
/// ```
#[must_use]
pub fn configuration_error_with_docs(
    issue: &str,
    component: &str,
    docs_link: &str,
) -> BearDogError {
    BearDogError::System {
        message: format!(
            "Configuration error in {component}: {issue}\n\
             📚 Documentation: {docs_link}"
        ),
        category: SystemErrorCategory::General,
    }
}

/// Cryptographic error with detailed context
///
/// Use this constructor when cryptographic operations fail and you want to
/// provide detailed information about what went wrong and how to fix it.
///
/// # Examples
///
/// ```
/// use beardog_errors::crypto_error_with_details;
///
/// let error = crypto_error_with_details(
///     "AES-256-GCM encryption",
///     "Key size is 128 bits, expected 256 bits",
///     "Ensure you're using generate_key_256() or check key derivation"
/// );
/// ```
#[must_use]
pub fn crypto_error_with_details(operation: &str, details: &str, hint: &str) -> BearDogError {
    BearDogError::Security {
        message: format!(
            "Cryptographic operation '{operation}' failed: {details}\n💡 Hint: {hint}"
        ),
        category: SecurityErrorCategory::Encryption,
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
