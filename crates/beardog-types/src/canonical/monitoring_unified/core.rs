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

#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct AlertingConfig {
    /// Enabled
    /// Whether feature is enabled
    pub enabled: bool,
    /// Notification Channels
    /// Collection of notification channels
    pub notification_channels: Vec<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct HealthCheckConfig {
    /// Enabled
    /// Whether feature is enabled
    pub enabled: bool,
    /// Check Interval
    /// Number of `check_interval`
    pub check_interval: u64,
    /// Timeout
    pub timeout: u64,
}
