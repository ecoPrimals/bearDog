// Module documentation
//
// This module provides functionality for the BearDog ecosystem.

use serde::{Deserialize, Serialize};
use std::time::Duration;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct KeyManagerMetrics {
    /// Number of `total_keys`
    pub total_keys: usize,
    /// Number of `keys_created`
    pub keys_created: u64,
    /// Number of `keys_accessed`
    pub keys_accessed: u64,
    /// Number of `keys_expired`
    pub keys_expired: u64,
    pub avg_access_time: Duration,
    /// The cache hit rate value
    pub cache_hit_rate: f64,
    /// Number of `memory_usage`
    pub memory_usage: usize,
}

impl Default for KeyManagerMetrics {
    fn default() -> Self {
        Self {
            total_keys: 0,
            keys_created: 0,
            keys_accessed: 0,
            keys_expired: 0,
            avg_access_time: Duration::from_millis(0),
            cache_hit_rate: 0.0,
            memory_usage: 0,
        }
    }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct OperationMetrics {
    /// The operation type value
    pub operation_type: String,
    /// The duration value
    pub duration: Duration,
    /// Whether success is enabled
    pub success: bool,
    /// Optional error
    pub error: Option<String>,
}

/// Extended metrics with recent operations
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ExtendedMetrics {
    /// The base metrics value
    pub base_metrics: KeyManagerMetrics,
    /// Collection of recent operations
    pub recent_operations: Vec<OperationMetrics>,
    pub timestamp: chrono::DateTime<chrono::Utc>,
}
