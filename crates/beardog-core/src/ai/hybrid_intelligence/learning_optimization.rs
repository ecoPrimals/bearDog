// SPDX-License-Identifier: AGPL-3.0-or-later

use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use std::time::Duration;

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
