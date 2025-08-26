

use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct MlModel {

    pub id: String,

    pub name: String,

    pub model_type: MlModelType,

    pub accuracy: f64,

    pub last_trained: DateTime<Utc>,

    pub feature_names: Vec<String>,
}

#[derive(Debug, Clone, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub enum MlModelType {
    Classification,
    Regression,
    Clustering,
    AnomalyDetection,
    NeuralNetwork,
    DecisionTree,
    RandomForest,
    SupportVector,
    SupportVectorMachine,
    LogisticRegression,
    KMeans,
    ReinforcementLearning,
    DeepLearning,
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

    pub fn new(
        id: &str,
        name: &str,
        model_type: MlModelType,
        accuracy: f64,
        feature_names: Vec<&str>,
    ) -> Self {
        Self {
            id: id.to_string(),
            name: name.to_string(),
            model_type,
            accuracy: accuracy.clamp(0.0, 1.0),
            last_trained: Utc::now(),
            feature_names: feature_names.into_iter().map(|s| s.to_string()).collect(),
        }
    }

    pub fn get_age_days(&self) -> i64 {
        Utc::now().signed_duration_since(self.last_trained).num_days()
    }

    pub fn feature_count(&self) -> usize {
        self.feature_names.len()
    }

    pub fn is_high_accuracy(&self) -> bool {
        self.accuracy >= 0.8
    }

    pub fn update_training_timestamp(&mut self) {
        self.last_trained = Utc::now();
    }

    pub fn needs_retraining(&self) -> bool {
        self.accuracy < 0.8 || self.get_age_days() > 30
    }

    pub fn get_accuracy_percentage(&self) -> f64 {
        self.accuracy * 100.0
    }

    pub fn is_production_ready(&self) -> bool {
        self.accuracy >= 0.9 && self.get_age_days() <= 30 && !self.feature_names.is_empty()
    }

    pub fn get_summary(&self) -> String {
        format!(
            "Model: {} ({}), Accuracy: {:.1}%, Age: {} days, Features: {}",
            self.name,
            self.model_type,
            self.get_accuracy_percentage(),
            self.get_age_days(),
            self.feature_count()
        )
    }

    pub fn update_accuracy(&mut self, new_accuracy: f64) {
        self.accuracy = new_accuracy.clamp(0.0, 1.0);
        self.update_training_timestamp();
    }

    pub fn add_feature(&mut self, feature_name: &str) {
        let feature_string = feature_name.to_string();
        if !self.feature_names.contains(&feature_string) {
            self.feature_names.push(feature_string);
        }
    }

    pub fn remove_feature(&mut self, feature_name: &str) {
        self.feature_names.retain(|f| f != feature_name);
    }

    pub fn get_metadata(&self) -> std::collections::HashMap<String, String> {
        let mut metadata = std::collections::HashMap::with_capacity(16);
        metadata.insert("id".to_string(), self.id.clone());
        metadata.insert("name".to_string(), self.name.clone());
        metadata.insert("type".to_string(), format_args!("{}", self.model_type).to_string());
        metadata.insert("accuracy".to_string(), format_args!("{:.3}", self.accuracy).to_string());
        metadata.insert("age_days".to_string(), self.get_age_days().to_string());
        metadata.insert("feature_count".to_string(), self.feature_count().to_string());
        metadata
    }
}

impl std::fmt::Display for MlModelType {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            MlModelType::NeuralNetwork => write!(f, "Neural Network"),
            MlModelType::RandomForest => write!(f, "Random Forest"),
            MlModelType::SupportVectorMachine => write!(f, "Support Vector Machine"),
            MlModelType::DecisionTree => write!(f, "Decision Tree"),
            MlModelType::LogisticRegression => write!(f, "Logistic Regression"),
            MlModelType::KMeans => write!(f, "K-Means Clustering"),
            MlModelType::Regression => write!(f, "Regression"),
            MlModelType::ReinforcementLearning => write!(f, "Reinforcement Learning"),
            MlModelType::DeepLearning => write!(f, "Deep Learning"),
            MlModelType::Classification => write!(f, "Classification"),
            MlModelType::Clustering => write!(f, "Clustering"),
            MlModelType::AnomalyDetection => write!(f, "Anomaly Detection"),
            MlModelType::SupportVectorMachine => write!(f, "Support Vector Machine"),
            MlModelType::LogisticRegression => write!(f, "Logistic Regression"),
            MlModelType::KMeans => write!(f, "K-Means"),
            MlModelType::ReinforcementLearning => write!(f, "Reinforcement Learning"),
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
