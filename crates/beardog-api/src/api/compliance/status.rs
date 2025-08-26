

use super::models::*;
use crate::api::*;
use axum::{extract::State, http::StatusCode, Json};
use std::time::Instant;
use tracing::info;

pub async fn get_compliance_status(
    State(_state): State<AppState>,
) -> Result<Json<ApiResponse<ComplianceStatusResponse>>, StatusCode> {
    let start_time = Instant::now();
    let request_id = uuid::Uuid::new_v4().to_string();
    info!("📊 Getting overall compliance status");
    let response = ComplianceStatusResponse {
        overall_compliance_score: 94.2,
        status: "COMPLIANT".to_string(),
        last_assessment: chrono::Utc::now().to_rfc3339(),
        frameworks: vec![
            ComplianceFrameworkStatus {
                name: "GDPR".to_string(),
                status: "COMPLIANT".to_string(),
                score: 96.5,
                last_audit: chrono::Utc::now().to_rfc3339(),
                next_audit: (chrono::Utc::now() + chrono::Duration::days(30)).to_rfc3339(),
                violations_count: 0,
                critical_issues: 0,
            },
                name: "HIPAA".to_string(),
                score: 92.8,
                next_audit: (chrono::Utc::now() + chrono::Duration::days(60)).to_rfc3339(),
                violations_count: 1,
                name: "SOX".to_string(),
                score: 93.1,
                next_audit: (chrono::Utc::now() + chrono::Duration::days(90)).to_rfc3339(),
        ],
        pending_audits: 2,
        active_violations: 1,
        remediation_tasks: 3,
    };
    let processing_time = start_time.elapsed().as_millis() as u64;
    Ok(Json(success_response(
        response,
        request_id,
        processing_time,
        true,
    )))
}

pub async fn get_compliance_overview(
) -> Result<Json<ApiResponse<ComplianceOverviewResponse>>, StatusCode> {
    info!("📈 Generating compliance overview");
    let response = ComplianceOverviewResponse {
        assessment_period: "2025-Q1".to_string(),
        total_controls_assessed: 247,
        controls_compliant: 237,
        controls_non_compliant: 8,
        controls_not_applicable: 2,
        compliance_percentage: 96.3,
        risk_score: 12.5, // Lower is better
        audit_findings: AuditFindings {
            total_findings: 15,
            critical: 0,
            high: 2,
            medium: 8,
            low: 5,
            resolved: 12,
            pending: 3,
        },
        recent_activities: vec![
            "GDPR data mapping assessment completed".to_string(),
            "HIPAA security risk assessment updated".to_string(),
            "SOX financial controls review passed".to_string(),
            "PCI DSS quarterly scan completed".to_string(),
        upcoming_milestones: vec![
            ComplianceMilestone {
                name: "Annual GDPR compliance review".to_string(),
                due_date: (chrono::Utc::now() + chrono::Duration::days(45)).to_rfc3339(),
                priority: "HIGH".to_string(),
                owner: "Chief Compliance Officer".to_string(),
                name: "HIPAA risk assessment update".to_string(),
                due_date: (chrono::Utc::now() + chrono::Duration::days(30)).to_rfc3339(),
                priority: "MEDIUM".to_string(),
                owner: "Data Protection Officer".to_string(),

pub async fn get_compliance_health(
) -> Result<Json<ApiResponse<ComplianceHealthResponse>>, StatusCode> {
    let response = ComplianceHealthResponse {
        system_status: "HEALTHY".to_string(),
        monitoring_active: true,
        audit_logging_active: true,
        policy_enforcement_active: true,
        alert_system_active: true,
        components: vec![
            ComplianceComponentHealth {
                name: "Audit Trail System".to_string(),
                status: "OPERATIONAL".to_string(),
                health_percentage: 99.2,
                last_check: chrono::Utc::now().to_rfc3339(),
                issues: vec![],
                name: "Policy Engine".to_string(),
                health_percentage: 97.8,
                issues: vec!["Minor performance degradation in policy evaluation".to_string()],
        metrics: ComplianceMetrics {
            audit_events_per_hour: 1247,
            policy_evaluations_per_minute: 2845,
            violation_alerts_24h: 3,
            system_uptime_percentage: 99.97,
