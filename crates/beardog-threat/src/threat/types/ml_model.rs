// SPDX-License-Identifier: AGPL-3.0-or-later

//! Lightweight ML model registry rows for policy and scoring.

use serde::{Deserialize, Serialize};
use std::time::SystemTime;

/// Lightweight registry entry for a trained model used in policy or scoring.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct MlModel {
    /// Model identifier
    pub id: String,
    /// Model name
    /// Name of the item
    pub name: String,
    /// Model type
    /// The model type value
    pub model_type: MlModelType,
    /// Model accuracy score
    /// The accuracy value
    pub accuracy: f64,
    /// Model version
    /// The version value
    pub version: String,
    /// Model training timestamp
    /// The trained at value
    pub trained_at: SystemTime,
}

/// Algorithm family for [`MlModel`] entries in this crate’s type layer.
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
pub enum MlModelType {
    /// Anomaly detection model
    AnomalyDetection,
    /// Classification model
    Classification,
    /// Clustering model
    Clustering,
    /// Neural network model
    NeuralNetwork,
    /// Decision tree model
    DecisionTree,
    /// Ensemble model
    Ensemble,
}
