// SPDX-License-Identifier: AGPL-3.0-or-later

// Learning systems and optimization algorithms
//
// ⚠️ DEPRECATED: This module is being migrated to the canonical location.
//
// **New Location**: `beardog_types::canonical::config::domains::ai_config`
//
// These types will be removed in v3.3.0 (Q1 2026). Import from
// `beardog_types::canonical::config::domains::ai_config` (OnlineLearningConfig, transfer/meta/ensemble configs, etc.).
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

#[path = "learning_optimization.rs"]
mod learning_optimization;
pub use learning_optimization::*;

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
            enabled: beardog_errors::process_env::var("BEARDOG_AI_ONLINE_LEARNING_ENABLED")
                .ok()
                .and_then(|s| s.parse().ok())
                .unwrap_or(false),
            learning_rate: beardog_errors::process_env::var("BEARDOG_AI_ONLINE_LEARNING_RATE")
                .ok()
                .and_then(|s| s.parse().ok())
                .unwrap_or(0.001),
            batch_size: beardog_errors::process_env::var("BEARDOG_AI_ONLINE_BATCH_SIZE")
                .ok()
                .and_then(|s| s.parse().ok())
                .unwrap_or(32),
            update_frequency: beardog_errors::process_env::var(
                "BEARDOG_AI_ONLINE_UPDATE_FREQUENCY",
            )
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
