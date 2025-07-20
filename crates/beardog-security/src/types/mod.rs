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

pub mod audit_types;
pub mod auth_types;
pub mod config_types;
pub mod crypto_types;

// Re-export commonly used types for backward compatibility and convenience
pub use audit_types::*;
pub use auth_types::*;
pub use config_types::*;
pub use crypto_types::*;

// Additional types that don't fit cleanly into the above categories

use async_trait::async_trait;
use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};
use std::collections::HashMap;

/// Rate limiter implementation for controlling operation frequency
#[derive(Debug, Clone, Serialize, Deserialize, Default)]
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

/// Main BearDog Security Provider implementation
#[derive(Debug)]
pub struct BearDogSecurityProvider {
    /// Security provider configuration
    pub config: SecurityProviderConfig,
    /// Rate limiter for security operations
    pub rate_limiter: RateLimiter,
    /// Security rules engine
    pub security_rules: SecurityRules,
    /// Locked accounts storage
    pub locked_accounts:
        std::sync::Arc<tokio::sync::RwLock<HashMap<String, chrono::DateTime<chrono::Utc>>>>,
    /// Failed authentication attempts tracking
    pub failed_attempts:
        std::sync::Arc<tokio::sync::RwLock<HashMap<String, Vec<chrono::DateTime<chrono::Utc>>>>>,
    /// Security session store
    pub session_store: auth_types::SessionStore,
    /// Audit manager
    pub audit_manager: audit_types::AuditManager,
    /// Security metrics collector
    pub metrics: SecurityProviderMetrics,
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
#[derive(Debug, Clone, Serialize, Deserialize, Default)]
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
    async fn authenticate(
        &self,
        credentials: &HashMap<String, String>,
    ) -> Result<AuthenticationResult, SecurityError>;

    /// Authorize a subject to perform an action on a resource
    async fn authorize(
        &self,
        subject: &Subject,
        action: &Action,
        resource: &Resource,
    ) -> Result<AuthorizationResult, SecurityError>;

    /// Create a new security session
    async fn create_session(
        &self,
        user: &UserInfo,
        client_ip: String,
        user_agent: String,
    ) -> Result<SecuritySession, SecurityError>;

    /// Validate an existing session
    async fn validate_session(
        &self,
        session_id: &str,
    ) -> Result<Option<SecuritySession>, SecurityError>;

    /// Revoke a security session
    async fn revoke_session(&self, session_id: &str) -> Result<(), SecurityError>;

    /// Log a security audit event
    async fn audit(&self, event: SecurityAuditEvent) -> Result<(), SecurityError>;

    /// Get current health status
    async fn health_check(&self) -> Result<SecurityProviderHealth, SecurityError>;

    /// Get current metrics
    async fn get_metrics(&self) -> Result<SecurityProviderMetrics, SecurityError>;
}

/// Security provider error types
#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum SecurityError {
    /// Authentication failed
    AuthenticationFailed {
        /// Error message
        message: String,
    },
    /// Authorization failed
    AuthorizationFailed {
        /// Error message
        message: String,
    },
    /// Rate limiting error
    RateLimit {
        /// Error message
        message: String,
    },
    /// Configuration error
    ConfigurationError {
        /// Error message
        message: String,
    },
    /// Cryptographic operation error
    CryptographicError {
        /// Error message
        message: String,
    },
    /// Unknown error
    Unknown {
        /// Error message
        message: String,
    },
}

// Conversion from BearDogError to SecurityError
impl From<beardog_errors::BearDogError> for SecurityError {
    fn from(error: beardog_errors::BearDogError) -> Self {
        match error {
            beardog_errors::BearDogError::Authentication { message } => {
                SecurityError::AuthenticationFailed { message }
            }
            beardog_errors::BearDogError::Authorization { message } => {
                SecurityError::AuthorizationFailed { message }
            }
            beardog_errors::BearDogError::RateLimit { message } => {
                SecurityError::RateLimit { message }
            }
            beardog_errors::BearDogError::Configuration { message } => {
                SecurityError::ConfigurationError { message }
            }
            beardog_errors::BearDogError::Cryptographic { operation } => {
                SecurityError::CryptographicError {
                    message: format!("Cryptographic operation failed: {operation}"),
                }
            }
            _ => SecurityError::Unknown {
                message: format!("Unknown error: {error}"),
            },
        }
    }
}

// Default implementations

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

impl BearDogSecurityProvider {
    /// Create a new security provider with default configuration
    pub fn new() -> Self {
        Self {
            config: SecurityProviderConfig::default(),
            rate_limiter: RateLimiter {
                config: RateLimitConfig::default(),
                state: HashMap::new(),
            },
            security_rules: SecurityRules {
                rules: Vec::new(),
                default_policy: PolicyDecision::Deny,
                context: auth_types::SecurityContext::default(),
            },
            locked_accounts: std::sync::Arc::new(tokio::sync::RwLock::new(HashMap::new())),
            failed_attempts: std::sync::Arc::new(tokio::sync::RwLock::new(HashMap::new())),
            session_store: auth_types::SessionStore::default(),
            audit_manager: audit_types::AuditManager::default(),
            metrics: SecurityProviderMetrics {
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
            },
        }
    }
}

impl Default for BearDogSecurityProvider {
    fn default() -> Self {
        Self::new()
    }
}
