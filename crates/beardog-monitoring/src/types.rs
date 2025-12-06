// Module documentation
//
// This module provides functionality for the BearDog ecosystem.


use beardog_errors::BearDogError;
use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use std::time::Duration;

pub use beardog_types::canonical::monitoring::{
    MonitoringConfig, MetricsConfig, AlertConfig, AlertSeverity,
    HealthCheckConfig, PrometheusConfig
};

#[derive(Debug, Clone)]
    /// Number of batch_size
    pub batch_size: usize,
    pub timeout_seconds: u64,
    /// Number of retry_attempts
    pub retry_attempts: u32,
    /// Whether enable_compression is enabled
    pub enable_compression: bool,
    /// Number of collection_interval
    pub collection_interval: u64,
}

impl Default for MetricCollectionConfig {
    fn default() -> Self {
        Self {
            config_id: "default_collection".to_string()]
mod tests {
#[allow(unused_imports, clippy::float_cmp, clippy::absurd_extreme_comparisons, unused_comparisons, clippy::nonminimal_bool)]
    #[cfg(test)]
    use super::*;
    // TEST_CATEGORY: unit
    // TEST_DOMAIN: monitoring
    // TEST_PRIORITY: normal
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
