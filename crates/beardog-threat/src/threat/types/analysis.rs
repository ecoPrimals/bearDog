// Threat Analysis Types - Modern Implementation
//
// **MODERNIZED**: Clean, production-ready analysis types for the BearDog threat detection system.

use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};
use std::collections::HashMap;

#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct AnalysisMetrics {
    /// Number of total_events_processed
    /// Number of total_events_processed
    pub total_events_processed: usize,
    /// Number of threats_detected
    /// Number of threats_detected
    pub threats_detected: usize,
    /// Number of false_positives
    /// Number of false_positives
    pub false_positives: usize,
    pub processing_time_ms: u64,
    /// The accuracy score value
    /// The accuracy score value
    pub accuracy_score: f64,
}

impl AnalysisMetrics {
    /// Create new analysis metrics
    /// Creates a new instance
    pub fn new() -> Self {
        Self::default()
    }

    /// Update metrics with new analysis results
    /// Updates with_analysis
    /// Updates with_analysis
    pub fn update_with_analysis(&mut self, processing_time: f64, threat_detected: bool) {
        self.total_events_processed += 1;
        self.processing_time_ms += processing_time as u64;

        if threat_detected {
            self.threats_detected += 1;
        }
    }

    /// Calculate threat detection rate
    pub fn threat_detection_rate(&self) -> f64 {
        if self.total_events_processed == 0 {
            0.0
        } else {
            self.threats_detected as f64 / self.total_events_processed as f64
        }
    }

    /// Calculate average processing time
    pub fn avg_processing_time(&self) -> f64 {
        if self.total_events_processed == 0 {
            0.0
        } else {
            self.processing_time_ms as f64 / self.total_events_processed as f64
        }
    }

    /// Calculate average threats per event
    pub fn avg_threats_per_event(&self) -> f64 {
        if self.total_events_processed == 0 {
            0.0
        } else {
            self.threats_detected as f64 / self.total_events_processed as f64
        }
    }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SecurityEvent {
    pub id: String,
    /// The event type value
    /// The event type value
    pub event_type: String,
    pub timestamp: DateTime<Utc>,
    /// The severity value
    /// The severity value
    pub severity: String,
    /// Optional source ip
    /// Optional source ip
    pub source_ip: Option<String>,
    /// Optional destination ip
    /// Optional destination ip
    pub destination_ip: Option<String>,
    pub user_id: Option<String>,
    /// Name of the process
    /// Name of the process
    pub process_name: Option<String>,
    /// Optional file path
    /// Optional file path
    pub file_path: Option<String>,
    /// Optional command line
    /// Optional command line
    pub command_line: Option<String>,
    /// Optional network protocol
    /// Optional network protocol
    pub network_protocol: Option<String>,
    /// Optional bytes transferred
    /// Optional bytes transferred
    pub bytes_transferred: Option<u64>,
    /// Mapping of metadata
    /// Mapping of metadata
    pub metadata: HashMap<String, String>,
}

impl SecurityEvent {
    /// Create a new security event
    /// Creates a new instance
    pub fn new(event_type: &str, timestamp: DateTime<Utc>, severity: &str) -> Self {
        Self {
            id: uuid::Uuid::new_v4().to_string(),
            event_type: event_type.to_string(),
            timestamp,
            severity: severity.to_string(),
            source_ip: None,
            destination_ip: None,
            user_id: None,
            process_name: None,
            file_path: None,
            command_line: None,
            network_protocol: None,
            bytes_transferred: None,
            metadata: HashMap::new(),
        }
    }

    /// Add source IP to the event
    /// Creates instance with source ip
    pub fn with_source_ip(mut self, ip: &str) -> Self {
        self.source_ip = Some(ip.to_string());
        self
    }

    /// Add user ID to the event
    /// Creates instance with user id
    pub fn with_user_id(mut self, user_id: &str) -> Self {
        self.user_id = Some(user_id.to_string());
        self
    }

    /// Add destination IP to the event
    /// Creates instance with destination ip
    pub fn with_destination_ip(mut self, ip: &str) -> Self {
        self.destination_ip = Some(ip.to_string());
        self
    }

    /// Add process name to the event
    /// Creates instance with process name
    pub fn with_process_name(mut self, process: &str) -> Self {
        self.process_name = Some(process.to_string());
        self
    }
}

/// Threat analysis result
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ThreatAnalysisResult {
    pub analysis_id: String,
    pub event_id: String,
    /// Whether threat_detected is enabled
    /// Whether threat_detected is enabled
    pub threat_detected: bool,
    pub confidence: f64,
    /// The threat level value
    /// The threat level value
    pub threat_level: String,
    /// The details value
    /// The details value
    pub details: String,
    pub timestamp: DateTime<Utc>,
    pub processing_time_ms: u64,
}

impl ThreatAnalysisResult {
    /// Create a new threat analysis result
    /// Creates a new instance
    pub fn new(analysis_id: &str, event_id: &str) -> Self {
        Self {
            analysis_id: analysis_id.to_string(),
            event_id: event_id.to_string(),
            threat_detected: false,
            confidence: 0.0,
            threat_level: "low".to_string(),
            details: format!("Analysis {analysis_id} for event {event_id}"),
            timestamp: Utc::now(),
            processing_time_ms: 0,
        }
    }

    /// Set threat detected flag
    /// Creates instance with threat detected
    pub fn with_threat_detected(mut self, detected: bool) -> Self {
        self.threat_detected = detected;
        self
    }

    /// Set confidence score
    /// Creates instance with confidence
    pub fn with_confidence(mut self, confidence: f64) -> Self {
        self.confidence = confidence;
        self
    }

    /// Set threat level
    /// Creates instance with threat level
    pub fn with_threat_level(mut self, level: &str) -> Self {
        self.threat_level = level.to_string();
        self
    }

    /// Check if analysis has threats
    /// Checks if threats
    /// Checks if threats
    pub fn has_threats(&self) -> bool {
        self.threat_detected
    }

    /// Check if analysis is high confidence
    /// Checks if high confidence
    /// Checks if high confidence
    pub fn is_high_confidence(&self) -> bool {
        self.confidence > 0.8
    }
}

/// Event correlation result
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct EventCorrelationResult {
    pub correlation_id: String,
    /// Collection of events
    /// Collection of events
    pub events: Vec<SecurityEvent>,
    /// The correlation type value
    /// The correlation type value
    pub correlation_type: CorrelationType,
    /// The correlation score value
    /// The correlation score value
    pub correlation_score: f64,
    pub timestamp: DateTime<Utc>,
    /// The description value
    /// The description value
    pub description: String,
}

/// Correlation types
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
/// Types of correlation
/// Types of correlation
pub enum CorrelationType {
    /// Represents temporal variant
    Temporal,
    /// Represents spatial variant
    Spatial,
    /// Represents behavioral variant
    Behavioral,
    /// Represents signature variant
    Signature,
    /// Represents anomaly variant
    Anomaly,
}

/// Threat analysis session
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ThreatAnalysisSession {
    pub session_id: String,
    /// The started at value
    /// The started at value
    pub started_at: DateTime<Utc>,
    /// Optional completed at
    /// Optional completed at
    pub completed_at: Option<DateTime<Utc>>,
    /// Number of events_analyzed
    /// Number of events_analyzed
    pub events_analyzed: usize,
    /// Number of threats_found
    /// Number of threats_found
    pub threats_found: usize,
    /// Collection of analysis results
    /// Collection of analysis results
    pub analysis_results: Vec<ThreatAnalysisResult>,
    /// The session metrics value
    /// The session metrics value
    pub session_metrics: AnalysisMetrics,
}

impl ThreatAnalysisSession {
    /// Create a new analysis session
    /// Creates a new instance
    pub fn new(session_id: &str) -> Self {
        Self {
            session_id: session_id.to_string(),
            started_at: Utc::now(),
            completed_at: None,
            events_analyzed: 0,
            threats_found: 0,
            analysis_results: Vec::new(),
            session_metrics: AnalysisMetrics::new(),
        }
    }

    /// Add analysis result to session
    pub fn add_result(&mut self, result: ThreatAnalysisResult) {
        self.events_analyzed += 1;
        if result.threat_detected {
            self.threats_found += 1;
        }
        self.analysis_results.push(result);
    }

    /// Complete the analysis session
    pub fn complete(&mut self) {
        self.completed_at = Some(Utc::now());
    }

    /// Check if session is completed
    /// Checks if completed
    /// Checks if completed
    pub fn is_completed(&self) -> bool {
        self.completed_at.is_some()
    }

    /// Get session duration in seconds
    pub fn duration_seconds(&self) -> Option<i64> {
        self.completed_at
            .map(|end| (end - self.started_at).num_seconds())
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    async fn test_analysis_metrics() {
        let mut metrics = AnalysisMetrics::new();

        metrics.update_with_analysis(100.0, true);
        metrics.update_with_analysis(150.0, false);
        metrics.update_with_analysis(200.0, true);

        assert_eq!(metrics.total_events_processed, 3);
        // TEST_CATEGORY: unit
        // TEST_DOMAIN: core
        // TEST_PRIORITY: normal
        assert_eq!(metrics.threats_detected, 2);
        assert_eq!(metrics.threat_detection_rate(), 2.0 / 3.0);
        assert!(metrics.avg_processing_time() > 0.0);
    }

    #[test]
    async fn test_security_event_creation() {
        let event = SecurityEvent::new("login_failure", Utc::now(), "high")
            .with_source_ip("192.168.1.100")
            .with_user_id("admin");

        // TEST_CATEGORY: unit
        // TEST_DOMAIN: core
        // TEST_PRIORITY: normal
        assert_eq!(event.event_type, "login_failure");
        assert_eq!(event.severity, "high");
        assert_eq!(event.source_ip, Some("192.168.1.100".to_string()));
        assert_eq!(event.user_id, Some("admin".to_string()));
    }

    #[test]
    async fn test_threat_analysis_result() {
        let result = ThreatAnalysisResult::new("analysis-1", "event-1")
            // TEST_CATEGORY: unit
            // TEST_DOMAIN: core
            // TEST_PRIORITY: normal
            .with_threat_detected(true)
            .with_confidence(0.9)
            .with_threat_level("high");

        assert!(result.threat_detected);
        assert!(result.has_threats());
        assert!(result.is_high_confidence());
        assert_eq!(result.confidence, 0.9);
        assert_eq!(result.threat_level, "high");
    }

    // TEST_CATEGORY: unit
    // TEST_DOMAIN: core
    // TEST_PRIORITY: normal
    #[test]
    async fn test_analysis_session() {
        let mut session = ThreatAnalysisSession::new("session-1");

        let result1 = ThreatAnalysisResult::new("analysis-1", "event-1").with_threat_detected(true);
        let result2 =
            ThreatAnalysisResult::new("analysis-2", "event-2").with_threat_detected(false);

        session.add_result(result1);
        session.add_result(result2);

        assert_eq!(session.events_analyzed, 2);
        assert_eq!(session.threats_found, 1);
        assert!(!session.is_completed());

        session.complete();
        assert!(session.is_completed());
        assert!(session.duration_seconds().is_some());
    }
}
