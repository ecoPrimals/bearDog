// SPDX-License-Identifier: AGPL-3.0-or-later

//! Normalized security telemetry prior to enrichment into threat events.

use super::taxonomy::ThreatSeverity;
use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use std::time::SystemTime;

/// Normalized security telemetry row before enrichment into a threat event.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SecurityEvent {
    /// Event identifier
    pub id: String,
    /// Event type
    /// The event type value
    pub event_type: String,
    /// Event timestamp
    pub timestamp: SystemTime,
    /// Event severity
    /// The severity value
    pub severity: ThreatSeverity,
    /// Event source
    /// The source value
    pub source: String,
    /// Event description
    /// The description value
    pub description: String,
    /// Additional event data
    /// Mapping of data
    pub data: HashMap<String, String>,
}

impl SecurityEvent {
    /// Create a new security event
    /// Creates a new instance
    #[must_use]
    pub fn new(event_type: &str, timestamp: chrono::DateTime<chrono::Utc>, source: &str) -> Self {
        Self {
            id: uuid::Uuid::new_v4().to_string(),
            event_type: event_type.to_string(),
            timestamp: timestamp.into(),
            severity: ThreatSeverity::Low,
            source: source.to_string(),
            description: String::new(),
            data: HashMap::new(),
        }
    }

    /// Add source IP to event data
    /// Creates instance with source ip
    #[must_use]
    pub fn with_source_ip(mut self, source_ip: &str) -> Self {
        self.data
            .insert("source_ip".to_string(), source_ip.to_string());
        self
    }

    /// Add user ID to event data
    /// Creates instance with user id
    #[must_use]
    pub fn with_user_id(mut self, user_id: &str) -> Self {
        self.data.insert("user_id".to_string(), user_id.to_string());
        self
    }
}
