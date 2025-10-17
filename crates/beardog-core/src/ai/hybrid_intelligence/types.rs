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

/// Training configuration for hybrid intelligence models
///
/// Specifies hyperparameters and settings for training machine learning models,
/// including batch size, learning rate, regularization, and early stopping criteria.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct TrainingConfig {
    /// Number of samples processed in each training iteration before model weights are updated
    pub batch_size: u32,
    /// Total number of complete passes through the entire training dataset during training
    pub epochs: u32,
    /// Step size at each iteration while moving toward a minimum of the loss function
    pub learning_rate: f64,
    /// Proportion of training data (0.0-1.0) reserved for validation during training to prevent overfitting
    pub validation_split: f64,
    /// Configuration for automatically stopping training when validation metrics stop improving
    pub early_stopping: Option<EarlyStoppingConfig>,
    /// Configuration for techniques to prevent model overfitting (L1/L2, dropout, batch normalization)
    pub regularization: Option<RegularizationConfig>,
    /// Algorithm and parameters used to update model weights based on computed gradients
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

/// Inference configuration for model predictions in production
///
/// Defines settings for running trained models in production environments,
/// including batch processing, timeout limits, serving parameters, and caching strategies.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct InferenceConfig {
    /// Number of prediction requests processed together in a single batch for efficiency
    pub batch_size: u32,
    /// Maximum time (in milliseconds) allowed for a single inference request before timing out
    pub max_inference_time_ms: u64,
    /// Configuration for model serving infrastructure, load balancing, and request handling
    pub serving_config: ServingConfig,
    /// Optional caching configuration to store and reuse recent predictions for identical inputs
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

/// Model management configuration for `MLOps` lifecycle
///
/// Comprehensive configuration for managing AI models throughout their lifecycle,
/// including versioning, registry storage, deployment strategies, and performance monitoring.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ModelManagementConfig {
    /// Strategy for assigning version identifiers to models (semantic, timestamp, hash, or incremental)
    pub versioning_strategy: VersioningStrategy,
    /// Configuration for the model registry where trained models are stored and cataloged
    pub registry_config: AIRegistryConfig,
    /// Configuration for deploying models to production environments (blue-green, canary, rolling, etc.)
    pub deployment_config: DeploymentConfig,
    /// Configuration for monitoring model performance, resource usage, and operational metrics in production
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

/// Data preprocessing configuration for input transformation
///
/// Defines transformations applied to raw data before training or inference,
/// including normalization, feature selection, data augmentation, and missing value handling.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PreprocessingConfig {
    /// Strategy for scaling numerical features to a common range (min-max, z-score, robust, etc.)
    pub normalization: NormalizationStrategy,
    /// Optional configuration for selecting the most relevant features from the input dataset
    pub feature_selection: Option<FeatureSelectionConfig>,
    /// Optional configuration for artificially expanding the training dataset with transformed variations
    pub data_augmentation: Option<DataAugmentationConfig>,
    /// Strategy for handling missing values in the input data (drop, fill mean/median/mode, interpolate)
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

/// Neural network configuration for deep learning models
///
/// Complete configuration for constructing, training, and optimizing neural network models,
/// including architecture definition, training parameters, optimization techniques, and regularization.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct NeuralNetworkConfig {
    /// Network structure definition including layers, activations, and connections
    pub architecture: NetworkArchitecture,
    /// Hyperparameters controlling the training process (batch size, epochs, learning rate, optimizer)
    pub training_params: TrainingParams,
    /// Performance optimization settings (mixed precision, gradient clipping, memory optimization)
    pub optimization: NetworkOptimization,
    /// Techniques to prevent overfitting (dropout, batch normalization, weight decay, early stopping)
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

/// Decision engine configuration for hybrid AI decision-making
///
/// Configures the decision engine that combines multiple AI strategies and human input
/// to make intelligent, consensus-based decisions within specified time constraints.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct DecisionEngineConfig {
    /// Collection of decision-making strategies to employ (rule-based, ML, heuristic, human-in-loop)
    pub strategies: Vec<DecisionStrategy>,
    /// Criteria and weights used to evaluate and compare decision options
    pub criteria: DecisionCriteria,
    /// Mechanism for reaching consensus when multiple strategies produce different recommendations
    pub consensus_mechanism: ConsensusStrategy,
    /// Maximum time allowed for the decision-making process before returning the best available decision
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

/// Learning configuration for adaptive AI systems
///
/// Configures how the hybrid intelligence system learns and adapts over time,
/// including online learning, transfer learning from other domains, and meta-learning strategies.
#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct LearningConfig {
    /// Collection of learning algorithm types to employ (supervised, unsupervised, reinforcement, etc.)
    pub algorithms: Vec<LearningAlgorithmType>,
    /// Configuration for continuous learning from new data as it arrives in production
    pub online_learning: OnlineLearningConfig,
    /// Optional configuration for transferring knowledge from pre-trained models in related domains
    pub transfer_learning: Option<TransferLearningConfig>,
    /// Optional configuration for learning how to learn, adapting learning strategies themselves
    pub meta_learning: Option<MetaLearningConfig>,
}

// Types moved to avoid duplication - using existing definitions

/// Prediction configuration for forecasting and inference
///
/// Configures how the system generates predictions, including which models to use,
/// ensemble methods, uncertainty estimation, and prediction time horizons.
#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct PredictionConfig {
    /// Collection of prediction model types to employ for generating forecasts
    pub models: Vec<PredictionModel>,
    /// Optional configuration for combining multiple models into an ensemble for improved accuracy
    pub ensemble: Option<EnsembleConfig>,
    /// Whether to calculate and report confidence intervals and uncertainty estimates for predictions
    pub uncertainty_quantification: bool,
    /// Time horizons for which to generate predictions (short-term, medium-term, long-term)
    pub horizons: Vec<PredictionHorizon>,
}

/// Optimization configuration for model and hyperparameter tuning
///
/// Configures optimization algorithms for finding optimal model parameters, hyperparameters,
/// and solutions to complex problems with multiple objectives and constraints.
#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct OptimizationConfig {
    /// Collection of optimization algorithms to employ (gradient descent, genetic algorithms, Bayesian optimization, etc.)
    pub algorithms: Vec<OptimizationAlgorithm>,
    /// Whether to optimize for multiple competing objectives simultaneously (e.g., accuracy vs. latency)
    pub multi_objective: bool,
    /// Optional constraints on the optimization search space (bounds, equality/inequality constraints)
    pub constraints: Option<ConstraintConfig>,
    /// Optional configuration for automated hyperparameter tuning (grid search, random search, Bayesian)
    pub hyperparameter_optimization: Option<HyperparameterOptimization>,
}

// Optimization types moved to avoid duplication - using existing definitions

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

/// Serving configuration for production model deployment
///
/// Configures how models handle inference requests in production,
/// including concurrency limits, timeouts, and load distribution.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ServingConfig {
    /// Maximum number of inference requests processed simultaneously
    pub max_concurrent_requests: u32,
    /// Maximum time allowed for processing a single inference request
    pub request_timeout: Duration,
    /// Strategy for distributing requests across multiple model instances
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

/// Caching configuration for inference result storage
///
/// Configures caching of model predictions to improve response time
/// and reduce computational load for repeated identical requests.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CachingConfig {
    /// Maximum number of cached prediction results to store in memory
    pub max_cache_size: u64,
    /// Time-to-live duration after which cached results expire and are removed
    pub ttl: Duration,
    /// Policy for removing entries when cache reaches capacity (LRU, LFU, FIFO, TTL)
    pub eviction_policy: EvictionPolicy,
}

/// Model versioning strategies for tracking model evolution
///
/// Different approaches to assigning version identifiers to trained models
/// for tracking, comparison, and rollback purposes.
#[derive(Debug, Clone, Copy, Serialize, Deserialize)]
pub enum VersioningStrategy {
    /// Semantic versioning following MAJOR.MINOR.PATCH format (e.g., 1.2.3)
    Semantic,
    /// Timestamp-based versioning using creation time (e.g., `20251011_143022`)
    Timestamp,
    /// Content-based hash versioning using model weights (e.g., SHA-256)
    Hash,
    /// Simple incremental integer versioning (e.g., v1, v2, v3)
    Incremental,
}

/// Registry configuration for AI model storage and catalog
///
/// Configures where and how trained models are stored, versioned, and retrieved.
/// Renamed from `RegistryConfig` to `AIRegistryConfig` for clarity.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct AIRegistryConfig {
    /// Type of registry storage backend (local filesystem, remote HTTP, database, or cloud storage)
    pub registry_type: RegistryType,
    /// Network address or path where the registry is accessible
    pub endpoint: String,
    /// Optional authentication credentials required to access the registry
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

/// Deployment configuration for model rollout strategies
///
/// Defines how models are deployed to production environments,
/// including deployment strategy, resource allocation, and health monitoring.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct DeploymentConfig {
    /// Strategy for rolling out model updates (blue-green, canary, rolling, or recreate)
    pub strategy: DeploymentStrategy,
    /// Required computational resources (CPU, memory, GPU, storage)
    pub resources: ResourceRequirements,
    /// Configuration for monitoring deployment health and readiness
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

/// AI-specific monitoring configuration for model observability
///
/// Configures collection of metrics and telemetry specific to AI model training,
/// inference, performance, and resource utilization in production.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct AIMonitoringConfig {
    /// Whether to collect and report training metrics (loss, accuracy, learning rate, etc.)
    pub collect_training_metrics: bool,
    /// Whether to collect and report inference metrics (latency, throughput, batch size, etc.)
    pub collect_inference_metrics: bool,
    /// Whether to track and monitor model prediction quality and accuracy over time
    pub track_model_performance: bool,
    /// Whether to monitor computational resource usage (CPU, memory, GPU utilization)
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

/// Normalization strategies for feature scaling
///
/// Different methods for transforming numerical features to a common scale,
/// improving model training stability and convergence.
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
pub enum NormalizationStrategy {
    /// Min-max scaling to range [0, 1]: (x - min) / (max - min)
    MinMax,
    /// Z-score standardization to mean 0, std 1: (x - mean) / std
    ZScore,
    /// Robust scaling using median and IQR, less sensitive to outliers
    Robust,
    /// Unit vector normalization, scales to unit length
    UnitVector,
    /// No normalization applied, use raw feature values
    None,
}

/// Feature selection configuration for dimensionality reduction
///
/// Configures automatic selection of the most relevant features from the input dataset,
/// reducing dimensionality and improving model performance.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct FeatureSelectionConfig {
    /// Algorithm used to identify and select important features
    pub method: FeatureSelectionMethod,
    /// Optional target number of features to select from the input dataset
    pub n_features: Option<u32>,
    /// Optional importance threshold for feature selection (features below threshold are removed)
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

/// Data augmentation configuration for training dataset expansion
///
/// Configures techniques for artificially expanding the training dataset
/// with transformed variations to improve model generalization.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct DataAugmentationConfig {
    /// Collection of transformation techniques to apply (rotation, translation, scaling, noise, etc.)
    pub techniques: Vec<AugmentationTechnique>,
    /// Probability (0.0-1.0) that augmentation is applied to each training sample
    pub probability: f64,
    /// Technique-specific parameters controlling transformation intensity and behavior
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

/// Authentication configuration for secure registry access
///
/// Configures authentication credentials and method for accessing
/// protected model registries and services.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct AuthConfig {
    /// Type of authentication mechanism to use (API key, bearer token, basic auth, `OAuth2`, or none)
    pub auth_type: AuthType,
    /// Authentication credentials as key-value pairs (e.g., "`api_key"`: "...", "username": "...", "password": "...")
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

/// Resource requirements for model deployment
///
/// Specifies the computational resources required to run a model
/// in production, used for scheduling and capacity planning.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ResourceRequirements {
    /// Number of CPU cores required (fractional values allowed, e.g., 0.5 for half a core)
    pub cpu: f64,
    /// Memory requirement in megabytes (MB)
    pub memory: u64,
    /// Optional number of GPU devices required for acceleration
    pub gpu: Option<u32>,
    /// Persistent storage requirement in gigabytes (GB) for model artifacts and logs
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

/// Alerting configuration for model monitoring notifications
///
/// Configures rules and channels for alerting operators when models
/// exhibit anomalous behavior or performance degradation.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct AlertingConfig {
    /// Collection of alert rules defining conditions that trigger notifications
    pub rules: Vec<AlertRule>,
    /// Collection of notification channels for delivering alerts (email, Slack, SMS, webhook)
    pub channels: Vec<NotificationChannel>,
}

/// Alert rule definition for monitoring thresholds
///
/// Defines a monitoring rule that triggers an alert when a metric
/// crosses a threshold within a specified time window.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct AlertRule {
    /// Human-readable name identifying this alert rule
    pub name: String,
    /// Type of metric to monitor (latency, throughput, error rate, accuracy, etc.)
    pub metric: MetricType,
    /// Numeric threshold value that triggers the alert when crossed
    pub threshold: f64,
    /// Comparison operator for threshold evaluation (>, <, ==, >=, <=)
    pub operator: ComparisonOperator,
    /// Time window over which to evaluate the metric before triggering
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
