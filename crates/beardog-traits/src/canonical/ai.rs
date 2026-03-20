// SPDX-License-Identifier: AGPL-3.0-only

//! Canonical AI/ML DTOs and the [`AiProvider`] trait for prompts, training, and search.

use super::base::BaseProvider;
use beardog_errors::BearDogError;
use serde::{Deserialize, Serialize};
use std::collections::HashMap;

/// Opaque analysis payload with a scalar confidence.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct AnalysisResult {
    /// The analysis value
    pub analysis: serde_json::Value,
    /// Model belief in the primary conclusion (0.0–1.0).
    pub confidence: f64,
}

/// Hyperparameters and architecture label for [`AiProvider::train_model`].
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ModelConfig {
    /// The model type value
    pub model_type: String,
    /// The parameters value
    pub parameters: std::collections::HashMap<String, serde_json::Value>,
}

/// Training or serving status snapshot.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ModelStatus {
    /// Current status of the component
    pub status: String,
    /// The accuracy value
    pub accuracy: f64,
    /// The last updated value
    pub last_updated: chrono::DateTime<chrono::Utc>,
}

/// Registry metadata for a deployable model.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ModelInfo {
    /// Registry id for this model artifact.
    pub id: String,
    /// Name of the item
    pub name: String,
    /// The version value
    pub version: String,
    /// Collection of capabilities
    pub capabilities: Vec<String>,
}

/// One hit from [`AiProvider::semantic_search`].
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SearchResult {
    /// Document or chunk id within the searched corpus.
    pub id: String,
    /// The content value
    pub content: String,
    /// The relevance score value
    pub relevance_score: f64,
}

/// Multiclass or multilabel classification output.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ClassificationResult {
    /// The category value
    pub category: String,
    /// Belief in the assigned `category` (0.0–1.0).
    pub confidence: f64,
    /// The metadata value
    pub metadata: std::collections::HashMap<String, serde_json::Value>,
}

#[allow(clippy::type_complexity)]
/// High-level AI operations built on [`BaseProvider`].
pub trait AiProvider: BaseProvider {
    /// Processes prompt
    fn process_prompt(
        prompt: &str,
        context: Option<HashMap<&str, &str>>,
    ) -> impl std::future::Future<Output = Result<String, BearDogError>> + Send;

    /// Runs batch or streaming analysis over raw bytes.
    fn analyze_data(
        data: &[u8],
        analysis_type: &str,
    ) -> impl std::future::Future<Output = Result<AnalysisResult, BearDogError>> + Send;

    /// Kicks off (or resumes) training; returns a job or model id.
    fn train_model(
        model_name: &str,
        training_data: &[u8],
        config: ModelConfig,
    ) -> impl std::future::Future<Output = Result<String, BearDogError>> + Send;

    /// Gets `model_status`
    fn get_model_status(
        &self,
        model_id: &str,
    ) -> impl std::future::Future<Output = Result<ModelStatus, BearDogError>> + Send;

    /// Removes model
    fn delete_model(
        &self,
        model_id: &str,
    ) -> impl std::future::Future<Output = Result<(), BearDogError>> + Send;

    /// Gets `model_info`
    fn get_model_info(
        model_id: &str,
    ) -> impl std::future::Future<Output = Result<ModelInfo, BearDogError>> + Send;

    /// Vector embedding for `text`, optionally using a specific `model_id`.
    fn generate_embeddings(
        text: &str,
        model_id: Option<&str>,
    ) -> impl std::future::Future<Output = Result<Vec<f32>, BearDogError>> + Send;

    /// Lexical or vector search over an in-memory `corpus` slice.
    fn semantic_search(
        query: &str,
        corpus: &[&str],
        limit: Option<usize>,
    ) -> impl std::future::Future<Output = Result<Vec<SearchResult>, BearDogError>> + Send;

    /// Picks the closest label from `categories`.
    fn classify_text(
        text: &str,
        categories: &[&str],
    ) -> impl std::future::Future<Output = Result<ClassificationResult, BearDogError>> + Send;
}
