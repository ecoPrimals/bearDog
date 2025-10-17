// Analysis Module - Anomaly Detection and Trend Analysis

use super::config::{AnomalyConfig, TrendConfig};
use super::types::MetricDataPoint;

/// Anomaly detector
#[derive(Debug)]
pub struct AnomalyDetector {
    config: AnomalyConfig,
}

impl AnomalyDetector {
    /// Create new anomaly detector
    /// Creates a new instance
    #[must_use]
    pub const fn new(config: AnomalyConfig) -> Self {
        Self { config }
    }

    /// Detect anomalies in metric data
    #[must_use]
    pub fn detect_anomalies(&self, metrics: &[MetricDataPoint]) -> Vec<AnomalyDetection> {
        let mut anomalies = Vec::new();

        if metrics.len() < self.config.min_data_points {
            return anomalies;
        }

        // Use config for anomaly detection based on sensitivity
        let avg = metrics.iter().map(|p| p.value).sum::<f64>() / metrics.len() as f64;
        let threshold = avg * (1.0 + self.config.sensitivity);

        for point in metrics {
            if point.value > threshold {
                anomalies.push(AnomalyDetection {
                    timestamp: point.timestamp,
                    score: (point.value - threshold) / threshold,
                    description: format!(
                        "Value {} exceeds threshold {:.2}",
                        point.value, threshold
                    ),
                });
            }
        }

        anomalies
    }
}

/// Trend analyzer
#[derive(Debug)]
pub struct TrendAnalyzer {
    #[allow(dead_code)] // Config reserved for future trend analysis features
    config: TrendConfig,
}

impl TrendAnalyzer {
    /// Create new trend analyzer
    /// Creates a new instance
    #[must_use]
    pub const fn new(config: TrendConfig) -> Self {
        Self { config }
    }

    /// Analyze trends in metric data
    #[must_use]
    pub fn analyze_trends(&self, metrics: &[MetricDataPoint]) -> TrendAnalysis {
        if metrics.len() < 3 {
            return TrendAnalysis {
                direction: TrendDirection::Stable,
                strength: 0.0,
                predictions: Vec::new(),
            };
        }

        // Simple trend analysis using available data
        let recent_metrics = if metrics.len() > 10 {
            &metrics[metrics.len() - 10..]
        } else {
            metrics
        };
        let trend_strength = self.calculate_trend_strength(recent_metrics);

        let direction = if trend_strength > 0.1 {
            TrendDirection::Increasing
        } else if trend_strength < -0.1 {
            TrendDirection::Decreasing
        } else {
            TrendDirection::Stable
        };

        TrendAnalysis {
            direction,
            strength: trend_strength.abs(),
            predictions: Vec::new(), // Simplified - no predictions for now
        }
    }

    /// Calculate trend strength using simple slope calculation
    fn calculate_trend_strength(&self, metrics: &[MetricDataPoint]) -> f64 {
        if metrics.len() < 2 {
            return 0.0;
        }

        // Simple slope calculation
        let first = metrics[0].value;
        let last = metrics[metrics.len() - 1].value;
        (last - first) / metrics.len() as f64
    }
}

/// Anomaly detection result
#[derive(Debug, Clone)]
pub struct AnomalyDetection {
    /// Anomaly timestamp
    pub timestamp: std::time::SystemTime,
    /// Anomaly score (0.0 to 1.0)
    /// The score value
    pub score: f64,
    /// Description
    /// The description value
    pub description: String,
}

/// Trend analysis result
#[derive(Debug, Clone)]
pub struct TrendAnalysis {
    /// Trend direction
    /// The direction value
    pub direction: TrendDirection,
    /// Trend strength (0.0 to 1.0)
    /// The strength value
    pub strength: f64,
    /// Future predictions
    /// Collection of predictions
    pub predictions: Vec<MetricDataPoint>,
}

/// Trend direction
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum TrendDirection {
    /// Currently increasing
    Increasing,
    /// Currently decreasing
    Decreasing,
    /// Represents stable variant
    Stable,
    /// Represents volatile variant
    Volatile,
}
