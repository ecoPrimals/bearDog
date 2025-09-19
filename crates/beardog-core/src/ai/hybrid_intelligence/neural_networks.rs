// Neural network architectures and configurations

use serde::{Deserialize, Serialize};
use std::collections::HashMap;

/// Network architecture definition
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct NetworkArchitecture {
    /// Architecture type
    /// The architecture type value
    pub architecture_type: ArchitectureType,
    /// Input layer configuration
    /// The input layer value
    pub input_layer: InputLayerConfig,
    /// Hidden layers configuration
    pub hidden_layers: Vec<LayerConfig>,
    /// Output layer configuration
    /// The output layer value
    pub output_layer: OutputLayerConfig,
    /// Skip connections
    /// Collection of skip connections
    pub skip_connections: Vec<SkipConnection>,
}

#[derive(Debug, Clone, Copy, Serialize, Deserialize)]
/// Types of architecture
pub enum ArchitectureType {
    Feedforward,
    /// Convolutional neural network
    Convolutional,
    /// Recurrent neural network
    Recurrent,
    /// Long Short-Term Memory network
    Lstm,
    /// Gated Recurrent Unit network
    Gru,
    Transformer,
    /// Autoencoder
    Autoencoder,
    /// Generative Adversarial Network
    Gan,
    /// Variational Autoencoder
    Vae,
    /// Custom architecture
    Custom,
}

/// Input layer configuration
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct InputLayerConfig {
    /// Input shape
    /// Collection of input shape
    pub input_shape: Vec<u32>,
    /// Input data type
    /// The data type value
    pub data_type: DataType,
    /// Normalization configuration
    /// Optional normalization
    pub normalization: Option<NormalizationConfig>,
}

/// Layer configuration
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct LayerConfig {
    /// Layer type
    /// The layer type value
    pub layer_type: LayerType,
    /// Layer parameters
    /// The parameters value
    pub parameters: LayerParameters,
    /// Activation function
    /// Optional activation
    pub activation: Option<ActivationFunction>,
    /// Regularization configuration
    /// Optional regularization
    pub regularization: Option<LayerRegularization>,
}

/// Output layer configuration
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct OutputLayerConfig {
    /// Number of output units
    /// Number of units
    pub units: u32,
    /// Activation function
    /// The activation value
    pub activation: ActivationFunction,
    /// Loss function
    /// The loss function value
    pub loss_function: LossFunction,
}

/// Skip connection definition
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SkipConnection {
    /// Source layer index
    /// Number of from_layer
    pub from_layer: u32,
    /// Target layer index
    /// Number of to_layer
    pub to_layer: u32,
    /// Connection type
    /// The connection type value
    pub connection_type: ConnectionType,
}

#[derive(Debug, Clone, Copy, Serialize, Deserialize)]
/// Types of data
pub enum DataType {
    /// 32-bit floating point
    Float32,
    /// 64-bit floating point
    Float64,
    /// 32-bit integer
    Int32,
    /// 64-bit integer
    Int64,
    /// Boolean
    Bool,
}

/// Normalization configuration
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct NormalizationConfig {
    /// Normalization type
    /// The normalization type value
    pub normalization_type: NormalizationType,
    /// Normalization parameters
    /// Mapping of parameters
    pub parameters: HashMap<String, f64>,
}

/// Normalization types
#[derive(Debug, Clone, Copy, Serialize, Deserialize, PartialEq, Eq)]
/// Types of normalization
pub enum NormalizationType {
    /// Batch normalization
    Batch,
    /// Layer normalization
    Layer,
    /// Instance normalization
    Instance,
    /// Group normalization
    Group,
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
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct LayerParameters {
    /// Dense layer parameters
    /// Optional dense
    pub dense: Option<DenseLayerConfig>,
    /// Convolutional layer parameters
    /// Optional conv
    pub conv: Option<ConvLayerConfig>,
    /// Pooling layer parameters
    /// Optional pooling
    pub pooling: Option<PoolingLayerConfig>,
    /// RNN layer parameters
    /// Optional rnn
    pub rnn: Option<RnnLayerConfig>,
    /// Attention layer parameters
    /// Optional attention
    pub attention: Option<AttentionLayerConfig>,
    /// Embedding layer parameters
    /// Optional embedding
    pub embedding: Option<EmbeddingLayerConfig>,
}

/// Dense layer configuration
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct DenseLayerConfig {
    /// Number of units
    /// Number of units
    pub units: u32,
    /// Use bias
    /// Whether use_bias is enabled
    pub use_bias: bool,
    /// Weight initialization
    /// The weight init value
    pub weight_init: WeightInitialization,
    /// Bias initialization
    /// The bias init value
    pub bias_init: WeightInitialization,
}

/// Convolutional layer configuration
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ConvLayerConfig {
    /// Number of filters
    /// Number of filters
    pub filters: u32,
    /// Kernel size
    /// Collection of kernel size
    pub kernel_size: Vec<u32>,
    /// Stride
    pub strides: Vec<u32>,
    /// Padding
    /// The padding value
    pub padding: PaddingType,
    /// Dilation rate
    /// Collection of dilation rate
    pub dilation_rate: Vec<u32>,
    /// Use bias
    /// Whether use_bias is enabled
    pub use_bias: bool,
}

/// Pooling layer configuration
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PoolingLayerConfig {
    /// Pool size
    /// Collection of pool size
    pub pool_size: Vec<u32>,
    /// Stride
    pub strides: Vec<u32>,
    /// Padding
    /// The padding value
    pub padding: PaddingType,
}

/// RNN layer configuration
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct RnnLayerConfig {
    /// Number of units
    /// Number of units
    pub units: u32,
    /// RNN cell type
    /// The cell type value
    pub cell_type: RnnCellType,
    /// Return sequences
    /// Whether return_sequences is enabled
    pub return_sequences: bool,
    /// Dropout rate
    /// The dropout rate value
    pub dropout_rate: f64,
}

#[derive(Debug, Clone, Copy, Serialize, Deserialize)]
pub struct AttentionLayerConfig {
    /// Number of attention heads
    /// Number of num_heads
    pub num_heads: u32,
    /// Key dimension
    /// Number of key_dim
    pub key_dim: u32,
    /// Value dimension
    /// Optional value dim
    pub value_dim: Option<u32>,
    /// Dropout rate
    /// The dropout rate value
    pub dropout_rate: f64,
}

#[derive(Debug, Clone, Copy, Serialize, Deserialize)]
pub struct EmbeddingLayerConfig {
    /// Input dimension (vocabulary size)
    /// Number of input_dim
    pub input_dim: u32,
    /// Output dimension (embedding size)
    /// Number of output_dim
    pub output_dim: u32,
    /// Mask zero values
    /// Whether mask_zero is enabled
    pub mask_zero: bool,
    /// Input length
    /// Optional input length
    pub input_length: Option<u32>,
}

#[derive(Debug, Clone, Copy, Serialize, Deserialize)]
pub enum ActivationFunction {
    /// ReLU activation
    Relu,
    /// Sigmoid activation
    Sigmoid,
    /// Tanh activation
    Tanh,
    /// Softmax activation
    Softmax,
    /// Leaky ReLU activation
    LeakyRelu,
    /// ELU activation
    Elu,
    /// Swish activation
    Swish,
    /// GELU activation
    Gelu,
    /// Linear activation
    Linear,
}

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

/// Loss functions
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
pub enum LossFunction {
    /// Mean squared error
    MeanSquaredError,
    /// Mean absolute error
    MeanAbsoluteError,
    /// Binary crossentropy
    BinaryCrossentropy,
    /// Categorical crossentropy
    CategoricalCrossentropy,
    /// Sparse categorical crossentropy
    SparseCategoricalCrossentropy,
    /// Huber loss
    Huber,
    /// Hinge loss
    Hinge,
    /// KL divergence
    KlDivergence,
    /// Custom loss
    Custom(String),
}

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
    /// Number of batch_size
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

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Optimizer {
    /// Optimizer type
    /// The optimizer type value
    pub optimizer_type: OptimizerType,
    /// Optimizer parameters
    /// Mapping of parameters
    pub parameters: HashMap<String, f64>,
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
    /// AdamW optimizer
    AdamW,
    /// RMSprop optimizer
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
    /// Whether mixed_precision is enabled
    pub mixed_precision: bool,
    /// Gradient clipping
    /// Optional gradient clipping
    pub gradient_clipping: Option<GradientClipping>,
    /// Batch size optimization
    /// Whether batch_size_optimization is enabled
    pub batch_size_optimization: bool,
    /// Memory optimization
    /// Whether memory_optimization is enabled
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
    /// Whether batch_normalization is enabled
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
    /// Whether training_only is enabled
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
    /// Whether restore_best_weights is enabled
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
