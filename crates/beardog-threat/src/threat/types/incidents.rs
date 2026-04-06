// SPDX-License-Identifier: AGPL-3.0-or-later

// Incident management types for threat handling

use serde::{Deserialize, Serialize};
use std::time::SystemTime;

use super::{ThreatEvent, ThreatSeverity, ThreatType};

/// Phase in the handler-driven incident lifecycle (create → classify → escalate → resolve → close).
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
pub enum IncidentLifecyclePhase {
    /// Record created from a [`ThreatEvent`].
    Created,
    /// Category and impact triage completed.
    Classified,
    /// Ownership escalated (e.g. to a response team).
    Escalated,
    /// Work completed; optional [`ManagedIncident::resolution`] text captured.
    Resolved,
    /// Incident closed; read-only terminal state.
    Closed,
}

/// In-memory incident record with typed fields (Phase 1 store; not persisted to a database).
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ManagedIncident {
    /// Stable incident identifier (`INC-{uuid}`).
    pub id: String,
    /// Originating [`ThreatEvent::id`].
    pub threat_id: String,
    /// Severity copied from the threat at creation time.
    pub severity: ThreatSeverity,
    /// High-level category ([`ThreatType`]) from the source event.
    pub category: ThreatType,
    /// Human-readable description (typically from the threat).
    pub description: String,
    /// Current lifecycle phase.
    pub lifecycle: IncidentLifecyclePhase,
    /// Wall-clock time when the incident record was created.
    pub created_at: SystemTime,
    /// Last mutation time for any field on this record.
    pub updated_at: SystemTime,
    /// Time of transition to [`IncidentLifecyclePhase::Classified`], if reached.
    pub classified_at: Option<SystemTime>,
    /// Time of transition to [`IncidentLifecyclePhase::Escalated`], if reached.
    pub escalated_at: Option<SystemTime>,
    /// Time of transition to [`IncidentLifecyclePhase::Resolved`], if reached.
    pub resolved_at: Option<SystemTime>,
    /// Time of transition to [`IncidentLifecyclePhase::Closed`], if reached.
    pub closed_at: Option<SystemTime>,
    /// Resolution summary set when moving to [`IncidentLifecyclePhase::Resolved`].
    pub resolution: Option<String>,
    /// Team or queue owning the response, when assigned.
    pub assigned_team: Option<String>,
}

impl ManagedIncident {
    /// Builds a new managed incident in [`IncidentLifecyclePhase::Created`] from a [`ThreatEvent`].
    #[must_use]
    pub fn from_threat(incident_id: String, threat: &ThreatEvent) -> Self {
        let now = SystemTime::now();
        Self {
            id: incident_id,
            threat_id: threat.id.clone(),
            severity: threat.severity.clone(),
            category: threat.threat_type.clone(),
            description: threat.description.clone(),
            lifecycle: IncidentLifecyclePhase::Created,
            created_at: now,
            updated_at: now,
            classified_at: None,
            escalated_at: None,
            resolved_at: None,
            closed_at: None,
            resolution: None,
            assigned_team: None,
        }
    }

    /// Enforces the canonical forward-only lifecycle transitions.
    ///
    /// # Errors
    /// Returns [`beardog_errors::BearDogError`] when the transition is invalid for the current phase.
    pub fn try_advance_to(
        &mut self,
        next: IncidentLifecyclePhase,
    ) -> Result<(), beardog_errors::BearDogError> {
        use IncidentLifecyclePhase::{Classified, Closed, Created, Escalated, Resolved};
        let valid = matches!(
            (&self.lifecycle, &next),
            (Created, Classified)
                | (Classified, Escalated)
                | (Escalated, Resolved)
                | (Resolved, Closed)
        );
        if !valid {
            return Err(beardog_errors::BearDogError::business(format!(
                "invalid incident transition {:?} -> {:?}",
                self.lifecycle, next
            )));
        }
        let now = SystemTime::now();
        self.lifecycle = next;
        self.updated_at = now;
        match self.lifecycle {
            Classified => self.classified_at = Some(now),
            Escalated => self.escalated_at = Some(now),
            Resolved => self.resolved_at = Some(now),
            Closed => self.closed_at = Some(now),
            Created => {}
        }
        Ok(())
    }
}

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
    pub title: String,
    /// Incident description
    /// The description value
    pub description: String,
    /// Incident severity level
    /// The severity value
    pub severity: super::ThreatSeverity,
    /// Current incident status
    /// Current status of the component
    pub status: IncidentStatus,
    /// Incident creation timestamp
    /// The created at value
    pub created_at: SystemTime,
    /// Incident last update timestamp
    /// The updated at value
    pub updated_at: SystemTime,
    /// Assigned analyst or team
    /// Optional assigned to
    pub assigned_to: Option<String>,
    /// Related threat event IDs
    /// Collection of threat events
    pub threat_events: Vec<String>,
    /// Incident response actions
    /// Collection of response actions
    pub response_actions: Vec<String>,
    /// Incident notes and updates
    /// Collection of notes
    pub notes: Vec<String>,
}

impl SecurityIncident {
    /// Create a new security incident
    /// Creates a new instance
    #[must_use]
    pub fn new(
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
