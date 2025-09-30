//! # Compliance Types Module
//!
//! This module provides compliance-related types for the BearDog ecosystem.
//! 
//! **NOTE**: These types are local to beardog-compliance and provide rich compliance functionality.
//! A simpler CanonicalComplianceConfig exists in beardog-types for basic config needs.

use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};
use std::collections::HashMap;

// Local compliance configuration types (richer than canonical)
#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
pub struct ComplianceConfig {
    /// Collection of standards
    pub standards: Vec<ComplianceStandard>,
    /// Whether `feature_standards` is enabled
    pub enabled_standards: Vec<ComplianceStandard>,
    /// The reporting value
    pub reporting: ReportingConfig,
    /// The data sovereignty value
    pub data_sovereignty: DataSovereigntyConfig,
    /// The privacy audit value
    pub privacy_audit: PrivacyAuditConfig,
}

impl Default for ComplianceConfig {
    fn default() -> Self {
        Self {
            standards: vec![ComplianceStandard::Gdpr],
            enabled_standards: vec![ComplianceStandard::Gdpr],
            reporting: ReportingConfig::default(),
            data_sovereignty: DataSovereigntyConfig::default(),
            privacy_audit: PrivacyAuditConfig::default(),
        }
    }
}

#[derive(Debug, Clone, PartialEq, Eq, Hash, serde::Serialize, serde::Deserialize)]
pub enum ComplianceStandard {
    /// Represents gdpr variant
    Gdpr,
    /// Represents hipaa variant
    Hipaa,
    /// Represents sox variant
    Sox,
    /// Represents pci variant
    Pci,
    /// Represents pci dss variant
    PciDss,
    /// Represents iso27001 variant
    Iso27001,
    /// Represents soc2 variant
    Soc2,
    /// Represents ccpa variant
    Ccpa,
    /// Represents custom variant
    Custom(String),
}

impl std::fmt::Display for ComplianceStandard {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::Gdpr => write!(f, "GDPR"),
            Self::Hipaa => write!(f, "HIPAA"),
            Self::Sox => write!(f, "SOX"),
            Self::Pci => write!(f, "PCI"),
            Self::PciDss => write!(f, "PCI-DSS"),
            Self::Iso27001 => write!(f, "ISO-27001"),
            Self::Soc2 => write!(f, "SOC-2"),
            Self::Ccpa => write!(f, "CCPA"),
            Self::Custom(name) => write!(f, "{name}"),
        }
    }
}

#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
pub struct DataSovereigntyConfig {
    /// The jurisdiction value
    pub jurisdiction: String,
    pub data_residency_required: bool,
    /// Collection of cross border restrictions
    pub cross_border_restrictions: Vec<String>,
}

impl Default for DataSovereigntyConfig {
    fn default() -> Self {
        Self {
            jurisdiction: "US".to_string(),
            data_residency_required: true,
            cross_border_restrictions: vec![],
        }
    }
}

#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
pub struct PrivacyAuditConfig {
    /// The frequency value
    pub frequency: ReportFrequency,
    /// Whether automated is enabled
    pub automated: bool,
    /// Optional external auditor
    pub external_auditor: Option<String>,
}

impl Default for PrivacyAuditConfig {
    fn default() -> Self {
        Self {
            frequency: ReportFrequency::Quarterly,
            automated: true,
            external_auditor: None,
        }
    }
}

#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
pub struct ReportingConfig {
    pub format: ReportFormat,
    /// The frequency value
    pub frequency: ReportFrequency,
    /// Collection of recipients
    pub recipients: Vec<String>,
}

impl Default for ReportingConfig {
    fn default() -> Self {
        Self {
            format: ReportFormat::Json,
            frequency: ReportFrequency::Monthly,
            recipients: vec![],
        }
    }
}

#[derive(Debug, Clone, PartialEq, serde::Serialize, serde::Deserialize)]
pub enum ReportFormat {
    /// Represents json variant
    Json,
    /// Represents xml variant
    Xml,
    /// Represents pdf variant
    Pdf,
    /// Represents html variant
    Html,
    /// Represents csv variant
    Csv,
}

#[derive(Debug, Clone, PartialEq, serde::Serialize, serde::Deserialize)]
pub enum ReportFrequency {
    /// Represents daily variant
    Daily,
    /// Represents weekly variant
    Weekly,
    /// Represents monthly variant
    Monthly,
    /// Represents quarterly variant
    Quarterly,
    /// Represents annually variant
    Annually,
}

// Additional canonical compliance types
#[derive(Debug, Clone, PartialEq, Eq, Hash, serde::Serialize, serde::Deserialize)]
/// Types of compliance event
pub enum ComplianceEventType {
    /// Represents data access variant
    DataAccess,
    /// Represents data modification variant
    DataModification,
    /// Represents data deletion variant
    DataDeletion,
    /// Represents system access variant
    SystemAccess,
    /// Represents configuration change variant
    ConfigurationChange,
    /// Represents policy violation variant
    PolicyViolation,
    /// Represents security incident variant
    SecurityIncident,
    /// Represents audit event variant
    AuditEvent,
    /// Represents financial transaction variant
    FinancialTransaction, // Added missing variant
}

#[derive(
    Debug, Clone, PartialEq, Eq, Hash, PartialOrd, Ord, serde::Serialize, serde::Deserialize,
)]
pub enum ComplianceSeverity {
    /// Represents low variant
    Low,
    /// Represents medium variant
    Medium,
    /// Represents high variant
    High,
    /// Represents critical variant
    Critical,
}

#[derive(Debug, Clone, PartialEq, Eq, Hash, serde::Serialize, serde::Deserialize)]
pub enum AuditOutcome {
    /// State indicating passed
    Passed,
    /// Error or failure state
    Failed,
    /// Currently warning
    Warning,
    /// Represents not applicable variant
    NotApplicable,
    /// Successful completion state
    Success, // Added missing variant
    /// Error or failure state
    Failure, // Added missing variant
}

#[derive(Debug, Clone, PartialEq, Eq, Hash, serde::Serialize, serde::Deserialize)]
pub enum ConsentMethod {
    /// Represents opt in variant
    OptIn,
    /// Represents opt out variant
    OptOut,
    /// Represents implicit variant
    Implicit,
    /// Represents explicit variant
    Explicit,
}

#[derive(Debug, Clone, PartialEq, Eq, Hash, serde::Serialize, serde::Deserialize)]
pub enum RiskLevel {
    /// Represents very low variant
    VeryLow,
    /// Represents low variant
    Low,
    /// Represents medium variant
    Medium,
    /// Represents high variant
    High,
    /// Represents very high variant
    VeryHigh,
}

#[derive(Debug, Clone, PartialEq, Eq, Hash, serde::Serialize, serde::Deserialize)]
pub enum ApprovalStatus {
    /// Operation in progress
    Pending,
    /// State indicating approved
    Approved,
    /// State indicating rejected
    Rejected,
    /// State indicating expired
    Expired,
}

#[derive(Debug, Clone, PartialEq, Eq, Hash, serde::Serialize, serde::Deserialize)]
/// Types of breach
pub enum BreachType {
    /// Represents data breach variant
    DataBreach,
    /// Represents system breach variant
    SystemBreach,
    /// Represents access breach variant
    AccessBreach,
    /// Represents configuration breach variant
    ConfigurationBreach,
}

#[derive(Debug, Clone, PartialEq, Eq, Hash, serde::Serialize, serde::Deserialize)]
pub enum IncidentStatus {
    /// Represents open variant
    Open,
    /// Operation in progress
    InProgress,
    /// State indicating resolved
    Resolved,
    /// State indicating closed
    Closed,
}

// Additional canonical compliance types
#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
pub struct ComplianceResult {
    /// The standard value
    pub standard: ComplianceStandard,
    /// Whether passed is enabled
    pub passed: bool,
    /// The score value
    pub score: f64,
    /// Collection of violations
    pub violations: Vec<String>,
    /// Collection of recommendations
    pub recommendations: Vec<String>,
    pub timestamp: DateTime<Utc>,
}

// Config structs imported from canonical location above

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ComplianceEvent {
    pub id: String,
    pub timestamp: DateTime<Utc>,
    /// The event type value
    pub event_type: ComplianceEventType,
    /// The standard value
    pub standard: ComplianceStandard,
    /// The description value
    pub description: String,
    /// The severity value
    pub severity: ComplianceSeverity,
    /// The metadata value
    pub metadata: serde_json::Value,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ComplianceReport {
    /// The standard value
    pub standard: ComplianceStandard,
    /// Whether passed is enabled
    pub passed: bool,
    /// The score value
    pub score: f64,
    /// Collection of violations
    pub violations: Vec<ComplianceViolation>,
    /// Collection of recommendations
    pub recommendations: Vec<String>,
    pub timestamp: DateTime<Utc>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ComplianceViolation {
    pub id: String,
    /// The rule value
    pub rule: String,
    /// The description value
    pub description: String,
    /// The severity value
    pub severity: ComplianceSeverity,
    /// The remediation value
    pub remediation: String,
    /// Optional affected data
    pub affected_data: Option<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct AuditEntry {
    pub id: String,
    pub timestamp: DateTime<Utc>,
    pub user_id: Option<String>,
    /// The action value
    pub action: String,
    /// The resource value
    pub resource: String,
    /// The outcome value
    pub outcome: AuditOutcome,
    /// The details value
    pub details: serde_json::Value,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PrivacyRecord {
    pub id: String,
    pub data_subject_id: String,
    /// The processing purpose value
    pub processing_purpose: String,
    /// The legal basis value
    pub legal_basis: String,
    /// Collection of data categories
    pub data_categories: Vec<String>,
    /// Collection of recipients
    pub recipients: Vec<String>,
    /// Optional retention period
    pub retention_period: Option<chrono::Duration>,
    /// Collection of cross border transfers
    pub cross_border_transfers: Vec<String>,
    pub timestamp: DateTime<Utc>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ConsentRecord {
    pub id: String,
    pub data_subject_id: String,
    /// The purpose value
    pub purpose: String,
    /// Whether `consent_given` is enabled
    pub consent_given: bool,
    /// The consent date value
    pub consent_date: DateTime<Utc>,
    /// Optional withdrawal date
    pub withdrawal_date: Option<DateTime<Utc>>,
    /// The consent method value
    pub consent_method: ConsentMethod,
    pub evidence: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PrivacyImpactAssessment {
    pub id: String,
    /// Name of the project
    pub project_name: String,
    /// The assessment date value
    pub assessment_date: DateTime<Utc>,
    /// Collection of data types
    pub data_types: Vec<String>,
    pub risks_identified: Vec<PrivacyRisk>,
    /// Collection of mitigation measures
    pub mitigation_measures: Vec<String>,
    pub residual_risk_level: RiskLevel,
    /// Current status of the approval
    pub approval_status: ApprovalStatus,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PrivacyRisk {
    /// The description value
    pub description: String,
    /// The likelihood value
    pub likelihood: RiskLevel,
    /// The impact value
    pub impact: RiskLevel,
    /// The overall risk value
    pub overall_risk: RiskLevel,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct DataBreachRecord {
    pub id: String,
    pub incident_date: DateTime<Utc>,
    /// The discovery date value
    pub discovery_date: DateTime<Utc>,
    pub incident_type: BreachType,
    /// Number of `affected_records`
    pub affected_records: u64,
    /// Collection of data types affected
    pub data_types_affected: Vec<String>,
    /// The cause value
    pub cause: String,
    /// Collection of containment measures
    pub containment_measures: Vec<String>,
    /// Whether `notification_required` is enabled
    pub notification_required: bool,
    /// Optional notification date
    pub notification_date: Option<DateTime<Utc>>,
    /// Whether `regulatory_reported` is enabled
    pub regulatory_reported: bool,
    /// Current status of the component
    pub status: IncidentStatus,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ComplianceMetrics {
    /// The overall score value
    pub overall_score: f64,
    /// Mapping of standards compliance
    pub standards_compliance: HashMap<ComplianceStandard, f64>,
    /// Collection of recent violations
    pub recent_violations: Vec<ComplianceViolation>,
    /// Number of `audit_trail_size`
    pub audit_trail_size: u64,
    /// Optional last assessment date
    pub last_assessment_date: Option<DateTime<Utc>>,
    /// Optional next assessment due
    pub next_assessment_due: Option<DateTime<Utc>>,
}

/// Supported compliance frameworks
pub enum ComplianceFramework {
    Ccpa,
}
