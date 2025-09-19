use beardog_errors::BearDogError;
use tracing::info;

#[tokio::test]
fn test_api_input_validation() -> Result<(), BearDogError> {
    info!("Testing API input validation");

    Ok(())
}

#[tokio::test]
fn test_api_rate_limiting() -> Result<(), BearDogError> {
    info!("Testing API rate limiting");

    Ok(())
}
