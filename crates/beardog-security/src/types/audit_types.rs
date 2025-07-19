//! Audit and Monitoring Types
//!
//! This module contains all types related to security auditing, health monitoring,
//! metrics collection, and security event logging.

use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};
use std::collections::HashMap;

/// Security audit event for tracking security-related activities
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SecurityAuditEvent {
    /// Unique identifier for the audit event
    pub event_id: String,
    /// Type of event that occurred
    pub event_type: String,
    /// Subject that performed the action
    pub subject: super::auth_types::Subject,
    /// Resource that was accessed
    pub resource: Resource,
    /// Action that was performed
    pub action: Action,
    /// Timestamp when the event occurred
    pub timestamp: DateTime<Utc>,
    /// Whether the action was successful
    pub success: bool,
    /// Additional event metadata
    pub metadata: HashMap<String, String>,
}

/// Security resource being accessed
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq, Hash)]
pub struct Resource {
    /// Unique identifier for the resource
    pub id: String,
    /// Type of resource (file, database, service, etc.)
    pub resource_type: String,
    /// Security classification of the resource
    pub classification: ResourceClassification,
    /// Additional resource metadata
    pub metadata: HashMap<String, String>,
}

/// Security classification levels for resources
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq, Hash)]
pub enum ResourceClassification {
    /// Public information
    Public,
    /// Internal use only
    Internal,
    /// Confidential information
    Confidential,
    /// Highly sensitive restricted information
    Restricted,
}

/// Security action being performed
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq, Hash)]
pub struct Action {
    /// Unique identifier for the action type
    pub action_type: ActionType,
    /// Human-readable description of the action
    pub description: String,
    /// Risk level associated with this action
    pub risk_level: RiskLevel,
}

/// Types of actions that can be performed on resources
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq, Hash)]
pub enum ActionType {
    /// Reading or viewing data
    Read,
    /// Creating new data
    Create,
    /// Modifying existing data
    Update,
    /// Removing data
    Delete,
    /// Executing code or operations
    Execute,
}

/// Risk level assessment for security actions
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq, Hash)]
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

/// Security provider health status information
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SecurityProviderHealth {
    /// Overall health status
    pub status: HealthStatus,
    /// Individual component health
    pub components: HashMap<String, ComponentHealth>,
    /// Last health check timestamp
    pub last_check: DateTime<Utc>,
    /// Health check metadata
    pub metadata: HashMap<String, String>,
}

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

/// Component health information
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ComponentHealth {
    /// Whether the component is healthy
    pub healthy: bool,
    /// Status message or error description
    pub status: String,
    /// Component-specific metrics
    pub metrics: HashMap<String, f64>,
}

/// Security provider metrics and performance data
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SecurityProviderMetrics {
    /// Authentication success rate (0.0 - 1.0)
    pub auth_success_rate: f64,
    /// Authorization success rate (0.0 - 1.0)
    pub authz_success_rate: f64,
    /// Average response time in milliseconds
    pub avg_response_time_ms: f64,
    /// Requests per second
    pub requests_per_second: f64,
    /// Error rate (0.0 - 1.0)
    pub error_rate: f64,
    /// Active sessions count
    pub active_sessions: u64,
    /// Failed login attempts in last hour
    pub failed_login_attempts: u64,
    /// Number of blocked requests
    pub blocked_requests: u64,
    /// Timestamp when metrics were collected
    pub collected_at: DateTime<Utc>,
    /// Custom metrics
    pub custom_metrics: HashMap<String, f64>,
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
            ActionType::Create => write!(f, "Create"),
            ActionType::Update => write!(f, "Update"),
            ActionType::Delete => write!(f, "Delete"),
            ActionType::Execute => write!(f, "Execute"),
        }
    }
} 