// SPDX-License-Identifier: AGPL-3.0-only

//! Neural Network Architecture Configuration
//!
//! This module contains all neural network architecture types including
//! detailed layer configurations, activation functions, and network structures.

use serde::{Deserialize, Serialize};

/// **NEURAL NETWORK CONFIGURATION** - Network architecture settings
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct NeuralNetworkConfig {
    /// Network architecture type
    pub architecture: NetworkArchitecture,

    /// Hidden layer sizes
    pub hidden_layers: Vec<usize>,

    /// Activation function
    pub activation: ActivationFunction,

    /// Dropout rate
    pub dropout_rate: f64,

    /// Regularization settings
    pub regularization: RegularizationConfig,

    /// Optimizer configuration
    pub optimizer: OptimizerConfig,

    /// Detailed network architecture (extended from beardog-core)
    pub detailed_architecture: Option<DetailedNetworkArchitecture>,
}

impl Default for NeuralNetworkConfig {
    fn default() -> Self {
        Self {
            architecture: NetworkArchitecture::Feedforward,
            hidden_layers: vec![128, 64],
            activation: ActivationFunction::Relu,
            dropout_rate: std::env::var("BEARDOG_AI_NEURAL_DROPOUT_RATE")
                .ok()
                .and_then(|s| s.parse().ok())
                .unwrap_or(0.2),
            regularization: RegularizationConfig::default(),
            optimizer: OptimizerConfig::default(),
            detailed_architecture: None,
        }
    }
}

// ============================================================================
// DETAILED NEURAL NETWORK TYPES (migrated from beardog-core)
// ============================================================================

/// Detailed network architecture definition with layer-by-layer configuration
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct DetailedNetworkArchitecture {
    /// Architecture type
    pub architecture_type: DetailedArchitectureType,
    /// Input layer configuration
    pub input_layer: InputLayerConfig,
    /// Hidden layers configuration
    pub hidden_layers: Vec<LayerConfig>,
    /// Output layer configuration
    pub output_layer: OutputLayerConfig,
    /// Skip connections (for ResNet-like architectures)
    pub skip_connections: Vec<SkipConnection>,
}

/// Detailed architecture types
#[derive(Debug, Clone, Copy, Serialize, Deserialize, PartialEq, Eq)]
pub enum DetailedArchitectureType {
    /// Feedforward neural network
    Feedforward,
    /// Convolutional neural network
    Convolutional,
    /// Recurrent neural network
    Recurrent,
    /// Transformer architecture
    Transformer,
    /// Graph neural network
    GraphNeuralNetwork,
    /// Attention-based network
    AttentionBased,
    /// Generative adversarial network
    GenerativeAdversarial,
    /// Autoencoder
    Autoencoder,
    /// Hybrid architecture
    Hybrid,
}

/// Input layer configuration
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct InputLayerConfig {
    /// Input shape dimensions
    pub input_shape: Vec<usize>,
    /// Data type for input
    pub data_type: DataType,
    /// Normalization to apply
    pub normalization: Option<NormalizationConfig>,
}

/// Layer configuration (combining type and parameters)
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct LayerConfig {
    /// Layer type
    pub layer_type: LayerType,
    /// Layer-specific parameters
    pub parameters: LayerParameters,
}

/// Output layer configuration
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
pub struct OutputLayerConfig {
    /// Number of output units
    pub output_units: usize,
    /// Activation function for output
    pub activation: ActivationFunction,
    /// Loss function
    pub loss_function: LossFunction,
}

/// Skip connection configuration for residual networks
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
pub struct SkipConnection {
    /// Source layer index
    pub from_layer: usize,
    /// Destination layer index
    pub to_layer: usize,
    /// Connection type
    pub connection_type: ConnectionType,
}

/// Data types for neural network computations
#[derive(Debug, Clone, Copy, Serialize, Deserialize, PartialEq, Eq)]
pub enum DataType {
    /// 16-bit floating point
    Float16,
    /// 32-bit floating point
    Float32,
    /// 64-bit floating point
    Float64,
    /// 8-bit integer
    Int8,
    /// 16-bit integer
    Int16,
    /// 32-bit integer
    Int32,
}

/// Normalization configuration
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct NormalizationConfig {
    /// Type of normalization
    pub normalization_type: NormalizationType,
    /// Epsilon for numerical stability
    pub epsilon: f64,
    /// Momentum for batch normalization
    pub momentum: Option<f64>,
}

/// Types of normalization
#[derive(Debug, Clone, Copy, Serialize, Deserialize, PartialEq, Eq)]
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

/// Layer types available in neural networks
#[derive(Debug, Clone, Copy, Serialize, Deserialize, PartialEq, Eq)]
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
    /// Simple RNN layer
    SimpleRnn,
    /// Attention layer
    Attention,
    /// Multi-head attention
    MultiHeadAttention,
    /// Self-attention layer
    SelfAttention,
    /// Embedding layer
    Embedding,
    /// Batch normalization layer
    BatchNorm,
    /// Layer normalization
    LayerNorm,
    /// Flatten layer
    Flatten,
    /// Reshape layer
    Reshape,
}

/// Layer-specific parameters
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
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

/// Dense (fully connected) layer configuration
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct DenseLayerConfig {
    /// Number of units/neurons
    pub units: usize,
    /// Activation function
    pub activation: ActivationFunction,
    /// Use bias term
    pub use_bias: bool,
    /// Weight initialization method
    pub weight_init: WeightInitialization,
}

/// Convolutional layer configuration
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
pub struct ConvLayerConfig {
    /// Number of filters
    pub filters: usize,
    /// Kernel size
    pub kernel_size: Vec<usize>,
    /// Stride
    pub stride: Vec<usize>,
    /// Padding
    pub padding: PaddingType,
    /// Activation function
    pub activation: ActivationFunction,
    /// Use bias term
    pub use_bias: bool,
}

/// Pooling layer configuration
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
pub struct PoolingLayerConfig {
    /// Pool size
    pub pool_size: Vec<usize>,
    /// Stride
    pub stride: Vec<usize>,
    /// Padding
    pub padding: PaddingType,
}

/// RNN layer configuration
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct RnnLayerConfig {
    /// Number of units
    pub units: usize,
    /// RNN cell type
    pub cell_type: RnnCellType,
    /// Return sequences
    pub return_sequences: bool,
    /// Dropout rate
    pub dropout: f64,
    /// Recurrent dropout
    pub recurrent_dropout: f64,
}

/// Attention layer configuration
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct AttentionLayerConfig {
    /// Number of attention heads
    pub num_heads: usize,
    /// Key dimension
    pub key_dim: usize,
    /// Value dimension
    pub value_dim: Option<usize>,
    /// Dropout rate
    pub dropout: f64,
}

/// Embedding layer configuration
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
pub struct EmbeddingLayerConfig {
    /// Vocabulary size
    pub vocab_size: usize,
    /// Embedding dimension
    pub embedding_dim: usize,
    /// Maximum sequence length
    pub max_length: Option<usize>,
    /// Mask zero values
    pub mask_zero: bool,
}

/// Padding types for convolutional layers
#[derive(Debug, Clone, Copy, Serialize, Deserialize, PartialEq, Eq)]
pub enum PaddingType {
    /// Valid padding (no padding)
    Valid,
    /// Same padding (output size = input size)
    Same,
    /// Custom padding
    Custom,
}

/// RNN cell types
#[derive(Debug, Clone, Copy, Serialize, Deserialize, PartialEq, Eq)]
pub enum RnnCellType {
    /// Simple RNN cell
    Simple,
    /// LSTM cell
    Lstm,
    /// GRU cell
    Gru,
}

/// Weight initialization methods
#[derive(Debug, Clone, Copy, Serialize, Deserialize, PartialEq)]
pub enum WeightInitialization {
    /// Zero initialization
    Zeros,
    /// Ones initialization
    Ones,
    /// Random uniform distribution
    RandomUniform { min: f64, max: f64 },
    /// Random normal distribution
    RandomNormal { mean: f64, stddev: f64 },
    /// Glorot uniform (Xavier uniform)
    GlorotUniform,
    /// Glorot normal (Xavier normal)
    GlorotNormal,
    /// He uniform
    HeUniform,
    /// He normal
    HeNormal,
    /// LeCun uniform
    LeCunUniform,
    /// LeCun normal
    LeCunNormal,
    /// Orthogonal initialization
    Orthogonal { gain: f64 },
    /// Truncated normal distribution
    TruncatedNormal { mean: f64, stddev: f64 },
    /// Entropy-based initialization (for sovereignty compliance)
    EntropyBased { distribution: EntropyDistribution },
}

/// Entropy distributions for sovereign weight initialization
#[derive(Debug, Clone, Copy, Serialize, Deserialize, PartialEq)]
pub enum EntropyDistribution {
    /// Uniform distribution from human entropy
    HumanUniform,
    /// Normal distribution from human entropy
    HumanNormal,
    /// Mixed human-machine entropy
    MixedEntropy { human_ratio: f64 },
    /// Quantum-inspired distribution
    QuantumInspired,
    /// Biometric-seeded distribution
    BiometricSeeded,
    /// Environmental noise-based
    EnvironmentalNoise,
}

/// Layer regularization configuration
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct LayerRegularization {
    /// L1 regularization factor
    pub l1: Option<f64>,
    /// L2 regularization factor
    pub l2: Option<f64>,
    /// Activity regularization
    pub activity: Option<f64>,
}

/// Loss functions for neural networks
#[derive(Debug, Clone, Copy, Serialize, Deserialize, PartialEq, Eq)]
pub enum LossFunction {
    /// Mean squared error
    MeanSquaredError,
    /// Mean absolute error
    MeanAbsoluteError,
    /// Mean absolute percentage error
    MeanAbsolutePercentageError,
    /// Mean squared logarithmic error
    MeanSquaredLogarithmicError,
    /// Binary crossentropy
    BinaryCrossentropy,
    /// Categorical crossentropy
    CategoricalCrossentropy,
    /// Sparse categorical crossentropy
    SparseCategoricalCrossentropy,
    /// Kullback-Leibler divergence
    KullbackLeiblerDivergence,
    /// Poisson loss
    Poisson,
    /// Cosine similarity
    CosineSimilarity,
    /// Huber loss
    Huber,
}

/// Connection types for skip connections
#[derive(Debug, Clone, Copy, Serialize, Deserialize, PartialEq, Eq)]
pub enum ConnectionType {
    /// Direct addition
    Add,
    /// Concatenation
    Concatenate,
    /// Multiplication
    Multiply,
    /// Average
    Average,
}

/// Network architecture enumeration (simplified)
#[derive(Debug, Clone, Copy, Serialize, Deserialize, PartialEq, Eq)]
pub enum NetworkArchitecture {
    /// Feedforward neural network
    Feedforward,
    /// Convolutional neural network
    Convolutional,
    /// Recurrent neural network
    Recurrent,
    /// Transformer
    Transformer,
    /// ResNet architecture
    ResNet,
    /// VGG architecture
    Vgg,
    /// Inception architecture
    Inception,
    /// DenseNet architecture
    DenseNet,
    /// MobileNet architecture
    MobileNet,
    /// EfficientNet architecture
    EfficientNet,
    /// Custom architecture
    Custom,
}

/// Activation functions
#[derive(Debug, Clone, Copy, Serialize, Deserialize, PartialEq, Eq)]
pub enum ActivationFunction {
    /// Sigmoid activation
    Sigmoid,
    /// Hyperbolic tangent
    Tanh,
    /// Rectified Linear Unit
    Relu,
    /// Leaky ReLU
    LeakyRelu,
    /// Parametric ReLU
    PRelu,
    /// Exponential Linear Unit
    Elu,
    /// Scaled Exponential Linear Unit
    Selu,
    /// Softmax
    Softmax,
    /// Softplus
    Softplus,
    /// Softsign
    Softsign,
    /// Swish
    Swish,
    /// GELU (Gaussian Error Linear Unit)
    Gelu,
    /// Linear (no activation)
    Linear,
}

/// Regularization configuration
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
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
            l1: std::env::var("BEARDOG_AI_REGULARIZATION_L1")
                .ok()
                .and_then(|s| s.parse().ok())
                .unwrap_or(0.0),
            l2: std::env::var("BEARDOG_AI_REGULARIZATION_L2")
                .ok()
                .and_then(|s| s.parse().ok())
                .unwrap_or(0.001),
            dropout: std::env::var("BEARDOG_AI_REGULARIZATION_DROPOUT")
                .ok()
                .and_then(|s| s.parse().ok())
                .unwrap_or(0.2),
        }
    }
}

/// Optimizer configuration
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct OptimizerConfig {
    /// Optimizer type
    pub optimizer_type: OptimizerType,
    /// Learning rate
    pub learning_rate: f64,
    /// Momentum (for applicable optimizers)
    pub momentum: Option<f64>,
}

impl Default for OptimizerConfig {
    fn default() -> Self {
        Self {
            optimizer_type: OptimizerType::Adam,
            learning_rate: std::env::var("BEARDOG_AI_OPTIMIZER_LEARNING_RATE")
                .ok()
                .and_then(|s| s.parse().ok())
                .unwrap_or(0.001),
            momentum: None,
        }
    }
}

/// Optimizer types
#[derive(Debug, Clone, Copy, Serialize, Deserialize, PartialEq, Eq)]
pub enum OptimizerType {
    /// Stochastic Gradient Descent
    Sgd,
    /// Adam optimizer
    Adam,
    /// AdaGrad
    AdaGrad,
    /// RMSprop
    RmsProp,
    /// AdaDelta
    AdaDelta,
    /// Adamax
    Adamax,
    /// Nadam
    Nadam,
}
