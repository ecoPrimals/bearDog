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
//! use beardog_errors::{BearDogError, BearDogResult, ResultExt};
//!
//! // Create domain-specific errors
//! fn authenticate_user(token: &str) -> BearDogResult<UserId> {
//!     if token.is_empty() {
//!         return Err(BearDogError::security("Invalid authentication token"));
//!     }
//!     // ... authentication logic
//!     Ok(UserId::new(123))
//! }
//!
//! // Add context to external errors
//! fn load_config(path: &str) -> BearDogResult<Config> {
//!     std::fs::read_to_string(path)
//!         .system_context("Failed to read configuration file")?;
//!     // ... parse config
//!     Ok(Config::default())
//! }
//!
//! // Error propagation with ?
//! fn process_request() -> BearDogResult<Response> {
//!     let user = authenticate_user(token)?;
//!     let config = load_config("/etc/beardog/config.toml")?;
//!     Ok(Response::success())
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
//! let err = BearDogError::security("Authentication failed");
//! let err = BearDogError::system("Out of memory");
//! let err = BearDogError::business("Invalid email format");
//! let err = BearDogError::network("Connection timeout");
//! ```

#![deny(unsafe_code)]
#![warn(missing_docs)]
#![warn(rust_2018_idioms)]

/// Core error types and definitions
///
/// The main `BearDogError` enum and core error handling functionality.
pub mod core;

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

pub use categories::*;
pub use core::BearDogError;

/// Convenient type alias for `Result<T, BearDogError>`
///
/// This type alias provides a consistent return type across the `BearDog` ecosystem
/// for operations that may fail with a `BearDogError`.
///
/// ## Usage
///
/// ```rust
/// use beardog_errors::{BearDogResult, BearDogError};
///
/// fn do_something() -> BearDogResult<String> {
///     if condition_fails() {
///         return Err(BearDogError::business("Operation failed"));
///     }
///     Ok("Success!".to_string())
/// }
/// ```
///
/// This is equivalent to `Result<T, BearDogError>` but more concise.
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
/// use beardog_errors::{ResultExt, BearDogResult};
///
/// fn read_user_data(path: &str) -> BearDogResult<Vec<u8>> {
///     // Wrap std::fs errors with BearDog context
///     let data = std::fs::read(path)
///         .system_context("Failed to read user data file")?;
///     Ok(data)
/// }
///
/// fn authenticate(token: &str) -> BearDogResult<User> {
///     // Wrap authentication errors with security context
///     validate_token(token)
///         .security_context("Token validation failed")?;
///     Ok(User::new())
/// }
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
    /// # use beardog_errors::{ResultExt, BearDogResult};
    /// fn verify_signature(data: &[u8], sig: &[u8]) -> BearDogResult<()> {
    ///     crypto_lib::verify(data, sig)
    ///         .security_context("Signature verification failed")?;
    ///     Ok(())
    /// }
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
    /// # use beardog_errors::{ResultExt, BearDogResult};
    /// fn allocate_buffer(size: usize) -> BearDogResult<Vec<u8>> {
    ///     vec_with_capacity(size)
    ///         .system_context("Buffer allocation failed")?;
    ///     Ok(vec![0; size])
    /// }
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
    /// # use beardog_errors::{ResultExt, BearDogResult};
    /// fn validate_email(email: &str) -> BearDogResult<()> {
    ///     email_validator::validate(email)
    ///         .business_context("Invalid email address format")?;
    ///     Ok(())
    /// }
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
    /// # use beardog_errors::{ResultExt, BearDogResult};
    /// fn fetch_data(url: &str) -> BearDogResult<String> {
    ///     http_client::get(url)
    ///         .network_context("Failed to fetch data from API")?;
    ///     Ok("data".to_string())
    /// }
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
mod tests {
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
        assert!(result.is_ok(), "Error usage validation should pass");

        let info = validation::error_system_info();
        assert!(!info.is_empty());
        let system_names: Vec<&str> = info.iter().map(|(name, _)| *name).collect();
        assert!(system_names.contains(&"beardog-errors"));
        assert!(system_names.contains(&"error-system"));
    }

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
