// SPDX-License-Identifier: AGPL-3.0-or-later

//! HSM Performance Tracking
//!
//! Tracks and analyzes HSM provider performance metrics.

use std::collections::HashMap;
use std::sync::Arc;
use std::time::Duration;
use tokio::sync::RwLock;
use tracing::{debug, info};

/// Operation performance metrics
#[derive(Debug, Clone)]
pub struct OperationMetrics {
    /// Total operations performed
    pub total_operations: u64,
    /// Successful operations
    pub successful_operations: u64,
    /// Failed operations
    pub failed_operations: u64,
    /// Average latency in milliseconds
    pub average_latency_ms: f64,
    /// Minimum latency in milliseconds
    pub min_latency_ms: f64,
    /// Maximum latency in milliseconds
    pub max_latency_ms: f64,
    /// Last operation timestamp
    pub last_operation_time: chrono::DateTime<chrono::Utc>,
}

impl Default for OperationMetrics {
    fn default() -> Self {
        Self {
            total_operations: 0,
            successful_operations: 0,
            failed_operations: 0,
            average_latency_ms: 0.0,
            min_latency_ms: f64::MAX,
            max_latency_ms: 0.0,
            last_operation_time: chrono::Utc::now(),
        }
    }
}

impl OperationMetrics {
    /// Calculate success rate
    pub fn success_rate(&self) -> f64 {
        if self.total_operations == 0 {
            0.0
        } else {
            #[expect(
                clippy::cast_precision_loss,
                reason = "percentage from operation counters"
            )]
            let succ = self.successful_operations as f64;
            #[expect(
                clippy::cast_precision_loss,
                reason = "percentage from operation counters"
            )]
            let tot = self.total_operations as f64;
            (succ / tot) * 100.0
        }
    }

    /// Calculate failure rate
    pub fn failure_rate(&self) -> f64 {
        100.0 - self.success_rate()
    }
}

/// Performance configuration
#[derive(Debug, Clone)]
pub struct PerformanceConfig {
    /// Enable performance tracking
    pub enabled: bool,
    /// Metrics retention period
    pub retention_period: Duration,
    /// Sample rate (0.0-1.0)
    pub sample_rate: f64,
}

impl Default for PerformanceConfig {
    fn default() -> Self {
        Self {
            enabled: true,
            retention_period: Duration::from_secs(3600), // 1 hour
            sample_rate: 1.0,                            // Track all operations
        }
    }
}

/// HSM performance tracker
pub struct HsmPerformanceTracker {
    /// Operation metrics per provider
    operation_metrics: Arc<RwLock<HashMap<String, OperationMetrics>>>,
    /// Performance configuration
    config: PerformanceConfig,
}

impl HsmPerformanceTracker {
    /// Create a new performance tracker
    pub fn new() -> Self {
        Self {
            operation_metrics: Arc::new(RwLock::new(HashMap::new())),
            config: PerformanceConfig::default(),
        }
    }

    /// Create with custom configuration
    pub fn with_config(config: PerformanceConfig) -> Self {
        Self {
            operation_metrics: Arc::new(RwLock::new(HashMap::new())),
            config,
        }
    }

    /// Record an operation
    pub async fn record_operation(&self, provider_id: String, success: bool, latency_ms: f64) {
        if !self.config.enabled {
            return;
        }

        let mut metrics = self.operation_metrics.write().await;
        let provider_metrics = metrics.entry(provider_id.clone()).or_default();

        provider_metrics.total_operations += 1;
        if success {
            provider_metrics.successful_operations += 1;
        } else {
            provider_metrics.failed_operations += 1;
        }

        // Update latency stats
        #[expect(
            clippy::cast_precision_loss,
            reason = "rolling average over operation count"
        )]
        let total_ops = provider_metrics.total_operations as f64;
        provider_metrics.average_latency_ms = provider_metrics
            .average_latency_ms
            .mul_add(total_ops - 1.0, latency_ms)
            / total_ops;

        if latency_ms < provider_metrics.min_latency_ms {
            provider_metrics.min_latency_ms = latency_ms;
        }
        if latency_ms > provider_metrics.max_latency_ms {
            provider_metrics.max_latency_ms = latency_ms;
        }

        provider_metrics.last_operation_time = chrono::Utc::now();

        debug!(
            "📊 Recorded operation for {}: success={}, latency={}ms",
            provider_id, success, latency_ms
        );
    }

    /// Get metrics for a provider
    pub async fn get_metrics(&self, provider_id: &str) -> Option<OperationMetrics> {
        let metrics = self.operation_metrics.read().await;
        metrics.get(provider_id).cloned()
    }

    /// Get all metrics
    pub async fn get_all_metrics(&self) -> HashMap<String, OperationMetrics> {
        self.operation_metrics.read().await.clone()
    }

    /// Reset metrics for a provider
    pub async fn reset_metrics(&self, provider_id: &str) {
        let mut metrics = self.operation_metrics.write().await;
        metrics.remove(provider_id);
        info!("🔄 Reset performance metrics for {}", provider_id);
    }

    /// Get configuration
    pub const fn config(&self) -> &PerformanceConfig {
        &self.config
    }
}

impl Default for HsmPerformanceTracker {
    fn default() -> Self {
        Self::new()
    }
}

#[cfg(test)]
#[path = "performance_tests.rs"]
mod tests;
