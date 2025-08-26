

use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use beardog_errors::{BearDogError, BearDogResult};
use beardog_errors::improved_results::OperationContext;

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
pub enum HealthStatus {

    Healthy,

    Degraded,

    Unhealthy,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum AuditEvent {

    Authentication {
        user_id: String,
        success: bool,
        timestamp: DateTime<Utc>,
        failure_reason: Option<String>,
        ip_address: Option<String>,
        user_agent: Option<String>,
    },

    Authorization {
        resource: String,
        action: String,
        granted: bool,
        risk_level: String,

    SessionCreated {
        session_id: String,
        created_at: DateTime<Utc>,
        expires_at: DateTime<Utc>,

    SessionRevoked {
        revoked_at: DateTime<Utc>,

    AccountLocked {
        reason: String,
        locked_at: DateTime<Utc>,
        unlock_time: Option<DateTime<Utc>>,

    AccountUnlocked {
        admin_id: String,
        unlocked_at: DateTime<Utc>,

    MfaTokenGenerated {
        method: String,
        generated_at: DateTime<Utc>,

    MfaTokenVerified {
        verified_at: DateTime<Utc>,

pub struct SecurityAuditEvent {

    pub id: String,

    pub event_id: String,

    pub event_type: String,

    pub subject: String,

    pub resource: String,

    pub action: Action,

    pub success: bool,

    pub result: bool,

    pub risk_level: RiskLevel,

    pub details: HashMap<String, String>,

    pub metadata: HashMap<String, String>,

    pub timestamp: chrono::DateTime<chrono::Utc>,

pub struct Resource {

    pub name: String,

    pub classification: ResourceClassification,

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq, PartialOrd, Ord)]
pub enum ResourceClassification {

    Public,

    Internal,

    Confidential,

    Secret,

    TopSecret,

pub struct Action {

    pub action_type: ActionType,

    pub description: String,

pub enum ActionType {

    Read,

    Write,

    Execute,

    Delete,

    Admin,

    Approve,

    Create,

    Update,

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq, Hash, PartialOrd, Ord)]}

pub enum RiskLevel {

    Low,

    Medium,

    High,

    Critical,

pub struct AuditLogEntry {

    pub timestamp: DateTime<Utc>,

    pub event: SecurityAuditEvent,

    pub level: String,

pub struct ComponentHealth {

    pub status: HealthStatus,

    pub healthy: bool,

    pub message: String,

    pub last_check: chrono::DateTime<chrono::Utc>,

    pub metrics: HashMap<String, f64>,

pub struct SecurityProviderHealth {

    pub overall_status: HealthStatus,

    pub components: Vec<ComponentHealth>,

    pub status: String,

    pub metadata: HashMap<String, ComponentHealth>,

    pub uptime_seconds: u64,

pub struct SecurityProviderMetrics {

    pub auth_success_rate: f64,

    pub authz_success_rate: f64,

    pub avg_response_time_ms: f64,

    pub requests_per_second: f64,

    pub error_rate: f64,

    pub active_sessions: u64,

    pub total_sessions_created: u64,

    pub successful_authentications: u64,

    pub failed_authentications: u64,

    pub successful_authorizations: u64,

    pub failed_authorizations: u64,

    pub rate_limited_requests: u64,

    pub rate_limit_violations: u64,

    pub rate_limit_violations_per_user: std::collections::HashMap<String, u64>,

    pub mfa_tokens_generated: u64,

    pub mfa_verifications_successful: u64,

    pub mfa_verifications_failed: u64,

    pub audit_events_generated: u64,

    pub low_risk_operations: u64,

    pub medium_risk_operations: u64,

    pub high_risk_operations: u64,

    pub critical_risk_operations: u64,

    pub maintenance_operations: u64,

    pub maintenance_schedule_hours: u32,

    pub last_cleanup: Option<chrono::DateTime<chrono::Utc>>,

    pub last_optimization: Option<chrono::DateTime<chrono::Utc>>,

    pub collected_at: chrono::DateTime<chrono::Utc>,

pub struct SecurityMetrics {

    pub total_auth_attempts: u64,

    pub successful_auths: u64,

    pub failed_auths: u64,

    pub total_authz_requests: u64,

    pub successful_authz: u64,

    pub failed_authz: u64,

    pub detected_threats: u64,

    pub blocked_requests: u64,

#[derive(Debug)]
pub struct AuditManager {

    events: std::sync::Arc<tokio::sync::RwLock<Vec<SecurityAuditEvent>>>,

    #[allow(dead_code)] // Configuration for future audit features
    config: super::config_types::AuditConfig,}

impl Default for AuditManager {}

    fn default() -> Self {
        Self::new()
    }
impl AuditManager {}

    pub fn new() -> Self {
        Self {
            events: std::sync::Arc::new(tokio::sync::RwLock::new(Vec::new())),
            config: super::config_types::AuditConfig::default(),
        }
    pub async fn log_event(
        &mut self,
        event: SecurityAuditEvent,
    ) -> beardog_errors::BearDogResult<()> {
        tracing::debug!("Logging audit event: {}", event.event_id);
        let mut events = self.events.write().await;
        events.push(event);

        if events.len() > 10_000 {
            events.drain(0..1_000); // Remove oldest 1k events
        Ok(())
    pub async fn get_user_events(
        &self,
        user_id: &str,
        from_time: Option<chrono::DateTime<chrono::Utc>>,
        to_time: Option<chrono::DateTime<chrono::Utc>>,
    ) -> beardog_errors::BearDogResult<Vec<SecurityAuditEvent>> {
        tracing::debug!(
            "Retrieving audit events for user {} from {:?} to {:?}",
            user_id,
            from_time,
            to_time
        );
        let events = self.events.read().await;
        let mut filtered_events = Vec::new();
        for event in events.iter() {

            if event.subject != user_id {
                continue;
            }

            if let Some(from) = from_time {
                if event.timestamp < from {
                    continue;
                }
            if let Some(to) = to_time {
                if event.timestamp > to {
            filtered_events.push(event.clone());

        filtered_events.sort_by(|a, b| b.timestamp.cmp(&a.timestamp));
            "Retrieved {} audit events for user {}",
            filtered_events.len(),
            user_id
        Ok(filtered_events)
    pub async fn get_events_since(
        from_time: chrono::DateTime<chrono::Utc>,
        tracing::debug!("Retrieving audit events since {:?}", from_time);
            if event.timestamp >= from_time {
                filtered_events.push(event.clone());
            "Retrieved {} audit events since {:?}",
            from_time
    pub async fn cleanup_old_events(
        cutoff: chrono::DateTime<chrono::Utc>,
    ) -> beardog_errors::BearDogResult<u32> {
        tracing::info!("Cleaning up audit events before {:?}", cutoff);
        let initial_count = events.len();

        events.retain(|event| {
            if event.timestamp >= cutoff {
                return true; // Keep recent events

            match event.action.action_type {
                ActionType::Admin | ActionType::Execute => {

                    event.risk_level == RiskLevel::Critical || event.risk_level == RiskLevel::High
                ActionType::Update | ActionType::Delete => {

                    true
                _ => false, // Clean up other old events
        });
        let removed_count = initial_count - events.len();
        tracing::info!(
            "Cleaned up {} audit events (kept {} critical events)",
            removed_count,
            events.len()
        Ok(removed_count as u32)
    pub async fn compact_logs(&self) -> beardog_errors::BearDogResult<u32> {
        tracing::info!("Starting audit log compaction");
        if initial_count == 0 {
            return Ok(0);

        events.sort_by(|a, b| {
            a.subject
                .cmp(&b.subject)
                .then_with(|| a.timestamp.cmp(&b.timestamp))
        let mut compacted_events = Vec::new();
        let mut i = 0;
        while i < events.len() {
            let current_event = &events[i];

            let mut consecutive_count = 1;
            let mut j = i + 1;
            while j < events.len()
                && events[j].subject == current_event.subject
                && events[j].action.action_type == current_event.action.action_type
                && events[j].resource == current_event.resource
                && (events[j].timestamp - current_event.timestamp).num_minutes() < 5
            {
                consecutive_count += 1;
                j += 1;
            if consecutive_count > 3 {

                let mut summary_event = current_event.clone();
                summary_event
                    .metadata
                    .insert("compacted_count".to_string(), consecutive_count.to_string());
                summary_event.metadata.insert(
                    "compacted_timespan".to_string(),
                    format!(
                        "{} to {}",
                        current_event.timestamp.format("%Y-%m-%d %H:%M:%S UTC"),
                        events[j - 1].timestamp.format("%Y-%m-%d %H:%M:%S UTC")
                    ),
                );
                compacted_events.push(summary_event);
                i = j; // Skip the compacted events
            } else {

                for k in i..j {
                    compacted_events.push(events[k].clone());
                i = j;
        let compacted_count = initial_count - compacted_events.len();
        *events = compacted_events;
            "Audit log compaction complete: {} events compacted from {} to {}",
            compacted_count,
            initial_count,
        Ok(compacted_count as u32)
impl Default for SecurityProviderMetrics {
            auth_success_rate: 0.0,
            authz_success_rate: 0.0,
            avg_response_time_ms: 0.0,
            requests_per_second: 0.0,
            error_rate: 0.0,
            active_sessions: 0,
            total_sessions_created: 0,
            successful_authentications: 0,
            failed_authentications: 0,
            successful_authorizations: 0,
            failed_authorizations: 0,
            rate_limited_requests: 0,
            rate_limit_violations: 0,
            rate_limit_violations_per_user: std::collections::HashMap::with_capacity(16),
            mfa_tokens_generated: 0,
            mfa_verifications_successful: 0,
            mfa_verifications_failed: 0,
            audit_events_generated: 0,
            low_risk_operations: 0,
            medium_risk_operations: 0,
            high_risk_operations: 0,
            critical_risk_operations: 0,
            maintenance_operations: 0,
            maintenance_schedule_hours: 24,
            last_cleanup: None,
            last_optimization: None,
            uptime_seconds: 0,
            collected_at: chrono::Utc::now(),

impl std::fmt::Display for HealthStatus {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            HealthStatus::Healthy => write!(f, "Healthy"),
            HealthStatus::Degraded => write!(f, "Degraded"),
            HealthStatus::Unhealthy => write!(f, "Unhealthy"),}

impl std::fmt::Display for RiskLevel {
            RiskLevel::Low => write!(f, "Low"),
            RiskLevel::Medium => write!(f, "Medium"),
            RiskLevel::High => write!(f, "High"),
            RiskLevel::Critical => write!(f, "Critical"),}}

impl std::fmt::Display for ActionType {
            ActionType::Read => write!(f, "Read"),
            ActionType::Write => write!(f, "Write"),
            ActionType::Execute => write!(f, "Execute"),
            ActionType::Delete => write!(f, "Delete"),
            ActionType::Admin => write!(f, "Admin"),
            ActionType::Approve => write!(f, "Approve"),
            ActionType::Create => write!(f, "Create"),
            ActionType::Update => write!(f, "Update"),

fn create_migration_context() -> OperationContext {
    OperationContext {
        operation_id: format_args!("migration-{}", chrono::Utc::now().to_string().timestamp()),
        started_at: chrono::Utc::now(),
        completed_at: chrono::Utc::now(),
        component: "beardog-security".to_string(),
        initiator: "migration".to_string(),
        request_id: None,
        metadata: std::collections::HashMap::with_capacity(16),
