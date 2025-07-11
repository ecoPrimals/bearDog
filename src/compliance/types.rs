//! Type definitions and data structures for compliance monitoring
//! 
//! Contains all structs, enums, and type aliases for compliance operations.

use chrono::{DateTime, Duration, Utc};
use serde::{Deserialize, Serialize};
use std::collections::HashMap;

/// Compliance configuration
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ComplianceConfig {
    /// List of enabled compliance standards
    pub enabled_standards: Vec<ComplianceStandard>,
    /// Monitoring interval for compliance checks
    pub monitoring_interval: Duration,
    /// Audit log retention period
    pub audit_retention: Duration,
    /// Dashboard refresh interval
    pub dashboard_refresh_interval: Duration,
    /// Reporting configuration
    pub reporting: ReportingConfig,
}

impl Default for ComplianceConfig {
    fn default() -> Self {
        Self {
            enabled_standards: vec![
                ComplianceStandard::GDPR,
                ComplianceStandard::SOX,
                ComplianceStandard::PCI_DSS,
            ],
            monitoring_interval: Duration::minutes(5),
            audit_retention: Duration::days(365),
            dashboard_refresh_interval: Duration::minutes(1),
            reporting: ReportingConfig::default(),
        }
    }
}

/// Reporting configuration
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ReportingConfig {
    /// Automatically generate reports
    pub auto_generate: bool,
    /// Report generation interval
    pub generation_interval: Duration,
    /// Storage path for reports
    pub storage_path: String,
    /// Report formats to generate
    pub formats: Vec<ReportFormat>,
}

impl Default for ReportingConfig {
    fn default() -> Self {
        Self {
            auto_generate: true,
            generation_interval: Duration::days(1),
            storage_path: "./reports".to_string(),
            formats: vec![ReportFormat::JSON, ReportFormat::PDF],
        }
    }
}

/// Report format options
#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum ReportFormat {
    PDF,
    JSON,
    CSV,
    HTML,
}

/// Compliance standards supported by BearDog
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq, Hash)]
pub enum ComplianceStandard {
    /// GDPR (General Data Protection Regulation)
    GDPR,
    /// SOX (Sarbanes-Oxley Act)
    SOX,
    /// PCI DSS (Payment Card Industry Data Security Standard)
    PciDss,
    /// PCI DSS (Payment Card Industry Data Security Standard) - alternative name
    #[allow(non_camel_case_types)]
    PCI_DSS,
    /// HIPAA (Health Insurance Portability and Accountability Act)
    HIPAA,
    /// ISO 27001 (Information Security Management)
    ISO27001,
    /// FedRAMP (Federal Risk and Authorization Management Program)
    FedRAMP,
}

/// Compliance event for monitoring
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ComplianceEvent {
    /// Unique event identifier
    pub id: String,
    /// Type of event (e.g., "DataAccess", "PolicyViolation")
    pub event_type: String,
    /// Event timestamp
    pub timestamp: DateTime<Utc>,
    /// User who triggered the event
    pub user_id: Option<String>,
    /// Resource affected by the event
    pub resource: Option<String>,
    /// Additional event data
    pub data: HashMap<String, String>,
    /// Additional metadata about the event
    pub metadata: HashMap<String, String>,
}

/// Compliance violation details
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ComplianceViolation {
    pub id: String,
    pub standard: ComplianceStandard,
    pub event_id: String,
    pub violation_type: String,
    pub severity: ComplianceSeverity,
    pub description: String,
    pub timestamp: DateTime<Utc>,
    pub remediation_required: bool,
}

/// Compliance warning details
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ComplianceWarning {
    pub id: String,
    pub standard: ComplianceStandard,
    pub warning_type: String,
    pub description: String,
    pub timestamp: DateTime<Utc>,
}

/// Compliance severity levels
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, PartialOrd, Eq, Hash)]
pub enum ComplianceSeverity {
    /// Information level
    Info,
    /// Warning level
    Warning,
    /// Violation level
    Violation,
    /// Critical level
    Critical,
}

/// Result of compliance evaluation for a single event
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ComplianceResult {
    pub event_id: String,
    pub compliance_score: f64,
    pub violations: Vec<ComplianceViolation>,
    pub warnings: Vec<ComplianceWarning>,
    pub evaluated_at: DateTime<Utc>,
    pub standards_checked: Vec<ComplianceStandard>,
}

/// Compliance evaluation for a standard
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct StandardEvaluationResult {
    pub score: f64,
    pub violations: Vec<ComplianceViolation>,
    pub warnings: Vec<ComplianceWarning>,
}

/// Date range for reports
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct DateRange {
    pub start_date: DateTime<Utc>,
    pub end_date: DateTime<Utc>,
}

/// Compliance report
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ComplianceReport {
    pub id: String,
    pub title: String,
    pub standard: ComplianceStandard,
    pub period: String,
    pub generated_at: DateTime<Utc>,
    pub overall_score: f64,
    pub total_events: u64,
    pub violations_found: u64,
    pub warnings_issued: u64,
    pub recommendations: Vec<String>,
    pub detailed_findings: Vec<String>,
    pub summary: String,
}

/// Compliance dashboard data
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ComplianceDashboard {
    pub compliance_status: ComplianceStatus,
    pub recent_events: Vec<ComplianceEvent>,
    pub active_violations: Vec<ComplianceViolation>,
    pub recent_warnings: Vec<ComplianceWarning>,
    pub recommendations: Vec<String>,
    pub metrics: ComplianceMetrics,
}

/// Overall compliance status
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ComplianceStatus {
    pub overall_status: String,
    pub compliance_percentage: f64,
    pub active_violations: u64,
    pub total_events_today: u64,
    pub last_updated: DateTime<Utc>,
}

/// Compliance metrics
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ComplianceMetrics {
    pub events_processed_today: u64,
    pub violations_detected_today: u64,
    pub warnings_issued_today: u64,
    pub average_compliance_score: f64,
    pub standards_monitored: Vec<ComplianceStandard>,
}
