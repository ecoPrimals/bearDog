

use beardog_errors::BearDogError;
use tracing::info;

#[tokio::test]
async fn test_api_response_times() -> Result<(), BearDogError> {
    info!("Testing API response times");

    Ok(())
}

#[tokio::test]
async fn test_api_throughput() -> Result<(), BearDogError> {
    info!("Testing API throughput");

    Ok(())
} 