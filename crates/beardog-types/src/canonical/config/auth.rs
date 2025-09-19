// Canonical Authentication Configuration

use serde::{Deserialize, Serialize};
use std::time::Duration;

/// Canonical authentication configuration
#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct CanonicalAuthConfig {
    /// Whether authentication is enabled
    /// Whether feature is enabled
    pub enabled: bool,
    /// List of authentication providers
    pub providers: Vec<AuthProvider>,
    /// Session timeout duration
    pub session_timeout: Duration,
    /// Number of `max_login_attempts`
    pub max_login_attempts: u32,
}

/// Authentication provider types
#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum AuthProvider {
    /// Local username/password authentication
    Local,
    /// `OAuth2` authentication provider
    OAuth2,
    /// LDAP directory authentication
    LDAP,
    /// SAML authentication provider
    SAML,
}

pub type AuthConfig = CanonicalAuthConfig;
