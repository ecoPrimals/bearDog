

use serde::{Deserialize, Serialize};
use std::time::Duration;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct IntegrationMonitoringConfig {
    pub enabled: bool,
    pub collection_interval: Duration,
    pub retention_period: Duration,
    pub alert_thresholds: AlertThresholds,
}

impl Default for IntegrationMonitoringConfig {
    fn default() -> Self {
        Self {
            enabled: true,
            collection_interval: Duration::from_secs(60),
            retention_period: Duration::from_secs(86400 * 7), // 7 days
            alert_thresholds: AlertThresholds::default(),
        }
    }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PerformanceThresholds {
    pub response_time_ms: u32,
    pub error_rate_percent: f64,
    pub throughput_per_second: u32,
}

impl Default for PerformanceThresholds {
    fn default() -> Self {
        Self {
            response_time_ms: 1000,
            error_rate_percent: 5.0,
            throughput_per_second: 100,
        }
    }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct IntegrationAuditConfig {
    pub enabled: bool,
    pub log_requests: bool,
    pub include_payload: bool,
}

impl Default for IntegrationAuditConfig {
    fn default() -> Self {
        Self {
            enabled: true,
            log_requests: true,
            include_payload: false,
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
