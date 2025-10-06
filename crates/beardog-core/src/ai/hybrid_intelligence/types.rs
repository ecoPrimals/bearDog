//! Configuration types for the hybrid intelligence system

use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use std::time::Duration;

// Import core types from the new module
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

/// Training configuration
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct TrainingConfig {
    /// Number of `batch_size`
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
            early_stopping: None,
            regularization: None,
            optimizer: OptimizerConfig::default(),
        }
    }
}

/// Inference configuration
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct InferenceConfig {
    /// Number of `batch_size`
    pub batch_size: u32,
    /// Maximum inference time in milliseconds
    pub max_inference_time_ms: u64,
    /// Model serving configuration
    pub serving_config: ServingConfig,
    /// Caching configuration
    /// Optional caching
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

/// Model management configuration
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ModelManagementConfig {
    /// Model versioning strategy
    /// The versioning strategy value
    pub versioning_strategy: VersioningStrategy,
    /// Model registry configuration
    pub registry_config: AIRegistryConfig,
    /// Model deployment configuration
    pub deployment_config: DeploymentConfig,
    /// Model monitoring configuration
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

/// Data preprocessing configuration
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PreprocessingConfig {
    /// Normalization strategy
    /// The normalization value
    pub normalization: NormalizationStrategy,
    /// Feature selection configuration
    /// Optional feature selection
    pub feature_selection: Option<FeatureSelectionConfig>,
    /// Data augmentation configuration
    /// Optional data augmentation
    pub data_augmentation: Option<DataAugmentationConfig>,
    /// Missing value handling
    /// The missing value handling value
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

/// Neural network configuration
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct NeuralNetworkConfig {
    /// Network architecture
    /// The architecture value
    pub architecture: NetworkArchitecture,
    /// Training hyperparameters
    /// The training params value
    pub training_params: TrainingParams,
    /// Network optimization settings
    /// The optimization value
    pub optimization: NetworkOptimization,
    /// Regularization techniques
    /// The regularization value
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

/// Decision engine configuration
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct DecisionEngineConfig {
    /// Decision strategies to use
    /// Collection of strategies
    pub strategies: Vec<DecisionStrategy>,
    /// Decision criteria
    /// The criteria value
    pub criteria: DecisionCriteria,
    /// The consensus mechanism value
    pub consensus_mechanism: ConsensusStrategy,
    /// Decision timeout
    pub timeout: Duration,
}

impl Default for DecisionEngineConfig {
    fn default() -> Self {
        use crate::ai::hybrid_intelligence::decision_engine::{
            ConsensusStrategy, DecisionCriteria, DecisionStrategy,
        };
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

/// Learning configuration
#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct LearningConfig {
    /// Learning algorithms to use
    /// Collection of algorithms
    pub algorithms: Vec<LearningAlgorithmType>,
    /// Online learning configuration
    /// The online learning value
    pub online_learning: OnlineLearningConfig,
    /// Transfer learning configuration
    /// Optional transfer learning
    pub transfer_learning: Option<TransferLearningConfig>,
    /// Meta-learning configuration
    /// Optional meta learning
    pub meta_learning: Option<MetaLearningConfig>,
}

// Types moved to avoid duplication - using existing definitions

/// Prediction configuration
#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct PredictionConfig {
    /// Prediction models to use
    /// Collection of models
    pub models: Vec<PredictionModel>,
    /// Ensemble configuration
    /// Optional ensemble
    pub ensemble: Option<EnsembleConfig>,
    /// Uncertainty quantification
    /// Whether `uncertainty_quantification` is enabled
    pub uncertainty_quantification: bool,
    /// Prediction horizons
    /// Collection of horizons
    pub horizons: Vec<PredictionHorizon>,
}

/// Optimization configuration
#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct OptimizationConfig {
    /// Optimization algorithms
    /// Collection of algorithms
    pub algorithms: Vec<OptimizationAlgorithm>,
    /// Multi-objective optimization
    /// Whether `multi_objective` is enabled
    pub multi_objective: bool,
    /// Constraint handling
    /// Optional constraints
    pub constraints: Option<ConstraintConfig>,
    /// Hyperparameter optimization
    /// Optional hyperparameter optimization
    pub hyperparameter_optimization: Option<HyperparameterOptimization>,
}

// Optimization types moved to avoid duplication - using existing definitions

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
}

#[derive(Debug, Clone, Copy, Serialize, Deserialize)]
pub struct RegularizationConfig {
    /// L1 regularization strength
    /// The l1 value
    pub l1: f64,
    /// L2 regularization strength
    /// The l2 value
    pub l2: f64,
    /// Dropout rate
    /// The dropout value
    pub dropout: f64,
    /// Batch normalization
    /// Whether `batch_normalization` is enabled
    pub batch_normalization: bool,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct OptimizerConfig {
    /// Optimizer type
    /// The optimizer type value
    pub optimizer_type: OptimizerType,
    /// Learning rate
    /// The learning rate value
    pub learning_rate: f64,
    /// Learning rate schedule
    /// Optional learning rate schedule
    pub learning_rate_schedule: Option<LearningRateSchedule>,
    /// Weight decay
    /// The weight decay value
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

/// Optimizer types with configuration parameters
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
/// Types of optimizer
pub enum OptimizerType {
    /// Stochastic Gradient Descent
    Sgd {
        /// Momentum parameter
        momentum: f64,
    },
    /// Adam optimizer with adaptive learning rates
    Adam {
        beta1: f64,
        beta2: f64,
        /// Small epsilon value to prevent division by zero
        epsilon: f64,
    },
    /// `AdamW` optimizer with weight decay
    AdamW {
        beta1: f64,
        beta2: f64,
        /// Small epsilon value to prevent division by zero
        epsilon: f64,
    },
    /// `RMSprop` optimizer with moving average of squared gradients
    RmsProp {
        alpha: f64,
        /// Small epsilon value to prevent division by zero
        epsilon: f64,
    },
    /// Adagrad optimizer with accumulated squared gradients
    Adagrad {
        /// Small epsilon value to prevent division by zero
        epsilon: f64,
    },
}

#[derive(Debug, Clone, Copy, Serialize, Deserialize)]
pub enum LearningRateSchedule {
    /// Exponential decay schedule
    ExponentialDecay {
        /// Rate at which learning rate decays
        decay_rate: f64,
        /// Number of steps between decay applications
        decay_steps: u32,
    },
    /// Step decay schedule with periodic drops
    StepDecay {
        /// Factor by which to multiply learning rate at each drop
        drop_rate: f64,
        /// Number of epochs between each learning rate drop
        epochs_drop: u32,
    },
    /// Cosine annealing schedule
    CosineAnnealing {
        t_max: u32,
        /// Minimum learning rate value
        eta_min: f64,
    },
    /// Reduce learning rate when metric plateaus
    ReduceOnPlateau {
        /// Factor by which to reduce learning rate
        factor: f64,
        patience: u32,
    },
}

/// Serving configuration
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ServingConfig {
    /// Maximum concurrent requests
    /// Number of `max_concurrent_requests`
    pub max_concurrent_requests: u32,
    /// Request timeout
    pub request_timeout: Duration,
    /// Load balancing strategy
    /// The load balancing value
    pub load_balancing: LoadBalancingStrategy,
}

impl Default for ServingConfig {
    fn default() -> Self {
        Self {
            max_concurrent_requests: 100,
            request_timeout: Duration::from_secs(30),
            load_balancing: LoadBalancingStrategy::RoundRobin,
        }
    }
}

/// Caching configuration
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CachingConfig {
    /// Cache size limit
    /// Number of `max_cache_size`
    pub max_cache_size: u64,
    /// Cache TTL
    /// The ttl value
    pub ttl: Duration,
    /// Cache eviction policy
    /// The eviction policy value
    pub eviction_policy: EvictionPolicy,
}

#[derive(Debug, Clone, Copy, Serialize, Deserialize)]
pub enum VersioningStrategy {
    /// Semantic versioning
    Semantic,
    /// Timestamp-based versioning
    Timestamp,
    /// Hash-based versioning
    Hash,
    /// Incremental versioning
    Incremental,
}

/// Registry configuration for AI systems
/// Renamed from `RegistryConfig` to `AIRegistryConfig` for clarity
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct AIRegistryConfig {
    /// Registry type
    /// The registry type value
    pub registry_type: RegistryType,
    /// Registry endpoint
    /// The endpoint value
    pub endpoint: String,
    /// Authentication configuration
    /// Optional auth
    pub auth: Option<AuthConfig>,
}

/// Backward compatibility alias
#[deprecated(since = "3.2.0", note = "Use AIRegistryConfig instead")]
pub type RegistryConfig = AIRegistryConfig;

impl Default for AIRegistryConfig {
    fn default() -> Self {
        Self {
            registry_type: RegistryType::Local,
            endpoint: "localhost:8080".to_string(),
            auth: None,
        }
    }
}

#[derive(Debug, Clone, Copy, Serialize, Deserialize)]
/// Types of registry
pub enum RegistryType {
    /// Local file system registry
    Local,
    /// Remote HTTP registry
    Remote,
    /// Database registry
    Database,
    /// Cloud storage registry
    CloudStorage,
}

/// Deployment configuration
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct DeploymentConfig {
    /// Deployment strategy
    /// The strategy value
    pub strategy: DeploymentStrategy,
    /// Resource requirements
    /// The resources value
    pub resources: ResourceRequirements,
    /// Health check configuration
    /// The health check value
    pub health_check:
        beardog_types::canonical::config::domains::network::monitoring::HealthCheckConfiguration,
}

impl Default for DeploymentConfig {
    fn default() -> Self {
        Self {
            strategy: DeploymentStrategy::Rolling,
            resources: ResourceRequirements::default(),
            health_check: beardog_types::canonical::config::domains::network::monitoring::HealthCheckConfiguration::default(),
        }
    }
}

/// AI-specific monitoring configuration
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct AIMonitoringConfig {
    /// Training metrics collection
    pub collect_training_metrics: bool,
    /// Inference metrics collection
    pub collect_inference_metrics: bool,
    /// Model performance tracking
    pub track_model_performance: bool,
    /// Resource usage monitoring
    pub monitor_resource_usage: bool,
}

impl Default for AIMonitoringConfig {
    fn default() -> Self {
        Self {
            collect_training_metrics: true,
            collect_inference_metrics: true,
            track_model_performance: true,
            monitor_resource_usage: true,
        }
    }
}

// Backward compatibility alias
#[deprecated(since = "3.1.0", note = "Use AIMonitoringConfig instead")]
pub type MonitoringConfig = AIMonitoringConfig;

/// Normalization strategies
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
pub enum NormalizationStrategy {
    /// Min-max normalization
    MinMax,
    /// Z-score normalization
    ZScore,
    /// Robust scaling
    Robust,
    /// Unit vector scaling
    UnitVector,
    /// No normalization
    None,
}

/// Feature selection configuration
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct FeatureSelectionConfig {
    /// Selection method
    /// The method value
    pub method: FeatureSelectionMethod,
    /// Number of features to select
    /// Optional n features
    pub n_features: Option<u32>,
    /// Selection threshold
    /// Optional threshold
    pub threshold: Option<f64>,
}

/// Feature selection methods
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
pub enum FeatureSelectionMethod {
    /// Univariate selection
    Univariate,
    /// Recursive feature elimination
    RecursiveElimination,
    /// L1-based selection
    L1Based,
    /// Tree-based selection
    TreeBased,
}

/// Data augmentation configuration
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct DataAugmentationConfig {
    /// Augmentation techniques
    /// Collection of techniques
    pub techniques: Vec<AugmentationTechnique>,
    /// Augmentation probability
    /// The probability value
    pub probability: f64,
    /// Augmentation parameters
    /// Mapping of parameters
    pub parameters: HashMap<String, f64>,
}

/// Augmentation techniques
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
pub enum AugmentationTechnique {
    /// Rotation
    Rotation,
    /// Translation
    Translation,
    /// Scaling
    Scaling,
    /// Flipping
    Flipping,
    /// Noise injection
    NoiseInjection,
    /// Cropping
    Cropping,
}

/// Missing value strategies
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
pub enum MissingValueStrategy {
    /// Remove rows with missing values
    DropRows,
    /// Remove columns with missing values
    DropColumns,
    /// Fill with mean
    FillMean,
    /// Fill with median
    FillMedian,
    /// Fill with mode
    FillMode,
    /// Forward fill
    ForwardFill,
    /// Backward fill
    BackwardFill,
    /// Interpolation
    Interpolate,
}

/// Load balancing strategies
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
pub enum LoadBalancingStrategy {
    /// Round robin
    RoundRobin,
    /// Least connections
    LeastConnections,
    /// Weighted round robin
    WeightedRoundRobin,
    /// Random
    Random,
}

/// Cache eviction policies
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
pub enum EvictionPolicy {
    /// Least recently used
    Lru,
    /// Least frequently used
    Lfu,
    /// First in, first out
    Fifo,
    /// Time-based expiration
    Ttl,
}

/// Authentication configuration
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct AuthConfig {
    /// Authentication type
    /// The auth type value
    pub auth_type: AuthType,
    /// Credentials
    /// Mapping of credentials
    pub credentials: HashMap<String, String>,
}

/// Authentication types
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
/// Types of auth
pub enum AuthType {
    /// No authentication
    None,
    /// API key authentication
    ApiKey,
    /// Bearer token authentication
    BearerToken,
    /// Basic authentication
    Basic,
    /// `OAuth2` authentication
    OAuth2,
}

/// Deployment strategies
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
pub enum DeploymentStrategy {
    /// Blue-green deployment
    BlueGreen,
    /// Canary deployment
    Canary,
    /// Rolling deployment
    Rolling,
    /// Recreate deployment
    Recreate,
}

/// Resource requirements
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ResourceRequirements {
    /// CPU requirements
    /// The cpu value
    pub cpu: f64,
    /// Memory requirements in MB
    /// Number of memory
    pub memory: u64,
    /// GPU requirements
    /// Optional gpu
    pub gpu: Option<u32>,
    /// Storage requirements in GB
    /// Number of storage
    pub storage: u64,
}

impl Default for ResourceRequirements {
    fn default() -> Self {
        Self {
            cpu: 1.0,
            memory: 1024,
            gpu: None,
            storage: 10,
        }
    }
}

/// Health check configuration
///
/// **DEPRECATED**: Use `beardog_types::canonical::config::domains::network::monitoring::HealthCheckConfiguration` instead.
#[deprecated(
    since = "3.1.0",
    note = "Use beardog_types::canonical::config::domains::network::monitoring::HealthCheckConfiguration instead"
)]
pub type HealthCheckConfig =
    beardog_types::canonical::config::domains::network::monitoring::HealthCheckConfiguration;

/// Metric types
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
/// Types of metric
pub enum MetricType {
    /// Request latency
    RequestLatency,
    /// Request throughput
    RequestThroughput,
    /// Error rate
    ErrorRate,
    /// Model accuracy
    ModelAccuracy,
    /// Resource utilization
    ResourceUtilization,
    /// Prediction confidence
    PredictionConfidence,
}

/// Alerting configuration
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct AlertingConfig {
    /// Alert rules
    /// Collection of rules
    pub rules: Vec<AlertRule>,
    /// Notification channels
    /// Collection of channels
    pub channels: Vec<NotificationChannel>,
}

/// Alert rule
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct AlertRule {
    /// Rule name
    /// Name of the item
    pub name: String,
    /// Metric to monitor
    /// The metric value
    pub metric: MetricType,
    /// Threshold value
    /// The threshold value
    pub threshold: f64,
    /// Comparison operator
    /// The operator value
    pub operator: ComparisonOperator,
    /// Evaluation window
    /// The window value
    pub window: Duration,
}

/// Comparison operators
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
pub enum ComparisonOperator {
    /// Greater than
    GreaterThan,
    /// Less than
    LessThan,
    /// Equal to
    EqualTo,
    /// Greater than or equal to
    GreaterThanOrEqual,
    /// Less than or equal to
    LessThanOrEqual,
}

/// Notification channels
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
pub enum NotificationChannel {
    /// Email notification
    Email,
    /// Slack notification
    Slack,
    /// SMS notification
    Sms,
    /// Webhook notification
    Webhook,
}

/// Logging configuration (DEPRECATED - use canonical)
///
/// **MIGRATION**: Use `beardog_types::canonical::config::domains::system::LoggingConfig` instead.
///
/// This type alias will be removed in v3.3.0.
#[deprecated(
    since = "3.1.0",
    note = "Use beardog_types::canonical::config::domains::system::LoggingConfig instead"
)]
pub type LoggingConfig = beardog_types::canonical::config::domains::system::LoggingConfig;

/// Log levels (DEPRECATED - use canonical)
///
/// **MIGRATION**: Use `beardog_types::canonical::config::domains::system::LogLevel` instead.
#[deprecated(
    since = "3.1.0",
    note = "Use beardog_types::canonical::config::domains::system::LogLevel instead"
)]
pub type LogLevel = beardog_types::canonical::config::domains::system::LogLevel;

/// Log format (DEPRECATED - use canonical)
///
/// **MIGRATION**: Use `beardog_types::canonical::config::domains::system::LogFormat` instead.
#[deprecated(
    since = "3.1.0",
    note = "Use beardog_types::canonical::config::domains::system::LogFormat instead"
)]
pub type LogFormat = beardog_types::canonical::config::domains::system::LogFormat;

/// Log destinations (DEPRECATED - use canonical)
///
/// **MIGRATION**: Use `beardog_types::canonical::config::domains::system::LogTargetType` instead.
#[deprecated(
    since = "3.1.0",
    note = "Use beardog_types::canonical::config::domains::system::LogTargetType instead"
)]
pub type LogDestination = beardog_types::canonical::config::domains::system::LogTargetType;
