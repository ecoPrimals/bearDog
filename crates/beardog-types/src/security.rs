// Security types and authentication structures
//
// This module provides comprehensive security types for authentication, authorization,
// session management, and security event tracking. All security operations maintain
// sovereignty compliance and zero hardcoded assumptions.

use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};

///
/// including security metadata and activity tracking.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SessionInfo {
    /// Unique session identifier
    pub session_id: String,
    /// User identifier associated with this session
    pub user_id: String,
    /// Timestamp when session was created
    /// The created at value
    pub created_at: DateTime<Utc>,
    /// Timestamp when session expires
    /// The expires at value
    pub expires_at: DateTime<Utc>,
    /// Timestamp of last activity in this session
    /// The last activity value
    pub last_activity: DateTime<Utc>,
    /// IP address from which session was created
    /// The ip address value
    pub ip_address: String,
    /// User agent string from the client
    /// The user agent value
    pub user_agent: String,
}

/// Authentication attempt result
///
/// Represents the outcome of an authentication attempt including
/// success/failure status and relevant metadata.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct AuthenticationResult {
    /// Whether authentication was successful
    /// Whether success is enabled
    pub success: bool,
    /// User identifier if authentication succeeded
    pub user_id: Option<String>,
    /// Session token if authentication succeeded
    /// Optional session token
    pub session_token: Option<String>,
    /// Error message if authentication failed
    /// Optional error message
    pub error_message: Option<String>,
    /// Whether multi-factor authentication is required
    /// Whether `requires_mfa` is enabled
    pub requires_mfa: bool,
}

/// JWT token configuration
///
/// including expiration times and security settings.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct JwtConfig {
    /// The jwt secret value
    pub jwt_secret: String,
    /// Token expiration time in seconds
    /// Number of `expiration_seconds`
    pub expiration_seconds: u64,
    /// Whether to enable refresh tokens
    /// Whether `enable_refresh` is enabled
    pub enable_refresh: bool,
    /// Refresh token expiration time in seconds
    /// Number of `refresh_expiration_seconds`
    pub refresh_expiration_seconds: u64,
}

/// Security audit event types
///
/// and compliance monitoring.
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
/// Types of security event
pub enum SecurityEventType {
    /// User login event
    Login,
    /// User logout event
    Logout,
    /// Failed authentication attempt
    AuthenticationFailure,
    /// Password change event
    PasswordChange,
    /// Permission granted to user
    PermissionGranted,
    /// Permission revoked from user
    PermissionRevoked,
    /// Cryptographic key generated
    KeyGenerated,
    /// Cryptographic key rotated
    KeyRotated,
    /// Sensitive data accessed
    DataAccessed,
    /// Sensitive data modified
    DataModified,
}
