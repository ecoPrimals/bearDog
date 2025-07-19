//! Authentication and Authorization Types
//!
//! This module contains all types related to user authentication, authorization,
//! multi-factor authentication, and session management.

use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};
use std::collections::HashMap;

/// Security subject (user or system) performing actions
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq, Hash)]
pub struct Subject {
    /// Unique identifier for the subject
    pub id: String,
    /// Display name or username
    pub name: String,
    /// Type of subject (User, Service, etc.)
    pub subject_type: SubjectType,
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
    /// Administrative account
    Admin,
}

/// Authorization result containing decision and context
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct AuthorizationResult {
    /// Whether the authorization was granted
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

/// Authentication result with user information
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct AuthenticationResult {
    /// Whether authentication was successful
    pub authenticated: bool,
    /// User information if authentication succeeded
    pub user: Option<UserInfo>,
    /// Error message if authentication failed
    pub error: Option<String>,
    /// Session token if authentication succeeded
    pub session_token: Option<String>,
    /// MFA required status
    pub mfa_required: bool,
    /// Available MFA methods
    pub mfa_methods: Vec<MfaMethod>,
}

/// User information structure
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct UserInfo {
    /// Unique user identifier
    pub id: String,
    /// Username
    pub username: String,
    /// User's email address
    pub email: String,
    /// User's full name
    pub full_name: String,
    /// User roles
    pub roles: Vec<String>,
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
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
pub enum MfaMethod {
    /// Time-based One-Time Password (TOTP)
    Totp,
    /// SMS-based verification
    Sms,
    /// Email-based verification  
    Email,
    /// Hardware security key (WebAuthn/FIDO2)
    HardwareKey,
    /// Backup codes
    BackupCodes,
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

/// Session management configuration
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SessionConfig {
    /// Session timeout in minutes
    pub timeout_minutes: u32,
    /// Whether to extend session on activity
    pub extend_on_activity: bool,
    /// Maximum session duration in hours
    pub max_duration_hours: u32,
}

/// Security session information
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SecuritySession {
    /// Session identifier
    pub session_id: String,
    /// User associated with this session
    pub user: UserInfo,
    /// When the session was created
    pub created_at: DateTime<Utc>,
    /// When the session expires
    pub expires_at: DateTime<Utc>,
    /// IP address of the client
    pub client_ip: String,
    /// User agent string
    pub user_agent: String,
    /// Session metadata
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
            timeout_minutes: 30,
            extend_on_activity: true,
            max_duration_hours: 8,
        }
    }
} 