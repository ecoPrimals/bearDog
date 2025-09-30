//! Security configuration
//!
//! This module contains security-related configuration types.

use serde::{Deserialize, Serialize};
use std::time::Duration;

/// Security configuration
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SecurityConfig {
    /// Enable security
    pub enabled: bool,
    /// Core security settings
    pub core: SecurityCoreConfig,
    /// Authentication settings
    pub authentication: AuthenticationConfig,
    /// Authorization settings
    pub authorization: AuthorizationConfig,
    /// Session settings
    pub session: SessionConfig,
    /// MFA settings
    pub mfa: MfaConfig,
    /// Encryption settings
    pub encryption: EncryptionConfig,
    /// Audit settings
    pub audit: AuditConfig,
}

/// Core security configuration
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SecurityCoreConfig {
    /// Security level
    pub security_level: u8,
    /// Enable encryption
    pub enable_encryption: bool,
}

/// Authentication configuration
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct AuthenticationConfig {
    /// Enable authentication
    pub enabled: bool,
    /// Authentication method
    pub method: String,
}

/// Authorization configuration
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct AuthorizationConfig {
    /// Enable authorization
    pub enabled: bool,
    /// Authorization method
    pub method: String,
}

/// Session configuration
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SessionConfig {
    /// Session timeout
    pub timeout: Duration,
    /// Enable session persistence
    pub persistent: bool,
}

/// Multi-factor authentication configuration
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct MfaConfig {
    /// Enable MFA
    pub enabled: bool,
    /// MFA method
    pub method: String,
}

/// Encryption configuration
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct EncryptionConfig {
    /// Encryption algorithm
    pub algorithm: String,
    /// Key size
    pub key_size: usize,
}

/// Audit configuration
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct AuditConfig {
    /// Enable auditing
    pub enabled: bool,
    /// Audit log path
    pub log_path: String,
}

impl Default for SecurityConfig {
    fn default() -> Self {
        Self {
            enabled: true,
            core: SecurityCoreConfig::default(),
            authentication: AuthenticationConfig::default(),
            authorization: AuthorizationConfig::default(),
            session: SessionConfig::default(),
            mfa: MfaConfig::default(),
            encryption: EncryptionConfig::default(),
            audit: AuditConfig::default(),
        }
    }
}

impl Default for SecurityCoreConfig {
    fn default() -> Self {
        Self {
            security_level: 5,
            enable_encryption: true,
        }
    }
}

impl Default for AuthenticationConfig {
    fn default() -> Self {
        Self {
            enabled: true,
            method: "jwt".to_string(),
        }
    }
}

impl Default for AuthorizationConfig {
    fn default() -> Self {
        Self {
            enabled: true,
            method: "rbac".to_string(),
        }
    }
}

impl Default for SessionConfig {
    fn default() -> Self {
        Self {
            timeout: Duration::from_secs(3600), // 1 hour
            persistent: false,
        }
    }
}

impl Default for MfaConfig {
    fn default() -> Self {
        Self {
            enabled: false,
            method: "totp".to_string(),
        }
    }
}

impl Default for EncryptionConfig {
    fn default() -> Self {
        Self {
            algorithm: "AES-256-GCM".to_string(),
            key_size: 256,
        }
    }
}

impl Default for AuditConfig {
    fn default() -> Self {
        Self {
            enabled: true,
            log_path: "./logs/audit.log".to_string(),
        }
    }
} 