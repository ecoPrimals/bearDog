

use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};
use std::collections::HashMap;

#[derive(Debug, Clone, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub enum DetectionMethod {

    Signature,

    Anomaly,

    Behavioral,

    MachineLearning,

    RuleBased,

    Heuristic,

    Reputation,

    ThreatIntelligence,

    Ueba,

    NetworkAnalysis,
}

#[derive(Debug, Clone, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub enum EvidenceType {

    NetworkTraffic,

    SystemLogs,

    FileAnalysis,

    MemoryDump,

    ProcessInformation,

    RegistryChanges,

    NetworkConnections,

    DnsQueries,

    HttpRequests,

    EmailHeaders,

    FileHashes,

    CryptographicSignatures,

    UserActivity,

    DatabaseQueries,

    ApiCalls,

    RuleMatch,

    ThreatIntelligence,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum EvidenceData {

    Text(String),

    Binary(Vec<u8>),

    Json(serde_json::Value),

    Hash(String),

    NetworkPacket(NetworkPacketData),

    LogEntry(LogEntryData),

    FileMetadata(FileMetadataData),
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ThreatEvidence {

    pub evidence_type: EvidenceType,

    pub description: String,

    pub data: EvidenceData,

    pub collected_at: DateTime<Utc>,

    pub chain_of_custody: Vec<String>,

    pub reliability: f64,
}

#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct NetworkPacketData {

    pub source_ip: String,

    pub dest_ip: String,

    pub source_port: u16,

    pub dest_port: u16,

    pub protocol: String,

    pub payload_size: usize,

    pub flags: Vec<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct LogEntryData {

    pub log_level: String,

    pub message: String,

    pub source: String,

    pub additional_fields: HashMap<String, String>,
}

#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct FileMetadataData {

    pub filename: String,

    pub file_size: u64,

    pub file_type: String,

    pub hash_md5: Option<String>,

    pub hash_sha256: Option<String>,

    pub created_at: Option<DateTime<Utc>>,

    pub modified_at: Option<DateTime<Utc>>,
}

impl Default for ThreatEvidence {
    fn default() -> Self {
        Self {
            evidence_type: EvidenceType::SystemLogs,
            description: String::with_capacity(64),
            data: EvidenceData::Text(String::with_capacity(64)),
            collected_at: Utc::now(),
            chain_of_custody: vec![],
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

    pub fn new(evidence_type: EvidenceType, description: &str, data: EvidenceData) -> Self {
        let reliability = evidence_type.typical_reliability();
        Self {
            evidence_type,
            description: description.to_string(),
            data,
            collected_at: Utc::now(),
            chain_of_custody: vec![],
            reliability,
        }
    }

    pub fn add_custody_handler(&mut self, handler: &str) {
        self.chain_of_custody.push(handler.to_string());
    }

    pub fn is_highly_reliable(&self) -> bool {
        self.reliability > 0.8
    }

    pub fn age_minutes(&self) -> i64 {
        let now = Utc::now();
        (now - self.collected_at).num_minutes()
    }
}

impl NetworkPacketData {

    pub fn new(
        source_ip: &str,
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
            payload_size: 0,
            flags: vec![],
        }
    }

    pub fn is_suspicious_port(&self) -> bool {
        let suspicious_ports = [1337, 31337, 4444, 5555, 6666, 7777, 8888, 9999];
        suspicious_ports.contains(&self.dest_port) || suspicious_ports.contains(&self.source_port)
    }
}

impl LogEntryData {

    pub fn new(log_level: &str, message: &str, source: &str) -> Self {
        Self {
            log_level: log_level.to_string(),
            message: message.to_string(),
            source: source.to_string(),
            additional_fields: HashMap::with_capacity(16),
        }
    }

    pub fn is_error_level(&self) -> bool {
        matches!(
            self.log_level.to_uppercase().as_str(),
            "ERROR" | "FATAL" | "CRITICAL" | "WARN" | "WARNING"
        )
    }
}

impl FileMetadataData {

    pub fn new(filename: &str, file_size: u64, file_type: &str) -> Self {
        Self {
            filename: filename.to_string(),
            file_size,
            file_type: file_type.to_string(),
            hash_md5: None,
            hash_sha256: None,
            created_at: None,
            modified_at: None,
        }
    }

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
