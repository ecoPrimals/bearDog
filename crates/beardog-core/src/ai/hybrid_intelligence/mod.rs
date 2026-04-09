// SPDX-License-Identifier: AGPL-3.0-or-later

//! # Hybrid Intelligence System
//!
//! Provides advanced AI capabilities with human-in-the-loop intelligence,
//! combining machine learning with human expertise for optimal decision making.
//!
//! ## Overview
//!
//! The hybrid intelligence system bridges the gap between pure AI and pure human
//! decision-making, creating a symbiotic relationship where:
//! - AI provides rapid pattern recognition and suggestions
//! - Humans provide ethical oversight and final decisions
//! - Together they achieve better outcomes than either alone
//!
//! ## Key Components
//!
//! - Configuration management
//! - `core` - Core hybrid intelligence functionality
//! - Core builder - Builder pattern for system construction
//! - `types` - Type definitions and data structures
//!
//! ## Example
//!
//! ```rust,ignore
//! use beardog_core::ai::hybrid_intelligence::{HybridIntelligenceConfig, core::HybridIntelligenceCore};
//!
//! # async fn example() -> Result<(), beardog_errors::BearDogError> {
//! let config = HybridIntelligenceConfig::default();
//! let intelligence = HybridIntelligenceCore::new(config)?;
//!
//! // AI assists with suggestions, human makes final decision
//! let suggestion = intelligence.suggest_action("scenario").await?;
//! # Ok(())
//! # }
//! ```

/// Configuration management for hybrid intelligence
pub mod config;

/// Core hybrid intelligence functionality
pub mod core;

/// Core builder pattern for hybrid intelligence
pub mod core_builder;
/// Core enumeration types
pub mod core_enums;
/// Core type definitions for hybrid intelligence
pub mod core_types;
/// AI-powered decision engine with human oversight
pub mod decision_engine;
/// Machine learning and model training components
pub mod learning;
/// Neural network implementations and architectures
pub mod neural_networks;
/// Sovereign random number generation with cryptographic guarantees
pub mod sovereign_rng;
/// Shared type definitions
pub mod types;

#[cfg(test)]
mod learning_tests;

// Selective re-exports to avoid ambiguity
pub use config::{HybridIntelligenceConfig, IntelligenceMode, LearningAlgorithm};
pub use core_types::{IntelligenceCapability, MachineLearningConfig, ModelType};
pub use decision_engine::{DecisionCriteria, DecisionStrategy};
pub use sovereign_rng::{SovereignRng, SovereignRngConfig, SovereignRngStats};
pub use types::InferenceConfig;
