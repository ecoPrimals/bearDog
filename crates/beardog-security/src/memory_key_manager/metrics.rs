use serde::{Deserialize, Serialize};
use std::time::Duration;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct KeyManagerMetrics {
    pub total_keys: usize,
    pub keys_created: u64,
    pub keys_accessed: u64,
    pub keys_expired: u64,
    pub avg_access_time: Duration,
    pub cache_hit_rate: f64,
    pub memory_usage: usize,
}

impl Default for KeyManagerMetrics {
    fn default() -> Self {
        Self {
            total_keys: 0,
            keys_created: 0,
            keys_accessed: 0,
            keys_expired: 0,
            avg_access_time: Duration::from_millis(1),
            cache_hit_rate: 0.0,
            memory_usage: 0,
        }
    }
}

#[derive(Debug, Clone)]
pub struct OperationMetrics {
    pub operation: String,
    pub duration: Duration,
    pub success: bool,
    pub error: Option<String>,
}

#[derive(Debug, Clone)]
pub struct MetricsReport {
    pub key_manager: KeyManagerMetrics,
    pub recent_operations: Vec<OperationMetrics>,
    pub timestamp: chrono::DateTime<chrono::Utc>,
}
