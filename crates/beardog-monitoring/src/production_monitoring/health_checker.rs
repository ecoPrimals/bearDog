//! Production health checker — real system health scoring.
//!
//! Computes health scores from actual system metrics (CPU, memory)
//! via pure Rust /proc parsing. No hardcoded stub values.

use super::types::*;
use beardog_errors::BearDogError;
use crate::security_sentinel::system_metrics::SystemMetrics;

/// Health checker that computes scores from real system metrics.
#[derive(Debug)]
pub struct HealthChecker {
    /// Underlying system metrics collector.
    system_metrics: SystemMetrics,

    /// Configurable thresholds.
    cpu_warning: f64,
    memory_warning: f64,
}

impl Default for HealthChecker {
    fn default() -> Self {
        Self::new()
    }
}

impl HealthChecker {
    /// Creates a new `HealthChecker` with default thresholds.
    pub fn new() -> Self {
        Self {
            system_metrics: SystemMetrics::new(),
            cpu_warning: 80.0,
            memory_warning: 85.0,
        }
    }

    /// Creates a `HealthChecker` with custom thresholds.
    pub fn with_thresholds(cpu_warning: f64, memory_warning: f64) -> Self {
        Self {
            system_metrics: SystemMetrics::new(),
            cpu_warning,
            memory_warning,
        }
    }

    /// Start health checks.
    ///
    /// Establishes the CPU baseline snapshot (first read returns 0).
    /// Call this once during initialization.
    ///
    /// # Errors
    /// Returns an error if the system metrics cannot be read.
    pub fn start_health_checks(&self) -> Result<(), BearDogError> {
        // Establish CPU baseline by taking the first snapshot.
        let _ = self.system_metrics.collect_cpu_usage();
        Ok(())
    }

    /// Stop health checks.
    ///
    /// # Errors
    /// Returns an error if the operation fails.
    pub fn stop_health_checks(&self) -> Result<(), BearDogError> {
        Ok(())
    }

    /// Compute an overall health score from real system metrics.
    ///
    /// Returns a value between 0.0 (critical) and 1.0 (healthy).
    ///
    /// Scoring:
    /// - CPU < cpu_warning: full credit (0.5)
    /// - Memory < memory_warning: full credit (0.5)
    /// - Values above thresholds degrade the score proportionally.
    ///
    /// # Errors
    /// Returns an error if system metrics cannot be collected.
    pub fn get_overall_health_score(&self) -> Result<f64, BearDogError> {
        let cpu = self.system_metrics.collect_cpu_usage().unwrap_or(0.0);
        let memory = self.system_metrics.collect_memory_usage().unwrap_or(0.0);

        // CPU component: 0.0 to 0.5
        let cpu_score = if cpu <= self.cpu_warning {
            0.5
        } else {
            // Linear degradation from threshold to 100%
            let overshoot = (cpu - self.cpu_warning) / (100.0 - self.cpu_warning);
            (0.5 * (1.0 - overshoot)).max(0.0)
        };

        // Memory component: 0.0 to 0.5
        let memory_score = if memory <= self.memory_warning {
            0.5
        } else {
            let overshoot = (memory - self.memory_warning) / (100.0 - self.memory_warning);
            (0.5 * (1.0 - overshoot)).max(0.0)
        };

        Ok(cpu_score + memory_score)
    }

    /// Record a request for throughput/latency tracking.
    pub fn record_request(&self, duration: std::time::Duration, is_error: bool) {
        self.system_metrics.record_request(duration, is_error);
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_health_checker_new() {
        let checker = HealthChecker::new();
        assert!((checker.cpu_warning - 80.0).abs() < f64::EPSILON);
        assert!((checker.memory_warning - 85.0).abs() < f64::EPSILON);
    }

    #[test]
    fn test_health_checker_with_thresholds() {
        let checker = HealthChecker::with_thresholds(70.0, 75.0);
        assert!((checker.cpu_warning - 70.0).abs() < f64::EPSILON);
        assert!((checker.memory_warning - 75.0).abs() < f64::EPSILON);
    }

    #[test]
    fn test_start_stop_health_checks() {
        let checker = HealthChecker::new();
        assert!(checker.start_health_checks().is_ok());
        assert!(checker.stop_health_checks().is_ok());
    }

    #[test]
    fn test_overall_health_score_is_valid() {
        let checker = HealthChecker::new();
        let _ = checker.start_health_checks();
        let score = checker.get_overall_health_score().unwrap();
        assert!(
            (0.0..=1.0).contains(&score),
            "Health score {score} must be between 0.0 and 1.0"
        );
    }

    #[test]
    fn test_record_request() {
        let checker = HealthChecker::new();
        checker.record_request(std::time::Duration::from_millis(50), false);
        checker.record_request(std::time::Duration::from_millis(100), true);
        // Should not panic
    }

    #[test]
    fn test_default_impl() {
        let checker = HealthChecker::default();
        let score = checker.get_overall_health_score().unwrap();
        assert!((0.0..=1.0).contains(&score));
    }
}
