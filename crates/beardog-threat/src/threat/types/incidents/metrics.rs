

use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};
use std::collections::HashMap;

use beardog_errors::BearDogResult;

// Define local types instead of importing from beardog_types
#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum IncidentSeverity {
    Low,
    Medium,
    High,
    Critical,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum IncidentStatus {
    Open,
    InProgress,
    Resolved,
    Closed,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SecurityIncident {
    pub status: IncidentStatus,
    pub severity: IncidentSeverity,
    pub incident_type: String,
}

#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct IncidentMetrics {

    pub total_incidents: u32,

    pub open_incidents: u32,

    pub closed_incidents: u32,

    pub mean_time_to_detection_minutes: f64,

    pub mean_time_to_response_minutes: f64,

    pub mean_time_to_resolution_minutes: f64,

    pub severity_breakdown: HashMap<String, u32>,

    pub type_breakdown: HashMap<String, u32>,

    pub monthly_trends: Vec<f64>,

    pub cost_breakdown: CostBreakdown,

    pub time_to_detection: Option<i64>,

    pub time_to_containment: Option<i64>,

    pub time_to_resolution: Option<i64>,

    pub total_duration: Option<i64>,

    pub estimated_cost: Option<f64>,
}
impl IncidentMetrics {

    pub fn new() -> Self {
        Self {
            total_incidents: 0,
            open_incidents: 0,
            closed_incidents: 0,
            mean_time_to_detection_minutes: 0.0,
            mean_time_to_response_minutes: 0.0,
            mean_time_to_resolution_minutes: 0.0,
            severity_breakdown: HashMap::with_capacity(16),
            type_breakdown: HashMap::with_capacity(16),
            monthly_trends: Vec::new(),
            cost_breakdown: CostBreakdown {
                investigation_cost: 0.0,
                containment_cost: 0.0,
                recovery_cost: 0.0,
                business_impact_cost: 0.0,
                total_cost: 0.0,
            },
            time_to_detection: None,
            time_to_containment: None,
            time_to_resolution: None,
            total_duration: None,
            estimated_cost: None,
        }
    }

    pub fn calculate_total_cost(&self) -> f64 {
        self.cost_breakdown.investigation_cost
            + self.cost_breakdown.containment_cost
            + self.cost_breakdown.recovery_cost
            + self.cost_breakdown.business_impact_cost
            + self.estimated_cost.unwrap_or(0.0)
    }

    pub fn is_complete(&self) -> bool {
        self.time_to_detection.is_some()
            && self.time_to_containment.is_some()
            && self.time_to_resolution.is_some()
            && self.total_duration.is_some()
    }

    pub fn update_with_incident(&mut self, incident: &SecurityIncident) {
        self.total_incidents += 1;
        
        match incident.status {
            IncidentStatus::Open | IncidentStatus::InProgress => {
                self.open_incidents += 1;
            }
            IncidentStatus::Resolved | IncidentStatus::Closed => {
                self.closed_incidents += 1;
            }
        }

        let severity_count = self.severity_breakdown.entry(incident.severity.to_string()).or_insert(0);
        *severity_count += 1;

        let type_count = self.type_breakdown.entry(incident.incident_type.clone()).or_insert(0);
        *type_count += 1;
    }

    pub fn resolution_rate(&self) -> f64 {
        if self.total_incidents == 0 {
            0.0
        } else {
            self.closed_incidents as f64 / self.total_incidents as f64
        }
    }
}

impl std::fmt::Display for IncidentSeverity {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            IncidentSeverity::Low => write!(f, "Low"),
            IncidentSeverity::Medium => write!(f, "Medium"),
            IncidentSeverity::High => write!(f, "High"),
            IncidentSeverity::Critical => write!(f, "Critical"),
        }
    }
}

#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct CostBreakdown {

    pub investigation_cost: f64,

    pub containment_cost: f64,

    pub recovery_cost: f64,

    pub business_impact_cost: f64,

    pub total_cost: f64,
}
