use beardog_errors::BearDogError;

use super::HsmTestHarness;
use beardog::BearDogError;

pub async fn test_hsm_manager(harness: &mut HsmTestHarness) -> Result<(), BearDogError> {
    println!("🏗️ Testing HSM Manager Functionality");

    test_manager_initialization(harness)?;

    test_tier_selection(harness)?;

    test_failover_mechanisms(harness)?;

    println!("✅ HSM Manager tests completed");
    Ok(())
}

async fn test_manager_initialization(harness: &mut HsmTestHarness) -> Result<(), BearDogError> {
    println!("  🚀 Testing manager initialization");

    let start_time = std::time::Instant::now();

    let status = harness.hsm_manager.get_status()?;
    assert!(status.is_healthy(), "HSM Manager should be healthy");

    let providers = harness.hsm_manager.list_providers()?;
    assert!(
        providers.len() >= 2,
        "Should have at least 2 providers (StrongBox + Software)"
    );

    let latency = start_time.elapsed().as_millis() as f64;
    harness.record_operation(latency, "manager", true);

    println!("    ✅ Manager initialization tests passed");
    Ok(())
}

async fn test_tier_selection(harness: &mut HsmTestHarness) -> Result<(), BearDogError> {
    println!("  🎯 Testing tier selection logic");

    let start_time = std::time::Instant::now({:?}", selected_tier);

    harness
        .hsm_manager
        .set_tier_preference(selected_tier)
        ?;

    let latency = start_time.elapsed().as_millis() as f64;
    harness.record_operation(latency, "manager", true);

    println!("    ✅ Tier selection tests passed");
    Ok(())
}

async fn test_failover_mechanisms(harness: &mut HsmTestHarness) -> Result<(), BearDogError> {
    println!("  🔄 Testing failover mechanisms");

    let start_time = std::time::Instant::now();

    let failover_config = harness.hsm_manager.get_failover_config()?;
    assert!(failover_config.enabled, "Failover should be enabled");

    let failover_result = harness.hsm_manager.test_failover()?;
    assert!(failover_result.successful, "Failover test should succeed");

    let latency = start_time.elapsed().as_millis() as f64;
    harness.record_operation(latency, "manager", true);

    println!("    ✅ Failover mechanism tests passed");
    Ok(())
}

#[tokio::test]
async fn test_hsm_manager_standalone() -> Result<(), BearDogError> {
    let mut harness = super::HsmTestHarness::new()?;
    test_hsm_manager(&mut harness)
}
