// SPDX-License-Identifier: AGPL-3.0-or-later

//! Core threat taxonomy: categories, severity, status, detection methods, and indicator shapes.

use serde::{Deserialize, Serialize};

/// High-level category assigned to a detected or suspected threat.
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
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

/// Ordinal severity used for alerting thresholds, SLAs, and automated response policy.
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

    /// Maps severity to a small integer rank for sorting and composite scoring.
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

/// Expected defensive posture for an asset when evaluating impact.
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
    /// Record the event for later hunting without immediate containment.
    LogForAnalysis,
}

/// How a detection rule derives its decision (signature, ML, heuristic, etc.).
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
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

/// Shape of a threat-indicator value for feed normalization and matching.
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
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
