// SPDX-License-Identifier: AGPL-3.0-only

//! # Compliance Types Module
//!
//! This module provides compliance-related types for the `BearDog` ecosystem.
//!
//! **Configuration types** are imported from the canonical location in beardog-types.
//! **Runtime/operational types** (events, reports, metrics) are defined here.

use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};
use std::collections::HashMap;

// ============================================================================
// CONFIGURATION TYPES - Imported from canonical location
// ============================================================================

// Import configuration types from canonical location
// Note: File is in domains/ directory but exported at config:: level
pub use beardog_types::canonical::config::compliance::{
    ComplianceStandard, ConsolidatedComplianceConfiguration as ComplianceConfig,
    DataSovereigntyConfiguration as DataSovereigntyConfig,
    PrivacyAuditConfiguration as PrivacyAuditConfig, ReportFormat, ReportFrequency,
    ReportingConfiguration as ReportingConfig,
};

// ============================================================================
// RUNTIME/OPERATIONAL TYPES - Defined locally
// ============================================================================
// These types represent runtime data (events, reports, metrics) rather than
// configuration, so they appropriately remain in the beardog-compliance crate.

/// What occurred in the system, used to select which compliance rules apply.
#[derive(Debug, Clone, PartialEq, Eq, Hash, serde::Serialize, serde::Deserialize)]
pub enum ComplianceEventType {
    /// Read or export of personal or sensitive data.
    DataAccess,
    /// Create, update, or transform stored data.
    DataModification,
    /// Erasure or anonymization of records.
    DataDeletion,
    /// Login, API call, or physical access to a protected system.
    SystemAccess,
    /// Change to security, network, or compliance-related settings.
    ConfigurationChange,
    /// Detected breach of internal or external policy.
    PolicyViolation,
    /// Suspected or confirmed attack, leak, or integrity event.
    SecurityIncident,
    /// Formal audit logging or evidence collection activity.
    AuditEvent,
    /// Ledger, payment, or other financially regulated operation.
    FinancialTransaction,
}

/// Relative urgency of a violation or event for triage and reporting.
#[derive(
    Debug, Clone, PartialEq, Eq, Hash, PartialOrd, Ord, serde::Serialize, serde::Deserialize,
)]
pub enum ComplianceSeverity {
    /// Informational or best-practice deviation.
    Low,
    /// Should be reviewed within normal operational windows.
    Medium,
    /// Significant exposure or control gap.
    High,
    /// Immediate remediation or incident response expected.
    Critical,
}

/// Terminal state of an audited action (supports both legacy and explicit success/failure labels).
#[derive(Debug, Clone, PartialEq, Eq, Hash, serde::Serialize, serde::Deserialize)]
pub enum AuditOutcome {
    /// Legacy: all checks passed.
    Passed,
    /// Legacy: one or more checks failed.
    Failed,
    /// Completed with policy warnings that do not block the action.
    Warning,
    /// Control intentionally does not apply to this case.
    NotApplicable,
    /// Explicit success (mirrors [`Passed`] for newer call sites).
    Success,
    /// Explicit failure (mirrors [`Failed`] for newer call sites).
    Failure,
}

/// How end-user consent was obtained, for privacy law evidence trails.
#[derive(Debug, Clone, PartialEq, Eq, Hash, serde::Serialize, serde::Deserialize)]
pub enum ConsentMethod {
    /// Affirmative action required before processing.
    OptIn,
    /// Processing allowed until the subject objects.
    OptOut,
    /// Implied by context (e.g. strict necessity); document carefully.
    Implicit,
    /// Recorded affirmative statement or signature.
    Explicit,
}

/// Qualitative risk score used in DPIAs and breach assessments.
#[derive(Debug, Clone, PartialEq, Eq, Hash, serde::Serialize, serde::Deserialize)]
pub enum RiskLevel {
    /// Negligible impact or likelihood under policy.
    VeryLow,
    /// Minor impact; routine monitoring.
    Low,
    /// Moderate; needs owner review.
    Medium,
    /// Major harm or likelihood without mitigation.
    High,
    /// Severe or systemic; escalate immediately.
    VeryHigh,
}

/// Approval workflow state for privacy or security sign-off.
#[derive(Debug, Clone, PartialEq, Eq, Hash, serde::Serialize, serde::Deserialize)]
pub enum ApprovalStatus {
    /// Awaiting decision.
    Pending,
    /// Formally accepted.
    Approved,
    /// Denied; processing must not proceed as proposed.
    Rejected,
    /// Previously approved scope or consent no longer valid.
    Expired,
}

/// Category of security or privacy incident for regulatory templates.
#[derive(Debug, Clone, PartialEq, Eq, Hash, serde::Serialize, serde::Deserialize)]
pub enum BreachType {
    /// Unauthorized disclosure or loss of data at rest or in motion.
    DataBreach,
    /// Compromise of platform integrity or availability.
    SystemBreach,
    /// Credential or permission abuse.
    AccessBreach,
    /// Unsafe or unauthorized configuration change.
    ConfigurationBreach,
}

/// Lifecycle stage from first report through closure.
#[derive(Debug, Clone, PartialEq, Eq, Hash, serde::Serialize, serde::Deserialize)]
pub enum IncidentStatus {
    /// Reported but not yet assigned.
    Open,
    /// Containment, investigation, or remediation underway.
    InProgress,
    /// Root cause addressed; may await formal sign-off.
    Resolved,
    /// Fully closed with documentation retained.
    Closed,
}

/// Outcome of scoring one [`ComplianceEvent`] against its declared [`ComplianceStandard`].
#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
pub struct ComplianceResult {
    /// Framework that was evaluated.
    pub standard: ComplianceStandard,
    /// True when no violations were recorded for this evaluation.
    pub passed: bool,
    /// Normalized score in roughly `0.0`–`100.0` (higher is better).
    pub score: f64,
    /// Human-readable violation summaries for operators.
    pub violations: Vec<String>,
    /// Suggested remediations or next steps.
    pub recommendations: Vec<String>,
    /// When this summary was produced (UTC).
    pub timestamp: DateTime<Utc>,
}

// Config structs imported from canonical location above

/// Input to the compliance engine describing something that happened in the ecosystem.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ComplianceEvent {
    /// Correlation id from the producer (often a UUID string).
    pub id: String,
    /// When the underlying action occurred (UTC).
    pub timestamp: DateTime<Utc>,
    /// High-level classification driving rule selection.
    pub event_type: ComplianceEventType,
    /// Primary framework this evaluation should be attributed to.
    pub standard: ComplianceStandard,
    /// Free-text summary for audit logs and dashboards.
    pub description: String,
    /// Caller-estimated severity before rule evaluation.
    pub severity: ComplianceSeverity,
    /// Structured payload (paths, attributes, policy ids) for checkers.
    pub metadata: serde_json::Value,
}

/// Scheduled or on-demand report bundle suitable for export to GRC tools.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ComplianceReport {
    /// Framework covered by this report.
    pub standard: ComplianceStandard,
    /// True if every attached violation list is empty for the window.
    pub passed: bool,
    /// Aggregate score for the reporting period.
    pub score: f64,
    /// Structured violations with remediation metadata.
    pub violations: Vec<ComplianceViolation>,
    /// Executive or operational recommendations.
    pub recommendations: Vec<String>,
    /// Report generation time (UTC).
    pub timestamp: DateTime<Utc>,
}

/// A single failed control produced by a checker or validator.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ComplianceViolation {
    /// Stable identifier for ticketing (may mirror policy id).
    pub id: String,
    /// Policy clause or automated rule name that fired.
    pub rule: String,
    /// Why the control failed, in operator-facing language.
    pub description: String,
    /// How urgent remediation is.
    pub severity: ComplianceSeverity,
    /// Concrete steps or references to fix the gap.
    pub remediation: String,
    /// Data class or system component in scope, if applicable.
    pub affected_data: Option<String>,
}

/// One append-only row in the compliance handler's audit trail.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct AuditEntry {
    /// Unique row id (often a UUID string).
    pub id: String,
    /// When the evaluation or action was recorded (UTC).
    pub timestamp: DateTime<Utc>,
    /// Acting principal when known (`system`, user id, service account).
    pub user_id: Option<String>,
    /// Verb or workflow step (e.g. `compliance_evaluation_DataAccess`).
    pub action: String,
    /// Resource or subject string (often mirrors event description).
    pub resource: String,
    /// Whether the step succeeded, failed, or was out of scope.
    pub outcome: AuditOutcome,
    /// Machine-readable JSON for SIEM export (scores, counts, ids).
    pub details: serde_json::Value,
}

/// Article-30 style record of processing for GDPR and similar regimes.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PrivacyRecord {
    /// Internal record id.
    pub id: String,
    /// Pseudonymous or external subject identifier.
    pub data_subject_id: String,
    /// Business reason the data is processed.
    pub processing_purpose: String,
    /// Lawful basis string (contract, consent, legitimate interest, etc.).
    pub legal_basis: String,
    /// Categories of personal data involved.
    pub data_categories: Vec<String>,
    /// Processors or controllers receiving data.
    pub recipients: Vec<String>,
    /// Planned retention as a duration when policy defines one.
    pub retention_period: Option<chrono::Duration>,
    /// Jurisdictions or entities involved in transfers.
    pub cross_border_transfers: Vec<String>,
    /// When this record was captured or last updated (UTC).
    pub timestamp: DateTime<Utc>,
}

/// Demonstrates valid consent (or withdrawal) for a specific processing purpose.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ConsentRecord {
    /// Record id for audits.
    pub id: String,
    /// Subject this consent applies to.
    pub data_subject_id: String,
    /// Narrow purpose tied to the consent text shown to the user.
    pub purpose: String,
    /// Whether consent is currently affirmative.
    pub consent_given: bool,
    /// When consent was granted (UTC).
    pub consent_date: DateTime<Utc>,
    /// When the subject revoked consent, if applicable.
    pub withdrawal_date: Option<DateTime<Utc>>,
    /// Mechanism used to capture consent.
    pub consent_method: ConsentMethod,
    /// Pointer to proof (document hash, URL, ticket id).
    pub evidence: String,
}

/// Data protection impact assessment (DPIA) snapshot for high-risk processing.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PrivacyImpactAssessment {
    /// Assessment record id.
    pub id: String,
    /// Human-readable initiative or system name.
    pub project_name: String,
    /// When the DPIA was performed (UTC).
    pub assessment_date: DateTime<Utc>,
    /// Personal or sensitive data classes in scope.
    pub data_types: Vec<String>,
    /// Identified risks before mitigation.
    pub risks_identified: Vec<PrivacyRisk>,
    /// Controls or process changes that reduce risk.
    pub mitigation_measures: Vec<String>,
    /// Risk after mitigations, per assessor judgment.
    pub residual_risk_level: RiskLevel,
    /// Sign-off workflow state.
    pub approval_status: ApprovalStatus,
}

/// Single risk row inside a [`PrivacyImpactAssessment`].
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PrivacyRisk {
    /// Narrative of what could go wrong.
    pub description: String,
    /// Estimated probability before controls.
    pub likelihood: RiskLevel,
    /// Estimated harm if the risk materializes.
    pub impact: RiskLevel,
    /// Combined rating used for prioritization.
    pub overall_risk: RiskLevel,
}

/// Regulatory-oriented incident log used for breach timelines and notifications.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct DataBreachRecord {
    /// Case id in the compliance system.
    pub id: String,
    /// When the incident began or was believed to have begun (UTC).
    pub incident_date: DateTime<Utc>,
    /// When the organization became aware (UTC).
    pub discovery_date: DateTime<Utc>,
    /// High-level breach category.
    pub incident_type: BreachType,
    /// Count of individuals or rows affected, when known.
    pub affected_records: u64,
    /// Data categories involved (PII, PHI, financial, etc.).
    pub data_types_affected: Vec<String>,
    /// Root cause or contributing factors.
    pub cause: String,
    /// Steps taken to limit spread or recurrence.
    pub containment_measures: Vec<String>,
    /// Whether law or contract mandates regulator or subject notice.
    pub notification_required: bool,
    /// When notifications were sent, if required.
    pub notification_date: Option<DateTime<Utc>>,
    /// Whether a regulator filing was completed.
    pub regulatory_reported: bool,
    /// Current state in the incident workflow.
    pub status: IncidentStatus,
}

/// Aggregate dashboard view built from recent evaluations and trail length.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ComplianceMetrics {
    /// Weighted score across enabled standards.
    pub overall_score: f64,
    /// Per-standard scores for heatmaps or drill-down.
    pub standards_compliance: HashMap<ComplianceStandard, f64>,
    /// Latest violations kept for trending widgets.
    pub recent_violations: Vec<ComplianceViolation>,
    /// Number of rows currently retained in the handler audit trail.
    pub audit_trail_size: u64,
    /// Last full or sample assessment timestamp, if tracked.
    pub last_assessment_date: Option<DateTime<Utc>>,
    /// Next scheduled assessment deadline from policy.
    pub next_assessment_due: Option<DateTime<Utc>>,
}

/// Jurisdiction-specific framework hooks beyond shared [`ComplianceStandard`] variants.
pub enum ComplianceFramework {
    /// California Consumer Privacy Act.
    Ccpa,
}
