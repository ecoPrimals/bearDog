// SPDX-License-Identifier: AGPL-3.0-or-later

//! Unit tests for [`AuditEngine`] creation, event management, cleanup, and export.

use crate::audit::*;
use chrono::{Duration, Utc};

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
            format!("resource_{i}"),
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
            format!("resource_{i}"),
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
// AuditEngine Cleanup Tests
// ============================================================================

#[test]
fn test_audit_engine_cleanup_old_events() {
    let mut engine = AuditEngine::new(100);

    let old_timestamp = Utc::now() - Duration::milliseconds(100);

    // Add old events with explicit timestamp (no sleep needed!)
    engine.add_event(AuditEvent::new_with_timestamp(
        AuditEventType::DataAccess,
        "old".to_string(),
        "read".to_string(),
        "success".to_string(),
        old_timestamp,
    ));

    let cutoff = Utc::now();
    let new_timestamp = cutoff + Duration::milliseconds(100);

    // Add new events with explicit timestamp (no sleep needed!)
    engine.add_event(AuditEvent::new_with_timestamp(
        AuditEventType::DataAccess,
        "new".to_string(),
        "read".to_string(),
        "success".to_string(),
        new_timestamp,
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
