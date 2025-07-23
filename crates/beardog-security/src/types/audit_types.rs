//! Security Audit Types
//!
//! This module contains all types related to security auditing, compliance,
//! monitoring, and security event logging.

use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};
use std::collections::HashMap;

/// Health status enumeration
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
pub enum HealthStatus {
    /// All systems operational
    Healthy,
    /// Some issues detected but operational
    Degraded,
    /// Critical issues detected
    Unhealthy,
}

/// Security audit event types used by the handlers
#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum AuditEvent {
    /// User authentication event
    Authentication {
        user_id: String,
        success: bool,
        timestamp: DateTime<Utc>,
        failure_reason: Option<String>,
        ip_address: Option<String>,
        user_agent: Option<String>,
    },
    /// Authorization decision event
    Authorization {
        user_id: String,
        resource: String,
        action: String,
        granted: bool,
        risk_level: String,
        timestamp: DateTime<Utc>,
    },
    /// Session created event
    SessionCreated {
        session_id: String,
        user_id: String,
        created_at: DateTime<Utc>,
        expires_at: DateTime<Utc>,
    },
    /// Session revoked event
    SessionRevoked {
        session_id: String,
        user_id: String,
        revoked_at: DateTime<Utc>,
    },
    /// Account locked event
    AccountLocked {
        user_id: String,
        reason: String,
        locked_at: DateTime<Utc>,
        unlock_time: Option<DateTime<Utc>>,
    },
    /// Account unlocked event
    AccountUnlocked {
        user_id: String,
        admin_id: String,
        unlocked_at: DateTime<Utc>,
    },
    /// MFA token generated event
    MfaTokenGenerated {
        user_id: String,
        method: String,
        generated_at: DateTime<Utc>,
        expires_at: DateTime<Utc>,
    },
    /// MFA token verified event
    MfaTokenVerified {
        user_id: String,
        success: bool,
        verified_at: DateTime<Utc>,
    },
}

/// Security audit event for tracking security-related activities
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SecurityAuditEvent {
    /// Unique identifier for this audit event
    pub id: String,
    /// Unique event identifier (for compatibility)
    pub event_id: String,
    /// Type of event
    pub event_type: String,
    /// Subject performing the action
    pub subject: String,
    /// Resource being accessed
    pub resource: String,
    /// Action being performed
    pub action: Action,
    /// Whether the operation was successful
    pub success: bool,
    /// Result of the operation (for compatibility)
    pub result: bool,
    /// Risk level assessed for this event
    pub risk_level: RiskLevel,
    /// Additional event details
    pub details: HashMap<String, String>,
    /// Event metadata
    pub metadata: HashMap<String, String>,
    /// Timestamp when the event occurred
    pub timestamp: chrono::DateTime<chrono::Utc>,
}

/// Security resource being accessed
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
pub struct Resource {
    /// Unique identifier for the resource
    pub id: String,
    /// Resource name or description
    pub name: String,
    /// Classification level of the resource
    pub classification: ResourceClassification,
    /// Additional resource metadata
    pub metadata: HashMap<String, String>,
}

/// Resource classification levels for access control
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq, PartialOrd, Ord)]
pub enum ResourceClassification {
    /// Public information
    Public,
    /// Internal company information
    Internal,
    /// Confidential information
    Confidential,
    /// Secret information
    Secret,
    /// Top secret information
    TopSecret,
}

/// Security action being performed
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Action {
    /// Type of action being performed
    pub action_type: ActionType,
    /// Detailed description of the action
    pub description: String,
    /// Risk level of this action
    pub risk_level: RiskLevel,
    /// Timestamp when the action occurred
    pub timestamp: chrono::DateTime<chrono::Utc>,
}

/// Types of actions that can be performed on resources
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
pub enum ActionType {
    /// Read operation
    Read,
    /// Write operation  
    Write,
    /// Execute operation
    Execute,
    /// Delete operation
    Delete,
    /// Administrative operation
    Admin,
    /// Approval operation
    Approve,
    /// Create operation
    Create,
    /// Update operation
    Update,
}

/// Risk level assessment for security actions
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq, Hash, PartialOrd, Ord)]
pub enum RiskLevel {
    /// Low risk operation
    Low,
    /// Medium risk operation
    Medium,
    /// High risk operation
    High,
    /// Critical risk operation requiring special approval
    Critical,
}

/// Audit log entry for persistent storage of security events
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct AuditLogEntry {
    /// Unique identifier for the log entry
    pub id: String,
    /// When the entry was created
    pub timestamp: DateTime<Utc>,
    /// The audit event being logged
    pub event: SecurityAuditEvent,
    /// Log level (info, warn, error, etc.)
    pub level: String,
    /// Additional context metadata
    pub metadata: HashMap<String, String>,
}

/// Component health information
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ComponentHealth {
    /// Component name
    pub name: String,
    /// Component status
    pub status: HealthStatus,
    /// Whether the component is healthy (for compatibility)
    pub healthy: bool,
    /// Status message or error description
    pub message: String,
    /// Timestamp of last health check
    pub last_check: chrono::DateTime<chrono::Utc>,
    /// Component-specific metrics
    pub metrics: HashMap<String, f64>,
}

/// Overall security provider health status
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SecurityProviderHealth {
    /// Overall system status
    pub overall_status: HealthStatus,
    /// Individual component health status
    pub components: Vec<ComponentHealth>,
    /// System status (for compatibility)
    pub status: String,
    /// Health metadata
    pub metadata: HashMap<String, ComponentHealth>,
    /// Last health check timestamp
    pub last_check: chrono::DateTime<chrono::Utc>,
    /// System uptime in seconds
    pub uptime_seconds: u64,
}

/// Security provider metrics for monitoring and analytics
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SecurityProviderMetrics {
    /// Authentication success rate (0.0 to 1.0)
    pub auth_success_rate: f64,
    /// Authorization success rate (0.0 to 1.0)
    pub authz_success_rate: f64,
    /// Average response time in milliseconds
    pub avg_response_time_ms: f64,
    /// Requests processed per second
    pub requests_per_second: f64,
    /// Error rate (0.0 to 1.0)
    pub error_rate: f64,
    /// Number of currently active sessions
    pub active_sessions: u64,
    /// Total number of sessions created
    pub total_sessions_created: u64,
    /// Number of successful authentications
    pub successful_authentications: u64,
    /// Number of failed authentications
    pub failed_authentications: u64,
    /// Number of successful authorizations
    pub successful_authorizations: u64,
    /// Number of failed authorizations
    pub failed_authorizations: u64,
    /// Number of rate-limited requests
    pub rate_limited_requests: u64,
    /// Number of rate limit violations
    pub rate_limit_violations: u64,
    /// Rate limit violations per user
    pub rate_limit_violations_per_user: std::collections::HashMap<String, u64>,
    /// Number of MFA tokens generated
    pub mfa_tokens_generated: u64,
    /// Number of successful MFA verifications
    pub mfa_verifications_successful: u64,
    /// Number of failed MFA verifications
    pub mfa_verifications_failed: u64,
    /// Number of audit events generated
    pub audit_events_generated: u64,
    /// Number of low-risk operations
    pub low_risk_operations: u64,
    /// Number of medium-risk operations
    pub medium_risk_operations: u64,
    /// Number of high-risk operations
    pub high_risk_operations: u64,
    /// Number of critical-risk operations
    pub critical_risk_operations: u64,
    /// Number of maintenance operations performed
    pub maintenance_operations: u64,
    /// Scheduled maintenance interval in hours
    pub maintenance_schedule_hours: u32,
    /// Last cleanup operation timestamp
    pub last_cleanup: Option<chrono::DateTime<chrono::Utc>>,
    /// Last optimization operation timestamp
    pub last_optimization: Option<chrono::DateTime<chrono::Utc>>,
    /// Provider uptime in seconds
    pub uptime_seconds: u64,
    /// When these metrics were collected
    pub collected_at: chrono::DateTime<chrono::Utc>,
}

/// Security metrics and statistics
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SecurityMetrics {
    /// Total number of authentication attempts
    pub total_auth_attempts: u64,
    /// Successful authentications
    pub successful_auths: u64,
    /// Failed authentication attempts
    pub failed_auths: u64,
    /// Total authorization requests
    pub total_authz_requests: u64,
    /// Successful authorizations
    pub successful_authz: u64,
    /// Failed authorizations
    pub failed_authz: u64,
    /// Active user sessions
    pub active_sessions: u64,
    /// Detected security threats
    pub detected_threats: u64,
    /// Blocked malicious requests
    pub blocked_requests: u64,
    /// Collection timestamp
    pub timestamp: DateTime<Utc>,
}

/// Audit manager for security events and compliance
#[derive(Debug)]
pub struct AuditManager {
    /// Audit events storage
    events: std::sync::Arc<tokio::sync::RwLock<Vec<SecurityAuditEvent>>>,
    /// Configuration
    #[allow(dead_code)] // Configuration for future audit features
    config: super::config_types::AuditConfig,
}

impl Default for AuditManager {
    fn default() -> Self {
        Self::new()
    }
}

impl AuditManager {
    pub fn new() -> Self {
        Self {
            events: std::sync::Arc::new(tokio::sync::RwLock::new(Vec::new())),
            config: super::config_types::AuditConfig::default(),
        }
    }

    pub async fn log_event(
        &mut self,
        event: SecurityAuditEvent,
    ) -> beardog_errors::BearDogResult<()> {
        tracing::debug!("Logging audit event: {}", event.event_id);

        let mut events = self.events.write().await;
        events.push(event);

        // Limit memory usage by keeping only recent events
        if events.len() > 10_000 {
            events.drain(0..1_000); // Remove oldest 1k events
        }

        Ok(())
    }

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
            // Check if event matches user_id
            if event.subject != user_id {
                continue;
            }

            // Apply time filters if specified
            if let Some(from) = from_time {
                if event.timestamp < from {
                    continue;
                }
            }

            if let Some(to) = to_time {
                if event.timestamp > to {
                    continue;
                }
            }

            filtered_events.push(event.clone());
        }

        // Sort by timestamp (most recent first)
        filtered_events.sort_by(|a, b| b.timestamp.cmp(&a.timestamp));

        tracing::debug!(
            "Retrieved {} audit events for user {}",
            filtered_events.len(),
            user_id
        );
        Ok(filtered_events)
    }

    pub async fn get_events_since(
        &self,
        from_time: chrono::DateTime<chrono::Utc>,
    ) -> beardog_errors::BearDogResult<Vec<SecurityAuditEvent>> {
        tracing::debug!("Retrieving audit events since {:?}", from_time);

        let events = self.events.read().await;
        let mut filtered_events = Vec::new();

        for event in events.iter() {
            if event.timestamp >= from_time {
                filtered_events.push(event.clone());
            }
        }

        // Sort by timestamp (most recent first)
        filtered_events.sort_by(|a, b| b.timestamp.cmp(&a.timestamp));

        tracing::debug!(
            "Retrieved {} audit events since {:?}",
            filtered_events.len(),
            from_time
        );
        Ok(filtered_events)
    }

    pub async fn cleanup_old_events(
        &self,
        cutoff: chrono::DateTime<chrono::Utc>,
    ) -> beardog_errors::BearDogResult<u32> {
        tracing::info!("Cleaning up audit events before {:?}", cutoff);

        let mut events = self.events.write().await;
        let initial_count = events.len();

        // Remove events older than cutoff, but preserve critical security events
        events.retain(|event| {
            if event.timestamp >= cutoff {
                return true; // Keep recent events
            }

            // Preserve critical security events even if old
            match event.action.action_type {
                ActionType::Admin | ActionType::Execute => {
                    // Keep admin/execute events for compliance
                    event.risk_level == RiskLevel::Critical || event.risk_level == RiskLevel::High
                }
                ActionType::Update | ActionType::Delete => {
                    // Always keep modification/deletion events for audit trail
                    true
                }
                _ => false, // Clean up other old events
            }
        });

        let removed_count = initial_count - events.len();
        tracing::info!(
            "Cleaned up {} audit events (kept {} critical events)",
            removed_count,
            events.len()
        );

        Ok(removed_count as u32)
    }

    pub async fn compact_logs(&self) -> beardog_errors::BearDogResult<u32> {
        tracing::info!("Starting audit log compaction");

        let mut events = self.events.write().await;
        let initial_count = events.len();

        if initial_count == 0 {
            return Ok(0);
        }

        // Sort events by timestamp and user for efficient compaction
        events.sort_by(|a, b| {
            a.subject
                .cmp(&b.subject)
                .then_with(|| a.timestamp.cmp(&b.timestamp))
        });

        let mut compacted_events = Vec::new();
        let mut i = 0;

        while i < events.len() {
            let current_event = &events[i];

            // Look for consecutive similar events from the same user
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
                // Create a compacted summary event
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
                // Keep individual events if not worth compacting
                for k in i..j {
                    compacted_events.push(events[k].clone());
                }
                i = j;
            }
        }

        let compacted_count = initial_count - compacted_events.len();
        *events = compacted_events;

        tracing::info!(
            "Audit log compaction complete: {} events compacted from {} to {}",
            compacted_count,
            initial_count,
            events.len()
        );

        Ok(compacted_count as u32)
    }
}

impl Default for SecurityProviderMetrics {
    fn default() -> Self {
        Self {
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
            rate_limit_violations_per_user: std::collections::HashMap::new(),
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
        }
    }
}

// Display implementations for better logging and debugging

impl std::fmt::Display for HealthStatus {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            HealthStatus::Healthy => write!(f, "Healthy"),
            HealthStatus::Degraded => write!(f, "Degraded"),
            HealthStatus::Unhealthy => write!(f, "Unhealthy"),
        }
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
