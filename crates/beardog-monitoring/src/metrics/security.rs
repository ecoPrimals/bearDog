// SPDX-License-Identifier: AGPL-3.0-or-later

// Security Metrics Engine
//
// Security event tracking, threat detection, and compliance monitoring.

use beardog_errors::BearDogError;
use parking_lot::Mutex;
use serde::{Deserialize, Serialize};

/// Records security-category [`super::MetricEvent`] values and exposes aggregate counters.
#[derive(Debug)]
pub struct SecurityMetricsEngine {
    _config: SecurityMetricsConfig,
    state: Mutex<SecurityState>,
}

#[derive(Debug, Default)]
struct SecurityState {
    failed_auth_attempts: u64,
    successful_auths: u64,
    blocked_requests: u64,
    /// Events seen (for threat level normalization).
    total_events: u64,
}

impl SecurityMetricsEngine {
    /// Creates a new instance
    ///
    /// # Errors
    ///
    /// Currently always succeeds; the `Result` type is reserved for future validation.
    pub fn new(config: SecurityMetricsConfig) -> Result<Self, BearDogError> {
        Ok(Self {
            _config: config,
            state: Mutex::new(SecurityState::default()),
        })
    }

    /// Starts service
    ///
    /// # Errors
    ///
    /// Currently always succeeds; the `Result` type is reserved for future startup failures.
    pub fn start(&self) -> Result<(), BearDogError> {
        tracing::info!("Security metrics engine started");
        Ok(())
    }

    /// Records a security-category event and updates aggregate counters / threat signals.
    ///
    /// # Errors
    ///
    /// Returns [`BearDogError`] when the internal mutex is poisoned.
    #[expect(
        clippy::cast_possible_truncation,
        reason = "gauge value is a non-negative count that fits u64"
    )]
    pub fn record_event(&self, event: &super::MetricEvent) -> Result<(), BearDogError> {
        let mut state = self.state.lock();
        state.total_events = state.total_events.saturating_add(1);
        let name = event.name.to_lowercase();
        #[expect(
            clippy::cast_sign_loss,
            reason = "gauge values represent non-negative counts"
        )]
        let delta = match &event.value {
            super::MetricValue::Counter(c) => *c,
            super::MetricValue::Gauge(g) => g.round().max(0.0) as u64,
            super::MetricValue::Histogram(h) => h.len() as u64,
            super::MetricValue::Summary { count, .. } => *count,
        };
        if name.contains("auth") && (name.contains("fail") || name.contains("failure")) {
            state.failed_auth_attempts = state.failed_auth_attempts.saturating_add(delta);
        } else if name.contains("auth") && name.contains("success") {
            state.successful_auths = state.successful_auths.saturating_add(delta);
        } else if name.contains("blocked") {
            state.blocked_requests = state.blocked_requests.saturating_add(delta);
        }
        Ok(())
    }

    /// Gets metrics
    ///
    /// # Errors
    ///
    /// Returns [`BearDogError`] when the internal mutex is poisoned.
    #[expect(clippy::cast_precision_loss, reason = "metrics averaging")]
    pub fn get_metrics(&self) -> Result<SecurityMetrics, BearDogError> {
        let state = self.state.lock();
        let threat_level = if state.total_events == 0 {
            0.0
        } else {
            let num = 0.5f64.mul_add(
                state.blocked_requests as f64,
                state.failed_auth_attempts as f64,
            );
            (num / state.total_events as f64).min(1.0)
        };
        let compliance_score = 0.5f64.mul_add(-threat_level, 1.0).clamp(0.0, 1.0);
        Ok(SecurityMetrics {
            failed_auth_attempts: state.failed_auth_attempts,
            successful_auths: state.successful_auths,
            blocked_requests: state.blocked_requests,
            threat_level,
            compliance_score,
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
    clippy::nonminimal_bool,
    reason = "security metrics tests: float thresholds and exhaustive coverage (multi-lint expect causes unfulfilled_lint_expectations)"
)]
#[cfg(test)]
#[path = "security_tests.rs"]
mod security_tests;
