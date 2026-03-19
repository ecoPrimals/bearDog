// SPDX-License-Identifier: AGPL-3.0-only

// Learning systems and optimization algorithms
//
// ⚠️ DEPRECATED: This module is being migrated to the canonical location.
//
// **New Location**: `beardog_types::canonical::config::domains::ai_config`
//
// These types will be removed in v3.3.0 (Q1 2026). Please update your imports to:
// ```rust
// use beardog_types::canonical::config::domains::ai_config::{
//     OnlineLearningConfig, TransferLearningConfig, MetaLearningConfig,
//     EnsembleConfigLearning, HyperparameterOptimizationConfig, ...
// };
// ```
//
// Migration Status: Phase 2 Complete (October 2025)
// - All learning config types now available in canonical location
// - OnlineLearningConfig duplicate resolved (use canonical version)
// - Transfer learning, meta-learning, ensemble configs migrated
// - Hyperparameter optimization configs unified

use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use std::time::Duration;
// Removed unused Uuid import

/// Types of learning algorithms
///
/// Classification of different machine learning paradigms and approaches.
#[derive(Debug, Clone, Copy, Serialize, Deserialize)]
pub enum LearningAlgorithmType {
    /// Supervised learning from labeled data
    Supervised,
    /// Unsupervised learning from unlabeled data
    Unsupervised,
    /// Reinforcement learning through rewards
    Reinforcement,
    /// Semi-supervised learning from mixed labeled/unlabeled data
    SemiSupervised,
    /// Transfer learning from pre-trained models
    Transfer,
    /// Meta-learning (learning to learn)
    Meta,
    /// Online learning with continuous updates
    Online,
    /// Federated learning across distributed data sources
    Federated,
}

// Note: OnlineLearningConfig is a temporary local definition pending canonical ai_config export.
// This will be replaced with a canonical import once ai_config is properly modularized.
/// Online learning configuration
#[derive(Debug, Clone, Copy, Serialize, Deserialize)]
pub struct OnlineLearningConfig {
    /// Whether online learning is enabled
    pub enabled: bool,
    /// Learning rate for online updates
    pub learning_rate: f64,
    /// Batch size for online learning
    pub batch_size: usize,
    /// Update frequency in samples
    pub update_frequency: usize,
}

impl Default for OnlineLearningConfig {
    fn default() -> Self {
        Self {
            enabled: std::env::var("BEARDOG_AI_ONLINE_LEARNING_ENABLED")
                .ok()
                .and_then(|s| s.parse().ok())
                .unwrap_or(false),
            learning_rate: std::env::var("BEARDOG_AI_ONLINE_LEARNING_RATE")
                .ok()
                .and_then(|s| s.parse().ok())
                .unwrap_or(0.001),
            batch_size: std::env::var("BEARDOG_AI_ONLINE_BATCH_SIZE")
                .ok()
                .and_then(|s| s.parse().ok())
                .unwrap_or(32),
            update_frequency: std::env::var("BEARDOG_AI_ONLINE_UPDATE_FREQUENCY")
                .ok()
                .and_then(|s| s.parse().ok())
                .unwrap_or(100),
        }
    }
}

/// Learning rate adaptation strategies
///
/// Strategies for adjusting the learning rate during training.
#[derive(Debug, Clone, Copy, Serialize, Deserialize, PartialEq, Eq)]
pub enum LearningRateAdaptation {
    /// Fixed learning rate throughout training
    Fixed,
    /// Adaptive learning rate based on gradient statistics
    Adaptive,
    /// Decay-based adaptation (exponential or linear)
    Decay,
    /// Performance-based adaptation using validation metrics
    PerformanceBased,
}

/// Update frequency for model parameters
///
/// Determines when model parameters should be updated during training.
#[derive(Debug, Clone, Copy, Serialize, Deserialize)]
pub enum UpdateFrequency {
    /// Update after each individual sample
    PerSample,
    /// Update after each batch of samples
    PerBatch,
    /// Update based on time interval
    TimeInterval(Duration),
    /// Update when performance crosses threshold
    PerformanceThreshold(f64),
}

/// Transfer learning configuration
///
/// Configuration for transfer learning from a pre-trained model.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct TransferLearningConfig {
    /// Source domain configuration where model was pre-trained
    pub source_domain: DomainConfig,
    /// Target domain configuration
    /// The target domain value
    pub target_domain: DomainConfig,
    /// Transfer strategy
    /// The transfer strategy value
    pub transfer_strategy: TransferStrategy,
    /// Fine-tuning configuration
    /// Optional fine tuning
    pub fine_tuning: Option<FineTuningConfig>,
}

/// Domain configuration
///
/// Configuration for a learning domain (source or target) in transfer learning.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct DomainConfig {
    /// Unique domain identifier
    pub domain_id: String,
    /// Human-readable domain description
    pub description: String,
    /// Feature space configuration for this domain
    pub feature_space: FeatureSpaceConfig,
    /// Label space configuration for this domain
    pub label_space: LabelSpaceConfig,
}

/// Feature space configuration
///
/// Defines the structure and types of input features.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct FeatureSpaceConfig {
    /// Number of features in this space
    pub num_features: u32,
    /// Types of each feature
    pub feature_types: Vec<FeatureType>,
    /// Normalization strategy for features
    pub normalization: NormalizationStrategy,
}

/// Feature types
#[derive(Debug, Clone, Copy, Serialize, Deserialize, PartialEq, Eq)]
/// Types of feature
pub enum FeatureType {
    /// Continuous feature
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

/// Label space configuration
///
/// Defines the structure and encoding of output labels.
#[derive(Debug, Clone, Copy, Serialize, Deserialize)]
pub struct LabelSpaceConfig {
    /// Number of labels in this space
    pub num_labels: u32,
    /// Type of classification/regression task
    pub label_type: LabelType,
    /// Encoding strategy for labels
    pub encoding: LabelEncoding,
}

/// Label types
#[derive(Debug, Clone, Copy, Serialize, Deserialize, PartialEq, Eq)]
/// Types of label
pub enum LabelType {
    /// Binary classification
    Binary,
    /// Multi-class classification
    MultiClass,
    /// Multi-label classification
    MultiLabel,
    /// Regression
    Regression,
    /// Structured prediction
    Structured,
}

/// Label encoding strategies
#[derive(Debug, Clone, Copy, Serialize, Deserialize, PartialEq, Eq)]
pub enum LabelEncoding {
    /// One-hot encoding
    OneHot,
    /// Label encoding
    Label,
    /// Binary encoding
    Binary,
    /// Target encoding
    Target,
}

/// Transfer learning strategies
#[derive(Debug, Clone, Copy, Serialize, Deserialize, PartialEq, Eq)]
pub enum TransferStrategy {
    /// Feature extraction
    FeatureExtraction,
    /// Fine-tuning
    FineTuning,
    /// Domain adaptation
    DomainAdaptation,
    /// Multi-task learning
    MultiTask,
    /// Few-shot learning
    FewShot,
    /// Zero-shot learning
    ZeroShot,
}

/// Fine-tuning configuration
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct FineTuningConfig {
    /// Layers to freeze
    /// Collection of frozen layers
    pub frozen_layers: Vec<u32>,
    /// The fine tune learning rate value
    pub fine_tune_learning_rate: f64,
    /// Number of fine-tuning epochs
    /// Number of `fine_tune_epochs`
    pub fine_tune_epochs: u32,
    /// Gradual unfreezing
    /// Whether `gradual_unfreezing` is enabled
    pub gradual_unfreezing: bool,
}

/// Meta-learning configuration
#[derive(Debug, Clone, Copy, Serialize, Deserialize)]
pub struct MetaLearningConfig {
    /// Meta-learning algorithm
    /// The algorithm value
    pub algorithm: MetaLearningAlgorithm,
    /// Number of `num_meta_train_tasks`
    pub num_meta_train_tasks: u32,
    /// Number of shots per task
    /// Number of `num_shots`
    pub num_shots: u32,
    /// Inner loop configuration
    /// The inner loop value
    pub inner_loop: InnerLoopConfig,
    /// Outer loop configuration
    /// The outer loop value
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

/// Configuration for inner loop of meta-learning
#[derive(Debug, Clone, Copy, Serialize, Deserialize)]
pub struct InnerLoopConfig {
    /// Inner learning rate
    /// The learning rate value
    pub learning_rate: f64,
    /// Number of inner steps
    /// Number of `num_steps`
    pub num_steps: u32,
    /// Inner optimizer
    /// The optimizer value
    pub optimizer: OptimizerType,
}

/// Configuration for outer loop of meta-learning
#[derive(Debug, Clone, Copy, Serialize, Deserialize)]
pub struct OuterLoopConfig {
    /// Outer learning rate
    /// The learning rate value
    pub learning_rate: f64,
    /// Number of outer steps
    /// Number of `num_steps`
    pub num_steps: u32,
    /// Outer optimizer
    /// The optimizer value
    pub optimizer: OptimizerType,
}

/// Machine learning prediction model types
///
/// Specifies the category of prediction task the model is designed
/// to perform, each with different output types and evaluation metrics.
#[derive(Debug, Clone, Copy, Serialize, Deserialize, PartialEq, Eq)]
pub enum PredictionModel {
    /// Time series forecasting model
    TimeSeries,
    /// Regression model for continuous value prediction
    Regression,
    /// Classification model for categorical predictions
    Classification,
    /// Anomaly detection model for identifying outliers
    AnomalyDetection,
    /// Clustering model for unsupervised grouping
    Clustering,
    /// Recommendation model for suggesting items
    Recommendation,
    /// Survival analysis model for time-to-event prediction
    SurvivalAnalysis,
}

/// Configuration for ensemble learning models
///
/// Ensemble learning combines multiple models to improve prediction accuracy
/// and robustness. This configuration defines the base models, combination
/// method, and diversity measures used in the ensemble.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct EnsembleConfig {
    /// Base models that make up the ensemble
    pub base_models: Vec<BaseModel>,
    /// Method used to combine base model predictions
    pub ensemble_method: EnsembleMethod,
    /// Optional weights for each model in voting schemes
    pub model_weights: Option<HashMap<String, f64>>,
    /// Measures to ensure diversity among base models
    pub diversity_measures: Vec<DiversityMeasure>,
}

/// A base model in an ensemble learning configuration
///
/// Each base model represents a distinct machine learning algorithm
/// with its own configuration and weighting within the ensemble.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct BaseModel {
    /// Unique identifier for this model
    pub id: String,
    /// Type of machine learning model
    pub model_type: ModelType,
    /// Model-specific configuration parameters
    pub configuration: HashMap<String, serde_json::Value>,
    /// Weight of this model in ensemble voting (0.0-1.0)
    pub weight: f64,
}

/// Ensemble methods
#[derive(Debug, Clone, Copy, Serialize, Deserialize, PartialEq, Eq)]
pub enum EnsembleMethod {
    /// Simple voting
    Voting,
    /// Weighted voting
    WeightedVoting,
    /// Stacking
    Stacking,
    /// Blending
    Blending,
    /// Bagging
    Bagging,
    /// Boosting
    Boosting,
    /// Random forest ensemble (combines bagging with decision trees)
    RandomForest,
}

/// Diversity measures for ensemble models
///
/// Metrics to ensure base models in an ensemble make different types of
/// errors, improving ensemble performance through complementary predictions.
#[derive(Debug, Clone, Copy, Serialize, Deserialize, PartialEq, Eq)]
pub enum DiversityMeasure {
    /// Disagreement measure (percentage of different predictions)
    Disagreement,
    /// Double-fault measure (both models wrong on same examples)
    DoubleFault,
    /// Kohavi-Wolpert variance (variance in model predictions)
    KohaviWolpert,
    /// Inter-rater agreement (Cohen's kappa statistic)
    InterRater,
    /// Entropy measure (Shannon entropy of predictions)
    Entropy,
}

/// Time horizons for predictions
///
/// Defines the temporal range for forecasting and prediction tasks,
/// from immediate short-term to extended long-term horizons.
#[derive(Debug, Clone, Copy, Serialize, Deserialize, PartialEq, Eq)]
pub enum PredictionHorizon {
    /// Short-term prediction (minutes to hours)
    ShortTerm,
    /// Medium-term prediction (hours to days)
    MediumTerm,
    /// Long-term prediction (days to weeks or longer)
    LongTerm,
    /// Custom horizon with specific duration
    Custom(Duration),
}

/// Hyperparameter optimization algorithms
///
/// Methods for searching the hyperparameter space to find optimal
/// model configurations, ranging from gradient-based to evolutionary approaches.
#[derive(Debug, Clone, Copy, Serialize, Deserialize, PartialEq, Eq)]
pub enum OptimizationAlgorithm {
    /// Gradient descent optimization
    GradientDescent,
    /// Genetic algorithm (evolutionary approach)
    GeneticAlgorithm,
    /// Particle swarm optimization (swarm intelligence)
    ParticleSwarmOptimization,
    /// Simulated annealing (probabilistic technique)
    SimulatedAnnealing,
    /// Differential evolution (population-based optimization)
    DifferentialEvolution,
    /// Ant colony optimization (swarm intelligence)
    AntColonyOptimization,
    /// Bayesian optimization (probabilistic model-based)
    BayesianOptimization,
    /// Grid search (exhaustive search)
    GridSearch,
    /// Random search (random sampling)
    RandomSearch,
}

/// Configuration for optimization constraints
///
/// Defines the constraints that must be satisfied during model optimization,
/// including equality, inequality, and bound constraints.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ConstraintConfig {
    /// Equality constraints (e.g., x + y = 10)
    pub equality_constraints: Vec<Constraint>,
    /// Inequality constraints (e.g., x + y <= 10)
    pub inequality_constraints: Vec<Constraint>,
    /// Bound constraints on individual variables (e.g., 0 <= x <= 100)
    pub bound_constraints: Vec<BoundConstraint>,
}

/// Single optimization constraint
///
/// Represents a mathematical constraint that must be satisfied during
/// optimization, such as equality or inequality conditions.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Constraint {
    /// Unique constraint identifier
    pub id: String,
    /// Mathematical expression defining the constraint
    /// The expression value
    pub expression: String,
    /// Constraint tolerance
    /// The tolerance value
    pub tolerance: f64,
}

/// Variable bound constraint for optimization
///
/// Restricts a single optimization variable to a specified range,
/// ensuring values remain within feasible bounds.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct BoundConstraint {
    /// Name of the variable being constrained
    pub variable: String,
    /// Minimum allowed value (None for unbounded below)
    pub lower_bound: Option<f64>,
    /// Maximum allowed value (None for unbounded above)
    pub upper_bound: Option<f64>,
}

/// Comprehensive hyperparameter optimization configuration
///
/// Controls the hyperparameter search process to automatically find
/// optimal model configurations through systematic exploration.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct HyperparameterOptimization {
    /// Search algorithm to use
    pub method: HyperparameterOptimizationMethod,
    /// Maximum number of configurations to try
    pub max_trials: u32,
    /// Maximum time allowed for optimization (in seconds)
    pub timeout_secs: u64,
    /// Metric used to evaluate configurations
    pub objective_metric: String,
    /// Whether to minimize or maximize the objective metric
    pub optimization_direction: OptimizationDirection,
}

/// Hyperparameter optimization search methods
///
/// Algorithms for exploring the hyperparameter space to find optimal
/// model configurations, from simple exhaustive search to advanced
/// model-based approaches.
#[derive(Debug, Clone, Copy, Serialize, Deserialize, PartialEq, Eq)]
pub enum HyperparameterOptimizationMethod {
    /// Random search (sample configurations randomly)
    RandomSearch,
    /// Grid search (exhaustive search over discrete space)
    GridSearch,
    /// Bayesian optimization (probabilistic model-based search)
    BayesianOptimization,
    /// Hyperband (bandit-based adaptive resource allocation)
    Hyperband,
    /// Population-based training (evolutionary strategy with online adaptation)
    PopulationBasedTraining,
    /// Optuna framework (automatic hyperparameter optimization)
    Optuna,
}

/// Neural Architecture Search (NAS) configuration
///
/// Automatically discovers optimal neural network architectures by
/// searching through possible configurations, layer types, and connections.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct NeuralArchitectureSearch {
    /// Defines the space of possible architectures to explore
    pub search_space: SearchSpace,
    /// Strategy for exploring the architecture space
    pub search_strategy: NasSearchStrategy,
    /// Method for estimating architecture performance without full training
    pub performance_estimation: PerformanceEstimation,
}

/// Neural Architecture Search space definition
///
/// Defines the boundaries of the architecture search, including
/// which layer types, network dimensions, and activation functions
/// are candidates for the optimal architecture.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SearchSpace {
    /// Candidate layer types (Dense, Conv, Pool, etc.)
    pub layer_types: Vec<LayerType>,
    /// Network depth range (`min_layers`, `max_layers`)
    pub depth_range: (u32, u32),
    /// Layer width range (`min_units`, `max_units`)
    pub width_range: (u32, u32),
    /// Candidate activation functions for layers
    pub activation_functions: Vec<ActivationFunction>,
}

/// Neural network layer types
///
/// Fundamental building blocks for constructing neural network architectures,
/// each with different computational properties and use cases.
#[derive(Debug, Clone, Copy, Serialize, Deserialize, PartialEq, Eq)]
pub enum LayerType {
    /// Dense/fully-connected layer (all inputs connected to all outputs)
    Dense,
    /// Convolutional layer (applies filters to spatial data)
    Convolutional,
    /// Pooling layer (downsamples spatial dimensions)
    Pooling,
    /// Batch normalization (normalizes layer inputs for stable training)
    BatchNormalization,
    /// Dropout layer (randomly disables neurons for regularization)
    Dropout,
    /// Skip connection (residual connection bypassing layers)
    SkipConnection,
}

/// Neural network activation functions
///
/// Non-linear functions applied to layer outputs, enabling neural networks
/// to learn complex patterns beyond linear transformations.
#[derive(Debug, Clone, Copy, Serialize, Deserialize, PartialEq, Eq)]
pub enum ActivationFunction {
    /// `ReLU` (Rectified Linear Unit): max(0, x) - most common, computationally efficient
    Relu,
    /// Sigmoid: 1/(1+e^-x) - squashes to (0,1), used for binary classification
    Sigmoid,
    /// Tanh: hyperbolic tangent - squashes to (-1,1), zero-centered
    Tanh,
    /// Softmax: normalized exponentials - converts logits to probabilities
    Softmax,
    /// Leaky `ReLU`: max(0.01x, x) - allows small negative gradients
    LeakyRelu,
    /// ELU (Exponential Linear Unit): smooth approximation to `ReLU`
    Elu,
    /// Swish: x * sigmoid(x) - self-gated, smooth, non-monotonic
    Swish,
    /// GELU (Gaussian Error Linear Unit): Gaussian-weighted `ReLU`, used in transformers
    Gelu,
}

/// Neural Architecture Search strategies
///
/// Methods for exploring the architecture space to find optimal
/// neural network designs, from random exploration to learned search policies.
#[derive(Debug, Clone, Copy, Serialize, Deserialize, PartialEq, Eq)]
pub enum NasSearchStrategy {
    /// Random search (sample architectures randomly)
    RandomSearch,
    /// Evolutionary search (genetic algorithm approach)
    EvolutionarySearch,
    /// Reinforcement learning (train agent to propose architectures)
    ReinforcementLearning,
    /// Differentiable architecture search (DARTS - gradient-based)
    DifferentiableSearch,
    /// Progressive search (incrementally grow architectures)
    ProgressiveSearch,
}

/// Configuration for estimating model performance during training
///
/// Enables efficient model evaluation by using techniques like early stopping
/// and learning curve extrapolation to avoid full training when possible.
#[derive(Debug, Clone, Copy, Serialize, Deserialize)]
pub struct PerformanceEstimation {
    /// Method used to estimate performance
    pub method: PerformanceEstimationMethod,
    /// Optional criteria for stopping training early
    pub early_stopping: Option<EarlyStoppingCriteria>,
    /// Resource limits for training operations
    pub resource_constraints: ResourceConstraints,
}

/// Performance estimation methods for efficient model evaluation
///
/// Techniques to evaluate model quality without full training,
/// enabling faster hyperparameter search and architecture selection.
#[derive(Debug, Clone, Copy, Serialize, Deserialize, PartialEq, Eq)]
pub enum PerformanceEstimationMethod {
    /// Full training (train to completion for accurate assessment)
    FullTraining,
    /// Early stopping (halt when validation loss stops improving)
    EarlyStopping,
    /// Learning curve extrapolation (predict final performance from partial training)
    LearningCurveExtrapolation,
    /// Network morphism (transform existing trained networks)
    NetworkMorphism,
    /// Weight inheritance (initialize from similar trained models)
    WeightInheritance,
}

/// Criteria for stopping model training early
///
/// Early stopping prevents overfitting by halting training when performance
/// stops improving on validation data, saving computational resources.
#[derive(Debug, Clone, Copy, Serialize, Deserialize)]
pub struct EarlyStoppingCriteria {
    /// Minimum number of training epochs before early stopping can occur
    pub min_epochs: u32,
    /// Maximum number of training epochs allowed
    pub max_epochs: u32,
    /// Number of epochs with no improvement before stopping
    pub patience: u32,
    /// Minimum performance threshold that must be reached
    pub performance_threshold: f64,
}

/// Resource constraints for model training operations
///
/// Defines limits on computational resources to prevent runaway training
/// operations and ensure fair resource allocation in multi-tenant environments.
#[derive(Debug, Clone, Copy, Serialize, Deserialize)]
pub struct ResourceConstraints {
    /// Maximum allowed training time before timeout
    pub max_training_time: Duration,
    /// Maximum memory usage in gigabytes
    pub max_memory_gb: f64,
    /// Maximum number of GPUs that can be used
    pub max_gpu_count: u32,
    /// Maximum number of CPU cores that can be used
    pub max_cpu_cores: u32,
}

/// Machine learning model types supported by the hybrid intelligence system
///
/// Specifies the fundamental machine learning algorithm type used for
/// making predictions and classifications.
#[derive(Debug, Clone, Copy, Serialize, Deserialize)]
pub enum ModelType {
    /// Linear regression model for continuous predictions
    Linear,
    /// Logistic regression model for binary classification
    Logistic,
    /// Decision tree model for interpretable rule-based predictions
    DecisionTree,
    /// Random forest ensemble of decision trees
    RandomForest,
    /// Support vector machine for classification and regression
    SVM,
    /// Neural network model for complex pattern recognition
    NeuralNetwork,
    /// Ensemble model combining multiple approaches for robust predictions
    Ensemble,
}

/// Phases of the machine learning training pipeline
///
/// Represents the sequential stages in developing and deploying a machine
/// learning model, from initial data processing to production deployment.
#[derive(Debug, Clone, Copy, Serialize, Deserialize)]
pub enum TrainingPhase {
    /// Data preparation and cleaning phase
    DataPreparation,
    /// Feature engineering and selection phase
    FeatureEngineering,
    /// Model architecture selection phase
    ModelSelection,
    /// Hyperparameter optimization phase
    HyperparameterTuning,
    /// Model performance evaluation phase
    ModelEvaluation,
    /// Production deployment phase
    Deployment,
}

/// Optimization algorithms for training neural networks
///
/// Specifies the gradient descent algorithm variant used to update model
/// weights during training. Each optimizer has different convergence
/// properties and is suited for different types of problems.
#[derive(Debug, Clone, Copy, Serialize, Deserialize, PartialEq, Eq)]
pub enum OptimizerType {
    /// Stochastic Gradient Descent with momentum
    Sgd,
    /// Adam (Adaptive Moment Estimation) optimizer
    Adam,
    /// `AdamW` optimizer with decoupled weight decay
    AdamW,
    /// `RMSprop` (Root Mean Square Propagation) optimizer
    RmsProp,
    /// Adagrad (Adaptive Gradient) optimizer
    Adagrad,
}

/// Data normalization strategies for preprocessing
///
/// Defines how to scale and normalize input data before training,
/// which can significantly impact model performance and convergence speed.
#[derive(Debug, Clone, Copy, Serialize, Deserialize, PartialEq, Eq)]
pub enum NormalizationStrategy {
    /// Min-max normalization to [0, 1] range
    MinMax,
    /// Z-score (standard) normalization to mean=0, std=1
    ZScore,
    /// Robust scaling using median and IQR (less sensitive to outliers)
    Robust,
    /// Unit vector scaling (normalize to unit length)
    UnitVector,
    /// No normalization applied
    None,
}

/// Optimization direction for objective functions
///
/// Specifies whether the optimization goal is to minimize or maximize
/// the objective function (e.g., minimize loss or maximize accuracy).
#[derive(Debug, Clone, Copy, Serialize, Deserialize, PartialEq, Eq)]
pub enum OptimizationDirection {
    /// Minimize the objective function (e.g., loss, error)
    Minimize,
    /// Maximize the objective function (e.g., accuracy, reward)
    Maximize,
}

/// Result of an optimization process
///
/// Contains the optimal parameters found during hyperparameter search
/// or model training optimization, along with convergence metadata.
#[derive(Debug, Clone)]
pub struct OptimizationResult {
    /// Best parameter values found during optimization
    pub best_params: Vec<f64>,
    /// Best objective function value achieved
    pub best_value: f64,
    /// Number of optimization iterations performed
    pub iterations: u32,
    /// Whether the optimization converged successfully
    pub converged: bool,
}

/// Hyperparameter optimization engine for automated model tuning
///
/// Systematically searches the hyperparameter space to find optimal
/// model configurations using various optimization strategies.
#[derive(Debug)]
pub struct HyperparameterOptimizer {
    /// Unique identifier for this optimizer instance
    pub id: String,
    /// Optimization strategy being used
    pub method: HyperparameterOptimizationMethod,
    /// Number of hyperparameter trials completed so far
    pub completed_trials: u32,
    /// Best trial result found during optimization
    pub best_trial: Option<TrialResult>,
}

/// Result of a single hyperparameter optimization trial
///
/// Records the parameters tested, the resulting performance metric,
/// and metadata about the trial execution.
#[derive(Debug, Clone)]
pub struct TrialResult {
    /// Unique identifier for this trial
    pub id: String,
    /// Hyperparameters tested in this trial
    pub params: HashMap<String, f64>,
    /// Objective function value achieved with these parameters
    pub objective_value: f64,
    /// Time taken to complete this trial
    pub duration: Duration,
}

/// Model validation strategies for assessing performance
///
/// Defines how to split and evaluate data to estimate model performance
/// on unseen data and detect overfitting.
#[derive(Debug, Clone, Copy, Serialize, Deserialize)]
pub enum ValidationStrategy {
    /// Hold-out validation with separate train/test split
    HoldOut,
    /// K-fold cross-validation for robust performance estimation
    CrossValidation,
    /// Time series-aware cross-validation preserving temporal order
    TimeSeriesCrossValidation,
    /// Bootstrap resampling validation
    Bootstrap,
    /// Monte Carlo cross-validation with random splits
    MonteCarlo,
}
