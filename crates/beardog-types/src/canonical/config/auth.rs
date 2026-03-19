// SPDX-License-Identifier: AGPL-3.0-only

//! Authentication Configuration
//!
//! Canonical authentication configuration for managing user identity, sessions, and access control.
//!
//! # Overview
//!
//! `CanonicalAuthConfig` provides comprehensive authentication settings including:
//! - Multiple authentication provider support (Local, OAuth2, LDAP, SAML)
//! - Session management and timeouts
//! - Login attempt limiting for security
//! - Provider-specific configuration
//!
//! # Quick Start
//!
//! ```rust
//! use beardog_types::canonical::config::auth::{CanonicalAuthConfig, AuthProvider};
//! use std::time::Duration;
//!
//! // Create authentication configuration
//! let config = CanonicalAuthConfig {
//!     enabled: true,
//!     providers: vec![AuthProvider::Local, AuthProvider::OAuth2],
//!     session_timeout: Duration::from_secs(3600), // 1 hour
//!     max_login_attempts: 5,
//! };
//!
//! assert!(config.enabled);
//! assert_eq!(config.providers.len(), 2);
//! ```
//!
//! # Production Configuration
//!
//! ```rust
//! use beardog_types::canonical::config::auth::{CanonicalAuthConfig, AuthProvider};
//! use std::time::Duration;
//!
//! // Secure production authentication
//! let config = CanonicalAuthConfig {
//!     enabled: true,
//!     providers: vec![
//!         AuthProvider::OAuth2,  // Primary
//!         AuthProvider::SAML,    // Enterprise SSO
//!     ],
//!     session_timeout: Duration::from_secs(1800), // 30 minutes
//!     max_login_attempts: 3,  // Stricter for production
//! };
//! ```
//!
//! # Multi-Provider Setup
//!
//! ```rust
//! use beardog_types::canonical::config::auth::{CanonicalAuthConfig, AuthProvider};
//! use std::time::Duration;
//!
//! // Support multiple authentication methods
//! let config = CanonicalAuthConfig {
//!     enabled: true,
//!     providers: vec![
//!         AuthProvider::Local,   // Username/password
//!         AuthProvider::OAuth2,  // Google, GitHub
//!         AuthProvider::LDAP,    // Corporate directory
//!         AuthProvider::SAML,    // Enterprise SSO
//!     ],
//!     session_timeout: Duration::from_secs(3600),
//!     max_login_attempts: 5,
//! };
//!
//! // Check if specific provider is enabled
//! let has_oauth = config.providers.iter().any(|p| matches!(p, AuthProvider::OAuth2));
//! assert!(has_oauth);
//! ```
//!
//! # Security Features
//!
//! - **Session Timeouts**: Automatic session expiration for security
//! - **Login Attempt Limiting**: Prevent brute force attacks
//! - **Multiple Providers**: Flexibility in authentication methods
//! - **Enterprise Integration**: LDAP and SAML support
//!
//! # Design Principles
//!
//! - **Security First**: Sensible defaults for production security
//! - **Flexibility**: Support multiple authentication providers
//! - **Enterprise Ready**: LDAP and SAML integration
//! - **Type Safety**: Strongly typed provider configuration

use serde::{Deserialize, Serialize};
use std::time::Duration;

/// Canonical authentication configuration
///
/// Comprehensive configuration for user authentication including provider selection,
/// session management, and security controls.
///
/// # Fields
///
/// * `enabled` - Whether authentication is required (disable for testing only)
/// * `providers` - List of enabled authentication providers
/// * `session_timeout` - How long sessions remain valid without activity
/// * `max_login_attempts` - Maximum failed login attempts before lockout
///
/// # Examples
///
/// ```rust
/// use beardog_types::canonical::config::auth::{CanonicalAuthConfig, AuthProvider};
/// use std::time::Duration;
///
/// let config = CanonicalAuthConfig {
///     enabled: true,
///     providers: vec![AuthProvider::Local, AuthProvider::OAuth2],
///     session_timeout: Duration::from_secs(3600),
///     max_login_attempts: 5,
/// };
/// ```
///
/// # Security Considerations
///
/// - Set `session_timeout` based on security requirements (shorter = more secure)
/// - Lower `max_login_attempts` in production to prevent brute force
/// - Use OAuth2/SAML for production; Local for development only
#[derive(Debug, Clone, Serialize, Deserialize)]
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

impl Default for CanonicalAuthConfig {
    fn default() -> Self {
        Self {
            enabled: true,
            providers: vec![AuthProvider::Local],
            session_timeout: Duration::from_secs(
                std::env::var("BEARDOG_AUTH_SESSION_TIMEOUT_SECS")
                    .ok()
                    .and_then(|s| s.parse().ok())
                    .unwrap_or(3600),
            ),
            max_login_attempts: 5, // Reasonable default to prevent brute force
        }
    }
}

/// Authentication provider types
///
/// Supported authentication mechanisms for user identity verification.
///
/// # Provider Types
///
/// * `Local` - Username/password stored in application database
/// * `OAuth2` - OAuth 2.0 (Google, GitHub, etc.)
/// * `LDAP` - Lightweight Directory Access Protocol (corporate directories)
/// * `SAML` - Security Assertion Markup Language (enterprise SSO)
///
/// # Examples
///
/// ```rust
/// use beardog_types::canonical::config::auth::AuthProvider;
///
/// // Development setup
/// let dev_providers = vec![AuthProvider::Local];
///
/// // Production setup
/// let prod_providers = vec![AuthProvider::OAuth2, AuthProvider::SAML];
///
/// // Check provider type
/// match prod_providers[0] {
///     AuthProvider::OAuth2 => println!("Using OAuth2"),
///     AuthProvider::Local => println!("Using local auth"),
///     _ => println!("Other provider"),
/// }
/// ```
///
/// # Security Recommendations
///
/// - **Local**: Use only for development; implement proper password hashing
/// - **OAuth2**: Recommended for user-facing applications; validate tokens properly
/// - **LDAP**: Good for corporate environments; use TLS for connections
/// - **SAML**: Best for enterprise SSO; validate assertions carefully
#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum AuthProvider {
    /// Local username/password authentication
    ///
    /// Users authenticate with username/password stored in the application database.
    /// **Security**: Ensure passwords are hashed with bcrypt/argon2.
    /// **Use Case**: Development, small deployments, backup authentication.
    Local,

    /// OAuth 2.0 authentication provider
    ///
    /// Delegates authentication to third-party OAuth2 providers (Google, GitHub, etc.).
    /// **Security**: Validate access tokens, use PKCE flow for public clients.
    /// **Use Case**: User-facing applications, mobile apps, web services.
    OAuth2,

    /// LDAP directory authentication
    ///
    /// Authenticates against corporate LDAP/Active Directory servers.
    /// **Security**: Use LDAPS (LDAP over TLS) for all connections.
    /// **Use Case**: Corporate deployments, internal applications.
    LDAP,

    /// SAML authentication provider
    ///
    /// Enterprise single sign-on using SAML 2.0 assertions.
    /// **Security**: Validate signatures, check assertion expiry.
    /// **Use Case**: Enterprise deployments, federated identity.
    SAML,
}

pub type AuthConfig = CanonicalAuthConfig;
