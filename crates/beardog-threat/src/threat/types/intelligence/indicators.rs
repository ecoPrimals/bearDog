

use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use uuid::Uuid;

// Import from enums module to avoid duplication
use super::enums::{IndicatorType, ConfidenceLevel};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ThreatIndicator {
    pub indicator_id: String,
    pub indicator_type: IndicatorType,
    pub value: String,
    pub confidence: ConfidenceLevel,
    pub first_seen: DateTime<Utc>,
    pub last_seen: DateTime<Utc>,
    pub tags: Vec<String>,
    pub metadata: HashMap<String, String>,
    pub threat_types: Vec<String>,
}

impl Default for ThreatIndicator {
    fn default() -> Self {
        Self {
            indicator_id: uuid::Uuid::new_v4().to_string(),
            indicator_type: super::enums::IndicatorType::IpAddress,
            value: String::new(),
            confidence: super::enums::ConfidenceLevel::Medium,
            first_seen: chrono::Utc::now(),
            last_seen: chrono::Utc::now(),
            tags: Vec::new(),
            source: String::new(),
            metadata: HashMap::new(),
            threat_types: vec![],
        }
    }
}
impl ThreatIndicator {

    pub fn new(indicator_type: IndicatorType, value: &str, confidence: f64) -> Self {
        Self {
            indicator_id: Uuid::new_v4().to_string(),
            indicator_type,
            value: value.to_string(),
            confidence: Self::f64_to_confidence_level(confidence),
            first_seen: Utc::now(),
            last_seen: Utc::now(),
            tags: Vec::new(),
            metadata: HashMap::new(),
            threat_types: vec![],
        }
    }

    fn f64_to_confidence_level(confidence: f64) -> ConfidenceLevel {
        match confidence {
            x if x >= 0.8 => ConfidenceLevel::Critical,
            x if x >= 0.6 => ConfidenceLevel::High,
            x if x >= 0.4 => ConfidenceLevel::Medium,
            _ => ConfidenceLevel::Low,
        }
    }

    pub fn add_threat_type(&mut self, threat_type: &str) {
        if !self.threat_types.contains(&threat_type.to_string()) {
            self.threat_types.push(threat_type.to_string());
        }
    }

    pub fn add_tag(&mut self, tag: &str) {
        let tag_string = tag.to_string();
        if !self.tags.contains(&tag_string) {
            self.tags.push(tag_string);
        }
    }

    pub fn update_last_seen(&mut self) {
        self.last_seen = Utc::now();
    }

    pub fn is_high_confidence(&self) -> bool {
        matches!(self.confidence, ConfidenceLevel::High | ConfidenceLevel::Critical)
    }

    pub fn is_recent(&self, hours: i64) -> bool {
        let now = Utc::now();
        (now - self.last_seen).num_hours() <= hours
    }

    pub fn age_hours(&self) -> i64 {
        (Utc::now() - self.first_seen).num_hours()
    }
}
impl IndicatorType {

    pub fn is_network_related(&self) -> bool {
        matches!(
            self,
            IndicatorType::IpAddress
                | IndicatorType::DomainName
                | IndicatorType::Url
                | IndicatorType::NetworkPattern
        )
    }

    pub fn is_file_related(&self) -> bool {
        matches!(
            self,
            IndicatorType::FileHash
                | IndicatorType::ProcessName
                | IndicatorType::RegistryKey
                | IndicatorType::Mutex
        )
    }

    pub fn validation_pattern(&self) -> &'static str {
        match self {
            IndicatorType::IpAddress => r"^(?:[0-9]{1,3}\.){3}[0-9]{1,3}$",
            IndicatorType::DomainName => {
                r"^[a-zA-Z0-9]([a-zA-Z0-9\-]{0,61}[a-zA-Z0-9])?(\.[a-zA-Z0-9]([a-zA-Z0-9\-]{0,61}[a-zA-Z0-9])?)*$"
            }
            IndicatorType::Url => r"^https?://[^\s/$.?#].[^\s]*$",
            IndicatorType::FileHash => r"^[a-fA-F0-9]{32}$|^[a-fA-F0-9]{40}$|^[a-fA-F0-9]{64}$",
            IndicatorType::EmailAddress => r"^[a-zA-Z0-9._%+-]+@[a-zA-Z0-9.-]+\.[a-zA-Z]{2,}$",
            _ => r".*", // Default: accept anything
        }
    }
}
