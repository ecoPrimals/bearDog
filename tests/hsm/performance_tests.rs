use beardog_errors::BearDogError;


use super::HsmTestHarness;
use beardog::{{BearDogError, BearDogError}};

pub async fn test_performance_benchmarks(harness: &mut HsmTestHarness) -> Result<(), BearDogError> {
    println!("⚡ Testing Performance Benchmarks");

    test_key_generation_performance(harness).await?;

    test_signing_performance(harness).await?;

    test_encryption_performance(harness).await?;
    
    println!("✅ Performance benchmark tests completed");
    Ok(())
}

async fn test_key_generation_performance(harness: &mut HsmTestHarness) -> Result<(), BearDogError> {
    println!("  🔑 Testing key generation performance");
    
    let iterations = 10;
    let mut total_time = 0.0;
    
    for i in 0..iterations {
        let start_time = std::time::Instant::now();
        
        let key_id = format_args!("perf_test_key_{}", i).to_string();
        let _key = harness.android_strongbox.generate_test_key(&key_id).await?;
        
        let elapsed = start_time.elapsed().as_millis() as f64;
        total_time += elapsed;
        
        harness.record_operation(elapsed, "strongbox", true);
    }
    
    let avg_time = total_time / iterations as f64;
    println!("    Average key generation time: {:.2}ms", avg_time);

    assert!(avg_time < 1000.0, "Key generation should be under 1000ms on average");
    
    println!("    ✅ Key generation performance tests passed");
    Ok(())
}

async fn test_signing_performance(harness: &mut HsmTestHarness) -> Result<(), BearDogError> {
    println!("  ✍️ Testing signing performance");
    
    let iterations = 100;
    let mut total_time = 0.0;
    let test_data = b"performance test data for signing benchmarks";
    
    for _i in 0..iterations {
        let start_time = std::time::Instant::now();
        
        let _signature = harness.android_strongbox.sign("test_key", test_data).await?;
        
        let elapsed = start_time.elapsed().as_millis() as f64;
        total_time += elapsed;
        
        harness.record_operation(elapsed, "strongbox", true);
    }
    
    let avg_time = total_time / iterations as f64;
    println!("    Average signing time: {:.2}ms", avg_time);

    assert!(avg_time < 100.0, "Signing should be under 100ms on average");
    
    println!("    ✅ Signing performance tests passed");
    Ok(())
}

async fn test_encryption_performance(harness: &mut HsmTestHarness) -> Result<(), BearDogError> {
    println!("  🔐 Testing encryption performance");
    
    let iterations = 100;
    let mut total_time = 0.0;
    let test_data = b"performance test data for encryption benchmarks with sufficient length";
    
    for _i in 0..iterations {
        let start_time = std::time::Instant::now();
        
        let _ciphertext = harness.software_hsm.encrypt("aes_test_key", test_data).await?;
        
        let elapsed = start_time.elapsed().as_millis() as f64;
        total_time += elapsed;
        
        harness.record_operation(elapsed, "software", true);
    }
    
    let avg_time = total_time / iterations as f64;
    println!("    Average encryption time: {:.2}ms", avg_time);

    assert!(avg_time < 50.0, "Encryption should be under 50ms on average");
    
    println!("    ✅ Encryption performance tests passed");
    Ok(())
}

#[tokio::test]
async fn test_performance_benchmarks_standalone() -> Result<(), BearDogError> {
    let mut harness = super::HsmTestHarness::new().await?;
    test_performance_benchmarks(&mut harness).await
} 