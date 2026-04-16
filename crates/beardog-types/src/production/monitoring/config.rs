// SPDX-License-Identifier: AGPL-3.0-or-later

//! Configuration types for production monitoring.

use serde::{Deserialize, Serialize};

/// Production monitoring configuration
///
/// This is a simplified config for production monitoring.
/// Internally uses the canonical `MonitoringConfig`.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct MonitoringConfig {
    /// Interval between monitoring cycles in seconds
    pub monitoring_interval_seconds: u64,
    /// Whether to enable real-time alerting
    pub enable_alerting: bool,
    /// Maximum number of alerts to retain in memory
    pub alert_retention_count: usize,
    /// Latency/throughput retention and toggles
    pub performance_config: PerformanceConfig,
    /// System monitoring configuration
    pub system_config: SystemConfig,
}

impl Default for MonitoringConfig {
    fn default() -> Self {
        Self {
            monitoring_interval_seconds: 60,
            enable_alerting: true,
            alert_retention_count: 1000,
            performance_config: PerformanceConfig::default(),
            system_config: SystemConfig::default(),
        }
    }
}

// Type alias for backward compatibility
// DEPRECATED: Transitional alias - use canonical::monitoring::MonitoringConfig directly
pub use crate::canonical::monitoring::MonitoringConfig as CanonicalMonitoringConfig;

/// Throughput and latency monitoring retention settings.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PerformanceConfig {
    /// Enable latency monitoring
    /// Whether `enable_latency_monitoring` is enabled
    pub enable_latency_monitoring: bool,
    /// Enable throughput monitoring
    /// Whether `enable_throughput_monitoring` is enabled
    pub enable_throughput_monitoring: bool,
    /// Number of `retention_hours`
    pub retention_hours: u32,
}

impl Default for PerformanceConfig {
    fn default() -> Self {
        Self {
            enable_latency_monitoring: true,
            enable_throughput_monitoring: true,
            retention_hours: 24,
        }
    }
}

/// System resource monitoring configuration
///
/// Configures monitoring of system resources like CPU, memory, and disk.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SystemConfig {
    /// Enable CPU monitoring
    /// Whether `enable_cpu_monitoring` is enabled
    pub enable_cpu_monitoring: bool,
    /// Enable memory monitoring
    /// Whether `enable_memory_monitoring` is enabled
    pub enable_memory_monitoring: bool,
    /// Enable disk monitoring
    /// Whether `enable_disk_monitoring` is enabled
    pub enable_disk_monitoring: bool,
    /// System monitoring interval in seconds
    /// Number of `system_interval_seconds`
    pub system_interval_seconds: u64,
}

impl Default for SystemConfig {
    fn default() -> Self {
        Self {
            enable_cpu_monitoring: true,
            enable_memory_monitoring: true,
            enable_disk_monitoring: true,
            system_interval_seconds: 30,
        }
    }
}
