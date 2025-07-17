//! Core threat types and definitions
//!
//! This module contains the fundamental types for threat detection and classification.
//!
//! ## Features
//! - Threat severity levels with scoring
//! - Comprehensive threat type definitions
//! - Threat event structure and metadata
//! - Utility functions for threat analysis
//! - Display implementations for human-readable output
//!
//! ## Example
//! ```rust
//! use beardog::threat::types::{ThreatSeverity, ThreatType, ThreatEvent};
//! use chrono::Utc;
//!
//! let severity = ThreatSeverity::High;
//! let threat_type = ThreatType::Malware;
//!
//! // Create a threat event
//! let event = ThreatEvent {
//!     id: "threat-001".to_string(),
//!     threat_type,
//!     severity,
//!     score: severity.to_score(),
//!     timestamp: Utc::now(),
//!     // ... other fields
//! };
//!
//! assert!(event.is_high_priority());
//! ```

use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};

use super::actions::{MitigationStep, ThreatAction, ThreatStatus};
use super::detection::{DetectionMethod, ThreatEvidence};
use super::sources::{ThreatSource, ThreatTarget};

/// Threat severity levels
///
/// Defines the severity levels for threats, from informational to critical.
/// Each level has an associated numeric score for automated processing.
#[derive(Debug, Clone, PartialEq, Eq, Hash, PartialOrd, Ord, Serialize, Deserialize, Default)]
pub enum ThreatSeverity {
    /// Informational - no immediate action required
    ///
    /// Used for events that provide information about system state
    /// but don't represent actual threats.
    Info,

    /// Low severity - monitor activity
    ///
    /// Minor threats that should be monitored but don't require
    /// immediate intervention.
    Low,

    /// Medium severity - investigate promptly
    ///
    /// Moderate threats that require investigation within
    /// a reasonable timeframe.
    #[default]
    Medium,

    /// High severity - immediate investigation required
    ///
    /// Serious threats that require immediate attention
    /// and investigation.
    High,

    /// Critical severity - emergency response needed
    ///
    /// Severe threats that require immediate emergency
    /// response and containment.
    Critical,
}

/// Comprehensive threat type enumeration
///
/// Covers all major categories of security threats that the system
/// can detect and respond to.
#[derive(Debug, Clone, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub enum ThreatType {
    /// Malware detection
    Malware,
    /// Malicious activity (generic)
    Malicious,
    /// Phishing attempt
    Phishing,
    /// Brute force attack
    BruteForce,
    /// Brute force attack variant
    BruteForceAttack,
    /// Suspicious login activity
    SuspiciousLogin,
    /// DDoS attack
    DdosAttack,
    /// Data exfiltration
    DataExfiltration,
    /// Privilege escalation
    PrivilegeEscalation,
    /// Insider threat
    InsiderThreat,
    /// Network intrusion
    NetworkIntrusion,
    /// SQL injection
    SqlInjection,
    /// Cross-site scripting
    Xss,
    /// Man-in-the-middle
    ManInTheMiddle,
    /// Ransomware
    Ransomware,
    /// Botnet activity
    BotnetActivity,
    /// Zero-day exploit
    ZeroDayExploit,
    /// Advanced persistent threat
    Apt,
    /// Social engineering
    SocialEngineering,
    /// Supply chain attack
    SupplyChainAttack,
    /// AI/ML poisoning
    AiPoisoning,
    /// Quantum threat
    QuantumThreat,
    /// Anomaly detection
    Anomaly,
    /// Unknown threat pattern
    Unknown,
}

/// Comprehensive threat event structure
///
/// Represents a complete threat event with all associated metadata,
/// evidence, and response information.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ThreatEvent {
    /// Unique event identifier
    pub id: String,

    /// Type of threat detected
    pub threat_type: ThreatType,

    /// Severity level
    pub severity: ThreatSeverity,

    /// Threat score (0-100)
    pub score: u8,

    /// Event timestamp
    pub timestamp: DateTime<Utc>,

    /// Source of the threat
    pub source: ThreatSource,

    /// Target information
    pub target: ThreatTarget,

    /// Event description
    pub description: String,

    /// Detection method used
    pub detection_method: DetectionMethod,

    /// Evidence collected
    pub evidence: Vec<ThreatEvidence>,

    /// Recommended actions
    pub recommended_actions: Vec<ThreatAction>,

    /// Current status
    pub status: ThreatStatus,

    /// Assigned analyst
    pub assigned_analyst: Option<String>,

    /// Related events
    pub related_events: Vec<String>,

    /// Mitigation steps taken
    pub mitigation_steps: Vec<MitigationStep>,
}

// Implementations

impl ThreatSeverity {
    /// Convert severity to numeric score
    ///
    /// Provides a numeric representation of threat severity for
    /// automated processing and comparison.
    ///
    /// # Returns
    /// Numeric score from 10 (Info) to 100 (Critical)
    ///
    /// # Example
    /// ```rust
    /// use beardog::threat::types::ThreatSeverity;
    ///
    /// assert_eq!(ThreatSeverity::Critical.to_score(), 100);
    /// assert_eq!(ThreatSeverity::High.to_score(), 80);
    /// assert_eq!(ThreatSeverity::Medium.to_score(), 50);
    /// ```
    pub fn to_score(&self) -> u8 {
        match self {
            ThreatSeverity::Info => 10,
            ThreatSeverity::Low => 30,
            ThreatSeverity::Medium => 50,
            ThreatSeverity::High => 80,
            ThreatSeverity::Critical => 100,
        }
    }

    /// Create severity from score
    ///
    /// Converts a numeric score back to a severity level.
    ///
    /// # Arguments
    /// * `score` - Numeric score (0-100)
    ///
    /// # Returns
    /// Corresponding `ThreatSeverity` level
    ///
    /// # Example
    /// ```rust
    /// use beardog::threat::types::ThreatSeverity;
    ///
    /// assert_eq!(ThreatSeverity::from_score(90), ThreatSeverity::Critical);
    /// assert_eq!(ThreatSeverity::from_score(45), ThreatSeverity::Medium);
    /// ```
    pub fn from_score(score: u8) -> Self {
        match score {
            0..=20 => ThreatSeverity::Info,
            21..=40 => ThreatSeverity::Low,
            41..=60 => ThreatSeverity::Medium,
            61..=80 => ThreatSeverity::High,
            81..=100 => ThreatSeverity::Critical,
            _ => ThreatSeverity::Medium,
        }
    }

    /// Check if severity requires immediate attention
    ///
    /// # Returns
    /// `true` if severity is High or Critical
    ///
    /// # Example
    /// ```rust
    /// use beardog::threat::types::ThreatSeverity;
    ///
    /// assert!(ThreatSeverity::Critical.is_urgent());
    /// assert!(ThreatSeverity::High.is_urgent());
    /// assert!(!ThreatSeverity::Medium.is_urgent());
    /// ```
    pub fn is_urgent(&self) -> bool {
        matches!(self, ThreatSeverity::High | ThreatSeverity::Critical)
    }
}

impl ThreatType {
    /// Get the typical severity for this threat type
    ///
    /// Provides default severity levels for different threat types
    /// based on their typical impact and urgency.
    ///
    /// # Returns
    /// Typical `ThreatSeverity` for this threat type
    ///
    /// # Example
    /// ```rust
    /// use beardog::threat::types::{ThreatType, ThreatSeverity};
    ///
    /// assert_eq!(ThreatType::Ransomware.typical_severity(), ThreatSeverity::Critical);
    /// assert_eq!(ThreatType::Phishing.typical_severity(), ThreatSeverity::Medium);
    /// ```
    pub fn typical_severity(&self) -> ThreatSeverity {
        match self {
            ThreatType::Phishing | ThreatType::SocialEngineering => ThreatSeverity::Medium,
            ThreatType::BruteForce
            | ThreatType::BruteForceAttack
            | ThreatType::SuspiciousLogin
            | ThreatType::SqlInjection
            | ThreatType::Xss => ThreatSeverity::High,
            ThreatType::Malware
            | ThreatType::Malicious
            | ThreatType::Ransomware
            | ThreatType::ZeroDayExploit => ThreatSeverity::Critical,
            ThreatType::DdosAttack | ThreatType::NetworkIntrusion => ThreatSeverity::High,
            ThreatType::DataExfiltration | ThreatType::PrivilegeEscalation => {
                ThreatSeverity::Critical
            }
            ThreatType::InsiderThreat | ThreatType::Apt => ThreatSeverity::Critical,
            ThreatType::ManInTheMiddle | ThreatType::BotnetActivity => ThreatSeverity::High,
            ThreatType::SupplyChainAttack | ThreatType::AiPoisoning => ThreatSeverity::Critical,
            ThreatType::QuantumThreat => ThreatSeverity::Critical,
            ThreatType::Anomaly => ThreatSeverity::Medium,
            ThreatType::Unknown => ThreatSeverity::Medium,
        }
    }

    /// Get MITRE ATT&CK techniques associated with this threat type
    ///
    /// # Returns
    /// Vector of MITRE ATT&CK technique IDs
    ///
    /// # Example
    /// ```rust
    /// use beardog::threat::types::ThreatType;
    ///
    /// let techniques = ThreatType::Phishing.mitre_techniques();
    /// assert!(techniques.contains(&"T1566".to_string()));
    /// ```
    pub fn mitre_techniques(&self) -> Vec<String> {
        match self {
            ThreatType::Phishing => vec!["T1566".to_string()],
            ThreatType::BruteForce | ThreatType::BruteForceAttack => vec!["T1110".to_string()],
            ThreatType::SqlInjection => vec!["T1190".to_string()],
            ThreatType::Ransomware => vec!["T1486".to_string(), "T1490".to_string()],
            ThreatType::PrivilegeEscalation => vec!["T1068".to_string(), "T1055".to_string()],
            ThreatType::DataExfiltration => vec!["T1041".to_string(), "T1048".to_string()],
            _ => vec![],
        }
    }
}

impl ThreatEvent {
    /// Check if the threat is considered high priority
    ///
    /// Determines if a threat event requires immediate attention
    /// based on severity and score.
    ///
    /// # Returns
    /// `true` if the threat is high priority
    ///
    /// # Example
    /// ```rust
    /// use beardog::threat::types::{ThreatEvent, ThreatSeverity};
    ///
    /// let event = ThreatEvent {
    ///     severity: ThreatSeverity::High,
    ///     score: 85,
    ///     // ... other fields
    /// };
    ///
    /// assert!(event.is_high_priority());
    /// ```
    pub fn is_high_priority(&self) -> bool {
        self.severity >= ThreatSeverity::High || self.score >= 80
    }

    /// Calculate age in minutes
    ///
    /// Calculates how long ago this threat event occurred.
    ///
    /// # Returns
    /// Age in minutes since the threat event timestamp
    ///
    /// # Example
    /// ```rust
    /// use beardog::threat::types::ThreatEvent;
    /// use chrono::{Utc, Duration};
    ///
    /// let event = ThreatEvent {
    ///     timestamp: Utc::now() - Duration::minutes(30),
    ///     // ... other fields
    /// };
    ///
    /// assert_eq!(event.age_minutes(), 30);
    /// ```
    pub fn age_minutes(&self) -> i64 {
        let now = Utc::now();
        (now - self.timestamp).num_minutes()
    }

    /// Check if the threat is stale
    ///
    /// Determines if a threat event is too old to be relevant.
    ///
    /// # Arguments
    /// * `max_age_minutes` - Maximum age in minutes before considering stale
    ///
    /// # Returns
    /// `true` if the threat is stale
    ///
    /// # Example
    /// ```rust
    /// use beardog::threat::types::ThreatEvent;
    /// use chrono::{Utc, Duration};
    ///
    /// let event = ThreatEvent {
    ///     timestamp: Utc::now() - Duration::hours(2),
    ///     // ... other fields
    /// };
    ///
    /// assert!(event.is_stale(60)); // 1 hour threshold
    /// ```
    pub fn is_stale(&self, max_age_minutes: i64) -> bool {
        self.age_minutes() > max_age_minutes
    }

    /// Get threat urgency score
    ///
    /// Calculates a composite urgency score based on severity,
    /// threat score, and age.
    ///
    /// # Returns
    /// Urgency score (0.0 to 1.0)
    ///
    /// # Example
    /// ```rust
    /// use beardog::threat::types::ThreatEvent;
    ///
    /// let event = ThreatEvent {
    ///     severity: ThreatSeverity::Critical,
    ///     score: 95,
    ///     // ... other fields
    /// };
    ///
    /// assert!(event.urgency_score() > 0.8);
    /// ```
    pub fn urgency_score(&self) -> f64 {
        let severity_weight = self.severity.to_score() as f64 / 100.0;
        let score_weight = self.score as f64 / 100.0;
        let age_weight = if self.age_minutes() < 60 { 1.0 } else { 0.5 };

        (severity_weight * 0.4 + score_weight * 0.4 + age_weight * 0.2).min(1.0)
    }
}

// Display implementations

impl std::fmt::Display for ThreatSeverity {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            ThreatSeverity::Info => write!(f, "INFO"),
            ThreatSeverity::Low => write!(f, "LOW"),
            ThreatSeverity::Medium => write!(f, "MEDIUM"),
            ThreatSeverity::High => write!(f, "HIGH"),
            ThreatSeverity::Critical => write!(f, "CRITICAL"),
        }
    }
}

impl std::fmt::Display for ThreatType {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let display_name = match self {
            ThreatType::Malware => "Malware",
            ThreatType::Malicious => "Malicious Activity",
            ThreatType::Phishing => "Phishing",
            ThreatType::BruteForce => "Brute Force",
            ThreatType::BruteForceAttack => "Brute Force Attack",
            ThreatType::SuspiciousLogin => "Suspicious Login",
            ThreatType::DdosAttack => "DDoS Attack",
            ThreatType::DataExfiltration => "Data Exfiltration",
            ThreatType::PrivilegeEscalation => "Privilege Escalation",
            ThreatType::InsiderThreat => "Insider Threat",
            ThreatType::NetworkIntrusion => "Network Intrusion",
            ThreatType::SqlInjection => "SQL Injection",
            ThreatType::Xss => "Cross-Site Scripting",
            ThreatType::ManInTheMiddle => "Man in the Middle",
            ThreatType::Ransomware => "Ransomware",
            ThreatType::BotnetActivity => "Botnet Activity",
            ThreatType::ZeroDayExploit => "Zero-Day Exploit",
            ThreatType::Apt => "Advanced Persistent Threat",
            ThreatType::SocialEngineering => "Social Engineering",
            ThreatType::SupplyChainAttack => "Supply Chain Attack",
            ThreatType::AiPoisoning => "AI/ML Poisoning",
            ThreatType::QuantumThreat => "Quantum Threat",
            ThreatType::Anomaly => "Anomaly",
            ThreatType::Unknown => "Unknown",
        };
        write!(f, "{display_name}")
    }
}
