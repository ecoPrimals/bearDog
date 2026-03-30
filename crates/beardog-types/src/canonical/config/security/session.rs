// SPDX-License-Identifier: AGPL-3.0-only

// Session Configuration
//
// Canonical session management configuration.

use beardog_errors::BearDogError;
use serde::{Deserialize, Serialize};

/// **CANONICAL SESSION CONFIGURATION**
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CanonicalSessionConfig {
    /// Session timeout in seconds
    pub timeout_seconds: u64,

    /// Enable secure cookies
    /// Whether `secure_cookies` is enabled
    pub secure_cookies: bool,

    /// Session cookie name
    /// Name of the cookie
    pub cookie_name: String,

    /// Cookie domain
    /// Optional cookie domain
    pub cookie_domain: Option<String>,

    /// Cookie path
    /// The cookie path value
    pub cookie_path: String,

    /// Enable HTTP-only cookies
    /// Whether `http_only` is enabled
    pub http_only: bool,

    /// Cookie `SameSite` policy
    /// The same site value
    pub same_site: String,

    /// Maximum concurrent sessions per user
    /// Number of `max_concurrent_sessions`
    pub max_concurrent_sessions: u32,

    /// Enable session persistence
    /// Whether `enable_persistence` is enabled
    pub enable_persistence: bool,

    /// Session storage backend
    /// The storage backend value
    pub storage_backend: String,
}

impl Default for CanonicalSessionConfig {
    fn default() -> Self {
        Self {
            timeout_seconds: 3600, // 1 hour
            secure_cookies: true,
            cookie_name: "beardog_session".to_string(),
            cookie_domain: None,
            cookie_path: "/".to_string(),
            http_only: true,
            same_site: "Strict".to_string(),
            max_concurrent_sessions: 5,
            enable_persistence: true,
            storage_backend: "redis".to_string(),
        }
    }
}

impl CanonicalSessionConfig {
    /// Production
    #[must_use]
    pub fn production() -> Self {
        Self {
            timeout_seconds: 1800, // 30 minutes for production
            max_concurrent_sessions: 3,
            ..Self::default()
        }
    }

    /// Validate
    /// Validates input
    ///
    /// # Errors
    ///
    /// Returns an error if session timeout is zero or the cookie name is empty.
    pub fn validate(&self) -> Result<(), BearDogError> {
        if self.timeout_seconds == 0 {
            return Err(BearDogError::security(
                "Session timeout must be greater than 0".to_string(),
            ));
        }
        if self.cookie_name.is_empty() {
            return Err(BearDogError::security(
                "Cookie name cannot be empty".to_string(),
            ));
        }
        Ok(())
    }
}
