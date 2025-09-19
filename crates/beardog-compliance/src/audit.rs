// Audit trail and compliance tracking for BearDog
// Provides comprehensive audit logging and compliance reporting

use beardog_errors::BearDogError;
use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use uuid::Uuid;

/// Canonical audit severity levels
#[derive(Debug, Clone, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub enum AuditSeverity {
    Low,
    /// Medium severity - notable events requiring attention
    Medium,
    /// High severity - important security or compliance events
    High,
    /// Critical severity - events requiring immediate attention
    Critical,
}

/// Types of audit events
#[derive(Debug, Clone, Serialize, Deserialize)]
/// Types of audit event
pub enum AuditEventType {
    /// Authentication-related events (login, logout, etc.)
    Authentication,
    /// Authorization and permission-related events
    Authorization,
    /// Data access and manipulation events
    DataAccess,
    /// System configuration or state changes
    SystemChange,
    /// Security-related events and violations
    SecurityEvent,
    /// Compliance verification and validation events
    ComplianceCheck,
}

/// Audit event record
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct AuditEvent {
    pub id: Uuid,
    /// Type of audit event
    /// The event type value
    pub event_type: AuditEventType,
    /// ID of the user who triggered this event
    pub user_id: Option<String>,
    /// Resource that was accessed or modified
    /// The resource value
    pub resource: String,
    /// The action value
    pub action: String,
    /// Result of the action (success, failure, etc.)
    /// The result value
    pub result: String,
    /// When this event occurred
    pub timestamp: chrono::DateTime<chrono::Utc>,
    /// IP address of the client (if applicable)
    /// Optional source ip
    pub source_ip: Option<String>,
    /// User agent string of the client (if applicable)
    /// Optional user agent
    pub user_agent: Option<String>,
    /// Compliance framework tags associated with this event
    /// Collection of compliance tags
    pub compliance_tags: Vec<String>,
    /// Mapping of metadata
    pub metadata: HashMap<String, String>,
}

impl AuditEvent {
    /// Create new audit event
    /// Creates a new instance
    #[must_use] pub fn new(
        event_type: AuditEventType,
        resource: String,
        action: String,
        result: String,
    ) -> Self {
        Self {
            id: Uuid::new_v4(),
            event_type,
            user_id: None,
            resource,
            action,
            result,
            timestamp: chrono::Utc::now(),
            source_ip: None,
            user_agent: None,
            compliance_tags: Vec::new(),
            metadata: HashMap::new(),
        }
    }

    /// Add user context to event
    /// Creates instance with user
    #[must_use] pub fn with_user(mut self, user_id: String) -> Self {
        self.user_id = Some(user_id);
        self
    }

    /// Add source IP to event
    /// Creates instance with source ip
    #[must_use] pub fn with_source_ip(mut self, ip: &str) -> Self {
        self.source_ip = Some(ip.to_string());
        self
    }

    /// Add compliance tags
    /// Creates instance with compliance tags
    pub fn with_compliance_tags(mut self, tags: Vec<String>) -> Self {
        self.compliance_tags = tags.iter().map(std::string::ToString::to_string).collect();
        self
    }

    /// Add metadata to event
    /// Creates instance with metadata
    #[must_use] pub fn with_metadata(mut self, key: String, value: String) -> Self {
        self.metadata.insert(key, value);
        self
    }
}

/// Audit trail storage and management
pub struct AuditEngine {
    events: Vec<AuditEvent>,
    max_events: usize,
}

impl AuditEngine {
    /// Create new audit engine
    /// Creates a new instance
    #[must_use] pub fn new(max_events: usize) -> Self {
        Self {
            events: Vec::with_capacity(max_events),
            max_events,
        }
    }

    /// Add audit event
    pub fn add_event(&mut self, event: AuditEvent) {
        self.events.push(event);

        // Rotate events if we exceed max capacity
        if self.events.len() > self.max_events {
            self.events.remove(0);
        }
    }

    /// Get events by type
    /// Gets `events_by_type`
    /// Gets `events_by_type`
    #[must_use] pub fn get_events_by_type(&self, event_type: &AuditEventType) -> Vec<&AuditEvent> {
        self.events
            .iter()
            .filter(|event| {
                std::mem::discriminant(&event.event_type) == std::mem::discriminant(event_type)
            })
            .collect()
    }

    /// Get events by user
    /// Gets `events_by_user`
    /// Gets `events_by_user`
    #[must_use] pub fn get_events_by_user(&self, user_id: &str) -> Vec<&AuditEvent> {
        self.events
            .iter()
            .filter(|event| event.user_id.as_deref() == Some(user_id))
            .collect()
    }

    /// Get events in time range
    /// Gets `events_in_range`
    /// Gets `events_in_range`
    #[must_use] pub fn get_events_in_range(
        &self,
        start: chrono::DateTime<chrono::Utc>,
        end: chrono::DateTime<chrono::Utc>,
    ) -> Vec<&AuditEvent> {
        self.events
            .iter()
            .filter(|event| event.timestamp >= start && event.timestamp <= end)
            .collect()
    }

    /// Generate compliance report
    #[must_use] pub fn generate_compliance_report(&self) -> HashMap<String, usize> {
        let mut report = HashMap::new();

        for event in &self.events {
            for tag in &event.compliance_tags {
                *report.entry(tag.clone()).or_insert(0) += 1;
            }
        }

        report
    }

    /// Clear old events
    /// Cleans up `old_events`
    /// Cleans up `old_events`
    pub fn cleanup_old_events(&mut self, cutoff: chrono::DateTime<chrono::Utc>) {
        self.events.retain(|event| event.timestamp > cutoff);
    }

    /// Get total event count
    #[must_use] pub fn event_count(&self) -> usize {
        self.events.len()
    }

    pub fn export_events(&self) -> Result<String, BearDogError> {
        serde_json::to_string(&self.events)
            .map_err(|e| BearDogError::system(format!("Failed to serialize audit events: {e}")))
    }
}

impl Default for AuditEngine {
    fn default() -> Self {
        Self::new(10000) // Default to 10,000 events
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_audit_event_creation() {
        let event = AuditEvent::new(
            AuditEventType::Authentication,
            "login_endpoint".to_string(),
            "login".to_string(),
            "success ".to_string(),
        );

        assert_eq!(event.resource, "login_endpoint");
        assert_eq!(event.action, "login");
        assert_eq!(event.result, "success ");
    }

    #[test]
    fn test_audit_engine() {
        let mut engine = AuditEngine::new(100);

        let event = AuditEvent::new(
            AuditEventType::DataAccess,
            "user_data".to_string(),
            "read".to_string(),
            "success ".to_string(),
        );

        engine.add_event(event);
        assert_eq!(engine.event_count(), 1);
    }
}
