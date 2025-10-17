//! AI-Powered Hybrid Intelligence System
//!
//! This module provides the AI capabilities for the BearDog ecosystem, including:
//!
//! - **Hybrid Intelligence**: Combines human decision-making with AI assistance
//! - **Neural Networks**: Machine learning models for pattern recognition
//! - **Decision Support**: AI-assisted decision-making frameworks
//! - **Learning Systems**: Adaptive learning and model training
//!
//! The hybrid intelligence system ensures human sovereignty while leveraging
//! AI capabilities for enhanced decision-making and system optimization.
//!
//! # Example
//!
//! ```rust,no_run
//! use beardog_core::ai::{HybridIntelligenceConfig, DecisionStrategy};
//!
//! let config = HybridIntelligenceConfig::default();
//! // Configure AI-assisted decision making
//! ```

/// Hybrid intelligence implementation combining human and AI capabilities
pub mod hybrid_intelligence;

// Re-export available types from hybrid_intelligence
pub use hybrid_intelligence::{
    DecisionCriteria,
    DecisionStrategy,
    HybridIntelligenceConfig,
    InferenceConfig,
    IntelligenceCapability,
    MachineLearningConfig,
    ModelType,
    // Note: Ambiguous types commented out due to multiple definitions:
    // ConsensusStrategy, LearningAlgorithm, OnlineLearningConfig
};

// Note: These types don't exist in the current implementation:
// ActiveHybridWorkflow, CapabilityRequest, CapabilityResponse, ExternalAIResult,
// HybridIntelligenceManager, SecurityMLResult, UniversalAdapter

// Day 2: AI Comprehensive Tests - October 17, 2025
#[cfg(test)]
pub mod tests;
