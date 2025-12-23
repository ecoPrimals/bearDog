// Unified Monitoring Trait System

pub use super::core::{HealthMonitored, MetricsCollector};

// Observable trait - simple event system
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
