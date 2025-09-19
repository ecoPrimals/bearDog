// Incident management types for threat handling

use serde::{Deserialize, Serialize};
use std::time::SystemTime;

/// Status of a security incident
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
pub enum IncidentStatus {
    /// Incident reported but not yet triaged
    Reported,
    /// Incident is being investigated
    Investigating,
    /// Incident confirmed as valid
    Confirmed,
    /// Incident is being contained
    Containing,
    /// Incident has been contained
    Contained,
    /// Incident is being remediated
    Remediating,
    /// Incident has been resolved
    Resolved,
    /// Incident was a false positive
    FalsePositive,
    /// Incident was closed without resolution
    Closed,
}

/// Security incident details
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SecurityIncident {
    /// Unique incident identifier
    pub id: String,
    /// Incident title
    /// The title value
    /// The title value
    pub title: String,
    /// Incident description
    /// The description value
    /// The description value
    pub description: String,
    /// Incident severity level
    /// The severity value
    /// The severity value
    pub severity: super::ThreatSeverity,
    /// Current incident status
    /// Current status of the component
    /// Current status of the component
    pub status: IncidentStatus,
    /// Incident creation timestamp
    /// The created at value
    /// The created at value
    pub created_at: SystemTime,
    /// Incident last update timestamp
    /// The updated at value
    /// The updated at value
    pub updated_at: SystemTime,
    /// Assigned analyst or team
    /// Optional assigned to
    /// Optional assigned to
    pub assigned_to: Option<String>,
    /// Related threat event IDs
    /// Collection of threat events
    /// Collection of threat events
    pub threat_events: Vec<String>,
    /// Incident response actions
    /// Collection of response actions
    /// Collection of response actions
    pub response_actions: Vec<String>,
    /// Incident notes and updates
    /// Collection of notes
    /// Collection of notes
    pub notes: Vec<String>,
}

impl SecurityIncident {
    /// Create a new security incident
    /// Creates a new instance
    #[must_use] pub fn new(
        id: String,
        title: String,
        description: String,
        severity: super::ThreatSeverity,
    ) -> Self {
        Self {
            id,
            title,
            description,
            severity,
            status: IncidentStatus::Reported,
            created_at: SystemTime::now(),
            updated_at: SystemTime::now(),
            assigned_to: None,
            threat_events: Vec::new(),
            response_actions: Vec::new(),
            notes: Vec::new(),
        }
    }

    /// Update incident status
    /// Updates status
    /// Updates status
    pub fn update_status(&mut self, status: IncidentStatus) {
        self.status = status;
        self.updated_at = SystemTime::now();
    }

    /// Assign incident to analyst or team
    pub fn assign_to(&mut self, assignee: String) {
        self.assigned_to = Some(assignee);
        self.updated_at = SystemTime::now();
    }

    /// Add a note to the incident
    pub fn add_note(&mut self, note: String) {
        self.notes.push(note);
        self.updated_at = SystemTime::now();
    }

    /// Link a threat event to this incident
    pub fn link_threat_event(&mut self, threat_id: String) {
        if !self.threat_events.contains(&threat_id) {
            self.threat_events.push(threat_id);
            self.updated_at = SystemTime::now();
        }
    }
}
