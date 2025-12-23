// Module documentation
//
// This module provides functionality for the BearDog ecosystem.


use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct CacheMetrics {
    /// Number of l1_hits
    pub l1_hits: u64,
    /// Number of l2_hits
    pub l2_hits: u64,
    /// Number of l3_hits
    pub l3_hits: u64,
    /// Number of total_misses
    pub total_misses: u64,
    /// Number of total_requests
    pub total_requests: u64,
    pub average_response_time_ms: f64,
}

impl CacheMetrics {
    /// Create new cache metrics
    /// Creates a new instance
    pub fn new() -> Self {
        Self::default()
    }

    /// Record a cache hit at specified level
    pub fn record_hit(&mut self, level: &str) {
        self.total_requests += 1;
        match level {
            "L1" | "l1" => self.l1_hits += 1,
            "L2" | "l2" => self.l2_hits += 1,
            "L3" | "l3" => self.l3_hits += 1,
            _ => {} // Unknown level
        }
    }

    /// Record a cache miss
    pub fn record_miss(&mut self) {
        self.total_requests += 1;
        self.total_misses += 1;
    }

    /// Calculate overall hit ratio
    pub fn hit_ratio(&self) -> f64 {
        if self.total_requests == 0 {
            0.0
        } else {
            let total_hits = self.l1_hits + self.l2_hits + self.l3_hits;
            total_hits as f64 / self.total_requests as f64
        }
    }

    /// Calculate L1 hit ratio
    pub fn l1_hit_ratio(&self) -> f64 {
        if self.total_requests == 0 {
            0.0
        } else {
            self.l1_hits as f64 / self.total_requests as f64
        }
    }

    /// Calculate miss ratio
    pub fn miss_ratio(&self) -> f64 {
        if self.total_requests == 0 {
            0.0
        } else {
            self.total_misses as f64 / self.total_requests as f64
        }
    }

    /// Update average response time
    /// Updates response_time
    /// Updates response_time
    pub fn update_response_time(&mut self, response_time_ms: f64) {
        // Simple exponential moving average
        if self.average_response_time_ms == 0.0 {
            self.average_response_time_ms = response_time_ms;
        } else {
            self.average_response_time_ms =
                0.9 * self.average_response_time_ms + 0.1 * response_time_ms;
        }
    }

    /// Get cache efficiency score (0.0 to 1.0)
    pub fn efficiency_score(&self) -> f64 {
        let hit_ratio = self.hit_ratio();
        let l1_preference = self.l1_hit_ratio() * 1.5; // Prefer L1 hits

        (hit_ratio + l1_preference).min(1.0)
    }

    /// Reset all metrics
    pub fn reset(&mut self) {
        *self = Self::default();
    }

    /// Get metrics summary as string
    pub fn summary(&self) -> String {
        format!(
            "Cache Metrics: {} requests, {:.1}% hit ratio, {:.2}ms avg response",
            self.total_requests,
            self.hit_ratio() * 100.0,
            self.average_response_time_ms
        )
    }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CachePerformanceReport {
    /// The metrics value
    pub metrics: CacheMetrics,
    pub timestamp: u64,
    /// Number of period_minutes
    pub period_minutes: u32,
}

impl CachePerformanceReport {
    /// Creates a new instance
    pub fn new(metrics: CacheMetrics, period_minutes: u32) -> Self {
        Self {
            metrics,
            timestamp: std::time::SystemTime::now()
                .duration_since(std::time::UNIX_EPOCH)
                .unwrap_or_default()
                .as_secs(),
            period_minutes,
        }
    }

    pub fn recommendations(&self) -> Vec<String> {
        let mut recommendations = Vec::new();

        if self.metrics.hit_ratio() < 0.8 {
            recommendations.push("Consider increasing cache size to improve hit ratio".to_string());
        }

        if self.metrics.l1_hit_ratio() < 0.5 {
            recommendations
                .push("L1 cache may be too small, consider increasing L1 capacity".to_string());
        }

        if self.metrics.average_response_time_ms > 10.0 {
            recommendations
                .push("High average response time, consider cache optimization".to_string());
        }

        if self.metrics.total_requests > 0 && self.metrics.miss_ratio() > 0.3 {
            recommendations.push("High miss ratio detected, review caching strategy".to_string());
        }

        if recommendations.is_empty() {
            recommendations.push("Cache performance is optimal".to_string());
        }

        recommendations
    }
}

#[allow(unused_imports, clippy::nonminimal_bool, dead_code)]
#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_cache_metrics_basic() {
        let mut metrics = CacheMetrics::new();

        assert_eq!(metrics.hit_ratio(), 0.0);
        assert_eq!(metrics.total_requests, 0);

        metrics.record_hit("L1");
        metrics.record_miss();

        assert_eq!(metrics.total_requests, 2);
        assert_eq!(metrics.l1_hits, 1);
        // TEST_CATEGORY: unit
        // TEST_DOMAIN: core
        // TEST_PRIORITY: normal
        assert_eq!(metrics.total_misses, 1);
        assert!((metrics.hit_ratio() - 0.5).abs() < f64::EPSILON);
    }

    #[test]
    fn test_cache_metrics_ratios() {
        let mut metrics = CacheMetrics::new();

        // 3 L1 hits, 2 L2 hits, 1 miss
        metrics.record_hit("L1");
        metrics.record_hit("L1");
        metrics.record_hit("L1");
        metrics.record_hit("L2");
        // TEST_CATEGORY: unit
        // TEST_DOMAIN: core
        // TEST_PRIORITY: normal
        metrics.record_hit("L2");
        metrics.record_miss();

        assert_eq!(metrics.total_requests, 6);
        assert!((metrics.hit_ratio() - 5.0 / 6.0).abs() < f64::EPSILON);
        assert!((metrics.l1_hit_ratio() - 0.5).abs() < f64::EPSILON);
        assert!((metrics.miss_ratio() - 1.0 / 6.0).abs() < f64::EPSILON);
    }

    #[test]
    fn test_response_time_tracking() {
        let mut metrics = CacheMetrics::new();

        metrics.update_response_time(10.0);
        assert_eq!(metrics.average_response_time_ms, 10.0);
 // TEST_CATEGORY: unit
 // TEST_DOMAIN: core
 // TEST_PRIORITY: normal

        metrics.update_response_time(20.0);
        // Should be exponential moving average
        assert!(metrics.average_response_time_ms > 10.0);
        assert!(metrics.average_response_time_ms < 20.0);
    }

    #[test]
    fn test_performance_report() {
        let mut metrics = CacheMetrics::new();
        // TEST_CATEGORY: unit
        // TEST_DOMAIN: core
        // TEST_PRIORITY: normal
        metrics.record_hit("L1");
        metrics.record_miss();

        let report = CachePerformanceReport::new(metrics, 60);
        let recommendations = report.recommendations();

        assert!(!recommendations.is_empty());
        assert!(recommendations.iter().any(|r| r.contains("hit ratio")));
    }

    // TEST_CATEGORY: unit
    // TEST_DOMAIN: core
    // TEST_PRIORITY: normal
    #[test]
    fn test_efficiency_score() {
        let mut metrics = CacheMetrics::new();

        // Perfect L1 performance
        for _ in 0..10 {
            metrics.record_hit("L1");
        }

        let score = metrics.efficiency_score();
        assert!(score > 0.9); // Should be very high
    }
}
