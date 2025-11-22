//! # AI Configuration Domain (Modular)
//!
//! This module contains all AI/ML related configuration types, split into
//! focused submodules for better maintainability (max 500 lines per module).
//!
//! ## Module Organization
//! - `hybrid`: Hybrid intelligence and human-AI collaboration settings
//! - `training`: ML model training configuration
//! - `inference`: Model serving and inference settings
//! - `neural`: Neural network architecture configuration
//! - `management`: Model management, decision engine, performance, security

use beardog_errors::BearDogError;
use beardog_errors::BearDogError;
use serde::{Deserialize, Serialize};

use crate::canonical::config::r#trait::BearDogConfig;

// Submodules
pub mod hybrid;
pub mod training;
pub mod inference;
pub mod neural;
pub mod management;

// Re-export all types for convenience
pub use hybrid::*;
pub use training::*;
pub use inference::*;
pub use neural::*;
pub use management::*;

/// **CONSOLIDATED AI CONFIGURATION** - Unifies all AI/ML related configs
///
/// Consolidates: `HybridIntelligenceConfig`, `MachineLearningConfig`, `TrainingConfig`,
/// `InferenceConfig`, `PredictionConfig`, `NeuralNetworkConfig`, `DecisionEngineConfig`, etc.
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Default)]
pub struct ConsolidatedAiConfig {
    /// Enable AI/ML functionality
    pub enabled: bool,
    
    /// Hybrid intelligence configuration
    pub hybrid_intelligence: HybridIntelligenceConfig,
    
    /// Machine learning training configuration
    pub training: TrainingConfig,
    
    /// Model inference and serving configuration
    pub inference: InferenceConfig,
    
    /// Neural network architecture configuration
    pub neural_networks: NeuralNetworkConfig,
    
    /// Decision engine configuration
    pub decision_engine: DecisionEngineConfig,
    
    /// Model management and deployment
    pub model_management: ModelManagementConfig,
    
    /// AI performance optimization settings
    pub performance: AiPerformanceConfig,
    
    /// AI security and privacy settings
    pub security: AiSecurityConfig,
}

impl BearDogConfig for ConsolidatedAiConfig {
    fn validate(&self) -> Result<()> {
        if !self.enabled {
            return Ok(());
        }
        
        // Validate subconfigs
        self.hybrid_intelligence.validate()?;
        self.training.validate()?;
        self.inference.validate()?;
        self.neural_networks.validate()?;
        self.decision_engine.validate()?;
        self.model_management.validate()?;
        self.performance.validate()?;
        self.security.validate()?;
        
        Ok(())
    }
    
    fn from_env() -> Result<Self> {
        Ok(Self::default())
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_consolidated_ai_config_default() {
        let config = ConsolidatedAiConfig::default();
        assert!(!config.enabled); // Disabled by default
    }

    #[test]
    fn test_ai_config_validation() {
        let config = ConsolidatedAiConfig::default();
        assert!(config.validate().is_ok());
    }
} 