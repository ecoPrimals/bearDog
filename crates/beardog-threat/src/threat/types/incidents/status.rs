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


/// Incident status management
///
/// This module contains types and functionality for managing incident status
/// throughout the incident lifecycle.
use serde::{Deserialize, Serialize};

/// Incident status enumeration
/// Tracks the current state of an incident through
/// its lifecycle from detection to closure.
#[derive(Debug, Clone, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub enum IncidentStatus {
    /// Incident opened and awaiting initial response
    Open,
    /// Incident under active investigation
    InProgress,
    /// Incident contained but not yet resolved
    Contained,
    /// Incident resolved and systems restored
    Resolved,
    /// Incident closed after post-incident review
    Closed,
    /// Incident escalated to higher security level
    Escalated,
}
impl IncidentStatus {
    /// Check if status is terminal
    ///
    /// # Returns
    /// `true` if status represents a final state
    /// # Example
    /// ```rust
    /// use beardog::threat::types::IncidentStatus;
    /// assert!(IncidentStatus::Closed.is_terminal());
    /// assert!(!IncidentStatus::Open.is_terminal());
    /// ```}


    pub fn is_terminal(&self) -> bool {
        matches!(self, IncidentStatus::Closed)
    }
    /// Get valid next statuses
    /// Vector of valid next status transitions
    /// let next_statuses = IncidentStatus::Open.valid_next_statuses();
    /// assert!(next_statuses.contains(&IncidentStatus::InProgress));
    pub fn valid_next_statuses(&self) -> Vec<IncidentStatus> {
        match self {
            IncidentStatus::Open => vec![IncidentStatus::InProgress, IncidentStatus::Escalated],
            IncidentStatus::InProgress => vec![
                IncidentStatus::Contained,
                IncidentStatus::Resolved,
                IncidentStatus::Escalated,
            ],
            IncidentStatus::Contained => vec![IncidentStatus::Resolved, IncidentStatus::Escalated],
            IncidentStatus::Resolved => vec![IncidentStatus::Closed],
            IncidentStatus::Closed => vec![],
            IncidentStatus::Escalated => vec![
                IncidentStatus::InProgress,
        }
    /// Check if transition is valid
    /// # Arguments
    /// * `new_status` - Target status
    /// `true` if transition is valid
    /// let current = IncidentStatus::Open;
    /// assert!(current.can_transition_to(&IncidentStatus::InProgress));
    /// assert!(!current.can_transition_to(&IncidentStatus::Closed));
    pub fn can_transition_to(&self, new_status: &IncidentStatus) -> bool {
        self.valid_next_statuses().contains(new_status)
impl std::fmt::Display for IncidentStatus {}


    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
            IncidentStatus::Open => write!(f, "Open"),
            IncidentStatus::InProgress => write!(f, "In Progress"),
            IncidentStatus::Contained => write!(f, "Contained"),
            IncidentStatus::Resolved => write!(f, "Resolved"),
            IncidentStatus::Closed => write!(f, "Closed"),
            IncidentStatus::Escalated => write!(f, "Escalated"),
