

use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};

use super::correlation::EventCorrelationResult;
use super::events::SecurityEvent;
use super::metrics::AnalysisMetrics;
use super::results::ThreatAnalysisResult;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct AnalysisSession {

    pub session_id: String,

    pub start_time: DateTime<Utc>,

    pub end_time: Option<DateTime<Utc>>,

    pub events: Vec<SecurityEvent>,

    pub analysis_results: Vec<ThreatAnalysisResult>,

    pub metrics: AnalysisMetrics,

    pub correlations: Vec<EventCorrelationResult>,
}
impl Default for AnalysisSession {
    fn default() -> Self {
        Self {
            session_id: uuid::Uuid::new_v4().to_string(),
            start_time: chrono::Utc::now(),
            end_time: None,
            events: vec![],
            analysis_results: vec![],
            metrics: AnalysisMetrics::default(),
            correlations: vec![],
        }
    }
}
impl AnalysisSession {

    pub fn new(session_id: &str) -> Self {
        Self {
            session_id: session_id.to_string(),
            start_time: Utc::now(),
            end_time: None,
            events: Vec::new(),
            analysis_results: Vec::new(),
            correlations: Vec::new(),
            metrics: AnalysisMetrics::new(),
        }
    }

    pub fn end_session(&mut self) {
        self.end_time = Some(Utc::now());
    }

    pub fn add_event(&mut self, event: SecurityEvent) {
        self.events.push(event);
    }

    pub fn add_analysis_result(&mut self, result: ThreatAnalysisResult) {

        self.metrics
            .update_with_analysis(result.analysis_time_ms, result.threats_detected);

        self.analysis_results.push(result);
    }

    pub fn add_correlation(&mut self, correlation: EventCorrelationResult) {
        self.correlations.push(correlation);
    }

    pub fn duration_minutes(&self) -> i64 {
        let end_time = self.end_time.unwrap_or_else(Utc::now);
        (end_time - self.start_time).num_minutes()
    }

    pub fn total_threats_detected(&self) -> usize {
        self.analysis_results
            .iter()
            .map(|r| r.threats_detected)
            .sum()
    }

    pub fn events_analyzed_count(&self) -> usize {
        self.analysis_results.len()
    }

    pub fn pending_events_count(&self) -> usize {
        self.events
            .len()
            .saturating_sub(self.events_analyzed_count())
    }

    pub fn correlation_count(&self) -> usize {
        self.correlations.len()
    }

    pub fn significant_correlations(&self) -> Vec<&EventCorrelationResult> {
        self.correlations
            .iter()
            .filter(|c| c.is_significant())
            .collect()
    }

    pub fn session_statistics(&self) -> SessionStatistics {
        SessionStatistics {
            session_id: self.session_id.clone(),
            duration_minutes: self.duration_minutes(),
            total_events: self.events.len(),
            events_analyzed: self.events_analyzed_count(),
            pending_events: self.pending_events_count(),
            total_threats: self.total_threats_detected(),
            correlations_found: self.correlation_count(),
            significant_correlations: self.significant_correlations().len(),
            avg_analysis_time_ms: self.metrics.avg_analysis_time_ms,
            threat_detection_rate: self.metrics.threat_detection_rate(),
        }
    }

    pub fn is_active(&self) -> bool {
        self.end_time.is_none()
    }

    pub fn summary(&self) -> String {
        format!(
            "Session {}: {} events, {} threats detected, {} correlations in {} minutes",
            self.session_id,
            self.events.len(),
            self.total_threats_detected(),
            self.correlation_count(),
            self.duration_minutes()
        )
    }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SessionStatistics {

    pub session_id: String,

    pub duration_minutes: i64,

    pub total_events: usize,

    pub events_analyzed: usize,

    pub pending_events: usize,

    pub total_threats: usize,

    pub correlations_found: usize,

    pub significant_correlations: usize,

    pub avg_analysis_time_ms: f64,

    pub threat_detection_rate: f64,
}
