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
        .map_err(|e| {
    tracing::error!("Operation failed ({}): {:?}", "Backup procedures should work", e);
    beardog_errors::BearDogError::internal(format!("Operation failed ({}): {:?}", "Backup procedures should work", e))
})?;

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
        .map_err(|e| {
    tracing::error!("Operation failed ({}): {:?}", "Maintenance procedures should work", e);
    beardog_errors::BearDogError::internal(format!("Operation failed ({}): {:?}", "Maintenance procedures should work", e))
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
    beardog_errors::BearDogError::internal(format!("Operation failed ({}): {:?}", "Core initialization failed", e))
})?,
    );

    let mut production_manager = ProductionManager::new(core.clone())
        .await
        .map_err(|e| {
    tracing::error!("Operation failed ({}): {:?}", "Production manager creation failed", e);
    beardog_errors::BearDogError::internal(format!("Operation failed ({}): {:?}", "Production manager creation failed", e))
})?;
        
    test_operational_procedures(&mut production_manager).await;
} 