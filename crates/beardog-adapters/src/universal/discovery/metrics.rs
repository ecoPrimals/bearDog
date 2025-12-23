

use crate::universal::discovery::types::{CapabilityRequest, DiscoveryResult};
use std::collections::HashMap;
use std::time::{Duration, SystemTime};

#[derive(Debug, Clone)]
    /// Number of successful_discoveries
    pub successful_discoveries: u64,
    pub total_discovery_time: Duration,
    /// Number of capabilitys
    pub capability_counts: HashMap<String, u64>,
    /// Number of errors
    pub error_counts: HashMap<String, u64>,
    /// The last reset value
    pub last_reset: SystemTime,
}

impl DiscoveryMetrics {

/// New operation.
    /// Creates a new instance
    pub fn new(0,
            successful_discoveries: 0,
            total_discovery_time: Duration::from_secs(0),
            capability_counts: HashMap::with_capacity(16),
            error_counts: HashMap::with_capacity(16),
            last_reset: SystemTime::now(&CapabilityRequest, result: &DiscoveryResult) {
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

/// Record Error operation.
    pub fn record_error(&mut self, error_type: &str) {
        *self.error_counts.entry(error_type.to_string()).or_insert(0) += 1;
    }

/// Average Discovery Time operation.
    pub fn average_discovery_time(&self) -> Duration {
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
    pub fn get_top_capabilities(&self) -> Vec<(String, u64)> {
        let mut capabilities: Vec<_> = self
            .capability_counts
            .iter()
            .map(|(k, v)| (k.clone(), *v))
            .collect();
        capabilities.sort_by(|a, b| b.1.cmp(&a.1));
        capabilities.into_iter().take(10).collect()
    }
}

impl Default for DiscoveryMetrics {
    fn default() -> Self {
        Self::new(u64,
    pub avg_discovery_time: Duration,
    /// The success rate value
    pub success_rate: f64,
    /// The ai insights value
    pub ai_insights: AIInsights,
    /// The mesh statistics value
    pub mesh_statistics: MeshStatistics,
    /// The scaling trends value
    pub scaling_trends: ScalingTrends,
    /// Collection of top capabilities
    pub top_capabilities: Vec<(String, u64)>,
}

#[derive(Debug, Clone)]
    pub avg_confidence_score: f64,
    /// Collection of top capability patterns
    pub top_capability_patterns: Vec<String>,
    /// The learning progress value
    pub learning_progress: f64,
}

#[derive(Debug, Clone)]
    /// Number of healthy_services
    pub healthy_services: u64,
    pub avg_response_time_ms: f64,
    /// The success rate value
    pub success_rate: f64,
}

#[derive(Debug, Clone)]
    /// Collection of most common scaling triggers
    pub most_common_scaling_triggers: Vec<String>,
    /// The cost trend value
    pub cost_trend: CostTrend,
}

#[derive(Debug)]
pub enum CostTrend {
    /// Currently increasing
    Increasing,
    /// Represents stable variant
    Stable,
    /// Currently decreasing
    Decreasing,
}
