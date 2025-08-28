use beardog_errors::BearDogError;


use super::HsmTestHarness;
use beardog::{{BearDogError, BearDogError}};

pub async fn test_security_validation(harness: &mut HsmTestHarness) -> Result<(), BearDogError> {
    println!("🔒 Testing Security Validation");

    test_signature_validation(harness).await?;

    test_key_security_properties(harness).await?;

    test_error_handling(harness).await?;
    
    println!("✅ Security validation tests completed");
    Ok(())
}

async fn test_signature_validation(harness: &mut HsmTestHarness) -> Result<(), BearDogError> {
    println!("  ✍️ Testing signature validation");
    
    let start_time = std::time::Instant::now();

    let test_data = b"test data for signature validation";
    let signature = harness.android_strongbox.sign("test_key", test_data).await?;
    let is_valid = harness.android_strongbox.verify("test_key", test_data, &signature).await?;
    assert!(is_valid, "Valid signature should verify correctly");

    let tampered_data = b"tampered data for signature validation";
    let is_invalid = harness.android_strongbox.verify("test_key", tampered_data, &signature).await?;
    assert!(!is_invalid, "Tampered data should not verify");
    
    let latency = start_time.elapsed().as_millis() as f64;
    harness.record_operation(latency, "security", true);
    
    println!("    ✅ Signature validation tests passed");
    Ok(())
}

async fn test_key_security_properties(harness: &mut HsmTestHarness) -> Result<(), BearDogError> {
    println!("  🔐 Testing key security properties");
    
    let start_time = std::time::Instant::now();

    let result = harness.android_strongbox.attempt_key_extraction("test_key").await;
    assert!(result.is_err(), "Key extraction should be prevented");

    let usage_result = harness.android_strongbox.test_key_usage_restrictions().await?;
    assert!(usage_result.restrictions_enforced, "Usage restrictions should be enforced");
    
    let latency = start_time.elapsed().as_millis() as f64;
    harness.record_operation(latency, "security", true);
    
    println!("    ✅ Key security property tests passed");
    Ok(())
}

async fn test_error_handling(harness: &mut HsmTestHarness) -> Result<(), BearDogError> {
    println!("  ⚠️ Testing error handling");
    
    let start_time = std::time::Instant::now();

    let invalid_result = harness.android_strongbox.perform_invalid_operation().await;
    match invalid_result {
        Err(BearDogError::internal("HSM operation failed")) => {
            println!("    ✓ Invalid operation correctly rejected");
        }
        _ => panic!("Invalid operation should return HSM error"),
    }
    
    let latency = start_time.elapsed().as_millis() as f64;
    harness.record_operation(latency, "security", true);
    
    println!("    ✅ Error handling tests passed");
    Ok(())
}

#[tokio::test]
async fn test_security_validation_standalone() -> Result<(), BearDogError> {
    let mut harness = super::HsmTestHarness::new().await?;
    test_security_validation(&mut harness).await
} 