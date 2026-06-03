// SPDX-License-Identifier: AGPL-3.0-or-later

//! Implementation-level neural network architecture types.
//!
//! These types define detailed neural network configurations used by `beardog-core`'s
//! hybrid intelligence pipeline. They complement the simplified configuration types
//! in the sibling `neural_networks` module.

use serde::{Deserialize, Serialize};
use std::collections::HashMap;

// ── Core Enums ───────────────────────────────────────────────────────────────

/// Activation function types
#[derive(Debug, Clone, Copy, Serialize, Deserialize)]
pub enum ActivationFunction {
    /// Sigmoid activation (0 to 1 range)
    Sigmoid,
    /// Hyperbolic tangent (-1 to 1 range)
    Tanh,
    /// Rectified Linear Unit
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
#[derive(Debug, Clone, Copy, Serialize, Deserialize)]
pub enum DataType {
    /// 16-bit floating point
    Float16,
    /// 32-bit floating point
    Float32,
    /// 64-bit floating point
    Float64,
    /// 8-bit signed integer
    Int8,
    /// 16-bit signed integer
    Int16,
    /// 32-bit signed integer
    Int32,
}

/// Neural network architecture types
#[derive(Debug, Clone, Copy, Serialize, Deserialize)]
pub enum ArchitectureType {
    /// Traditional feedforward network
    Feedforward,
    /// Convolutional network for spatial data
    Convolutional,
    /// Recurrent network for sequential data
    Recurrent,
    /// Transformer architecture with attention
    Transformer,
    /// Hybrid architecture combining multiple types
    Hybrid,
}

/// Loss function types
#[derive(Debug, Clone, Copy, Serialize, Deserialize)]
pub enum LossFunction {
    /// Mean squared error
    MeanSquaredError,
    /// Mean absolute error
    MeanAbsoluteError,
    /// General cross-entropy loss
    CrossEntropy,
    /// Binary cross-entropy
    BinaryCrossEntropy,
    /// Alternative spelling for binary cross-entropy
    BinaryCrossentropy,
    /// Categorical cross-entropy
    CategoricalCrossEntropy,
    /// Huber loss
    Huber,
}

// ── Layer Configurations ─────────────────────────────────────────────────────

/// Input layer configuration
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
#[derive(Debug, Clone, Copy, Serialize, Deserialize)]
pub struct OutputLayerConfig {
    /// Number of output units
    pub units: usize,
    /// Activation function for output
    pub activation: ActivationFunction,
    /// Loss function for training
    pub loss_function: LossFunction,
}

/// Network architecture configuration (struct form)
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct NetworkArchitecture {
    /// Type of neural network architecture
    pub architecture_type: ArchitectureType,
    /// Input layer configuration
    pub input_layer: InputLayerConfig,
    /// Hidden layer configurations
    pub hidden_layers: Vec<LayerParameters>,
    /// Output layer configuration
    pub output_layer: OutputLayerConfig,
    /// Skip/residual connections
    pub skip_connections: Vec<String>,
}

/// Layer configuration combining type and parameters
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct LayerConfig {
    /// Type of layer
    pub layer_type: LayerType,
    /// Layer-specific parameters
    pub parameters: LayerParameters,
}

/// Layer types
#[derive(Debug, Clone, Copy, Serialize, Deserialize)]
pub enum LayerType {
    /// Dense/fully connected layer
    Dense,
    /// 2D Convolutional layer
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

/// Layer parameters (union of all layer-specific configs)
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct LayerParameters {
    /// Dense layer parameters
    pub dense: Option<DenseLayerConfig>,
    /// Convolutional layer parameters
    pub conv: Option<ConvLayerConfig>,
    /// Pooling layer parameters
    pub pooling: Option<PoolingLayerConfig>,
    /// RNN layer parameters
    pub rnn: Option<RnnLayerConfig>,
    /// Attention layer parameters
    pub attention: Option<AttentionLayerConfig>,
    /// Embedding layer parameters
    pub embedding: Option<EmbeddingLayerConfig>,
}

/// Dense layer configuration
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct DenseLayerConfig {
    /// Number of neurons
    pub units: u32,
    /// Whether to include bias terms
    pub use_bias: bool,
    /// Weight initialization strategy
    pub weight_init: WeightInitialization,
    /// Bias initialization strategy
    pub bias_init: WeightInitialization,
}

/// Convolutional layer configuration
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ConvLayerConfig {
    /// Number of filters
    pub filters: u32,
    /// Kernel size
    pub kernel_size: Vec<u32>,
    /// Stride length
    pub strides: Vec<u32>,
    /// Padding strategy
    pub padding: PaddingType,
    /// Dilation rate
    pub dilation_rate: Vec<u32>,
    /// Whether to include bias
    pub use_bias: bool,
}

/// Pooling layer configuration
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PoolingLayerConfig {
    /// Pool window size
    pub pool_size: Vec<u32>,
    /// Stride length
    pub strides: Vec<u32>,
    /// Padding strategy
    pub padding: PaddingType,
}

/// RNN layer configuration
#[derive(Debug, Clone, Copy, Serialize, Deserialize)]
pub struct RnnLayerConfig {
    /// Number of RNN units
    pub units: u32,
    /// Type of RNN cell
    pub cell_type: RnnCellType,
    /// Whether to return full sequence
    pub return_sequences: bool,
    /// Dropout rate
    pub dropout_rate: f64,
}

/// Attention layer configuration
#[derive(Debug, Clone, Copy, Serialize, Deserialize)]
pub struct AttentionLayerConfig {
    /// Number of attention heads
    pub num_heads: u32,
    /// Key dimension
    pub key_dim: u32,
    /// Value dimension
    pub value_dim: Option<u32>,
    /// Dropout rate
    pub dropout_rate: f64,
}

/// Embedding layer configuration
#[derive(Debug, Clone, Copy, Serialize, Deserialize)]
pub struct EmbeddingLayerConfig {
    /// Input dimension (vocabulary size)
    pub input_dim: u32,
    /// Output dimension (embedding size)
    pub output_dim: u32,
    /// Whether to mask zero values
    pub mask_zero: bool,
    /// Fixed input sequence length
    pub input_length: Option<u32>,
}

/// Padding types
#[derive(Debug, Clone, Copy, Serialize, Deserialize)]
pub enum PaddingType {
    /// Valid padding (no padding)
    Valid,
    /// Same padding
    Same,
    /// Custom padding
    Custom(u32),
}

/// RNN cell types
#[derive(Debug, Clone, Copy, Serialize, Deserialize)]
pub enum RnnCellType {
    /// Simple RNN
    SimpleRnn,
    /// Long Short-Term Memory
    Lstm,
    /// Gated Recurrent Unit
    Gru,
}

// ── Weight Initialization ────────────────────────────────────────────────────

/// Weight initialization strategies
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub enum WeightInitialization {
    /// Zeros initialization
    Zeros,
    /// Ones initialization
    Ones,
    /// Random normal initialization
    RandomNormal {
        /// Mean value
        mean: f64,
        /// Standard deviation
        stddev: f64,
    },
    /// Random uniform initialization
    RandomUniform {
        /// Minimum value
        minval: f64,
        /// Maximum value
        maxval: f64,
    },
    /// Glorot/Xavier uniform initialization
    GlorotUniform,
    /// Xavier/Glorot normal initialization
    GlorotNormal,
    /// He uniform initialization
    HeUniform,
    /// He normal initialization
    HeNormal,
    /// Human entropy-driven initialization
    HumanEntropyInitialization {
        /// Entropy tier requirement
        required_entropy_tier: u8,
        /// Human identity providing entropy
        human_identity_id: String,
        /// Initialization distribution type
        distribution: EntropyDistribution,
        /// Whether to fall back to machine entropy
        fallback_to_machine: bool,
    },
}

/// Probability distributions for sovereign entropy weight initialization
#[derive(Debug, Clone, Copy, PartialEq, Serialize, Deserialize)]
pub enum EntropyDistribution {
    /// Normal (Gaussian) distribution
    Normal {
        /// Mean of the distribution
        mean: f64,
        /// Standard deviation
        stddev: f64,
    },
    /// Uniform distribution over a range
    Uniform {
        /// Minimum value (inclusive)
        min: f64,
        /// Maximum value (exclusive)
        max: f64,
    },
    /// Xavier/Glorot initialization
    Xavier,
    /// He initialization
    He,
}

// ── Regularization & Training ────────────────────────────────────────────────

/// Layer-specific regularization configuration
#[derive(Debug, Clone, Copy, Serialize, Deserialize)]
pub struct LayerRegularization {
    /// L1 regularization strength
    pub l1_strength: f32,
    /// L2 regularization strength
    pub l2_strength: f32,
    /// Dropout rate
    pub dropout_rate: f32,
    /// Batch normalization momentum
    pub batch_norm_momentum: f32,
}

/// Layer connection types
#[derive(Debug, Clone, Copy, Serialize, Deserialize)]
pub enum ConnectionType {
    /// Add connection (residual)
    Add,
    /// Concatenate connection
    Concatenate,
    /// Element-wise multiply (gating)
    Multiply,
}

/// Training parameters (implementation-level)
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct TrainingParams {
    /// Batch size
    pub batch_size: u32,
    /// Number of epochs
    pub epochs: u32,
    /// Learning rate
    pub learning_rate: f64,
    /// Learning rate schedule
    pub lr_scheduler: Option<LrScheduler>,
    /// Optimization algorithm configuration
    pub optimizer: Optimizer,
    /// Loss function
    pub loss_function: LossFunction,
    /// Performance metrics to track
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

/// Optimizer configuration
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Optimizer {
    /// Optimization algorithm type
    pub optimizer_type: OptimizerType,
    /// Algorithm-specific hyperparameters
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
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct LrScheduler {
    /// Schedule type
    pub scheduler_type: LrSchedulerType,
    /// Schedule-specific parameters
    pub parameters: HashMap<String, f64>,
}

/// Learning rate scheduler types
#[derive(Debug, Clone, Copy, Serialize, Deserialize)]
pub enum LrSchedulerType {
    /// Step decay
    StepDecay,
    /// Exponential decay
    ExponentialDecay,
    /// Cosine annealing
    CosineAnnealing,
    /// Reduce on plateau
    ReduceOnPlateau,
    /// Cyclic learning rate
    CyclicLr,
}

/// Optimizer types
#[derive(Debug, Clone, Copy, Serialize, Deserialize)]
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

/// Performance metrics
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
pub enum Metric {
    /// Accuracy
    Accuracy,
    /// Precision
    Precision,
    /// Recall
    Recall,
    /// F1 score
    F1Score,
    /// Area under ROC curve
    AucRoc,
    /// Area under PR curve
    AucPr,
    /// Mean absolute error
    Mae,
    /// Mean squared error
    Mse,
    /// Root mean squared error
    Rmse,
    /// Custom metric
    Custom(String),
}

/// Network training optimization settings
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct NetworkOptimization {
    /// Use mixed precision training
    pub mixed_precision: bool,
    /// Gradient clipping configuration
    pub gradient_clipping: Option<GradientClipping>,
    /// Dynamic batch size optimization
    pub batch_size_optimization: bool,
    /// Memory-saving techniques
    pub memory_optimization: bool,
}

/// Gradient clipping configuration
#[derive(Debug, Clone, Copy, Serialize, Deserialize)]
pub struct GradientClipping {
    /// Clipping method
    pub clip_type: ClipType,
    /// Maximum gradient magnitude
    pub clip_value: f64,
}

/// Gradient clipping methods
#[derive(Debug, Clone, Copy, Serialize, Deserialize)]
pub enum ClipType {
    /// Clip individual values
    Value,
    /// Clip by L2 norm
    Norm,
    /// Clip by global norm
    GlobalNorm,
}

/// Network regularization configuration
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct NetworkRegularization {
    /// Dropout configuration
    pub dropout: Option<DropoutConfig>,
    /// Batch normalization
    pub batch_normalization: bool,
    /// Weight decay (L2 regularization)
    pub weight_decay: f64,
    /// Early stopping configuration
    pub early_stopping: Option<EarlyStoppingConfig>,
}

/// Dropout configuration
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct DropoutConfig {
    /// Dropout rate
    pub rate: f64,
    /// Apply during training only
    pub training_only: bool,
    /// Dropout schedule
    pub schedule: Option<DropoutSchedule>,
}

/// Dropout rate scheduling
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct DropoutSchedule {
    /// Starting dropout rate
    pub initial_rate: f64,
    /// Final dropout rate
    pub final_rate: f64,
    /// Scheduling strategy
    pub schedule_type: ScheduleType,
}

/// Schedule strategy types
#[derive(Debug, Clone, Copy, Serialize, Deserialize)]
pub enum ScheduleType {
    /// Linear interpolation
    Linear,
    /// Exponential decay
    Exponential,
    /// Step-wise changes
    Step,
}

/// Early stopping configuration
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct EarlyStoppingConfig {
    /// Metric to monitor
    pub monitor: String,
    /// Minimum improvement required
    pub min_delta: f64,
    /// Patience (epochs to wait)
    pub patience: u32,
    /// Restore best weights
    pub restore_best_weights: bool,
    /// Monitoring mode
    pub mode: MonitoringMode,
}

/// Early stopping monitoring mode
#[derive(Debug, Clone, Copy, Serialize, Deserialize)]
pub enum MonitoringMode {
    /// Stop when metric reaches minimum
    Min,
    /// Stop when metric reaches maximum
    Max,
    /// Auto-detect from metric name
    Auto,
}
