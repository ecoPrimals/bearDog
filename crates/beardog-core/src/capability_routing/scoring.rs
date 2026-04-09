// SPDX-License-Identifier: AGPL-3.0-or-later

//! Load and latency tracking used when ranking primals and filtering by observed performance.

use std::collections::HashMap;
use std::time::Instant;
use tracing::warn;

/// Load tracking for primals
#[derive(Debug, Clone)]
pub(super) struct PrimalLoad {
    /// Current active requests
    pub(super) active_requests: usize,

    /// Total requests served
    pub(super) total_requests: usize,

    /// Last request timestamp
    pub(super) last_request: Option<Instant>,

    /// Average latency (ms)
    pub(super) avg_latency_ms: Option<f64>,
}

pub(super) fn record_success(
    load_tracker: &mut HashMap<String, PrimalLoad>,
    primal_name: &str,
    latency_ms: f64,
) {
    let load = load_tracker
        .entry(primal_name.to_string())
        .or_insert_with(|| PrimalLoad {
            active_requests: 0,
            total_requests: 0,
            last_request: None,
            avg_latency_ms: None,
        });

    load.total_requests += 1;
    load.last_request = Some(Instant::now());

    load.avg_latency_ms = Some(match load.avg_latency_ms {
        Some(avg) => avg.mul_add(0.9, latency_ms * 0.1),
        None => latency_ms,
    });
}

pub(super) fn record_failure(primal_name: &str) {
    warn!("Request failed for primal: {}", primal_name);
}
