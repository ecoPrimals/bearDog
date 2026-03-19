// SPDX-License-Identifier: AGPL-3.0-only

//! Additional comprehensive tests for ComplianceHandler
//!
//! These tests target previously uncovered code paths to increase coverage
//! from 70.79% to 90%+. Focus areas:
//! - Edge cases in compliance evaluation
//! - Helper method coverage
//! - Error paths
//! - All compliance standards

#[cfg(test)]
mod additional_compliance_handler_tests {
    use super::super::handlers::ComplianceHandler;
    use super::super::types::{
        ComplianceConfig, ComplianceEvent, ComplianceEventType, ComplianceSeverity,
        ComplianceStandard,
    };
    use chrono::Utc;
    use std::collections::HashMap;
    use uuid::Uuid;

    // ============================================================================
    // Helper Functions
    // ============================================================================

    fn create_test_config_with_standards(standards: Vec<ComplianceStandard>) -> ComplianceConfig {
        ComplianceConfig {
            enabled: true,
            enabled_standards: standards,
            audit_retention_days: 90,
            enforce_strict_mode: true,
            alert_on_violations: true,
            custom_rules: HashMap::new(),
        }
    }

    fn create_test_event(event_type: ComplianceEventType, standard: ComplianceStandard) -> ComplianceEvent {
        ComplianceEvent {
            id: Uuid::new_v4().to_string(),
            event_type,
            timestamp: Utc::now(),
            description: format!("Test event: {:?}", event_type),
            severity: ComplianceSeverity::Medium,
            standard,
            metadata: {
                let mut meta = HashMap::new();
                meta.insert("test".to_string(), "true".to_string());
                meta
            },
        }
    }

    // ============================================================================
    // GDPR Compliance Tests (expanding coverage)
    // ============================================================================

    #[test]
    fn test_gdpr_data_access_with_consent() {
        let config = create_test_config_with_standards(vec![ComplianceStandard::Gdpr]);
        let mut handler = ComplianceHandler::new(config);

        let mut event = create_test_event(ComplianceEventType::DataAccess, ComplianceStandard::Gdpr);
        // Add consent metadata
        event.metadata.insert("consent_obtained".to_string(), "true".to_string());

        let result = handler.evaluate_compliance(&event);
        assert!(result.is_ok());
        let compliance_result = result.unwrap();
        
        // With consent, should have fewer violations
        assert!(compliance_result.score >= 80.0, "Expected high score with consent");
    }

    #[test]
    fn test_gdpr_data_access_without_consent() {
        let config = create_test_config_with_standards(vec![ComplianceStandard::Gdpr]);
        let mut handler = ComplianceHandler::new(config);

        let event = create_test_event(ComplianceEventType::DataAccess, ComplianceStandard::Gdpr);
        
        let result = handler.evaluate_compliance(&event);
        assert!(result.is_ok());
        let compliance_result = result.unwrap();
        
        // Without consent, should have violations
        assert!(!compliance_result.violations.is_empty(), "Expected violations without consent");
        assert!(compliance_result.score < 100.0, "Expected reduced score without consent");
    }

    #[test]
    fn test_gdpr_data_processing() {
        let config = create_test_config_with_standards(vec![ComplianceStandard::Gdpr]);
        let mut handler = ComplianceHandler::new(config);

        let event = create_test_event(ComplianceEventType::DataProcessing, ComplianceStandard::Gdpr);
        
        let result = handler.evaluate_compliance(&event);
        assert!(result.is_ok());
    }

    #[test]
    fn test_gdpr_data_deletion() {
        let config = create_test_config_with_standards(vec![ComplianceStandard::Gdpr]);
        let mut handler = ComplianceHandler::new(config);

        let event = create_test_event(ComplianceEventType::DataDeletion, ComplianceStandard::Gdpr);
        
        let result = handler.evaluate_compliance(&event);
        assert!(result.is_ok());
        let compliance_result = result.unwrap();
        
        // Data deletion is generally compliant
        assert!(compliance_result.score >= 80.0, "Data deletion should score well");
    }

    // ============================================================================
    // SOX Compliance Tests
    // ============================================================================

    #[test]
    fn test_sox_financial_transaction_compliant() {
        let config = create_test_config_with_standards(vec![ComplianceStandard::Sox]);
        let mut handler = ComplianceHandler::new(config);

        let mut event = create_test_event(
            ComplianceEventType::FinancialTransaction,
            ComplianceStandard::Sox
        );
        event.metadata.insert("minimum_necessary".to_string(), "true".to_string());
        
        let result = handler.evaluate_compliance(&event);
        assert!(result.is_ok());
    }

    #[test]
    fn test_sox_financial_transaction_violation() {
        let config = create_test_config_with_standards(vec![ComplianceStandard::Sox]);
        let mut handler = ComplianceHandler::new(config);

        let event = create_test_event(
            ComplianceEventType::FinancialTransaction,
            ComplianceStandard::Sox
        );
        
        let result = handler.evaluate_compliance(&event);
        assert!(result.is_ok());
        let compliance_result = result.unwrap();
        
        // Should detect minimum necessary violation
        assert!(!compliance_result.violations.is_empty(), "Expected minimum necessary violation");
    }

    #[test]
    fn test_sox_audit_log() {
        let config = create_test_config_with_standards(vec![ComplianceStandard::Sox]);
        let mut handler = ComplianceHandler::new(config);

        let event = create_test_event(ComplianceEventType::AuditLog, ComplianceStandard::Sox);
        
        let result = handler.evaluate_compliance(&event);
        assert!(result.is_ok());
    }

    // ============================================================================
    // PCI DSS Compliance Tests
    // ============================================================================

    #[test]
    fn test_pci_payment_processing() {
        let config = create_test_config_with_standards(vec![ComplianceStandard::PciDss]);
        let mut handler = ComplianceHandler::new(config);

        let mut event = create_test_event(
            ComplianceEventType::PaymentProcessing,
            ComplianceStandard::PciDss
        );
        event.metadata.insert("payment_data".to_string(), "card_number".to_string());
        
        let result = handler.evaluate_compliance(&event);
        assert!(result.is_ok());
        let compliance_result = result.unwrap();
        
        // Payment data handling should trigger PCI checks
        assert!(!compliance_result.violations.is_empty(), "Expected PCI violations for unencrypted card data");
    }

    #[test]
    fn test_pci_legacy_standard() {
        let config = create_test_config_with_standards(vec![ComplianceStandard::Pci]);
        let mut handler = ComplianceHandler::new(config);

        let event = create_test_event(ComplianceEventType::DataAccess, ComplianceStandard::Pci);
        
        let result = handler.evaluate_compliance(&event);
        assert!(result.is_ok());
    }

    // ============================================================================
    // HIPAA Compliance Tests
    // ============================================================================

    #[test]
    fn test_hipaa_healthcare_data_compliant() {
        let config = create_test_config_with_standards(vec![ComplianceStandard::Hipaa]);
        let mut handler = ComplianceHandler::new(config);

        let mut event = create_test_event(
            ComplianceEventType::HealthcareDataAccess,
            ComplianceStandard::Hipaa
        );
        event.metadata.insert("encrypted".to_string(), "true".to_string());
        
        let result = handler.evaluate_compliance(&event);
        assert!(result.is_ok());
    }

    #[test]
    fn test_hipaa_healthcare_data_violation() {
        let config = create_test_config_with_standards(vec![ComplianceStandard::Hipaa]);
        let mut handler = ComplianceHandler::new(config);

        let event = create_test_event(
            ComplianceEventType::HealthcareDataAccess,
            ComplianceStandard::Hipaa
        );
        
        let result = handler.evaluate_compliance(&event);
        assert!(result.is_ok());
        let compliance_result = result.unwrap();
        
        // Unencrypted healthcare data should violate HIPAA
        assert!(!compliance_result.violations.is_empty(), "Expected HIPAA violations");
    }

    // ============================================================================
    // CCPA Compliance Tests
    // ============================================================================

    #[test]
    fn test_ccpa_data_sharing() {
        let config = create_test_config_with_standards(vec![ComplianceStandard::Ccpa]);
        let mut handler = ComplianceHandler::new(config);

        let event = create_test_event(ComplianceEventType::DataSharing, ComplianceStandard::Ccpa);
        
        let result = handler.evaluate_compliance(&event);
        assert!(result.is_ok());
    }

    #[test]
    fn test_ccpa_data_sale() {
        let config = create_test_config_with_standards(vec![ComplianceStandard::Ccpa]);
        let mut handler = ComplianceHandler::new(config);

        let event = create_test_event(ComplianceEventType::DataSale, ComplianceStandard::Ccpa);
        
        let result = handler.evaluate_compliance(&event);
        assert!(result.is_ok());
        let compliance_result = result.unwrap();
        
        // Data sale requires specific CCPA compliance
        assert!(!compliance_result.violations.is_empty(), "Expected CCPA violations for data sale");
    }

    // ============================================================================
    // ISO 27001 Compliance Tests
    // ============================================================================

    #[test]
    fn test_iso27001_security_event() {
        let config = create_test_config_with_standards(vec![ComplianceStandard::Iso27001]);
        let mut handler = ComplianceHandler::new(config);

        let event = create_test_event(ComplianceEventType::SecurityEvent, ComplianceStandard::Iso27001);
        
        let result = handler.evaluate_compliance(&event);
        assert!(result.is_ok());
    }

    #[test]
    fn test_iso27001_access_control() {
        let config = create_test_config_with_standards(vec![ComplianceStandard::Iso27001]);
        let mut handler = ComplianceHandler::new(config);

        let event = create_test_event(ComplianceEventType::AccessControl, ComplianceStandard::Iso27001);
        
        let result = handler.evaluate_compliance(&event);
        assert!(result.is_ok());
    }

    // ============================================================================
    // Multi-Standard Compliance Tests
    // ============================================================================

    #[test]
    fn test_multi_standard_evaluation() {
        let standards = vec![
            ComplianceStandard::Gdpr,
            ComplianceStandard::Sox,
            ComplianceStandard::PciDss,
        ];
        let config = create_test_config_with_standards(standards);
        let mut handler = ComplianceHandler::new(config);

        let event = create_test_event(ComplianceEventType::DataAccess, ComplianceStandard::Gdpr);
        
        let result = handler.evaluate_compliance(&event);
        assert!(result.is_ok());
        let compliance_result = result.unwrap();
        
        // Multi-standard evaluation may have more violations
        assert!(compliance_result.score <= 100.0);
    }

    #[test]
    fn test_empty_standards_list() {
        let config = create_test_config_with_standards(vec![]);
        let mut handler = ComplianceHandler::new(config);

        let event = create_test_event(ComplianceEventType::DataAccess, ComplianceStandard::Gdpr);
        
        let result = handler.evaluate_compliance(&event);
        assert!(result.is_ok());
        let compliance_result = result.unwrap();
        
        // With no standards enabled, should pass with perfect score
        assert_eq!(compliance_result.score, 100.0);
        assert!(compliance_result.violations.is_empty());
    }

    // ============================================================================
    // Audit Trail Tests
    // ============================================================================

    #[test]
    fn test_audit_trail_creation() {
        let config = create_test_config_with_standards(vec![ComplianceStandard::Gdpr]);
        let mut handler = ComplianceHandler::new(config);

        assert_eq!(handler.audit_trail.len(), 0, "Audit trail should start empty");

        let event = create_test_event(ComplianceEventType::DataAccess, ComplianceStandard::Gdpr);
        let _ = handler.evaluate_compliance(&event);

        assert_eq!(handler.audit_trail.len(), 1, "Audit trail should have one entry");
        
        let audit_entry = &handler.audit_trail[0];
        assert!(audit_entry.action.contains("compliance_evaluation"));
    }

    #[test]
    fn test_multiple_evaluations_audit_trail() {
        let config = create_test_config_with_standards(vec![ComplianceStandard::Gdpr]);
        let mut handler = ComplianceHandler::new(config);

        for _ in 0..5 {
            let event = create_test_event(ComplianceEventType::DataAccess, ComplianceStandard::Gdpr);
            let _ = handler.evaluate_compliance(&event);
        }

        assert_eq!(handler.audit_trail.len(), 5, "Audit trail should have 5 entries");
    }

    // ============================================================================
    // Edge Cases and Error Handling
    // ============================================================================

    #[test]
    fn test_unknown_event_type() {
        let config = create_test_config_with_standards(vec![ComplianceStandard::Gdpr]);
        let mut handler = ComplianceHandler::new(config);

        let event = create_test_event(ComplianceEventType::Other, ComplianceStandard::Gdpr);
        
        let result = handler.evaluate_compliance(&event);
        assert!(result.is_ok());
    }

    #[test]
    fn test_recommendation_generation() {
        let config = create_test_config_with_standards(vec![ComplianceStandard::Gdpr]);
        let mut handler = ComplianceHandler::new(config);

        let event = create_test_event(ComplianceEventType::DataAccess, ComplianceStandard::Gdpr);
        
        let result = handler.evaluate_compliance(&event);
        assert!(result.is_ok());
        let compliance_result = result.unwrap();
        
        // Should generate recommendations
        assert!(!compliance_result.recommendations.is_empty(), "Should have recommendations");
    }

    #[test]
    fn test_score_clamping() {
        let standards = vec![
            ComplianceStandard::Gdpr,
            ComplianceStandard::Sox,
            ComplianceStandard::PciDss,
            ComplianceStandard::Hipaa,
            ComplianceStandard::Ccpa,
            ComplianceStandard::Iso27001,
        ];
        let config = create_test_config_with_standards(standards);
        let mut handler = ComplianceHandler::new(config);

        let event = create_test_event(ComplianceEventType::DataAccess, ComplianceStandard::Gdpr);
        
        let result = handler.evaluate_compliance(&event);
        assert!(result.is_ok());
        let compliance_result = result.unwrap();
        
        // Score should never be negative
        assert!(compliance_result.score >= 0.0, "Score should be clamped at 0");
        assert!(compliance_result.score <= 100.0, "Score should be clamped at 100");
    }

    // ============================================================================
    // Severity Tests
    // ============================================================================

    #[test]
    fn test_critical_severity_event() {
        let config = create_test_config_with_standards(vec![ComplianceStandard::PciDss]);
        let mut handler = ComplianceHandler::new(config);

        let mut event = create_test_event(ComplianceEventType::PaymentProcessing, ComplianceStandard::PciDss);
        event.severity = ComplianceSeverity::Critical;
        
        let result = handler.evaluate_compliance(&event);
        assert!(result.is_ok());
    }

    #[test]
    fn test_low_severity_event() {
        let config = create_test_config_with_standards(vec![ComplianceStandard::Gdpr]);
        let mut handler = ComplianceHandler::new(config);

        let mut event = create_test_event(ComplianceEventType::DataAccess, ComplianceStandard::Gdpr);
        event.severity = ComplianceSeverity::Low;
        
        let result = handler.evaluate_compliance(&event);
        assert!(result.is_ok());
    }

    // ============================================================================
    // Integration Tests
    // ============================================================================

    #[test]
    fn test_handler_clone() {
        let config = create_test_config_with_standards(vec![ComplianceStandard::Gdpr]);
        let handler1 = ComplianceHandler::new(config);
        
        let handler2 = handler1.clone();
        
        assert_eq!(handler1.enabled_standards.len(), handler2.enabled_standards.len());
        assert_eq!(handler1.audit_trail.len(), handler2.audit_trail.len());
    }

    #[test]
    fn test_config_retention_days() {
        let mut config = create_test_config_with_standards(vec![ComplianceStandard::Gdpr]);
        config.audit_retention_days = 365;
        
        let handler = ComplianceHandler::new(config);
        
        assert_eq!(handler.config.audit_retention_days, 365);
    }

    #[test]
    fn test_strict_mode_enabled() {
        let mut config = create_test_config_with_standards(vec![ComplianceStandard::Gdpr]);
        config.enforce_strict_mode = true;
        
        let handler = ComplianceHandler::new(config);
        
        assert!(handler.config.enforce_strict_mode);
    }
}

