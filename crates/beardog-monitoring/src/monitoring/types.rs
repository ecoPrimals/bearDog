// SPDX-License-Identifier: AGPL-3.0-or-later

// Module documentation
//
// This module provides functionality for the BearDog ecosystem.

use beardog_config::env_keys;
use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};
use std::collections::BTreeMap;

use beardog_types::canonical::HealthStatus;

/// `ComponentHealth` represents the health status of a system component
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
pub struct ComponentHealth {
    /// Name of the component
    /// Name of the item
    pub name: String,
    /// Current health status
    /// Current status of the component
    pub status: HealthStatus,
    /// Optional message describing the health status
    /// Optional message
    pub message: Option<String>,
    /// Timestamp of the last health check
    /// The last check value
    pub last_check: DateTime<Utc>,
    /// Duration of the health check in milliseconds
    /// Number of `check_duration_ms`
    pub check_duration_ms: u64,
    /// Additional metadata about the component
    /// Mapping of metadata
    pub metadata: BTreeMap<String, String>,
}

/// Roll-up of every component's status plus a single overall verdict.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SystemHealth {
    /// Overall system health status
    /// Current status of the overall
    pub overall_status: HealthStatus,
    /// Individual component health statuses
    /// Collection of components
    pub components: Vec<ComponentHealth>,
    /// Timestamp of the health assessment
    pub timestamp: DateTime<Utc>,
}

/// Captures machine-wide performance counters at a moment in time (load, I/O, uptime).
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PerformanceSnapshot {
    /// CPU usage percentage (0.0 to 100.0)
    /// The cpu usage value
    pub cpu_usage: f64,
    /// Memory usage percentage (0.0 to 100.0)
    /// The memory usage value
    pub memory_usage: f64,
    /// Total available memory in bytes
    /// Number of `memory_total`
    pub memory_total: u64,
    /// Used memory in bytes
    /// Number of `memory_used`
    pub memory_used: u64,
    /// Disk usage percentage (0.0 to 100.0)
    /// The disk usage value
    pub disk_usage: f64,
    /// Total disk space in bytes
    /// Number of `disk_total`
    pub disk_total: u64,
    /// Used disk space in bytes
    /// Number of `disk_used`
    pub disk_used: u64,
    /// Network bytes received
    /// Number of `network_rx`
    pub network_rx: u64,
    /// Network bytes transmitted
    /// Number of `network_tx`
    pub network_tx: u64,
    /// System load average (1 minute)
    /// The load average 1m value
    pub load_average_1m: f64,
    /// System load average (5 minutes)
    /// The load average 5m value
    pub load_average_5m: f64,
    /// System load average (15 minutes)
    /// The load average 15m value
    pub load_average_15m: f64,
    /// Number of active network connections
    /// Number of `active_connections`
    pub active_connections: u32,
    /// System uptime in seconds
    pub uptime_seconds: u64,
    /// Timestamp when this snapshot was taken
    pub timestamp: DateTime<Utc>,
}

impl Default for PerformanceSnapshot {
    fn default() -> Self {
        Self {
            cpu_usage: 0.0,
            memory_usage: 0.0,
            memory_total: 0,
            memory_used: 0,
            disk_usage: 0.0,
            disk_total: 0,
            disk_used: 0,
            network_rx: 0,
            network_tx: 0,
            load_average_1m: 0.0,
            load_average_5m: 0.0,
            load_average_15m: 0.0,
            active_connections: 0,
            uptime_seconds: 0,
            timestamp: Utc::now(),
        }
    }
}

/// `ResourceUsage` tracks resource utilization over time
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ResourceUsage {
    /// Current CPU usage percentage
    /// The cpu percent value
    pub cpu_percent: f64,
    /// Current memory usage in bytes
    /// Number of `memory_bytes`
    pub memory_bytes: u64,
    /// Current disk usage in bytes
    /// Number of `disk_bytes`
    pub disk_bytes: u64,
    /// Network bytes received since last measurement
    /// Number of `network_in_bytes`
    pub network_in_bytes: u64,
    /// Network bytes sent since last measurement
    /// Number of `network_out_bytes`
    pub network_out_bytes: u64,
    /// Number of open file descriptors
    /// Number of `open_file_descriptors`
    pub open_file_descriptors: u32,
    /// Number of active threads
    /// Number of `active_threads`
    pub active_threads: u32,
    /// Timestamp of this resource measurement
    pub timestamp: DateTime<Utc>,
}

impl Default for ResourceUsage {
    fn default() -> Self {
        Self {
            cpu_percent: 0.0,
            memory_bytes: 0,
            disk_bytes: 0,
            network_in_bytes: 0,
            network_out_bytes: 0,
            open_file_descriptors: 0,
            active_threads: 0,
            timestamp: Utc::now(),
        }
    }
}

/// `MonitoringAlert` represents a system alert
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct MonitoringAlert {
    /// Stable identifier used for deduplication, routing, and acknowledgment.
    pub id: String,
    /// Alert message
    /// The message value
    pub message: String,
    /// Alert severity level
    /// The severity value
    pub severity: AlertSeverity,
    /// Timestamp when the alert was created
    /// The created at value
    pub created_at: DateTime<Utc>,
}

/// `AlertThresholds` defines monitoring thresholds
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct AlertThresholds {
    /// CPU usage threshold percentage
    /// The cpu threshold value
    pub cpu_threshold: f64,
    /// Memory usage threshold percentage
    /// The memory threshold value
    pub memory_threshold: f64,
    /// Disk usage threshold percentage
    /// The disk threshold value
    pub disk_threshold: f64,
    /// Network latency threshold in milliseconds
    /// Number of `network_latency_ms`
    pub network_latency_ms: u64,
    /// Error rate threshold percentage
    /// The error rate threshold value
    pub error_rate_threshold: f64,
}

impl Default for AlertThresholds {
    fn default() -> Self {
        Self {
            cpu_threshold: 80.0,
            memory_threshold: 85.0,
            disk_threshold: 90.0,
            network_latency_ms: beardog_errors::process_env::var(
                "BEARDOG_NETWORK_LATENCY_THRESHOLD_MS",
            )
            .ok()
            .and_then(|l| l.parse().ok())
            .unwrap_or(1000), // 1 second default
            error_rate_threshold: 5.0,
        }
    }
}

/// `MetricCollection` holds a collection of metrics
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct MetricCollection {
    /// Collection of counter metrics
    /// Number of itemsers
    pub counters: BTreeMap<String, u64>,
    /// Collection of gauge metrics
    /// The gauges value
    pub gauges: BTreeMap<String, f64>,
    /// Collection of histogram metrics
    /// The histograms value
    pub histograms: BTreeMap<String, Vec<f64>>,
    /// Collection of timer metrics
    pub timers: BTreeMap<String, std::time::Duration>,
    /// Timestamp when this collection was created
    pub timestamp: DateTime<Utc>,
}

impl Default for MetricCollection {
    fn default() -> Self {
        Self {
            counters: BTreeMap::new(),
            gauges: BTreeMap::new(),
            histograms: BTreeMap::new(),
            timers: BTreeMap::new(),
            timestamp: Utc::now(),
        }
    }
}

/// `MetricValue` represents different types of metric values
#[derive(Debug, Clone)]
pub enum MetricValue {
    /// Counter metric (monotonically increasing)
    Counter(u64),
    /// Gauge metric (can increase or decrease)
    Gauge(f64),
    /// Histogram metric (collection of values)
    Histogram(Vec<f64>),
    /// Timer metric (duration measurement)
    Timer(std::time::Duration),
}

/// High-level category for a fired alert (performance, security, capacity, etc.).
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
pub enum AlertType {
    /// Latency, throughput, or saturation style alerts.
    Performance,
    /// Security-related alert
    Security,
    /// Resource usage alert
    Resource,
    /// System error alert
    Error,
    /// Custom alert type
    Custom(String),
}

/// `AlertSeverity` defines alert severity levels
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq, PartialOrd, Ord)]
pub enum AlertSeverity {
    /// Low severity alert
    Low,
    /// Medium severity alert
    Medium,
    /// High severity alert
    High,
    /// Critical severity alert
    Critical,
}

/// Backwards-compatible alias for [`AlertSeverity`] in user-facing APIs.
pub type AlertLevel = AlertSeverity;

/// Monitoring configuration
/// Local monitoring configuration for simplified use cases.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct MonitoringConfig {
    /// Whether monitoring is enabled
    pub enabled: bool,
    /// Maximum number of snapshots to retain
    pub max_snapshots: usize,
    /// Snapshot interval in seconds
    pub snapshot_interval_seconds: u64,
    /// Alert thresholds configuration
    pub alert_thresholds: AlertThresholds,
    /// Prometheus configuration
    pub prometheus: PrometheusConfig,
}

/// Prometheus configuration
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PrometheusConfig {
    /// Whether Prometheus export is enabled
    pub enabled: bool,
    /// Prometheus metrics endpoint
    pub endpoint: String,
    /// Prometheus server port
    pub port: u16,
    /// Prefix applied to each exported metric name (e.g. primal or subsystem).
    pub prefix: String,
}

impl Default for PrometheusConfig {
    fn default() -> Self {
        Self {
            enabled: true,
            endpoint: "/metrics".to_string(),
            port: beardog_errors::process_env::var(env_keys::ENV_PROMETHEUS_PORT)
                .ok()
                .and_then(|p| p.parse().ok())
                .unwrap_or(9090), // Prometheus standard port
            prefix: beardog_errors::process_env::var(env_keys::ENV_PRIMAL_NAME)
                .unwrap_or_else(|_| env!("CARGO_PKG_NAME").to_string()),
        }
    }
}

impl Default for MonitoringConfig {
    fn default() -> Self {
        Self {
            enabled: true,
            max_snapshots: 100,
            snapshot_interval_seconds: 60,
            alert_thresholds: AlertThresholds::default(),
            prometheus: PrometheusConfig::default(),
        }
    }
}
