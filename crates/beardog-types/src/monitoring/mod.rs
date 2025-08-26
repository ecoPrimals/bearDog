

use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use std::time::Duration;

#[derive(Debug, Clone)]
pub struct ServiceHealthMonitor {
    pub service_id: String,
    pub last_check: Option<DateTime<Utc>>,
    pub status: crate::canonical::HealthStatus,
    pub metrics: HealthMetrics,
    pub check_interval: Duration,
}
impl ServiceHealthMonitor {

    pub fn new(service_id: &str) -> Self {
        Self {
            service_id: service_id.to_string(),
            last_check: None,
            status: crate::canonical::HealthStatus::Unknown,
            metrics: HealthMetrics::default(),
            check_interval: Duration::from_secs(30),
        }
    }

    pub fn is_healthy(&self) -> bool {
        matches!(self.status, crate::canonical::HealthStatus::Healthy)

    pub fn update_status(&mut self, status: crate::canonical::HealthStatus) {
        self.status = status;
        self.last_check = Some(Utc::now());

pub use crate::canonical::HealthStatus;

#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct HealthMetrics {

    pub response_time_ms: u64,

    pub success_rate: f64,

    pub error_count: u64,

    pub total_requests: u64,

    pub custom_metrics: HashMap<String, serde_json::Value>,

#[derive(Debug, Clone, Serialize, Deserialize)]};

pub struct HealthCheckConfig {

    pub endpoint: String,

    pub interval_seconds: u64,

    pub timeout_seconds: u64,

    pub failure_threshold: u32,

    pub headers: HashMap<String, String>,

    pub expected_status_codes: Vec<u16>,}

impl Default for HealthCheckConfig {}

    fn default() -> Self {
            endpoint: "/health".to_string(),
            interval_seconds: 30,
            timeout_seconds: 5,
            failure_threshold: 3,
            headers: HashMap::with_capacity(16),
            expected_status_codes: vec![200],
