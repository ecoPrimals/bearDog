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
        .map_err(|e| {
    tracing::error!("Operation failed ({}): {:?}", "Failover procedures should work", e);
    beardog_errors::BearDogError::internal(format!("Operation failed ({}): {:?}", "Failover procedures should work", e))
})?;

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
        .map_err(|e| {
    tracing::error!("Operation failed ({}): {:?}", "Recovery procedures should work", e);
    beardog_errors::BearDogError::internal(format!("Operation failed ({}): {:?}", "Recovery procedures should work", e))
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
    beardog_errors::BearDogError::internal(format!("Operation failed ({}): {:?}", "Core initialization failed", e))
})?,
    );

    let mut production_manager = ProductionManager::new(core.clone())
        .await
        .map_err(|e| {
    tracing::error!("Operation failed ({}): {:?}", "Production manager creation failed", e);
    beardog_errors::BearDogError::internal(format!("Operation failed ({}): {:?}", "Production manager creation failed", e))
})?;
        
    test_disaster_recovery(&mut production_manager).await;
} 