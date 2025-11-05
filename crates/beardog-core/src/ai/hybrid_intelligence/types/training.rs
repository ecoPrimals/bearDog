//! Training configuration types for the hybrid intelligence system
//!
//! This module contains all training-related configuration types including
//! training parameters, early stopping, regularization, and optimizer configurations.

use serde::{Deserialize, Serialize};
use std::collections::HashMap;

/// Training configuration
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct TrainingConfig {
    /// Number of batch_size
    pub batch_size: u32,
    /// Number of training epochs
    /// Number of epochs
    pub epochs: u32,
    /// Learning rate
    /// The learning rate value
    pub learning_rate: f64,
    /// Validation split ratio
    pub validation_split: f64,
    /// Early stopping configuration
    /// Optional early stopping
    pub early_stopping: Option<EarlyStoppingConfig>,
    /// Regularization configuration
    /// Optional regularization
    pub regularization: Option<RegularizationConfig>,
    /// Optimizer configuration
    /// The optimizer value
    pub optimizer: OptimizerConfig,
}

impl Default for TrainingConfig {
    fn default() -> Self {
        Self {
            batch_size: 32,
            epochs: 100,
            learning_rate: 0.001,
            validation_split: 0.2,
            early_stopping: Some(EarlyStoppingConfig::default()),
            regularization: Some(RegularizationConfig::default()),
            optimizer: OptimizerConfig::default(),
        }
    }
}

/// Early stopping configuration
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct EarlyStoppingConfig {
    /// Enable early stopping
    pub enabled: bool,
    /// Patience (number of epochs to wait)
    pub patience: u32,
    /// Minimum improvement threshold
    pub min_delta: f64,
    /// Metric to monitor
    pub monitor: String,
}

impl Default for EarlyStoppingConfig {
    fn default() -> Self {
        Self {
            enabled: true,
            patience: 10,
            min_delta: 0.001,
            monitor: "val_loss".to_string(),
        }
    }
}

/// Regularization configuration
#[derive(Debug, Clone, Copy, Serialize, Deserialize)]
pub struct RegularizationConfig {
    /// L1 regularization strength
    pub l1: f64,
    /// L2 regularization strength
    pub l2: f64,
    /// Dropout rate
    pub dropout: f64,
}

impl Default for RegularizationConfig {
    fn default() -> Self {
        Self {
            l1: 0.0,
            l2: 0.001,
            dropout: 0.1,
        }
    }
}

/// Optimizer configuration
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct OptimizerConfig {
    /// Optimizer type
    pub optimizer_type: OptimizerType,
    /// Learning rate
    pub learning_rate: f64,
    /// Optimizer-specific parameters
    pub params: HashMap<String, f64>,
}

impl Default for OptimizerConfig {
    fn default() -> Self {
        let mut params = HashMap::new();
        params.insert("beta1".to_string(), 0.9);
        params.insert("beta2".to_string(), 0.999);
        params.insert("epsilon".to_string(), 1e-8);

        Self {
            optimizer_type: OptimizerType::Adam,
            learning_rate: 0.001,
            params,
        }
    }
}

/// Available optimizer types
#[derive(Debug, Clone, Copy, Serialize, Deserialize)]
pub enum OptimizerType {
    /// Stochastic Gradient Descent
    SGD,
    /// Adam optimizer
    Adam,
    /// AdamW optimizer
    AdamW,
    /// RMSprop optimizer
    RMSprop,
    /// Adagrad optimizer
    Adagrad,
    /// Adadelta optimizer
    Adadelta,
    /// Adamax optimizer
    Adamax,
    /// Nadam optimizer
    Nadam,
    /// LBFGS optimizer
    LBFGS,
    /// Rprop optimizer
    Rprop,
}

impl Default for OptimizerType {
    fn default() -> Self {
        Self::Adam
    }
}

impl std::fmt::Display for OptimizerType {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::SGD => write!(f, "SGD"),
            Self::Adam => write!(f, "Adam"),
            Self::AdamW => write!(f, "AdamW"),
            Self::RMSprop => write!(f, "RMSprop"),
            Self::Adagrad => write!(f, "Adagrad"),
            Self::Adadelta => write!(f, "Adadelta"),
            Self::Adamax => write!(f, "Adamax"),
            Self::Nadam => write!(f, "Nadam"),
            Self::LBFGS => write!(f, "LBFGS"),
            Self::Rprop => write!(f, "Rprop"),
        }
    }
}

/// Optimization configuration for training
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct OptimizationConfig {
    /// Enable optimization
    pub enabled: bool,
    /// Optimization algorithm
    pub algorithm: OptimizationAlgorithm,
    /// Maximum number of optimization iterations
    pub max_iterations: u32,
    /// Convergence tolerance
    pub tolerance: f64,
    /// Learning rate schedule
    pub lr_schedule: LearningRateSchedule,
}

impl Default for OptimizationConfig {
    fn default() -> Self {
        Self {
            enabled: true,
            algorithm: OptimizationAlgorithm::Adam,
            max_iterations: std::env::var("BEARDOG_AI_MAX_ITERATIONS")
                .ok()
                .and_then(|i| i.parse().ok())
                .unwrap_or(1000), // 1000 iterations default
            tolerance: 1e-6,
            lr_schedule: LearningRateSchedule::Constant,
        }
    }
}

/// Available optimization algorithms
#[derive(Debug, Clone, Copy, Serialize, Deserialize)]
pub enum OptimizationAlgorithm {
    /// Adam optimizer
    Adam,
    /// SGD with momentum
    SGD,
    /// RMSprop
    RMSprop,
    /// L-BFGS
    LBFGS,
}

impl Default for OptimizationAlgorithm {
    fn default() -> Self {
        Self::Adam
    }
}

/// Learning rate schedule types
#[derive(Debug, Clone, Copy, Serialize, Deserialize)]
pub enum LearningRateSchedule {
    /// Constant learning rate
    Constant,
    /// Exponential decay
    ExponentialDecay,
    /// Step decay
    StepDecay,
    /// Cosine annealing
    CosineAnnealing,
    /// Polynomial decay
    PolynomialDecay,
}

impl Default for LearningRateSchedule {
    fn default() -> Self {
        Self::Constant
    }
} 