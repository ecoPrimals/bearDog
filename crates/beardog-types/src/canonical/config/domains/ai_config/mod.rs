// SPDX-License-Identifier: AGPL-3.0-or-later

//! # AI Configuration Domain - Modular Structure
//!
//! This module contains all AI/ML related configuration types, now properly
//! split into focused submodules for better maintainability.
//!
//! **Migration Note**: This replaces the monolithic `ai_config_original.rs` (1756 lines)
//! with a clean modular structure following the 1000-line coding standard.

use beardog_config::env_keys;
use beardog_errors::BearDogError;
use serde::{Deserialize, Serialize};
use std::time::Duration;

use crate::canonical::config::r#trait::BearDogConfig;

// Submodule declarations
pub mod core_learning;
pub mod core_neural_networks;
pub mod learning;
pub mod management;
pub mod neural_networks;
pub mod training;

// Re-export all public types for compatibility
pub use learning::*;
pub use management::{
    AiPerformanceConfig, AiSecurityConfig, DecisionEngineConfig, ModelManagementConfig,
};
pub use neural_networks::NeuralNetworkConfig;
pub use training::TrainingConfig;

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
            human_oversight_level: std::env::var(env_keys::ENV_AI_HUMAN_OVERSIGHT_LEVEL)
                .ok()
                .and_then(|s| s.parse().ok())
                .unwrap_or(0.5),
            auto_decision_threshold: std::env::var(env_keys::ENV_AI_AUTO_DECISION_THRESHOLD)
                .ok()
                .and_then(|s| s.parse().ok())
                .unwrap_or(0.9),
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
            batch_size: std::env::var(env_keys::ENV_AI_BATCH_SIZE)
                .ok()
                .and_then(|s| s.parse().ok())
                .unwrap_or(32),
            max_latency_ms: std::env::var(env_keys::ENV_AI_MAX_LATENCY_MS)
                .ok()
                .and_then(|s| s.parse().ok())
                .unwrap_or(100),
            enable_caching: true,
            cache_size_mb: std::env::var(env_keys::ENV_AI_CACHE_SIZE_MB)
                .ok()
                .and_then(|s| s.parse().ok())
                .unwrap_or(1024),
        }
    }
}

impl BearDogConfig for ConsolidatedAiConfig {
    fn validate(&self) -> Result<(), BearDogError> {
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

    fn merge(&self, other: &Self) -> Result<Self, BearDogError> {
        // When other is enabled, it takes full precedence
        // This is more idiomatic and reduces clones from 8 to 1
        if other.enabled {
            Ok(other.clone())
        } else {
            Ok(self.clone())
        }
    }

    fn from_env() -> Result<Self, BearDogError> {
        // AI config is typically loaded from files, not environment variables
        Ok(Self::default())
    }

    fn to_toml(&self) -> Result<String, BearDogError> {
        toml::to_string_pretty(self)
            .map_err(|e| BearDogError::validation(&format!("Failed to serialize to TOML: {e}")))
    }

    fn domain() -> &'static str {
        "ai"
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::canonical::config::r#trait::BearDogConfig;

    #[test]
    fn consolidated_ai_default_and_domain() {
        let c = ConsolidatedAiConfig::default();
        assert!(!c.enabled);
        assert_eq!(ConsolidatedAiConfig::domain(), "ai");
    }

    #[test]
    fn consolidated_ai_validate_ok_when_disabled_even_if_invalid_floats() {
        let mut c = ConsolidatedAiConfig::default();
        c.hybrid_intelligence.human_oversight_level = 2.0;
        c.validate().expect("validation skipped when AI disabled");
    }

    #[test]
    fn consolidated_ai_validate_oversight_range() {
        let mut c = ConsolidatedAiConfig::default();
        c.enabled = true;
        c.hybrid_intelligence.human_oversight_level = 1.5;
        assert!(c.validate().is_err());
        c.hybrid_intelligence.human_oversight_level = 0.5;
        c.hybrid_intelligence.auto_decision_threshold = 2.0;
        assert!(c.validate().is_err());
    }

    #[test]
    fn consolidated_ai_merge() {
        let a = ConsolidatedAiConfig::default();
        let mut b = ConsolidatedAiConfig::default();
        b.enabled = true;
        let m = a.merge(&b).expect("merge");
        assert!(m.enabled);

        let b2 = ConsolidatedAiConfig::default();
        let m2 = b.merge(&b2).expect("merge when other disabled");
        assert_eq!(m2.enabled, b.enabled);
    }

    #[test]
    fn consolidated_ai_from_env_and_toml() {
        let c = ConsolidatedAiConfig::from_env().expect("from_env");
        let s = c.to_toml().expect("to_toml");
        assert!(!s.is_empty());
    }

    #[test]
    fn hybrid_intelligence_and_inference_defaults_serde() {
        let h = HybridIntelligenceConfig::default();
        let v = serde_json::to_value(&h).expect("serialize hybrid");
        let _: HybridIntelligenceConfig = serde_json::from_value(v).expect("deserialize hybrid");

        let i = InferenceConfig::default();
        let v = serde_json::to_value(&i).expect("serialize inference");
        let back: InferenceConfig = serde_json::from_value(v).expect("deserialize inference");
        assert_eq!(i, back);
    }
}
