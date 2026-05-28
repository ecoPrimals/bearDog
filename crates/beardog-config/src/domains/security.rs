// SPDX-License-Identifier: AGPL-3.0-or-later

//! Concurrent-Safe Security Configuration Module
//!
//! Security configuration and policies for `BearDog`.
//!
//! ## Design Pattern: Explicit Environment Loading
//!
//! To ensure **concurrent safety** and **testability**, this module separates:
//! - **Static defaults** (`Default` trait) - Pure, no environment reads
//! - **Environment loading** (`from_env()`) - Explicit environment variable reads
//! - **Flexible construction** (`builder()`) - Testing without env var pollution
//!
//! TLS version strings and localhost-bypass policy are **configuration-driven**; the literal
//! `DEFAULT_MIN_TLS_VERSION_FALLBACK` is a documented last resort when `BEARDOG_MIN_TLS_VERSION`
//! is unset (see `ZERO_HARDCODING_SPECIFICATION`).

use crate::env_keys;
use crate::error::ConfigResult;
use serde::{Deserialize, Serialize};

/// **Fallback** minimum TLS version when `BEARDOG_MIN_TLS_VERSION` is unset.
pub const DEFAULT_MIN_TLS_VERSION_FALLBACK: &str = "1.2";

fn env_bool(key: &str, default: bool) -> bool {
    match std::env::var(key) {
        Ok(v) => matches!(v.to_ascii_lowercase().as_str(), "1" | "true" | "yes" | "on"),
        Err(_) => default,
    }
}

/// Security configuration and policies
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
pub struct SecurityConfig {
    /// Enable strict mode (enforces all security policies)
    pub strict_mode: bool,

    /// Require mutual TLS for connections
    pub require_mtls: bool,

    /// Minimum TLS version
    pub min_tls_version: String,

    /// Allow loopback connections without authentication (**fallback** policy; override with `BEARDOG_ALLOW_LOCALHOST_BYPASS`)
    pub allow_localhost_bypass: bool,

    /// Enable audit logging
    pub enable_audit_log: bool,

    /// Enable rate limiting
    pub enable_rate_limiting: bool,

    /// Block suspicious IPs automatically
    pub auto_block_suspicious_ips: bool,

    /// Require authentication for all endpoints
    pub require_authentication: bool,
}

impl SecurityConfig {
    /// Pure static defaults (no environment variable reads)
    pub const fn const_defaults() -> Self {
        Self {
            strict_mode: false,
            require_mtls: false,
            min_tls_version: String::new(), // Will be set to "1.2" in Default
            allow_localhost_bypass: true,
            enable_audit_log: true,
            enable_rate_limiting: true,
            auto_block_suspicious_ips: false,
            require_authentication: true,
        }
    }

    /// Load configuration from environment variables with fallback to [`SecurityConfig::default`]
    ///
    /// Recognized variables: `BEARDOG_STRICT_MODE`, `BEARDOG_REQUIRE_MTLS`, `BEARDOG_MIN_TLS_VERSION`,
    /// `BEARDOG_ALLOW_LOCALHOST_BYPASS`, `BEARDOG_ENABLE_AUDIT_LOG`, `BEARDOG_ENABLE_RATE_LIMITING`,
    /// `BEARDOG_AUTO_BLOCK_SUSPICIOUS_IPS`, `BEARDOG_REQUIRE_AUTHENTICATION` (truthy: `1`, `true`, `yes`, `on`, case-insensitive).
    pub fn from_env() -> Self {
        let defaults = Self::default();
        Self {
            strict_mode: env_bool(env_keys::ENV_STRICT_MODE, defaults.strict_mode),
            require_mtls: env_bool(env_keys::ENV_REQUIRE_MTLS, defaults.require_mtls),
            min_tls_version: std::env::var(env_keys::ENV_MIN_TLS_VERSION)
                .ok()
                .filter(|s| !s.is_empty())
                .unwrap_or_else(|| DEFAULT_MIN_TLS_VERSION_FALLBACK.to_string()),
            allow_localhost_bypass: env_bool(
                env_keys::ENV_ALLOW_LOCALHOST_BYPASS,
                defaults.allow_localhost_bypass,
            ),
            enable_audit_log: env_bool(env_keys::ENV_ENABLE_AUDIT_LOG, defaults.enable_audit_log),
            enable_rate_limiting: env_bool(
                env_keys::ENV_ENABLE_RATE_LIMITING,
                defaults.enable_rate_limiting,
            ),
            auto_block_suspicious_ips: env_bool(
                env_keys::ENV_AUTO_BLOCK_SUSPICIOUS_IPS,
                defaults.auto_block_suspicious_ips,
            ),
            require_authentication: env_bool(
                env_keys::ENV_REQUIRE_AUTHENTICATION,
                defaults.require_authentication,
            ),
        }
    }

    /// Create a builder for flexible configuration construction
    pub fn builder() -> SecurityConfigBuilder {
        SecurityConfigBuilder::new()
    }

    /// Validate security configuration
    ///
    /// # Errors
    ///
    /// Currently always succeeds; the `Result` type is reserved for future policy checks.
    pub const fn validate(&self) -> ConfigResult<()> {
        Ok(())
    }
}

impl Default for SecurityConfig {
    fn default() -> Self {
        Self {
            strict_mode: false,
            require_mtls: false,
            min_tls_version: DEFAULT_MIN_TLS_VERSION_FALLBACK.to_string(),
            allow_localhost_bypass: true,
            enable_audit_log: true,
            enable_rate_limiting: true,
            auto_block_suspicious_ips: false,
            require_authentication: true,
        }
    }
}

/// Builder for `SecurityConfig`
#[derive(Debug, Default)]
pub struct SecurityConfigBuilder {
    strict_mode: Option<bool>,
    require_mtls: Option<bool>,
    min_tls_version: Option<String>,
    allow_localhost_bypass: Option<bool>,
    enable_audit_log: Option<bool>,
    enable_rate_limiting: Option<bool>,
    auto_block_suspicious_ips: Option<bool>,
    require_authentication: Option<bool>,
}

impl SecurityConfigBuilder {
    /// Starts a builder; unset toggles use [`SecurityConfig::default`].
    pub fn new() -> Self {
        Self::default()
    }

    /// Enforces stricter validation and rejects ambiguous security posture.
    pub const fn strict_mode(mut self, enabled: bool) -> Self {
        self.strict_mode = Some(enabled);
        self
    }

    /// Requires mutual TLS for API clients when supported by the deployment.
    pub const fn require_mtls(mut self, enabled: bool) -> Self {
        self.require_mtls = Some(enabled);
        self
    }

    /// Minimum TLS protocol version string accepted by listeners (e.g. `1.2`).
    pub fn min_tls_version(mut self, version: String) -> Self {
        self.min_tls_version = Some(version);
        self
    }

    /// Permits relaxed checks for loopback clients (useful in dev, risky in prod).
    pub const fn allow_localhost_bypass(mut self, enabled: bool) -> Self {
        self.allow_localhost_bypass = Some(enabled);
        self
    }

    /// Persists security-relevant actions to the audit subsystem.
    pub const fn enable_audit_log(mut self, enabled: bool) -> Self {
        self.enable_audit_log = Some(enabled);
        self
    }

    /// Applies request rate limits to mitigate abuse and credential stuffing.
    pub const fn enable_rate_limiting(mut self, enabled: bool) -> Self {
        self.enable_rate_limiting = Some(enabled);
        self
    }

    /// Automatically blocks source IPs that trip threat heuristics.
    pub const fn auto_block_suspicious_ips(mut self, enabled: bool) -> Self {
        self.auto_block_suspicious_ips = Some(enabled);
        self
    }

    /// Denies anonymous access to protected API surface area.
    pub const fn require_authentication(mut self, enabled: bool) -> Self {
        self.require_authentication = Some(enabled);
        self
    }

    /// Builds the final [`SecurityConfig`].
    pub fn build(self) -> SecurityConfig {
        let defaults = SecurityConfig::default();

        SecurityConfig {
            strict_mode: self.strict_mode.unwrap_or(defaults.strict_mode),
            require_mtls: self.require_mtls.unwrap_or(defaults.require_mtls),
            min_tls_version: self.min_tls_version.unwrap_or(defaults.min_tls_version),
            allow_localhost_bypass: self
                .allow_localhost_bypass
                .unwrap_or(defaults.allow_localhost_bypass),
            enable_audit_log: self.enable_audit_log.unwrap_or(defaults.enable_audit_log),
            enable_rate_limiting: self
                .enable_rate_limiting
                .unwrap_or(defaults.enable_rate_limiting),
            auto_block_suspicious_ips: self
                .auto_block_suspicious_ips
                .unwrap_or(defaults.auto_block_suspicious_ips),
            require_authentication: self
                .require_authentication
                .unwrap_or(defaults.require_authentication),
        }
    }
}

#[cfg(test)]
#[path = "security_comprehensive_tests.rs"]
mod security_comprehensive_tests;

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_default_security() {
        let config = SecurityConfig::default();
        assert!(config.validate().is_ok());
        assert!(!config.strict_mode);
        assert!(config.enable_audit_log);
        assert_eq!(config.min_tls_version, DEFAULT_MIN_TLS_VERSION_FALLBACK);
    }

    #[test]
    fn test_strict_mode() {
        let config = SecurityConfig::builder().strict_mode(true).build();
        assert!(config.validate().is_ok());
        assert!(config.strict_mode);
    }

    #[test]
    fn test_builder() {
        let config = SecurityConfig::builder()
            .require_mtls(true)
            .min_tls_version("1.3".to_string())
            .enable_audit_log(false)
            .build();

        assert!(config.require_mtls);
        assert_eq!(config.min_tls_version, "1.3");
        assert!(!config.enable_audit_log);
    }

    #[test]
    fn test_production_config() {
        let config = SecurityConfig::builder()
            .strict_mode(true)
            .require_mtls(true)
            .auto_block_suspicious_ips(true)
            .allow_localhost_bypass(false)
            .build();

        assert!(config.strict_mode);
        assert!(config.require_mtls);
        assert!(config.auto_block_suspicious_ips);
        assert!(!config.allow_localhost_bypass);
    }
}
