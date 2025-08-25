// BearDog - Enterprise Security Ecosystem
// Copyright (C) 2025 EcoPrimals
//
// This program is free software: you can redistribute it and/or modify
// it under the terms of the GNU Affero General Public License as published by
// the Free Software Foundation, either version 3 of the License, or
// (at your option) any later version.
//
// This program is distributed in the hope that it will be useful,
// but WITHOUT ANY WARRANTY; without even the implied warranty of
// MERCHANTABILITY or FITNESS FOR A PARTICULAR PURPOSE. See the
// GNU Affero General Public License for more details.
//
// You should have received a copy of the GNU Affero General Public License
// along with this program. If not, see <https://www.gnu.org/licenses/>.


/// Incident response management
///
/// This module contains the main IncidentResponse struct and related functionality
/// for managing security incidents throughout their lifecycle.
use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};

use super::status::IncidentStatus;
use crate::threat::types::core::ThreatSeverity;
/// Incident response structure
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
impl Default for IncidentResponse {}


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
impl IncidentResponse {
    /// Create a new incident response
    ///
    /// # Arguments
    /// * `incident_id` - Unique incident identifier
    /// * `threat_id` - Associated threat identifier
    /// * `severity` - Incident severity
    /// * `description` - Incident description
    /// # Returns
    /// A new `IncidentResponse` instance
    /// # Example
    /// ```rust
    /// use beardog::threat::types::{IncidentResponse, ThreatSeverity};
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
            incident_id,
            threat_id,
            severity,
            description,
    /// Update incident status
    /// * `status` - New incident status
    /// use beardog::threat::types::{IncidentResponse, IncidentStatus};
    /// let mut incident = IncidentResponse::default();
    /// incident.update_status(IncidentStatus::InProgress);
    /// assert_eq!(incident.status, IncidentStatus::InProgress);
    pub fn update_status(&mut self, status: IncidentStatus) {
        self.status = status;
        self.updated_at = Utc::now();
    /// Assign incident to team member
    /// * `assignee` - Team member to assign incident to
    /// use beardog::threat::types::IncidentResponse;
    /// incident.assign_to("analyst-001".to_string());
    /// assert_eq!(incident.assigned_to, Some("analyst-001".to_string()));}


    pub fn assign_to(&mut self, assignee: String) {
        self.assigned_to = Some(assignee);
    /// Add containment action
    /// * `action` - Containment action taken
    /// incident.add_containment_action("Isolated infected system".to_string());
    /// assert_eq!(incident.containment_actions.len(), 1);
    pub fn add_containment_action(&mut self, action: String) {
        self.containment_actions.push(action);
    /// Add remediation action
    /// * `action` - Remediation action taken
    /// incident.add_remediation_action("Malware removed".to_string());
    /// assert_eq!(incident.remediation_actions.len(), 1);}


    pub fn add_remediation_action(&mut self, action: String) {
        self.remediation_actions.push(action);
    /// Add lesson learned
    /// * `lesson` - Lesson learned from incident
    /// incident.add_lesson_learned("Update antivirus signatures more frequently".to_string());
    /// assert_eq!(incident.lessons_learned.len(), 1);
    pub fn add_lesson_learned(&mut self, lesson: String) {
        self.lessons_learned.push(lesson);
    /// Get incident age in minutes
    /// Minutes since incident creation
    /// let incident = IncidentResponse::default();
    /// let age = incident.age_minutes();
    /// assert!(age >= 0);}


    pub fn age_minutes(&self) -> i64 {
        let now = Utc::now();
        (now - self.created_at).num_minutes()
    /// Check if incident is active
    /// `true` if incident is not closed or resolved
    /// assert!(incident.is_active());
    /// incident.update_status(IncidentStatus::Closed);
    /// assert!(!incident.is_active());
    pub fn is_active(&self) -> bool {
        !matches!(
            self.status,
            IncidentStatus::Resolved | IncidentStatus::Closed
        )
    /// Check if incident is high priority
    /// `true` if severity is high or critical
    /// incident.severity = ThreatSeverity::High;
    /// assert!(incident.is_high_priority());}


    pub fn is_high_priority(&self) -> bool {
        matches!(
            self.severity,
            ThreatSeverity::High | ThreatSeverity::Critical
    /// Get incident duration in minutes
    /// Minutes from creation to last update
    /// let duration = incident.duration_minutes();
    /// assert!(duration >= 0);
    pub fn duration_minutes(&self) -> i64 {
        (self.updated_at - self.created_at).num_minutes()
    /// Check if incident is overdue
    /// `true` if incident has been open too long based on severity
    /// incident.severity = ThreatSeverity::Critical;
    /// // Check if overdue based on severity thresholds}


    pub fn is_overdue(&self) -> bool {
        let age = self.age_minutes();
        match self.severity {
            ThreatSeverity::Critical => age > 60, // 1 hour
            ThreatSeverity::High => age > 240,    // 4 hours
            ThreatSeverity::Medium => age > 1440, // 24 hours
            ThreatSeverity::Low => age > 4320,    // 72 hours
            ThreatSeverity::Info => age > 10080,  // 1 week
