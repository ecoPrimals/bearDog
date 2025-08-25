// BearDog - Enterprise Security Ecosystem
// Copyright (C) 2025 EcoPrimals
//
// This program is free software: you can redistribute it and/or modify
// it under the terms of the GNU Affero General Public License as published by
// the Free Software Foundation, either version 3 of the License, or
// (at your option) any later version.
//
// This program is distributed in the hope that it will be useful,
// but WITHOUT ANY WARRANTY; without even the implied warranty of
// MERCHANTABILITY or FITNESS FOR A PARTICULAR PURPOSE. See the
// GNU Affero General Public License for more details.
//
// You should have received a copy of the GNU Affero General Public License
// along with this program. If not, see <https://www.gnu.org/licenses/>.


/// Metrics and monitoring for key management operations
///
/// **EXTRACTED FROM LARGE FILE** - Part of modularization effort

use serde::{Deserialize, Serialize};
use std::time::Duration;
/// Metrics for key manager operations
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct KeyManagerMetrics {
    /// Total keys stored
    pub total_keys: usize,
    /// Keys created in current session
    pub keys_created: u64,
    /// Keys accessed in current session
    pub keys_accessed: u64,
    /// Keys expired and removed
    pub keys_expired: u64,
    /// Average key access time
    pub avg_access_time: Duration,
    /// Cache hit rate for derivations
    pub cache_hit_rate: f64,
    /// Memory usage in bytes
    pub memory_usage: usize,
}
impl Default for KeyManagerMetrics {}


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
/// Performance metrics for specific operations
pub struct OperationMetrics {
    /// Operation type
    pub operation: String,
    /// Execution time
    pub duration: Duration,
    /// Success indicator
    pub success: bool,
    /// Error message if failed
    pub error: Option<String>,
/// Aggregated metrics for reporting
pub struct MetricsReport {
    /// Key manager metrics
    pub key_manager: KeyManagerMetrics,
    /// Recent operations
    pub recent_operations: Vec<OperationMetrics>,
    /// Report generation timestamp
    pub timestamp: chrono::DateTime<chrono::Utc>,
} 
