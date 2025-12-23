//! # Performance Recommendations Module
//!
//! This module provides performance optimization recommendations.

use super::metrics::PerformanceMetrics;

/// Performance recommendation
#[derive(Debug, Clone)]
pub struct Recommendation {
    /// Recommendation title
    pub title: String,
    /// Recommendation description
    pub description: String,
    /// Priority level (1-10)
    pub priority: u8,
}

/// Generates performance recommendations based on metrics
pub fn generate_recommendations(metrics: &PerformanceMetrics) -> Vec<Recommendation> {
    let mut recommendations = Vec::new();

    if metrics.avg_response_time_ms > 1000.0 {
        recommendations.push(Recommendation {
            title: "High Response Time".to_string(),
            description: "Consider optimizing slow operations or adding caching".to_string(),
            priority: 8,
        });
    }

    if metrics.total_operations < 10 {
        recommendations.push(Recommendation {
            title: "Low Operation Count".to_string(),
            description: "Increase load testing to get more accurate metrics".to_string(),
            priority: 3,
        });
    }

    recommendations
} 