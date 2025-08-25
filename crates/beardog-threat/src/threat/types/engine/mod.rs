// BearDog - Enterprise Security Ecosystem
// Copyright (C) 2025 EcoPrimals
//
// This program is free software: you can redistribute it and/or modify
// it under the terms of the GNU Affero General Public License as published by
// the Free Software Foundation, either version 3 of the License, or
// (at your option) any later version.
//
// This program is distributed in the hope that it will be useful,
// but WITHOUT ANY WARRANTY; without even the implied warranty of
// MERCHANTABILITY or FITNESS FOR A PARTICULAR PURPOSE. See the
// GNU Affero General Public License for more details.
//
// You should have received a copy of the GNU Affero General Public License
// along with this program. If not, see <https://www.gnu.org/licenses/>.


/// Threat detection engine types and structures
///
/// This module contains all the types and structures used by the threat detection engine,
/// organized into focused sub-modules for better maintainability.
/// ## Modules
/// - `engine`: Main threat detection engine structure
/// - `rules`: Detection rule types and management
/// - `conditions`: Rule condition types and evaluation logic
/// - `ml_models`: Machine learning model types and metadata
/// ## Example Usage
/// ```rust
/// use beardog::threat::types::engine::{
///     ThreatDetectionEngine, DetectionRule, RuleCondition, MlModel, MlModelType
/// };
/// // Create a new engine
/// let engine = ThreatDetectionEngine::default();
/// // Create a detection rule
/// let rule = DetectionRule::new(
///     "brute-force-001".to_string(),
///     "Brute Force Detection".to_string(),
///     "Detects brute force login attempts".to_string(),
///     RuleCondition::frequency_threshold(10, 5),
///     ThreatType::BruteForce,
///     ThreatSeverity::High,
/// );
/// // Create an ML model
/// let model = MlModel::new(
///     "anomaly-detector-v1".to_string(),
///     "Network Anomaly Detector".to_string(),
///     MlModelType::AnomalyDetection,
///     vec!["packet_count".to_string(), "byte_count".to_string()],
/// ```
pub mod conditions;
#[allow(clippy::module_inception)]
pub mod engine;
pub mod ml_models;
pub mod rules;

// Re-export all types for convenience
pub use conditions::RuleCondition;
pub use engine::ThreatDetectionEngine;
pub use ml_models::{MlModel, MlModelType};
pub use rules::{DetectionRule, ThreatDetectionRule};
