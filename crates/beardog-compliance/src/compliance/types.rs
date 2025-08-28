use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use uuid::Uuid;

pub use beardog_types::canonical::configuration::{
    ComplianceConfig, ComplianceStandard, DataSovereigntyConfig, PrivacyAuditConfig,
    ReportingConfig,
};

pub use beardog_types::canonical::configuration::compliance::{ReportFormat, ReportFrequency};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ComplianceEvent {
    pub id: Uuid,
    pub timestamp: DateTime<Utc>,
    pub event_type: ComplianceEventType,
    pub standard: ComplianceStandard,
    pub description: String,
    pub severity: ComplianceSeverity,
    pub metadata: serde_json::Value,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
pub enum ComplianceEventType {
    PolicyViolation,
    AuditCheck,
    DataAccess,
    DataModification,
    ConsentUpdate,
    SecurityIncident,
    ComplianceCheck,
    ReportGeneration,
    FinancialTransaction,
    SystemAccess,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq, PartialOrd, Ord)]
pub enum ComplianceSeverity {
    Low,
    Medium,
    High,
    Critical,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ComplianceResult {
    pub standard: ComplianceStandard,
    pub passed: bool,
    pub score: f64,
    pub violations: Vec<ComplianceViolation>,
    pub recommendations: Vec<String>,
    pub timestamp: DateTime<Utc>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ComplianceViolation {
    pub id: Uuid,
    pub rule: String,
    pub description: String,
    pub severity: ComplianceSeverity,
    pub remediation: String,
    pub affected_data: Option<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct AuditEntry {
    pub id: Uuid,
    pub timestamp: DateTime<Utc>,
    pub user_id: Option<String>,
    pub action: String,
    pub resource: String,
    pub outcome: AuditOutcome,
    pub details: serde_json::Value,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
pub enum AuditOutcome {
    Success,
    Failure,
    Partial,
    Denied,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct DataProcessingRecord {
    pub id: Uuid,
    pub data_subject_id: String,
    pub processing_purpose: String,
    pub legal_basis: String,
    pub data_categories: Vec<String>,
    pub recipients: Vec<String>,
    pub retention_period: Option<chrono::Duration>,
    pub cross_border_transfers: Vec<String>,
    pub timestamp: DateTime<Utc>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ConsentRecord {
    pub id: Uuid,
    pub data_subject_id: String,
    pub purpose: String,
    pub consent_given: bool,
    pub consent_date: DateTime<Utc>,
    pub withdrawal_date: Option<DateTime<Utc>>,
    pub consent_method: ConsentMethod,
    pub evidence: String,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
pub enum ConsentMethod {
    ExplicitConsent,
    ImpliedConsent,
    OptIn,
    OptOut,
    LegitimateInterest,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PrivacyImpactAssessment {
    pub id: Uuid,
    pub project_name: String,
    pub assessment_date: DateTime<Utc>,
    pub data_types: Vec<String>,
    pub risks_identified: Vec<PrivacyRisk>,
    pub mitigation_measures: Vec<String>,
    pub residual_risk_level: RiskLevel,
    pub approval_status: ApprovalStatus,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PrivacyRisk {
    pub description: String,
    pub likelihood: RiskLevel,
    pub impact: RiskLevel,
    pub overall_risk: RiskLevel,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq, PartialOrd, Ord)]
pub enum RiskLevel {
    Low,
    Medium,
    High,
    Critical,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
pub enum ApprovalStatus {
    Pending,
    Approved,
    Rejected,
    RequiresRevision,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct DataBreachIncident {
    pub id: Uuid,
    pub incident_date: DateTime<Utc>,
    pub discovery_date: DateTime<Utc>,
    pub incident_type: BreachType,
    pub affected_records: u64,
    pub data_types_affected: Vec<String>,
    pub cause: String,
    pub containment_measures: Vec<String>,
    pub notification_required: bool,
    pub notification_date: Option<DateTime<Utc>>,
    pub regulatory_reported: bool,
    pub status: IncidentStatus,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
pub enum BreachType {
    UnauthorizedAccess,
    DataTheft,
    AccidentalDisclosure,
    SystemCompromise,
    PhysicalLoss,
    Other(String),
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
pub enum IncidentStatus {
    Open,
    InvestigationInProgress,
    ContainmentInProgress,
    Resolved,
    Closed,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ComplianceMetrics {
    pub overall_score: f64,
    pub standards_compliance: HashMap<ComplianceStandard, f64>,
    pub recent_violations: Vec<ComplianceViolation>,
    pub audit_trail_size: u64,
    pub last_assessment_date: Option<DateTime<Utc>>,
    pub next_assessment_due: Option<DateTime<Utc>>,
}

pub enum ComplianceFramework {
    Ccpa,
}
