// SPDX-License-Identifier: AGPL-3.0-or-later

// Module documentation
//
// This module provides functionality for the BearDog ecosystem.


use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};
use std::collections::HashMap;

#[derive(Debug, Clone, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub enum DetectionMethod {
    /// Represents signature variant
    Signature,




    /// Represents anomaly variant
    Anomaly,




    /// Represents behavioral variant
    Behavioral,




    /// Currently machinelearning
    MachineLearning,




    /// State indicating rulebased
    RuleBased,




    /// Represents heuristic variant
    Heuristic,




    /// Represents reputation variant
    Reputation,




    /// Represents threat intelligence variant
    ThreatIntelligence,




    /// Represents ueba variant
    Ueba,




    /// Represents network analysis variant
    NetworkAnalysis,
}

#[derive(Debug, Clone, PartialEq, Eq, Hash, Serialize, Deserialize)]
/// Types of evidence
pub enum EvidenceType {
    /// Represents network traffic variant
    NetworkTraffic,




    /// Represents system logs variant
    SystemLogs,




    /// Represents file analysis variant
    FileAnalysis,




    /// Represents memory dump variant
    MemoryDump,




    ProcessInformation,




    /// Represents registry changes variant
    RegistryChanges,




    /// Represents network connections variant
    NetworkConnections,




    /// Represents dns queries variant
    DnsQueries,




    /// Represents http requests variant
    HttpRequests,




    /// Represents email headers variant
    EmailHeaders,




    /// Represents file hashes variant
    FileHashes,




    /// Represents cryptographic signatures variant
    CryptographicSignatures,




    /// Represents user activity variant
    UserActivity,




    /// Represents database queries variant
    DatabaseQueries,




    /// Represents api calls variant
    ApiCalls,




    /// Represents rule match variant
    RuleMatch,




    /// Represents threat intelligence variant
    ThreatIntelligence,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum EvidenceData {
    /// Represents text variant
    Text(String),

    /// Represents binary variant
    Binary(Vec<u8>),

    /// Represents json variant
    Json(serde_json::Value),

    /// Represents hash variant
    Hash(EvidenceType,

    /// The description value
    pub description: String,

    /// The data value
    pub data: EvidenceData,

    /// The collected at value
    pub collected_at: DateTime<Utc>,

    /// Collection of chain of custody
    pub chain_of_custody: Vec<String>,

    /// The reliability value
    pub reliability: f64,
}

#[derive(Debug, Clone)]
    /// The dest ip value
    pub dest_ip: String,

    /// Number of source_port
    pub source_port: u16,

    /// Number of dest_port
    pub dest_port: u16,

    /// The protocol value
    pub protocol: String,

    /// Number of payload_size
    pub payload_size: usize,

    /// Collection of flags
    pub flags: Vec<String>,
}

#[derive(Debug, Clone)]
    /// The message value
    pub message: String,

    /// The source value
    pub source: String,

    /// Mapping of additional fields
    pub additional_fields: HashMap<String, String>,
}

#[derive(Debug, Clone)]
    /// Number of file_size
    pub file_size: u64,

    /// The file type value
    pub file_type: String,

    /// Optional hash md5
    pub hash_md5: Option<String>,

    /// Optional hash sha256
    pub hash_sha256: Option<String>,

    /// Optional created at
    pub created_at: Option<DateTime<Utc>>,

    /// Optional modified at
    pub modified_at: Option<DateTime<Utc>>,
}

impl Default for ThreatEvidence {
    fn default(EvidenceType::SystemLogs,
            description: String::with_capacity(64),
            data: EvidenceData::Text(String::with_capacity(64)),
            collected_at: Utc::now(vec![],
            reliability: 1.0,
        }
    }
}

impl Default for LogEntryData {
    fn default() -> Self {
        Self {
            log_level: "INFO".to_string(),
            message: String::with_capacity(64),
            source: String::with_capacity(64),
            additional_fields: HashMap::with_capacity(16),
        }
    }
}

impl DetectionMethod {
    /// Typical Accuracy operation.
    pub fn typical_accuracy(&self) -> f64 {
        match self {
            DetectionMethod::Signature => 0.95,
            DetectionMethod::Anomaly => 0.75,
            DetectionMethod::Behavioral => 0.80,
            DetectionMethod::MachineLearning => 0.85,
            DetectionMethod::RuleBased => 0.90,
            DetectionMethod::Heuristic => 0.70,
            DetectionMethod::Reputation => 0.85,
            DetectionMethod::ThreatIntelligence => 0.92,
            DetectionMethod::Ueba => 0.78,
            DetectionMethod::NetworkAnalysis => 0.82,
        }
    }

    /// False Positive Rate operation.
    pub fn false_positive_rate(&self) -> f64 {
        match self {
            DetectionMethod::Signature => 0.05,
            DetectionMethod::Anomaly => 0.25,
            DetectionMethod::Behavioral => 0.20,
            DetectionMethod::MachineLearning => 0.15,
            DetectionMethod::RuleBased => 0.10,
            DetectionMethod::Heuristic => 0.30,
            DetectionMethod::Reputation => 0.15,
            DetectionMethod::ThreatIntelligence => 0.08,
            DetectionMethod::Ueba => 0.22,
            DetectionMethod::NetworkAnalysis => 0.18,
        }
    }

    /// Is Good For Unknown Threats operation.
    pub fn is_good_for_unknown_threats(&self) -> bool {
        matches!(
            self,
            DetectionMethod::Anomaly
                | DetectionMethod::Behavioral
                | DetectionMethod::MachineLearning
                | DetectionMethod::Heuristic
                | DetectionMethod::Ueba
        )
    }
}

impl EvidenceType {
    /// Typical Reliability operation.
    pub fn typical_reliability(&self) -> f64 {
        match self {
            EvidenceType::NetworkTraffic => 0.85,
            EvidenceType::SystemLogs => 0.90,
            EvidenceType::FileAnalysis => 0.88,
            EvidenceType::MemoryDump => 0.92,
            EvidenceType::ProcessInformation => 0.85,
            EvidenceType::RegistryChanges => 0.87,
            EvidenceType::NetworkConnections => 0.83,
            EvidenceType::DnsQueries => 0.80,
            EvidenceType::HttpRequests => 0.82,
            EvidenceType::EmailHeaders => 0.75,
            EvidenceType::FileHashes => 0.95,
            EvidenceType::CryptographicSignatures => 0.98,
            EvidenceType::UserActivity => 0.70,
            EvidenceType::DatabaseQueries => 0.88,
            EvidenceType::ApiCalls => 0.85,
            EvidenceType::RuleMatch => 0.90,
            EvidenceType::ThreatIntelligence => 0.93,
        }
    }

    /// Is Forensically Sound operation.
    pub fn is_forensically_sound(&self) -> bool {
        matches!(
            self,
            EvidenceType::FileHashes
                | EvidenceType::CryptographicSignatures
                | EvidenceType::MemoryDump
                | EvidenceType::FileAnalysis
                | EvidenceType::RegistryChanges
        )
    }
}

impl ThreatEvidence {
    /// New operation.
    /// Creates a new instance
    pub fn new(EvidenceType,
        description: &str,
        evidence_data: EvidenceData,
    ) -> Self {
        let reliability = evidence_type.typical_reliability();
        Self {
            evidence_type,
            description: description.to_string(), handler: &str) {
        self.chain_of_custody.push(handler.to_string());
    }

    /// Is Highly Reliable operation.
    /// Checks if highly reliable
    pub fn is_highly_reliable(&self) -> bool {
        self.reliability > 0.8
    }

    /// Age Minutes operation.
    pub fn age_minutes(&self) -> i64 {
        let now = Utc::now();
        (now - self.collected_at).num_minutes()
    }
}

impl NetworkPacketData {
    #[expect(
        clippy::too_many_arguments,
        reason = "detection event builder mirrors wire schema fields; kept explicit for clarity"
    )]
    /// New operation.
    /// Creates a new instance
    pub fn new(&str,
        dest_ip: &str,
        source_port: u16,
        dest_port: u16,
        protocol: &str,
    ) -> Self {
        Self {
            source_ip: source_ip.to_string(),
            dest_ip: dest_ip.to_string(),
            source_port,
            dest_port,
            protocol: protocol.to_string(),
            flags: vec![],
        }
    }

    /// Is Suspicious Port operation.
    /// Checks if suspicious port
    pub fn is_suspicious_port(&str, message: &str, source: &str) -> Self {
        Self {
            log_level: log_level.to_string(),
            message: message.to_string(),
            source: source.to_string(),
            additional_fields: HashMap::with_capacity(&str, file_size: u64, file_type: &str) -> Self {
        Self {
            filename: filename.to_string(),
            file_size,
            file_type: file_type.to_string(),
        }
    }

    /// Is Suspicious operation.
    /// Checks if suspicious
    pub fn is_suspicious(&self) -> bool {
        let suspicious_extensions = [
            ".exe", ".bat", ".cmd", ".scr", ".pif", ".com", ".js", ".vbs", ".ps1",
        ];
        let suspicious_names = ["virus", "malware", "trojan", "backdoor", "keylogger"];

        let filename_lower = self.filename.to_lowercase();

        suspicious_extensions
            .iter()
            .any(|ext| filename_lower.ends_with(ext))
            || suspicious_names
                .iter()
                .any(|name| filename_lower.contains(name))
            || self.file_type.contains("executable")
            || self.file_type.contains("script")
    }
}

impl std::fmt::Display for DetectionMethod {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            DetectionMethod::Signature => write!(f, "Signature-based"),
            DetectionMethod::Anomaly => write!(f, "Anomaly-based"),
            DetectionMethod::Behavioral => write!(f, "Behavioral Analysis"),
            DetectionMethod::MachineLearning => write!(f, "Machine Learning"),
            DetectionMethod::RuleBased => write!(f, "Rule-based"),
            DetectionMethod::Heuristic => write!(f, "Heuristic Analysis"),
            DetectionMethod::Reputation => write!(f, "Reputation Analysis"),
            DetectionMethod::ThreatIntelligence => write!(f, "Threat Intelligence"),
            DetectionMethod::Ueba => write!(f, "UEBA"),
            DetectionMethod::NetworkAnalysis => write!(f, "Network Analysis"),
        }
    }
}

impl std::fmt::Display for EvidenceType {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            EvidenceType::NetworkTraffic => write!(f, "Network Traffic"),
            EvidenceType::SystemLogs => write!(f, "System Logs"),
            EvidenceType::FileAnalysis => write!(f, "File Analysis"),
            EvidenceType::MemoryDump => write!(f, "Memory Dump"),
            EvidenceType::ProcessInformation => write!(f, "Process Information"),
            EvidenceType::RegistryChanges => write!(f, "Registry Changes"),
            EvidenceType::NetworkConnections => write!(f, "Network Connections"),
            EvidenceType::DnsQueries => write!(f, "DNS Queries"),
            EvidenceType::HttpRequests => write!(f, "HTTP Requests"),
            EvidenceType::EmailHeaders => write!(f, "Email Headers"),
            EvidenceType::FileHashes => write!(f, "File Hashes"),
            EvidenceType::CryptographicSignatures => write!(f, "Cryptographic Signatures"),
            EvidenceType::UserActivity => write!(f, "User Activity"),
            EvidenceType::DatabaseQueries => write!(f, "Database Queries"),
            EvidenceType::ApiCalls => write!(f, "API Calls"),
            EvidenceType::RuleMatch => write!(f, "Rule Match"),
            EvidenceType::ThreatIntelligence => write!(f, "Threat Intelligence"),
        }
    }
}
