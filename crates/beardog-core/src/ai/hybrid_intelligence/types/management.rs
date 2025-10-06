//! Management configuration types for the hybrid intelligence system
//!
//! This module contains all management-related configuration types including
//! model management, registry, deployment, and monitoring configurations.

use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use std::time::Duration;

/// Model management configuration
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ModelManagementConfig {
    /// Model storage path
    pub model_path: String,
    /// Enable versioning
    pub versioning: bool,
    /// Maximum number of versions to keep
    pub max_versions: u32,
    /// Auto-cleanup old models
    pub auto_cleanup: bool,
}

impl Default for ModelManagementConfig {
    fn default() -> Self {
        Self {
            model_path: "./models".to_string(),
            versioning: true,
            max_versions: 5,
            auto_cleanup: true,
        }
    }
}

/// Model registry configuration
/// Renamed from RegistryConfig to AIModelRegistryConfig for clarity
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct AIModelRegistryConfig {
    /// Registry type
    pub registry_type: RegistryType,
    /// Registry URL
    pub url: String,
    /// Authentication token
    pub auth_token: Option<String>,
    /// Connection timeout
    pub connection_timeout: Duration,
    /// Request timeout
    pub request_timeout: Duration,
    /// Retry attempts
    pub retry_attempts: u32,
    /// Enable SSL verification
    pub ssl_verify: bool,
}

/// Backward compatibility alias
#[deprecated(since = "3.2.0", note = "Use AIModelRegistryConfig instead")]
pub type RegistryConfig = AIModelRegistryConfig;

impl Default for AIModelRegistryConfig {
    fn default() -> Self {
        Self {
            registry_type: RegistryType::Local,
            url: "http://localhost:8080".to_string(),
            auth_token: None,
            connection_timeout: Duration::from_secs(10),
            request_timeout: Duration::from_secs(30),
            retry_attempts: 3,
            ssl_verify: true,
        }
    }
}

/// Registry types
#[derive(Debug, Clone, Copy, Serialize, Deserialize)]
pub enum RegistryType {
    /// Local file system registry
    Local,
    /// Remote HTTP registry
    Remote,
    /// AWS S3 registry
    S3,
    /// Google Cloud Storage registry
    GCS,
    /// Azure Blob Storage registry
    Azure,
    /// MLflow registry
    MLflow,
    /// Weights & Biases registry
    WandB,
}

impl Default for RegistryType {
    fn default() -> Self {
        Self::Local
    }
}

/// Versioning strategy
#[derive(Debug, Clone, Copy, Serialize, Deserialize)]
pub enum VersioningStrategy {
    /// Semantic versioning (major.minor.patch)
    Semantic,
    /// Timestamp-based versioning
    Timestamp,
    /// Sequential numbering
    Sequential,
    /// Git commit hash
    GitHash,
    /// Custom versioning scheme
    Custom,
}

impl Default for VersioningStrategy {
    fn default() -> Self {
        Self::Semantic
    }
}

/// Deployment configuration
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct DeploymentConfig {
    /// Deployment environment
    pub environment: DeploymentEnvironment,
    /// Number of replicas
    pub replicas: u32,
    /// Resource requirements
    pub resources: ResourceRequirements,
    /// Health check configuration
    pub health_check: HealthCheckConfig,
    /// Auto-scaling configuration
    pub auto_scaling: Option<AutoScalingConfig>,
    /// Rolling update strategy
    pub rolling_update: RollingUpdateConfig,
    /// Environment variables
    pub env_vars: HashMap<String, String>,
}

impl Default for DeploymentConfig {
    fn default() -> Self {
        Self {
            environment: DeploymentEnvironment::Development,
            replicas: 1,
            resources: ResourceRequirements::default(),
            health_check: HealthCheckConfig::default(),
            auto_scaling: None,
            rolling_update: RollingUpdateConfig::default(),
            env_vars: HashMap::new(),
        }
    }
}

/// Deployment environments
#[derive(Debug, Clone, Copy, Serialize, Deserialize)]
pub enum DeploymentEnvironment {
    /// Development environment
    Development,
    /// Staging environment
    Staging,
    /// Production environment
    Production,
    /// Testing environment
    Testing,
}

impl Default for DeploymentEnvironment {
    fn default() -> Self {
        Self::Development
    }
}

/// Resource requirements
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ResourceRequirements {
    /// CPU requirements (in cores)
    pub cpu: f64,
    /// Memory requirements (in MB)
    pub memory_mb: u64,
    /// GPU requirements
    pub gpu: Option<GpuRequirements>,
    /// Disk space requirements (in MB)
    pub disk_mb: u64,
}

impl Default for ResourceRequirements {
    fn default() -> Self {
        Self {
            cpu: 1.0,
            memory_mb: 1024, // 1GB
            gpu: None,
            disk_mb: 5120, // 5GB
        }
    }
}

/// GPU requirements
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct GpuRequirements {
    /// Number of GPUs
    pub count: u32,
    /// GPU memory requirements (in MB)
    pub memory_mb: u64,
    /// GPU type preference
    pub gpu_type: Option<String>,
}

/// Health check configuration
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct HealthCheckConfig {
    /// Health check endpoint
    pub endpoint: String,
    /// Check interval
    pub interval: Duration,
    /// Timeout for each check
    pub timeout: Duration,
    /// Number of consecutive failures before unhealthy
    pub failure_threshold: u32,
    /// Number of consecutive successes before healthy
    pub success_threshold: u32,
}

impl Default for HealthCheckConfig {
    fn default() -> Self {
        Self {
            endpoint: "/health".to_string(),
            interval: Duration::from_secs(30),
            timeout: Duration::from_secs(5),
            failure_threshold: 3,
            success_threshold: 1,
        }
    }
}

/// Auto-scaling configuration
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct AutoScalingConfig {
    /// Enable auto-scaling
    pub enabled: bool,
    /// Minimum number of replicas
    pub min_replicas: u32,
    /// Maximum number of replicas
    pub max_replicas: u32,
    /// Target CPU utilization percentage
    pub target_cpu_utilization: f64,
    /// Target memory utilization percentage
    pub target_memory_utilization: f64,
    /// Scale-up cooldown period
    pub scale_up_cooldown: Duration,
    /// Scale-down cooldown period
    pub scale_down_cooldown: Duration,
}

impl Default for AutoScalingConfig {
    fn default() -> Self {
        Self {
            enabled: false,
            min_replicas: 1,
            max_replicas: 10,
            target_cpu_utilization: 70.0,
            target_memory_utilization: 80.0,
            scale_up_cooldown: Duration::from_secs(300), // 5 minutes
            scale_down_cooldown: Duration::from_secs(600), // 10 minutes
        }
    }
}

/// Rolling update configuration
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct RollingUpdateConfig {
    /// Maximum number of unavailable replicas during update
    pub max_unavailable: u32,
    /// Maximum number of extra replicas during update
    pub max_surge: u32,
    /// Update strategy
    pub strategy: UpdateStrategy,
}

impl Default for RollingUpdateConfig {
    fn default() -> Self {
        Self {
            max_unavailable: 1,
            max_surge: 1,
            strategy: UpdateStrategy::RollingUpdate,
        }
    }
}

/// Update strategies
#[derive(Debug, Clone, Copy, Serialize, Deserialize)]
pub enum UpdateStrategy {
    /// Rolling update (gradual replacement)
    RollingUpdate,
    /// Blue-green deployment
    BlueGreen,
    /// Canary deployment
    Canary,
    /// Recreate (all at once)
    Recreate,
}

impl Default for UpdateStrategy {
    fn default() -> Self {
        Self::RollingUpdate
    }
}

/// AI Management monitoring configuration
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct AIManagementMonitoringConfig {
    /// Management operations monitoring
    pub monitor_operations: bool,
    /// Resource allocation tracking
    pub track_resource_allocation: bool,
    /// Performance metrics
    pub performance_metrics: bool,
}

impl Default for AIManagementMonitoringConfig {
    fn default() -> Self {
        Self {
            monitor_operations: true,
            track_resource_allocation: true,
            performance_metrics: true,
        }
    }
}

// Backward compatibility alias
#[deprecated(since = "3.1.0", note = "Use AIManagementMonitoringConfig instead")]
pub type MonitoringConfig = AIManagementMonitoringConfig;

/// Log levels
#[derive(Debug, Clone, Copy, Serialize, Deserialize)]
pub enum LogLevel {
    /// Trace level
    Trace,
    /// Debug level
    Debug,
    /// Info level
    Info,
    /// Warning level
    Warn,
    /// Error level
    Error,
}

impl Default for LogLevel {
    fn default() -> Self {
        Self::Info
    }
}

/// Metrics backends
#[derive(Debug, Clone, Copy, Serialize, Deserialize)]
pub enum MetricsBackend {
    /// Prometheus metrics
    Prometheus,
    /// InfluxDB metrics
    InfluxDB,
    /// CloudWatch metrics
    CloudWatch,
    /// Custom metrics backend
    Custom,
}

impl Default for MetricsBackend {
    fn default() -> Self {
        Self::Prometheus
    }
}

/// Alert configuration
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct AlertConfig {
    /// Enable alerts
    pub enabled: bool,
    /// Alert rules
    pub rules: Vec<AlertRule>,
    /// Notification channels
    pub channels: Vec<NotificationChannel>,
}

impl Default for AlertConfig {
    fn default() -> Self {
        Self {
            enabled: true,
            rules: Vec::new(),
            channels: Vec::new(),
        }
    }
}

/// Alert rule
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct AlertRule {
    /// Rule name
    pub name: String,
    /// Metric to monitor
    pub metric: String,
    /// Threshold value
    pub threshold: f64,
    /// Comparison operator
    pub operator: ComparisonOperator,
    /// Duration before triggering alert
    pub duration: Duration,
    /// Alert severity
    pub severity: AlertSeverity,
}

/// Comparison operators
#[derive(Debug, Clone, Copy, Serialize, Deserialize)]
pub enum ComparisonOperator {
    /// Greater than
    GreaterThan,
    /// Less than
    LessThan,
    /// Equal to
    Equal,
    /// Not equal to
    NotEqual,
    /// Greater than or equal to
    GreaterThanOrEqual,
    /// Less than or equal to
    LessThanOrEqual,
}

/// Alert severity levels
#[derive(Debug, Clone, Copy, Serialize, Deserialize)]
pub enum AlertSeverity {
    /// Low severity
    Low,
    /// Medium severity
    Medium,
    /// High severity
    High,
    /// Critical severity
    Critical,
}

/// Notification channels
#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum NotificationChannel {
    /// Email notifications
    Email {
        /// Recipient addresses
        recipients: Vec<String>,
    },
    /// Slack notifications
    Slack {
        /// Webhook URL
        webhook_url: String,
        /// Channel name
        channel: String,
    },
    /// SMS notifications
    SMS {
        /// Phone numbers
        phone_numbers: Vec<String>,
    },
    /// Webhook notifications
    Webhook {
        /// Webhook URL
        url: String,
        /// HTTP headers
        headers: HashMap<String, String>,
    },
}

/// Custom metric definition
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CustomMetric {
    /// Metric name
    pub name: String,
    /// Metric type
    pub metric_type: MetricType,
    /// Description
    pub description: String,
    /// Labels
    pub labels: HashMap<String, String>,
}

/// Metric types
#[derive(Debug, Clone, Copy, Serialize, Deserialize)]
pub enum MetricType {
    /// Counter metric (monotonically increasing)
    Counter,
    /// Gauge metric (can go up or down)
    Gauge,
    /// Histogram metric (distribution of values)
    Histogram,
    /// Summary metric (quantiles)
    Summary,
}

impl Default for MetricType {
    fn default() -> Self {
        Self::Gauge
    }
} 