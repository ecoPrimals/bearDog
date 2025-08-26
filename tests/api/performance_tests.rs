

use beardog::BearDogResult;
use tracing::info;

#[tokio::test]
async fn test_api_response_times() -> BearDogResult<()> {
    info!("Testing API response times");

    Ok(())
}

#[tokio::test]
async fn test_api_throughput() -> BearDogResult<()> {
    info!("Testing API throughput");

    Ok(())
} 