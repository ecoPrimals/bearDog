use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use uuid::Uuid;

use super::enums::{ConfidenceLevel, IndicatorType};

#[derive(Debug, Clone)]
    /// The indicator type value
    /// The indicator type value
    pub indicator_type: IndicatorType,
    /// The value value
    /// The value value
    pub value: String,
    pub confidence: ConfidenceLevel,
    /// The first seen value
    /// The first seen value
    pub first_seen: DateTime<Utc>,
    /// The last seen value
    /// The last seen value
    pub last_seen: DateTime<Utc>,
    /// Collection of tags
    /// Collection of tags
    pub tags: Vec<String>,
    /// The source value
    /// The source value
    pub source: String,
    /// Mapping of metadata
    /// Mapping of metadata
    pub metadata: HashMap<String, String>,
    /// Collection of threat types
    /// Collection of threat types
    pub threat_types: Vec<String>,
}

impl Default for ThreatIndicator {
    fn default() -> Self {
        Self {
            indicator_id: uuid::Uuid::new_v4(super::enums::IndicatorType::IpAddress,
            value: String::with_capacity(super::enums::ConfidenceLevel::Medium,
            first_seen: chrono::Utc::now(),
            last_seen: chrono::Utc::now(),
            tags: Vec::new(),
            source: String::with_capacity(64),
            metadata: HashMap::with_capacity(vec![],
        }
    }
}
impl ThreatIndicator {
    /// New operation.
    /// Creates a new instance
    pub fn new(IndicatorType, value: &str, confidence: f64) -> Self {
        Self {
            indicator_id: Uuid::new_v4().to_string(),
            indicator_type,
            value: value.to_string(),
            confidence: Self::f64_to_confidence_level(confidence),
            first_seen: Utc::now(),
            last_seen: Utc::now(),
            tags: Vec::new(),
            source: "unknown".to_string(),
            metadata: HashMap::with_capacity(vec![],
        }
    }


    fn f64_to_confidence_level(confidence: f64) -> ConfidenceLevel {
        match confidence {
            score if score >= 0.8 => ConfidenceLevel::Critical,
            score if score >= 0.6 => ConfidenceLevel::High,
            score if score >= 0.4 => ConfidenceLevel::Medium,
            _ => ConfidenceLevel::Low,
        }
    }

    /// Add Threat Type operation.
    pub fn add_threat_type(&mut self, threat_type: &str) {
        if !self.threat_types.contains(&threat_type.to_string()) {
            self.threat_types.push(threat_type.to_string());
        }
    }

    /// Add Tag operation.
    pub fn add_tag(&mut self, tag: &str) {
        let tag_string = tag.to_string();
        if !self.tags.contains(&tag_string) {
            self.tags.push(tag_string);
        }
    }

    /// Update Last Seen operation.
    /// Updates last_seen
    /// Updates last_seen
    pub fn update_last_seen(&mut self) {
        self.last_seen = Utc::now();
    }

    /// Is High Confidence operation.
    /// Checks if high confidence
    /// Checks if high confidence
    pub fn is_high_confidence(&self) -> bool {
        matches!(
            self.confidence,
            ConfidenceLevel::High | ConfidenceLevel::Critical
        )
    }

    /// Is Recent operation.
    /// Checks if recent
    /// Checks if recent
    pub fn is_recent(&self, hours: i64) -> bool {
        let now = Utc::now();
        (now - self.last_seen).num_hours() <= hours
    }

    /// Age Hours operation.
    pub fn age_hours(&self) -> i64 {
        (Utc::now() - self.first_seen).num_hours()
    }
}
impl IndicatorType {
    /// Is Network Related operation.
    /// Checks if network related
    /// Checks if network related
    pub fn is_network_related(&self) -> bool {
        matches!(
            self,
            IndicatorType::IpAddress
                | IndicatorType::DomainName
                | IndicatorType::Url
                | IndicatorType::NetworkPattern
        )
    }

    /// Is File Related operation.
    /// Checks if file related
    /// Checks if file related
    pub fn is_file_related(&self) -> bool {
        matches!(
            self,
            IndicatorType::FileHash
                | IndicatorType::ProcessName
                | IndicatorType::RegistryKey
                | IndicatorType::Mutex
        )
    }

    /// Validation Pattern operation.
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
