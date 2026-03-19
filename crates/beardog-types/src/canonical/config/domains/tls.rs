// SPDX-License-Identifier: AGPL-3.0-only

//! Canonical TLS Configuration
//!
//! This module provides the unified, canonical TLS/SSL configuration
//! for all BearDog components (network, discovery, HSM, etc.).
//!
//! ## Design Principles:
//! - **Single Source of Truth**: One TLS configuration across all domains
//! - **Security First**: Strong defaults with customization options
//! - **Verification Control**: Fine-grained peer verification settings
//! - **Protocol Flexibility**: Support for TLS 1.2, 1.3, and custom protocols
//!
//! ## Usage:
//!
//! ```rust
//! use beardog_types::canonical::config::domains::tls::{
//!     CanonicalTlsConfig, TlsVersion, TlsVerificationMode
//! };
//!
//! let config = CanonicalTlsConfig {
//!     enabled: true,
//!     cert_path: Some("/path/to/cert.pem".to_string()),
//!     key_path: Some("/path/to/key.pem".to_string()),
//!     min_version: TlsVersion::Tls12,
//!     ..Default::default()
//! };
//! ```

use serde::{Deserialize, Serialize};

/// Canonical TLS/SSL configuration for all BearDog components
///
/// This is the single, unified TLS configuration.
/// All domain-specific TlsConfigs should migrate to this.
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct CanonicalTlsConfig {
    /// Whether TLS is enabled
    pub enabled: bool,

    /// Path to certificate file (PEM format)
    pub cert_path: Option<String>,

    /// Path to private key file (PEM format)
    pub key_path: Option<String>,

    /// Path to CA certificate file for verification (optional)
    pub ca_path: Option<String>,

    /// Whether to verify peer certificates
    pub verify_peer: bool,

    /// TLS verification mode
    pub verification_mode: TlsVerificationMode,

    /// Minimum TLS protocol version
    pub min_version: TlsVersion,

    /// Maximum TLS protocol version (None = no maximum)
    pub max_version: Option<TlsVersion>,

    /// Allowed cipher suites (empty = use defaults)
    pub cipher_suites: Vec<String>,

    /// ALPN protocols to negotiate
    pub alpn_protocols: Vec<String>,

    /// Server Name Indication (SNI) hostname
    pub sni_hostname: Option<String>,

    /// Client certificate required (for mTLS)
    pub require_client_cert: bool,
}

/// Type alias for convenience (follows BearDog naming conventions)
pub type TlsConfig = CanonicalTlsConfig;

/// TLS protocol version
#[derive(Debug, Clone, Copy, Serialize, Deserialize, PartialEq, Eq, PartialOrd, Ord)]
#[serde(rename_all = "snake_case")]
pub enum TlsVersion {
    /// TLS 1.0 (deprecated, not recommended)
    Tls10,
    /// TLS 1.1 (deprecated, not recommended)
    Tls11,
    /// TLS 1.2 (minimum recommended)
    Tls12,
    /// TLS 1.3 (most secure, recommended)
    Tls13,
}

/// TLS peer verification mode
#[derive(Debug, Clone, Copy, Serialize, Deserialize, PartialEq, Eq)]
#[serde(rename_all = "snake_case")]
pub enum TlsVerificationMode {
    /// No verification (insecure, testing only)
    None,
    /// Verify certificate chain only
    VerifyChain,
    /// Verify certificate chain and hostname
    Full,
    /// Strict verification (recommended for production)
    Strict,
}

impl Default for CanonicalTlsConfig {
    fn default() -> Self {
        Self {
            enabled: true,
            cert_path: None,
            key_path: None,
            ca_path: None,
            verify_peer: true,
            verification_mode: TlsVerificationMode::Full,
            min_version: TlsVersion::Tls12,
            max_version: Some(TlsVersion::Tls13),
            cipher_suites: Vec::new(), // Use system defaults
            alpn_protocols: vec!["h2".to_string(), "http/1.1".to_string()],
            sni_hostname: None,
            require_client_cert: false,
        }
    }
}

impl Default for TlsVersion {
    fn default() -> Self {
        Self::Tls12
    }
}

impl Default for TlsVerificationMode {
    fn default() -> Self {
        Self::Full
    }
}

impl CanonicalTlsConfig {
    /// Create a new TLS config with strict verification (recommended for production)
    pub fn strict() -> Self {
        Self {
            verification_mode: TlsVerificationMode::Strict,
            min_version: TlsVersion::Tls13,
            require_client_cert: true,
            ..Default::default()
        }
    }

    /// Create a new TLS config with relaxed verification (for testing/development)
    pub fn relaxed() -> Self {
        Self {
            verification_mode: TlsVerificationMode::VerifyChain,
            verify_peer: true,
            min_version: TlsVersion::Tls12,
            ..Default::default()
        }
    }

    /// Create a new TLS config with no verification (INSECURE - testing only!)
    pub fn insecure() -> Self {
        Self {
            verification_mode: TlsVerificationMode::None,
            verify_peer: false,
            ..Default::default()
        }
    }

    /// Validate the TLS configuration
    pub fn validate(&self) -> Result<(), String> {
        if self.enabled {
            if self.cert_path.is_some() && self.key_path.is_none() {
                return Err("cert_path specified but key_path is missing".to_string());
            }
            if self.key_path.is_some() && self.cert_path.is_none() {
                return Err("key_path specified but cert_path is missing".to_string());
            }
            if self.verification_mode == TlsVerificationMode::Strict && self.ca_path.is_none() {
                return Err("Strict verification mode requires ca_path".to_string());
            }
            if let Some(max_version) = self.max_version {
                if max_version < self.min_version {
                    return Err(format!(
                        "max_version ({:?}) cannot be less than min_version ({:?})",
                        max_version, self.min_version
                    ));
                }
            }
        }
        Ok(())
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_default_config() {
        let config = CanonicalTlsConfig::default();
        assert!(config.enabled);
        assert!(config.verify_peer);
        assert_eq!(config.verification_mode, TlsVerificationMode::Full);
        assert_eq!(config.min_version, TlsVersion::Tls12);
        assert!(config.validate().is_ok());
    }

    #[test]
    fn test_strict_constructor() {
        let config = CanonicalTlsConfig::strict();
        assert_eq!(config.verification_mode, TlsVerificationMode::Strict);
        assert_eq!(config.min_version, TlsVersion::Tls13);
        assert!(config.require_client_cert);
    }

    #[test]
    fn test_relaxed_constructor() {
        let config = CanonicalTlsConfig::relaxed();
        assert_eq!(config.verification_mode, TlsVerificationMode::VerifyChain);
        assert!(config.verify_peer);
        assert_eq!(config.min_version, TlsVersion::Tls12);
    }

    #[test]
    fn test_insecure_constructor() {
        let config = CanonicalTlsConfig::insecure();
        assert_eq!(config.verification_mode, TlsVerificationMode::None);
        assert!(!config.verify_peer);
    }

    #[test]
    fn test_validation_cert_key_mismatch() {
        let mut config = CanonicalTlsConfig::default();
        config.cert_path = Some("/path/to/cert.pem".to_string());
        assert!(config.validate().is_err());

        config.key_path = Some("/path/to/key.pem".to_string());
        assert!(config.validate().is_ok());
    }

    #[test]
    fn test_validation_version_mismatch() {
        let mut config = CanonicalTlsConfig::default();
        config.min_version = TlsVersion::Tls13;
        config.max_version = Some(TlsVersion::Tls12);
        assert!(config.validate().is_err());

        config.max_version = Some(TlsVersion::Tls13);
        assert!(config.validate().is_ok());
    }

    #[test]
    fn test_validation_strict_mode_requires_ca() {
        let mut config = CanonicalTlsConfig::default();
        config.verification_mode = TlsVerificationMode::Strict;
        assert!(config.validate().is_err());

        config.ca_path = Some("/path/to/ca.pem".to_string());
        assert!(config.validate().is_ok());
    }

    #[test]
    fn test_tls_version_ordering() {
        assert!(TlsVersion::Tls10 < TlsVersion::Tls11);
        assert!(TlsVersion::Tls11 < TlsVersion::Tls12);
        assert!(TlsVersion::Tls12 < TlsVersion::Tls13);
    }

    #[test]
    fn test_serialization() {
        let config = CanonicalTlsConfig {
            enabled: true,
            cert_path: Some("/cert.pem".to_string()),
            key_path: Some("/key.pem".to_string()),
            ca_path: Some("/ca.pem".to_string()),
            verify_peer: true,
            verification_mode: TlsVerificationMode::Strict,
            min_version: TlsVersion::Tls13,
            max_version: Some(TlsVersion::Tls13),
            cipher_suites: vec!["TLS_AES_256_GCM_SHA384".to_string()],
            alpn_protocols: vec!["h2".to_string()],
            sni_hostname: Some("example.com".to_string()),
            require_client_cert: true,
        };

        let json = serde_json::to_string(&config).unwrap();
        let deserialized: CanonicalTlsConfig = serde_json::from_str(&json).unwrap();
        assert_eq!(deserialized, config);
    }

    #[test]
    fn test_type_alias() {
        let _config: TlsConfig = CanonicalTlsConfig::default();
    }
}

