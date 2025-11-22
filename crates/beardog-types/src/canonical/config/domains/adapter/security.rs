//! Security and monitoring configuration for adapters

use super::core::AuthLevel;
use serde::{Deserialize, Serialize};
use std::time::Duration;

/// Security configuration for adapters
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
pub struct AdapterSecurityConfig {
    /// Enable security features
    pub enabled: bool,

    /// Authentication required
    pub auth_required: bool,

    /// Authorization level
    pub auth_level: AuthLevel,

    /// Enable encryption in transit
    pub encryption_in_transit: bool,

    /// Enable encryption at rest
    pub encryption_at_rest: bool,

    /// Security audit logging
    pub audit_logging: bool,
}

/// Monitoring configuration for adapters
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
pub struct AdapterMonitoringConfig {
    /// Enable monitoring
    pub enabled: bool,

    /// Metrics collection interval
    pub metrics_interval: Duration,

    /// Enable performance metrics
    pub performance_metrics: bool,

    /// Enable error metrics
    pub error_metrics: bool,

    /// Enable usage metrics
    pub usage_metrics: bool,

    /// Metrics retention period
    pub retention_period: Duration,
}

impl Default for AdapterSecurityConfig {
    fn default() -> Self {
        Self {
            enabled: true,
            auth_required: true,
            auth_level: AuthLevel::Token,
            encryption_in_transit: true,
            encryption_at_rest: false,
            audit_logging: true,
        }
    }
}

impl Default for AdapterMonitoringConfig {
    fn default() -> Self {
        Self {
            enabled: true,
            metrics_interval: Duration::from_secs(
                std::env::var("BEARDOG_ADAPTER_METRICS_INTERVAL_SECS")
                    .ok()
                    .and_then(|s| s.parse().ok())
                    .unwrap_or(60),
            ),
            performance_metrics: true,
            error_metrics: true,
            usage_metrics: true,
            retention_period: Duration::from_secs(
                std::env::var("BEARDOG_ADAPTER_RETENTION_PERIOD_SECS")
                    .ok()
                    .and_then(|s| s.parse().ok())
                    .unwrap_or(86400), // 24 hours
            ),
        }
    }
}
