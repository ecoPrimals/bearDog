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
#[derive(Debug, Clone, Serialize, Deserialize)]
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
            enabled: false,
            learning_rate: 0.001,
            batch_size: 32,
            update_frequency: 100,
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
#[derive(Debug, Clone, Serialize, Deserialize)]
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

/// Prediction models available
#[derive(Debug, Clone, Copy, Serialize, Deserialize, PartialEq, Eq)]
pub enum PredictionModel {
    TimeSeries,
    /// Regression model
    Regression,
    /// Classification model
    Classification,
    /// Anomaly detection model
    AnomalyDetection,
    /// Clustering model
    Clustering,
    /// Recommendation model
    Recommendation,
    /// Survival analysis model
    SurvivalAnalysis,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct EnsembleConfig {
    /// Base models
    /// Collection of base models
    pub base_models: Vec<BaseModel>,
    /// Ensemble method
    /// The ensemble method value
    pub ensemble_method: EnsembleMethod,
    /// Model weights
    /// Optional model weights
    pub model_weights: Option<HashMap<String, f64>>,
    /// Diversity measures
    /// Collection of diversity measures
    pub diversity_measures: Vec<DiversityMeasure>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct BaseModel {
    /// Model identifier
    pub id: String,
    /// Model type
    /// The model type value
    pub model_type: ModelType,
    /// Model configuration
    pub configuration: HashMap<String, serde_json::Value>,
    /// Model weight
    /// The weight value
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
    RandomForest,
}

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

/// Prediction horizons
#[derive(Debug, Clone, Copy, Serialize, Deserialize, PartialEq, Eq)]
pub enum PredictionHorizon {
    /// Short-term prediction
    ShortTerm,
    /// Medium-term prediction
    MediumTerm,
    /// Long-term prediction
    LongTerm,
    /// Custom horizon
    Custom(Duration),
}

/// Optimization algorithms
#[derive(Debug, Clone, Copy, Serialize, Deserialize, PartialEq, Eq)]
pub enum OptimizationAlgorithm {
    /// Gradient descent
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

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ConstraintConfig {
    /// Equality constraints
    /// Collection of equality constraints
    pub equality_constraints: Vec<Constraint>,
    /// Inequality constraints
    /// Collection of inequality constraints
    pub inequality_constraints: Vec<Constraint>,
    /// Bound constraints
    /// Collection of bound constraints
    pub bound_constraints: Vec<BoundConstraint>,
}

/// Optimization constraint
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Constraint {
    /// Constraint identifier
    pub id: String,
    /// Constraint expression
    /// The expression value
    pub expression: String,
    /// Constraint tolerance
    /// The tolerance value
    pub tolerance: f64,
}

/// Bound constraint
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct BoundConstraint {
    /// Variable name
    /// The variable value
    pub variable: String,
    /// Lower bound
    /// Optional lower bound
    pub lower_bound: Option<f64>,
    /// Upper bound
    /// Optional upper bound
    pub upper_bound: Option<f64>,
}

/// Hyperparameter optimization configuration
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct HyperparameterOptimization {
    /// Optimization method
    /// The method value
    pub method: HyperparameterOptimizationMethod,
    /// Maximum trials
    /// Number of `max_trials`
    pub max_trials: u32,
    /// Optimization timeout in seconds
    pub timeout_secs: u64,
    /// Objective metric
    /// The objective metric value
    pub objective_metric: String,
    /// Optimization direction
    /// The optimization direction value
    pub optimization_direction: OptimizationDirection,
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
    /// Optuna
    Optuna,
}

/// Neural architecture search configuration
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct NeuralArchitectureSearch {
    /// Search space
    /// The search space value
    pub search_space: SearchSpace,
    /// Search strategy
    /// The search strategy value
    pub search_strategy: NasSearchStrategy,
    pub performance_estimation: PerformanceEstimation,
}

/// Neural architecture search space
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SearchSpace {
    /// Layer types to consider
    /// Collection of layer types
    pub layer_types: Vec<LayerType>,
    /// Depth range
    /// The depth range value
    pub depth_range: (u32, u32),
    /// Width range
    pub width_range: (u32, u32),
    /// Activation functions to consider
    /// Collection of activation functions
    pub activation_functions: Vec<ActivationFunction>,
}

#[derive(Debug, Clone, Copy, Serialize, Deserialize, PartialEq, Eq)]
/// Types of layer
pub enum LayerType {
    /// Dense layer
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

#[derive(Debug, Clone, Copy, Serialize, Deserialize, PartialEq, Eq)]
pub enum ActivationFunction {
    /// `ReLU` activation
    Relu,
    /// Sigmoid activation
    Sigmoid,
    /// Tanh activation
    Tanh,
    /// Softmax activation
    Softmax,
    /// Leaky `ReLU` activation
    LeakyRelu,
    /// ELU activation
    Elu,
    /// Swish activation
    Swish,
    /// GELU activation
    Gelu,
}

/// NAS search strategies
#[derive(Debug, Clone, Copy, Serialize, Deserialize, PartialEq, Eq)]
pub enum NasSearchStrategy {
    /// Random search
    RandomSearch,
    /// Evolutionary search
    EvolutionarySearch,
    ReinforcementLearning,
    /// Differentiable architecture search
    DifferentiableSearch,
    /// Progressive search
    ProgressiveSearch,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PerformanceEstimation {
    /// Estimation method
    /// The method value
    pub method: PerformanceEstimationMethod,
    /// Early stopping criteria
    /// Optional early stopping
    pub early_stopping: Option<EarlyStoppingCriteria>,
    /// Resource constraints
    /// The resource constraints value
    pub resource_constraints: ResourceConstraints,
}

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

#[derive(Debug, Clone, Copy, Serialize, Deserialize)]
pub struct EarlyStoppingCriteria {
    /// Minimum epochs
    /// Number of `min_epochs`
    pub min_epochs: u32,
    /// Maximum epochs
    /// Number of `max_epochs`
    pub max_epochs: u32,
    /// Patience
    /// Number of patience
    pub patience: u32,
    pub performance_threshold: f64,
}

#[derive(Debug, Clone, Copy, Serialize, Deserialize)]
pub struct ResourceConstraints {
    /// Maximum training time
    pub max_training_time: Duration,
    /// Maximum memory usage
    /// The max memory gb value
    pub max_memory_gb: f64,
    /// Maximum GPU usage
    /// Number of `max_gpu`
    pub max_gpu_count: u32,
    /// Maximum CPU cores
    /// Number of `max_cpu_cores`
    pub max_cpu_cores: u32,
}

#[derive(Debug, Clone, Copy, Serialize, Deserialize)]
/// Types of model
pub enum ModelType {
    /// Linear regression model
    Linear,
    /// Logistic regression model
    Logistic,
    /// Decision tree model
    DecisionTree,
    RandomForest,
    /// Support vector machine model
    SVM,
    /// Neural network model
    NeuralNetwork,
    /// Ensemble model combining multiple approaches
    Ensemble,
}

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

/// Optimizer types
#[derive(Debug, Clone, Copy, Serialize, Deserialize, PartialEq, Eq)]
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
}

/// Normalization strategies
#[derive(Debug, Clone, Copy, Serialize, Deserialize, PartialEq, Eq)]
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

/// Optimization directions
#[derive(Debug, Clone, Copy, Serialize, Deserialize, PartialEq, Eq)]
pub enum OptimizationDirection {
    /// Minimize objective
    Minimize,
    /// Maximize objective
    Maximize,
}

/// Optimization result
#[derive(Debug, Clone)]
pub struct OptimizationResult {
    /// Best parameters found
    /// Collection of best params
    pub best_params: Vec<f64>,
    /// Best objective value
    /// The best value value
    pub best_value: f64,
    /// Number of iterations
    /// Number of iterations
    pub iterations: u32,
    /// Convergence status
    /// Whether converged is enabled
    pub converged: bool,
}

#[derive(Debug)]
pub struct HyperparameterOptimizer {
    /// Optimizer ID
    pub id: String,
    /// Optimization method
    /// The method value
    pub method: HyperparameterOptimizationMethod,
    /// Completed trials
    /// Number of `completed_trials`
    pub completed_trials: u32,
    /// Best trial result
    /// Optional best trial
    pub best_trial: Option<TrialResult>,
}

/// Trial result
#[derive(Debug, Clone)]
pub struct TrialResult {
    /// Trial ID
    pub id: String,
    /// Parameters tested
    /// Mapping of params
    pub params: HashMap<String, f64>,
    /// Objective value achieved
    /// The objective value value
    pub objective_value: f64,
    /// Trial duration
    /// The duration value
    pub duration: Duration,
}

#[derive(Debug, Clone, Copy, Serialize, Deserialize)]
pub enum ValidationStrategy {
    /// Hold-out validation
    HoldOut,
    /// Cross-validation
    CrossValidation,
    /// Time series cross-validation
    TimeSeriesCrossValidation,
    /// Bootstrap validation
    Bootstrap,
    /// Monte Carlo cross-validation
    MonteCarlo,
}
