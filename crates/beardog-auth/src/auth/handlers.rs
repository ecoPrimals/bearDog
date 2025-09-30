// Authentication handlers for BearDog
// Provides secure authentication and session management

use beardog_errors::BearDogError;
use serde::{Deserialize, Serialize};
use std::collections::HashMap;

/// Authentication configuration
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct AuthConfig {
    pub session_timeout_hours: u64,
    /// Number of max_login_attempts
    pub max_login_attempts: u32,
    /// Whether require_mfa is enabled
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

/// Authentication handler
pub struct AuthenticationHandler {
    config: AuthConfig,
    active_sessions: HashMap<String, SessionData>,
    login_attempts: HashMap<String, u32>,
}

impl AuthenticationHandler {
    /// Create new authentication handler
    /// Creates a new instance
    pub fn new(config: AuthConfig) -> Self {
        Self {
            config,
            active_sessions: HashMap::new(),
            login_attempts: HashMap::new(),
        }
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
                    "Account locked due to {} failed login attempts. Please try again later.",
                    current_attempts
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
                token: token.clone(),
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
                        "Authentication failed. Account locked after {} attempts.",
                        new_attempts
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

    /// Verify credentials (placeholder implementation)
    async fn verify_credentials(&self, credentials: &str) -> Result<bool, BearDogError> {
        // Simulate credential verification with some processing time
        tokio::time::sleep(std::time::Duration::from_millis(10)).await;

        // Simple demo: credentials in format "user:password", valid if password is "valid"
        let parts: Vec<&str> = credentials.split(':').collect();
        if parts.len() != 2 {
            return Ok(false);
        }

        let (_username, password) = (parts[0], parts[1]);
        Ok(password == "valid")
    }

    /// Get user permissions (placeholder implementation)
    /// Gets user_permissions
    fn get_user_permissions(&self, user_id: &str) -> Result<Vec<String>, BearDogError> {
        // In a real implementation, this would query a database or directory service
        match user_id {
            "admin" => Ok(vec![
                "read".to_string(),
                "write".to_string(),
                "admin".to_string(),
            ]),
            "test_user" => Ok(vec!["read".to_string(), "write".to_string()]),
            _ => Ok(vec!["read".to_string()]),
        }
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
    /// Cleans up expired_sessions
    /// Cleans up expired_sessions
    pub fn cleanup_expired_sessions(&mut self) {
        let now = chrono::Utc::now();
        self.active_sessions
            .retain(|_, session| session.expires_at > now);
    }

    pub fn reset_login_attempts(&mut self, user_id: &str) -> Result<(), BearDogError> {
        self.login_attempts.remove(user_id);
        Ok(())
    }

    /// Gets login_attempts
    /// Gets login_attempts
    pub fn get_login_attempts(&self, user_id: &str) -> u32 {
        self.login_attempts.get(user_id).copied().unwrap_or(0)
    }

    /// Check if a user is currently locked out
    /// Checks if user locked
    /// Checks if user locked
    pub fn is_user_locked(&self, user_id: &str) -> bool {
        let attempts = self.login_attempts.get(user_id).copied().unwrap_or(0);
        attempts >= self.config.max_login_attempts
    }
}
