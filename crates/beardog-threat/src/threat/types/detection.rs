//! Detection and evidence types
//!
//! This module contains types for threat detection methods, evidence collection,
//! and evidence data structures.
//!
//! ## Features
//! - Detection method enumeration
//! - Evidence type classification
//! - Evidence data structures
//! - Network packet analysis
//! - Log entry processing
//! - File metadata analysis
//!
//! ## Example
//! ```rust
//! use beardog::threat::types::{DetectionMethod, EvidenceType, ThreatEvidence};
//! use chrono::Utc;
//!
//! let evidence = ThreatEvidence {
//!     evidence_type: EvidenceType::NetworkTraffic,
//!     description: "Suspicious network connection".to_string(),
//!     data: EvidenceData::Text("192.168.1.100:443 -> 10.0.0.1:80".to_string()),
//!     collected_at: Utc::now(),
//!     reliability: 0.85,
//!     ..Default::default()
//! };
//! ```

use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};
use std::collections::HashMap;

/// Detection methods supported by the system
///
/// Enumeration of different approaches used for threat detection,
/// each with different characteristics and capabilities.
#[derive(Debug, Clone, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub enum DetectionMethod {
    /// Signature-based detection
    ///
    /// Uses known patterns and signatures to identify threats.
    /// High accuracy for known threats but poor for zero-day attacks.
    Signature,

    /// Anomaly-based detection
    ///
    /// Detects deviations from normal behavior patterns.
    /// Good for unknown threats but higher false positive rate.
    Anomaly,

    /// Behavioral analysis
    ///
    /// Analyzes behavior patterns to identify suspicious activities.
    /// Effective for insider threats and advanced persistent threats.
    Behavioral,

    /// Machine learning
    ///
    /// Uses ML models to classify and detect threats.
    /// Adaptive and can learn from new data.
    MachineLearning,

    /// Rule-based detection
    ///
    /// Uses predefined rules and conditions to detect threats.
    /// Deterministic and explainable but requires maintenance.
    RuleBased,

    /// Heuristic analysis
    ///
    /// Uses algorithms to identify potentially malicious behavior.
    /// Balance between accuracy and unknown threat detection.
    Heuristic,

    /// Reputation analysis
    ///
    /// Uses reputation data to assess threat likelihood.
    /// Effective for known bad actors and IP addresses.
    Reputation,

    /// Threat intelligence
    ///
    /// Uses external threat intelligence feeds.
    /// Provides context and attribution for threats.
    ThreatIntelligence,

    /// User and entity behavior analytics
    ///
    /// Analyzes user and entity behavior for anomalies.
    /// Effective for insider threats and account compromise.
    Ueba,

    /// Network traffic analysis
    ///
    /// Analyzes network traffic patterns and content.
    /// Effective for network-based attacks and lateral movement.
    NetworkAnalysis,
}

/// Types of evidence that can be collected
///
/// Comprehensive enumeration of different types of evidence
/// that can be collected during threat detection and analysis.
#[derive(Debug, Clone, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub enum EvidenceType {
    /// Network traffic data and packet analysis
    NetworkTraffic,

    /// System log files and entries
    SystemLogs,

    /// File system analysis and file inspection
    FileAnalysis,

    /// Memory dump analysis
    MemoryDump,

    /// Running process information
    ProcessInformation,

    /// Windows registry changes
    RegistryChanges,

    /// Active network connections
    NetworkConnections,

    /// DNS query logs
    DnsQueries,

    /// HTTP request logs
    HttpRequests,

    /// Email header information
    EmailHeaders,

    /// Cryptographic file hashes
    FileHashes,

    /// Digital signatures and certificates
    CryptographicSignatures,

    /// User activity logs
    UserActivity,

    /// Database query logs
    DatabaseQueries,

    /// API call logs
    ApiCalls,

    /// Detection rule matches
    RuleMatch,

    /// Threat intelligence feed matches
    ThreatIntelligence,
}

/// Evidence data variants
///
/// Represents different types of evidence data that can be collected
/// and stored as part of threat investigation.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum EvidenceData {
    /// Plain text evidence data
    Text(String),

    /// Binary evidence data
    Binary(Vec<u8>),

    /// JSON structured evidence data
    Json(serde_json::Value),

    /// Hash value evidence
    Hash(String),

    /// Network packet data
    NetworkPacket(NetworkPacketData),

    /// System log entry data
    LogEntry(LogEntryData),

    /// File metadata information
    FileMetadata(FileMetadataData),
}

/// Threat evidence structure
///
/// Represents a piece of evidence collected during threat detection
/// or investigation, including metadata and chain of custody information.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ThreatEvidence {
    /// Evidence type
    pub evidence_type: EvidenceType,

    /// Evidence description
    pub description: String,

    /// Evidence data
    pub data: EvidenceData,

    /// Collection timestamp
    pub collected_at: DateTime<Utc>,

    /// Chain of custody
    pub chain_of_custody: Vec<String>,

    /// Reliability score (0.0 to 1.0)
    pub reliability: f64,
}

/// Network packet data structure
///
/// Represents network packet information collected during
/// network traffic analysis.
#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct NetworkPacketData {
    /// Source IP address
    pub source_ip: String,

    /// Destination IP address
    pub dest_ip: String,

    /// Source port number
    pub source_port: u16,

    /// Destination port number
    pub dest_port: u16,

    /// Network protocol (TCP, UDP, etc.)
    pub protocol: String,

    /// Size of the packet payload in bytes
    pub payload_size: usize,

    /// Protocol-specific flags
    pub flags: Vec<String>,
}

/// Log entry data structure
///
/// Represents system log entry information collected during
/// log analysis and monitoring.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct LogEntryData {
    /// Log severity level
    pub log_level: String,

    /// Log message content
    pub message: String,

    /// Source system or component
    pub source: String,

    /// Additional structured log fields
    pub additional_fields: HashMap<String, String>,
}

/// File metadata structure
///
/// Represents file metadata information collected during
/// file system analysis.
#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct FileMetadataData {
    /// Name of the file
    pub filename: String,

    /// File size in bytes
    pub file_size: u64,

    /// File type or MIME type
    pub file_type: String,

    /// MD5 hash of the file
    pub hash_md5: Option<String>,

    /// SHA-256 hash of the file
    pub hash_sha256: Option<String>,

    /// File creation timestamp
    pub created_at: Option<DateTime<Utc>>,

    /// File last modification timestamp
    pub modified_at: Option<DateTime<Utc>>,
}

// Default implementations

impl Default for ThreatEvidence {
    fn default() -> Self {
        Self {
            evidence_type: EvidenceType::SystemLogs,
            description: String::new(),
            data: EvidenceData::Text(String::new()),
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
            message: String::new(),
            source: String::new(),
            additional_fields: HashMap::new(),
        }
    }
}

// Utility implementations

impl DetectionMethod {
    /// Get the typical accuracy for this detection method
    ///
    /// # Returns
    /// Accuracy score (0.0 to 1.0)
    ///
    /// # Example
    /// ```rust
    /// use beardog::threat::types::DetectionMethod;
    ///
    /// assert!(DetectionMethod::Signature.typical_accuracy() > 0.9);
    /// assert!(DetectionMethod::Anomaly.typical_accuracy() < 0.8);
    /// ```
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

    /// Get the typical false positive rate for this detection method
    ///
    /// # Returns
    /// False positive rate (0.0 to 1.0)
    ///
    /// # Example
    /// ```rust
    /// use beardog::threat::types::DetectionMethod;
    ///
    /// assert!(DetectionMethod::Signature.false_positive_rate() < 0.1);
    /// assert!(DetectionMethod::Anomaly.false_positive_rate() > 0.1);
    /// ```
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

    /// Check if this detection method is good for unknown threats
    ///
    /// # Returns
    /// `true` if effective against zero-day and unknown threats
    ///
    /// # Example
    /// ```rust
    /// use beardog::threat::types::DetectionMethod;
    ///
    /// assert!(DetectionMethod::Anomaly.is_good_for_unknown_threats());
    /// assert!(!DetectionMethod::Signature.is_good_for_unknown_threats());
    /// ```
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
    /// Get the typical reliability for this evidence type
    ///
    /// # Returns
    /// Reliability score (0.0 to 1.0)
    ///
    /// # Example
    /// ```rust
    /// use beardog::threat::types::EvidenceType;
    ///
    /// assert!(EvidenceType::FileHashes.typical_reliability() > 0.9);
    /// assert!(EvidenceType::UserActivity.typical_reliability() < 0.8);
    /// ```
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

    /// Check if this evidence type is forensically sound
    ///
    /// # Returns
    /// `true` if evidence type maintains forensic integrity
    ///
    /// # Example
    /// ```rust
    /// use beardog::threat::types::EvidenceType;
    ///
    /// assert!(EvidenceType::FileHashes.is_forensically_sound());
    /// assert!(EvidenceType::MemoryDump.is_forensically_sound());
    /// ```
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
    /// Create new threat evidence
    ///
    /// # Arguments
    /// * `evidence_type` - Type of evidence
    /// * `description` - Description of the evidence
    /// * `data` - Evidence data
    ///
    /// # Returns
    /// A new `ThreatEvidence` instance
    ///
    /// # Example
    /// ```rust
    /// use beardog::threat::types::{ThreatEvidence, EvidenceType, EvidenceData};
    ///
    /// let evidence = ThreatEvidence::new(
    ///     EvidenceType::NetworkTraffic,
    ///     "Suspicious connection".to_string(),
    ///     EvidenceData::Text("192.168.1.100".to_string())
    /// );
    /// ```
    pub fn new(evidence_type: EvidenceType, description: String, data: EvidenceData) -> Self {
        let reliability = evidence_type.typical_reliability();
        Self {
            evidence_type,
            description,
            data,
            collected_at: Utc::now(),
            chain_of_custody: vec![],
            reliability,
        }
    }

    /// Add to chain of custody
    ///
    /// # Arguments
    /// * `handler` - Name or ID of the handler
    ///
    /// # Example
    /// ```rust
    /// use beardog::threat::types::ThreatEvidence;
    ///
    /// let mut evidence = ThreatEvidence::default();
    /// evidence.add_custody_handler("analyst-001".to_string());
    /// evidence.add_custody_handler("forensics-team".to_string());
    ///
    /// assert_eq!(evidence.chain_of_custody.len(), 2);
    /// ```
    pub fn add_custody_handler(&mut self, handler: String) {
        self.chain_of_custody.push(handler);
    }

    /// Check if evidence is highly reliable
    ///
    /// # Returns
    /// `true` if reliability is above 0.8
    ///
    /// # Example
    /// ```rust
    /// use beardog::threat::types::ThreatEvidence;
    ///
    /// let mut evidence = ThreatEvidence::default();
    /// evidence.reliability = 0.9;
    ///
    /// assert!(evidence.is_highly_reliable());
    /// ```
    pub fn is_highly_reliable(&self) -> bool {
        self.reliability > 0.8
    }

    /// Get evidence age in minutes
    ///
    /// # Returns
    /// Age in minutes since collection
    ///
    /// # Example
    /// ```rust
    /// use beardog::threat::types::ThreatEvidence;
    /// use chrono::{Utc, Duration};
    ///
    /// let mut evidence = ThreatEvidence::default();
    /// evidence.collected_at = Utc::now() - Duration::minutes(30);
    ///
    /// assert_eq!(evidence.age_minutes(), 30);
    /// ```
    pub fn age_minutes(&self) -> i64 {
        let now = Utc::now();
        (now - self.collected_at).num_minutes()
    }
}

impl NetworkPacketData {
    /// Create new network packet data
    ///
    /// # Arguments
    /// * `source_ip` - Source IP address
    /// * `dest_ip` - Destination IP address
    /// * `source_port` - Source port
    /// * `dest_port` - Destination port
    /// * `protocol` - Network protocol
    ///
    /// # Returns
    /// A new `NetworkPacketData` instance
    ///
    /// # Example
    /// ```rust
    /// use beardog::threat::types::NetworkPacketData;
    ///
    /// let packet = NetworkPacketData::new(
    ///     "192.168.1.100".to_string(),
    ///     "10.0.0.1".to_string(),
    ///     12345,
    ///     80,
    ///     "TCP".to_string()
    /// );
    /// ```
    pub fn new(
        source_ip: String,
        dest_ip: String,
        source_port: u16,
        dest_port: u16,
        protocol: String,
    ) -> Self {
        Self {
            source_ip,
            dest_ip,
            source_port,
            dest_port,
            protocol,
            payload_size: 0,
            flags: vec![],
        }
    }

    /// Check if packet is using a suspicious port
    ///
    /// # Returns
    /// `true` if using commonly exploited ports
    ///
    /// # Example
    /// ```rust
    /// use beardog::threat::types::NetworkPacketData;
    ///
    /// let packet = NetworkPacketData::new(
    ///     "192.168.1.100".to_string(),
    ///     "10.0.0.1".to_string(),
    ///     12345,
    ///     1337,
    ///     "TCP".to_string()
    /// );
    ///
    /// // Check if using suspicious port
    /// ```
    pub fn is_suspicious_port(&self) -> bool {
        let suspicious_ports = [1337, 31337, 4444, 5555, 6666, 7777, 8888, 9999];
        suspicious_ports.contains(&self.dest_port) || suspicious_ports.contains(&self.source_port)
    }
}

impl LogEntryData {
    /// Create new log entry data
    ///
    /// # Arguments
    /// * `log_level` - Log level (INFO, WARN, ERROR, etc.)
    /// * `message` - Log message
    /// * `source` - Source system
    ///
    /// # Returns
    /// A new `LogEntryData` instance
    ///
    /// # Example
    /// ```rust
    /// use beardog::threat::types::LogEntryData;
    ///
    /// let log = LogEntryData::new(
    ///     "ERROR".to_string(),
    ///     "Failed login attempt".to_string(),
    ///     "auth-server".to_string()
    /// );
    /// ```
    pub fn new(log_level: String, message: String, source: String) -> Self {
        Self {
            log_level,
            message,
            source,
            additional_fields: HashMap::new(),
        }
    }

    /// Check if log entry indicates an error
    ///
    /// # Returns
    /// `true` if log level indicates error or warning
    ///
    /// # Example
    /// ```rust
    /// use beardog::threat::types::LogEntryData;
    ///
    /// let log = LogEntryData::new(
    ///     "ERROR".to_string(),
    ///     "System failure".to_string(),
    ///     "main-server".to_string()
    /// );
    ///
    /// assert!(log.is_error_level());
    /// ```
    pub fn is_error_level(&self) -> bool {
        matches!(
            self.log_level.to_uppercase().as_str(),
            "ERROR" | "FATAL" | "CRITICAL" | "WARN" | "WARNING"
        )
    }
}

impl FileMetadataData {
    /// Create new file metadata
    ///
    /// # Arguments
    /// * `filename` - Name of the file
    /// * `file_size` - Size in bytes
    /// * `file_type` - File type or MIME type
    ///
    /// # Returns
    /// A new `FileMetadataData` instance
    ///
    /// # Example
    /// ```rust
    /// use beardog::threat::types::FileMetadataData;
    ///
    /// let metadata = FileMetadataData::new(
    ///     "suspicious.exe".to_string(),
    ///     1024000,
    ///     "application/x-executable".to_string()
    /// );
    /// ```
    pub fn new(filename: String, file_size: u64, file_type: String) -> Self {
        Self {
            filename,
            file_size,
            file_type,
            hash_md5: None,
            hash_sha256: None,
            created_at: None,
            modified_at: None,
        }
    }

    /// Check if file is potentially suspicious
    ///
    /// # Returns
    /// `true` if file has suspicious characteristics
    ///
    /// # Example
    /// ```rust
    /// use beardog::threat::types::FileMetadataData;
    ///
    /// let metadata = FileMetadataData::new(
    ///     "virus.exe".to_string(),
    ///     1024,
    ///     "application/x-executable".to_string()
    /// );
    ///
    /// // Check if suspicious based on name/type
    /// ```
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

// Display implementations

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
