use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, PartialEq, Eq, Hash, Serialize, Deserialize)]
/// Types of indicator
/// Types of indicator
pub enum IndicatorType {
    /// Represents ip address variant
    IpAddress,
    /// Represents domain variant
    Domain,
    /// Represents domain name variant
    DomainName,
    /// Represents url variant
    Url,
    /// Represents file hash variant
    FileHash,
    /// Represents email address variant
    EmailAddress,
    /// Represents registry variant
    Registry,
    /// Represents registry key variant
    RegistryKey, // Alias for Registry
    /// Represents mutex variant
    Mutex,
    /// Represents user agent variant
    UserAgent,
    /// Represents certificate variant
    Certificate,
    /// Represents network traffic variant
    NetworkTraffic,
    /// Represents network pattern variant
    NetworkPattern, // Add missing variant
    /// Represents behavioral pattern variant
    BehavioralPattern,
    /// Represents process name variant
    ProcessName, // Add missing variant
    /// Represents yara variant
    Yara,
    /// Represents sigma variant
    Sigma,
    /// Represents custom variant
    Custom(String), // Add missing variant
}

#[derive(Debug, Clone, PartialEq, Eq, Hash, PartialOrd, Ord, Serialize, Deserialize)]
pub enum ConfidenceLevel {
    /// Represents low variant
    Low,
    /// Represents medium variant
    Medium,
    /// Represents high variant
    High,
    /// Represents critical variant
    Critical,
}

#[derive(Debug, Clone, PartialEq, Eq, Hash, Serialize, Deserialize)]
/// Types of feed
/// Types of feed
pub enum FeedType {
    /// Represents ip reputation variant
    IpReputation,
    /// Represents domain reputation variant
    DomainReputation,
    /// Represents file hashes variant
    FileHashes,
    /// Represents url blocklist variant
    UrlBlocklist,
    /// Represents malware signatures variant
    MalwareSignatures,
    /// Represents attack patterns variant
    AttackPatterns,
    /// Represents threat actors variant
    ThreatActors,
    /// Represents vulnerabilities variant
    Vulnerabilities,
}

#[derive(Debug, Clone, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub enum UpdateFrequency {
    /// Represents real time variant
    RealTime,
    /// Represents hourly variant
    Hourly,
    /// Represents daily variant
    Daily,
    /// Represents weekly variant
    Weekly,
    /// Represents monthly variant
    Monthly,
    /// Represents on demand variant
    OnDemand,
}

#[derive(Debug, Clone, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub enum FeedFormat {
    /// Represents json variant
    Json,
    /// Represents xml variant
    Xml,
    /// Represents csv variant
    Csv,
    /// Represents stix variant
    Stix,
    /// Represents taxii variant
    Taxii,
}

#[derive(Debug, Clone, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub enum FeedStatus {
    /// Active or enabled state
    Active,
    /// Inactive or disabled state
    Inactive,
    /// Error or failure state
    Error,
    /// Represents maintenance variant
    Maintenance,
}

impl std::fmt::Display for IndicatorType {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            IndicatorType::IpAddress => write!(f, "IP Address"),
            IndicatorType::Domain => write!(f, "Domain"),
            IndicatorType::DomainName => write!(f, "Domain Name"),
            IndicatorType::Url => write!(f, "URL"),
            IndicatorType::FileHash => write!(f, "File Hash"),
            IndicatorType::EmailAddress => write!(f, "Email Address"),
            IndicatorType::Registry => write!(f, "Registry"),
            IndicatorType::RegistryKey => write!(f, "Registry Key"),
            IndicatorType::Mutex => write!(f, "Mutex"),
            IndicatorType::UserAgent => write!(f, "User Agent"),
            IndicatorType::Certificate => write!(f, "Certificate"),
            IndicatorType::NetworkTraffic => write!(f, "Network Traffic"),
            IndicatorType::NetworkPattern => write!(f, "Network Pattern"),
            IndicatorType::BehavioralPattern => write!(f, "Behavioral Pattern"),
            IndicatorType::ProcessName => write!(f, "Process Name"),
            IndicatorType::Yara => write!(f, "YARA Rule"),
            IndicatorType::Sigma => write!(f, "Sigma Rule"),
            IndicatorType::Custom(custom) => write!(f, "Custom: {custom}"),
        }
    }
}

impl std::fmt::Display for ConfidenceLevel {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            ConfidenceLevel::Low => write!(f, "Low"),
            ConfidenceLevel::Medium => write!(f, "Medium"),
            ConfidenceLevel::High => write!(f, "High"),
            ConfidenceLevel::Critical => write!(f, "Critical"),
        }
    }
}

impl std::fmt::Display for FeedType {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            FeedType::IpReputation => write!(f, "IP Reputation"),
            FeedType::DomainReputation => write!(f, "Domain Reputation"),
            FeedType::FileHashes => write!(f, "File Hashes"),
            FeedType::UrlBlocklist => write!(f, "URL Blocklist"),
            FeedType::MalwareSignatures => write!(f, "Malware Signatures"),
            FeedType::AttackPatterns => write!(f, "Attack Patterns"),
            FeedType::ThreatActors => write!(f, "Threat Actors"),
            FeedType::Vulnerabilities => write!(f, "Vulnerabilities"),
        }
    }
}

impl std::fmt::Display for UpdateFrequency {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            UpdateFrequency::RealTime => write!(f, "Real-time"),
            UpdateFrequency::Hourly => write!(f, "Hourly"),
            UpdateFrequency::Daily => write!(f, "Daily"),
            UpdateFrequency::Weekly => write!(f, "Weekly"),
            UpdateFrequency::Monthly => write!(f, "Monthly"),
            UpdateFrequency::OnDemand => write!(f, "On Demand"),
        }
    }
}

impl std::fmt::Display for FeedStatus {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            FeedStatus::Active => write!(f, "Active"),
            FeedStatus::Inactive => write!(f, "Inactive"),
            FeedStatus::Error => write!(f, "Error"),
            FeedStatus::Maintenance => write!(f, "Maintenance"),
        }
    }
}
