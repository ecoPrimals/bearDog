//! Type definitions for threat detection and response
//!
//! Contains all structs, enums, and type aliases for the threat detection system.

use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};
use std::collections::{HashMap, HashSet};

/// Configuration for threat detection engine
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ThreatDetectionConfig {
    /// Enable real-time threat detection
    pub real_time_detection: bool,
    /// Threat scoring threshold (0-100)
    pub threat_threshold: u8,
    /// Enable automated response
    pub automated_response: bool,
    /// Maximum alerts per minute
    pub max_alerts_per_minute: u32,
    /// Enable machine learning enhancement
    pub ml_enhancement: bool,
    /// Threat intelligence feeds
    pub threat_feeds: Vec<String>,
    /// Quarantine suspicious activity
    pub auto_quarantine: bool,
    /// Alert notification endpoints
    pub notification_endpoints: Vec<String>,
    /// Enable threat detection
    pub enabled: bool,
    /// Path to detection rules
    pub rules_path: String,
    /// Paths to monitor for threats
    pub monitor_paths: Vec<String>,
    /// Alert threshold for triggering actions
    pub alert_threshold: f64,
    /// Cache size for threat data
    pub cache_size: usize,
    /// Monitoring interval in seconds
    pub monitoring_interval: u64,
}

/// Threat severity levels
#[derive(Debug, Clone, PartialEq, Eq, Hash, PartialOrd, Ord, Serialize, Deserialize)]
pub enum ThreatSeverity {
    /// Informational - no immediate action required
    Info,
    /// Low severity - monitor activity
    Low,
    /// Medium severity - investigate promptly
    Medium,
    /// High severity - immediate investigation required
    High,
    /// Critical severity - emergency response needed
    Critical,
}

/// Types of security threats
#[derive(Debug, Clone, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub enum ThreatType {
    /// Malware detection
    Malware,
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

/// Threat detection event
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

/// Source of a threat
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct ThreatSource {
    /// Source IP address
    pub ip_address: Option<String>,
    /// Source hostname
    pub hostname: Option<String>,
    /// Geographic location
    pub geolocation: Option<GeoLocation>,
    /// User agent information
    pub user_agent: Option<String>,
    /// Source reputation score
    pub reputation_score: f64,
    /// Known threat actor
    pub threat_actor: Option<String>,
    /// Source classification
    pub classification: SourceClassification,
}

/// Target of a threat
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ThreatTarget {
    /// Target resource ID
    pub resource_id: String,
    /// Resource type
    pub resource_type: String,
    /// Target node ID
    pub node_id: Option<String>,
    /// Target user account
    pub user_account: Option<String>,
    /// Asset criticality
    pub criticality: AssetCriticality,
    /// Protection level
    pub protection_level: ProtectionLevel,
}

/// Geographic location information for threat sources
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct GeoLocation {
    /// Country name or code
    pub country: String,
    /// State or region within the country
    pub region: Option<String>,
    /// City name
    pub city: Option<String>,
    /// Latitude coordinate
    pub latitude: Option<f64>,
    /// Longitude coordinate
    pub longitude: Option<f64>,
    /// Whether this is a Tor exit node
    pub is_tor_exit: bool,
    /// Whether this is a VPN endpoint
    pub is_vpn: bool,
    /// Whether this is a proxy server
    pub is_proxy: bool,
}

/// Classification of threat source reliability and trustworthiness
#[derive(Debug, Clone, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub enum SourceClassification {
    /// Trusted source with high confidence
    Trusted,
    /// Neutral source with no specific reputation
    Neutral,
    /// Suspicious source requiring monitoring
    Suspicious,
    /// Malicious source confirmed as threat
    Malicious,
    /// Blacklisted source blocked by security policies
    Blacklisted,
    /// Unknown source with no classification data
    Unknown,
}

/// Criticality level of assets for threat prioritization
#[derive(Debug, Clone, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub enum AssetCriticality {
    /// Low criticality - minimal impact if compromised
    Low,
    /// Medium criticality - moderate impact if compromised
    Medium,
    /// High criticality - significant impact if compromised
    High,
    /// Critical assets - severe impact if compromised
    Critical,
    /// Mission-critical assets - catastrophic impact if compromised
    Mission,
}

/// Protection level applied to assets
#[derive(Debug, Clone, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub enum ProtectionLevel {
    /// No protection applied
    None,
    /// Basic protection with minimal security measures
    Basic,
    /// Standard protection with typical security controls
    Standard,
    /// Enhanced protection with additional security layers
    Enhanced,
    /// Maximum protection with all available security measures
    Maximum,
}

/// Detection methods
#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum DetectionMethod {
    /// Signature-based detection
    Signature,
    /// Anomaly-based detection
    Anomaly,
    /// Behavioral analysis
    Behavioral,
    /// Machine learning
    MachineLearning,
    /// Rule-based detection
    RuleBased,
    /// Heuristic analysis
    Heuristic,
    /// Reputation analysis
    Reputation,
    /// Threat intelligence
    ThreatIntelligence,
    /// User and entity behavior analytics
    Ueba,
    /// Network traffic analysis
    NetworkAnalysis,
}

/// Types of evidence that can be collected during threat detection
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
}

/// Data format for evidence collected during threat detection
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

/// Evidence collected during threat detection
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
    /// Reliability score
    pub reliability: f64,
}

/// Network packet data for evidence collection
#[derive(Debug, Clone, Serialize, Deserialize)]
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

/// Log entry data for evidence collection
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

/// File metadata for evidence collection
#[derive(Debug, Clone, Serialize, Deserialize)]
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

/// Recommended threat actions
#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum ThreatAction {
    /// Block the source
    BlockSource,
    /// Quarantine affected systems
    QuarantineSystem,
    /// Alert security team
    AlertSecurityTeam,
    /// Isolate network segment
    IsolateNetwork,
    /// Reset user credentials
    ResetCredentials,
    /// Update security policies
    UpdatePolicies,
    /// Deploy patches
    DeployPatches,
    /// Backup critical data
    BackupData,
    /// Initiate incident response
    InitiateIncidentResponse,
    /// Notify law enforcement
    NotifyLawEnforcement,
    /// Engage threat hunting team
    EngageThreatHunting,
    /// Update threat intelligence
    UpdateThreatIntelligence,
}

/// Status of a threat event in the investigation lifecycle
#[derive(Debug, Clone, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub enum ThreatStatus {
    /// New threat detected and awaiting initial assessment
    New,
    /// Threat under active investigation
    Investigating,
    /// Threat confirmed as genuine security issue
    Confirmed,
    /// Threat determined to be false positive
    FalsePositive,
    /// Threat has been mitigated
    Mitigated,
    /// Threat has been fully resolved
    Resolved,
    /// Threat escalated to higher security level
    Escalated,
    /// Threat alerts suppressed due to noise
    Suppressed,
    /// Threat is currently active and ongoing
    Active,
    /// Threat acknowledged by security team
    Acknowledged,
    /// Threat contained but not yet eliminated
    Contained,
    /// Threat fully eradicated from systems
    Eradicated,
    /// System recovery in progress after threat
    Recovery,
    /// Post-incident analysis phase
    PostIncidentAnalysis,
}

/// Mitigation steps
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct MitigationStep {
    /// Step description
    pub description: String,
    /// Execution timestamp
    pub executed_at: DateTime<Utc>,
    /// Success status
    pub success: bool,
    /// Additional details
    pub details: Option<String>,
    /// Executing system/user
    pub executor: String,
}

/// Threat detection statistics
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct ThreatDetectionStats {
    /// Total threats detected
    pub total_threats: u64,
    /// Threats by severity
    pub threats_by_severity: HashMap<String, u64>,
    /// Threats by type
    pub threats_by_type: HashMap<String, u64>,
    /// Detection methods performance
    pub detection_methods: HashMap<String, DetectionMethodStats>,
    /// False positive rate
    pub false_positive_rate: f64,
    /// Average response time
    pub avg_response_time_seconds: f64,
    /// Blocked sources
    pub blocked_sources: u64,
    /// Quarantined systems
    pub quarantined_systems: u64,
    /// Threat trends
    pub threat_trends: Vec<ThreatTrend>,
}

/// Performance statistics for threat detection methods
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct DetectionMethodStats {
    /// Number of detections by this method
    pub detections: u64,
    /// Detection accuracy percentage
    pub accuracy: f64,
    /// Number of false positives generated
    pub false_positives: u64,
    /// Average confidence score of detections
    pub avg_confidence: f64,
}

/// Trend data for threat detection over time
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ThreatTrend {
    /// Timestamp of the trend data point
    pub timestamp: DateTime<Utc>,
    /// Total number of threats at this timestamp
    pub threat_count: u64,
    /// Distribution of threats by severity level
    pub severity_distribution: HashMap<ThreatSeverity, u64>,
}

/// Threat intelligence feed
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ThreatIntelligenceFeed {
    /// Feed identifier
    pub id: String,
    /// Feed name
    pub name: String,
    /// Feed URL
    pub url: String,
    /// Feed type
    pub feed_type: FeedType,
    /// Update frequency
    pub update_frequency: UpdateFrequency,
    /// Last update
    pub last_updated: DateTime<Utc>,
    /// Feed status
    pub status: FeedStatus,
    /// Indicators of compromise
    pub indicators: Vec<ThreatIndicator>,
}

/// Types of threat intelligence feeds
#[derive(Debug, Clone, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub enum FeedType {
    /// IP address reputation feed
    IpReputation,
    /// Domain reputation feed
    DomainReputation,
    /// Known malicious file hashes
    FileHashes,
    /// URL blacklist feed
    UrlBlacklist,
    /// Malware signature definitions
    MalwareSignatures,
    /// Attack pattern descriptions
    AttackPatterns,
    /// Known threat actor profiles
    ThreatActors,
    /// Vulnerability information feed
    Vulnerabilities,
}

/// Update frequency for threat intelligence feeds
#[derive(Debug, Clone, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub enum UpdateFrequency {
    /// Real-time continuous updates
    RealTime,
    /// Updated every hour
    Hourly,
    /// Updated daily
    Daily,
    /// Updated weekly
    Weekly,
    /// Updated monthly
    Monthly,
}

/// Status of threat intelligence feeds
#[derive(Debug, Clone, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub enum FeedStatus {
    /// Feed is active and operational
    Active,
    /// Feed is inactive or disabled
    Inactive,
    /// Feed encountered an error
    Error,
    /// Feed is currently being updated
    Updating,
}

/// Threat indicators
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ThreatIndicator {
    /// Indicator type
    pub indicator_type: IndicatorType,
    /// Indicator value
    pub value: String,
    /// Confidence level
    pub confidence: f64,
    /// First seen
    pub first_seen: DateTime<Utc>,
    /// Last seen
    pub last_seen: DateTime<Utc>,
    /// Associated threat types
    pub threat_types: Vec<ThreatType>,
    /// Tags
    pub tags: Vec<String>,
}

/// Types of threat indicators
#[derive(Debug, Clone, PartialEq, Eq, Hash, Serialize, Deserialize)]
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
    /// Windows registry key indicator
    RegistryKey,
    /// Process mutex indicator
    Mutex,
    /// User agent string indicator
    UserAgent,
    /// Digital certificate indicator
    Certificate,
    /// Process name indicator
    ProcessName,
    /// Network traffic pattern indicator
    NetworkPattern,
    /// Behavioral pattern indicator
    BehaviorPattern,
    /// Custom indicator type
    Custom(String),
}

/// Main threat detection engine
pub struct ThreatDetectionEngine {
    pub config: ThreatDetectionConfig,
    pub active_threats: HashMap<String, ThreatEvent>,
    pub blocked_sources: HashSet<String>,
    pub quarantined_systems: HashSet<String>,
    pub threat_feeds: HashMap<String, ThreatIntelligenceFeed>,
    pub detection_rules: Vec<DetectionRule>,
    pub stats: ThreatDetectionStats,
    pub ml_models: HashMap<String, MlModel>,
}

/// Detection rule definition
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct DetectionRule {
    /// Rule identifier
    pub id: String,
    /// Rule name
    pub name: String,
    /// Rule description
    pub description: String,
    /// Rule condition
    pub condition: RuleCondition,
    /// Threat type to generate
    pub threat_type: ThreatType,
    /// Severity level
    pub severity: ThreatSeverity,
    /// Rule enabled status
    pub enabled: bool,
    /// Detection count
    pub detection_count: u64,
    /// False positive count
    pub false_positive_count: u64,
}

/// Rule conditions
#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum RuleCondition {
    /// Simple field matching
    FieldEquals { field: String, value: String },
    /// Pattern matching
    PatternMatch { field: String, pattern: String },
    /// Threshold detection
    Threshold {
        field: String,
        operator: String,
        value: f64,
    },
    /// Complex boolean logic
    Complex { expression: String },
    /// Machine learning prediction
    MlPrediction { model: String, confidence: f64 },
    /// Event type matching
    EventType { event_type: String },
    /// Source IP matching
    SourceIp { ip_address: String },
    /// User agent matching
    UserAgent { user_agent: String },
    /// Data size threshold
    DataSize { size_bytes: u64 },
    /// Time range condition
    TimeRange { start_hour: u8, end_hour: u8 },
    /// Frequency threshold
    FrequencyThreshold { count: u64, window_minutes: u64 },
}

/// Machine learning model
#[derive(Debug, Clone)]
pub struct MlModel {
    pub id: String,
    pub name: String,
    pub model_type: MlModelType,
    pub accuracy: f64,
    pub last_trained: DateTime<Utc>,
    pub feature_names: Vec<String>,
}

/// ML model types
#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum MlModelType {
    AnomalyDetection,
    Classification,
    Clustering,
    NeuralNetwork,
    RandomForest,
    SupportVector,
    NaiveBayes,
}

/// Comprehensive security event structure
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SecurityEvent {
    pub event_id: String,
    pub timestamp: DateTime<Utc>,
    pub event_type: String,
    pub source_ip: String,
    pub destination_ip: String,
    pub user_id: String,
    pub user_agent: Option<String>,
    pub data_size: f64,
    pub location: Option<String>,
    pub file_hash: Option<String>,
    pub additional_data: HashMap<String, String>,
}

/// Response action types
#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum ResponseAction {
    LogAlert(String),
    BlockIp(String),
    QuarantineUser(String),
    NotifyAdmin(String),
    IsolateSystem(String),
}

/// Enhanced threat detection rule
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ThreatDetectionRule {
    pub rule_id: String,
    pub name: String,
    pub description: String,
    pub threat_type: ThreatType,
    pub severity: ThreatSeverity,
    pub conditions: Vec<RuleCondition>,
    pub false_positive_rate: f64,
    pub mitre_techniques: Vec<String>,
    pub response_actions: Vec<ResponseAction>,
    pub enabled: bool,
}

// Default implementations

impl Default for ThreatDetectionConfig {
    fn default() -> Self {
        Self {
            real_time_detection: true,
            threat_threshold: 70,
            automated_response: false,
            max_alerts_per_minute: 100,
            ml_enhancement: true,
            threat_feeds: vec![],
            auto_quarantine: false,
            notification_endpoints: vec![],
            enabled: true,
            rules_path: String::new(),
            monitor_paths: vec![],
            alert_threshold: 0.8,
            cache_size: 1000,
            monitoring_interval: 300,
        }
    }
}

impl Default for ThreatSeverity {
    fn default() -> Self {
        ThreatSeverity::Medium
    }
}

impl Default for ThreatStatus {
    fn default() -> Self {
        ThreatStatus::New
    }
}

impl Default for SourceClassification {
    fn default() -> Self {
        SourceClassification::Unknown
    }
}

impl Default for AssetCriticality {
    fn default() -> Self {
        AssetCriticality::Medium
    }
}

impl Default for ProtectionLevel {
    fn default() -> Self {
        ProtectionLevel::Standard
    }
}

// Utility implementations

impl ThreatSeverity {
    /// Convert severity to numeric score
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
    pub fn from_score(score: u8) -> Self {
        match score {
            0..=20 => ThreatSeverity::Info,
            21..=40 => ThreatSeverity::Low,
            41..=60 => ThreatSeverity::Medium,
            61..=80 => ThreatSeverity::High,
            81..=100 => ThreatSeverity::Critical,
            _ => ThreatSeverity::Critical, // Handle values > 100 as Critical
        }
    }
}

impl ThreatType {
    /// Get the typical severity for this threat type
    pub fn typical_severity(&self) -> ThreatSeverity {
        match self {
            ThreatType::Phishing | ThreatType::SocialEngineering => ThreatSeverity::Medium,
            ThreatType::BruteForce
            | ThreatType::BruteForceAttack
            | ThreatType::SuspiciousLogin
            | ThreatType::SqlInjection
            | ThreatType::Xss => ThreatSeverity::High,
            ThreatType::Malware | ThreatType::Ransomware | ThreatType::ZeroDayExploit => {
                ThreatSeverity::Critical
            }
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
}

impl ThreatEvent {
    /// Check if the threat is considered high priority
    pub fn is_high_priority(&self) -> bool {
        self.severity >= ThreatSeverity::High || self.score >= 80
    }

    /// Calculate age in minutes
    pub fn age_minutes(&self) -> i64 {
        let now = Utc::now();
        (now - self.timestamp).num_minutes()
    }
}

impl std::fmt::Display for ThreatSeverity {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            ThreatSeverity::Info => write!(f, "Info"),
            ThreatSeverity::Low => write!(f, "Low"),
            ThreatSeverity::Medium => write!(f, "Medium"),
            ThreatSeverity::High => write!(f, "High"),
            ThreatSeverity::Critical => write!(f, "Critical"),
        }
    }
}

impl std::fmt::Display for ThreatType {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            ThreatType::Malware => write!(f, "Malware"),
            ThreatType::Phishing => write!(f, "Phishing"),
            ThreatType::BruteForce => write!(f, "Brute Force"),
            ThreatType::BruteForceAttack => write!(f, "Brute Force Attack"),
            ThreatType::SuspiciousLogin => write!(f, "Suspicious Login"),
            ThreatType::DdosAttack => write!(f, "DDoS Attack"),
            ThreatType::DataExfiltration => write!(f, "Data Exfiltration"),
            ThreatType::PrivilegeEscalation => write!(f, "Privilege Escalation"),
            ThreatType::InsiderThreat => write!(f, "Insider Threat"),
            ThreatType::NetworkIntrusion => write!(f, "Network Intrusion"),
            ThreatType::SqlInjection => write!(f, "SQL Injection"),
            ThreatType::Xss => write!(f, "Cross-Site Scripting"),
            ThreatType::ManInTheMiddle => write!(f, "Man-in-the-Middle"),
            ThreatType::Ransomware => write!(f, "Ransomware"),
            ThreatType::BotnetActivity => write!(f, "Botnet Activity"),
            ThreatType::ZeroDayExploit => write!(f, "Zero-Day Exploit"),
            ThreatType::Apt => write!(f, "Advanced Persistent Threat"),
            ThreatType::SocialEngineering => write!(f, "Social Engineering"),
            ThreatType::SupplyChainAttack => write!(f, "Supply Chain Attack"),
            ThreatType::AiPoisoning => write!(f, "AI/ML Poisoning"),
            ThreatType::QuantumThreat => write!(f, "Quantum Threat"),
            ThreatType::Anomaly => write!(f, "Anomaly"),
            ThreatType::Unknown => write!(f, "Unknown"),
        }
    }
}

impl std::fmt::Display for IndicatorType {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            IndicatorType::IpAddress => write!(f, "IP Address"),
            IndicatorType::DomainName => write!(f, "Domain Name"),
            IndicatorType::Url => write!(f, "URL"),
            IndicatorType::FileHash => write!(f, "File Hash"),
            IndicatorType::EmailAddress => write!(f, "Email Address"),
            IndicatorType::RegistryKey => write!(f, "Registry Key"),
            IndicatorType::Mutex => write!(f, "Mutex"),
            IndicatorType::UserAgent => write!(f, "User Agent"),
            IndicatorType::Certificate => write!(f, "Certificate"),
            IndicatorType::ProcessName => write!(f, "Process Name"),
            IndicatorType::NetworkPattern => write!(f, "Network Pattern"),
            IndicatorType::BehaviorPattern => write!(f, "Behavior Pattern"),
            IndicatorType::Custom(custom) => write!(f, "Custom: {}", custom),
        }
    }
}
