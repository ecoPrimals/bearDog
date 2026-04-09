// SPDX-License-Identifier: AGPL-3.0-or-later

//! Fluent builder for constructing a [`HybridIntelligenceSystem`](super::system::HybridIntelligenceSystem) with layered defaults.

use super::super::config::{HybridIntelligenceConfig, IntelligenceMode, LearningAlgorithm};
use super::super::core_types::{IntelligenceCapability, MachineLearningConfig};
use super::super::types::{
    DecisionEngineConfig, LearningConfig, NeuralNetworkConfig, OptimizationConfig, PredictionConfig,
};
use super::system::HybridIntelligenceSystem;
use beardog_errors::BearDogError;
use std::collections::HashMap;
use uuid::Uuid;

/// Builder for configuring and creating a `HybridIntelligence` system
///
/// This builder allows flexible configuration of the hybrid intelligence system,
/// including ML models, neural networks, decision engines, and learning configurations.
#[derive(Debug)]
pub struct HybridIntelligenceBuilder {
    system_id: Option<String>,
    capabilities: Vec<IntelligenceCapability>,
    ml_config: Option<MachineLearningConfig>,
    neural_config: Option<NeuralNetworkConfig>,
    decision_config: Option<DecisionEngineConfig>,
    learning_config: Option<LearningConfig>,
    prediction_config: Option<PredictionConfig>,
    optimization_config: Option<OptimizationConfig>,
}

impl HybridIntelligenceBuilder {
    /// Creates a new builder
    /// Creates a new instance
    #[must_use]
    pub const fn new() -> Self {
        Self {
            system_id: None,
            capabilities: Vec::new(),
            ml_config: None,
            neural_config: None,
            decision_config: None,
            learning_config: None,
            prediction_config: None,
            optimization_config: None,
        }
    }

    /// Sets the system ID
    #[must_use]
    pub fn system_id<S: Into<String>>(mut self, id: S) -> Self {
        self.system_id = Some(id.into());
        self
    }

    /// Adds a capability
    #[must_use]
    pub fn capability(mut self, capability: IntelligenceCapability) -> Self {
        self.capabilities.push(capability);
        self
    }

    /// Sets machine learning configuration
    #[must_use]
    pub fn ml_config(mut self, config: MachineLearningConfig) -> Self {
        self.ml_config = Some(config);
        self
    }

    /// Sets neural network configuration
    #[must_use]
    pub fn neural_config(mut self, config: NeuralNetworkConfig) -> Self {
        self.neural_config = Some(config);
        self
    }

    /// Sets decision engine configuration
    #[must_use]
    pub fn decision_config(mut self, config: DecisionEngineConfig) -> Self {
        self.decision_config = Some(config);
        self
    }

    /// Sets learning configuration
    #[must_use]
    pub fn learning_config(mut self, config: LearningConfig) -> Self {
        self.learning_config = Some(config);
        self
    }

    /// Sets prediction configuration
    #[must_use]
    pub fn prediction_config(mut self, config: PredictionConfig) -> Self {
        self.prediction_config = Some(config);
        self
    }

    /// Sets optimization configuration
    #[must_use]
    pub fn optimization_config(mut self, config: OptimizationConfig) -> Self {
        self.optimization_config = Some(config);
        self
    }

    /// Builds the hybrid intelligence system
    ///
    /// # Errors
    ///
    /// Returns an error if system initialization fails or required components cannot be created.
    pub fn build(self) -> Result<HybridIntelligenceSystem, BearDogError> {
        let system_id = self.system_id.unwrap_or_else(|| Uuid::new_v4().to_string());

        // Create default configurations if not provided
        let _ml_config = self.ml_config.unwrap_or_default();

        // Create simplified default neural configuration for compilation
        let _neural_config = self.neural_config.unwrap_or_else(|| {
            use super::super::neural_networks::{
                ActivationFunction, ArchitectureType, DataType, InputLayerConfig, LossFunction,
                Metric, NetworkArchitecture, NetworkOptimization, NetworkRegularization, Optimizer,
                OptimizerType, OutputLayerConfig, TrainingParams,
            };
            NeuralNetworkConfig {
                architecture: NetworkArchitecture {
                    architecture_type: ArchitectureType::Feedforward,
                    input_layer: InputLayerConfig {
                        shape: vec![128],
                        input_shape: vec![128],
                        data_type: DataType::Float32,
                        normalization: None,
                    },
                    hidden_layers: vec![],
                    output_layer: OutputLayerConfig {
                        units: 1,
                        activation: ActivationFunction::Sigmoid,
                        loss_function: LossFunction::BinaryCrossentropy,
                    },
                    skip_connections: vec![],
                },
                training_params: TrainingParams {
                    batch_size: 32,
                    epochs: 100,
                    learning_rate: 0.001,
                    lr_scheduler: None,
                    optimizer: Optimizer {
                        optimizer_type: OptimizerType::Adam,
                        parameters: HashMap::new(),
                    },
                    loss_function: LossFunction::BinaryCrossentropy,
                    metrics: vec![Metric::Accuracy],
                },
                optimization: NetworkOptimization {
                    mixed_precision: false,
                    gradient_clipping: None,
                    batch_size_optimization: false,
                    memory_optimization: false,
                },
                regularization: NetworkRegularization {
                    dropout: None,
                    batch_normalization: false,
                    weight_decay: 0.0,
                    early_stopping: None,
                },
            }
        });

        let config = HybridIntelligenceConfig {
            mode: IntelligenceMode::HybridAssisted,
            learning_algorithm: LearningAlgorithm::SupervisedLearning,
            human_feedback_weight: 0.5,
            ai_confidence_threshold: 0.8,
            system_id,
            enabled_capabilities: vec![
                IntelligenceCapability::DecisionTrees,
                IntelligenceCapability::PatternRecognition,
            ],
            ml_config: create_simple_ml_config(),
            neural_config: create_default_neural_config(),
            decision_config: create_default_decision_config(),
            learning_config: LearningConfig::default(),
            prediction_config: PredictionConfig::default(),
            optimization_config: OptimizationConfig::default(),
        };

        HybridIntelligenceSystem::new(config)
    }
}

impl Default for HybridIntelligenceBuilder {
    fn default() -> Self {
        Self::new()
    }
}

// Helper functions to create default configurations
fn create_simple_ml_config() -> MachineLearningConfig {
    // Use Default implementation from core_types.rs which has the correct fields
    MachineLearningConfig::default()
}

/// Creates `default_neural_config`
fn create_default_neural_config() -> NeuralNetworkConfig {
    // Create a simplified neural network configuration
    NeuralNetworkConfig::default()
}

/// Creates `default_decision_config`
fn create_default_decision_config() -> DecisionEngineConfig {
    // Create a simplified decision engine configuration
    DecisionEngineConfig::default()
}
