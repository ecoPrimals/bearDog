// SPDX-License-Identifier: AGPL-3.0-or-later

//! Runtime types for hybrid intelligence: decisions, predictions, events, and status.

use super::super::core_types::IntelligenceCapability;
use super::super::learning::PredictionHorizon;
use beardog_types::canonical::HealthStatus;
use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use uuid::Uuid;

/// Decision context information
///
/// Provides contextual information for making a decision, including
/// priority, time constraints, and confidence requirements.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct DecisionContext {
    /// Unique identifier for this decision context
    pub context_id: String,
    /// Priority level of the decision (0-100, higher is more urgent)
    pub priority_level: u8,
    /// Maximum time allowed for decision in milliseconds
    pub time_limit_ms: u64,
    /// Required confidence level for automated decision (0.0-1.0)
    pub required_confidence: f64,
}

/// System control commands
///
/// Commands for controlling the hybrid intelligence system lifecycle,
/// configuration, and capabilities.
#[derive(Debug, Clone)]
pub enum SystemCommand {
    /// Start the system
    Start,
    /// Stop the system
    Stop,
    /// Restart the system
    Restart,
    /// Update configuration
    UpdateConfig(Box<super::super::config::HybridIntelligenceConfig>),
    /// Add capability
    AddCapability(IntelligenceCapability),
    /// Remove capability
    RemoveCapability(IntelligenceCapability),
    /// Get system status
    GetStatus,
    /// Reset system
    Reset,
}

/// Prediction result
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PredictionResult {
    /// Prediction ID
    pub id: Uuid,
    /// Predicted values
    /// Collection of predictions
    pub predictions: Vec<f64>,
    /// Confidence intervals (if available)
    pub confidence_intervals: Option<Vec<(f64, f64)>>,
    /// Uncertainty estimates (if available)
    pub uncertainty: Option<Vec<f64>>,
    /// Identifier of the model that generated this prediction
    pub model_id: String,
    /// Prediction timestamp
    pub timestamp: DateTime<Utc>,
    /// Prediction horizon
    /// Optional horizon
    pub horizon: Option<PredictionHorizon>,
}

/// Intelligence event
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct IntelligenceEvent {
    /// Event ID
    pub id: Uuid,
    /// Event type
    /// The event type value
    pub event_type: IntelligenceEventType,
    /// Event data
    /// Mapping of data
    pub data: HashMap<String, serde_json::Value>,
    /// Event timestamp
    pub timestamp: DateTime<Utc>,
}

#[derive(Debug, Clone, Copy, Serialize, Deserialize)]
/// Types of intelligence event
pub enum IntelligenceEventType {
    /// Model trained
    ModelTrained,
    /// Prediction made
    PredictionMade,
    /// Decision made
    DecisionMade,
    /// Learning update
    LearningUpdate,
    /// Optimization completed
    OptimizationCompleted,
    /// Anomaly detected
    AnomalyDetected,
    /// Performance threshold crossed (above or below limit)
    PerformanceThresholdCrossed,
}

/// Intelligence system performance metrics
///
/// Tracks key performance indicators for the hybrid intelligence system
/// including predictions, decisions, and model training statistics.
#[derive(Debug, Clone, Copy, Serialize, Deserialize)]
pub struct IntelligenceMetrics {
    /// Total predictions made
    pub total_predictions: u64,
    /// Total decisions made
    pub total_decisions: u64,
    /// Total models trained
    pub total_models_trained: u64,
    /// Average prediction accuracy (0.0 to 1.0)
    pub avg_prediction_accuracy: f64,
    /// Average decision confidence (0.0 to 1.0)
    pub avg_decision_confidence: f64,
    /// System uptime in seconds
    pub uptime_secs: u64,
    /// Memory usage in MB
    /// The memory usage mb value
    pub memory_usage_mb: f64,
    /// CPU usage percentage
    /// The cpu usage percent value
    pub cpu_usage_percent: f64,
    /// GPU usage percentage (if available)
    /// Optional gpu usage percent
    pub gpu_usage_percent: Option<f64>,
}

impl Default for IntelligenceMetrics {
    fn default() -> Self {
        Self {
            total_predictions: 0,
            total_decisions: 0,
            total_models_trained: 0,
            avg_prediction_accuracy: 0.0,
            avg_decision_confidence: 0.0,
            uptime_secs: 0,
            memory_usage_mb: 0.0,
            cpu_usage_percent: 0.0,
            gpu_usage_percent: None,
        }
    }
}

/// Decision result
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct DecisionResult {
    /// Decision ID
    pub decision_id: Uuid,
    /// Decision outcome
    /// The decision value
    pub decision: String,
    /// Confidence level
    pub confidence: f64,
    /// Reasoning explanation for the decision
    pub reasoning: String,
    /// Decision timestamp
    pub timestamp: DateTime<Utc>,
    /// Additional contextual data used to make the decision
    pub context: HashMap<String, serde_json::Value>,
}

/// System status information for the hybrid intelligence system
///
/// Provides comprehensive status including health, active capabilities,
/// metrics, and last update timestamp.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SystemStatus {
    /// System ID
    pub system_id: String,
    /// Current health status
    /// The health value
    pub health: HealthStatus,
    /// Active capabilities
    /// Collection of active capabilities
    pub active_capabilities: Vec<IntelligenceCapability>,
    /// Current metrics
    /// The metrics value
    pub metrics: IntelligenceMetrics,
    /// Last updated timestamp
    /// The last updated value
    pub last_updated: DateTime<Utc>,
}
