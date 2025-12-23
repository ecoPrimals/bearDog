//! # Core Threat Detection Types
//!
//! This module provides the fundamental types for threat detection including
//! configuration, events, and core enumerations using canonical systems.

use beardog_types::constants::domains::system::defaults::DEFAULT_QUEUE_SIZE;
use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use std::time::SystemTime;

// Use canonical threat detection configuration
pub use beardog_types::canonical::config::domains::threat::ThreatDetectionConfig;

/// Threat event representing a detected security incident
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ThreatEvent {
    /// Unique identifier for the threat event
    pub id: String,
    /// Type of threat detected
    pub threat_type: ThreatType,
    /// Severity level of the threat
    pub severity: ThreatSeverity,
    /// Current status of the threat
    pub status: ThreatStatus,
    /// Source of the threat
    pub source: ThreatSource,
    /// Target of the threat
    pub target: ThreatTarget,
    /// Timestamp when threat was detected
    pub detected_at: SystemTime,
    /// Alternative timestamp field for compatibility
    pub timestamp: SystemTime,
    /// Threat confidence score (0.0 - 1.0)
    pub confidence: f64,
    /// Threat score (0-100)
    pub score: u8,
    /// Threat description
    pub description: String,
    /// Detection method used
    pub detection_method: DetectionMethod,
    /// Additional metadata
    pub metadata: HashMap<String, String>,
    /// Affected assets
    pub affected_assets: Vec<String>,
    /// Indicators of compromise
    pub indicators: Vec<ThreatIndicator>,
    /// Recommended actions
    pub recommended_actions: Vec<ThreatAction>,
    /// Threat intelligence sources
    pub intelligence_sources: Vec<String>,
    /// Related events
    pub related_events: Vec<String>,
    /// Mitigation steps
    pub mitigation_steps: Vec<MitigationStep>,
    
    // Compatibility fields
    /// Evidence (compatibility)
    #[serde(skip_serializing_if = "Vec::is_empty", default)]
    pub evidence: Vec<String>,
    /// Assigned analyst (compatibility)
    #[serde(skip_serializing_if = "Option::is_none")]
    pub assigned_analyst: Option<String>,
    /// Raw data (compatibility)
    #[serde(skip_serializing_if = "Option::is_none")]
    pub raw_data: Option<String>,
    /// Whether mitigated (compatibility)
    #[serde(skip_serializing_if = "std::ops::Not::not", default)]
    pub mitigated: bool,
    /// Mitigation actions (compatibility - alias for recommended_actions)
    #[serde(skip_serializing_if = "Vec::is_empty", default)]
    pub mitigation_actions: Vec<ThreatAction>,
}

/// Source information for threats
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ThreatSource {
    /// IP address of the source
    pub ip_address: Option<String>,
    /// Geographic location
    pub location: Option<String>,
    /// Organization associated with the source
    pub organization: Option<String>,
    /// Source classification
    pub classification: SourceClassification,
    /// Reputation score (0-100)
    pub reputation_score: u8,
    /// Whether source is on blocklist
    pub is_blocklisted: bool,
    /// Previous threat history count
    pub threat_history_count: u32,
    /// Additional source metadata
    pub metadata: HashMap<String, String>,
    
    // Compatibility fields
    /// Source type (compatibility)
    #[serde(skip_serializing_if = "Option::is_none")]
    pub source_type: Option<String>,
    /// Source identifier (compatibility)
    #[serde(skip_serializing_if = "Option::is_none")]
    pub identifier: Option<String>,
    /// Source ID (compatibility)
    #[serde(skip_serializing_if = "Option::is_none")]
    pub id: Option<String>,
    /// Hostname (compatibility)
    #[serde(skip_serializing_if = "Option::is_none")]
    pub hostname: Option<String>,
    /// User agent (compatibility)
    #[serde(skip_serializing_if = "Option::is_none")]
    pub user_agent: Option<String>,
    /// Geolocation (compatibility)
    #[serde(skip_serializing_if = "Option::is_none")]
    pub geolocation: Option<String>,
    /// Threat actor (compatibility)
    #[serde(skip_serializing_if = "Option::is_none")]
    pub threat_actor: Option<String>,
    /// Reputation score as float (compatibility)
    #[serde(skip_serializing_if = "Option::is_none")]
    pub reputation: Option<f64>,
    /// Confidence score (compatibility)
    #[serde(skip_serializing_if = "Option::is_none")]
    pub confidence_score: Option<f64>,
    /// First seen timestamp (compatibility)
    #[serde(skip_serializing_if = "Option::is_none")]
    pub first_seen: Option<std::time::SystemTime>,
    /// Last seen timestamp (compatibility)
    #[serde(skip_serializing_if = "Option::is_none")]
    pub last_seen: Option<std::time::SystemTime>,
}

impl Default for ThreatSource {
    fn default() -> Self {
        Self {
            ip_address: None,
            location: None,
            organization: None,
            classification: SourceClassification::Unknown,
            reputation_score: 50,
            is_blocklisted: false,
            threat_history_count: 0,
            metadata: HashMap::new(),
            // Compatibility fields
            source_type: None,
            identifier: None,
            id: None,
            hostname: None,
            user_agent: None,
            geolocation: None,
            threat_actor: None,
            reputation: None,
            confidence_score: None,
            first_seen: None,
            last_seen: None,
        }
    }
}

/// Target information for threats
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ThreatTarget {
    /// Asset identifier
    pub asset_id: String,
    /// Asset type
    pub asset_type: String,
    /// Asset criticality level
    pub criticality: AssetCriticality,
    /// Current protection level
    pub protection_level: ProtectionLevel,
    /// Asset owner
    pub owner: Option<String>,
    /// Asset location
    pub location: Option<String>,
    /// Asset metadata
    pub metadata: HashMap<String, String>,
    
    // Compatibility fields
    /// Target type (compatibility - alias for asset_type)
    #[serde(skip_serializing_if = "Option::is_none")]
    pub target_type: Option<String>,
    /// Target identifier (compatibility)
    #[serde(skip_serializing_if = "Option::is_none")]
    pub identifier: Option<String>,
    /// Target ID (compatibility)
    #[serde(skip_serializing_if = "Option::is_none")]
    pub id: Option<String>,
    /// Resource ID (compatibility)
    #[serde(skip_serializing_if = "Option::is_none")]
    pub resource_id: Option<String>,
    /// Node ID (compatibility)
    #[serde(skip_serializing_if = "Option::is_none")]
    pub node_id: Option<String>,
    /// User account (compatibility)
    #[serde(skip_serializing_if = "Option::is_none")]
    pub user_account: Option<String>,
    /// Asset criticality (compatibility - alias for criticality)
    #[serde(skip_serializing_if = "Option::is_none")]
    pub asset_criticality: Option<AssetCriticality>,
    /// Service (compatibility)
    #[serde(skip_serializing_if = "Option::is_none")]
    pub service: Option<String>,
    /// Port (compatibility)
    #[serde(skip_serializing_if = "Option::is_none")]
    pub port: Option<u16>,
    /// Protocol (compatibility)
    #[serde(skip_serializing_if = "Option::is_none")]
    pub protocol: Option<String>,
    /// Resource type (compatibility)
    #[serde(skip_serializing_if = "Option::is_none")]
    pub resource_type: Option<String>,
    /// IP address (compatibility)
    #[serde(skip_serializing_if = "Option::is_none")]
    pub ip_address: Option<String>,
    /// Hostname (compatibility)
    #[serde(skip_serializing_if = "Option::is_none")]
    pub hostname: Option<String>,
}

impl Default for ThreatTarget {
    fn default() -> Self {
        Self {
            asset_id: String::new(),
            asset_type: String::new(),
            criticality: AssetCriticality::Medium,
            protection_level: ProtectionLevel::Standard,
            owner: None,
            location: None,
            metadata: HashMap::new(),
            // Compatibility fields
            target_type: None,
            identifier: None,
            id: None,
            resource_id: None,
            node_id: None,
            user_account: None,
            asset_criticality: None,
            service: None,
            port: None,
            protocol: None,
            resource_type: None,
            ip_address: None,
            hostname: None,
        }
    }
}

/// Types of threats that can be detected
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
pub enum ThreatType {
    /// Malware detection
    Malware,
    /// Phishing attempt
    Phishing,
    /// Brute force attack
    BruteForce,
    /// Denial of Service attack
    DenialOfService,
    /// Data exfiltration attempt
    DataExfiltration,
    /// Unauthorized access
    UnauthorizedAccess,
    /// Suspicious network activity
    SuspiciousNetworkActivity,
    /// Anomalous behavior
    AnomalousBehavior,
    /// Policy violation
    PolicyViolation,
    /// Insider threat
    InsiderThreat,
    /// Advanced Persistent Threat
    AdvancedPersistentThreat,
    /// Zero-day exploit
    ZeroDayExploit,
    /// Unknown threat type
    Unknown,
    /// Suspicious activity (compatibility)
    Suspicious,
    /// Generic malicious activity (compatibility)
    Malicious,
    /// Anomaly detected (compatibility)
    Anomaly,
}

/// Severity levels for threats
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq, PartialOrd, Ord)]
pub enum ThreatSeverity {
    /// Low severity threat
    Low,
    /// Medium severity threat
    Medium,
    /// High severity threat
    High,
    /// Critical severity threat
    Critical,
}

impl ThreatSeverity {
    /// Convert severity to numeric score (0-100)
    #[must_use]
    pub const fn to_score(&self) -> u8 {
        match self {
            Self::Low => 25,
            Self::Medium => 50,
            Self::High => 75,
            Self::Critical => 100,
        }
    }

    #[must_use]
    /// Convert numeric score to severity level
    pub const fn from_score(score: u8) -> Self {
        match score {
            0..=30 => Self::Low,
            31..=60 => Self::Medium,
            61..=85 => Self::High,
            86..=100 => Self::Critical,
            _ => Self::Critical, // Fallback for any edge cases
        }
    }
 #[must_use]

    /// Convert to string representation (compatibility)
    pub fn as_str(&self) -> &'static str {
        match self {
            Self::Low => "Low",
            Self::Medium => "Medium",
            Self::High => "High",
            Self::Critical => "Critical",
        }
    }
}

/// Current status of a threat
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
pub enum ThreatStatus {
    /// Threat detected but not yet analyzed
    Detected,
    /// Threat is being analyzed
    Analyzing,
    /// Threat confirmed as valid
    Confirmed,
    /// Threat determined to be false positive
    FalsePositive,
    /// Threat is being mitigated
    Mitigating,
    /// Threat has been mitigated
    Mitigated,
    /// Threat is resolved
    Resolved,
    /// Threat is active (compatibility)
    Active,
}

/// Method used to detect the threat
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
pub enum DetectionMethod {
    /// Signature-based detection
    SignatureBased,
    /// Behavioral analysis
    BehavioralAnalysis,
    /// Machine learning model
    MachineLearning,
    /// Anomaly detection
    AnomalyDetection,
    /// Threat intelligence
    ThreatIntelligence,
    /// User report
    UserReport,
    /// Manual analysis
    ManualAnalysis,
}

/// Classification of threat sources
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
pub enum SourceClassification {
    /// Known malicious source
    Malicious,
    /// Suspicious source
    Suspicious,
    /// Legitimate source
    Legitimate,
    /// Unknown source
    Unknown,
    /// Trusted source (compatibility)
    Trusted,
    /// Known malicious source (compatibility alias)
    KnownMalicious,
}

/// Criticality level of assets
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq, PartialOrd, Ord)]
pub enum AssetCriticality {
    /// Low criticality asset
    Low,
    /// Medium criticality asset
    Medium,
    /// High criticality asset
    High,
    /// Critical asset
    Critical,
}

/// Protection level of assets
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

/// Actions that can be taken in response to threats
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
pub enum ThreatAction {
    /// Block the threat source
    Block,
    /// Quarantine affected assets
    Quarantine,
    /// Alert administrators
    Alert,
    /// Log the event
    Log,
    /// Analyze further
    Analyze,
    /// Ignore the threat
    Ignore,
    /// Escalate to human analyst
    Escalate,
    /// Block the source (compatibility)
    BlockSource,
    /// Alert security team (compatibility)
    AlertSecurityTeam,
    /// Investigate activity (compatibility)
    InvestigateActivity,
}

/// Threat indicator information
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ThreatIndicator {
    /// Type of indicator
    pub indicator_type: IndicatorType,
    /// Indicator value
    pub value: String,
    /// Confidence in the indicator (0.0 - 1.0)
    pub confidence: f64,
    /// Source of the indicator
    pub source: String,
    /// When the indicator was first seen
    pub first_seen: SystemTime,
    /// When the indicator was last seen
    pub last_seen: SystemTime,
}

/// Types of threat indicators
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
pub enum IndicatorType {
    /// IP address
    IpAddress,
    /// Domain name
    Domain,
    /// URL
    Url,
    /// File hash
    FileHash,
    /// Email address
    Email,
    /// Registry key
    RegistryKey,
    /// Process name
    Process,
    /// Network signature
    NetworkSignature,
    /// Domain name (compatibility)
    DomainName,
    /// Email address (compatibility)
    EmailAddress,
    /// User agent (compatibility)
    UserAgent,
    /// Process name (compatibility)
    ProcessName,
}

impl std::fmt::Display for IndicatorType {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::IpAddress => write!(f, "IP Address"),
            Self::Domain => write!(f, "Domain"),
            Self::Url => write!(f, "URL"),
            Self::FileHash => write!(f, "File Hash"),
            Self::Email => write!(f, "Email"),
            Self::RegistryKey => write!(f, "Registry Key"),
            Self::Process => write!(f, "Process"),
            Self::NetworkSignature => write!(f, "Network Signature"),
            Self::DomainName => write!(f, "Domain Name"),
            Self::EmailAddress => write!(f, "Email Address"),
            Self::UserAgent => write!(f, "User Agent"),
            Self::ProcessName => write!(f, "Process Name"),
        }
    }
}

/// Mitigation step for threat response
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct MitigationStep {
    /// Step identifier
    pub id: String,
    /// Step description
    pub description: String,
    /// Priority of the step (1-10)
    pub priority: u8,
    /// Whether the step is automated
    pub is_automated: bool,
    /// Estimated time to complete (in minutes)
    pub estimated_time_minutes: u32,
    /// Required permissions
    pub required_permissions: Vec<String>,
} 