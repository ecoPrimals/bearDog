// SPDX-License-Identifier: AGPL-3.0-or-later

//! Metadata-driven simulation paths: violations, audit outcomes, and standard-specific rules.

use crate::ComplianceHandler;
use crate::compliance::types::{
    AuditOutcome, ComplianceConfig, ComplianceEventType, ComplianceStandard,
};

use super::fixtures::{create_event, event_with_simulation};

#[test]
fn simulated_standard_failure_reduces_score_and_skips_violation_merge() {
    let config = ComplianceConfig {
        enabled_standards: vec![ComplianceStandard::Gdpr],
        ..Default::default()
    };
    let mut handler = ComplianceHandler::new(config);
    let event = event_with_simulation(
        ComplianceEventType::DataAccess,
        ComplianceStandard::Gdpr,
        serde_json::json!({ "compliance_simulation": { "standard_evaluation_failure": true } }),
    );

    let result = handler
        .evaluate_compliance(&event)
        .expect("evaluation succeeds; Err is per-standard only");

    assert!(
        (result.score - 90.0).abs() < f64::EPSILON,
        "expected -10 for failed standard, got {}",
        result.score
    );
    assert!(result.violations.is_empty());
    assert!(result.passed);
}

#[test]
fn simulated_missing_consent_produces_violation_and_audit_failure() {
    let config = ComplianceConfig {
        enabled_standards: vec![ComplianceStandard::Gdpr],
        ..Default::default()
    };
    let mut handler = ComplianceHandler::new(config);
    let event = event_with_simulation(
        ComplianceEventType::DataAccess,
        ComplianceStandard::Gdpr,
        serde_json::json!({ "compliance_simulation": { "missing_consent": true } }),
    );

    let result = handler
        .evaluate_compliance(&event)
        .expect("compliance evaluation");

    assert!(!result.passed);
    assert!(!result.violations.is_empty());
    assert_eq!(handler.audit_trail.len(), 1);
    assert_eq!(handler.audit_trail[0].outcome, AuditOutcome::Failure);
}

#[test]
fn simulated_pci_payment_data_critical_violation() {
    let config = ComplianceConfig {
        enabled_standards: vec![ComplianceStandard::PciDss],
        ..Default::default()
    };
    let mut handler = ComplianceHandler::new(config);
    let event = event_with_simulation(
        ComplianceEventType::SystemAccess,
        ComplianceStandard::PciDss,
        serde_json::json!({ "compliance_simulation": { "payment_data": true } }),
    );

    let result = handler.evaluate_compliance(&event).expect("ok");
    assert!(!result.passed);
    assert!(
        result
            .violations
            .iter()
            .any(|v| v.contains("PCI") || v.contains("payment")),
        "{:?}",
        result.violations
    );
}

#[test]
fn simulated_sox_minimum_necessary_violation() {
    let config = ComplianceConfig {
        enabled_standards: vec![ComplianceStandard::Sox],
        ..Default::default()
    };
    let mut handler = ComplianceHandler::new(config);
    let event = event_with_simulation(
        ComplianceEventType::FinancialTransaction,
        ComplianceStandard::Sox,
        serde_json::json!({ "compliance_simulation": { "minimum_necessary_violation": true } }),
    );

    let result = handler.evaluate_compliance(&event).expect("ok");
    assert!(!result.passed);
    assert!(
        result
            .violations
            .iter()
            .any(|v| v.contains("SOX") || v.contains("minimum"))
    );
}

#[test]
fn simulated_data_minimization_hits_iso27001_extend_path() {
    let config = ComplianceConfig {
        enabled_standards: vec![ComplianceStandard::Iso27001],
        ..Default::default()
    };
    let mut handler = ComplianceHandler::new(config);
    let event = event_with_simulation(
        ComplianceEventType::AuditEvent,
        ComplianceStandard::Iso27001,
        serde_json::json!({ "compliance_simulation": { "data_minimization_violation": true } }),
    );

    let result = handler.evaluate_compliance(&event).expect("ok");
    assert!(!result.passed);
    assert!(
        result
            .violations
            .iter()
            .any(|v| v.contains("minim") || v.contains("Data")),
        "{:?}",
        result.violations
    );
}

#[test]
fn score_clamps_at_zero_with_many_simulated_failures() {
    let standards: Vec<ComplianceStandard> = (0..8).map(|_| ComplianceStandard::Gdpr).collect();
    let config = ComplianceConfig {
        enabled_standards: standards,
        ..Default::default()
    };
    let mut handler = ComplianceHandler::new(config);
    let event = event_with_simulation(
        ComplianceEventType::DataAccess,
        ComplianceStandard::Gdpr,
        serde_json::json!({
            "compliance_simulation": {
                "missing_consent": true,
                "data_minimization_violation": true
            }
        }),
    );

    let result = handler.evaluate_compliance(&event).expect("ok");
    assert!(
        result.score <= f64::EPSILON,
        "expected score clamped to 0, got {}",
        result.score
    );
}

#[test]
fn compliance_simulation_non_bool_flags_are_ignored() {
    let config = ComplianceConfig {
        enabled_standards: vec![ComplianceStandard::Gdpr],
        ..Default::default()
    };
    let mut handler = ComplianceHandler::new(config);
    let mut event = create_event(ComplianceEventType::DataAccess, ComplianceStandard::Gdpr);
    event.metadata = serde_json::json!({
        "compliance_simulation": { "missing_consent": "not-a-bool" }
    });

    let result = handler.evaluate_compliance(&event).expect("ok");
    assert!(result.passed);
    assert!(result.violations.is_empty());
}

#[test]
fn hipaa_data_access_minimum_necessary_and_minimization_violations() {
    let config = ComplianceConfig {
        enabled_standards: vec![ComplianceStandard::Hipaa],
        ..Default::default()
    };
    let mut handler = ComplianceHandler::new(config);
    let mut event = create_event(ComplianceEventType::DataAccess, ComplianceStandard::Hipaa);
    event.metadata = serde_json::json!({
        "compliance_simulation": {
            "minimum_necessary_violation": true,
            "data_minimization_violation": true
        }
    });

    let result = handler.evaluate_compliance(&event).expect("ok");
    assert!(!result.passed);
    assert!(result.violations.len() >= 2);
    assert!(
        result
            .violations
            .iter()
            .any(|v| v.contains("HIPAA") || v.contains("Minimum")),
        "{:?}",
        result.violations
    );
}

#[test]
fn soc2_minimum_necessary_violation_simulation() {
    let config = ComplianceConfig {
        enabled_standards: vec![ComplianceStandard::Soc2],
        ..Default::default()
    };
    let mut handler = ComplianceHandler::new(config);
    let event = event_with_simulation(
        ComplianceEventType::AuditEvent,
        ComplianceStandard::Soc2,
        serde_json::json!({ "compliance_simulation": { "minimum_necessary_violation": true } }),
    );

    let result = handler.evaluate_compliance(&event).expect("ok");
    assert!(!result.passed);
    assert!(
        result
            .violations
            .iter()
            .any(|v| v.contains("SOC2") || v.contains("access"))
    );
}

#[test]
fn pci_enum_variant_triggers_payment_violation_like_pci_dss() {
    let config = ComplianceConfig {
        enabled_standards: vec![ComplianceStandard::Pci],
        ..Default::default()
    };
    let mut handler = ComplianceHandler::new(config);
    let event = event_with_simulation(
        ComplianceEventType::DataAccess,
        ComplianceStandard::Pci,
        serde_json::json!({ "compliance_simulation": { "payment_data": true } }),
    );

    let result = handler.evaluate_compliance(&event).expect("ok");
    assert!(!result.passed);
    assert!(
        result
            .violations
            .iter()
            .any(|v| v.contains("PCI") || v.contains("payment")),
        "{:?}",
        result.violations
    );
}

#[test]
fn gdpr_system_access_skips_data_access_only_rules() {
    let config = ComplianceConfig {
        enabled_standards: vec![ComplianceStandard::Gdpr],
        ..Default::default()
    };
    let mut handler = ComplianceHandler::new(config);
    let event = event_with_simulation(
        ComplianceEventType::SystemAccess,
        ComplianceStandard::Gdpr,
        serde_json::json!({
            "compliance_simulation": {
                "missing_consent": true,
                "data_minimization_violation": true
            }
        }),
    );

    let result = handler.evaluate_compliance(&event).expect("ok");
    assert!(result.passed);
    assert!(result.violations.is_empty());
}

#[test]
fn sox_non_financial_event_ignores_minimum_necessary_simulation() {
    let config = ComplianceConfig {
        enabled_standards: vec![ComplianceStandard::Sox],
        ..Default::default()
    };
    let mut handler = ComplianceHandler::new(config);
    let event = event_with_simulation(
        ComplianceEventType::SystemAccess,
        ComplianceStandard::Sox,
        serde_json::json!({ "compliance_simulation": { "minimum_necessary_violation": true } }),
    );

    let result = handler.evaluate_compliance(&event).expect("ok");
    assert!(result.passed);
    assert!(result.violations.is_empty());
}

#[test]
fn ccpa_missing_consent_and_data_minimization_combine() {
    let config = ComplianceConfig {
        enabled_standards: vec![ComplianceStandard::Ccpa],
        ..Default::default()
    };
    let mut handler = ComplianceHandler::new(config);
    let mut event = create_event(ComplianceEventType::DataAccess, ComplianceStandard::Ccpa);
    event.metadata = serde_json::json!({
        "compliance_simulation": {
            "missing_consent": true,
            "data_minimization_violation": true
        }
    });

    let result = handler.evaluate_compliance(&event).expect("ok");
    assert!(!result.passed);
    assert!(result.violations.len() >= 2);
}

#[test]
fn custom_standard_consent_and_minimization_combine() {
    let std = ComplianceStandard::Custom("OrgPolicy".to_string());
    let config = ComplianceConfig {
        enabled_standards: vec![std.clone()],
        ..Default::default()
    };
    let mut handler = ComplianceHandler::new(config);
    let mut event = create_event(ComplianceEventType::DataAccess, std);
    event.metadata = serde_json::json!({
        "compliance_simulation": {
            "missing_consent": true,
            "data_minimization_violation": true
        }
    });

    let result = handler.evaluate_compliance(&event).expect("ok");
    assert!(!result.passed);
    assert!(result.violations.len() >= 2);
}

#[test]
fn multiple_standards_with_violations_accumulate_and_lower_score() {
    let config = ComplianceConfig {
        enabled_standards: vec![ComplianceStandard::Gdpr, ComplianceStandard::Iso27001],
        ..Default::default()
    };
    let mut handler = ComplianceHandler::new(config);
    let mut event = create_event(ComplianceEventType::DataAccess, ComplianceStandard::Gdpr);
    event.metadata = serde_json::json!({
        "compliance_simulation": { "missing_consent": true }
    });

    let result = handler.evaluate_compliance(&event).expect("ok");
    assert!(!result.passed);
    assert!(result.score < 100.0);
    assert_eq!(handler.audit_trail.len(), 1);
    assert_eq!(handler.audit_trail[0].outcome, AuditOutcome::Failure);
}

#[test]
fn hipaa_non_data_access_skips_hipaa_specific_checks() {
    let config = ComplianceConfig {
        enabled_standards: vec![ComplianceStandard::Hipaa],
        ..Default::default()
    };
    let mut handler = ComplianceHandler::new(config);
    let event = event_with_simulation(
        ComplianceEventType::FinancialTransaction,
        ComplianceStandard::Hipaa,
        serde_json::json!({
            "compliance_simulation": {
                "minimum_necessary_violation": true,
                "data_minimization_violation": true
            }
        }),
    );

    let result = handler.evaluate_compliance(&event).expect("ok");
    assert!(result.passed);
    assert!(result.violations.is_empty());
}
