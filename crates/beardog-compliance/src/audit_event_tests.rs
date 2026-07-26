// SPDX-License-Identifier: AGPL-3.0-or-later

//! Unit tests for [`AuditEventType`] and [`AuditEvent`] creation, builders, and serialization.

use crate::audit::*;
use chrono::Utc;

// ============================================================================
// AuditEventType Tests
// ============================================================================

#[test]
fn test_audit_event_type_variants() {
    let types = [
        AuditEventType::Authentication,
        AuditEventType::Authorization,
        AuditEventType::DataAccess,
        AuditEventType::SystemChange,
        AuditEventType::SecurityEvent,
        AuditEventType::ComplianceCheck,
    ];

    assert_eq!(types.len(), 6);
}

#[test]
fn test_audit_event_type_clone() {
    let type1 = AuditEventType::Authentication;
    let type2 = type1;

    assert!(matches!(type2, AuditEventType::Authentication));
}

#[test]
fn test_audit_event_type_serialization() {
    let event_type = AuditEventType::SecurityEvent;
    let serialized = serde_json::to_string(&event_type).expect("Serialization should succeed");
    let deserialized: AuditEventType =
        serde_json::from_str(&serialized).expect("Deserialization should succeed");

    assert!(matches!(deserialized, AuditEventType::SecurityEvent));
}

#[test]
fn test_audit_event_type_debug_format() {
    let event_type = AuditEventType::ComplianceCheck;
    let debug_str = format!("{event_type:?}");

    assert!(debug_str.contains("ComplianceCheck"));
}

// ============================================================================
// AuditEvent Creation Tests
// ============================================================================

#[test]
fn test_audit_event_new() {
    let event = AuditEvent::new(
        AuditEventType::Authentication,
        "login_endpoint".to_string(),
        "login".to_string(),
        "success".to_string(),
    );

    assert!(matches!(event.event_type, AuditEventType::Authentication));
    assert_eq!(event.resource, "login_endpoint");
    assert_eq!(event.action, "login");
    assert_eq!(event.result, "success");
    assert!(event.user_id.is_none());
    // TEST_CATEGORY: unit
    // TEST_DOMAIN: core
    // TEST_PRIORITY: normal
    assert!(event.source_ip.is_none());
    assert!(event.user_agent.is_none());
    assert!(event.compliance_tags.is_empty());
    assert!(event.metadata.is_empty());
}

#[test]
fn test_audit_event_unique_ids() {
    let event1 = AuditEvent::new(
        AuditEventType::DataAccess,
        // TEST_CATEGORY: unit
        // TEST_DOMAIN: core
        // TEST_PRIORITY: normal
        "resource".to_string(),
        "read".to_string(),
        "success".to_string(),
    );
    let event2 = AuditEvent::new(
        // TEST_CATEGORY: unit
        // TEST_DOMAIN: core
        // TEST_PRIORITY: normal
        AuditEventType::DataAccess,
        "resource".to_string(),
        "read".to_string(),
        "success".to_string(),
    );

    assert_ne!(event1.id, event2.id);
    // TEST_CATEGORY: unit
    // TEST_DOMAIN: core
    // TEST_PRIORITY: normal
}

#[test]
fn test_audit_event_timestamp() {
    let before = Utc::now();
    let event = AuditEvent::new(
        AuditEventType::SystemChange,
        "config".to_string(),
        "update".to_string(),
        // TEST_CATEGORY: unit
        // TEST_DOMAIN: core
        // TEST_PRIORITY: normal
        "success".to_string(),
    );
    let after = Utc::now();

    assert!(event.timestamp >= before);
    assert!(event.timestamp <= after);
}
// TEST_CATEGORY: unit
// TEST_DOMAIN: core
// TEST_PRIORITY: normal

// ============================================================================
// AuditEvent Builder Pattern Tests
// ============================================================================

#[test]
fn test_audit_event_with_user() {
    let event = AuditEvent::new(
        AuditEventType::Authentication,
        // TEST_CATEGORY: unit
        // TEST_DOMAIN: core
        // TEST_PRIORITY: normal
        "api".to_string(),
        "login".to_string(),
        "success".to_string(),
    )
    .with_user("user123".to_string());

    assert_eq!(event.user_id, Some("user123".to_string()));
}

#[test]
fn test_audit_event_with_source_ip() {
    // TEST_CATEGORY: unit
    // TEST_DOMAIN: core
    // TEST_PRIORITY: normal
    let event = AuditEvent::new(
        AuditEventType::Authentication,
        "api".to_string(),
        "login".to_string(),
        "success".to_string(),
        // TEST_CATEGORY: unit
        // TEST_DOMAIN: core
        // TEST_PRIORITY: normal
    )
    .with_source_ip("192.168.1.1");

    assert_eq!(event.source_ip, Some("192.168.1.1".to_string()));
}

#[test]
// TEST_CATEGORY: unit
// TEST_DOMAIN: core
// TEST_PRIORITY: normal
fn test_audit_event_with_compliance_tags() {
    let tags = vec!["GDPR".to_string(), "HIPAA".to_string()];
    let event = AuditEvent::new(
        AuditEventType::DataAccess,
        "patient_records".to_string(),
        "read".to_string(),
        "success".to_string(),
    )
    .with_compliance_tags(tags);
    // TEST_CATEGORY: unit
    // TEST_DOMAIN: core
    // TEST_PRIORITY: normal

    assert_eq!(event.compliance_tags.len(), 2);
    assert!(event.compliance_tags.contains(&"GDPR".to_string()));
    assert!(event.compliance_tags.contains(&"HIPAA".to_string()));
}

#[test]
fn test_audit_event_with_metadata() {
    let event = AuditEvent::new(
        AuditEventType::SecurityEvent,
        "firewall".to_string(),
        "block".to_string(),
        "success".to_string(),
    )
    .with_metadata("reason".to_string(), "suspicious_activity".to_string())
    .with_metadata("severity".to_string(), "high".to_string());

    // TEST_CATEGORY: unit
    // TEST_DOMAIN: core
    // TEST_PRIORITY: normal
    assert_eq!(event.metadata.len(), 2);
    assert_eq!(
        event.metadata.get("reason"),
        Some(&"suspicious_activity".to_string())
    );
    assert_eq!(event.metadata.get("severity"), Some(&"high".to_string()));
}

#[test]
fn test_audit_event_builder_chain() {
    let event = AuditEvent::new(
        AuditEventType::Authorization,
        "/api/admin".to_string(),
        "access".to_string(),
        "denied".to_string(),
        // TEST_CATEGORY: unit
        // TEST_DOMAIN: core
        // TEST_PRIORITY: normal
    )
    .with_user("user456".to_string())
    .with_source_ip("10.0.0.1")
    .with_compliance_tags(vec!["SOC2".to_string()])
    .with_metadata("role".to_string(), "user".to_string());

    assert_eq!(event.user_id, Some("user456".to_string()));
    assert_eq!(event.source_ip, Some("10.0.0.1".to_string()));
    assert_eq!(event.compliance_tags.len(), 1);
    assert_eq!(event.metadata.len(), 1);
}

#[test]
fn test_audit_event_empty_compliance_tags() {
    let event = AuditEvent::new(
        AuditEventType::DataAccess,
        // TEST_CATEGORY: unit
        // TEST_DOMAIN: core
        // TEST_PRIORITY: normal
        "resource".to_string(),
        "read".to_string(),
        "success".to_string(),
    )
    .with_compliance_tags(vec![]);

    assert!(event.compliance_tags.is_empty());
}

// ============================================================================
// AuditEvent Serialization Tests
// ============================================================================

#[test]
fn test_audit_event_serialization() {
    let event = AuditEvent::new(
        AuditEventType::ComplianceCheck,
        "policy_engine".to_string(),
        "validate".to_string(),
        "pass".to_string(),
        // TEST_CATEGORY: unit
        // TEST_DOMAIN: core
        // TEST_PRIORITY: normal
    )
    .with_user("system".to_string())
    .with_compliance_tags(vec!["PCI-DSS".to_string()]);

    let serialized = serde_json::to_string(&event).expect("Serialization should succeed");
    assert!(serialized.contains("ComplianceCheck"));
    assert!(serialized.contains("policy_engine"));
    assert!(serialized.contains("system"));
    assert!(serialized.contains("PCI-DSS"));
}

#[test]
fn test_audit_event_deserialization() {
    // TEST_CATEGORY: unit
    // TEST_DOMAIN: core
    // TEST_PRIORITY: normal
    let event = AuditEvent::new(
        AuditEventType::SystemChange,
        "config".to_string(),
        "update".to_string(),
        "success".to_string(),
    );

    let serialized = serde_json::to_string(&event).expect("Serialization should succeed");
    let deserialized: AuditEvent =
        serde_json::from_str(&serialized).expect("Deserialization should succeed");

    assert_eq!(event.resource, deserialized.resource);
    assert_eq!(event.action, deserialized.action);
    assert_eq!(event.result, deserialized.result);
}

// TEST_CATEGORY: unit
// TEST_DOMAIN: core
// TEST_PRIORITY: normal
#[test]
fn test_audit_event_clone() {
    let event1 = AuditEvent::new(
        AuditEventType::DataAccess,
        "database".to_string(),
        "query".to_string(),
        "success".to_string(),
    );
    let event2 = event1.clone();

    assert_eq!(event1.id, event2.id);
    assert_eq!(event1.resource, event2.resource);
}

#[test]
fn test_audit_event_debug_format() {
    // TEST_CATEGORY: unit
    // TEST_DOMAIN: core
    // TEST_PRIORITY: normal
    let event = AuditEvent::new(
        AuditEventType::SecurityEvent,
        "test".to_string(),
        "test".to_string(),
        "test".to_string(),
    );
    let debug_str = format!("{event:?}");

    assert!(debug_str.contains("AuditEvent"));
    assert!(debug_str.contains("SecurityEvent"));
}
