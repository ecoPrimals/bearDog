//! # Discovery Metrics
//!
//! Metrics collection and monitoring for capability discovery operations.

/// Discovery metrics
#[derive(Debug, Clone, Default)]
pub struct DiscoveryMetrics {
    /// Total number of discovery operations attempted
    pub total_discoveries: u64,
    /// Number of successful discovery operations
    pub successful_discoveries: u64,
    /// Number of failed discovery operations
    pub failed_discoveries: u64,
    /// Average discovery time in milliseconds
    pub avg_discovery_time_ms: f64,
    /// Number of capabilities cached
    pub capabilities_cached: u64,
    /// Number of cache hits
    pub cache_hits: u64,
    /// Number of cache misses
    pub cache_misses: u64,
}

impl DiscoveryMetrics {
    /// Record a successful discovery operation
    pub fn record_success(&mut self, duration_ms: u64) {
        self.total_discoveries += 1;
        self.successful_discoveries += 1;
        self.update_avg_time(duration_ms);
    }
    
    /// Record a failed discovery operation
    pub fn record_failure(&mut self, duration_ms: u64) {
        self.total_discoveries += 1;
        self.failed_discoveries += 1;
        self.update_avg_time(duration_ms);
    }
    
    /// Record a cache hit
    pub fn record_cache_hit(&mut self) {
        self.cache_hits += 1;
    }
    
    /// Record a cache miss
    pub fn record_cache_miss(&mut self) {
        self.cache_misses += 1;
    }
    
    /// Update cached capability count
    pub fn update_cached_count(&mut self, count: u64) {
        self.capabilities_cached = count;
    }
    
    /// Get success rate as a percentage
    pub fn success_rate(&self) -> f64 {
        if self.total_discoveries == 0 {
            0.0
        } else {
            (self.successful_discoveries as f64 / self.total_discoveries as f64) * 100.0
        }
    }
    
    /// Get cache hit rate as a percentage
    pub fn cache_hit_rate(&self) -> f64 {
        let total_cache_ops = self.cache_hits + self.cache_misses;
        if total_cache_ops == 0 {
            0.0
        } else {
            (self.cache_hits as f64 / total_cache_ops as f64) * 100.0
        }
    }
    
    fn update_avg_time(&mut self, duration_ms: u64) {
        let total_time = self.avg_discovery_time_ms * (self.total_discoveries - 1) as f64;
        self.avg_discovery_time_ms = (total_time + duration_ms as f64) / self.total_discoveries as f64;
    }
} 