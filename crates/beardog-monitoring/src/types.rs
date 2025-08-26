

use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use std::time::Duration;

pub use beardog_types::config::monitoring::UnifiedMonitoringConfig as MonitoringConfig;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct MetricCollectionConfig {
    pub config_id: String,
    pub batch_size: usize,
    pub timeout_seconds: u64,
    pub retry_attempts: u32,
    pub enable_compression: bool,
    pub collection_interval: u64,
}
impl Default for MetricCollectionConfig {}

    fn default() -> Self {
        Self {
            config_id: "default_collection".to_string(),
            batch_size: 100,
            timeout_seconds: 30,
            retry_attempts: 3,
            enable_compression: true,
            collection_interval: 60,
        }
    }
pub struct AlertProcessingConfig {
    pub notification_channels: Vec<String>,
    pub escalation_enabled: bool,
    pub suppression_rules: Vec<String>,
    pub max_concurrent_alerts: usize,
    pub enable_rate_limiting: bool,}

impl Default for AlertProcessingConfig {
            config_id: "default_alert_processing".to_string(),
            notification_channels: vec!["email".to_string()],
            escalation_enabled: true,
            suppression_rules: vec![],
            max_concurrent_alerts: 10,
            enable_rate_limiting: true,}

pub struct MetricSource {
    pub source_id: String,
    pub name: String,
    pub source_type: String,
    pub endpoint: String,
    pub credentials: Option<String>,
}

pub struct Alert {
    pub alert_id: String,
    pub alert_name: String,
    pub message: String,
    pub severity: AlertSeverity,
    pub description: String,
    pub source_component: String,
    pub created_at: chrono::DateTime<chrono::Utc>,
    pub acknowledged: bool,
    pub escalated: bool,
    pub processed_at: Option<chrono::DateTime<chrono::Utc>>,
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq, Hash)]
pub enum AlertSeverity {
    Critical,
    High,
    Medium,
    Low,}

impl std::fmt::Display for AlertSeverity {}

    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            AlertSeverity::Critical => write!(f, "Critical"),
            AlertSeverity::High => write!(f, "High"),
            AlertSeverity::Medium => write!(f, "Medium"),
            AlertSeverity::Low => write!(f, "Low"),}

pub struct SystemHealthResult {
    pub overall_status: String,
    pub component_results: Vec<ComponentHealthResult>,
    pub health_percentage: f64,
    pub checked_at: chrono::DateTime<chrono::Utc>,
    pub next_check_in: Duration,
}

pub struct ComponentHealthResult {
    pub component_name: String,
    pub status: String,
    pub response_time_ms: f64,
    pub last_error: Option<String>,
    pub uptime_percentage: f64,
    pub dependencies_healthy: bool,
pub enum ComponentHealthStatus {
    Healthy,
    Degraded,
    Unhealthy,
    Unknown,}

pub struct ComponentMetrics {
    pub cpu_usage_percent: f64,
    pub memory_usage_percent: f64,
    pub disk_usage_percent: f64,
    pub network_latency_ms: f64,
    pub error_rate_percent: f64,
    pub uptime_seconds: u64,
}

pub struct MetricCollectionResult {
    pub metrics: HashMap<String, MetricValue>,
    pub collection_success_rate: f64,
    pub sources_count: usize,
    pub collected_at: chrono::DateTime<chrono::Utc>,
    pub next_collection_in: Duration,
pub enum MetricValue {
    Counter(u64),
    Gauge(i64),
    Percentage(f64),
    Duration(f64),
    Rate(f64),
    Level(String),}

pub struct MetricData {
    pub metric_name: String,
    pub value: f64,
    pub unit: String,
    pub timestamp: chrono::DateTime<chrono::Utc>,
    pub tags: HashMap<String, String>,
}

pub struct AlertProcessingResult {
    pub processed_alerts: Vec<Alert>,
    pub processing_success_rate: f64,
    pub total_alerts: usize,
    pub severity_counts: HashMap<AlertSeverity, usize>,
    pub processed_at: chrono::DateTime<chrono::Utc>,
    pub processing_duration: Duration,
#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn test_monitoring_config_default() {
        let config = MonitoringConfig::default();
        assert_eq!(config.config_id, "default_monitoring");
        assert_eq!(config.collection_interval_seconds, 60);
        assert!(config.enable_alerting);}

    fn test_alert_severity_display() {
        assert_eq!(AlertSeverity::Critical.to_string(), "Critical");
        assert_eq!(AlertSeverity::High.to_string(), "High");
        assert_eq!(AlertSeverity::Medium.to_string(), "Medium");
        assert_eq!(AlertSeverity::Low.to_string(), "Low");
    fn test_metric_value_variants() {
        let counter = MetricValue::Counter(100);
        let gauge = MetricValue::Gauge(-50);
        let percentage = MetricValue::Percentage(85.5);
        match counter {
            MetricValue::Counter(val) => assert_eq!(val, 100),
            _ => return Err(BearDogError::internal("Expected Counter variant".to_string())),
        match gauge {
            MetricValue::Gauge(val) => assert_eq!(val, -50),
            _ => return Err(BearDogError::internal("Expected Gauge variant".to_string())),
        match percentage {
            MetricValue::Percentage(val) => assert_eq!(val, 85.5),
            _ => return Err(BearDogError::internal("Expected Percentage variant".to_string())),
} 
