// SPDX-License-Identifier: AGPL-3.0-or-later

//! Coverage for `generate_recommendations` across event types.

use crate::ComplianceHandler;
use crate::compliance::types::{ComplianceConfig, ComplianceEventType, ComplianceStandard};

use super::fixtures::create_event;

#[test]
fn test_recommendations_data_access() {
    let config = ComplianceConfig::default();
    let mut handler = ComplianceHandler::new(config);
    let event = create_event(ComplianceEventType::DataAccess, ComplianceStandard::Gdpr);

    let result = handler
        .evaluate_compliance(&event)
        .expect("compliance evaluation should succeed for test event");

    assert!(!result.recommendations.is_empty());
    assert!(
        result
            .recommendations
            .iter()
            .any(|r| r.contains("access logging"))
    );
    assert!(result.recommendations.iter().any(|r| r.contains("review")));
    assert!(
        result
            .recommendations
            .iter()
            .any(|r| r.contains("permissions"))
    );
}

#[test]
fn test_recommendations_financial_transaction() {
    let config = ComplianceConfig::default();
    let mut handler = ComplianceHandler::new(config);
    let event = create_event(
        ComplianceEventType::FinancialTransaction,
        ComplianceStandard::Sox,
    );

    let result = handler
        .evaluate_compliance(&event)
        .expect("compliance evaluation should succeed for test event");

    assert!(!result.recommendations.is_empty());
    assert!(
        result
            .recommendations
            .iter()
            .any(|r| r.contains("dual approval"))
    );
    assert!(
        result
            .recommendations
            .iter()
            .any(|r| r.contains("audit trail"))
    );
    assert!(
        result
            .recommendations
            .iter()
            .any(|r| r.contains("financial controls"))
    );
}

#[test]
fn test_recommendations_security_incident() {
    let config = ComplianceConfig::default();
    let mut handler = ComplianceHandler::new(config);
    let event = create_event(
        ComplianceEventType::SecurityIncident,
        ComplianceStandard::Iso27001,
    );

    let result = handler
        .evaluate_compliance(&event)
        .expect("compliance evaluation should succeed for test event");

    assert!(!result.recommendations.is_empty());
    assert!(
        result
            .recommendations
            .iter()
            .any(|r| r.contains("incident response"))
    );
    assert!(
        result
            .recommendations
            .iter()
            .any(|r| r.contains("post-incident"))
    );
    assert!(
        result
            .recommendations
            .iter()
            .any(|r| r.contains("security controls"))
    );
}

#[test]
fn test_recommendations_system_access() {
    let config = ComplianceConfig::default();
    let mut handler = ComplianceHandler::new(config);
    let event = create_event(ComplianceEventType::SystemAccess, ComplianceStandard::Soc2);

    let result = handler
        .evaluate_compliance(&event)
        .expect("compliance evaluation should succeed for test event");

    assert!(!result.recommendations.is_empty());
    assert!(
        result
            .recommendations
            .iter()
            .any(|r| r.contains("access logging"))
    );
    assert!(
        result
            .recommendations
            .iter()
            .any(|r| r.contains("permissions"))
    );
    assert!(
        result
            .recommendations
            .iter()
            .any(|r| r.contains("unusual access"))
    );
}

#[test]
fn test_recommendations_policy_violation() {
    let config = ComplianceConfig::default();
    let mut handler = ComplianceHandler::new(config);
    let event = create_event(
        ComplianceEventType::PolicyViolation,
        ComplianceStandard::Gdpr,
    );

    let result = handler
        .evaluate_compliance(&event)
        .expect("compliance evaluation should succeed for test event");

    // Default recommendations for unhandled event types
    assert!(!result.recommendations.is_empty());
    assert!(
        result
            .recommendations
            .iter()
            .any(|r| r.contains("compliance policies"))
    );
    assert!(
        result
            .recommendations
            .iter()
            .any(|r| r.contains("monitoring"))
    );
}

#[test]
fn test_recommendations_configuration_change() {
    let config = ComplianceConfig::default();
    let mut handler = ComplianceHandler::new(config);
    let event = create_event(
        ComplianceEventType::ConfigurationChange,
        ComplianceStandard::Soc2,
    );

    let result = handler
        .evaluate_compliance(&event)
        .expect("compliance evaluation should succeed for test event");

    // Default recommendations
    assert!(!result.recommendations.is_empty());
}
