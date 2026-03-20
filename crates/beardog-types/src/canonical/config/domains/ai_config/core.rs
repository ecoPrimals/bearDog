// SPDX-License-Identifier: AGPL-3.0-only

//! Core AI Configuration
//!
//! Consolidated configuration that brings together all AI/ML subsystems.

use beardog_errors::BearDogError;
use super::{HybridIntelligenceConfig, InferenceConfig, TrainingConfig};
use serde::{Deserialize, Serialize};

/// Consolidated AI configuration - unifies all AI/ML related configs
///
/// This is the top-level configuration that brings together:
/// - Hybrid intelligence (human-AI collaboration)
/// - Training (ML model training)
/// - Inference (model serving)
/// - Neural networks (architecture)
/// - Decision engine
/// - Model management
/// - Performance optimization
/// - Security settings
///
/// # Examples
///
/// ```ignore
/// let ai_config = ConsolidatedAiConfig::builder()
///     .hybrid_intelligence(HybridIntelligenceConfig::builder()
///         .oversight_level(OversightLevel::balanced())
///         .build()?)
///     .training(TrainingConfig::builder()
///         .data_source("./data")
///         .build()?)
///     .build();
/// ```
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct ConsolidatedAiConfig {
    /// Enable AI/ML functionality
    enabled: bool,

    /// Hybrid intelligence configuration
    hybrid_intelligence: HybridIntelligenceConfig,

    /// Machine learning training configuration
    training: TrainingConfig,

    /// Model inference and serving configuration
    inference: InferenceConfig,
    // Note: Additional subsystems pending refactoring:
    // - neural_networks: NeuralNetworkConfig
    // - decision_engine: DecisionEngineConfig
    // - model_management: ModelManagementConfig
    // performance: AiPerformanceConfig,
    // security: AiSecurityConfig,
}

impl ConsolidatedAiConfig {
    /// Create a new builder
    pub fn builder() -> ConsolidatedAiConfigBuilder {
        ConsolidatedAiConfigBuilder::default()
    }

    /// Check if AI functionality is enabled
    pub const fn is_enabled(&self) -> bool {
        self.enabled
    }

    /// Get hybrid intelligence configuration
    pub const fn hybrid_intelligence(&self) -> &HybridIntelligenceConfig {
        &self.hybrid_intelligence
    }

    /// Get training configuration
    pub const fn training(&self) -> &TrainingConfig {
        &self.training
    }

    /// Get inference configuration
    pub const fn inference(&self) -> &InferenceConfig {
        &self.inference
    }
}

impl Default for ConsolidatedAiConfig {
    fn default() -> Self {
        Self {
            enabled: false,
            hybrid_intelligence: HybridIntelligenceConfig::default(),
            training: TrainingConfig::default(),
            inference: InferenceConfig::default(),
        }
    }
}

/// Builder for `ConsolidatedAiConfig`
#[derive(Debug, Clone, Default)]
pub struct ConsolidatedAiConfigBuilder {
    enabled: bool,
    hybrid_intelligence: Option<HybridIntelligenceConfig>,
    training: Option<TrainingConfig>,
    inference: Option<InferenceConfig>,
}

impl ConsolidatedAiConfigBuilder {
    /// Enable or disable AI functionality
    pub fn enabled(mut self, enabled: bool) -> Self {
        self.enabled = enabled;
        self
    }

    /// Set hybrid intelligence configuration
    pub fn hybrid_intelligence(mut self, config: HybridIntelligenceConfig) -> Self {
        self.hybrid_intelligence = Some(config);
        self
    }

    /// Set training configuration
    pub fn training(mut self, config: TrainingConfig) -> Self {
        self.training = Some(config);
        self
    }

    /// Set inference configuration
    pub fn inference(mut self, config: InferenceConfig) -> Self {
        self.inference = Some(config);
        self
    }

    /// Build the configuration
    pub fn build(self) -> std::result::Result<ConsolidatedAiConfig, BearDogError> {
        Ok(ConsolidatedAiConfig {
            enabled: self.enabled,
            hybrid_intelligence: self.hybrid_intelligence.unwrap_or_default(),
            training: self.training.unwrap_or_default(),
            inference: self.inference.unwrap_or_default(),
        })
    }

    /// Preset: Production AI configuration with human oversight
    pub fn production_safe() -> std::result::Result<Self, BearDogError> {
        let hybrid = super::HybridIntelligenceConfigBuilder::human_centric().build()?;
        Ok(Self::default().enabled(true).hybrid_intelligence(hybrid))
    }

    /// Preset: Development AI configuration with full automation
    pub fn development() -> std::result::Result<Self, BearDogError> {
        let hybrid = super::HybridIntelligenceConfigBuilder::full_automation().build()?;
        Ok(Self::default().enabled(true).hybrid_intelligence(hybrid))
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_consolidated_builder() {
        let config = ConsolidatedAiConfig::builder()
            // TEST_CATEGORY: unit
            // TEST_DOMAIN: types
            // TEST_PRIORITY: normal
            .enabled(true)
            .build()
            ?;

        assert!(config.is_enabled());
    }

    // TEST_CATEGORY: unit
    // TEST_DOMAIN: types
    // TEST_PRIORITY: normal
    #[test]
    fn test_presets() {
        let prod = ConsolidatedAiConfigBuilder::production_safe()?.build()?;
        assert!(prod.is_enabled());

        let dev = ConsolidatedAiConfigBuilder::development()?.build()?;
        assert!(dev.is_enabled());
    }
}
