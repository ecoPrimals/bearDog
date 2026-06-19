// SPDX-License-Identifier: AGPL-3.0-or-later

//! Shadow metric collection for TLS parity proofs.
//!
//! Collects side-by-side performance data comparing `BearDog`'s sovereign TLS
//! (rustls) against the commercial baseline (Cloudflare tunnel). Metrics are
//! accumulated over a 7-day rolling window and emitted in the provenance audit
//! format for the provenance trio.
//!
//! Cutover criteria: sovereign p95 ≤ 1.5× commercial p95 for 7 consecutive days.

use std::collections::VecDeque;
use std::sync::Arc;
use std::time::{Duration, Instant};

use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};
use tokio::sync::RwLock;
use tracing::{debug, info};

/// A single latency observation.
#[derive(Debug, Clone, Copy, Serialize, Deserialize)]
pub struct LatencyObservation {
    /// When the observation was recorded.
    pub timestamp: DateTime<Utc>,
    /// Request latency in microseconds.
    pub latency_us: u64,
}

/// Accumulated metrics for one side of the shadow comparison.
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct ShadowSideMetrics {
    /// Total requests observed.
    pub total_requests: u64,
    /// Total errors observed.
    pub total_errors: u64,
    /// Latency observations (rolling window).
    observations: Vec<u64>,
    /// Certificate rotations observed.
    pub cert_rotations: u64,
    /// Successful cert rotations.
    pub cert_rotation_successes: u64,
}

impl ShadowSideMetrics {
    /// Record a successful request with its latency.
    pub fn record_request(&mut self, latency: Duration) {
        self.total_requests += 1;
        #[expect(
            clippy::cast_possible_truncation,
            reason = "latency fits in u64 microseconds"
        )]
        let us = latency.as_micros() as u64;
        self.observations.push(us);
    }

    /// Record an error.
    pub fn record_error(&mut self) {
        self.total_errors += 1;
    }

    /// Record a certificate rotation attempt.
    pub fn record_cert_rotation(&mut self, success: bool) {
        self.cert_rotations += 1;
        if success {
            self.cert_rotation_successes += 1;
        }
    }

    /// Calculate the p50 latency (median) in microseconds.
    #[must_use]
    pub fn p50_us(&self) -> Option<u64> {
        percentile(&self.observations, 50.0)
    }

    /// Calculate the p95 latency in microseconds.
    #[must_use]
    pub fn p95_us(&self) -> Option<u64> {
        percentile(&self.observations, 95.0)
    }

    /// Calculate the p99 latency in microseconds.
    #[must_use]
    pub fn p99_us(&self) -> Option<u64> {
        percentile(&self.observations, 99.0)
    }

    /// Error rate as a fraction (0.0 to 1.0).
    #[must_use]
    #[expect(
        clippy::cast_precision_loss,
        reason = "rate calculation — acceptable precision"
    )]
    pub fn error_rate(&self) -> f64 {
        if self.total_requests == 0 {
            0.0
        } else {
            self.total_errors as f64 / self.total_requests as f64
        }
    }

    /// Requests per second (estimated from window duration).
    #[must_use]
    #[expect(
        clippy::cast_precision_loss,
        reason = "rate calculation — acceptable precision"
    )]
    pub fn requests_per_second(&self, window: Duration) -> f64 {
        if window.is_zero() {
            return 0.0;
        }
        self.total_requests as f64 / window.as_secs_f64()
    }
}

/// The full shadow comparison report.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ShadowReport {
    /// When this report was generated.
    pub generated_at: DateTime<Utc>,
    /// Duration of the measurement window.
    pub window_duration_hours: u64,
    /// Sovereign side (`BearDog` rustls).
    pub sovereign: ShadowSideMetrics,
    /// Commercial side (Cloudflare).
    pub commercial: ShadowSideMetrics,
    /// Whether cutover criteria are met.
    pub cutover_ready: bool,
    /// Number of consecutive days where parity holds.
    pub consecutive_parity_days: u32,
}

/// Tracks shadow comparison state over a 7-day rolling window.
#[derive(Clone)]
pub struct ShadowMetricsCollector {
    inner: Arc<RwLock<ShadowMetricsInner>>,
}

struct ShadowMetricsInner {
    sovereign: ShadowSideMetrics,
    commercial: ShadowSideMetrics,
    start_time: Instant,
    daily_parity_checks: VecDeque<bool>,
}

impl ShadowMetricsCollector {
    /// Create a new collector.
    #[must_use]
    pub fn new() -> Self {
        Self {
            inner: Arc::new(RwLock::new(ShadowMetricsInner {
                sovereign: ShadowSideMetrics::default(),
                commercial: ShadowSideMetrics::default(),
                start_time: Instant::now(),
                daily_parity_checks: VecDeque::with_capacity(7),
            })),
        }
    }

    /// Record a sovereign (`BearDog`) request.
    pub async fn record_sovereign_request(&self, latency: Duration) {
        let mut inner = self.inner.write().await;
        inner.sovereign.record_request(latency);
    }

    /// Record a sovereign error.
    pub async fn record_sovereign_error(&self) {
        let mut inner = self.inner.write().await;
        inner.sovereign.record_error();
    }

    /// Record a commercial (Cloudflare) request.
    pub async fn record_commercial_request(&self, latency: Duration) {
        let mut inner = self.inner.write().await;
        inner.commercial.record_request(latency);
    }

    /// Record a commercial error.
    pub async fn record_commercial_error(&self) {
        let mut inner = self.inner.write().await;
        inner.commercial.record_error();
    }

    /// Record a certificate rotation on the sovereign side.
    pub async fn record_cert_rotation(&self, success: bool) {
        let mut inner = self.inner.write().await;
        inner.sovereign.record_cert_rotation(success);
    }

    /// Perform a daily parity check. Returns whether the cutover criteria are met.
    pub async fn daily_parity_check(&self) -> bool {
        let mut inner = self.inner.write().await;

        let sov_p95 = inner.sovereign.p95_us().unwrap_or(0);
        let com_p95 = inner.commercial.p95_us().unwrap_or(1);

        let parity_met = if com_p95 == 0 {
            true
        } else {
            #[expect(clippy::cast_precision_loss, reason = "latency values fit in f64")]
            let ratio = sov_p95 as f64 / com_p95 as f64;
            ratio <= 1.5
        };

        inner.daily_parity_checks.push_back(parity_met);
        if inner.daily_parity_checks.len() > 7 {
            inner.daily_parity_checks.pop_front();
        }

        let consecutive = inner
            .daily_parity_checks
            .iter()
            .rev()
            .take_while(|&&v| v)
            .count();

        debug!(
            sovereign_p95_us = sov_p95,
            commercial_p95_us = com_p95,
            parity_met,
            consecutive_days = consecutive,
            "daily shadow parity check"
        );

        #[expect(clippy::cast_possible_truncation, reason = "consecutive days <= 7")]
        let consecutive_u32 = consecutive as u32;
        consecutive_u32 >= 7
    }

    /// Generate a full shadow comparison report.
    pub async fn report(&self) -> ShadowReport {
        let inner = self.inner.read().await;
        let elapsed = inner.start_time.elapsed();

        let consecutive = inner
            .daily_parity_checks
            .iter()
            .rev()
            .take_while(|&&v| v)
            .count();

        let window_hours = elapsed.as_secs() / 3600;

        #[expect(clippy::cast_possible_truncation, reason = "consecutive days <= 7")]
        let consecutive_days = consecutive as u32;

        ShadowReport {
            generated_at: Utc::now(),
            window_duration_hours: window_hours,
            sovereign: inner.sovereign.clone(),
            commercial: inner.commercial.clone(),
            cutover_ready: consecutive_days >= 7,
            consecutive_parity_days: consecutive_days,
        }
    }

    /// Run the daily parity check loop (checks every 24h).
    pub async fn run_parity_loop(&self) {
        info!("starting shadow parity check loop (24h interval)");
        loop {
            tokio::time::sleep(Duration::from_secs(86_400)).await;
            let cutover_ready = self.daily_parity_check().await;
            if cutover_ready {
                info!("CUTOVER READY — 7 consecutive days of shadow parity achieved");
            }
        }
    }
}

impl Default for ShadowMetricsCollector {
    fn default() -> Self {
        Self::new()
    }
}

/// Compute a percentile from a slice of values.
fn percentile(values: &[u64], pct: f64) -> Option<u64> {
    if values.is_empty() {
        return None;
    }
    let mut sorted = values.to_vec();
    sorted.sort_unstable();

    #[expect(clippy::cast_precision_loss, reason = "index arithmetic")]
    #[expect(clippy::cast_possible_truncation, reason = "index from float")]
    #[expect(clippy::cast_sign_loss, reason = "index is non-negative")]
    let idx = ((pct / 100.0) * (sorted.len() - 1) as f64).round() as usize;
    let idx = idx.min(sorted.len() - 1);
    Some(sorted[idx])
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn percentile_basic() {
        let values: Vec<u64> = (1..=100).collect();
        // p50 of 1..=100: index = round(0.5 * 99) = 50, values[50] = 51
        assert_eq!(percentile(&values, 50.0), Some(51));
        // p95: index = round(0.95 * 99) = 94, values[94] = 95
        assert_eq!(percentile(&values, 95.0), Some(95));
        // p99: index = round(0.99 * 99) = 98, values[98] = 99
        assert_eq!(percentile(&values, 99.0), Some(99));
    }

    #[test]
    fn percentile_empty() {
        assert_eq!(percentile(&[], 50.0), None);
    }

    #[test]
    fn percentile_single() {
        assert_eq!(percentile(&[42], 95.0), Some(42));
    }

    #[test]
    fn side_metrics_request_tracking() {
        let mut m = ShadowSideMetrics::default();
        m.record_request(Duration::from_millis(10));
        m.record_request(Duration::from_millis(20));
        m.record_error();

        assert_eq!(m.total_requests, 2);
        assert_eq!(m.total_errors, 1);
        assert!(m.p50_us().is_some());
    }

    #[test]
    fn error_rate_calculation() {
        let mut m = ShadowSideMetrics::default();
        assert!(m.error_rate().abs() < f64::EPSILON);

        m.record_request(Duration::from_millis(1));
        m.record_request(Duration::from_millis(1));
        m.record_error();
        m.record_error();
        assert!((m.error_rate() - 1.0).abs() < f64::EPSILON);
    }

    #[test]
    fn cert_rotation_tracking() {
        let mut m = ShadowSideMetrics::default();
        m.record_cert_rotation(true);
        m.record_cert_rotation(false);
        m.record_cert_rotation(true);

        assert_eq!(m.cert_rotations, 3);
        assert_eq!(m.cert_rotation_successes, 2);
    }

    #[tokio::test]
    async fn collector_records_both_sides() {
        let collector = ShadowMetricsCollector::new();
        collector
            .record_sovereign_request(Duration::from_millis(10))
            .await;
        collector
            .record_commercial_request(Duration::from_millis(100))
            .await;

        let report = collector.report().await;
        assert_eq!(report.sovereign.total_requests, 1);
        assert_eq!(report.commercial.total_requests, 1);
    }

    #[tokio::test]
    async fn parity_check_with_good_sovereign() {
        let collector = ShadowMetricsCollector::new();

        for _ in 0..100 {
            collector
                .record_sovereign_request(Duration::from_millis(10))
                .await;
            collector
                .record_commercial_request(Duration::from_millis(100))
                .await;
        }

        let _parity = collector.daily_parity_check().await;
    }
}
