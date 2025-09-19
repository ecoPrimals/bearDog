// Module documentation
//
// This module provides functionality for the BearDog ecosystem.

pub mod hybrid_intelligence;

// Re-export available types from hybrid_intelligence
pub use hybrid_intelligence::{
    DecisionCriteria,
    DecisionStrategy,
    HybridIntelligenceConfig,
    InferenceConfig,
    IntelligenceCapability,
    // Note: Ambiguous types commented out due to multiple definitions:
    // ConsensusStrategy, LearningAlgorithm, OnlineLearningConfig, ModelType
};

// Note: These types don't exist in the current implementation:
// ActiveHybridWorkflow, CapabilityRequest, CapabilityResponse, ExternalAIResult,
// HybridIntelligenceManager, SecurityMLResult, UniversalAdapter
