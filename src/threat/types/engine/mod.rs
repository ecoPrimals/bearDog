//! Threat detection engine types and structures
//!
//! This module contains all the types and structures used by the threat detection engine,
//! organized into focused sub-modules for better maintainability.
//!
//! ## Modules
//!
//! - `engine`: Main threat detection engine structure
//! - `rules`: Detection rule types and management
//! - `conditions`: Rule condition types and evaluation logic
//! - `ml_models`: Machine learning model types and metadata
//!
//! ## Example Usage
//!
//! ```rust
//! use beardog::threat::types::engine::{
//!     ThreatDetectionEngine, DetectionRule, RuleCondition, MlModel, MlModelType
//! };
//!
//! // Create a new engine
//! let engine = ThreatDetectionEngine::default();
//!
//! // Create a detection rule
//! let rule = DetectionRule::new(
//!     "brute-force-001".to_string(),
//!     "Brute Force Detection".to_string(),
//!     "Detects brute force login attempts".to_string(),
//!     RuleCondition::frequency_threshold(10, 5),
//!     ThreatType::BruteForce,
//!     ThreatSeverity::High,
//! );
//!
//! // Create an ML model
//! let model = MlModel::new(
//!     "anomaly-detector-v1".to_string(),
//!     "Network Anomaly Detector".to_string(),
//!     MlModelType::AnomalyDetection,
//!     vec!["packet_count".to_string(), "byte_count".to_string()],
//! );
//! ```

pub mod engine;
pub mod rules;
pub mod conditions;
pub mod ml_models;

// Re-export all types for convenience
pub use engine::ThreatDetectionEngine;
pub use rules::{DetectionRule, ThreatDetectionRule};
pub use conditions::RuleCondition;
pub use ml_models::{MlModel, MlModelType}; 