use beardog_config;
use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use std::time::Duration;

/// Metric value types that can be recorded in the monitoring system
#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum MetricValue {
    /// Counter metric that only increases
    Counter(u64),
    /// Gauge metric that can go up or down
    Gauge(f64),
    /// Histogram metric containing multiple values
    Histogram(Vec<f64>),
    /// Timer metric representing duration
    Timer(Duration),
    /// String metric for categorical data
    String(String),
}

/// System health status levels
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub enum HealthStatus {
    /// System is operating normally
    Healthy,
    /// System is operational but with some issues
    Degraded,
    /// System is not functioning properly
    Unhealthy,
    /// Health status cannot be determined
    Unknown,
}

/// Individual component health information
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ComponentHealth {
    /// Name of the component being monitored
    pub name: String,
    /// Current health status of the component
    pub status: HealthStatus,
    /// Optional message providing additional context
    pub message: Option<String>,
    /// Timestamp of the last health check
    pub last_check: DateTime<Utc>,
    /// Duration of the health check in milliseconds
    pub check_duration_ms: u64,
    /// Additional metadata about the component
    pub metadata: HashMap<String, String>,
}

/// Overall system health information
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SystemHealth {
    /// Overall health status of the system
    pub status: HealthStatus,
    /// System uptime in seconds
    pub uptime_seconds: i64,
    /// Version of the BearDog system
    pub version: String,
    /// Health status of individual components
    pub components: Vec<ComponentHealth>,
    /// Timestamp when this health information was last updated
    pub last_updated: DateTime<Utc>,
}

/// Resource metrics for system capacity monitoring
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ResourceMetrics {
    /// Total number of CPU cores available
    pub cpu_cores_total: u32,
    /// Number of CPU cores currently being used
    pub cpu_cores_used: u32,
    /// Total memory available in gigabytes
    pub memory_total_gb: u32,
    /// Memory currently being used in gigabytes
    pub memory_used_gb: u32,
    /// Total disk space available in gigabytes
    pub disk_total_gb: u32,
    /// Disk space currently being used in gigabytes
    pub disk_used_gb: u32,
    /// Network bandwidth capacity in megabits per second
    pub network_bandwidth_mbps: u32,
    /// Network utilization as a percentage (0.0 to 100.0)
    pub network_utilization_percent: f64,
    /// Number of currently active network connections
    pub active_connections: u32,
    /// Maximum allowed network connections
    pub max_connections: u32,
    /// Total size of the thread pool
    pub thread_pool_size: u32,
    /// Number of threads currently active
    pub active_threads: u32,
}

/// Performance metrics for system monitoring
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PerformanceMetrics {
    /// CPU usage as a percentage (0.0 to 100.0)
    pub cpu_usage_percent: f64,
    /// Memory usage in bytes
    pub memory_usage_bytes: u64,
    /// Total memory available in bytes
    pub memory_total_bytes: u64,
    /// Disk usage in bytes
    pub disk_usage_bytes: u64,
    /// Total disk space available in bytes
    pub disk_total_bytes: u64,
    /// Network bytes received
    pub network_rx_bytes: u64,
    /// Network bytes transmitted
    pub network_tx_bytes: u64,
    /// Number of currently active connections
    pub active_connections: u32,
    /// Total number of requests processed
    pub request_count: u64,
    /// Total number of errors encountered
    pub error_count: u64,
    /// Average response time in milliseconds
    pub avg_response_time_ms: f64,
}

/// System metrics aggregating performance and custom metrics
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SystemMetrics {
    /// Timestamp when these metrics were collected
    pub timestamp: DateTime<Utc>,
    /// Performance-related metrics
    pub performance: PerformanceMetrics,
    /// Custom application-specific metrics
    pub custom_metrics: HashMap<String, f64>,
}

/// Alert thresholds for monitoring system alerts
#[derive(Debug, Clone)]
pub struct AlertThresholds {
    /// CPU usage threshold percentage (0.0 to 100.0)
    pub cpu_threshold: f64,
    /// Memory usage threshold percentage (0.0 to 100.0)
    pub memory_threshold: f64,
    /// Disk usage threshold percentage (0.0 to 100.0)
    pub disk_threshold: f64,
    /// Error rate threshold percentage (0.0 to 100.0)
    pub error_rate_threshold: f64,
    /// Response time threshold in milliseconds
    pub response_time_threshold: f64,
}

impl Default for AlertThresholds {
    fn default() -> Self {
        Self {
            cpu_threshold: 80.0,
            memory_threshold: 85.0,
            disk_threshold: 90.0,
            error_rate_threshold: 5.0,
            response_time_threshold:
                beardog_config::constants::performance::DEFAULT_RESPONSE_TIME_THRESHOLD as f64,
        }
    }
}

/// Internal metrics summary (always free)
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct InternalMetricsSummary {
    /// Total number of security events recorded
    pub security_events: u64,
    /// Total number of encryption operations performed
    pub encryption_operations: u64,
    /// Total number of threat detections
    pub threat_detections: u64,
    /// Total number of compliance checks performed
    pub compliance_checks: u64,
    /// Total number of API requests processed
    pub api_requests: u64,
    /// Total number of errors encountered
    pub error_count: u64,
    /// Number of currently active sessions
    pub active_sessions: u64,
    /// Timestamp when metrics were last updated
    pub last_updated: DateTime<Utc>,
}

/// Prometheus configuration settings
#[derive(Debug, Clone)]
pub struct PrometheusConfig {
    /// Prometheus endpoint URL
    pub endpoint: String,
    /// Port number for Prometheus metrics server
    pub port: u16,
}
