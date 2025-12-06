//! Integration tests for HSM functionality

pub mod key_lifecycle_tests;
pub mod provider_selection_tests;

#[cfg(test)]
pub mod hsm_provider_error_paths;

#[cfg(test)]
pub mod connection_error_paths;

#[cfg(test)]
pub mod crypto_provider_failures;

#[cfg(test)]
pub mod session_management_errors;

// December 1, 2025: Substantive integration tests for production paths
#[cfg(test)]
pub mod key_lifecycle_integration_tests;
