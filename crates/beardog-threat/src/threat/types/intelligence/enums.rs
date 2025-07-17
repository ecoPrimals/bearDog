//! Threat intelligence enumeration types
//!
//! This module contains enumeration types used throughout the threat intelligence
//! system, including feed types, update frequencies, and status indicators.

use serde::{Deserialize, Serialize};

/// Feed type enumeration
///
/// Categorizes different types of threat intelligence feeds
/// based on the type of data they provide.
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

/// Update frequency enumeration
///
/// Defines how frequently a threat intelligence feed
/// should be updated with new data.
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

/// Feed status enumeration
///
/// Tracks the current operational status of a
/// threat intelligence feed.
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
        }
    }
}

impl std::fmt::Display for FeedStatus {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            FeedStatus::Active => write!(f, "Active"),
            FeedStatus::Inactive => write!(f, "Inactive"),
            FeedStatus::Error => write!(f, "Error"),
            FeedStatus::Updating => write!(f, "Updating"),
        }
    }
}
