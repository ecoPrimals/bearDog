//! # Performance Comparisons Module
//!
//! This module provides performance comparison functionality.

use super::metrics::PerformanceMetrics;

/// Performance comparison result
#[derive(Debug, Clone)]
pub struct ComparisonResult {
    /// Comparison name
    pub name: String,
    /// Performance improvement percentage
    pub improvement_percent: f64,
}

/// Compares two performance metrics
pub fn compare_metrics(baseline: &PerformanceMetrics, current: &PerformanceMetrics) -> ComparisonResult {
    let improvement = if baseline.avg_response_time_ms > 0.0 {
        ((baseline.avg_response_time_ms - current.avg_response_time_ms) / baseline.avg_response_time_ms) * 100.0
    } else {
        0.0
    };

    ComparisonResult {
        name: "Response Time Comparison".to_string(),
        improvement_percent: improvement,
    }
} 