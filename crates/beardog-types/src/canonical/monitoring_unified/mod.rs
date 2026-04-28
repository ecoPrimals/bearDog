// SPDX-License-Identifier: AGPL-3.0-or-later

// Unified Monitoring Configuration
//
// ⚠️  DEPRECATED: This module is deprecated. Use `beardog_types::canonical::monitoring` instead.
//
// Consolidated monitoring configuration for the BearDog ecosystem.

#![allow(deprecated)]

use serde::{Deserialize, Serialize};

/// Alerting module
pub mod alerting;
/// Core module
/// Core functionality
pub mod core;
/// Health module
pub mod health;
/// Metrics module
pub mod metrics;

pub use core::{AlertingConfig, MetricsConfig, MonitoringCoreConfig, MonitoringHealthCheckConfig};

/// Canonical monitoring configuration
///
/// ⚠️  DEPRECATED: Use `beardog_types::canonical::monitoring::MonitoringConfig` instead
#[allow(
    deprecated,
    reason = "deprecated type definition; rustc does not emit deprecated on this line — expect would be unfulfilled"
)]
#[deprecated(
    since = "3.1.0",
    note = "Use beardog_types::canonical::monitoring::MonitoringConfig instead"
)]
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
    pub health: MonitoringHealthCheckConfig,
}

// Backward compatibility: redirect to canonical location
pub use crate::canonical::monitoring::MonitoringConfig as NewMonitoringConfig;
