use super::{AnalysisMetrics, EventCorrelationResult, SecurityEvent, ThreatAnalysisResult};
use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ThreatAnalysisSession {
    pub session_id: String,
    pub start_time: DateTime<Utc>,
    pub end_time: Option<DateTime<Utc>>,
    pub events_analyzed: Vec<SecurityEvent>,
    pub analysis_results: Vec<ThreatAnalysisResult>,
    pub metrics: AnalysisMetrics,
    pub correlation_results: Vec<EventCorrelationResult>,
}

impl ThreatAnalysisSession {
    pub fn new(session_id: String) -> Self {
        Self {
            session_id,
            start_time: Utc::now(),
            end_time: None,
            events_analyzed: Vec::new(),
            analysis_results: Vec::new(),
            metrics: AnalysisMetrics::new(),
            correlation_results: Vec::new(),
        }
    }

    pub fn add_event(&mut self, event: SecurityEvent) {
        self.events_analyzed.push(event);
    }

    pub fn add_analysis_result(&mut self, result: ThreatAnalysisResult) {
        self.analysis_results.push(result);
    }

    pub fn finalize_session(&mut self) {
        self.end_time = Some(Utc::now());
    }

    pub fn total_threats_detected(&self) -> usize {
        self.analysis_results
            .iter()
            .map(|r| if r.threat_detected { 1 } else { 0 })
            .sum()
    }

    pub fn average_confidence(&self) -> f64 {
        if self.analysis_results.is_empty() {
            0.0
        } else {
            let total_confidence: f64 = self.analysis_results.iter().map(|r| r.confidence).sum();
            total_confidence / self.analysis_results.len() as f64
        }
    }

    pub fn session_duration_seconds(&self) -> Option<i64> {
        self.end_time
            .map(|end_time| (end_time - self.start_time).num_seconds())
    }
}
