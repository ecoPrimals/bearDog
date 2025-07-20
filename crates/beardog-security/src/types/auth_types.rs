//! Authentication and Authorization Types
//!
//! This module contains all types related to user authentication, authorization,
//! multi-factor authentication, and session management.

use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};
use std::collections::HashMap;

/// Security subject (user or system) performing actions
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
pub struct Subject {
    /// Unique identifier for the subject
    pub id: String,
    /// Display name or username
    pub name: String,
    /// Type of subject (User, Service, etc.)
    pub subject_type: SubjectType,
    /// Subject roles
    pub roles: Vec<String>,
    /// Security clearance level
    pub clearance_level: Option<u32>,
    /// Additional metadata about the subject
    pub metadata: HashMap<String, String>,
}

/// Types of security subjects
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq, Hash)]
pub enum SubjectType {
    /// Human user
    User,
    /// Service account or system
    Service,
    /// System account
    System,
    /// Device account
    Device,
    /// Administrative account
    Admin,
}

/// Authorization result containing decision and context
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct AuthorizationResult {
    /// Whether the authorization was granted
    pub permitted: bool,
    /// Whether user is authorized (for compatibility)
    pub authorized: bool,
    /// Reason for the authorization decision
    pub reason: String,
    /// Risk level assessed for this authorization
    pub risk_level: super::RiskLevel,
    /// Additional requirements that must be met
    pub additional_requirements: Vec<String>,
    /// When this authorization expires
    pub expires_at: Option<DateTime<Utc>>,
    /// Unique identifier for audit trail
    pub audit_id: String,
}

/// Authentication result from security provider
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct AuthenticationResult {
    /// Whether authentication was successful
    pub success: bool,
    /// Whether user is authenticated (for compatibility)
    pub authenticated: bool,
    /// Authenticated user ID
    pub user_id: Option<String>,
    /// User information (for compatibility)
    pub user: Option<UserInfo>,
    /// Session ID if authentication successful
    pub session_id: Option<String>,
    /// Session token if created
    pub session_token: Option<SessionToken>,
    /// Whether MFA is required
    pub mfa_required: bool,
    /// Available MFA methods
    pub mfa_methods: Vec<MfaMethod>,
    /// Reason for authentication result
    pub reason: String,
    /// Error message if authentication failed
    pub error: Option<String>,
    /// When the authentication expires
    pub expires_at: Option<DateTime<Utc>>,
}

/// User information structure (updated with needed fields)
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct UserInfo {
    /// Unique user identifier
    pub id: String,
    /// Unique user identifier (for compatibility)
    pub user_id: String,
    /// Username
    pub username: String,
    /// User's email address
    pub email: String,
    /// User's full name
    pub full_name: String,
    /// User roles
    pub roles: Vec<String>,
    /// User permissions
    pub permissions: Vec<String>,
    /// Account status
    pub status: AccountStatus,
    /// Last login timestamp
    pub last_login: Option<DateTime<Utc>>,
}

/// Account status for user accounts
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
pub enum AccountStatus {
    /// Account is active and can be used
    Active,
    /// Account is suspended temporarily
    Suspended,
    /// Account is locked due to security concerns
    Locked,
    /// Account has been disabled
    Disabled,
    /// Account is pending activation
    Pending,
}

/// Multi-factor authentication configuration
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct MfaConfig {
    /// Whether MFA is enabled
    pub enabled: bool,
    /// Required MFA methods
    pub required_methods: Vec<MfaMethod>,
    /// Grace period for MFA setup (in hours)
    pub setup_grace_period_hours: u32,
}

/// Multi-factor authentication method types
#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum MfaMethod {
    /// Time-based One-Time Password (TOTP)
    Totp,
    /// SMS-based token
    Sms,
    /// Email-based token
    Email,
}

/// MFA token information
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct MfaToken {
    /// Token identifier
    pub id: String,
    /// MFA method used
    pub method: MfaMethod,
    /// Token value (encrypted)
    pub token: String,
    /// When the token expires
    pub expires_at: DateTime<Utc>,
    /// Whether the token has been used
    pub used: bool,
    /// User ID associated with this token
    pub user_id: String,
}

/// Session configuration for session creation
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SessionConfig {
    /// Maximum session age in seconds
    pub max_age_seconds: u64,
    /// Whether MFA is required for this session type
    pub require_mfa: bool,
    /// Whether to bind session to IP address
    pub ip_binding: bool,
    /// Maximum concurrent sessions allowed
    pub concurrent_sessions: u32,
}

/// Session token returned after session creation
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SessionToken {
    /// Unique session identifier
    pub session_id: String,
    /// Token value (could be JWT or random string)
    pub token: String,
    /// Token expiration time
    pub expires_at: chrono::DateTime<Utc>,
    /// Token type (e.g., "Bearer")
    pub token_type: String,
}

/// User session information
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Session {
    /// Session identifier
    pub session_id: String,
    /// Associated user ID
    pub user_id: String,
    /// User information
    pub user_info: UserInfo,
    /// Session creation time
    pub created_at: DateTime<Utc>,
    /// Session expiration time
    pub expires_at: DateTime<Utc>,
    /// Whether the session is active
    pub is_active: bool,
    /// Last activity timestamp
    pub last_activity: DateTime<Utc>,
    /// User permissions for this session
    pub permissions: Vec<String>,
    /// Additional session metadata
    pub metadata: HashMap<String, String>,
}

/// Security session (compatibility with existing interfaces)
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SecuritySession {
    /// Session identifier
    pub id: String,
    /// User identifier
    pub user_id: String,
    /// Whether session is active
    pub is_active: bool,
    /// Session creation time
    pub created_at: DateTime<Utc>,
    /// Session expiration time
    pub expires_at: DateTime<Utc>,
    /// Client IP address
    pub client_ip: Option<String>,
    /// User agent string
    pub user_agent: Option<String>,
    /// User permissions
    pub permissions: Vec<String>,
}

/// MFA token entry for internal storage
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct MfaTokenEntry {
    pub token: String,
    pub user_id: String,
    pub method: MfaMethod,
    pub created_at: chrono::DateTime<chrono::Utc>,
    pub expires_at: chrono::DateTime<chrono::Utc>,
    pub attempts: u32,
    pub max_attempts: u32,
}

/// Session storage for managing active sessions
#[derive(Debug, Default, Clone, Serialize, Deserialize)]
pub struct SessionStore {
    /// Active sessions
    sessions: HashMap<String, Session>,
    /// MFA tokens
    pub mfa_tokens: HashMap<String, MfaTokenEntry>,
}

impl SessionStore {
    pub fn new() -> Self {
        Self::default()
    }

    pub fn insert(&mut self, session_id: String, session: Session) {
        self.sessions.insert(session_id, session);
    }

    pub fn get(&self, session_id: &str) -> Option<&Session> {
        self.sessions.get(session_id)
    }

    pub fn get_mut(&mut self, session_id: &str) -> Option<&mut Session> {
        self.sessions.get_mut(session_id)
    }

    pub fn remove(&mut self, session_id: &str) -> Option<Session> {
        self.sessions.remove(session_id)
    }

    pub fn len(&self) -> usize {
        self.sessions.len()
    }

    pub fn is_empty(&self) -> bool {
        self.sessions.is_empty()
    }

    pub fn capacity(&self) -> usize {
        self.sessions.capacity()
    }

    pub fn shrink_to_fit(&mut self) {
        self.sessions.shrink_to_fit();
    }

    pub fn retain<F>(&mut self, f: F)
    where
        F: FnMut(&String, &mut Session) -> bool,
    {
        self.sessions.retain(f);
    }

    pub fn iter(&self) -> impl Iterator<Item = (&String, &Session)> {
        self.sessions.iter()
    }

    pub fn values(&self) -> impl Iterator<Item = &Session> {
        self.sessions.values()
    }
}

/// Security context for authorization decisions
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct SecurityContext {
    /// Current timestamp
    pub timestamp: Option<DateTime<Utc>>,
    /// Client IP address
    pub client_ip: Option<String>,
    /// User agent
    pub user_agent: Option<String>,
    /// Request method
    pub method: Option<String>,
    /// Request path
    pub path: Option<String>,
    /// Additional context metadata
    pub metadata: HashMap<String, String>,
}

impl Default for MfaConfig {
    fn default() -> Self {
        Self {
            enabled: false,
            required_methods: vec![MfaMethod::Totp],
            setup_grace_period_hours: 24,
        }
    }
}

impl Default for SessionConfig {
    fn default() -> Self {
        Self {
            max_age_seconds: 3600, // 1 hour
            require_mfa: false,
            ip_binding: false,
            concurrent_sessions: 5,
        }
    }
}
