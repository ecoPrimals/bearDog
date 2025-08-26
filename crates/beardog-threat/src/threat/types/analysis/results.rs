

use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};
use std::collections::HashMap;

use crate::threat::ml_engine::MlPrediction;
use crate::threat::types::core::ThreatEvent;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ThreatAnalysisResult {

    pub analysis_id: String,

    pub event_id: String,

    pub timestamp: DateTime<Utc>,

    pub threats_detected: usize,

    pub confidence_score: f64,

    pub analysis_time_ms: u64,

    pub threat_events: Vec<ThreatEvent>,

    pub ml_predictions: Vec<MlPrediction>,

    pub metadata: HashMap<String, String>,
}
impl Default for ThreatAnalysisResult {
    fn default() -> Self {
        Self {
            analysis_id: uuid::Uuid::new_v4().to_string(),
            event_id: String::new(),
            timestamp: chrono::Utc::now(),
            threats_detected: 0,
            confidence_score: 0.0,
            analysis_time_ms: 0,
            threat_events: Vec::new(),
            ml_predictions: Vec::new(),
            metadata: HashMap::new(),
        }
    }
}
impl ThreatAnalysisResult {

    pub fn new(analysis_id: &str, event_id: &str) -> Self {
        Self {
            analysis_id: analysis_id.to_string(),
            event_id: event_id.to_string(),
            timestamp: chrono::Utc::now(),
            threats_detected: 0,
            confidence_score: 0.0,
            analysis_time_ms: 0,
            threat_events: Vec::new(),
            ml_predictions: Vec::new(),
            metadata: HashMap::new(),
        }
    }

    pub fn add_threat_event(&mut self, event: ThreatEvent) {
        self.threat_events.push(event);
        self.threats_detected = self.threat_events.len();
    }

    pub fn add_ml_prediction(&mut self, prediction: MlPrediction) {
        self.ml_predictions.push(prediction);
    }

    pub fn set_analysis_time(&mut self, time_ms: u64) {
        self.analysis_time_ms = time_ms;
    }

    pub fn set_confidence_score(&mut self, score: f64) {
        self.confidence_score = score.clamp(0.0, 1.0);
    }

    pub fn has_threats(&self) -> bool {
        self.threats_detected > 0
    }

    pub fn highest_threat_severity(&self) -> Option<crate::threat::types::ThreatSeverity> {
        self.threat_events.iter().map(|e| e.severity.clone()).max()
    }

    pub fn total_threat_score(&self) -> f64 {
        self.threat_events.iter().map(|e| e.score as f64).sum()
    }

    pub fn summary(&self) -> String {
        format!(
            "Analysis {} found {} threats with {:.2}% confidence in {}ms",
            self.analysis_id,
            self.threats_detected,
            self.confidence_score * 100.0,
            self.analysis_time_ms
        )
    }

    pub fn add_recommendation(&mut self, recommendation: &str) {
        self.metadata
            .insert("recommendation".to_string(), recommendation.to_string());
    }
}
