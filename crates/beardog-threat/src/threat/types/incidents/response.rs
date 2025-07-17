//! Incident response management
//!
//! This module contains the main IncidentResponse struct and related functionality
//! for managing security incidents throughout their lifecycle.

use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};

use super::status::IncidentStatus;
use crate::threat::types::core::ThreatSeverity;

/// Incident response structure
///
/// Represents a security incident with full lifecycle tracking,
/// from detection through resolution and lessons learned.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct IncidentResponse {
    /// Unique incident identifier
    pub incident_id: String,

    /// Associated threat identifier
    pub threat_id: String,

    /// Current incident severity
    pub severity: ThreatSeverity,

    /// Current incident status
    pub status: IncidentStatus,

    /// Incident creation timestamp
    pub created_at: DateTime<Utc>,

    /// Last update timestamp
    pub updated_at: DateTime<Utc>,

    /// Assigned team member or system
    pub assigned_to: Option<String>,

    /// Incident description
    pub description: String,

    /// Containment actions taken
    pub containment_actions: Vec<String>,

    /// Remediation actions taken
    pub remediation_actions: Vec<String>,

    /// Lessons learned from incident
    pub lessons_learned: Vec<String>,
}

impl Default for IncidentResponse {
    fn default() -> Self {
        Self {
            incident_id: String::new(),
            threat_id: String::new(),
            severity: ThreatSeverity::Medium,
            status: IncidentStatus::Open,
            created_at: Utc::now(),
            updated_at: Utc::now(),
            assigned_to: None,
            description: String::new(),
            containment_actions: vec![],
            remediation_actions: vec![],
            lessons_learned: vec![],
        }
    }
}

impl IncidentResponse {
    /// Create a new incident response
    ///
    /// # Arguments
    /// * `incident_id` - Unique incident identifier
    /// * `threat_id` - Associated threat identifier
    /// * `severity` - Incident severity
    /// * `description` - Incident description
    ///
    /// # Returns
    /// A new `IncidentResponse` instance
    ///
    /// # Example
    /// ```rust
    /// use beardog::threat::types::{IncidentResponse, ThreatSeverity};
    ///
    /// let incident = IncidentResponse::new(
    ///     "INC-2024-001".to_string(),
    ///     "threat-001".to_string(),
    ///     ThreatSeverity::High,
    ///     "Malware detected".to_string()
    /// );
    /// ```
    pub fn new(
        incident_id: String,
        threat_id: String,
        severity: ThreatSeverity,
        description: String,
    ) -> Self {
        Self {
            incident_id,
            threat_id,
            severity,
            status: IncidentStatus::Open,
            created_at: Utc::now(),
            updated_at: Utc::now(),
            assigned_to: None,
            description,
            containment_actions: vec![],
            remediation_actions: vec![],
            lessons_learned: vec![],
        }
    }

    /// Update incident status
    ///
    /// # Arguments
    /// * `status` - New incident status
    ///
    /// # Example
    /// ```rust
    /// use beardog::threat::types::{IncidentResponse, IncidentStatus};
    ///
    /// let mut incident = IncidentResponse::default();
    /// incident.update_status(IncidentStatus::InProgress);
    ///
    /// assert_eq!(incident.status, IncidentStatus::InProgress);
    /// ```
    pub fn update_status(&mut self, status: IncidentStatus) {
        self.status = status;
        self.updated_at = Utc::now();
    }

    /// Assign incident to team member
    ///
    /// # Arguments
    /// * `assignee` - Team member to assign incident to
    ///
    /// # Example
    /// ```rust
    /// use beardog::threat::types::IncidentResponse;
    ///
    /// let mut incident = IncidentResponse::default();
    /// incident.assign_to("analyst-001".to_string());
    ///
    /// assert_eq!(incident.assigned_to, Some("analyst-001".to_string()));
    /// ```
    pub fn assign_to(&mut self, assignee: String) {
        self.assigned_to = Some(assignee);
        self.updated_at = Utc::now();
    }

    /// Add containment action
    ///
    /// # Arguments
    /// * `action` - Containment action taken
    ///
    /// # Example
    /// ```rust
    /// use beardog::threat::types::IncidentResponse;
    ///
    /// let mut incident = IncidentResponse::default();
    /// incident.add_containment_action("Isolated infected system".to_string());
    ///
    /// assert_eq!(incident.containment_actions.len(), 1);
    /// ```
    pub fn add_containment_action(&mut self, action: String) {
        self.containment_actions.push(action);
        self.updated_at = Utc::now();
    }

    /// Add remediation action
    ///
    /// # Arguments
    /// * `action` - Remediation action taken
    ///
    /// # Example
    /// ```rust
    /// use beardog::threat::types::IncidentResponse;
    ///
    /// let mut incident = IncidentResponse::default();
    /// incident.add_remediation_action("Malware removed".to_string());
    ///
    /// assert_eq!(incident.remediation_actions.len(), 1);
    /// ```
    pub fn add_remediation_action(&mut self, action: String) {
        self.remediation_actions.push(action);
        self.updated_at = Utc::now();
    }

    /// Add lesson learned
    ///
    /// # Arguments
    /// * `lesson` - Lesson learned from incident
    ///
    /// # Example
    /// ```rust
    /// use beardog::threat::types::IncidentResponse;
    ///
    /// let mut incident = IncidentResponse::default();
    /// incident.add_lesson_learned("Update antivirus signatures more frequently".to_string());
    ///
    /// assert_eq!(incident.lessons_learned.len(), 1);
    /// ```
    pub fn add_lesson_learned(&mut self, lesson: String) {
        self.lessons_learned.push(lesson);
        self.updated_at = Utc::now();
    }

    /// Get incident age in minutes
    ///
    /// # Returns
    /// Minutes since incident creation
    ///
    /// # Example
    /// ```rust
    /// use beardog::threat::types::IncidentResponse;
    ///
    /// let incident = IncidentResponse::default();
    /// let age = incident.age_minutes();
    /// assert!(age >= 0);
    /// ```
    pub fn age_minutes(&self) -> i64 {
        let now = Utc::now();
        (now - self.created_at).num_minutes()
    }

    /// Check if incident is active
    ///
    /// # Returns
    /// `true` if incident is not closed or resolved
    ///
    /// # Example
    /// ```rust
    /// use beardog::threat::types::{IncidentResponse, IncidentStatus};
    ///
    /// let mut incident = IncidentResponse::default();
    /// assert!(incident.is_active());
    ///
    /// incident.update_status(IncidentStatus::Closed);
    /// assert!(!incident.is_active());
    /// ```
    pub fn is_active(&self) -> bool {
        !matches!(
            self.status,
            IncidentStatus::Resolved | IncidentStatus::Closed
        )
    }

    /// Check if incident is high priority
    ///
    /// # Returns
    /// `true` if severity is high or critical
    ///
    /// # Example
    /// ```rust
    /// use beardog::threat::types::{IncidentResponse, ThreatSeverity};
    ///
    /// let mut incident = IncidentResponse::default();
    /// incident.severity = ThreatSeverity::High;
    ///
    /// assert!(incident.is_high_priority());
    /// ```
    pub fn is_high_priority(&self) -> bool {
        matches!(
            self.severity,
            ThreatSeverity::High | ThreatSeverity::Critical
        )
    }

    /// Get incident duration in minutes
    ///
    /// # Returns
    /// Minutes from creation to last update
    ///
    /// # Example
    /// ```rust
    /// use beardog::threat::types::IncidentResponse;
    ///
    /// let incident = IncidentResponse::default();
    /// let duration = incident.duration_minutes();
    /// assert!(duration >= 0);
    /// ```
    pub fn duration_minutes(&self) -> i64 {
        (self.updated_at - self.created_at).num_minutes()
    }

    /// Check if incident is overdue
    ///
    /// # Returns
    /// `true` if incident has been open too long based on severity
    ///
    /// # Example
    /// ```rust
    /// use beardog::threat::types::{IncidentResponse, ThreatSeverity};
    ///
    /// let mut incident = IncidentResponse::default();
    /// incident.severity = ThreatSeverity::Critical;
    ///
    /// // Check if overdue based on severity thresholds
    /// ```
    pub fn is_overdue(&self) -> bool {
        let age = self.age_minutes();

        match self.severity {
            ThreatSeverity::Critical => age > 60, // 1 hour
            ThreatSeverity::High => age > 240,    // 4 hours
            ThreatSeverity::Medium => age > 1440, // 24 hours
            ThreatSeverity::Low => age > 4320,    // 72 hours
            ThreatSeverity::Info => age > 10080,  // 1 week
        }
    }
}
