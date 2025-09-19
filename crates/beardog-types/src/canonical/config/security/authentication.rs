// Authentication Configuration
//
// Canonical authentication configuration for JWT, OAuth, API keys, and identity providers.

use beardog_errors::BearDogError;
use serde::{Deserialize, Serialize};
use std::collections::HashMap;

/// **CANONICAL AUTHENTICATION CONFIGURATION**
///
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CanonicalAuthenticationConfig {
    /// The jwt secret value
    pub jwt_secret: String,

    /// JWT expiration time in seconds
    /// Number of `jwt_expiration_seconds`
    pub jwt_expiration_seconds: u64,

    /// JWT issuer identifier
    /// The jwt issuer value
    pub jwt_issuer: String,

    /// JWT audience identifier
    /// The jwt audience value
    pub jwt_audience: String,

    /// Enable JWT token refresh
    /// Whether `enable_jwt_refresh` is enabled
    pub enable_jwt_refresh: bool,

    /// JWT refresh token expiration in seconds
    /// Number of `jwt_refresh_expiration_seconds`
    pub jwt_refresh_expiration_seconds: u64,

    /// Enable OAuth 2.0 authentication
    /// Whether `enable_oauth` is enabled
    pub enable_oauth: bool,

    /// OAuth client ID
    pub oauth_client_id: String,

    /// OAuth client secret
    /// The oauth client secret value
    pub oauth_client_secret: String,

    /// OAuth redirect URI
    /// The oauth redirect uri value
    pub oauth_redirect_uri: String,

    /// OAuth scopes
    /// Collection of oauth scopes
    pub oauth_scopes: Vec<String>,

    /// Enable API key authentication
    /// Whether `enable_api_key` is enabled
    pub enable_api_key: bool,

    /// API key header name
    /// The api key header value
    pub api_key_header: String,

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

impl Default for CanonicalAuthenticationConfig {
    fn default() -> Self {
        Self {
            jwt_secret: "CHANGE_ME_IN_PRODUCTION".to_string(),
            jwt_expiration_seconds: 3600, // 1 hour
            jwt_issuer: "beardog ".to_string(),
            jwt_audience: "beardog-api".to_string(),
            enable_jwt_refresh: true,
            jwt_refresh_expiration_seconds: 86400, // 24 hours
            enable_oauth: false,
            oauth_client_id: String::new(),
            oauth_client_secret: String::new(),
            oauth_redirect_uri: String::new(),
            oauth_scopes: vec!["read".to_string(), "write".to_string()],
            enable_api_key: true,
            api_key_header: "X-API-Key".to_string(),
            api_key_min_length: 32,
            enable_basic_auth: false,
            password_min_length: 8,
            password_require_uppercase: true,
            password_require_lowercase: true,
            password_require_numbers: true,
            password_require_symbols: false,
            max_auth_attempts: 3,
            lockout_duration_seconds: 900, // 15 minutes
            identity_providers: HashMap::new(),
        }
    }
}

impl CanonicalAuthenticationConfig {
    /// Create production-ready authentication configuration
    #[must_use]
    pub fn production() -> Self {
        Self {
            jwt_secret: std::env::var("BEARDOG_JWT_SECRET")
                .unwrap_or_else(|_| "MUST_SET_IN_PRODUCTION".to_string()),
            jwt_expiration_seconds: 1800, // 30 minutes for production
            jwt_refresh_expiration_seconds: 604_800, // 7 days
            enable_oauth: true,
            oauth_client_id: std::env::var("BEARDOG_OAUTH_CLIENT_ID").unwrap_or_default(),
            oauth_client_secret: std::env::var("BEARDOG_OAUTH_CLIENT_SECRET").unwrap_or_default(),
            oauth_redirect_uri: std::env::var("BEARDOG_OAUTH_REDIRECT_URI").unwrap_or_default(),
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
    /// Validates input
    pub fn validate(&self) -> Result<(), BearDogError> {
        if self.jwt_secret == "CHANGE_ME_IN_PRODUCTION"
            || self.jwt_secret == "MUST_SET_IN_PRODUCTION"
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

/// Identity provider configuration
#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct IdentityProviderConfig {
    /// Provider type (e.g., "google", "github", "`universal_cloud`")
    pub provider_type: String,

    pub client_id: String,

    /// The client secret value
    pub client_secret: String,

    /// Authorization endpoint
    /// The auth endpoint value
    pub auth_endpoint: String,

    /// Token endpoint
    /// The token endpoint value
    pub token_endpoint: String,

    /// User info endpoint
    /// The userinfo endpoint value
    pub userinfo_endpoint: String,

    /// Required scopes
    /// Collection of scopes
    pub scopes: Vec<String>,

    /// Enable this identity provider
    /// Whether feature is enabled
    pub enabled: bool,
}
