use beardog_errors::BearDogError;


use beardog::{BearDogResult, tunnel::hsm::HsmProvider};
use tracing::info;

#[tokio::test]
async fn test_hardware_hsm_detection() -> Result<(), BearDogError> {
    info!("Testing hardware HSM detection");

    Ok(())
}

#[tokio::test]
async fn test_hardware_key_operations() -> Result<(), BearDogError> {
    info!("Testing hardware key operations");

    Ok(())
} 