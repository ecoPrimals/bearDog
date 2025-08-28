use serde::{Deserialize, Serialize};

pub mod session;

pub use session::*;

// Placeholder types for future implementation
#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct AnalysisMetrics {
    pub threat_count: usize,
    pub confidence_score: f64,
    pub avg_analysis_time_ms: f64,
    pub total_events_processed: usize,
}

impl AnalysisMetrics {
    pub fn new() -> Self {
        Self::default()
    }

    pub fn update_with_analysis(&mut self, analysis_time_ms: f64, threats_detected: bool) {
        self.total_events_processed += 1;
        self.avg_analysis_time_ms = (self.avg_analysis_time_ms
            * (self.total_events_processed - 1) as f64
            + analysis_time_ms)
            / self.total_events_processed as f64;

        if threats_detected {
            self.threat_count += 1;
        }

        // Update confidence score based on detection patterns
        self.confidence_score = if self.total_events_processed > 0 {
            (self.threat_count as f64 / self.total_events_processed as f64) * 100.0
        } else {
            0.0
        };
    }

    pub fn threat_detection_rate(&self) -> f64 {
        if self.total_events_processed > 0 {
            self.threat_count as f64 / self.total_events_processed as f64
        } else {
            0.0
        }
    }

    pub fn avg_threats_per_event(&self) -> f64 {
        self.threat_detection_rate()
    }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SecurityEvent {
    pub event_type: String,
    pub timestamp: chrono::DateTime<chrono::Utc>,
    pub severity: String,
    pub source_ip: Option<String>,
    pub user_id: Option<String>,
}

impl SecurityEvent {
    pub fn new(
        event_type: String,
        timestamp: chrono::DateTime<chrono::Utc>,
        severity: String,
    ) -> Self {
        Self {
            event_type,
            timestamp,
            severity,
            source_ip: None,
            user_id: None,
        }
    }

    pub fn with_source_ip(mut self, source_ip: String) -> Self {
        self.source_ip = Some(source_ip);
        self
    }

    pub fn with_user_id(mut self, user_id: String) -> Self {
        self.user_id = Some(user_id);
        self
    }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ThreatAnalysisResult {
    pub analysis_id: String,
    pub event_id: String,
    pub threat_level: String,
    pub threat_detected: bool,
    pub confidence: f64,
    pub details: String,
    pub indicators: Vec<String>,
    pub timestamp: chrono::DateTime<chrono::Utc>,
}

impl ThreatAnalysisResult {
    pub fn new(analysis_id: &str, event_id: &str) -> Self {
        Self {
            analysis_id: analysis_id.to_string(),
            event_id: event_id.to_string(),
            threat_level: "low".to_string(),
            threat_detected: false,
            confidence: 0.0,
            details: format!("Analysis {} for event {}", analysis_id, event_id),
            indicators: Vec::new(),
            timestamp: chrono::Utc::now(),
        }
    }

    pub fn has_threats(&self) -> bool {
        self.threat_detected || self.threat_level != "low" || self.confidence > 0.5
    }

    pub fn with_threat_detected(mut self, detected: bool) -> Self {
        self.threat_detected = detected;
        self
    }

    pub fn with_confidence(mut self, confidence: f64) -> Self {
        self.confidence = confidence;
        self
    }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct EventCorrelationResult {
    pub correlated_events: Vec<SecurityEvent>,
    pub correlation_score: f64,
}

impl EventCorrelationResult {
    pub fn is_significant(&self) -> bool {
        self.correlation_score > 0.7
    }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum CorrelationType {
    Temporal,
    Spatial,
    Behavioral,
}

// ML prediction functionality will be implemented in future iterations
// pub use crate::threat::analysis::MlPrediction;
