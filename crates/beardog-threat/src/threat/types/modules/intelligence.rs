// SPDX-License-Identifier: AGPL-3.0-or-later

//! # Threat Intelligence and ML Types
//!
//! This module provides types for threat intelligence feeds, machine learning models,
//! and advanced analysis capabilities using canonical systems.

use super::core::{ThreatIndicator, IndicatorType};
use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use std::time::SystemTime;

/// Threat intelligence feed configuration and data
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ThreatIntelligenceFeed {
    /// Feed identifier
    pub feed_id: String,
    /// Feed name
    pub name: String,
    /// Feed provider
    pub provider: String,
    /// Feed URL or endpoint
    pub endpoint: String,
    /// Feed type
    pub feed_type: IntelligenceFeedType,
    /// Update frequency in minutes
    pub update_frequency_minutes: u32,
    /// Last update timestamp
    pub last_updated: SystemTime,
    /// Feed reliability score (0.0 - 1.0)
    pub reliability_score: f64,
    /// Whether feed is currently active
    pub is_active: bool,
    /// Whether feed is enabled (compatibility)
    pub enabled: bool,
    /// Authentication configuration
    pub auth_config: FeedAuthConfig,
    /// Feed-specific metadata
    pub metadata: HashMap<String, String>,
    
    // Compatibility fields
    /// ID field (compatibility - alias for feed_id)
    #[serde(skip_serializing_if = "Option::is_none")]
    pub id: Option<String>,
    /// Indicators (compatibility)
    #[serde(skip_serializing_if = "Vec::is_empty", default)]
    pub indicators: Vec<super::core::ThreatIndicator>,
}

/// Types of threat intelligence feeds
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
pub enum IntelligenceFeedType {
    /// IP reputation feed
    IpReputation,
    /// Domain reputation feed
    DomainReputation,
    /// URL reputation feed
    UrlReputation,
    /// File hash feed
    FileHash,
    /// Malware signatures
    MalwareSignatures,
    /// Vulnerability data
    VulnerabilityData,
    /// Attack patterns
    AttackPatterns,
    /// Custom feed
    Custom,
}

/// Authentication configuration for intelligence feeds
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct FeedAuthConfig {
    /// Authentication type
    pub auth_type: AuthenticationType,
    /// API key (if applicable)
    pub api_key: Option<String>,
    /// Username (if applicable)
    pub username: Option<String>,
    /// Password (if applicable) - should be encrypted
    pub password: Option<String>,
    /// Additional headers
    pub headers: HashMap<String, String>,
}

/// Authentication types for feeds
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
pub enum AuthenticationType {
    /// No authentication required
    None,
    /// API key authentication
    ApiKey,
    /// Basic authentication
    Basic,
    /// Bearer token
    Bearer,
    /// Custom authentication
    Custom,
}

/// Machine learning model for threat detection
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct MlModel {
    /// Model identifier
    pub model_id: String,
    /// Model name
    pub name: String,
    /// Model type
    pub model_type: MlModelType,
    /// Model version
    pub version: String,
    /// Training data description
    pub training_data: String,
    /// Model accuracy score (0.0 - 1.0)
    pub accuracy: f64,
    /// Model precision score (0.0 - 1.0)
    pub precision: f64,
    /// Model recall score (0.0 - 1.0)
    pub recall: f64,
    /// F1 score (0.0 - 1.0)
    pub f1_score: f64,
    /// When the model was last trained
    pub last_trained: SystemTime,
    /// Whether the model is currently active
    pub is_active: bool,
    /// Model-specific configuration
    pub config: HashMap<String, String>,
    /// Feature importance scores
    pub feature_importance: HashMap<String, f64>,
    
    // Compatibility fields
    /// ID field (compatibility - alias for model_id)
    #[serde(skip_serializing_if = "Option::is_none")]
    pub id: Option<String>,
}

/// Types of machine learning models
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
pub enum MlModelType {
    /// Anomaly detection model
    AnomalyDetection,
    /// Classification model
    Classification,
    /// Regression model
    Regression,
    /// Clustering model
    Clustering,
    /// Deep learning model
    DeepLearning,
    /// Ensemble model
    Ensemble,
    /// Time series model
    TimeSeries,
}

/// Threat analysis result from ML models or intelligence
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ThreatAnalysisResult {
    /// Analysis identifier
    pub analysis_id: String,
    /// Threat event identifier being analyzed
    pub threat_event_id: String,
    /// Analysis method used
    pub analysis_method: AnalysisMethod,
    /// Analysis confidence score (0.0 - 1.0)
    pub confidence: f64,
    /// Risk score (0-100)
    pub risk_score: u8,
    /// Analysis findings
    pub findings: Vec<AnalysisFinding>,
    /// Recommended actions
    pub recommendations: Vec<String>,
    /// Analysis timestamp
    pub analyzed_at: SystemTime,
    /// Analysis duration in milliseconds
    pub analysis_duration_ms: u64,
    /// Model or intelligence source used
    pub source: String,
}

/// Methods used for threat analysis
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
pub enum AnalysisMethod {
    /// Machine learning model analysis
    MachineLearning,
    /// Threat intelligence lookup
    ThreatIntelligence,
    /// Behavioral analysis
    BehavioralAnalysis,
    /// Statistical analysis
    StatisticalAnalysis,
    /// Rule-based analysis
    RuleBased,
    /// Hybrid analysis (multiple methods)
    Hybrid,
}

/// Individual finding from threat analysis
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct AnalysisFinding {
    /// Finding identifier
    pub finding_id: String,
    /// Finding category
    pub category: FindingCategory,
    /// Finding description
    pub description: String,
    /// Confidence in this finding (0.0 - 1.0)
    pub confidence: f64,
    /// Severity of this finding
    pub severity: FindingSeverity,
    /// Supporting evidence
    pub evidence: Vec<String>,
    /// Related indicators
    pub indicators: Vec<ThreatIndicator>,
}

/// Categories of analysis findings
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
pub enum FindingCategory {
    /// Malicious activity detected
    MaliciousActivity,
    /// Suspicious behavior observed
    SuspiciousBehavior,
    /// Policy violation detected
    PolicyViolation,
    /// Anomalous pattern identified
    AnomalousPattern,
    /// Known threat signature matched
    KnownThreatSignature,
    /// Vulnerability exploited
    VulnerabilityExploit,
    /// Data exfiltration attempt
    DataExfiltration,
    /// Unauthorized access attempt
    UnauthorizedAccess,
}

/// Severity levels for analysis findings
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq, PartialOrd, Ord)]
pub enum FindingSeverity {
    /// Informational finding
    Info,
    /// Low severity finding
    Low,
    /// Medium severity finding
    Medium,
    /// High severity finding
    High,
    /// Critical severity finding
    Critical,
}

/// Threat hunting query for proactive threat detection
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ThreatHuntingQuery {
    /// Query identifier
    pub query_id: String,
    /// Query name
    pub name: String,
    /// Query description
    pub description: String,
    /// Query logic/pattern
    pub query: String,
    /// Query language/type
    pub query_type: QueryType,
    /// Expected indicators to find
    pub expected_indicators: Vec<IndicatorType>,
    /// Query tags for categorization
    pub tags: Vec<String>,
    /// Query author
    pub author: String,
    /// When query was created
    pub created_at: SystemTime,
    /// When query was last modified
    pub last_modified: SystemTime,
    /// Whether query is active
    pub is_active: bool,
}

/// Types of threat hunting queries
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
pub enum QueryType {
    /// SQL-based query
    Sql,
    /// KQL (Kusto Query Language)
    Kql,
    /// Elasticsearch query
    Elasticsearch,
    /// Splunk query
    Splunk,
    /// Regular expression
    Regex,
    /// YARA rule
    Yara,
    /// Sigma rule
    Sigma,
    /// Custom query format
    Custom,
}

/// Result from threat hunting activities
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ThreatHuntingResult {
    /// Result identifier
    pub result_id: String,
    /// Query that generated this result
    pub query_id: String,
    /// Hunt session identifier
    pub hunt_session_id: String,
    /// Matching events or indicators found
    pub matches: Vec<HuntMatch>,
    /// Total number of matches
    pub match_count: u32,
    /// Hunt execution timestamp
    pub executed_at: SystemTime,
    /// Execution duration in milliseconds
    pub execution_duration_ms: u64,
    /// Hunter who executed the query
    pub hunter: String,
    /// Result status
    pub status: HuntStatus,
}

/// Individual match from threat hunting
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct HuntMatch {
    /// Match identifier
    pub match_id: String,
    /// Matched data/event
    pub matched_data: String,
    /// Match confidence score (0.0 - 1.0)
    pub confidence: f64,
    /// Context information
    pub context: HashMap<String, String>,
    /// Related threat indicators
    pub indicators: Vec<ThreatIndicator>,
    /// Match timestamp
    pub matched_at: SystemTime,
}

/// Status of threat hunting activities
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
pub enum HuntStatus {
    /// Hunt in progress
    InProgress,
    /// Hunt completed successfully
    Completed,
    /// Hunt failed with errors
    Failed,
    /// Hunt was cancelled
    Cancelled,
    /// Hunt timed out
    TimedOut,
}

impl Default for FeedAuthConfig {
    fn default() -> Self {
        Self {
            auth_type: AuthenticationType::None,
            api_key: None,
            username: None,
            password: None,
            headers: HashMap::new(),
        }
    }
} 