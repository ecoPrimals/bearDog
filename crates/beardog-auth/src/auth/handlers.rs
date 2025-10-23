// Authentication handlers for BearDog
// Provides secure authentication and session management

use argon2::password_hash::PasswordHash;
use argon2::{Argon2, PasswordVerifier};
use beardog_errors::BearDogError;
use serde::{Deserialize, Serialize};
use std::collections::HashMap;

/// Authentication configuration
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct AuthConfig {
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

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SessionData {
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
    pub fn register_user(
        &mut self,
        user_id: &str,
        password: &str,
        permissions: Vec<String>,
    ) -> Result<(), BearDogError> {
        use argon2::password_hash::{rand_core::OsRng, PasswordHasher, SaltString};

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
        let stored_credential = match self.credential_store.get(username) {
            Some(cred) => cred,
            None => {
                // User not found - perform dummy hash verification to prevent timing attacks
                // This ensures consistent timing whether user exists or not
                let _ = Self::verify_dummy_password(password);
                return Ok(false);
            }
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
    fn verify_dummy_password(_password: &str) -> bool {
        // Use a pre-computed dummy hash to maintain consistent timing
        const DUMMY_HASH: &str = "$argon2id$v=19$m=19456,t=2,p=1$\
            YW5vdGhlcmR1bW15c2FsdA$\
            /Yqo9o6/9ZqJmYqGmZqamw";

        let parsed_hash = PasswordHash::new(DUMMY_HASH)
            .unwrap_or_else(|e| {
                panic!("CRITICAL: Invalid DUMMY_HASH constant in timing attack mitigation - this should never fail: {}", e)
            });
        Argon2::default()
            .verify_password(b"dummy", &parsed_hash)
            .is_ok()
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
    /// Validates session
    pub fn validate_session(&self, token: &str) -> Result<&SessionData, BearDogError> {
        self.active_sessions
            .values()
            .find(|session| session.token == token)
            .ok_or_else(|| BearDogError::security("Invalid session token".to_string()))
    }

    /// Logout and invalidate session
    pub fn logout(&mut self, token: &str) -> Result<(), BearDogError> {
        self.active_sessions
            .retain(|_, session| session.token != token);
        Ok(())
    }

    /// Clean expired sessions
    /// Cleans up `expired_sessions`
    /// Cleans up `expired_sessions`
    pub fn cleanup_expired_sessions(&mut self) {
        let now = chrono::Utc::now();
        self.active_sessions
            .retain(|_, session| session.expires_at > now);
    }

    pub fn reset_login_attempts(&mut self, user_id: &str) -> Result<(), BearDogError> {
        self.login_attempts.remove(user_id);
        Ok(())
    }

    /// Gets `login_attempts`
    /// Gets `login_attempts`
    #[must_use]
    pub fn get_login_attempts(&self, user_id: &str) -> u32 {
        self.login_attempts.get(user_id).copied().unwrap_or(0)
    }

    /// Check if a user is currently locked out
    /// Checks if user locked
    /// Checks if user locked
    #[must_use]
    pub fn is_user_locked(&self, user_id: &str) -> bool {
        let attempts = self.login_attempts.get(user_id).copied().unwrap_or(0);
        attempts >= self.config.max_login_attempts
    }
}
