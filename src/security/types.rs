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

/// MFA method types
#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum MfaMethod {
    TOTP,
    SMS,
    Email,
    Hardware,
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

/// Security subject (user or system)
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Subject {
    pub id: String,
    pub subject_type: SubjectType,
    pub attributes: HashMap<String, String>,
    pub roles: Vec<String>,
    pub clearance_level: Option<u32>,
}

/// Subject type enumeration
#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum SubjectType {
    User,
    System,
    Service,
    Device,
}

/// Security resource
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Resource {
    pub id: String,
    pub resource_type: String,
    pub classification: ResourceClassification,
    pub attributes: HashMap<String, String>,
    pub owner: Option<String>,
}

/// Resource classification levels
#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum ResourceClassification {
    Public,
    Internal,
    Confidential,
    Secret,
    TopSecret,
}

/// Security action
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Action {
    pub action_type: ActionType,
    pub context: HashMap<String, String>,
    pub timestamp: DateTime<Utc>,
    pub source_ip: Option<String>,
}

/// Action type enumeration
#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum ActionType {
    Read,
    Write,
    Delete,
    Execute,
    Approve,
    Admin,
}

/// Risk level assessment
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub enum RiskLevel {
    Low,
    Medium,
    High,
    Critical,
}

/// Authorization result
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct AuthorizationResult {
    pub permitted: bool,
    pub reason: String,
    pub risk_level: RiskLevel,
    pub additional_requirements: Vec<String>,
    pub expires_at: Option<DateTime<Utc>>,
    pub audit_id: String,
}

/// Compliance status
#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum ComplianceStatus {
    Compliant,
    NonCompliant,
    ConditionallyCompliant,
    Unknown,
}

/// Security audit event
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SecurityAuditEvent {
    pub id: String,
    pub timestamp: DateTime<Utc>,
    pub subject_id: String,
    pub resource_id: String,
    pub action: ActionType,
    pub result: bool,
    pub risk_level: RiskLevel,
    pub details: HashMap<String, String>,
}

/// Authentication result
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct AuthenticationResult {
    pub success: bool,
    pub user_id: Option<String>,
    pub session_id: Option<String>,
    pub mfa_required: bool,
    pub reason: String,
    pub expires_at: Option<DateTime<Utc>>,
}

/// User information
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct UserInfo {
    pub id: String,
    pub username: String,
    pub email: Option<String>,
    pub roles: Vec<String>,
    pub account_status: AccountStatus,
    pub last_login: Option<DateTime<Utc>>,
}

/// Account status enumeration
#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum AccountStatus {
    Active,
    Inactive,
    Locked,
    Suspended,
    Pending,
}

/// Security provider health status
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SecurityProviderHealth {
    pub overall_status: HealthStatus,
    pub components: Vec<ComponentHealth>,
    pub last_check: DateTime<Utc>,
    pub uptime_seconds: u64,
}

/// Health status enumeration
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub enum HealthStatus {
    Healthy,
    Degraded,
    Unhealthy,
}

/// Component health information
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ComponentHealth {
    pub name: String,
    pub status: HealthStatus,
    pub message: String,
    pub last_check: DateTime<Utc>,
}

/// Security provider metrics
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct SecurityProviderMetrics {
    // Authentication metrics
    pub total_authentications: u64,
    pub successful_authentications: u64,
    pub failed_authentications: u64,
    pub mfa_authentications: u64,
    
    // Authorization metrics
    pub total_authorizations: u64,
    pub permitted_authorizations: u64,
    pub denied_authorizations: u64,
    
    // Session metrics
    pub active_sessions: u64,
    pub total_sessions_created: u64,
    pub expired_sessions: u64,
    
    // Security metrics
    pub security_violations: u64,
    pub rate_limit_violations: u64,
    pub suspicious_activities: u64,
    
    // Performance metrics
    pub avg_auth_time_ms: f64,
    pub avg_authz_time_ms: f64,
    pub avg_session_duration_minutes: f64,
    
    // Compliance metrics
    pub compliance_checks: u64,
    pub compliance_violations: u64,
    pub audit_events_generated: u64,
    
    // System metrics
    pub uptime_seconds: u64,
    pub memory_usage_bytes: u64,
    pub cpu_usage_percent: f64,
    pub disk_usage_bytes: u64,
    pub network_io_bytes: u64,
    
    // Error metrics
    pub total_errors: u64,
    pub critical_errors: u64,
    pub warning_count: u64,
    
    // Rate limiting metrics
    pub rate_limited_requests: u64,
    pub rate_limit_violations_per_user: HashMap<String, u64>,
}

/// Security session
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SecuritySession {
    pub id: String,
    pub user_id: String,
    pub created_at: DateTime<Utc>,
    pub expires_at: DateTime<Utc>,
    pub last_activity: DateTime<Utc>,
    pub ip_address: Option<String>,
    pub user_agent: Option<String>,
    pub is_active: bool,
    pub mfa_verified: bool,
    pub attributes: HashMap<String, String>,
}

/// Rate limiter implementation
#[derive(Debug, Clone)]
pub struct RateLimiter {
    pub max_requests: u32,
    pub window_seconds: u32,
    pub requests: HashMap<String, Vec<DateTime<Utc>>>,
}

/// MFA token information
#[derive(Debug, Clone)]
pub struct MfaToken {
    pub id: String,
    pub user_id: String,
    pub method: MfaMethod,
    pub token: String,
    pub created_at: DateTime<Utc>,
    pub expires_at: DateTime<Utc>,
    pub is_used: bool,
}

/// Main security provider implementation
pub struct BearDogSecurityProvider {
    pub config: SecurityProviderConfig,
    pub rate_limiter: RateLimiter,
    pub active_sessions: HashMap<String, SecuritySession>,
    pub mfa_tokens: HashMap<String, MfaToken>,
    pub failed_attempts: HashMap<String, u32>,
    pub locked_accounts: HashMap<String, DateTime<Utc>>,
    pub metrics: SecurityProviderMetrics,
    pub audit_events: Vec<SecurityAuditEvent>,
    pub threat_detector: Option<crate::threat::ThreatDetectionEngine>,
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
    fn authenticate(&self, username: &str, password: &str) -> impl std::future::Future<Output = BearDogResult<AuthenticationResult>> + Send;
    
    /// Authorize an action
    fn authorize(&self, subject: &Subject, resource: &Resource, action: &Action) -> impl std::future::Future<Output = BearDogResult<AuthorizationResult>> + Send;
    
    /// Validate a session
    fn validate_session(&self, session_id: &str) -> impl std::future::Future<Output = BearDogResult<bool>> + Send;
    
    /// Get health status
    fn health(&self) -> impl std::future::Future<Output = BearDogResult<SecurityProviderHealth>> + Send;
    
    /// Get metrics
    fn metrics(&self) -> impl std::future::Future<Output = BearDogResult<SecurityProviderMetrics>> + Send;
}

impl BearDogSecurityProvider {
    /// Create placeholder security provider for testing
    pub fn new_placeholder() -> Self {
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
        Self::new_placeholder()
    }
}

// Add missing types for security provider
#[derive(Debug, Clone)]
pub struct SecurityConfig {
    pub auth_timeout: Duration,
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

#[derive(Debug, Clone)]
pub struct SecurityMetrics {
    pub authentication_attempts: u64,
    pub authorization_checks: u64,
    pub security_events: u64,
}

impl SecurityMetrics {
    pub fn new() -> Self {
        Self {
            authentication_attempts: 0,
            authorization_checks: 0,
            security_events: 0,
        }
    }
}



#[derive(Debug, Clone)]
pub struct ThreatAnalyzer {
    pub active_threats: u64,
}

impl ThreatAnalyzer {
    pub fn new() -> Self {
        Self {
            active_threats: 0,
        }
    }
}

#[derive(Debug, Clone)]
pub struct SecurityRules {
    pub rules: Vec<String>,
}

impl SecurityRules {
    pub fn new() -> Self {
        Self {
            rules: Vec::new(),
        }
    }
}

#[derive(Debug, Clone)]
pub struct AuditLogger {
    pub entries: Vec<String>,
}

impl AuditLogger {
    pub fn new() -> Self {
        Self {
            entries: Vec::new(),
        }
    }
}

#[derive(Debug, Clone)]
pub struct SecurityRateLimiter {
    pub limits: HashMap<String, u32>,
}

impl SecurityRateLimiter {
    pub fn new() -> Self {
        Self {
            limits: HashMap::new(),
        }
    }
}

