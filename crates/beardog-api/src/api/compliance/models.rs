

use serde::{Deserialize, Serialize};
use std::collections::HashMap;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ComplianceStatusResponse {
    pub overall_compliance_score: f64,
    pub status: String,
    pub last_assessment: String,
    pub frameworks: Vec<ComplianceFrameworkStatus>,
    pub pending_audits: u32,
    pub active_violations: u32,
    pub remediation_tasks: u32,
}
pub struct ComplianceFrameworkStatus {
    pub name: String,
    pub score: f64,
    pub last_audit: String,
    pub next_audit: String,
    pub violations_count: u32,
    pub critical_issues: u32,
}

pub struct ComplianceOverviewResponse {
    pub assessment_period: String,
    pub total_controls_assessed: u32,
    pub controls_compliant: u32,
    pub controls_non_compliant: u32,
    pub controls_not_applicable: u32,
    pub compliance_percentage: f64,
    pub risk_score: f64,
    pub audit_findings: AuditFindings,
    pub recent_activities: Vec<String>,
    pub upcoming_milestones: Vec<ComplianceMilestone>,
pub struct AuditFindings {
    pub total_findings: u32,
    pub critical: u32,
    pub high: u32,
    pub medium: u32,
    pub low: u32,
    pub resolved: u32,
    pub pending: u32,
}

pub struct ComplianceMilestone {
    pub due_date: String,
    pub priority: String,
    pub owner: String,
pub struct ComplianceHealthResponse {
    pub system_status: String,
    pub monitoring_active: bool,
    pub audit_logging_active: bool,
    pub policy_enforcement_active: bool,
    pub alert_system_active: bool,
    pub components: Vec<ComplianceComponentHealth>,
    pub metrics: ComplianceMetrics,
}

pub struct ComplianceComponentHealth {
    pub health_percentage: f64,
    pub last_check: String,
    pub issues: Vec<String>,
pub struct ComplianceMetrics {
    pub audit_events_per_hour: u32,
    pub policy_evaluations_per_minute: u32,
    pub violation_alerts_24h: u32,
    pub system_uptime_percentage: f64,
}

pub struct AuditTrailResponse {
    pub events: Vec<AuditEvent>,
    pub total_count: u32,
    pub page: u32,
    pub per_page: u32,
    pub total_pages: u32,
    pub has_more: bool,
    pub filters_applied: AuditFilters,
pub struct AuditEvent {
    pub id: String,
    pub timestamp: String,
    pub event_type: String,
    pub severity: String,
    pub actor: String,
    pub resource: String,
    pub action: String,
    pub outcome: String,
    pub ip_address: Option<String>,
    pub user_agent: Option<String>,
    pub session_id: Option<String>,
    pub metadata: HashMap<String, String>,
}

pub struct AuditFilters {
    pub start_date: Option<String>,
    pub end_date: Option<String>,
    pub event_type: Option<String>,
    pub severity: Option<String>,
    pub actor: Option<String>,
pub struct GdprComplianceResponse {
    pub compliance_status: String,
    pub compliance_score: f64,
    pub data_processing_activities: DataProcessingActivities,
    pub data_subject_rights: DataSubjectRights,
    pub privacy_by_design: PrivacyByDesign,
    pub international_transfers: InternationalTransfers,
}

pub struct DataProcessingActivities {
    pub total_activities: u32,
    pub lawful_basis_documented: u32,
    pub consent_mechanisms_active: u32,
    pub legitimate_interest_assessments: u32,
pub struct DataSubjectRights {
    pub requests_this_month: u32,
    pub access_requests: u32,
    pub rectification_requests: u32,
    pub erasure_requests: u32,
    pub portability_requests: u32,
    pub average_response_time_hours: f64,
    pub compliance_rate: f64,
}

pub struct PrivacyByDesign {
    pub impact_assessments_completed: u32,
    pub data_minimization_score: f64,
    pub purpose_limitation_score: f64,
    pub storage_limitation_score: f64,
    pub security_measures_score: f64,
pub struct InternationalTransfers {
    pub adequacy_decisions_used: u32,
    pub standard_contractual_clauses: u32,
    pub binding_corporate_rules: u32,
    pub derogations_used: u32,

pub struct AuditTrailQuery {
    pub page: Option<u32>,
    pub per_page: Option<u32>,
}

pub struct LogAuditEventRequest {
pub struct LogAuditEventResponse {
    pub event_id: String,
    pub retention_period_days: u32,
    pub compliance_frameworks: Vec<String>,
    pub encrypted: bool,
    pub tamper_proof: bool,
}

pub struct DataSubjectRequest {
    pub data_subject_id: String,
    pub request_type: String, // "access", "rectification", "erasure", "portability"
    pub contact_email: String,
    pub identity_verification: String,
    pub specific_data_categories: Option<Vec<String>>,
    pub reason: Option<String>,
pub struct DataSubjectRequestResponse {
    pub request_id: String,
    pub estimated_completion: String,
    pub request_type: String,
    pub data_controller: String,
    pub legal_basis: String,
    pub processing_steps: Vec<String>,
    pub estimated_data_volume: String,
}

pub struct RightToBeForgottenRequest {
    pub verification_method: String,
    pub reason_for_erasure: String,
    pub scope: String, // "all", "specific_categories", "time_period"
    pub specific_categories: Option<Vec<String>>,
    pub time_period_start: Option<String>,
    pub time_period_end: Option<String>,
pub struct RightToBeForgottenResponse {
    pub erasure_id: String,
    pub data_categories_identified: Vec<String>,
    pub systems_affected: Vec<String>,
    pub verification_required: bool,
    pub third_party_notifications: Vec<String>,
    pub exceptions_identified: Vec<String>,
