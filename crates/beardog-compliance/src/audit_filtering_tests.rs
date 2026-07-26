// SPDX-License-Identifier: AGPL-3.0-or-later

//! Unit tests for [`AuditEngine`] event filtering and querying.

use crate::audit::*;
use chrono::{Duration, Utc};

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
        .with_user("alice".to_string()),
    );
    engine.add_event(
        AuditEvent::new(
            AuditEventType::DataAccess,
            "file2".to_string(),
            "write".to_string(),
            "success".to_string(),
        )
        .with_user("bob".to_string()),
    );
    engine.add_event(
        AuditEvent::new(
            AuditEventType::DataAccess,
            "file3".to_string(),
            "delete".to_string(),
            "success".to_string(),
        )
        .with_user("alice".to_string()),
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
        .with_user("alice".to_string()),
    );

    let events = engine.get_events_by_user("bob");
    assert!(events.is_empty());
}

#[test]
fn test_audit_engine_get_events_in_range() {
    let mut engine = AuditEngine::new(100);

    let now = Utc::now();

    // Add events with different timestamps using explicit timing (no sleeps!)
    let timestamp1 = now + Duration::milliseconds(10);
    engine.add_event(AuditEvent::new_with_timestamp(
        AuditEventType::DataAccess,
        "resource1".to_string(),
        "read".to_string(),
        "success".to_string(),
        timestamp1,
    ));

    let middle = now + Duration::milliseconds(20);

    let timestamp2 = now + Duration::milliseconds(30);
    engine.add_event(AuditEvent::new_with_timestamp(
        AuditEventType::DataAccess,
        "resource2".to_string(),
        "read".to_string(),
        "success".to_string(),
        timestamp2,
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
