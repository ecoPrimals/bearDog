use chrono::{DateTime, Utc};
use std::collections::HashMap;
use std::sync::Arc;
use tokio::sync::RwLock;

pub use beardog_types::canonical::configuration::consolidated::{AuthConfig, SecurityConfig};
pub use beardog_types::canonical::security::*;

#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
pub struct SecurityProviderConfig {
    pub max_failed_attempts: u32,
    pub lockout_duration_minutes: u32,
    pub session_timeout_minutes: u32,
    pub enable_audit_logging: bool,
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

pub use beardog_types::canonical::configuration::consolidated::RateLimitConfig;

#[derive(Debug, Clone)]
pub struct BearDogSecurityProvider {
    pub config: SecurityProviderConfig,
    pub rate_limiter: RateLimiter,
    pub security_rules: SecurityRules,
    pub locked_accounts: Arc<RwLock<HashMap<String, DateTime<Utc>>>>,
    pub failed_attempts: Arc<RwLock<HashMap<String, u32>>>,
    pub metrics: SecurityProviderMetrics,
    pub session_store: SessionStore,
    pub audit_manager: AuditManager,
}

#[derive(Debug, Clone)]
pub struct RateLimiter {
    pub config: RateLimitConfig,
    pub state: HashMap<String, RateLimiterState>,
}

#[derive(Debug, Clone)]
pub struct RateLimiterState {
    pub requests: u32,
    pub last_reset: DateTime<Utc>,
}

#[derive(Debug, Clone)]
pub struct SecurityRules {
    pub rules: Vec<SecurityRule>,
    pub default_policy: PolicyDecision,
    pub context: SecurityContext,
}

#[derive(Debug, Clone)]
pub struct SecurityRule {
    pub id: String,
    pub condition: String,
    pub action: PolicyDecision,
}

#[derive(Debug, Clone, PartialEq)]
pub enum PolicyDecision {
    Allow,
    Deny,
    RequireAdditionalAuth,
}

#[derive(Debug, Clone)]
pub struct SecurityContext {
    pub user_id: Option<String>,
    pub ip_address: Option<String>,
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

#[derive(Debug, Clone, Default)]
pub struct SecurityProviderMetrics {
    pub successful_authentications: u64,
    pub failed_authentications: u64,
    pub blocked_requests: u64,
    pub active_sessions: u64,
    pub security_events: u64,
}

#[derive(Debug, Clone)]
pub struct SessionStore {
    pub sessions: Arc<RwLock<HashMap<String, SessionData>>>,
    pub config: AuthConfig,
}

#[derive(Debug, Clone)]
pub struct SessionData {
    pub user_id: String,
    pub created_at: DateTime<Utc>,
    pub last_accessed: DateTime<Utc>,
    pub ip_address: String,
    pub user_agent: String,
    pub is_authenticated: bool,
}

// SessionConfig removed - use AuthConfig instead

#[derive(Debug, Clone)]
pub struct AuditManager {
    pub config: AuditConfig,
    pub events: Arc<RwLock<Vec<AuditEvent>>>,
}

#[derive(Debug, Clone)]
pub struct AuditConfig {
    pub enabled: bool,
    pub log_level: AuditLevel,
    pub retention_days: u32,
    pub max_events: usize,
}

#[derive(Debug, Clone, PartialEq)]
pub enum AuditLevel {
    Critical,
    High,
    Medium,
    Low,
    Debug,
}

#[derive(Debug, Clone)]
pub struct AuditEvent {
    pub id: String,
    pub event_type: String,
    pub user_id: Option<String>,
    pub timestamp: DateTime<Utc>,
    pub details: HashMap<String, String>,
    pub level: AuditLevel,
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

    pub async fn is_account_locked(&self, user_id: &str) -> bool {
        let locked_accounts = self.locked_accounts.read().await;
        if let Some(locked_until) = locked_accounts.get(user_id) {
            Utc::now() < *locked_until
        } else {
            false
        }
    }

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

    pub async fn record_successful_auth(&mut self, user_id: &str) {
        let mut failed_attempts = self.failed_attempts.write().await;
        failed_attempts.remove(user_id);

        self.metrics.successful_authentications += 1;
    }
}

impl RateLimiter {
    pub fn new(config: RateLimitConfig) -> Self {
        Self {
            config,
            state: HashMap::with_capacity(16),
        }
    }

    pub fn is_rate_limited(&mut self, identifier: &str) -> bool {
        let now = Utc::now();
        let state = self
            .state
            .entry(identifier.to_string())
            .or_insert(RateLimiterState {
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
    pub fn new(config: AuthConfig) -> Self {
        Self {
            sessions: Arc::new(RwLock::new(HashMap::with_capacity(16))),
            config,
        }
    }

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

    pub async fn validate_session(&self, session_id: &str) -> Option<SessionData> {
        let sessions = self.sessions.read().await;
        sessions.get(session_id).cloned()
    }
}

impl AuditManager {
    pub fn new(config: AuditConfig) -> Self {
        Self {
            config,
            events: Arc::new(RwLock::new(Vec::new())),
        }
    }

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
            user_id: user_id.map(|s| s.to_string()),
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
