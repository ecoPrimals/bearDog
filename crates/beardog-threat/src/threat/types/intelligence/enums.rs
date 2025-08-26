

use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub enum IndicatorType {
    IpAddress,
    Domain,
    DomainName,
    Url,
    FileHash,
    EmailAddress,
    Registry,
    RegistryKey, // Alias for Registry
    Mutex,
    UserAgent,
    Certificate,
    NetworkTraffic,
    NetworkPattern, // Add missing variant
    BehavioralPattern,
    ProcessName, // Add missing variant
    Yara,
    Sigma,
    Custom(String), // Add missing variant
}

#[derive(Debug, Clone, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub enum ConfidenceLevel {
    Low,
    Medium,
    High,
    Critical,
}

#[derive(Debug, Clone, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub enum FeedType {
    IpReputation,
    DomainReputation,
    FileHashes,
    UrlBlacklist,
    MalwareSignatures,
    AttackPatterns,
    ThreatActors,
    Vulnerabilities,
}

#[derive(Debug, Clone, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub enum UpdateFrequency {
    RealTime,
    Hourly,
    Daily,
    Weekly,
    Monthly,
    OnDemand,
}

#[derive(Debug, Clone, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub enum FeedFormat {
    Json,
    Xml,
    Csv,
    Stix,
    Taxii,
}

#[derive(Debug, Clone, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub enum FeedStatus {
    Active,
    Inactive,
    Error,
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
            FeedType::UrlBlacklist => write!(f, "URL Blacklist"),
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
