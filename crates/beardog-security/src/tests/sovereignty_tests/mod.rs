//! Sovereignty Comprehensive Test Suite
//!
//! TEST_CATEGORY: unit + integration
//! TEST_DOMAIN: security/sovereignty
//! TEST_PRIORITY: critical
//!
//! This module provides comprehensive testing for sovereignty mechanisms.
//! Originally consolidated from a single 1,574-line file, now split into focused modules.
//!
//! ## Test Modules
//!
//! - `types` - Shared test helper types and utilities
//! - `compliance_tests` - Data residency and compliance tests
//! - `crypto_tests` - Cryptographic sovereignty enforcement tests
//! - `trust_tests` - Trust management tests
//! - `access_control_tests` - Access control sovereignty tests
//! - `audit_tests` - Sovereignty audit trail tests
//! - `edge_cases_tests` - Edge case and boundary tests

pub mod access_control_tests;
pub mod audit_tests;
pub mod compliance_tests;
pub mod crypto_tests;
pub mod edge_cases_tests;
pub mod trust_tests;
pub mod types;

// Re-export commonly used items for convenience
