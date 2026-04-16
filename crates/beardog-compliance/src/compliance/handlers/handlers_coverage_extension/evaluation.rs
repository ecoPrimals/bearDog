// SPDX-License-Identifier: AGPL-3.0-or-later

//! Multi-standard evaluation, scoring, audit trail, metrics, and domain-specific checks.

use crate::ComplianceHandler;
use crate::compliance::types::{ComplianceConfig, ComplianceEventType, ComplianceStandard};

use super::fixtures::create_event;

#[test]
fn test_multiple_standards_multiple_events() {
    let config = ComplianceConfig {
        enabled_standards: vec![
            ComplianceStandard::Gdpr,
            ComplianceStandard::Sox,
            ComplianceStandard::Hipaa,
            ComplianceStandard::PciDss,
            ComplianceStandard::Iso27001,
            ComplianceStandard::Soc2,
            ComplianceStandard::Ccpa,
        ],
        ..Default::default()
    };
    let mut handler = ComplianceHandler::new(config);

    let event_types = vec![
        ComplianceEventType::DataAccess,
        ComplianceEventType::FinancialTransaction,
        ComplianceEventType::SecurityIncident,
        ComplianceEventType::SystemAccess,
        ComplianceEventType::PolicyViolation,
    ];

    for event_type in event_types {
        let event = create_event(event_type, ComplianceStandard::Gdpr);
        let result = handler.evaluate_compliance(&event);
        assert!(result.is_ok());
    }

    assert_eq!(handler.audit_trail.len(), 5);
}

#[test]
fn test_compliance_score_calculation() {
    let config = ComplianceConfig {
        enabled_standards: vec![ComplianceStandard::Gdpr],
        ..Default::default()
    };
    let mut handler = ComplianceHandler::new(config);
    let event = create_event(ComplianceEventType::DataAccess, ComplianceStandard::Gdpr);

    let result = handler
        .evaluate_compliance(&event)
        .expect("compliance evaluation should succeed for test event");

    // Score should be between 0 and 100
    assert!(result.score >= 0.0);
    assert!(result.score <= 100.0);
}

#[test]
fn test_audit_trail_details() {
    let config = ComplianceConfig::default();
    let mut handler = ComplianceHandler::new(config);
    let event = create_event(ComplianceEventType::DataAccess, ComplianceStandard::Gdpr);

    let _ = handler.evaluate_compliance(&event);

    assert_eq!(handler.audit_trail.len(), 1);
    let audit_entry = &handler.audit_trail[0];

    // Verify audit entry structure
    assert!(!audit_entry.id.is_empty());
    assert_eq!(audit_entry.user_id, Some("system".to_string()));
    assert!(audit_entry.action.contains("compliance_evaluation"));

    // Verify details contain expected fields
    if let serde_json::Value::Object(details) = &audit_entry.details {
        assert!(details.contains_key("event_id"));
        assert!(details.contains_key("violations_count"));
        assert!(details.contains_key("score"));
    }
}

#[test]
fn test_generate_metrics_with_audit_trail() {
    let config = ComplianceConfig {
        enabled_standards: vec![
            ComplianceStandard::Gdpr,
            ComplianceStandard::Sox,
            ComplianceStandard::Hipaa,
        ],
        ..Default::default()
    };
    let mut handler = ComplianceHandler::new(config);

    // Add evaluations to populate audit trail
    for i in 0..10 {
        let event_type = match i % 4 {
            0 => ComplianceEventType::DataAccess,
            1 => ComplianceEventType::FinancialTransaction,
            2 => ComplianceEventType::SecurityIncident,
            _ => ComplianceEventType::SystemAccess,
        };
        let event = create_event(event_type, ComplianceStandard::Gdpr);
        let _ = handler.evaluate_compliance(&event);
    }

    let metrics = handler.generate_metrics();

    assert!(
        metrics.overall_score > 0.0 && metrics.overall_score <= 100.0,
        "score should be in (0, 100] range after evaluations, got {}",
        metrics.overall_score
    );
    assert_eq!(metrics.audit_trail_size, 10);
    assert_eq!(metrics.standards_compliance.len(), 3);
    assert!(metrics.last_assessment_date.is_some());
    assert!(metrics.next_assessment_due.is_some());
}

#[test]
fn test_check_sovereignty_compliance() {
    let config = ComplianceConfig::default();
    let handler = ComplianceHandler::new(config);
    let event = create_event(ComplianceEventType::DataAccess, ComplianceStandard::Gdpr);

    let result = handler.check_sovereignty_compliance(&event);

    assert!(result.is_ok());
    let issues = result.expect("sovereignty check should succeed");
    assert!(issues.is_empty());
}

#[test]
fn test_check_privacy_compliance() {
    let config = ComplianceConfig::default();
    let handler = ComplianceHandler::new(config);
    let event = create_event(ComplianceEventType::DataAccess, ComplianceStandard::Gdpr);

    let result = handler.check_privacy_compliance(&event);

    assert!(result.is_ok());
    let issues = result.expect("privacy check should succeed");
    assert!(issues.is_empty());
}
