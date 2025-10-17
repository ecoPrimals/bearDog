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
            (self.successful_operations as f64 / self.total_operations as f64) * 100.0
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
        let total_ops = provider_metrics.total_operations as f64;
        provider_metrics.average_latency_ms =
            ((provider_metrics.average_latency_ms * (total_ops - 1.0)) + latency_ms) / total_ops;

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
    pub fn config(&self) -> &PerformanceConfig {
        &self.config
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
    fn test_operation_metrics_default() {
        let metrics = OperationMetrics::default();
        assert_eq!(metrics.total_operations, 0);
        assert_eq!(metrics.success_rate(), 0.0);
    }

    #[test]
    fn test_operation_metrics_success_rate() {
        let metrics = OperationMetrics {
            total_operations: 100,
            successful_operations: 95,
            failed_operations: 5,
            ..Default::default()
        };
        assert_eq!(metrics.success_rate(), 95.0);
        assert_eq!(metrics.failure_rate(), 5.0);
    }

    #[tokio::test]
    async fn test_tracker_creation() {
        let tracker = HsmPerformanceTracker::new();
        let metrics = tracker.get_all_metrics().await;
        assert!(metrics.is_empty());
    }

    #[tokio::test]
    async fn test_record_operation() {
        let tracker = HsmPerformanceTracker::new();

        tracker
            .record_operation("provider-1".to_string(), true, 10.0)
            .await;
        tracker
            .record_operation("provider-1".to_string(), true, 20.0)
            .await;
        tracker
            .record_operation("provider-1".to_string(), false, 30.0)
            .await;

        let metrics = tracker.get_metrics("provider-1").await.unwrap();
        assert_eq!(metrics.total_operations, 3);
        assert_eq!(metrics.successful_operations, 2);
        assert_eq!(metrics.failed_operations, 1);
    }

    #[tokio::test]
    async fn test_latency_tracking() {
        let tracker = HsmPerformanceTracker::new();

        tracker
            .record_operation("provider-1".to_string(), true, 10.0)
            .await;
        tracker
            .record_operation("provider-1".to_string(), true, 30.0)
            .await;

        let metrics = tracker.get_metrics("provider-1").await.unwrap();
        assert_eq!(metrics.min_latency_ms, 10.0);
        assert_eq!(metrics.max_latency_ms, 30.0);
        assert_eq!(metrics.average_latency_ms, 20.0);
    }

    #[tokio::test]
    async fn test_reset_metrics() {
        let tracker = HsmPerformanceTracker::new();

        tracker
            .record_operation("provider-1".to_string(), true, 10.0)
            .await;
        assert!(tracker.get_metrics("provider-1").await.is_some());

        tracker.reset_metrics("provider-1").await;
        assert!(tracker.get_metrics("provider-1").await.is_none());
    }

    #[tokio::test]
    async fn test_multiple_providers() {
        let tracker = HsmPerformanceTracker::new();

        tracker
            .record_operation("provider-1".to_string(), true, 10.0)
            .await;
        tracker
            .record_operation("provider-2".to_string(), true, 20.0)
            .await;

        let all_metrics = tracker.get_all_metrics().await;
        assert_eq!(all_metrics.len(), 2);
    }

    #[test]
    fn test_performance_config_default() {
        let config = PerformanceConfig::default();
        assert!(config.enabled);
        assert_eq!(config.sample_rate, 1.0);
    }
}
