// SPDX-License-Identifier: AGPL-3.0-or-later

//! System metrics data structures and monitoring configuration.

use beardog_types::canonical::HealthStatus;

/// Controls monitoring intervals, alert thresholds, and system limits.
#[derive(Debug, Clone, Copy)]
pub struct SystemMonitorConfig {
    /// Interval between system checks in milliseconds.
    pub check_interval_ms: u64,
    /// CPU alert threshold (%).
    pub alert_threshold_cpu: f64,
    /// Memory alert threshold (%).
    pub alert_threshold_memory: f64,
    /// Disk alert threshold (%).
    pub alert_threshold_disk: f64,
    /// Maximum number of alerts to keep in history.
    pub max_alert_history: usize,
}

impl Default for SystemMonitorConfig {
    fn default() -> Self {
        Self {
            check_interval_ms: 5000,
            alert_threshold_cpu: 80.0,
            alert_threshold_memory: 85.0,
            alert_threshold_disk: 90.0,
            max_alert_history: 1000,
        }
    }
}

/// Contains current system resource usage statistics including CPU, memory,
/// disk, network, and uptime.
#[derive(Debug, Clone, Copy, Default)]
pub struct SystemMetrics {
    /// Current CPU usage as a percentage (0.0 to 100.0).
    pub cpu_usage_percent: f64,
    /// Current memory usage as a percentage (0.0 to 100.0).
    pub memory_usage_percent: f64,
    /// Current disk usage as a percentage (0.0 to 100.0).
    pub disk_usage_percent: f64,
    /// Total bytes received over the network since monitoring began.
    pub network_bytes_in: u64,
    /// Total bytes sent over the network since monitoring began.
    pub network_bytes_out: u64,
    /// Seconds since monitoring started.
    pub uptime_seconds: u64,
    /// When these metrics were last updated.
    pub last_updated: Option<chrono::DateTime<chrono::Utc>>,
}

/// Per-component health snapshot.
#[derive(Debug, Clone)]
pub struct ComponentHealth {
    /// Canonical component name.
    pub component_name: String,
    /// Derived health status.
    pub status: HealthStatus,
    /// When the last health probe ran.
    pub last_check: chrono::DateTime<chrono::Utc>,
    /// Probe round-trip time in milliseconds.
    pub response_time_ms: u64,
    /// Cumulative error count since monitoring started.
    pub error_count: u64,
    /// Uptime percentage over the monitoring period.
    pub uptime_percent: f64,
}
