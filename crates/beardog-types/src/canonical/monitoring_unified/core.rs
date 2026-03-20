// SPDX-License-Identifier: AGPL-3.0-only

// Core Monitoring Types

use serde::{Deserialize, Serialize};

/// Core monitoring system configuration
#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct MonitoringCoreConfig {
    /// Enabled
    /// Whether feature is enabled
    pub enabled: bool,
    /// Sampling Rate
    /// The sampling rate value
    pub sampling_rate: f64,
    /// Buffer Size
    /// Number of `buffer_size`
    pub buffer_size: usize,
}

/// Metrics collection toggles and retention-related defaults.
#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct MetricsConfig {
    /// Enabled
    /// Whether feature is enabled
    pub enabled: bool,
    /// Collection Interval
    /// Number of `collection_interval`
    pub collection_interval: u64,
    /// Retention Days
    /// Number of `retention_days`
    pub retention_days: u32,
}

/// Alert routing: enabled flag and channel identifiers.
#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct AlertingConfig {
    /// Enabled
    /// Whether feature is enabled
    pub enabled: bool,
    /// Notification Channels
    /// Collection of notification channels
    pub notification_channels: Vec<String>,
}

/// Monitoring system health check configuration
///
/// Domain-specific configuration for monitoring system health checks.
/// Renamed from `HealthCheckConfig` for clarity and to avoid ambiguity
/// with other domain-specific health check configurations.
#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct MonitoringHealthCheckConfig {
    /// Enabled
    /// Whether feature is enabled
    pub enabled: bool,
    /// Check Interval
    /// Number of `check_interval`
    pub check_interval: u64,
    /// Timeout
    pub timeout: u64,
}

/// Health check configuration
///
/// **DEPRECATED**: Use `super::super::config::domains::network::monitoring::HealthCheckConfiguration` instead.
#[deprecated(
    since = "3.1.0",
    note = "Use canonical::config::domains::network::monitoring::HealthCheckConfiguration instead"
)]
pub type HealthCheckConfig =
    super::super::config::domains::network::monitoring::HealthCheckConfiguration;
