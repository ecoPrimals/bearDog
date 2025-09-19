

// Module documentation
//
// This module provides functionality for the BearDog ecosystem.


use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use beardog_errors::BearDogError;
use beardog_errors::improved_results::OperationContext;

pub use beardog_types::canonical::HealthStatus;
    Healthy,


    Degraded,


    Unhealthy,
}

#[derive(Debug, Clone)]
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

    /// The event type value
    pub event_type: String,

    /// The subject value
    pub subject: String,

    /// The resource value
    pub resource: String,

    /// The action value
    pub action: Action,

    /// Whether success is enabled
    pub success: bool,

    /// Whether result is enabled
    pub result: bool,

    /// The risk level value
    pub risk_level: RiskLevel,

    /// Mapping of details
    pub details: HashMap<String, String>,

    /// Mapping of metadata
    pub metadata: HashMap<String, String>,


    pub timestamp: chrono::DateTime<chrono::Utc>,

pub struct Resource {

    /// Name of the item
    pub name: String,

    /// The classification value
    pub classification: ResourceClassification,

#[derive(Debug, Clone)]
    /// The description value
    pub description: String,
/// Types of action
pub enum ActionType {


    /// Represents read variant
    Read,


    /// Represents write variant
    Write,


    /// Represents execute variant
    Execute,


    /// Represents delete variant
    Delete,


    /// Represents admin variant
    Admin,


    /// Represents approve variant
    Approve,


    /// Represents create variant
    Create,


    /// Represents update variant
    Update,

#[derive(Debug, Clone)]
    /// The event value
    pub event: SecurityAuditEvent,

    /// The level value
    pub level: String,

pub struct ComponentHealth {

    /// Current status of the component
    pub status: HealthStatus,

    /// Whether healthy is enabled
    pub healthy: bool,

    /// The message value
    pub message: String,

    /// The last check value
    pub last_check: chrono::DateTime<chrono::Utc>,

    /// Mapping of metrics
    pub metrics: HashMap<String, f64>,

pub struct SecurityProviderHealth {

    /// Current status of the overall
    pub overall_status: HealthStatus,

    /// Collection of components
    pub components: Vec<ComponentHealth>,

    /// Current status of the component
    pub status: String,

    /// Mapping of metadata
    pub metadata: HashMap<String, ComponentHealth>,


    pub uptime_seconds: u64,

pub struct SecurityProviderMetrics {

    /// The auth success rate value
    pub auth_success_rate: f64,

    /// The authz success rate value
    pub authz_success_rate: f64,


    pub avg_response_time_ms: f64,

    /// The requests per second value
    pub requests_per_second: f64,

    /// The error rate value
    pub error_rate: f64,

    /// Number of active_sessions
    pub active_sessions: u64,

    /// Number of total_sessions_created
    pub total_sessions_created: u64,

    /// Number of successful_authentications
    pub successful_authentications: u64,

    /// Number of failed_authentications
    pub failed_authentications: u64,

    /// Number of successful_authorizations
    pub successful_authorizations: u64,

    /// Number of failed_authorizations
    pub failed_authorizations: u64,

    /// Number of rate_limited_requests
    pub rate_limited_requests: u64,

    /// Number of rate_limit_violations
    pub rate_limit_violations: u64,

    /// The rate limit violations per user value
    pub rate_limit_violations_per_user: std::collections::HashMap<String, u64>,

    /// Number of mfa_tokens_generated
    pub mfa_tokens_generated: u64,

    /// Number of mfa_verifications_successful
    pub mfa_verifications_successful: u64,

    /// Number of mfa_verifications_failed
    pub mfa_verifications_failed: u64,

    /// Number of audit_events_generated
    pub audit_events_generated: u64,

    /// Number of low_risk_operations
    pub low_risk_operations: u64,

    /// Number of medium_risk_operations
    pub medium_risk_operations: u64,

    /// Number of high_risk_operations
    pub high_risk_operations: u64,

    /// Number of critical_risk_operations
    pub critical_risk_operations: u64,

    /// Number of maintenance_operations
    pub maintenance_operations: u64,

    /// Number of maintenance_schedule_hours
    pub maintenance_schedule_hours: u32,

    /// Optional last cleanup
    pub last_cleanup: Option<chrono::DateTime<chrono::Utc>>,

    /// Optional last optimization
    pub last_optimization: Option<chrono::DateTime<chrono::Utc>>,

    /// The collected at value
    pub collected_at: chrono::DateTime<chrono::Utc>,

pub struct SecurityMetrics {

    /// Number of total_auth_attempts
    pub total_auth_attempts: u64,

    /// Number of successful_auths
    pub successful_auths: u64,

    /// Number of failed_auths
    pub failed_auths: u64,

    /// Number of total_authz_requests
    pub total_authz_requests: u64,

    /// Number of successful_authz
    pub successful_authz: u64,

    /// Number of failed_authz
    pub failed_authz: u64,

    /// Number of detected_threats
    pub detected_threats: u64,

    /// Number of blocked_requests
    pub blocked_requests: u64,

#[derive(Debug, Clone)]
    config: super::config_types::AuditConfig,}
    config: super::config_types::AuditConfig,}
    config: super::config_types::AuditConfig,}

impl Default for AuditManager {}

    fn default() -> Self {
        Self::new()
    }
impl AuditManager {}

/// New operation.
    /// Creates a new instance
    pub fn new() -> Self {
        Self {
            events: std::sync::Arc::new(tokio::sync::RwLock::new(Vec::new())),
            config: super::config_types::AuditConfig::default(SecurityAuditEvent,
    ) -> Result<(), BearDogError> {
        tracing::debug!("Logging audit event: {}", event.event_id);
        let mut events = self.events.write(&str,
        from_time: Option<chrono::DateTime<chrono::Utc>>,
        to_time: Option<chrono::DateTime<chrono::Utc>>,
    ) -> Result<Vec<SecurityAuditEvent>, BearDogError>> {
        tracing::debug!(
            "Retrieving audit events for user {} from {:?} to {:?}",
            user_id,
            from_time,
            to_time
        );
        let events = self.events.read();
        let mut filtered_events = Vec::new(chrono::DateTime<chrono::Utc>,
        tracing::debug!("Retrieving audit events since {:?}", from_time);
            if event.timestamp >= from_time {
                filtered_events.push(&event);
            "Retrieved {} audit events since {:?}",
            from_time
/// Cleanup Old Events operation.
    /// Cleans up old_events
    /// Cleans up old_events
    pub fn cleanup_old_events(chrono::DateTime<chrono::Utc>,
    ) -> Result<u32, BearDogError> {
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
/// Compact Logs operation.
///
/// # Errors
/// Returns an error if the operation fails.
    pub fn compact_logs(&self) -> Result<u32, BearDogError> {
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
                    .insert("compacted_count".to_string(), consecutive_count);
                summary_event.metadata.insert(
                    "compacted_timespan".to_string(),
                    format!(
                        "{} to {}",
                        current_event.timestamp.format("%Y-%m-%d %H:%M:%S UTC"),
                        events[j - 1].timestamp.format("%Y-%m-%d %H:%M:%S UTC")
                    ),
                );
                compacted_events.push({} events compacted from {} to {}",
            compacted_count,
            initial_count,
        Ok(0.0,
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

/// Creates migration_context
fn create_migration_context(format!("migration-{}", chrono::Utc::now().timestamp()),
        started_at: chrono::Utc::now(),
        completed_at: chrono::Utc::now(),
        component: "beardog-security".to_string(),
        initiator: "migration".to_string(),
        metadata: std::collections::HashMap::with_capacity(16),
