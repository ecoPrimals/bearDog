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
/// This is the master security configuration that consolidates all security
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
    /// Create a new canonical security configuration with secure defaults
    #[must_use]
    /// Creates a new instance
    pub fn new() -> Self {
        Self::default()
    }

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
    /// Validates input
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

impl Default for RateLimitingConfig {
    fn default() -> Self {
        Self {
            enabled: true,
            max_requests_per_minute: 100,
            burst_capacity: 10,
            window_seconds: 60,
        }
    }
}

impl RateLimitingConfig {
    /// Create production rate limiting configuration
    #[must_use]
    pub fn production() -> Self {
        Self {
            enabled: true,
            max_requests_per_minute: 1000,
            burst_capacity: 50,
            window_seconds: 60,
        }
    }
}
