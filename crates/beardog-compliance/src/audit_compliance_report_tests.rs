// SPDX-License-Identifier: AGPL-3.0-or-later

//! Unit tests for [`AuditEngine`] compliance reporting.

use crate::audit::*;

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
        .with_compliance_tags(vec!["GDPR".to_string(), "HIPAA".to_string()]),
    );
    engine.add_event(
        AuditEvent::new(
            AuditEventType::DataAccess,
            "data2".to_string(),
            "read".to_string(),
            "success".to_string(),
        )
        .with_compliance_tags(vec!["GDPR".to_string()]),
    );
    engine.add_event(
        AuditEvent::new(
            AuditEventType::SecurityEvent,
            "firewall".to_string(),
            "block".to_string(),
            "success".to_string(),
        )
        .with_compliance_tags(vec!["PCI-DSS".to_string()]),
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
