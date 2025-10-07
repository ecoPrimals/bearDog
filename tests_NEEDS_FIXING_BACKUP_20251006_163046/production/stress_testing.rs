use beardog_errors::BearDogError;

use beardog::core::*;
use beardog::production::*;
use std::sync::Arc;

#[tokio::test]
async fn test_production_stress_conditions() {
    println!("💪 Testing production environment under stress...");

    let config = BearDogConfig::production();
    let core = Arc::new(
        BearDogCore::new(config)
            .map_err(|e| {
    tracing::error!("Operation failed ({}): {:?}", "Core initialization failed", e);
    beardog_errors::BearDogError::internal(format!("Error: {:?}", "Core initialization failed", e))
})?,
    );

    let mut production_manager = ProductionManager::new(core.clone())
        .map_err(|e| {
    tracing::error!("Operation failed ({}): {:?}", "Production manager creation failed", e);
    beardog_errors::BearDogError::internal(format!("Error: {:?}", "Production manager creation failed", e))
})?;

    let stress_test_config = StressTestConfiguration {
        cpu_stress_percentage: 80,
        memory_stress_percentage: 85,
        network_stress_mbps: 900,
        concurrent_operations: 1000,
        stress_duration_seconds: 60,
    };

    let stress_test_results = production_manager
        .run_stress_test(&stress_test_config)
        .map_err(|e| {
    tracing::error!("Operation failed ({}): {:?}", "Stress test should succeed", e);
    beardog_errors::BearDogError::internal(format!("Error: {:?}", "Stress test should succeed", e))
})?;

    assert!(
        stress_test_results.system_remained_stable,
        "System should remain stable under stress"
    );
    assert!(
        stress_test_results.performance_degradation_acceptable,
        "Performance degradation should be acceptable"
    );
    assert!(
        stress_test_results.error_rate_within_limits,
        "Error rate should be within limits"
    );
    assert!(
        stress_test_results.recovery_time_acceptable,
        "Recovery time should be acceptable"
    );

    let resource_exhaustion_test = production_manager
        .test_resource_exhaustion_handling()
        .map_err(|e| {
    tracing::error!("Operation failed ({}): {:?}", "Resource exhaustion test should succeed", e);
    beardog_errors::BearDogError::internal(format!("Error: {:?}", "Resource exhaustion test should succeed", e))
})?;

    assert!(
        resource_exhaustion_test.graceful_degradation_functional,
        "Should degrade gracefully"
    );
    assert!(
        resource_exhaustion_test.critical_operations_preserved,
        "Critical operations should be preserved"
    );
    assert!(
        resource_exhaustion_test.recovery_procedures_effective,
        "Recovery should be effective"
    );

    let cascade_prevention_test = production_manager
        .test_cascade_failure_prevention()
        .map_err(|e| {
    tracing::error!("Operation failed ({}): {:?}", "Cascade failure test should succeed", e);
    beardog_errors::BearDogError::internal(format!("Error: {:?}", "Cascade failure test should succeed", e))
})?;

    assert!(
        cascade_prevention_test.circuit_breakers_functional,
        "Circuit breakers should work"
    );
    assert!(
        cascade_prevention_test.isolation_mechanisms_effective,
        "Isolation should be effective"
    );
    assert!(
        cascade_prevention_test.system_resilience_maintained,
        "Resilience should be maintained"
    );
}

pub fn test_stress_testing_comprehensive(prod_manager: &mut ProductionManager) {

    println!("🚀 Running comprehensive stress tests...");

} 