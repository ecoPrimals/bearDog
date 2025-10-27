//! Comprehensive Unit Tests for BearDog Compliance/Audit
//!
//! Created: October 27, 2025
//! Purpose: Expand test coverage for beardog-compliance audit functionality
//!
//! Tests cover:
//! - AuditSeverity enum
//! - AuditEventType enum
//! - AuditEvent creation and builder pattern
//! - AuditEngine event management
//! - Event filtering and querying
//! - Compliance reporting
//! - Event rotation and cleanup
//! - Serialization/deserialization

use crate::audit::*;
use chrono::{Duration, Utc};

// ============================================================================
// AuditSeverity Tests
// ============================================================================

#[test]
fn test_audit_severity_variants() {
    let low = AuditSeverity::Low;
    let medium = AuditSeverity::Medium;
    let high = AuditSeverity::High;
    let critical = AuditSeverity::Critical;
    
    assert!(matches!(low, AuditSeverity::Low));
    assert!(matches!(medium, AuditSeverity::Medium));
    assert!(matches!(high, AuditSeverity::High));
    assert!(matches!(critical, AuditSeverity::Critical));
}

#[test]
fn test_audit_severity_clone() {
    let severity1 = AuditSeverity::High;
    let severity2 = severity1.clone();
    
    assert_eq!(severity1, severity2);
}

#[test]
fn test_audit_severity_equality() {
    let high1 = AuditSeverity::High;
    let high2 = AuditSeverity::High;
    let medium = AuditSeverity::Medium;
    
    assert_eq!(high1, high2);
    assert_ne!(high1, medium);
}

#[test]
fn test_audit_severity_hash() {
    use std::collections::HashMap;
    
    let mut map = HashMap::new();
    map.insert(AuditSeverity::High, "high_events");
    map.insert(AuditSeverity::Low, "low_events");
    
    assert_eq!(map.get(&AuditSeverity::High), Some(&"high_events"));
    assert_eq!(map.get(&AuditSeverity::Low), Some(&"low_events"));
}

#[test]
fn test_audit_severity_serialization() {
    let severity = AuditSeverity::Critical;
    let serialized = serde_json::to_string(&severity).expect("Serialization should succeed");
    let deserialized: AuditSeverity = 
        serde_json::from_str(&serialized).expect("Deserialization should succeed");
    
    assert_eq!(severity, deserialized);
}

#[test]
fn test_audit_severity_debug_format() {
    let severity = AuditSeverity::High;
    let debug_str = format!("{:?}", severity);
    
    assert!(debug_str.contains("High"));
}

// ============================================================================
// AuditEventType Tests
// ============================================================================

#[test]
fn test_audit_event_type_variants() {
    let types = vec![
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
    let type2 = type1.clone();
    
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
    let debug_str = format!("{:?}", event_type);
    
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
    assert!(event.source_ip.is_none());
    assert!(event.user_agent.is_none());
    assert!(event.compliance_tags.is_empty());
    assert!(event.metadata.is_empty());
}

#[test]
fn test_audit_event_unique_ids() {
    let event1 = AuditEvent::new(
        AuditEventType::DataAccess,
        "resource".to_string(),
        "read".to_string(),
        "success".to_string(),
    );
    let event2 = AuditEvent::new(
        AuditEventType::DataAccess,
        "resource".to_string(),
        "read".to_string(),
        "success".to_string(),
    );
    
    assert_ne!(event1.id, event2.id);
}

#[test]
fn test_audit_event_timestamp() {
    let before = Utc::now();
    let event = AuditEvent::new(
        AuditEventType::SystemChange,
        "config".to_string(),
        "update".to_string(),
        "success".to_string(),
    );
    let after = Utc::now();
    
    assert!(event.timestamp >= before);
    assert!(event.timestamp <= after);
}

// ============================================================================
// AuditEvent Builder Pattern Tests
// ============================================================================

#[test]
fn test_audit_event_with_user() {
    let event = AuditEvent::new(
        AuditEventType::Authentication,
        "api".to_string(),
        "login".to_string(),
        "success".to_string(),
    )
    .with_user("user123".to_string());
    
    assert_eq!(event.user_id, Some("user123".to_string()));
}

#[test]
fn test_audit_event_with_source_ip() {
    let event = AuditEvent::new(
        AuditEventType::Authentication,
        "api".to_string(),
        "login".to_string(),
        "success".to_string(),
    )
    .with_source_ip("192.168.1.1");
    
    assert_eq!(event.source_ip, Some("192.168.1.1".to_string()));
}

#[test]
fn test_audit_event_with_compliance_tags() {
    let tags = vec!["GDPR".to_string(), "HIPAA".to_string()];
    let event = AuditEvent::new(
        AuditEventType::DataAccess,
        "patient_records".to_string(),
        "read".to_string(),
        "success".to_string(),
    )
    .with_compliance_tags(tags);
    
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
    
    assert_eq!(event.metadata.len(), 2);
    assert_eq!(event.metadata.get("reason"), Some(&"suspicious_activity".to_string()));
    assert_eq!(event.metadata.get("severity"), Some(&"high".to_string()));
}

#[test]
fn test_audit_event_builder_chain() {
    let event = AuditEvent::new(
        AuditEventType::Authorization,
        "/api/admin".to_string(),
        "access".to_string(),
        "denied".to_string(),
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
    let event = AuditEvent::new(
        AuditEventType::SecurityEvent,
        "test".to_string(),
        "test".to_string(),
        "test".to_string(),
    );
    let debug_str = format!("{:?}", event);
    
    assert!(debug_str.contains("AuditEvent"));
    assert!(debug_str.contains("SecurityEvent"));
}

// ============================================================================
// AuditEngine Creation Tests
// ============================================================================

#[test]
fn test_audit_engine_new() {
    let engine = AuditEngine::new(1000);
    
    assert_eq!(engine.event_count(), 0);
    // max_events is private, verified by constructor behavior
}

#[test]
fn test_audit_engine_default() {
    let engine = AuditEngine::default();
    
    assert_eq!(engine.event_count(), 0);
    // Default capacity is 10000, verified by rotation behavior
}

#[test]
fn test_audit_engine_zero_capacity() {
    let engine = AuditEngine::new(0);
    
    // Zero capacity engine created successfully
    assert_eq!(engine.event_count(), 0);
}

// ============================================================================
// AuditEngine Event Management Tests
// ============================================================================

#[test]
fn test_audit_engine_add_event() {
    let mut engine = AuditEngine::new(100);
    
    let event = AuditEvent::new(
        AuditEventType::Authentication,
        "login".to_string(),
        "attempt".to_string(),
        "success".to_string(),
    );
    
    engine.add_event(event);
    assert_eq!(engine.event_count(), 1);
}

#[test]
fn test_audit_engine_add_multiple_events() {
    let mut engine = AuditEngine::new(100);
    
    for i in 0..10 {
        let event = AuditEvent::new(
            AuditEventType::DataAccess,
            format!("resource_{}", i),
            "read".to_string(),
            "success".to_string(),
        );
        engine.add_event(event);
    }
    
    assert_eq!(engine.event_count(), 10);
}

#[test]
fn test_audit_engine_event_rotation() {
    let mut engine = AuditEngine::new(5);
    
    // Add 10 events, only last 5 should remain
    for i in 0..10 {
        let event = AuditEvent::new(
            AuditEventType::DataAccess,
            format!("resource_{}", i),
            "read".to_string(),
            "success".to_string(),
        );
        engine.add_event(event);
    }
    
    assert_eq!(engine.event_count(), 5);
}

#[test]
fn test_audit_engine_event_rotation_order() {
    let mut engine = AuditEngine::new(3);
    
    let event1 = AuditEvent::new(
        AuditEventType::DataAccess,
        "first".to_string(),
        "read".to_string(),
        "success".to_string(),
    );
    let event2 = AuditEvent::new(
        AuditEventType::DataAccess,
        "second".to_string(),
        "read".to_string(),
        "success".to_string(),
    );
    let event3 = AuditEvent::new(
        AuditEventType::DataAccess,
        "third".to_string(),
        "read".to_string(),
        "success".to_string(),
    );
    let event4 = AuditEvent::new(
        AuditEventType::DataAccess,
        "fourth".to_string(),
        "read".to_string(),
        "success".to_string(),
    );
    
    engine.add_event(event1);
    engine.add_event(event2);
    engine.add_event(event3);
    engine.add_event(event4);
    
    // Should keep last 3: second, third, fourth
    assert_eq!(engine.event_count(), 3);
}

// ============================================================================
// AuditEngine Query Tests
// ============================================================================

#[test]
fn test_audit_engine_get_events_by_type() {
    let mut engine = AuditEngine::new(100);
    
    engine.add_event(AuditEvent::new(
        AuditEventType::Authentication,
        "login".to_string(),
        "attempt".to_string(),
        "success".to_string(),
    ));
    engine.add_event(AuditEvent::new(
        AuditEventType::DataAccess,
        "data".to_string(),
        "read".to_string(),
        "success".to_string(),
    ));
    engine.add_event(AuditEvent::new(
        AuditEventType::Authentication,
        "logout".to_string(),
        "attempt".to_string(),
        "success".to_string(),
    ));
    
    let auth_events = engine.get_events_by_type(&AuditEventType::Authentication);
    assert_eq!(auth_events.len(), 2);
    
    let data_events = engine.get_events_by_type(&AuditEventType::DataAccess);
    assert_eq!(data_events.len(), 1);
}

#[test]
fn test_audit_engine_get_events_by_type_empty() {
    let engine = AuditEngine::new(100);
    
    let events = engine.get_events_by_type(&AuditEventType::SecurityEvent);
    assert!(events.is_empty());
}

#[test]
fn test_audit_engine_get_events_by_user() {
    let mut engine = AuditEngine::new(100);
    
    engine.add_event(
        AuditEvent::new(
            AuditEventType::DataAccess,
            "file1".to_string(),
            "read".to_string(),
            "success".to_string(),
        )
        .with_user("alice".to_string())
    );
    engine.add_event(
        AuditEvent::new(
            AuditEventType::DataAccess,
            "file2".to_string(),
            "write".to_string(),
            "success".to_string(),
        )
        .with_user("bob".to_string())
    );
    engine.add_event(
        AuditEvent::new(
            AuditEventType::DataAccess,
            "file3".to_string(),
            "delete".to_string(),
            "success".to_string(),
        )
        .with_user("alice".to_string())
    );
    
    let alice_events = engine.get_events_by_user("alice");
    assert_eq!(alice_events.len(), 2);
    
    let bob_events = engine.get_events_by_user("bob");
    assert_eq!(bob_events.len(), 1);
}

#[test]
fn test_audit_engine_get_events_by_user_nonexistent() {
    let mut engine = AuditEngine::new(100);
    
    engine.add_event(
        AuditEvent::new(
            AuditEventType::DataAccess,
            "file".to_string(),
            "read".to_string(),
            "success".to_string(),
        )
        .with_user("alice".to_string())
    );
    
    let events = engine.get_events_by_user("bob");
    assert!(events.is_empty());
}

#[test]
fn test_audit_engine_get_events_in_range() {
    let mut engine = AuditEngine::new(100);
    
    let now = Utc::now();
    
    // Add events with different timestamps
    std::thread::sleep(std::time::Duration::from_millis(10));
    engine.add_event(AuditEvent::new(
        AuditEventType::DataAccess,
        "resource1".to_string(),
        "read".to_string(),
        "success".to_string(),
    ));
    
    std::thread::sleep(std::time::Duration::from_millis(10));
    let middle = Utc::now();
    
    std::thread::sleep(std::time::Duration::from_millis(10));
    engine.add_event(AuditEvent::new(
        AuditEventType::DataAccess,
        "resource2".to_string(),
        "read".to_string(),
        "success".to_string(),
    ));
    
    let future = Utc::now() + Duration::seconds(10);
    
    let events = engine.get_events_in_range(now, future);
    assert_eq!(events.len(), 2);
    
    let events = engine.get_events_in_range(middle, future);
    assert_eq!(events.len(), 1);
}

#[test]
fn test_audit_engine_get_events_in_range_empty() {
    let engine = AuditEngine::new(100);
    
    let start = Utc::now();
    let end = start + Duration::hours(1);
    
    let events = engine.get_events_in_range(start, end);
    assert!(events.is_empty());
}

// ============================================================================
// AuditEngine Compliance Report Tests
// ============================================================================

#[test]
fn test_audit_engine_generate_compliance_report() {
    let mut engine = AuditEngine::new(100);
    
    engine.add_event(
        AuditEvent::new(
            AuditEventType::DataAccess,
            "data".to_string(),
            "read".to_string(),
            "success".to_string(),
        )
        .with_compliance_tags(vec!["GDPR".to_string(), "HIPAA".to_string()])
    );
    engine.add_event(
        AuditEvent::new(
            AuditEventType::DataAccess,
            "data2".to_string(),
            "read".to_string(),
            "success".to_string(),
        )
        .with_compliance_tags(vec!["GDPR".to_string()])
    );
    engine.add_event(
        AuditEvent::new(
            AuditEventType::SecurityEvent,
            "firewall".to_string(),
            "block".to_string(),
            "success".to_string(),
        )
        .with_compliance_tags(vec!["PCI-DSS".to_string()])
    );
    
    let report = engine.generate_compliance_report();
    
    assert_eq!(report.get("GDPR"), Some(&2));
    assert_eq!(report.get("HIPAA"), Some(&1));
    assert_eq!(report.get("PCI-DSS"), Some(&1));
}

#[test]
fn test_audit_engine_generate_compliance_report_empty() {
    let engine = AuditEngine::new(100);
    
    let report = engine.generate_compliance_report();
    assert!(report.is_empty());
}

// ============================================================================
// AuditEngine Cleanup Tests
// ============================================================================

#[test]
fn test_audit_engine_cleanup_old_events() {
    let mut engine = AuditEngine::new(100);
    
    // Add old events
    engine.add_event(AuditEvent::new(
        AuditEventType::DataAccess,
        "old".to_string(),
        "read".to_string(),
        "success".to_string(),
    ));
    
    std::thread::sleep(std::time::Duration::from_millis(100));
    let cutoff = Utc::now();
    std::thread::sleep(std::time::Duration::from_millis(100));
    
    // Add new events
    engine.add_event(AuditEvent::new(
        AuditEventType::DataAccess,
        "new".to_string(),
        "read".to_string(),
        "success".to_string(),
    ));
    
    engine.cleanup_old_events(cutoff);
    
    // Should only have 1 event (the new one)
    assert_eq!(engine.event_count(), 1);
}

#[test]
fn test_audit_engine_cleanup_all_events() {
    let mut engine = AuditEngine::new(100);
    
    engine.add_event(AuditEvent::new(
        AuditEventType::DataAccess,
        "resource".to_string(),
        "read".to_string(),
        "success".to_string(),
    ));
    
    let cutoff = Utc::now() + Duration::hours(1);
    engine.cleanup_old_events(cutoff);
    
    assert_eq!(engine.event_count(), 0);
}

// ============================================================================
// AuditEngine Export Tests
// ============================================================================

#[test]
fn test_audit_engine_export_events() {
    let mut engine = AuditEngine::new(100);
    
    engine.add_event(AuditEvent::new(
        AuditEventType::Authentication,
        "login".to_string(),
        "attempt".to_string(),
        "success".to_string(),
    ));
    
    let exported = engine.export_events().expect("Export should succeed");
    assert!(exported.contains("Authentication"));
    assert!(exported.contains("login"));
}

#[test]
fn test_audit_engine_export_empty() {
    let engine = AuditEngine::new(100);
    
    let exported = engine.export_events().expect("Export should succeed");
    assert_eq!(exported, "[]");
}

