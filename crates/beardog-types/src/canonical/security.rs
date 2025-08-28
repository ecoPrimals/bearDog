use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};
use std::collections::HashMap;

#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct SecurityContext {
    pub user_id: String,
    pub permissions: Vec<String>,
    pub roles: Vec<String>,
    pub session_id: Option<String>,
    pub authenticated: bool,
    pub authorization_level: AuthorizationLevel,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
pub enum AuthorizationLevel {
    None,
    Read,
    Write,
    Admin,
    Root,
}

impl Default for AuthorizationLevel {
    fn default() -> Self {
        Self::None
    }
}

#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct SecurityFlags {
    pub encrypted: bool,
    pub authenticated: bool,
    pub authorized: bool,
    pub audited: bool,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
pub enum PolicyDecision {
    Allow,
    Deny,
    Conditional,
}

impl Default for PolicyDecision {
    fn default() -> Self {
        Self::Deny
    }
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
pub enum RiskLevel {
    Low,
    Medium,
    High,
    Critical,
}

impl Default for RiskLevel {
    fn default() -> Self {
        Self::Medium
    }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SecurityAuditEvent {
    pub event_id: String,
    pub timestamp: DateTime<Utc>,
    pub event_type: SecurityEventType,
    pub user_id: Option<String>,
    pub resource: Option<String>,
    pub action: String,
    pub result: SecurityEventResult,
    pub details: HashMap<String, String>,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
pub enum SecurityEventType {
    Authentication,
    Authorization,
    DataAccess,
    ConfigurationChange,
    SystemEvent,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
pub enum SecurityEventResult {
    Success,
    Failure,
    Warning,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
pub enum ThreatLevel {
    None,
    Low,
    Medium,
    High,
    Critical,
}

impl Default for ThreatLevel {
    fn default() -> Self {
        Self::None
    }
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
pub enum ComplianceLevel {
    NonCompliant,
    PartiallyCompliant,
    Compliant,
    FullyCompliant,
}

impl Default for ComplianceLevel {
    fn default() -> Self {
        Self::NonCompliant
    }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SessionToken {
    pub token: String,
    pub user_id: String,
    pub created_at: DateTime<Utc>,
    pub expires_at: DateTime<Utc>,
    pub permissions: Vec<String>,
    pub active: bool,
}

impl Default for SessionToken {
    fn default() -> Self {
        let now = Utc::now();
        Self {
            token: String::with_capacity(64),
            user_id: String::with_capacity(64),
            created_at: now,
            expires_at: now + chrono::Duration::hours(24),
            permissions: Vec::new(),
            active: false,
        }
    }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Session {
    pub session_id: String,
    pub user_id: String,
    pub created_at: DateTime<Utc>,
    pub expires_at: DateTime<Utc>,
    pub last_activity: DateTime<Utc>,
    pub active: bool,
    pub mfa_verified: bool,
}

impl Default for Session {
    fn default() -> Self {
        let now = Utc::now();
        Self {
            session_id: String::with_capacity(64),
            user_id: String::with_capacity(64),
            created_at: now,
            expires_at: now + chrono::Duration::hours(24),
            last_activity: now,
            active: false,
            mfa_verified: false,
        }
    }
}
