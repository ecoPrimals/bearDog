//! Incident status management
//!
//! This module contains types and functionality for managing incident status
//! throughout the incident lifecycle.

use serde::{Deserialize, Serialize};

/// Incident status enumeration
///
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
    ///
    /// # Example
    /// ```rust
    /// use beardog::threat::types::IncidentStatus;
    ///
    /// assert!(IncidentStatus::Closed.is_terminal());
    /// assert!(!IncidentStatus::Open.is_terminal());
    /// ```
    pub fn is_terminal(&self) -> bool {
        matches!(self, IncidentStatus::Closed)
    }

    /// Get valid next statuses
    ///
    /// # Returns
    /// Vector of valid next status transitions
    ///
    /// # Example
    /// ```rust
    /// use beardog::threat::types::IncidentStatus;
    ///
    /// let next_statuses = IncidentStatus::Open.valid_next_statuses();
    /// assert!(next_statuses.contains(&IncidentStatus::InProgress));
    /// ```
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
                IncidentStatus::Contained,
                IncidentStatus::Resolved,
            ],
        }
    }

    /// Check if transition is valid
    ///
    /// # Arguments
    /// * `new_status` - Target status
    ///
    /// # Returns
    /// `true` if transition is valid
    ///
    /// # Example
    /// ```rust
    /// use beardog::threat::types::IncidentStatus;
    ///
    /// let current = IncidentStatus::Open;
    /// assert!(current.can_transition_to(&IncidentStatus::InProgress));
    /// assert!(!current.can_transition_to(&IncidentStatus::Closed));
    /// ```
    pub fn can_transition_to(&self, new_status: &IncidentStatus) -> bool {
        self.valid_next_statuses().contains(new_status)
    }
}

impl std::fmt::Display for IncidentStatus {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            IncidentStatus::Open => write!(f, "Open"),
            IncidentStatus::InProgress => write!(f, "In Progress"),
            IncidentStatus::Contained => write!(f, "Contained"),
            IncidentStatus::Resolved => write!(f, "Resolved"),
            IncidentStatus::Closed => write!(f, "Closed"),
            IncidentStatus::Escalated => write!(f, "Escalated"),
        }
    }
} 