// SPDX-License-Identifier: AGPL-3.0-only

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

#[cfg(test)]
mod tests {
    use super::*;

    // SessionInfo tests
    #[test]
    fn test_session_info_creation() {
        let now = Utc::now();
        let session = SessionInfo {
            session_id: "sess-123".to_string(),
            user_id: "user-456".to_string(),
            created_at: now,
            expires_at: now + chrono::Duration::hours(24),
            last_activity: now,
            ip_address: "192.168.1.1".to_string(),
            user_agent: "Mozilla/5.0".to_string(),
        };

        assert_eq!(session.session_id, "sess-123");
        assert_eq!(session.user_id, "user-456");
        assert!(session.expires_at > session.created_at);
        assert_eq!(session.ip_address, "192.168.1.1");
    }

    #[test]
    fn test_session_info_expiry() {
        let now = Utc::now();
        let session = SessionInfo {
            session_id: "sess-expired".to_string(),
            user_id: "user-789".to_string(),
            created_at: now - chrono::Duration::hours(48),
            expires_at: now - chrono::Duration::hours(24),
            last_activity: now - chrono::Duration::hours(25),
            ip_address: "10.0.0.1".to_string(),
            user_agent: "Test Agent".to_string(),
        };

        assert!(session.expires_at < now, "Session should be expired");
        assert!(
            session.last_activity < now,
            "Last activity should be in the past"
        );
    }

    // AuthenticationResult tests
    #[test]
    fn test_auth_result_success() {
        let result = AuthenticationResult {
            success: true,
            user_id: Some("user-123".to_string()),
            session_token: Some("token-abc".to_string()),
            error_message: None,
            requires_mfa: false,
        };

        assert!(result.success);
        assert!(result.user_id.is_some());
        assert!(result.session_token.is_some());
        assert!(result.error_message.is_none());
        assert!(!result.requires_mfa);
    }

    #[test]
    fn test_auth_result_failure() {
        let result = AuthenticationResult {
            success: false,
            user_id: None,
            session_token: None,
            error_message: Some("Invalid credentials".to_string()),
            requires_mfa: false,
        };

        assert!(!result.success);
        assert!(result.user_id.is_none());
        assert!(result.session_token.is_none());
        assert_eq!(
            result.error_message,
            Some("Invalid credentials".to_string())
        );
    }

    #[test]
    fn test_auth_result_mfa_required() {
        let result = AuthenticationResult {
            success: false,
            user_id: Some("user-456".to_string()),
            session_token: None,
            error_message: None,
            requires_mfa: true,
        };

        assert!(!result.success);
        assert!(result.requires_mfa);
        assert!(result.user_id.is_some());
        assert!(result.session_token.is_none());
    }

    // JwtConfig tests
    #[test]
    fn test_jwt_config_creation() {
        let config = JwtConfig {
            jwt_secret: "secret-key-123".to_string(),
            expiration_seconds: 3600,
            enable_refresh: true,
            refresh_expiration_seconds: 86400,
        };

        assert_eq!(config.jwt_secret, "secret-key-123");
        assert_eq!(config.expiration_seconds, 3600);
        assert!(config.enable_refresh);
        assert_eq!(config.refresh_expiration_seconds, 86400);
    }

    #[test]
    fn test_jwt_config_no_refresh() {
        let config = JwtConfig {
            jwt_secret: "another-secret".to_string(),
            expiration_seconds: 1800,
            enable_refresh: false,
            refresh_expiration_seconds: 0,
        };

        assert!(!config.enable_refresh);
        assert_eq!(config.expiration_seconds, 1800);
    }

    #[test]
    fn test_jwt_config_long_expiration() {
        let config = JwtConfig {
            jwt_secret: "long-lived-secret".to_string(),
            expiration_seconds: 604_800, // 7 days
            enable_refresh: true,
            refresh_expiration_seconds: 2_592_000, // 30 days
        };

        assert!(config.refresh_expiration_seconds > config.expiration_seconds);
    }

    // SecurityEventType tests
    #[test]
    fn test_security_event_type_variants() {
        let events = [
            SecurityEventType::Login,
            SecurityEventType::Logout,
            SecurityEventType::AuthenticationFailure,
            SecurityEventType::PasswordChange,
            SecurityEventType::DataAccessed,
            SecurityEventType::DataModified,
        ];

        assert_eq!(events.len(), 6);
    }

    #[test]
    fn test_security_event_type_equality() {
        assert_eq!(SecurityEventType::Login, SecurityEventType::Login);
        assert_ne!(SecurityEventType::Login, SecurityEventType::Logout);
    }

    #[test]
    fn test_security_event_type_auth_events() {
        let login = SecurityEventType::Login;
        let auth_failure = SecurityEventType::AuthenticationFailure;
        let logout = SecurityEventType::Logout;

        assert_ne!(login, auth_failure);
        assert_ne!(login, logout);
    }

    #[test]
    fn test_security_event_type_permission_events() {
        let granted = SecurityEventType::PermissionGranted;
        let revoked = SecurityEventType::PermissionRevoked;

        assert_ne!(granted, revoked);
    }

    #[test]
    fn test_security_event_type_key_events() {
        let generated = SecurityEventType::KeyGenerated;
        let rotated = SecurityEventType::KeyRotated;

        assert_ne!(generated, rotated);
    }

    #[test]
    fn test_security_event_type_data_events() {
        let accessed = SecurityEventType::DataAccessed;
        let modified = SecurityEventType::DataModified;

        assert_ne!(accessed, modified);
    }

    // Serialization tests
    #[test]
    fn test_session_info_serialization() {
        let now = Utc::now();
        let session = SessionInfo {
            session_id: "sess-test".to_string(),
            user_id: "user-test".to_string(),
            created_at: now,
            expires_at: now + chrono::Duration::hours(1),
            last_activity: now,
            ip_address: "127.0.0.1".to_string(),
            user_agent: "Test".to_string(),
        };

        let json = serde_json::to_string(&session);
        assert!(json.is_ok(), "Should be able to serialize SessionInfo");
    }

    #[test]
    fn test_auth_result_serialization() {
        let result = AuthenticationResult {
            success: true,
            user_id: Some("user".to_string()),
            session_token: Some("token".to_string()),
            error_message: None,
            requires_mfa: false,
        };

        let json = serde_json::to_string(&result);
        assert!(
            json.is_ok(),
            "Should be able to serialize AuthenticationResult"
        );
    }

    #[test]
    fn test_jwt_config_serialization() {
        let config = JwtConfig {
            jwt_secret: "secret".to_string(),
            expiration_seconds: 3600,
            enable_refresh: true,
            refresh_expiration_seconds: 86400,
        };

        let json = serde_json::to_string(&config);
        assert!(json.is_ok(), "Should be able to serialize JwtConfig");
    }

    #[test]
    fn test_security_event_type_serialization() {
        let event = SecurityEventType::Login;
        let json = serde_json::to_string(&event);
        assert!(
            json.is_ok(),
            "Should be able to serialize SecurityEventType"
        );
    }
}
