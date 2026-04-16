// SPDX-License-Identifier: AGPL-3.0-or-later

//! Handler identity, defaults, and broad event/standard enumeration coverage.

use crate::ComplianceHandler;
use crate::compliance::types::{ComplianceConfig, ComplianceEventType, ComplianceStandard};

use super::fixtures::create_event;

#[test]
fn test_handler_clone() {
    let config = ComplianceConfig::default();
    let handler1 = ComplianceHandler::new(config);
    let handler2 = handler1.clone();

    assert_eq!(
        handler1.enabled_standards.len(),
        handler2.enabled_standards.len()
    );
    assert_eq!(handler1.audit_trail.len(), handler2.audit_trail.len());
}

#[test]
fn test_handler_debug() {
    let config = ComplianceConfig::default();
    let handler = ComplianceHandler::new(config);

    let debug_str = format!("{handler:?}");
    assert!(debug_str.contains("ComplianceHandler"));
}

#[test]
fn test_all_event_types_coverage() {
    let config = ComplianceConfig::default();
    let mut handler = ComplianceHandler::new(config);

    let all_event_types = vec![
        ComplianceEventType::DataAccess,
        ComplianceEventType::FinancialTransaction,
        ComplianceEventType::SecurityIncident,
        ComplianceEventType::SystemAccess,
        ComplianceEventType::PolicyViolation,
        ComplianceEventType::ConfigurationChange,
    ];

    for event_type in all_event_types {
        let event = create_event(event_type.clone(), ComplianceStandard::Gdpr);
        let result = handler.evaluate_compliance(&event);
        assert!(
            result.is_ok(),
            "Event type {event_type:?} should evaluate successfully"
        );
    }
}

#[test]
fn test_all_standards_with_data_access() {
    let standards = vec![
        ComplianceStandard::Gdpr,
        ComplianceStandard::Sox,
        ComplianceStandard::PciDss,
        ComplianceStandard::Pci,
        ComplianceStandard::Hipaa,
        ComplianceStandard::Iso27001,
        ComplianceStandard::Soc2,
        ComplianceStandard::Ccpa,
        ComplianceStandard::Custom("Custom".to_string()),
    ];

    for standard in standards {
        let config = ComplianceConfig {
            enabled_standards: vec![standard.clone()],
            ..Default::default()
        };
        let mut handler = ComplianceHandler::new(config);
        let event = create_event(ComplianceEventType::DataAccess, standard.clone());

        let result = handler.evaluate_compliance(&event);
        assert!(
            result.is_ok(),
            "Standard {standard:?} should evaluate successfully"
        );
    }
}

#[test]
fn test_compliance_handler_default_matches_new() {
    let a = ComplianceHandler::default();
    let b = ComplianceHandler::new(ComplianceConfig::default());
    assert_eq!(
        a.config.enabled_standards.len(),
        b.config.enabled_standards.len()
    );
}
