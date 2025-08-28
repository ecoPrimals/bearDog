use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};

use super::actions::{MitigationStep, ThreatAction, ThreatStatus};
use super::detection::{DetectionMethod, ThreatEvidence};
use super::sources::{ThreatSource, ThreatTarget};

#[derive(Debug, Clone, PartialEq, Eq, Hash, PartialOrd, Ord, Serialize, Deserialize, Default)]
pub enum ThreatSeverity {
    Info,

    Low,

    #[default]
    Medium,

    High,

    Critical,
}

#[derive(Debug, Clone, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub enum ThreatType {
    Malware,

    Malicious,

    Phishing,

    BruteForce,

    BruteForceAttack,

    SuspiciousLogin,

    DdosAttack,

    DataExfiltration,

    PrivilegeEscalation,

    InsiderThreat,

    NetworkIntrusion,

    SqlInjection,

    Xss,

    ManInTheMiddle,

    Ransomware,

    BotnetActivity,

    ZeroDayExploit,

    Apt,

    SocialEngineering,

    SupplyChainAttack,

    AiPoisoning,

    QuantumThreat,

    Anomaly,

    AuthenticationFailure, // Add missing variant

    Unknown,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ThreatEvent {
    pub id: String,
    pub threat_type: ThreatType,
    pub severity: ThreatSeverity,
    pub score: u8,
    pub timestamp: DateTime<Utc>,
    pub source: ThreatSource,
    pub target: ThreatTarget,
    pub description: String,
    pub detection_method: DetectionMethod,
    pub evidence: Vec<ThreatEvidence>,
    pub recommended_actions: Vec<ThreatAction>,
    pub status: ThreatStatus,
    pub assigned_analyst: Option<String>,
    pub related_events: Vec<String>,
    pub mitigation_steps: Vec<MitigationStep>,
    // Add missing fields referenced in handlers
    pub confidence: f64,
    pub raw_data: Option<String>,
    pub mitigated: bool,
    pub mitigation_actions: Vec<String>,
}

impl ThreatSeverity {
    pub fn to_score(&self) -> u8 {
        match self {
            ThreatSeverity::Info => 10,
            ThreatSeverity::Low => 30,
            ThreatSeverity::Medium => 50,
            ThreatSeverity::High => 80,
            ThreatSeverity::Critical => 100,
        }
    }

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

    pub fn is_urgent(&self) -> bool {
        matches!(self, ThreatSeverity::High | ThreatSeverity::Critical)
    }
}

impl ThreatType {
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
            ThreatType::AuthenticationFailure => ThreatSeverity::Medium,
            ThreatType::Unknown => ThreatSeverity::Medium,
        }
    }

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

impl Default for ThreatEvent {
    fn default() -> Self {
        Self {
            id: String::new(),
            threat_type: ThreatType::Unknown,
            severity: ThreatSeverity::Medium,
            score: 50,
            timestamp: Utc::now(),
            source: ThreatSource::default(),
            target: ThreatTarget::default(),
            description: String::new(),
            detection_method: DetectionMethod::Signature,
            evidence: Vec::new(),
            recommended_actions: Vec::new(),
            status: ThreatStatus::New,
            assigned_analyst: None,
            related_events: Vec::new(),
            mitigation_steps: Vec::new(),
            confidence: 0.5,
            raw_data: None,
            mitigated: false,
            mitigation_actions: Vec::new(),
        }
    }
}

impl ThreatEvent {
    pub fn is_high_priority(&self) -> bool {
        self.severity >= ThreatSeverity::High || self.score >= 80
    }

    pub fn age_minutes(&self) -> i64 {
        let now = Utc::now();
        (now - self.timestamp).num_minutes()
    }

    pub fn is_stale(&self, max_age_minutes: i64) -> bool {
        self.age_minutes() > max_age_minutes
    }

    pub fn urgency_score(&self) -> f64 {
        let severity_weight = self.severity.to_score() as f64 / 100.0;
        let score_weight = self.score as f64 / 100.0;
        let age_weight = if self.age_minutes() < 60 { 1.0 } else { 0.5 };

        (severity_weight * 0.4 + score_weight * 0.4 + age_weight * 0.2).min(1.0)
    }
}

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
            ThreatType::AuthenticationFailure => "Authentication Failure",
            ThreatType::Unknown => "Unknown",
        };
        write!(f, "{display_name}")
    }
}
