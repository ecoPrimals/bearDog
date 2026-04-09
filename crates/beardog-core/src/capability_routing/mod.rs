// SPDX-License-Identifier: AGPL-3.0-or-later

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

mod filters;
mod scoring;
pub mod strategy;
mod types;

pub use strategy::{RequestContext, SelectionStrategy};
pub use types::RoutingDecision;

use crate::primal_discovery::{DiscoveredPrimal, DiscoveryQuery, PrimalDiscovery};
use crate::self_knowledge::SimpleCapability;
use beardog_errors::BearDogError;
use std::collections::HashMap;
use std::time::Instant;
use tracing::info;

use scoring::PrimalLoad;

/// Capability-based router
pub struct CapabilityRouter {
    /// Discovery engine
    discovery: PrimalDiscovery,

    /// Load tracking by primal name
    load_tracker: HashMap<String, PrimalLoad>,

    /// Round-robin counters by capability
    rr_counters: HashMap<SimpleCapability, usize>,
}

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
    ///
    /// # Errors
    ///
    /// Returns [`BearDogError`] when [`PrimalDiscovery::from_env`] fails.
    pub fn from_env() -> Result<Self, BearDogError> {
        Ok(Self::new(PrimalDiscovery::from_env()?))
    }

    /// Route a request to the best primal for a capability
    ///
    /// # Errors
    ///
    /// Returns [`BearDogError`] when discovery fails, no primals provide the capability, or routing
    /// cannot select a target.
    pub async fn route(
        &mut self,
        capability: SimpleCapability,
        context: RequestContext,
    ) -> Result<RoutingDecision, BearDogError> {
        info!("🧭 Routing request for capability: {:?}", capability);

        let query = DiscoveryQuery::by_capability(capability.clone()).with_timeout(context.timeout);

        let mut primals = self.discovery.discover(query).await?;

        if primals.is_empty() {
            return Err(BearDogError::not_found(format!(
                "No primals found providing capability: {capability:?}"
            )));
        }

        primals = filters::apply_filters(&self.load_tracker, primals, &context);

        if primals.is_empty() {
            return Err(BearDogError::not_found(
                "No primals match routing criteria".to_string(),
            ));
        }

        let (selected, reason) = self.select_primal(&primals, &context)?;

        info!("✅ Routed to {} (reason: {})", selected.name, reason);

        Ok(RoutingDecision {
            primal: selected,
            reason,
            alternatives: primals,
            decided_at: Instant::now(),
        })
    }

    fn select_primal(
        &mut self,
        primals: &[DiscoveredPrimal],
        context: &RequestContext,
    ) -> Result<(DiscoveredPrimal, String), BearDogError> {
        strategy::select_primal(primals, context, &self.load_tracker, &mut self.rr_counters)
    }

    /// Record successful request completion (for load tracking)
    pub fn record_success(&mut self, primal_name: &str, latency_ms: f64) {
        scoring::record_success(&mut self.load_tracker, primal_name, latency_ms);
    }

    /// Record request failure (for load tracking)
    pub fn record_failure(&mut self, primal_name: &str) {
        scoring::record_failure(primal_name);
    }
}

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
            .with_timeout(std::time::Duration::from_secs(10));

        assert_eq!(ctx.capability, SimpleCapability::Cryptography);
        assert_eq!(ctx.strategy, SelectionStrategy::LeastLoaded);
        assert_eq!(ctx.max_latency_ms, Some(100));
        assert_eq!(ctx.min_trust_score, Some(0.8));
        assert_eq!(ctx.exclude_primals, vec!["untrusted-primal"]);
        assert_eq!(ctx.timeout, std::time::Duration::from_secs(10));
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

        let (selected, reason) = router
            .select_primal(&primals, &context)
            .expect("select_primal highest trust");

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

        let load = router
            .load_tracker
            .get("test-primal")
            .expect("load_tracker entry after record_success");
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
        let (selected1, _) = router
            .select_primal(&primals, &context)
            .expect("round robin first");
        let (selected2, _) = router
            .select_primal(&primals, &context)
            .expect("round robin second");
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
        let (selected, _) = router
            .select_primal(&primals, &context)
            .expect("first available selection");
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
        let err = router.select_primal(&empty, &ctx).expect_err("empty slice");
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
        let (picked_ll, reason_ll) = router
            .select_primal(&primals, &ctx_ll)
            .expect("least loaded selection");
        assert_eq!(picked_ll.name, "a");
        assert!(reason_ll.contains("least loaded"));

        let ctx_lat = RequestContext::new(SimpleCapability::Cryptography)
            .with_strategy(SelectionStrategy::LowestLatency);
        let (picked_lat, reason_lat) = router
            .select_primal(&primals, &ctx_lat)
            .expect("lowest latency selection");
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
        let (sel, reason) = router
            .select_primal(&primals, &ctx)
            .expect("random selection");
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
        let dir = tempfile::tempdir().expect("tempdir for primal sockets");
        let s_keep = dir.path().join("keep.sock");
        let s_skip = dir.path().join("skip.sock");
        std::fs::File::create(&s_keep).expect("create keep.sock fixture");
        std::fs::File::create(&s_skip).expect("create skip.sock fixture");

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
        let dir = tempfile::tempdir().expect("tempdir for latency filter test");
        let fast_sock = dir.path().join("fast.sock");
        let slow_sock = dir.path().join("slow.sock");
        std::fs::File::create(&fast_sock).expect("create fast.sock");
        std::fs::File::create(&slow_sock).expect("create slow.sock");

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
        let dir = tempfile::tempdir().expect("tempdir for latency error test");
        let sock = dir.path().join("only.sock");
        std::fs::File::create(&sock).expect("create only.sock");
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
