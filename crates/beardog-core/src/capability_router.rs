// SPDX-License-Identifier: AGPL-3.0-only

//! Capability-Based Routing Module
//!
//! **Core Principle**: "Route requests by capability, not by hardcoded service names"
//!
//! This module implements capability-based routing for cross-primal communication,
//! eliminating hardcoded service dependencies and enabling dynamic peer selection.
//!
//! # Design Philosophy
//!
//! - **Capability-First**: Route by what is needed, not who provides it
//! - **Dynamic Selection**: Choose providers based on trust, load, proximity
//! - **Failover Support**: Automatic fallback to alternative providers
//! - **Load Balancing**: Distribute requests across capable primals
//!
//! # Routing Flow
//!
//! ```text
//! 1. Request: "I need SecureTunneling"
//! 2. Discovery: Find all primals providing SecureTunneling
//! 3. Selection: Choose best provider (trust, load, latency)
//! 4. Execution: Send request to selected primal
//! 5. Failover: If failed, try next provider
//! ```
//!
//! # Usage Example
//!
//! ```rust,no_run
//! use beardog_core::capability_router::{CapabilityRouter, RequestContext};
//! use beardog_core::self_knowledge::SimpleCapability;
//!
//! # async fn example() -> Result<(), beardog_errors::BearDogError> {
//! let mut router = CapabilityRouter::from_env()?;
//!
//! // Route by capability, not by service name!
//! let decision = router.route(
//!     SimpleCapability::Cryptography,
//!     RequestContext::default()
//! ).await?;
//!
//! println!("Routing to primal: {:?}", decision.primal);
//! println!("Reason: {}", decision.reason);
//! # Ok(())
//! # }
//! ```

use crate::primal_discovery::{DiscoveredPrimal, DiscoveryQuery, PrimalDiscovery};
use crate::self_knowledge::SimpleCapability;
use beardog_errors::BearDogError;
use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use std::time::{Duration, Instant};
use tracing::{info, warn};

// =============================================================================
// CORE TYPES
// =============================================================================

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

/// Routing decision result
#[derive(Debug, Clone)]
pub struct RoutingDecision {
    /// Selected primal
    pub primal: DiscoveredPrimal,

    /// Selection reason
    pub reason: String,

    /// Alternative primals (for failover)
    pub alternatives: Vec<DiscoveredPrimal>,

    /// Decision timestamp
    pub decided_at: Instant,
}

/// Load tracking for primals
#[derive(Debug, Clone)]
struct PrimalLoad {
    /// Current active requests
    active_requests: usize,

    /// Total requests served
    total_requests: usize,

    /// Last request timestamp
    last_request: Option<Instant>,

    /// Average latency (ms)
    avg_latency_ms: Option<f64>,
}

/// Capability-based router
pub struct CapabilityRouter {
    /// Discovery engine
    discovery: PrimalDiscovery,

    /// Load tracking by primal name
    load_tracker: HashMap<String, PrimalLoad>,

    /// Round-robin counters by capability
    rr_counters: HashMap<SimpleCapability, usize>,
}

// =============================================================================
// REQUEST CONTEXT BUILDERS
// =============================================================================

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

// =============================================================================
// CAPABILITY ROUTER IMPLEMENTATION
// =============================================================================

impl CapabilityRouter {
    /// Create a router with an explicit [`PrimalDiscovery`] (no environment reads here).
    #[must_use]
    pub fn new(discovery: PrimalDiscovery) -> Self {
        info!("🧭 Initializing capability-based router...");

        Self {
            discovery,
            load_tracker: HashMap::new(),
            rr_counters: HashMap::new(),
        }
    }

    /// Build discovery from the process environment (`std::env::var` via [`PrimalDiscovery::from_env`]).
    pub fn from_env() -> Result<Self, BearDogError> {
        Ok(Self::new(PrimalDiscovery::from_env()?))
    }

    /// Route a request to the best primal for a capability
    pub async fn route(
        &mut self,
        capability: SimpleCapability,
        context: RequestContext,
    ) -> Result<RoutingDecision, BearDogError> {
        info!("🧭 Routing request for capability: {:?}", capability);

        // Discover primals providing this capability
        let query = DiscoveryQuery::by_capability(capability.clone()).with_timeout(context.timeout);

        let mut primals = self.discovery.discover(query).await?;

        if primals.is_empty() {
            return Err(BearDogError::not_found(format!(
                "No primals found providing capability: {capability:?}"
            )));
        }

        // Apply filters
        primals = self.apply_filters(primals, &context);

        if primals.is_empty() {
            return Err(BearDogError::not_found(
                "No primals match routing criteria".to_string(),
            ));
        }

        // Select best primal using strategy
        let (selected, reason) = self.select_primal(&mut primals, &context)?;

        info!("✅ Routed to {} (reason: {})", selected.name, reason);

        Ok(RoutingDecision {
            primal: selected,
            reason,
            alternatives: primals,
            decided_at: Instant::now(),
        })
    }

    /// Apply filters to candidate primals
    #[expect(
        clippy::cast_possible_truncation,
        reason = "Rounded latency ms compared to u64 budget; practical ms ranges"
    )]
    #[expect(
        clippy::cast_sign_loss,
        reason = "Latency milliseconds are non-negative before integer comparison"
    )]
    fn apply_filters(
        &self,
        mut primals: Vec<DiscoveredPrimal>,
        context: &RequestContext,
    ) -> Vec<DiscoveredPrimal> {
        // Filter by excluded primals
        if !context.exclude_primals.is_empty() {
            primals.retain(|p| !context.exclude_primals.contains(&p.name));
        }

        // Filter by minimum trust score
        if let Some(min_trust) = context.min_trust_score {
            primals.retain(|p| p.trust_score.is_some_and(|score| score >= min_trust));
        }

        // Filter by maximum latency (if we have latency data)
        if let Some(max_latency) = context.max_latency_ms {
            primals.retain(|p| {
                self.load_tracker
                    .get(&p.name)
                    .and_then(|load| load.avg_latency_ms)
                    .is_none_or(|latency| {
                        // Truncate f64 latency to u64 for integer comparison
                        // Safe: latency values are practical millisecond ranges
                        (latency.round() as u64) <= max_latency
                    })
            });
        }

        primals
    }

    /// Select the best primal using the given strategy
    fn select_primal(
        &mut self,
        primals: &mut [DiscoveredPrimal],
        context: &RequestContext,
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
                        self.load_tracker
                            .get(&p.name)
                            .map_or(0, |load| load.active_requests)
                    })
                    .map_or(0, |(i, _)| i);

                let load = self
                    .load_tracker
                    .get(&primals[idx].name)
                    .map_or(0, |l| l.active_requests);
                (idx, format!("least loaded: {load} active requests"))
            }

            SelectionStrategy::LowestLatency => {
                let idx = primals
                    .iter()
                    .enumerate()
                    .min_by(|(_, a), (_, b)| {
                        let a_latency = self
                            .load_tracker
                            .get(&a.name)
                            .and_then(|load| load.avg_latency_ms)
                            .unwrap_or(f64::MAX);
                        let b_latency = self
                            .load_tracker
                            .get(&b.name)
                            .and_then(|load| load.avg_latency_ms)
                            .unwrap_or(f64::MAX);
                        a_latency
                            .partial_cmp(&b_latency)
                            .unwrap_or(std::cmp::Ordering::Equal)
                    })
                    .map_or(0, |(i, _)| i);

                let latency = self
                    .load_tracker
                    .get(&primals[idx].name)
                    .and_then(|l| l.avg_latency_ms)
                    .map_or_else(|| "unknown".to_string(), |lat| format!("{lat:.1}ms"));
                (idx, format!("lowest latency: {latency}"))
            }

            SelectionStrategy::RoundRobin => {
                let counter = self
                    .rr_counters
                    .entry(context.capability.clone())
                    .or_insert(0);
                let idx = *counter % primals.len();
                *counter = (*counter + 1) % primals.len();
                (idx, format!("round-robin (counter: {counter})"))
            }

            SelectionStrategy::Random => {
                use std::collections::hash_map::RandomState;
                use std::hash::BuildHasher;

                let s = RandomState::new();
                let hash = s.hash_one(Instant::now());
                // Truncation is fine: we only need a uniform index into a small slice
                let idx = usize::try_from(hash % primals.len() as u64).unwrap_or(0);
                (idx, "random selection".to_string())
            }

            SelectionStrategy::FirstAvailable => (0, "first available".to_string()),
        };

        let selected = primals[index].clone();
        Ok((selected, reason))
    }

    /// Record successful request completion (for load tracking)
    pub fn record_success(&mut self, primal_name: &str, latency_ms: f64) {
        let load = self
            .load_tracker
            .entry(primal_name.to_string())
            .or_insert_with(|| PrimalLoad {
                active_requests: 0,
                total_requests: 0,
                last_request: None,
                avg_latency_ms: None,
            });

        load.total_requests += 1;
        load.last_request = Some(Instant::now());

        // Update rolling average latency
        load.avg_latency_ms = Some(match load.avg_latency_ms {
            Some(avg) => avg.mul_add(0.9, latency_ms * 0.1), // Exponential moving average
            None => latency_ms,
        });
    }

    /// Record request failure (for load tracking)
    pub fn record_failure(&mut self, primal_name: &str) {
        warn!("Request failed for primal: {}", primal_name);
        // Could implement backoff/circuit breaker logic here
    }
}

// =============================================================================
// TESTS
// =============================================================================

#[cfg(test)]
mod tests {
    use super::*;
    use crate::primal_discovery::DiscoveryMethod;
    use std::collections::HashMap;

    #[test]
    fn test_request_context_builder() {
        let ctx = RequestContext::new(SimpleCapability::Cryptography)
            .with_strategy(SelectionStrategy::LeastLoaded)
            .with_max_latency(100)
            .with_min_trust(0.8)
            .excluding("untrusted-primal")
            .with_timeout(Duration::from_secs(10));

        assert_eq!(ctx.capability, SimpleCapability::Cryptography);
        assert_eq!(ctx.strategy, SelectionStrategy::LeastLoaded);
        assert_eq!(ctx.max_latency_ms, Some(100));
        assert_eq!(ctx.min_trust_score, Some(0.8));
        assert_eq!(ctx.exclude_primals, vec!["untrusted-primal"]);
        assert_eq!(ctx.timeout, Duration::from_secs(10));
    }

    #[tokio::test]
    async fn test_router_initialization() {
        let router = CapabilityRouter::new(PrimalDiscovery::new(
            crate::primal_discovery::DiscoveryMethod::Environment,
        ));
        assert!(router.load_tracker.is_empty());
        assert!(router.rr_counters.is_empty());
    }

    #[test]
    fn test_selection_strategy_highest_trust() {
        use crate::primal_discovery::DiscoveredPrimal;

        use std::time::SystemTime;

        let mut primals = vec![
            DiscoveredPrimal {
                name: "low-trust".to_string(),
                endpoints: vec![],
                capabilities: vec![],
                trust_score: Some(0.3),
                discovered_at: SystemTime::now(),
            },
            DiscoveredPrimal {
                name: "high-trust".to_string(),
                endpoints: vec![],
                capabilities: vec![],
                trust_score: Some(0.9),
                discovered_at: SystemTime::now(),
            },
        ];

        let mut router = CapabilityRouter {
            discovery: PrimalDiscovery::new(DiscoveryMethod::Environment),
            load_tracker: HashMap::new(),
            rr_counters: HashMap::new(),
        };

        let context = RequestContext::new(SimpleCapability::Cryptography)
            .with_strategy(SelectionStrategy::HighestTrust);

        let (selected, reason) = router.select_primal(&mut primals, &context).unwrap();

        assert_eq!(selected.name, "high-trust");
        assert!(reason.contains("trust"));
    }

    #[test]
    fn test_record_success_updates_stats() {
        let mut router = CapabilityRouter {
            discovery: PrimalDiscovery::new(DiscoveryMethod::Environment),
            load_tracker: HashMap::new(),
            rr_counters: HashMap::new(),
        };

        router.record_success("test-primal", 50.0);
        router.record_success("test-primal", 60.0);

        let load = router.load_tracker.get("test-primal").unwrap();
        assert_eq!(load.total_requests, 2);
        assert!(load.avg_latency_ms.is_some());
    }

    #[test]
    fn test_selection_strategy_round_robin() {
        use crate::primal_discovery::DiscoveredPrimal;
        use std::time::SystemTime;

        let mut primals = vec![
            DiscoveredPrimal {
                name: "first".to_string(),
                endpoints: vec![],
                capabilities: vec![],
                trust_score: Some(0.9),
                discovered_at: SystemTime::now(),
            },
            DiscoveredPrimal {
                name: "second".to_string(),
                endpoints: vec![],
                capabilities: vec![],
                trust_score: Some(0.9),
                discovered_at: SystemTime::now(),
            },
        ];

        let mut router = CapabilityRouter {
            discovery: PrimalDiscovery::new(DiscoveryMethod::Environment),
            load_tracker: HashMap::new(),
            rr_counters: HashMap::new(),
        };
        let context = RequestContext::new(SimpleCapability::Cryptography)
            .with_strategy(SelectionStrategy::RoundRobin);
        let (selected1, _) = router.select_primal(&mut primals, &context).unwrap();
        let (selected2, _) = router.select_primal(&mut primals, &context).unwrap();
        assert_eq!(selected1.name, "first");
        assert_eq!(selected2.name, "second");
    }

    #[test]
    fn test_selection_strategy_first_available() {
        use crate::primal_discovery::DiscoveredPrimal;
        use std::time::SystemTime;

        let mut primals = vec![
            DiscoveredPrimal {
                name: "first".to_string(),
                endpoints: vec![],
                capabilities: vec![],
                trust_score: Some(0.5),
                discovered_at: SystemTime::now(),
            },
            DiscoveredPrimal {
                name: "second".to_string(),
                endpoints: vec![],
                capabilities: vec![],
                trust_score: Some(0.9),
                discovered_at: SystemTime::now(),
            },
        ];

        let mut router = CapabilityRouter {
            discovery: PrimalDiscovery::new(DiscoveryMethod::Environment),
            load_tracker: HashMap::new(),
            rr_counters: HashMap::new(),
        };
        let context = RequestContext::new(SimpleCapability::Cryptography)
            .with_strategy(SelectionStrategy::FirstAvailable);
        let (selected, _) = router.select_primal(&mut primals, &context).unwrap();
        assert_eq!(selected.name, "first");
    }

    #[test]
    fn test_record_failure() {
        let mut router = CapabilityRouter {
            discovery: PrimalDiscovery::new(DiscoveryMethod::Environment),
            load_tracker: HashMap::new(),
            rr_counters: HashMap::new(),
        };
        router.record_failure("failed-primal");
        // No panic - failure recorded
    }

    #[test]
    fn request_context_default_uses_secure_tunneling() {
        let ctx = RequestContext::default();
        assert_eq!(ctx.capability, SimpleCapability::SecureTunneling);
        assert_eq!(ctx.strategy, SelectionStrategy::HighestTrust);
    }

    #[test]
    fn select_primal_errors_when_candidate_list_empty() {
        let mut router = CapabilityRouter::new(PrimalDiscovery::new(DiscoveryMethod::Environment));
        let mut empty: Vec<crate::primal_discovery::DiscoveredPrimal> = vec![];
        let ctx = RequestContext::new(SimpleCapability::Cryptography);
        let err = router
            .select_primal(&mut empty, &ctx)
            .expect_err("empty slice");
        assert!(err.to_string().contains("No primals available"));
    }

    #[test]
    fn selection_strategy_least_loaded_and_lowest_latency_reason_strings() {
        use crate::primal_discovery::DiscoveredPrimal;
        use std::time::SystemTime;

        let mut primals = vec![
            DiscoveredPrimal {
                name: "a".to_string(),
                endpoints: vec![],
                capabilities: vec![],
                trust_score: Some(0.5),
                discovered_at: SystemTime::now(),
            },
            DiscoveredPrimal {
                name: "b".to_string(),
                endpoints: vec![],
                capabilities: vec![],
                trust_score: Some(0.5),
                discovered_at: SystemTime::now(),
            },
        ];

        let mut router = CapabilityRouter::new(PrimalDiscovery::new(DiscoveryMethod::Environment));
        router.record_success("a", 100.0);
        router.record_success("b", 10.0);

        let ctx_ll = RequestContext::new(SimpleCapability::Cryptography)
            .with_strategy(SelectionStrategy::LeastLoaded);
        let (picked_ll, reason_ll) = router.select_primal(&mut primals, &ctx_ll).unwrap();
        assert_eq!(picked_ll.name, "a");
        assert!(reason_ll.contains("least loaded"));

        let ctx_lat = RequestContext::new(SimpleCapability::Cryptography)
            .with_strategy(SelectionStrategy::LowestLatency);
        let (picked_lat, reason_lat) = router.select_primal(&mut primals, &ctx_lat).unwrap();
        assert_eq!(picked_lat.name, "b");
        assert!(reason_lat.contains("lowest latency"));
    }

    #[test]
    fn selection_strategy_random_returns_index_in_range() {
        use crate::primal_discovery::DiscoveredPrimal;
        use std::time::SystemTime;

        let mut primals = vec![
            DiscoveredPrimal {
                name: "x".to_string(),
                endpoints: vec![],
                capabilities: vec![],
                trust_score: None,
                discovered_at: SystemTime::now(),
            },
            DiscoveredPrimal {
                name: "y".to_string(),
                endpoints: vec![],
                capabilities: vec![],
                trust_score: None,
                discovered_at: SystemTime::now(),
            },
        ];
        let mut router = CapabilityRouter::new(PrimalDiscovery::new(DiscoveryMethod::Environment));
        let ctx = RequestContext::new(SimpleCapability::Cryptography)
            .with_strategy(SelectionStrategy::Random);
        let (sel, reason) = router.select_primal(&mut primals, &ctx).unwrap();
        assert!(["x", "y"].contains(&sel.name.as_str()));
        assert_eq!(reason, "random selection");
    }

    #[tokio::test]
    async fn route_returns_not_found_when_discovery_empty() {
        let discovery =
            PrimalDiscovery::new(DiscoveryMethod::Environment).with_env_override(HashMap::new());
        let mut router = CapabilityRouter::new(discovery);
        let err = router
            .route(
                SimpleCapability::Cryptography,
                RequestContext::new(SimpleCapability::Cryptography),
            )
            .await
            .expect_err("empty env");
        assert!(
            err.to_string().contains("No primals found"),
            "unexpected: {err}"
        );
    }

    #[tokio::test]
    async fn route_excludes_named_primal_and_picks_remaining() {
        let dir = tempfile::tempdir().unwrap();
        let s_keep = dir.path().join("keep.sock");
        let s_skip = dir.path().join("skip.sock");
        std::fs::File::create(&s_keep).unwrap();
        std::fs::File::create(&s_skip).unwrap();

        let mut env = HashMap::new();
        env.insert(
            "PRIMAL_KEEP_ADDR".to_string(),
            format!("unix://{}", s_keep.display()),
        );
        env.insert(
            "PRIMAL_SKIP_ADDR".to_string(),
            format!("unix://{}", s_skip.display()),
        );
        env.insert(
            "PRIMAL_KEEP_CAPABILITIES".to_string(),
            "Cryptography".to_string(),
        );
        env.insert(
            "PRIMAL_SKIP_CAPABILITIES".to_string(),
            "Cryptography".to_string(),
        );

        let discovery = PrimalDiscovery::new(DiscoveryMethod::Environment).with_env_override(env);
        let mut router = CapabilityRouter::new(discovery);
        let decision = router
            .route(
                SimpleCapability::Cryptography,
                RequestContext::new(SimpleCapability::Cryptography).excluding("skip"),
            )
            .await
            .expect("route");
        assert_eq!(decision.primal.name, "keep");
    }

    #[tokio::test]
    async fn route_filters_out_high_latency_primal_when_max_latency_set() {
        let dir = tempfile::tempdir().unwrap();
        let fast_sock = dir.path().join("fast.sock");
        let slow_sock = dir.path().join("slow.sock");
        std::fs::File::create(&fast_sock).unwrap();
        std::fs::File::create(&slow_sock).unwrap();

        let mut env = HashMap::new();
        env.insert(
            "PRIMAL_FAST_ADDR".to_string(),
            format!("unix://{}", fast_sock.display()),
        );
        env.insert(
            "PRIMAL_SLOW_ADDR".to_string(),
            format!("unix://{}", slow_sock.display()),
        );
        env.insert(
            "PRIMAL_FAST_CAPABILITIES".to_string(),
            "Cryptography".to_string(),
        );
        env.insert(
            "PRIMAL_SLOW_CAPABILITIES".to_string(),
            "Cryptography".to_string(),
        );

        let discovery = PrimalDiscovery::new(DiscoveryMethod::Environment).with_env_override(env);
        let mut router = CapabilityRouter::new(discovery);
        router.record_success("slow", 500.0);
        router.record_success("fast", 5.0);

        let decision = router
            .route(
                SimpleCapability::Cryptography,
                RequestContext::new(SimpleCapability::Cryptography).with_max_latency(50),
            )
            .await
            .expect("route");
        assert_eq!(decision.primal.name, "fast");
    }

    #[tokio::test]
    async fn route_errors_when_all_primals_filtered_by_latency() {
        let dir = tempfile::tempdir().unwrap();
        let sock = dir.path().join("only.sock");
        std::fs::File::create(&sock).unwrap();
        let mut env = HashMap::new();
        env.insert(
            "PRIMAL_ONLY_ADDR".to_string(),
            format!("unix://{}", sock.display()),
        );
        env.insert(
            "PRIMAL_ONLY_CAPABILITIES".to_string(),
            "Cryptography".to_string(),
        );

        let discovery = PrimalDiscovery::new(DiscoveryMethod::Environment).with_env_override(env);
        let mut router = CapabilityRouter::new(discovery);
        router.record_success("only", 900.0);

        let err = router
            .route(
                SimpleCapability::Cryptography,
                RequestContext::new(SimpleCapability::Cryptography).with_max_latency(10),
            )
            .await
            .expect_err("filtered");
        assert!(
            err.to_string()
                .contains("No primals match routing criteria"),
            "unexpected: {err}"
        );
    }
}
