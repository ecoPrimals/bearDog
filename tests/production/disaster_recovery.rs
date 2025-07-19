//! Production Disaster Recovery Tests
//!
//! Tests for disaster recovery procedures, failover mechanisms,
//! data recovery, and business continuity validation.

use beardog::production::*;

/// Test production disaster recovery procedures
pub async fn test_disaster_recovery(prod_manager: &mut ProductionManager) {
    println!("🚑 Testing disaster recovery procedures...");

    // Test failover procedures
    let failover_test = prod_manager
        .test_failover_procedures()
        .await
        .expect("Failover procedures should work");

    assert!(
        failover_test.failover_time_acceptable,
        "Failover time should be acceptable"
    );
    assert!(
        failover_test.data_consistency_maintained,
        "Data consistency should be maintained"
    );

    // Test recovery procedures
    let recovery_test = prod_manager
        .test_recovery_procedures()
        .await
        .expect("Recovery procedures should work");

    assert!(
        recovery_test.recovery_time_objective_met,
        "Recovery time objective should be met"
    );
    assert!(
        recovery_test.recovery_point_objective_met,
        "Recovery point objective should be met"
    );

    println!("✅ Disaster recovery tests completed");
}

#[tokio::test]
async fn test_disaster_recovery_standalone() {
    use beardog::core::*;
    use std::sync::Arc;
    
    let config = BearDogConfig::production();
    let core = Arc::new(
        BearDogCore::new(config)
            .await
            .expect("Core initialization failed"),
    );

    let mut production_manager = ProductionManager::new(core.clone())
        .await
        .expect("Production manager creation failed");
        
    test_disaster_recovery(&mut production_manager).await;
} 