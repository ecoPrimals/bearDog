//! BearDog Security Tests Module
//!
//! Comprehensive test coverage for security functionality

#[cfg(test)]
mod crypto_primitives_tests;

#[cfg(test)]
mod access_control_tests;

#[cfg(test)]
mod security_integration_tests;

#[cfg(test)]
mod key_management_tests;

#[cfg(test)]
mod encryption_edge_cases_tests;

#[cfg(test)]
mod encryption_edge_cases_comprehensive_tests;

#[cfg(test)]
mod security_primitives_comprehensive_tests;

#[cfg(test)]
mod encryption_comprehensive_tests;

#[cfg(test)]
mod crypto_coverage_tests;

// New comprehensive test modules - October 17, 2025
#[cfg(test)]
mod crypto_utils_comprehensive_tests;

#[cfg(test)]
mod access_control_expanded_tests;

// Day 2: HSM Operations Comprehensive Tests - October 17, 2025
// Split into 3 modules on October 24, 2025 (file size compliance: 1291→~400 lines each)
#[cfg(test)]
mod hsm_basic_tests;

#[cfg(test)]
mod hsm_advanced_tests;

#[cfg(test)]
mod hsm_integration_tests;

// Day 3: Error Handling Comprehensive Tests - October 17, 2025
#[cfg(test)]
mod error_handling_tests;

#[cfg(test)]
mod crypto_operations_comprehensive_tests;

// Day 4: Error Path Tests - October 17, 2025 Evening
#[cfg(test)]
mod error_path_tests;

// October 18, 2025: Hash Comprehensive Tests
#[cfg(test)]
mod hash_comprehensive_tests;

// October 18, 2025: Constant-Time Operations Comprehensive Tests
#[cfg(test)]
mod constant_time_comprehensive_tests;

// October 18, 2025 Evening: Authentication Comprehensive Tests
#[cfg(test)]
mod authentication_comprehensive_tests;

// October 18, 2025 Evening: Signature Verification Comprehensive Tests
#[cfg(test)]
mod signature_verification_comprehensive_tests;

// October 19, 2025: Crypto Error Path Tests (Coverage Expansion)
// DISABLED: Outdated - references non-existent functions (hash_data, verify_hash)
// #[cfg(test)]
// mod crypto_error_paths_tests;

// October 19, 2025 Evening: Key Rotation Tests (Critical Security Paths)
#[cfg(test)]
mod key_rotation_tests;

// October 20, 2025: Phase 1 Sovereign Science Grade - Error Recovery & Input Validation
#[cfg(test)]
mod error_recovery_comprehensive_tests;

#[cfg(test)]
mod input_validation_comprehensive_tests;

// October 20, 2025 - Day 2: Crypto & HSM Edge Cases Expansion
#[cfg(test)]
mod crypto_edge_cases_extended_tests;

#[cfg(test)]
mod hsm_edge_cases_extended_tests;

// October 20, 2025 - Day 3: Authentication & Key Lifecycle Expansion
#[cfg(test)]
mod authentication_flow_tests;

#[cfg(test)]
mod key_lifecycle_tests;

// October 22, 2025 - Test Coverage Expansion Phase 1
#[cfg(test)]
mod config_validation_tests;

#[cfg(test)]
mod entropy_source_tests;

#[cfg(test)]
mod hash_validation_tests;

#[cfg(test)]
mod error_context_tests;

// October 22, 2025 - Crypto Error Boundary Tests (Coverage Expansion)
#[cfg(test)]
mod crypto_error_boundary_tests;

// October 22, 2025 - Security Edge Cases (High-Value Test Coverage Expansion)
#[cfg(test)]
mod security_edge_cases_oct22;
