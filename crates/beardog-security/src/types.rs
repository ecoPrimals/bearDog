//! Security Types - Authentication and Authorization
//!
//! Provides security-related types including:
//! - Rate limiting and policy decisions
//! - Session management and audit logging
//! - Security contexts and rules

use chrono::{DateTime, Utc};
use std::collections::HashMap;
use std::sync::Arc;
use tokio::sync::RwLock;

// Use specific imports to avoid ambiguous glob re-exports
pub use beardog_types::canonical::config;

// Re-export commonly used types explicitly - using a placeholder config for now
#[derive(Debug, Clone, Default, serde::Serialize, serde::Deserialize)]
pub struct AuthConfig {
    pub session_timeout_seconds: u64,
    pub require_mfa: bool,
    pub jwt_secret: String,
}

#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
pub struct SecurityProviderConfig {
    /// Maximum failed authentication attempts before lockout
    pub max_failed_attempts: u32,
    /// Lockout duration in minutes after max attempts exceeded
    pub lockout_duration_minutes: u32,
    /// Session timeout in minutes
    pub session_timeout_minutes: u32,
    /// Enable audit logging for security events
    pub enable_audit_logging: bool,
    /// Require multi-factor authentication
    pub require_mfa: bool,
}

impl Default for SecurityProviderConfig {
    fn default() -> Self {
        Self {
            max_failed_attempts: 5,
            lockout_duration_minutes: 30,
            session_timeout_minutes: 60,
            enable_audit_logging: true,
            require_mfa: false,
        }
    }
}

/// Rate limiting configuration (DEPRECATED - use canonical)
///
/// **MIGRATION**: Use `beardog_types::canonical::config::domains::network::RateLimitConfig` instead.
///
/// This type alias will be removed in v3.3.0.
#[deprecated(
    since = "3.1.0",
    note = "Use beardog_types::canonical::config::domains::network::RateLimitConfig instead"
)]
pub type RateLimitConfig = beardog_types::canonical::config::domains::network::RateLimitConfig;

#[derive(Debug, Clone)]
pub struct BearDogSecurityProvider {
    /// Security provider configuration
    pub config: SecurityProviderConfig,
    /// Rate limiter for request throttling
    pub rate_limiter: RateLimiter,
    /// Security rules for policy evaluation
    pub security_rules: SecurityRules,
    /// Map of locked account IDs to unlock times
    pub locked_accounts: Arc<RwLock<HashMap<String, DateTime<Utc>>>>,
    /// Map of user IDs to failed attempt counts
    pub failed_attempts: Arc<RwLock<HashMap<String, u32>>>,
    /// Security metrics counters
    pub metrics: SecurityProviderMetrics,
    /// Session store for active sessions
    pub session_store: SessionStore,
    /// Audit manager for security event logging
    pub audit_manager: AuditManager,
}

#[derive(Debug, Clone)]
pub struct RateLimiter {
    /// Rate limit configuration
    pub config: RateLimitConfig,
    /// Per-identifier rate limit state
    pub state: HashMap<String, RateLimiterState>,
}

#[derive(Debug, Clone)]
pub struct RateLimiterState {
    /// Number of requests in current window
    pub requests: u32,
    /// Timestamp when the window last reset
    pub last_reset: DateTime<Utc>,
}

#[derive(Debug, Clone)]
pub struct SecurityRules {
    /// Active security rules
    pub rules: Vec<SecurityRule>,
    /// Default policy when no rule matches
    pub default_policy: PolicyDecision,
    /// Current security context
    pub context: SecurityContext,
}

#[derive(Debug, Clone)]
pub struct SecurityRule {
    /// Rule identifier
    pub name: String,
    /// Condition expression for rule matching
    pub condition: String,
    /// Action to take when rule matches
    pub action: PolicyDecision,
}

#[derive(Debug, Clone)]
pub struct SecurityContext {
    /// Authenticated user ID (if any)
    pub user_id: Option<String>,
    /// Client IP address
    pub ip_address: Option<String>,
    /// Client user agent string
    pub user_agent: Option<String>,
    /// Context creation timestamp
    pub timestamp: DateTime<Utc>,
}

impl Default for SecurityContext {
    fn default() -> Self {
        Self {
            user_id: None,
            ip_address: None,
            user_agent: None,
            timestamp: Utc::now(),
        }
    }
}

#[derive(Debug, Clone, serde::Serialize, serde::Deserialize, Default)]
pub struct SecurityProviderMetrics {
    /// Total successful authentication attempts
    pub successful_authentications: u64,
    /// Total failed authentication attempts
    pub failed_authentications: u64,
    /// Total requests blocked by rate limiter
    pub blocked_requests: u64,
    /// Current number of active sessions
    pub active_sessions: u64,
    /// Total security events logged
    pub security_events: u64,
}

#[derive(Debug, Clone)]
pub struct SessionStore {
    /// Active sessions by session ID
    pub sessions: Arc<RwLock<HashMap<String, SessionData>>>,
    /// Authentication configuration
    pub config: AuthConfig,
}

#[derive(Debug, Clone)]
pub struct SessionData {
    /// User ID associated with this session
    pub user_id: String,
    /// Session creation timestamp
    pub created_at: DateTime<Utc>,
    /// Last activity timestamp
    pub last_accessed: DateTime<Utc>,
    /// Client IP address at session creation
    pub ip_address: String,
    /// Client user agent at session creation
    pub user_agent: String,
    /// Whether session is currently authenticated
    pub is_authenticated: bool,
}

#[derive(Debug, Clone)]
pub struct AuditManager {
    /// Audit logging configuration
    pub config: AuditConfig,
    /// In-memory audit event buffer
    pub events: Arc<RwLock<Vec<AuditEvent>>>,
}

#[derive(Debug, Clone)]
pub struct AuditConfig {
    /// Whether audit logging is enabled
    pub enabled: bool,
    /// Minimum severity level to log
    pub log_level: AuditLevel,
    /// Event retention period in days
    pub retention_days: u32,
    /// Maximum events to keep in memory buffer
    pub max_events: usize,
}

#[derive(Debug, Clone)]
pub struct AuditEvent {
    /// Unique event identifier
    pub id: String,
    /// Event type (e.g., "login", "access_denied")
    pub event_type: String,
    /// Associated user ID (if applicable)
    pub user_id: Option<String>,
    /// Event timestamp
    pub timestamp: DateTime<Utc>,
    /// Additional event details as key-value pairs
    pub details: HashMap<String, String>,
    /// Event severity level
    pub level: AuditLevel,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum AuditLevel {
    /// Debugging/trace level events
    Debug,
    /// Low importance informational events
    Low,
    /// Normal operational events
    Medium,
    /// Important security-relevant events
    High,
    /// Critical security events requiring attention
    Critical,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum PolicyDecision {
    /// Allow the request to proceed
    Allow,
    /// Deny the request
    Deny,
    /// Require additional authentication challenge
    Challenge,
}

impl Default for AuditConfig {
    fn default() -> Self {
        Self {
            enabled: true,
            log_level: AuditLevel::Medium,
            retention_days: 90,
            max_events: 10000,
        }
    }
}

impl BearDogSecurityProvider {
    /// Creates a new security provider with the given configuration.
    pub fn new(config: SecurityProviderConfig) -> Self {
        Self {
            config: config.clone(),
            rate_limiter: RateLimiter::new(RateLimitConfig::default()),
            security_rules: SecurityRules::default(),
            locked_accounts: Arc::new(RwLock::new(HashMap::with_capacity(16))),
            failed_attempts: Arc::new(RwLock::new(HashMap::with_capacity(16))),
            metrics: SecurityProviderMetrics::default(),
            session_store: SessionStore::new(AuthConfig::default()),
            audit_manager: AuditManager::new(AuditConfig::default()),
        }
    }

    /// Checks if the given user account is currently locked.
    pub async fn is_account_locked(&self, user_id: &str) -> bool {
        let locked_accounts = self.locked_accounts.read().await;
        if let Some(locked_until) = locked_accounts.get(user_id) {
            Utc::now() < *locked_until
        } else {
            false
        }
    }

    /// Records a failed authentication attempt for rate limiting.
    pub async fn record_failed_attempt(&mut self, user_id: &str) {
        let mut failed_attempts = self.failed_attempts.write().await;
        let attempts = failed_attempts.entry(user_id.to_string()).or_insert(0);
        *attempts += 1;

        if *attempts >= self.config.max_failed_attempts {
            let lockout_duration =
                chrono::Duration::minutes(self.config.lockout_duration_minutes as i64);
            let locked_until = Utc::now() + lockout_duration;

            let mut locked_accounts = self.locked_accounts.write().await;
            locked_accounts.insert(user_id.to_string(), locked_until);

            failed_attempts.remove(user_id);
        }
    }

    /// Records a successful authentication, clearing failed attempt counter.
    pub async fn record_successful_auth(&mut self, user_id: &str) {
        let mut failed_attempts = self.failed_attempts.write().await;
        failed_attempts.remove(user_id);

        self.metrics.successful_authentications += 1;
    }
}

impl RateLimiter {
    /// Creates a new rate limiter with the given configuration.
    pub fn new(config: RateLimitConfig) -> Self {
        Self {
            config,
            state: HashMap::with_capacity(16),
        }
    }

    /// Checks if the given identifier is currently rate limited.
    pub fn is_rate_limited(&mut self, identifier: &str) -> bool {
        let now = Utc::now();
        let state = self
            .state
            .entry(identifier.to_string())
            .or_insert_with(|| RateLimiterState {
                requests: 0,
                last_reset: now,
            });

        if now.signed_duration_since(state.last_reset)
            >= chrono::Duration::from_std(self.config.window).unwrap_or_default()
        {
            state.requests = 0;
            state.last_reset = now;
        }

        state.requests += 1;
        state.requests > self.config.requests_per_minute
    }
}

impl Default for SecurityRules {
    fn default() -> Self {
        Self {
            rules: Vec::new(),
            default_policy: PolicyDecision::Deny,
            context: SecurityContext::default(),
        }
    }
}

impl SessionStore {
    /// Creates a new session store with the given configuration.
    pub fn new(config: AuthConfig) -> Self {
        Self {
            sessions: Arc::new(RwLock::new(HashMap::with_capacity(beardog_types::constants::domains::system::defaults::DEFAULT_QUEUE_SIZE))),
            config,
        }
    }

    /// Creates a new authenticated session, returning the session ID.
    pub async fn create_session(
        &self,
        user_id: &str,
        ip_address: &str,
        user_agent: &str,
    ) -> String {
        let session_id = uuid::Uuid::new_v4().to_string();
        let session_data = SessionData {
            user_id: user_id.to_string(),
            created_at: Utc::now(),
            last_accessed: Utc::now(),
            ip_address: ip_address.to_string(),
            user_agent: user_agent.to_string(),
            is_authenticated: true,
        };

        let mut sessions = self.sessions.write().await;
        sessions.insert(session_id.clone(), session_data);
        session_id
    }

    /// Validates a session and returns its data if valid.
    pub async fn validate_session(&self, session_id: &str) -> Option<SessionData> {
        let sessions = self.sessions.read().await;
        sessions.get(session_id).cloned()
    }
}

impl AuditManager {
    /// Creates a new audit manager with the given configuration.
    pub fn new(config: AuditConfig) -> Self {
        Self {
            config,
            events: Arc::new(RwLock::new(Vec::new())),
        }
    }

    /// Logs a security audit event with the given details.
    #[allow(clippy::too_many_arguments)]
    pub async fn log_event(
        &self,
        event_type: &str,
        user_id: Option<&str>,
        details: HashMap<&str, &str>,
        level: AuditLevel,
    ) {
        if !self.config.enabled || level < self.config.log_level {
            return;
        }

        let event = AuditEvent {
            id: uuid::Uuid::new_v4().to_string(),
            event_type: event_type.to_string(),
            user_id: user_id.map(std::string::ToString::to_string),
            timestamp: Utc::now(),
            details: details
                .into_iter()
                .map(|(k, v)| (k.to_string(), v.to_string()))
                .collect(),
            level,
        };

        let mut events = self.events.write().await;
        events.push(event);

        if events.len() > self.config.max_events {
            events.remove(0);
        }
    }
}

impl PartialOrd for AuditLevel {
    fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> {
        let self_val = match self {
            AuditLevel::Debug => 0,
            AuditLevel::Low => 1,
            AuditLevel::Medium => 2,
            AuditLevel::High => 3,
            AuditLevel::Critical => 4,
        };
        let other_val = match other {
            AuditLevel::Debug => 0,
            AuditLevel::Low => 1,
            AuditLevel::Medium => 2,
            AuditLevel::High => 3,
            AuditLevel::Critical => 4,
        };
        self_val.partial_cmp(&other_val)
    }
}
