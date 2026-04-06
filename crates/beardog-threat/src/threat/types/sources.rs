// SPDX-License-Identifier: AGPL-3.0-or-later

// Module documentation
//
// This module provides functionality for the BearDog ecosystem.


use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};
use std::collections::HashMap;

#[derive(Debug, Clone)]
    /// The source type value
    pub source_type: String,
    /// Optional ip address
    pub ip_address: Option<String>,
    /// Name of the hostitem
    pub hostname: Option<String>,
    /// Optional geolocation
    pub geolocation: Option<GeoLocation>,
    /// Optional user agent
    pub user_agent: Option<String>,
    /// The reputation score value
    pub reputation_score: f64,
    /// Optional threat actor
    pub threat_actor: Option<String>,
    /// The classification value
    pub classification: SourceClassification,
    pub confidence_score: f64,
    /// Optional first seen
    pub first_seen: Option<DateTime<Utc>>,
    /// Optional last seen
    pub last_seen: Option<DateTime<Utc>>,
    /// The threat score value
    pub threat_score: f64,
    /// Mapping of metadata
    pub metadata: HashMap<String, serde_json::Value>,
}

impl Default for ThreatSource {
    fn default() -> Self {
        Self {
            id: String::with_capacity(64),
            source_type: String::with_capacity(None,
            hostname: None,
            geolocation: None,
            user_agent: None,
            reputation_score: 0.5,
            threat_actor: None,
            classification: SourceClassification::Unknown,
            confidence_score: 0.5,
            first_seen: None,
            last_seen: Some(chrono::Utc::now(0.0,
            metadata: HashMap::with_capacity(String,
    /// The target type value
    pub target_type: String,
    pub resource_id: String,
    pub node_id: Option<String>,
    /// Number of user_acitems
    pub user_account: Option<String>,
    /// The asset criticality value
    pub asset_criticality: AssetCriticality,
    /// The protection level value
    pub protection_level: ProtectionLevel,
    /// Optional service
    pub service: Option<String>,
    /// Optional port
    pub port: Option<u16>,
    /// Optional protocol
    pub protocol: Option<String>,
    /// Mapping of metadata
    pub metadata: HashMap<String, serde_json::Value>,

    /// The resource type value
    pub resource_type: String,
    /// The criticality value
    pub criticality: AssetCriticality,
    /// Optional ip address
    pub ip_address: Option<String>,
    /// Name of the hostitem
    pub hostname: Option<String>,
}

impl Default for ThreatTarget {
    fn default() -> Self {
        Self {
            id: String::with_capacity(64),
            target_type: String::with_capacity(64),
            resource_id: String::with_capacity(None,
            user_account: None,
            asset_criticality: AssetCriticality::Medium,
            protection_level: ProtectionLevel::Standard,
            service: None,
            port: None,
            protocol: None,
            metadata: HashMap::with_capacity(16),
            resource_type: String::with_capacity(AssetCriticality::Medium,
            ip_address: None,
            hostname: Some(String::with_capacity(64)),
        }
    }
}

#[derive(Debug, Clone, PartialEq, Eq, Hash, PartialOrd, Ord, Serialize, Deserialize)]
pub enum SourceClassification {
    /// State indicating trusted
    Trusted,
    /// Represents neutral variant
    Neutral,
    /// Represents suspicious variant
    Suspicious,
    /// Represents malicious variant
    Malicious,
    /// Represents known malicious variant
    KnownMalicious,
    /// State indicating blocklisted
    Blocklisted,
    /// Unknown or undefined state
    Unknown,
    /// Represents external variant
    External,
    /// Represents internal variant
    Internal,
    /// Represents hostile variant
    Hostile,
}

impl Default for SourceClassification {
    fn default() -> Self {
        Self::Unknown
    }
}

#[derive(Debug, Clone, PartialEq, Eq, Hash, PartialOrd, Ord, Serialize, Deserialize)]
pub enum AssetCriticality {
    /// Represents low variant
    Low,
    /// Represents medium variant
    Medium,
    /// Represents high variant
    High,
    /// Represents critical variant
    Critical,
    /// Represents mission variant
    Mission,
}

impl Default for AssetCriticality {
    fn default() -> Self {
        Self::Medium
    }
}

#[derive(Debug, Clone, PartialEq, Eq, Hash, PartialOrd, Ord, Serialize, Deserialize)]
pub enum ProtectionLevel {
    /// No none specified
    None,
    /// Represents basic variant
    Basic,
    /// Represents standard variant
    Standard,
    /// State indicating enhanced
    Enhanced,
    /// Represents maximum variant
    Maximum,
}

impl Default for ProtectionLevel {
    fn default() -> Self {
        Self::Standard
    }
}

#[derive(Debug, Clone)]
    /// Optional region
    pub region: Option<String>,
    /// Optional city
    pub city: Option<String>,
    /// Optional latitude
    pub latitude: Option<f64>,
    /// Optional longitude
    pub longitude: Option<f64>,
    /// Whether is_tor_exit is enabled
    pub is_tor_exit: bool,
    /// Whether is_vpn is enabled
    pub is_vpn: bool,
    /// Whether is_proxy is enabled
    pub is_proxy: bool,
}

impl Default for GeoLocation {
    fn default() -> Self {
        Self {
            country: "Unknown".to_string() -> Self {
        self.classification = classification;
        self
    }

    /// With Geo Location operation.
    /// Creates instance with geo location
    pub fn with_geo_location(mut self, geo_location: GeoLocation) -> Self {
        self.geolocation = Some(geo_location);
        self
    }

    /// With Confidence operation.
    /// Creates instance with confidence
    pub fn with_confidence(mut self, confidence: f64) -> Self {
        self.confidence_score = confidence.clamp(&str, value: serde_json::Value) {
        self.metadata.insert(key.to_string(), value);
    }

    /// Is Trusted operation.
    /// Checks if trusted
    pub fn is_trusted(&self) -> bool {
        matches!(self.classification, SourceClassification::Trusted)
    }

    /// Is External operation.
    /// Checks if external
    pub fn is_external(&self) -> bool {
        matches!(self.classification, SourceClassification::External)
    }

    /// Is High Threat operation.
    /// Checks if high threat
    pub fn is_high_threat(&self) -> bool {
        matches!(
            self.classification,
            SourceClassification::Hostile | SourceClassification::Malicious
        )
    }

    /// Is Malicious operation.
    /// Checks if malicious
    pub fn is_malicious(&self) -> bool {
        matches!(self.classification, SourceClassification::Malicious)
    }

    /// Is Trustworthy operation.
    /// Checks if trustworthy
    pub fn is_trustworthy(&self) -> bool {
        self.is_trusted()
    }

    /// Hours Since Last Seen operation.
    pub fn hours_since_last_seen(&self) -> i64 {
        let now = Utc::now(&str, target_type: &str) -> Self {
        Self {
            id: id.to_string(),
            target_type: target_type.to_string(),
            resource_id: String::with_capacity(None,
            user_account: None,
            asset_criticality: AssetCriticality::Medium,
            protection_level: ProtectionLevel::Standard,
            service: None,
            port: None,
            protocol: None,
            metadata: HashMap::with_capacity(16),
            resource_type: String::with_capacity(AssetCriticality::Medium,
            ip_address: None,
            hostname: None,
        }
    }

    /// With Criticality operation.
    /// Creates instance with criticality
    pub fn with_criticality(mut self, criticality: AssetCriticality) -> Self {
        self.asset_criticality = criticality;
        self
    }

    /// With Protection Level operation.
    /// Creates instance with protection level
    pub fn with_protection_level(mut self, level: ProtectionLevel) -> Self {
        self.protection_level = level;
        self
    }

    /// Add Metadata operation.
    pub fn add_metadata(&str, value: serde_json::Value) {
        self.metadata.insert(key.to_string(), value);
    }

    /// Is Critical operation.
    /// Checks if critical
    pub fn is_critical(&self) -> bool {
        matches!(
            self.asset_criticality,
            AssetCriticality::Critical | AssetCriticality::Mission
        )
    }

    /// Is Well Protected operation.
    /// Checks if well protected
    pub fn is_well_protected(&self) -> bool {
        matches!(
            self.protection_level,
            ProtectionLevel::Enhanced | ProtectionLevel::Maximum
        )
    }

    /// Is High Value operation.
    /// Checks if high value
    pub fn is_high_value(&self) -> bool {
        self.is_critical()
    }

    /// Risk Score operation.
    pub fn risk_score(&self) -> f64 {
        let criticality_score = self.asset_criticality.risk_score();
        let protection_score = self.protection_level.protection_score();
        criticality_score * (1.0 - protection_score)
    }
}

impl GeoLocation {
    /// New operation.
    /// Creates a new instance
    pub fn new(country: &str) -> Self {
        Self {
            country: country.to_string(),
        }
    }

    /// Is High Risk Country operation.
    /// Checks if high risk country
    pub fn is_high_risk_country(&self) -> bool {
        matches!(self.country.as_str(), "CN" | "RU" | "KP" | "IR" | "Unknown")
    }

    /// Is Anonymized operation.
    /// Checks if anonymized
    pub fn is_anonymized(&self) -> bool {
        self.is_tor_exit || self.is_vpn || self.is_proxy
    }

    /// Risk Score operation.
    pub fn risk_score(&self) -> f64 {
        let mut score: f64 = 0.0;

        if self.is_high_risk_country() {
            score += 0.3;
        }

        if self.is_anonymized() {
            score += 0.4;
        }

        score.min(1.0)
    }
}

impl SourceClassification {
    /// Risk Score operation.
    pub fn risk_score(&self) -> f64 {
        match self {
            SourceClassification::Trusted => 0.1,
            SourceClassification::Neutral => 0.3,
            SourceClassification::Suspicious => 0.6,
            SourceClassification::Malicious => 0.8,
            SourceClassification::KnownMalicious => 0.9,
            SourceClassification::Blocklisted => 1.0,
            SourceClassification::Unknown => 0.5,
            SourceClassification::External => 0.4,
            SourceClassification::Internal => 0.2,
            SourceClassification::Hostile => 0.95,
        }
    }
}

impl AssetCriticality {
    /// Criticality Score operation.
    pub fn criticality_score(&self) -> f64 {
        match self {
            AssetCriticality::Mission => 1.0,
            AssetCriticality::Critical => 0.8,
            AssetCriticality::High => 0.6,
            AssetCriticality::Medium => 0.4,
            AssetCriticality::Low => 0.2,
        }
    }

    /// Risk Score operation.
    pub fn risk_score(&self) -> f64 {
        self.criticality_score()
    }
}

impl ProtectionLevel {
    /// Protection Score operation.
    pub fn protection_score(&self) -> f64 {
        match self {
            ProtectionLevel::None => 0.0,
            ProtectionLevel::Basic => 0.25,
            ProtectionLevel::Standard => 0.5,
            ProtectionLevel::Enhanced => 0.75,
            ProtectionLevel::Maximum => 1.0,
        }
    }
}

impl std::fmt::Display for SourceClassification {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            SourceClassification::Trusted => write!(f, "Trusted"),
            SourceClassification::Neutral => write!(f, "Neutral"),
            SourceClassification::Suspicious => write!(f, "Suspicious"),
            SourceClassification::Malicious => write!(f, "Malicious"),
            SourceClassification::KnownMalicious => write!(f, "Known Malicious"),
            SourceClassification::Blocklisted => write!(f, "Blocklisted"),
            SourceClassification::Unknown => write!(f, "Unknown"),
            SourceClassification::External => write!(f, "External"),
            SourceClassification::Internal => write!(f, "Internal"),
            SourceClassification::Hostile => write!(f, "Hostile"),
        }
    }
}

impl std::fmt::Display for AssetCriticality {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            AssetCriticality::Low => write!(f, "Low"),
            AssetCriticality::Medium => write!(f, "Medium"),
            AssetCriticality::High => write!(f, "High"),
            AssetCriticality::Critical => write!(f, "Critical"),
            AssetCriticality::Mission => write!(f, "Mission-Critical"),
        }
    }
}

impl std::fmt::Display for ProtectionLevel {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            ProtectionLevel::None => write!(f, "None"),
            ProtectionLevel::Basic => write!(f, "Basic"),
            ProtectionLevel::Standard => write!(f, "Standard"),
            ProtectionLevel::Enhanced => write!(f, "Enhanced"),
            ProtectionLevel::Maximum => write!(f, "Maximum"),
        }
    }
}
