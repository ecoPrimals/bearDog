//! # AI Configuration Domain - Modular Structure
//!
//! This module contains all AI/ML related configuration types, now properly
//! split into focused submodules for better maintainability.
//!
//! **Migration Note**: This replaces the monolithic `ai_config_original.rs` (1756 lines)
//! with a clean modular structure following the 1000-line coding standard.

use beardog_errors::{BearDogError, BearDogResult};
use serde::{Deserialize, Serialize};
use std::time::Duration;

use crate::canonical::config::r#trait::BearDogConfig;

// Submodule declarations
pub mod learning;
pub mod management;
pub mod neural_networks;
pub mod training;

// Re-export all public types for compatibility
pub use learning::*;
pub use management::*;
pub use neural_networks::*;
pub use training::*;

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

/// **HYBRID INTELLIGENCE CONFIGURATION** - Human-AI collaboration settings
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct HybridIntelligenceConfig {
    /// Enable hybrid intelligence mode
    pub enabled: bool,

    /// Human oversight level (0.0 = full automation, 1.0 = full human control)
    pub human_oversight_level: f64,

    /// Confidence threshold for automatic decisions
    pub auto_decision_threshold: f64,

    /// Enable human feedback learning
    pub feedback_learning: bool,

    /// Maximum time to wait for human input
    pub human_input_timeout: Duration,
}

impl Default for HybridIntelligenceConfig {
    fn default() -> Self {
        Self {
            enabled: false,
            human_oversight_level: 0.5,
            auto_decision_threshold: 0.9,
            feedback_learning: true,
            human_input_timeout: Duration::from_secs(30),
        }
    }
}

/// **INFERENCE CONFIGURATION** - Model serving and inference settings
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
pub struct InferenceConfig {
    /// Enable inference mode
    pub enabled: bool,

    /// Model serving endpoints
    pub endpoints: Vec<String>,

    /// Inference batch size
    pub batch_size: usize,

    /// Maximum inference latency (milliseconds)
    pub max_latency_ms: u64,

    /// Enable model caching
    pub enable_caching: bool,

    /// Cache size (MB)
    pub cache_size_mb: usize,
}

impl Default for InferenceConfig {
    fn default() -> Self {
        Self {
            enabled: true,
            endpoints: vec![],
            batch_size: 32,
            max_latency_ms: 100,
            enable_caching: true,
            cache_size_mb: 1024,
        }
    }
}

impl BearDogConfig for ConsolidatedAiConfig {
    fn validate(&self) -> BearDogResult<()> {
        if self.enabled {
            if self.hybrid_intelligence.human_oversight_level < 0.0
                || self.hybrid_intelligence.human_oversight_level > 1.0
            {
                return Err(BearDogError::validation(
                    "Human oversight level must be between 0.0 and 1.0",
                ));
            }

            if self.hybrid_intelligence.auto_decision_threshold < 0.0
                || self.hybrid_intelligence.auto_decision_threshold > 1.0
            {
                return Err(BearDogError::validation(
                    "Auto decision threshold must be between 0.0 and 1.0",
                ));
            }
        }

        Ok(())
    }

    fn merge(&self, other: &Self) -> BearDogResult<Self> {
        let mut merged = self.clone();
        if other.enabled {
            merged.enabled = true;
            merged.hybrid_intelligence = other.hybrid_intelligence.clone();
            merged.training = other.training.clone();
            merged.inference = other.inference.clone();
            merged.neural_networks = other.neural_networks.clone();
            merged.decision_engine = other.decision_engine.clone();
            merged.model_management = other.model_management.clone();
            merged.performance = other.performance.clone();
            merged.security = other.security.clone();
        }
        Ok(merged)
    }

    fn from_env() -> BearDogResult<Self> {
        // AI config is typically loaded from files, not environment variables
        Ok(Self::default())
    }

    fn to_toml(&self) -> BearDogResult<String> {
        toml::to_string_pretty(self)
            .map_err(|e| BearDogError::validation(&format!("Failed to serialize to TOML: {}", e)))
    }

    fn domain() -> &'static str {
        "ai"
    }
}
