// SPDX-License-Identifier: AGPL-3.0-or-later

//! Concurrent-Safe Monitoring Configuration Module
//!
//! Monitoring, logging, and metrics configuration for `BearDog`.
//!
//! ## Design Pattern: Explicit Environment Loading
//!
//! To ensure **concurrent safety** and **testability**, this module separates:
//! - **Static defaults** (`Default` trait) - Pure, no environment reads
//! - **Environment loading** (`from_env()`) - Explicit environment variable reads
//! - **Flexible construction** (`builder()`) - Testing without env var pollution

use crate::error::{ConfigError, ConfigResult};
use serde::{Deserialize, Serialize};

/// Monitoring, logging, and metrics configuration
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct MonitoringConfig {
    /// Log level (trace, debug, info, warn, error)
    pub log_level: String,

    /// Log format (json, text)
    pub log_format: String,

    /// Enable structured logging
    pub structured_logging: bool,

    /// Enable metrics collection
    pub enable_metrics: bool,

    /// Metrics port (0 = disabled)
    pub metrics_port: u16,

    /// Enable health check endpoint
    pub enable_health_check: bool,

    /// Health check port
    pub health_check_port: u16,

    /// Enable performance tracking
    pub enable_performance_tracking: bool,

    /// Tracing sample rate (0.0 to 1.0)
    pub tracing_sample_rate: f64,
}

impl MonitoringConfig {
    /// Pure static defaults (no environment variable reads)
    pub fn const_defaults() -> Self {
        Self {
            log_level: "info".to_string(),
            log_format: "text".to_string(),
            structured_logging: true,
            enable_metrics: true,
            metrics_port: 9092,
            enable_health_check: true,
            health_check_port: 9093,
            enable_performance_tracking: false,
            tracing_sample_rate: 0.1, // 10% sampling
        }
    }

    /// Load configuration from environment variables with fallback to defaults
    pub fn from_env() -> Self {
        let defaults = Self::const_defaults();

        Self {
            log_level: std::env::var("BEARDOG_LOG_LEVEL")
                .ok()
                .unwrap_or(defaults.log_level),

            log_format: std::env::var("BEARDOG_LOG_FORMAT")
                .ok()
                .unwrap_or(defaults.log_format),

            structured_logging: defaults.structured_logging,
            enable_metrics: defaults.enable_metrics,

            metrics_port: std::env::var("BEARDOG_METRICS_PORT")
                .ok()
                .and_then(|s| s.parse().ok())
                .unwrap_or(defaults.metrics_port),

            enable_health_check: defaults.enable_health_check,

            health_check_port: std::env::var("BEARDOG_HEALTH_PORT")
                .ok()
                .and_then(|s| s.parse().ok())
                .unwrap_or(defaults.health_check_port),

            enable_performance_tracking: defaults.enable_performance_tracking,

            tracing_sample_rate: std::env::var("BEARDOG_TRACING_SAMPLE_RATE")
                .ok()
                .and_then(|s| s.parse().ok())
                .unwrap_or(defaults.tracing_sample_rate),
        }
    }

    /// Create a builder for flexible configuration construction
    pub fn builder() -> MonitoringConfigBuilder {
        MonitoringConfigBuilder::new()
    }

    /// Validate monitoring configuration
    ///
    /// # Errors
    ///
    /// Returns [`ConfigError`] when log level, format, or sampling values are invalid.
    pub fn validate(&self) -> ConfigResult<()> {
        // Validate log level
        let valid_levels = ["trace", "debug", "info", "warn", "error"];
        if !valid_levels.contains(&self.log_level.as_str()) {
            return Err(ConfigError::invalid_value(
                "monitoring.log_level",
                "Must be one of: trace, debug, info, warn, error",
            ));
        }

        // Validate log format
        let valid_formats = ["json", "text"];
        if !valid_formats.contains(&self.log_format.as_str()) {
            return Err(ConfigError::invalid_value(
                "monitoring.log_format",
                "Must be either 'json' or 'text'",
            ));
        }

        // Validate tracing sample rate
        if !(0.0..=1.0).contains(&self.tracing_sample_rate) {
            return Err(ConfigError::invalid_value(
                "monitoring.tracing_sample_rate",
                "Must be between 0.0 and 1.0",
            ));
        }

        Ok(())
    }
}

impl Default for MonitoringConfig {
    fn default() -> Self {
        Self::const_defaults()
    }
}

/// Builder for `MonitoringConfig`
#[derive(Debug, Default)]
pub struct MonitoringConfigBuilder {
    log_level: Option<String>,
    log_format: Option<String>,
    structured_logging: Option<bool>,
    enable_metrics: Option<bool>,
    metrics_port: Option<u16>,
    enable_health_check: Option<bool>,
    health_check_port: Option<u16>,
    enable_performance_tracking: Option<bool>,
    tracing_sample_rate: Option<f64>,
}

impl MonitoringConfigBuilder {
    /// Starts a builder; unspecified options inherit [`MonitoringConfig::const_defaults`].
    pub fn new() -> Self {
        Self::default()
    }

    /// Log verbosity name (e.g. `info`, `debug`) for process-wide logging.
    pub fn log_level(mut self, level: String) -> Self {
        self.log_level = Some(level);
        self
    }

    /// Output format for log records (e.g. JSON vs plain text), when the runtime supports it.
    pub fn log_format(mut self, format: String) -> Self {
        self.log_format = Some(format);
        self
    }

    /// Enables structured key/value fields instead of unstructured lines.
    pub const fn structured_logging(mut self, enabled: bool) -> Self {
        self.structured_logging = Some(enabled);
        self
    }

    /// Turns Prometheus-style or internal metrics collection on or off.
    pub const fn enable_metrics(mut self, enabled: bool) -> Self {
        self.enable_metrics = Some(enabled);
        self
    }

    /// TCP port bound for scraping or exporting metrics.
    pub const fn metrics_port(mut self, port: u16) -> Self {
        self.metrics_port = Some(port);
        self
    }

    /// Exposes a lightweight HTTP (or similar) health endpoint when enabled.
    pub const fn enable_health_check(mut self, enabled: bool) -> Self {
        self.enable_health_check = Some(enabled);
        self
    }

    /// Port for the health check listener.
    pub const fn health_check_port(mut self, port: u16) -> Self {
        self.health_check_port = Some(port);
        self
    }

    /// Collects latency and throughput samples for operational dashboards.
    pub const fn enable_performance_tracking(mut self, enabled: bool) -> Self {
        self.enable_performance_tracking = Some(enabled);
        self
    }

    /// Fraction of distributed traces to retain (`0.0`–`1.0`) to control overhead.
    pub const fn tracing_sample_rate(mut self, rate: f64) -> Self {
        self.tracing_sample_rate = Some(rate);
        self
    }

    /// Builds [`MonitoringConfig`] with defaults for any unset fields.
    pub fn build(self) -> MonitoringConfig {
        let defaults = MonitoringConfig::const_defaults();

        MonitoringConfig {
            log_level: self.log_level.unwrap_or(defaults.log_level),
            log_format: self.log_format.unwrap_or(defaults.log_format),
            structured_logging: self
                .structured_logging
                .unwrap_or(defaults.structured_logging),
            enable_metrics: self.enable_metrics.unwrap_or(defaults.enable_metrics),
            metrics_port: self.metrics_port.unwrap_or(defaults.metrics_port),
            enable_health_check: self
                .enable_health_check
                .unwrap_or(defaults.enable_health_check),
            health_check_port: self.health_check_port.unwrap_or(defaults.health_check_port),
            enable_performance_tracking: self
                .enable_performance_tracking
                .unwrap_or(defaults.enable_performance_tracking),
            tracing_sample_rate: self
                .tracing_sample_rate
                .unwrap_or(defaults.tracing_sample_rate),
        }
    }
}

#[cfg(test)]
#[path = "monitoring_comprehensive_tests.rs"]
mod monitoring_comprehensive_tests;

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_default_monitoring() {
        let config = MonitoringConfig::default();
        assert!(config.validate().is_ok());
        assert_eq!(config.log_level, "info");
        assert_eq!(config.log_format, "text");
    }

    #[test]
    fn test_invalid_log_level() {
        let config = MonitoringConfig::builder()
            .log_level("invalid".to_string())
            .build();
        assert!(config.validate().is_err());
    }

    #[test]
    fn test_invalid_tracing_rate() {
        let config = MonitoringConfig::builder()
            .tracing_sample_rate(1.5) // Out of range
            .build();
        assert!(config.validate().is_err());
    }

    #[test]
    fn test_builder() {
        let config = MonitoringConfig::builder()
            .log_level("debug".to_string())
            .log_format("json".to_string())
            .metrics_port(8888)
            .build();

        assert_eq!(config.log_level, "debug");
        assert_eq!(config.log_format, "json");
        assert_eq!(config.metrics_port, 8888);
    }

    #[test]
    fn test_production_monitoring() {
        let config = MonitoringConfig::builder()
            .log_level("warn".to_string())
            .log_format("json".to_string())
            .structured_logging(true)
            .enable_metrics(true)
            .enable_performance_tracking(true)
            .tracing_sample_rate(0.5)
            .build();

        assert!(config.validate().is_ok());
        assert_eq!(config.log_level, "warn");
        assert!(config.enable_performance_tracking);
    }
}
