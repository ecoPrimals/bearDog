//! HSM Audit Event Types
//!
//! Provides audit logging types for HSM operations.

use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use std::time::SystemTime;

/// HSM audit event
///
/// Records security-relevant events in HSM operations for compliance
/// and security monitoring.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct AuditEvent {
    /// Event type identifier
    pub event_type: String,

    /// When the event occurred
    pub timestamp: SystemTime,

    /// Additional event metadata
    pub metadata: HashMap<String, String>,
}

impl AuditEvent {
    /// Creates a new audit event
    #[must_use]
    pub fn new(event_type: impl Into<String>) -> Self {
        Self {
            event_type: event_type.into(),
            timestamp: SystemTime::now(),
            metadata: HashMap::new(),
        }
    }

    /// Adds metadata to the event
    #[must_use]
    pub fn with_metadata(mut self, key: impl Into<String>, value: impl Into<String>) -> Self {
        self.metadata.insert(key.into(), value.into());
        self
    }

    /// Adds a user ID to the metadata
    #[must_use]
    pub fn with_user(self, user_id: impl Into<String>) -> Self {
        self.with_metadata("user_id", user_id)
    }

    /// Adds a resource to the metadata
    #[must_use]
    pub fn with_resource(self, resource: impl Into<String>) -> Self {
        self.with_metadata("resource", resource)
    }

    /// Adds an action to the metadata
    #[must_use]
    pub fn with_action(self, action: impl Into<String>) -> Self {
        self.with_metadata("action", action)
    }

    /// Adds an outcome to the metadata
    #[must_use]
    pub fn with_outcome(self, outcome: impl Into<String>) -> Self {
        self.with_metadata("outcome", outcome)
    }
}

impl Default for AuditEvent {
    fn default() -> Self {
        Self::new("unknown")
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_audit_event_creation() {
        let event = AuditEvent::new("key_generation");
        assert_eq!(event.event_type, "key_generation");
        assert!(event.metadata.is_empty());
    }

    #[test]
    fn test_audit_event_with_metadata() {
        let event = AuditEvent::new("key_access")
            .with_user("user123")
            .with_resource("hsm-key-001")
            .with_action("encrypt")
            .with_outcome("success");

        assert_eq!(event.metadata.get("user_id"), Some(&"user123".to_string()));
        assert_eq!(
            event.metadata.get("resource"),
            Some(&"hsm-key-001".to_string())
        );
        assert_eq!(event.metadata.get("action"), Some(&"encrypt".to_string()));
        assert_eq!(event.metadata.get("outcome"), Some(&"success".to_string()));
    }

    #[test]
    fn test_audit_event_default() {
        let event = AuditEvent::default();
        assert_eq!(event.event_type, "unknown");
    }
}
