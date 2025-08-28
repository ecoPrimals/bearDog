use beardog_errors::BearDogError;


use beardog::production::*;

pub async fn test_operational_procedures(prod_manager: &mut ProductionManager) {
    println!("🔧 Testing operational procedures...");

    let backup_test = prod_manager
        .test_backup_procedures()
        .await
        .map_err(|e| {
    tracing::error!("Operation failed ({}): {:?}", "Backup procedures should work", e);
    beardog_errors::BearDogError::internal(format_args!("Operation failed ({}): {:?}", "Backup procedures should work", e).to_string())
})?;

    assert!(
        backup_test.backup_creation_successful,
        "Backup creation should succeed"
    );
    assert!(
        backup_test.backup_integrity_verified,
        "Backup integrity should be verified"
    );

    let maintenance_test = prod_manager
        .test_maintenance_procedures()
        .await
        .map_err(|e| {
    tracing::error!("Operation failed ({}): {:?}", "Maintenance procedures should work", e);
    beardog_errors::BearDogError::internal(format_args!("Operation failed ({}): {:?}", "Maintenance procedures should work", e).to_string())
})?;

    assert!(
        maintenance_test.graceful_shutdown_functional,
        "Graceful shutdown should work"
    );
    assert!(
        maintenance_test.startup_procedures_functional,
        "Startup procedures should work"
    );

    println!("✅ Operational procedures tests completed");
}

#[tokio::test]
async fn test_operational_procedures_standalone() {
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
        
    test_operational_procedures(&mut production_manager).await;
} 