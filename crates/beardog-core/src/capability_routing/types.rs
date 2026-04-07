// SPDX-License-Identifier: AGPL-3.0-or-later

//! Public result types for routing decisions.

use crate::primal_discovery::DiscoveredPrimal;
use std::time::Instant;

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
