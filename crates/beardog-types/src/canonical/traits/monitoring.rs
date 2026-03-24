// SPDX-License-Identifier: AGPL-3.0-only

//! # Monitoring Configuration Trait
//!
//! This module provides a polymorphic interface for monitoring configurations across different
//! domains (metrics, logging, tracing, alerting) while preserving their unique monitoring features.
//!
//! ## Design Rationale
//!
//! Rather than forcing all monitoring configs into a single struct, we provide a common
//! trait interface that enables:
//! - **Polymorphic monitoring setup**: Functions that work with any monitoring config
//! - **Domain preservation**: Each config retains its domain-specific features
//! - **Type safety**: Compiler-enforced monitoring policies
//! - **Easy extension**: New monitoring configs just implement the trait
//!
//! ## Example Usage
//!
//! ```rust
//! use beardog_types::canonical::traits::MonitoringConfig;
//! use std::time::Duration;
//!
//! fn setup_monitoring<M: MonitoringConfig>(config: &M) {
//!     if config.is_enabled() {
//!         println!("Monitoring enabled");
//!         println!("Metrics endpoint: {}", config.metrics_endpoint());
//!         println!("Reporting every: {:?}", config.reporting_interval());
//!         
//!         if config.detailed_metrics() {
//!             println!("Detailed metrics enabled");
//!         }
//!     }
//! }
//! ```

use std::time::Duration;

/// Monitoring level enum
///
/// Defines the verbosity and detail level of monitoring data collection.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub enum MonitoringLevel {
    /// Minimal monitoring - only critical metrics
    ///
    /// Use for: Production with strict performance requirements
    Minimal,

    /// Basic monitoring - essential metrics only
    ///
    /// Use for: Standard production deployments
    Basic,

    /// Standard monitoring - common metrics and some details
    ///
    /// Use for: Most production scenarios (recommended)
    Standard,

    /// Detailed monitoring - comprehensive metrics
    ///
    /// Use for: Debugging, performance analysis
    Detailed,

    /// Verbose monitoring - all available metrics
    ///
    /// Use for: Development, troubleshooting
    Verbose,
}

impl MonitoringLevel {
    /// Returns true if this level includes detailed metrics
    pub const fn includes_detailed_metrics(&self) -> bool {
        matches!(self, Self::Detailed | Self::Verbose)
    }

    /// Returns true if this level includes trace data
    pub const fn includes_tracing(&self) -> bool {
        matches!(self, Self::Standard | Self::Detailed | Self::Verbose)
    }

    /// Returns the relative performance overhead of this level
    ///
    /// Returns a value from 1 (lowest) to 5 (highest)
    pub const fn overhead_level(&self) -> u8 {
        match self {
            Self::Minimal => 1,
            Self::Basic => 2,
            Self::Standard => 3,
            Self::Detailed => 4,
            Self::Verbose => 5,
        }
    }

    /// Returns the typical reporting interval for this level
    pub const fn typical_interval(&self) -> Duration {
        match self {
            Self::Minimal => Duration::from_secs(300), // 5 minutes
            Self::Basic => Duration::from_secs(120),   // 2 minutes
            Self::Standard => Duration::from_secs(60), // 1 minute
            Self::Detailed => Duration::from_secs(30), // 30 seconds
            Self::Verbose => Duration::from_secs(10),  // 10 seconds
        }
    }
}

impl std::fmt::Display for MonitoringLevel {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::Minimal => write!(f, "Minimal"),
            Self::Basic => write!(f, "Basic"),
            Self::Standard => write!(f, "Standard"),
            Self::Detailed => write!(f, "Detailed"),
            Self::Verbose => write!(f, "Verbose"),
        }
    }
}

/// Trait for monitoring configuration
///
/// Provides a common interface for monitoring settings across different domains while
/// allowing each implementation to maintain domain-specific monitoring features.
///
/// ## Monitoring Properties
///
/// - **Enabled State**: Whether monitoring is active
/// - **Metrics Endpoint**: Where metrics are reported
/// - **Reporting Interval**: How often metrics are sent
/// - **Detail Level**: How much data is collected
///
/// ## Thread Safety
///
/// The trait requires `Send + Sync` to enable use in async contexts and across threads.
pub trait MonitoringConfig: Send + Sync {
    /// Whether monitoring is enabled
    ///
    /// Returns `false` if monitoring is completely disabled.
    /// All other methods may still return valid values for configuration purposes.
    fn is_enabled(&self) -> bool;

    /// Metrics endpoint URL or address
    ///
    /// Returns the destination where metrics should be reported.
    /// This could be:
    /// - HTTP endpoint: `http://metrics.example.com:9090/api/v1/metrics`
    /// - Local file: `/var/log/beardog/metrics.log`
    /// - Service name: `prometheus` (for service discovery)
    ///
    /// ## Returns
    /// The metrics endpoint as a string slice
    fn metrics_endpoint(&self) -> &str;

    /// Reporting interval
    ///
    /// Returns how often metrics should be reported/collected.
    ///
    /// ## Typical Values
    /// - Real-time monitoring: 5-15 seconds
    /// - Standard monitoring: 30-60 seconds
    /// - Low-overhead monitoring: 2-5 minutes
    fn reporting_interval(&self) -> Duration;

    /// Whether to include detailed metrics
    ///
    /// Returns `true` if detailed/verbose metrics should be collected.
    /// Detailed metrics include:
    /// - Per-operation timings
    /// - Memory usage breakdowns
    /// - Network traffic details
    /// - Error stack traces
    ///
    /// ## Default Implementation
    /// Returns `false` (summary metrics only) for lower overhead.
    fn detailed_metrics(&self) -> bool {
        false // Summary by default for performance
    }

    /// Monitoring level
    ///
    /// Returns the verbosity level for monitoring data collection.
    ///
    /// ## Default Implementation
    /// Returns `MonitoringLevel::Standard` for balanced monitoring.
    fn monitoring_level(&self) -> MonitoringLevel {
        MonitoringLevel::Standard
    }

    /// Whether health checks are enabled
    ///
    /// Returns `true` if the system should expose health check endpoints.
    ///
    /// ## Default Implementation
    /// Returns `true` - health checks are generally essential.
    fn health_checks_enabled(&self) -> bool {
        true // Health checks are important
    }

    /// Whether distributed tracing is enabled
    ///
    /// Returns `true` if distributed tracing should be active.
    /// Tracing helps track requests across services.
    ///
    /// ## Default Implementation
    /// Returns `false` - tracing adds overhead, enable explicitly.
    fn tracing_enabled(&self) -> bool {
        false // Disabled by default due to overhead
    }

    /// Whether alerting is enabled
    ///
    /// Returns `true` if the monitoring system should generate alerts
    /// based on thresholds and conditions.
    fn alerting_enabled(&self) -> bool {
        false // Disabled by default
    }

    /// Sample rate for metrics (0.0 to 1.0)
    ///
    /// Returns the fraction of events that should be sampled.
    /// - `1.0` = Sample everything (highest overhead)
    /// - `0.1` = Sample 10% of events
    /// - `0.0` = No sampling (monitoring effectively disabled)
    ///
    /// ## Use Cases
    /// - High-traffic systems: 0.01 - 0.1 (1-10%)
    /// - Standard systems: 1.0 (100%)
    /// - Development: 1.0 (100%)
    fn sample_rate(&self) -> f64 {
        1.0 // 100% sampling by default
    }

    /// Calculate expected monitoring overhead
    ///
    /// Returns an estimate of the CPU overhead (0.0 to 1.0) for monitoring.
    /// - `0.0` = No overhead (monitoring disabled)
    /// - `0.05` = 5% CPU overhead (typical for standard monitoring)
    /// - `0.20` = 20% CPU overhead (verbose monitoring)
    fn estimated_overhead(&self) -> f64 {
        if !self.is_enabled() {
            return 0.0;
        }

        let level_overhead = match self.monitoring_level() {
            MonitoringLevel::Minimal => 0.01,
            MonitoringLevel::Basic => 0.02,
            MonitoringLevel::Standard => 0.05,
            MonitoringLevel::Detailed => 0.10,
            MonitoringLevel::Verbose => 0.20,
        };

        let tracing_overhead = if self.tracing_enabled() { 0.05 } else { 0.0 };
        let detailed_overhead = if self.detailed_metrics() { 0.03 } else { 0.0 };

        (level_overhead + tracing_overhead + detailed_overhead) * self.sample_rate()
    }

    /// Validate the monitoring configuration
    ///
    /// Checks that the configuration is valid and reasonable.
    /// Returns `Ok(())` if valid, or `Err(String)` with error description.
    fn validate(&self) -> Result<(), String> {
        // Check if disabled
        if !self.is_enabled() {
            return Ok(()); // Disabled monitoring is valid
        }

        // Check endpoint
        let endpoint = self.metrics_endpoint();
        if endpoint.is_empty() {
            return Err("Metrics endpoint cannot be empty when monitoring is enabled".to_string());
        }

        // Check interval
        let interval = self.reporting_interval();
        if interval == Duration::ZERO {
            return Err("Reporting interval cannot be zero".to_string());
        }

        if interval < Duration::from_secs(1) {
            eprintln!(
                "WARNING: Very short reporting interval ({interval:?}), may cause performance issues"
            );
        }

        if interval > Duration::from_secs(600) {
            eprintln!("WARNING: Very long reporting interval ({interval:?}), metrics may be stale");
        }

        // Check sample rate
        let sample_rate = self.sample_rate();
        if !(0.0..=1.0).contains(&sample_rate) {
            return Err(format!(
                "Sample rate must be between 0.0 and 1.0, got {sample_rate}"
            ));
        }

        // Check overhead
        let overhead = self.estimated_overhead();
        if overhead > 0.25 {
            eprintln!(
                "WARNING: High monitoring overhead ({:.1}%), may impact performance",
                overhead * 100.0
            );
        }

        Ok(())
    }

    /// Returns true if the configuration is suitable for production
    ///
    /// Production configurations should have:
    /// - Monitoring enabled
    /// - Reasonable reporting intervals
    /// - Acceptable overhead levels
    /// - Valid endpoints
    fn is_production_ready(&self) -> bool {
        // Must be valid
        if self.validate().is_err() {
            return false;
        }

        // Must be enabled
        if !self.is_enabled() {
            return false; // Production should have monitoring
        }

        // Reasonable interval
        let interval = self.reporting_interval();
        if interval < Duration::from_secs(10) || interval > Duration::from_secs(300) {
            return false; // Too frequent or too infrequent
        }

        // Acceptable overhead
        if self.estimated_overhead() > 0.15 {
            return false; // >15% overhead is too high for production
        }

        // Should have health checks
        if !self.health_checks_enabled() {
            return false;
        }

        true
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    // Test-only [`MonitoringConfig`] implementation.
    struct TestMonitoringConfig {
        enabled: bool,
        endpoint: String,
        interval: Duration,
        detailed: bool,
        level: MonitoringLevel,
        health_checks: bool,
        tracing: bool,
        alerting: bool,
        sample_rate: f64,
    }

    impl MonitoringConfig for TestMonitoringConfig {
        fn is_enabled(&self) -> bool {
            self.enabled
        }

        fn metrics_endpoint(&self) -> &str {
            &self.endpoint
        }

        fn reporting_interval(&self) -> Duration {
            self.interval
        }

        fn detailed_metrics(&self) -> bool {
            self.detailed
        }

        fn monitoring_level(&self) -> MonitoringLevel {
            self.level
        }

        fn health_checks_enabled(&self) -> bool {
            self.health_checks
        }

        fn tracing_enabled(&self) -> bool {
            self.tracing
        }

        fn alerting_enabled(&self) -> bool {
            self.alerting
        }

        fn sample_rate(&self) -> f64 {
            self.sample_rate
        }
    }

    #[test]
    fn test_monitoring_level_properties() {
        assert!(!MonitoringLevel::Basic.includes_detailed_metrics());
        assert!(MonitoringLevel::Detailed.includes_detailed_metrics());
        assert!(MonitoringLevel::Verbose.includes_detailed_metrics());

        assert!(!MonitoringLevel::Minimal.includes_tracing());
        assert!(MonitoringLevel::Standard.includes_tracing());
        assert!(MonitoringLevel::Verbose.includes_tracing());
    }

    #[test]
    fn test_monitoring_level_overhead() {
        assert_eq!(MonitoringLevel::Minimal.overhead_level(), 1);
        assert_eq!(MonitoringLevel::Basic.overhead_level(), 2);
        assert_eq!(MonitoringLevel::Standard.overhead_level(), 3);
        assert_eq!(MonitoringLevel::Detailed.overhead_level(), 4);
        assert_eq!(MonitoringLevel::Verbose.overhead_level(), 5);
    }

    #[test]
    fn test_basic_config() {
        let config = TestMonitoringConfig {
            enabled: true,
            endpoint: "http://metrics.example.com:9090".to_string(),
            interval: Duration::from_secs(60),
            detailed: false,
            level: MonitoringLevel::Standard,
            health_checks: true,
            tracing: false,
            alerting: false,
            sample_rate: 1.0,
        };

        assert!(config.is_enabled());
        assert_eq!(config.metrics_endpoint(), "http://metrics.example.com:9090");
        assert_eq!(config.reporting_interval(), Duration::from_secs(60));
        assert!(!config.detailed_metrics());
        assert_eq!(config.monitoring_level(), MonitoringLevel::Standard);
    }

    #[test]
    fn test_overhead_calculation() {
        // Minimal config
        let minimal = TestMonitoringConfig {
            enabled: true,
            endpoint: "http://metrics.example.com".to_string(),
            interval: Duration::from_secs(60),
            detailed: false,
            level: MonitoringLevel::Minimal,
            health_checks: true,
            tracing: false,
            alerting: false,
            sample_rate: 1.0,
        };
        assert_eq!(minimal.estimated_overhead(), 0.01);

        // Verbose with tracing
        let verbose = TestMonitoringConfig {
            enabled: true,
            endpoint: "http://metrics.example.com".to_string(),
            interval: Duration::from_secs(10),
            detailed: true,
            level: MonitoringLevel::Verbose,
            health_checks: true,
            tracing: true,
            alerting: true,
            sample_rate: 1.0,
        };
        assert_eq!(verbose.estimated_overhead(), 0.28); // 0.20 + 0.05 + 0.03

        // With sampling
        let sampled = TestMonitoringConfig {
            enabled: true,
            endpoint: "http://metrics.example.com".to_string(),
            interval: Duration::from_secs(60),
            detailed: false,
            level: MonitoringLevel::Standard,
            health_checks: true,
            tracing: false,
            alerting: false,
            sample_rate: 0.1, // 10% sampling
        };
        let overhead = sampled.estimated_overhead();
        assert!(
            (overhead - 0.005).abs() < 0.0001,
            "Expected ~0.005, got {overhead}"
        ); // 0.05 * 0.1
    }

    #[test]
    fn test_validation() {
        // Valid config
        let valid = TestMonitoringConfig {
            enabled: true,
            endpoint: "http://metrics.example.com".to_string(),
            interval: Duration::from_secs(60),
            detailed: false,
            level: MonitoringLevel::Standard,
            health_checks: true,
            tracing: false,
            alerting: false,
            sample_rate: 1.0,
        };
        assert!(valid.validate().is_ok());

        // Invalid: empty endpoint
        let invalid1 = TestMonitoringConfig {
            enabled: true,
            endpoint: String::new(),
            interval: Duration::from_secs(60),
            detailed: false,
            level: MonitoringLevel::Standard,
            health_checks: true,
            tracing: false,
            alerting: false,
            sample_rate: 1.0,
        };
        assert!(invalid1.validate().is_err());

        // Invalid: zero interval
        let invalid2 = TestMonitoringConfig {
            enabled: true,
            endpoint: "http://metrics.example.com".to_string(),
            interval: Duration::ZERO,
            detailed: false,
            level: MonitoringLevel::Standard,
            health_checks: true,
            tracing: false,
            alerting: false,
            sample_rate: 1.0,
        };
        assert!(invalid2.validate().is_err());

        // Invalid: sample rate out of range
        let invalid3 = TestMonitoringConfig {
            enabled: true,
            endpoint: "http://metrics.example.com".to_string(),
            interval: Duration::from_secs(60),
            detailed: false,
            level: MonitoringLevel::Standard,
            health_checks: true,
            tracing: false,
            alerting: false,
            sample_rate: 1.5, // Invalid: > 1.0
        };
        assert!(invalid3.validate().is_err());
    }

    #[test]
    fn test_production_ready() {
        // Production ready
        let prod = TestMonitoringConfig {
            enabled: true,
            endpoint: "http://metrics.example.com".to_string(),
            interval: Duration::from_secs(60),
            detailed: false,
            level: MonitoringLevel::Standard,
            health_checks: true,
            tracing: false,
            alerting: true,
            sample_rate: 1.0,
        };
        assert!(prod.is_production_ready());

        // Not production ready: disabled
        let not_prod1 = TestMonitoringConfig {
            enabled: false,
            endpoint: "http://metrics.example.com".to_string(),
            interval: Duration::from_secs(60),
            detailed: false,
            level: MonitoringLevel::Standard,
            health_checks: true,
            tracing: false,
            alerting: false,
            sample_rate: 1.0,
        };
        assert!(!not_prod1.is_production_ready());

        // Not production ready: too frequent
        let not_prod2 = TestMonitoringConfig {
            enabled: true,
            endpoint: "http://metrics.example.com".to_string(),
            interval: Duration::from_secs(5),
            detailed: false,
            level: MonitoringLevel::Standard,
            health_checks: true,
            tracing: false,
            alerting: false,
            sample_rate: 1.0,
        };
        assert!(!not_prod2.is_production_ready());

        // Not production ready: too much overhead
        let not_prod3 = TestMonitoringConfig {
            enabled: true,
            endpoint: "http://metrics.example.com".to_string(),
            interval: Duration::from_secs(60),
            detailed: true,
            level: MonitoringLevel::Verbose,
            health_checks: true,
            tracing: true,
            alerting: true,
            sample_rate: 1.0,
        };
        assert!(!not_prod3.is_production_ready()); // 28% overhead
    }

    #[test]
    fn test_disabled_monitoring() {
        let disabled = TestMonitoringConfig {
            enabled: false,
            endpoint: String::new(),
            interval: Duration::ZERO,
            detailed: false,
            level: MonitoringLevel::Minimal,
            health_checks: false,
            tracing: false,
            alerting: false,
            sample_rate: 0.0,
        };

        assert!(!disabled.is_enabled());
        assert_eq!(disabled.estimated_overhead(), 0.0);
        assert!(disabled.validate().is_ok()); // Disabled is valid
        assert!(!disabled.is_production_ready()); // But not production ready
    }
}
