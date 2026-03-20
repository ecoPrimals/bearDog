// SPDX-License-Identifier: AGPL-3.0-only

//! Re-exports core health/metrics traits plus a minimal pub/sub [`Observable`] surface.

pub use super::core::{HealthMonitored, MetricsCollector};

/// Optional publish/subscribe hook for streaming monitoring events.
pub trait Observable: Send + Sync {
    /// Event type
    type Event: Send + Sync + Clone;

    /// Subscribe to events (simplified)
    /// Checks if subscribers
    fn has_subscribers(&self) -> bool {
        false
    }

    /// Get subscriber count
    fn subscriber_count(&self) -> usize {
        0
    }
}
