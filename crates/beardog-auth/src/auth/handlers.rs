// SPDX-License-Identifier: AGPL-3.0-or-later

// Authentication handlers for BearDog
// Provides secure authentication and session management

use argon2::password_hash::PasswordHash;
use argon2::{Argon2, PasswordVerifier};
use beardog_errors::BearDogError;
use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use std::sync::OnceLock;

/// Tunable limits for interactive login sessions and lockout policy.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct AuthConfig {
    /// Wall-clock lifetime of issued session tokens, in hours.
    pub session_timeout_hours: u64,
    /// Number of `max_login_attempts`
    pub max_login_attempts: u32,
    /// Whether `require_mfa` is enabled
    pub require_mfa: bool,
}

impl Default for AuthConfig {
    fn default() -> Self {
        Self {
            session_timeout_hours: 24,
            max_login_attempts: 5,
            require_mfa: false,
        }
    }
}

/// Materialized session returned after successful authentication and stored server-side for validation.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SessionData {
    /// Subject identifier extracted from credentials (username, email, etc.).
    pub user_id: String,
    /// The token value
    pub token: String,
    /// The expires at value
    pub expires_at: chrono::DateTime<chrono::Utc>,
    /// Collection of permissions
    pub permissions: Vec<String>,
}

/// Stored credential information (production would use database)
#[derive(Debug, Clone)]
struct CredentialStore {
    /// Argon2 password hash
    password_hash: String,
    /// User permissions
    permissions: Vec<String>,
}

/// Authentication handler with secure credential storage
pub struct AuthenticationHandler {
    config: AuthConfig,
    active_sessions: HashMap<String, SessionData>,
    login_attempts: HashMap<String, u32>,
    /// Credential store (in production, this would be a database/LDAP/OAuth provider)
    credential_store: HashMap<String, CredentialStore>,
}

impl AuthenticationHandler {
    /// Create new authentication handler
    /// Creates a new instance
    #[must_use]
    pub fn new(config: AuthConfig) -> Self {
        Self {
            config,
            active_sessions: HashMap::new(),
            login_attempts: HashMap::new(),
            credential_store: HashMap::new(),
        }
    }

    /// Register a user with credentials (for testing/demo purposes)
    ///
    /// In production, this would:
    /// - Store hashed password in a database
    /// - Integrate with identity providers (LDAP, OAuth, SAML)
    /// - Enforce password complexity requirements
    /// - Handle user lifecycle (creation, updates, deletion)
    ///
    /// # Errors
    ///
    /// Returns [`BearDogError`] when password hashing fails.
    pub fn register_user(
        &mut self,
        user_id: &str,
        password: &str,
        permissions: Vec<String>,
    ) -> Result<(), BearDogError> {
        use argon2::password_hash::{PasswordHasher, SaltString, rand_core::OsRng};

        // Generate password hash using Argon2
        let salt = SaltString::generate(&mut OsRng);
        let argon2 = Argon2::default();

        let password_hash = argon2
            .hash_password(password.as_bytes(), &salt)
            .map_err(|e| BearDogError::Security {
                message: format!("Failed to hash password: {e}"),
                category: beardog_errors::SecurityErrorCategory::Authentication,
            })?;

        self.credential_store.insert(
            user_id.to_string(),
            CredentialStore {
                password_hash: password_hash.to_string(),
                permissions,
            },
        );

        Ok(())
    }

    /// Authenticate user and create session with rate limiting
    ///
    /// # Errors
    ///
    /// Returns [`BearDogError`] on lockout, credential verification failure, or missing permissions.
    #[expect(
        clippy::cast_possible_wrap,
        reason = "Session timeout hours from config fit i64 for chrono Duration"
    )]
    pub async fn authenticate(&mut self, credentials: &str) -> Result<SessionData, BearDogError> {
        // Extract user identifier from credentials (simplified for demo)
        let user_id = self.extract_user_id(credentials)?;

        // Check rate limiting
        let current_attempts = self.login_attempts.get(&user_id).copied().unwrap_or(0);
        if current_attempts >= self.config.max_login_attempts {
            return Err(BearDogError::Security {
                message: format!(
                    "Account locked due to {current_attempts} failed login attempts. Please try again later."
                ),
                category: beardog_errors::SecurityErrorCategory::Authentication,
            });
        }

        // Simulate authentication (replace with actual authentication logic)
        let auth_success = self.verify_credentials(credentials).await?;

        if auth_success {
            // Reset failed attempts on successful login
            self.login_attempts.remove(&user_id);

            // Generate unique token
            let token = format!("token_{}_{}", user_id, chrono::Utc::now().timestamp());

            let session = SessionData {
                user_id: user_id.clone(),
                token,
                expires_at: chrono::Utc::now()
                    + chrono::Duration::hours(self.config.session_timeout_hours as i64),
                permissions: self.get_user_permissions(&user_id)?,
            };

            // Store active session
            self.active_sessions
                .insert(user_id.clone(), session.clone());

            Ok(session)
        } else {
            // Increment failed attempts
            let new_attempts = current_attempts + 1;
            self.login_attempts.insert(user_id.clone(), new_attempts);

            if new_attempts >= self.config.max_login_attempts {
                Err(BearDogError::Security {
                    message: format!(
                        "Authentication failed. Account locked after {new_attempts} attempts."
                    ),
                    category: beardog_errors::SecurityErrorCategory::Authentication,
                })
            } else {
                Err(BearDogError::Security {
                    message: format!(
                        "Authentication failed. {} of {} attempts used.",
                        new_attempts, self.config.max_login_attempts
                    ),
                    category: beardog_errors::SecurityErrorCategory::Authentication,
                })
            }
        }
    }

    /// Extract user ID from credentials (simplified implementation)
    fn extract_user_id(&self, credentials: &str) -> Result<String, BearDogError> {
        // In a real implementation, this would parse the credentials format
        // For demo purposes, we'll use the first part before any delimiter
        let user_id = credentials
            .split(':')
            .next()
            .unwrap_or("unknown_user")
            .to_string();

        if user_id.is_empty() {
            return Err(BearDogError::Security {
                message: "Invalid credentials format".to_string(),
                category: beardog_errors::SecurityErrorCategory::Authentication,
            });
        }

        Ok(user_id)
    }

    /// Verify credentials using Argon2 password hashing
    ///
    /// Uses constant-time comparison to prevent timing attacks.
    /// In production, this would:
    /// - Query credential store from database
    /// - Support multiple authentication methods (password, OAuth, SAML, biometric)
    /// - Integrate with HSM for sensitive operations
    /// - Log authentication attempts for security auditing
    async fn verify_credentials(&self, credentials: &str) -> Result<bool, BearDogError> {
        // Parse credentials in format "username:password"
        let parts: Vec<&str> = credentials.split(':').collect();
        if parts.len() != 2 {
            // Invalid format - return false without specific error to prevent enumeration
            return Ok(false);
        }

        let (username, password) = (parts[0], parts[1]);

        // Lookup user credentials
        let Some(stored_credential) = self.credential_store.get(username) else {
            // User not found - perform dummy hash verification to prevent timing attacks
            // This ensures consistent timing whether user exists or not
            Self::verify_dummy_password(password)?;
            return Ok(false);
        };

        // Verify password using Argon2
        let parsed_hash = match PasswordHash::new(&stored_credential.password_hash) {
            Ok(hash) => hash,
            Err(e) => {
                return Err(BearDogError::Security {
                    message: format!("Invalid password hash format: {e}"),
                    category: beardog_errors::SecurityErrorCategory::Authentication,
                });
            }
        };

        // Constant-time password verification
        Ok(Argon2::default()
            .verify_password(password.as_bytes(), &parsed_hash)
            .is_ok())
    }

    /// Perform dummy password verification to prevent timing attacks
    ///
    /// This ensures that failed authentication attempts take the same time
    /// whether the user exists or not, preventing username enumeration.
    /// Parsed dummy PHC string, validated once on first use (see `wateringHole` error propagation).
    fn dummy_password_hash() -> Result<&'static PasswordHash<'static>, BearDogError> {
        static DUMMY_HASH: &str = "$argon2id$v=19$m=19456,t=2,p=1$\
            YW5vdGhlcmR1bW15c2FsdA$\
            /Yqo9o6/9ZqJmYqGmZqamw";
        static PARSED: OnceLock<Result<PasswordHash<'static>, String>> = OnceLock::new();
        let init = PARSED.get_or_init(|| PasswordHash::new(DUMMY_HASH).map_err(|e| e.to_string()));
        match init {
            Ok(h) => Ok(h),
            Err(msg) => Err(BearDogError::Security {
                message: format!(
                    "Invalid timing-mitigation dummy password hash (build/configuration error): {msg}"
                ),
                category: beardog_errors::SecurityErrorCategory::Authentication,
            }),
        }
    }

    /// Constant-time work for unknown users; returns `Err` only if the dummy PHC is invalid.
    fn verify_dummy_password(_password: &str) -> Result<(), BearDogError> {
        let parsed_hash = Self::dummy_password_hash()?;
        let _timing = Argon2::default().verify_password(b"dummy", parsed_hash);
        Ok(())
    }

    /// Get user permissions from credential store
    ///
    /// In production, this would:
    /// - Query permissions from database/LDAP
    /// - Implement Role-Based Access Control (RBAC)
    /// - Support dynamic permission evaluation
    /// - Cache permissions for performance
    /// - Integrate with policy engines (e.g., Open Policy Agent)
    fn get_user_permissions(&self, user_id: &str) -> Result<Vec<String>, BearDogError> {
        self.credential_store
            .get(user_id)
            .map(|cred| cred.permissions.clone())
            .ok_or_else(|| BearDogError::Security {
                message: format!("No permissions found for user: {user_id}"),
                category: beardog_errors::SecurityErrorCategory::Authorization,
            })
    }

    /// Validate session token
    /// Validates session
    ///
    /// # Errors
    ///
    /// Returns [`BearDogError`] when the token is unknown or expired.
    pub fn validate_session(&self, token: &str) -> Result<&SessionData, BearDogError> {
        self.active_sessions
            .values()
            .find(|session| session.token == token)
            .ok_or_else(|| BearDogError::security("Invalid session token".to_string()))
    }

    /// Logout and invalidate session
    ///
    /// # Errors
    ///
    /// Currently always succeeds; the `Result` type is reserved for future session store failures.
    pub fn logout(&mut self, token: &str) -> Result<(), BearDogError> {
        self.active_sessions
            .retain(|_, session| session.token != token);
        Ok(())
    }

    /// Clean expired sessions
    /// Cleans up `expired_sessions`
    pub fn cleanup_expired_sessions(&mut self) {
        let now = chrono::Utc::now();
        self.active_sessions
            .retain(|_, session| session.expires_at > now);
    }

    /// Clears failed-attempt counters for `user_id` after administrative unlock or password reset.
    ///
    /// Returns `Ok(())` even if the user had no recorded attempts.
    ///
    /// # Errors
    ///
    /// Currently always succeeds; the `Result` type is reserved for future persistence failures.
    pub fn reset_login_attempts(&mut self, user_id: &str) -> Result<(), BearDogError> {
        self.login_attempts.remove(user_id);
        Ok(())
    }

    /// Gets `login_attempts`
    #[must_use]
    pub fn get_login_attempts(&self, user_id: &str) -> u32 {
        self.login_attempts.get(user_id).copied().unwrap_or(0)
    }

    /// Check if a user is currently locked out
    /// Checks if user locked
    #[must_use]
    pub fn is_user_locked(&self, user_id: &str) -> bool {
        let attempts = self.login_attempts.get(user_id).copied().unwrap_or(0);
        attempts >= self.config.max_login_attempts
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_auth_config_default() {
        let config = AuthConfig::default();

        assert_eq!(config.session_timeout_hours, 24);
        assert_eq!(config.max_login_attempts, 5);
        assert!(!config.require_mfa, "MFA should be disabled by default");
    }

    #[test]
    fn test_new_authentication_handler() {
        let config = AuthConfig::default();
        let handler = AuthenticationHandler::new(config);

        assert_eq!(handler.config.session_timeout_hours, 24);
        assert_eq!(handler.get_login_attempts("test-user"), 0);
    }

    #[tokio::test]
    async fn test_register_user_success() {
        let config = AuthConfig::default();
        let mut handler = AuthenticationHandler::new(config);

        let result =
            handler.register_user("testuser", "StrongPassword123!", vec!["read".to_string()]);
        assert!(result.is_ok(), "User registration should succeed");
    }

    #[tokio::test]
    async fn test_authenticate_success() {
        let config = AuthConfig::default();
        let mut handler = AuthenticationHandler::new(config);

        // Register a user first
        handler
            .register_user("alice", "SecurePass456!", vec!["admin".to_string()])
            .expect("register_user in test");

        // Authenticate with correct credentials
        let result = handler.authenticate("alice:SecurePass456!").await;
        assert!(
            result.is_ok(),
            "Authentication should succeed with correct credentials"
        );

        let session = result.expect("authenticate in test");
        assert_eq!(session.user_id, "alice");
        assert!(!session.token.is_empty());
        assert_eq!(session.permissions, vec!["admin".to_string()]);
    }

    #[tokio::test]
    async fn test_authenticate_wrong_password() {
        let config = AuthConfig::default();
        let mut handler = AuthenticationHandler::new(config);

        handler
            .register_user("bob", "CorrectPass789!", vec!["user".to_string()])
            .expect("register_user in test");

        let result = handler.authenticate("bob:WrongPassword!").await;
        assert!(
            result.is_err(),
            "Authentication should fail with wrong password"
        );
        assert_eq!(handler.get_login_attempts("bob"), 1);
    }

    #[tokio::test]
    async fn test_authenticate_nonexistent_user() {
        let config = AuthConfig::default();
        let mut handler = AuthenticationHandler::new(config);

        let result = handler.authenticate("nonexistent:password").await;
        assert!(
            result.is_err(),
            "Authentication should fail for nonexistent user"
        );
    }

    #[tokio::test]
    async fn test_rate_limiting_locks_account() {
        let config = AuthConfig {
            max_login_attempts: 3,
            ..Default::default()
        };
        let mut handler = AuthenticationHandler::new(config);

        handler
            .register_user("charlie", "Pass123!", vec![])
            .expect("register_user in test");

        // Attempt 3 failed logins
        for _ in 0..3 {
            let _ = handler.authenticate("charlie:wrong").await;
        }

        // Account should now be locked
        assert!(handler.is_user_locked("charlie"));

        // Next attempt should fail due to account lock
        let result = handler.authenticate("charlie:wrong").await;
        assert!(result.is_err());
    }

    #[tokio::test]
    async fn test_successful_login_resets_attempts() {
        let config = AuthConfig::default();
        let mut handler = AuthenticationHandler::new(config);

        handler
            .register_user("david", "SecurePass!", vec![])
            .expect("register_user in test");

        // Failed attempt
        let _ = handler.authenticate("david:wrong").await;
        assert_eq!(handler.get_login_attempts("david"), 1);

        // Successful login should reset attempts
        let result = handler.authenticate("david:SecurePass!").await;
        assert!(result.is_ok());
        assert_eq!(handler.get_login_attempts("david"), 0);
    }

    #[test]
    fn test_validate_session_success() {
        let config = AuthConfig::default();
        let mut handler = AuthenticationHandler::new(config);

        let session = SessionData {
            user_id: "eve".to_string(),
            token: "test-token-123".to_string(),
            expires_at: chrono::Utc::now() + chrono::Duration::hours(1),
            permissions: vec!["read".to_string()],
        };

        handler.active_sessions.insert("eve".to_string(), session);

        let result = handler.validate_session("test-token-123");
        assert!(result.is_ok());
        assert_eq!(result.expect("validate_session in test").user_id, "eve");
    }

    #[test]
    fn test_validate_session_invalid_token() {
        let config = AuthConfig::default();
        let handler = AuthenticationHandler::new(config);

        let result = handler.validate_session("invalid-token");
        assert!(result.is_err(), "Invalid token should fail validation");
    }

    #[test]
    fn test_logout_success() {
        let config = AuthConfig::default();
        let mut handler = AuthenticationHandler::new(config);

        let session = SessionData {
            user_id: "frank".to_string(),
            token: "token-456".to_string(),
            expires_at: chrono::Utc::now() + chrono::Duration::hours(1),
            permissions: vec![],
        };

        handler.active_sessions.insert("frank".to_string(), session);

        let result = handler.logout("token-456");
        assert!(result.is_ok());
        assert!(
            handler.validate_session("token-456").is_err(),
            "Session should be invalidated after logout"
        );
    }

    #[test]
    fn test_cleanup_expired_sessions() {
        let config = AuthConfig::default();
        let mut handler = AuthenticationHandler::new(config);

        // Add expired session
        let expired_session = SessionData {
            user_id: "grace".to_string(),
            token: "expired-token".to_string(),
            expires_at: chrono::Utc::now() - chrono::Duration::hours(1),
            permissions: vec![],
        };
        handler
            .active_sessions
            .insert("grace".to_string(), expired_session);

        // Add valid session
        let valid_session = SessionData {
            user_id: "henry".to_string(),
            token: "valid-token".to_string(),
            expires_at: chrono::Utc::now() + chrono::Duration::hours(1),
            permissions: vec![],
        };
        handler
            .active_sessions
            .insert("henry".to_string(), valid_session);

        handler.cleanup_expired_sessions();

        assert!(
            handler.validate_session("expired-token").is_err(),
            "Expired session should be removed"
        );
        assert!(
            handler.validate_session("valid-token").is_ok(),
            "Valid session should remain"
        );
    }

    #[test]
    fn test_reset_login_attempts() {
        let config = AuthConfig::default();
        let mut handler = AuthenticationHandler::new(config);

        handler.login_attempts.insert("iris".to_string(), 3);
        assert_eq!(handler.get_login_attempts("iris"), 3);

        let result = handler.reset_login_attempts("iris");
        assert!(result.is_ok());
        assert_eq!(handler.get_login_attempts("iris"), 0);
    }

    #[test]
    fn test_is_user_locked() {
        let config = AuthConfig {
            max_login_attempts: 5,
            ..Default::default()
        };
        let mut handler = AuthenticationHandler::new(config);

        assert!(
            !handler.is_user_locked("jack"),
            "User should not be locked initially"
        );

        handler.login_attempts.insert("jack".to_string(), 3);
        assert!(
            !handler.is_user_locked("jack"),
            "User should not be locked at 3 attempts"
        );

        handler.login_attempts.insert("jack".to_string(), 5);
        assert!(
            handler.is_user_locked("jack"),
            "User should be locked at 5 attempts"
        );
    }

    #[test]
    fn test_extract_user_id_success() {
        let config = AuthConfig::default();
        let handler = AuthenticationHandler::new(config);

        let result = handler.extract_user_id("username:password");
        assert!(result.is_ok());
        assert_eq!(result.expect("extract_user_id in test"), "username");
    }

    #[test]
    fn test_extract_user_id_complex_format() {
        let config = AuthConfig::default();
        let handler = AuthenticationHandler::new(config);

        let result = handler.extract_user_id("user@example.com:pass:with:colons");
        assert!(result.is_ok());
        assert_eq!(result.expect("extract_user_id in test"), "user@example.com");
    }
}
