// SPDX-License-Identifier: AGPL-3.0-or-later

//! # Performance Thresholds
//!
//! This module provides threshold types for performance monitoring
//! in the security sentinel system.

use beardog_errors::BearDogError;
use serde::{Deserialize, Serialize};

/// Performance thresholds for monitoring
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PerformanceThresholds {
    /// Maximum CPU usage percentage
    pub max_cpu_percent: f64,

    /// Maximum memory usage in MB
    pub max_memory_mb: f64,

    /// Maximum latency in milliseconds
    pub max_latency_ms: u64,

    /// Minimum success rate percentage
    pub min_success_rate: f64,

    /// Maximum error rate percentage
    pub max_error_rate: f64,
}

impl Default for PerformanceThresholds {
    fn default() -> Self {
        Self {
            max_cpu_percent: 80.0,
            max_memory_mb: 4096.0,
            max_latency_ms: 1000,
            min_success_rate: 95.0,
            max_error_rate: 5.0,
        }
    }
}

impl PerformanceThresholds {
    /// Create new thresholds with validation
    ///
    /// # Errors
    /// Returns an error if any threshold value is invalid.
    pub fn new(
        max_cpu_percent: f64,
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
    }

    /// Validate threshold values
    ///
    /// # Errors
    /// Returns an error if any threshold value is invalid.
    pub fn validate(&self) -> Result<(), BearDogError> {
        if self.max_cpu_percent < 0.0 || self.max_cpu_percent > 100.0 {
            return Err(BearDogError::validation(format!(
                "Invalid CPU threshold: {}% (must be 0-100)",
                self.max_cpu_percent
            )));
        }

        if self.max_memory_mb < 0.0 {
            return Err(BearDogError::validation(format!(
                "Invalid memory threshold: {}MB (must be positive)",
                self.max_memory_mb
            )));
        }

        if self.max_latency_ms == 0 {
            return Err(BearDogError::validation(
                "Invalid latency threshold: 0ms (must be positive)".to_string(),
            ));
        }

        if self.min_success_rate < 0.0 || self.min_success_rate > 100.0 {
            return Err(BearDogError::validation(format!(
                "Invalid success rate threshold: {}% (must be 0-100)",
                self.min_success_rate
            )));
        }

        if self.max_error_rate < 0.0 || self.max_error_rate > 100.0 {
            return Err(BearDogError::validation(format!(
                "Invalid error rate threshold: {}% (must be 0-100)",
                self.max_error_rate
            )));
        }

        Ok(())
    }

    /// Check if CPU usage exceeds threshold
    pub fn is_cpu_exceeded(&self, cpu_percent: f64) -> bool {
        cpu_percent > self.max_cpu_percent
    }

    /// Check if memory usage exceeds threshold
    pub fn is_memory_exceeded(&self, memory_mb: f64) -> bool {
        memory_mb > self.max_memory_mb
    }

    /// Check if latency exceeds threshold
    pub fn is_latency_exceeded(&self, latency_ms: u64) -> bool {
        latency_ms > self.max_latency_ms
    }

    /// Check if success rate is below threshold
    pub fn is_success_rate_low(&self, success_rate: f64) -> bool {
        success_rate < self.min_success_rate
    }

    /// Check if error rate exceeds threshold
    pub fn is_error_rate_exceeded(&self, error_rate: f64) -> bool {
        error_rate > self.max_error_rate
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_default_thresholds() {
        let thresholds = PerformanceThresholds::default();
        assert_eq!(thresholds.max_cpu_percent, 80.0);
        assert_eq!(thresholds.max_memory_mb, 4096.0);
        assert_eq!(thresholds.max_latency_ms, 1000);
        assert!(thresholds.validate().is_ok());
    }

    #[test]
    fn test_new_with_valid_values() {
        let result = PerformanceThresholds::new(75.0, 2048.0, 500, 90.0, 10.0);
        assert!(result.is_ok());
    }

    #[test]
    fn test_validation_invalid_cpu() {
        let thresholds = PerformanceThresholds {
            max_cpu_percent: 150.0,
            ..Default::default()
        };
        assert!(thresholds.validate().is_err());
    }

    #[test]
    fn test_threshold_checks() {
        let thresholds = PerformanceThresholds::default();
        assert!(!thresholds.is_cpu_exceeded(50.0));
        assert!(thresholds.is_cpu_exceeded(90.0));
        assert!(!thresholds.is_latency_exceeded(500));
        assert!(thresholds.is_latency_exceeded(1500));
    }
}
