// SPDX-License-Identifier: AGPL-3.0-or-later

//! Metrics for memory-based key manager
//!
//! This module provides metrics and monitoring structures for key management operations.

use serde::{Deserialize, Serialize};
use std::time::Duration;

/// Key manager metrics structure
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct KeyManagerMetrics {
    /// Total number of keys currently stored
    pub total_keys: usize,
    /// Total number of keys created
    pub keys_created: u64,
    /// Total number of key access operations
    pub keys_accessed: u64,
    /// Total number of keys that have expired
    pub keys_expired: u64,
    /// Average time to access a key
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

/// Operation-level metrics
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct OperationMetrics {
    /// Type of operation performed
    pub operation_type: String,
    /// Duration of the operation
    pub duration: Duration,
    /// Whether the operation succeeded
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
    /// Timestamp when these metrics were captured
    pub timestamp: chrono::DateTime<chrono::Utc>,
}
