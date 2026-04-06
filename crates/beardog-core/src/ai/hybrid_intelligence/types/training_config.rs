// SPDX-License-Identifier: AGPL-3.0-or-later

//! Top-level hybrid intelligence pipeline configuration (training, inference, models, learning).

use std::collections::HashMap;
use std::time::Duration;

use serde::{Deserialize, Serialize};

use crate::ai::hybrid_intelligence::decision_engine::{
    ConsensusStrategy, DecisionCriteria, DecisionStrategy,
};
use crate::ai::hybrid_intelligence::learning::{
    ConstraintConfig, EnsembleConfig, HyperparameterOptimization, LearningAlgorithmType,
    MetaLearningConfig, OnlineLearningConfig, OptimizationAlgorithm, PredictionHorizon,
    PredictionModel, TransferLearningConfig,
};
use crate::ai::hybrid_intelligence::neural_networks::{
    NetworkArchitecture, NetworkOptimization, NetworkRegularization, TrainingParams,
};

use super::deployment::DeploymentConfig;
use super::monitoring::AIMonitoringConfig;
use super::optimizer::{EarlyStoppingConfig, OptimizerConfig, RegularizationConfig};
use super::preprocessing::{
    DataAugmentationConfig, FeatureSelectionConfig, MissingValueStrategy, NormalizationStrategy,
};
use super::registry::{AIRegistryConfig, VersioningStrategy};
use super::serving::{CachingConfig, ServingConfig};

/// Training configuration for hybrid intelligence models
///
/// Specifies hyperparameters and settings for training machine learning models,
/// including batch size, learning rate, regularization, and early stopping criteria.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct TrainingConfig {
    /// Number of samples processed in each training iteration before model weights are updated
    pub batch_size: u32,
    /// Total number of complete passes through the entire training dataset during training
    pub epochs: u32,
    /// Step size at each iteration while moving toward a minimum of the loss function
    pub learning_rate: f64,
    /// Proportion of training data (0.0-1.0) reserved for validation during training to prevent overfitting
    pub validation_split: f64,
    /// Configuration for automatically stopping training when validation metrics stop improving
    pub early_stopping: Option<EarlyStoppingConfig>,
    /// Configuration for techniques to prevent model overfitting (L1/L2, dropout, batch normalization)
    pub regularization: Option<RegularizationConfig>,
    /// Algorithm and parameters used to update model weights based on computed gradients
    pub optimizer: OptimizerConfig,
}

impl Default for TrainingConfig {
    fn default() -> Self {
        Self {
            batch_size: 32,
            epochs: 100,
            learning_rate: 0.001,
            validation_split: 0.2,
            early_stopping: None,
            regularization: None,
            optimizer: OptimizerConfig::default(),
        }
    }
}

impl TrainingConfig {
    /// Load training hyperparameters from `BEARDOG_AI_*` via `std::env::var`.
    #[must_use]
    pub fn from_env() -> Self {
        Self {
            batch_size: std::env::var("BEARDOG_AI_TRAINING_BATCH_SIZE")
                .ok()
                .and_then(|v| v.parse().ok())
                .unwrap_or(32),
            epochs: std::env::var("BEARDOG_AI_TRAINING_EPOCHS")
                .ok()
                .and_then(|v| v.parse().ok())
                .unwrap_or(100),
            learning_rate: std::env::var("BEARDOG_AI_LEARNING_RATE")
                .ok()
                .and_then(|v| v.parse().ok())
                .unwrap_or(0.001),
            validation_split: 0.2,
            early_stopping: None,
            regularization: None,
            optimizer: OptimizerConfig::default(),
        }
    }
}

/// Inference configuration for model predictions in production
///
/// Defines settings for running trained models in production environments,
/// including batch processing, timeout limits, serving parameters, and caching strategies.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct InferenceConfig {
    /// Number of prediction requests processed together in a single batch for efficiency
    pub batch_size: u32,
    /// Maximum time (in milliseconds) allowed for a single inference request before timing out
    pub max_inference_time_ms: u64,
    /// Configuration for model serving infrastructure, load balancing, and request handling
    pub serving_config: ServingConfig,
    /// Optional caching configuration to store and reuse recent predictions for identical inputs
    pub caching: Option<CachingConfig>,
}

impl Default for InferenceConfig {
    fn default() -> Self {
        Self {
            batch_size: 1,
            max_inference_time_ms: 1000,
            serving_config: ServingConfig::default(),
            caching: None,
        }
    }
}

impl InferenceConfig {
    /// Load inference settings from `BEARDOG_AI_*` via `std::env::var`.
    #[must_use]
    pub fn from_env() -> Self {
        Self {
            batch_size: std::env::var("BEARDOG_AI_INFERENCE_BATCH_SIZE")
                .ok()
                .and_then(|v| v.parse().ok())
                .unwrap_or(1),
            max_inference_time_ms: std::env::var("BEARDOG_AI_MAX_INFERENCE_TIME_MS")
                .ok()
                .and_then(|v| v.parse().ok())
                .unwrap_or(1000),
            serving_config: ServingConfig::from_env(),
            caching: None,
        }
    }
}

/// Model management configuration for `MLOps` lifecycle
///
/// Comprehensive configuration for managing AI models throughout their lifecycle,
/// including versioning, registry storage, deployment strategies, and performance monitoring.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ModelManagementConfig {
    /// Strategy for assigning version identifiers to models (semantic, timestamp, hash, or incremental)
    pub versioning_strategy: VersioningStrategy,
    /// Configuration for the model registry where trained models are stored and cataloged
    pub registry_config: AIRegistryConfig,
    /// Configuration for deploying models to production environments (blue-green, canary, rolling, etc.)
    pub deployment_config: DeploymentConfig,
    /// Configuration for monitoring model performance, resource usage, and operational metrics in production
    pub monitoring_config: AIMonitoringConfig,
}

impl Default for ModelManagementConfig {
    fn default() -> Self {
        Self {
            versioning_strategy: VersioningStrategy::Semantic,
            registry_config: AIRegistryConfig::default(),
            deployment_config: DeploymentConfig::default(),
            monitoring_config: AIMonitoringConfig::default(),
        }
    }
}

/// Data preprocessing configuration for input transformation
///
/// Defines transformations applied to raw data before training or inference,
/// including normalization, feature selection, data augmentation, and missing value handling.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PreprocessingConfig {
    /// Strategy for scaling numerical features to a common range (min-max, z-score, robust, etc.)
    pub normalization: NormalizationStrategy,
    /// Optional configuration for selecting the most relevant features from the input dataset
    pub feature_selection: Option<FeatureSelectionConfig>,
    /// Optional configuration for artificially expanding the training dataset with transformed variations
    pub data_augmentation: Option<DataAugmentationConfig>,
    /// Strategy for handling missing values in the input data (drop, fill mean/median/mode, interpolate)
    pub missing_value_handling: MissingValueStrategy,
}

impl Default for PreprocessingConfig {
    fn default() -> Self {
        Self {
            normalization: NormalizationStrategy::ZScore,
            feature_selection: None,
            data_augmentation: None,
            missing_value_handling: MissingValueStrategy::FillMean,
        }
    }
}

/// Neural network configuration for deep learning models
///
/// Complete configuration for constructing, training, and optimizing neural network models,
/// including architecture definition, training parameters, optimization techniques, and regularization.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct NeuralNetworkConfig {
    /// Network structure definition including layers, activations, and connections
    pub architecture: NetworkArchitecture,
    /// Hyperparameters controlling the training process (batch size, epochs, learning rate, optimizer)
    pub training_params: TrainingParams,
    /// Performance optimization settings (mixed precision, gradient clipping, memory optimization)
    pub optimization: NetworkOptimization,
    /// Techniques to prevent overfitting (dropout, batch normalization, weight decay, early stopping)
    pub regularization: NetworkRegularization,
}

impl Default for NeuralNetworkConfig {
    fn default() -> Self {
        // Create a basic neural network configuration with placeholder values
        Self {
            architecture: NetworkArchitecture {
                architecture_type: crate::ai::hybrid_intelligence::neural_networks::ArchitectureType::Feedforward,
                input_layer: crate::ai::hybrid_intelligence::neural_networks::InputLayerConfig {
                    shape: vec![32],
                    input_shape: vec![32],
                    data_type: crate::ai::hybrid_intelligence::neural_networks::DataType::Float32,
                    normalization: None,
                },
                hidden_layers: vec![],
                output_layer: crate::ai::hybrid_intelligence::neural_networks::OutputLayerConfig {
                    units: 10,
                    activation: crate::ai::hybrid_intelligence::neural_networks::ActivationFunction::Softmax,
                    loss_function: crate::ai::hybrid_intelligence::neural_networks::LossFunction::MeanSquaredError,
                },
                skip_connections: vec![],
            },
            training_params: TrainingParams {
                batch_size: 32,
                epochs: 100,
                learning_rate: 0.001,
                lr_scheduler: None,
                optimizer: crate::ai::hybrid_intelligence::neural_networks::Optimizer {
                    optimizer_type: crate::ai::hybrid_intelligence::neural_networks::OptimizerType::Adam,
                    parameters: HashMap::new(),
                },
                loss_function: crate::ai::hybrid_intelligence::neural_networks::LossFunction::MeanSquaredError,
                metrics: vec![],
            },
            optimization: NetworkOptimization {
                mixed_precision: false,
                gradient_clipping: None,
                batch_size_optimization: false,
                memory_optimization: true,
            },
            regularization: NetworkRegularization {
                dropout: Some(crate::ai::hybrid_intelligence::neural_networks::DropoutConfig {
                    rate: 0.2,
                    training_only: true,
                    schedule: None,
                }),
                batch_normalization: true,
                weight_decay: 0.001,
                early_stopping: Some(crate::ai::hybrid_intelligence::neural_networks::EarlyStoppingConfig {
                    monitor: "val_loss".to_string(),
                    min_delta: 0.001,
                    patience: 10,
                    restore_best_weights: true,
                    mode: crate::ai::hybrid_intelligence::neural_networks::MonitoringMode::Min,
                }),
            },
        }
    }
}

/// Decision engine configuration for hybrid AI decision-making
///
/// Configures the decision engine that combines multiple AI strategies and human input
/// to make intelligent, consensus-based decisions within specified time constraints.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct DecisionEngineConfig {
    /// Collection of decision-making strategies to employ (rule-based, ML, heuristic, human-in-loop)
    pub strategies: Vec<DecisionStrategy>,
    /// Criteria and weights used to evaluate and compare decision options
    pub criteria: DecisionCriteria,
    /// Mechanism for reaching consensus when multiple strategies produce different recommendations
    pub consensus_mechanism: ConsensusStrategy,
    /// Maximum time allowed for the decision-making process before returning the best available decision
    pub timeout: Duration,
}

impl Default for DecisionEngineConfig {
    fn default() -> Self {
        Self {
            strategies: vec![DecisionStrategy::MachineLearning],
            criteria: DecisionCriteria {
                primary_criteria: vec![],
                secondary_criteria: vec![],
                weights: HashMap::new(),
                thresholds: HashMap::new(),
            },
            consensus_mechanism: ConsensusStrategy::Majority,
            timeout: Duration::from_millis(1000),
        }
    }
}

/// Learning configuration for adaptive AI systems
///
/// Configures how the hybrid intelligence system learns and adapts over time,
/// including online learning, transfer learning from other domains, and meta-learning strategies.
#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct LearningConfig {
    /// Collection of learning algorithm types to employ (supervised, unsupervised, reinforcement, etc.)
    pub algorithms: Vec<LearningAlgorithmType>,
    /// Configuration for continuous learning from new data as it arrives in production
    pub online_learning: OnlineLearningConfig,
    /// Optional configuration for transferring knowledge from pre-trained models in related domains
    pub transfer_learning: Option<TransferLearningConfig>,
    /// Optional configuration for learning how to learn, adapting learning strategies themselves
    pub meta_learning: Option<MetaLearningConfig>,
}

// Types moved to avoid duplication - using existing definitions

/// Prediction configuration for forecasting and inference
///
/// Configures how the system generates predictions, including which models to use,
/// ensemble methods, uncertainty estimation, and prediction time horizons.
#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct PredictionConfig {
    /// Collection of prediction model types to employ for generating forecasts
    pub models: Vec<PredictionModel>,
    /// Optional configuration for combining multiple models into an ensemble for improved accuracy
    pub ensemble: Option<EnsembleConfig>,
    /// Whether to calculate and report confidence intervals and uncertainty estimates for predictions
    pub uncertainty_quantification: bool,
    /// Time horizons for which to generate predictions (short-term, medium-term, long-term)
    pub horizons: Vec<PredictionHorizon>,
}

/// Optimization configuration for model and hyperparameter tuning
///
/// Configures optimization algorithms for finding optimal model parameters, hyperparameters,
/// and solutions to complex problems with multiple objectives and constraints.
#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct OptimizationConfig {
    /// Collection of optimization algorithms to employ (gradient descent, genetic algorithms, Bayesian optimization, etc.)
    pub algorithms: Vec<OptimizationAlgorithm>,
    /// Whether to optimize for multiple competing objectives simultaneously (e.g., accuracy vs. latency)
    pub multi_objective: bool,
    /// Optional constraints on the optimization search space (bounds, equality/inequality constraints)
    pub constraints: Option<ConstraintConfig>,
    /// Optional configuration for automated hyperparameter tuning (grid search, random search, Bayesian)
    pub hyperparameter_optimization: Option<HyperparameterOptimization>,
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::ai::hybrid_intelligence::decision_engine::DecisionStrategy;

    #[test]
    fn training_config_default() {
        let c = TrainingConfig::default();
        assert_eq!(c.batch_size, 32);
        assert_eq!(c.epochs, 100);
        assert_eq!(c.learning_rate, 0.001);
        assert_eq!(c.validation_split, 0.2);
        assert!(c.early_stopping.is_none());
        assert!(c.regularization.is_none());
    }

    #[test]
    fn training_config_from_env_defaults() {
        let c = TrainingConfig::from_env();
        assert_eq!(c.batch_size, 32);
        assert_eq!(c.epochs, 100);
        assert_eq!(c.learning_rate, 0.001);
        assert_eq!(c.validation_split, 0.2);
    }

    #[test]
    fn inference_config_default_and_from_env() {
        let d = InferenceConfig::default();
        assert_eq!(d.batch_size, 1);
        assert_eq!(d.max_inference_time_ms, 1000);
        assert!(d.caching.is_none());
        let e = InferenceConfig::from_env();
        assert_eq!(e.batch_size, 1);
        assert_eq!(e.max_inference_time_ms, 1000);
        assert!(e.caching.is_none());
    }

    #[test]
    fn neural_network_config_default() {
        let c = NeuralNetworkConfig::default();
        assert_eq!(c.training_params.batch_size, 32);
        assert_eq!(c.training_params.epochs, 100);
        assert!(!c.optimization.mixed_precision);
    }

    #[test]
    fn decision_engine_config_default() {
        let c = DecisionEngineConfig::default();
        assert_eq!(c.strategies, vec![DecisionStrategy::MachineLearning]);
        assert_eq!(c.timeout, std::time::Duration::from_millis(1000));
    }

    #[test]
    fn model_management_config_default() {
        let c = ModelManagementConfig::default();
        assert!(matches!(
            c.versioning_strategy,
            VersioningStrategy::Semantic
        ));
    }

    #[test]
    fn preprocessing_config_default() {
        let c = PreprocessingConfig::default();
        assert_eq!(c.normalization, NormalizationStrategy::ZScore);
        assert_eq!(c.missing_value_handling, MissingValueStrategy::FillMean);
    }

    #[test]
    fn learning_prediction_optimization_serde_roundtrip() {
        let lc = LearningConfig::default();
        let v = serde_json::to_value(&lc).unwrap();
        let back: LearningConfig = serde_json::from_value(v).unwrap();
        assert_eq!(
            serde_json::to_value(&lc).unwrap(),
            serde_json::to_value(&back).unwrap()
        );
        let pc = PredictionConfig::default();
        let v = serde_json::to_value(&pc).unwrap();
        let back: PredictionConfig = serde_json::from_value(v).unwrap();
        assert_eq!(
            serde_json::to_value(&pc).unwrap(),
            serde_json::to_value(&back).unwrap()
        );
        let oc = OptimizationConfig::default();
        let v = serde_json::to_value(&oc).unwrap();
        let back: OptimizationConfig = serde_json::from_value(v).unwrap();
        assert_eq!(
            serde_json::to_value(&oc).unwrap(),
            serde_json::to_value(&back).unwrap()
        );
    }
}
