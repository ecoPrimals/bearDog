use super::super::core::ThreatSeverity;
use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};
use std::collections::HashMap;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SecurityEvent {
    pub event_id: String,
    pub event_type: String,
    pub timestamp: DateTime<Utc>,
    pub source_ip: Option<String>,
    pub destination_ip: String,
    pub user_id: Option<String>,
    pub severity: ThreatSeverity,
    pub description: String,
    pub data_size: f64,
    pub metadata: HashMap<String, String>,
}
impl SecurityEvent {
    pub fn new(
        event_id: &str,
        event_type: &str,
        source_ip: Option<String>,
        destination_ip: &str,
    ) -> Self {
        Self {
            event_id: event_id.to_string(),
            event_type: event_type.to_string(),
            timestamp: Utc::now(),
            source_ip,
            destination_ip: destination_ip.to_string(),
            user_id: None,
            severity: ThreatSeverity::Info,
            description: String::new(),
            data_size: 0.0,
            metadata: HashMap::new(),
        }
    }

    pub fn with_user_id(mut self, user_id: &str) -> Self {
        self.user_id = Some(user_id.to_string());
        self
    }

    pub fn with_severity(mut self, severity: ThreatSeverity) -> Self {
        self.severity = severity;
        self
    }

    pub fn with_description(mut self, description: &str) -> Self {
        self.description = description.to_string();
        self
    }

    pub fn with_data_size(mut self, size: f64) -> Self {
        self.data_size = size;
        self
    }

    pub fn add_metadata(&mut self, key: &str, value: &str) {
        self.metadata.insert(key.to_string(), value.to_string());
    }

    pub fn is_high_risk(&self) -> bool {
        matches!(
            self.severity,
            ThreatSeverity::High | ThreatSeverity::Critical
        )
    }

    pub fn is_external_source(&self) -> bool {
        // Simple check for external IPs (not comprehensive)
        if let Some(ip) = &self.source_ip {
            !ip.starts_with("192.168.") && !ip.starts_with("10.") && !ip.starts_with("172.")
        } else {
            false
        }
    }

    pub fn is_internal_source(&self) -> bool {
        !self.is_external_source()
    }

    pub fn is_recent(&self, minutes: i64) -> bool {
        let now = Utc::now();
        let duration = now.signed_duration_since(self.timestamp);
        duration.num_minutes() <= minutes
    }

    pub fn is_large_data_transfer(&self) -> bool {
        self.data_size > 10_000_000.0 // 10MB threshold
    }

    pub fn severity_score(&self) -> f64 {
        self.get_risk_score()
    }

    pub fn get_risk_score(&self) -> f64 {
        let mut score = match self.severity {
            ThreatSeverity::Critical => 0.9,
            ThreatSeverity::High => 0.7,
            ThreatSeverity::Medium => 0.5,
            ThreatSeverity::Low => 0.3,
            ThreatSeverity::Info => 0.1,
        };

        if self.is_external_source() {
            score += 0.1;
        }

        // Normalize data size to reasonable range
        let normalized_size = (self.data_size / 1024.0_f64).min(1000.0_f64);
        if normalized_size > 100.0_f64 {
            // > 100MB
            score += 0.1_f64;
        }

        score.min(1.0_f64)
    }

    pub fn new_with_data(
        event_type: &str,
        source_ip: Option<String>,
        metadata: HashMap<String, String>,
    ) -> Self {
        Self {
            event_id: uuid::Uuid::new_v4().to_string(),
            event_type: event_type.to_string(),
            timestamp: Utc::now(),
            source_ip,
            destination_ip: "0.0.0.0".to_string(),
            user_id: None,
            severity: ThreatSeverity::Info,
            description: String::new(),
            data_size: 0.0,
            metadata,
        }
    }
}

impl Default for SecurityEvent {
    fn default() -> Self {
        Self {
            event_id: String::new(),
            event_type: String::new(),
            timestamp: chrono::Utc::now(),
            source_ip: None,
            destination_ip: String::new(),
            user_id: None,
            severity: crate::threat::types::core::ThreatSeverity::Info,
            description: String::new(),
            data_size: 0.0,
            metadata: HashMap::new(),
        }
    }
}
