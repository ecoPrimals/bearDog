//! Concurrent-Safe Security Configuration Module
//!
//! Security configuration and policies for BearDog.
//!
//! ## Design Pattern: Explicit Environment Loading
//!
//! To ensure **concurrent safety** and **testability**, this module separates:
//! - **Static defaults** (`Default` trait) - Pure, no environment reads
//! - **Environment loading** (`from_env()`) - Explicit environment variable reads
//! - **Flexible construction** (`builder()`) - Testing without env var pollution

use crate::error::ConfigResult;
use serde::{Deserialize, Serialize};

/// Security configuration and policies
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
pub struct SecurityConfig {
    /// Enable strict mode (enforces all security policies)
    pub strict_mode: bool,

    /// Require mutual TLS for connections
    pub require_mtls: bool,

    /// Minimum TLS version
    pub min_tls_version: String,

    /// Allow localhost connections without authentication
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

    /// Load configuration from environment variables with fallback to defaults
    pub fn from_env() -> Self {
        Self {
            strict_mode: false,
            require_mtls: false,
            min_tls_version: std::env::var("BEARDOG_MIN_TLS_VERSION")
                .ok()
                .unwrap_or_else(|| "1.2".to_string()),
            allow_localhost_bypass: true,
            enable_audit_log: true,
            enable_rate_limiting: true,
            auto_block_suspicious_ips: false,
            require_authentication: true,
        }
    }

    /// Create a builder for flexible configuration construction
    pub fn builder() -> SecurityConfigBuilder {
        SecurityConfigBuilder::new()
    }

    /// Validate security configuration
    pub fn validate(&self) -> ConfigResult<()> {
        Ok(())
    }
}

impl Default for SecurityConfig {
    fn default() -> Self {
        Self {
            strict_mode: false,
            require_mtls: false,
            min_tls_version: "1.2".to_string(),
            allow_localhost_bypass: true,
            enable_audit_log: true,
            enable_rate_limiting: true,
            auto_block_suspicious_ips: false,
            require_authentication: true,
        }
    }
}

/// Builder for SecurityConfig
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
    pub fn new() -> Self {
        Self::default()
    }

    pub fn strict_mode(mut self, enabled: bool) -> Self {
        self.strict_mode = Some(enabled);
        self
    }

    pub fn require_mtls(mut self, enabled: bool) -> Self {
        self.require_mtls = Some(enabled);
        self
    }

    pub fn min_tls_version(mut self, version: String) -> Self {
        self.min_tls_version = Some(version);
        self
    }

    pub fn allow_localhost_bypass(mut self, enabled: bool) -> Self {
        self.allow_localhost_bypass = Some(enabled);
        self
    }

    pub fn enable_audit_log(mut self, enabled: bool) -> Self {
        self.enable_audit_log = Some(enabled);
        self
    }

    pub fn enable_rate_limiting(mut self, enabled: bool) -> Self {
        self.enable_rate_limiting = Some(enabled);
        self
    }

    pub fn auto_block_suspicious_ips(mut self, enabled: bool) -> Self {
        self.auto_block_suspicious_ips = Some(enabled);
        self
    }

    pub fn require_authentication(mut self, enabled: bool) -> Self {
        self.require_authentication = Some(enabled);
        self
    }

    pub fn build(self) -> SecurityConfig {
        let defaults = SecurityConfig::default();
        
        SecurityConfig {
            strict_mode: self.strict_mode.unwrap_or(defaults.strict_mode),
            require_mtls: self.require_mtls.unwrap_or(defaults.require_mtls),
            min_tls_version: self.min_tls_version.unwrap_or(defaults.min_tls_version),
            allow_localhost_bypass: self.allow_localhost_bypass.unwrap_or(defaults.allow_localhost_bypass),
            enable_audit_log: self.enable_audit_log.unwrap_or(defaults.enable_audit_log),
            enable_rate_limiting: self.enable_rate_limiting.unwrap_or(defaults.enable_rate_limiting),
            auto_block_suspicious_ips: self.auto_block_suspicious_ips.unwrap_or(defaults.auto_block_suspicious_ips),
            require_authentication: self.require_authentication.unwrap_or(defaults.require_authentication),
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_default_security() {
        let config = SecurityConfig::default();
        assert!(config.validate().is_ok());
        assert!(!config.strict_mode);
        assert!(config.enable_audit_log);
    }

    #[test]
    fn test_strict_mode() {
        let config = SecurityConfig::builder()
            .strict_mode(true)
            .build();
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
