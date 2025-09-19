//! Core types for the hybrid intelligence system

use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use std::time::Duration;

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

/// AI intelligence capabilities available in the hybrid system
#[derive(Debug, Clone, Copy, Serialize, Deserialize)]
pub enum IntelligenceCapability {
    /// Predictive analytics
    PredictiveAnalytics,
    /// Anomaly detection
    AnomalyDetection,
    /// Pattern recognition
    PatternRecognition,
    /// Natural language processing
    NaturalLanguageProcessing,
    /// Computer vision
    ComputerVision,
    ReinforcementLearning,
    /// Decision trees
    DecisionTrees,
    /// Neural networks
    NeuralNetworks,
    /// Genetic algorithms
    GeneticAlgorithms,
    /// Fuzzy logic
    FuzzyLogic,
    /// Expert systems
    ExpertSystems,
    /// Automated reasoning
    AutomatedReasoning,
}

/// Machine learning model types
#[derive(Debug, Clone, Copy, Serialize, Deserialize)]
/// Types of model
pub enum ModelType {
    /// Linear regression
    LinearRegression,
    /// Logistic regression
    LogisticRegression,
    /// Decision tree
    DecisionTree,
    /// Random forest
    RandomForest,
    /// Support vector machine
    SVM,
    /// Neural network
    NeuralNetwork,
    /// Gradient boosting
    GradientBoosting,
    /// Clustering
    Clustering,
    /// Deep learning
    DeepLearning,
    /// Reinforcement learning
    ReinforcementLearning,
}

/// Machine learning configuration
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct MachineLearningConfig {
    /// Model type to use
    pub model_type: ModelType,
    /// Training parameters
    pub training_params: TrainingParams,
    /// Network architecture for neural networks
    pub network_architecture: Option<NetworkArchitecture>,
    /// Network optimization settings
    pub network_optimization: Option<NetworkOptimization>,
    /// Network regularization settings
    pub network_regularization: Option<NetworkRegularization>,
    /// Learning algorithm configuration
    pub learning_algorithm: LearningAlgorithmType,
    /// Online learning configuration
    pub online_learning: Option<OnlineLearningConfig>,
    /// Meta-learning configuration
    pub meta_learning: Option<MetaLearningConfig>,
    /// Transfer learning configuration
    pub transfer_learning: Option<TransferLearningConfig>,
    /// Ensemble configuration
    pub ensemble: Option<EnsembleConfig>,
    /// Hyperparameter optimization
    pub hyperparameter_optimization: Option<HyperparameterOptimization>,
    /// Constraint configuration
    pub constraints: Option<ConstraintConfig>,
    /// Prediction model settings
    pub prediction_model: Option<PredictionModel>,
    /// Prediction horizon
    pub prediction_horizon: Option<PredictionHorizon>,
    /// Optimization algorithm
    pub optimization_algorithm: Option<OptimizationAlgorithm>,
} 