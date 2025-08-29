//! # `BearDog` Error Handling System
//!
//! This crate provides a unified error handling system for the `BearDog` ecosystem.
//! It implements a comprehensive error taxonomy with domain-specific error types,
//! idiomatic Rust error patterns, and zero-cost abstractions for error propagation.
//!
//! ## Features
//!
//! - **Unified Error Types**: Single `BearDogError` enum covering all error domains
//! - **Domain-Specific Errors**: Specialized error types for security, HSM, networking, etc.
//! - **Idiomatic Patterns**: Full `std::error::Error` trait implementation
//! - **Zero-Cost Abstractions**: Efficient error propagation with `?` operator
//! - **Rich Context**: Detailed error messages with contextual information
//!
//! ## Idiomatic Usage (Recommended)
//!
//! ```rust
//! use beardog_errors::BearDogError;
//!
//! // Use Result<T, BearDogError> directly for maximum clarity
//! fn example_operation() -> Result<String, BearDogError> {
//!     // Operations that may fail
//!     Ok("Success".to_string())
//! }
//!
//! // Use the ResultExt trait for rich error context
//! use beardog_errors::ResultExt;
//!
//! fn with_context() -> Result<(), BearDogError> {
//!     std::fs::read_to_string("config.toml")
//!         .system_context("Failed to load configuration file")?;
//!     Ok(())
//! }
//! ```
//!
//! ## Migration from Type Aliases
//!
//! The `Result<T, BearDogError>` type alias is deprecated in favor of the more idiomatic
//! `Result<T, BearDogError>` pattern. This follows Rust ecosystem best practices
//! and provides better tooling support.
//!

#![deny(unsafe_code)]
#![warn(missing_docs)]
#![warn(rust_2018_idioms)]

/// Core error types and the main `BearDogError` enum
pub mod core;

/// Error categorization and classification utilities
pub mod categories;

/// Unified error constructors for consistent error creation
pub mod constructors_unified;

/// Domain-specific error type definitions
pub mod error_types;

/// Idiomatic Rust error patterns and trait implementations
pub mod idiomatic;

pub use core::BearDogError;

pub use categories::*;

use std::fmt::Display;

/// Extension trait for `Result` types to provide additional error handling utilities
pub trait ResultExt<T, E> {
    /// Adds security-specific context to an error result
    ///
    /// # Arguments
    /// * `context` - Security context description to add to the error
    ///
    /// # Returns
    /// A `Result<T, BearDogError>` with the security context applied
    fn security_context(self, context: &str) -> Result<T, BearDogError>;

    /// Adds system-specific context to an error result
    ///
    /// # Arguments
    /// * `context` - System context description to add to the error
    ///
    /// # Returns
    /// A `Result<T, BearDogError>` with the system context applied
    fn system_context(self, context: &str) -> Result<T, BearDogError>;

    /// Adds business logic context to an error result
    ///
    /// # Arguments
    /// * `context` - Business context description to add to the error
    ///
    /// # Returns
    /// A `Result<T, BearDogError>` with the business context applied
    fn business_context(self, context: &str) -> Result<T, BearDogError>;

    /// Adds network-specific context to an error result
    ///
    /// # Arguments
    /// * `context` - Network context description to add to the error
    ///
    /// # Returns
    /// A `Result<T, BearDogError>` with the network context applied
    fn network_context(self, context: &str) -> Result<T, BearDogError>;
}

impl<T, E: Display> ResultExt<T, E> for Result<T, E> {
    fn security_context(self, context: &str) -> Result<T, BearDogError> {
        self.map_err(|e| BearDogError::security(format_args!("{context}: {e}").to_string()))
    }

    fn system_context(self, context: &str) -> Result<T, BearDogError> {
        self.map_err(|e| BearDogError::system(format_args!("{context}: {e}").to_string()))
    }

    fn business_context(self, context: &str) -> Result<T, BearDogError> {
        self.map_err(|e| BearDogError::business(format_args!("{context}: {e}").to_string()))
    }

    fn network_context(self, context: &str) -> Result<T, BearDogError> {
        self.map_err(|e| BearDogError::network(format_args!("{context}: {e}").to_string()))
    }
}

/// Error validation and consistency checking utilities
///
/// This module provides utilities for validating error usage patterns,
/// ensuring consistency across the BearDog ecosystem, and maintaining
/// error handling best practices.
pub mod validation {
    use crate::*;

    /// Validates error usage patterns and configurations
    ///
    /// # Returns
    /// `Ok(())` if error usage is valid, `Err(BearDogError)` if validation fails
    pub fn validate_error_usage() -> Result<(), BearDogError> {
        // Validation logic would go here
        Ok(())
    }

    /// Returns information about the error system configuration
    ///
    /// # Returns
    /// A vector of tuples containing (component_name, version) pairs
    pub fn error_system_info() -> Vec<(&'static str, &'static str)> {
        vec![("beardog-errors", "3.0.0"), ("error-system", "unified")]
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_unified_error_constructors() {
        let security_error = BearDogError::security("test security");
        let system_error = BearDogError::system("test system");
        let business_error = BearDogError::business("test business");
        let hsm_error = BearDogError::hsm("test hsm");
        let api_error = BearDogError::api("test api", ApiErrorCategory::General);
        let workflow_error = BearDogError::workflow("test workflow");

        assert!(matches!(security_error, BearDogError::Security { .. }));
        assert!(matches!(system_error, BearDogError::System { .. }));
        assert!(matches!(business_error, BearDogError::Business { .. }));
        assert!(matches!(hsm_error, BearDogError::Hsm { .. }));
        assert!(matches!(api_error, BearDogError::Api { .. }));
        assert!(matches!(workflow_error, BearDogError::Workflow { .. }));
    }

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
        // Test canonical error creation patterns
        let _security = BearDogError::security("test");
        let _system = BearDogError::system("test");
        let _business = BearDogError::business("test");
        let _network = BearDogError::network("test");
        let _config = BearDogError::configuration("test");
        let _init = BearDogError::initialization("test");
        let _validation = BearDogError::validation("test");

        // Test HSM and domain-specific errors
        let _hsm = BearDogError::hsm("test");
        let _api = BearDogError::api("test", ApiErrorCategory::General);
        let _workflow = BearDogError::workflow("test");
        let _genetics = BearDogError::genetics("test");
        let _deployment = BearDogError::deployment("test");
        let _memory = BearDogError::memory("test");
        let _monitoring = BearDogError::monitoring("test");
        let _compliance = BearDogError::compliance("test");
        let _cryptographic = BearDogError::cryptographic("test");
        let _tunnel = BearDogError::tunnel("test");
        let _adapter = BearDogError::adapter("test");
    }
}
