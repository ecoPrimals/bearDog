// SPDX-License-Identifier: AGPL-3.0-only

//! Serializable model weights, telemetry samples, and recommendation records for the AI optimizer.

use beardog_errors::BearDogError;
use serde::{Deserialize, Serialize};
use std::collections::HashMap;

/// Learned linear-style weights per subsystem used to score health and pick actions.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PerformanceModel {
    /// Collection of cpu utilization weights
    pub cpu_utilization_weights: Vec<f64>,
    /// Collection of memory usage weights
    pub memory_usage_weights: Vec<f64>,
    /// Collection of network latency weights
    pub network_latency_weights: Vec<f64>,
    /// Relative importance of cryptographic throughput observations.
    pub crypto_performance_weights: Vec<f64>,
    /// Named knobs → model certainty in `[0, 1]`.
    pub confidence_scores: HashMap<String, f64>,
    /// Number of `last_updated`
    pub last_updated: u64,
}

/// Single point-in-time metrics fed into [`PerformanceModel::update_weights`].
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PerformanceSample {
    /// Epoch millis (or monotonic tick) for ordering samples.
    pub timestamp: u64,
    /// The cpu usage value
    pub cpu_usage: f64,
    /// The memory usage value
    pub memory_usage: f64,
    /// The network latency value
    pub network_latency: f64,
    /// The crypto throughput value
    pub crypto_throughput: f64,
    /// End-to-end latency of a representative request.
    pub response_time: f64,
    /// The error rate value
    pub error_rate: f64,
    /// The system load value
    pub system_load: f64,
}

/// Record of an optimization attempt and its measured outcome (if known).
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct OptimizationAction {
    /// When the action was applied.
    pub timestamp: u64,
    /// The action type value
    pub action_type: OptimizationType,
    /// Mapping of parameters
    pub parameters: HashMap<String, f64>,
    /// The expected improvement value
    pub expected_improvement: f64,
    /// Optional actual improvement
    pub actual_improvement: Option<f64>,
    /// Optional success
    pub success: Option<bool>,
}

#[derive(Debug, Clone, PartialEq, Eq, Hash, Serialize, Deserialize)]
/// Types of optimization
pub enum OptimizationType {
    /// Represents thread pool variant
    ThreadPool,
    /// Represents memory variant
    Memory,
    /// Represents crypto variant
    Crypto,
    /// Represents network variant
    Network,
    /// Represents cache variant
    Cache,
    /// Represents garbage collection variant
    GarbageCollection,
    /// Represents simd variant
    Simd,
    /// Represents pool variant
    Pool,
}

/// Suggested tuning produced by the engine before execution.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct OptimizationRecommendation {
    /// The optimization type value
    pub optimization_type: OptimizationType,
    /// Model belief in this recommendation, typically `[0, 1]`.
    pub confidence: f64,
    /// The expected improvement value
    pub expected_improvement: f64,
    /// Mapping of parameters
    pub parameters: HashMap<String, f64>,
    /// The reasoning value
    pub reasoning: String,
    /// The priority value
    pub priority: RecommendationPriority,
}

/// Relative urgency for applying a recommendation.
#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord, Serialize, Deserialize)]
pub enum RecommendationPriority {
    /// Represents low variant
    Low,
    /// Represents medium variant
    Medium,
    /// Represents high variant
    High,
    /// Represents critical variant
    Critical,
}

/// Rolling quality metrics for how well predictions matched reality.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct AIOptimizationStats {
    /// Number of `total_optimizations`
    pub total_optimizations: u64,
    /// Number of `successful_optimizations`
    pub successful_optimizations: u64,
    /// The average improvement value
    pub average_improvement: f64,
    /// The learning accuracy value
    pub learning_accuracy: f64,
    /// The prediction accuracy value
    pub prediction_accuracy: f64,
    /// Number of `anomalies_detected`
    pub anomalies_detected: u64,
    /// Aggregate trust in the current weight tensor.
    pub model_confidence: f64,
}

impl Default for PerformanceModel {
    fn default() -> Self {
        Self::new()
    }
}

impl PerformanceModel {
    /// Default heuristic weights with empty confidence map.
    #[must_use]
    pub fn new() -> Self {
        Self {
            cpu_utilization_weights: vec![0.3, 0.2, 0.1, 0.4],
            memory_usage_weights: vec![0.4, 0.3, 0.2, 0.1],
            network_latency_weights: vec![0.2, 0.3, 0.3, 0.2],
            crypto_performance_weights: vec![0.1, 0.4, 0.4, 0.1],
            confidence_scores: HashMap::new(),
            last_updated: 0,
        }
    }

    /// Applies a tiny learning-rate nudge when CPU/memory look saturated.
    ///
    /// # Errors
    ///
    /// Currently always returns `Ok(())`; the `Result` is reserved for future validation.
    pub fn update_weights(&mut self, sample: &PerformanceSample) -> Result<(), BearDogError> {
        // Simplified weight update logic
        let learning_rate = 0.01;

        // Update CPU weights based on performance
        if sample.cpu_usage > 0.8 {
            for weight in &mut self.cpu_utilization_weights {
                *weight *= 1.0 + learning_rate;
            }
        }

        // Update memory weights
        if sample.memory_usage > 0.7 {
            for weight in &mut self.memory_usage_weights {
                *weight *= 1.0 + learning_rate;
            }
        }

        self.last_updated = sample.timestamp;
        Ok(())
    }
}
