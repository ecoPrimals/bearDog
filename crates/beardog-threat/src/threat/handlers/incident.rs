

use chrono::{DateTime, Utc};
use std::collections::HashMap;
use std::sync::Arc;
use tokio::sync::RwLock;
use tracing::{error, info, warn};
use uuid::Uuid;

use crate::threat::types::incidents::response::{IncidentResponse, ResponseType, IncidentStatus, IncidentType};
use crate::threat::types::core::ThreatEvent;
use beardog_errors::BearDogResult;
use beardog_errors::BearDogError;
use crate::threat::types::ThreatDetectionEngine;

impl ThreatDetectionEngine {

    pub async fn create_incident_response(
        &mut self,
        threat_event: &ThreatEvent,
    ) -> BearDogResult<String> {
        let incident_id = Uuid::new_v4().to_string();
        let timestamp = Utc::now();
        let incident_response = IncidentResponse {
            response_id: Uuid::new_v4().to_string(),
            incident_id: incident_id.clone(),
            response_type: ResponseType::Investigation,
            status: IncidentStatus::Open,
            assigned_team: vec!["Security Incident Response Team".to_string()],
            actions_taken: Vec::new(),
            created_at: timestamp,
            updated_at: timestamp,
            metadata: HashMap::new(),
            incident_type: IncidentType::Security,
            estimated_cost: None,
            actual_cost: None,
        };

        {
            let mut incidents = self.active_incidents.write().await;
            incidents.insert(incident_id.clone(), incident_response);
        }
        info!(
            "Created incident response: {} for threat: {}",
            incident_id, threat_event.id
        );
        Ok(incident_id)
    }

    pub async fn update_incident_status(
        &self,
        incident_id: &str,
        status: IncidentStatus,
    ) -> BearDogResult<()> {
        let mut incidents = self.active_incidents.write().await;
        if let Some(incident) = incidents.get_mut(incident_id) {
            incident.status = status;
            incident.updated_at = Utc::now();
            info!(
                "Updated incident {} status to {:?}",
                incident_id, incident.status
            );
            Ok(())
        } else {
            warn!("Incident not found: {}", incident_id);
            Err(BearDogError::not_found(format!(
                "Incident not found: {incident_id}"
            )))
        }
    }

    pub async fn get_incident(&self, incident_id: &str) -> BearDogResult<Option<IncidentResponse>> {
        let incidents = self.active_incidents.read().await;
        Ok(incidents.get(incident_id).cloned())
    }

    pub async fn add_incident_action(
        &self,
        incident_id: &str,
        action: &str,
    ) -> BearDogResult<()> {
        let mut incidents = self.active_incidents.write().await;
        if let Some(incident) = incidents.get_mut(incident_id) {
            let timestamp = Utc::now();
            let timestamped_action = format!("[{}] {}", timestamp.format("%Y-%m-%d %H:%M:%S UTC"), action);
            incident.actions_taken.push(timestamped_action);
            incident.updated_at = timestamp;
            info!("Added action to incident {}: {}", incident_id, action);
            Ok(())
        } else {
            Err(BearDogError::not_found(format!(
                "Incident not found: {incident_id}"
            )))
        }
    }

    pub async fn assign_incident_team(
        &self,
        incident_id: &str,
        team: &str,
    ) -> BearDogResult<()> {
        let mut incidents = self.active_incidents.write().await;
        if let Some(incident) = incidents.get_mut(incident_id) {
            let previous_team = incident.assigned_team.clone();
            incident.assigned_team = vec![team.to_string()];
            incident.updated_at = Utc::now();
            let timestamped_action = format!(
                "[{}] Team assignment changed from {:?} to {}",
                incident.updated_at.format("%Y-%m-%d %H:%M:%S UTC"),
                previous_team,
                team
            );
            incident.actions_taken.push(timestamped_action);
            info!("Assigned incident {} to team: {}", incident_id, team);
            Ok(())
        } else {
            Err(BearDogError::not_found(format!(
                "Incident not found: {incident_id}"
            )))
        }
    }

    pub async fn get_active_incidents(&self) -> BearDogResult<Vec<IncidentResponse>> {
        let incidents = self.active_incidents.read().await;
        let active_incidents: Vec<IncidentResponse> = incidents
            .values()
            .filter(|incident| {
                matches!(
                    incident.status,
                    IncidentStatus::Open | IncidentStatus::InProgress | IncidentStatus::Escalated
                )
            })
            .cloned()
            .collect();
        Ok(active_incidents)
    }

    pub async fn escalate_incident(
        &self,
        incident_id: &str,
        escalation_reason: &str,
    ) -> BearDogResult<()> {
        info!(
            "[{}] Incident escalated: {}",
            incident_id, escalation_reason
        );

        info!("Escalated incident {}: {}", incident_id, escalation_reason);
        Ok(())
    }

    pub async fn close_incident(
        &self,
        incident_id: &str,
        resolution_summary: &str,
    ) -> BearDogResult<()> {
        info!("Closing incident {}: {}", incident_id, resolution_summary);

        Ok(())
    }
}
