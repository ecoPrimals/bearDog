// SPDX-License-Identifier: AGPL-3.0-or-later

// Compliance Handler Tests
// Focus: Policy validation, enforcement, audit logging, standards evaluation

#[cfg(test)]
mod compliance_handler_tests {
    use super::super::handlers::ComplianceHandler;
    use crate::compliance::types::{
        ComplianceConfig, ComplianceEvent, ComplianceEventType, ComplianceSeverity,
        ComplianceStandard,
    };
    use chrono::Utc;
    use uuid::Uuid;

    fn create_test_config() -> ComplianceConfig {
        ComplianceConfig {
            enabled_standards: vec![
                ComplianceStandard::Gdpr,
                ComplianceStandard::Sox,
                ComplianceStandard::Hipaa,
            ],
            ..Default::default()
        }
    }

    fn create_test_event(event_type: ComplianceEventType) -> ComplianceEvent {
        ComplianceEvent {
            id: Uuid::new_v4().to_string(),
            event_type,
            standard: ComplianceStandard::Gdpr,
            description: "Test event".to_string(),
            severity: ComplianceSeverity::Medium,
            timestamp: Utc::now(),
            metadata: serde_json::json!({}),
        }
    }

    #[test]
    fn test_compliance_handler_creation() {
        let config = create_test_config();
        let handler = ComplianceHandler::new(config);

        assert_eq!(handler.enabled_standards.len(), 3);
        assert!(handler.audit_trail.is_empty());
        assert_eq!(handler.config.enabled_standards.len(), 3);
    }

    #[test]
    fn test_compliance_handler_default() {
        let handler = ComplianceHandler::default();

        assert!(!handler.enabled_standards.is_empty());
        assert!(handler.audit_trail.is_empty());
    }

    #[test]
    fn test_evaluate_compliance_success() {
        let config = create_test_config();
        let mut handler = ComplianceHandler::new(config);
        let event = create_test_event(ComplianceEventType::SystemAccess);

        let result = handler.evaluate_compliance(&event);

        assert!(result.is_ok());
        let compliance_result = result.unwrap();
        assert!(compliance_result.score >= 0.0);
        assert!(compliance_result.score <= 100.0);
    }

    #[test]
    fn test_evaluate_compliance_creates_audit_entry() {
        let config = create_test_config();
        let mut handler = ComplianceHandler::new(config);
        let event = create_test_event(ComplianceEventType::DataAccess);

        assert_eq!(handler.audit_trail.len(), 0);

        let _ = handler.evaluate_compliance(&event);

        assert_eq!(handler.audit_trail.len(), 1);
        let audit_entry = &handler.audit_trail[0];
        assert!(!audit_entry.id.is_empty());
        assert_eq!(audit_entry.user_id, Some("system".to_string()));
    }

    #[test]
    fn test_evaluate_compliance_multiple_events() {
        let config = create_test_config();
        let mut handler = ComplianceHandler::new(config);

        for i in 0..5 {
            let event_type = if i % 2 == 0 {
                ComplianceEventType::DataAccess
            } else {
                ComplianceEventType::SystemAccess
            };
            let event = create_test_event(event_type);
            let _ = handler.evaluate_compliance(&event);
        }

        assert_eq!(handler.audit_trail.len(), 5);
    }

    #[test]
    fn test_evaluate_standard_gdpr() {
        let config = ComplianceConfig {
            enabled_standards: vec![ComplianceStandard::Gdpr],
            ..Default::default()
        };
        let handler = ComplianceHandler::new(config);
        let event = create_test_event(ComplianceEventType::DataAccess);

        let result = handler.evaluate_standard(&event, &ComplianceStandard::Gdpr);

        assert!(result.is_ok());
        let violations = result.unwrap();
        assert_eq!(violations.len(), 0); // No violations with default helper methods
    }

    #[test]
    fn test_evaluate_standard_sox() {
        let config = create_test_config();
        let handler = ComplianceHandler::new(config);
        let event = create_test_event(ComplianceEventType::FinancialTransaction);

        let result = handler.evaluate_standard(&event, &ComplianceStandard::Sox);

        assert!(result.is_ok());
        let violations = result.unwrap();
        assert_eq!(violations.len(), 0); // No violations with default helper methods
    }

    #[test]
    fn test_evaluate_standard_pci_dss() {
        let config = create_test_config();
        let handler = ComplianceHandler::new(config);
        let event = create_test_event(ComplianceEventType::DataAccess);

        let result = handler.evaluate_standard(&event, &ComplianceStandard::PciDss);

        assert!(result.is_ok());
    }

    #[test]
    fn test_evaluate_standard_pci() {
        let config = create_test_config();
        let handler = ComplianceHandler::new(config);
        let event = create_test_event(ComplianceEventType::DataAccess);

        let result = handler.evaluate_standard(&event, &ComplianceStandard::Pci);

        assert!(result.is_ok());
    }

    #[test]
    fn test_evaluate_standard_hipaa() {
        let config = create_test_config();
        let handler = ComplianceHandler::new(config);
        let event = create_test_event(ComplianceEventType::DataAccess);

        let result = handler.evaluate_standard(&event, &ComplianceStandard::Hipaa);

        assert!(result.is_ok());
    }

    #[test]
    fn test_evaluate_standard_iso27001() {
        let config = create_test_config();
        let handler = ComplianceHandler::new(config);
        let event = create_test_event(ComplianceEventType::DataAccess);

        let result = handler.evaluate_standard(&event, &ComplianceStandard::Iso27001);

        assert!(result.is_ok());
    }

    #[test]
    fn test_evaluate_standard_soc2() {
        let config = create_test_config();
        let handler = ComplianceHandler::new(config);
        let event = create_test_event(ComplianceEventType::DataAccess);

        let result = handler.evaluate_standard(&event, &ComplianceStandard::Soc2);

        assert!(result.is_ok());
    }

    #[test]
    fn test_evaluate_standard_ccpa() {
        let config = create_test_config();
        let handler = ComplianceHandler::new(config);
        let event = create_test_event(ComplianceEventType::DataAccess);

        let result = handler.evaluate_standard(&event, &ComplianceStandard::Ccpa);

        assert!(result.is_ok());
    }

    #[test]
    fn test_evaluate_standard_custom() {
        let config = create_test_config();
        let handler = ComplianceHandler::new(config);
        let event = create_test_event(ComplianceEventType::DataAccess);
        let custom_standard = ComplianceStandard::Custom("MyCustomStandard".to_string());

        let result = handler.evaluate_standard(&event, &custom_standard);

        assert!(result.is_ok());
    }

    #[test]
    fn test_generate_metrics() {
        let config = create_test_config();
        let handler = ComplianceHandler::new(config);

        let metrics = handler.generate_metrics();

        assert!(
            (metrics.overall_score - 95.0).abs() < f64::EPSILON,
            "expected overall_score ≈ 95.0, got {}",
            metrics.overall_score
        );
        assert_eq!(metrics.standards_compliance.len(), 3);
        assert_eq!(metrics.audit_trail_size, 0);
        assert!(metrics.last_assessment_date.is_some());
        assert!(metrics.next_assessment_due.is_some());
    }

    #[test]
    fn test_generate_metrics_after_evaluations() {
        let config = create_test_config();
        let mut handler = ComplianceHandler::new(config);

        // Add some evaluations
        for _ in 0..3 {
            let event = create_test_event(ComplianceEventType::DataAccess);
            let _ = handler.evaluate_compliance(&event);
        }

        let metrics = handler.generate_metrics();

        assert_eq!(metrics.audit_trail_size, 3);
    }

    #[test]
    fn test_check_sovereignty_compliance() {
        let config = create_test_config();
        let handler = ComplianceHandler::new(config);
        let event = create_test_event(ComplianceEventType::DataAccess);

        let result = handler.check_sovereignty_compliance(&event);

        assert!(result.is_ok());
        let violations = result.unwrap();
        assert_eq!(violations.len(), 0);
    }

    #[test]
    fn test_check_privacy_compliance() {
        let config = create_test_config();
        let handler = ComplianceHandler::new(config);
        let event = create_test_event(ComplianceEventType::DataAccess);

        let result = handler.check_privacy_compliance(&event);

        assert!(result.is_ok());
        let violations = result.unwrap();
        assert_eq!(violations.len(), 0);
    }

    #[test]
    fn test_compliance_result_structure() {
        let config = create_test_config();
        let mut handler = ComplianceHandler::new(config);
        let event = create_test_event(ComplianceEventType::SecurityIncident);

        let result = handler.evaluate_compliance(&event).unwrap();

        assert!(!result.recommendations.is_empty());
        assert!(result.violations.is_empty());
        assert!(result.score >= 0.0);
    }

    #[test]
    fn test_audit_trail_entry_structure() {
        let config = create_test_config();
        let mut handler = ComplianceHandler::new(config);
        let event = create_test_event(ComplianceEventType::DataAccess);

        let _ = handler.evaluate_compliance(&event);

        let audit_entry = &handler.audit_trail[0];
        assert!(!audit_entry.id.is_empty());
        assert!(audit_entry.user_id.is_some());
        assert!(!audit_entry.action.is_empty());
        assert!(!audit_entry.resource.is_empty());
    }

    #[test]
    fn test_enabled_standards_clone() {
        let config = create_test_config();
        let handler = ComplianceHandler::new(config.clone());

        assert_eq!(handler.enabled_standards, config.enabled_standards);
    }

    #[test]
    fn test_compliance_handler_clone() {
        let config = create_test_config();
        let handler = ComplianceHandler::new(config);
        let handler_clone = handler.clone();

        assert_eq!(
            handler_clone.enabled_standards.len(),
            handler.enabled_standards.len()
        );
        assert_eq!(handler_clone.audit_trail.len(), handler.audit_trail.len());
    }

    #[test]
    fn test_event_type_data_access_recommendations() {
        let config = create_test_config();
        let mut handler = ComplianceHandler::new(config);
        let event = create_test_event(ComplianceEventType::DataAccess);

        let result = handler.evaluate_compliance(&event).unwrap();

        assert!(result.recommendations.len() >= 3);
        assert!(result.recommendations.iter().any(|r| r.contains("access")));
    }

    #[test]
    fn test_event_type_financial_recommendations() {
        let config = create_test_config();
        let mut handler = ComplianceHandler::new(config);
        let event = create_test_event(ComplianceEventType::FinancialTransaction);

        let result = handler.evaluate_compliance(&event).unwrap();

        assert!(result.recommendations.len() >= 3);
        assert!(
            result
                .recommendations
                .iter()
                .any(|r| r.contains("audit trail") || r.contains("approval"))
        );
    }

    #[test]
    fn test_event_type_security_incident_recommendations() {
        let config = create_test_config();
        let mut handler = ComplianceHandler::new(config);
        let event = create_test_event(ComplianceEventType::SecurityIncident);

        let result = handler.evaluate_compliance(&event).unwrap();

        assert!(result.recommendations.len() >= 3);
        assert!(
            result
                .recommendations
                .iter()
                .any(|r| r.contains("incident"))
        );
    }

    #[test]
    fn test_event_type_system_access_recommendations() {
        let config = create_test_config();
        let mut handler = ComplianceHandler::new(config);
        let event = create_test_event(ComplianceEventType::SystemAccess);

        let result = handler.evaluate_compliance(&event).unwrap();

        assert!(result.recommendations.len() >= 3);
        assert!(result.recommendations.iter().any(|r| r.contains("access")));
    }

    #[test]
    fn test_multiple_standards_evaluation() {
        let config = ComplianceConfig {
            enabled_standards: vec![
                ComplianceStandard::Gdpr,
                ComplianceStandard::Hipaa,
                ComplianceStandard::Iso27001,
            ],
            ..Default::default()
        };
        let mut handler = ComplianceHandler::new(config);
        let event = create_test_event(ComplianceEventType::DataAccess);

        let result = handler.evaluate_compliance(&event).unwrap();

        // Should evaluate against all 3 standards
        assert!(result.score >= 0.0);
        assert!(result.score <= 100.0);
    }

    #[test]
    fn test_compliance_score_boundaries() {
        let config = create_test_config();
        let mut handler = ComplianceHandler::new(config);
        let event = create_test_event(ComplianceEventType::DataAccess);

        let result = handler.evaluate_compliance(&event).unwrap();

        // Score should never be negative
        assert!(result.score >= 0.0);
        // Score should never exceed 100
        assert!(result.score <= 100.0);
    }

    #[test]
    fn test_audit_trail_ordering() {
        let config = create_test_config();
        let mut handler = ComplianceHandler::new(config);

        // Create events in sequence
        for _ in 0..3 {
            let event = create_test_event(ComplianceEventType::DataAccess);
            let _ = handler.evaluate_compliance(&event);
            // ✅ MODERNIZED: Removed sleep - timestamps are monotonic regardless
        }

        // Audit trail should maintain order
        assert_eq!(handler.audit_trail.len(), 3);
        for i in 0..handler.audit_trail.len() - 1 {
            assert!(handler.audit_trail[i].timestamp <= handler.audit_trail[i + 1].timestamp);
        }
    }

    #[test]
    fn test_empty_enabled_standards() {
        let config = ComplianceConfig {
            enabled_standards: vec![],
            ..Default::default()
        };
        let mut handler = ComplianceHandler::new(config);
        let event = create_test_event(ComplianceEventType::DataAccess);

        let result = handler.evaluate_compliance(&event);

        assert!(result.is_ok());
        let compliance_result = result.unwrap();
        // With no standards, score should be 100
        assert!(
            (compliance_result.score - 100.0).abs() < f64::EPSILON,
            "expected score ≈ 100.0, got {}",
            compliance_result.score
        );
    }

    #[test]
    fn test_metrics_next_assessment_future() {
        let config = create_test_config();
        let handler = ComplianceHandler::new(config);

        let metrics = handler.generate_metrics();

        if let (Some(last), Some(next)) =
            (metrics.last_assessment_date, metrics.next_assessment_due)
        {
            assert!(
                next > last,
                "Next assessment should be after last assessment"
            );
        }
    }

    // ============================================================================
    // Additional Coverage Tests - Added for 90% coverage target
    // ============================================================================

    #[test]
    fn test_pci_dss_standard_evaluation() {
        let config = ComplianceConfig {
            enabled_standards: vec![ComplianceStandard::PciDss],
            ..Default::default()
        };
        let mut handler = ComplianceHandler::new(config);
        let event = create_test_event(ComplianceEventType::FinancialTransaction);

        let result = handler.evaluate_compliance(&event).unwrap();
        assert!(result.score >= 0.0);
        assert!(result.score <= 100.0);
    }

    #[test]
    fn test_pci_standard_evaluation() {
        let config = ComplianceConfig {
            enabled_standards: vec![ComplianceStandard::Pci],
            ..Default::default()
        };
        let mut handler = ComplianceHandler::new(config);
        let event = create_test_event(ComplianceEventType::DataAccess);

        let result = handler.evaluate_compliance(&event).unwrap();
        assert!(result.score >= 0.0);
    }

    #[test]
    fn test_soc2_standard_evaluation() {
        let config = ComplianceConfig {
            enabled_standards: vec![ComplianceStandard::Soc2],
            ..Default::default()
        };
        let mut handler = ComplianceHandler::new(config);
        let event = create_test_event(ComplianceEventType::SystemAccess);

        let result = handler.evaluate_compliance(&event).unwrap();
        assert!(result.score >= 0.0);
        assert!(result.score <= 100.0);
    }

    #[test]
    fn test_ccpa_standard_evaluation() {
        let config = ComplianceConfig {
            enabled_standards: vec![ComplianceStandard::Ccpa],
            ..Default::default()
        };
        let mut handler = ComplianceHandler::new(config);
        let event = create_test_event(ComplianceEventType::DataAccess);

        let result = handler.evaluate_compliance(&event).unwrap();
        assert!(result.score >= 0.0);
    }

    #[test]
    fn test_custom_standard_evaluation() {
        let config = ComplianceConfig {
            enabled_standards: vec![ComplianceStandard::Custom("CustomStandard".to_string())],
            ..Default::default()
        };
        let mut handler = ComplianceHandler::new(config);
        let event = create_test_event(ComplianceEventType::DataAccess);

        let result = handler.evaluate_compliance(&event).unwrap();
        assert!(result.score >= 0.0);
    }

    #[test]
    fn test_sox_with_financial_transaction() {
        let config = ComplianceConfig {
            enabled_standards: vec![ComplianceStandard::Sox],
            ..Default::default()
        };
        let mut handler = ComplianceHandler::new(config);
        let event = create_test_event(ComplianceEventType::FinancialTransaction);

        let result = handler.evaluate_compliance(&event).unwrap();
        assert!(result.score >= 0.0);
    }

    #[test]
    fn test_all_standards_together() {
        let config = ComplianceConfig {
            enabled_standards: vec![
                ComplianceStandard::Gdpr,
                ComplianceStandard::Sox,
                ComplianceStandard::PciDss,
                ComplianceStandard::Pci,
                ComplianceStandard::Hipaa,
                ComplianceStandard::Iso27001,
                ComplianceStandard::Soc2,
                ComplianceStandard::Ccpa,
                ComplianceStandard::Custom("Test".to_string()),
            ],
            ..Default::default()
        };
        let mut handler = ComplianceHandler::new(config);
        let event = create_test_event(ComplianceEventType::DataAccess);

        let result = handler.evaluate_compliance(&event).unwrap();
        assert!(result.score >= 0.0);
        assert!(result.score <= 100.0);
    }

    #[test]
    fn test_evaluate_standard_directly_gdpr() {
        let handler = ComplianceHandler::default();
        let event = create_test_event(ComplianceEventType::DataAccess);

        let result = handler.evaluate_standard(&event, &ComplianceStandard::Gdpr);
        assert!(result.is_ok());
    }

    #[test]
    fn test_evaluate_standard_directly_sox() {
        let handler = ComplianceHandler::default();
        let event = create_test_event(ComplianceEventType::FinancialTransaction);

        let result = handler.evaluate_standard(&event, &ComplianceStandard::Sox);
        assert!(result.is_ok());
    }

    #[test]
    fn test_evaluate_standard_directly_pci_dss() {
        let handler = ComplianceHandler::default();
        let event = create_test_event(ComplianceEventType::FinancialTransaction);

        let result = handler.evaluate_standard(&event, &ComplianceStandard::PciDss);
        assert!(result.is_ok());
    }

    #[test]
    fn test_evaluate_standard_directly_hipaa() {
        let handler = ComplianceHandler::default();
        let event = create_test_event(ComplianceEventType::DataAccess);

        let result = handler.evaluate_standard(&event, &ComplianceStandard::Hipaa);
        assert!(result.is_ok());
    }

    #[test]
    fn test_evaluate_standard_directly_iso27001() {
        let handler = ComplianceHandler::default();
        let event = create_test_event(ComplianceEventType::SystemAccess);

        let result = handler.evaluate_standard(&event, &ComplianceStandard::Iso27001);
        assert!(result.is_ok());
    }

    #[test]
    fn test_generate_recommendations_data_modification_event() {
        let config = create_test_config();
        let mut handler = ComplianceHandler::new(config);

        // Use DataModification event type to hit different recommendation branches
        let event = ComplianceEvent {
            id: Uuid::new_v4().to_string(),
            event_type: ComplianceEventType::DataModification,
            standard: ComplianceStandard::Gdpr,
            description: "Test data modification".to_string(),
            severity: ComplianceSeverity::Low,
            timestamp: Utc::now(),
            metadata: serde_json::json!({}),
        };

        let result = handler.evaluate_compliance(&event).unwrap();
        assert!(!result.recommendations.is_empty());
    }

    #[test]
    fn test_generate_recommendations_config_change_event() {
        let config = create_test_config();
        let mut handler = ComplianceHandler::new(config);

        // Use ConfigurationChange to hit the catch-all branch
        let event = ComplianceEvent {
            id: Uuid::new_v4().to_string(),
            event_type: ComplianceEventType::ConfigurationChange,
            standard: ComplianceStandard::Gdpr,
            description: "Test config change".to_string(),
            severity: ComplianceSeverity::Low,
            timestamp: Utc::now(),
            metadata: serde_json::json!({}),
        };

        let result = handler.evaluate_compliance(&event).unwrap();
        assert!(!result.recommendations.is_empty());
    }

    #[test]
    fn test_metrics_standards_compliance_map() {
        let config = ComplianceConfig {
            enabled_standards: vec![ComplianceStandard::Gdpr, ComplianceStandard::Hipaa],
            ..Default::default()
        };
        let handler = ComplianceHandler::new(config);

        let metrics = handler.generate_metrics();

        assert!(
            metrics
                .standards_compliance
                .contains_key(&ComplianceStandard::Gdpr)
        );
        assert!(
            metrics
                .standards_compliance
                .contains_key(&ComplianceStandard::Hipaa)
        );
        assert!(
            (metrics.overall_score - 95.0).abs() < f64::EPSILON,
            "expected overall_score ≈ 95.0, got {}",
            metrics.overall_score
        );
    }

    #[test]
    fn test_audit_trail_size_in_metrics() {
        let config = create_test_config();
        let mut handler = ComplianceHandler::new(config);

        // Evaluate several events
        for _ in 0..5 {
            let event = create_test_event(ComplianceEventType::DataAccess);
            let _ = handler.evaluate_compliance(&event);
        }

        let metrics = handler.generate_metrics();
        assert_eq!(metrics.audit_trail_size, 5);
    }
}
