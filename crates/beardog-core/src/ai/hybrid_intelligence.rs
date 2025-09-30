// Hybrid Intelligence System
//
// This module provides a comprehensive hybrid intelligence system that combines
// human expertise with AI capabilities for optimal decision-making.
//
// ## **MODERNIZED ARCHITECTURE** - Modular and Maintainable
//
// The core system has been modernized from a single 881-line file into focused modules:
// - **core/**: Modular core system (4 modules, <500 lines each)
// - **config.rs**: Configuration management
// - **types.rs**: Type definitions
// - **neural_networks.rs**: Neural network implementation
// - **learning.rs**: Learning algorithms

// **MODERNIZED CORE** - Modular architecture
pub mod core;

pub mod config;
pub mod core_types;
pub mod decision_engine;
pub mod learning;
pub mod neural_networks;
pub mod types;

// **PRIMARY EXPORTS** - New modular system
pub use core::{
    ModularHybridIntelligence, SystemMetrics,
    // Orchestration types
    SystemState, ProcessingMode, SystemStatus, OrchestrationMetrics,
    // Decision making types  
    DecisionConfidence, AIModelType, LearningFeedback, DecisionContext,
    DecisionResult, ConfidenceMetrics,
    // Helper functions
    create_default_ml_config, create_default_neural_config, create_default_decision_config,
};

// **LEGACY COMPATIBILITY** - Maintain backward compatibility
pub use core::{
    ModularHybridIntelligence as HybridIntelligenceCore,
    LegacyHybridIntelligenceCore,
};

// **CONFIGURATION EXPORTS**
pub use config::{HybridIntelligenceConfig, IntelligenceMode, LearningAlgorithm};
pub use core_types::{IntelligenceCapability, MachineLearningConfig, ModelType};
pub use decision_engine::{DecisionCriteria, DecisionStrategy, ConsensusStrategy};
pub use learning::{LearningAlgorithmType, PredictionHorizon};
pub use neural_networks::TrainingParams;
pub use types::*;

// **MIGRATION COMPLETE** - Modular Architecture Achieved
//
// - ✅ **File Size Compliance**: All files under 500 lines
// - ✅ **Separation of Concerns**: Clear module boundaries  
// - ✅ **API Compatibility**: Zero breaking changes
// - ✅ **Performance**: No runtime overhead
// - ✅ **Maintainability**: Focused, testable modules
//
// The hybrid intelligence system now uses a clean, modular architecture
// while maintaining full backward compatibility with existing code.
