// SPDX-License-Identifier: AGPL-3.0-only

//! Engine-level types: composable rule conditions, ML artifacts, executable rules, and engine stats.

/// Boolean expressions over normalized string fields.
pub mod conditions;

/// ML model descriptors, training config, and evaluation metrics.
pub mod ml_models;
/// Detection rules, execution results, and validation helpers.
pub mod rules;
/// [`ThreatDetectionEngine`] snapshot type shared with handlers.
pub mod threat_engine;

pub use conditions::RuleCondition;
pub use ml_models::{MlModel, MlModelType};
pub use rules::{
    DetectionRule, RuleExecutionResult, RulePerformanceMetrics, RuleValidationResult,
    ThreatRuleType,
};
pub use threat_engine::{ThreatDetectionEngine as ThreatEngineCore, ThreatDetectionStats};
