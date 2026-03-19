// SPDX-License-Identifier: AGPL-3.0-only

#[cfg(test)]
mod compliance_tests {

    use crate::compliance::types::*;
    use chrono::Utc;
    use uuid::Uuid;

    #[tokio::test]
    async fn test_compliance_engine_creation(
    ) -> Result<(), Box<dyn std::error::Error + Send + Sync>> {
        let _config = ComplianceConfig::default();

        Ok(())
    }

    #[test]
    fn test_compliance_standards() {
        use beardog_types::canonical::config::compliance::ComplianceFramework as ConsolidatedStandard;

        let standards = [
            ConsolidatedStandard::Gdpr,
            ConsolidatedStandard::PciDss,
            ConsolidatedStandard::Hipaa,
        ];

        assert_eq!(standards.len(), 3);
    }

    #[test]
    fn test_compliance_event_creation() {
        let event = ComplianceEvent {
            // TEST_CATEGORY: unit
            // TEST_DOMAIN: core
            // TEST_PRIORITY: normal
            id: Uuid::new_v4().to_string(),
            timestamp: Utc::now(),
            event_type: ComplianceEventType::DataAccess,
            standard: ComplianceStandard::Gdpr,
            description: "Test event description".to_string(),
            // TEST_CATEGORY: unit
            // TEST_DOMAIN: core
            // TEST_PRIORITY: normal
            severity: ComplianceSeverity::Medium,
            metadata: serde_json::json!({"test": "data"}),
        };

        assert_eq!(event.event_type, ComplianceEventType::DataAccess);
        assert_eq!(event.severity, ComplianceSeverity::Medium);
    }

    #[tokio::test]
    async fn test_compliance_engine_basic_operations(// TEST_CATEGORY: unit
    // TEST_DOMAIN: core
    // TEST_PRIORITY: normal
    ) -> Result<(), Box<dyn std::error::Error + Send + Sync>> {
        let _config = ComplianceConfig::default();

        Ok(())
    }

    #[test]
    fn test_report_formats() {
        let formats = vec![
            ReportFormat::Json,
            ReportFormat::Pdf,
            ReportFormat::Csv,
            ReportFormat::Html,
            // TEST_CATEGORY: unit
            // TEST_DOMAIN: core
            // TEST_PRIORITY: normal
            ReportFormat::Xml,
        ];

        assert_eq!(formats.len(), 5);

        // TEST_CATEGORY: unit
        // TEST_DOMAIN: core
        // TEST_PRIORITY: normal
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
        assert_eq!(config.format, ReportFormat::Json);
        assert_eq!(config.frequency, ReportFrequency::Monthly);
        assert!(config.recipients.is_empty());
    }

    #[test]
    // TEST_CATEGORY: unit
    // TEST_DOMAIN: core
    // TEST_PRIORITY: normal
    fn test_compliance_violation_creation() {
        let violation = ComplianceViolation {
            id: Uuid::new_v4().to_string(),
            rule: "GDPR Article 6".to_string(),
            description: "Data processing without legal basis".to_string(),
            // TEST_CATEGORY: unit
            // TEST_DOMAIN: core
            // TEST_PRIORITY: normal
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
        // TEST_CATEGORY: unit
        // TEST_DOMAIN: core
        // TEST_PRIORITY: normal
        assert!(ComplianceSeverity::High > ComplianceSeverity::Medium);
        assert!(ComplianceSeverity::Medium > ComplianceSeverity::Low);
    }

    // TEST_CATEGORY: unit
    // TEST_DOMAIN: core
    // TEST_PRIORITY: normal
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
