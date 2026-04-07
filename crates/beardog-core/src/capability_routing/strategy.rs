// SPDX-License-Identifier: AGPL-3.0-or-later

//! Selection strategies and per-request routing context.

use crate::primal_discovery::DiscoveredPrimal;
use crate::self_knowledge::SimpleCapability;
use beardog_errors::BearDogError;
use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use std::time::{Duration, Instant};

use super::scoring::PrimalLoad;

/// Selection strategy for capability routing
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum SelectionStrategy {
    /// Choose primal with highest trust score
    HighestTrust,

    /// Choose primal with lowest current load
    LeastLoaded,

    /// Choose primal with lowest latency
    LowestLatency,

    /// Round-robin selection
    RoundRobin,

    /// Random selection
    Random,

    /// First available (discovery order)
    FirstAvailable,
}

/// Request context for routing decisions
#[derive(Debug, Clone)]
pub struct RequestContext {
    /// Required capability
    pub capability: SimpleCapability,

    /// Selection strategy
    pub strategy: SelectionStrategy,

    /// Maximum acceptable latency (ms)
    pub max_latency_ms: Option<u64>,

    /// Minimum required trust score (0.0 - 1.0)
    pub min_trust_score: Option<f64>,

    /// Exclude specific primals by name
    pub exclude_primals: Vec<String>,

    /// Request timeout
    pub timeout: Duration,
}

impl RequestContext {
    /// Create a new request context
    #[must_use]
    pub const fn new(capability: SimpleCapability) -> Self {
        Self {
            capability,
            strategy: SelectionStrategy::HighestTrust,
            max_latency_ms: None,
            min_trust_score: None,
            exclude_primals: Vec::new(),
            timeout: Duration::from_secs(5),
        }
    }

    /// Set selection strategy
    #[must_use]
    pub const fn with_strategy(mut self, strategy: SelectionStrategy) -> Self {
        self.strategy = strategy;
        self
    }

    /// Set maximum acceptable latency
    #[must_use]
    pub const fn with_max_latency(mut self, latency_ms: u64) -> Self {
        self.max_latency_ms = Some(latency_ms);
        self
    }

    /// Set minimum required trust score
    #[must_use]
    pub const fn with_min_trust(mut self, trust_score: f64) -> Self {
        self.min_trust_score = Some(trust_score);
        self
    }

    /// Exclude a specific primal
    #[must_use]
    pub fn excluding(mut self, primal_name: impl Into<String>) -> Self {
        self.exclude_primals.push(primal_name.into());
        self
    }

    /// Set request timeout
    #[must_use]
    pub const fn with_timeout(mut self, timeout: Duration) -> Self {
        self.timeout = timeout;
        self
    }
}

impl Default for RequestContext {
    fn default() -> Self {
        Self {
            capability: SimpleCapability::SecureTunneling,
            strategy: SelectionStrategy::HighestTrust,
            max_latency_ms: None,
            min_trust_score: None,
            exclude_primals: Vec::new(),
            timeout: Duration::from_secs(5),
        }
    }
}

/// Select the best primal using the given strategy
pub(crate) fn select_primal(
    primals: &mut [DiscoveredPrimal],
    context: &RequestContext,
    load_tracker: &HashMap<String, PrimalLoad>,
    rr_counters: &mut HashMap<SimpleCapability, usize>,
) -> Result<(DiscoveredPrimal, String), BearDogError> {
    if primals.is_empty() {
        return Err(BearDogError::not_found("No primals available".to_string()));
    }

    let (index, reason) = match context.strategy {
        SelectionStrategy::HighestTrust => {
            let idx = primals
                .iter()
                .enumerate()
                .max_by(|(_, a), (_, b)| {
                    let a_trust = a.trust_score.unwrap_or(0.0);
                    let b_trust = b.trust_score.unwrap_or(0.0);
                    a_trust
                        .partial_cmp(&b_trust)
                        .unwrap_or(std::cmp::Ordering::Equal)
                })
                .map_or(0, |(i, _)| i);

            let trust = primals[idx].trust_score.unwrap_or(0.0);
            (idx, format!("highest trust score: {trust:.2}"))
        }

        SelectionStrategy::LeastLoaded => {
            let idx = primals
                .iter()
                .enumerate()
                .min_by_key(|(_, p)| {
                    load_tracker
                        .get(&p.name)
                        .map_or(0, |load| load.active_requests)
                })
                .map_or(0, |(i, _)| i);

            let load = load_tracker
                .get(&primals[idx].name)
                .map_or(0, |l| l.active_requests);
            (idx, format!("least loaded: {load} active requests"))
        }

        SelectionStrategy::LowestLatency => {
            let idx = primals
                .iter()
                .enumerate()
                .min_by(|(_, a), (_, b)| {
                    let a_latency = load_tracker
                        .get(&a.name)
                        .and_then(|load| load.avg_latency_ms)
                        .unwrap_or(f64::MAX);
                    let b_latency = load_tracker
                        .get(&b.name)
                        .and_then(|load| load.avg_latency_ms)
                        .unwrap_or(f64::MAX);
                    a_latency
                        .partial_cmp(&b_latency)
                        .unwrap_or(std::cmp::Ordering::Equal)
                })
                .map_or(0, |(i, _)| i);

            let latency = load_tracker
                .get(&primals[idx].name)
                .and_then(|l| l.avg_latency_ms)
                .map_or_else(|| "unknown".to_string(), |lat| format!("{lat:.1}ms"));
            (idx, format!("lowest latency: {latency}"))
        }

        SelectionStrategy::RoundRobin => {
            let counter = rr_counters.entry(context.capability.clone()).or_insert(0);
            let idx = *counter % primals.len();
            *counter = (*counter + 1) % primals.len();
            (idx, format!("round-robin (counter: {counter})"))
        }

        SelectionStrategy::Random => {
            use std::collections::hash_map::RandomState;
            use std::hash::BuildHasher;

            let s = RandomState::new();
            let hash = s.hash_one(Instant::now());
            let idx = usize::try_from(hash % primals.len() as u64).unwrap_or(0);
            (idx, "random selection".to_string())
        }

        SelectionStrategy::FirstAvailable => (0, "first available".to_string()),
    };

    let selected = primals[index].clone();
    Ok((selected, reason))
}
