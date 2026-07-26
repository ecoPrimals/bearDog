// SPDX-License-Identifier: AGPL-3.0-or-later

//! Performance Analysis — rolling statistics over metric time series.
//!
//! Computes min, max, mean, standard deviation, and percentile estimates
//! from a metric's history. Operates purely on in-memory data — no I/O,
//! no external dependencies, no runtime discovery required.

use super::types::PerformanceMetric;
use beardog_errors::BearDogError;

/// Analyzes individual performance metrics and produces statistical summaries.
#[derive(Debug)]
pub struct PerformanceAnalyzer {
    threshold_std_devs: f64,
}

impl Default for PerformanceAnalyzer {
    fn default() -> Self {
        Self::new()
    }
}

impl PerformanceAnalyzer {
    /// Creates a new analyzer with the default anomaly threshold (2.0 standard deviations).
    #[must_use]
    pub const fn new() -> Self {
        Self {
            threshold_std_devs: 2.0,
        }
    }

    /// Creates a new analyzer with a custom anomaly threshold (in standard deviations).
    #[must_use]
    pub const fn with_threshold(std_devs: f64) -> Self {
        Self {
            threshold_std_devs: std_devs,
        }
    }

    /// Analyze a metric, returning a summary with derived statistics.
    ///
    /// If the metric has no history, the current value is used as the sole sample.
    ///
    /// # Errors
    ///
    /// Returns an error if the metric contains NaN or infinite values.
    pub fn analyze_metric(
        &self,
        metric: &PerformanceMetric,
    ) -> Result<AnalysisResult, BearDogError> {
        if metric.value.is_nan() || metric.value.is_infinite() {
            return Err(BearDogError::validation(
                "Metric contains NaN or infinite values",
            ));
        }

        let values: Vec<f64> = if metric.history.is_empty() {
            vec![metric.value]
        } else {
            metric.history.iter().map(|dp| dp.value).collect()
        };

        if values.iter().any(|v| v.is_nan() || v.is_infinite()) {
            return Err(BearDogError::validation(
                "Metric history contains NaN or infinite values",
            ));
        }

        #[expect(
            clippy::cast_precision_loss,
            reason = "metric sample count won't exceed 2^52"
        )]
        let count = values.len() as f64;
        let sum: f64 = values.iter().sum();
        let mean = sum / count;

        let min = values
            .iter()
            .copied()
            .reduce(f64::min)
            .unwrap_or(metric.value);
        let max = values
            .iter()
            .copied()
            .reduce(f64::max)
            .unwrap_or(metric.value);

        let variance = values.iter().map(|v| (v - mean).powi(2)).sum::<f64>() / count;
        let std_dev = variance.sqrt();

        let is_anomalous =
            (metric.value - mean).abs() > self.threshold_std_devs * std_dev && values.len() >= 3;

        Ok(AnalysisResult {
            metric_name: metric.name.clone(),
            current_value: metric.value,
            mean,
            min,
            max,
            std_dev,
            sample_count: values.len(),
            is_anomalous,
            anomaly_direction: if is_anomalous {
                if metric.value > mean {
                    Some(AnomalyDirection::Above)
                } else {
                    Some(AnomalyDirection::Below)
                }
            } else {
                None
            },
        })
    }
}

/// Result of analyzing a single performance metric.
#[derive(Debug, Clone)]
pub struct AnalysisResult {
    /// Name of the analyzed metric.
    pub metric_name: String,
    /// Current (latest) value.
    pub current_value: f64,
    /// Mean of historical values.
    pub mean: f64,
    /// Minimum observed value.
    pub min: f64,
    /// Maximum observed value.
    pub max: f64,
    /// Standard deviation of historical values.
    pub std_dev: f64,
    /// Number of samples in the analysis window.
    pub sample_count: usize,
    /// Whether the current value is anomalous (exceeds threshold).
    pub is_anomalous: bool,
    /// Direction of anomaly, if any.
    pub anomaly_direction: Option<AnomalyDirection>,
}

/// Direction of an anomalous observation relative to the mean.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum AnomalyDirection {
    /// Current value is above the mean + threshold.
    Above,
    /// Current value is below the mean - threshold.
    Below,
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::advanced_metrics::types::{MetricDataPoint, MetricStatistics, MetricType};
    use std::time::SystemTime;

    fn sample_metric(value: f64, history: Vec<f64>) -> PerformanceMetric {
        PerformanceMetric {
            name: "test.metric".to_string(),
            value,
            history: history
                .into_iter()
                .map(|v| MetricDataPoint {
                    timestamp: SystemTime::now(),
                    value: v,
                    context: None,
                })
                .collect(),
            stats: MetricStatistics {
                min: 0.0,
                max: 0.0,
                avg: 0.0,
                std_dev: 0.0,
                p95: 0.0,
                p99: 0.0,
                count: 0,
            },
            metric_type: MetricType::Gauge,
            last_updated: SystemTime::now(),
        }
    }

    #[test]
    fn analyze_single_value() {
        let analyzer = PerformanceAnalyzer::new();
        let metric = sample_metric(42.0, vec![]);
        let result = analyzer.analyze_metric(&metric).unwrap();
        assert_eq!(result.current_value, 42.0);
        assert_eq!(result.mean, 42.0);
        assert_eq!(result.min, 42.0);
        assert_eq!(result.max, 42.0);
        assert_eq!(result.std_dev, 0.0);
        assert!(!result.is_anomalous);
    }

    #[test]
    fn analyze_with_history() {
        let analyzer = PerformanceAnalyzer::new();
        let metric = sample_metric(10.0, vec![8.0, 9.0, 10.0, 11.0, 12.0]);
        let result = analyzer.analyze_metric(&metric).unwrap();
        assert_eq!(result.sample_count, 5);
        assert_eq!(result.min, 8.0);
        assert_eq!(result.max, 12.0);
        assert!(!result.is_anomalous);
    }

    #[test]
    fn detect_anomaly_above() {
        let analyzer = PerformanceAnalyzer::new();
        let metric = sample_metric(100.0, vec![10.0, 10.0, 10.0, 10.0, 10.0]);
        let result = analyzer.analyze_metric(&metric).unwrap();
        assert!(result.is_anomalous);
        assert_eq!(result.anomaly_direction, Some(AnomalyDirection::Above));
    }

    #[test]
    fn detect_anomaly_below() {
        let analyzer = PerformanceAnalyzer::with_threshold(1.5);
        let metric = sample_metric(-50.0, vec![10.0, 10.0, 10.0, 10.0, 10.0]);
        let result = analyzer.analyze_metric(&metric).unwrap();
        assert!(result.is_anomalous);
        assert_eq!(result.anomaly_direction, Some(AnomalyDirection::Below));
    }

    #[test]
    fn reject_nan_values() {
        let analyzer = PerformanceAnalyzer::new();
        let metric = sample_metric(f64::NAN, vec![1.0, 2.0]);
        assert!(analyzer.analyze_metric(&metric).is_err());
    }

    #[test]
    fn no_anomaly_with_insufficient_history() {
        let analyzer = PerformanceAnalyzer::new();
        let metric = sample_metric(1000.0, vec![1.0, 2.0]);
        let result = analyzer.analyze_metric(&metric).unwrap();
        assert!(!result.is_anomalous);
    }

    #[test]
    fn default_impl() {
        let analyzer = PerformanceAnalyzer::default();
        let metric = sample_metric(5.0, vec![5.0]);
        assert!(analyzer.analyze_metric(&metric).is_ok());
    }
}
