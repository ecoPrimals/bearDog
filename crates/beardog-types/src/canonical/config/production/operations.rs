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
    pub health_checks: super::super::domains::network::monitoring::HealthCheckConfiguration,

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
///
/// **DEPRECATED**: Use `super::super::domains::network::monitoring::HealthCheckConfiguration` instead.
#[deprecated(
    since = "3.1.0",
    note = "Use canonical::config::domains::network::monitoring::HealthCheckConfiguration instead"
)]
pub type HealthCheckConfig = super::super::domains::network::monitoring::HealthCheckConfiguration;

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

// Default impl removed - HealthCheckConfig now uses canonical HealthCheckConfiguration::default()

impl Default for MaintenanceConfig {
    fn default() -> Self {
        Self {
            enabled: false,
            window_duration: Duration::from_secs(
                std::env::var("BEARDOG_MAINTENANCE_WINDOW_SECS")
                    .ok()
                    .and_then(|s| s.parse().ok())
                    .unwrap_or(3600),
            ),
        }
    }
}

impl Default for BackupConfig {
    fn default() -> Self {
        Self {
            enabled: true,
            interval: Duration::from_secs(
                std::env::var("BEARDOG_BACKUP_INTERVAL_SECS")
                    .ok()
                    .and_then(|s| s.parse().ok())
                    .unwrap_or(86400),
            ),
        }
    }
}

impl Default for DisasterRecoveryConfig {
    fn default() -> Self {
        Self {
            enabled: false,
            rto: Duration::from_secs(
                std::env::var("BEARDOG_DR_RTO_SECS")
                    .ok()
                    .and_then(|s| s.parse().ok())
                    .unwrap_or(3600),
            ),
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
