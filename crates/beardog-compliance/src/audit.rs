

use beardog_errors::BearDogResult;
use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use std::sync::Arc;
use tokio::sync::RwLock;
use uuid::Uuid;

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
pub enum AuditSeverity {
    Low,
    Medium,
    High,
    Critical,
}

impl std::fmt::Display for AuditSeverity {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            AuditSeverity::Low => write!(f, "LOW"),
            AuditSeverity::Medium => write!(f, "MEDIUM"),
            AuditSeverity::High => write!(f, "HIGH"),
            AuditSeverity::Critical => write!(f, "CRITICAL"),
        }
    }
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
pub enum AuditEventType {
    UserAction,
    SystemEvent,
    SecurityEvent,
    ConfigurationChange,
    DataAccess,
    AuthenticationAttempt,
    AuthorizationCheck,
    ComplianceViolation,
}

impl std::fmt::Display for AuditEventType {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            AuditEventType::UserAction => write!(f, "USER_ACTION"),
            AuditEventType::SystemEvent => write!(f, "SYSTEM_EVENT"),
            AuditEventType::SecurityEvent => write!(f, "SECURITY_EVENT"),
            AuditEventType::ConfigurationChange => write!(f, "CONFIG_CHANGE"),
            AuditEventType::DataAccess => write!(f, "DATA_ACCESS"),
            AuditEventType::AuthenticationAttempt => write!(f, "AUTH_ATTEMPT"),
            AuditEventType::AuthorizationCheck => write!(f, "AUTHZ_CHECK"),
            AuditEventType::ComplianceViolation => write!(f, "COMPLIANCE_VIOLATION"),
        }
    }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct AuditEvent {
    pub id: Uuid,
    pub timestamp: chrono::DateTime<chrono::Utc>,
    pub event_type: AuditEventType,
    pub severity: AuditSeverity,
    pub user_id: Option<String>,
    pub session_id: Option<String>,
    pub source_ip: Option<String>,
    pub resource: String,
    pub action: String,
    pub outcome: String,
    pub details: HashMap<String, String>,
    pub compliance_tags: Vec<String>,
}

impl AuditEvent {

    pub fn new(
        event_type: AuditEventType,
        severity: AuditSeverity,
        resource: &str,
        action: &str,
        outcome: &str,
    ) -> Self {
        Self {
            id: Uuid::new_v4(),
            timestamp: chrono::Utc::now(),
            event_type,
            severity,
            user_id: None,
            session_id: None,
            source_ip: None,
            resource: resource.to_string(),
            action: action.to_string(),
            outcome: outcome.to_string(),
            details: HashMap::with_capacity(16),
            compliance_tags: Vec::new(),
        }
    }

    pub fn with_user_id(mut self, user_id: &str) -> Self {
        self.user_id = Some(user_id.to_string());
        self
    }

    pub fn with_session(mut self, session_id: &str) -> Self {
        self.session_id = Some(session_id.to_string());
        self
    }

    pub fn with_source_ip(mut self, ip: &str) -> Self {
        self.source_ip = Some(ip.to_string());
        self
    }

    pub fn with_detail(mut self, key: &str, value: &str) -> Self {
        self.details.insert(key.to_string(), value.to_string());
        self
    }

    pub fn with_compliance_tags(mut self, tags: Vec<&str>) -> Self {
        self.compliance_tags = tags.iter().map(|s| s.to_string()).collect();
        self
    }
}

pub struct AuditEngine {
    events: Arc<RwLock<Vec<AuditEvent>>>,
}

impl AuditEngine {

    pub fn new() -> Self {
        Self {
            events: Arc::new(RwLock::new(Vec::new())),
        }
    }

    pub async fn log_event(&self, event: AuditEvent) -> BearDogResult<()> {
        self.events.write().await.push(event);
        Ok(())
    }

    pub async fn get_recent_events(&self, limit: usize) -> BearDogResult<Vec<AuditEvent>> {
        let events = self.events.read().await;
        let start = events.len().saturating_sub(limit);
        Ok(events[start..].to_vec())
    }

    pub async fn query_events(
        &self,
        event_type: Option<AuditEventType>,
        severity: Option<AuditSeverity>,
        user_id: Option<&str>,
        limit: Option<usize>,
    ) -> BearDogResult<Vec<AuditEvent>> {
        let events = self.events.read().await;
        let mut filtered: Vec<AuditEvent> = events
            .iter()
            .filter(|event| {
                if let Some(ref et) = event_type {
                    if &event.event_type != et {
                        return false;
                    }
                }
                if let Some(ref sev) = severity {
                    if &event.severity != sev {
                        return false;
                    }
                }
                if let Some(ref uid) = user_id {
                    if let Some(ref event_uid) = event.user_id {
                        if event_uid != uid {
                            return false;
                        }
                    } else {
                        return false;
                    }
                }
                true
            })
            .cloned()
            .collect();

        filtered.sort_by(|a, b| b.timestamp.cmp(&a.timestamp));

        if let Some(limit) = limit {
            filtered.truncate(limit);
        }

        Ok(filtered)
    }

    pub async fn event_count(&self) -> usize {
        self.events.read().await.len()
    }

    pub async fn clear_events(&self) {
        self.events.write().await.clear();
    }
}

impl Default for AuditEngine {
    fn default() -> Self {
        Self::new()
    }
}

pub struct AuditLog {
    events: Vec<AuditEvent>,
}

impl AuditLog {

    pub fn new() -> Self {
        Self {
            events: Vec::new(),
        }
    }

    pub fn add_event(&mut self, event: AuditEvent) {
        self.events.push(event);
    }

    pub fn events(&self) -> &[AuditEvent] {
        &self.events
    }

    pub fn filter_events(
        &self,
        event_type: Option<AuditEventType>,
        severity: Option<AuditSeverity>,
        user_id: Option<&str>,
    ) -> BearDogResult<Vec<AuditEvent>> {
        let filtered: Vec<AuditEvent> = self
            .events
            .iter()
            .filter(|event| {
                if let Some(ref et) = event_type {
                    if &event.event_type != et {
                        return false;
                    }
                }
                if let Some(ref sev) = severity {
                    if &event.severity != sev {
                        return false;
                    }
                }
                if let Some(ref uid) = user_id {
                    if let Some(ref event_uid) = event.user_id {
                        if event_uid != uid {
                            return false;
                        }
                    } else {
                        return false;
                    }
                }
                true
            })
            .cloned()
            .collect();

        Ok(filtered)
    }

    pub fn placeholder() -> Self {
        Self {
            events: Vec::new(),
        }
    }
}

impl Default for AuditLog {
    fn default() -> Self {
        Self::placeholder()
    }
}
