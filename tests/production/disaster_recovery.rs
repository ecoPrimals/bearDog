

use beardog::production::*;

pub async fn test_disaster_recovery(prod_manager: &mut ProductionManager) {
    println!("🚑 Testing disaster recovery procedures...");

    let failover_test = prod_manager
        .test_failover_procedures()
        .await
        .map_err(|e| {
    tracing::error!("Operation failed ({}): {:?}", "Failover procedures should work", e);
    beardog_errors::BearDogError::internal(format_args!("Operation failed ({}): {:?}", "Failover procedures should work", e).to_string())
})?;

    assert!(
        failover_test.failover_time_acceptable,
        "Failover time should be acceptable"
    );
    assert!(
        failover_test.data_consistency_maintained,
        "Data consistency should be maintained"
    );

    let recovery_test = prod_manager
        .test_recovery_procedures()
        .await
        .map_err(|e| {
    tracing::error!("Operation failed ({}): {:?}", "Recovery procedures should work", e);
    beardog_errors::BearDogError::internal(format_args!("Operation failed ({}): {:?}", "Recovery procedures should work", e).to_string())
})?;

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
            .map_err(|e| {
    tracing::error!("Operation failed ({}): {:?}", "Core initialization failed", e);
    beardog_errors::BearDogError::internal(format_args!("Operation failed ({}): {:?}", "Core initialization failed", e).to_string())
})?,
    );

    let mut production_manager = ProductionManager::new(core.clone())
        .await
        .map_err(|e| {
    tracing::error!("Operation failed ({}): {:?}", "Production manager creation failed", e);
    beardog_errors::BearDogError::internal(format_args!("Operation failed ({}): {:?}", "Production manager creation failed", e).to_string())
})?;
        
    test_disaster_recovery(&mut production_manager).await;
} 