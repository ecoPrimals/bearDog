// SPDX-License-Identifier: AGPL-3.0-only

// Module documentation
//
// This module provides functionality for the BearDog ecosystem.

pub mod conditions;

pub mod ml_models;
pub mod rules;
pub mod threat_engine;

pub use conditions::RuleCondition;
pub use ml_models::{MlModel, MlModelType};
pub use rules::{
    DetectionRule, RuleExecutionResult, RulePerformanceMetrics, RuleValidationResult,
    ThreatRuleType,
};
pub use threat_engine::{ThreatDetectionEngine as ThreatEngineCore, ThreatDetectionStats};
