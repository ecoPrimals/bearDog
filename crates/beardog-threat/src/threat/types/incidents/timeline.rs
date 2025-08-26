

use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use uuid::Uuid;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct IncidentTimelineEntry {
    pub entry_id: String,
    pub incident_id: String,
    pub timestamp: DateTime<Utc>,
    pub action: String,
    pub actor: String,
    pub entry_type: TimelineEntryType,
    pub event_type: TimelineEntryType, // Alias for entry_type
    pub description: String,
    pub details: Option<String>,
    pub metadata: HashMap<String, String>,
}

#[derive(Debug, Clone, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub enum TimelineEntryType {
    Created,
    Detection,
    Investigation,
    Containment,
    Remediation,
    Communication,
    StatusChange,
    ExternalCoordination,
    Administrative,
}

impl Default for IncidentTimelineEntry {
    fn default() -> Self {
        Self {
            entry_id: uuid::Uuid::new_v4().to_string(),
            incident_id: String::new(),
            timestamp: chrono::Utc::now(),
            action: String::new(),
            actor: String::new(),
            entry_type: TimelineEntryType::Created,
            event_type: TimelineEntryType::Created,
            description: String::new(),
            details: None,
            metadata: HashMap::new(),
        }
    }
}

impl IncidentTimelineEntry {

    pub fn new(action: &str, actor: &str) -> Self {
        Self {
            entry_id: Uuid::new_v4().to_string(),
            incident_id: String::new(), // Will be set when added to incident
            event_type: action.to_string(),
            description: actor.to_string(),
            timestamp: Utc::now(),
            metadata: HashMap::new(),
        }
    }

    pub fn with_details(mut self, details: &str) -> Self {
        self.metadata.insert("details".to_string(), details.to_string());
        self
    }

    pub fn set_details(&mut self, details: &str) {
        self.metadata.insert("details".to_string(), details.to_string());
    }

    pub fn age_minutes(&self) -> i64 {
        let now = Utc::now();
        (now - self.timestamp).num_minutes()
    }
}

impl std::fmt::Display for TimelineEntryType {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            TimelineEntryType::Created => write!(f, "Created"),
            TimelineEntryType::Detection => write!(f, "Detection"),
            TimelineEntryType::Investigation => write!(f, "Investigation"),
            TimelineEntryType::Containment => write!(f, "Containment"),
            TimelineEntryType::Remediation => write!(f, "Remediation"),
            TimelineEntryType::Communication => write!(f, "Communication"),
            TimelineEntryType::StatusChange => write!(f, "Status Change"),
            TimelineEntryType::ExternalCoordination => write!(f, "External Coordination"),
            TimelineEntryType::Administrative => write!(f, "Administrative"),
        }
    }
}

