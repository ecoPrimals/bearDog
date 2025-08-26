

use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use std::time::Duration;

#[derive(Debug, Clone, Serialize, Deserialize)]
#[derive(Default)]
pub struct UnifiedMonitoringConfig {

    pub health_checks: HealthCheckConfig,

    pub metrics: MetricCollectionConfig,

    pub alerts: AlertProcessingConfig,

    pub prometheus: PrometheusConfig,

    pub security_monitoring: SecurityMonitoringConfig,

    pub performance_monitoring: PerformanceMonitoringConfig,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct HealthCheckConfig {

    pub interval: Duration,

    pub timeout: Duration,

    pub failure_threshold: u32,

    pub success_threshold: u32,

    pub endpoints: Vec<String>,

    pub custom_checks: HashMap<String, serde_json::Value>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct MetricCollectionConfig {

    pub collection_interval: Duration,

    pub retention_period: Duration,

    pub max_metrics: u64,

    pub aggregation: MetricAggregationConfig,

    pub custom_collectors: HashMap<String, serde_json::Value>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct AlertProcessingConfig {

    pub processing_interval: Duration,

    pub escalation_timeout: Duration,

    pub max_batch_size: u32,

    pub routing_rules: Vec<AlertRoutingRule>,

    pub notification_channels: HashMap<String, NotificationChannelConfig>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PrometheusConfig {

    pub endpoint: String,

    pub scrape_interval: Duration,

    pub query_timeout: Duration,

    pub retention_period: Duration,

    pub custom_config: HashMap<String, serde_json::Value>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SecurityMonitoringConfig {

    pub enabled: bool,

    pub collection_interval: Duration,

    pub alert_thresholds: HashMap<String, f64>,

    pub threat_detection: ThreatDetectionConfig,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PerformanceMonitoringConfig {

    pub enabled: bool,

    pub collection_interval: Duration,

    pub baseline_config: PerformanceBaselineConfig,

    pub alert_thresholds: HashMap<String, f64>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct MetricAggregationConfig {

    pub method: String,

    pub window_size: Duration,

    pub enable_downsampling: bool,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct AlertRoutingRule {

    pub name: String,

    pub condition: String,

    pub channel: String,

    pub priority: u32,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct NotificationChannelConfig {

    pub channel_type: String,

    pub endpoint: String,

    pub config: HashMap<String, serde_json::Value>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ThreatDetectionConfig {

    pub enable_ml_detection: bool,

    pub sensitivity: f64,

    pub rules: Vec<ThreatDetectionRule>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PerformanceBaselineConfig {

    pub calculation_window: Duration,

    pub update_interval: Duration,

    pub deviation_threshold: f64,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ThreatDetectionRule {

    pub name: String,

    pub pattern: String,

    pub severity: String,

    pub action: String,
}

impl Default for HealthCheckConfig {
    fn default() -> Self {
        Self {
            interval: Duration::from_secs(30),
            timeout: Duration::from_secs(5),
            failure_threshold: 3,
            success_threshold: 2,
            endpoints: vec!["/health".to_string()],
            custom_checks: HashMap::with_capacity(16),
        }
    }
}

impl Default for MetricCollectionConfig {
    fn default() -> Self {
        Self {
            collection_interval: Duration::from_secs(60),
            retention_period: Duration::from_secs(86400), // 24 hours
            max_metrics: 100000,
            aggregation: MetricAggregationConfig::default(),
            custom_collectors: HashMap::with_capacity(16),
        }
    }
}

impl Default for AlertProcessingConfig {
    fn default() -> Self {
        Self {
            processing_interval: Duration::from_secs(30),
            escalation_timeout: Duration::from_secs(300), // 5 minutes
            max_batch_size: 100,
            routing_rules: Vec::new(),
            notification_channels: HashMap::with_capacity(16),
        }
    }
}

impl Default for PrometheusConfig {
    fn default() -> Self {
        Self {
            endpoint: "http://localhost:9090".to_string(),
            scrape_interval: Duration::from_secs(15),
            query_timeout: Duration::from_secs(30),
            retention_period: Duration::from_secs(86400 * 7), // 7 days
            custom_config: HashMap::with_capacity(16),
        }
    }
}

impl Default for SecurityMonitoringConfig {
    fn default() -> Self {
        Self {
            enabled: true,
            collection_interval: Duration::from_secs(10),
            alert_thresholds: HashMap::with_capacity(16),
            threat_detection: ThreatDetectionConfig::default(),
        }
    }
}

impl Default for PerformanceMonitoringConfig {
    fn default() -> Self {
        Self {
            enabled: true,
            collection_interval: Duration::from_secs(60),
            baseline_config: PerformanceBaselineConfig::default(),
            alert_thresholds: HashMap::with_capacity(16),
        }
    }
}

impl Default for MetricAggregationConfig {
    fn default() -> Self {
        Self {
            method: "avg".to_string(),
            window_size: Duration::from_secs(300), // 5 minutes
            enable_downsampling: true,
        }
    }
}

impl Default for ThreatDetectionConfig {
    fn default() -> Self {
        Self {
            enable_ml_detection: true,
            sensitivity: 0.8,
            rules: Vec::new(),
        }
    }
}

impl Default for PerformanceBaselineConfig {
    fn default() -> Self {
        Self {
            calculation_window: Duration::from_secs(3600), // 1 hour
            update_interval: Duration::from_secs(86400), // 24 hours
            deviation_threshold: 2.0, // 2 standard deviations
        }
    }
} 