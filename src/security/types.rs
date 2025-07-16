//! Type definitions and data structures for security
//!
//! Contains all structs, enums, and type aliases for the security module.

use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use std::time::Duration;

use crate::BearDogResult;

/// Security provider configuration
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SecurityProviderConfig {
    /// Enable rate limiting
    pub rate_limiting_enabled: bool,
    /// Rate limit configuration
    pub rate_limit: RateLimitConfig,
    /// Multi-factor authentication configuration
    pub mfa: MfaConfig,
    /// Session management configuration
    pub session: SessionConfig,
    /// Maximum failed login attempts before lockout
    pub max_failed_attempts: u32,
    /// Account lockout duration in minutes
    pub lockout_duration_minutes: u32,
    /// Enable security audit logging
    pub audit_logging_enabled: bool,
    /// Password policy enforcement
    pub password_policy_enabled: bool,
    /// Minimum password length
    pub min_password_length: usize,
}

/// Rate limiting configuration
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct RateLimitConfig {
    /// Maximum requests per minute
    pub max_requests_per_minute: u32,
    /// Enable per-user rate limiting
    pub per_user_limiting: bool,
    /// Rate limit window in seconds
    pub window_seconds: u32,
}

/// Multi-factor authentication configuration
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct MfaConfig {
    /// Enable MFA requirement
    pub enabled: bool,
    /// Required MFA methods
    pub required_methods: Vec<MfaMethod>,
    /// MFA token validity duration in minutes
    pub token_validity_minutes: u32,
}

/// Multi-factor authentication method types
#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum MfaMethod {
    /// Time-based One-Time Password (TOTP) authentication
    TOTP,
    /// SMS-based authentication
    SMS,
    /// Email-based authentication
    Email,
    /// Hardware token authentication
    Hardware,
    /// Biometric authentication
    Biometric,
}

/// Session management configuration
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SessionConfig {
    /// Session timeout in minutes
    pub timeout_minutes: u32,
    /// Maximum concurrent sessions per user
    pub max_concurrent_sessions: u32,
    /// Enable session encryption
    pub encryption_enabled: bool,
}

/// Security subject (user or system) performing actions
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Subject {
    /// Unique identifier for the subject
    pub id: String,
    /// Type of subject (user, system, service, device)
    pub subject_type: SubjectType,
    /// Additional attributes describing the subject
    pub attributes: HashMap<String, String>,
    /// Roles assigned to this subject
    pub roles: Vec<String>,
    /// Security clearance level if applicable
    pub clearance_level: Option<u32>,
}

/// Types of security subjects
#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum SubjectType {
    /// Human user
    User,
    /// System service or daemon
    System,
    /// External service
    Service,
    /// IoT device or hardware
    Device,
}

/// Security resource being accessed
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Resource {
    /// Unique resource identifier
    pub id: String,
    /// Type of resource
    pub resource_type: String,
    /// Security classification level
    pub classification: ResourceClassification,
    /// Additional resource attributes
    pub attributes: HashMap<String, String>,
    /// Owner of the resource
    pub owner: Option<String>,
}

/// Security classification levels for resources
#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum ResourceClassification {
    /// Publicly accessible information
    Public,
    /// Internal use only
    Internal,
    /// Confidential information requiring authorization
    Confidential,
    /// Secret information requiring high clearance
    Secret,
    /// Top secret information requiring highest clearance
    TopSecret,
}

/// Security action being performed
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Action {
    /// Type of action being performed
    pub action_type: ActionType,
    /// Additional context for the action
    pub context: HashMap<String, String>,
    /// When the action was performed
    pub timestamp: DateTime<Utc>,
    /// Source IP address if applicable
    pub source_ip: Option<String>,
}

/// Types of actions that can be performed on resources
#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum ActionType {
    /// Read or view operation
    Read,
    /// Write or modify operation
    Write,
    /// Delete operation
    Delete,
    /// Execute operation
    Execute,
    /// Approve operation
    Approve,
    /// Administrative operation
    Admin,
}

/// Risk level assessment for security actions
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub enum RiskLevel {
    /// Low risk action
    Low,
    /// Medium risk action
    Medium,
    /// High risk action
    High,
    /// Critical risk action
    Critical,
}

/// Compliance status for security policies
#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum ComplianceStatus {
    /// Fully compliant with all policies
    Compliant,
    /// Not compliant with one or more policies
    NonCompliant,
    /// Conditionally compliant with additional requirements
    ConditionallyCompliant,
    /// Compliance status unknown
    Unknown,
}

/// Account status for user accounts
#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum AccountStatus {
    /// Account is active and can be used
    Active,
    /// Account is inactive but can be reactivated
    Inactive,
    /// Account is locked due to security concerns
    Locked,
    /// Account is suspended temporarily
    Suspended,
    /// Account is pending activation
    Pending,
}

/// Authorization result
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct AuthorizationResult {
    /// Whether the authorization was granted
    pub permitted: bool,
    /// Reason for the authorization decision
    pub reason: String,
    /// Risk level assessed for this authorization
    pub risk_level: RiskLevel,
    /// Additional requirements that must be met
    pub additional_requirements: Vec<String>,
    /// When this authorization expires
    pub expires_at: Option<DateTime<Utc>>,
    /// Unique identifier for audit trail
    pub audit_id: String,
}

/// Security audit event
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SecurityAuditEvent {
    /// Unique identifier for the audit event
    pub id: String,
    /// When the event occurred
    pub timestamp: DateTime<Utc>,
    /// ID of the subject performing the action
    pub subject_id: String,
    /// ID of the resource being accessed
    pub resource_id: String,
    /// Type of action performed
    pub action: ActionType,
    /// Whether the action was successful
    pub result: bool,
    /// Risk level of the action
    pub risk_level: RiskLevel,
    /// Additional details about the event
    pub details: HashMap<String, String>,
}

/// Authentication result
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct AuthenticationResult {
    /// Whether authentication was successful
    pub success: bool,
    /// ID of the authenticated user
    pub user_id: Option<String>,
    /// Session ID if authentication was successful
    pub session_id: Option<String>,
    /// Whether multi-factor authentication is required
    pub mfa_required: bool,
    /// Reason for authentication result
    pub reason: String,
    /// When the authentication expires
    pub expires_at: Option<DateTime<Utc>>,
}

/// User information
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct UserInfo {
    /// Unique user identifier
    pub id: String,
    /// Username for the user
    pub username: String,
    /// Email address of the user
    pub email: Option<String>,
    /// Roles assigned to the user
    pub roles: Vec<String>,
    /// Current status of the user account
    pub account_status: AccountStatus,
    /// Timestamp of last login
    pub last_login: Option<DateTime<Utc>>,
}

/// Security provider health status
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SecurityProviderHealth {
    /// Overall health status of the security provider
    pub overall_status: HealthStatus,
    /// Health status of individual components
    pub components: Vec<ComponentHealth>,
    /// Timestamp of last health check
    pub last_check: DateTime<Utc>,
    /// System uptime in seconds
    pub uptime_seconds: u64,
}

/// Health status enumeration
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub enum HealthStatus {
    /// System is operating normally
    Healthy,
    /// System is functional but with reduced performance
    Degraded,
    /// System is not functioning properly
    Unhealthy,
}

/// Component health information
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ComponentHealth {
    /// Name of the component
    pub name: String,
    /// Health status of the component
    pub status: HealthStatus,
    /// Health status message
    pub message: String,
    /// Timestamp of last health check
    pub last_check: DateTime<Utc>,
}

/// Security provider metrics
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct SecurityProviderMetrics {
    /// Total number of authentication attempts
    pub total_authentications: u64,
    /// Number of successful authentication attempts
    pub successful_authentications: u64,
    /// Number of failed authentication attempts
    pub failed_authentications: u64,
    /// Number of multi-factor authentication attempts
    pub mfa_authentications: u64,

    /// Total number of authorization checks
    pub total_authorizations: u64,
    /// Number of permitted authorization checks
    pub permitted_authorizations: u64,
    /// Number of denied authorization checks
    pub denied_authorizations: u64,

    /// Number of currently active sessions
    pub active_sessions: u64,
    /// Total number of sessions created
    pub total_sessions_created: u64,
    /// Number of expired sessions
    pub expired_sessions: u64,

    /// Number of security violations detected
    pub security_violations: u64,
    /// Number of rate limit violations
    pub rate_limit_violations: u64,
    /// Number of suspicious activities detected
    pub suspicious_activities: u64,

    /// Average authentication time in milliseconds
    pub avg_auth_time_ms: f64,
    /// Average authorization time in milliseconds
    pub avg_authz_time_ms: f64,
    /// Average session duration in minutes
    pub avg_session_duration_minutes: f64,

    /// Number of compliance checks performed
    pub compliance_checks: u64,
    /// Number of compliance violations found
    pub compliance_violations: u64,
    /// Number of audit events generated
    pub audit_events_generated: u64,

    /// System uptime in seconds
    pub uptime_seconds: u64,
    /// Memory usage in bytes
    pub memory_usage_bytes: u64,
    /// CPU usage percentage
    pub cpu_usage_percent: f64,
    /// Disk usage in bytes
    pub disk_usage_bytes: u64,
    /// Network I/O in bytes
    pub network_io_bytes: u64,

    /// Total number of errors
    pub total_errors: u64,
    /// Number of critical errors
    pub critical_errors: u64,
    /// Number of warnings
    pub warning_count: u64,

    /// Number of rate limited requests
    pub rate_limited_requests: u64,
    /// Rate limit violations per user
    pub rate_limit_violations_per_user: HashMap<String, u64>,
}

/// Security session
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SecuritySession {
    /// Unique session identifier
    pub id: String,
    /// ID of the user who owns this session
    pub user_id: String,
    /// When the session was created
    pub created_at: DateTime<Utc>,
    /// When the session expires
    pub expires_at: DateTime<Utc>,
    /// Timestamp of last activity
    pub last_activity: DateTime<Utc>,
    /// IP address of the client
    pub ip_address: Option<String>,
    /// User agent string of the client
    pub user_agent: Option<String>,
    /// Whether the session is currently active
    pub is_active: bool,
    /// Whether multi-factor authentication was verified
    pub mfa_verified: bool,
    /// Additional session attributes
    pub attributes: HashMap<String, String>,
}

/// Rate limiter implementation
#[derive(Debug, Clone)]
pub struct RateLimiter {
    /// Maximum number of requests allowed
    pub max_requests: u32,
    /// Time window in seconds for rate limiting
    pub window_seconds: u32,
    /// Request history for each client
    pub requests: HashMap<String, Vec<DateTime<Utc>>>,
}

/// MFA token information
#[derive(Debug, Clone)]
pub struct MfaToken {
    /// Unique token identifier
    pub id: String,
    /// ID of the user this token belongs to
    pub user_id: String,
    /// MFA method used for this token
    pub method: MfaMethod,
    /// The actual token value
    pub token: String,
    /// When the token was created
    pub created_at: DateTime<Utc>,
    /// When the token expires
    pub expires_at: DateTime<Utc>,
    /// Whether the token has been used
    pub is_used: bool,
}

/// Main security provider implementation
pub struct BearDogSecurityProvider {
    /// Configuration for the security provider
    pub config: SecurityProviderConfig,
    /// Rate limiter for request throttling
    pub rate_limiter: RateLimiter,
    /// Currently active user sessions
    pub active_sessions: HashMap<String, SecuritySession>,
    /// Active MFA tokens
    pub mfa_tokens: HashMap<String, MfaToken>,
    /// Failed login attempts per user
    pub failed_attempts: HashMap<String, u32>,
    /// Locked accounts with unlock timestamps
    pub locked_accounts: HashMap<String, DateTime<Utc>>,
    /// Security provider metrics
    pub metrics: SecurityProviderMetrics,
    /// Audit events for security actions
    pub audit_events: Vec<SecurityAuditEvent>,
    /// Optional threat detection engine
    pub threat_detector: Option<crate::threat::ThreatDetectionEngine>,
    /// Optional workflow engine for approval processes
    pub workflow_engine: Option<crate::workflows::MultiPartyWorkflowEngine>,
}

// Default implementations

impl Default for SecurityProviderConfig {
    fn default() -> Self {
        Self {
            rate_limiting_enabled: true,
            rate_limit: RateLimitConfig::default(),
            mfa: MfaConfig::default(),
            session: SessionConfig::default(),
            max_failed_attempts: 5,
            lockout_duration_minutes: 30,
            audit_logging_enabled: true,
            password_policy_enabled: true,
            min_password_length: 12,
        }
    }
}

impl Default for RateLimitConfig {
    fn default() -> Self {
        Self {
            max_requests_per_minute: 60,
            per_user_limiting: true,
            window_seconds: 60,
        }
    }
}

impl Default for MfaConfig {
    fn default() -> Self {
        Self {
            enabled: true,
            required_methods: vec![MfaMethod::TOTP],
            token_validity_minutes: 5,
        }
    }
}

impl Default for SessionConfig {
    fn default() -> Self {
        Self {
            timeout_minutes: 30,
            max_concurrent_sessions: 3,
            encryption_enabled: true,
        }
    }
}

impl Default for RateLimiter {
    fn default() -> Self {
        Self {
            max_requests: 60,
            window_seconds: 60,
            requests: HashMap::new(),
        }
    }
}

// Display implementations

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
            ActionType::Delete => write!(f, "Delete"),
            ActionType::Execute => write!(f, "Execute"),
            ActionType::Approve => write!(f, "Approve"),
            ActionType::Admin => write!(f, "Admin"),
        }
    }
}

/// Security provider trait definition
pub trait SecurityProvider: Send + Sync {
    /// Authenticate a user
    fn authenticate(
        &self,
        username: &str,
        password: &str,
    ) -> impl std::future::Future<Output = BearDogResult<AuthenticationResult>> + Send;

    /// Authorize an action
    fn authorize(
        &self,
        subject: &Subject,
        resource: &Resource,
        action: &Action,
    ) -> impl std::future::Future<Output = BearDogResult<AuthorizationResult>> + Send;

    /// Validate a session
    fn validate_session(
        &self,
        session_id: &str,
    ) -> impl std::future::Future<Output = BearDogResult<bool>> + Send;

    /// Get health status
    fn health(
        &self,
    ) -> impl std::future::Future<Output = BearDogResult<SecurityProviderHealth>> + Send;

    /// Get metrics
    fn metrics(
        &self,
    ) -> impl std::future::Future<Output = BearDogResult<SecurityProviderMetrics>> + Send;
}

impl BearDogSecurityProvider {
    /// Create placeholder security provider for testing
    ///
    /// ⚠️ WARNING: This method is deprecated and should not be used in production.
    /// Use `BearDogSecurityProvider::new(config)` instead for proper initialization.
    #[deprecated(
        since = "0.1.0",
        note = "Use BearDogSecurityProvider::new(config) instead for proper initialization"
    )]
    pub fn new_placeholder() -> Self {
        // Log warning for deprecated usage
        tracing::warn!("🚨 Using deprecated BearDogSecurityProvider::new_placeholder() method. Use BearDogSecurityProvider::new(config) instead.");

        Self {
            config: SecurityProviderConfig::default(),
            rate_limiter: RateLimiter::default(),
            active_sessions: HashMap::new(),
            mfa_tokens: HashMap::new(),
            failed_attempts: HashMap::new(),
            locked_accounts: HashMap::new(),
            metrics: SecurityProviderMetrics::default(),
            audit_events: Vec::new(),
            threat_detector: None,
            workflow_engine: None,
        }
    }

    /// Create minimal security provider for development
    pub fn new_minimal() -> Self {
        Self {
            config: SecurityProviderConfig::default(),
            rate_limiter: RateLimiter::default(),
            active_sessions: HashMap::new(),
            mfa_tokens: HashMap::new(),
            failed_attempts: HashMap::new(),
            locked_accounts: HashMap::new(),
            metrics: SecurityProviderMetrics::default(),
            audit_events: Vec::new(),
            threat_detector: None,
            workflow_engine: None,
        }
    }
}

// Add missing types for security provider
/// Security configuration settings
#[derive(Debug, Clone)]
pub struct SecurityConfig {
    /// Authentication timeout duration
    pub auth_timeout: Duration,
    /// Maximum number of concurrent sessions
    pub max_sessions: u32,
}

impl Default for SecurityConfig {
    fn default() -> Self {
        Self {
            auth_timeout: Duration::from_secs(3600),
            max_sessions: 1000,
        }
    }
}

/// Security metrics and statistics
#[derive(Debug, Clone)]
pub struct SecurityMetrics {
    /// Number of authentication attempts
    pub authentication_attempts: u64,
    /// Number of authorization checks performed
    pub authorization_checks: u64,
    /// Number of security events recorded
    pub security_events: u64,
}

impl Default for SecurityMetrics {
    fn default() -> Self {
        Self::new()
    }
}

impl SecurityMetrics {
    /// Create a new SecurityMetrics instance with default values
    pub fn new() -> Self {
        Self {
            authentication_attempts: 0,
            authorization_checks: 0,
            security_events: 0,
        }
    }
}

/// Threat analysis and detection system
#[derive(Debug, Clone)]
pub struct ThreatAnalyzer {
    /// Number of currently active threats
    pub active_threats: u64,
}

impl Default for ThreatAnalyzer {
    fn default() -> Self {
        Self::new()
    }
}

impl ThreatAnalyzer {
    /// Create a new ThreatAnalyzer instance with default values
    pub fn new() -> Self {
        Self { active_threats: 0 }
    }
}

/// Security rules and policies
#[derive(Debug, Clone)]
pub struct SecurityRules {
    /// List of security rules
    pub rules: Vec<String>,
}

impl Default for SecurityRules {
    fn default() -> Self {
        Self::new()
    }
}

impl SecurityRules {
    /// Create a new SecurityRules instance with default values
    pub fn new() -> Self {
        Self { rules: Vec::new() }
    }
}

/// Audit logging system for security events
#[derive(Debug, Clone)]
pub struct AuditLogger {
    /// Audit log entries
    pub entries: Vec<String>,
}

impl Default for AuditLogger {
    fn default() -> Self {
        Self::new()
    }
}

impl AuditLogger {
    /// Create a new AuditLogger instance with default values
    pub fn new() -> Self {
        Self {
            entries: Vec::new(),
        }
    }
}

/// Rate limiting system for security operations
#[derive(Debug, Clone)]
pub struct SecurityRateLimiter {
    /// Rate limits per client or endpoint
    pub limits: HashMap<String, u32>,
}

impl Default for SecurityRateLimiter {
    fn default() -> Self {
        Self::new()
    }
}

impl SecurityRateLimiter {
    /// Create a new SecurityRateLimiter instance with default values
    pub fn new() -> Self {
        Self {
            limits: HashMap::new(),
        }
    }
}
