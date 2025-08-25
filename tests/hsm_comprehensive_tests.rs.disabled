//! HSM Comprehensive Tests - Modular Entry Point
//!
//! Comprehensive test suite for BearDog's Hardware Security Module integration.
//! Now organized into focused, maintainable modules.

mod hsm;

pub use hsm::*;

/// Main HSM comprehensive test suite
#[tokio::test]
async fn test_hsm_comprehensive_suite() -> beardog::BearDogResult<()> {
    hsm::integration_tests::run_comprehensive_hsm_tests().await
}

/// HSM error handling test
#[tokio::test]
async fn test_hsm_error_handling() -> beardog::BearDogResult<()> {
    hsm::integration_tests::test_hsm_error_handling().await
}

/// HSM system integration end-to-end test
#[tokio::test]
async fn test_hsm_system_integration_e2e() -> beardog::BearDogResult<()> {
    hsm::integration_tests::test_hsm_system_integration_e2e().await
} 