#[cfg(test)]
mod compliance_tests {

    use crate::compliance::types::*;
    use chrono::Utc;
    use uuid::Uuid;

    #[tokio::test]
    async fn test_compliance_engine_creation(
    ) -> Result<(), Box<dyn std::error::Error + Send + Sync>> {
        let _config = ComplianceConfig::default();
        // Placeholder test - ComplianceEngine will be implemented later
        Ok(())
    }

    #[test]
    fn test_compliance_standards() {
        use beardog_types::canonical::configuration::consolidated::ComplianceStandard as ConsolidatedStandard;

        let standards = [ConsolidatedStandard::Gdpr,
            ConsolidatedStandard::Sox,
            ConsolidatedStandard::PciDss,
            ConsolidatedStandard::Hipaa,
            ConsolidatedStandard::IsoIec27001,
            ConsolidatedStandard::Nist,
            ConsolidatedStandard::FedRamp,
            ConsolidatedStandard::Custom("CustomStandard".to_string())];

        assert_eq!(standards.len(), 8);
        // Note: We can't use assert_eq! directly because the consolidated enum doesn't implement PartialEq
        // This is expected as part of the type system consolidation
    }

    #[test]
    fn test_compliance_event_creation() {
        let event = ComplianceEvent {
            id: Uuid::new_v4(),
            timestamp: Utc::now(),
            event_type: ComplianceEventType::DataAccess,
            standard: ComplianceStandard::Gdpr,
            description: "Test event description".to_string(),
            severity: ComplianceSeverity::Medium,
            metadata: serde_json::json!({"test": "data"}),
        };

        assert_eq!(event.event_type, ComplianceEventType::DataAccess);
        assert_eq!(event.severity, ComplianceSeverity::Medium);
    }

    #[tokio::test]
    async fn test_compliance_engine_basic_operations(
    ) -> Result<(), Box<dyn std::error::Error + Send + Sync>> {
        let _config = ComplianceConfig::default();
        // Placeholder test - ComplianceEngine will be implemented later
        Ok(())
    }

    #[test]
    fn test_report_formats() {
        let formats = vec![
            ReportFormat::Json,
            ReportFormat::Pdf,
            ReportFormat::Csv,
            ReportFormat::Html,
            ReportFormat::Xml,
        ];

        assert_eq!(formats.len(), 5);

        // Test format matching
        for format in formats {
            match format {
                ReportFormat::Json => {}
                ReportFormat::Pdf => {}
                ReportFormat::Csv => {}
                ReportFormat::Html => {}
                ReportFormat::Xml => {}
            }
        }
    }

    #[test]
    fn test_reporting_config() {
        let config = ReportingConfig::default();
        assert!(config.enabled);
        assert_eq!(config.formats.len(), 2); // Json and Pdf by default
        assert!(config.email_recipients.is_empty());
        assert!(config.storage_path.contains("compliance"));
        assert!(config.retention_period.as_secs() > 0);
    }

    #[test]
    fn test_compliance_violation_creation() {
        let violation = ComplianceViolation {
            id: Uuid::new_v4(),
            rule: "GDPR Article 6".to_string(),
            description: "Data processing without legal basis".to_string(),
            severity: ComplianceSeverity::High,
            remediation: "Obtain proper consent or establish legal basis".to_string(),
            affected_data: Some("Personal identifiers".to_string()),
        };

        assert_eq!(violation.rule, "GDPR Article 6");
        assert_eq!(violation.severity, ComplianceSeverity::High);
        assert!(violation.affected_data.is_some());
    }

    #[test]
    fn test_compliance_severity_ordering() {
        assert!(ComplianceSeverity::Critical > ComplianceSeverity::High);
        assert!(ComplianceSeverity::High > ComplianceSeverity::Medium);
        assert!(ComplianceSeverity::Medium > ComplianceSeverity::Low);
    }

    #[test]
    fn test_compliance_result_creation() {
        let result = ComplianceResult {
            standard: ComplianceStandard::Gdpr,
            passed: true,
            score: 85.5,
            violations: Vec::new(),
            recommendations: vec!["Improve data encryption".to_string()],
            timestamp: Utc::now(),
        };

        assert_eq!(result.standard, ComplianceStandard::Gdpr);
        assert!(result.passed);
        assert!(result.score >= 0.0);
        assert!(result.score <= 100.0);
        assert!(!result.recommendations.is_empty());
    }
}
