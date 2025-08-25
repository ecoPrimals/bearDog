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


/// # Compliance Types - Canonical System
///
/// This module provides unified compliance types and standards for regulatory
/// compliance across different jurisdictions and frameworks.

use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use chrono::{DateTime, Utc};
use uuid::Uuid;

/// **CANONICAL MIGRATION COMPLETE** ✅
/// All compliance types now use canonical definitions from beardog-types
pub use beardog_types::canonical::configuration::{
    ComplianceConfig, ComplianceStandard, ReportingConfig,
    PrivacyAuditConfig, DataSovereigntyConfig
};

// Re-export specific types from compliance module
pub use beardog_types::canonical::configuration::compliance::{
    ReportFormat, ReportFrequency
};

/// Compliance event for audit trail
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

/// Types of compliance events
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
}

/// Severity levels for compliance events
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq, PartialOrd, Ord)]
pub enum ComplianceSeverity {
    Low,
    Medium,
    High,
    Critical,
}

/// Compliance check result
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ComplianceResult {
    pub standard: ComplianceStandard,
    pub passed: bool,
    pub score: f64,
    pub violations: Vec<ComplianceViolation>,
    pub recommendations: Vec<String>,
    pub timestamp: DateTime<Utc>,
}

/// Compliance violation details
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ComplianceViolation {
    pub id: Uuid,
    pub rule: String,
    pub description: String,
    pub severity: ComplianceSeverity,
    pub remediation: String,
    pub affected_data: Option<String>,
}

/// Audit trail entry
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

/// Audit outcome
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
pub enum AuditOutcome {
    Success,
    Failure,
    Partial,
    Denied,
}

/// Data processing record for GDPR compliance
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

/// Consent record for privacy compliance
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

/// Method of consent collection
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
pub enum ConsentMethod {
    ExplicitConsent,
    ImpliedConsent,
    OptIn,
    OptOut,
    LegitimateInterest,
}

/// Privacy impact assessment
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

/// Privacy risk assessment
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PrivacyRisk {
    pub description: String,
    pub likelihood: RiskLevel,
    pub impact: RiskLevel,
    pub overall_risk: RiskLevel,
}

/// Risk level enumeration
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq, PartialOrd, Ord)]
pub enum RiskLevel {
    Low,
    Medium,
    High,
    Critical,
}

/// Approval status for assessments
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
pub enum ApprovalStatus {
    Pending,
    Approved,
    Rejected,
    RequiresRevision,
}

/// Data breach incident record
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

/// Types of data breaches
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
pub enum BreachType {
    UnauthorizedAccess,
    DataTheft,
    AccidentalDisclosure,
    SystemCompromise,
    PhysicalLoss,
    Other(String),
}

/// Incident status tracking
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
pub enum IncidentStatus {
    Open,
    InvestigationInProgress,
    ContainmentInProgress,
    Resolved,
    Closed,
}

/// Compliance dashboard metrics
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ComplianceMetrics {
    pub overall_score: f64,
    pub standards_compliance: HashMap<ComplianceStandard, f64>,
    pub recent_violations: Vec<ComplianceViolation>,
    pub audit_trail_size: u64,
    pub last_assessment_date: Option<DateTime<Utc>>,
    pub next_assessment_due: Option<DateTime<Utc>>,
}

// Legacy framework enum for backward compatibility
pub enum ComplianceFramework {
    Ccpa,
}
