mod hsm;

pub use hsm::*;

#[tokio::test]
async fn test_hsm_comprehensive_suite() -> beardog::BearDogResult<()> {
    hsm::integration_tests::run_comprehensive_hsm_tests()
}

#[tokio::test]
async fn test_hsm_error_handling() -> beardog::BearDogResult<()> {
    hsm::integration_tests::test_hsm_error_handling()
}

#[tokio::test]
async fn test_hsm_system_integration_e2e() -> beardog::BearDogResult<()> {
    hsm::integration_tests::test_hsm_system_integration_e2e()
}
