// SPDX-License-Identifier: AGPL-3.0-or-later

// ML Models - Modern Implementation
//
// **MODERNIZED**: Clean, production-ready ML model types for the BearDog threat detection engine.

use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};
use std::collections::BTreeMap;

/// Full-fidelity ML model record including metrics, features, and lifecycle timestamps.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct MlModel {
    /// Stable model id used in registries and predictions.
    pub id: String,
    /// Name of the item
    pub name: String,
    /// The description value
    pub description: String,
    /// The model type value
    pub model_type: MlModelType,
    /// The version value
    pub version: String,
    /// The accuracy value
    pub accuracy: f64,
    /// The precision value
    pub precision: f64,
    /// The recall value
    pub recall: f64,
    /// The f1 score value
    pub f1_score: f64,
    /// Number of `training_data_size`
    pub training_data_size: usize,
    /// Number of feature
    pub feature_count: usize,
    /// Collection of features
    pub features: Vec<String>,
    /// Name of the features
    pub feature_names: Vec<String>, // Alias for compatibility
    /// The created at value
    pub created_at: DateTime<Utc>,
    /// The last trained value
    pub last_trained: DateTime<Utc>,
    /// The last updated value
    pub last_updated: DateTime<Utc>,
    /// Whether `is_active` is enabled
    pub is_active: bool,
    /// Latest offline evaluation metrics for this artifact.
    pub performance_metrics: ModelPerformanceMetrics,
    /// Mapping of metadata
    pub metadata: BTreeMap<String, String>,
}

/// Types of machine learning models
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
/// Types of ml model
pub enum MlModelType {
    /// Classification model
    Classification,

    /// Regression model
    Regression,

    /// Clustering model
    Clustering,

    /// Anomaly detection model
    AnomalyDetection,

    /// Natural language processing model
    NaturalLanguageProcessing,

    /// Time series analysis model
    TimeSeries,

    /// Deep learning neural network
    DeepLearning,

    /// Ensemble model combining multiple approaches
    Ensemble,

    /// Custom model type
    Custom(String),
}

/// Offline evaluation metrics bundled with [`MlModel::performance_metrics`].
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ModelPerformanceMetrics {
    /// The accuracy value
    pub accuracy: f64,
    /// The precision value
    pub precision: f64,
    /// The recall value
    pub recall: f64,
    /// The f1 score value
    pub f1_score: f64,
    /// The specificity value
    pub specificity: f64,
    /// The sensitivity value
    pub sensitivity: f64,
    /// The auc roc value
    pub auc_roc: f64,
    /// The confusion matrix value
    pub confusion_matrix: ConfusionMatrix,
    /// Mean cross-validation score from the last training run.
    pub cross_validation_score: f64,
    /// Wall-clock time spent training the most recent artifact.
    pub training_time_ms: u64,
    /// Typical single-sample inference latency measured during validation.
    pub inference_time_ms: u64,
    /// Number of `model_size_bytes`
    pub model_size_bytes: u64,
}

/// Standard TP/TN/FP/FN counts for classifier evaluation.
#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct ConfusionMatrix {
    /// Number of `true_positives`
    pub true_positives: u64,
    /// Number of `true_negatives`
    pub true_negatives: u64,
    /// Number of `false_positives`
    pub false_positives: u64,
    /// Number of `false_negatives`
    pub false_negatives: u64,
}

/// Model prediction result
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ModelPrediction {
    /// Id of the model that produced this output.
    pub model_id: String,
    /// The prediction value
    pub prediction: PredictionValue,
    /// Normalized confidence for the top prediction or marginal.
    pub confidence: f64,
    /// Mapping of probability scores
    pub probability_scores: BTreeMap<String, f64>,
    /// Mapping of feature importance
    pub feature_importance: BTreeMap<String, f64>,
    /// Time spent computing this prediction on the serving path.
    pub prediction_time_ms: u64,
    /// The model version value
    pub model_version: String,
    /// When the prediction was emitted.
    pub timestamp: DateTime<Utc>,
}

/// Prediction value types
#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum PredictionValue {
    /// Binary classification result
    Binary(bool),

    /// Multi-class classification result
    Class(String),

    /// Numeric regression result
    Numeric(f64),

    /// Probability distribution
    Probability(BTreeMap<String, f64>),

    /// Anomaly score
    AnomalyScore(f64),

    /// Opaque JSON payload for user-defined model heads.
    Custom(serde_json::Value),
}

/// Model training configuration
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ModelTrainingConfig {
    /// The algorithm value
    pub algorithm: String,
    /// Mapping of hyperparameters
    pub hyperparameters: BTreeMap<String, serde_json::Value>,
    /// The training data path value
    pub training_data_path: String,
    /// Fraction of data reserved for validation (0.0–1.0).
    pub validation_split: f64,
    /// The test split value
    /// Holdout fraction for final testing.
    pub test_split: f64,
    /// Number of folds when cross-validation is enabled.
    pub cross_validation_folds: u32,
    /// Hard cap on training duration to bound operational cost.
    pub max_training_time_minutes: u32,
    /// Number of `early_stopping_patience`
    pub early_stopping_patience: u32,
    /// The target accuracy value
    pub target_accuracy: f64,
    /// Whether `feature_selection` is enabled
    pub feature_selection: bool,
    /// Whether `feature_engineering` is enabled
    pub feature_engineering: bool,
}

impl MlModel {
    /// Create a new ML model
    /// Creates a new instance
    #[must_use]
    pub fn new(
        name: &str,
        description: &str,
        model_type: MlModelType,
        features: Vec<String>,
    ) -> Self {
        let now = Utc::now();
        Self {
            id: uuid::Uuid::new_v4().to_string(),
            name: name.to_string(),
            description: description.to_string(),
            model_type,
            version: "1.0.0".to_string(),
            accuracy: 0.0,
            precision: 0.0,
            recall: 0.0,
            f1_score: 0.0,
            training_data_size: 0,
            feature_count: features.len(),
            features: features.clone(),
            feature_names: features,
            created_at: now,
            last_trained: now,
            last_updated: now,
            is_active: false,
            performance_metrics: ModelPerformanceMetrics::default(),
            metadata: BTreeMap::new(),
        }
    }

    /// Check if model is high accuracy
    /// Checks if high accuracy
    #[must_use]
    pub fn is_high_accuracy(&self) -> bool {
        self.accuracy > 0.85
    }

    /// Check if model needs retraining
    #[must_use]
    pub fn needs_retraining(&self) -> bool {
        let days_since_training = (Utc::now() - self.last_trained).num_days();
        days_since_training > 30 || self.accuracy < 0.7
    }

    /// Get model age in days
    /// Gets `age_days`
    #[must_use]
    pub fn get_age_days(&self) -> i64 {
        (Utc::now() - self.created_at).num_days()
    }

    /// Update model accuracy
    /// Updates accuracy
    pub fn update_accuracy(&mut self, accuracy: f64) {
        self.accuracy = accuracy;
        self.last_updated = Utc::now();
    }

    /// Update training timestamp
    /// Updates `training_timestamp`
    pub fn update_training_timestamp(&mut self) {
        self.last_trained = Utc::now();
        self.last_updated = Utc::now();
    }

    /// Activate the model
    pub fn activate(&mut self) {
        self.is_active = true;
        self.last_updated = Utc::now();
    }

    /// Deactivate the model
    pub fn deactivate(&mut self) {
        self.is_active = false;
        self.last_updated = Utc::now();
    }

    /// Add metadata
    pub fn add_metadata(&mut self, key: &str, value: &str) {
        self.metadata.insert(key.to_string(), value.to_string());
        self.last_updated = Utc::now();
    }

    /// Get model summary
    #[must_use]
    pub fn summary(&self) -> ModelSummary {
        ModelSummary {
            id: self.id.clone(),
            name: self.name.clone(),
            model_type: self.model_type.clone(),
            accuracy: self.accuracy,
            is_active: self.is_active,
            age_days: self.get_age_days(),
            feature_count: self.feature_count,
            last_trained: self.last_trained,
        }
    }
}

/// Compact dashboard view of a deployed model.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ModelSummary {
    /// Model id matching [`MlModel::id`].
    pub id: String,
    /// Name of the item
    pub name: String,
    /// The model type value
    pub model_type: MlModelType,
    /// The accuracy value
    pub accuracy: f64,
    /// Whether `is_active` is enabled
    pub is_active: bool,
    /// Number of `age_days`
    pub age_days: i64,
    /// Number of feature
    pub feature_count: usize,
    /// The last trained value
    pub last_trained: DateTime<Utc>,
}

impl Default for ModelPerformanceMetrics {
    fn default() -> Self {
        Self {
            accuracy: 0.0,
            precision: 0.0,
            recall: 0.0,
            f1_score: 0.0,
            specificity: 0.0,
            sensitivity: 0.0,
            auc_roc: 0.0,
            confusion_matrix: ConfusionMatrix::default(),
            cross_validation_score: 0.0,
            training_time_ms: 0,
            inference_time_ms: 0,
            model_size_bytes: 0,
        }
    }
}

impl ConfusionMatrix {
    /// Calculate accuracy from confusion matrix
    #[must_use]
    #[expect(
        clippy::cast_precision_loss,
        reason = "Confusion matrix counts as f64 ratio; acceptable for ML metrics"
    )]
    pub fn accuracy(&self) -> f64 {
        let total =
            self.true_positives + self.true_negatives + self.false_positives + self.false_negatives;
        if total == 0 {
            0.0
        } else {
            (self.true_positives + self.true_negatives) as f64 / total as f64
        }
    }

    /// Calculate precision from confusion matrix
    #[must_use]
    #[expect(
        clippy::cast_precision_loss,
        reason = "Confusion matrix counts as f64 ratio; acceptable for ML metrics"
    )]
    pub fn precision(&self) -> f64 {
        let predicted_positive = self.true_positives + self.false_positives;
        if predicted_positive == 0 {
            0.0
        } else {
            self.true_positives as f64 / predicted_positive as f64
        }
    }

    /// Calculate recall from confusion matrix
    #[must_use]
    #[expect(
        clippy::cast_precision_loss,
        reason = "Confusion matrix counts as f64 ratio; acceptable for ML metrics"
    )]
    pub fn recall(&self) -> f64 {
        let actual_positive = self.true_positives + self.false_negatives;
        if actual_positive == 0 {
            0.0
        } else {
            self.true_positives as f64 / actual_positive as f64
        }
    }

    /// Calculate F1 score from confusion matrix
    #[must_use]
    pub fn f1_score(&self) -> f64 {
        let precision = self.precision();
        let recall = self.recall();
        if precision + recall == 0.0 {
            0.0
        } else {
            2.0 * (precision * recall) / (precision + recall)
        }
    }
}

impl ModelPrediction {
    /// Create a new model prediction
    /// Creates a new instance
    #[must_use]
    pub fn new(model_id: &str, prediction: PredictionValue, confidence: f64) -> Self {
        Self {
            model_id: model_id.to_string(),
            prediction,
            confidence,
            probability_scores: BTreeMap::new(),
            feature_importance: BTreeMap::new(),
            prediction_time_ms: 0,
            model_version: "1.0.0".to_string(),
            timestamp: Utc::now(),
        }
    }

    /// Check if prediction is high confidence
    /// Checks if high confidence
    #[must_use]
    pub fn is_high_confidence(&self) -> bool {
        self.confidence > 0.8
    }

    /// Inserts or replaces a class label’s marginal probability.
    pub fn add_probability_score(&mut self, class: &str, score: f64) {
        self.probability_scores.insert(class.to_string(), score);
    }

    /// Add feature importance score
    pub fn add_feature_importance(&mut self, feature: &str, importance: f64) {
        self.feature_importance
            .insert(feature.to_string(), importance);
    }
}

impl std::fmt::Display for MlModelType {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::Classification => write!(f, "Classification"),
            Self::Regression => write!(f, "Regression"),
            Self::Clustering => write!(f, "Clustering"),
            Self::AnomalyDetection => write!(f, "Anomaly Detection"),
            Self::NaturalLanguageProcessing => write!(f, "Natural Language Processing"),
            Self::TimeSeries => write!(f, "Time Series"),
            Self::DeepLearning => write!(f, "Deep Learning"),
            Self::Ensemble => write!(f, "Ensemble"),
            Self::Custom(name) => write!(f, "Custom: {name}"),
        }
    }
}

#[allow(
    unused_imports,
    clippy::module_inception,
    clippy::manual_range_contains,
    clippy::assertions_on_constants,
    clippy::useless_vec,
    clippy::absurd_extreme_comparisons,
    unused_comparisons,
    reason = "ML model unit tests: exhaustive patterns (multi-lint expect causes unfulfilled_lint_expectations)"
)]
#[cfg(test)]
mod tests {
    use super::*;
    use crate::float_assert::near_f64;

    #[test]
    fn test_ml_model_creation() {
        let features = vec!["feature1".to_string(), "feature2".to_string()];
        let model = MlModel::new(
            "Test Model",
            "A test machine learning model",
            MlModelType::Classification,
            features,
        );

        assert_eq!(model.name, "Test Model");
        assert_eq!(model.model_type, MlModelType::Classification);
        assert_eq!(model.feature_count, 2);
        assert!(!model.is_active);
        assert!(!model.is_high_accuracy()); // Default accuracy is 0.0
    }

    #[test]
    // TEST_CATEGORY: unit
    // TEST_DOMAIN: core
    // TEST_PRIORITY: normal
    fn test_model_accuracy_update() {
        let mut model = MlModel::new(
            "Test Model",
            "Test",
            MlModelType::Classification,
            vec!["feature1".to_string()],
        );

        assert!(!model.is_high_accuracy());

        model.update_accuracy(0.9);
        assert!(model.is_high_accuracy());
        near_f64(model.accuracy, 0.9);
    }
    // TEST_CATEGORY: unit
    // TEST_DOMAIN: core
    // TEST_PRIORITY: normal

    #[test]
    fn test_model_retraining_check() {
        let mut model = MlModel::new(
            "Test Model",
            "Test",
            MlModelType::Classification,
            vec!["feature1".to_string()],
        );

        // Low accuracy should trigger retraining
        model.update_accuracy(0.5);
        assert!(model.needs_retraining());
        // TEST_CATEGORY: unit
        // TEST_DOMAIN: core
        // TEST_PRIORITY: normal

        // High accuracy should not trigger retraining (for recent models)
        model.update_accuracy(0.95);
        assert!(!model.needs_retraining());
    }

    #[test]
    fn test_confusion_matrix_calculations() {
        let matrix = ConfusionMatrix {
            true_positives: 80,
            true_negatives: 70,
            false_positives: 10,
            false_negatives: 20,
        };

        // TEST_CATEGORY: unit
        // TEST_DOMAIN: core
        // TEST_PRIORITY: normal
        near_f64(matrix.accuracy(), 150.0 / 180.0);
        near_f64(matrix.precision(), 80.0 / 90.0);
        near_f64(matrix.recall(), 80.0 / 100.0);

        let expected_f1 =
            2.0 * (matrix.precision() * matrix.recall()) / (matrix.precision() + matrix.recall());
        near_f64(matrix.f1_score(), expected_f1);
    }

    #[test]
    fn test_model_prediction() {
        let mut prediction = ModelPrediction::new("model-1", PredictionValue::Binary(true), 0.85);

        assert!(prediction.is_high_confidence());

        // TEST_CATEGORY: unit
        // TEST_DOMAIN: core
        // TEST_PRIORITY: normal
        prediction.add_probability_score("positive", 0.85);
        prediction.add_probability_score("negative", 0.15);

        assert_eq!(prediction.probability_scores.len(), 2);
        assert_eq!(prediction.probability_scores.get("positive"), Some(&0.85));
    }

    #[test]
    fn test_model_type_display() {
        assert_eq!(MlModelType::Classification.to_string(), "Classification");
        // TEST_CATEGORY: unit
        // TEST_DOMAIN: core
        // TEST_PRIORITY: normal
        assert_eq!(
            MlModelType::AnomalyDetection.to_string(),
            "Anomaly Detection"
        );
        assert_eq!(
            MlModelType::Custom("XGBoost".to_string()).to_string(),
            "Custom: XGBoost"
        );
    }

    // TEST_CATEGORY: unit
    // TEST_DOMAIN: core
    // TEST_PRIORITY: normal
    #[test]
    fn test_model_activation() {
        let mut model = MlModel::new(
            "Test Model",
            "Test",
            MlModelType::Classification,
            vec!["feature1".to_string()],
        );

        assert!(!model.is_active);

        model.activate();
        assert!(model.is_active);

        model.deactivate();
        assert!(!model.is_active);
    }
}
