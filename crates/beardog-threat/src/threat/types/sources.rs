

use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};
use std::collections::HashMap;

use super::core::ThreatSeverity;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ThreatSource {
    pub id: String,
    pub source_type: String,
    pub ip_address: Option<String>,
    pub hostname: Option<String>,
    pub geolocation: Option<GeoLocation>,
    pub user_agent: Option<String>,
    pub reputation_score: f64,
    pub threat_actor: Option<String>,
    pub classification: SourceClassification,
    pub confidence_score: f64,
    pub first_seen: Option<DateTime<Utc>>,
    pub last_seen: Option<DateTime<Utc>>,
    pub threat_score: f64,
    pub metadata: HashMap<String, serde_json::Value>,
}

impl Default for ThreatSource {
    fn default() -> Self {
        Self {
            id: String::new(),
            source_type: String::new(),
            ip_address: None,
            hostname: None,
            geolocation: None,
            user_agent: None,
            reputation_score: 0.5,
            threat_actor: None,
            classification: SourceClassification::Unknown,
            confidence_score: 0.5,
            first_seen: None,
            last_seen: Some(chrono::Utc::now()),
            threat_score: 0.0,
            metadata: HashMap::new(),
        }
    }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ThreatTarget {
    pub id: String,
    pub target_type: String,
    pub resource_id: String,
    pub node_id: Option<String>,
    pub user_account: Option<String>,
    pub asset_criticality: AssetCriticality,
    pub protection_level: ProtectionLevel,
    pub service: Option<String>,
    pub port: Option<u16>,
    pub protocol: Option<String>,
    pub metadata: HashMap<String, serde_json::Value>,
    // Additional fields referenced in handlers
    pub resource_type: String,
    pub criticality: AssetCriticality,
    pub ip_address: Option<String>,
    pub hostname: Option<String>,
}

impl Default for ThreatTarget {
    fn default() -> Self {
        Self {
            id: String::new(),
            target_type: String::new(),
            resource_id: String::new(),
            node_id: None,
            user_account: None,
            asset_criticality: AssetCriticality::Medium,
            protection_level: ProtectionLevel::Standard,
            service: None,
            port: None,
            protocol: None,
            metadata: HashMap::new(),
            resource_type: String::new(),
            criticality: AssetCriticality::Medium,
            ip_address: None,
            hostname: None,
        }
    }
}

#[derive(Debug, Clone, PartialEq, Eq, Hash, PartialOrd, Ord, Serialize, Deserialize)]
pub enum SourceClassification {
    Trusted,
    Neutral,
    Suspicious,
    Malicious,
    KnownMalicious,
    Blacklisted,
    Unknown,
    External,
    Internal,
    Hostile,
}

impl Default for SourceClassification {
    fn default() -> Self {
        Self::Unknown
    }
}

#[derive(Debug, Clone, PartialEq, Eq, Hash, PartialOrd, Ord, Serialize, Deserialize)]
pub enum AssetCriticality {
    Low,
    Medium,
    High,
    Critical,
    Mission,
}

impl Default for AssetCriticality {
    fn default() -> Self {
        Self::Medium
    }
}

#[derive(Debug, Clone, PartialEq, Eq, Hash, PartialOrd, Ord, Serialize, Deserialize)]
pub enum ProtectionLevel {
    None,
    Basic,
    Standard,
    Enhanced,
    Maximum,
}

impl Default for ProtectionLevel {
    fn default() -> Self {
        Self::Standard
    }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct GeoLocation {
    pub country: String,
    pub region: Option<String>,
    pub city: Option<String>,
    pub latitude: Option<f64>,
    pub longitude: Option<f64>,
    pub is_tor_exit: bool,
    pub is_vpn: bool,
    pub is_proxy: bool,
}

impl Default for GeoLocation {
    fn default() -> Self {
        Self {
            country: "Unknown".to_string(),
            region: None,
            city: None,
            latitude: None,
            longitude: None,
            is_tor_exit: false,
            is_vpn: false,
            is_proxy: false,
        }
    }
}

impl ThreatSource {
    pub fn new(id: String, source_type: String) -> Self {
        Self {
            id,
            source_type,
            ip_address: None,
            hostname: None,
            geolocation: None,
            user_agent: None,
            reputation_score: 0.5,
            threat_actor: None,
            classification: SourceClassification::Suspicious,
            confidence_score: 0.5,
            first_seen: None,
            last_seen: Some(chrono::Utc::now()),
            threat_score: 0.0,
            metadata: HashMap::new(),
        }
    }

    pub fn with_classification(mut self, classification: SourceClassification) -> Self {
        self.classification = classification;
        self
    }

    pub fn with_geo_location(mut self, geo_location: GeoLocation) -> Self {
        self.geolocation = Some(geo_location);
        self
    }

    pub fn with_confidence(mut self, confidence: f64) -> Self {
        self.confidence_score = confidence.clamp(0.0, 1.0);
        self
    }

    pub fn add_metadata(&mut self, key: String, value: serde_json::Value) {
        self.metadata.insert(key, value);
    }

    pub fn is_trusted(&self) -> bool {
        matches!(self.classification, SourceClassification::Trusted)
    }

    pub fn is_external(&self) -> bool {
        matches!(self.classification, SourceClassification::External)
    }

    pub fn is_high_threat(&self) -> bool {
        matches!(
            self.classification,
            SourceClassification::Hostile | SourceClassification::Malicious
        )
    }

    pub fn hours_since_last_seen(&self) -> i64 {
        let now = Utc::now();
        match self.last_seen {
            Some(last_seen) => (now - last_seen).num_hours(),
            None => -1, // Never seen
        }
    }
}

impl ThreatTarget {
    pub fn new(id: String, target_type: String) -> Self {
        Self {
            id,
            target_type,
            resource_id: String::new(),
            node_id: None,
            user_account: None,
            asset_criticality: AssetCriticality::Medium,
            protection_level: ProtectionLevel::Standard,
            service: None,
            port: None,
            protocol: None,
            metadata: HashMap::new(),
            resource_type: String::new(),
            criticality: AssetCriticality::Medium,
            ip_address: None,
            hostname: None,
        }
    }

    pub fn with_criticality(mut self, criticality: AssetCriticality) -> Self {
        self.asset_criticality = criticality;
        self
    }

    pub fn with_protection_level(mut self, level: ProtectionLevel) -> Self {
        self.protection_level = level;
        self
    }

    pub fn add_metadata(&mut self, key: String, value: serde_json::Value) {
        self.metadata.insert(key, value);
    }

    pub fn is_critical(&self) -> bool {
        matches!(
            self.asset_criticality,
            AssetCriticality::Critical | AssetCriticality::Mission
        )
    }

    pub fn is_well_protected(&self) -> bool {
        matches!(
            self.protection_level,
            ProtectionLevel::Enhanced | ProtectionLevel::Maximum
        )
    }
}

impl GeoLocation {
    pub fn new(country: &str) -> Self {
        Self {
            country: country.to_string(),
            region: None,
            city: None,
            latitude: None,
            longitude: None,
            is_tor_exit: false,
            is_vpn: false,
            is_proxy: false,
        }
    }

    pub fn is_high_risk_country(&self) -> bool {
        // List of countries commonly associated with cyber threats
        matches!(
            self.country.as_str(),
            "CN" | "RU" | "KP" | "IR" | "Unknown"
        )
    }

    pub fn is_anonymized(&self) -> bool {
        self.is_tor_exit || self.is_vpn || self.is_proxy
    }

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
    pub fn risk_score(&self) -> f64 {
        match self {
            SourceClassification::Trusted => 0.1,
            SourceClassification::Neutral => 0.3,
            SourceClassification::Suspicious => 0.6,
            SourceClassification::Malicious => 0.8,
            SourceClassification::KnownMalicious => 0.9,
            SourceClassification::Blacklisted => 1.0,
            SourceClassification::Unknown => 0.5,
            SourceClassification::External => 0.4,
            SourceClassification::Internal => 0.2,
            SourceClassification::Hostile => 0.95,
        }
    }
}

impl AssetCriticality {

    pub fn criticality_score(&self) -> f64 {
        match self {
            AssetCriticality::Mission => 1.0,
            AssetCriticality::Critical => 0.8,
            AssetCriticality::High => 0.6,
            AssetCriticality::Medium => 0.4,
            AssetCriticality::Low => 0.2,
        }
    }
}

impl ProtectionLevel {

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
            SourceClassification::Blacklisted => write!(f, "Blacklisted"),
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


