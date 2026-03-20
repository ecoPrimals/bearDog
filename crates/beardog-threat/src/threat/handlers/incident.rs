// SPDX-License-Identifier: AGPL-3.0-only

// Incident Response Handlers
//
// This module provides incident response functionality for the BearDog threat detection system.

use super::core::ThreatDetectionEngine;
use crate::threat::types::incidents::{IncidentLifecyclePhase, ManagedIncident};
use crate::threat::types::{ThreatEvent, ThreatSeverity};
use beardog_errors::BearDogError;
use chrono::{DateTime, Utc};
use std::collections::BTreeMap;
use std::time::SystemTime;
use tracing::{info, warn};

impl ThreatDetectionEngine {
    fn lock_managed_incidents_mut(
        &self,
    ) -> Result<std::sync::RwLockWriteGuard<'_, BTreeMap<String, ManagedIncident>>, BearDogError>
    {
        self.managed_incidents.write().map_err(|e| {
            BearDogError::system(format!("managed incident store poisoned (write): {e}"))
        })
    }

    fn lock_managed_incidents(
        &self,
    ) -> Result<std::sync::RwLockReadGuard<'_, BTreeMap<String, ManagedIncident>>, BearDogError>
    {
        self.managed_incidents.read().map_err(|e| {
            BearDogError::system(format!("managed incident store poisoned (read): {e}"))
        })
    }

    /// Create incident from threat event (lifecycle: **Created**).
    pub fn create_incident_from_threat(
        &self,
        threat_event: &ThreatEvent,
    ) -> Result<String, BearDogError> {
        let incident_id = format!("INC-{}", uuid::Uuid::new_v4());
        let managed = ManagedIncident::from_threat(incident_id.clone(), threat_event);

        self.lock_managed_incidents_mut()?
            .insert(incident_id.clone(), managed);

        info!(
            incident_id = %incident_id,
            threat_id = %threat_event.id,
            severity = %threat_event.severity.as_str(),
            category = ?threat_event.threat_type,
            "created incident from threat",
        );

        Ok(incident_id)
    }

    /// Advance **Created → Classified** (triage / category confirmation).
    pub fn classify_incident(&self, incident_id: &str) -> Result<ManagedIncident, BearDogError> {
        let mut guard = self.lock_managed_incidents_mut()?;
        let entry = guard
            .get_mut(incident_id)
            .ok_or_else(|| BearDogError::business(format!("unknown incident: {incident_id}")))?;
        entry.try_advance_to(IncidentLifecyclePhase::Classified)?;
        info!(incident_id = %incident_id, "incident classified");
        Ok(entry.clone())
    }

    /// Advance **Classified → Escalated** (ownership / response team).
    pub fn escalate_incident(&self, incident_id: &str) -> Result<ManagedIncident, BearDogError> {
        let mut guard = self.lock_managed_incidents_mut()?;
        let entry = guard
            .get_mut(incident_id)
            .ok_or_else(|| BearDogError::business(format!("unknown incident: {incident_id}")))?;
        entry.try_advance_to(IncidentLifecyclePhase::Escalated)?;
        info!(incident_id = %incident_id, "incident escalated");
        Ok(entry.clone())
    }

    /// Advance **Escalated → Resolved** and record resolution text.
    pub fn resolve_incident(
        &self,
        incident_id: &str,
        resolution: &str,
    ) -> Result<ManagedIncident, BearDogError> {
        let mut guard = self.lock_managed_incidents_mut()?;
        let entry = guard
            .get_mut(incident_id)
            .ok_or_else(|| BearDogError::business(format!("unknown incident: {incident_id}")))?;
        entry.resolution = Some(resolution.to_string());
        entry.try_advance_to(IncidentLifecyclePhase::Resolved)?;
        info!(incident_id = %incident_id, "incident resolved");
        Ok(entry.clone())
    }

    /// Records an assignment to a response team (updates typed store when present).
    pub fn assign_incident_to_team(
        &self,
        incident_id: &str,
        team: &str,
    ) -> Result<(), BearDogError> {
        let mut guard = self.lock_managed_incidents_mut()?;
        if let Some(entry) = guard.get_mut(incident_id) {
            entry.assigned_team = Some(team.to_string());
            entry.updated_at = SystemTime::now();
        }
        info!(
            incident_id = %incident_id,
            team = %team,
            "assign_incident_to_team",
        );
        Ok(())
    }

    /// Update incident status using legacy string labels (see `legacy_status_label`).
    ///
    /// When the incident id is unknown, returns a synthetic snapshot so existing callers keep working.
    pub fn update_incident_status(
        &self,
        incident_id: &str,
        status: &str,
    ) -> Result<Option<IncidentResponse>, BearDogError> {
        let mut guard = self.lock_managed_incidents_mut()?;
        if let Some(entry) = guard.get_mut(incident_id) {
            apply_legacy_status(entry, status);
            let view = managed_to_legacy(entry);
            return Ok(Some(view));
        }
        drop(guard);

        warn!(
            incident_id = %incident_id,
            status = %status,
            "update_incident_status: unknown id; returning synthetic snapshot",
        );
        Ok(Some(synthetic_legacy(incident_id, status)))
    }

    /// Lists incidents that are not [`IncidentLifecyclePhase::Closed`].
    pub fn get_active_incidents(&self) -> Result<Vec<IncidentResponse>, BearDogError> {
        let guard = self.lock_managed_incidents()?;
        let mut out: Vec<IncidentResponse> = guard
            .values()
            .filter(|m| m.lifecycle != IncidentLifecyclePhase::Closed)
            .map(managed_to_legacy)
            .collect();
        out.sort_by(|a, b| a.id.cmp(&b.id));
        Ok(out)
    }

    /// Advance **Resolved → Closed** when possible; always returns `Ok(())` for API compatibility.
    pub fn close_incident(&self, incident_id: &str) -> Result<(), BearDogError> {
        let mut guard = self.lock_managed_incidents_mut()?;
        if let Some(entry) = guard.get_mut(incident_id) {
            if entry.lifecycle == IncidentLifecyclePhase::Resolved {
                if let Err(e) = entry.try_advance_to(IncidentLifecyclePhase::Closed) {
                    warn!(incident_id = %incident_id, error = %e, "close_incident: transition skipped");
                } else {
                    info!(incident_id = %incident_id, "incident closed");
                }
            }
        } else {
            info!(
                incident_id = %incident_id,
                "close_incident: unknown id (no-op)",
            );
        }
        Ok(())
    }
}

/// Maps a managed incident to the legacy string status expected by older call sites.
fn legacy_status_label(phase: &IncidentLifecyclePhase) -> &'static str {
    match phase {
        IncidentLifecyclePhase::Created | IncidentLifecyclePhase::Classified => "open",
        IncidentLifecyclePhase::Escalated => "in_progress",
        IncidentLifecyclePhase::Resolved => "resolved",
        IncidentLifecyclePhase::Closed => "closed",
    }
}

fn system_time_to_utc(st: SystemTime) -> DateTime<Utc> {
    st.into()
}

fn managed_to_legacy(m: &ManagedIncident) -> IncidentResponse {
    IncidentResponse {
        id: m.id.clone(),
        threat_id: m.threat_id.clone(),
        status: legacy_status_label(&m.lifecycle).to_string(),
        priority: m.severity.as_str().to_string(),
        assigned_team: m.assigned_team.clone(),
        created_at: system_time_to_utc(m.created_at),
        updated_at: system_time_to_utc(m.updated_at),
        estimated_cost: None,
        actual_cost: None,
        containment_actions: Vec::new(),
        remediation_actions: Vec::new(),
        lessons_learned: Vec::new(),
        severity: m.severity.as_str().to_string(),
    }
}

fn synthetic_legacy(incident_id: &str, status: &str) -> IncidentResponse {
    let now = Utc::now();
    IncidentResponse {
        id: incident_id.to_string(),
        threat_id: "sample-threat".to_string(),
        status: status.to_string(),
        priority: ThreatSeverity::Medium.as_str().to_string(),
        assigned_team: None,
        created_at: now,
        updated_at: now,
        estimated_cost: None,
        actual_cost: None,
        containment_actions: Vec::new(),
        remediation_actions: Vec::new(),
        lessons_learned: Vec::new(),
        severity: ThreatSeverity::Medium.as_str().to_string(),
    }
}

fn apply_legacy_status(m: &mut ManagedIncident, status: &str) {
    let now = SystemTime::now();
    m.updated_at = now;
    match status {
        "open" => {
            m.lifecycle = IncidentLifecyclePhase::Created;
        }
        "in_progress" => {
            m.lifecycle = IncidentLifecyclePhase::Escalated;
            m.escalated_at.get_or_insert(now);
        }
        "resolved" => {
            m.lifecycle = IncidentLifecyclePhase::Resolved;
            m.resolved_at.get_or_insert(now);
            m.resolution
                .get_or_insert_with(|| "resolved via legacy status update".to_string());
        }
        "closed" => {
            m.lifecycle = IncidentLifecyclePhase::Closed;
            m.closed_at.get_or_insert(now);
        }
        _ => {}
    }
}

/// Legacy incident DTO retained for backward-compatible APIs and tests.
#[derive(Debug, Clone)]
pub struct IncidentResponse {
    /// Incident identifier returned to callers.
    pub id: String,
    /// Linked [`ThreatEvent::id`].
    pub threat_id: String,
    /// Legacy string status (`open`, `in_progress`, `resolved`, `closed`).
    pub status: String,
    /// Priority label (mirrors severity string form).
    pub priority: String,
    /// Optional assigned team
    pub assigned_team: Option<String>,
    /// The created at value
    pub created_at: chrono::DateTime<Utc>,
    /// The updated at value
    pub updated_at: chrono::DateTime<Utc>,
    /// Optional estimated cost
    pub estimated_cost: Option<f64>,
    /// Optional actual cost
    pub actual_cost: Option<f64>,
    /// Collection of containment actions
    pub containment_actions: Vec<String>,
    /// Collection of remediation actions
    pub remediation_actions: Vec<String>,
    /// Collection of lessons learned
    pub lessons_learned: Vec<String>,
    /// The severity value
    pub severity: String,
}
