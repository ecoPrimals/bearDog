//! Canonical Circuit Breaker Configuration
//!
//! This module provides the unified, canonical circuit breaker configuration
//! for all BearDog components (network, discovery, services, etc.).
//!
//! ## Design Principles:
//! - **Single Source of Truth**: One CircuitBreakerConfig across all domains
//! - **Failure Resilience**: Prevent cascading failures with configurable thresholds
//! - **Automatic Recovery**: Half-open state for testing recovery
//! - **Zero-Cost Abstractions**: Efficient configuration with no runtime overhead
//!
//! ## Circuit Breaker States:
//! - **Closed**: Normal operation, requests flow through
//! - **Open**: Failure threshold exceeded, requests fail fast
//! - **Half-Open**: Testing recovery, limited requests allowed
//!
//! ## Usage:
//!
//! ```rust
//! use beardog_types::canonical::config::domains::circuit_breaker::CanonicalCircuitBreakerConfig;
//!
//! let config = CanonicalCircuitBreakerConfig {
//!     enabled: true,
//!     failure_threshold: 5,
//!     success_threshold: 2,
//!     timeout_seconds: 60,
//!     ..Default::default()
//! };
//! ```

use serde::{Deserialize, Serialize};
use std::time::Duration;

/// Canonical circuit breaker configuration for all BearDog components
///
/// This is the single, unified circuit breaker configuration.
/// All domain-specific CircuitBreakerConfigs should migrate to this.
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct CanonicalCircuitBreakerConfig {
    /// Whether circuit breaker is enabled
    pub enabled: bool,

    /// Number of consecutive failures to open the circuit
    pub failure_threshold: u32,

    /// Number of consecutive successes in half-open state to close circuit
    pub success_threshold: u32,

    /// Time to wait in open state before transitioning to half-open (seconds)
    pub timeout_seconds: u64,

    /// Time window for counting failures (sliding window)
    pub window_duration: Duration,

    /// Minimum throughput required for circuit breaker decisions
    pub min_throughput: u32,

    /// Error threshold percentage (0-100) for opening circuit
    pub error_threshold_percentage: f64,

    /// Whether to enable half-open state
    pub half_open_enabled: bool,

    /// Maximum requests allowed in half-open state
    pub half_open_max_requests: u32,
}

/// Type alias for convenience (follows BearDog naming conventions)
pub type CircuitBreakerConfig = CanonicalCircuitBreakerConfig;

impl Default for CanonicalCircuitBreakerConfig {
    fn default() -> Self {
        Self {
            enabled: true,
            failure_threshold: 5,
            success_threshold: 2,
            timeout_seconds: 60,
            window_duration: Duration::from_secs(60),
            min_throughput: 10,
            error_threshold_percentage: 50.0,
            half_open_enabled: true,
            half_open_max_requests: 3,
        }
    }
}

impl CanonicalCircuitBreakerConfig {
    /// Create a new circuit breaker config with custom failure threshold
    pub fn with_failure_threshold(mut self, threshold: u32) -> Self {
        self.failure_threshold = threshold;
        self
    }

    /// Create a new circuit breaker config with custom timeout
    pub fn with_timeout(mut self, timeout_seconds: u64) -> Self {
        self.timeout_seconds = timeout_seconds;
        self
    }

    /// Create a new circuit breaker config with custom error percentage
    pub fn with_error_percentage(mut self, percentage: f64) -> Self {
        self.error_threshold_percentage = percentage.clamp(0.0, 100.0);
        self
    }

    /// Validate the configuration
    pub fn validate(&self) -> Result<(), String> {
        if self.failure_threshold == 0 {
            return Err("failure_threshold must be > 0".to_string());
        }
        if self.success_threshold == 0 {
            return Err("success_threshold must be > 0".to_string());
        }
        if self.error_threshold_percentage < 0.0 || self.error_threshold_percentage > 100.0 {
            return Err("error_threshold_percentage must be between 0.0 and 100.0".to_string());
        }
        if self.half_open_max_requests == 0 && self.half_open_enabled {
            return Err("half_open_max_requests must be > 0 when half_open_enabled".to_string());
        }
        Ok(())
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_default_config() {
        let config = CanonicalCircuitBreakerConfig::default();
        assert!(config.enabled);
        assert_eq!(config.failure_threshold, 5);
        assert_eq!(config.success_threshold, 2);
        assert_eq!(config.timeout_seconds, 60);
        assert!(config.validate().is_ok());
    }

    #[test]
    fn test_builder_pattern() {
        let config = CanonicalCircuitBreakerConfig::default()
            .with_failure_threshold(10)
            .with_timeout(120)
            .with_error_percentage(75.0);

        assert_eq!(config.failure_threshold, 10);
        assert_eq!(config.timeout_seconds, 120);
        assert_eq!(config.error_threshold_percentage, 75.0);
    }

    #[test]
    fn test_validation() {
        let mut config = CanonicalCircuitBreakerConfig::default();

        // Valid config
        assert!(config.validate().is_ok());

        // Invalid: zero failure threshold
        config.failure_threshold = 0;
        assert!(config.validate().is_err());

        // Invalid: error percentage > 100
        config.failure_threshold = 5;
        config.error_threshold_percentage = 150.0;
        assert!(config.validate().is_err());
    }

    #[test]
    fn test_serialization() {
        let config = CanonicalCircuitBreakerConfig {
            enabled: true,
            failure_threshold: 10,
            success_threshold: 3,
            timeout_seconds: 90,
            window_duration: Duration::from_secs(120),
            min_throughput: 20,
            error_threshold_percentage: 60.0,
            half_open_enabled: false,
            half_open_max_requests: 5,
        };

        let json = serde_json::to_string(&config).unwrap();
        let deserialized: CanonicalCircuitBreakerConfig = serde_json::from_str(&json).unwrap();
        assert_eq!(deserialized, config);
    }

    #[test]
    fn test_type_alias() {
        // Ensure type alias works correctly
        let _config: CircuitBreakerConfig = CanonicalCircuitBreakerConfig::default();
    }

    #[test]
    fn test_error_percentage_clamping() {
        let config = CanonicalCircuitBreakerConfig::default()
            .with_error_percentage(150.0); // Should clamp to 100.0

        assert_eq!(config.error_threshold_percentage, 100.0);

        let config = CanonicalCircuitBreakerConfig::default()
            .with_error_percentage(-50.0); // Should clamp to 0.0

        assert_eq!(config.error_threshold_percentage, 0.0);
    }
}

