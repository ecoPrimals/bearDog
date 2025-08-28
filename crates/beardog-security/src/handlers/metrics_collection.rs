use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};

pub struct BearDogSecurityProvider {
    metrics: SecurityProviderMetrics,
}

impl BearDogSecurityProvider {
    pub fn new() -> Self {
        Self {
            metrics: SecurityProviderMetrics::default(),
        }
    }

    pub async fn get_current_metrics(&self) -> SecurityProviderMetrics {
        self.metrics.clone()
    }

    #[allow(dead_code)]
    pub async fn update_auth_metrics(&mut self, success: bool, _duration_ms: u64) {
        if success {
            self.metrics.auth_success_count += 1;
        } else {
            self.metrics.auth_failure_count += 1;
        }
    }

    #[allow(dead_code)]
    pub async fn reset_metrics(&mut self) {
        self.metrics = SecurityProviderMetrics::default();
    }
}

impl Default for BearDogSecurityProvider {
    fn default() -> Self {
        Self::new()
    }
}

#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct SecurityProviderMetrics {
    pub auth_success_count: u64,
    pub auth_failure_count: u64,
    pub auth_success_rate: f64,
    pub authz_success_rate: f64,
    pub error_rate: f64,
    pub active_sessions: u64,
    pub high_risk_operations: u64,
    pub avg_response_time_ms: f64,
    pub last_check: DateTime<Utc>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SecurityHealth {
    pub status: String,
    pub score: f64,
    pub last_check: DateTime<Utc>,
}
