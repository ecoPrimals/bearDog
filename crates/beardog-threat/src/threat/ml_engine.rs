// Machine Learning Engine - Modern Implementation
//
// **MODERNIZED**: Clean, production-ready ML engine for threat detection.

use crate::threat::types::SecurityEvent;
use beardog_errors::BearDogError;
use std::collections::HashMap;
use std::future::Future;
use std::pin::Pin;
use std::sync::Arc;
use tokio::sync::RwLock;

pub trait UniversalComputeAdapter: Send + Sync {
    fn request_compute(
        &self,
        endpoint: &str,
        request: &serde_json::Value,
    ) -> Pin<Box<dyn Future<Output = Result<serde_json::Value, BearDogError>> + Send + '_>>;
}

/// ML prediction result
#[derive(Debug, Clone)]
pub struct MlPrediction {
    pub confidence: f64,
    /// The risk level value
    /// The risk level value
    pub risk_level: RiskLevel,
    /// Collection of reasoning
    /// Collection of reasoning
    pub reasoning: Vec<String>,
    /// The model version value
    /// The model version value
    pub model_version: String,
    pub processing_time_ms: u64,
}

/// Risk level classification
#[derive(Debug, Clone, PartialEq)]
pub enum RiskLevel {
    /// Represents critical variant
    Critical,
    /// Represents high variant
    High,
    /// Represents medium variant
    Medium,
    /// Represents low variant
    Low,
    /// Represents minimal variant
    Minimal,
}

/// ML model representation
#[derive(Debug, Clone)]
pub struct MlModel {
    /// Name of the item
    /// Name of the item
    pub name: String,
    /// The version value
    /// The version value
    pub version: String,
    /// The model type value
    /// The model type value
    pub model_type: String,
    /// The accuracy value
    /// The accuracy value
    pub accuracy: f64,
    /// The last updated value
    /// The last updated value
    pub last_updated: chrono::DateTime<chrono::Utc>,
}

pub struct MlEngine {
    models: HashMap<String, MlModel>,
    universal_adapter: Option<Box<dyn UniversalComputeAdapter>>,
    prediction_cache: Arc<RwLock<HashMap<String, MlPrediction>>>,
    local_predictions: u64,
    network_predictions: u64,
}

pub type SmartThreatMLEngine = MlEngine;

impl Default for MlEngine {
    fn default() -> Self {
        Self::new()
    }
}

impl MlEngine {
    /// Create a new ML engine instance
    /// Creates a new instance
    #[must_use]
    pub fn new() -> Self {
        Self {
            models: HashMap::with_capacity(16),
            universal_adapter: None,
            prediction_cache: Arc::new(RwLock::new(HashMap::with_capacity(1000))),
            local_predictions: 0,
            network_predictions: 0,
        }
    }

    /// Configure the engine with a universal compute adapter
    /// Creates instance with universal adapter
    #[must_use]
    pub fn with_universal_adapter(mut self, adapter: Box<dyn UniversalComputeAdapter>) -> Self {
        self.universal_adapter = Some(adapter);
        self
    }

    /// Add a model to the engine
    pub fn add_model(&mut self, model: MlModel) {
        self.models.insert(model.name.clone(), model);
    }

    /// Predict threat from security event
    pub async fn predict_threat(
        &self,
        event: &SecurityEvent,
    ) -> Result<MlPrediction, BearDogError> {
        let cache_key = self.generate_cache_key(event);

        // Check cache first
        {
            let cache = self.prediction_cache.read().await;
            if let Some(cached) = cache.get(&cache_key) {
                return Ok(cached.clone());
            }
        }

        // Try universal adapter first, fallback to local prediction
        let prediction = if let Some(adapter) = &self.universal_adapter {
            match self
                .predict_via_universal_adapter(event, adapter.as_ref())
                .await
            {
                Ok(pred) => pred,
                Err(_) => self.predict_local(event),
            }
        } else {
            self.predict_local(event)
        };

        // Cache the result
        {
            let mut cache = self.prediction_cache.write().await;
            cache.insert(cache_key, prediction.clone());
        }

        Ok(prediction)
    }

    fn predict_local(&self, event: &SecurityEvent) -> MlPrediction {
        let start_time = std::time::Instant::now();

        // Calculate threat score based on event characteristics
        let threat_score = self.calculate_threat_score(event);
        let risk_level = self.score_to_risk_level(threat_score);

        let reasoning = vec![
            "Local heuristic analysis".to_string(),
            format!("Event type: {}", event.event_type),
            format!("Threat score: {:.2}", threat_score),
        ];

        MlPrediction {
            confidence: 0.75, // Moderate confidence for local predictions
            risk_level,
            reasoning,
            model_version: "beardog-local-v1.0".to_string(),
            processing_time_ms: start_time.elapsed().as_millis() as u64,
        }
    }

    /// Predict via universal compute adapter
    async fn predict_via_universal_adapter(
        &self,
        event: &SecurityEvent,
        adapter: &dyn UniversalComputeAdapter,
    ) -> Result<MlPrediction, BearDogError> {
        let start_time = std::time::Instant::now();

        let event_data = serde_json::to_value(event)
            .map_err(|e| BearDogError::validation(&format!("Failed to serialize event: {e}")))?;

        let compute_request = serde_json::json!({
            "type": "threat_analysis",
            "data": event_data,
            "model": "advanced_threat_detection",
            "priority": "high"
        });

        let response = adapter
            .request_compute("ml_threat_analysis", &compute_request)
            .await?;

        // Parse response
        let confidence = response
            .get("confidence")
            .and_then(serde_json::Value::as_f64)
            .unwrap_or(0.5);

        let risk_score = response
            .get("risk_score")
            .and_then(serde_json::Value::as_f64)
            .unwrap_or(0.5);

        let reasoning = response
            .get("reasoning")
            .and_then(|v| v.as_array())
            .map(|arr| {
                arr.iter()
                    .filter_map(|v| v.as_str())
                    .map(String::from)
                    .collect()
            })
            .unwrap_or_else(|| vec!["Network ML analysis".to_string()]);

        Ok(MlPrediction {
            confidence,
            risk_level: self.score_to_risk_level(risk_score),
            reasoning,
            model_version: "beardog-network-v2.0".to_string(),
            processing_time_ms: start_time.elapsed().as_millis() as u64,
        })
    }

    fn generate_cache_key(&self, event: &SecurityEvent) -> String {
        format!("{}:{:?}", event.event_type, event.timestamp)
    }

    /// Calculate threat score based on event characteristics
    fn calculate_threat_score(&self, event: &SecurityEvent) -> f64 {
        let mut score: f64 = 0.0;

        // Base score by event type
        score += match event.event_type.as_str() {
            "authentication_failure" => 0.6,
            "network_scan" => 0.7,
            "malware_detected" => 0.9,
            "data_exfiltration" => 0.95,
            "privilege_escalation" => 0.8,
            "suspicious_process" => 0.5,
            _ => 0.3,
        };

        // Adjust based on severity
        score += match event.severity.as_str() {
            "critical" => 0.3,
            "high" => 0.2,
            "medium" => 0.1,
            "low" => 0.05,
            _ => 0.0,
        };

        // Cap at 1.0
        score.min(1.0)
    }

    /// Convert threat score to risk level
    fn score_to_risk_level(&self, score: f64) -> RiskLevel {
        match score {
            s if s >= 0.9 => RiskLevel::Critical,
            s if s >= 0.7 => RiskLevel::High,
            s if s >= 0.5 => RiskLevel::Medium,
            s if s >= 0.3 => RiskLevel::Low,
            _ => RiskLevel::Minimal,
        }
    }

    /// Get engine statistics
    /// Gets stats
    /// Gets stats
    #[must_use]
    pub fn get_stats(&self) -> MlEngineStats {
        MlEngineStats {
            local_predictions: self.local_predictions,
            network_predictions: self.network_predictions,
            models_loaded: self.models.len(),
            cache_size: 0, // Would need async access to get actual size
        }
    }

    /// Clear prediction cache
    pub async fn clear_cache(&self) {
        let mut cache = self.prediction_cache.write().await;
        cache.clear();
    }
}

/// ML engine statistics
#[derive(Debug, Clone)]
pub struct MlEngineStats {
    /// Number of `local_predictions`
    /// Number of `local_predictions`
    pub local_predictions: u64,
    /// Number of `network_predictions`
    /// Number of `network_predictions`
    pub network_predictions: u64,
    /// Number of `models_loaded`
    /// Number of `models_loaded`
    pub models_loaded: usize,
    /// Number of `cache_size`
    /// Number of `cache_size`
    pub cache_size: usize,
}

#[cfg(test)]
mod tests {
    use super::*;
    use chrono::Utc;

    #[tokio::test]
    fn test_ml_engine_creation() {
        let engine = MlEngine::new();
        assert_eq!(engine.models.len(), 0);
        assert!(engine.universal_adapter.is_none());
    }

    #[tokio::test]
    fn test_risk_level_conversion() {
        let engine = MlEngine::new();

        assert_eq!(engine.score_to_risk_level(0.95), RiskLevel::Critical);
        assert_eq!(engine.score_to_risk_level(0.75), RiskLevel::High);
        assert_eq!(engine.score_to_risk_level(0.55), RiskLevel::Medium);
        assert_eq!(engine.score_to_risk_level(0.35), RiskLevel::Low);
        assert_eq!(engine.score_to_risk_level(0.15), RiskLevel::Minimal);
    }

    #[tokio::test]
    fn test_threat_score_calculation() {
        let engine = MlEngine::new();

        let event = SecurityEvent {
            id: "test-1".to_string(),
            event_type: "malware_detected".to_string(),
            severity: ThreatSeverity::Critical,
            timestamp: Utc::now().into(),
            source: "test_source".to_string(),
            description: "Malware detected".to_string(),
            data: std::collections::HashMap::new(),
        };

        let score = engine.calculate_threat_score(&event);
        assert!(score > 0.8); // Should be high for malware + critical
    }

    #[tokio::test]
    fn test_local_prediction() {
        let engine = MlEngine::new();

        let mut event_data = std::collections::HashMap::new();
        event_data.insert("source_ip".to_string(), "192.168.1.100".to_string());
        event_data.insert("user_id".to_string(), "admin".to_string());

        let event = SecurityEvent {
            id: "test-2".to_string(),
            event_type: "authentication_failure".to_string(),
            severity: ThreatSeverity::Medium,
            timestamp: Utc::now().into(),
            source: "login_system".to_string(),
            description: "Authentication failure".to_string(),
            data: event_data,
        };

        let prediction = engine.predict_local(&event);
        assert!(prediction.confidence > 0.0);
        assert!(!prediction.reasoning.is_empty());
        assert_eq!(prediction.model_version, "beardog-local-v1.0");
    }
}
