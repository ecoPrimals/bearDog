use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};
use std::collections::HashMap;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ComponentHealth {
    pub name: String,

    pub status: beardog_types::canonical::health_status::HealthStatus,

    pub message: Option<String>,

    pub last_check: DateTime<Utc>,

    pub check_duration_ms: u64,

    pub metadata: HashMap<String, String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SystemHealth {
    pub status: beardog_types::canonical::health_status::HealthStatus,

    pub components: Vec<ComponentHealth>,

    pub last_check: DateTime<Utc>,

    pub summary: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ResourceMetrics {
    pub cpu_usage_percent: f64,

    pub memory_usage_bytes: u64,

    pub memory_total_bytes: u64,

    pub disk_usage_bytes: u64,

    pub disk_total_bytes: u64,

    pub network_bytes_sent: u64,

    pub network_bytes_received: u64,

    pub active_connections: u32,

    pub uptime_seconds: u64,
}

impl Default for ResourceMetrics {
    fn default() -> Self {
        Self {
            cpu_usage_percent: 0.0,
            memory_usage_bytes: 0,
            memory_total_bytes: 0,
            disk_usage_bytes: 0,
            disk_total_bytes: 0,
            network_bytes_sent: 0,
            network_bytes_received: 0,
            active_connections: 0,
            uptime_seconds: 0,
        }
    }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PerformanceMetrics {
    pub avg_request_time_ms: f64,

    pub requests_per_second: f64,

    pub total_requests: u64,

    pub failed_requests: u64,

    pub error_rate_percent: f64,

    pub p95_response_time_ms: f64,

    pub p99_response_time_ms: f64,
}

impl Default for PerformanceMetrics {
    fn default() -> Self {
        Self {
            avg_request_time_ms: 0.0,
            requests_per_second: 0.0,
            total_requests: 0,
            failed_requests: 0,
            error_rate_percent: 0.0,
            p95_response_time_ms: 0.0,
            p99_response_time_ms: 0.0,
        }
    }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SystemMetrics {
    pub timestamp: DateTime<Utc>,

    pub resources: ResourceMetrics,

    pub performance: PerformanceMetrics,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct AlertThresholds {
    pub cpu_threshold: f64,

    pub memory_threshold: f64,

    pub disk_threshold: f64,

    pub error_rate_threshold: f64,

    pub response_time_threshold: f64,
}

impl Default for AlertThresholds {
    fn default() -> Self {
        Self {
            cpu_threshold: 80.0,
            memory_threshold: 85.0,
            disk_threshold: 90.0,
            error_rate_threshold: 5.0,
            response_time_threshold: 1000.0,
        }
    }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct InternalMetricsSummary {
    pub security_events: u64,

    pub encryption_operations: u64,

    pub threat_detections: u64,

    pub compliance_checks: u64,

    pub api_requests: u64,

    pub error_count: u64,

    pub active_sessions: u64,

    pub last_updated: DateTime<Utc>,
}

// UNIFIED: Use canonical PrometheusConfig
pub use beardog_types::canonical::monitoring::PrometheusConfig;

// Default implementation moved to canonical PrometheusConfig in beardog-types

#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum MetricValue {
    Counter(u64),

    Gauge(f64),

    Histogram(Vec<f64>),

    Summary { sum: f64, count: u64 },

    Label(String),
}

impl MetricValue {
    pub fn as_f64(&self) -> Option<f64> {
        match self {
            MetricValue::Counter(v) => Some(*v as f64),
            MetricValue::Gauge(v) => Some(*v),
            MetricValue::Summary { sum, count: _ } => Some(*sum),
            MetricValue::Histogram(values) => {
                if values.is_empty() {
                    Some(0.0)
                } else {
                    Some(values.iter().sum::<f64>() / values.len() as f64)
                }
            }
            MetricValue::Label(_) => None,
        }
    }
}
