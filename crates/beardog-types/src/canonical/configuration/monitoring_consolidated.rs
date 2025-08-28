use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use std::time::Duration;

/// Consolidated security sentinel configuration
#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct SecuritySentinelConfig {
    pub enabled: bool,
    pub monitoring_interval: Duration,
    pub threat_detection: ThreatDetectionConfig,
    pub alert_config: AlertConfig,
    pub response_config: ResponseConfig,
}

#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct ThreatDetectionConfig {
    pub ml_enhancement: bool,
    pub detection_rules: Vec<String>,
    pub sensitivity_level: SensitivityLevel,
    pub custom_patterns: Vec<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub enum SensitivityLevel {
    #[default]
    Normal,
    High,
    Maximum,
}

#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct AlertConfig {
    pub notification_channels: Vec<NotificationChannel>,
    pub severity_thresholds: HashMap<String, u8>,
    pub rate_limiting: RateLimitConfig,
}

#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct NotificationChannel {
    pub channel_type: String,
    pub endpoint: String,
    pub enabled: bool,
    pub severity_filter: Vec<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct RateLimitConfig {
    pub max_alerts_per_minute: u32,
    pub burst_threshold: u32,
    pub cooldown_period: Duration,
}

#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct ResponseConfig {
    pub auto_response_enabled: bool,
    pub response_actions: Vec<String>,
    pub escalation_rules: Vec<EscalationRule>,
}

#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct EscalationRule {
    pub condition: String,
    pub action: String,
    pub delay: Duration,
}

/// Unified Prometheus configuration
#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct PrometheusConfigUnified {
    pub endpoint: String,
    pub scrape_interval: Duration,
    pub metrics_path: String,
    pub labels: HashMap<String, String>,
    pub authentication: Option<AuthenticationConfig>,
}

#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct AuthenticationConfig {
    pub auth_type: String,
    pub credentials: HashMap<String, String>,
}

/// Unified metric collection configuration
#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct MetricCollectionConfigUnified {
    pub collection_interval: Duration,
    pub batch_size: usize,
    pub retention_period: Duration,
    pub storage_backend: StorageBackend,
    pub metric_filters: Vec<MetricFilter>,
}

#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct StorageBackend {
    pub backend_type: String,
    pub connection_string: String,
    pub compression_enabled: bool,
}

#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct MetricFilter {
    pub metric_name: String,
    pub enabled: bool,
    pub sampling_rate: f64,
}

/// Unified monitoring configuration
#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct MonitoringConfigUnified {
    pub prometheus: PrometheusConfigUnified,
    pub metrics: MetricCollectionConfigUnified,
    pub security_sentinel: SecuritySentinelConfig,
    pub health_checks: crate::canonical::monitoring::HealthCheckConfig,
    pub logging: LoggingConfig,
}

// Re-export the canonical HealthCheckConfig from monitoring module
pub use crate::canonical::monitoring::{HealthCheckConfig, HealthCheckEndpoint};

#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct LoggingConfig {
    pub level: String,
    pub format: String,
    pub outputs: Vec<LogOutput>,
    pub structured_logging: bool,
}

#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct LogOutput {
    pub output_type: String,
    pub destination: String,
    pub rotation_policy: Option<RotationPolicy>,
}

#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct RotationPolicy {
    pub max_size: u64,
    pub max_age: Duration,
    pub max_files: u32,
}
