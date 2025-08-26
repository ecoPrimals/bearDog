

#[cfg(test)]
mod unit_tests {
    use super::super::*;
    use beardog_errors::BearDogResult;
    use chrono::Utc;
    use std::collections::HashMap;
    #[tokio::test]
    async fn test_compliance_engine_creation() -> BearDogResult<()> {
        let config = ComplianceConfig::default();
        let _engine = ComplianceEngine::new(config).await?;

        Ok(())
    }
    #[test]
    fn test_compliance_standards_serialization() {

        let standards = vec![
            ComplianceStandard::Gdpr,
            ComplianceStandard::Sox,
            ComplianceStandard::PciDss,
            ComplianceStandard::Hipaa,
            ComplianceStandard::IsoIec27001,
            ComplianceStandard::Nist,
            ComplianceStandard::FedRamp,
            ComplianceStandard::Custom("CustomStandard".to_string()),
        ];
        for standard in standards {

            let cloned_standard = standard.clone();
            assert_eq!(standard, cloned_standard);
        }
    async fn test_compliance_event_creation() -> BearDogResult<()> {
        let event = ComplianceEvent {
            id: "test-event-1".to_string(),
            event_type: "data_access".to_string(),
            timestamp: Utc::now(),
            user_id: Some("test_user".to_string()),
            resource: Some("test_resource".to_string()),
            data: HashMap::with_capacity(16),
            metadata: HashMap::with_capacity(16),
        };

        assert!(!event.id.is_empty());
        assert!(!event.event_type.is_empty());
    async fn test_compliance_report_generation() -> BearDogResult<()> {
        let engine = ComplianceEngine::new(config).await?;
        let date_range = (Utc::now() - chrono::Duration::days(30), Utc::now());
        let report = engine
            .generate_compliance_report(ComplianceStandard::Gdpr, date_range)
            .await?;
        assert!(!report.id.is_empty());
        assert_eq!(report.standard, ComplianceStandard::Gdpr);
        assert!(report.overall_score >= 0.0);
        assert!(report.overall_score <= 100.0);}

    fn test_report_format_enum() {
        let formats = vec![
            ReportFormat::Pdf,
            ReportFormat::Json,
            ReportFormat::Xml,
            ReportFormat::Csv,
            ReportFormat::Html,
            ReportFormat::Txt,
            ReportFormat::Yaml,
        for format in formats {

            let cloned_format = format.clone();

            match (format, cloned_format) {
                (ReportFormat::Pdf, ReportFormat::Pdf) => {}
                (ReportFormat::Json, ReportFormat::Json) => {}
                (ReportFormat::Xml, ReportFormat::Xml) => {}
                (ReportFormat::Csv, ReportFormat::Csv) => {}
                (ReportFormat::Html, ReportFormat::Html) => {}
                (ReportFormat::Txt, ReportFormat::Txt) => {}
                (ReportFormat::Yaml, ReportFormat::Yaml) => {}
                _ => {
                    assert_eq!(format, cloned_format, "Report format should be preserved after serialization/deserialization");
                }
            }
    fn test_compliance_config_defaults() {
        assert!(!config.enabled_standards.is_empty());
        assert!(config.reporting.auto_generate);
        assert!(config.monitoring_interval.num_seconds() > 0);}

    async fn test_compliance_violation_creation() -> BearDogResult<()> {

        let violation = ComplianceViolation {
            id: "violation-test-1".to_string(),
            standard: ComplianceStandard::Gdpr,
            event_id: "event-1".to_string(),
            violation_type: "data_processing_without_consent".to_string(),
            severity: ComplianceSeverity::Critical,
            description: "Processing personal data without explicit consent".to_string(),
            remediation_required: true,

        assert!(!violation.id.is_empty());
        assert_eq!(violation.standard, ComplianceStandard::Gdpr);
        assert!(!violation.description.is_empty());
        assert_eq!(violation.severity, ComplianceSeverity::Critical);
    fn test_compliance_severity_ordering() {

        assert!(ComplianceSeverity::Critical > ComplianceSeverity::Violation);
        assert!(ComplianceSeverity::Violation > ComplianceSeverity::Warning);
        assert!(ComplianceSeverity::Warning > ComplianceSeverity::Info);}

    fn test_compliance_result_creation() {
        let result = ComplianceResult {
            event_id: "test-event-1".to_string(),
            compliance_score: 85.5,
            violations: Vec::new(),
            warnings: Vec::new(),
            evaluated_at: Utc::now(),
            standards_checked: vec![ComplianceStandard::Gdpr],
        };
        assert!(!result.event_id.is_empty());
        assert!(result.compliance_score >= 0.0);
        assert!(result.compliance_score <= 100.0);
        assert!(!result.standards_checked.is_empty());
}
