//! # Performance Metrics Module
//!
//! This module provides performance metrics functionality.

/// Performance metrics
#[derive(Debug, Clone, Default)]
pub struct PerformanceMetrics {
    /// Total operations
    pub total_operations: u64,
    /// Average response time
    pub avg_response_time_ms: f64,
}

impl PerformanceMetrics {
    /// Creates new metrics
    pub fn new() -> Self {
        Self::default()
    }

    /// Records an operation
    pub fn record_operation(&mut self, response_time_ms: f64) {
        self.total_operations += 1;
        self.avg_response_time_ms = 
            (self.avg_response_time_ms * (self.total_operations - 1) as f64 + response_time_ms) 
            / self.total_operations as f64;
    }
} 