//! Production Operational Procedures Tests
//!
//! Tests for operational procedures, maintenance operations,
//! backup/restore procedures, and operational safety checks.

use beardog::production::*;

/// Test production operational procedures
pub async fn test_operational_procedures(prod_manager: &mut ProductionManager) {
    println!("🔧 Testing operational procedures...");

    // Test backup procedures
    let backup_test = prod_manager
        .test_backup_procedures()
        .await
        .expect("Backup procedures should work");

    assert!(
        backup_test.backup_creation_successful,
        "Backup creation should succeed"
    );
    assert!(
        backup_test.backup_integrity_verified,
        "Backup integrity should be verified"
    );

    // Test maintenance procedures
    let maintenance_test = prod_manager
        .test_maintenance_procedures()
        .await
        .expect("Maintenance procedures should work");

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
            .expect("Core initialization failed"),
    );

    let mut production_manager = ProductionManager::new(core.clone())
        .await
        .expect("Production manager creation failed");
        
    test_operational_procedures(&mut production_manager).await;
} 