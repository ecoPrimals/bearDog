use beardog_errors::BearDogError;

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

#[tokio::test]
async fn test_production_environment_comprehensive() {
    let config = BearDogConfig::production();
    let core = Arc::new(
        BearDogCore::new(config)
            .map_err(|e| {
    tracing::error!("Operation failed ({}): {:?}", "Core initialization failed", e);
    beardog_errors::BearDogError::internal({:?}", "Core initialization failed", e))
})?,
    );

    let mut production_manager = ProductionManager::new(core.clone())
        .map_err(|e| {
    tracing::error!("Operation failed ({}): {:?}", "Production manager creation failed", e);
    beardog_errors::BearDogError::internal({:?}", "Production manager creation failed", e))
})?;

    deployment_validation::test_deployment_validation(&mut production_manager);
    health_monitoring::test_health_monitoring(&mut production_manager);
    performance_monitoring::test_performance_monitoring(&mut production_manager);
    security_hardening::test_security_hardening(&mut production_manager);
    operational_procedures::test_operational_procedures(&mut production_manager);
    disaster_recovery::test_disaster_recovery(&mut production_manager);
} 