//! Genetic Integration Tests
//!
//! Tests for HSM integration with genetic spawning

use super::HsmTestHarness;
use beardog::{BearDogError, BearDogResult};

/// Test genetic spawning integration
pub async fn test_genetic_spawning_integration(harness: &mut HsmTestHarness) -> BearDogResult<()> {
    println!("🧬 Testing Genetic Spawning Integration");

    // Test HSM-backed genetic spawning
    test_hsm_genetic_spawning(harness).await?;
    
    // Test entropy hierarchy with HSM
    test_entropy_hierarchy_integration(harness).await?;
    
    // Test genetic key derivation
    test_genetic_key_derivation(harness).await?;
    
    harness.test_metrics.genetic_integrations += 1;
    println!("✅ Genetic spawning integration tests completed");
    Ok(())
}

async fn test_hsm_genetic_spawning(harness: &mut HsmTestHarness) -> BearDogResult<()> {
    println!("  🧪 Testing HSM-backed genetic spawning");
    
    let start_time = std::time::Instant::now();
    
    // Test genetic spawn with HSM backing
    let spawn_request = harness.genetics_api.create_spawn_request().await?;
    let spawn_result = harness.genetics_api.execute_spawn(spawn_request).await?;
    
    assert!(spawn_result.success, "Genetic spawn should succeed");
    assert!(spawn_result.hsm_backed, "Spawn should be HSM-backed");
    
    let latency = start_time.elapsed().as_millis() as f64;
    harness.record_operation(latency, "genetic", true);
    
    println!("    ✅ HSM genetic spawning tests passed");
    Ok(())
}

async fn test_entropy_hierarchy_integration(harness: &mut HsmTestHarness) -> BearDogResult<()> {
    println!("  🌊 Testing entropy hierarchy integration");
    
    let start_time = std::time::Instant::now();
    
    // Test entropy generation with HSM
    let entropy = harness.genetics_api.generate_entropy_with_hsm().await?;
    assert!(entropy.len() >= 32, "Should generate sufficient entropy");
    assert!(entropy.hsm_source, "Entropy should be HSM-sourced");
    
    let latency = start_time.elapsed().as_millis() as f64;
    harness.record_operation(latency, "genetic", true);
    
    println!("    ✅ Entropy hierarchy integration tests passed");
    Ok(())
}

async fn test_genetic_key_derivation(harness: &mut HsmTestHarness) -> BearDogResult<()> {
    println!("  🔑 Testing genetic key derivation");
    
    let start_time = std::time::Instant::now();
    
    // Test key derivation from genetic material
    let derived_key = harness.genetics_api.derive_key_from_genetics().await?;
    assert!(derived_key.is_secure(), "Derived key should be secure");
    assert!(derived_key.hsm_protected, "Key should be HSM-protected");
    
    let latency = start_time.elapsed().as_millis() as f64;
    harness.record_operation(latency, "genetic", true);
    
    println!("    ✅ Genetic key derivation tests passed");
    Ok(())
}

#[tokio::test]
async fn test_genetic_integration_standalone() -> BearDogResult<()> {
    let mut harness = super::HsmTestHarness::new().await?;
    test_genetic_spawning_integration(&mut harness).await
} 