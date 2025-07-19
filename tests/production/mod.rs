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
            .expect("Core initialization failed"),
    );

    let mut production_manager = ProductionManager::new(core.clone())
        .await
        .expect("Production manager creation failed");

    // Run all production test suites
    deployment_validation::test_deployment_validation(&mut production_manager).await;
    health_monitoring::test_health_monitoring(&mut production_manager).await;
    performance_monitoring::test_performance_monitoring(&mut production_manager).await;
    security_hardening::test_security_hardening(&mut production_manager).await;
    operational_procedures::test_operational_procedures(&mut production_manager).await;
    disaster_recovery::test_disaster_recovery(&mut production_manager).await;
} 