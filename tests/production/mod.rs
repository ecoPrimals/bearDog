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


//! Production Environment Test Suite
//!
//! Comprehensive production testing broken down into focused modules
//! for better maintainability and organization.

use beardog::core::*;
use beardog::production::*;
use std::sync::Arc;

pub mod deployment_validation;
pub mod health_monitoring;
pub mod performance_monitoring;
pub mod security_hardening;
pub mod operational_procedures;
pub mod disaster_recovery;
pub mod stress_testing;

/// Main production environment test orchestration
/// Tests all production deployment and operational scenarios
#[tokio::test]
async fn test_production_environment_comprehensive() {
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

    // Run all production test suites
    deployment_validation::test_deployment_validation(&mut production_manager).await;
    health_monitoring::test_health_monitoring(&mut production_manager).await;
    performance_monitoring::test_performance_monitoring(&mut production_manager).await;
    security_hardening::test_security_hardening(&mut production_manager).await;
    operational_procedures::test_operational_procedures(&mut production_manager).await;
    disaster_recovery::test_disaster_recovery(&mut production_manager).await;
} 