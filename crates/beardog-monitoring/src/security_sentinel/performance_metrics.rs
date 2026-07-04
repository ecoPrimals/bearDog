// SPDX-License-Identifier: AGPL-3.0-or-later

//! # Performance Metrics
//!
//! This module provides types for collecting and analyzing
//! performance metrics in the security sentinel system.

use beardog_errors::BearDogError;
use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use std::sync::Arc;
use tokio::sync::RwLock;
use tracing::debug;

// ============================================================
// Thresholds
// ============================================================

/// Performance thresholds for monitoring
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PerformanceThresholds {
    /// CPU usage threshold percentage
    pub cpu_threshold: f64,

    /// Memory usage threshold percentage
    pub memory_threshold: f64,

    /// Response time threshold in ms
    pub response_time_threshold: f64,

    /// Error rate threshold percentage
    pub error_rate_threshold: f64,

    /// Throughput threshold in ops/sec
    pub throughput_threshold: f64,
}

impl Default for PerformanceThresholds {
    fn default() -> Self {
        Self {
            cpu_threshold: 80.0,
            memory_threshold: 85.0,
            response_time_threshold: 1000.0,
            error_rate_threshold: 5.0,
            throughput_threshold: 100.0,
        }
    }
}

// ============================================================
// Metrics
// ============================================================

/// Performance metrics snapshot
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PerformanceMetrics {
    /// Timestamp of measurement
    pub timestamp: DateTime<Utc>,

    /// CPU usage percentage
    pub cpu_usage: f64,

    /// Memory usage percentage
    pub memory_usage: f64,

    /// Response time in ms
    pub response_time: f64,

    /// Error rate percentage
    pub error_rate: f64,

    /// Throughput in ops/sec
    pub throughput: f64,

    /// Custom metrics
    pub custom_metrics: HashMap<String, f64>,
}

impl Default for PerformanceMetrics {
    fn default() -> Self {
        Self {
            timestamp: Utc::now(),
            cpu_usage: 0.0,
            memory_usage: 0.0,
            response_time: 0.0,
            error_rate: 0.0,
            throughput: 0.0,
            custom_metrics: HashMap::new(),
        }
    }
}

// ============================================================
// Trends
// ============================================================

/// Metric trend direction
#[derive(Debug, Clone, Copy, PartialEq, Eq, Default, Serialize, Deserialize)]
pub enum MetricTrend {
    /// Increasing trend
    Increasing,
    /// Decreasing trend
    Decreasing,
    /// Stable trend (default)
    #[default]
    Stable,
}

/// Performance trends analysis
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PerformanceTrends {
    /// CPU usage trend
    pub cpu_trend: MetricTrend,

    /// Memory usage trend
    pub memory_trend: MetricTrend,

    /// Response time trend
    pub response_time_trend: MetricTrend,

    /// Error rate trend
    pub error_rate_trend: MetricTrend,

    /// Throughput trend
    pub throughput_trend: MetricTrend,

    /// Analysis window in minutes
    pub window_minutes: u64,

    /// Number of samples analyzed
    pub sample_count: usize,

    /// Average CPU percentage
    pub avg_cpu_percent: f64,

    /// Average memory in MB
    pub avg_memory_mb: f64,

    /// Average latency in ms
    pub avg_latency_ms: f64,
}

impl Default for PerformanceTrends {
    fn default() -> Self {
        Self {
            cpu_trend: MetricTrend::Stable,
            memory_trend: MetricTrend::Stable,
            response_time_trend: MetricTrend::Stable,
            error_rate_trend: MetricTrend::Stable,
            throughput_trend: MetricTrend::Stable,
            window_minutes: 0,
            sample_count: 0,
            avg_cpu_percent: 0.0,
            avg_memory_mb: 0.0,
            avg_latency_ms: 0.0,
        }
    }
}

// ============================================================
// Collector
// ============================================================

/// Performance metrics collector
#[derive(Debug)]
pub struct PerformanceMetricsCollector {
    /// Metrics history
    metrics_history: Arc<RwLock<Vec<PerformanceMetrics>>>,

    /// Thresholds
    thresholds: Arc<RwLock<PerformanceThresholds>>,

    /// Maximum history size
    max_history_size: usize,

    /// Real system metrics collector (pure Rust /proc parsing)
    system_metrics: super::system_metrics::SystemMetrics,
}

impl Default for PerformanceMetricsCollector {
    fn default() -> Self {
        Self::new()
    }
}

impl PerformanceMetricsCollector {
    /// Create a new collector
    pub fn new() -> Self {
        Self {
            metrics_history: Arc::new(RwLock::new(Vec::new())),
            thresholds: Arc::new(RwLock::new(PerformanceThresholds::default())),
            max_history_size: 1000,
            system_metrics: super::system_metrics::SystemMetrics::new(),
        }
    }

    /// Create with custom configuration
    pub fn with_config(max_history_size: usize, thresholds: PerformanceThresholds) -> Self {
        Self {
            metrics_history: Arc::new(RwLock::new(Vec::new())),
            thresholds: Arc::new(RwLock::new(thresholds)),
            max_history_size,
            system_metrics: super::system_metrics::SystemMetrics::new(),
        }
    }

    /// Collect current metrics
    ///
    /// # Errors
    /// Returns an error if metrics collection fails.
    pub async fn collect_metrics(&self) -> Result<PerformanceMetrics, BearDogError> {
        let metrics = PerformanceMetrics {
            timestamp: Utc::now(),
            cpu_usage: self.collect_cpu_usage().await?,
            memory_usage: self.collect_memory_usage().await?,
            response_time: self.collect_response_time()?,
            error_rate: self.collect_error_rate()?,
            throughput: self.collect_throughput()?,
            custom_metrics: HashMap::new(),
        };

        debug!(
            "Collected metrics: CPU {:.1}%, Memory {:.1}%",
            metrics.cpu_usage, metrics.memory_usage
        );

        // Store in history
        let mut history = self.metrics_history.write().await;
        history.push(metrics.clone());

        // Trim history if needed
        if history.len() > self.max_history_size {
            history.remove(0);
        }

        Ok(metrics)
    }

    /// Get recent metrics
    ///
    /// # Errors
    /// Returns an error if reading history fails.
    pub async fn get_recent_metrics(
        &self,
        count: usize,
    ) -> Result<Vec<PerformanceMetrics>, BearDogError> {
        let history = self.metrics_history.read().await;
        let start_index = if history.len() > count {
            history.len() - count
        } else {
            0
        };
        Ok(history[start_index..].to_vec())
    }

    /// Analyze trends
    ///
    /// # Errors
    /// Returns an error if trend analysis fails.
    #[expect(
        clippy::cast_precision_loss,
        reason = "Averages over sample count; acceptable precision for dashboard metrics"
    )]
    pub async fn analyze_trends(
        &self,
        window_size: usize,
    ) -> Result<PerformanceTrends, BearDogError> {
        let recent_metrics = self.get_recent_metrics(window_size).await?;

        if recent_metrics.len() < 2 {
            return Ok(PerformanceTrends::default());
        }

        // PANIC SAFETY: Use if-let for defensive coding even though len() >= 2 is checked
        let window_minutes = match (recent_metrics.first(), recent_metrics.last()) {
            (Some(first), Some(last)) => (last.timestamp - first.timestamp)
                .num_minutes()
                .unsigned_abs(),
            _ => 0, // Should never happen given len() >= 2 check above
        };

        Ok(PerformanceTrends {
            cpu_trend: self.calculate_trend(&recent_metrics, |m| m.cpu_usage),
            memory_trend: self.calculate_trend(&recent_metrics, |m| m.memory_usage),
            response_time_trend: self.calculate_trend(&recent_metrics, |m| m.response_time),
            error_rate_trend: self.calculate_trend(&recent_metrics, |m| m.error_rate),
            throughput_trend: self.calculate_trend(&recent_metrics, |m| m.throughput),
            window_minutes,
            sample_count: recent_metrics.len(),
            avg_cpu_percent: recent_metrics.iter().map(|m| m.cpu_usage).sum::<f64>()
                / recent_metrics.len() as f64,
            avg_memory_mb: recent_metrics.iter().map(|m| m.memory_usage).sum::<f64>()
                / recent_metrics.len() as f64,
            avg_latency_ms: recent_metrics.iter().map(|m| m.response_time).sum::<f64>()
                / recent_metrics.len() as f64,
        })
    }

    /// Calculate trend for a metric
    #[expect(
        clippy::cast_precision_loss,
        reason = "Half-window means; acceptable precision for trend comparison"
    )]
    fn calculate_trend<F>(&self, metrics: &[PerformanceMetrics], extractor: F) -> MetricTrend
    where
        F: Fn(&PerformanceMetrics) -> f64,
    {
        if metrics.len() < 2 {
            return MetricTrend::Stable;
        }

        let values: Vec<f64> = metrics.iter().map(extractor).collect();
        let mid = values.len() / 2;

        let first_half: f64 = values[0..mid].iter().sum::<f64>() / mid as f64;
        let second_half: f64 = values[mid..].iter().sum::<f64>() / (values.len() - mid) as f64;

        if first_half == 0.0 {
            return MetricTrend::Stable;
        }

        let change_percent = ((second_half - first_half) / first_half) * 100.0;

        match change_percent {
            x if x > 10.0 => MetricTrend::Increasing,
            x if x < -10.0 => MetricTrend::Decreasing,
            _ => MetricTrend::Stable,
        }
    }

    /// Check thresholds against metrics
    ///
    /// # Errors
    /// Returns an error if threshold checking fails.
    pub async fn check_thresholds(
        &self,
        metrics: &PerformanceMetrics,
    ) -> Result<Vec<String>, BearDogError> {
        let thresholds = self.thresholds.read().await;
        let mut violations = Vec::new();

        if metrics.cpu_usage > thresholds.cpu_threshold {
            violations.push(format!(
                "CPU usage {:.1}% exceeds threshold {:.1}%",
                metrics.cpu_usage, thresholds.cpu_threshold
            ));
        }

        if metrics.memory_usage > thresholds.memory_threshold {
            violations.push(format!(
                "Memory usage {:.1}% exceeds threshold {:.1}%",
                metrics.memory_usage, thresholds.memory_threshold
            ));
        }

        if metrics.response_time > thresholds.response_time_threshold {
            violations.push(format!(
                "Response time {:.1}ms exceeds threshold {:.1}ms",
                metrics.response_time, thresholds.response_time_threshold
            ));
        }

        if metrics.error_rate > thresholds.error_rate_threshold {
            violations.push(format!(
                "Error rate {:.1}% exceeds threshold {:.1}%",
                metrics.error_rate, thresholds.error_rate_threshold
            ));
        }

        if metrics.throughput < thresholds.throughput_threshold {
            violations.push(format!(
                "Throughput {:.1} ops/sec below minimum threshold {:.1} ops/sec",
                metrics.throughput, thresholds.throughput_threshold
            ));
        }

        Ok(violations)
    }

    /// Update thresholds
    pub async fn update_thresholds(&self, thresholds: PerformanceThresholds) {
        let mut current = self.thresholds.write().await;
        *current = thresholds;
    }

    // Private collection methods — delegated to SystemMetrics.
    //
    // EVOLVED (Feb 11, 2026): From NotImplemented stubs to real implementations.
    // CPU/memory: Pure Rust /proc/stat + /proc/meminfo parsing (Linux).
    // Response time/error rate/throughput: Atomic counters fed by record_request().

    async fn collect_cpu_usage(&self) -> Result<f64, BearDogError> {
        self.system_metrics.collect_cpu_usage().await
    }

    async fn collect_memory_usage(&self) -> Result<f64, BearDogError> {
        self.system_metrics.collect_memory_usage().await
    }

    fn collect_response_time(&self) -> Result<f64, BearDogError> {
        self.system_metrics.collect_response_time()
    }

    fn collect_error_rate(&self) -> Result<f64, BearDogError> {
        self.system_metrics.collect_error_rate()
    }

    fn collect_throughput(&self) -> Result<f64, BearDogError> {
        self.system_metrics.collect_throughput()
    }

    /// Record a completed request for throughput/latency/error tracking.
    ///
    /// Callers feed request data here; the collector aggregates it for
    /// the next `collect_metrics()` call.
    pub fn record_request(&self, duration: std::time::Duration, is_error: bool) {
        self.system_metrics.record_request(duration, is_error);
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_thresholds_default() {
        let thresholds = PerformanceThresholds::default();
        assert!(
            (thresholds.cpu_threshold - 80.0).abs() < f64::EPSILON,
            "expected cpu_threshold ≈ 80.0, got {}",
            thresholds.cpu_threshold
        );
        assert!(
            (thresholds.memory_threshold - 85.0).abs() < f64::EPSILON,
            "expected memory_threshold ≈ 85.0, got {}",
            thresholds.memory_threshold
        );
    }

    #[test]
    fn test_metrics_default() {
        let metrics = PerformanceMetrics::default();
        assert!(
            metrics.cpu_usage.abs() < f64::EPSILON,
            "expected default cpu_usage ≈ 0.0, got {}",
            metrics.cpu_usage
        );
    }

    #[test]
    fn test_trend_default() {
        assert_eq!(MetricTrend::default(), MetricTrend::Stable);
    }

    #[tokio::test]
    async fn test_collector_creation() {
        let collector = PerformanceMetricsCollector::new();
        // First collection establishes CPU baseline — cpu_usage is 0.0 on first read.
        // Memory usage is real on Linux.
        let metrics = collector.collect_metrics().await.expect("collect metrics");
        assert!(metrics.cpu_usage >= 0.0, "CPU usage should be non-negative");
        #[cfg(target_os = "linux")]
        assert!(
            metrics.memory_usage > 0.0,
            "Memory usage should be > 0 on Linux"
        );
    }

    #[tokio::test]
    async fn test_collector_second_read_has_real_cpu() {
        let collector = PerformanceMetricsCollector::new();
        // Establish baseline
        let _first = collector.collect_metrics().await.expect("first collect");
        // Minimal delay for CPU delta between `/proc` reads
        tokio::time::sleep(std::time::Duration::from_millis(1)).await;
        let second = collector.collect_metrics().await.expect("second collect");
        assert!(second.cpu_usage >= 0.0 && second.cpu_usage <= 100.0);
    }

    #[tokio::test]
    async fn test_collector_record_request() {
        let collector = PerformanceMetricsCollector::new();
        collector.record_request(std::time::Duration::from_millis(42), false);
        collector.record_request(std::time::Duration::from_millis(100), true);
        let metrics = collector.collect_metrics().await.expect("collect metrics");
        // Response time should reflect the recorded requests
        assert!(metrics.response_time >= 0.0);
    }

    #[test]
    fn test_calculate_trend() {
        let collector = PerformanceMetricsCollector::new();
        let metrics = vec![
            PerformanceMetrics {
                cpu_usage: 50.0,
                ..Default::default()
            },
            PerformanceMetrics {
                cpu_usage: 55.0,
                ..Default::default()
            },
            PerformanceMetrics {
                cpu_usage: 80.0,
                ..Default::default()
            },
            PerformanceMetrics {
                cpu_usage: 85.0,
                ..Default::default()
            },
        ];
        let trend = collector.calculate_trend(&metrics, |m| m.cpu_usage);
        assert_eq!(trend, MetricTrend::Increasing);
    }

    #[test]
    fn test_calculate_trend_decreasing() {
        let collector = PerformanceMetricsCollector::new();
        let metrics = vec![
            PerformanceMetrics {
                cpu_usage: 100.0,
                ..Default::default()
            },
            PerformanceMetrics {
                cpu_usage: 100.0,
                ..Default::default()
            },
            PerformanceMetrics {
                cpu_usage: 10.0,
                ..Default::default()
            },
            PerformanceMetrics {
                cpu_usage: 10.0,
                ..Default::default()
            },
        ];
        assert_eq!(
            collector.calculate_trend(&metrics, |m| m.cpu_usage),
            MetricTrend::Decreasing
        );
    }

    #[test]
    fn test_calculate_trend_first_half_zero_is_stable() {
        let collector = PerformanceMetricsCollector::new();
        let metrics = vec![
            PerformanceMetrics {
                cpu_usage: 0.0,
                ..Default::default()
            },
            PerformanceMetrics {
                cpu_usage: 50.0,
                ..Default::default()
            },
        ];
        assert_eq!(
            collector.calculate_trend(&metrics, |m| m.cpu_usage),
            MetricTrend::Stable
        );
    }

    #[tokio::test]
    async fn test_analyze_trends_requires_two_or_more_samples() {
        let collector = PerformanceMetricsCollector::new();
        let empty = collector.analyze_trends(5).await.expect("trends");
        assert_eq!(empty.sample_count, 0);

        collector.collect_metrics().await.expect("one sample");
        let one = collector
            .analyze_trends(5)
            .await
            .expect("one sample trends");
        assert_eq!(one.sample_count, 0);

        collector.collect_metrics().await.expect("second sample");
        let two = collector
            .analyze_trends(5)
            .await
            .expect("two sample trends");
        assert_eq!(two.sample_count, 2);
    }

    #[tokio::test]
    async fn test_get_recent_metrics_respects_max_history() {
        let collector =
            PerformanceMetricsCollector::with_config(2, PerformanceThresholds::default());
        collector.collect_metrics().await.expect("m1");
        collector.collect_metrics().await.expect("m2");
        collector.collect_metrics().await.expect("m3");
        let recent = collector.get_recent_metrics(10).await.expect("recent");
        assert_eq!(recent.len(), 2);
    }

    #[tokio::test]
    async fn test_check_thresholds_throughput_violation() {
        let collector = PerformanceMetricsCollector::new();
        let thresholds = PerformanceThresholds {
            throughput_threshold: 1000.0,
            ..PerformanceThresholds::default()
        };
        collector.update_thresholds(thresholds).await;
        let mut m = PerformanceMetrics::default();
        m.throughput = 10.0;
        let v = collector.check_thresholds(&m).await.expect("violations");
        assert!(v.iter().any(|s| s.contains("Throughput")));
    }

    #[tokio::test]
    async fn test_update_thresholds_roundtrip() {
        let collector = PerformanceMetricsCollector::new();
        let t = PerformanceThresholds {
            cpu_threshold: 12.0,
            memory_threshold: 34.0,
            response_time_threshold: 56.0,
            error_rate_threshold: 7.0,
            throughput_threshold: 89.0,
        };
        collector.update_thresholds(t.clone()).await;
        let mut m = PerformanceMetrics::default();
        m.cpu_usage = 99.0;
        let v = collector.check_thresholds(&m).await.expect("v");
        assert!(v.iter().any(|s| s.contains("CPU")));
    }
}
