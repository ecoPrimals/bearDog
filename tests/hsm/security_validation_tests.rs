// BearDog - Enterprise Security Ecosystem
// Copyright (C) 2025 EcoPrimals
//
// This program is free software: you can redistribute it and/or modify
// it under the terms of the GNU Affero General Public License as published by
// the Free Software Foundation, either version 3 of the License, or
// (at your option) any later version.
//
// This program is distributed in the hope that it will be useful,
// but WITHOUT ANY WARRANTY; without even the implied warranty of
// MERCHANTABILITY or FITNESS FOR A PARTICULAR PURPOSE. See the
// GNU Affero General Public License for more details.
//
// You should have received a copy of the GNU Affero General Public License
// along with this program. If not, see <https://www.gnu.org/licenses/>.


//! Security Validation Tests
//!
//! Tests for HSM security validation and error handling

use super::HsmTestHarness;
use beardog::{BearDogError, BearDogResult};

/// Test security validation
pub async fn test_security_validation(harness: &mut HsmTestHarness) -> BearDogResult<()> {
    println!("🔒 Testing Security Validation");

    // Test signature validation
    test_signature_validation(harness).await?;
    
    // Test key security properties
    test_key_security_properties(harness).await?;
    
    // Test error handling
    test_error_handling(harness).await?;
    
    println!("✅ Security validation tests completed");
    Ok(())
}

async fn test_signature_validation(harness: &mut HsmTestHarness) -> BearDogResult<()> {
    println!("  ✍️ Testing signature validation");
    
    let start_time = std::time::Instant::now();
    
    // Test valid signature verification
    let test_data = b"test data for signature validation";
    let signature = harness.android_strongbox.sign("test_key", test_data).await?;
    let is_valid = harness.android_strongbox.verify("test_key", test_data, &signature).await?;
    assert!(is_valid, "Valid signature should verify correctly");
    
    // Test invalid signature detection
    let tampered_data = b"tampered data for signature validation";
    let is_invalid = harness.android_strongbox.verify("test_key", tampered_data, &signature).await?;
    assert!(!is_invalid, "Tampered data should not verify");
    
    let latency = start_time.elapsed().as_millis() as f64;
    harness.record_operation(latency, "security", true);
    
    println!("    ✅ Signature validation tests passed");
    Ok(())
}

async fn test_key_security_properties(harness: &mut HsmTestHarness) -> BearDogResult<()> {
    println!("  🔐 Testing key security properties");
    
    let start_time = std::time::Instant::now();
    
    // Test key extraction protection
    let result = harness.android_strongbox.attempt_key_extraction("test_key").await;
    assert!(result.is_err(), "Key extraction should be prevented");
    
    // Test key usage restrictions
    let usage_result = harness.android_strongbox.test_key_usage_restrictions().await?;
    assert!(usage_result.restrictions_enforced, "Usage restrictions should be enforced");
    
    let latency = start_time.elapsed().as_millis() as f64;
    harness.record_operation(latency, "security", true);
    
    println!("    ✅ Key security property tests passed");
    Ok(())
}

async fn test_error_handling(harness: &mut HsmTestHarness) -> BearDogResult<()> {
    println!("  ⚠️ Testing error handling");
    
    let start_time = std::time::Instant::now();
    
    // Test graceful handling of invalid operations
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
async fn test_security_validation_standalone() -> BearDogResult<()> {
    let mut harness = super::HsmTestHarness::new().await?;
    test_security_validation(&mut harness).await
} 