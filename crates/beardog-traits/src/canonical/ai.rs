// Module documentation
//
// This module provides functionality for the BearDog ecosystem.

use super::base::BaseProvider;
use beardog_errors::BearDogError;
use serde::{Deserialize, Serialize};
use std::collections::HashMap;

// Temporary type definitions for AI traits
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct AnalysisResult {
    /// The analysis value
    pub analysis: serde_json::Value,
    pub confidence: f64,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ModelConfig {
    /// The model type value
    pub model_type: String,
    /// The parameters value
    pub parameters: std::collections::HashMap<String, serde_json::Value>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ModelStatus {
    /// Current status of the component
    pub status: String,
    /// The accuracy value
    pub accuracy: f64,
    /// The last updated value
    pub last_updated: chrono::DateTime<chrono::Utc>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ModelInfo {
    pub id: String,
    /// Name of the item
    pub name: String,
    /// The version value
    pub version: String,
    /// Collection of capabilities
    pub capabilities: Vec<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SearchResult {
    pub id: String,
    /// The content value
    pub content: String,
    /// The relevance score value
    pub relevance_score: f64,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ClassificationResult {
    /// The category value
    pub category: String,
    pub confidence: f64,
    /// The metadata value
    pub metadata: std::collections::HashMap<String, serde_json::Value>,
}

#[allow(clippy::type_complexity)]
pub trait AiProvider: BaseProvider {
    /// Processes prompt
    fn process_prompt(
        prompt: &str,
        context: Option<HashMap<&str, &str>>,
    ) -> impl std::future::Future<Output = Result<String, BearDogError>> + Send;

    fn analyze_data(
        data: &[u8],
        analysis_type: &str,
    ) -> impl std::future::Future<Output = Result<AnalysisResult, BearDogError>> + Send;

    fn train_model(
        model_name: &str,
        training_data: &[u8],
        config: ModelConfig,
    ) -> impl std::future::Future<Output = Result<String, BearDogError>> + Send;

    /// Gets model_status
    fn get_model_status(
        &self,
        model_id: &str,
    ) -> impl std::future::Future<Output = Result<ModelStatus, BearDogError>> + Send;

    /// Removes model
    fn delete_model(
        &self,
        model_id: &str,
    ) -> impl std::future::Future<Output = Result<(), BearDogError>> + Send;

    /// Gets model_info
    fn get_model_info(
        model_id: &str,
    ) -> impl std::future::Future<Output = Result<ModelInfo, BearDogError>> + Send;

    fn generate_embeddings(
        text: &str,
        model_id: Option<&str>,
    ) -> impl std::future::Future<Output = Result<Vec<f32>, BearDogError>> + Send;

    fn semantic_search(
        query: &str,
        corpus: &[&str],
        limit: Option<usize>,
    ) -> impl std::future::Future<Output = Result<Vec<SearchResult>, BearDogError>> + Send;

    fn classify_text(
        text: &str,
        categories: &[&str],
    ) -> impl std::future::Future<Output = Result<ClassificationResult, BearDogError>> + Send;
}
