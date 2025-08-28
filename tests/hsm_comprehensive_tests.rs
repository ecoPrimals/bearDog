use beardog_errors::BearDogError;


mod hsm;

pub use hsm::*;

#[tokio::test]
async fn test_hsm_comprehensive_suite() -> beardog::Result<(), BearDogError> {
    hsm::integration_tests::run_comprehensive_hsm_tests().await
}

#[tokio::test]
async fn test_hsm_error_handling() -> beardog::Result<(), BearDogError> {
    hsm::integration_tests::test_hsm_error_handling().await
}

#[tokio::test]
async fn test_hsm_system_integration_e2e() -> beardog::Result<(), BearDogError> {
    hsm::integration_tests::test_hsm_system_integration_e2e().await
} 