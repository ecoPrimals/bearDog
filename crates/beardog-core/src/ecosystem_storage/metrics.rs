// SPDX-License-Identifier: AGPL-3.0-or-later

// Storage Metrics Module

use super::types::StorageOperationMetrics;
use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};
use std::collections::HashMap;

/// Storage system metrics
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct StorageMetrics {
    /// Total storage capacity in bytes
    /// Number of `total_capacity_bytes`
    pub total_capacity_bytes: u64,
    /// Used storage space in bytes
    /// Number of `used_space_bytes`
    pub used_space_bytes: u64,
    /// Available storage space in bytes
    /// Number of `available_space_bytes`
    pub available_space_bytes: u64,
    /// Storage utilization percentage (0.0 to 1.0)
    /// The utilization percentage value
    pub utilization_percentage: f64,
    /// Operation metrics by operation type
    /// Mapping of operation metrics
    pub operation_metrics: HashMap<String, StorageOperationMetrics>,
    /// Total number of stored items
    /// Number of `total_items`
    pub total_items: u64,
    /// Average item size in bytes
    /// The avg item size bytes value
    pub avg_item_size_bytes: f64,
    /// Metrics collection timestamp
    pub timestamp: DateTime<Utc>,
}

impl Default for StorageMetrics {
    fn default() -> Self {
        Self {
            total_capacity_bytes: 0,
            used_space_bytes: 0,
            available_space_bytes: 0,
            utilization_percentage: 0.0,
            operation_metrics: HashMap::new(),
            total_items: 0,
            avg_item_size_bytes: 0.0,
            timestamp: Utc::now(),
        }
    }
}
