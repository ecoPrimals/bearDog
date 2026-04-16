// SPDX-License-Identifier: AGPL-3.0-or-later

//! Coverage for per-standard evaluation paths.

use crate::ComplianceHandler;
use crate::compliance::types::{ComplianceConfig, ComplianceEventType, ComplianceStandard};

use super::fixtures::create_event;

#[test]
fn test_iso27001_standard_evaluation() {
    let config = ComplianceConfig {
        enabled_standards: vec![ComplianceStandard::Iso27001],
        ..Default::default()
    };
    let mut handler = ComplianceHandler::new(config);
    let event = create_event(
        ComplianceEventType::DataAccess,
        ComplianceStandard::Iso27001,
    );

    let result = handler
        .evaluate_compliance(&event)
        .expect("compliance evaluation should succeed for test event");

    assert!(result.score >= 0.0);
    assert!(result.score <= 100.0);
}

#[test]
fn test_soc2_standard_evaluation() {
    let config = ComplianceConfig {
        enabled_standards: vec![ComplianceStandard::Soc2],
        ..Default::default()
    };
    let mut handler = ComplianceHandler::new(config);
    let event = create_event(ComplianceEventType::SystemAccess, ComplianceStandard::Soc2);

    let result = handler
        .evaluate_compliance(&event)
        .expect("compliance evaluation should succeed for test event");

    assert!(result.score >= 0.0);
}

#[test]
fn test_ccpa_standard_evaluation() {
    let config = ComplianceConfig {
        enabled_standards: vec![ComplianceStandard::Ccpa],
        ..Default::default()
    };
    let mut handler = ComplianceHandler::new(config);
    let event = create_event(ComplianceEventType::DataAccess, ComplianceStandard::Ccpa);

    let result = handler
        .evaluate_compliance(&event)
        .expect("compliance evaluation should succeed for test event");

    assert!(result.score >= 0.0);
}

#[test]
fn test_pci_standard_evaluation() {
    let config = ComplianceConfig {
        enabled_standards: vec![ComplianceStandard::Pci],
        ..Default::default()
    };
    let mut handler = ComplianceHandler::new(config);
    let event = create_event(
        ComplianceEventType::FinancialTransaction,
        ComplianceStandard::Pci,
    );

    let result = handler
        .evaluate_compliance(&event)
        .expect("compliance evaluation should succeed for test event");

    assert!(result.score >= 0.0);
}

#[test]
fn test_custom_standard_evaluation() {
    let config = ComplianceConfig {
        enabled_standards: vec![ComplianceStandard::Custom("TestStandard".to_string())],
        ..Default::default()
    };
    let mut handler = ComplianceHandler::new(config);
    let event = create_event(
        ComplianceEventType::DataAccess,
        ComplianceStandard::Custom("TestStandard".to_string()),
    );

    let result = handler
        .evaluate_compliance(&event)
        .expect("compliance evaluation should succeed for test event");

    assert!(result.score >= 0.0);
    assert!(!result.recommendations.is_empty());
}
