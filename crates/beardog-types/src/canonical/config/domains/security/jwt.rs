// SPDX-License-Identifier: AGPL-3.0-or-later

//! JWT Token Configuration
//!
//! Comprehensive JWT configuration including token generation, validation, and refresh tokens.

use beardog_errors::BearDogError;
use serde::{Deserialize, Serialize};

/// JWT token configuration
///
/// Comprehensive JWT token settings including expiration times, refresh tokens,
/// and security settings.
///
/// # Security Considerations
///
/// - Store `jwt_secret` securely (use environment variables or secret management)
/// - Use appropriate expiration times based on security requirements
/// - Enable refresh tokens for long-lived sessions
/// - Rotate secrets regularly
///
/// # Examples
///
/// ```rust
/// use beardog_types::canonical::config::domains::security::JwtConfig;
///
/// let config = JwtConfig::default();
/// assert_eq!(config.expiration_seconds, 3600); // 1 hour
/// ```
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct JwtConfig {
    /// JWT secret key for signing tokens
    /// 
    /// **Security**: Must be stored securely (env vars, secret management)
    pub jwt_secret: String,
    /// Token expiration time in seconds (default: 3600 = 1 hour)
    pub expiration_seconds: u64,
    /// Whether to enable refresh tokens (default: true)
    pub enable_refresh: bool,
    /// Refresh token expiration time in seconds (default: 604800 = 7 days)
    pub refresh_expiration_seconds: u64,
}

impl Default for JwtConfig {
    fn default() -> Self {
        Self::development()
    }
}

impl JwtConfig {
    /// Create JWT configuration from environment variables
    ///
    /// Uses the following environment variables:
    /// - `BEARDOG_JWT_SECRET` - JWT signing secret (required in production)
    /// - `BEARDOG_JWT_EXPIRY_SECS` - Token expiration (default: 3600)
    /// - `BEARDOG_JWT_ENABLE_REFRESH` - Enable refresh tokens (default: true)
    /// - `BEARDOG_JWT_REFRESH_EXPIRY_SECS` - Refresh token expiration (default: 604800)
    pub fn from_env() -> Self {
        Self {
            jwt_secret: std::env::var("BEARDOG_JWT_SECRET")
                .unwrap_or_else(|_| Self::development_secret()),
            expiration_seconds: std::env::var("BEARDOG_JWT_EXPIRY_SECS")
                .ok()
                .and_then(|s| s.parse().ok())
                .unwrap_or(3600), // 1 hour
            enable_refresh: std::env::var("BEARDOG_JWT_ENABLE_REFRESH")
                .ok()
                .and_then(|s| s.parse().ok())
                .unwrap_or(true),
            refresh_expiration_seconds: std::env::var("BEARDOG_JWT_REFRESH_EXPIRY_SECS")
                .ok()
                .and_then(|s| s.parse().ok())
                .unwrap_or(604800), // 7 days
        }
    }

    /// Create configuration from a config source (modern pattern)
    pub fn from_source(source: &dyn crate::canonical::config::source::ConfigSource) -> Self {
        use crate::canonical::config::source::{get_bool, get_parsed};

        Self {
            jwt_secret: source
                .get("BEARDOG_JWT_SECRET")
                .unwrap_or_else(|| Self::development_secret()),
            expiration_seconds: get_parsed(source, "BEARDOG_JWT_EXPIRY_SECS", 3600),
            enable_refresh: get_bool(source, "BEARDOG_JWT_ENABLE_REFRESH", true),
            refresh_expiration_seconds: get_parsed(
                source,
                "BEARDOG_JWT_REFRESH_EXPIRY_SECS",
                604800,
            ),
        }
    }

    /// Development-only secret (DO NOT use in production)
    fn development_secret() -> String {
        "INSECURE_DEVELOPMENT_SECRET_DO_NOT_USE_IN_PRODUCTION".to_string()
    }

    /// Create development configuration
    pub fn development() -> Self {
        Self {
            jwt_secret: Self::development_secret(),
            expiration_seconds: 3600,
            enable_refresh: true,
            refresh_expiration_seconds: 604800,
        }
    }

    /// Create production configuration
    ///
    /// **Important**: jwt_secret MUST be provided via environment variables
    pub fn production() -> Result<Self, BearDogError> {
        let config = Self::from_env();
        config.validate()?;

        if config.jwt_secret == Self::development_secret() {
            return Err(BearDogError::configuration(
                "JWT secret must be set via BEARDOG_JWT_SECRET in production",
            ));
        }

        Ok(config)
    }

    /// Validate JWT configuration
    pub fn validate(&self) -> Result<(), BearDogError> {
        if self.jwt_secret.is_empty() {
            return Err(BearDogError::configuration("JWT secret cannot be empty"));
        }

        if self.jwt_secret.len() < 32 {
            return Err(BearDogError::configuration(
                "JWT secret must be at least 32 characters for security",
            ));
        }

        if self.expiration_seconds == 0 {
            return Err(BearDogError::configuration(
                "JWT expiration must be greater than 0",
            ));
        }

        if self.enable_refresh && self.refresh_expiration_seconds == 0 {
            return Err(BearDogError::configuration(
                "Refresh token expiration must be greater than 0 when refresh is enabled",
            ));
        }

        if self.enable_refresh && self.refresh_expiration_seconds <= self.expiration_seconds {
            return Err(BearDogError::configuration(
                "Refresh token expiration must be greater than access token expiration",
            ));
        }

        Ok(())
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_default_jwt_config() {
        let config = JwtConfig::default();
        assert_eq!(config.expiration_seconds, 3600);
        assert!(config.enable_refresh);
        assert_eq!(config.refresh_expiration_seconds, 604800);
    }

    #[test]
    fn test_jwt_config_validation() {
        let mut config = JwtConfig::development();
        assert!(config.validate().is_ok());

        // Test empty secret
        config.jwt_secret = String::new();
        assert!(config.validate().is_err());

        // Test short secret
        config.jwt_secret = "short".to_string();
        assert!(config.validate().is_err());

        // Test valid secret
        config.jwt_secret = "a".repeat(32);
        assert!(config.validate().is_ok());
    }

    #[test]
    fn test_refresh_token_validation() {
        let mut config = JwtConfig::development();
        config.enable_refresh = true;
        config.refresh_expiration_seconds = config.expiration_seconds - 1;
        assert!(config.validate().is_err());

        config.refresh_expiration_seconds = config.expiration_seconds + 1;
        assert!(config.validate().is_ok());
    }

    #[test]
    fn test_production_config_requires_secret() {
        let loaded = JwtConfig::from_env();
        let dev = JwtConfig::development();
        if loaded.jwt_secret == dev.jwt_secret {
            assert!(JwtConfig::production().is_err());
        }
    }
}

