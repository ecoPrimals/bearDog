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
#[derive(Debug, Clone, Serialize, Deserialize)]
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
#[derive(Debug, Clone, Serialize, Deserialize)]
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

#[derive(Debug, Clone, Copy, Serialize, Deserialize)]
pub struct EmbeddingLayerConfig {
    /// Input dimension (vocabulary size)
    /// Number of `input_dim`
    pub input_dim: u32,
    /// Output dimension (embedding size)
    /// Number of `output_dim`
    pub output_dim: u32,
    /// Mask zero values
    /// Whether `mask_zero` is enabled
    pub mask_zero: bool,
    /// Input length
    /// Optional input length
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
    RandomUniform {
        /// Minimum value
        minval: f64,
        /// Maximum value
        maxval: f64,
    },
    GlorotUniform,
    /// Xavier/Glorot normal initialization
    GlorotNormal,
    HeUniform,
    /// He normal initialization
    HeNormal,
    /// Human entropy-driven initialization (Sovereign Security Enhancement)
    HumanEntropyInitialization {
        /// Entropy tier requirement (1=Machine, 2=HumanSupervised, 3=HumanLived)
        required_entropy_tier: u8,
        human_identity_id: String,
        /// Initialization distribution type
        distribution: EntropyDistribution,
        fallback_to_machine: bool,
    },
}

#[derive(Debug, Clone, Copy, PartialEq, Serialize, Deserialize)]
pub enum EntropyDistribution {
    /// Normal distribution using human entropy as seed
    Normal {
        /// Mean value of the normal distribution
        mean: f64,
        /// Standard deviation of the normal distribution
        stddev: f64,
    },
    Uniform {
        min: f64,
        max: f64,
    },
    /// Xavier/Glorot initialization using human entropy
    Xavier,
    /// He initialization using human entropy
    He,
}

#[derive(Debug, Clone, Copy, Serialize, Deserialize)]
pub struct LayerRegularization {
    /// L1 regularization strength coefficient
    /// The l1 strength value
    pub l1_strength: f32,
    /// L2 regularization strength coefficient
    /// The l2 strength value
    pub l2_strength: f32,
    /// The dropout rate value
    pub dropout_rate: f32,
    /// Batch normalization momentum parameter
    /// The batch norm momentum value
    pub batch_norm_momentum: f32,
}

// LossFunction now imported from canonical location

#[derive(Debug, Clone, Copy, Serialize, Deserialize)]
/// Types of connection
pub enum ConnectionType {
    /// Add connection (residual)
    Add,
    /// Concatenate connection
    Concatenate,
    /// Multiply connection
    Multiply,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct TrainingParams {
    /// Batch size
    /// Number of `batch_size`
    pub batch_size: u32,
    /// Number of epochs
    /// Number of epochs
    pub epochs: u32,
    /// Learning rate
    /// The learning rate value
    pub learning_rate: f64,
    /// Learning rate scheduler
    /// Optional lr scheduler
    pub lr_scheduler: Option<LrScheduler>,
    /// Optimizer
    /// The optimizer value
    pub optimizer: Optimizer,
    /// Loss function
    /// The loss function value
    pub loss_function: LossFunction,
    /// Metrics to track
    /// Collection of metrics
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

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Optimizer {
    /// Optimizer type
    /// The optimizer type value
    pub optimizer_type: OptimizerType,
    /// Optimizer parameters
    /// Mapping of parameters
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

/// Learning rate scheduler
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct LrScheduler {
    /// Scheduler type
    /// The scheduler type value
    pub scheduler_type: LrSchedulerType,
    /// Scheduler parameters
    /// Mapping of parameters
    pub parameters: HashMap<String, f64>,
}

#[derive(Debug, Clone, Copy, Serialize, Deserialize)]
/// Types of lr scheduler
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
    /// Area under precision-recall curve
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

/// Network optimization settings
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct NetworkOptimization {
    /// Use mixed precision training
    /// Whether `mixed_precision` is enabled
    pub mixed_precision: bool,
    /// Gradient clipping
    /// Optional gradient clipping
    pub gradient_clipping: Option<GradientClipping>,
    /// Batch size optimization
    /// Whether `batch_size_optimization` is enabled
    pub batch_size_optimization: bool,
    /// Memory optimization
    /// Whether `memory_optimization` is enabled
    pub memory_optimization: bool,
}

/// Gradient clipping configuration
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct GradientClipping {
    /// Clipping type
    /// The clip type value
    pub clip_type: ClipType,
    /// Clipping value
    /// The clip value value
    pub clip_value: f64,
}

#[derive(Debug, Clone, Copy, Serialize, Deserialize)]
/// Types of clip
pub enum ClipType {
    /// Clip by value
    Value,
    /// Clip by norm
    Norm,
    /// Clip by global norm
    GlobalNorm,
}

/// Network regularization techniques
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct NetworkRegularization {
    /// Dropout configuration
    /// Optional dropout
    pub dropout: Option<DropoutConfig>,
    /// Batch normalization
    /// Whether `batch_normalization` is enabled
    pub batch_normalization: bool,
    /// Weight decay
    /// The weight decay value
    pub weight_decay: f64,
    /// Early stopping
    /// Optional early stopping
    pub early_stopping: Option<EarlyStoppingConfig>,
}

/// Dropout configuration
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

/// Dropout schedule
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct DropoutSchedule {
    /// Initial dropout rate
    /// The initial rate value
    pub initial_rate: f64,
    /// Final dropout rate
    /// The final rate value
    pub final_rate: f64,
    /// Schedule type
    /// The schedule type value
    pub schedule_type: ScheduleType,
}

#[derive(Debug, Clone, Copy, Serialize, Deserialize)]
/// Types of schedule
pub enum ScheduleType {
    /// Linear schedule
    Linear,
    /// Exponential schedule
    Exponential,
    /// Step schedule
    Step,
}

/// Early stopping configuration
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct EarlyStoppingConfig {
    /// Metric to monitor
    /// The monitor value
    pub monitor: String,
    /// Minimum change threshold
    /// The min delta value
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

#[derive(Debug, Clone, Copy, Serialize, Deserialize)]
pub enum MonitoringMode {
    Min,
    Max,
    /// Auto-detect based on metric name
    Auto,
}
