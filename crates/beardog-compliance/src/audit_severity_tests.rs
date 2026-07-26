// SPDX-License-Identifier: AGPL-3.0-or-later

//! Unit tests for [`AuditSeverity`].

use crate::audit::*;

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
    let debug_str = format!("{severity:?}");

    assert!(debug_str.contains("High"));
}
