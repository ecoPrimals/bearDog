// Operations Configuration
//
// This module contains operational configuration including health checks,
// maintenance windows, backup procedures, and disaster recovery settings.

use beardog_errors::BearDogError;
use serde::{Deserialize, Serialize};
use std::time::Duration;

/// **OPERATIONAL CONFIGURATION** - Day-to-day operations
#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct OperationalConfig {
    /// Health check configuration
    /// The health checks value
    pub health_checks: HealthCheckConfig,

    /// Maintenance windows
    /// The maintenance value
    pub maintenance: MaintenanceConfig,

    /// Backup and recovery
    /// The backup value
    pub backup: BackupConfig,

    /// Disaster recovery
    /// The disaster recovery value
    pub disaster_recovery: DisasterRecoveryConfig,

    /// Operational procedures
    /// The procedures value
    pub procedures: ProcedureConfig,
}

/// Health check configuration
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct HealthCheckConfig {
    /// Enable health checks
    /// Whether feature is enabled
    pub enabled: bool,
    /// Health check interval
    /// The interval value
    pub interval: Duration,
    /// Health check timeout
    pub timeout: Duration,
    /// Number of `failure_threshold`
    pub failure_threshold: u32,
    /// Number of `success_threshold`
    pub success_threshold: u32,
}

/// Maintenance configuration
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct MaintenanceConfig {
    /// Enable maintenance mode
    /// Whether feature is enabled
    pub enabled: bool,
    /// Maintenance window duration
    /// The window duration value
    pub window_duration: Duration,
}

/// Backup configuration
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct BackupConfig {
    /// Enable backups
    /// Whether feature is enabled
    pub enabled: bool,
    /// Backup interval
    /// The interval value
    pub interval: Duration,
}

/// Disaster recovery configuration
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct DisasterRecoveryConfig {
    /// Enable disaster recovery
    /// Whether feature is enabled
    pub enabled: bool,
    /// Recovery time objective
    /// The rto value
    pub rto: Duration,
}

/// Procedure configuration
#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct ProcedureConfig {
    /// Enable automated procedures
    /// Whether automated is enabled
    pub automated: bool,
}

impl Default for HealthCheckConfig {
    fn default() -> Self {
        Self {
            enabled: true,
            interval: Duration::from_secs(30),
            timeout: Duration::from_secs(5),
            failure_threshold: 3,
            success_threshold: 1,
        }
    }
}

impl Default for MaintenanceConfig {
    fn default() -> Self {
        Self {
            enabled: false,
            window_duration: Duration::from_secs(3600),
        }
    }
}

impl Default for BackupConfig {
    fn default() -> Self {
        Self {
            enabled: true,
            interval: Duration::from_secs(86400), // Daily
        }
    }
}

impl Default for DisasterRecoveryConfig {
    fn default() -> Self {
        Self {
            enabled: false,
            rto: Duration::from_secs(3600), // 1 hour
        }
    }
}

impl OperationalConfig {
    /// Validate
    /// Validates input
    /// Validates input
    pub fn validate(&self) -> Result<(), BearDogError> {
        Ok(())
    }
}
