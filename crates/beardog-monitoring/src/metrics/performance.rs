// SPDX-License-Identifier: AGPL-3.0-or-later

// Performance Metrics Engine
//
// Performance monitoring, analysis, and optimization tracking.

use beardog_errors::BearDogError;
use serde::{Deserialize, Serialize};
use std::sync::Mutex;

/// Collects and surfaces performance-related [`super::MetricEvent`] values.
#[derive(Debug)]
pub struct PerformanceEngine {
    _config: PerformanceConfig,
    state: Mutex<PerformanceState>,
}

#[derive(Debug, Default)]
struct PerformanceState {
    /// Latest CPU gauge (0.0–1.0 or percent, depending on producer).
    cpu_usage: Option<f64>,
    /// Latest memory gauge.
    memory_usage: Option<f64>,
    /// Recent latency samples (milliseconds), capped for bounded memory.
    latency_samples: Vec<f64>,
    /// Latest throughput gauge (requests per second).
    throughput_rps: Option<f64>,
    /// Count of error-class events (name contains `error`).
    error_events: u64,
    /// Total performance events recorded (denominator for error rate).
    total_events: u64,
}

const LATENCY_SAMPLE_CAP: usize = 1000;

impl PerformanceEngine {
    /// Creates a new instance
    ///
    /// # Errors
    ///
    /// Currently always succeeds; the `Result` type is reserved for future validation.
    pub fn new(config: PerformanceConfig) -> Result<Self, BearDogError> {
        Ok(Self {
            _config: config,
            state: Mutex::new(PerformanceState::default()),
        })
    }

    /// Starts service
    ///
    /// # Errors
    ///
    /// Currently always succeeds; the `Result` type is reserved for future startup failures.
    pub fn start(&self) -> Result<(), BearDogError> {
        tracing::info!("Performance metrics engine started");
        Ok(())
    }

    /// Records a performance-category event: updates rolling aggregates used by [`Self::get_metrics`].
    ///
    /// # Errors
    ///
    /// Returns [`BearDogError`] when the internal mutex is poisoned.
    #[expect(clippy::cast_precision_loss, reason = "metrics averaging")]
    pub fn record_event(&self, event: &super::MetricEvent) -> Result<(), BearDogError> {
        let mut state = self
            .state
            .lock()
            .map_err(|_| BearDogError::system("performance metrics lock poisoned".to_string()))?;
        state.total_events = state.total_events.saturating_add(1);
        let name = event.name.to_lowercase();
        match &event.value {
            super::MetricValue::Gauge(g) => {
                if name.contains("cpu") {
                    state.cpu_usage = Some(*g);
                } else if name.contains("memory") || name.contains("mem") {
                    state.memory_usage = Some(*g);
                } else if name.contains("latency") {
                    push_latency(&mut state.latency_samples, *g);
                } else if name.contains("throughput") || name.contains("rps") {
                    state.throughput_rps = Some(*g);
                }
            }
            super::MetricValue::Counter(c) => {
                if name.contains("error") {
                    state.error_events = state.error_events.saturating_add(*c);
                }
            }
            super::MetricValue::Histogram(h) => {
                if !h.is_empty() && name.contains("latency") {
                    let sum: f64 = h.iter().sum();
                    push_latency(&mut state.latency_samples, sum / h.len() as f64);
                }
            }
            super::MetricValue::Summary { sum, count } => {
                if *count > 0 && name.contains("latency") {
                    push_latency(&mut state.latency_samples, sum / *count as f64);
                }
            }
        }
        Ok(())
    }

    /// Gets metrics
    ///
    /// # Errors
    ///
    /// Returns [`BearDogError`] when the internal mutex is poisoned.
    #[expect(clippy::cast_precision_loss, reason = "metrics averaging")]
    pub fn get_metrics(&self) -> Result<PerformanceMetrics, BearDogError> {
        let state = self
            .state
            .lock()
            .map_err(|_| BearDogError::system("performance metrics lock poisoned".to_string()))?;
        let request_latency_ms = if state.latency_samples.is_empty() {
            0.0
        } else {
            state.latency_samples.iter().sum::<f64>() / state.latency_samples.len() as f64
        };
        let error_rate = if state.total_events == 0 {
            0.0
        } else {
            state.error_events as f64 / state.total_events as f64
        };
        Ok(PerformanceMetrics {
            cpu_usage: state.cpu_usage.unwrap_or(0.0),
            memory_usage: state.memory_usage.unwrap_or(0.0),
            request_latency_ms,
            throughput_rps: state.throughput_rps.unwrap_or(0.0),
            error_rate,
        })
    }
}

fn push_latency(samples: &mut Vec<f64>, v: f64) {
    samples.push(v);
    if samples.len() > LATENCY_SAMPLE_CAP {
        let overflow = samples.len() - LATENCY_SAMPLE_CAP;
        samples.drain(0..overflow);
    }
}

/// Normalized CPU, memory, latency, throughput, and error-rate figures from the performance engine.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PerformanceMetrics {
    /// The cpu usage value
    pub cpu_usage: f64,
    /// The memory usage value
    pub memory_usage: f64,
    /// The request latency ms value
    pub request_latency_ms: f64,
    /// The throughput rps value
    pub throughput_rps: f64,
    /// The error rate value
    pub error_rate: f64,
}

/// Sampling density and retention for [`PerformanceEngine`].
#[derive(Debug, Clone)]
pub struct PerformanceConfig {
    /// The sample rate value
    pub sample_rate: f64,
    /// How long raw samples are retained before rollover/pruning.
    pub retention_hours: u32,
}

impl Default for PerformanceConfig {
    fn default() -> Self {
        Self {
            sample_rate: 1.0,
            retention_hours: 24,
        }
    }
}

#[allow(
    unused_imports,
    clippy::float_cmp,
    clippy::absurd_extreme_comparisons,
    unused_comparisons,
    clippy::nonminimal_bool,
    reason = "performance metrics tests: float thresholds and exhaustive coverage (multi-lint expect causes unfulfilled_lint_expectations)"
)]
#[cfg(test)]
#[path = "performance_tests.rs"]
mod performance_tests;
