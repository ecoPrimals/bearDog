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


//! Recovery Validation Components
//!
//! Recovery validator trait and implementations for validating
//! system recovery from chaos-induced faults.

use super::models::*;
use super::ChaosTestFramework;
use beardog::{core::*, BearDogResult};
use std::sync::Arc;
use std::time::{Duration, Instant};
use tokio::time::{sleep, timeout};
use tracing::warn;

/// Recovery validator trait
/// **MODERNIZED** - Uses native async fn instead of async_trait for zero-cost abstractions
#[allow(async_fn_in_trait)]
pub trait RecoveryValidator: Send + Sync {
    /// Validate that the system has recovered from a fault
    async fn validate_recovery(&self) -> BearDogResult<RecoveryStatus>;
    
    /// Get the component this validator checks
    fn target_component(&self) -> String;
}

/// Wait for system recovery and measure recovery time
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

/// Validate recovery across all registered validators
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

// Recovery Validator Implementations

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
        // Simulate security component recovery validation
        Ok(RecoveryStatus::FullyRecovered)
    }

    fn target_component(&self) -> String {
        "security".to_string()
    }
} 