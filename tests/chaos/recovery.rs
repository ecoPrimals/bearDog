// Chaos Testing Recovery Validation
// Migrated October 7, 2025 - Updated for modular architecture

use super::models::*;
use beardog_errors::BearDogError;
use std::time::Instant;
use tracing::{info, warn};

/// Trait for recovery validation
pub trait RecoveryValidator: Send + Sync {
    /// Validate that a component has recovered
    fn validate_recovery(&self, component: &str) -> Result<RecoveryResult, BearDogError>;

    /// Get the component this validator checks
    fn component_name(&self) -> &str;
}

/// Wait for a component to recover after fault injection
pub async fn wait_for_recovery(
    component: &str,
    timeout_ms: u64,
) -> Result<u64, BearDogError> {
    info!("Waiting for component '{}' to recover (timeout: {}ms)", component, timeout_ms);
    
    let start = Instant::now();
    let timeout = std::time::Duration::from_millis(timeout_ms);

    while start.elapsed() < timeout {
        // Check if component is healthy (simulated for now)
        if check_component_health(component).await? {
            let recovery_time_ms = start.elapsed().as_millis() as u64;
            info!("Component '{}' recovered in {}ms", component, recovery_time_ms);
            return Ok(recovery_time_ms);
        }

        // No sleep needed - checking health status should be instant in tests
        // For real health polling, use tokio::sync::watch or tokio::sync::Notify
        tokio::task::yield_now().await; // Prevent busy-waiting
    }

    warn!("Component '{}' failed to recover within timeout", component);
    Err(BearDogError::internal(format!(
        "Component '{}' did not recover within {}ms",
        component, timeout_ms
    )))
}

/// Check if a component is healthy
async fn check_component_health(component: &str) -> Result<bool, BearDogError> {
    // In a real implementation, this would check actual component health
    // For testing, we'll simulate recovery after a delay
    info!("Checking health of component '{}'", component);
    
    // Simulate recovery (always returns true for now)
    Ok(true)
}

/// Validate recovery of all components
pub async fn validate_all_recoveries(
    validators: &[std::sync::Arc<dyn RecoveryValidator>],
    components: &[String],
) -> Result<Vec<RecoveryResult>, BearDogError> {
    info!("Validating recovery of {} components", components.len());
    
    let mut results = Vec::new();

    for component in components {
        // Find appropriate validator
        let validator = validators.iter()
            .find(|v| v.component_name() == component || v.component_name() == "all");

        if let Some(validator) = validator {
            match validator.validate_recovery(component) {
                Ok(result) => results.push(result),
                Err(e) => {
                    warn!("Failed to validate recovery of '{}': {}", component, e);
                    results.push(RecoveryResult {
                        component: component.clone(),
                        recovered: false,
                        recovery_time_ms: 0,
                        health_check_passed: false,
                        data_integrity_verified: false,
                    });
                }
            }
        } else {
            warn!("No validator found for component '{}'", component);
        }
    }

    Ok(results)
}

// ============================================================================
// Mock Recovery Validators for Testing
// ============================================================================

/// Core recovery validator
pub struct CoreRecoveryValidator {
    component_name: String,
}

impl CoreRecoveryValidator {
    pub fn new(component_name: String) -> Self {
        Self { component_name }
    }
}

impl RecoveryValidator for CoreRecoveryValidator {
    fn validate_recovery(&self, component: &str) -> Result<RecoveryResult, BearDogError> {
        info!("Validating core component recovery: {}", component);
        
        // Simulate recovery validation
        Ok(RecoveryResult {
            component: component.to_string(),
            recovered: true,
            recovery_time_ms: 1000,
            health_check_passed: true,
            data_integrity_verified: true,
        })
    }

    fn component_name(&self) -> &str {
        &self.component_name
    }
}

/// Security recovery validator
pub struct SecurityRecoveryValidator;

impl SecurityRecoveryValidator {
    pub fn new() -> Self {
        Self
    }
}

impl RecoveryValidator for SecurityRecoveryValidator {
    fn validate_recovery(&self, component: &str) -> Result<RecoveryResult, BearDogError> {
        info!("Validating security component recovery: {}", component);
        
        Ok(RecoveryResult {
            component: component.to_string(),
            recovered: true,
            recovery_time_ms: 800,
            health_check_passed: true,
            data_integrity_verified: true,
        })
    }

    fn component_name(&self) -> &str {
        "security"
    }
}

/// Network recovery validator
pub struct NetworkRecoveryValidator;

impl NetworkRecoveryValidator {
    pub fn new() -> Self {
        Self
    }
}

impl RecoveryValidator for NetworkRecoveryValidator {
    fn validate_recovery(&self, component: &str) -> Result<RecoveryResult, BearDogError> {
        info!("Validating network component recovery: {}", component);
        
        Ok(RecoveryResult {
            component: component.to_string(),
            recovered: true,
            recovery_time_ms: 1200,
            health_check_passed: true,
            data_integrity_verified: true,
        })
    }

    fn component_name(&self) -> &str {
        "network"
    }
}

/// Database recovery validator
pub struct DatabaseRecoveryValidator;

impl DatabaseRecoveryValidator {
    pub fn new() -> Self {
        Self
    }
}

impl RecoveryValidator for DatabaseRecoveryValidator {
    fn validate_recovery(&self, component: &str) -> Result<RecoveryResult, BearDogError> {
        info!("Validating database component recovery: {}", component);
        
        Ok(RecoveryResult {
            component: component.to_string(),
            recovered: true,
            recovery_time_ms: 1500,
            health_check_passed: true,
            data_integrity_verified: true,
        })
    }

    fn component_name(&self) -> &str {
        "database"
    }
}

