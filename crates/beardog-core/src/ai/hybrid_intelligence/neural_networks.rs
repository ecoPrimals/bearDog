// SPDX-License-Identifier: AGPL-3.0-only

// Neural network architectures and configurations
//
// ⚠️ DEPRECATED: This module is being migrated to the canonical location.
//
// **New Location**: `beardog_types::canonical::config::domains::ai_config`
//
// These types will be removed in v3.3.0 (Q1 2026). Please update your imports to:
// ```rust
// use beardog_types::canonical::config::domains::ai_config::{
//     DetailedNetworkArchitecture, LayerConfig, InputLayerConfig, ...
// };
// ```
//
// Migration Status: Phase 1 Complete (October 2025)
// - All neural network config types now available in canonical location
// - Detailed architecture types fully migrated
// - Training params and optimizer configs unified

use serde::{Deserialize, Serialize};
use std::collections::HashMap;

// Note: These types are temporary local definitions pending canonical ai_config modularization.
// Once ai_config is properly split and refactored, these will be imported from canonical types.
// This enables compilation during the ongoing canonical structure modernization.

/// Activation function types
///
/// Supported activation functions for neural network layers.
#[derive(Debug, Clone, Copy, Serialize, Deserialize)]
pub enum ActivationFunction {
    /// Sigmoid activation (0 to 1 range)
    Sigmoid,
    /// Hyperbolic tangent (-1 to 1 range)
    Tanh,
    /// Rectified Linear Unit (`ReLU`)
    Relu,
    /// Leaky `ReLU` with small negative slope
    LeakyRelu,
    /// Exponential Linear Unit
    Elu,
    /// Scaled Exponential Linear Unit
    Selu,
    /// Softmax for multi-class classification
    Softmax,
    /// Linear activation (no transformation)
    Linear,
}

/// Data type for neural network computations
///
/// Supported numerical precisions for neural network operations.
#[derive(Debug, Clone, Copy, Serialize, Deserialize)]
pub enum DataType {
    /// 16-bit floating point (half precision)
    Float16,
    /// 32-bit floating point (single precision)
    Float32,
    /// 64-bit floating point (double precision)
    Float64,
    /// 8-bit signed integer
    Int8,
    /// 16-bit signed integer
    Int16,
    /// 32-bit signed integer
    Int32,
}

/// Neural network architecture types
///
/// Defines the overall structure and dataflow pattern of the network.
#[derive(Debug, Clone, Copy, Serialize, Deserialize)]
pub enum ArchitectureType {
    /// Traditional feedforward network
    Feedforward,
    /// Convolutional network for spatial data
    Convolutional,
    /// Recurrent network for sequential data
    Recurrent,
    /// Transformer architecture with attention mechanisms
    Transformer,
    /// Hybrid architecture combining multiple types
    Hybrid,
}

/// Loss function types
///
/// Supported loss/objective functions for neural network training.
#[derive(Debug, Clone, Copy, Serialize, Deserialize)]
pub enum LossFunction {
    /// Mean squared error (regression)
    MeanSquaredError,
    /// Mean absolute error (robust regression)
    MeanAbsoluteError,
    /// General cross-entropy loss
    CrossEntropy,
    /// Binary cross-entropy (binary classification)
    BinaryCrossEntropy,
    /// Alternative spelling for binary cross-entropy
    BinaryCrossentropy,
    /// Categorical cross-entropy (multi-class classification)
    CategoricalCrossEntropy,
    /// Huber loss (robust to outliers)
    Huber,
}

/// Input layer configuration
///
/// Defines the shape, data type, and preprocessing for network input.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct InputLayerConfig {
    /// Input tensor shape
    pub shape: Vec<usize>,
    /// Alias for shape (compatibility)
    pub input_shape: Vec<usize>,
    /// Data type for input values
    pub data_type: DataType,
    /// Optional normalization strategy
    pub normalization: Option<String>,
}

/// Output layer configuration
///
/// Defines the final layer structure, activation, and loss function.
#[derive(Debug, Clone, Copy, Serialize, Deserialize)]
pub struct OutputLayerConfig {
    /// Number of output units
    pub units: usize,
    /// Activation function for output
    pub activation: ActivationFunction,
    /// Loss function for training
    pub loss_function: LossFunction,
}

/// Network architecture configuration
///
/// Complete specification of a neural network's structure from input to output.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct NetworkArchitecture {
    /// Type of neural network architecture
    pub architecture_type: ArchitectureType,
    /// Input layer configuration
    pub input_layer: InputLayerConfig,
    /// Configuration for hidden layers
    pub hidden_layers: Vec<LayerParameters>,
    /// Output layer configuration
    pub output_layer: OutputLayerConfig,
    /// Skip/residual connections between layers
    pub skip_connections: Vec<String>,
}

/// Layer configuration combining type and parameters
///
/// Complete specification for a single layer in the network.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct LayerConfig {
    /// Type of layer (Dense, Conv, etc.)
    pub layer_type: LayerType,
    /// Layer-specific parameters
    pub parameters: LayerParameters,
}

#[derive(Debug, Clone, Copy, Serialize, Deserialize)]
/// Types of layer
pub enum LayerType {
    /// Dense/fully connected layer
    Dense,
    /// Convolutional layer
    Conv2d,
    /// 1D Convolutional layer
    Conv1d,
    /// 3D Convolutional layer
    Conv3d,
    /// Max pooling layer
    MaxPool2d,
    /// Average pooling layer
    AvgPool2d,
    /// Global average pooling
    GlobalAvgPool,
    /// Dropout layer
    Dropout,
    /// LSTM layer
    Lstm,
    /// GRU layer
    Gru,
    /// Attention layer
    Attention,
    /// Multi-head attention
    MultiHeadAttention,
    /// Embedding layer
    Embedding,
    /// Batch normalization layer
    BatchNorm,
    /// Layer normalization
    LayerNorm,
}

/// Layer parameters
///
/// Union of all possible layer-specific parameter configurations.
/// Only one field should be populated based on the layer type.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct LayerParameters {
    /// Dense layer parameters (for fully connected layers)
    pub dense: Option<DenseLayerConfig>,
    /// Convolutional layer parameters
    pub conv: Option<ConvLayerConfig>,
    /// Pooling layer parameters
    pub pooling: Option<PoolingLayerConfig>,
    /// RNN layer parameters (LSTM, GRU)
    pub rnn: Option<RnnLayerConfig>,
    /// Attention layer parameters
    pub attention: Option<AttentionLayerConfig>,
    /// Embedding layer parameters
    pub embedding: Option<EmbeddingLayerConfig>,
}

/// Dense (fully connected) layer configuration
///
/// Configuration for a traditional fully connected neural network layer.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct DenseLayerConfig {
    /// Number of neurons/units in this layer
    pub units: u32,
    /// Whether to include bias terms
    pub use_bias: bool,
    /// Weight initialization strategy
    pub weight_init: WeightInitialization,
    /// Bias initialization strategy
    pub bias_init: WeightInitialization,
}

/// Convolutional layer configuration
///
/// Configuration for convolutional layers used in CNNs for spatial data processing.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ConvLayerConfig {
    /// Number of convolutional filters (output channels)
    pub filters: u32,
    /// Size of the convolution kernel
    pub kernel_size: Vec<u32>,
    /// Stride length for convolution
    pub strides: Vec<u32>,
    /// Padding strategy (same, valid, etc.)
    pub padding: PaddingType,
    /// Dilation rate for dilated/atrous convolution
    pub dilation_rate: Vec<u32>,
    /// Whether to include bias terms
    pub use_bias: bool,
}

/// Pooling layer configuration
///
/// Configuration for pooling layers that downsample spatial dimensions.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PoolingLayerConfig {
    /// Size of pooling window
    pub pool_size: Vec<u32>,
    /// Stride length for pooling operation
    pub strides: Vec<u32>,
    /// Padding strategy
    pub padding: PaddingType,
}

/// RNN layer configuration
///
/// Configuration for recurrent neural network layers (LSTM, GRU, etc.).
#[derive(Debug, Clone, Copy, Serialize, Deserialize)]
pub struct RnnLayerConfig {
    /// Number of RNN units/cells
    pub units: u32,
    /// Type of RNN cell (LSTM, GRU, `SimpleRNN`)
    pub cell_type: RnnCellType,
    /// Whether to return full sequence or just final output
    pub return_sequences: bool,
    /// Dropout rate for regularization (0.0 to 1.0)
    pub dropout_rate: f64,
}

/// Attention layer configuration
///
/// Configuration for attention mechanisms used in transformers and attention networks.
#[derive(Debug, Clone, Copy, Serialize, Deserialize)]
pub struct AttentionLayerConfig {
    /// Number of attention heads for multi-head attention
    pub num_heads: u32,
    /// Dimension of key vectors
    pub key_dim: u32,
    /// Dimension of value vectors (defaults to `key_dim` if not specified)
    pub value_dim: Option<u32>,
    /// Dropout rate for attention weights
    /// The dropout rate value
    pub dropout_rate: f64,
}

/// Configuration for embedding layers in neural networks
///
/// Embedding layers map discrete input tokens (like words) to dense vector
/// representations, enabling neural networks to process categorical data.
#[derive(Debug, Clone, Copy, Serialize, Deserialize)]
pub struct EmbeddingLayerConfig {
    /// Input dimension (vocabulary size or number of unique tokens)
    pub input_dim: u32,
    /// Output dimension (size of the embedding vectors)
    pub output_dim: u32,
    /// Whether to mask zero values in the input
    pub mask_zero: bool,
    /// Optional fixed length for input sequences
    pub input_length: Option<u32>,
}

// ActivationFunction now imported from canonical location

#[derive(Debug, Clone, Copy, Serialize, Deserialize)]
/// Types of padding
pub enum PaddingType {
    /// Valid padding (no padding)
    Valid,
    /// Same padding
    Same,
    /// Custom padding
    Custom(u32),
}

#[derive(Debug, Clone, Copy, Serialize, Deserialize)]
/// Types of rnn cell
pub enum RnnCellType {
    /// Simple RNN
    SimpleRnn,
    /// Long Short-Term Memory
    Lstm,
    /// Gated Recurrent Unit
    Gru,
}

/// Weight initialization strategies
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub enum WeightInitialization {
    /// Zeros initialization
    Zeros,
    /// Ones initialization
    Ones,
    /// Random normal initialization
    /// Random normal initialization
    RandomNormal {
        /// Mean value
        mean: f64,
        /// Standard deviation
        stddev: f64,
    },
    /// Random uniform initialization
    RandomUniform {
        /// Minimum value of the uniform distribution
        minval: f64,
        /// Maximum value of the uniform distribution
        maxval: f64,
    },
    /// Glorot/Xavier uniform initialization (recommended for tanh/sigmoid)
    GlorotUniform,
    /// Xavier/Glorot normal initialization
    GlorotNormal,
    /// He uniform initialization (recommended for `ReLU` networks)
    HeUniform,
    /// He normal initialization
    HeNormal,
    /// Human entropy-driven initialization (Sovereign Security Enhancement)
    HumanEntropyInitialization {
        /// Entropy tier requirement (1=Machine, 2=HumanSupervised, 3=HumanLived)
        required_entropy_tier: u8,
        /// Human identity providing the entropy source
        human_identity_id: String,
        /// Initialization distribution type
        distribution: EntropyDistribution,
        /// Whether to fall back to machine entropy if human entropy unavailable
        fallback_to_machine: bool,
    },
}

/// Probability distributions for weight initialization using sovereign entropy
///
/// Defines statistical distributions that can be seeded with human-owned entropy
/// for initializing neural network weights, preserving sovereignty over AI training.
#[derive(Debug, Clone, Copy, PartialEq, Serialize, Deserialize)]
pub enum EntropyDistribution {
    /// Normal (Gaussian) distribution using human entropy as seed
    Normal {
        /// Mean (center) of the distribution
        mean: f64,
        /// Standard deviation (spread) of the distribution
        stddev: f64,
    },
    /// Uniform distribution over a range
    Uniform {
        /// Minimum value (inclusive)
        min: f64,
        /// Maximum value (exclusive)
        max: f64,
    },
    /// Xavier/Glorot initialization (variance scaled by fan-in/fan-out)
    Xavier,
    /// He initialization (variance scaled by fan-in only, for `ReLU` networks)
    He,
}

/// Layer-specific regularization configuration
///
/// Defines regularization techniques applied to a neural network layer
/// to prevent overfitting and improve generalization.
#[derive(Debug, Clone, Copy, Serialize, Deserialize)]
pub struct LayerRegularization {
    /// L1 regularization strength (encourages sparsity)
    pub l1_strength: f32,
    /// L2 regularization strength (prevents large weights)
    pub l2_strength: f32,
    /// Dropout rate for randomly disabling neurons during training
    pub dropout_rate: f32,
    /// Batch normalization momentum for running statistics
    pub batch_norm_momentum: f32,
}

// LossFunction now imported from canonical location

/// Neural network layer connection types
///
/// Defines how layers are connected in complex architectures
/// like `ResNets`, `DenseNets`, and attention networks.
#[derive(Debug, Clone, Copy, Serialize, Deserialize)]
pub enum ConnectionType {
    /// Add connection (residual/skip connection)
    Add,
    /// Concatenate connection (along feature dimension)
    Concatenate,
    /// Element-wise multiply connection (gating mechanism)
    Multiply,
}

/// Neural network training parameters
///
/// Comprehensive configuration for training neural networks, including
/// hyperparameters, optimization settings, and performance metrics.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct TrainingParams {
    /// Number of samples per training batch
    pub batch_size: u32,
    /// Number of complete passes through training data
    pub epochs: u32,
    /// Learning rate for gradient descent updates
    pub learning_rate: f64,
    /// Optional learning rate schedule for adaptive learning
    pub lr_scheduler: Option<LrScheduler>,
    /// Optimization algorithm configuration
    pub optimizer: Optimizer,
    /// Loss function to minimize during training
    pub loss_function: LossFunction,
    /// Performance metrics to track during training
    pub metrics: Vec<Metric>,
}

impl Default for TrainingParams {
    fn default() -> Self {
        Self {
            batch_size: 32,
            epochs: 100,
            learning_rate: 0.001,
            lr_scheduler: None,
            optimizer: Optimizer::default(),
            loss_function: LossFunction::MeanSquaredError,
            metrics: vec![],
        }
    }
}

/// Optimizer configuration for neural network training
///
/// Specifies the optimization algorithm and its hyperparameters
/// for updating model weights during training.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Optimizer {
    /// Type of optimization algorithm
    pub optimizer_type: OptimizerType,
    /// Algorithm-specific hyperparameters (e.g., momentum, beta1, beta2)
    pub parameters: HashMap<String, f64>,
}

impl Default for Optimizer {
    fn default() -> Self {
        Self {
            optimizer_type: OptimizerType::Adam,
            parameters: HashMap::new(),
        }
    }
}

/// Learning rate scheduler configuration
///
/// Dynamically adjusts the learning rate during training to improve
/// convergence and final model performance.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct LrScheduler {
    /// Type of learning rate schedule
    pub scheduler_type: LrSchedulerType,
    /// Schedule-specific parameters (e.g., decay rate, step size, milestones)
    pub parameters: HashMap<String, f64>,
}

/// Learning rate scheduler types
///
/// Strategies for dynamically adjusting the learning rate during training
/// to improve convergence and avoid local minima.
#[derive(Debug, Clone, Copy, Serialize, Deserialize)]
pub enum LrSchedulerType {
    /// Step decay (reduce LR at fixed intervals)
    StepDecay,
    /// Exponential decay (exponential reduction over time)
    ExponentialDecay,
    /// Cosine annealing (smooth cosine-shaped reduction)
    CosineAnnealing,
    /// Reduce on plateau (reduce when metrics stop improving)
    ReduceOnPlateau,
    /// Cyclic learning rate (oscillate between min/max values)
    CyclicLr,
}

#[derive(Debug, Clone, Copy, Serialize, Deserialize)]
/// Types of optimizer
pub enum OptimizerType {
    /// Stochastic Gradient Descent
    Sgd,
    /// Adam optimizer
    Adam,
    /// `AdamW` optimizer
    AdamW,
    /// `RMSprop` optimizer
    RmsProp,
    /// Adagrad optimizer
    Adagrad,
    /// Adadelta optimizer
    Adadelta,
}

/// Performance metrics for model evaluation
///
/// Standard metrics used to evaluate machine learning model performance
/// during training and testing, with different metrics suited for different tasks.
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
pub enum Metric {
    /// Accuracy (correct predictions / total predictions)
    Accuracy,
    /// Precision (true positives / predicted positives)
    Precision,
    /// Recall (true positives / actual positives)
    Recall,
    /// F1 score (harmonic mean of precision and recall)
    F1Score,
    /// Area under ROC curve (classification performance across thresholds)
    AucRoc,
    /// Area under precision-recall curve
    AucPr,
    /// Mean absolute error (average absolute difference)
    Mae,
    /// Mean squared error (average squared difference)
    Mse,
    /// Root mean squared error (square root of MSE)
    Rmse,
    /// Custom metric with user-defined calculation
    Custom(String),
}

/// Neural network training optimization settings
///
/// Advanced optimization techniques to improve training speed,
/// memory efficiency, and numerical stability.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct NetworkOptimization {
    /// Use mixed precision (FP16/FP32) for faster training
    pub mixed_precision: bool,
    /// Gradient clipping to prevent exploding gradients
    pub gradient_clipping: Option<GradientClipping>,
    /// Dynamically optimize batch size for hardware
    pub batch_size_optimization: bool,
    /// Enable memory-saving techniques (gradient checkpointing, etc.)
    pub memory_optimization: bool,
}

/// Gradient clipping configuration
///
/// Prevents exploding gradients by limiting their magnitude during
/// backpropagation, improving training stability.
#[derive(Debug, Clone, Copy, Serialize, Deserialize)]
pub struct GradientClipping {
    /// Method used for clipping gradients
    pub clip_type: ClipType,
    /// Maximum allowed gradient magnitude
    pub clip_value: f64,
}

/// Gradient clipping methods
///
/// Different strategies for constraining gradient magnitudes during
/// neural network training to prevent instability.
#[derive(Debug, Clone, Copy, Serialize, Deserialize)]
pub enum ClipType {
    /// Clip individual gradient values to a range
    Value,
    /// Clip based on L2 norm of gradient tensor
    Norm,
    /// Clip based on global norm across all parameters
    GlobalNorm,
}

/// Neural network regularization configuration
///
/// Comprehensive regularization techniques to prevent overfitting
/// and improve model generalization to unseen data.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct NetworkRegularization {
    /// Dropout for randomly disabling neurons
    pub dropout: Option<DropoutConfig>,
    /// Batch normalization for stable training
    pub batch_normalization: bool,
    /// Weight decay (L2 regularization) strength
    pub weight_decay: f64,
    /// Early stopping to prevent overtraining
    pub early_stopping: Option<EarlyStoppingConfig>,
}

/// Dropout layer configuration
///
/// Controls the dropout regularization technique, which randomly disables
/// neurons during training to prevent co-adaptation and overfitting.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct DropoutConfig {
    /// Dropout rate
    /// The rate value
    pub rate: f64,
    /// Apply dropout during training only
    /// Whether `training_only` is enabled
    pub training_only: bool,
    /// Dropout schedule
    /// Optional schedule
    pub schedule: Option<DropoutSchedule>,
}

/// Dropout rate scheduling over training
///
/// Dynamically adjusts the dropout rate during training, typically
/// starting high and reducing over time as the model converges.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct DropoutSchedule {
    /// Starting dropout rate at beginning of training
    pub initial_rate: f64,
    /// Final dropout rate at end of training
    pub final_rate: f64,
    /// Strategy for transitioning between initial and final rates
    pub schedule_type: ScheduleType,
}

/// Scheduling strategy types
///
/// Methods for transitioning a parameter (like dropout rate or learning rate)
/// from an initial value to a final value during training.
#[derive(Debug, Clone, Copy, Serialize, Deserialize)]
pub enum ScheduleType {
    /// Linear interpolation between initial and final values
    Linear,
    /// Exponential decay from initial to final values
    Exponential,
    /// Step-wise changes at fixed intervals
    Step,
}

/// Early stopping configuration
///
/// Automatically stops training when a monitored metric stops improving,
/// preventing overfitting and saving computation time.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct EarlyStoppingConfig {
    /// Name of the metric to monitor (e.g., "`validation_loss`")
    pub monitor: String,
    /// Minimum improvement required to consider it significant
    pub min_delta: f64,
    /// Patience (epochs to wait)
    /// Number of patience
    pub patience: u32,
    /// Restore best weights
    /// Whether `restore_best_weights` is enabled
    pub restore_best_weights: bool,
    /// Monitoring mode
    /// The mode value
    pub mode: MonitoringMode,
}

/// Early stopping monitoring mode
///
/// Specifies whether to stop training when a metric reaches a minimum
/// or maximum value, or automatically detect based on the metric name.
#[derive(Debug, Clone, Copy, Serialize, Deserialize)]
pub enum MonitoringMode {
    /// Stop when metric reaches minimum (e.g., for loss)
    Min,
    /// Stop when metric reaches maximum (e.g., for accuracy)
    Max,
    /// Auto-detect based on metric name (loss→min, accuracy→max)
    Auto,
}
