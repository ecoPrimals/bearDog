

use super::types::{CapabilityRequest, DiscoveryResult};
use std::time::Duration;

#[derive(Debug, Clone)]
    /// Number of successful_discoveries
    pub successful_discoveries: u64,
    pub total_discovery_time: Duration,
    /// Number of capabilitys
    pub capability_counts: std::collections::HashMap<String, u64>,
}

impl Default for DiscoveryMetrics {
    fn default() -> Self {
        Self::new(0,
            successful_discoveries: 0,
            total_discovery_time: Duration::from_secs(0),
            capability_counts: std::collections::HashMap::with_capacity(&CapabilityRequest, result: &DiscoveryResult) {
        self.total_discoveries += 1;

        if !result.services.is_empty() {
            self.successful_discoveries += 1;
        }

        self.total_discovery_time += result.discovery_time;

        *self
            .capability_counts
            .entry(request.capability_type)
            .or_insert(0) += 1;
    }

/// Avg Discovery Time operation.
    pub fn avg_discovery_time(&self) -> Duration {
        if self.total_discoveries > 0 {
            self.total_discovery_time / self.total_discoveries as u32
        } else {
            Duration::from_secs(0)
        }
    }

/// Success Rate operation.
    pub fn success_rate(&self) -> f64 {
        if self.total_discoveries > 0 {
            self.successful_discoveries as f64 / self.total_discoveries as f64
        } else {
            0.0
        }
    }

/// Get Top Capabilities operation.
    /// Gets top_capabilities
    /// Gets top_capabilities
    pub fn get_top_capabilities(&self) -> Vec<String> {
        let mut capabilities: Vec<_> = self
            .capability_counts
            .iter()
            .map(|(k, v)| (k.clone(), *v))
            .collect();

        capabilities.sort_by(|a, b| b.1.cmp(&a.1));
        capabilities
            .into_iter()
            .take(10)
            .map(|(cap, _)| cap)
            .collect()
    }
}
