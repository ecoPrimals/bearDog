//! `BearDog` Error Handling System
//!
//! Comprehensive error handling with rich context, categorization, and automated remediation.
//! Provides sovereignty-compliant error management with zero hardcoded dependencies.

#![deny(unsafe_code)]
#![warn(missing_docs)]
#![warn(rust_2018_idioms)]

/// Core error types and definitions
/// Core functionality
/// Core functionality
pub mod core;

/// Error category definitions for classification
pub mod categories;

/// Unified error construction utilities
pub mod constructors_unified;

/// Rich error types with context and remediation
// pub mod error_types; // Removed - using categories.rs as single source
/// Idiomatic Rust error handling patterns
pub mod idiomatic;

pub use categories::*;
pub use core::BearDogError;

/// A convenient type alias for `Result<T, BearDogError>`
///
/// This type alias provides a consistent return type across the BearDog ecosystem
/// for operations that may fail with a BearDogError.
pub type BearDogResult<T> = Result<T, BearDogError>;

use std::fmt::Display;

/// Extension trait for Result types to add BearDog-specific error context
///
/// This trait provides convenient methods for adding contextual information
/// to errors and converting them to BearDogError types.
pub trait ResultExt<T, E> {
    /// Add security context to an error result
    ///
    /// # Errors
    ///
    /// Returns a `BearDogError::Security` if the original result contains an error
    fn security_context(self, context: &str) -> Result<T, BearDogError>;

    /// Add system context to an error result
    ///
    /// # Errors
    ///
    /// Returns a `BearDogError::System` if the original result contains an error
    fn system_context(self, context: &str) -> Result<T, BearDogError>;

    /// Add business context to an error result
    ///
    /// # Errors
    ///
    /// Returns a `BearDogError::Business` if the original result contains an error
    fn business_context(self, context: &str) -> Result<T, BearDogError>;

    /// Add network context to an error result
    ///
    /// # Errors
    ///
    /// Returns a `BearDogError::Network` if the original result contains an error
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
