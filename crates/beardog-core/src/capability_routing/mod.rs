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
//! use beardog_core::capability_routing::{CapabilityRouter, RequestContext};
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
#[path = "capability_routing_tests.rs"]
mod tests;
