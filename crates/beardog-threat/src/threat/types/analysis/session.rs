use super::{AnalysisMetrics, EventCorrelationResult, SecurityEvent, ThreatAnalysisResult};
use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};

#[derive(Debug, Clone)]
    pub start_time: DateTime<Utc>,
    pub end_time: Option<DateTime<Utc>>,
    /// Collection of events analyzed
    /// Collection of events analyzed
    pub events_analyzed: Vec<SecurityEvent>,
    /// Collection of analysis results
    /// Collection of analysis results
    pub analysis_results: Vec<ThreatAnalysisResult>,
    /// The metrics value
    /// The metrics value
    pub metrics: AnalysisMetrics,
    /// Collection of correlation results
    /// Collection of correlation results
    pub correlation_results: Vec<EventCorrelationResult>,
}

impl ThreatAnalysisSession {
    /// New operation.
    /// Creates a new instance
    pub fn new(session_id: &str) -> Self {
        Self {
            session_id: session_id.to_string(),
            start_time: Utc::now(None,
            events_analyzed: Vec::new(),
            analysis_results: Vec::new(),
            metrics: AnalysisMetrics::new(),
            correlation_results: Vec::new(),
        }
    }

    /// Add Event operation.
    pub fn add_event(&mut self, event: SecurityEvent) {
        self.events_analyzed.push(event);
    }

    /// Add Analysis Result operation.
    pub fn add_analysis_result(&mut self, result: ThreatAnalysisResult) {
        self.analysis_results.push(result);
    }

    /// Finalize Session operation.
    pub fn finalize_session(&mut self) {
        self.end_time = Some(Utc::now());
    }

    /// Total Threats Detected operation.
    pub fn total_threats_detected(&self) -> usize {
        self.analysis_results
            .iter()
            .map(|r| if r.threat_detected { 1 } else { 0 })
            .sum()
    }

    /// Average Confidence operation.
    pub fn average_confidence(&self) -> f64 {
        if self.analysis_results.is_empty() {
            0.0
        } else {
            let total_confidence: f64 = self.analysis_results.iter().map(|r| r.confidence).sum();
            total_confidence / self.analysis_results.len() as f64
        }
    }

    /// Session Duration Seconds operation.
    pub fn session_duration_seconds(&self) -> Option<i64> {
        self.end_time
            .map(|end_time| (end_time - self.start_time).num_seconds())
    }
}
