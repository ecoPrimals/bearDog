//! Machine learning model types for threat detection
//!
//! This module contains types for managing machine learning models used in
//! threat detection, including model metadata, training information, and
//! performance metrics.

use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};

/// Machine learning model structure
///
/// Represents a machine learning model used for
/// threat detection and classification.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct MlModel {
    /// Model identifier
    pub id: String,

    /// Model name
    pub name: String,

    /// Model type
    pub model_type: MlModelType,

    /// Model accuracy
    pub accuracy: f64,

    /// Last training timestamp
    pub last_trained: DateTime<Utc>,

    /// Feature names
    pub feature_names: Vec<String>,
}

/// Machine learning model type enumeration
///
/// Categorizes different types of machine learning
/// models by their algorithm type.
#[derive(Debug, Clone, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub enum MlModelType {
    /// Anomaly detection model
    AnomalyDetection,

    /// Classification model
    Classification,

    /// Clustering model
    Clustering,

    /// Neural network model
    NeuralNetwork,

    /// Random forest model
    RandomForest,

    /// Support vector machine model
    SupportVector,

    /// Naive Bayes model
    NaiveBayes,
}

impl Default for MlModel {
    fn default() -> Self {
        Self {
            id: "default-model".to_string(),
            name: "Default Model".to_string(),
            model_type: MlModelType::AnomalyDetection,
            accuracy: 0.0,
            last_trained: Utc::now(),
            feature_names: Vec::new(),
        }
    }
}

impl MlModel {
    /// Create a new ML model
    ///
    /// # Arguments
    /// * `id` - Model identifier
    /// * `name` - Model name
    /// * `model_type` - Type of ML model
    /// * `feature_names` - Names of features used by the model
    ///
    /// # Returns
    /// New ML model instance
    ///
    /// # Example
    /// ```rust
    /// use beardog::threat::types::{MlModel, MlModelType};
    ///
    /// let model = MlModel::new(
    ///     "anomaly-detector-v1".to_string(),
    ///     "Network Anomaly Detector".to_string(),
    ///     MlModelType::AnomalyDetection,
    ///     vec!["packet_count".to_string(), "byte_count".to_string()],
    /// );
    /// ```
    pub fn new(
        id: String,
        name: String,
        model_type: MlModelType,
        feature_names: Vec<String>,
    ) -> Self {
        Self {
            id,
            name,
            model_type,
            accuracy: 0.0,
            last_trained: Utc::now(),
            feature_names,
        }
    }

    /// Update model accuracy
    ///
    /// # Arguments
    /// * `accuracy` - New accuracy value (0.0 to 1.0)
    ///
    /// # Example
    /// ```rust
    /// use beardog::threat::types::MlModel;
    ///
    /// let mut model = MlModel::default();
    /// model.update_accuracy(0.95);
    /// assert_eq!(model.accuracy, 0.95);
    /// ```
    pub fn update_accuracy(&mut self, accuracy: f64) {
        self.accuracy = accuracy.clamp(0.0, 1.0);
    }

    /// Mark model as recently trained
    ///
    /// # Example
    /// ```rust
    /// use beardog::threat::types::MlModel;
    ///
    /// let mut model = MlModel::default();
    /// model.mark_trained();
    /// ```
    pub fn mark_trained(&mut self) {
        self.last_trained = Utc::now();
    }

    /// Add a feature to the model
    ///
    /// # Arguments
    /// * `feature_name` - Name of the feature to add
    ///
    /// # Example
    /// ```rust
    /// use beardog::threat::types::MlModel;
    ///
    /// let mut model = MlModel::default();
    /// model.add_feature("request_frequency".to_string());
    /// ```
    pub fn add_feature(&mut self, feature_name: String) {
        if !self.feature_names.contains(&feature_name) {
            self.feature_names.push(feature_name);
        }
    }

    /// Remove a feature from the model
    ///
    /// # Arguments
    /// * `feature_name` - Name of the feature to remove
    ///
    /// # Example
    /// ```rust
    /// use beardog::threat::types::MlModel;
    ///
    /// let mut model = MlModel::default();
    /// model.add_feature("test_feature".to_string());
    /// model.remove_feature("test_feature");
    /// ```
    pub fn remove_feature(&mut self, feature_name: &str) {
        self.feature_names.retain(|f| f != feature_name);
    }

    /// Get feature count
    ///
    /// # Returns
    /// Number of features in the model
    ///
    /// # Example
    /// ```rust
    /// use beardog::threat::types::MlModel;
    ///
    /// let mut model = MlModel::default();
    /// model.add_feature("feature1".to_string());
    /// model.add_feature("feature2".to_string());
    /// assert_eq!(model.feature_count(), 2);
    /// ```
    pub fn feature_count(&self) -> usize {
        self.feature_names.len()
    }

    /// Check if model has a specific feature
    ///
    /// # Arguments
    /// * `feature_name` - Name of the feature to check
    ///
    /// # Returns
    /// `true` if the model has the feature
    ///
    /// # Example
    /// ```rust
    /// use beardog::threat::types::MlModel;
    ///
    /// let mut model = MlModel::default();
    /// model.add_feature("test_feature".to_string());
    /// assert!(model.has_feature("test_feature"));
    /// ```
    pub fn has_feature(&self, feature_name: &str) -> bool {
        self.feature_names.contains(&feature_name.to_string())
    }

    /// Check if model is well-trained
    ///
    /// # Returns
    /// `true` if model has good accuracy and features
    ///
    /// # Example
    /// ```rust
    /// use beardog::threat::types::MlModel;
    ///
    /// let mut model = MlModel::default();
    /// model.update_accuracy(0.95);
    /// model.add_feature("feature1".to_string());
    /// assert!(model.is_well_trained());
    /// ```
    pub fn is_well_trained(&self) -> bool {
        self.accuracy >= 0.8 && !self.feature_names.is_empty()
    }

    /// Check if model is recently trained
    ///
    /// # Returns
    /// `true` if model was trained within the last 30 days
    ///
    /// # Example
    /// ```rust
    /// use beardog::threat::types::MlModel;
    ///
    /// let model = MlModel::default();
    /// assert!(model.is_recently_trained());
    /// ```
    pub fn is_recently_trained(&self) -> bool {
        let now = Utc::now();
        let days_since_training = now.signed_duration_since(self.last_trained).num_days();
        days_since_training <= 30
    }

    /// Get model age in days
    ///
    /// # Returns
    /// Number of days since last training
    ///
    /// # Example
    /// ```rust
    /// use beardog::threat::types::MlModel;
    ///
    /// let model = MlModel::default();
    /// let age = model.get_age_days();
    /// ```
    pub fn get_age_days(&self) -> i64 {
        let now = Utc::now();
        now.signed_duration_since(self.last_trained).num_days()
    }

    /// Get age in days (alias for get_age_days)
    pub fn age_days(&self) -> i64 {
        self.get_age_days()
    }

    /// Check if model has high accuracy
    pub fn is_high_accuracy(&self) -> bool {
        self.accuracy >= 0.8
    }

    /// Update training timestamp
    pub fn update_training_timestamp(&mut self) {
        self.last_trained = Utc::now();
    }

    /// Check if model needs retraining
    ///
    /// # Returns
    /// `true` if model should be retrained
    ///
    /// # Example
    /// ```rust
    /// use beardog::threat::types::MlModel;
    ///
    /// let model = MlModel::default();
    /// let needs_retraining = model.needs_retraining();
    /// ```
    pub fn needs_retraining(&self) -> bool {
        self.accuracy < 0.8 || self.get_age_days() > 30
    }

    /// Get accuracy percentage
    ///
    /// # Returns
    /// Accuracy as a percentage (0-100)
    ///
    /// # Example
    /// ```rust
    /// use beardog::threat::types::MlModel;
    ///
    /// let mut model = MlModel::default();
    /// model.update_accuracy(0.95);
    /// assert_eq!(model.get_accuracy_percentage(), 95.0);
    /// ```
    pub fn get_accuracy_percentage(&self) -> f64 {
        self.accuracy * 100.0
    }

    /// Check if model is suitable for production
    ///
    /// # Returns
    /// `true` if model meets production criteria
    ///
    /// # Example
    /// ```rust
    /// use beardog::threat::types::MlModel;
    ///
    /// let mut model = MlModel::default();
    /// model.update_accuracy(0.95);
    /// model.add_feature("feature1".to_string());
    /// assert!(model.is_production_ready());
    /// ```
    pub fn is_production_ready(&self) -> bool {
        self.accuracy >= 0.9 && !self.feature_names.is_empty() && self.is_recently_trained()
    }

    /// Get model performance grade
    ///
    /// # Returns
    /// Performance grade as a string
    ///
    /// # Example
    /// ```rust
    /// use beardog::threat::types::MlModel;
    ///
    /// let mut model = MlModel::default();
    /// model.update_accuracy(0.95);
    /// assert_eq!(model.get_performance_grade(), "A");
    /// ```
    pub fn get_performance_grade(&self) -> &str {
        match self.accuracy {
            a if a >= 0.95 => "A+",
            a if a >= 0.90 => "A",
            a if a >= 0.85 => "B+",
            a if a >= 0.80 => "B",
            a if a >= 0.75 => "C+",
            a if a >= 0.70 => "C",
            a if a >= 0.65 => "D+",
            a if a >= 0.60 => "D",
            _ => "F",
        }
    }

    /// Update model type
    ///
    /// # Arguments
    /// * `model_type` - New model type
    ///
    /// # Example
    /// ```rust
    /// use beardog::threat::types::{MlModel, MlModelType};
    ///
    /// let mut model = MlModel::default();
    /// model.update_model_type(MlModelType::Classification);
    /// ```
    pub fn update_model_type(&mut self, model_type: MlModelType) {
        self.model_type = model_type;
    }

    /// Clear all features
    ///
    /// # Example
    /// ```rust
    /// use beardog::threat::types::MlModel;
    ///
    /// let mut model = MlModel::default();
    /// model.add_feature("feature1".to_string());
    /// model.clear_features();
    /// assert_eq!(model.feature_count(), 0);
    /// ```
    pub fn clear_features(&mut self) {
        self.feature_names.clear();
    }

    /// Set multiple features at once
    ///
    /// # Arguments
    /// * `features` - Vector of feature names
    ///
    /// # Example
    /// ```rust
    /// use beardog::threat::types::MlModel;
    ///
    /// let mut model = MlModel::default();
    /// model.set_features(vec!["feature1".to_string(), "feature2".to_string()]);
    /// assert_eq!(model.feature_count(), 2);
    /// ```
    pub fn set_features(&mut self, features: Vec<String>) {
        self.feature_names = features;
    }
}

impl std::fmt::Display for MlModelType {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            MlModelType::AnomalyDetection => write!(f, "Anomaly Detection"),
            MlModelType::Classification => write!(f, "Classification"),
            MlModelType::Clustering => write!(f, "Clustering"),
            MlModelType::NeuralNetwork => write!(f, "Neural Network"),
            MlModelType::RandomForest => write!(f, "Random Forest"),
            MlModelType::SupportVector => write!(f, "Support Vector Machine"),
            MlModelType::NaiveBayes => write!(f, "Naive Bayes"),
        }
    }
}

impl std::fmt::Display for MlModel {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "MlModel(id={}, name='{}', type={}, accuracy={:.2}%, features={})",
            self.id,
            self.name,
            self.model_type,
            self.get_accuracy_percentage(),
            self.feature_count()
        )
    }
}
