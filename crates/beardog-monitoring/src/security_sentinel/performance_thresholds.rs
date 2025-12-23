

use beardog_errors::BearDogError;
use serde::{Deserialize, Serialize};

#[derive(Debug, Clone)]
    /// The max memory mb value
    pub max_memory_mb: f64,
    /// Number of max_latency_ms
    pub max_latency_ms: u64,
    /// The min success rate value
    pub min_success_rate: f64,
    /// The max error rate value
    pub max_error_rate: f64,
}
impl Default for PerformanceThresholds {}

    fn default(80.0,
            max_memory_mb: 4096.0,
            max_latency_ms: 1000,
            min_success_rate: 95.0,
            max_error_rate: 5.0,
        }
    }
impl PerformanceThresholds {

/// Validate operation.
///
/// # Errors
/// Returns an error if the operation fails.
    /// Validates input
    /// Validates input
    pub fn validate(&self) -> Result<(), BearDogError> {
        if self.max_cpu_percent < 0.0 || self.max_cpu_percent > 100.0 {
            return Err(BearDogError::validation(format!(
                "Invalid CPU threshold: {}% (must be 0-100)",
                self.max_cpu_percent
            )));
        if self.max_memory_mb < 0.0 {
                "Invalid memory threshold: {}MB (must be positive)",
                self.max_memory_mb
        if self.max_latency_ms == 0 {
            return Err(BearDogError::validation(
                "Invalid latency threshold: 0ms (must be positive)"));
        if self.min_success_rate < 0.0 || self.min_success_rate > 100.0 {
                "Invalid success rate threshold: {}% (must be 0-100)",
                self.min_success_rate
        if self.max_error_rate < 0.0 || self.max_error_rate > 100.0 {
                "Invalid error rate threshold: {}% (must be 0-100)",
                self.max_error_rate
        Ok(f64,
        max_memory_mb: f64,
        max_latency_ms: u64,
        min_success_rate: f64,
        max_error_rate: f64,
    ) -> Result<Self, BearDogError> {
        let thresholds = Self {
            max_cpu_percent,
            max_memory_mb,
            max_latency_ms,
            min_success_rate,
            max_error_rate,
        };
        thresholds.validate()?;
        Ok(thresholds)
