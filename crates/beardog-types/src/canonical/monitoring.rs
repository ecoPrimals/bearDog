

use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use std::time::Duration;

/// Unified canonical monitoring configuration
/// Consolidates all monitoring configuration patterns across the codebase
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct MonitoringConfig {
    pub enabled: bool,
    pub health_checks: HealthCheckConfig,
    pub metrics: MetricsConfig,
    pub alerts: AlertConfig,
    pub logging: LoggingConfig,
    pub tracing: TracingConfig,
    pub prometheus: PrometheusConfig,
    pub security_monitoring: SecurityMonitoringConfig,
    pub performance_monitoring: PerformanceMonitoringConfig,
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
pub struct HealthCheckConfig {
    pub enabled: bool,
    pub check_interval: Duration,
    pub timeout: Duration,
    pub failure_threshold: u32,
    pub success_threshold: u32,
    pub endpoints: Vec<HealthCheckEndpoint>,
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

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct HealthCheckEndpoint {
    pub name: String,
    pub url: String,
    pub method: String,
    pub expected_status: u16,
    pub timeout: Duration,
    pub headers: HashMap<String, String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct MetricsConfig {
    pub enabled: bool,
    pub collection_interval: Duration,
    pub retention_period: Duration,
    pub max_metrics: u64,
    pub batch_size: usize,
    pub storage_backend: StorageBackend,
    pub aggregation: MetricAggregationConfig,
    pub custom_collectors: HashMap<String, serde_json::Value>,
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

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct StorageBackend {
    pub backend_type: String,
    pub connection_string: String,
    pub compression_enabled: bool,
    pub encryption_enabled: bool,
}

impl Default for StorageBackend {
    fn default() -> Self {
        Self {
            backend_type: "memory".to_string(),
            connection_string: "".to_string(),
            compression_enabled: true,
            encryption_enabled: false,
        }
    }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct MetricAggregationConfig {
    pub enabled: bool,
    pub window_size: Duration,
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

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct MetricFilter {
    pub metric_name: String,
    pub enabled: bool,
    pub sampling_rate: f64,
    pub tags: HashMap<String, String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct AlertConfig {
    pub enabled: bool,
    pub processing_interval: Duration,
    pub escalation_timeout: Duration,
    pub max_batch_size: u32,
    pub notification_channels: Vec<NotificationChannel>,
    pub routing_rules: Vec<AlertRoutingRule>,
    pub severity_thresholds: HashMap<String, f64>,
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

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct NotificationChannel {
    pub channel_type: String,
    pub endpoint: String,
    pub enabled: bool,
    pub severity_filter: Vec<AlertSeverity>,
    pub authentication: Option<HashMap<String, String>>,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
pub enum AlertSeverity {
    Critical,
    High,
    Medium,
    Low,
    Info,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct AlertRoutingRule {
    pub condition: String,
    pub action: String,
    pub priority: u32,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct RateLimitConfig {
    pub max_alerts_per_minute: u32,
    pub burst_threshold: u32,
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

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct LoggingConfig {
    pub enabled: bool,
    pub level: String,
    pub format: String,
    pub output: String,
    pub rotation: LogRotationConfig,
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

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct LogRotationConfig {
    pub enabled: bool,
    pub max_size_mb: u64,
    pub max_files: u32,
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

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct TracingConfig {
    pub enabled: bool,
    pub endpoint: String,
    pub sample_rate: f64,
    pub service_name: String,
    pub environment: String,
    pub trace_id_header: String,
}

impl Default for TracingConfig {
    fn default() -> Self {
        Self {
            enabled: true,
            endpoint: "http://localhost:14268/api/traces".to_string(),
            sample_rate: 0.1,
            service_name: "beardog".to_string(),
            environment: "development".to_string(),
            trace_id_header: "x-trace-id".to_string(),
        }
    }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PrometheusConfig {
    pub enabled: bool,
    pub endpoint: String,
    pub scrape_interval: Duration,
    pub query_timeout: Duration,
    pub retention_period: Duration,
    pub metrics_path: String,
    pub labels: HashMap<String, String>,
    pub authentication: Option<AuthenticationConfig>,
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
            labels: HashMap::with_capacity(8),
            authentication: None,
        }
    }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct AuthenticationConfig {
    pub auth_type: String,
    pub credentials: HashMap<String, String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SecurityMonitoringConfig {
    pub enabled: bool,
    pub collection_interval: Duration,
    pub alert_thresholds: HashMap<String, f64>,
    pub threat_detection: ThreatDetectionConfig,
    pub audit_logging: bool,
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

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ThreatDetectionConfig {
    pub ml_enhancement: bool,
    pub detection_rules: Vec<String>,
    pub sensitivity_level: SensitivityLevel,
    pub custom_patterns: Vec<String>,
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

#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum SensitivityLevel {
    Low,
    Normal,
    High,
    Maximum,
}

impl Default for SensitivityLevel {
    fn default() -> Self {
        Self::Normal
    }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PerformanceMonitoringConfig {
    pub enabled: bool,
    pub cpu_monitoring: bool,
    pub memory_monitoring: bool,
    pub disk_monitoring: bool,
    pub network_monitoring: bool,
    pub thresholds: PerformanceThresholds,
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

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PerformanceThresholds {
    pub cpu_usage_percent: f64,
    pub memory_usage_percent: f64,
    pub disk_usage_percent: f64,
    pub response_time_ms: u32,
    pub error_rate_percent: f64,
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

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ProfilingConfig {
    pub enabled: bool,
    pub cpu_profiling: bool,
    pub memory_profiling: bool,
    pub profiling_interval: Duration,
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

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct IntegrationMonitoringConfig {
    pub enabled: bool,
    pub collection_interval: Duration,
    pub retention_period: Duration,
    pub alert_thresholds: AlertThresholds,
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

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct AlertThresholds {
    pub error_count_threshold: u32,
    pub latency_threshold_ms: u32,
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

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct IntegrationAuditConfig {
    pub enabled: bool,
    pub log_requests: bool,
    pub include_payload: bool,
    pub include_headers: bool,
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
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct MonitoringMetrics {
    pub timestamp: chrono::DateTime<chrono::Utc>,
    pub system_metrics: SystemMetrics,
    pub application_metrics: ApplicationMetrics,
    pub custom_metrics: HashMap<String, f64>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SystemMetrics {
    pub cpu_usage: f64,
    pub memory_usage: f64,
    pub disk_usage: f64,
    pub network_io: NetworkUsage,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ApplicationMetrics {
    pub request_count: u64,
    pub error_count: u64,
    pub response_times: ResponseTimeMetrics,
    pub active_connections: u32,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ResponseTimeMetrics {
    pub avg_ms: f64,
    pub p50_ms: f64,
    pub p95_ms: f64,
    pub p99_ms: f64,
    pub max_ms: f64,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct NetworkUsage {
    pub bytes_sent: u64,
    pub bytes_received: u64,
    pub packets_sent: u64,
    pub packets_received: u64,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct RequestMetrics {
    pub total_requests: u64,
    pub successful_requests: u64,
    pub failed_requests: u64,
    pub avg_response_time: f64,
    pub requests_per_second: f64,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
pub enum HealthCheckResult {
    Healthy,
    Degraded,
    Unhealthy,
    Unknown,
}

impl Default for HealthCheckResult {
    fn default() -> Self {
        Self::Unknown
    }
}

// Service health monitor
pub trait ServiceHealthMonitor: Send + Sync {
    fn check_health(&self) -> impl std::future::Future<Output = HealthCheckResult> + Send;
    fn get_metrics(&self) -> impl std::future::Future<Output = RequestMetrics> + Send;
}

// Legacy compatibility types - to be removed after migration
#[deprecated(note = "Use MonitoringConfig instead")]
pub type LegacyMonitoringConfig = MonitoringConfig;

#[deprecated(note = "Use MetricsConfig instead")]
pub type LegacyMetricsConfig = MetricsConfig;

#[deprecated(note = "Use AlertConfig instead")]
pub type LegacyAlertConfig = AlertConfig;
