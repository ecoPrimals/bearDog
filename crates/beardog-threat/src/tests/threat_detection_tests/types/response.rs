// SPDX-License-Identifier: AGPL-3.0-only

//! Incident Response Test Types

use super::behavior::Threat;
use super::pattern::ThreatSeverity;

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum ResponseAction {
    Block,
    BlockIP,
    Alert,
    NotifyAdmin,
    Log,
    Quarantine,
    LogIncident,
    QuarantineFile,
    IsolateHost,
    EscalateToSOC,
}

#[derive(Debug, Clone)]
pub struct ResponseRule {
    pub name: String,
    pub threat_type: super::behavior::ThreatType,
    pub severity: ThreatSeverity,
    pub actions: Vec<ResponseAction>,
}

impl ResponseRule {
    pub fn new(
        name: impl Into<String>,
        threat_type: super::behavior::ThreatType,
        severity: ThreatSeverity,
        actions: Vec<ResponseAction>,
    ) -> Self {
        Self {
            name: name.into(),
            threat_type,
            severity,
            actions,
        }
    }
}

pub struct IncidentResponseEngine {
    rules: Vec<ResponseRule>,
}

impl IncidentResponseEngine {
    pub fn new() -> Self {
        Self { rules: Vec::new() }
    }

    pub fn add_rule(&mut self, rule: ResponseRule) {
        self.rules.push(rule);
    }

    pub fn get_response(&self, severity: ThreatSeverity) -> Option<ResponseAction> {
        self.rules
            .iter()
            .find(|r| r.severity == severity)
            .and_then(|r| r.actions.first().copied())
    }

    pub fn get_response_actions(&self, threat: &Threat) -> Vec<ResponseAction> {
        let severity = threat.severity();
        let threat_type = threat.threat_type();

        self.rules
            .iter()
            .filter(|r| r.threat_type == threat_type && r.severity == severity)
            .flat_map(|r| r.actions.iter().copied())
            .collect()
    }
}

use std::collections::HashMap;

type ActionHandler = Box<dyn Fn(&Threat) + Send + Sync>;

pub struct ResponseExecutor {
    actions_executed: Vec<ResponseAction>,
    action_handlers: HashMap<ResponseAction, ActionHandler>,
}

impl ResponseExecutor {
    pub fn new() -> Self {
        Self {
            actions_executed: Vec::new(),
            action_handlers: HashMap::new(),
        }
    }

    pub fn execute(&mut self, action: ResponseAction, threat: &Threat) {
        if let Some(handler) = self.action_handlers.get(&action) {
            handler(threat);
        }
        self.actions_executed.push(action);
    }

    pub fn action_count(&self) -> usize {
        self.actions_executed.len()
    }

    pub fn set_action_handler<F>(&mut self, action: ResponseAction, handler: F)
    where
        F: Fn(&Threat) + Send + Sync + 'static,
    {
        self.action_handlers.insert(action, Box::new(handler));
    }

    pub fn execute_responses(&mut self, threat: &Threat, actions: &[ResponseAction]) {
        for &action in actions {
            self.execute(action, threat);
        }
    }
}

use std::time::Duration;

#[derive(Debug, Clone)]
pub struct EscalationLevel {
    name: String,
    severity: ThreatSeverity,
    contacts: Vec<String>,
    timeout: Duration,
}

impl EscalationLevel {
    pub fn new(
        name: impl Into<String>,
        severity: ThreatSeverity,
        contacts: Vec<&str>,
        timeout: Duration,
    ) -> Self {
        Self {
            name: name.into(),
            severity,
            contacts: contacts.iter().map(|s| s.to_string()).collect(),
            timeout,
        }
    }

    pub fn name(&self) -> &str {
        &self.name
    }
}

pub struct EscalationWorkflow {
    current_level: Option<EscalationLevel>,
    levels: Vec<EscalationLevel>,
}

impl EscalationWorkflow {
    pub fn new() -> Self {
        Self {
            current_level: None,
            levels: Vec::new(),
        }
    }

    pub fn escalate(&mut self) {
        // Move to next level if available
        if let Some(current) = &self.current_level {
            let current_idx = self
                .levels
                .iter()
                .position(|l| l.name() == current.name())
                .unwrap_or(0);
            if current_idx + 1 < self.levels.len() {
                self.current_level = Some(self.levels[current_idx + 1].clone());
            }
        } else if !self.levels.is_empty() {
            self.current_level = Some(self.levels[0].clone());
        }
    }

    pub fn current_level(&self) -> Option<&EscalationLevel> {
        self.current_level.as_ref()
    }

    pub fn add_level(&mut self, level: EscalationLevel) {
        self.levels.push(level);
    }

    pub fn determine_level(&self, threat: &Threat) -> EscalationLevel {
        let severity = threat.severity();

        // Find matching level or return the first one
        self.levels
            .iter()
            .find(|l| l.severity == severity)
            .cloned()
            .or_else(|| self.levels.first().cloned())
            .unwrap_or_else(|| {
                EscalationLevel::new(
                    "Level1",
                    ThreatSeverity::Low,
                    vec![],
                    Duration::from_secs(0),
                )
            })
    }
}

use std::time::Instant;

#[derive(Debug, Clone)]
pub struct Incident {
    pub id: String,
    pub created_at: Instant,
    pub mitigated: bool,
    pub mitigation_time: Option<Instant>,
}

pub struct ResponseTracker {
    responses: Vec<(String, ResponseAction, bool)>, // (incident_id, action, success)
    incidents: Vec<Incident>,
    successful_responses: usize,
}

impl ResponseTracker {
    pub fn new() -> Self {
        Self {
            responses: Vec::new(),
            incidents: Vec::new(),
            successful_responses: 0,
        }
    }

    pub fn track(&mut self, action: ResponseAction) {
        self.responses.push(("unknown".to_string(), action, true));
    }

    pub fn response_count(&self, incident_id: &str) -> usize {
        // Count responses for this specific incident
        self.responses
            .iter()
            .filter(|(id, _, _)| id == incident_id)
            .count()
    }

    pub fn create_incident(&mut self, _threat: &Threat) -> String {
        use std::sync::atomic::{AtomicUsize, Ordering};
        static COUNTER: AtomicUsize = AtomicUsize::new(0);

        let id = format!("incident_{}", COUNTER.fetch_add(1, Ordering::SeqCst));
        self.incidents.push(Incident {
            id: id.clone(),
            created_at: Instant::now(),
            mitigated: false,
            mitigation_time: None,
        });
        id
    }

    pub fn record_response(&mut self, incident_id: &str, action: ResponseAction, success: bool) {
        self.responses
            .push((incident_id.to_string(), action, success));
        if success {
            self.successful_responses += 1;
        }
    }

    pub fn success_rate(&self, incident_id: &str) -> f64 {
        let incident_responses: Vec<_> = self
            .responses
            .iter()
            .filter(|(id, _, _)| id == incident_id)
            .collect();

        if incident_responses.is_empty() {
            return 0.0;
        }

        let successful = incident_responses
            .iter()
            .filter(|(_, _, success)| *success)
            .count();
        (successful as f64 / incident_responses.len() as f64) * 100.0
    }

    pub fn mark_mitigated(&mut self, incident_id: &str) {
        if let Some(incident) = self.incidents.iter_mut().find(|i| i.id == incident_id) {
            incident.mitigated = true;
            incident.mitigation_time = Some(Instant::now());
        }
    }

    pub fn mitigation_time(&self, incident_id: &str) -> Option<std::time::Duration> {
        self.incidents
            .iter()
            .find(|i| i.id == incident_id)
            .and_then(|i| i.mitigation_time)
            .map(|mt| {
                mt.duration_since(
                    self.incidents
                        .iter()
                        .find(|i| i.id == incident_id)
                        .unwrap()
                        .created_at,
                )
            })
    }

    pub fn get_metrics(&self) -> ResponseMetrics {
        let overall_success_rate = if self.responses.is_empty() {
            0.0
        } else {
            let successful = self
                .responses
                .iter()
                .filter(|(_, _, success)| *success)
                .count();
            (successful as f64 / self.responses.len() as f64) * 100.0
        };

        ResponseMetrics {
            total_responses: self.responses.len(),
            successful_responses: self.successful_responses,
            success_rate: overall_success_rate,
            total_incidents: self.incidents.len(),
            mitigated_incidents: self.incidents.iter().filter(|i| i.mitigated).count(),
        }
    }
}

#[derive(Debug, Clone)]
pub struct ResponseMetrics {
    pub total_responses: usize,
    pub successful_responses: usize,
    pub success_rate: f64,
    pub total_incidents: usize,
    pub mitigated_incidents: usize,
}

impl ResponseMetrics {
    pub fn success_rate(&self) -> f64 {
        self.success_rate
    }
}
