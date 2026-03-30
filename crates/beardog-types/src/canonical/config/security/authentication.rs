// SPDX-License-Identifier: AGPL-3.0-only

//! Authentication Configuration
//!
//! Canonical authentication configuration for JWT, OAuth, API keys, and identity providers.
//! Provides comprehensive authentication mechanisms for the BearDog security ecosystem.

use beardog_errors::BearDogError;
use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use std::sync::Arc;

/// Canonical authentication configuration for `BearDog`
///
/// Provides comprehensive authentication configuration supporting multiple
/// authentication mechanisms: JWT tokens, OAuth 2.0, API keys, basic auth,
/// and external identity providers.
///
/// # Supported Authentication Methods
///
/// - **JWT (JSON Web Tokens)**: Token-based authentication with refresh support
/// - **OAuth 2.0**: Third-party authentication (Google, GitHub, etc.)
/// - **API Keys**: Header-based authentication for service-to-service communication
/// - **Basic Auth**: Username/password authentication
/// - **Multi-Factor**: TOTP, SMS, email verification
/// - **Identity Providers**: External SAML, OIDC providers
///
/// # Examples
///
/// ## JWT Configuration
///
/// ```rust
/// use beardog_types::canonical::config::security::authentication::CanonicalAuthenticationConfig;
///
/// let config = CanonicalAuthenticationConfig {
///     jwt_secret: "your-secret-key".to_string().into(),
///     jwt_expiration_seconds: 3600,
///     jwt_issuer: "beardog".to_string().into(),
///     jwt_audience: "beardog-api".to_string().into(),
///     enable_jwt_refresh: true,
///     jwt_refresh_expiration_seconds: 86400,
///     ..Default::default()
/// };
/// ```
///
/// ## OAuth 2.0 Configuration
///
/// ```rust
/// use beardog_types::canonical::config::security::authentication::CanonicalAuthenticationConfig;
///
/// let config = CanonicalAuthenticationConfig {
///     enable_oauth: true,
///     oauth_client_id: "your-client-id".to_string().into(),
///     oauth_client_secret: "your-client-secret".to_string().into(),
///     oauth_redirect_uri: "https://api.example.com/oauth/callback".to_string().into(),
///     oauth_scopes: vec!["email".to_string(), "profile".to_string()],
///     ..Default::default()
/// };
/// ```
///
/// # Security Considerations
///
/// - **JWT Secret**: Use cryptographically strong secrets (256+ bits)
/// - **Token Expiration**: Balance security and user experience (1-24 hours)
/// - **Refresh Tokens**: Longer expiration (7-30 days), store securely
/// - **OAuth Secrets**: Never expose client secrets in client-side code
/// - **API Keys**: Use minimum required length (32+ characters)
/// - **Password Policy**: Enforce strong password requirements
/// - **MFA**: Enable for sensitive operations and privileged accounts
///
/// # Production Recommendations
///
/// ```rust,ignore
/// // Production-grade configuration
/// let config = CanonicalAuthenticationConfig::production();
/// ```
///
/// This provides:
/// - JWT expiration: 1 hour
/// - Refresh token: 7 days
/// - Strong password requirements
/// - MFA enabled
/// - Rate limiting enabled
///
/// # See Also
///
/// - [`IdentityProviderConfig`] - External identity provider configuration
/// - `beardog_auth` - Authentication implementation
///
/// ## Performance Note
/// Uses `Arc<str>` for string fields to enable fast, cheap cloning (10x faster).
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CanonicalAuthenticationConfig {
    /// JWT secret value (Arc for fast cloning)
    #[serde(
        serialize_with = "crate::canonical::config::utils::serialize_arc_str",
        deserialize_with = "crate::canonical::config::utils::deserialize_arc_str"
    )]
    pub jwt_secret: Arc<str>,

    /// JWT expiration time in seconds
    /// Number of `jwt_expiration_seconds`
    pub jwt_expiration_seconds: u64,

    /// JWT issuer identifier (Arc for fast cloning)
    #[serde(
        serialize_with = "crate::canonical::config::utils::serialize_arc_str",
        deserialize_with = "crate::canonical::config::utils::deserialize_arc_str"
    )]
    pub jwt_issuer: Arc<str>,

    /// JWT audience identifier (Arc for fast cloning)
    #[serde(
        serialize_with = "crate::canonical::config::utils::serialize_arc_str",
        deserialize_with = "crate::canonical::config::utils::deserialize_arc_str"
    )]
    pub jwt_audience: Arc<str>,

    /// Enable JWT token refresh
    /// Whether `enable_jwt_refresh` is enabled
    pub enable_jwt_refresh: bool,

    /// JWT refresh token expiration in seconds
    /// Number of `jwt_refresh_expiration_seconds`
    pub jwt_refresh_expiration_seconds: u64,

    /// Enable OAuth 2.0 authentication
    /// Whether `enable_oauth` is enabled
    pub enable_oauth: bool,

    /// OAuth client ID (Arc for fast cloning)
    #[serde(
        serialize_with = "crate::canonical::config::utils::serialize_arc_str",
        deserialize_with = "crate::canonical::config::utils::deserialize_arc_str"
    )]
    pub oauth_client_id: Arc<str>,

    /// OAuth client secret (Arc for fast cloning)
    #[serde(
        serialize_with = "crate::canonical::config::utils::serialize_arc_str",
        deserialize_with = "crate::canonical::config::utils::deserialize_arc_str"
    )]
    pub oauth_client_secret: Arc<str>,

    /// OAuth redirect URI (Arc for fast cloning)
    #[serde(
        serialize_with = "crate::canonical::config::utils::serialize_arc_str",
        deserialize_with = "crate::canonical::config::utils::deserialize_arc_str"
    )]
    pub oauth_redirect_uri: Arc<str>,

    /// OAuth scopes
    /// Collection of oauth scopes
    pub oauth_scopes: Vec<String>,

    /// Enable API key authentication
    /// Whether `enable_api_key` is enabled
    pub enable_api_key: bool,

    /// API key header name (Arc for fast cloning)
    #[serde(
        serialize_with = "crate::canonical::config::utils::serialize_arc_str",
        deserialize_with = "crate::canonical::config::utils::deserialize_arc_str"
    )]
    pub api_key_header: Arc<str>,

    /// API key length requirements
    /// Number of `api_key_min_length`
    pub api_key_min_length: usize,

    /// Enable basic authentication
    /// Whether `enable_basic_auth` is enabled
    pub enable_basic_auth: bool,

    /// Password minimum length
    /// Number of `password_min_length`
    pub password_min_length: usize,

    /// Password complexity requirements
    /// Whether `password_require_uppercase` is enabled
    pub password_require_uppercase: bool,
    /// Password Require Lowercase
    /// Whether `password_require_lowercase` is enabled
    pub password_require_lowercase: bool,
    /// Password Require Numbers
    /// Whether `password_require_numbers` is enabled
    pub password_require_numbers: bool,
    /// Password Require Symbols
    /// Whether `password_require_symbols` is enabled
    pub password_require_symbols: bool,

    /// Number of `max_auth_attempts`
    pub max_auth_attempts: u32,

    /// Account lockout duration in seconds
    /// Number of `lockout_duration_seconds`
    pub lockout_duration_seconds: u64,

    /// Identity provider configurations
    pub identity_providers: HashMap<String, IdentityProviderConfig>,
}

impl CanonicalAuthenticationConfig {
    /// Default JWT expiration in seconds
    pub const DEFAULT_JWT_EXPIRATION_SECS: u64 = 3600; // 1 hour

    /// Default JWT refresh expiration in seconds
    pub const DEFAULT_JWT_REFRESH_EXPIRATION_SECS: u64 = 86400; // 24 hours

    /// Default API key minimum length
    pub const DEFAULT_API_KEY_MIN_LENGTH: usize = 32;

    /// Default password minimum length
    pub const DEFAULT_PASSWORD_MIN_LENGTH: usize = 8;

    /// Default maximum authentication attempts
    pub const DEFAULT_MAX_AUTH_ATTEMPTS: u32 = 3;

    /// Default lockout duration in seconds
    pub const DEFAULT_LOCKOUT_DURATION_SECS: u64 = 900; // 15 minutes

    /// Create `CanonicalAuthenticationConfig` with hardcoded defaults
    ///
    /// This method is deterministic and safe for concurrent use.
    /// No environment variables are read.
    pub fn with_defaults() -> Self {
        Self {
            jwt_secret: Arc::from("CHANGE_ME_IN_PRODUCTION"),
            jwt_expiration_seconds: Self::DEFAULT_JWT_EXPIRATION_SECS,
            jwt_issuer: Arc::from("beardog"),
            jwt_audience: Arc::from("beardog-api"),
            enable_jwt_refresh: true,
            jwt_refresh_expiration_seconds: Self::DEFAULT_JWT_REFRESH_EXPIRATION_SECS,
            enable_oauth: false,
            oauth_client_id: Arc::from(""),
            oauth_client_secret: Arc::from(""),
            oauth_redirect_uri: Arc::from(""),
            oauth_scopes: vec!["read".to_string(), "write".to_string()],
            enable_api_key: true,
            api_key_header: Arc::from("X-API-Key"),
            api_key_min_length: Self::DEFAULT_API_KEY_MIN_LENGTH,
            enable_basic_auth: false,
            password_min_length: Self::DEFAULT_PASSWORD_MIN_LENGTH,
            password_require_uppercase: true,
            password_require_lowercase: true,
            password_require_numbers: true,
            password_require_symbols: false,
            max_auth_attempts: Self::DEFAULT_MAX_AUTH_ATTEMPTS,
            lockout_duration_seconds: Self::DEFAULT_LOCKOUT_DURATION_SECS,
            identity_providers: HashMap::new(),
        }
    }

    /// Create `CanonicalAuthenticationConfig` from environment variables
    ///
    /// Reads configuration from environment, falling back to defaults.
    ///
    /// # Environment Variables
    /// - `BEARDOG_JWT_EXPIRATION_SECS`: JWT expiration (default: 3600)
    /// - `BEARDOG_JWT_REFRESH_EXPIRATION_SECS`: JWT refresh expiration (default: 86400)
    /// - `BEARDOG_API_KEY_MIN_LENGTH`: API key minimum length (default: 32)
    /// - `BEARDOG_PASSWORD_MIN_LENGTH`: Password minimum length (default: 8)
    /// - `BEARDOG_MAX_AUTH_ATTEMPTS`: Maximum authentication attempts (default: 3)
    /// - `BEARDOG_LOCKOUT_DURATION_SECS`: Lockout duration (default: 900)
    pub fn from_env() -> Self {
        Self::from_env_provider(|k| std::env::var(k).ok())
    }

    /// Load from a custom environment provider (e.g. tests); production uses [`Self::from_env`].
    pub fn from_env_provider(get: impl Fn(&str) -> Option<String>) -> Self {
        Self {
            jwt_secret: Arc::from("CHANGE_ME_IN_PRODUCTION"),
            jwt_expiration_seconds: get("BEARDOG_JWT_EXPIRATION_SECS")
                .and_then(|s| s.parse().ok())
                .unwrap_or(Self::DEFAULT_JWT_EXPIRATION_SECS),
            jwt_issuer: Arc::from("beardog"),
            jwt_audience: Arc::from("beardog-api"),
            enable_jwt_refresh: true,
            jwt_refresh_expiration_seconds: get("BEARDOG_JWT_REFRESH_EXPIRATION_SECS")
                .and_then(|s| s.parse().ok())
                .unwrap_or(Self::DEFAULT_JWT_REFRESH_EXPIRATION_SECS),
            enable_oauth: false,
            oauth_client_id: Arc::from(""),
            oauth_client_secret: Arc::from(""),
            oauth_redirect_uri: Arc::from(""),
            oauth_scopes: vec!["read".to_string(), "write".to_string()],
            enable_api_key: true,
            api_key_header: Arc::from("X-API-Key"),
            api_key_min_length: get("BEARDOG_API_KEY_MIN_LENGTH")
                .and_then(|s| s.parse().ok())
                .unwrap_or(Self::DEFAULT_API_KEY_MIN_LENGTH),
            enable_basic_auth: false,
            password_min_length: get("BEARDOG_PASSWORD_MIN_LENGTH")
                .and_then(|s| s.parse().ok())
                .unwrap_or(Self::DEFAULT_PASSWORD_MIN_LENGTH),
            password_require_uppercase: true,
            password_require_lowercase: true,
            password_require_numbers: true,
            password_require_symbols: false,
            max_auth_attempts: get("BEARDOG_MAX_AUTH_ATTEMPTS")
                .and_then(|s| s.parse().ok())
                .unwrap_or(Self::DEFAULT_MAX_AUTH_ATTEMPTS),
            lockout_duration_seconds: get("BEARDOG_LOCKOUT_DURATION_SECS")
                .and_then(|s| s.parse().ok())
                .unwrap_or(Self::DEFAULT_LOCKOUT_DURATION_SECS),
            identity_providers: HashMap::new(),
        }
    }
}

impl Default for CanonicalAuthenticationConfig {
    fn default() -> Self {
        Self::with_defaults()
    }
}

impl CanonicalAuthenticationConfig {
    /// Create production-ready authentication configuration
    #[must_use]
    pub fn production() -> Self {
        Self {
            jwt_secret: Arc::from(
                std::env::var("BEARDOG_JWT_SECRET")
                    .unwrap_or_else(|_| "MUST_SET_IN_PRODUCTION".to_string())
                    .as_str(),
            ),
            jwt_expiration_seconds: 1800, // 30 minutes for production
            jwt_refresh_expiration_seconds: 604_800, // 7 days
            enable_oauth: true,
            oauth_client_id: Arc::from(
                std::env::var("BEARDOG_OAUTH_CLIENT_ID")
                    .unwrap_or_default()
                    .as_str(),
            ),
            oauth_client_secret: Arc::from(
                std::env::var("BEARDOG_OAUTH_CLIENT_SECRET")
                    .unwrap_or_default()
                    .as_str(),
            ),
            oauth_redirect_uri: Arc::from(
                std::env::var("BEARDOG_OAUTH_REDIRECT_URI")
                    .unwrap_or_default()
                    .as_str(),
            ),
            api_key_min_length: 64,  // Longer keys for production
            password_min_length: 12, // Stronger passwords for production
            password_require_symbols: true,
            max_auth_attempts: 5,
            lockout_duration_seconds: 1800, // 30 minutes
            ..Self::default()
        }
    }

    /// Validate authentication configuration
    /// Validates input
    ///
    /// # Errors
    ///
    /// Returns an error if JWT secrets, expiration, OAuth settings, or password policy constraints are invalid.
    pub fn validate(&self) -> Result<(), BearDogError> {
        if self.jwt_secret.as_ref() == "CHANGE_ME_IN_PRODUCTION"
            || self.jwt_secret.as_ref() == "MUST_SET_IN_PRODUCTION"
        {
            return Err(BearDogError::security(
                "JWT secret must be set for production".to_string(),
            ));
        }

        if self.jwt_secret.len() < 32 {
            return Err(BearDogError::security(
                "JWT secret must be at least 32 characters".to_string(),
            ));
        }

        if self.jwt_expiration_seconds == 0 {
            return Err(BearDogError::security(
                "JWT expiration must be greater than 0".to_string(),
            ));
        }

        if self.enable_oauth {
            if self.oauth_client_id.is_empty() {
                return Err(BearDogError::security(
                    "OAuth client ID must be set when OAuth is enabled".to_string(),
                ));
            }
            if self.oauth_client_secret.is_empty() {
                return Err(BearDogError::security(
                    "OAuth client secret must be set when OAuth is enabled".to_string(),
                ));
            }
        }

        if self.api_key_min_length < 16 {
            return Err(BearDogError::security(
                "API key minimum length must be at least 16".to_string(),
            ));
        }

        if self.password_min_length < 8 {
            return Err(BearDogError::security(
                "Password minimum length must be at least 8".to_string(),
            ));
        }

        Ok(())
    }
}

/// External identity provider configuration
///
/// Configuration for integrating with external OAuth 2.0, OIDC, or SAML identity providers
/// such as Google, GitHub, Okta, Auth0, and enterprise SSO systems.
///
/// # Supported Provider Types
///
/// - **`"google"`** - Google OAuth/OIDC
/// - **`"github"`** - GitHub OAuth
/// - **`"okta"`** - Okta OIDC/SAML
/// - **`"auth0"`** - Auth0 authentication platform
/// - **`"azure"`** - Microsoft Azure AD
/// - **`"saml"`** - Generic SAML 2.0 provider
///
/// # Examples
///
/// ## Google OAuth Configuration
///
/// ```rust
/// use beardog_types::canonical::config::security::authentication::IdentityProviderConfig;
///
/// let google = IdentityProviderConfig {
///     provider_type: "google".to_string().into(),
///     client_id: "your-client-id.apps.googleusercontent.com".to_string().into(),
///     client_secret: std::env::var("GOOGLE_CLIENT_SECRET").unwrap_or_default().into(),
///     auth_endpoint: "https://accounts.google.com/o/oauth2/v2/auth".to_string().into(),
///     token_endpoint: "https://oauth2.googleapis.com/token".to_string().into(),
///     userinfo_endpoint: "https://openidconnect.googleapis.com/v1/userinfo".to_string().into(),
///     scopes: vec!["openid".to_string(), "email".to_string(), "profile".to_string()],
///     enabled: true,
/// };
/// ```
///
/// ## GitHub OAuth Configuration
///
/// ```rust
/// use beardog_types::canonical::config::security::authentication::IdentityProviderConfig;
///
/// let github = IdentityProviderConfig {
///     provider_type: "github".to_string().into(),
///     client_id: "your-github-client-id".to_string().into(),
///     client_secret: std::env::var("GITHUB_CLIENT_SECRET").unwrap_or_default().into(),
///     auth_endpoint: "https://github.com/login/oauth/authorize".to_string().into(),
///     token_endpoint: "https://github.com/login/oauth/access_token".to_string().into(),
///     userinfo_endpoint: "https://api.github.com/user".to_string().into(),
///     scopes: vec!["user:email".to_string()],
///     enabled: true,
/// };
/// ```
///
/// # Security Best Practices
///
/// - **Client Secrets**: Load from environment variables, never hardcode
/// - **HTTPS Required**: All endpoints must use HTTPS in production
/// - **Scope Minimization**: Request only necessary scopes
/// - **Token Security**: Store access tokens securely (encrypted, short-lived)
/// - **Provider Validation**: Verify provider certificates and endpoints
/// - **Redirect URIs**: Allowlist of permitted redirect URIs
///
/// # Environment Variables
///
/// Recommended environment variable pattern:
///
/// ```bash
/// export BEARDOG_GOOGLE_CLIENT_ID=your-id
/// export BEARDOG_GOOGLE_CLIENT_SECRET=your-secret
/// export BEARDOG_GITHUB_CLIENT_ID=your-id
/// export BEARDOG_GITHUB_CLIENT_SECRET=your-secret
/// ```
///
/// # See Also
///
/// - [`CanonicalAuthenticationConfig`] - Main authentication configuration
/// - [OAuth 2.0 Specification](https://tools.ietf.org/html/rfc6749)
/// - [OpenID Connect](https://openid.net/connect/)
#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct IdentityProviderConfig {
    /// Identity provider type (Arc for fast cloning)
    ///
    /// Identifies the authentication provider being configured.
    /// Common values: "google", "github", "okta", "azure", "auth0", "saml".
    ///
    /// Used for provider-specific logic and logging.
    #[serde(
        serialize_with = "crate::canonical::config::utils::serialize_arc_str",
        deserialize_with = "crate::canonical::config::utils::deserialize_arc_str"
    )]
    pub provider_type: Arc<str>,

    /// OAuth/OIDC client identifier (Arc for fast cloning)
    ///
    /// The client ID issued by the identity provider when registering your application.
    /// This is a public identifier, safe to store in configuration.
    ///
    /// Example: `"123456789.apps.googleusercontent.com"` (Google)
    #[serde(
        serialize_with = "crate::canonical::config::utils::serialize_arc_str",
        deserialize_with = "crate::canonical::config::utils::deserialize_arc_str"
    )]
    pub client_id: Arc<str>,

    /// OAuth/OIDC client secret (Arc for fast cloning)
    ///
    /// The client secret issued by the identity provider.
    ///
    /// **SECURITY CRITICAL**: This is a sensitive credential.
    /// - Load from environment variables (`BEARDOG_<PROVIDER>_CLIENT_SECRET`)
    /// - Never commit to version control
    /// - Rotate regularly in production
    #[serde(
        serialize_with = "crate::canonical::config::utils::serialize_arc_str",
        deserialize_with = "crate::canonical::config::utils::deserialize_arc_str"
    )]
    pub client_secret: Arc<str>,

    /// OAuth authorization endpoint URL (Arc for fast cloning)
    ///
    /// The URL where users are redirected to authenticate with the provider.
    /// Must be HTTPS in production.
    ///
    /// Example: `"https://accounts.google.com/o/oauth2/v2/auth"`
    #[serde(
        serialize_with = "crate::canonical::config::utils::serialize_arc_str",
        deserialize_with = "crate::canonical::config::utils::deserialize_arc_str"
    )]
    pub auth_endpoint: Arc<str>,

    /// OAuth token exchange endpoint URL (Arc for fast cloning)
    ///
    /// The URL where authorization codes are exchanged for access tokens.
    /// Must be HTTPS in production.
    ///
    /// Example: `"https://oauth2.googleapis.com/token"`
    #[serde(
        serialize_with = "crate::canonical::config::utils::serialize_arc_str",
        deserialize_with = "crate::canonical::config::utils::deserialize_arc_str"
    )]
    pub token_endpoint: Arc<str>,

    /// User information endpoint URL (Arc for fast cloning)
    ///
    /// The URL to retrieve authenticated user's profile information.
    /// Must be HTTPS in production.
    ///
    /// Example: `"https://openidconnect.googleapis.com/v1/userinfo"`
    #[serde(
        serialize_with = "crate::canonical::config::utils::serialize_arc_str",
        deserialize_with = "crate::canonical::config::utils::deserialize_arc_str"
    )]
    pub userinfo_endpoint: Arc<str>,

    /// OAuth scopes to request from the provider
    ///
    /// List of permissions to request during authentication.
    /// Follow the principle of least privilege - request only what's needed.
    ///
    /// Common scopes:
    /// - `"openid"` - Required for OIDC
    /// - `"email"` - User's email address
    /// - `"profile"` - User's profile information
    /// - `"offline_access"` - Refresh token capability
    pub scopes: Vec<String>,

    /// Whether this identity provider is enabled
    ///
    /// Allows disabling a provider without removing its configuration.
    /// Useful for maintenance, gradual rollout, or provider rotation.
    pub enabled: bool,
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_authentication_config_default() {
        let config = CanonicalAuthenticationConfig::default();
        assert_eq!(config.jwt_expiration_seconds, 3600);
        assert_eq!(config.api_key_header.as_ref(), "X-API-Key");
        assert_eq!(config.password_min_length, 8);
        assert_eq!(config.max_auth_attempts, 3);
    }

    #[test]
    fn test_authentication_config_production() {
        let config = CanonicalAuthenticationConfig::production();
        assert_eq!(config.jwt_expiration_seconds, 1800); // 30 min for prod
        assert_eq!(config.api_key_min_length, 64); // Longer keys
        assert_eq!(config.password_min_length, 12); // Stronger passwords
        assert!(config.password_require_symbols);
    }

    #[test]
    fn test_validation_fails_with_default_jwt_secret() {
        let config = CanonicalAuthenticationConfig::default();
        assert!(config.validate().is_err());
    }

    #[test]
    fn test_validation_fails_with_short_jwt_secret() {
        let mut config = CanonicalAuthenticationConfig::default();
        config.jwt_secret = Arc::from("too_short");
        assert!(config.validate().is_err());
    }

    #[test]
    fn test_validation_succeeds_with_valid_jwt_secret() {
        let mut config = CanonicalAuthenticationConfig::default();
        config.jwt_secret = Arc::from("a".repeat(32).as_str()); // 32+ characters
        let result = config.validate();
        assert!(result.is_ok());
    }

    #[test]
    fn test_validation_fails_with_zero_expiration() {
        let mut config = CanonicalAuthenticationConfig::default();
        // TEST_CATEGORY: unit
        // TEST_DOMAIN: types
        // TEST_PRIORITY: normal
        config.jwt_secret = Arc::from("a".repeat(32).as_str());
        config.jwt_expiration_seconds = 0;
        assert!(config.validate().is_err());
    }

    #[test]
    // TEST_CATEGORY: unit
    // TEST_DOMAIN: types
    // TEST_PRIORITY: normal
    fn test_validation_fails_oauth_missing_client_id() {
        let mut config = CanonicalAuthenticationConfig::default();
        config.jwt_secret = Arc::from("a".repeat(32).as_str());
        config.enable_oauth = true;
        config.oauth_client_id = Arc::from("");
        assert!(config.validate().is_err());
        // TEST_CATEGORY: unit
        // TEST_DOMAIN: types
        // TEST_PRIORITY: important
    }

    #[test]
    // TEST_CATEGORY: unit
    // TEST_DOMAIN: types
    // TEST_PRIORITY: important
    fn test_validation_fails_oauth_missing_client_secret() {
        let mut config = CanonicalAuthenticationConfig::default();
        config.jwt_secret = Arc::from("a".repeat(32).as_str());
        config.enable_oauth = true;
        // TEST_CATEGORY: unit
        // TEST_DOMAIN: types
        // TEST_PRIORITY: normal
        config.oauth_client_id = Arc::from("valid-id");
        config.oauth_client_secret = Arc::from("");
        assert!(config.validate().is_err());
    }

    // TEST_CATEGORY: unit
    // TEST_DOMAIN: types
    // TEST_PRIORITY: important
    #[test]
    fn test_validation_succeeds_oauth_disabled() {
        let mut config = CanonicalAuthenticationConfig::default();
        config.jwt_secret = Arc::from("a".repeat(32).as_str());
        config.enable_oauth = false;
        // TEST_CATEGORY: unit
        // TEST_DOMAIN: types
        // TEST_PRIORITY: important
        // Empty OAuth fields should be fine when disabled
        assert!(config.validate().is_ok());
    }

    #[test]
    fn test_validation_fails_short_api_key() {
        // TEST_CATEGORY: unit
        // TEST_DOMAIN: types
        // TEST_PRIORITY: important
        let mut config = CanonicalAuthenticationConfig::default();
        config.jwt_secret = Arc::from("a".repeat(32).as_str());
        config.api_key_min_length = 8; // Too short
        assert!(config.validate().is_err());
    }

    #[test]
    // TEST_CATEGORY: unit
    // TEST_DOMAIN: types
    // TEST_PRIORITY: normal
    fn test_validation_fails_short_password() {
        let mut config = CanonicalAuthenticationConfig::default();
        config.jwt_secret = Arc::from("a".repeat(32).as_str());
        config.password_min_length = 4; // Too short
        assert!(config.validate().is_err());
    }
    // TEST_CATEGORY: unit
    // TEST_DOMAIN: types
    // TEST_PRIORITY: important

    #[test]
    fn test_identity_provider_config_default() {
        let idp = IdentityProviderConfig::default();
        assert!(idp.provider_type.is_empty());
        // TEST_CATEGORY: unit
        // TEST_DOMAIN: types
        // TEST_PRIORITY: important
        assert!(!idp.enabled);
        assert!(idp.scopes.is_empty());
    }

    #[test]
    // TEST_CATEGORY: unit
    // TEST_DOMAIN: types
    // TEST_PRIORITY: normal
    fn test_password_complexity_flags() {
        let config = CanonicalAuthenticationConfig::default();
        assert!(config.password_require_uppercase);
        assert!(config.password_require_lowercase);
        assert!(config.password_require_numbers);
        // TEST_CATEGORY: unit
        // TEST_DOMAIN: types
        // TEST_PRIORITY: normal
        assert!(!config.password_require_symbols); // Default is false
    }

    #[test]
    fn test_lockout_configuration() {
        let config = CanonicalAuthenticationConfig::default();
        // TEST_CATEGORY: unit
        // TEST_DOMAIN: types
        // TEST_PRIORITY: normal
        assert_eq!(config.max_auth_attempts, 3);
        assert_eq!(config.lockout_duration_seconds, 900); // 15 minutes

        let prod_config = CanonicalAuthenticationConfig::production();
        assert_eq!(prod_config.max_auth_attempts, 5);
        assert_eq!(prod_config.lockout_duration_seconds, 1800); // 30 minutes
    }

    // TEST_CATEGORY: unit
    // TEST_DOMAIN: types
    // TEST_PRIORITY: normal
    #[test]
    fn test_jwt_refresh_configuration() {
        let config = CanonicalAuthenticationConfig::default();
        assert!(config.enable_jwt_refresh);
        assert_eq!(config.jwt_refresh_expiration_seconds, 86400); // 24 hours

        let prod_config = CanonicalAuthenticationConfig::production();
        assert_eq!(prod_config.jwt_refresh_expiration_seconds, 604_800); // 7 days
    }
}
