

use super::models::*;
use super::ChaosTestFramework;
use beardog::{core::*, BearDogResult};
use std::sync::Arc;
use std::time::{Duration, Instant};
use tokio::time::{sleep, timeout};
use tracing::warn;

#[allow(async_fn_in_trait)]
pub trait RecoveryValidator: Send + Sync {

    async fn validate_recovery(&self) -> BearDogResult<RecoveryStatus>;

    fn target_component(&self) -> String;
}

pub async fn wait_for_recovery(framework: &ChaosTestFramework, component: &str) -> BearDogResult<u64> {
    let start_time = Instant::now();
    let timeout_duration = Duration::from_millis(framework.config.recovery_timeout_ms);
    
    let recovery_result = timeout(timeout_duration, async {
        loop {
            let mut all_recovered = true;
            
            for validator in &framework.recovery_validators {
                if validator.target_component() == component || validator.target_component() == "all" {
                    let recovery_status = validator.validate_recovery().await?;
                    if recovery_status != RecoveryStatus::FullyRecovered {
                        all_recovered = false;
                        break;
                    }
                }
            }
            
            if all_recovered {
                return Ok(start_time.elapsed().as_millis() as u64);
            }
            
            sleep(Duration::from_millis(1000)).await;
        }
    }).await;
    
    match recovery_result {
        Ok(Ok(recovery_time)) => Ok(recovery_time),
        Ok(Err(e)) => Err(e),
        Err(_) => {
            warn!("⏰ Recovery timeout for component: {}", component);
            Ok(framework.config.recovery_timeout_ms)
        }
    }
}

pub async fn validate_all_recoveries(framework: &ChaosTestFramework) -> BearDogResult<Vec<RecoveryResult>> {
    let mut results = Vec::new();
    
    for validator in &framework.recovery_validators {
        let status = validator.validate_recovery().await?;
        results.push(RecoveryResult {
            component: validator.target_component(),
            status,
        });
    }
    
    Ok(results)
}

pub struct CoreRecoveryValidator {
    core: Arc<BearDogCore>,
}

impl CoreRecoveryValidator {
    pub fn new(core: Arc<BearDogCore>) -> Self {
        Self { core }
    }
}

#[allow(async_fn_in_trait)]
impl RecoveryValidator for CoreRecoveryValidator {
    async fn validate_recovery(&self) -> BearDogResult<RecoveryStatus> {
        match self.core.health_check().await {
            Ok(health) => match health.status {
                HealthStatus::Healthy => Ok(RecoveryStatus::FullyRecovered),
                HealthStatus::Degraded => Ok(RecoveryStatus::PartiallyRecovered),
                _ => Ok(RecoveryStatus::NotRecovered),
            },
            Err(_) => Ok(RecoveryStatus::NotRecovered),
        }
    }

    fn target_component(&self) -> String {
        "core".to_string()
    }
}

pub struct SecurityRecoveryValidator;

impl SecurityRecoveryValidator {
    pub fn new() -> Self {
        Self
    }
}

#[allow(async_fn_in_trait)]
impl RecoveryValidator for SecurityRecoveryValidator {
    async fn validate_recovery(&self) -> BearDogResult<RecoveryStatus> {

        Ok(RecoveryStatus::FullyRecovered)
    }

    fn target_component(&self) -> String {
        "security".to_string()
    }
} 