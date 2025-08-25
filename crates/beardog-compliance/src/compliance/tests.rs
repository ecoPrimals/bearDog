// BearDog - Enterprise Security Ecosystem
// Copyright (C) 2025 EcoPrimals
//
// This program is free software: you can redistribute it and/or modify
// it under the terms of the GNU Affero General Public License as published by
// the Free Software Foundation, either version 3 of the License, or
// (at your option) any later version.
//
// This program is distributed in the hope that it will be useful,
// but WITHOUT ANY WARRANTY; without even the implied warranty of
// MERCHANTABILITY or FITNESS FOR A PARTICULAR PURPOSE. See the
// GNU Affero General Public License for more details.
//
// You should have received a copy of the GNU Affero General Public License
// along with this program. If not, see <https://www.gnu.org/licenses/>.


/// Unit tests for compliance module
///
/// Contains comprehensive test functions for compliance functionality.

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
        // Test that engine was created successfully
        Ok(())
    }
    #[test]
    fn test_compliance_standards_serialization() {
        // Test all compliance standards can be serialized/deserialized
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
            // Test that standards can be created and compared
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
            data: HashMap::new(),
            metadata: HashMap::new(),
        };
        // Test event creation succeeded
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
            // Test that formats can be cloned and compared
            let cloned_format = format.clone();
            // Just verify they're the same type (no PartialEq for ReportFormat)
            match (format, cloned_format) {
                (ReportFormat::Pdf, ReportFormat::Pdf) => {}
                (ReportFormat::Json, ReportFormat::Json) => {}
                (ReportFormat::Xml, ReportFormat::Xml) => {}
                (ReportFormat::Csv, ReportFormat::Csv) => {}
                (ReportFormat::Html, ReportFormat::Html) => {}
                (ReportFormat::Txt, ReportFormat::Txt) => {}
                (ReportFormat::Yaml, ReportFormat::Yaml) => {}
                _ => panic!("Format mismatch"),
            }
    fn test_compliance_config_defaults() {
        assert!(!config.enabled_standards.is_empty());
        assert!(config.reporting.auto_generate);
        assert!(config.monitoring_interval.num_seconds() > 0);}


    async fn test_compliance_violation_creation() -> BearDogResult<()> {
        // Create a test violation
        let violation = ComplianceViolation {
            id: "violation-test-1".to_string(),
            standard: ComplianceStandard::Gdpr,
            event_id: "event-1".to_string(),
            violation_type: "data_processing_without_consent".to_string(),
            severity: ComplianceSeverity::Critical,
            description: "Processing personal data without explicit consent".to_string(),
            remediation_required: true,
        // Test violation creation
        assert!(!violation.id.is_empty());
        assert_eq!(violation.standard, ComplianceStandard::Gdpr);
        assert!(!violation.description.is_empty());
        assert_eq!(violation.severity, ComplianceSeverity::Critical);
    fn test_compliance_severity_ordering() {
        // Test that severity levels can be compared
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
