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

// December 6, 2025: Phase 2 Advanced HSM Provider Coverage Tests
#[cfg(test)]
pub mod hsm_advanced_coverage_tests;

// December 16, 2025: Comprehensive edge case coverage (78% → 90%)
#[cfg(test)]
pub mod crypto_edge_cases_comprehensive;
