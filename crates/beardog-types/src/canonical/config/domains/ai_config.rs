//! # AI Configuration Domain
//!
//! This module contains all AI/ML related configuration types, extracted from
//! the large `consolidated_domains.rs` file for better maintainability.

use beardog_errors::{BearDogError, BearDogResult};
use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use std::time::Duration;

use crate::canonical::config::r#trait::BearDogConfig;

/// **CONSOLIDATED AI CONFIGURATION** - Unifies all AI/ML related configs
///
/// Consolidates: `HybridIntelligenceConfig`, `MachineLearningConfig`, `TrainingConfig`,
/// `InferenceConfig`, `PredictionConfig`, `NeuralNetworkConfig`, `DecisionEngineConfig`, etc.
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
#[derive(Default)]
pub struct ConsolidatedAiConfig {
    /// Enable AI/ML functionality
    pub enabled: bool,
    
    /// Hybrid intelligence configuration
    pub hybrid_intelligence: HybridIntelligenceConfig,
    
    /// Machine learning training configuration
    pub training: TrainingConfig,
    
    /// Model inference and serving configuration
    pub inference: InferenceConfig,
    
    /// Neural network architecture configuration
    pub neural_networks: NeuralNetworkConfig,
    
    /// Decision engine configuration
    pub decision_engine: DecisionEngineConfig,
    
    /// Model management and deployment
    pub model_management: ModelManagementConfig,
    
    /// AI performance optimization settings
    pub performance: AiPerformanceConfig,
    
    /// AI security and privacy settings
    pub security: AiSecurityConfig,
}

/// **HYBRID INTELLIGENCE CONFIGURATION** - Human-AI collaboration settings
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct HybridIntelligenceConfig {
    /// Enable hybrid intelligence mode
    pub enabled: bool,
    
    /// Human oversight level (0.0 = full automation, 1.0 = full human control)
    pub human_oversight_level: f64,
    
    /// Confidence threshold for automatic decisions
    pub auto_decision_threshold: f64,
    
    /// Enable human feedback learning
    pub feedback_learning: bool,
    
    /// Maximum time to wait for human input
    pub human_input_timeout: Duration,
}

/// **TRAINING CONFIGURATION** - ML model training settings
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct TrainingConfig {
    /// Enable training mode
    pub enabled: bool,
    
    /// Training data sources
    pub data_sources: Vec<String>,
    
    /// Training batch size
    pub batch_size: usize,
    
    /// Learning rate
    pub learning_rate: f64,
    
    /// Number of training epochs
    pub epochs: u32,
    
    /// Validation split ratio
    pub validation_split: f64,
    
    /// Early stopping patience
    pub early_stopping_patience: u32,
    
    /// Model checkpoint frequency
    pub checkpoint_frequency: u32,
}

/// **INFERENCE CONFIGURATION** - Model serving and inference settings
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct InferenceConfig {
    /// Enable inference mode
    pub enabled: bool,
    
    /// Model serving endpoints
    pub endpoints: Vec<String>,
    
    /// Maximum batch size for inference
    pub max_batch_size: usize,
    
    /// Inference timeout
    pub timeout: Duration,
    
    /// Enable model caching
    pub enable_caching: bool,
    
    /// Cache size limit
    pub cache_size_limit: usize,
    
    /// Enable GPU acceleration
    pub enable_gpu: bool,
}

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
    /// Long Short-Term Memory network
    Lstm,
    /// Gated Recurrent Unit network
    Gru,
    /// Transformer architecture
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
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct InputLayerConfig {
    /// Input shape (e.g., [28, 28, 1] for images)
    pub input_shape: Vec<u32>,
    /// Input data type
    pub data_type: DataType,
    /// Normalization configuration
    pub normalization: Option<NormalizationConfig>,
}

/// Layer configuration for hidden and other layers
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct LayerConfig {
    /// Layer type
    pub layer_type: LayerType,
    /// Layer-specific parameters
    pub parameters: LayerParameters,
    /// Activation function
    pub activation: Option<ActivationFunction>,
    /// Regularization configuration
    pub regularization: Option<LayerRegularization>,
}

/// Output layer configuration
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct OutputLayerConfig {
    /// Number of output units
    pub units: u32,
    /// Activation function
    pub activation: ActivationFunction,
    /// Loss function
    pub loss_function: LossFunction,
}

/// Skip connection definition (for residual networks)
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct SkipConnection {
    /// Source layer index
    pub from_layer: u32,
    /// Target layer index
    pub to_layer: u32,
    /// Connection type
    pub connection_type: ConnectionType,
}

/// Data types for neural network inputs
#[derive(Debug, Clone, Copy, Serialize, Deserialize, PartialEq, Eq)]
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
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct NormalizationConfig {
    /// Normalization type
    pub normalization_type: NormalizationType,
    /// Normalization parameters
    pub parameters: HashMap<String, f64>,
}

/// Normalization types
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
    /// Max pooling layer (2D)
    MaxPool2d,
    /// Average pooling layer (2D)
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

/// Layer parameters container (holds type-specific configs)
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
    pub units: u32,
    /// Use bias term
    pub use_bias: bool,
    /// Weight initialization strategy
    pub weight_init: WeightInitialization,
    /// Bias initialization strategy
    pub bias_init: WeightInitialization,
}

/// Convolutional layer configuration
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct ConvLayerConfig {
    /// Number of filters/kernels
    pub filters: u32,
    /// Kernel size (e.g., [3, 3] for 3x3 kernel)
    pub kernel_size: Vec<u32>,
    /// Stride (e.g., [1, 1])
    pub strides: Vec<u32>,
    /// Padding type
    pub padding: PaddingType,
    /// Dilation rate
    pub dilation_rate: Vec<u32>,
    /// Use bias term
    pub use_bias: bool,
}

/// Pooling layer configuration
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct PoolingLayerConfig {
    /// Pool size (e.g., [2, 2] for 2x2 pooling)
    pub pool_size: Vec<u32>,
    /// Stride
    pub strides: Vec<u32>,
    /// Padding type
    pub padding: PaddingType,
}

/// RNN layer configuration
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct RnnLayerConfig {
    /// Number of units
    pub units: u32,
    /// RNN cell type
    pub cell_type: RnnCellType,
    /// Return sequences (all timesteps) or just last output
    pub return_sequences: bool,
    /// Dropout rate
    pub dropout_rate: f64,
}

/// Attention layer configuration
#[derive(Debug, Clone, Copy, Serialize, Deserialize, PartialEq)]
pub struct AttentionLayerConfig {
    /// Number of attention heads
    pub num_heads: u32,
    /// Key dimension
    pub key_dim: u32,
    /// Value dimension (if None, uses key_dim)
    pub value_dim: Option<u32>,
    /// Dropout rate
    pub dropout_rate: f64,
}

/// Embedding layer configuration
#[derive(Debug, Clone, Copy, Serialize, Deserialize, PartialEq)]
pub struct EmbeddingLayerConfig {
    /// Input dimension (vocabulary size)
    pub input_dim: u32,
    /// Output dimension (embedding size)
    pub output_dim: u32,
    /// Mask zero values
    pub mask_zero: bool,
    /// Input length (sequence length)
    pub input_length: Option<u32>,
}

/// Padding types for convolutional and pooling layers
#[derive(Debug, Clone, Copy, Serialize, Deserialize, PartialEq, Eq)]
pub enum PaddingType {
    /// Valid padding (no padding)
    Valid,
    /// Same padding (output same size as input)
    Same,
    /// Custom padding value
    Custom(u32),
}

/// RNN cell types
#[derive(Debug, Clone, Copy, Serialize, Deserialize, PartialEq, Eq)]
pub enum RnnCellType {
    /// Simple RNN cell
    SimpleRnn,
    /// Long Short-Term Memory cell
    Lstm,
    /// Gated Recurrent Unit cell
    Gru,
}

/// Weight initialization strategies
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub enum WeightInitialization {
    /// Initialize with zeros
    Zeros,
    /// Initialize with ones
    Ones,
    /// Random normal distribution
    RandomNormal {
        /// Mean value
        mean: f64,
        /// Standard deviation
        stddev: f64,
    },
    /// Random uniform distribution
    RandomUniform {
        /// Minimum value
        minval: f64,
        /// Maximum value
        maxval: f64,
    },
    /// Glorot/Xavier uniform initialization
    GlorotUniform,
    /// Glorot/Xavier normal initialization
    GlorotNormal,
    /// He uniform initialization
    HeUniform,
    /// He normal initialization
    HeNormal,
    /// Human entropy-driven initialization (Sovereign Security Enhancement)
    HumanEntropyInitialization {
        /// Entropy tier requirement (1=Machine, 2=HumanSupervised, 3=HumanLived)
        required_entropy_tier: u8,
        /// Human identity identifier
        human_identity_id: String,
        /// Distribution type for entropy
        distribution: EntropyDistribution,
        /// Fallback to machine RNG if human entropy unavailable
        fallback_to_machine: bool,
    },
}

/// Entropy distribution types for human-entropy-driven initialization
#[derive(Debug, Clone, Copy, PartialEq, Serialize, Deserialize)]
pub enum EntropyDistribution {
    /// Normal distribution using human entropy as seed
    Normal {
        /// Mean value
        mean: f64,
        /// Standard deviation
        stddev: f64,
    },
    /// Uniform distribution
    Uniform {
        /// Minimum value
        min: f64,
        /// Maximum value
        max: f64,
    },
    /// Xavier/Glorot initialization
    Xavier,
    /// He initialization
    He,
}

/// Layer regularization configuration
#[derive(Debug, Clone, Copy, Serialize, Deserialize, PartialEq)]
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

/// Loss functions for training
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
pub enum LossFunction {
    /// Mean squared error (regression)
    MeanSquaredError,
    /// Mean absolute error (regression)
    MeanAbsoluteError,
    /// Binary crossentropy (binary classification)
    BinaryCrossentropy,
    /// Categorical crossentropy (multi-class classification)
    CategoricalCrossentropy,
    /// Sparse categorical crossentropy
    SparseCategoricalCrossentropy,
    /// Huber loss (robust regression)
    Huber,
    /// Hinge loss (SVM-like)
    Hinge,
    /// KL divergence
    KlDivergence,
    /// Custom loss function
    Custom(String),
}

/// Connection types for skip connections
#[derive(Debug, Clone, Copy, Serialize, Deserialize, PartialEq, Eq)]
pub enum ConnectionType {
    /// Add connection (residual)
    Add,
    /// Concatenate connection
    Concatenate,
    /// Multiply connection
    Multiply,
}

/// Training parameters
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct TrainingParams {
    /// Batch size
    pub batch_size: u32,
    /// Number of epochs
    pub epochs: u32,
    /// Learning rate
    pub learning_rate: f64,
    /// Learning rate scheduler
    pub lr_scheduler: Option<LrScheduler>,
    /// Optimizer
    pub optimizer: Optimizer,
    /// Loss function
    pub loss_function: LossFunction,
    /// Metrics to track during training
    pub metrics: Vec<Metric>,
}

/// Optimizer configuration
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct Optimizer {
    /// Optimizer type
    pub optimizer_type: OptimizerType,
    /// Optimizer-specific parameters
    pub parameters: HashMap<String, f64>,
}

/// Learning rate scheduler
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct LrScheduler {
    /// Scheduler type
    pub scheduler_type: LrSchedulerType,
    /// Scheduler parameters
    pub parameters: HashMap<String, f64>,
}

/// Learning rate scheduler types
#[derive(Debug, Clone, Copy, Serialize, Deserialize, PartialEq, Eq)]
pub enum LrSchedulerType {
    /// Step decay (reduce LR at fixed intervals)
    StepDecay,
    /// Exponential decay
    ExponentialDecay,
    /// Cosine annealing
    CosineAnnealing,
    /// Reduce on plateau (adaptive)
    ReduceOnPlateau,
    /// Cyclic learning rate
    CyclicLr,
}

/// Training metrics
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
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct NetworkOptimization {
    /// Use mixed precision training (FP16/FP32)
    pub mixed_precision: bool,
    /// Gradient clipping configuration
    pub gradient_clipping: Option<GradientClipping>,
    /// Enable batch size optimization
    pub batch_size_optimization: bool,
    /// Enable memory optimization
    pub memory_optimization: bool,
}

/// Gradient clipping configuration
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct GradientClipping {
    /// Clipping type
    pub clip_type: ClipType,
    /// Clipping value
    pub clip_value: f64,
}

/// Gradient clipping types
#[derive(Debug, Clone, Copy, Serialize, Deserialize, PartialEq, Eq)]
pub enum ClipType {
    /// Clip by value
    Value,
    /// Clip by norm
    Norm,
    /// Clip by global norm
    GlobalNorm,
}

/// Network regularization techniques
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct NetworkRegularization {
    /// Dropout configuration
    pub dropout: Option<DropoutConfig>,
    /// Enable batch normalization
    pub batch_normalization: bool,
    /// Weight decay coefficient
    pub weight_decay: f64,
    /// Early stopping configuration
    pub early_stopping: Option<EarlyStoppingConfig>,
}

/// Dropout configuration
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct DropoutConfig {
    /// Dropout rate (0.0 to 1.0)
    pub rate: f64,
    /// Apply dropout during training only
    pub training_only: bool,
    /// Dropout schedule (if adaptive)
    pub schedule: Option<DropoutSchedule>,
}

/// Dropout schedule for adaptive dropout
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct DropoutSchedule {
    /// Initial dropout rate
    pub initial_rate: f64,
    /// Final dropout rate
    pub final_rate: f64,
    /// Schedule type
    pub schedule_type: ScheduleType,
}

/// Schedule types for dropout
#[derive(Debug, Clone, Copy, Serialize, Deserialize, PartialEq, Eq)]
pub enum ScheduleType {
    /// Linear schedule
    Linear,
    /// Exponential schedule
    Exponential,
    /// Step schedule
    Step,
}

/// Early stopping configuration
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct EarlyStoppingConfig {
    /// Metric to monitor (e.g., "val_loss")
    pub monitor: String,
    /// Minimum change to qualify as improvement
    pub min_delta: f64,
    /// Number of epochs with no improvement to wait
    pub patience: u32,
    /// Restore best weights on early stop
    pub restore_best_weights: bool,
    /// Monitoring mode
    pub mode: MonitoringMode,
}

/// Monitoring mode for early stopping
#[derive(Debug, Clone, Copy, Serialize, Deserialize, PartialEq, Eq)]
pub enum MonitoringMode {
    /// Minimize the monitored metric
    Min,
    /// Maximize the monitored metric
    Max,
    /// Auto-detect based on metric name
    Auto,
}

// ============================================================================
// LEARNING CONFIGURATION TYPES (Phase 2 - migrated from beardog-core)
// ============================================================================

/// Online learning configuration for continuous model updates
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct OnlineLearningConfig {
    /// Learning rate adaptation strategy
    pub learning_rate_adaptation: LearningRateAdaptationType,
    /// Batch size for online learning
    pub online_batch_size: u32,
    /// Memory buffer size for experience replay
    pub memory_buffer_size: u32,
    /// Update frequency for model updates
    pub update_frequency: UpdateFrequency,
}

/// Learning rate adaptation strategies
#[derive(Debug, Clone, Copy, Serialize, Deserialize, PartialEq, Eq)]
pub enum LearningRateAdaptationType {
    /// Fixed learning rate throughout training
    Fixed,
    /// Adaptive learning rate based on performance
    Adaptive,
    /// Decay-based learning rate schedule
    Decay,
    /// Performance-based adaptation
    PerformanceBased,
}

/// Update frequency for online learning
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub enum UpdateFrequency {
    /// Update after each sample
    PerSample,
    /// Update after each batch
    PerBatch,
    /// Update after each epoch
    PerEpoch,
    /// Update based on time interval
    TimeInterval(Duration),
    /// Update when performance threshold reached
    PerformanceThreshold(f64),
}

/// Transfer learning configuration
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct TransferLearningConfig {
    /// Source domain configuration
    pub source_domain: DomainConfig,
    /// Target domain configuration
    pub target_domain: DomainConfig,
    /// Transfer learning strategy
    pub transfer_strategy: TransferStrategy,
    /// Fine-tuning configuration
    pub fine_tuning: Option<FineTuningConfig>,
}

/// Domain configuration for transfer learning
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct DomainConfig {
    /// Domain identifier
    pub domain_id: String,
    /// Domain description
    pub description: String,
    /// Feature space configuration
    pub feature_space: FeatureSpaceConfig,
    /// Label space configuration
    pub label_space: LabelSpaceConfig,
}

/// Feature space configuration
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct FeatureSpaceConfig {
    /// Number of features
    pub num_features: u32,
    /// Feature types
    pub feature_types: Vec<FeatureType>,
    /// Normalization strategy
    pub normalization: NormalizationStrategy,
}

/// Feature types
#[derive(Debug, Clone, Copy, Serialize, Deserialize, PartialEq, Eq)]
pub enum FeatureType {
    /// Continuous numerical feature
    Continuous,
    /// Categorical feature
    Categorical,
    /// Binary feature
    Binary,
    /// Ordinal feature
    Ordinal,
    /// Text feature
    Text,
    /// Image feature
    Image,
}

/// Normalization strategies
#[derive(Debug, Clone, Copy, Serialize, Deserialize, PartialEq, Eq)]
pub enum NormalizationStrategy {
    /// Min-max normalization (0-1 range)
    MinMax,
    /// Z-score normalization (mean=0, std=1)
    ZScore,
    /// Robust scaling (median and IQR)
    Robust,
    /// Unit vector scaling
    UnitVector,
    /// No normalization
    None,
}

/// Label space configuration
#[derive(Debug, Clone, Copy, Serialize, Deserialize, PartialEq)]
pub struct LabelSpaceConfig {
    /// Number of labels
    pub num_labels: u32,
    /// Label type
    pub label_type: LabelType,
    /// Label encoding strategy
    pub encoding: LabelEncoding,
}

/// Label types
#[derive(Debug, Clone, Copy, Serialize, Deserialize, PartialEq, Eq)]
pub enum LabelType {
    /// Binary classification (2 classes)
    Binary,
    /// Multi-class classification (3+ classes)
    MultiClass,
    /// Multi-label classification (multiple labels per instance)
    MultiLabel,
    /// Regression (continuous output)
    Regression,
    /// Structured prediction (sequences, graphs)
    Structured,
}

/// Label encoding strategies
#[derive(Debug, Clone, Copy, Serialize, Deserialize, PartialEq, Eq)]
pub enum LabelEncoding {
    /// One-hot encoding
    OneHot,
    /// Label encoding (integer mapping)
    Label,
    /// Binary encoding
    Binary,
    /// Target encoding
    Target,
}

/// Transfer learning strategies
#[derive(Debug, Clone, Copy, Serialize, Deserialize, PartialEq, Eq)]
pub enum TransferStrategy {
    /// Use pre-trained model for feature extraction only
    FeatureExtraction,
    /// Fine-tune pre-trained model
    FineTuning,
    /// Domain adaptation techniques
    DomainAdaptation,
    /// Multi-task learning
    MultiTask,
    /// Few-shot learning
    FewShot,
    /// Zero-shot learning
    ZeroShot,
}

/// Fine-tuning configuration
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct FineTuningConfig {
    /// Layers to freeze (by index)
    pub frozen_layers: Vec<u32>,
    /// Learning rate for fine-tuning
    pub fine_tune_learning_rate: f64,
    /// Number of fine-tuning epochs
    pub fine_tune_epochs: u32,
    /// Enable gradual unfreezing of layers
    pub gradual_unfreezing: bool,
}

/// Meta-learning configuration (learning to learn)
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct MetaLearningConfig {
    /// Meta-learning algorithm
    pub algorithm: MetaLearningAlgorithm,
    /// Number of meta-training tasks
    pub num_meta_train_tasks: u32,
    /// Number of shots per task (examples per class)
    pub num_shots: u32,
    /// Inner loop configuration
    pub inner_loop: InnerLoopConfig,
    /// Outer loop configuration
    pub outer_loop: OuterLoopConfig,
}

/// Meta-learning algorithms
#[derive(Debug, Clone, Copy, Serialize, Deserialize, PartialEq, Eq)]
pub enum MetaLearningAlgorithm {
    /// Model-Agnostic Meta-Learning
    Maml,
    /// First-Order MAML
    Fomaml,
    /// Reptile
    Reptile,
    /// Prototypical Networks
    ProtoNet,
    /// Matching Networks
    MatchingNet,
    /// Relation Networks
    RelationNet,
}

/// Inner loop configuration for meta-learning
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct InnerLoopConfig {
    /// Inner loop learning rate
    pub learning_rate: f64,
    /// Number of inner gradient steps
    pub num_steps: u32,
    /// Inner loop optimizer
    pub optimizer: OptimizerType,
}

/// Outer loop configuration for meta-learning
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct OuterLoopConfig {
    /// Outer loop learning rate
    pub learning_rate: f64,
    /// Number of outer gradient steps
    pub num_steps: u32,
    /// Outer loop optimizer
    pub optimizer: OptimizerType,
}

/// Ensemble learning configuration
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct EnsembleConfigLearning {
    /// Base models in the ensemble
    pub base_models: Vec<BaseModelLearning>,
    /// Ensemble combination method
    pub ensemble_method: EnsembleMethodType,
    /// Model weights (if weighted voting/averaging)
    pub model_weights: Option<HashMap<String, f64>>,
    /// Diversity measures to track
    pub diversity_measures: Vec<DiversityMeasure>,
}

/// Base model configuration for ensembles
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct BaseModelLearning {
    /// Model identifier
    pub id: String,
    /// Model type
    pub model_type: ModelTypeLearning,
    /// Model-specific configuration
    pub configuration: HashMap<String, serde_json::Value>,
    /// Model weight in ensemble
    pub weight: f64,
}

/// Model types
#[derive(Debug, Clone, Copy, Serialize, Deserialize, PartialEq, Eq)]
pub enum ModelTypeLearning {
    /// Linear model
    Linear,
    /// Logistic regression
    Logistic,
    /// Decision tree
    DecisionTree,
    /// Random forest
    RandomForest,
    /// Support vector machine
    SVM,
    /// Neural network
    NeuralNetwork,
    /// Ensemble model
    Ensemble,
}

/// Ensemble combination methods
#[derive(Debug, Clone, Copy, Serialize, Deserialize, PartialEq, Eq)]
pub enum EnsembleMethodType {
    /// Simple majority voting
    Voting,
    /// Weighted voting
    WeightedVoting,
    /// Stacking (meta-learner)
    Stacking,
    /// Blending
    Blending,
    /// Bagging (Bootstrap Aggregating)
    Bagging,
    /// Boosting
    Boosting,
    /// Random Forest
    RandomForest,
}

/// Diversity measures for ensemble models
#[derive(Debug, Clone, Copy, Serialize, Deserialize, PartialEq, Eq)]
pub enum DiversityMeasure {
    /// Disagreement measure
    Disagreement,
    /// Double-fault measure
    DoubleFault,
    /// Kohavi-Wolpert variance
    KohaviWolpert,
    /// Inter-rater agreement
    InterRater,
    /// Entropy measure
    Entropy,
}

/// Hyperparameter optimization configuration
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct HyperparameterOptimizationConfig {
    /// Optimization method
    pub method: HyperparameterOptimizationMethod,
    /// Maximum number of trials
    pub max_trials: u32,
    /// Optimization timeout in seconds
    pub timeout_secs: u64,
    /// Objective metric to optimize
    pub objective_metric: String,
    /// Optimization direction
    pub optimization_direction: OptimizationDirectionType,
}

/// Hyperparameter optimization methods
#[derive(Debug, Clone, Copy, Serialize, Deserialize, PartialEq, Eq)]
pub enum HyperparameterOptimizationMethod {
    /// Random search
    RandomSearch,
    /// Grid search
    GridSearch,
    /// Bayesian optimization
    BayesianOptimization,
    /// Hyperband
    Hyperband,
    /// Population-based training
    PopulationBasedTraining,
    /// Optuna framework
    Optuna,
}

/// Optimization direction
#[derive(Debug, Clone, Copy, Serialize, Deserialize, PartialEq, Eq)]
pub enum OptimizationDirectionType {
    /// Minimize the objective
    Minimize,
    /// Maximize the objective
    Maximize,
}

/// Prediction horizon for time-series forecasting
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
pub enum PredictionHorizon {
    /// Short-term prediction
    ShortTerm,
    /// Medium-term prediction
    MediumTerm,
    /// Long-term prediction
    LongTerm,
    /// Custom horizon duration
    Custom(Duration),
}

/// Learning algorithm types
#[derive(Debug, Clone, Copy, Serialize, Deserialize, PartialEq, Eq)]
pub enum LearningAlgorithmType {
    /// Supervised learning
    Supervised,
    /// Unsupervised learning
    Unsupervised,
    /// Reinforcement learning
    Reinforcement,
    /// Semi-supervised learning
    SemiSupervised,
    /// Transfer learning
    Transfer,
    /// Meta learning
    Meta,
    /// Online learning
    Online,
    /// Federated learning
    Federated,
}

/// Validation strategies
#[derive(Debug, Clone, Copy, Serialize, Deserialize, PartialEq, Eq)]
pub enum ValidationStrategy {
    /// Hold-out validation
    HoldOut,
    /// K-fold cross-validation
    CrossValidation,
    /// Time series cross-validation
    TimeSeriesCrossValidation,
    /// Bootstrap validation
    Bootstrap,
    /// Monte Carlo cross-validation
    MonteCarlo,
}

// ============================================================================
// Default implementations for learning configuration types
// ============================================================================

impl Default for OnlineLearningConfig {
    fn default() -> Self {
        Self {
            learning_rate_adaptation: LearningRateAdaptationType::Fixed,
            online_batch_size: 32,
            memory_buffer_size: 1000,
            update_frequency: UpdateFrequency::PerBatch,
        }
    }
}

// ============================================================================
// Default implementations for detailed neural network types
// ============================================================================

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

impl Default for Optimizer {
    fn default() -> Self {
        Self {
            optimizer_type: OptimizerType::Adam,
            parameters: HashMap::new(),
        }
    }
}

/// **DECISION ENGINE CONFIGURATION** - AI decision making settings
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct DecisionEngineConfig {
    /// Enable decision engine
    pub enabled: bool,
    
    /// Decision strategies
    pub strategies: Vec<DecisionStrategy>,
    
    /// Confidence thresholds for different decision types
    pub confidence_thresholds: HashMap<String, f64>,
    
    /// Enable explainable AI
    pub enable_explainability: bool,
    
    /// Decision audit trail settings
    pub audit_trail: AuditTrailConfig,
}

/// **MODEL MANAGEMENT CONFIGURATION** - Model lifecycle management
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct ModelManagementConfig {
    /// Model registry endpoint
    pub registry_endpoint: String,
    
    /// Model versioning strategy
    pub versioning_strategy: VersioningStrategy,
    
    /// Automatic model deployment
    pub auto_deployment: bool,
    
    /// Model performance monitoring
    pub performance_monitoring: bool,
    
    /// Model drift detection
    pub drift_detection: DriftDetectionConfig,
    
    /// Model rollback settings
    pub rollback: RollbackConfig,
}

/// **AI PERFORMANCE CONFIGURATION** - Performance optimization settings
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct AiPerformanceConfig {
    /// Enable performance optimization
    pub enabled: bool,
    
    /// CPU thread pool size
    pub cpu_threads: usize,
    
    /// Memory limit for AI operations
    pub memory_limit_mb: usize,
    
    /// Enable mixed precision training
    pub mixed_precision: bool,
    
    /// Enable model quantization
    pub quantization: bool,
    
    /// Performance profiling settings
    pub profiling: ProfilingConfig,
}

/// **AI SECURITY CONFIGURATION** - AI security and privacy settings
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct AiSecurityConfig {
    /// Enable AI security features
    pub enabled: bool,
    
    /// Data privacy settings
    pub privacy: PrivacyConfig,
    
    /// Model security settings
    pub model_security: ModelSecurityConfig,
    
    /// Adversarial attack protection
    pub adversarial_protection: bool,
    
    /// Differential privacy settings
    pub differential_privacy: DifferentialPrivacyConfig,
}

// Supporting types and enums

/// Neural network architecture types
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub enum NetworkArchitecture {
    /// Dense/fully connected layers
    Dense,
    /// Convolutional neural network
    Convolutional,
    /// Recurrent neural network (RNN/LSTM/GRU)
    Recurrent,
    /// Transformer architecture
    Transformer,
    /// Custom architecture specification
    Custom(String),
}

/// Activation function types for neural networks
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub enum ActivationFunction {
    /// Rectified Linear Unit
    ReLU,
    /// Sigmoid activation function
    Sigmoid,
    /// Hyperbolic tangent
    Tanh,
    /// Softmax activation function
    Softmax,
    /// Leaky Rectified Linear Unit
    LeakyReLU,
    /// Custom activation function
    Custom(String),
}

/// Regularization configuration for neural networks
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct RegularizationConfig {
    /// L1 regularization parameter
    pub l1_lambda: f64,
    /// L2 regularization parameter
    pub l2_lambda: f64,
    /// Whether to enable batch normalization
    pub enable_batch_norm: bool,
}

/// Configuration for neural network optimizers
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct OptimizerConfig {
    /// The type of optimizer to use
    pub optimizer_type: OptimizerType,
    /// Learning rate for gradient descent
    pub learning_rate: f64,
    /// Momentum factor for momentum-based optimizers
    pub momentum: f64,
    /// Weight decay coefficient for regularization
    pub weight_decay: f64,
}

/// Supported optimizer algorithms for neural network training
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub enum OptimizerType {
    /// Stochastic Gradient Descent
    SGD,
    /// Adaptive Moment Estimation
    Adam,
    /// Adam with decoupled weight decay
    AdamW,
    /// Root Mean Square Propagation
    RMSprop,
    /// Custom optimizer implementation
    Custom(String),
}

/// Decision-making strategy for AI systems
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub enum DecisionStrategy {
    /// Greedy algorithm - always chooses the locally optimal choice
    Greedy,
    /// Probabilistic approach using weighted random selection
    Probabilistic,
    /// Ensemble method combining multiple strategies
    Ensemble,
    /// Human-in-the-loop decision making
    HumanInTheLoop,
    /// Custom strategy with user-defined logic
    Custom(String),
}

/// Configuration for AI decision audit trails
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct AuditTrailConfig {
    /// Whether audit trail logging is enabled
    pub enabled: bool,
    /// Number of days to retain audit trail data
    pub retention_days: u32,
    /// Whether to include intermediate decision steps in the audit trail
    pub include_intermediate_steps: bool,
}

/// Versioning strategy for AI models and configurations
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub enum VersioningStrategy {
    /// Semantic versioning (major.minor.patch)
    Semantic,
    /// Timestamp-based versioning
    Timestamp,
    /// Hash-based versioning using content hash
    Hash,
    /// Custom versioning scheme
    Custom(String),
}

/// Configuration for AI model drift detection
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct DriftDetectionConfig {
    /// Whether drift detection is enabled
    pub enabled: bool,
    /// Time window for drift detection analysis
    pub detection_window: Duration,
    /// Threshold value for detecting significant drift
    pub threshold: f64,
    /// Whether to send alerts when drift is detected
    pub alert_on_drift: bool,
}

/// Configuration for AI model rollback functionality
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct RollbackConfig {
    /// Whether rollback functionality is enabled
    pub enabled: bool,
    /// Whether to automatically rollback on performance degradation
    pub automatic_rollback: bool,
    /// Performance threshold below which to trigger rollback
    pub rollback_threshold: f64,
    /// Maximum number of previous versions to keep for rollback
    pub max_rollback_versions: u32,
}

/// Configuration for AI performance profiling
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct ProfilingConfig {
    /// Whether performance profiling is enabled
    pub enabled: bool,
    /// Sampling rate for profiling (0.0 to 1.0)
    pub sample_rate: f64,
    /// Whether to include memory profiling in addition to CPU profiling
    pub include_memory_profiling: bool,
    /// Whether to include GPU profiling for AI workloads
    pub include_gpu_profiling: bool,
}

/// Configuration for AI privacy and data protection
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct PrivacyConfig {
    /// Whether privacy protection features are enabled
    pub enabled: bool,
    /// Level of data anonymization to apply
    pub anonymization_level: AnonymizationLevel,
    /// Number of days to retain data before automatic deletion
    pub data_retention_days: u32,
    /// Whether to enable right-to-be-forgotten functionality
    pub enable_right_to_be_forgotten: bool,
}

/// Level of data anonymization for privacy protection
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub enum AnonymizationLevel {
    /// No anonymization applied
    None,
    /// Basic anonymization (remove direct identifiers)
    Basic,
    /// Advanced anonymization (statistical disclosure control)
    Advanced,
    /// Full anonymization (differential privacy)
    Full,
}

/// Configuration for AI model security features
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct ModelSecurityConfig {
    /// Whether to encrypt AI models at rest and in transit
    pub enable_model_encryption: bool,
    /// Whether to use secure inference techniques
    pub enable_secure_inference: bool,
    /// Whether to use trusted execution environments for AI processing
    pub trusted_execution_environment: bool,
}

/// Configuration for differential privacy in AI systems
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct DifferentialPrivacyConfig {
    /// Whether differential privacy is enabled
    pub enabled: bool,
    /// Privacy budget parameter (smaller = more private)
    pub epsilon: f64,
    /// Failure probability parameter for differential privacy
    pub delta: f64,
    /// Noise mechanism to use for differential privacy
    pub noise_mechanism: NoiseMechanism,
}

/// Noise mechanisms for differential privacy
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub enum NoiseMechanism {
    /// Laplace noise mechanism
    Laplace,
    /// Gaussian noise mechanism  
    Gaussian,
    /// Exponential noise mechanism
    Exponential,
}

// Default implementations

impl Default for HybridIntelligenceConfig {
    fn default() -> Self {
        Self {
            enabled: false,
            human_oversight_level: 0.5,
            auto_decision_threshold: 0.8,
            feedback_learning: true,
            human_input_timeout: Duration::from_secs(30),
        }
    }
}

impl Default for TrainingConfig {
    fn default() -> Self {
        Self {
            enabled: false,
            data_sources: vec![],
            batch_size: 32,
            learning_rate: 0.001,
            epochs: 100,
            validation_split: 0.2,
            early_stopping_patience: 10,
            checkpoint_frequency: 10,
        }
    }
}

impl Default for InferenceConfig {
    fn default() -> Self {
        Self {
            enabled: false,
            endpoints: vec![],
            max_batch_size: 64,
            timeout: Duration::from_secs(30),
            enable_caching: true,
            cache_size_limit: 1000,
            enable_gpu: false,
        }
    }
}

impl Default for NeuralNetworkConfig {
    fn default() -> Self {
        Self {
            architecture: NetworkArchitecture::Dense,
            hidden_layers: vec![128, 64],
            activation: ActivationFunction::ReLU,
            dropout_rate: 0.1,
            regularization: RegularizationConfig::default(),
            optimizer: OptimizerConfig::default(),
            detailed_architecture: None,
        }
    }
}

impl Default for DecisionEngineConfig {
    fn default() -> Self {
        Self {
            enabled: false,
            strategies: vec![DecisionStrategy::Greedy],
            confidence_thresholds: HashMap::new(),
            enable_explainability: true,
            audit_trail: AuditTrailConfig::default(),
        }
    }
}

impl Default for ModelManagementConfig {
    fn default() -> Self {
        use crate::constants::domains::network::defaults::DEFAULT_API_PORT;
        Self {
            registry_endpoint: format!("http://localhost:{DEFAULT_API_PORT}/models"),
            versioning_strategy: VersioningStrategy::Semantic,
            auto_deployment: false,
            performance_monitoring: true,
            drift_detection: DriftDetectionConfig::default(),
            rollback: RollbackConfig::default(),
        }
    }
}

impl Default for AiPerformanceConfig {
    fn default() -> Self {
        Self {
            enabled: true,
            cpu_threads: 4,
            memory_limit_mb: 2048,
            mixed_precision: false,
            quantization: false,
            profiling: ProfilingConfig::default(),
        }
    }
}

impl Default for AiSecurityConfig {
    fn default() -> Self {
        Self {
            enabled: true,
            privacy: PrivacyConfig::default(),
            model_security: ModelSecurityConfig::default(),
            adversarial_protection: true,
            differential_privacy: DifferentialPrivacyConfig::default(),
        }
    }
}

impl Default for RegularizationConfig {
    fn default() -> Self {
        Self {
            l1_lambda: 0.0,
            l2_lambda: 0.001,
            enable_batch_norm: true,
        }
    }
}

impl Default for OptimizerConfig {
    fn default() -> Self {
        Self {
            optimizer_type: OptimizerType::Adam,
            learning_rate: 0.001,
            momentum: 0.9,
            weight_decay: 0.0001,
        }
    }
}

impl Default for AuditTrailConfig {
    fn default() -> Self {
        Self {
            enabled: true,
            retention_days: 90,
            include_intermediate_steps: false,
        }
    }
}

impl Default for DriftDetectionConfig {
    fn default() -> Self {
        Self {
            enabled: true,
            detection_window: Duration::from_secs(24 * 60 * 60),
            threshold: 0.1,
            alert_on_drift: true,
        }
    }
}

impl Default for RollbackConfig {
    fn default() -> Self {
        Self {
            enabled: true,
            automatic_rollback: false,
            rollback_threshold: 0.05,
            max_rollback_versions: 5,
        }
    }
}

impl Default for ProfilingConfig {
    fn default() -> Self {
        Self {
            enabled: false,
            sample_rate: 0.1,
            include_memory_profiling: true,
            include_gpu_profiling: false,
        }
    }
}

impl Default for PrivacyConfig {
    fn default() -> Self {
        Self {
            enabled: true,
            anonymization_level: AnonymizationLevel::Basic,
            data_retention_days: 365,
            enable_right_to_be_forgotten: true,
        }
    }
}

impl Default for ModelSecurityConfig {
    fn default() -> Self {
        Self {
            enable_model_encryption: true,
            enable_secure_inference: false,
            trusted_execution_environment: false,
        }
    }
}

impl Default for DifferentialPrivacyConfig {
    fn default() -> Self {
        Self {
            enabled: false,
            epsilon: 1.0,
            delta: 1e-5,
            noise_mechanism: NoiseMechanism::Gaussian,
        }
    }
}

// BearDogConfig implementation for ConsolidatedAiConfig
impl BearDogConfig for ConsolidatedAiConfig {
    fn validate(&self) -> BearDogResult<()> {
        if self.enabled {
            if self.hybrid_intelligence.human_oversight_level < 0.0 || self.hybrid_intelligence.human_oversight_level > 1.0 {
                return Err(BearDogError::validation("Human oversight level must be between 0.0 and 1.0"));
            }
            
            if self.training.learning_rate <= 0.0 {
                return Err(BearDogError::validation("Learning rate must be positive"));
            }
            
            if self.training.batch_size == 0 {
                return Err(BearDogError::validation("Batch size must be greater than 0"));
            }
        }
        Ok(())
    }
    
    fn merge(&self, other: &Self) -> BearDogResult<Self> {
        Ok(Self {
            enabled: other.enabled,
            hybrid_intelligence: if other.hybrid_intelligence.enabled { 
                other.hybrid_intelligence.clone() 
            } else { 
                self.hybrid_intelligence.clone() 
            },
            training: if other.training.enabled { 
                other.training.clone() 
            } else { 
                self.training.clone() 
            },
            inference: if other.inference.enabled { 
                other.inference.clone() 
            } else { 
                self.inference.clone() 
            },
            neural_networks: other.neural_networks.clone(),
            decision_engine: if other.decision_engine.enabled { 
                other.decision_engine.clone() 
            } else { 
                self.decision_engine.clone() 
            },
            model_management: other.model_management.clone(),
            performance: if other.performance.enabled { 
                other.performance.clone() 
            } else { 
                self.performance.clone() 
            },
            security: if other.security.enabled { 
                other.security.clone() 
            } else { 
                self.security.clone() 
            },
        })
    }
    
    fn from_env() -> BearDogResult<Self> {
        let mut config = Self::default();
        
        if let Ok(enabled) = std::env::var("BEARDOG_AI_ENABLED") {
            config.enabled = enabled.parse().unwrap_or(false);
        }
        
        if let Ok(oversight) = std::env::var("BEARDOG_AI_HUMAN_OVERSIGHT") {
            config.hybrid_intelligence.human_oversight_level = oversight.parse().unwrap_or(0.5);
        }
        
        if let Ok(lr) = std::env::var("BEARDOG_AI_LEARNING_RATE") {
            config.training.learning_rate = lr.parse().unwrap_or(0.001);
        }
        
        config.validate()?;
        Ok(config)
    }
    
    fn to_toml(&self) -> BearDogResult<String> {
        toml::to_string(self)
            .map_err(|e| BearDogError::system(format!("Failed to serialize AI config to TOML: {e}")))
    }
    
    fn domain() -> &'static str {
        "ai"
    }
} 