use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use uuid::Uuid;

use super::super::core::ThreatSeverity;

#[derive(Debug, Clone, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub enum ResponseType {
    Investigation,
    Containment,
    Eradication,
    Recovery,
    LessonsLearned,
}

#[derive(Debug, Clone, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub enum IncidentStatus {
    Open,
    InProgress,
    Escalated, // Add missing variant
    Contained,
    Resolved,
    Closed,
}

impl IncidentStatus {
    pub fn valid_next_statuses(&self) -> Vec<IncidentStatus> {
        match self {
            IncidentStatus::Open => vec![IncidentStatus::InProgress, IncidentStatus::Escalated],
            IncidentStatus::InProgress => {
                vec![IncidentStatus::Contained, IncidentStatus::Escalated]
            }
            IncidentStatus::Escalated => {
                vec![IncidentStatus::InProgress, IncidentStatus::Contained]
            }
            IncidentStatus::Contained => vec![IncidentStatus::Resolved],
            IncidentStatus::Resolved => vec![IncidentStatus::Closed],
            IncidentStatus::Closed => vec![],
        }
    }

    pub fn can_transition_to(&self, target: &IncidentStatus) -> bool {
        self.valid_next_statuses().contains(target)
    }
}

#[derive(Debug, Clone, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub enum IncidentType {
    Security,
    Performance,
    Availability,
    Compliance,
    DataBreach,
    Malware,
    Phishing,
    DenialOfService,
}

impl Default for IncidentType {
    fn default() -> Self {
        Self::Security
    }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct IncidentResponse {
    pub response_id: String,
    pub incident_id: String,
    pub response_type: ResponseType,
    pub status: IncidentStatus,
    pub assigned_team: Vec<String>,
    pub actions_taken: Vec<String>,
    pub created_at: DateTime<Utc>,
    pub updated_at: DateTime<Utc>,
    pub metadata: HashMap<String, String>,
    // Add missing fields
    pub incident_type: IncidentType,
    pub estimated_cost: Option<f64>,
    pub actual_cost: Option<f64>,
    pub containment_actions: Vec<String>,
    pub remediation_actions: Vec<String>,
    pub lessons_learned: Vec<String>,
    pub severity: ThreatSeverity,
    pub assigned_to: Option<String>,
}
impl Default for IncidentResponse {
    fn default() -> Self {
        Self {
            response_id: uuid::Uuid::new_v4().to_string(),
            incident_id: String::new(),
            response_type: ResponseType::Investigation,
            status: IncidentStatus::Open,
            assigned_team: Vec::new(),
            actions_taken: Vec::new(),
            created_at: chrono::Utc::now(),
            updated_at: chrono::Utc::now(),
            metadata: HashMap::new(),
            incident_type: IncidentType::Security,
            estimated_cost: None,
            actual_cost: None,
            containment_actions: Vec::new(),
            remediation_actions: Vec::new(),
            lessons_learned: Vec::new(),
            severity: ThreatSeverity::Medium,
            assigned_to: None,
        }
    }
}
impl IncidentResponse {
    pub fn new(incident_id: &str, _severity: ThreatSeverity, _description: &str) -> Self {
        Self {
            response_id: Uuid::new_v4().to_string(),
            incident_id: incident_id.to_string(),
            response_type: ResponseType::Investigation,
            status: IncidentStatus::Open,
            assigned_team: Vec::new(), // Fixed: should be Vec<String>, not Option
            actions_taken: Vec::new(),
            created_at: Utc::now(),
            updated_at: Utc::now(),
            metadata: HashMap::new(),
            // Add missing required fields
            incident_type: IncidentType::Security,
            estimated_cost: None,
            actual_cost: None,
            containment_actions: Vec::new(),
            remediation_actions: Vec::new(),
            lessons_learned: Vec::new(),
            severity: ThreatSeverity::Medium,
            assigned_to: None,
        }
    }

    pub fn update_status(&mut self, status: IncidentStatus) {
        self.status = status;
        self.updated_at = Utc::now();
    }

    pub fn assign_to(&mut self, assignee: &str) {
        self.assigned_team.push(assignee.to_string());
        self.assigned_to = Some(assignee.to_string());
        self.updated_at = Utc::now();
    }

    pub fn add_containment_action(&mut self, action: &str) {
        self.containment_actions.push(action.to_string());
        self.updated_at = Utc::now();
    }

    pub fn add_remediation_action(&mut self, action: &str) {
        self.remediation_actions.push(action.to_string());
        self.updated_at = Utc::now();
    }

    pub fn add_lesson_learned(&mut self, lesson: &str) {
        self.lessons_learned.push(lesson.to_string());
        self.updated_at = Utc::now();
    }

    pub fn set_estimated_cost(&mut self, cost: f64) {
        self.estimated_cost = Some(cost);
        self.updated_at = Utc::now();
    }

    pub fn set_actual_cost(&mut self, cost: f64) {
        self.actual_cost = Some(cost);
        self.updated_at = Utc::now();
    }

    pub fn is_active(&self) -> bool {
        matches!(
            self.status,
            IncidentStatus::Open | IncidentStatus::InProgress
        )
    }

    pub fn is_resolved(&self) -> bool {
        matches!(
            self.status,
            IncidentStatus::Resolved | IncidentStatus::Closed
        )
    }

    pub fn is_high_priority(&self) -> bool {
        matches!(
            self.severity,
            ThreatSeverity::High | ThreatSeverity::Critical
        )
    }

    pub fn get_age_minutes(&self) -> i64 {
        let now = Utc::now();
        now.signed_duration_since(self.created_at).num_minutes()
    }

    pub fn is_stale(&self) -> bool {
        let age = self.get_age_minutes();
        match self.severity {
            ThreatSeverity::Critical => age > 60, // 1 hour
            ThreatSeverity::High => age > 240,    // 4 hours
            ThreatSeverity::Medium => age > 1440, // 24 hours
            ThreatSeverity::Low => age > 4320,    // 3 days
            ThreatSeverity::Info => age > 10080,  // 1 week
        }
    }

    pub fn get_response_metrics(&self) -> ResponseMetrics {
        let age_minutes = self.get_age_minutes();
        ResponseMetrics {
            incident_id: self.incident_id.clone(),
            time_to_detection: Some(age_minutes),
            time_to_response: if self.assigned_to.is_some() {
                Some(age_minutes)
            } else {
                None
            },
            time_to_containment: if !self.containment_actions.is_empty() {
                Some(age_minutes)
            } else {
                None
            },
            time_to_resolution: if self.is_resolved() {
                Some(age_minutes)
            } else {
                None
            },
            total_actions: self.containment_actions.len() + self.remediation_actions.len(),
            lessons_learned_count: self.lessons_learned.len(),
        }
    }
}

#[derive(Debug, Clone)]
pub struct ResponseMetrics {
    pub incident_id: String,
    pub time_to_detection: Option<i64>,
    pub time_to_response: Option<i64>,
    pub time_to_containment: Option<i64>,
    pub time_to_resolution: Option<i64>,
    pub total_actions: usize,
    pub lessons_learned_count: usize,
}
