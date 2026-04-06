// SPDX-License-Identifier: AGPL-3.0-or-later

//! AI monitoring configuration and metric categories.

use serde::{Deserialize, Serialize};
use std::collections::HashSet;

/// AI monitoring metric types
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub enum AIMetricType {
    /// Training metrics (loss, accuracy, learning rate, etc.)
    Training,
    /// Inference metrics (latency, throughput, batch size, etc.)
    Inference,
    /// Model performance tracking over time
    ModelPerformance,
    /// Resource usage monitoring (CPU, memory, GPU)
    ResourceUsage,
}

/// AI-specific monitoring configuration for model observability
///
/// Configures collection of metrics and telemetry specific to AI model training,
/// inference, performance, and resource utilization in production.
///
/// Evolved from multiple boolean fields to a set-based approach, following
/// modern idiomatic Rust patterns and clippy recommendations.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct AIMonitoringConfig {
    /// Enabled metric types
    #[serde(default = "default_enabled_ai_metrics")]
    pub enabled_metrics: HashSet<AIMetricType>,
}

fn default_enabled_ai_metrics() -> HashSet<AIMetricType> {
    [
        AIMetricType::Training,
        AIMetricType::Inference,
        AIMetricType::ModelPerformance,
        AIMetricType::ResourceUsage,
    ]
    .into_iter()
    .collect()
}

impl Default for AIMonitoringConfig {
    fn default() -> Self {
        Self {
            enabled_metrics: default_enabled_ai_metrics(),
        }
    }
}

impl AIMonitoringConfig {
    /// Check if training metrics are enabled
    #[must_use]
    pub fn collects_training_metrics(&self) -> bool {
        self.enabled_metrics.contains(&AIMetricType::Training)
    }

    /// Check if inference metrics are enabled
    #[must_use]
    pub fn collects_inference_metrics(&self) -> bool {
        self.enabled_metrics.contains(&AIMetricType::Inference)
    }

    /// Check if model performance tracking is enabled
    #[must_use]
    pub fn tracks_model_performance(&self) -> bool {
        self.enabled_metrics
            .contains(&AIMetricType::ModelPerformance)
    }

    /// Check if resource usage monitoring is enabled
    #[must_use]
    pub fn monitors_resource_usage(&self) -> bool {
        self.enabled_metrics.contains(&AIMetricType::ResourceUsage)
    }
}

/// Backward compatibility alias for `AIMonitoringConfig`
///
/// Deprecated: Use `AIMonitoringConfig` directly instead.
#[deprecated(since = "3.1.0", note = "Use AIMonitoringConfig instead")]
pub type MonitoringConfig = AIMonitoringConfig;
