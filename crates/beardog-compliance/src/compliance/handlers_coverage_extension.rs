// SPDX-License-Identifier: AGPL-3.0-or-later

//! Coverage extension tests for `ComplianceHandler`
//!
//! Added December 8, 2025 to increase coverage from 65.91% to 90%+
//! Targets: `generate_recommendations`, edge cases, all compliance standards

#[cfg(test)]
mod handlers_coverage_extension_tests {
    use crate::ComplianceHandler;
    use crate::compliance::types::{
        AuditOutcome, ComplianceConfig, ComplianceEvent, ComplianceEventType, ComplianceSeverity,
        ComplianceStandard,
    };
    use chrono::Utc;
    use uuid::Uuid;

    fn create_event(
        event_type: ComplianceEventType,
        standard: ComplianceStandard,
    ) -> ComplianceEvent {
        let description = format!("Test event: {event_type:?}");
        ComplianceEvent {
            id: Uuid::new_v4().to_string(),
            event_type,
            timestamp: Utc::now(),
            description,
            severity: ComplianceSeverity::Medium,
            standard,
            metadata: serde_json::json!({}),
        }
    }

    // ============================================================================
    // generate_recommendations coverage (all event types)
    // ============================================================================

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

    // ============================================================================
    // All compliance standards evaluation paths
    // ============================================================================

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

    // ============================================================================
    // Edge cases and error paths
    // ============================================================================

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

    // ============================================================================
    // Simulation metadata: violations, audit failure, standard evaluation Err
    // ============================================================================

    fn event_with_simulation(
        event_type: ComplianceEventType,
        standard: ComplianceStandard,
        simulation: serde_json::Value,
    ) -> ComplianceEvent {
        let mut event = create_event(event_type, standard);
        event.metadata = simulation;
        event
    }

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
}
