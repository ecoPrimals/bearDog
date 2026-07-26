// SPDX-License-Identifier: AGPL-3.0-or-later

//! HSM Performance Tracking
//!
//! This module provides performance monitoring and metrics for HSM operations.

use parking_lot::RwLock;
use std::collections::HashMap;
use std::sync::Arc;
use tracing::{debug, info};

/// HSM performance tracker
pub struct HsmPerformanceTracker {
    operation_metrics: Arc<RwLock<HashMap<String, OperationMetrics>>>,
}

/// Metrics for a single operation
#[derive(Debug, Clone)]
pub struct OperationMetrics {
    /// Total number of times this operation was invoked
    pub total_count: u64,
    /// Number of successful completions
    pub success_count: u64,
    /// Number of failures
    pub failure_count: u64,
    /// Total cumulative duration in milliseconds
    pub total_duration_ms: u64,
    /// Average duration per operation in milliseconds
    pub avg_duration_ms: f64,
    /// Fastest operation duration in milliseconds
    pub min_duration_ms: u64,
    /// Slowest operation duration in milliseconds
    pub max_duration_ms: u64,
}

impl Default for OperationMetrics {
    fn default() -> Self {
        Self {
            total_count: 0,
            success_count: 0,
            failure_count: 0,
            total_duration_ms: 0,
            avg_duration_ms: 0.0,
            min_duration_ms: u64::MAX,
            max_duration_ms: 0,
        }
    }
}

impl HsmPerformanceTracker {
    /// Creates a new performance tracker
    pub fn new() -> Self {
        info!("📊 Initializing HSM performance tracker");
        Self {
            operation_metrics: Arc::new(RwLock::new(HashMap::new())),
        }
    }

    /// Records operation metrics
    pub fn record_operation(&self, operation_name: &str, duration_ms: u64, success: bool) {
        let mut metrics_map = self.operation_metrics.write();
        let metrics = metrics_map.entry(operation_name.to_string()).or_default();

        metrics.total_count += 1;
        if success {
            metrics.success_count += 1;
        } else {
            metrics.failure_count += 1;
        }

        metrics.total_duration_ms += duration_ms;
        #[expect(
            clippy::cast_precision_loss,
            reason = "mean duration from integer accumulators"
        )]
        let total_ms = metrics.total_duration_ms as f64;
        #[expect(
            clippy::cast_precision_loss,
            reason = "mean duration from integer accumulators"
        )]
        let count = metrics.total_count as f64;
        metrics.avg_duration_ms = total_ms / count;
        metrics.min_duration_ms = metrics.min_duration_ms.min(duration_ms);
        metrics.max_duration_ms = metrics.max_duration_ms.max(duration_ms);

        debug!(
            "📈 Recorded operation '{}': {}ms (success={})",
            operation_name, duration_ms, success
        );
    }
    /// Gets metrics for a specific operation
    #[must_use]
    pub fn get_metrics(&self, operation_name: &str) -> Option<OperationMetrics> {
        let metrics_map = self.operation_metrics.read();
        metrics_map.get(operation_name).cloned()
    }
    /// Gets all metrics
    #[must_use]
    pub fn get_all_metrics(&self) -> HashMap<String, OperationMetrics> {
        self.operation_metrics.read().clone()
    }

    /// Resets all metrics
    pub fn reset(&self) {
        let mut metrics_map = self.operation_metrics.write();
        metrics_map.clear();
        info!("🔄 Reset all HSM performance metrics");
    }
}

impl Default for HsmPerformanceTracker {
    fn default() -> Self {
        Self::new()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_tracker_creation() {
        let tracker = HsmPerformanceTracker::new();
        assert!(tracker.get_all_metrics().is_empty());
    }

    #[test]
    fn test_record_operation() {
        let tracker = HsmPerformanceTracker::new();
        tracker.record_operation("test_op", 100, true);

        let metrics = tracker.get_metrics("test_op").unwrap();
        assert_eq!(metrics.total_count, 1);
        assert_eq!(metrics.success_count, 1);
    }

    #[test]
    fn test_metrics_calculation() {
        let tracker = HsmPerformanceTracker::new();
        tracker.record_operation("test", 100, true);
        tracker.record_operation("test", 200, true);

        let metrics = tracker.get_metrics("test").unwrap();
        assert_eq!(metrics.avg_duration_ms, 150.0);
        assert_eq!(metrics.min_duration_ms, 100);
        assert_eq!(metrics.max_duration_ms, 200);
    }

    #[test]
    fn test_reset() {
        let tracker = HsmPerformanceTracker::new();
        tracker.record_operation("test", 100, true);
        tracker.reset();

        assert!(tracker.get_all_metrics().is_empty());
    }
}
