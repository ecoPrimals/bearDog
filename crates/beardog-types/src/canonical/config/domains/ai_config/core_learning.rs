// SPDX-License-Identifier: AGPL-3.0-or-later

//! Implementation-level learning algorithm types.
//!
//! These types define detailed ML learning configurations used by `beardog-core`'s
//! hybrid intelligence pipeline. They complement the simplified configuration types
//! in the sibling `learning` module.

use beardog_config::env_keys;
use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use std::time::Duration;

// ── Learning Algorithm Types ─────────────────────────────────────────────────

/// Types of learning algorithms
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

/// Online learning configuration (implementation-level)
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
            enabled: std::env::var(env_keys::ENV_AI_ONLINE_LEARNING_ENABLED)
                .ok()
                .and_then(|s| s.parse().ok())
                .unwrap_or(false),
            learning_rate: std::env::var(env_keys::ENV_AI_ONLINE_LEARNING_RATE)
                .ok()
                .and_then(|s| s.parse().ok())
                .unwrap_or(0.001),
            batch_size: std::env::var(env_keys::ENV_AI_ONLINE_BATCH_SIZE)
                .ok()
                .and_then(|s| s.parse().ok())
                .unwrap_or(32),
            update_frequency: std::env::var(env_keys::ENV_AI_ONLINE_UPDATE_FREQUENCY)
                .ok()
                .and_then(|s| s.parse().ok())
                .unwrap_or(100),
        }
    }
}

/// Learning rate adaptation strategies
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

// ── Transfer Learning ────────────────────────────────────────────────────────

/// Transfer learning configuration (implementation-level)
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct TransferLearningConfig {
    /// Source domain configuration where model was pre-trained
    pub source_domain: DomainConfig,
    /// Target domain configuration
    pub target_domain: DomainConfig,
    /// Transfer strategy
    pub transfer_strategy: TransferStrategy,
    /// Fine-tuning configuration
    pub fine_tuning: Option<FineTuningConfig>,
}

/// Domain configuration for transfer learning
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

/// Fine-tuning configuration (implementation-level)
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct FineTuningConfig {
    /// Layers to freeze
    pub frozen_layers: Vec<u32>,
    /// The fine tune learning rate value
    pub fine_tune_learning_rate: f64,
    /// Number of fine-tuning epochs
    pub fine_tune_epochs: u32,
    /// Gradual unfreezing
    pub gradual_unfreezing: bool,
}

// ── Meta-Learning ────────────────────────────────────────────────────────────

/// Meta-learning configuration (implementation-level)
#[derive(Debug, Clone, Copy, Serialize, Deserialize)]
pub struct MetaLearningConfig {
    /// Meta-learning algorithm
    pub algorithm: MetaLearningAlgorithm,
    /// Number of meta-training tasks
    pub num_meta_train_tasks: u32,
    /// Number of shots per task
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
#[derive(Debug, Clone, Copy, Serialize, Deserialize)]
pub struct InnerLoopConfig {
    /// Inner learning rate
    pub learning_rate: f64,
    /// Number of inner steps
    pub num_steps: u32,
    /// Inner optimizer
    pub optimizer: OptimizerType,
}

/// Outer loop configuration for meta-learning
#[derive(Debug, Clone, Copy, Serialize, Deserialize)]
pub struct OuterLoopConfig {
    /// Outer learning rate
    pub learning_rate: f64,
    /// Number of outer steps
    pub num_steps: u32,
    /// Outer optimizer
    pub optimizer: OptimizerType,
}

// ── Prediction & Ensemble ────────────────────────────────────────────────────

/// Machine learning prediction model types
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

/// Ensemble learning configuration (implementation-level)
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
    /// Random forest ensemble
    RandomForest,
}

// ── Optimization (from learning_optimization.rs) ─────────────────────────────

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

/// Time horizons for predictions
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
#[derive(Debug, Clone, Copy, Serialize, Deserialize, PartialEq, Eq)]
pub enum OptimizationAlgorithm {
    /// Gradient descent optimization
    GradientDescent,
    /// Genetic algorithm
    GeneticAlgorithm,
    /// Particle swarm optimization
    ParticleSwarmOptimization,
    /// Simulated annealing
    SimulatedAnnealing,
    /// Differential evolution
    DifferentialEvolution,
    /// Ant colony optimization
    AntColonyOptimization,
    /// Bayesian optimization
    BayesianOptimization,
    /// Grid search
    GridSearch,
    /// Random search
    RandomSearch,
}

/// Configuration for optimization constraints
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ConstraintConfig {
    /// Equality constraints
    pub equality_constraints: Vec<Constraint>,
    /// Inequality constraints
    pub inequality_constraints: Vec<Constraint>,
    /// Bound constraints on individual variables
    pub bound_constraints: Vec<BoundConstraint>,
}

/// Single optimization constraint
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Constraint {
    /// Unique constraint identifier
    pub id: String,
    /// Mathematical expression defining the constraint
    pub expression: String,
    /// Constraint tolerance
    pub tolerance: f64,
}

/// Variable bound constraint for optimization
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct BoundConstraint {
    /// Name of the variable being constrained
    pub variable: String,
    /// Minimum allowed value
    pub lower_bound: Option<f64>,
    /// Maximum allowed value
    pub upper_bound: Option<f64>,
}

/// Comprehensive hyperparameter optimization configuration
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

/// Neural Architecture Search configuration
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct NeuralArchitectureSearch {
    /// Search space definition
    pub search_space: SearchSpace,
    /// Strategy for exploring the architecture space
    pub search_strategy: NasSearchStrategy,
    /// Performance estimation method
    pub performance_estimation: PerformanceEstimation,
}

/// NAS search space definition
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SearchSpace {
    /// Candidate layer types
    pub layer_types: Vec<LayerType>,
    /// Network depth range (`min_layers`, `max_layers`)
    pub depth_range: (u32, u32),
    /// Layer width range (`min_units`, `max_units`)
    pub width_range: (u32, u32),
    /// Candidate activation functions
    pub activation_functions: Vec<ActivationFunction>,
}

/// Layer types for NAS search
#[derive(Debug, Clone, Copy, Serialize, Deserialize, PartialEq, Eq)]
pub enum LayerType {
    /// Dense/fully-connected layer
    Dense,
    /// Convolutional layer
    Convolutional,
    /// Pooling layer
    Pooling,
    /// Batch normalization
    BatchNormalization,
    /// Dropout layer
    Dropout,
    /// Skip connection
    SkipConnection,
}

/// Activation functions for NAS search
#[derive(Debug, Clone, Copy, Serialize, Deserialize, PartialEq, Eq)]
pub enum ActivationFunction {
    /// `ReLU`
    Relu,
    /// Sigmoid
    Sigmoid,
    /// Tanh
    Tanh,
    /// Softmax
    Softmax,
    /// Leaky `ReLU`
    LeakyRelu,
    /// ELU
    Elu,
    /// Swish
    Swish,
    /// GELU
    Gelu,
}

/// NAS search strategies
#[derive(Debug, Clone, Copy, Serialize, Deserialize, PartialEq, Eq)]
pub enum NasSearchStrategy {
    /// Random search
    RandomSearch,
    /// Evolutionary search
    EvolutionarySearch,
    /// Reinforcement learning
    ReinforcementLearning,
    /// Differentiable architecture search (DARTS)
    DifferentiableSearch,
    /// Progressive search
    ProgressiveSearch,
}

/// Performance estimation configuration
#[derive(Debug, Clone, Copy, Serialize, Deserialize)]
pub struct PerformanceEstimation {
    /// Estimation method
    pub method: PerformanceEstimationMethod,
    /// Early stopping criteria
    pub early_stopping: Option<EarlyStoppingCriteria>,
    /// Resource constraints
    pub resource_constraints: ResourceConstraints,
}

/// Performance estimation methods
#[derive(Debug, Clone, Copy, Serialize, Deserialize, PartialEq, Eq)]
pub enum PerformanceEstimationMethod {
    /// Full training
    FullTraining,
    /// Early stopping
    EarlyStopping,
    /// Learning curve extrapolation
    LearningCurveExtrapolation,
    /// Network morphism
    NetworkMorphism,
    /// Weight inheritance
    WeightInheritance,
}

/// Early stopping criteria for NAS
#[derive(Debug, Clone, Copy, Serialize, Deserialize)]
pub struct EarlyStoppingCriteria {
    /// Minimum training epochs
    pub min_epochs: u32,
    /// Maximum training epochs
    pub max_epochs: u32,
    /// Patience
    pub patience: u32,
    /// Performance threshold
    pub performance_threshold: f64,
}

/// Resource constraints for training
#[derive(Debug, Clone, Copy, Serialize, Deserialize)]
pub struct ResourceConstraints {
    /// Maximum training time
    pub max_training_time: Duration,
    /// Maximum memory in GB
    pub max_memory_gb: f64,
    /// Maximum GPU count
    pub max_gpu_count: u32,
    /// Maximum CPU cores
    pub max_cpu_cores: u32,
}

/// Machine learning model types
#[derive(Debug, Clone, Copy, Serialize, Deserialize)]
pub enum ModelType {
    /// Linear regression
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

/// Training pipeline phases
#[derive(Debug, Clone, Copy, Serialize, Deserialize)]
pub enum TrainingPhase {
    /// Data preparation
    DataPreparation,
    /// Feature engineering
    FeatureEngineering,
    /// Model selection
    ModelSelection,
    /// Hyperparameter tuning
    HyperparameterTuning,
    /// Model evaluation
    ModelEvaluation,
    /// Deployment
    Deployment,
}

/// Optimizer types (for meta-learning inner/outer loops)
#[derive(Debug, Clone, Copy, Serialize, Deserialize, PartialEq, Eq)]
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
}

/// Data normalization strategies
#[derive(Debug, Clone, Copy, Serialize, Deserialize, PartialEq, Eq)]
pub enum NormalizationStrategy {
    /// Min-max normalization to [0, 1]
    MinMax,
    /// Z-score normalization to mean=0, std=1
    ZScore,
    /// Robust scaling using median and IQR
    Robust,
    /// Unit vector scaling
    UnitVector,
    /// No normalization
    None,
}

/// Optimization direction for objectives
#[derive(Debug, Clone, Copy, Serialize, Deserialize, PartialEq, Eq)]
pub enum OptimizationDirection {
    /// Minimize the objective
    Minimize,
    /// Maximize the objective
    Maximize,
}

/// Optimization result
#[derive(Debug, Clone)]
pub struct OptimizationResult {
    /// Best parameter values found
    pub best_params: Vec<f64>,
    /// Best objective value achieved
    pub best_value: f64,
    /// Number of iterations performed
    pub iterations: u32,
    /// Whether optimization converged
    pub converged: bool,
}

/// Hyperparameter optimizer engine
#[derive(Debug)]
pub struct HyperparameterOptimizer {
    /// Unique identifier
    pub id: String,
    /// Optimization method
    pub method: HyperparameterOptimizationMethod,
    /// Completed trials count
    pub completed_trials: u32,
    /// Best trial result
    pub best_trial: Option<TrialResult>,
}

/// Result of a single hyperparameter trial
#[derive(Debug, Clone)]
pub struct TrialResult {
    /// Trial identifier
    pub id: String,
    /// Parameters tested
    pub params: HashMap<String, f64>,
    /// Objective value achieved
    pub objective_value: f64,
    /// Trial duration
    pub duration: Duration,
}

/// Model validation strategies
#[derive(Debug, Clone, Copy, Serialize, Deserialize)]
pub enum ValidationStrategy {
    /// Hold-out validation
    HoldOut,
    /// K-fold cross-validation
    CrossValidation,
    /// Time series-aware cross-validation
    TimeSeriesCrossValidation,
    /// Bootstrap resampling
    Bootstrap,
    /// Monte Carlo cross-validation
    MonteCarlo,
}
