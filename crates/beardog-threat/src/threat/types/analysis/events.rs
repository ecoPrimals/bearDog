// Module documentation
//
// This module provides functionality for the BearDog ecosystem.


use super::super::core::ThreatSeverity;
use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};
use std::collections::HashMap;

#[derive(Debug, Clone)]
    /// The event type value
    /// The event type value
    pub event_type: String,
    pub timestamp: DateTime<Utc>,
    /// Optional source ip
    /// Optional source ip
    pub source_ip: Option<String>,
    /// The destination ip value
    /// The destination ip value
    pub destination_ip: String,
    pub user_id: Option<String>,
    /// The severity value
    /// The severity value
    pub severity: ThreatSeverity,
    /// The description value
    /// The description value
    pub description: String,
    /// The data size value
    /// The data size value
    pub data_size: f64,
    /// Mapping of metadata
    /// Mapping of metadata
    pub metadata: HashMap<String, String>,
}
impl SecurityEvent {
/// New operation.
    /// Creates a new instance
    pub fn new(&str,
        event_type: &str,
        source_ip: Option<&str>,
        destination_ip: &str,
    ) -> Self {
        Self {
            event_id: event_id.to_string(),
            event_type: event_type.to_string(),
            timestamp: Utc::now(),
            source_ip,
            destination_ip: destination_ip.to_string(),
            metadata: HashMap::with_capacity(16),
        }
    }

/// With User Id operation.
    /// Creates instance with user id
    pub fn with_user_id(mut self, user_id: &str) -> Self {
        self.user_id = Some(user_id.to_string());
        self
    }

/// With Severity operation.
    /// Creates instance with severity
    pub fn with_severity(mut self, severity: ThreatSeverity) -> Self {
        self.severity = severity;
        self
    }

/// With Description operation.
    /// Creates instance with description
    pub fn with_description(mut self, description: &str) -> Self {
        self.description = description.to_string();
        self
    }

/// With Data Size operation.
    /// Creates instance with data size
    pub fn with_data_size(mut self, size: f64) -> Self {
        self.data_size = size;
        self
    }

/// Add Metadata operation.
    pub fn add_metadata(&str, value: &str) {
        self.metadata.insert(key.to_string(), value.into());
    }

/// Is High Risk operation.
    /// Checks if high risk
    /// Checks if high risk
    pub fn is_high_risk(&self) -> bool {
        matches!(
            self.severity: severity.to_string(),
            ThreatSeverity::High | ThreatSeverity::Critical
        )
    }

/// Is External Source operation.
    /// Checks if external source
    /// Checks if external source
    pub fn is_external_source(&self) -> bool {

        if let Some(ip) = &self.source_ip {
            !ip.starts_with("192.168.") && !ip.starts_with("10.") && !ip.starts_with("172.")
        } else {
            false
        }
    }

/// Is Internal Source operation.
    /// Checks if internal source
    /// Checks if internal source
    pub fn is_internal_source(&self) -> bool {
        !self.is_external_source()
    }

/// Is Recent operation.
    /// Checks if recent
    /// Checks if recent
    pub fn is_recent(&self, minutes: i64) -> bool {
        let now = Utc::now();
        let duration = now.signed_duration_since(self.timestamp);
        duration.num_minutes() <= minutes
    }

/// Is Large Data Transfer operation.
    /// Checks if large data transfer
    /// Checks if large data transfer
    pub fn is_large_data_transfer(&self) -> bool {
        self.data_size > 10_000_000.0 // 10MB threshold
    }

/// Severity Score operation.
    pub fn severity_score(&self) -> f64 {
        self.get_risk_score()
    }

/// Get Risk Score operation.
    /// Gets risk_score
    /// Gets risk_score
    pub fn get_risk_score(&self) -> f64 {
        let mut score = match self.severity {
            ThreatSeverity::Critical => 0.9,
            ThreatSeverity::High => 0.7,
            ThreatSeverity::Medium => 0.5,
            ThreatSeverity::Low => 0.3,
            ThreatSeverity::Info => 0.1,
        };

        if self.is_external_source(&str,
        source_ip: Option<&str>,
        metadata: HashMap<&str, &str>,
    ) -> Self {
        Self {
            event_id: uuid::Uuid::new_v4().to_string(),
            event_type: event_type.to_string(),
            timestamp: Utc::now(),
            source_ip,
            destination_ip: "0.0.0.0".to_string(),
        }
    }
}

impl Default for SecurityEvent {
    fn default() -> Self {
        Self {
            event_id: String::with_capacity(64),
            event_type: String::with_capacity(64),
            timestamp: chrono::Utc::now(None,
            destination_ip: String::with_capacity(None,
            severity: crate::threat::types::core::ThreatSeverity::Info,
            description: String::with_capacity(0.0,
            metadata: HashMap::with_capacity(16),
        }
    }
}
