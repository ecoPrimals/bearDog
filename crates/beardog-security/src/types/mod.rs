//! BearDog Security Types
//!
//! This module provides a clean, modular type system for security operations.
//! Types are organized by concern to reduce complexity and improve maintainability.
//!
//! ## Module Organization
//! - `auth_types` - Authentication, authorization, and session management
//! - `crypto_types` - Cryptographic operations and key management
//! - `audit_types` - Security auditing, monitoring, and metrics
//! - `config_types` - Security configuration and policy management

pub mod auth_types;
pub mod crypto_types;
pub mod audit_types;
pub mod config_types;

// Re-export commonly used types for backward compatibility and convenience
pub use auth_types::*;
pub use crypto_types::*;
pub use audit_types::*;
pub use config_types::*;

// Additional types that don't fit cleanly into the above categories

use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use async_trait::async_trait;

/// Rate limiter implementation for controlling operation frequency
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct RateLimiter {
    /// Rate limiting configuration
    pub config: RateLimitConfig,
    /// Internal state tracking (not serialized in production)
    #[serde(skip)]
    pub state: HashMap<String, RateLimiterState>,
}

/// Internal rate limiter state
#[derive(Debug, Clone)]
pub struct RateLimiterState {
    /// Number of operations in current window
    pub count: u32,
    /// Window start time
    pub window_start: DateTime<Utc>,
    /// Last operation time
    pub last_operation: DateTime<Utc>,
}

/// Main BearDog security provider implementation
pub struct BearDogSecurityProvider {
    /// Security provider configuration
    pub config: SecurityProviderConfig,
    /// Rate limiter for security operations
    pub rate_limiter: RateLimiter,
    /// Security rules engine
    pub security_rules: SecurityRules,
    /// Audit logger
    pub audit_logger: AuditLogger,
    /// Threat analyzer (optional)
    pub threat_analyzer: Option<ThreatAnalyzer>,
    /// Security metrics collector
    pub metrics: SecurityProviderMetrics,
    /// Security session manager
    pub sessions: HashMap<String, SecuritySession>,
}

/// Audit logging system for security events
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct AuditLogger {
    /// Whether audit logging is enabled
    pub enabled: bool,
    /// Audit configuration
    pub config: AuditConfig,
    /// Log entries buffer
    pub entries: Vec<AuditLogEntry>,
    /// Maximum entries before flush
    pub max_buffer_size: usize,
}

/// Rate limiting system for security operations
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SecurityRateLimiter {
    /// Rate limiting configuration
    pub config: RateLimitConfig,
    /// Current rate limiting state
    pub state: HashMap<String, u32>,
    /// Window start times
    pub windows: HashMap<String, DateTime<Utc>>,
}

/// Main security provider trait definition
#[async_trait]
pub trait SecurityProvider: Send + Sync {
    /// Authenticate a user with the given credentials
    async fn authenticate(&self, credentials: &HashMap<String, String>) -> Result<AuthenticationResult, SecurityError>;

    /// Authorize a subject to perform an action on a resource
    async fn authorize(&self, subject: &Subject, action: &Action, resource: &Resource) -> Result<AuthorizationResult, SecurityError>;

    /// Create a new security session
    async fn create_session(&self, user: &UserInfo, client_ip: String, user_agent: String) -> Result<SecuritySession, SecurityError>;

    /// Validate an existing session
    async fn validate_session(&self, session_id: &str) -> Result<Option<SecuritySession>, SecurityError>;

    /// Revoke a security session
    async fn revoke_session(&self, session_id: &str) -> Result<(), SecurityError>;

    /// Log a security audit event
    async fn audit(&self, event: SecurityAuditEvent) -> Result<(), SecurityError>;

    /// Get current health status
    async fn health_check(&self) -> Result<SecurityProviderHealth, SecurityError>;

    /// Get current metrics
    async fn get_metrics(&self) -> Result<SecurityProviderMetrics, SecurityError>;
}

/// Security error types
#[derive(Debug, thiserror::Error)]
pub enum SecurityError {
    /// Authentication failed
    #[error("Authentication failed: {message}")]
    AuthenticationFailed { message: String },

    /// Authorization denied
    #[error("Authorization denied: {message}")]
    AuthorizationDenied { message: String },

    /// Invalid session
    #[error("Invalid session: {message}")]
    InvalidSession { message: String },

    /// Rate limit exceeded
    #[error("Rate limit exceeded: {message}")]
    RateLimitExceeded { message: String },

    /// Configuration error
    #[error("Configuration error: {message}")]
    ConfigurationError { message: String },

    /// Internal error
    #[error("Internal error: {message}")]
    InternalError { message: String },

    /// Cryptographic error
    #[error("Cryptographic error: {message}")]
    CryptographicError { message: String },

    /// Audit logging error
    #[error("Audit logging error: {message}")]
    AuditError { message: String },
}

// Conversion from BearDogError to SecurityError
impl From<beardog_errors::BearDogError> for SecurityError {
    fn from(error: beardog_errors::BearDogError) -> Self {
        match error {
            beardog_errors::BearDogError::Authentication { message } => SecurityError::AuthenticationFailed { message },
            beardog_errors::BearDogError::Authorization { message } => SecurityError::AuthorizationDenied { message },
            beardog_errors::BearDogError::Configuration { message } => SecurityError::ConfigurationError { message },
            beardog_errors::BearDogError::Cryptographic { message } => SecurityError::CryptographicError { message },
            _ => SecurityError::InternalError { 
                message: error.to_string() 
            },
        }
    }
}

// Default implementations

impl Default for RateLimiter {
    fn default() -> Self {
        Self {
            config: RateLimitConfig::default(),
            state: HashMap::new(),
        }
    }
}

impl Default for AuditLogger {
    fn default() -> Self {
        Self {
            enabled: true,
            config: AuditConfig::default(),
            entries: Vec::new(),
            max_buffer_size: 1000,
        }
    }
}

impl Default for SecurityRateLimiter {
    fn default() -> Self {
        Self {
            config: RateLimitConfig::default(),
            state: HashMap::new(),
            windows: HashMap::new(),
        }
    }
}

impl BearDogSecurityProvider {
    /// Create a new security provider with default configuration
    pub fn new() -> Self {
        Self::with_config(SecurityProviderConfig::default())
    }

    /// Create a new security provider with custom configuration
    pub fn with_config(config: SecurityProviderConfig) -> Self {
        Self {
            rate_limiter: RateLimiter {
                config: config.rate_limit_config.clone(),
                state: HashMap::new(),
            },
            security_rules: SecurityRules {
                rules: Vec::new(),
                default_policy: PolicyDecision::Deny,
                timeout_ms: 5000,
                enable_caching: true,
            },
            audit_logger: AuditLogger::default(),
            threat_analyzer: config.threat_detector.clone(),
            metrics: SecurityProviderMetrics {
                auth_success_rate: 0.0,
                authz_success_rate: 0.0,
                avg_response_time_ms: 0.0,
                requests_per_second: 0.0,
                error_rate: 0.0,
                active_sessions: 0,
                failed_login_attempts: 0,
                blocked_requests: 0,
                collected_at: Utc::now(),
                custom_metrics: HashMap::new(),
            },
            sessions: HashMap::new(),
            config,
        }
    }
}

impl Default for BearDogSecurityProvider {
    fn default() -> Self {
        Self::new()
    }
} 