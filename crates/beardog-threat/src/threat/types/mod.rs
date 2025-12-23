// Threat Detection Types - Modern Implementation
//
// **MODERNIZED**: Clean, production-ready type definitions for the BearDog threat detection system.

use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use std::time::SystemTime;

pub mod incidents;

pub mod engine;

// Re-export engine types
pub use engine::threat_engine::*;

// Re-export canonical threat detection configuration
pub use beardog_types::canonical::config::domains::threat::ThreatDetectionConfig;

/// Threat event representing a detected security incident
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ThreatEvent {
    pub id: String,
    /// Type of threat detected
    /// The threat type value
    /// The threat type value
    pub threat_type: ThreatType,
    /// Severity level of the threat
    /// The severity value
    /// The severity value
    pub severity: ThreatSeverity,
    /// Current status of the threat
    /// Current status of the component
    /// Current status of the component
    pub status: ThreatStatus,
    /// Source of the threat
    /// The source value
    /// The source value
    pub source: ThreatSource,
    /// Target of the threat
    /// The target value
    /// The target value
    pub target: ThreatTarget,
    /// Timestamp when threat was detected
    /// The detected at value
    /// The detected at value
    pub detected_at: SystemTime,
    pub timestamp: SystemTime,
    /// Threat confidence score (0.0 - 1.0)
    pub confidence: f64,
    /// Threat score (0-100)
    /// Number of score
    /// Number of score
    pub score: u8,
    /// Threat description
    /// The description value
    /// The description value
    pub description: String,
    /// Detection method used
    /// The detection method value
    /// The detection method value
    pub detection_method: DetectionMethod,
    /// Evidence collected
    pub evidence: Vec<String>,
    /// Recommended actions
    /// Collection of recommended actions
    /// Collection of recommended actions
    pub recommended_actions: Vec<ThreatAction>,
    /// Assigned analyst
    /// Optional assigned analyst
    /// Optional assigned analyst
    pub assigned_analyst: Option<String>,
    /// Related events
    /// Collection of related events
    /// Collection of related events
    pub related_events: Vec<String>,
    /// Raw event data
    /// Optional raw data
    /// Optional raw data
    pub raw_data: Option<serde_json::Value>,
    /// Whether threat has been mitigated
    /// Whether mitigated is enabled
    /// Whether mitigated is enabled
    pub mitigated: bool,
    /// Mitigation actions taken
    /// Collection of mitigation actions
    /// Collection of mitigation actions
    pub mitigation_actions: Vec<ThreatAction>,
    /// Additional metadata
    /// Mapping of metadata
    /// Mapping of metadata
    pub metadata: HashMap<String, String>,
    /// Mitigation steps taken
    /// Collection of mitigation steps
    /// Collection of mitigation steps
    pub mitigation_steps: Vec<MitigationStep>,
}

/// Source of the threat
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ThreatSource {
    /// Source type (IP, domain, process, etc.)
    /// The source type value
    /// The source type value
    pub source_type: String,
    /// Source identifier
    pub identifier: String,
    pub id: String,
    /// IP address if applicable
    /// Optional ip address
    /// Optional ip address
    pub ip_address: Option<String>,
    /// Hostname if applicable
    /// Name of the hostitem
    /// Name of the hostitem
    pub hostname: Option<String>,
    /// User agent if applicable
    /// Optional user agent
    /// Optional user agent
    pub user_agent: Option<String>,
    /// Geographic location if available
    /// Optional location
    /// Optional location
    pub location: Option<String>,
    /// Optional geolocation
    /// Optional geolocation
    pub geolocation: Option<String>,
    /// Optional threat actor
    /// Optional threat actor
    pub threat_actor: Option<String>,
    /// Source classification
    /// The classification value
    /// The classification value
    pub classification: SourceClassification,
    /// Reputation score if available
    /// Optional reputation
    /// Optional reputation
    pub reputation: Option<f64>,
    /// Reputation score (alias)
    /// The reputation score value
    /// The reputation score value
    pub reputation_score: f64,
    /// Confidence score
    pub confidence_score: f64,
    /// First seen timestamp
    /// Optional first seen
    /// Optional first seen
    pub first_seen: Option<SystemTime>,
    /// Last seen timestamp
    /// Optional last seen
    /// Optional last seen
    pub last_seen: Option<SystemTime>,
    /// Additional metadata
    /// Mapping of metadata
    /// Mapping of metadata
    pub metadata: HashMap<String, String>,
}

impl Default for ThreatSource {
    fn default() -> Self {
        Self {
            source_type: "unknown".to_string(),
            identifier: "unknown".to_string(),
            id: uuid::Uuid::new_v4().to_string(),
            ip_address: None,
            hostname: None,
            user_agent: None,
            location: None,
            geolocation: None,
            threat_actor: None,
            classification: SourceClassification::Unknown,
            reputation: None,
            reputation_score: 0.0,
            confidence_score: 0.0,
            first_seen: None,
            last_seen: None,
            metadata: HashMap::new(),
        }
    }
}

/// Target of the threat
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ThreatTarget {
    /// Target type (system, service, data, etc.)
    /// The target type value
    /// The target type value
    pub target_type: String,
    /// Target identifier
    pub identifier: String,
    pub id: String,
    /// Resource identifier
    pub resource_id: String,
    /// Node identifier
    pub node_id: Option<String>,
    /// User account if applicable
    /// Number of `user_acitems`
    /// Number of `user_acitems`
    pub user_account: Option<String>,
    /// Asset criticality
    /// The asset criticality value
    /// The asset criticality value
    pub asset_criticality: AssetCriticality,
    /// Protection level
    /// The protection level value
    /// The protection level value
    pub protection_level: ProtectionLevel,
    /// Service name if applicable
    /// Optional service
    /// Optional service
    pub service: Option<String>,
    /// Port number if applicable
    /// Optional port
    /// Optional port
    pub port: Option<u16>,
    /// Protocol if applicable
    /// Optional protocol
    /// Optional protocol
    pub protocol: Option<String>,
    /// Additional metadata
    /// Mapping of metadata
    /// Mapping of metadata
    pub metadata: HashMap<String, String>,
    /// Resource type
    /// The resource type value
    /// The resource type value
    pub resource_type: String,
    /// Criticality level of the target
    /// The criticality value
    /// The criticality value
    pub criticality: ThreatSeverity,
    /// IP address if applicable
    /// Optional ip address
    /// Optional ip address
    pub ip_address: Option<String>,
    /// Hostname if applicable
    /// Name of the hostitem
    /// Name of the hostitem
    pub hostname: Option<String>,
}

impl Default for ThreatTarget {
    fn default() -> Self {
        Self {
            target_type: "unknown".to_string(),
            identifier: "unknown".to_string(),
            id: uuid::Uuid::new_v4().to_string(),
            resource_id: "unknown".to_string(),
            node_id: None,
            user_account: None,
            asset_criticality: AssetCriticality::Low,
            protection_level: ProtectionLevel::Basic,
            service: None,
            port: None,
            protocol: None,
            metadata: HashMap::new(),
            resource_type: "unknown".to_string(),
            criticality: ThreatSeverity::Low,
            ip_address: None,
            hostname: None,
        }
    }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum RuleCondition {
    /// Field equals a specific value
    FieldEquals { field: String, value: String },
    /// Field greater than a threshold
    FieldGreaterThan { field: String, threshold: f64 },
    /// Field less than a threshold
    FieldLessThan { field: String, threshold: f64 },
    /// Field contains a pattern
    FieldContains { field: String, pattern: String },
    /// Complex condition with multiple criteria
    Complex { conditions: Vec<RuleCondition> },
}

/// Types of threats that can be detected
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
/// Types of threat
/// Types of threat
pub enum ThreatType {
    /// Malware detection
    Malware,
    /// Intrusion attempt
    Intrusion,
    /// Data exfiltration
    DataExfiltration,
    /// Denial of service attack
    DenialOfService,
    /// Privilege escalation
    PrivilegeEscalation,
    /// Suspicious network activity
    SuspiciousNetwork,
    /// Configuration tampering
    ConfigurationTampering,
    /// Unauthorized access
    UnauthorizedAccess,
    /// Resource abuse
    ResourceAbuse,
    /// Anomalous behavior detected
    Anomaly,
    /// Suspicious activity
    Suspicious,
    /// Malicious activity confirmed
    Malicious,
    /// Unknown threat pattern
    Unknown,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq, PartialOrd, Ord)]
pub enum ThreatSeverity {
    /// Low severity - monitoring only
    Low,
    /// Medium severity - requires attention
    Medium,
    /// High severity - immediate action required
    High,
    /// Critical severity - emergency response
    Critical,
}

impl ThreatSeverity {
    /// Convert severity to string representation
    /// Returns as str
    #[must_use]
    pub const fn as_str(&self) -> &'static str {
        match self {
            Self::Low => "low",
            Self::Medium => "medium",
            Self::High => "high",
            Self::Critical => "critical",
        }
    }

    #[must_use]
    pub const fn score(&self) -> u8 {
        match self {
            Self::Low => 1,
            Self::Medium => 2,
            Self::High => 3,
            Self::Critical => 4,
        }
    }
}

/// Status of threat handling
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
pub enum ThreatStatus {
    /// Threat detected but not yet processed
    Detected,
    /// Threat is being analyzed
    Analyzing,
    /// Threat is being mitigated
    Mitigating,
    /// Threat has been contained
    Contained,
    /// Threat has been resolved
    Resolved,
    /// False positive - not a real threat
    FalsePositive,
    /// Threat is currently active
    Active,
}

/// Detection method used to identify threat
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
pub enum DetectionMethod {
    /// Rule-based detection
    RuleBased,
    /// Machine learning detection
    MachineLearning,
    /// Threat intelligence matching
    ThreatIntelligence,
    /// Behavioral analysis
    BehavioralAnalysis,
    /// Signature matching
    SignatureMatching,
    /// Anomaly detection
    AnomalyDetection,
}

/// Classification of threat source
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
pub enum SourceClassification {
    /// Trusted source
    Trusted,
    /// Unknown source
    Unknown,
    /// Suspicious source
    Suspicious,
    /// Known malicious source
    KnownMalicious,
    /// Compromised legitimate source
    Compromised,
}

/// Asset criticality levels
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
pub enum AssetCriticality {
    /// Low criticality
    Low,
    /// Medium criticality
    Medium,
    /// High criticality
    High,
    /// Critical asset
    Critical,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
pub enum ProtectionLevel {
    /// Basic protection
    Basic,
    /// Standard protection
    Standard,
    /// Enhanced protection
    Enhanced,
    /// Maximum protection
    Maximum,
}

/// Threat response actions
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
pub enum ThreatAction {
    /// Block the source
    BlockSource,
    /// Alert security team
    AlertSecurityTeam,
    /// Investigate activity
    InvestigateActivity,
    /// Quarantine asset
    QuarantineAsset,
    /// Escalate to administrator
    EscalateToAdmin,
    LogForAnalysis,
}

/// Types of threat detection rules
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
/// Types of threat rule
/// Types of threat rule
pub enum ThreatRuleType {
    /// Signature-based rule
    Signature,
    /// Anomaly detection rule
    Anomaly,
    /// Behavioral analysis rule
    Behavioral,
    /// Machine learning rule
    MachineLearning,
    /// Heuristic rule
    Heuristic,
    /// Custom rule
    Custom,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
/// Types of indicator
/// Types of indicator
pub enum IndicatorType {
    /// IP address indicator
    IpAddress,
    /// Domain name indicator
    DomainName,
    /// URL indicator
    Url,
    /// File hash indicator
    FileHash,
    /// Email address indicator
    EmailAddress,
    /// User agent indicator
    UserAgent,
    /// Registry key indicator
    RegistryKey,
    /// Process name indicator
    ProcessName,
}

impl std::fmt::Display for IndicatorType {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let s = match self {
            Self::IpAddress => "IP Address",
            Self::DomainName => "Domain Name",
            Self::Url => "URL",
            Self::FileHash => "File Hash",
            Self::EmailAddress => "Email Address",
            Self::UserAgent => "User Agent",
            Self::RegistryKey => "Registry Key",
            Self::ProcessName => "Process Name",
        };
        write!(f, "{s}")
    }
}

/// Threat intelligence indicator
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ThreatIndicator {
    /// Indicator type
    /// The indicator type value
    /// The indicator type value
    pub indicator_type: IndicatorType,
    /// Indicator value
    /// The value value
    /// The value value
    pub value: String,
    /// Confidence in indicator
    pub confidence: f64,
    /// Source of indicator
    /// The source value
    /// The source value
    pub source: String,
    /// Timestamp when indicator was created
    /// The created at value
    /// The created at value
    pub created_at: SystemTime,
}

/// Threat intelligence feed
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ThreatIntelligenceFeed {
    /// Feed identifier
    pub id: String,
    /// Feed name
    /// Name of the item
    /// Name of the item
    pub name: String,
    /// Feed source
    /// The source value
    /// The source value
    pub source: String,
    /// Last update timestamp
    /// The last updated value
    /// The last updated value
    pub last_updated: SystemTime,
    /// Whether feed is enabled
    /// Whether feature is enabled
    /// Whether feature is enabled
    pub enabled: bool,
    /// Feed reliability score
    /// The reliability value
    /// The reliability value
    pub reliability: f64,
    /// Indicators in this feed
    /// Collection of indicators
    /// Collection of indicators
    pub indicators: Vec<ThreatIndicator>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct MitigationStep {
    /// Step identifier
    pub id: String,
    /// Action taken
    /// The action value
    /// The action value
    pub action: String,
    /// Timestamp when action was taken
    pub timestamp: SystemTime,
    /// Result of the action
    /// The result value
    /// The result value
    pub result: String,
    /// Success status
    /// Whether success is enabled
    /// Whether success is enabled
    pub success: bool,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SecurityEvent {
    /// Event identifier
    pub id: String,
    /// Event type
    /// The event type value
    /// The event type value
    pub event_type: String,
    /// Event timestamp
    pub timestamp: SystemTime,
    /// Event severity
    /// The severity value
    /// The severity value
    pub severity: ThreatSeverity,
    /// Event source
    /// The source value
    /// The source value
    pub source: String,
    /// Event description
    /// The description value
    /// The description value
    pub description: String,
    /// Additional event data
    /// Mapping of data
    /// Mapping of data
    pub data: HashMap<String, String>,
}

impl SecurityEvent {
    /// Create a new security event
    /// Creates a new instance
    #[must_use]
    pub fn new(event_type: &str, timestamp: chrono::DateTime<chrono::Utc>, source: &str) -> Self {
        Self {
            id: uuid::Uuid::new_v4().to_string(),
            event_type: event_type.to_string(),
            timestamp: timestamp.into(),
            severity: ThreatSeverity::Low,
            source: source.to_string(),
            description: String::new(),
            data: HashMap::new(),
        }
    }

    /// Add source IP to event data
    /// Creates instance with source ip
    #[must_use]
    pub fn with_source_ip(mut self, source_ip: &str) -> Self {
        self.data
            .insert("source_ip".to_string(), source_ip.to_string());
        self
    }

    /// Add user ID to event data
    /// Creates instance with user id
    #[must_use]
    pub fn with_user_id(mut self, user_id: &str) -> Self {
        self.data.insert("user_id".to_string(), user_id.to_string());
        self
    }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct MlModel {
    /// Model identifier
    pub id: String,
    /// Model name
    /// Name of the item
    /// Name of the item
    pub name: String,
    /// Model type
    /// The model type value
    /// The model type value
    pub model_type: MlModelType,
    /// Model accuracy score
    /// The accuracy value
    /// The accuracy value
    pub accuracy: f64,
    /// Model version
    /// The version value
    /// The version value
    pub version: String,
    /// Model training timestamp
    /// The trained at value
    /// The trained at value
    pub trained_at: SystemTime,
}

/// Types of machine learning models
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
/// Types of ml model
/// Types of ml model
pub enum MlModelType {
    /// Anomaly detection model
    AnomalyDetection,
    /// Classification model
    Classification,
    /// Clustering model
    Clustering,
    /// Neural network model
    NeuralNetwork,
    /// Decision tree model
    DecisionTree,
    /// Ensemble model
    Ensemble,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct IncidentResponse {
    /// Incident identifier
    pub id: String,
    /// Related threat event ID
    pub threat_id: String,
    /// Response status
    /// Current status of the component
    /// Current status of the component
    pub status: ResponseStatus,
    /// Response team assigned
    /// The assigned team value
    /// The assigned team value
    pub assigned_team: String,
    /// Response start time
    /// The started at value
    /// The started at value
    pub started_at: SystemTime,
    /// Response completion time
    /// Optional completed at
    /// Optional completed at
    pub completed_at: Option<SystemTime>,
    /// Response actions taken
    /// Collection of actions
    /// Collection of actions
    pub actions: Vec<ResponseAction>,
    /// Response notes
    /// The notes value
    /// The notes value
    pub notes: String,
}

/// Status of incident response
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
pub enum ResponseStatus {
    /// Response initiated
    Initiated,
    /// Response in progress
    InProgress,
    /// Response escalated
    Escalated,
    /// Response completed
    Completed,
    /// Response failed
    Failed,
}

/// Action taken during incident response
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ResponseAction {
    /// Action identifier
    pub id: String,
    /// Action type
    /// The action type value
    /// The action type value
    pub action_type: String,
    /// Action description
    /// The description value
    /// The description value
    pub description: String,
    /// Action timestamp
    pub timestamp: SystemTime,
    /// Action result
    /// The result value
    /// The result value
    pub result: String,
    /// Action success status
    /// Whether success is enabled
    /// Whether success is enabled
    pub success: bool,
}

impl ThreatEvent {
    /// Create a new threat event
    /// Creates a new instance
    #[must_use]
    pub fn new(
        id: String,
        threat_type: ThreatType,
        severity: ThreatSeverity,
        source: ThreatSource,
        target: ThreatTarget,
    ) -> Self {
        let now = SystemTime::now();
        Self {
            id,
            threat_type,
            severity,
            status: ThreatStatus::Detected,
            source,
            target,
            detected_at: now,
            timestamp: now,
            confidence: 0.5,
            score: 50,
            description: String::new(),
            detection_method: DetectionMethod::RuleBased,
            evidence: Vec::new(),
            recommended_actions: Vec::new(),
            assigned_analyst: None,
            related_events: Vec::new(),
            raw_data: None,
            mitigated: false,
            mitigation_actions: Vec::new(),
            metadata: HashMap::new(),
            mitigation_steps: Vec::new(),
        }
    }

    /// Add a mitigation step to the threat event
    pub fn add_mitigation_step(&mut self, step: MitigationStep) {
        self.mitigation_steps.push(step);
    }

    /// Update threat status
    /// Updates status
    /// Updates status
    pub fn update_status(&mut self, status: ThreatStatus) {
        self.status = status;
    }

    /// Check if threat is active
    /// Checks if active
    /// Checks if active
    #[must_use]
    pub const fn is_active(&self) -> bool {
        matches!(
            self.status,
            ThreatStatus::Detected | ThreatStatus::Analyzing | ThreatStatus::Mitigating
        )
    }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct DetectionRule {
    /// Unique rule identifier
    pub id: String,
    /// Human-readable rule name
    /// Name of the item
    /// Name of the item
    pub name: String,
    /// Rule description
    /// The description value
    /// The description value
    pub description: String,
    /// Rule pattern or signature
    /// The pattern value
    /// The pattern value
    pub pattern: String,
    /// Rule severity
    /// The severity value
    /// The severity value
    pub severity: ThreatSeverity,
    /// Rule enabled status
    /// Whether feature is enabled
    /// Whether feature is enabled
    pub enabled: bool,
    /// Rule confidence score
    pub confidence: f64,
    /// The condition value
    /// The condition value
    pub condition: RuleCondition,
    /// The rule type value
    /// The rule type value
    pub rule_type: ThreatRuleType,
}

impl Default for DetectionRule {
    fn default() -> Self {
        Self {
            id: String::new(),
            name: String::new(),
            description: String::new(),
            pattern: String::new(),
            severity: ThreatSeverity::Medium,
            enabled: true,
            confidence: 0.5,
            condition: RuleCondition::FieldEquals {
                field: String::new(),
                value: String::new(),
            },
            rule_type: ThreatRuleType::Signature,
        }
    }
}

impl DetectionRule {
    /// Create a new detection rule
    /// Creates a new instance
    #[must_use]
    pub const fn new(
        id: String,
        name: String,
        description: String,
        pattern: String,
        severity: ThreatSeverity,
    ) -> Self {
        Self {
            id,
            name,
            description,
            pattern,
            severity,
            enabled: true,
            confidence: 0.8,
            condition: RuleCondition::FieldEquals {
                field: String::new(),
                value: String::new(),
            },
            rule_type: ThreatRuleType::Signature,
        }
    }

    /// Enable the detection rule
    pub fn enable(&mut self) {
        self.enabled = true;
    }

    /// Disable the detection rule
    pub fn disable(&mut self) {
        self.enabled = false;
    }
}

impl MitigationStep {
    /// Create a new mitigation step
    /// Creates a new instance
    #[must_use]
    pub fn new(id: String, action: String, result: String, success: bool) -> Self {
        Self {
            id,
            action,
            timestamp: SystemTime::now(),
            result,
            success,
        }
    }
}
