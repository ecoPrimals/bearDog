// Module documentation
//
// This module provides functionality for the BearDog ecosystem.

use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use std::time::Duration;

/// Unified canonical monitoring configuration
/// Consolidates all monitoring configuration patterns across the codebase
/// `MonitoringConfig`
///
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct MonitoringConfig {
    /// Whether monitoring is globally enabled
    /// Whether feature is enabled
    pub enabled: bool,
    /// Health check configuration
    /// The health checks value
    pub health_checks: HealthCheckConfig,
    /// Metrics collection configuration
    /// The metrics value
    pub metrics: MetricsConfig,
    /// Alert system configuration
    /// The alerts value
    pub alerts: AlertConfig,
    /// Logging system configuration
    /// The logging value
    pub logging: LoggingConfig,
    /// Distributed tracing configuration
    /// The tracing value
    pub tracing: TracingConfig,
    /// Prometheus metrics configuration
    /// The prometheus value
    pub prometheus: PrometheusConfig,
    /// Security-specific monitoring configuration
    /// The security monitoring value
    pub security_monitoring: SecurityMonitoringConfig,
    pub performance_monitoring: PerformanceMonitoringConfig,
    /// Integration and external system monitoring
    /// The integration monitoring value
    pub integration_monitoring: IntegrationMonitoringConfig,
}

impl Default for MonitoringConfig {
    fn default() -> Self {
        Self {
            enabled: true,
            health_checks: HealthCheckConfig::default(),
            metrics: MetricsConfig::default(),
            alerts: AlertConfig::default(),
            logging: LoggingConfig::default(),
            tracing: TracingConfig::default(),
            prometheus: PrometheusConfig::default(),
            security_monitoring: SecurityMonitoringConfig::default(),
            performance_monitoring: PerformanceMonitoringConfig::default(),
            integration_monitoring: IntegrationMonitoringConfig::default(),
        }
    }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
/// `HealthCheckConfig`
///
pub struct HealthCheckConfig {
    /// Whether health checks are enabled
    /// Whether feature is enabled
    pub enabled: bool,
    /// Interval between health check runs
    /// The check interval value
    pub check_interval: Duration,
    pub timeout: Duration,
    /// Number of `failure_threshold`
    pub failure_threshold: u32,
    /// Number of successes needed to mark as healthy
    /// Number of `success_threshold`
    pub success_threshold: u32,
    /// List of health check endpoints to monitor
    /// Collection of endpoints
    pub endpoints: Vec<HealthCheckEndpoint>,
    /// Custom health checks with configuration
    /// Mapping of custom checks
    pub custom_checks: HashMap<String, serde_json::Value>,
}

impl Default for HealthCheckConfig {
    fn default() -> Self {
        Self {
            enabled: true,
            check_interval: Duration::from_secs(30),
            timeout: Duration::from_secs(10),
            failure_threshold: 3,
            success_threshold: 2,
            endpoints: Vec::new(),
            custom_checks: HashMap::with_capacity(8),
        }
    }
}

/// `HealthCheckEndpoint`
///
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct HealthCheckEndpoint {
    /// Name of the health check endpoint
    /// Name of the item
    pub name: String,
    /// The url value
    pub url: String,
    /// HTTP method to use (GET, POST, etc.)
    /// The method value
    pub method: String,
    /// Current status of the expected
    pub expected_status: u16,
    pub timeout: Duration,
    /// Additional HTTP headers to send with the request
    /// Mapping of headers
    pub headers: HashMap<String, String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
/// `MetricsConfig`
///
pub struct MetricsConfig {
    /// Whether metrics collection is enabled
    /// Whether feature is enabled
    pub enabled: bool,
    /// Interval between metric collection runs
    /// The collection interval value
    pub collection_interval: Duration,
    /// How long to retain metrics data
    /// The retention period value
    pub retention_period: Duration,
    /// Maximum number of metrics to store
    /// Number of `max_metrics`
    pub max_metrics: u64,
    /// Number of `batch_size`
    pub batch_size: usize,
    /// The storage backend value
    pub storage_backend: StorageBackend,
    /// Metric aggregation configuration
    /// The aggregation value
    pub aggregation: MetricAggregationConfig,
    /// Custom metric collectors with configuration
    /// Mapping of custom collectors
    pub custom_collectors: HashMap<String, serde_json::Value>,
    /// Filters to apply to metrics collection
    /// Collection of metric filters
    pub metric_filters: Vec<MetricFilter>,
}

impl Default for MetricsConfig {
    fn default() -> Self {
        Self {
            enabled: true,
            collection_interval: Duration::from_secs(30),
            retention_period: Duration::from_secs(86400 * 7), // 7 days
            max_metrics: 10000,
            batch_size: 100,
            storage_backend: StorageBackend::default(),
            aggregation: MetricAggregationConfig::default(),
            custom_collectors: HashMap::with_capacity(8),
            metric_filters: Vec::new(),
        }
    }
}

/// `StorageBackend`
///
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct StorageBackend {
    /// Type of storage backend (memory, postgres, influxdb, etc.)
    /// The backend type value
    pub backend_type: String,
    /// The connection string value
    pub connection_string: String,
    /// Whether compression is enabled
    pub compression_enabled: bool,
    /// Whether encryption is enabled
    pub encryption_enabled: bool,
}

impl Default for StorageBackend {
    fn default() -> Self {
        Self {
            backend_type: "memory".to_string(),
            connection_string: String::new(),
            compression_enabled: true,
            encryption_enabled: false,
        }
    }
}

/// `MetricAggregationConfig`
///
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct MetricAggregationConfig {
    /// Whether metric aggregation is enabled
    /// Whether feature is enabled
    pub enabled: bool,
    /// The window size value
    pub window_size: Duration,
    /// List of aggregation functions to apply (avg, max, min, count, etc.)
    /// Collection of aggregation functions
    pub aggregation_functions: Vec<String>,
}

impl Default for MetricAggregationConfig {
    fn default() -> Self {
        Self {
            enabled: true,
            window_size: Duration::from_secs(60),
            aggregation_functions: vec!["avg".to_string(), "max".to_string(), "count".to_string()],
        }
    }
}

/// `MetricFilter`
///
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct MetricFilter {
    /// Name of the metric
    pub metric_name: String,
    /// Whether this filter is enabled
    /// Whether feature is enabled
    pub enabled: bool,
    /// The sampling rate value
    pub sampling_rate: f64,
    /// Mapping of tags
    pub tags: HashMap<String, String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
/// `AlertConfig`
///
pub struct AlertConfig {
    /// Enabled
    /// Whether feature is enabled
    pub enabled: bool,
    /// Processing Interval
    /// The processing interval value
    pub processing_interval: Duration,
    /// Escalation Timeout
    pub escalation_timeout: Duration,
    /// Max Batch Size
    /// Number of `max_batch_size`
    pub max_batch_size: u32,
    /// Notification Channels
    /// Collection of notification channels
    pub notification_channels: Vec<NotificationChannel>,
    /// Routing Rules
    /// Collection of routing rules
    pub routing_rules: Vec<AlertRoutingRule>,
    /// Severity Thresholds
    /// Mapping of severity thresholds
    pub severity_thresholds: HashMap<String, f64>,
    /// Rate Limiting
    /// The rate limiting value
    pub rate_limiting: RateLimitConfig,
}

impl Default for AlertConfig {
    fn default() -> Self {
        Self {
            enabled: true,
            processing_interval: Duration::from_secs(10),
            escalation_timeout: Duration::from_secs(300),
            max_batch_size: 50,
            notification_channels: Vec::new(),
            routing_rules: Vec::new(),
            severity_thresholds: HashMap::with_capacity(8),
            rate_limiting: RateLimitConfig::default(),
        }
    }
}

/// `NotificationChannel`
///
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct NotificationChannel {
    /// Channel Type
    /// The channel type value
    pub channel_type: String,
    /// Endpoint
    /// The endpoint value
    pub endpoint: String,
    /// Enabled
    /// Whether feature is enabled
    pub enabled: bool,
    /// Severity Filter
    /// Collection of severity filter
    pub severity_filter: Vec<AlertSeverity>,
    /// Authentication
    /// Optional authentication
    pub authentication: Option<HashMap<String, String>>,
}

/// `AlertSeverity`
///
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
pub enum AlertSeverity {
    /// Critical variant
    /// Critical
    Critical,
    /// High variant
    /// High
    High,
    /// Medium variant
    /// Medium
    Medium,
    /// Low variant
    /// Low
    Low,
    /// Info variant
    /// Info
    Info,
}

/// `AlertRoutingRule`
///
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct AlertRoutingRule {
    /// Condition
    /// The condition value
    pub condition: String,
    /// Action
    /// The action value
    pub action: String,
    /// Priority
    /// Number of priority
    pub priority: u32,
}

/// `RateLimitConfig`
///
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct RateLimitConfig {
    /// Max Alerts Per Minute
    /// Number of `max_alerts_per_minute`
    pub max_alerts_per_minute: u32,
    /// Burst Threshold
    /// Number of `burst_threshold`
    pub burst_threshold: u32,
    /// Cooldown Period
    /// The cooldown period value
    pub cooldown_period: Duration,
}

impl Default for RateLimitConfig {
    fn default() -> Self {
        Self {
            max_alerts_per_minute: 10,
            burst_threshold: 20,
            cooldown_period: Duration::from_secs(60),
        }
    }
}

/// `LoggingConfig`
///
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct LoggingConfig {
    /// Enabled
    /// Whether feature is enabled
    pub enabled: bool,
    /// Level
    /// The level value
    pub level: String,
    /// Format
    pub format: String,
    /// Output
    /// The output value
    pub output: String,
    /// Rotation
    /// The rotation value
    pub rotation: LogRotationConfig,
    /// Structured Logging
    /// Whether `structured_logging` is enabled
    pub structured_logging: bool,
}

impl Default for LoggingConfig {
    fn default() -> Self {
        Self {
            enabled: true,
            level: "info".to_string(),
            format: "json".to_string(),
            output: "stdout".to_string(),
            rotation: LogRotationConfig::default(),
            structured_logging: true,
        }
    }
}

/// `LogRotationConfig`
///
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct LogRotationConfig {
    /// Enabled
    /// Whether feature is enabled
    pub enabled: bool,
    /// Max Size Mb
    /// Number of `max_size_mb`
    pub max_size_mb: u64,
    /// Max Files
    /// Number of `max_files`
    pub max_files: u32,
    /// Rotation Period
    /// The rotation period value
    pub rotation_period: Duration,
}

impl Default for LogRotationConfig {
    fn default() -> Self {
        Self {
            enabled: true,
            max_size_mb: 100,
            max_files: 10,
            rotation_period: Duration::from_secs(86400), // 24 hours
        }
    }
}

/// `TracingConfig`
///
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct TracingConfig {
    /// Enabled
    /// Whether feature is enabled
    pub enabled: bool,
    /// Endpoint
    /// The endpoint value
    pub endpoint: String,
    /// Sample Rate
    /// The sample rate value
    pub sample_rate: f64,
    /// Service Name
    /// Name of the service
    pub service_name: String,
    /// Environment
    /// The environment value
    pub environment: String,
    /// Trace Id Header
    pub trace_id_header: String,
}

impl Default for TracingConfig {
    fn default() -> Self {
        Self {
            enabled: true,
            endpoint: "http://localhost:14268/api/traces".to_string(),
            sample_rate: 0.1,
            service_name: "beardog ".to_string(),
            environment: "development".to_string(),
            trace_id_header: "x-trace-id".to_string(),
        }
    }
}

/// `PrometheusConfig`
///
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PrometheusConfig {
    /// Enabled
    /// Whether feature is enabled
    pub enabled: bool,
    /// Endpoint
    /// The endpoint value
    pub endpoint: String,
    /// Scrape Interval
    /// The scrape interval value
    pub scrape_interval: Duration,
    /// Query Timeout
    pub query_timeout: Duration,
    /// Retention Period
    /// The retention period value
    pub retention_period: Duration,
    /// Metrics Path
    /// The metrics path value
    pub metrics_path: String,
    /// Path
    /// The path value
    pub path: String,
    /// Port
    /// Number of port
    pub port: u16,
    /// Labels
    /// The labels value
    pub labels: std::collections::HashMap<String, String>,
    /// Authentication
    /// The authentication value
    pub authentication: AuthenticationConfig,
}

impl Default for PrometheusConfig {
    fn default() -> Self {
        Self {
            enabled: true,
            endpoint: "http://localhost:9090".to_string(),
            scrape_interval: Duration::from_secs(15),
            query_timeout: Duration::from_secs(30),
            retention_period: Duration::from_secs(86400 * 15), // 15 days
            metrics_path: "/metrics".to_string(),
            path: "/metrics".to_string(),
            port: 9090,
            labels: std::collections::HashMap::new(),
            authentication: AuthenticationConfig::default(),
        }
    }
}

/// `AuthenticationConfig`
///
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct AuthenticationConfig {
    /// Auth Type
    /// The auth type value
    pub auth_type: String,
    /// Credentials
    /// Mapping of credentials
    pub credentials: HashMap<String, String>,
}

impl Default for AuthenticationConfig {
    fn default() -> Self {
        Self {
            auth_type: "none".to_string(),
            credentials: HashMap::new(),
        }
    }
}

/// `SecurityMonitoringConfig`
///
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SecurityMonitoringConfig {
    /// Enabled
    /// Whether feature is enabled
    pub enabled: bool,
    /// Collection Interval
    /// The collection interval value
    pub collection_interval: Duration,
    /// Alert Thresholds
    /// Mapping of alert thresholds
    pub alert_thresholds: HashMap<String, f64>,
    /// Threat Detection
    /// The threat detection value
    pub threat_detection: ThreatDetectionConfig,
    /// Audit Logging
    /// Whether `audit_logging` is enabled
    pub audit_logging: bool,
    /// Compliance Monitoring
    /// Whether `compliance_monitoring` is enabled
    pub compliance_monitoring: bool,
}

impl Default for SecurityMonitoringConfig {
    fn default() -> Self {
        Self {
            enabled: true,
            collection_interval: Duration::from_secs(10),
            alert_thresholds: HashMap::with_capacity(16),
            threat_detection: ThreatDetectionConfig::default(),
            audit_logging: true,
            compliance_monitoring: true,
        }
    }
}

/// `ThreatDetectionConfig`
///
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ThreatDetectionConfig {
    /// Ml Enhancement
    /// Whether `ml_enhancement` is enabled
    pub ml_enhancement: bool,
    /// Detection Rules
    /// Collection of detection rules
    pub detection_rules: Vec<String>,
    /// Sensitivity Level
    /// The sensitivity level value
    pub sensitivity_level: SensitivityLevel,
    /// Custom Patterns
    /// Collection of custom patterns
    pub custom_patterns: Vec<String>,
    /// Real Time Analysis
    pub real_time_analysis: bool,
}

impl Default for ThreatDetectionConfig {
    fn default() -> Self {
        Self {
            ml_enhancement: false,
            detection_rules: Vec::new(),
            sensitivity_level: SensitivityLevel::Normal,
            custom_patterns: Vec::new(),
            real_time_analysis: true,
        }
    }
}

/// `SensitivityLevel`
///
#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum SensitivityLevel {
    /// Low variant
    /// Low
    Low,
    /// Normal variant
    /// Normal
    Normal,
    /// High variant
    /// High
    High,
    /// Maximum variant
    /// Maximum
    Maximum,
}

impl Default for SensitivityLevel {
    fn default() -> Self {
        Self::Normal
    }
}

///
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PerformanceMonitoringConfig {
    /// Enabled
    /// Whether feature is enabled
    pub enabled: bool,
    /// Cpu Monitoring
    /// Whether `cpu_monitoring` is enabled
    pub cpu_monitoring: bool,
    /// Memory Monitoring
    /// Whether `memory_monitoring` is enabled
    pub memory_monitoring: bool,
    /// Disk Monitoring
    /// Whether `disk_monitoring` is enabled
    pub disk_monitoring: bool,
    /// Network Monitoring
    /// Whether `network_monitoring` is enabled
    pub network_monitoring: bool,
    /// Thresholds
    /// The thresholds value
    pub thresholds: PerformanceThresholds,
    /// Profiling
    /// The profiling value
    pub profiling: ProfilingConfig,
}

impl Default for PerformanceMonitoringConfig {
    fn default() -> Self {
        Self {
            enabled: true,
            cpu_monitoring: true,
            memory_monitoring: true,
            disk_monitoring: true,
            network_monitoring: true,
            thresholds: PerformanceThresholds::default(),
            profiling: ProfilingConfig::default(),
        }
    }
}

///
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PerformanceThresholds {
    /// Cpu Usage Percent
    /// The cpu usage percent value
    pub cpu_usage_percent: f64,
    /// Memory Usage Percent
    /// The memory usage percent value
    pub memory_usage_percent: f64,
    /// Disk Usage Percent
    /// The disk usage percent value
    pub disk_usage_percent: f64,
    /// Response Time Ms
    pub response_time_ms: u32,
    /// Error Rate Percent
    /// The error rate percent value
    pub error_rate_percent: f64,
    /// Throughput Per Second
    /// Number of `throughput_per_second`
    pub throughput_per_second: u32,
}

impl Default for PerformanceThresholds {
    fn default() -> Self {
        Self {
            cpu_usage_percent: 80.0,
            memory_usage_percent: 85.0,
            disk_usage_percent: 90.0,
            response_time_ms: 1000,
            error_rate_percent: 5.0,
            throughput_per_second: 100,
        }
    }
}

/// `ProfilingConfig`
///
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ProfilingConfig {
    /// Enabled
    /// Whether feature is enabled
    pub enabled: bool,
    /// Cpu Profiling
    /// Whether `cpu_profiling` is enabled
    pub cpu_profiling: bool,
    /// Memory Profiling
    /// Whether `memory_profiling` is enabled
    pub memory_profiling: bool,
    /// Profiling Interval
    /// The profiling interval value
    pub profiling_interval: Duration,
    /// Profile Duration
    /// The profile duration value
    pub profile_duration: Duration,
}

impl Default for ProfilingConfig {
    fn default() -> Self {
        Self {
            enabled: false,
            cpu_profiling: true,
            memory_profiling: true,
            profiling_interval: Duration::from_secs(300), // 5 minutes
            profile_duration: Duration::from_secs(30),
        }
    }
}

/// `IntegrationMonitoringConfig`
///
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct IntegrationMonitoringConfig {
    /// Enabled
    /// Whether feature is enabled
    pub enabled: bool,
    /// Collection Interval
    /// The collection interval value
    pub collection_interval: Duration,
    /// Retention Period
    /// The retention period value
    pub retention_period: Duration,
    /// Alert Thresholds
    /// The alert thresholds value
    pub alert_thresholds: AlertThresholds,
    /// Audit Config
    pub audit_config: IntegrationAuditConfig,
}

impl Default for IntegrationMonitoringConfig {
    fn default() -> Self {
        Self {
            enabled: true,
            collection_interval: Duration::from_secs(60),
            retention_period: Duration::from_secs(86400 * 7), // 7 days
            alert_thresholds: AlertThresholds::default(),
            audit_config: IntegrationAuditConfig::default(),
        }
    }
}

/// `AlertThresholds`
///
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct AlertThresholds {
    /// Error Count Threshold
    /// Number of `error_threshold`
    pub error_count_threshold: u32,
    /// Latency Threshold Ms
    /// Number of `latency_threshold_ms`
    pub latency_threshold_ms: u32,
    /// Availability Threshold Percent
    /// The availability threshold percent value
    pub availability_threshold_percent: f64,
}

impl Default for AlertThresholds {
    fn default() -> Self {
        Self {
            error_count_threshold: 10,
            latency_threshold_ms: 2000,
            availability_threshold_percent: 99.0,
        }
    }
}

/// `IntegrationAuditConfig`
///
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct IntegrationAuditConfig {
    /// Enabled
    /// Whether feature is enabled
    pub enabled: bool,
    /// Log Requests
    /// Whether `log_requests` is enabled
    pub log_requests: bool,
    /// Include Payload
    /// Whether `include_payload` is enabled
    pub include_payload: bool,
    /// Include Headers
    /// Whether `include_headers` is enabled
    pub include_headers: bool,
    /// Retention Period
    /// The retention period value
    pub retention_period: Duration,
}

impl Default for IntegrationAuditConfig {
    fn default() -> Self {
        Self {
            enabled: true,
            log_requests: true,
            include_payload: false,
            include_headers: false,
            retention_period: Duration::from_secs(86400 * 30), // 30 days
        }
    }
}

// Unified monitoring metrics structures
/// `MonitoringMetrics`
///
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct MonitoringMetrics {
    /// Timestamp
    pub timestamp: chrono::DateTime<chrono::Utc>,
    /// System Metrics
    /// The system metrics value
    pub system_metrics: SystemMetrics,
    /// Application Metrics
    /// The application metrics value
    pub application_metrics: ApplicationMetrics,
    /// Custom Metrics
    /// Mapping of custom metrics
    pub custom_metrics: HashMap<String, f64>,
}

/// `SystemMetrics`
///
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SystemMetrics {
    /// Cpu Usage
    /// The cpu usage value
    pub cpu_usage: f64,
    /// Memory Usage
    /// The memory usage value
    pub memory_usage: f64,
    /// Disk Usage
    /// The disk usage value
    pub disk_usage: f64,
    /// Network Io
    /// The network io value
    pub network_io: NetworkUsage,
}

/// `ApplicationMetrics`
///
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ApplicationMetrics {
    /// Request Count
    /// Number of request
    pub request_count: u64,
    /// Error Count
    /// Number of error
    pub error_count: u64,
    /// Response Times
    pub response_times: ResponseTimeMetrics,
    /// Active Connections
    /// Number of `active_connections`
    pub active_connections: u32,
}

/// `ResponseTimeMetrics`
///
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ResponseTimeMetrics {
    /// Average response time in milliseconds
    /// The avg ms value
    pub avg_ms: f64,
    /// 50th percentile (median) response time in milliseconds
    /// The p50 ms value
    pub p50_ms: f64,
    /// 95th percentile response time in milliseconds
    /// The p95 ms value
    pub p95_ms: f64,
    /// 99th percentile response time in milliseconds
    /// The p99 ms value
    pub p99_ms: f64,
    /// Maximum response time in milliseconds
    /// The max ms value
    pub max_ms: f64,
}

/// `NetworkUsage`
///
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct NetworkUsage {
    /// Bytes Sent
    /// Number of `bytes_sent`
    pub bytes_sent: u64,
    /// Bytes Received
    /// Number of `bytes_received`
    pub bytes_received: u64,
    /// Packets Sent
    /// Number of `packets_sent`
    pub packets_sent: u64,
    /// Packets Received
    /// Number of `packets_received`
    pub packets_received: u64,
}

/// `RequestMetrics`
///
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct RequestMetrics {
    /// Total Requests
    /// Number of `total_requests`
    pub total_requests: u64,
    /// Successful Requests
    /// Number of `successful_requests`
    pub successful_requests: u64,
    /// Failed Requests
    /// Number of `failed_requests`
    pub failed_requests: u64,
    /// Avg Response Time
    pub avg_response_time: f64,
    /// Requests Per Second
    /// The requests per second value
    pub requests_per_second: f64,
}

/// `HealthCheckResult`
///
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
pub enum HealthCheckResult {
    /// Healthy variant
    /// Healthy
    Healthy,
    /// Degraded variant
    /// Degraded
    Degraded,
    /// Unhealthy variant
    /// Unhealthy
    Unhealthy,
    /// Unknown variant
    /// Unknown
    Unknown,
}

impl Default for HealthCheckResult {
    fn default() -> Self {
        Self::Unknown
    }
}

///
/// Provides standardized health checking and metrics collection
pub trait ServiceHealthMonitor: Send + Sync {
    ///
    /// Returns the current health status of the monitored service,
    fn check_health(&self) -> impl std::future::Future<Output = HealthCheckResult> + Send;

    ///
    /// Returns comprehensive metrics including response times,
    /// error rates, and throughput statistics.
    /// Gets metrics
    fn get_metrics(&self) -> impl std::future::Future<Output = RequestMetrics> + Send;
}
