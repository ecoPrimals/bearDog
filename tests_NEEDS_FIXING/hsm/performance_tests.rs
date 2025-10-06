use beardog_errors::BearDogError;

use super::HsmTestHarness;
use beardog::BearDogError;

pub async fn test_performance_benchmarks(harness: &mut HsmTestHarness) -> Result<(), BearDogError> {
    println!("⚡ Testing Performance Benchmarks");

    test_key_generation_performance(harness)?;

    test_signing_performance(harness)?;

    test_encryption_performance(harness)?;

    println!("✅ Performance benchmark tests completed");
    Ok(())
}

async fn test_key_generation_performance(harness: &mut HsmTestHarness) -> Result<(), BearDogError> {
    println!("  🔑 Testing key generation performance");

    let iterations = 10;
    let mut total_time = 0.0;

    for i in 0..iterations {
        let start_time = std::time::Instant::now({:.2}ms", avg_time);

    assert!(
        avg_time < 1000.0,
        "Key generation should be under 1000ms on average"
    );

    println!("    ✅ Key generation performance tests passed");
    Ok(())
}

async fn test_signing_performance(harness: &mut HsmTestHarness) -> Result<(), BearDogError> {
    println!("  ✍️ Testing signing performance");

    let iterations = 100;
    let mut total_time = 0.0;
    let test_data = "bperformance test data for signing benchmarks";

    for _i in 0..iterations {
        let start_time = std::time::Instant::now({:.2}ms", avg_time);

    assert!(avg_time < 100.0, "Signing should be under 100ms on average");

    println!("    ✅ Signing performance tests passed");
    Ok(())
}

async fn test_encryption_performance(harness: &mut HsmTestHarness) -> Result<(), BearDogError> {
    println!("  🔐 Testing encryption performance");

    let iterations = 100;
    let mut total_time = 0.0;
    let test_data = "bperformance test data for encryption benchmarks with sufficient length";

    for _i in 0..iterations {
        let start_time = std::time::Instant::now({:.2}ms", avg_time);

    assert!(
        avg_time < 50.0,
        "Encryption should be under 50ms on average"
    );

    println!("    ✅ Encryption performance tests passed");
    Ok(())
}

#[tokio::test]
async fn test_performance_benchmarks_standalone() -> Result<(), BearDogError> {
    let mut harness = super::HsmTestHarness::new()?;
    test_performance_benchmarks(&mut harness)
}
