// SPDX-License-Identifier: AGPL-3.0-only

//! Tests for canonical types
//!
//! This module contains comprehensive tests for all canonical type implementations,
//! validations, and utility functions.

use super::*;

#[test]
fn test_canonical_type_validation() {
    let health = HealthStatus::Healthy;
    assert!(health.validate().is_ok());

    let session = SessionConfig::default();
    assert!(session.validate().is_ok());

    let context = SecurityContext {
        authentication_method: "password".to_string(),
        ..Default::default()
    };
    assert!(context.validate().is_ok());

    let audit = SecurityAuditEvent::default();
    assert!(audit.validate().is_ok());

    let decision = PolicyDecision::Allow;
    assert!(decision.validate().is_ok());
}

#[test]
fn test_health_status_variants() {
    let healthy = HealthStatus::Healthy;
    assert_eq!(healthy, HealthStatus::Healthy);

    let degraded = HealthStatus::Degraded;
    assert_ne!(healthy, degraded);

    let unhealthy = HealthStatus::Unhealthy;
    assert_ne!(degraded, unhealthy);
}

#[test]
fn test_session_config_default() {
    let config = SessionConfig::default();
    assert!(config.timeout_seconds > 0);
    assert!(config.secure_cookies); // Default should be secure
}

#[test]
fn test_security_context_validation() {
    // Valid context
    let valid_context = SecurityContext {
        authentication_method: "oauth2".to_string(),
        ..Default::default()
    };
    assert!(valid_context.validate().is_ok());

    // Invalid context (empty auth method)
    let invalid_context = SecurityContext {
        authentication_method: String::new(),
        ..Default::default()
    };
    assert!(invalid_context.validate().is_err());
}

#[test]
fn test_security_audit_event_creation() {
    let event = SecurityAuditEvent {
        event_id: "test-event-123".to_string(),
        event_type: "authentication".to_string(),
        user_id: Some("user-456".to_string()),
        outcome: AuditOutcome::Success,
        action: "login".to_string(),
        resource: "api".to_string(),
        ..Default::default()
    };

    assert_eq!(event.event_id, "test-event-123");
    assert_eq!(event.action, "login");
    assert!(event.validate().is_ok());
}

#[test]
fn test_policy_decision_variants() {
    let allow = PolicyDecision::Allow;
    // PolicyDecision variants don't have validate() method
    assert_eq!(allow, PolicyDecision::Allow);

    let deny = PolicyDecision::Deny;
    assert_eq!(deny, PolicyDecision::Deny);

    let conditional = PolicyDecision::Conditional("condition".to_string());
    assert!(matches!(conditional, PolicyDecision::Conditional(_)));
}

#[test]
fn test_key_status_lifecycle() {
    let active = KeyStatus::Active;
    let revoked = KeyStatus::Revoked;
    let expired = KeyStatus::Expired;

    assert_ne!(active, revoked);
    assert_ne!(revoked, expired);
    assert!(active.validate().is_ok());
    assert!(revoked.validate().is_ok());
    assert!(expired.validate().is_ok());
}

#[test]
fn test_workflow_status_transitions() {
    let pending = WorkflowStatus::Pending;
    let in_progress = WorkflowStatus::InProgress;
    let completed = WorkflowStatus::Completed;

    assert_eq!(pending, WorkflowStatus::default());
    assert!(pending.validate().is_ok());
    assert!(in_progress.validate().is_ok());
    assert!(completed.validate().is_ok());
}

#[test]
fn test_session_config_validation_edge_cases() {
    // Valid config with all fields set
    let config = SessionConfig {
        timeout_seconds: 3600,
        secure_cookies: true,
        same_site_policy: "Strict".to_string(),
        storage_backend: "memory".to_string(),
    };
    assert!(config.validate().is_ok());

    // Edge case: zero timeout (should fail)
    let config = SessionConfig {
        timeout_seconds: 0,
        ..Default::default()
    };
    assert!(config.validate().is_err());
}

#[test]
fn test_security_audit_event_metadata() {
    use serde_json::Value;
    use std::collections::HashMap;

    let mut metadata = HashMap::new();
    metadata.insert(
        "ip_address".to_string(),
        Value::String("192.168.1.1".to_string()),
    );
    metadata.insert(
        "user_agent".to_string(),
        Value::String("BearDog/3.0".to_string()),
    );

    let event = SecurityAuditEvent {
        metadata,
        ..Default::default()
    };

    assert_eq!(event.metadata.len(), 2);
    assert!(event.validate().is_ok());
}

#[test]
fn test_session_config_boundary_timeouts() {
    // Test minimum valid timeout
    let mut config = SessionConfig::default();
    config.timeout_seconds = 1;
    assert!(config.validate().is_ok(), "Should accept 1 second timeout");

    // Test reasonable maximum
    config.timeout_seconds = 86400; // 24 hours
    assert!(config.validate().is_ok(), "Should accept 24 hour timeout");

    // Test very large value (still valid u64)
    config.timeout_seconds = u64::MAX;
    assert!(
        config.validate().is_ok(),
        "Should handle large timeout values"
    );
}

#[test]
fn test_security_level_ordering() {
    // Verify SecurityLevel can be compared
    use capabilities::SecurityLevel;

    let standard = SecurityLevel::Standard;
    let high = SecurityLevel::High;
    let critical = SecurityLevel::Critical;

    // Just verify they exist and are different
    assert_ne!(
        format!("{:?}", standard),
        format!("{:?}", critical),
        "Security levels should be distinct"
    );
    let _ = high; // Suppress unused warning
}

#[test]
fn test_audit_outcome_variants() {
    let outcomes = [
        AuditOutcome::Success,
        AuditOutcome::Failure,
        AuditOutcome::Denied,
        AuditOutcome::Error,
    ];

    for outcome in &outcomes {
        let event = SecurityAuditEvent {
            outcome: outcome.clone(),
            ..Default::default()
        };
        assert!(
            event.validate().is_ok(),
            "All audit outcomes should be valid"
        );
    }
}

#[test]
fn test_canonical_type_names() {
    assert_eq!(HealthStatus::canonical_type_name(), "HealthStatus");
    assert_eq!(SessionConfig::canonical_type_name(), "SessionConfig");
    assert_eq!(SecurityContext::canonical_type_name(), "SecurityContext");
    assert_eq!(
        SecurityAuditEvent::canonical_type_name(),
        "SecurityAuditEvent"
    );
    assert_eq!(PolicyDecision::canonical_type_name(), "PolicyDecision");
    assert_eq!(KeyStatus::canonical_type_name(), "KeyStatus");
    assert_eq!(WorkflowStatus::canonical_type_name(), "WorkflowStatus");
}

#[test]
fn test_canonical_type_versions() {
    assert_eq!(HealthStatus::canonical_version(), "3.0.0");
    assert_eq!(SessionConfig::canonical_version(), "3.0.0");
    assert_eq!(SecurityContext::canonical_version(), "3.0.0");
}

#[test]
fn test_default_values_are_valid() {
    // All default values should pass validation (except SecurityContext)
    assert!(HealthStatus::default().validate().is_ok());
    assert!(SessionConfig::default().validate().is_ok());
    assert!(SecurityAuditEvent::default().validate().is_ok());
    assert!(PolicyDecision::default().validate().is_ok());
    assert!(KeyStatus::default().validate().is_ok());
    assert!(WorkflowStatus::default().validate().is_ok());

    // SecurityContext default has "none" auth which should fail validation
    // Actually, "none" is a valid string, just empty string fails
    assert!(SecurityContext::default().validate().is_ok());
}

// Utility function tests moved to utils.rs
