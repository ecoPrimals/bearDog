// SPDX-License-Identifier: AGPL-3.0-only

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

impl MaintenanceConfig {
    /// Default maintenance window duration in seconds
    pub const DEFAULT_WINDOW_DURATION_SECS: u64 = 3600;

    /// Create MaintenanceConfig with hardcoded defaults
    ///
    /// This method is deterministic and safe for concurrent use.
    /// No environment variables are read.
    pub const fn with_defaults() -> Self {
        Self {
            enabled: false,
            window_duration: Duration::from_secs(Self::DEFAULT_WINDOW_DURATION_SECS),
        }
    }

    /// Create MaintenanceConfig from environment variables
    ///
    /// Reads configuration from environment, falling back to defaults.
    ///
    /// # Environment Variables
    /// - `BEARDOG_MAINTENANCE_WINDOW_SECS`: Window duration (default: 3600)
    pub fn from_env() -> Self {
        Self::from_env_provider(|k| std::env::var(k).ok())
    }

    /// Load from a custom environment provider (e.g. tests); production uses [`Self::from_env`].
    pub fn from_env_provider(get: impl Fn(&str) -> Option<String>) -> Self {
        Self {
            enabled: false,
            window_duration: Duration::from_secs(
                get("BEARDOG_MAINTENANCE_WINDOW_SECS")
                    .and_then(|s| s.parse().ok())
                    .unwrap_or(Self::DEFAULT_WINDOW_DURATION_SECS),
            ),
        }
    }
}

impl BackupConfig {
    /// Default backup interval in seconds (24 hours)
    pub const DEFAULT_INTERVAL_SECS: u64 = 86400;

    /// Create BackupConfig with hardcoded defaults
    ///
    /// This method is deterministic and safe for concurrent use.
    /// No environment variables are read.
    pub const fn with_defaults() -> Self {
        Self {
            enabled: true,
            interval: Duration::from_secs(Self::DEFAULT_INTERVAL_SECS),
        }
    }

    /// Create BackupConfig from environment variables
    ///
    /// Reads configuration from environment, falling back to defaults.
    ///
    /// # Environment Variables
    /// - `BEARDOG_BACKUP_INTERVAL_SECS`: Backup interval (default: 86400)
    pub fn from_env() -> Self {
        Self::from_env_provider(|k| std::env::var(k).ok())
    }

    /// Load from a custom environment provider (e.g. tests); production uses [`Self::from_env`].
    pub fn from_env_provider(get: impl Fn(&str) -> Option<String>) -> Self {
        Self {
            enabled: true,
            interval: Duration::from_secs(
                get("BEARDOG_BACKUP_INTERVAL_SECS")
                    .and_then(|s| s.parse().ok())
                    .unwrap_or(Self::DEFAULT_INTERVAL_SECS),
            ),
        }
    }
}

impl DisasterRecoveryConfig {
    /// Default Recovery Time Objective in seconds (1 hour)
    pub const DEFAULT_RTO_SECS: u64 = 3600;

    /// Create DisasterRecoveryConfig with hardcoded defaults
    ///
    /// This method is deterministic and safe for concurrent use.
    /// No environment variables are read.
    pub const fn with_defaults() -> Self {
        Self {
            enabled: false,
            rto: Duration::from_secs(Self::DEFAULT_RTO_SECS),
        }
    }

    /// Create DisasterRecoveryConfig from environment variables
    ///
    /// Reads configuration from environment, falling back to defaults.
    ///
    /// # Environment Variables
    /// - `BEARDOG_DR_RTO_SECS`: Recovery Time Objective (default: 3600)
    pub fn from_env() -> Self {
        Self::from_env_provider(|k| std::env::var(k).ok())
    }

    /// Load from a custom environment provider (e.g. tests); production uses [`Self::from_env`].
    pub fn from_env_provider(get: impl Fn(&str) -> Option<String>) -> Self {
        Self {
            enabled: false,
            rto: Duration::from_secs(
                get("BEARDOG_DR_RTO_SECS")
                    .and_then(|s| s.parse().ok())
                    .unwrap_or(Self::DEFAULT_RTO_SECS),
            ),
        }
    }
}

impl Default for MaintenanceConfig {
    fn default() -> Self {
        Self::with_defaults()
    }
}

impl Default for BackupConfig {
    fn default() -> Self {
        Self::with_defaults()
    }
}

impl Default for DisasterRecoveryConfig {
    fn default() -> Self {
        Self::with_defaults()
    }
}

impl OperationalConfig {
    /// Validate
    /// Validates input
    /// Validates input
    pub const fn validate(&self) -> Result<(), BearDogError> {
        Ok(())
    }
}
