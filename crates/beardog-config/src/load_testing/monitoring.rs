//! Load Test Monitoring Configuration
//!
//! This module defines monitoring configurations for load testing,
//! including metrics collection, alerting, and reporting.

use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use std::time::Duration;

/// Load test monitoring configuration
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct LoadTestMonitoringConfig {
    /// Enable monitoring
    pub enabled: bool,
    /// Metrics collection
    pub metrics: MetricsCollection,
    /// Alerting configuration
    pub alerting: AlertingConfig,
    /// Reporting configuration
    pub reporting: ReportingConfig,
}

/// Metrics collection configuration
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct MetricsCollection {
    /// Collection interval
    pub interval: Duration,
    /// Metrics to collect
    pub metrics: Vec<String>,
    /// Storage configuration
    pub storage: MetricsStorage,
}

/// Metrics storage configuration
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct MetricsStorage {
    /// Storage type
    pub storage_type: MetricsStorageType,
    /// Storage location
    pub location: String,
    /// Retention period
    pub retention_period: Duration,
}

/// Metrics storage type
#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum MetricsStorageType {
    /// In-memory storage
    InMemory,
    /// File storage
    File,
    /// Database storage
    Database,
    /// Time-series database
    TimeSeries,
}

/// Alerting configuration
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct AlertingConfig {
    /// Enable alerting
    pub enabled: bool,
    /// Alert rules
    pub rules: Vec<AlertRule>,
    /// Notification channels
    pub channels: Vec<NotificationChannel>,
}

/// Alert rule
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct AlertRule {
    /// Rule name
    pub name: String,
    /// Rule condition
    pub condition: AlertCondition,
    /// Rule severity
    pub severity: AlertSeverity,
    /// Rule channels
    pub channels: Vec<String>,
}

/// Alert condition
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct AlertCondition {
    /// Metric name
    pub metric: String,
    /// Operator
    pub operator: AlertOperator,
    /// Threshold value
    pub threshold: f64,
    /// Evaluation window
    pub window: Duration,
}

/// Alert operator
#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum AlertOperator {
    /// Greater than
    GreaterThan,
    /// Less than
    LessThan,
    /// Equal to
    EqualTo,
    /// Not equal to
    NotEqualTo,
}

/// Alert severity
#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum AlertSeverity {
    /// Info level
    Info,
    /// Warning level
    Warning,
    /// Error level
    Error,
    /// Critical level
    Critical,
}

/// Notification channel
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct NotificationChannel {
    /// Channel name
    pub name: String,
    /// Channel type
    pub channel_type: NotificationChannelType,
    /// Channel configuration
    pub configuration: HashMap<String, String>,
}

/// Notification channel type
#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum NotificationChannelType {
    /// Email notification
    Email,
    /// Slack notification
    Slack,
    /// Webhook notification
    Webhook,
    /// SMS notification
    SMS,
}

/// Reporting configuration
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ReportingConfig {
    /// Enable reporting
    pub enabled: bool,
    /// Report types
    pub report_types: Vec<ReportType>,
    /// Report schedule
    pub schedule: ReportSchedule,
}

/// Report type
#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum ReportType {
    /// Summary report
    Summary,
    /// Detailed report
    Detailed,
    /// Performance report
    Performance,
    /// Error report
    Error,
}

/// Report schedule
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ReportSchedule {
    /// Schedule type
    pub schedule_type: ScheduleType,
    /// Schedule interval
    pub interval: Duration,
    /// Schedule time
    pub time: Option<String>,
}

/// Schedule type
#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum ScheduleType {
    /// One-time schedule
    OneTime,
    /// Recurring schedule
    Recurring,
    /// Cron schedule
    Cron { expression: String },
}

impl Default for LoadTestMonitoringConfig {
    fn default() -> Self {
        Self {
            enabled: true,
            metrics: MetricsCollection::default(),
            alerting: AlertingConfig::default(),
            reporting: ReportingConfig::default(),
        }
    }
}

impl Default for MetricsCollection {
    fn default() -> Self {
        Self {
            interval: Duration::from_secs(10),
            metrics: vec![
                "response_time".to_string(),
                "throughput".to_string(),
                "error_rate".to_string(),
                "cpu_usage".to_string(),
                "memory_usage".to_string(),
            ],
            storage: MetricsStorage::default(),
        }
    }
}

impl Default for MetricsStorage {
    fn default() -> Self {
        Self {
            storage_type: MetricsStorageType::InMemory,
            location: "./metrics".to_string(),
            retention_period: Duration::from_secs(24 * 60 * 60), // 1 day
        }
    }
}

impl Default for AlertingConfig {
    fn default() -> Self {
        Self {
            enabled: true,
            rules: Vec::new(),
            channels: Vec::new(),
        }
    }
}

impl Default for ReportingConfig {
    fn default() -> Self {
        Self {
            enabled: true,
            report_types: vec![ReportType::Summary, ReportType::Performance],
            schedule: ReportSchedule::default(),
        }
    }
}

impl Default for ReportSchedule {
    fn default() -> Self {
        Self {
            schedule_type: ScheduleType::OneTime,
            interval: Duration::from_secs(3600), // 1 hour
            time: None,
        }
    }
}
