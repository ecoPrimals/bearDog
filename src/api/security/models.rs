//! Security API models and data structures
//!
//! This module contains all request and response types used by the security API,
//! including threat analysis, ML predictions, behavioral analysis, and more.

use serde::{Deserialize, Serialize};
use std::collections::HashMap;

// ============================================================================
// REQUEST/RESPONSE MODELS
// ============================================================================

/// Security event request for threat analysis
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SecurityEventRequest {
    /// Type of security event (e.g., "login_attempt", "data_access", "network_anomaly")
    pub event_type: String,
    /// Source IP address of the event
    pub source_ip: String,
    /// Destination IP address of the event
    pub destination_ip: String,
    /// User ID associated with the event
    pub user_id: String,
    /// Optional data size in bytes for the event
    pub data_size: Option<f64>,
    /// Optional user agent string from the request
    pub user_agent: Option<String>,
    /// Optional geographic location information
    pub location: Option<String>,
    /// Optional file hash for file-related events
    pub file_hash: Option<String>,
    /// Additional custom data fields
    pub additional_data: Option<HashMap<String, String>>,
}

/// Response containing threat analysis results
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ThreatAnalysisResponse {
    /// Unique identifier for the analyzed event
    pub event_id: String,
    /// Number of threats detected in the event
    pub threats_detected: usize,
    /// Overall risk level assessment (Low, Medium, High, Critical)
    pub risk_level: String,
    /// Detailed information about detected threats
    pub detected_threats: Vec<ThreatEventResponse>,
    /// Machine learning predictions for the event
    pub ml_predictions: Vec<MlPredictionResponse>,
    /// Security recommendations based on analysis
    pub recommendations: Vec<String>,
    /// Time taken to process the event in milliseconds
    pub processing_time_ms: u64,
    /// Optional incident ID if an incident was created
    pub incident_created: Option<String>,
}

/// Individual threat event details
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ThreatEventResponse {
    /// Unique identifier for the threat
    pub threat_id: String,
    /// Type of threat detected
    pub threat_type: String,
    /// Severity level of the threat
    pub severity: String,
    /// Human-readable description of the threat
    pub description: String,
    /// Method used to detect the threat
    pub detection_method: String,
    /// MITRE ATT&CK techniques associated with the threat
    pub mitre_techniques: Vec<String>,
    /// Confidence level in the detection (0.0 to 1.0)
    pub confidence: f64,
    /// Number of evidence pieces supporting the detection
    pub evidence_count: usize,
    /// Recommended response actions
    pub response_actions: Vec<String>,
}

/// Machine learning prediction response
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct MlPredictionResponse {
    /// Unique identifier for the ML model
    pub model_id: String,
    /// Type of prediction made (e.g., "anomaly", "classification", "risk_score")
    pub prediction_type: String,
    /// Confidence score for the prediction (0.0 to 1.0)
    pub confidence_score: f64,
    /// Risk level assessment based on prediction
    pub risk_level: String,
    /// Evidence supporting the prediction
    pub evidence: Vec<String>,
    /// Recommended actions based on prediction
    pub recommendations: Vec<String>,
}

/// Batch analysis request for multiple security events
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct BatchAnalysisRequest {
    /// List of security events to analyze
    pub events: Vec<SecurityEventRequest>,
    /// Whether to perform correlation analysis across events
    pub correlation_analysis: Option<bool>,
    /// Priority level for batch processing
    pub priority: Option<String>,
}

/// Batch analysis response containing results for multiple events
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct BatchAnalysisResponse {
    /// Unique identifier for the batch analysis
    pub batch_id: String,
    /// Total number of events processed
    pub total_events: usize,
    /// Number of threats detected across all events
    pub threats_detected: usize,
    /// Number of high severity threats detected
    pub high_severity_threats: usize,
    /// Number of incidents created from the analysis
    pub incidents_created: usize,
    /// Total processing time for all events in milliseconds
    pub processing_time_ms: u64,
    /// Individual analysis results for each event
    pub results: Vec<ThreatAnalysisResponse>,
}

/// Behavioral analysis request for user behavior patterns
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct BehavioralAnalysisRequest {
    /// User ID to analyze behavior for
    pub user_id: String,
    /// Time window for analysis in hours
    pub time_window_hours: Option<u32>,
    /// Whether to include ML-based behavioral analysis
    pub include_ml_analysis: Option<bool>,
}

/// Behavioral analysis response with user behavior insights
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct BehavioralAnalysisResponse {
    /// User ID that was analyzed
    pub user_id: String,
    /// Anomaly score for the user's behavior (0.0 to 1.0)
    pub anomaly_score: f64,
    /// Risk level assessment based on behavior
    pub risk_level: String,
    /// List of behavioral anomalies detected
    pub anomalies_detected: Vec<String>,
    /// Insights about the user's behavioral patterns
    pub behavioral_insights: Vec<String>,
    /// Recommended actions based on behavioral analysis
    pub recommendations: Vec<String>,
}

/// Threat intelligence request for indicator lookup
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ThreatIntelRequest {
    /// List of indicators to check (IPs, domains, hashes, etc.)
    pub indicators: Vec<String>,
    /// Type of indicators being submitted
    pub indicator_type: String,
}

/// Threat intelligence response with indicator analysis
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ThreatIntelResponse {
    /// Number of malicious indicators found
    pub matches_found: usize,
    /// Details of malicious indicators detected
    pub malicious_indicators: Vec<ThreatIndicatorMatch>,
    /// Confidence scores for each indicator
    pub confidence_scores: HashMap<String, f64>,
}

/// Individual threat indicator match details
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ThreatIndicatorMatch {
    /// The indicator that was matched
    pub indicator: String,
    /// Type of threat associated with the indicator
    pub threat_type: String,
    /// Confidence level in the match (0.0 to 1.0)
    pub confidence: f64,
    /// First time this indicator was seen
    pub first_seen: String,
    /// Source of the threat intelligence
    pub source: String,
}

/// Request to create a custom detection rule
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CreateDetectionRuleRequest {
    /// Name of the detection rule
    pub name: String,
    /// Description of what the rule detects
    pub description: String,
    /// Type of threat this rule detects
    pub threat_type: String,
    /// Severity level of threats detected by this rule
    pub severity: String,
    /// Conditions that must be met for the rule to trigger
    pub conditions: Vec<RuleConditionRequest>,
    /// MITRE ATT&CK techniques covered by this rule
    pub mitre_techniques: Vec<String>,
    /// Automated response actions when rule triggers
    pub response_actions: Vec<String>,
}

/// Individual condition for a detection rule
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct RuleConditionRequest {
    /// Type of condition (e.g., "field_match", "threshold", "pattern")
    pub condition_type: String,
    /// Field name to evaluate (optional)
    pub field: Option<String>,
    /// Operator for comparison (optional)
    pub operator: Option<String>,
    /// Value to compare against (optional)
    pub value: Option<String>,
    /// Threshold value for numeric comparisons (optional)
    pub threshold: Option<f64>,
}

/// Security statistics response with system-wide metrics
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SecurityStatisticsResponse {
    /// Total number of security events analyzed
    pub total_events_analyzed: u64,
    /// Number of threats detected today
    pub threats_detected_today: u64,
    /// Number of active security incidents
    pub active_incidents: u64,
    /// Number of active ML models
    pub ml_models_active: u64,
    /// Number of active detection rules
    pub detection_rules_active: u64,
    /// Number of active threat intelligence feeds
    pub threat_feeds_active: u64,
    /// Average processing time for events in milliseconds
    pub average_processing_time_ms: f64,
    /// Distribution of threat types detected
    pub threat_distribution: HashMap<String, u32>,
    /// Top threat sources by IP address
    pub top_threat_sources: Vec<String>,
} 