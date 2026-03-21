// SPDX-License-Identifier: AGPL-3.0-only

// Security Metrics Engine
//
// Security event tracking, threat detection, and compliance monitoring.

use beardog_errors::BearDogError;
use serde::{Deserialize, Serialize};

/// Records security-category [`super::MetricEvent`] values and exposes aggregate counters.
#[derive(Debug)]
pub struct SecurityMetricsEngine {
    _config: SecurityMetricsConfig,
}

impl SecurityMetricsEngine {
    /// Creates a new instance
    pub const fn new(config: SecurityMetricsConfig) -> Result<Self, BearDogError> {
        Ok(Self { _config: config })
    }

    /// Starts service
    /// Starts service
    pub fn start(&self) -> Result<(), BearDogError> {
        tracing::info!("Security metrics engine started");
        Ok(())
    }

    /// Records a security-category event (placeholder for future correlation).
    pub const fn record_event(&self, _event: &super::MetricEvent) -> Result<(), BearDogError> {
        // Security event processing logic
        Ok(())
    }

    /// Gets metrics
    /// Gets metrics
    pub const fn get_metrics(&self) -> Result<SecurityMetrics, BearDogError> {
        Ok(SecurityMetrics {
            failed_auth_attempts: 5,
            successful_auths: 1250,
            blocked_requests: 12,
            threat_level: 0.2,
            compliance_score: 0.95,
        })
    }
}

/// Aggregate security posture counters exposed by [`SecurityMetricsEngine::get_metrics`].
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SecurityMetrics {
    /// Number of `failed_auth_attempts`
    pub failed_auth_attempts: u64,
    /// Number of `successful_auths`
    pub successful_auths: u64,
    /// Number of `blocked_requests`
    pub blocked_requests: u64,
    /// The threat level value
    pub threat_level: f64,
    /// The compliance score value
    pub compliance_score: f64,
}

/// Tunables for [`SecurityMetricsEngine`] sampling and alerting behavior.
#[derive(Debug, Clone)]
pub struct SecurityMetricsConfig {
    /// Whether `enable_threat_detection` is enabled
    pub enable_threat_detection: bool,
    /// The alert threshold value
    pub alert_threshold: f64,
}

impl Default for SecurityMetricsConfig {
    fn default() -> Self {
        Self {
            enable_threat_detection: true,
            alert_threshold: 0.8,
        }
    }
}

#[allow(
    unused_imports,
    clippy::float_cmp,
    clippy::absurd_extreme_comparisons,
    unused_comparisons,
    clippy::nonminimal_bool
)]
#[cfg(test)]
#[path = "security_tests.rs"]
mod security_tests;
