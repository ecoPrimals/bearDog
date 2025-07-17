//! Monitoring and observability configuration types
//!
//! Contains all logging, metrics, audit, and monitoring configuration structures.

use serde::{Deserialize, Serialize};
use std::time::Duration;

/// Comprehensive monitoring configuration
#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct MonitoringConfig {
    /// Logging configuration
    pub logging: LoggingConfig,
    /// Metrics configuration
    pub metrics: MetricsConfig,
    /// Health monitoring configuration
    pub health: HealthMonitoringConfig,
    /// Performance monitoring configuration
    pub performance: PerformanceMonitoringConfig,
    /// Alerting configuration
    pub alerting: AlertingConfig,
}

/// Logging configuration
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct LoggingConfig {
    /// Log level
    pub level: String,
    /// Log format
    pub format: String,
    /// Log output configuration
    pub output: LoggingOutputConfig,
}

/// Logging output configuration
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct LoggingOutputConfig {
    /// Console output
    pub console: bool,
    /// File output
    pub file: Option<String>,
    /// Syslog output
    pub syslog: bool,
}

/// Metrics configuration
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct MetricsConfig {
    /// Enable metrics collection
    pub enabled: bool,
    /// Metrics collection interval
    pub interval: Duration,
    /// Metrics storage backend
    pub backend: MetricsBackend,
    /// Metrics retention period
    pub retention: Duration,
}

/// Metrics backend configuration
#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum MetricsBackend {
    /// Prometheus metrics backend
    Prometheus,
    /// InfluxDB metrics backend
    InfluxDB,
    /// In-memory metrics backend
    Memory,
}

/// Health monitoring configuration
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct HealthMonitoringConfig {
    /// Enable health monitoring
    pub enabled: bool,
    /// Health check interval
    pub interval: Duration,
    /// Health check timeout
    pub timeout: Duration,
    /// Health check endpoints
    pub endpoints: Vec<HealthEndpoint>,
}

/// Health endpoint configuration
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct HealthEndpoint {
    /// Endpoint name
    pub name: String,
    /// Endpoint URL
    pub url: String,
    /// Expected status code
    pub expected_status: u16,
    /// Timeout for this endpoint
    pub timeout: Duration,
}

/// Performance monitoring configuration
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PerformanceMonitoringConfig {
    /// Enable performance monitoring
    pub enabled: bool,
    /// Performance metrics collection interval
    pub interval: Duration,
    /// CPU usage threshold for alerts
    pub cpu_threshold: f32,
    /// Memory usage threshold for alerts
    pub memory_threshold: f32,
    /// Disk usage threshold for alerts
    pub disk_threshold: f32,
}

/// Alerting configuration
#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct AlertingConfig {
    /// Enable alerting
    pub enabled: bool,
    /// Alert channels
    pub channels: Vec<AlertChannel>,
    /// Alert thresholds
    pub thresholds: AlertThresholds,
}

/// Alert channel configuration
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct AlertChannel {
    /// Channel name
    pub name: String,
    /// Channel type
    pub channel_type: AlertChannelType,
    /// Channel configuration
    pub config: AlertChannelConfig,
}

/// Alert channel type
#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum AlertChannelType {
    /// Email alert channel
    Email,
    /// Slack alert channel
    Slack,
    /// Webhook alert channel
    Webhook,
    /// SMS alert channel
    SMS,
}

/// Alert channel configuration
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct AlertChannelConfig {
    /// Channel-specific configuration
    pub settings: std::collections::HashMap<String, String>,
}

/// Alert thresholds configuration
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct AlertThresholds {
    /// CPU usage threshold
    pub cpu: f32,
    /// Memory usage threshold
    pub memory: f32,
    /// Disk usage threshold
    pub disk: f32,
    /// Error rate threshold
    pub error_rate: f32,
    /// Response time threshold
    pub response_time: Duration,
}

// Default implementations

impl Default for LoggingConfig {
    fn default() -> Self {
        Self {
            level: "info".to_string(),
            format: "json".to_string(),
            output: LoggingOutputConfig::default(),
        }
    }
}

impl Default for LoggingOutputConfig {
    fn default() -> Self {
        Self {
            console: true,
            file: None,
            syslog: false,
        }
    }
}

impl Default for MetricsConfig {
    fn default() -> Self {
        Self {
            enabled: true,
            interval: Duration::from_secs(60),
            backend: MetricsBackend::Memory,
            retention: Duration::from_secs(7 * 24 * 60 * 60), // 7 days
        }
    }
}

impl Default for HealthMonitoringConfig {
    fn default() -> Self {
        Self {
            enabled: true,
            interval: Duration::from_secs(30),
            timeout: Duration::from_secs(10),
            endpoints: vec![],
        }
    }
}

impl Default for PerformanceMonitoringConfig {
    fn default() -> Self {
        Self {
            enabled: true,
            interval: Duration::from_secs(60),
            cpu_threshold: 80.0,
            memory_threshold: 85.0,
            disk_threshold: 90.0,
        }
    }
}

impl Default for AlertThresholds {
    fn default() -> Self {
        Self {
            cpu: 80.0,
            memory: 85.0,
            disk: 90.0,
            error_rate: 5.0,
            response_time: Duration::from_millis(1000),
        }
    }
}
