

use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub enum IncidentStatus {

    Open,

    InProgress,

    Contained,

    Resolved,

    Closed,

    Escalated,
}
impl IncidentStatus {

    pub fn valid_next_statuses(&self) -> Vec<IncidentStatus> {
        match self {
            IncidentStatus::Open => vec![
                IncidentStatus::InProgress,
                IncidentStatus::Escalated,
                IncidentStatus::Closed,
            ],
            IncidentStatus::InProgress => vec![
                IncidentStatus::Resolved,
                IncidentStatus::Escalated,
                IncidentStatus::Closed,
            ],
            IncidentStatus::Escalated => vec![
                IncidentStatus::InProgress,
                IncidentStatus::Resolved,
                IncidentStatus::Closed,
            ],
            IncidentStatus::Resolved => vec![
                IncidentStatus::Closed,
                IncidentStatus::InProgress, // Reopen if needed
            ],
            IncidentStatus::Closed => vec![
                IncidentStatus::InProgress, // Reopen
            ],
            IncidentStatus::Contained => vec![
                IncidentStatus::Resolved,
                IncidentStatus::InProgress, // If containment fails
                IncidentStatus::Closed,
            ],
        }
    }

    pub fn can_transition_to(&self, new_status: &IncidentStatus) -> bool {
        self.valid_next_statuses().contains(new_status)
    }
}

impl std::fmt::Display for IncidentStatus {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            IncidentStatus::Open => write!(f, "Open"),
            IncidentStatus::InProgress => write!(f, "In Progress"),
            IncidentStatus::Resolved => write!(f, "Resolved"),
            IncidentStatus::Closed => write!(f, "Closed"),
            IncidentStatus::Escalated => write!(f, "Escalated"),
            IncidentStatus::Contained => write!(f, "Contained"),
        }
    }
}
