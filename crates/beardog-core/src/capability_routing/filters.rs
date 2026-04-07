// SPDX-License-Identifier: AGPL-3.0-or-later

//! Candidate filtering before strategy-based selection (trust, latency budget, exclusions).

use crate::primal_discovery::DiscoveredPrimal;
use std::collections::HashMap;

use super::scoring::PrimalLoad;
use super::strategy::RequestContext;

#[expect(
    clippy::cast_possible_truncation,
    reason = "Rounded latency ms compared to u64 budget; practical ms ranges"
)]
#[expect(
    clippy::cast_sign_loss,
    reason = "Latency milliseconds are non-negative before integer comparison"
)]
pub(crate) fn apply_filters(
    load_tracker: &HashMap<String, PrimalLoad>,
    mut primals: Vec<DiscoveredPrimal>,
    context: &RequestContext,
) -> Vec<DiscoveredPrimal> {
    if !context.exclude_primals.is_empty() {
        primals.retain(|p| !context.exclude_primals.contains(&p.name));
    }

    if let Some(min_trust) = context.min_trust_score {
        primals.retain(|p| p.trust_score.is_some_and(|score| score >= min_trust));
    }

    if let Some(max_latency) = context.max_latency_ms {
        primals.retain(|p| {
            load_tracker
                .get(&p.name)
                .and_then(|load| load.avg_latency_ms)
                .is_none_or(|latency| (latency.round() as u64) <= max_latency)
        });
    }

    primals
}
