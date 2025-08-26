

use super::HsmTestHarness;
use beardog::{BearDogError, BearDogResult};

#[tokio::test]
async fn test_hsm_error_handling() -> BearDogResult<()> {
    println!("🚨 Testing HSM Error Handling and Resilience");
    
    let mut harness = HsmTestHarness::new().await?;

    test_invalid_key_operations(&mut harness).await?;
    test_provider_unavailability(&mut harness).await?;
    test_resource_exhaustion(&mut harness).await?;
    
    println!("✅ HSM Error Handling tests PASSED");
    Ok(())
}

async fn test_invalid_key_operations(harness: &mut HsmTestHarness) -> BearDogResult<()> {
    println!("  ❌ Testing invalid key operations");

    let result = harness.android_strongbox.sign("non_existent_key", b"test").await;
    assert!(result.is_err(), "Operation with non-existent key should fail");

    let result = harness.software_hsm.generate_invalid_key().await;
    assert!(result.is_err(), "Invalid key generation should fail");
    
    println!("    ✅ Invalid key operation tests passed");
    Ok(())
}

async fn test_provider_unavailability(harness: &mut HsmTestHarness) -> BearDogResult<()> {
    println!("  🔌 Testing provider unavailability");

    let result = harness.hsm_manager.handle_provider_failure().await?;
    assert!(result.graceful_degradation, "Should handle provider failure gracefully");
    
    println!("    ✅ Provider unavailability tests passed");
    Ok(())
}

async fn test_resource_exhaustion(harness: &mut HsmTestHarness) -> BearDogResult<()> {
    println!("  💾 Testing resource exhaustion scenarios");

    let result = harness.hsm_manager.test_resource_limits().await?;
    assert!(result.handles_exhaustion, "Should handle resource exhaustion");
    
    println!("    ✅ Resource exhaustion tests passed");
    Ok(())
}

#[tokio::test]
async fn test_hsm_system_integration_e2e() -> BearDogResult<()> {
    println!("🔄 Testing HSM System Integration End-to-End");
    
    let mut harness = HsmTestHarness::new().await?;

    test_complete_workflow(&mut harness).await?;
    test_cross_provider_operations(&mut harness).await?;
    test_system_recovery(&mut harness).await?;
    
    harness.generate_test_report();
    println!("✅ HSM System Integration E2E tests PASSED");
    Ok(())
}

async fn test_complete_workflow(harness: &mut HsmTestHarness) -> BearDogResult<()> {
    println!("  🔄 Testing complete HSM workflow");

    let system_status = harness.hsm_manager.get_system_status().await?;
    assert!(system_status.operational, "System should be operational");

    let strongbox_key = harness.android_strongbox.generate_test_key("e2e_strongbox").await?;
    let software_key = harness.software_hsm.generate_test_key("e2e_software").await?;

    let test_data = b"end-to-end test data";
    let strongbox_sig = harness.android_strongbox.sign(&strongbox_key.key_id, test_data).await?;
    let software_sig = harness.software_hsm.sign(&software_key.key_id, test_data).await?;

    let strongbox_valid = harness.android_strongbox.verify(&strongbox_key.key_id, test_data, &strongbox_sig).await?;
    let software_valid = harness.software_hsm.verify(&software_key.key_id, test_data, &software_sig).await?;
    
    assert!(strongbox_valid && software_valid, "All signatures should be valid");
    
    println!("    ✅ Complete workflow tests passed");
    Ok(())
}

async fn test_cross_provider_operations(harness: &mut HsmTestHarness) -> BearDogResult<()> {
    println!("  🔀 Testing cross-provider operations");

    let result = harness.hsm_manager.execute_cross_provider_operation().await?;
    assert!(result.successful, "Cross-provider operation should succeed");
    
    println!("    ✅ Cross-provider operation tests passed");
    Ok(())
}

async fn test_system_recovery(harness: &mut HsmTestHarness) -> BearDogResult<()> {
    println!("  🔧 Testing system recovery");

    let recovery_result = harness.hsm_manager.test_system_recovery().await?;
    assert!(recovery_result.recovered_successfully, "System should recover");
    
    println!("    ✅ System recovery tests passed");
    Ok(())
}

pub async fn run_comprehensive_hsm_tests() -> BearDogResult<()> {
    println!("🔐 Running Comprehensive HSM Test Suite");
    println!("═══════════════════════════════════════");

    test_hsm_error_handling().await?;
    test_hsm_system_integration_e2e().await?;
    
    println!("🎉 All HSM tests completed successfully!");
    Ok(())
} 