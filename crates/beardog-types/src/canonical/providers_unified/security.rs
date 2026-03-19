// SPDX-License-Identifier: AGPL-3.0-only

// Security Configuration
//
// Provider security settings, authentication, authorization, and encryption configuration.

use serde::{Deserialize, Serialize};

/// Provider security configuration
#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct ProviderSecurityConfig {
    /// Security level
    /// The security level value
    pub security_level: SecurityLevel,

    /// Authentication configuration
    /// The authentication value
    pub authentication: AuthenticationConfig,

    /// Authorization configuration
    /// The authorization value
    pub authorization: AuthorizationConfig,

    /// Encryption configuration
    /// The encryption value
    pub encryption: EncryptionConfig,
}

/// Security level classification
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize, Default)]
pub enum SecurityLevel {
    /// Low variant
    Low,
    /// Medium variant
    Medium,
    /// High variant
    #[default]
    /// Represents high variant
    High,
    /// Critical variant
    Critical,
}

/// Authentication configuration
#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct AuthenticationConfig {
    /// Authentication enabled
    /// Whether feature is enabled
    pub enabled: bool,
}

/// Authorization configuration
#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct AuthorizationConfig {
    /// Authorization enabled
    /// Whether feature is enabled
    pub enabled: bool,
}

/// Encryption configuration
#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct EncryptionConfig {
    /// Encryption enabled
    /// Whether feature is enabled
    pub enabled: bool,
}
