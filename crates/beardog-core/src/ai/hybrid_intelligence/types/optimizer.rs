// SPDX-License-Identifier: AGPL-3.0-only

//! Optimizer, learning-rate schedule, and related training hyperparameter types.

use serde::{Deserialize, Serialize};

/// Early stopping configuration to prevent overfitting
///
/// Automatically halts training when a monitored metric stops improving,
/// preventing overfitting and saving computational resources.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct EarlyStoppingConfig {
    /// Name of the metric to monitor for improvement (e.g., "`val_loss`", "`val_accuracy`")
    pub monitor: String,
    /// Minimum change in monitored metric required to qualify as an improvement
    pub min_delta: f64,
    /// Number of epochs to wait for improvement before stopping training
    pub patience: u32,
    /// Whether to restore model weights from the epoch with the best monitored metric value
    pub restore_best_weights: bool,
}

/// Regularization configuration to prevent overfitting
///
/// Configures various regularization techniques that constrain model complexity
/// and improve generalization to unseen data.
#[derive(Debug, Clone, Copy, Serialize, Deserialize)]
pub struct RegularizationConfig {
    /// L1 regularization strength (lasso), promotes sparsity by driving some weights to zero
    pub l1: f64,
    /// L2 regularization strength (ridge), penalizes large weights to prevent overfitting
    pub l2: f64,
    /// Dropout rate (0.0-1.0), probability of randomly dropping neurons during training
    pub dropout: f64,
    /// Whether to apply batch normalization to stabilize and accelerate training
    pub batch_normalization: bool,
}

/// Optimizer configuration for gradient-based training
///
/// Configures the optimization algorithm used to update model weights,
/// including learning rate, scheduling, and weight decay parameters.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct OptimizerConfig {
    /// Type of optimization algorithm to use (SGD, Adam, `AdamW`, `RMSprop`, Adagrad)
    pub optimizer_type: OptimizerType,
    /// Initial step size for weight updates during training
    pub learning_rate: f64,
    /// Optional schedule for adjusting learning rate over time (exponential decay, step decay, cosine annealing)
    pub learning_rate_schedule: Option<LearningRateSchedule>,
    /// L2 regularization penalty applied to weights during optimization
    pub weight_decay: f64,
}

impl Default for OptimizerConfig {
    fn default() -> Self {
        Self {
            optimizer_type: OptimizerType::Adam {
                beta1: 0.9,
                beta2: 0.999,
                epsilon: 1e-8,
            },
            learning_rate: 0.001,
            learning_rate_schedule: None,
            weight_decay: 0.0001,
        }
    }
}

/// Optimizer types with algorithm-specific hyperparameters
///
/// Different optimization algorithms suitable for various training scenarios,
/// each with their own convergence properties and parameter requirements.
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub enum OptimizerType {
    /// Stochastic Gradient Descent with optional momentum
    Sgd {
        /// Momentum coefficient (0.0-1.0) to accelerate convergence and dampen oscillations
        momentum: f64,
    },
    /// Adaptive Moment Estimation optimizer with adaptive learning rates per parameter
    Adam {
        /// Exponential decay rate for first moment estimates (typically 0.9)
        beta1: f64,
        /// Exponential decay rate for second moment estimates (typically 0.999)
        beta2: f64,
        /// Small constant to prevent division by zero (typically 1e-8)
        epsilon: f64,
    },
    /// Adam with decoupled weight decay regularization (improved generalization)
    AdamW {
        /// Exponential decay rate for first moment estimates (typically 0.9)
        beta1: f64,
        /// Exponential decay rate for second moment estimates (typically 0.999)
        beta2: f64,
        /// Small constant to prevent division by zero (typically 1e-8)
        epsilon: f64,
    },
    /// `RMSprop` optimizer using moving average of squared gradients for adaptive learning rates
    RmsProp {
        /// Decay rate for moving average of squared gradients (typically 0.9)
        alpha: f64,
        /// Small constant to prevent division by zero (typically 1e-8)
        epsilon: f64,
    },
    /// Adaptive Gradient optimizer with individual learning rates based on historical gradients
    Adagrad {
        /// Small constant to prevent division by zero (typically 1e-8)
        epsilon: f64,
    },
}

/// Learning rate schedules for dynamic adjustment during training
///
/// Different strategies for modifying the learning rate over time to improve
/// convergence and final model performance.
#[derive(Debug, Clone, Copy, Serialize, Deserialize)]
pub enum LearningRateSchedule {
    /// Exponentially decay learning rate over time
    ExponentialDecay {
        /// Multiplicative factor by which learning rate decays each period (0.0-1.0)
        decay_rate: f64,
        /// Number of training steps between each decay application
        decay_steps: u32,
    },
    /// Reduce learning rate by a fixed factor at regular intervals
    StepDecay {
        /// Multiplicative factor applied to learning rate at each drop (0.0-1.0)
        drop_rate: f64,
        /// Number of epochs between successive learning rate reductions
        epochs_drop: u32,
    },
    /// Gradually reduce learning rate using a cosine function
    CosineAnnealing {
        /// Maximum number of iterations before restart (if using restart variant)
        t_max: u32,
        /// Minimum learning rate floor after annealing
        eta_min: f64,
    },
    /// Adaptively reduce learning rate when a monitored metric stops improving
    ReduceOnPlateau {
        /// Multiplicative factor for learning rate reduction (0.0-1.0)
        factor: f64,
        /// Number of epochs with no improvement before reducing learning rate
        patience: u32,
    },
}
