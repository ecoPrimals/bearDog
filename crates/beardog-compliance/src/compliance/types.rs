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
    /// PDF format
    PDF,
    /// JSON format
    JSON,
    /// CSV format
    CSV,
    /// HTML format
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
    /// Unique violation identifier
    pub id: String,
    /// Compliance standard that was violated
    pub standard: ComplianceStandard,
    /// Event ID that triggered the violation
    pub event_id: String,
    /// Type of violation
    pub violation_type: String,
    /// Severity level of the violation
    pub severity: ComplianceSeverity,
    /// Human-readable description of the violation
    pub description: String,
    /// Timestamp when the violation was detected
    pub timestamp: DateTime<Utc>,
    /// Whether remediation is required
    pub remediation_required: bool,
}

/// Compliance warning details
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ComplianceWarning {
    /// Unique warning identifier
    pub id: String,
    /// Compliance standard that triggered the warning
    pub standard: ComplianceStandard,
    /// Type of warning
    pub warning_type: String,
    /// Human-readable description of the warning
    pub description: String,
    /// Timestamp when the warning was issued
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
    /// Event ID that was evaluated
    pub event_id: String,
    /// Compliance score (0.0 to 100.0)
    pub compliance_score: f64,
    /// List of violations found
    pub violations: Vec<ComplianceViolation>,
    /// List of warnings issued
    pub warnings: Vec<ComplianceWarning>,
    /// Timestamp when evaluation was performed
    pub evaluated_at: DateTime<Utc>,
    /// List of standards that were checked
    pub standards_checked: Vec<ComplianceStandard>,
}

/// Compliance evaluation for a standard
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct StandardEvaluationResult {
    /// Compliance score for this standard (0.0 to 100.0)
    pub score: f64,
    /// List of violations found for this standard
    pub violations: Vec<ComplianceViolation>,
    /// List of warnings issued for this standard
    pub warnings: Vec<ComplianceWarning>,
}

/// Date range for reports
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct DateRange {
    /// Start date of the range
    pub start_date: DateTime<Utc>,
    /// End date of the range
    pub end_date: DateTime<Utc>,
}

/// Compliance report
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ComplianceReport {
    /// Unique report identifier
    pub id: String,
    /// Report title
    pub title: String,
    /// Compliance standard covered by the report
    pub standard: ComplianceStandard,
    /// Time period covered by the report
    pub period: String,
    /// Timestamp when the report was generated
    pub generated_at: DateTime<Utc>,
    /// Overall compliance score (0.0 to 100.0)
    pub overall_score: f64,
    /// Total number of events processed
    pub total_events: u64,
    /// Number of violations found
    pub violations_found: u64,
    /// Number of warnings issued
    pub warnings_issued: u64,
    /// List of recommendations
    pub recommendations: Vec<String>,
    /// Detailed findings
    pub detailed_findings: Vec<String>,
    /// Executive summary
    pub summary: String,
}

/// Compliance dashboard data
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ComplianceDashboard {
    /// Current compliance status
    pub compliance_status: ComplianceStatus,
    /// Recent compliance events
    pub recent_events: Vec<ComplianceEvent>,
    /// Active violations requiring attention
    pub active_violations: Vec<ComplianceViolation>,
    /// Recent warnings issued
    pub recent_warnings: Vec<ComplianceWarning>,
    /// Recommendations for improvement
    pub recommendations: Vec<String>,
    /// Compliance metrics
    pub metrics: ComplianceMetrics,
}

/// Overall compliance status
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ComplianceStatus {
    /// Overall status description
    pub overall_status: String,
    /// Compliance percentage (0.0 to 100.0)
    pub compliance_percentage: f64,
    /// Number of active violations
    pub active_violations: u64,
    /// Total events processed today
    pub total_events_today: u64,
    /// Timestamp of last update
    pub last_updated: DateTime<Utc>,
}

/// Compliance metrics
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ComplianceMetrics {
    /// Number of events processed today
    pub events_processed_today: u64,
    /// Number of violations detected today
    pub violations_detected_today: u64,
    /// Number of warnings issued today
    pub warnings_issued_today: u64,
    /// Average compliance score across all standards
    pub average_compliance_score: f64,
    /// List of standards being monitored
    pub standards_monitored: Vec<ComplianceStandard>,
}
