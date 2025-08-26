

use serde::{Deserialize, Serialize};
use std::collections::HashMap;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SecurityEventRequest {

    pub event_type: String,

    pub source_ip: String,

    pub destination_ip: String,

    pub user_id: String,

    pub data_size: Option<f64>,

    pub user_agent: Option<String>,

    pub location: Option<String>,

    pub file_hash: Option<String>,

    pub additional_data: Option<HashMap<String, String>>,
}

pub struct ThreatAnalysisResponse {

    pub event_id: String,

    pub threats_detected: usize,

    pub risk_level: String,

    pub detected_threats: Vec<ThreatEventResponse>,

    pub ml_predictions: Vec<MlPredictionResponse>,

    pub recommendations: Vec<String>,

    pub processing_time_ms: u64,

    pub incident_created: Option<String>,

pub struct ThreatEventResponse {

    pub threat_id: String,

    pub threat_type: String,

    pub severity: String,

    pub description: String,

    pub evidence: Vec<String>,

pub struct MlPredictionResponse {

    pub model_id: String,

    pub prediction_type: String,

    pub confidence_score: f64,

pub struct BatchAnalysisRequest {

    pub events: Vec<SecurityEventRequest>,

    pub correlation_analysis: Option<bool>,

    pub priority: Option<String>,

pub struct BatchAnalysisResponse {

    pub batch_id: String,

    pub total_events: usize,

    pub high_severity_threats: usize,

    pub incidents_created: usize,

    pub results: Vec<ThreatAnalysisResponse>,

pub struct BehavioralAnalysisRequest {

    pub time_window_hours: Option<u32>,

    pub include_ml_analysis: Option<bool>,

pub struct BehavioralAnalysisResponse {

    pub anomaly_score: f64,

    pub anomalies_detected: Vec<String>,

    pub behavioral_insights: Vec<String>,

pub struct ThreatIntelRequest {

    pub indicators: Vec<String>,

    pub indicator_type: String,

pub struct ThreatIntelResponse {

    pub matches_found: usize,

    pub malicious_indicators: Vec<ThreatIndicatorMatch>,

    pub confidence_scores: HashMap<String, f64>,

pub struct ThreatIndicatorMatch {

    pub indicator: String,

    pub confidence: f64,

    pub first_seen: String,

    pub source: String,

pub struct CreateDetectionRuleRequest {

    pub name: String,

    pub conditions: Vec<RuleConditionRequest>,

    pub mitre_techniques: Vec<String>,

    pub response_actions: Vec<String>,

pub struct RuleConditionRequest {

    pub condition_type: String,

    pub field: Option<String>,

    pub operator: Option<String>,

    pub value: Option<String>,

    pub threshold: Option<f64>,

pub struct SecurityStatisticsResponse {

    pub total_events_analyzed: u64,

    pub threats_detected_today: u64,

    pub active_incidents: u64,

    pub ml_models_active: u64,

    pub detection_rules_active: u64,

    pub threat_feeds_active: u64,

    pub average_processing_time_ms: f64,

    pub threat_distribution: HashMap<String, u32>,

    pub top_threat_sources: Vec<String>,
