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

    // ========================================================================
    // COMPREHENSIVE PERFORMANCE TESTS (Final Coverage Push)
    // ========================================================================

    #[test]
    fn test_operation_metrics_clone() {
        let metrics1 = OperationMetrics {
            total_operations: 100,
            successful_operations: 90,
            failed_operations: 10,
            average_latency_ms: 15.5,
            min_latency_ms: 5.0,
            max_latency_ms: 50.0,
            last_operation_time: chrono::Utc::now(),
        };

        let metrics2 = metrics1.clone();

        assert_eq!(metrics1.total_operations, metrics2.total_operations);
        assert_eq!(
            metrics1.successful_operations,
            metrics2.successful_operations
        );
        assert_eq!(metrics1.failed_operations, metrics2.failed_operations);
        assert_eq!(metrics1.average_latency_ms, metrics2.average_latency_ms);
    }

    #[test]
    fn test_success_rate_edge_cases() {
        // All successful
        let metrics = OperationMetrics {
            total_operations: 100,
            successful_operations: 100,
            failed_operations: 0,
            ..Default::default()
        };
        assert_eq!(metrics.success_rate(), 100.0);
        assert_eq!(metrics.failure_rate(), 0.0);

        // All failed
        let metrics = OperationMetrics {
            total_operations: 100,
            successful_operations: 0,
            failed_operations: 100,
            ..Default::default()
        };
        assert_eq!(metrics.success_rate(), 0.0);
        assert_eq!(metrics.failure_rate(), 100.0);

        // Zero operations
        let metrics = OperationMetrics::default();
        assert_eq!(metrics.success_rate(), 0.0);
    }

    #[test]
    fn test_performance_config_clone() {
        let config1 = PerformanceConfig {
            enabled: false,
            retention_period: Duration::from_secs(7200),
            sample_rate: 0.5,
        };

        let config2 = config1.clone();

        assert_eq!(config1.enabled, config2.enabled);
        assert_eq!(config1.retention_period, config2.retention_period);
        assert_eq!(config1.sample_rate, config2.sample_rate);
    }

    #[test]
    fn test_performance_config_custom() {
        let config = PerformanceConfig {
            enabled: false,
            retention_period: Duration::from_secs(1800),
            sample_rate: 0.25,
        };

        assert!(!config.enabled);
        assert_eq!(config.retention_period, Duration::from_secs(1800));
        assert_eq!(config.sample_rate, 0.25);
    }

    #[tokio::test]
    async fn test_tracker_with_custom_config() {
        let config = PerformanceConfig {
            enabled: true,
            retention_period: Duration::from_secs(600),
            sample_rate: 0.5,
        };

        let tracker = HsmPerformanceTracker::with_config(config.clone());

        // Verify configuration is used
        tracker
            .record_operation("provider-1".to_string(), true, 10.0)
            .await;

        let metrics = tracker.get_metrics("provider-1").await;
        assert!(metrics.is_some());
    }

    #[tokio::test]
    async fn test_record_many_operations() {
        let tracker = HsmPerformanceTracker::new();

        for i in 0..100 {
            let success = i % 10 != 0; // 90% success rate
            let latency = (i as f64) * 1.5;
            tracker
                .record_operation("provider-1".to_string(), success, latency)
                .await;
        }

        let metrics = tracker.get_metrics("provider-1").await.unwrap();
        assert_eq!(metrics.total_operations, 100);
        assert_eq!(metrics.successful_operations, 90);
        assert_eq!(metrics.failed_operations, 10);
        assert!(metrics.average_latency_ms > 0.0);
    }

    #[tokio::test]
    async fn test_concurrent_operations() {
        let tracker = Arc::new(HsmPerformanceTracker::new());
        let mut handles = vec![];

        for i in 0..10 {
            let tracker_clone = Arc::clone(&tracker);
            let handle = tokio::spawn(async move {
                tracker_clone
                    .record_operation(format!("provider-{i}"), true, 10.0)
                    .await;
            });
            handles.push(handle);
        }

        for handle in handles {
            handle.await.unwrap();
        }

        let all_metrics = tracker.get_all_metrics().await;
        assert_eq!(all_metrics.len(), 10);
    }

    #[tokio::test]
    async fn test_concurrent_same_provider() {
        let tracker = Arc::new(HsmPerformanceTracker::new());
        let mut handles = vec![];

        for _ in 0..20 {
            let tracker_clone = Arc::clone(&tracker);
            let handle = tokio::spawn(async move {
                tracker_clone
                    .record_operation("provider-1".to_string(), true, 15.0)
                    .await;
            });
            handles.push(handle);
        }

        for handle in handles {
            handle.await.unwrap();
        }

        let metrics = tracker.get_metrics("provider-1").await.unwrap();
        assert_eq!(metrics.total_operations, 20);
        assert_eq!(metrics.successful_operations, 20);
    }

    #[tokio::test]
    async fn test_latency_calculation_precision() {
        let tracker = HsmPerformanceTracker::new();

        tracker
            .record_operation("provider-1".to_string(), true, 10.5)
            .await;
        tracker
            .record_operation("provider-1".to_string(), true, 20.3)
            .await;
        tracker
            .record_operation("provider-1".to_string(), true, 15.7)
            .await;

        let metrics = tracker.get_metrics("provider-1").await.unwrap();
        assert_eq!(metrics.min_latency_ms, 10.5);
        assert_eq!(metrics.max_latency_ms, 20.3);

        // Average should be (10.5 + 20.3 + 15.7) / 3 = 15.5
        assert!((metrics.average_latency_ms - 15.5).abs() < 0.01);
    }

    #[tokio::test]
    async fn test_get_nonexistent_metrics() {
        let tracker = HsmPerformanceTracker::new();
        let metrics = tracker.get_metrics("nonexistent").await;
        assert!(metrics.is_none());
    }

    #[tokio::test]
    async fn test_reset_nonexistent_metrics() {
        let tracker = HsmPerformanceTracker::new();
        // Should not panic
        tracker.reset_metrics("nonexistent").await;
    }

    #[tokio::test]
    async fn test_multiple_providers_isolation() {
        let tracker = HsmPerformanceTracker::new();

        tracker
            .record_operation("provider-1".to_string(), true, 10.0)
            .await;
        tracker
            .record_operation("provider-2".to_string(), true, 20.0)
            .await;
        tracker
            .record_operation("provider-3".to_string(), true, 30.0)
            .await;

        let metrics1 = tracker.get_metrics("provider-1").await.unwrap();
        let metrics2 = tracker.get_metrics("provider-2").await.unwrap();
        let metrics3 = tracker.get_metrics("provider-3").await.unwrap();

        assert_eq!(metrics1.average_latency_ms, 10.0);
        assert_eq!(metrics2.average_latency_ms, 20.0);
        assert_eq!(metrics3.average_latency_ms, 30.0);
    }

    #[tokio::test]
    async fn test_reset_one_provider_keeps_others() {
        let tracker = HsmPerformanceTracker::new();

        tracker
            .record_operation("provider-1".to_string(), true, 10.0)
            .await;
        tracker
            .record_operation("provider-2".to_string(), true, 20.0)
            .await;

        tracker.reset_metrics("provider-1").await;

        assert!(tracker.get_metrics("provider-1").await.is_none());
        assert!(tracker.get_metrics("provider-2").await.is_some());
    }

    #[test]
    fn test_operation_metrics_debug() {
        let metrics = OperationMetrics::default();
        let debug_str = format!("{:?}", metrics);
        assert!(debug_str.contains("OperationMetrics"));
    }

    #[test]
    fn test_performance_config_debug() {
        let config = PerformanceConfig::default();
        let debug_str = format!("{:?}", config);
        assert!(debug_str.contains("PerformanceConfig"));
    }

    #[tokio::test]
    async fn test_very_high_latency() {
        let tracker = HsmPerformanceTracker::new();

        tracker
            .record_operation("provider-1".to_string(), true, 5000.0)
            .await;

        let metrics = tracker.get_metrics("provider-1").await.unwrap();
        assert_eq!(metrics.max_latency_ms, 5000.0);
    }

    #[tokio::test]
    async fn test_very_low_latency() {
        let tracker = HsmPerformanceTracker::new();

        tracker
            .record_operation("provider-1".to_string(), true, 0.001)
            .await;

        let metrics = tracker.get_metrics("provider-1").await.unwrap();
        assert_eq!(metrics.min_latency_ms, 0.001);
    }

    #[tokio::test]
    async fn test_mixed_success_failure() {
        let tracker = HsmPerformanceTracker::new();

        for i in 0..10 {
            let success = i % 2 == 0;
            tracker
                .record_operation("provider-1".to_string(), success, 10.0)
                .await;
        }

        let metrics = tracker.get_metrics("provider-1").await.unwrap();
        assert_eq!(metrics.total_operations, 10);
        assert_eq!(metrics.successful_operations, 5);
        assert_eq!(metrics.failed_operations, 5);
        assert_eq!(metrics.success_rate(), 50.0);
        assert_eq!(metrics.failure_rate(), 50.0);
    }

    #[tokio::test]
    async fn test_get_all_metrics_empty() {
        let tracker = HsmPerformanceTracker::new();
        let all_metrics = tracker.get_all_metrics().await;
        assert!(all_metrics.is_empty());
    }

    #[tokio::test]
    async fn test_get_all_metrics_populated() {
        let tracker = HsmPerformanceTracker::new();

        for i in 1..=5 {
            tracker
                .record_operation(format!("provider-{i}"), true, i as f64)
                .await;
        }

        let all_metrics = tracker.get_all_metrics().await;
        assert_eq!(all_metrics.len(), 5);

        for i in 1..=5 {
            let key = format!("provider-{i}");
            assert!(all_metrics.contains_key(&key));
        }
    }
}
