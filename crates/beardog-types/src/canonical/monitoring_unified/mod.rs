// Unified Monitoring Configuration
//
// Consolidated monitoring configuration for the BearDog ecosystem.

use serde::{Deserialize, Serialize};

/// Alerting module
pub mod alerting;
/// Core module
/// Core functionality
/// Core functionality
pub mod core;
/// Health module
pub mod health;
/// Metrics module
pub mod metrics;

pub use core::*;

/// Canonical monitoring configuration
#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct CanonicalMonitoringConfig {
    /// Core monitoring settings
    /// The core value
    pub core: MonitoringCoreConfig,

    /// Metrics configuration
    /// The metrics value
    pub metrics: MetricsConfig,

    /// Alerting configuration
    /// The alerting value
    pub alerting: AlertingConfig,

    /// Health check configuration
    /// The health value
    pub health: HealthCheckConfig,
}

pub type MonitoringConfig = CanonicalMonitoringConfig;
