// SPDX-License-Identifier: AGPL-3.0-or-later

//! Learning Algorithms Configuration
//!
//! This module contains configuration types for various learning algorithms
//! including online learning, transfer learning, meta-learning, and ensembles.

use serde::{Deserialize, Serialize};

/// Online learning configuration
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
pub struct OnlineLearningConfig {
    /// Enable online learning
    pub enabled: bool,
    /// Learning rate adaptation
    pub adaptation_type: LearningRateAdaptationType,
    /// Update frequency
    pub update_frequency: UpdateFrequency,
    /// Mini-batch size
    pub mini_batch_size: usize,
}

impl Default for OnlineLearningConfig {
    fn default() -> Self {
        Self {
            enabled: false,
            adaptation_type: LearningRateAdaptationType::Constant,
            update_frequency: UpdateFrequency::PerBatch,
            mini_batch_size: std::env::var("BEARDOG_AI_MINI_BATCH_SIZE")
                .ok()
                .and_then(|s| s.parse().ok())
                .unwrap_or(32),
        }
    }
}

/// Learning rate adaptation types
#[derive(Debug, Clone, Copy, Serialize, Deserialize, PartialEq, Eq)]
pub enum LearningRateAdaptationType {
    /// Constant learning rate
    Constant,
    /// Adaptive learning rate
    Adaptive,
    /// Scheduled learning rate
    Scheduled,
}

/// Update frequency for online learning
#[derive(Debug, Clone, Copy, Serialize, Deserialize, PartialEq, Eq)]
pub enum UpdateFrequency {
    /// Update per batch
    PerBatch,
    /// Update per epoch
    PerEpoch,
    /// Update per sample
    PerSample,
}

/// Transfer learning configuration
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Default)]
pub struct TransferLearningConfig {
    /// Enable transfer learning
    pub enabled: bool,
    /// Source model path
    pub source_model: String,
    /// Layers to freeze
    pub frozen_layers: Vec<String>,
    /// Fine-tuning configuration
    pub fine_tuning: FineTuningConfig,
}

/// Fine-tuning configuration
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct FineTuningConfig {
    /// Initial learning rate
    pub initial_learning_rate: f64,
    /// Number of fine-tuning epochs
    pub epochs: u32,
    /// Gradual unfreezing
    pub gradual_unfreeze: bool,
}

impl Default for FineTuningConfig {
    fn default() -> Self {
        Self {
            initial_learning_rate: std::env::var("BEARDOG_AI_FINETUNING_INITIAL_LR")
                .ok()
                .and_then(|s| s.parse().ok())
                .unwrap_or(0.0001),
            epochs: std::env::var("BEARDOG_AI_FINETUNING_EPOCHS")
                .ok()
                .and_then(|s| s.parse().ok())
                .unwrap_or(10),
            gradual_unfreeze: false,
        }
    }
}

/// Meta-learning configuration
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct MetaLearningConfig {
    /// Enable meta-learning
    pub enabled: bool,
    /// Meta-learning algorithm
    pub algorithm: MetaLearningAlgorithm,
    /// Inner loop configuration
    pub inner_loop: InnerLoopConfig,
    /// Outer loop configuration
    pub outer_loop: OuterLoopConfig,
}

impl Default for MetaLearningConfig {
    fn default() -> Self {
        Self {
            enabled: false,
            algorithm: MetaLearningAlgorithm::Maml,
            inner_loop: InnerLoopConfig::default(),
            outer_loop: OuterLoopConfig::default(),
        }
    }
}

/// Meta-learning algorithms
#[derive(Debug, Clone, Copy, Serialize, Deserialize, PartialEq, Eq)]
pub enum MetaLearningAlgorithm {
    /// Model-Agnostic Meta-Learning
    Maml,
    /// Reptile algorithm
    Reptile,
    /// Prototypical networks
    PrototypicalNetworks,
}

/// Inner loop configuration for meta-learning
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct InnerLoopConfig {
    /// Number of inner steps
    pub steps: u32,
    /// Inner learning rate
    pub learning_rate: f64,
}

impl Default for InnerLoopConfig {
    fn default() -> Self {
        Self {
            steps: std::env::var("BEARDOG_AI_INNER_LOOP_STEPS")
                .ok()
                .and_then(|s| s.parse().ok())
                .unwrap_or(5),
            learning_rate: std::env::var("BEARDOG_AI_INNER_LOOP_LR")
                .ok()
                .and_then(|s| s.parse().ok())
                .unwrap_or(0.01),
        }
    }
}

/// Outer loop configuration for meta-learning
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct OuterLoopConfig {
    /// Number of outer steps
    pub steps: u32,
    /// Outer learning rate
    pub learning_rate: f64,
}

impl Default for OuterLoopConfig {
    fn default() -> Self {
        Self {
            steps: std::env::var("BEARDOG_AI_OUTER_LOOP_STEPS")
                .ok()
                .and_then(|s| s.parse().ok())
                .unwrap_or(100),
            learning_rate: std::env::var("BEARDOG_AI_OUTER_LOOP_LR")
                .ok()
                .and_then(|s| s.parse().ok())
                .unwrap_or(0.001),
        }
    }
}

/// Ensemble learning configuration
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
pub struct EnsembleConfigLearning {
    /// Enable ensemble learning
    pub enabled: bool,
    /// Base models
    pub base_models: Vec<String>,
    /// Ensemble method
    pub method: EnsembleMethodType,
}

impl Default for EnsembleConfigLearning {
    fn default() -> Self {
        Self {
            enabled: false,
            base_models: vec![],
            method: EnsembleMethodType::Voting,
        }
    }
}

/// Ensemble methods
#[derive(Debug, Clone, Copy, Serialize, Deserialize, PartialEq, Eq)]
pub enum EnsembleMethodType {
    /// Voting ensemble
    Voting,
    /// Bagging ensemble
    Bagging,
    /// Boosting ensemble
    Boosting,
    /// Stacking ensemble
    Stacking,
}

/// Hyperparameter optimization configuration
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
pub struct HyperparameterOptimizationConfig {
    /// Enable hyperparameter optimization
    pub enabled: bool,
    /// Optimization method
    pub method: HyperparameterOptimizationMethod,
    /// Maximum trials
    pub max_trials: u32,
}

impl Default for HyperparameterOptimizationConfig {
    fn default() -> Self {
        Self {
            enabled: false,
            method: HyperparameterOptimizationMethod::GridSearch,
            max_trials: std::env::var("BEARDOG_AI_HYPERPARAMETER_MAX_TRIALS")
                .ok()
                .and_then(|s| s.parse().ok())
                .unwrap_or(100),
        }
    }
}

/// Hyperparameter optimization methods
#[derive(Debug, Clone, Copy, Serialize, Deserialize, PartialEq, Eq)]
pub enum HyperparameterOptimizationMethod {
    /// Grid search
    GridSearch,
    /// Random search
    RandomSearch,
    /// Bayesian optimization
    BayesianOptimization,
}
