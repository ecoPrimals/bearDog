// SPDX-License-Identifier: AGPL-3.0-or-later

// Canonical Security Configuration System
//
// This module provides the **single source of truth** for all security configuration
// across the BearDog ecosystem. It replaces the large monolithic security.rs file
// with a modular, maintainable structure.
//
// ## Architecture
//
// Security configuration is organized into logical domains:
// - **Authentication**: JWT, OAuth, API keys, identity providers
// - **Authorization**: RBAC, permissions, access control policies
// - **Encryption**: Algorithms, key management, HSM integration
// - **Audit**: Logging, compliance, security event tracking
// - **Session**: Session management, timeouts, security policies
// - **MFA**: Multi-factor authentication configuration

use beardog_errors::BearDogError;
use serde::{Deserialize, Serialize};

// Domain-specific security configuration modules
/// Audit module
pub mod audit;
/// Authentication module
pub mod authentication;
/// Authorization module
pub mod authorization;
/// Encryption module
pub mod encryption;
/// Mfa module
pub mod mfa;
/// Session module
pub mod session;

// Re-export canonical types
pub use audit::*;
pub use authentication::*;
pub use authorization::*;
pub use encryption::*;
pub use mfa::*;
pub use session::*;

/// **CANONICAL SECURITY CONFIGURATION** - Single source of truth
///
/// This is the primary security configuration that consolidates all security
/// settings across the `BearDog` ecosystem, replacing fragmented security configs.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CanonicalSecurityConfig {
    /// Enable encryption system-wide
    /// Whether `enable_encryption` is enabled
    pub enable_encryption: bool,

    /// Enable HSM integration
    /// Whether `enable_hsm` is enabled
    pub enable_hsm: bool,

    /// Authentication configuration
    /// The authentication value
    pub authentication: CanonicalAuthenticationConfig,

    /// Authorization configuration\
    /// The authorization value
    pub authorization: CanonicalAuthorizationConfig,

    /// Encryption configuration
    /// The encryption value
    pub encryption: CanonicalEncryptionConfig,

    /// Rate limiting configuration
    /// The rate limiting value
    pub rate_limiting: RateLimitingConfig,

    /// Session configuration
    /// The session value
    pub session: CanonicalSessionConfig,

    /// Multi-factor authentication configuration
    /// The mfa value
    pub mfa: CanonicalMfaConfig,

    /// Audit configuration
    /// The audit value
    pub audit: CanonicalAuditConfig,
}

impl Default for CanonicalSecurityConfig {
    fn default() -> Self {
        Self {
            enable_encryption: true,
            enable_hsm: false,
            authentication: CanonicalAuthenticationConfig::default(),
            authorization: CanonicalAuthorizationConfig::default(),
            encryption: CanonicalEncryptionConfig::default(),
            rate_limiting: RateLimitingConfig::default(),
            session: CanonicalSessionConfig::default(),
            mfa: CanonicalMfaConfig::default(),
            audit: CanonicalAuditConfig::default(),
        }
    }
}

impl CanonicalSecurityConfig {
    /// Returns [`Default`] security settings (suitable for development and CI).
    #[must_use]
    pub fn new() -> Self {
        Self::default()
    }

    /// Hardened presets: encryption, HSM, MFA, auditing, and strict sessions enabled.
    #[must_use]
    pub fn production() -> Self {
        Self {
            enable_encryption: true,
            enable_hsm: true,
            authentication: CanonicalAuthenticationConfig::production(),
            authorization: CanonicalAuthorizationConfig::production(),
            encryption: CanonicalEncryptionConfig::production(),
            rate_limiting: RateLimitingConfig::production(),
            session: CanonicalSessionConfig::production(),
            mfa: CanonicalMfaConfig::production(),
            audit: CanonicalAuditConfig::production(),
        }
    }

    /// Validate the security configuration
    /// Validates input
    ///
    /// # Errors
    ///
    /// Returns an error if authentication, authorization, encryption, session, MFA, or audit validation fails.
    pub fn validate(&self) -> Result<(), BearDogError> {
        self.authentication.validate()?;
        self.authorization.validate()?;
        self.encryption.validate()?;
        self.session.validate()?;
        self.mfa.validate()?;
        self.audit.validate()?;
        Ok(())
    }
}

/// Rate limiting configuration
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct RateLimitingConfig {
    /// Enable rate limiting
    /// Whether feature is enabled
    pub enabled: bool,

    /// Maximum requests per minute
    /// Number of `max_requests_per_minute`
    pub max_requests_per_minute: u32,

    /// Burst capacity
    /// Number of `burst_capacity`
    pub burst_capacity: u32,

    /// Rate limit window in seconds
    /// Number of `window_seconds`
    pub window_seconds: u32,
}

impl RateLimitingConfig {
    /// Default maximum requests per minute
    pub const DEFAULT_MAX_REQUESTS_PER_MINUTE: u32 = 100;

    /// Default burst capacity
    pub const DEFAULT_BURST_CAPACITY: u32 = 10;

    /// Default window in seconds
    pub const DEFAULT_WINDOW_SECS: u32 = 60;

    /// Create `RateLimitingConfig` with hardcoded defaults
    ///
    /// This method is deterministic and safe for concurrent use.
    /// No environment variables are read.
    pub const fn with_defaults() -> Self {
        Self {
            enabled: true,
            max_requests_per_minute: Self::DEFAULT_MAX_REQUESTS_PER_MINUTE,
            burst_capacity: Self::DEFAULT_BURST_CAPACITY,
            window_seconds: Self::DEFAULT_WINDOW_SECS,
        }
    }

    /// Create `RateLimitingConfig` from environment variables
    ///
    /// Reads configuration from environment, falling back to defaults.
    ///
    /// # Environment Variables
    /// - `BEARDOG_RATE_LIMIT_MAX_REQUESTS_PER_MIN`: Max requests per minute (default: 100)
    /// - `BEARDOG_RATE_LIMIT_BURST_CAPACITY`: Burst capacity (default: 10)
    /// - `BEARDOG_RATE_LIMIT_WINDOW_SECS`: Window in seconds (default: 60)
    pub fn from_env() -> Self {
        Self::from_env_provider(|k| std::env::var(k).ok())
    }

    /// Load from a custom environment provider (e.g. tests); production uses [`Self::from_env`].
    pub fn from_env_provider(get: impl Fn(&str) -> Option<String>) -> Self {
        Self {
            enabled: true,
            max_requests_per_minute: get("BEARDOG_RATE_LIMIT_MAX_REQUESTS_PER_MIN")
                .and_then(|v| v.parse().ok())
                .unwrap_or(Self::DEFAULT_MAX_REQUESTS_PER_MINUTE),
            burst_capacity: get("BEARDOG_RATE_LIMIT_BURST_CAPACITY")
                .and_then(|v| v.parse().ok())
                .unwrap_or(Self::DEFAULT_BURST_CAPACITY),
            window_seconds: get("BEARDOG_RATE_LIMIT_WINDOW_SECS")
                .and_then(|v| v.parse().ok())
                .unwrap_or(Self::DEFAULT_WINDOW_SECS),
        }
    }
}

impl Default for RateLimitingConfig {
    fn default() -> Self {
        Self::with_defaults()
    }
}

impl RateLimitingConfig {
    /// Create production rate limiting configuration
    #[must_use]
    pub fn production() -> Self {
        Self {
            enabled: true,
            max_requests_per_minute: std::env::var("BEARDOG_PROD_RATE_LIMIT_MAX_REQUESTS_PER_MIN")
                .ok()
                .and_then(|v| v.parse().ok())
                .unwrap_or(1000),
            burst_capacity: std::env::var("BEARDOG_PROD_RATE_LIMIT_BURST_CAPACITY")
                .ok()
                .and_then(|v| v.parse().ok())
                .unwrap_or(50),
            window_seconds: std::env::var("BEARDOG_PROD_RATE_LIMIT_WINDOW_SECS")
                .ok()
                .and_then(|v| v.parse().ok())
                .unwrap_or(60),
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::sync::Arc;

    fn sample_valid_security_config() -> CanonicalSecurityConfig {
        let mut c = CanonicalSecurityConfig::default();
        c.authentication.jwt_secret = Arc::from("01234567890123456789012345678901");
        c.authorization = CanonicalAuthorizationConfig::production();
        c
    }

    #[test]
    fn canonical_security_default_new_equivalent() {
        let a = CanonicalSecurityConfig::default();
        let b = CanonicalSecurityConfig::new();
        let ja = serde_json::to_value(&a).expect("serialize default");
        let jb = serde_json::to_value(&b).expect("serialize new");
        assert_eq!(ja, jb);
    }

    #[test]
    fn canonical_security_production_sets_flags() {
        let p = CanonicalSecurityConfig::production();
        assert!(p.enable_encryption);
        assert!(p.enable_hsm);
        let v = serde_json::to_value(&p).expect("serialize production");
        assert!(v.get("authentication").is_some());
    }

    #[test]
    fn canonical_security_validate_happy_path() {
        let c = sample_valid_security_config();
        c.validate().expect("sample config should validate");
    }

    #[test]
    fn canonical_security_default_validate_fails_on_placeholder_jwt() {
        let c = CanonicalSecurityConfig::default();
        assert!(c.validate().is_err());
    }

    #[test]
    fn rate_limiting_with_defaults_matches_default_impl() {
        let a = RateLimitingConfig::with_defaults();
        let b = RateLimitingConfig::default();
        assert_eq!(
            serde_json::to_value(&a).expect("a"),
            serde_json::to_value(&b).expect("b")
        );
    }

    #[test]
    fn rate_limiting_from_env_provider_overrides() {
        let r = RateLimitingConfig::from_env_provider(|k| match k {
            "BEARDOG_RATE_LIMIT_MAX_REQUESTS_PER_MIN" => Some("250".to_string()),
            "BEARDOG_RATE_LIMIT_BURST_CAPACITY" => Some("20".to_string()),
            "BEARDOG_RATE_LIMIT_WINDOW_SECS" => Some("120".to_string()),
            _ => None,
        });
        assert_eq!(r.max_requests_per_minute, 250);
        assert_eq!(r.burst_capacity, 20);
        assert_eq!(r.window_seconds, 120);
    }

    #[test]
    fn rate_limiting_production_has_nonzero_fields() {
        let r = RateLimitingConfig::production();
        assert!(r.max_requests_per_minute > 0);
        assert!(r.burst_capacity > 0);
        assert!(r.window_seconds > 0);
    }

    #[test]
    fn rate_limiting_serde_roundtrip() {
        let r = RateLimitingConfig::with_defaults();
        let v = serde_json::to_value(&r).expect("to json");
        let back: RateLimitingConfig = serde_json::from_value(v).expect("from json");
        assert_eq!(
            serde_json::to_value(&back).expect("back"),
            serde_json::to_value(&r).expect("orig")
        );
    }
}
