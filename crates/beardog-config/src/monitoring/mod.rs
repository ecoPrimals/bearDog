//! Monitoring and Observability Configuration
//!
//! This module provides comprehensive monitoring and observability configuration for BearDog,
//! including metrics collection, alerting, dashboards, and observability features.

use serde::{Deserialize, Serialize};

pub mod alerting;
pub mod dashboards;
pub mod integration;
pub mod metrics;
pub mod observability;

pub use alerting::*;
pub use dashboards::*;
pub use integration::*;
pub use metrics::*;
pub use observability::*;

/// Production monitoring configuration
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ProductionMonitoringConfig {
    /// Enable monitoring
    pub enabled: bool,
    /// Metrics collection configuration
    pub metrics: MetricsConfig,
    /// Alerting configuration
    pub alerting: AlertingConfig,
    /// Dashboard configuration
    pub dashboards: DashboardConfig,
    /// Observability configuration
    pub observability: ObservabilityConfig,
    /// Integration configuration
    pub integrations: IntegrationConfig,
}

impl Default for ProductionMonitoringConfig {
    fn default() -> Self {
        Self {
            enabled: true,
            metrics: MetricsConfig::default(),
            alerting: AlertingConfig::default(),
            dashboards: DashboardConfig::default(),
            observability: ObservabilityConfig::default(),
            integrations: IntegrationConfig::default(),
        }
    }
}

impl ProductionMonitoringConfig {
    /// Create a production monitoring configuration
    pub fn production() -> Self {
        Self {
            enabled: true,
            metrics: MetricsConfig::production(),
            alerting: AlertingConfig::production(),
            dashboards: DashboardConfig::production(),
            observability: ObservabilityConfig::production(),
            integrations: IntegrationConfig::production(),
        }
    }

    /// Create a development monitoring configuration
    pub fn development() -> Self {
        Self {
            enabled: false,
            metrics: MetricsConfig::development(),
            alerting: AlertingConfig::development(),
            dashboards: DashboardConfig::development(),
            observability: ObservabilityConfig::development(),
            integrations: IntegrationConfig::development(),
        }
    }
}
