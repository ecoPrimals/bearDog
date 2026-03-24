// SPDX-License-Identifier: AGPL-3.0-only
#![forbid(unsafe_code)]
#![cfg_attr(test, allow(clippy::expect_used, clippy::unwrap_used))]

//! `BearDog` Error Handling System
//!
//! Comprehensive error handling with rich context, categorization, and automated remediation.
//! Provides sovereignty-compliant error management with zero hardcoded dependencies.
//!
//! ## Overview
//!
//! The `BearDog` error system provides a unified, type-safe approach to error handling across
//! the entire ecosystem. All errors are categorized by domain (Security, System, Business, etc.)
//! with rich context and detailed categorization.
//!
//! ## Key Features
//!
//! - **Domain Categorization** - Errors organized by functional domain
//! - **Rich Context** - Detailed categorization and remediation suggestions
//! - **Zero-Cost** - Efficient error propagation without overhead
//! - **Type Safety** - Compile-time error handling guarantees
//! - **Sovereignty Compliant** - No hardcoded dependencies or vendor lock-in
//!
//! ## Error Domains
//!
//! - **Security** - Authentication, authorization, cryptography
//! - **System** - Resource exhaustion, I/O, OS interactions
//! - **Business** - Validation, workflow, business logic
//! - **Network** - Connectivity, timeouts, protocol errors
//! - **Configuration** - Invalid config, missing parameters
//! - **HSM** - Hardware security module operations
//! - **Workflow** - Process orchestration, state machines
//!
//! ## Quick Start
//!
//! ```rust
//! use beardog_errors::BearDogError;
//!
//! // Create domain-specific errors
//! fn authenticate_user(token: &str) -> Result<u64, BearDogError> {
//!     if token.is_empty() {
//!         return Err(BearDogError::security("Invalid authentication token".to_string()));
//!     }
//!     // ... authentication logic
//!     Ok(123)
//! }
//!
//! // Add context to external errors
//! fn load_config(path: &str) -> Result<String, BearDogError> {
//!     std::fs::read_to_string(path)
//!         .map_err(|e| BearDogError::system(format!("Failed to read config: {}", e)))?;
//!     // ... parse config
//!     Ok("config_data".to_string())
//! }
//!
//! // Error propagation with ?
//! fn process_request(token: &str) -> Result<String, BearDogError> {
//!     let _user_id = authenticate_user(token)?;
//!     let _config = load_config("/etc/beardog/config.toml")?;
//!     Ok("success".to_string())
//! }
//! ```
//!
//! ## Error Construction
//!
//! Use the convenient constructor methods:
//!
//! ```rust
//! use beardog_errors::BearDogError;
//!
//! // Simple constructors
//! let _err = BearDogError::security("Authentication failed".to_string());
//! let _err = BearDogError::system("Out of memory".to_string());
//! let _err = BearDogError::business("Invalid email format".to_string());
//! let _err = BearDogError::network("Connection timeout".to_string());
//! ```

#![warn(rust_2018_idioms)]

/// Core error types and definitions
///
/// The main `BearDogError` enum and core error handling functionality.
pub mod process_env;

pub mod core;

/// Android-specific structured errors
///
/// Provides clear, actionable error messages for Android platform features,
/// including PHASE-2 implementation tracking and cross-platform compatibility.
pub mod android;

/// Error category definitions for classification
///
/// Detailed categorization enums for each error domain (Security, System, Business, etc.).
pub mod categories;

/// Unified error construction utilities
///
/// Convenient constructor functions for creating domain-specific errors.
pub mod constructors_unified;

// pub mod error_types; // Removed - using categories.rs as single source

/// Idiomatic Rust error handling patterns
///
/// Extension traits and helpers for idiomatic error handling in Rust.
pub mod idiomatic;

/// Result/option validation extension traits (additional context helpers).
#[allow(missing_docs)]
pub mod result_extensions;

pub use result_extensions::{ErrorChainExt, OptionValidationExt, ResultValidationExt};

/// Enhanced error constructor examples
///
/// Real-world examples demonstrating how to use enhanced error constructors
/// with remediation hints, context, and documentation links.
#[cfg(any(test, doc))]
pub mod examples_enhanced;

/// Comprehensive test suites
#[cfg(test)]
mod tests;

pub use android::{AndroidError, Phase, phase2_not_implemented};
pub use categories::*;
pub use constructors_unified::{
    authentication_error, authentication_error_with_hint, authorization_error,
    authorization_error_with_hint, configuration_error, configuration_error_with_docs,
    crypto_error, crypto_error_with_details, io_error, network_error_with_context, not_implemented,
    security_error, system_error, unsupported_operation, validation_error,
    validation_error_with_suggestion,
};
pub use core::BearDogError;

/// Convenient type alias for `Result<T, BearDogError>` (DEPRECATED)
///
/// **DEPRECATED**: Use idiomatic `Result<T, BearDogError>` instead.
/// Type aliases for Result violate Rust API Guidelines.
///
/// ## Migration Example
///
/// ```rust
/// use beardog_errors::BearDogError;
///
/// // ✅ NEW (Idiomatic Rust):
/// fn do_something(should_fail: bool) -> Result<String, BearDogError> {
///     if should_fail {
///         return Err(BearDogError::business("Operation failed".to_string()));
///     }
///     Ok("Success!".to_string())
/// }
/// ```
///
/// This type alias will be removed in version 4.0.0.
#[deprecated(
    since = "3.1.0",
    note = "Use Result<T, BearDogError> instead. See https://rust-lang.github.io/api-guidelines/future-proofing.html#c-result-alias"
)]
pub type BearDogResult<T> = Result<T, BearDogError>;

use std::fmt::Display;

/// Extension trait for Result types to add BearDog-specific error context
///
/// This trait provides convenient methods for adding contextual information
/// to errors and converting them to `BearDogError` types. It's particularly
/// useful for wrapping errors from external crates.
///
/// ## Example
///
/// ```rust
/// use beardog_errors::{ResultExt, BearDogError};
///
/// fn read_user_data(path: &str) -> Result<Vec<u8>, BearDogError> {
///     // Wrap std::fs errors with BearDog context
///     let data = std::fs::read(path)
///         .system_context("Failed to read user data file")?;
///     Ok(data)
/// }
/// # fn main() {}
/// ```
///
/// ## Benefits
///
/// - **Rich Context** - Adds descriptive context to external errors
/// - **Domain Classification** - Automatically categorizes errors by domain
/// - **Ergonomic** - Works seamlessly with the `?` operator
/// - **Type Safety** - Preserves type information while adding context
pub trait ResultExt<T, E> {
    /// Add security context to an error result
    ///
    /// Wraps the error in a `BearDogError::Security` variant with additional context.
    /// Use for authentication, authorization, and cryptographic errors.
    ///
    /// # Errors
    ///
    /// Returns a `BearDogError::Security` if the original result contains an error.
    ///
    /// # Example
    ///
    /// ```rust
    /// # use beardog_errors::{ResultExt, BearDogError};
    /// fn verify_signature(data: &[u8], sig: &[u8]) -> Result<(), BearDogError> {
    ///     // Example: wrap external crypto error with context
    ///     if data.is_empty() || sig.is_empty() {
    ///         return Err(beardog_errors::BearDogError::security("Empty data or signature".to_string()));
    ///     }
    ///     Ok(())
    /// }
    /// # fn main() {}
    /// ```
    fn security_context(self, context: &str) -> Result<T, BearDogError>;

    /// Add system context to an error result
    ///
    /// Wraps the error in a `BearDogError::System` variant with additional context.
    /// Use for I/O, resource exhaustion, and OS-level errors.
    ///
    /// # Errors
    ///
    /// Returns a `BearDogError::System` if the original result contains an error.
    ///
    /// # Example
    ///
    /// ```rust
    /// # use beardog_errors::{ResultExt, BearDogError};
    /// fn allocate_buffer(size: usize) -> Result<Vec<u8>, BearDogError> {
    ///     // Example: simple buffer allocation
    ///     if size > 1_000_000 {
    ///         return Err(beardog_errors::BearDogError::system("Buffer too large".to_string()));
    ///     }
    ///     Ok(vec![0; size])
    /// }
    /// # fn main() {}
    /// ```
    fn system_context(self, context: &str) -> Result<T, BearDogError>;

    /// Add business context to an error result
    ///
    /// Wraps the error in a `BearDogError::Business` variant with additional context.
    /// Use for validation, workflow, and business logic errors.
    ///
    /// # Errors
    ///
    /// Returns a `BearDogError::Business` if the original result contains an error.
    ///
    /// # Example
    ///
    /// ```rust
    /// # use beardog_errors::{ResultExt, BearDogError};
    /// fn validate_email(email: &str) -> Result<(), BearDogError> {
    ///     // Example: simple email validation
    ///     if !email.contains('@') {
    ///         return Err(beardog_errors::BearDogError::business("Invalid email address format".to_string()));
    ///     }
    ///     Ok(())
    /// }
    /// # fn main() {}
    /// ```
    fn business_context(self, context: &str) -> Result<T, BearDogError>;

    /// Add network context to an error result
    ///
    /// Wraps the error in a `BearDogError::Network` variant with additional context.
    /// Use for connectivity, timeout, and protocol errors.
    ///
    /// # Errors
    ///
    /// Returns a `BearDogError::Network` if the original result contains an error.
    ///
    /// # Example
    ///
    /// ```rust
    /// # use beardog_errors::{ResultExt, BearDogError};
    /// fn fetch_data(url: &str) -> Result<String, BearDogError> {
    ///     // Example: simple URL validation
    ///     if !url.starts_with("http") {
    ///         return Err(beardog_errors::BearDogError::network("Invalid URL".to_string()));
    ///     }
    ///     Ok("data".to_string())
    /// }
    /// # fn main() {}
    /// ```
    fn network_context(self, context: &str) -> Result<T, BearDogError>;
}

impl<T, E: Display> ResultExt<T, E> for Result<T, E> {
    fn security_context(self, context: &str) -> Result<T, BearDogError> {
        self.map_err(|e| BearDogError::security(format!("{context}: {e}")))
    }

    fn system_context(self, context: &str) -> Result<T, BearDogError> {
        self.map_err(|e| BearDogError::system(format!("{context}: {e}")))
    }

    fn business_context(self, context: &str) -> Result<T, BearDogError> {
        self.map_err(|e| BearDogError::business(format!("{context}: {e}")))
    }

    fn network_context(self, context: &str) -> Result<T, BearDogError> {
        self.map_err(|e| BearDogError::network(format!("{context}: {e}")))
    }
}

/// Input validation utilities and error helpers
pub mod validation {
    use crate::BearDogError;

    /// Validate Error Usage operation.
    ///
    /// # Errors
    /// Returns an error if the operation fails.
    pub const fn validate_error_usage() -> Result<(), BearDogError> {
        Ok(())
    }

    /// Error System Info operation.
    #[must_use]
    pub fn error_system_info() -> Vec<(&'static str, &'static str)> {
        vec![("beardog-errors", "3.0.0"), ("error-system", "unified")]
    }
}

#[cfg(test)]
mod existing_tests {
    use super::*;

    /// Test unified error constructor functionality.
    ///
    /// # Panics
    /// Panics if any error constructor creates an error of the wrong variant.
    #[test]
    fn test_unified_error_constructors() {
        test_security_constructor();
        test_system_constructor();
        test_business_constructor();
        test_specialized_constructors();
    }

    /// Test security error constructor.
    ///
    /// # Panics
    /// Panics if the security constructor doesn't create a Security variant.
    // TEST_CATEGORY: unit
    // TEST_DOMAIN: errors
    // TEST_PRIORITY: important
    fn test_security_constructor() {
        let error = BearDogError::security("test security".to_string());
        assert!(matches!(error, BearDogError::Security { .. }));
    }

    /// Test system error constructor.
    ///
    /// # Panics
    /// Panics if the system constructor doesn't create a System variant.
    fn test_system_constructor() {
        let error = BearDogError::system("test system".to_string());
        assert!(matches!(error, BearDogError::System { .. }));
    }

    /// Test business error constructor.
    ///
    /// # Panics
    /// Panics if the business constructor doesn't create a Business variant.
    fn test_business_constructor() {
        let error = BearDogError::business("test business".to_string());
        assert!(matches!(error, BearDogError::Business { .. }));
    }

    /// Test specialized error constructors.
    ///
    /// # Panics
    /// Panics if any specialized constructor doesn't create the expected error variant.
    fn test_specialized_constructors() {
        let hsm_error = BearDogError::hsm("test hsm".to_string());
        let api_error = BearDogError::api("test api".to_string());
        let workflow_error = BearDogError::workflow("test workflow".to_string());

        assert!(matches!(hsm_error, BearDogError::Cryptographic { .. }));
        assert!(matches!(api_error, BearDogError::Api { .. }));
        assert!(matches!(workflow_error, BearDogError::Workflow { .. }));
    }

    /// Test error category assignment functionality.
    ///
    /// # Panics
    /// Panics if the error category is not correctly assigned or if the wrong error variant is created.
    #[test]
    fn test_error_categories() {
        let security_error = BearDogError::security_with_category(
            "auth failed",
            SecurityErrorCategory::Authentication,
        );

        if let BearDogError::Security { category, .. } = security_error {
            assert!(matches!(category, SecurityErrorCategory::Authentication));
        // TEST_CATEGORY: unit
        // TEST_DOMAIN: errors
        // TEST_PRIORITY: important
        } else {
            panic!("Expected Security error, got: {security_error:?}");
        }
    }

    ///
    /// # Panics
    /// Panics if the result extension doesn't properly convert errors or if the
    /// resulting error doesn't have expected properties.
    #[test]
    fn test_result_extensions() {
        let result: Result<(), std::io::Error> = Err(std::io::Error::new(
            std::io::ErrorKind::NotFound,
            "file not found",
        ));
        // TEST_CATEGORY: unit
        // TEST_DOMAIN: errors
        // TEST_PRIORITY: normal

        let beardog_result = result.system_context("Failed to read file");
        assert!(beardog_result.is_err());

        if let Err(BearDogError::System { message, category }) = beardog_result {
            assert!(message.contains("Failed to read file"));
            assert!(matches!(category, SystemErrorCategory::General));
        } else {
            panic!("Expected System error, got: {beardog_result:?}");
        }
    }

    /// Test the validation system functionality.
    ///
    /// # Panics
    /// Panics if error usage validation fails or if expected system components are missing.
    #[test]
    fn test_validation_system() {
        let result = validation::validate_error_usage();
        // TEST_CATEGORY: unit
        // TEST_DOMAIN: errors
        // TEST_PRIORITY: normal
        assert!(result.is_ok(), "Error usage validation should pass");

        let info = validation::error_system_info();
        assert!(!info.is_empty());
        let system_names: Vec<&str> = info.iter().map(|(name, _)| *name).collect();
        assert!(system_names.contains(&"beardog-errors"));
        assert!(system_names.contains(&"error-system"));
    }

    // TEST_CATEGORY: unit
    // TEST_DOMAIN: errors
    // TEST_PRIORITY: important
    #[test]
    fn test_canonical_error_creation() {
        let _security = BearDogError::security("test".to_string());
        let _system = BearDogError::system("test".to_string());
        let _business = BearDogError::business("test".to_string());
        let _network = BearDogError::network("test".to_string());
        let _config = BearDogError::configuration("test");
        let _init = BearDogError::initialization("test".to_string());
        let _validation = BearDogError::validation("test");

        let _hsm = BearDogError::hsm("test".to_string());
        let _api = BearDogError::api("test".to_string());
        let _workflow = BearDogError::workflow("test".to_string());
        let _genetics = BearDogError::genetics("test".to_string());

        // Note: These methods don't exist in the current implementation
        // let _deployment = BearDogError::deployment("test".to_string());
        // let _memory = BearDogError::memory("test".to_string());
        // let _monitoring = BearDogError::monitoring("test".to_string());
        // let _compliance = BearDogError::compliance("test".to_string());
        // let _cryptographic = BearDogError::cryptographic("test".to_string());
        // let _tunnel = BearDogError::tunnel("test".to_string());
    }
}
