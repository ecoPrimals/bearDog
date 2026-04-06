// SPDX-License-Identifier: AGPL-3.0-or-later

//! # Audit Types
//!
//! This module provides types for security auditing and event logging.

use beardog_errors::improved_results::OperationContext;
use beardog_errors::BearDogError;
use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use std::sync::Arc;
use tokio::sync::RwLock;

// Re-export HealthStatus
pub use beardog_types::canonical::HealthStatus;

// ============================================================
// Risk Level
// ============================================================

/// Risk level for operations
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize)]
pub enum RiskLevel {
    /// Low risk
    Low,

    /// Medium risk
    Medium,

    /// High risk
    High,

    /// Critical risk
    Critical,
}

impl Default for RiskLevel {
    fn default() -> Self {
        Self::Low
    }
}

impl std::fmt::Display for RiskLevel {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            RiskLevel::Low => write!(f, "Low"),
            RiskLevel::Medium => write!(f, "Medium"),
            RiskLevel::High => write!(f, "High"),
            RiskLevel::Critical => write!(f, "Critical"),
        }
    }
}

// ============================================================
// Action Types
// ============================================================

/// Action type for audit events
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub enum ActionType {
    /// Read operation
    Read,

    /// Write operation
    Write,

    /// Execute operation
    Execute,

    /// Delete operation
    Delete,

    /// Admin operation
    Admin,

    /// Approve operation
    Approve,

    /// Create operation
    Create,

    /// Update operation
    Update,
}

impl Default for ActionType {
    fn default() -> Self {
        Self::Read
    }
}

impl std::fmt::Display for ActionType {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            ActionType::Read => write!(f, "Read"),
            ActionType::Write => write!(f, "Write"),
            ActionType::Execute => write!(f, "Execute"),
            ActionType::Delete => write!(f, "Delete"),
            ActionType::Admin => write!(f, "Admin"),
            ActionType::Approve => write!(f, "Approve"),
            ActionType::Create => write!(f, "Create"),
            ActionType::Update => write!(f, "Update"),
        }
    }
}

/// Action with type and target
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Action {
    /// Type of action
    pub action_type: ActionType,

    /// Target of the action
    pub target: String,

    /// Description of the action
    pub description: String,
}

impl Default for Action {
    fn default() -> Self {
        Self {
            action_type: ActionType::Read,
            target: String::new(),
            description: String::new(),
        }
    }
}

// ============================================================
// Resource Classification
// ============================================================

/// Resource classification
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum ResourceClassification {
    /// Public resource
    Public,

    /// Internal resource
    Internal,

    /// Confidential resource
    Confidential,

    /// Secret resource
    Secret,

    /// Top secret resource
    TopSecret,
}

impl Default for ResourceClassification {
    fn default() -> Self {
        Self::Internal
    }
}

/// Resource being accessed
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct Resource {
    /// Resource identifier
    pub id: String,

    /// Resource name
    pub name: String,

    /// Resource classification
    pub classification: ResourceClassification,
}

// ============================================================
// Audit Events
// ============================================================

/// Detailed audit event types
#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum AuditEventDetail {
    /// Authentication event
    Authentication {
        success: bool,
        timestamp: DateTime<Utc>,
        failure_reason: Option<String>,
        ip_address: Option<String>,
        user_agent: Option<String>,
    },

    /// Authorization event
    Authorization {
        resource: String,
        action: String,
        granted: bool,
        risk_level: String,
    },

    /// Session created
    SessionCreated {
        session_id: String,
        created_at: DateTime<Utc>,
        expires_at: DateTime<Utc>,
    },

    /// Session revoked
    SessionRevoked {
        session_id: String,
        revoked_at: DateTime<Utc>,
    },

    /// Account locked
    AccountLocked {
        user_id: String,
        reason: String,
        locked_at: DateTime<Utc>,
        unlock_time: Option<DateTime<Utc>>,
    },

    /// Account unlocked
    AccountUnlocked {
        user_id: String,
        admin_id: String,
        unlocked_at: DateTime<Utc>,
    },

    /// MFA token generated
    MfaTokenGenerated {
        user_id: String,
        method: String,
        generated_at: DateTime<Utc>,
    },

    /// MFA token verified
    MfaTokenVerified {
        user_id: String,
        method: String,
        verified_at: DateTime<Utc>,
    },
}

/// Security audit event
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SecurityAuditEvent {
    /// Unique event ID
    pub id: String,

    /// Event ID (for correlation)
    pub event_id: String,

    /// Event type description
    pub event_type: String,

    /// Subject (user/entity)
    pub subject: String,

    /// Resource being accessed
    pub resource: String,

    /// Action performed
    pub action: Action,

    /// Whether the action succeeded
    pub success: bool,

    /// Result of the action
    pub result: bool,

    /// Risk level
    pub risk_level: RiskLevel,

    /// Additional details
    pub details: HashMap<String, String>,

    /// Metadata
    pub metadata: HashMap<String, String>,

    /// Event timestamp
    pub timestamp: DateTime<Utc>,
}

impl Default for SecurityAuditEvent {
    fn default() -> Self {
        Self {
            id: uuid::Uuid::new_v4().to_string(),
            event_id: uuid::Uuid::new_v4().to_string(),
            event_type: String::new(),
            subject: String::new(),
            resource: String::new(),
            action: Action::default(),
            success: false,
            result: false,
            risk_level: RiskLevel::Low,
            details: HashMap::new(),
            metadata: HashMap::new(),
            timestamp: Utc::now(),
        }
    }
}

/// Audit log entry
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct AuditLogEntry {
    /// The audit event
    pub event: SecurityAuditEvent,

    /// Log level
    pub level: String,
}

// ============================================================
// Health Types
// ============================================================

/// Component health status
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct ComponentHealth {
    /// Health status
    pub status: HealthStatus,

    /// Whether the component is healthy
    pub healthy: bool,

    /// Status message
    pub message: String,

    /// Last health check time
    pub last_check: DateTime<Utc>,

    /// Health metrics
    pub metrics: HashMap<String, f64>,
}

/// Security provider health
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct SecurityProviderHealth {
    /// Overall health status
    pub overall_status: HealthStatus,

    /// Component health statuses
    pub components: Vec<ComponentHealth>,

    /// Status description
    pub status: String,

    /// Component metadata
    pub metadata: HashMap<String, ComponentHealth>,

    /// Uptime in seconds
    pub uptime_seconds: u64,
}

// ============================================================
// Metrics Types
// ============================================================

/// Security provider metrics
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct SecurityProviderMetrics {
    /// Authentication success rate
    pub auth_success_rate: f64,

    /// Authorization success rate
    pub authz_success_rate: f64,

    /// Average response time in milliseconds
    pub avg_response_time_ms: f64,

    /// Requests per second
    pub requests_per_second: f64,

    /// Error rate
    pub error_rate: f64,

    /// Active sessions count
    pub active_sessions: u64,

    /// Total sessions created
    pub total_sessions_created: u64,

    /// Successful authentications
    pub successful_authentications: u64,

    /// Failed authentications
    pub failed_authentications: u64,

    /// Successful authorizations
    pub successful_authorizations: u64,

    /// Failed authorizations
    pub failed_authorizations: u64,

    /// Rate limited requests
    pub rate_limited_requests: u64,

    /// Rate limit violations
    pub rate_limit_violations: u64,

    /// Rate limit violations per user
    pub rate_limit_violations_per_user: HashMap<String, u64>,

    /// MFA tokens generated
    pub mfa_tokens_generated: u64,

    /// Successful MFA verifications
    pub mfa_verifications_successful: u64,

    /// Failed MFA verifications
    pub mfa_verifications_failed: u64,

    /// Audit events generated
    pub audit_events_generated: u64,

    /// Low risk operations
    pub low_risk_operations: u64,

    /// Medium risk operations
    pub medium_risk_operations: u64,

    /// High risk operations
    pub high_risk_operations: u64,

    /// Critical risk operations
    pub critical_risk_operations: u64,

    /// Maintenance operations
    pub maintenance_operations: u64,

    /// Maintenance schedule in hours
    pub maintenance_schedule_hours: u32,

    /// Last cleanup time
    pub last_cleanup: Option<DateTime<Utc>>,

    /// Last optimization time
    pub last_optimization: Option<DateTime<Utc>>,

    /// Uptime in seconds
    pub uptime_seconds: u64,

    /// When metrics were collected
    pub collected_at: DateTime<Utc>,
}

/// Security metrics summary
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct SecurityMetrics {
    /// Total authentication attempts
    pub total_auth_attempts: u64,

    /// Successful authentications
    pub successful_auths: u64,

    /// Failed authentications
    pub failed_auths: u64,

    /// Total authorization requests
    pub total_authz_requests: u64,

    /// Successful authorizations
    pub successful_authz: u64,

    /// Failed authorizations
    pub failed_authz: u64,

    /// Detected threats
    pub detected_threats: u64,

    /// Blocked requests
    pub blocked_requests: u64,
}

// ============================================================
// Audit Manager
// ============================================================

/// Audit manager for event logging
#[derive(Debug)]
pub struct AuditManager {
    /// Stored events
    events: Arc<RwLock<Vec<SecurityAuditEvent>>>,

    /// Configuration
    config: super::config_types::AuditConfig,
}

impl Default for AuditManager {
    fn default() -> Self {
        Self::new()
    }
}

impl AuditManager {
    /// Create a new audit manager
    pub fn new() -> Self {
        Self {
            events: Arc::new(RwLock::new(Vec::new())),
            config: super::config_types::AuditConfig::default(),
        }
    }

    /// Log an audit event
    pub async fn log_event(&self, event: SecurityAuditEvent) -> Result<(), BearDogError> {
        tracing::debug!("Logging audit event: {}", event.event_id);
        let mut events = self.events.write().await;
        events.push(event);
        Ok(())
    }

    /// Get events for a user
    pub async fn get_events_for_user(
        &self,
        user_id: &str,
        from_time: Option<DateTime<Utc>>,
        to_time: Option<DateTime<Utc>>,
    ) -> Result<Vec<SecurityAuditEvent>, BearDogError> {
        tracing::debug!(
            "Retrieving audit events for user {} from {:?} to {:?}",
            user_id,
            from_time,
            to_time
        );

        let events = self.events.read().await;
        let filtered_events: Vec<SecurityAuditEvent> = events
            .iter()
            .filter(|e| {
                e.subject == user_id
                    && from_time.map_or(true, |ft| e.timestamp >= ft)
                    && to_time.map_or(true, |tt| e.timestamp <= tt)
            })
            .cloned()
            .collect();

        Ok(filtered_events)
    }

    /// Get events since a time
    pub async fn get_events_since(
        &self,
        from_time: DateTime<Utc>,
    ) -> Result<Vec<SecurityAuditEvent>, BearDogError> {
        tracing::debug!("Retrieving audit events since {:?}", from_time);

        let events = self.events.read().await;
        let filtered_events: Vec<SecurityAuditEvent> = events
            .iter()
            .filter(|e| e.timestamp >= from_time)
            .cloned()
            .collect();

        tracing::debug!(
            "Retrieved {} audit events since {:?}",
            filtered_events.len(),
            from_time
        );

        Ok(filtered_events)
    }

    /// Clean up old events
    pub async fn cleanup_old_events(
        &self,
        cutoff: DateTime<Utc>,
    ) -> Result<u32, BearDogError> {
        tracing::info!("Cleaning up audit events before {:?}", cutoff);

        let mut events = self.events.write().await;
        let initial_count = events.len();

        events.retain(|event| {
            if event.timestamp >= cutoff {
                return true; // Keep recent events
            }

            // Keep critical events regardless of age
            match event.action.action_type {
                ActionType::Admin | ActionType::Execute => {
                    event.risk_level == RiskLevel::Critical || event.risk_level == RiskLevel::High
                }
                ActionType::Update | ActionType::Delete => true,
                _ => false,
            }
        });

        let removed_count = initial_count - events.len();
        tracing::info!(
            "Cleaned up {} audit events (kept {} events)",
            removed_count,
            events.len()
        );

        Ok(removed_count as u32)
    }

    /// Compact audit logs
    pub async fn compact_logs(&self) -> Result<u32, BearDogError> {
        tracing::info!("Starting audit log compaction");

        let mut events = self.events.write().await;
        let initial_count = events.len();

        if initial_count == 0 {
            return Ok(0);
        }

        // Sort events for compaction
        events.sort_by(|a, b| {
            a.subject
                .cmp(&b.subject)
                .then_with(|| a.timestamp.cmp(&b.timestamp))
        });

        let mut compacted_events = Vec::new();
        let mut i = 0;

        while i < events.len() {
            let current_event = &events[i];

            // Count consecutive similar events
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
            }

            if consecutive_count > 3 {
                // Compact these events into a summary
                let mut summary_event = current_event.clone();
                summary_event.metadata.insert(
                    "compacted_count".to_string(),
                    consecutive_count.to_string(),
                );
                summary_event.metadata.insert(
                    "compacted_timespan".to_string(),
                    format!(
                        "{} to {}",
                        current_event.timestamp.format("%Y-%m-%d %H:%M:%S UTC"),
                        events[j - 1].timestamp.format("%Y-%m-%d %H:%M:%S UTC")
                    ),
                );
                compacted_events.push(summary_event);
                i = j;
            } else {
                compacted_events.push(current_event.clone());
                i += 1;
            }
        }

        let compacted_count = initial_count - compacted_events.len();
        *events = compacted_events;

        tracing::info!(
            "{} events compacted from {} to {}",
            compacted_count,
            initial_count,
            events.len()
        );

        Ok(compacted_count as u32)
    }

    /// Get current metrics
    pub async fn get_metrics(&self) -> SecurityProviderMetrics {
        let events = self.events.read().await;

        let mut metrics = SecurityProviderMetrics {
            audit_events_generated: events.len() as u64,
            collected_at: Utc::now(),
            ..Default::default()
        };

        // Count by risk level
        for event in events.iter() {
            match event.risk_level {
                RiskLevel::Low => metrics.low_risk_operations += 1,
                RiskLevel::Medium => metrics.medium_risk_operations += 1,
                RiskLevel::High => metrics.high_risk_operations += 1,
                RiskLevel::Critical => metrics.critical_risk_operations += 1,
            }
        }

        metrics
    }
}

/// Create a migration context
pub fn create_migration_context() -> OperationContext {
    OperationContext {
        operation_id: format!("migration-{}", Utc::now().timestamp()),
        started_at: Utc::now(),
        completed_at: Utc::now(),
        component: "beardog-security".to_string(),
        initiator: "migration".to_string(),
        metadata: HashMap::with_capacity(16),
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_risk_level_ordering() {
        assert!(RiskLevel::Critical > RiskLevel::High);
        assert!(RiskLevel::High > RiskLevel::Medium);
        assert!(RiskLevel::Medium > RiskLevel::Low);
    }

    #[test]
    fn test_action_type_display() {
        assert_eq!(format!("{}", ActionType::Read), "Read");
        assert_eq!(format!("{}", ActionType::Admin), "Admin");
    }

    #[test]
    fn test_risk_level_display() {
        assert_eq!(format!("{}", RiskLevel::Critical), "Critical");
    }

    #[tokio::test]
    async fn test_audit_manager() -> Result<(), BearDogError> {
        let manager = AuditManager::new();

        let event = SecurityAuditEvent {
            subject: "test-user".to_string(),
            ..Default::default()
        };

        manager.log_event(event).await?;

        let events = manager.get_events_for_user("test-user", None, None).await?;
        assert_eq!(events.len(), 1);

        Ok(())
    }

    #[test]
    fn test_security_metrics_default() {
        let metrics = SecurityMetrics::default();
        assert_eq!(metrics.total_auth_attempts, 0);
        assert_eq!(metrics.detected_threats, 0);
    }
}
