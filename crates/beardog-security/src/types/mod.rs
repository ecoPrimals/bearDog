// SPDX-License-Identifier: AGPL-3.0-or-later

// Module documentation
//
// This module provides functionality for the BearDog ecosystem.


use chrono::{DateTime, Utc};
use std::collections::HashMap;
use std::sync::Arc;
use tokio::sync::RwLock;

// Re-export commonly used types explicitly from canonical location
pub use beardog_types::canonical::config::AuthConfig;

#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
pub struct SecurityProviderConfig {
    /// Number of max_failed_attempts
    pub max_failed_attempts: u32,
    /// Number of lockout_duration_minutes
    pub lockout_duration_minutes: u32,
    pub session_timeout_minutes: u32,
    /// Whether enable_audit_logging is enabled
    pub enable_audit_logging: bool,
    /// Whether require_mfa is enabled
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

use beardog_types::canonical::config::domains::network::RateLimitConfig;

#[derive(Debug, Clone)]
pub struct BearDogSecurityProvider {
    pub config: SecurityProviderConfig,
    /// The rate limiter value
    pub rate_limiter: RateLimiter,
    /// The security rules value
    pub security_rules: SecurityRules,
    /// Number of locked_acitemss
    pub locked_accounts: Arc<RwLock<HashMap<String, DateTime<Utc>>>>,
    /// The failed attempts value
    pub failed_attempts: Arc<RwLock<HashMap<String, u32>>>,
    /// The metrics value
    pub metrics: SecurityProviderMetrics,
    /// The session store value
    pub session_store: SessionStore,
    /// The audit manager value
    pub audit_manager: AuditManager,
}

#[derive(Debug, Clone)]
pub struct RateLimiter {
    pub config: RateLimitConfig,
    /// Mapping of state
    pub state: HashMap<String, RateLimiterState>,
}

#[derive(Debug, Clone)]
pub struct RateLimiterState {
    /// Number of requests
    pub requests: u32,
    /// The last reset value
    pub last_reset: DateTime<Utc>,
}

#[derive(Debug, Clone)]
pub struct SecurityRules {
    /// Collection of rules
    pub rules: Vec<SecurityRule>,
    /// The default policy value
    pub default_policy: PolicyDecision,
    /// The context value
    pub context: SecurityContext,
}

#[derive(Debug, Clone)]
pub struct SecurityRule {
    /// Name of the item
    pub name: String,
    /// The condition value
    pub condition: String,
    /// The action value
    pub action: PolicyDecision,
}

#[derive(Debug, Clone)]
pub struct SecurityContext {
    pub user_id: Option<String>,
    /// Optional ip address
    pub ip_address: Option<String>,
    /// Optional user agent
    pub user_agent: Option<String>,
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
    /// Number of successful_authentications
    pub successful_authentications: u64,
    /// Number of failed_authentications
    pub failed_authentications: u64,
    /// Number of blocked_requests
    pub blocked_requests: u64,
    /// Number of active_sessions
    pub active_sessions: u64,
    /// Number of security_events
    pub security_events: u64,
}

#[derive(Debug, Clone)]
pub struct SessionStore {
    /// The sessions value
    pub sessions: Arc<RwLock<HashMap<String, SessionData>>>,
    pub config: AuthConfig,
}

#[derive(Debug, Clone)]
pub struct SessionData {
    pub user_id: String,
    /// The created at value
    pub created_at: DateTime<Utc>,
    /// The last accessed value
    pub last_accessed: DateTime<Utc>,
    /// The ip address value
    pub ip_address: String,
    /// The user agent value
    pub user_agent: String,
    /// Whether is_authenticated is enabled
    pub is_authenticated: bool,
}

#[derive(Debug, Clone)]
pub struct AuditManager {
    pub config: AuditConfig,
    /// The events value
    pub events: Arc<RwLock<Vec<AuditEvent>>>,
}

#[derive(Debug, Clone)]
pub struct AuditConfig {
    /// Whether feature is enabled
    pub enabled: bool,
    /// The log level value
    pub log_level: AuditLevel,
    /// Number of retention_days
    pub retention_days: u32,
    /// Number of max_events
    pub max_events: usize,
}

#[derive(Debug, Clone)]
pub struct AuditEvent {
    pub id: String,
    /// The event type value
    pub event_type: String,
    pub user_id: Option<String>,
    pub timestamp: DateTime<Utc>,
    /// Mapping of details
    pub details: HashMap<String, String>,
    /// The level value
    pub level: AuditLevel,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum AuditLevel {
    /// Represents debug variant
    Debug,
    /// Represents low variant
    Low,
    /// Represents medium variant
    Medium,
    /// Represents high variant
    High,
    /// Represents critical variant
    Critical,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum PolicyDecision {
    /// Represents allow variant
    Allow,
    /// Represents deny variant
    Deny,
    /// Represents challenge variant
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
    /// New operation.
    /// Creates a new instance
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

    /// Is Account Locked operation.
    /// Checks if account locked
    pub fn is_account_locked(&self, user_id: &str) -> bool {
        let locked_accounts = self.locked_accounts.read();
        if let Some(locked_until) = locked_accounts.get(user_id) {
            Utc::now() < *locked_until
        } else {
            false
        }
    }

    /// Record Failed Attempt operation.
    pub fn record_failed_attempt(&mut self, user_id: &str) {
        let mut failed_attempts = self.failed_attempts.write();
        let attempts = failed_attempts.entry(user_id.to_string()).or_insert(0);
        *attempts += 1;

        if *attempts >= self.config.max_failed_attempts {
            let lockout_duration =
                chrono::Duration::minutes(self.config.lockout_duration_minutes as i64);
            let locked_until = Utc::now() + lockout_duration;

            let mut locked_accounts = self.locked_accounts.write();
            locked_accounts.insert(user_id.to_string(), locked_until);

            failed_attempts.remove(user_id);
        }
    }

    /// Record Successful Auth operation.
    pub fn record_successful_auth(&mut self, user_id: &str) {
        let mut failed_attempts = self.failed_attempts.write();
        failed_attempts.remove(user_id);

        self.metrics.successful_authentications += 1;
    }
}

impl RateLimiter {
    /// New operation.
    /// Creates a new instance
    pub fn new(config: RateLimitConfig) -> Self {
        Self {
            config,
            state: HashMap::with_capacity(16),
        }
    }

    /// Is Rate Limited operation.
    /// Checks if rate limited
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
    /// New operation.
    /// Creates a new instance
    pub fn new(config: AuthConfig) -> Self {
        Self {
            sessions: Arc::new(RwLock::new(HashMap::with_capacity(1000))),
            config,
        }
    }

    /// Create Session operation.
    /// Creates session
    pub fn create_session(
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

        let mut sessions = self.sessions.write();
        sessions.insert(session_id.clone(), session_data);
        session_id
    }

    /// Validate Session operation.
    /// Validates session
    pub fn validate_session(&self, session_id: &str) -> Option<SessionData> {
        let sessions = self.sessions.read();
        sessions.get(session_id).cloned()
    }
}

impl AuditManager {
    /// New operation.
    /// Creates a new instance
    pub fn new(config: AuditConfig) -> Self {
        Self {
            config,
            events: Arc::new(RwLock::new(Vec::new())),
        }
    }

    #[expect(
        clippy::too_many_arguments,
        reason = "Audit log_event mirrors structured audit record fields"
    )]
    /// Log Event operation.
    pub fn log_event(
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

        let mut events = self.events.write();
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
