// SPDX-License-Identifier: AGPL-3.0-only

//! Processing configuration types for the hybrid intelligence system
//!
//! This module contains all processing-related configuration types including
//! preprocessing, neural network, decision engine, and learning configurations.

use serde::{Deserialize, Serialize};
use std::collections::HashMap;

// MIGRATED: Using canonical config types from beardog-types
use beardog_types::canonical::config::domains::ai_config::{
    OnlineLearningConfig, TransferLearningConfig, MetaLearningConfig,
};
use crate::ai::hybrid_intelligence::decision_engine::{
    ConsensusStrategy, DecisionCriteria, DecisionStrategy,
};
use crate::ai::hybrid_intelligence::learning::{
    ConstraintConfig, EnsembleConfig, HyperparameterOptimization, LearningAlgorithmType,
};
use crate::ai::hybrid_intelligence::neural_networks::{
    NetworkArchitecture, NetworkOptimization, NetworkRegularization, TrainingParams,
};

/// Preprocessing configuration
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PreprocessingConfig {
    /// Enable normalization
    pub normalize: bool,
    /// Normalization method
    pub normalization_method: NormalizationMethod,
    /// Enable feature scaling
    pub scale_features: bool,
    /// Handle missing values
    pub handle_missing: bool,
    /// Missing value strategy
    pub missing_value_strategy: MissingValueStrategy,
    /// Feature selection
    pub feature_selection: Option<FeatureSelectionConfig>,
}

impl Default for PreprocessingConfig {
    fn default() -> Self {
        Self {
            normalize: true,
            normalization_method: NormalizationMethod::StandardScaling,
            scale_features: true,
            handle_missing: true,
            missing_value_strategy: MissingValueStrategy::Mean,
            feature_selection: None,
        }
    }
}

/// Normalization methods
#[derive(Debug, Clone, Copy, Serialize, Deserialize)]
pub enum NormalizationMethod {
    /// Standard scaling (z-score normalization)
    StandardScaling,
    /// Min-max normalization
    MinMaxScaling,
    /// Robust scaling
    RobustScaling,
    /// Unit vector scaling
    UnitVector,
}

impl Default for NormalizationMethod {
    fn default() -> Self {
        Self::StandardScaling
    }
}

/// Missing value handling strategies
#[derive(Debug, Clone, Copy, Serialize, Deserialize)]
pub enum MissingValueStrategy {
    /// Fill with mean value
    Mean,
    /// Fill with median value
    Median,
    /// Fill with mode (most frequent value)
    Mode,
    /// Fill with constant value
    Constant,
    /// Forward fill
    ForwardFill,
    /// Backward fill
    BackwardFill,
    /// Linear interpolation
    LinearInterpolation,
    /// Remove rows with missing values
    Remove,
}

impl Default for MissingValueStrategy {
    fn default() -> Self {
        Self::Mean
    }
}

/// Feature selection configuration
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct FeatureSelectionConfig {
    /// Enable feature selection
    pub enabled: bool,
    /// Selection method
    pub method: FeatureSelectionMethod,
    /// Number of features to select
    pub num_features: Option<u32>,
    /// Selection threshold
    pub threshold: Option<f64>,
}

/// Feature selection methods
#[derive(Debug, Clone, Copy, Serialize, Deserialize)]
pub enum FeatureSelectionMethod {
    /// Univariate statistical tests
    Univariate,
    /// Recursive feature elimination
    RFE,
    /// L1-based feature selection
    L1Based,
    /// Tree-based feature importance
    TreeBased,
    /// Principal component analysis
    PCA,
}

/// Neural network configuration
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct NeuralNetworkConfig {
    /// Network architecture
    pub architecture: NetworkArchitecture,
    /// Training parameters
    pub training: TrainingParams,
    /// Optimization settings
    pub optimization: NetworkOptimization,
    /// Regularization settings
    pub regularization: NetworkRegularization,
    /// Enable GPU acceleration
    pub use_gpu: bool,
    /// Mixed precision training
    pub mixed_precision: bool,
    /// Gradient clipping
    pub gradient_clipping: Option<f64>,
}

impl Default for NeuralNetworkConfig {
    fn default() -> Self {
        Self {
            architecture: NetworkArchitecture::default(),
            training: TrainingParams::default(),
            optimization: NetworkOptimization::default(),
            regularization: NetworkRegularization::default(),
            use_gpu: false,
            mixed_precision: false,
            gradient_clipping: Some(1.0),
        }
    }
}

/// Decision engine configuration
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct DecisionEngineConfig {
    /// Decision strategy
    pub strategy: DecisionStrategy,
    /// Decision criteria
    pub criteria: Vec<DecisionCriteria>,
    /// Consensus strategy for multiple models
    pub consensus: ConsensusStrategy,
    /// Confidence threshold for decisions
    pub confidence_threshold: f64,
    /// Enable uncertainty quantification
    pub uncertainty_quantification: bool,
}

impl Default for DecisionEngineConfig {
    fn default() -> Self {
        Self {
            strategy: DecisionStrategy::default(),
            criteria: vec![DecisionCriteria::default()],
            consensus: ConsensusStrategy::default(),
            confidence_threshold: 0.8,
            uncertainty_quantification: true,
        }
    }
}

/// Learning configuration
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct LearningConfig {
    /// Learning algorithm type
    pub algorithm_type: LearningAlgorithmType,
    /// Online learning configuration
    pub online_learning: Option<OnlineLearningConfig>,
    /// Transfer learning configuration
    pub transfer_learning: Option<TransferLearningConfig>,
    /// Meta-learning configuration
    pub meta_learning: Option<MetaLearningConfig>,
    /// Ensemble configuration
    pub ensemble: Option<EnsembleConfig>,
    /// Hyperparameter optimization
    pub hyperparameter_optimization: Option<HyperparameterOptimization>,
    /// Constraints configuration
    pub constraints: Option<ConstraintConfig>,
}

impl Default for LearningConfig {
    fn default() -> Self {
        Self {
            algorithm_type: LearningAlgorithmType::default(),
            online_learning: None,
            transfer_learning: None,
            meta_learning: None,
            ensemble: None,
            hyperparameter_optimization: None,
            constraints: None,
        }
    }
}

/// Data processing pipeline configuration
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct DataPipelineConfig {
    /// Input data format
    pub input_format: DataFormat,
    /// Output data format
    pub output_format: DataFormat,
    /// Batch size for processing
    pub batch_size: u32,
    /// Number of parallel workers
    pub num_workers: u32,
    /// Enable data validation
    pub validate_data: bool,
    /// Data validation rules
    pub validation_rules: Vec<ValidationRule>,
    /// Enable data augmentation
    pub augmentation: Option<DataAugmentationConfig>,
}

impl Default for DataPipelineConfig {
    fn default() -> Self {
        Self {
            input_format: DataFormat::CSV,
            output_format: DataFormat::JSON,
            batch_size: 1000,
            num_workers: 4,
            validate_data: true,
            validation_rules: Vec::new(),
            augmentation: None,
        }
    }
}

/// Supported data formats
#[derive(Debug, Clone, Copy, Serialize, Deserialize)]
pub enum DataFormat {
    /// Comma-separated values
    CSV,
    /// JavaScript Object Notation
    JSON,
    /// Apache Parquet
    Parquet,
    /// Apache Avro
    Avro,
    /// Protocol Buffers
    Protobuf,
    /// MessagePack
    MessagePack,
    /// Binary format
    Binary,
}

impl Default for DataFormat {
    fn default() -> Self {
        Self::JSON
    }
}

/// Data validation rule
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ValidationRule {
    /// Field name to validate
    pub field: String,
    /// Validation type
    pub validation_type: ValidationType,
    /// Expected value or range
    pub expected: ValidationExpected,
    /// Action to take on validation failure
    pub action: ValidationAction,
}

/// Validation types
#[derive(Debug, Clone, Copy, Serialize, Deserialize)]
pub enum ValidationType {
    /// Check data type
    DataType,
    /// Check value range
    Range,
    /// Check for null values
    NotNull,
    /// Check string format (regex)
    Format,
    /// Check uniqueness
    Unique,
    /// Custom validation
    Custom,
}

/// Expected validation values
#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum ValidationExpected {
    /// Expected data type
    DataType(String),
    /// Expected numeric range
    Range { min: f64, max: f64 },
    /// Expected string pattern
    Pattern(String),
    /// Expected boolean value
    Boolean(bool),
    /// Custom validation expression
    Custom(String),
}

/// Actions to take on validation failure
#[derive(Debug, Clone, Copy, Serialize, Deserialize)]
pub enum ValidationAction {
    /// Log warning and continue
    Warn,
    /// Reject the record
    Reject,
    /// Fix the value if possible
    Fix,
    /// Stop processing
    Stop,
}

/// Data augmentation configuration
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct DataAugmentationConfig {
    /// Enable augmentation
    pub enabled: bool,
    /// Augmentation techniques
    pub techniques: Vec<AugmentationTechnique>,
    /// Augmentation probability
    pub probability: f64,
    /// Number of augmented samples per original
    pub augmentation_factor: u32,
}

/// Data augmentation techniques
#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum AugmentationTechnique {
    /// Add Gaussian noise
    GaussianNoise { std: f64 },
    /// Random scaling
    Scaling { min_scale: f64, max_scale: f64 },
    /// Random rotation (for time series)
    Rotation { max_angle: f64 },
    /// Random time shift
    TimeShift { max_shift: i32 },
    /// Synthetic minority oversampling
    SMOTE { k_neighbors: u32 },
    /// Custom augmentation
    Custom { name: String, params: HashMap<String, f64> },
}

/// Model evaluation configuration
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct EvaluationConfig {
    /// Evaluation metrics
    pub metrics: Vec<EvaluationMetric>,
    /// Cross-validation configuration
    pub cross_validation: Option<CrossValidationConfig>,
    /// Test data split ratio
    pub test_split: f64,
    /// Enable stratified sampling
    pub stratified: bool,
    /// Random seed for reproducibility
    pub random_seed: Option<u64>,
}

impl Default for EvaluationConfig {
    fn default() -> Self {
        Self {
            metrics: vec![EvaluationMetric::Accuracy, EvaluationMetric::F1Score],
            cross_validation: Some(CrossValidationConfig::default()),
            test_split: 0.2,
            stratified: true,
            random_seed: Some(42),
        }
    }
}

/// Evaluation metrics
#[derive(Debug, Clone, Copy, Serialize, Deserialize)]
pub enum EvaluationMetric {
    /// Accuracy score
    Accuracy,
    /// Precision score
    Precision,
    /// Recall score
    Recall,
    /// F1 score
    F1Score,
    /// Area under ROC curve
    AUC,
    /// Mean absolute error
    MAE,
    /// Mean squared error
    MSE,
    /// Root mean squared error
    RMSE,
    /// R-squared score
    R2Score,
}

/// Cross-validation configuration
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CrossValidationConfig {
    /// Number of folds
    pub n_folds: u32,
    /// Cross-validation strategy
    pub strategy: CrossValidationStrategy,
    /// Shuffle data before splitting
    pub shuffle: bool,
    /// Random seed for shuffling
    pub random_seed: Option<u64>,
}

impl Default for CrossValidationConfig {
    fn default() -> Self {
        Self {
            n_folds: 5,
            strategy: CrossValidationStrategy::KFold,
            shuffle: true,
            random_seed: Some(42),
        }
    }
}

/// Cross-validation strategies
#[derive(Debug, Clone, Copy, Serialize, Deserialize)]
pub enum CrossValidationStrategy {
    /// K-fold cross-validation
    KFold,
    /// Stratified K-fold
    StratifiedKFold,
    /// Time series split
    TimeSeriesSplit,
    /// Leave-one-out
    LeaveOneOut,
    /// Leave-P-out
    LeavePOut { p: u32 },
}

impl Default for CrossValidationStrategy {
    fn default() -> Self {
        Self::KFold
    }
} 