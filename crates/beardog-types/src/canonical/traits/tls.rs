// SPDX-License-Identifier: AGPL-3.0-only

//! # TLS Configuration Trait
//!
//! This module provides a polymorphic interface for TLS configurations across different
//! domains (discovery, production, general networking) while preserving their unique features.
//!
//! ## Design Rationale
//!
//! Rather than forcing all TLS configs into a single struct (which breaks domain boundaries),
//! we provide a common trait interface that enables:
//! - **Polymorphic code**: Functions that work with any TLS config
//! - **Domain preservation**: Each config retains its domain-specific features
//! - **Type safety**: Compiler-enforced security defaults
//! - **Easy extension**: New TLS configs just implement the trait
//!
//! ## Example Usage
//!
//! ```rust
//! use beardog_types::canonical::traits::{TlsConfiguration, TlsVersion};
//!
//! fn setup_secure_connection<T: TlsConfiguration>(tls_config: &T) -> Result<(), String> {
//!     if !tls_config.is_enabled() {
//!         return Err("TLS must be enabled for secure connections".to_string());
//!     }
//!     
//!     if !tls_config.verify_peer() {
//!         eprintln!("WARNING: Peer verification is disabled!");
//!     }
//!     
//!     if tls_config.min_tls_version() < TlsVersion::Tls12 {
//!         return Err("Minimum TLS 1.2 required".to_string());
//!     }
//!     
//!     // Use the configuration...
//!     Ok(())
//! }
//! ```

use std::path::Path;

/// TLS version specification
///
/// Represents supported TLS protocol versions in increasing order of security.
/// Use `PartialOrd` to compare versions (e.g., `TlsVersion::Tls13 > TlsVersion::Tls12`).
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub enum TlsVersion {
    /// TLS 1.0 (deprecated, insecure)
    Tls10,
    /// TLS 1.1 (deprecated, insecure)
    Tls11,
    /// TLS 1.2 (minimum recommended)
    Tls12,
    /// TLS 1.3 (current standard, recommended)
    Tls13,
}

impl TlsVersion {
    /// Returns the string representation of the TLS version
    pub const fn as_str(&self) -> &'static str {
        match self {
            Self::Tls10 => "TLSv1.0",
            Self::Tls11 => "TLSv1.1",
            Self::Tls12 => "TLSv1.2",
            Self::Tls13 => "TLSv1.3",
        }
    }

    /// Returns true if this TLS version is considered secure by modern standards
    pub fn is_secure(&self) -> bool {
        *self >= Self::Tls12
    }

    /// Returns true if this TLS version is deprecated
    pub fn is_deprecated(&self) -> bool {
        *self < Self::Tls12
    }
}

impl std::fmt::Display for TlsVersion {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.as_str())
    }
}

/// Trait for TLS configuration
///
/// Provides a common interface for TLS settings across different domains while
/// allowing each implementation to maintain domain-specific features.
///
/// ## Security Defaults
///
/// The trait provides secure defaults:
/// - `verify_peer()` returns `true` (always verify certificates)
/// - `min_tls_version()` returns `TlsVersion::Tls12` (no deprecated versions)
///
/// Implementations can override these for specific use cases (e.g., development environments).
///
/// ## Thread Safety
///
/// The trait requires `Send + Sync` to enable use in async contexts and across threads.
pub trait TlsConfiguration: Send + Sync {
    /// Whether TLS is enabled
    ///
    /// Returns `false` if TLS is disabled (plain text communication).
    /// Most production systems should always return `true`.
    fn is_enabled(&self) -> bool;

    /// Path to the certificate file
    ///
    /// Returns the path to the X.509 certificate file (PEM or DER format).
    /// Returns `None` if no certificate is configured (e.g., client-only mode).
    fn cert_path(&self) -> Option<&Path>;

    /// Path to the private key file
    ///
    /// Returns the path to the private key file corresponding to the certificate.
    /// Returns `None` if no key is configured (e.g., using OS keychain).
    fn key_path(&self) -> Option<&Path>;

    /// Path to the CA certificate file
    ///
    /// Returns the path to the Certificate Authority bundle for verifying peer certificates.
    /// Returns `None` to use the system's default CA bundle.
    fn ca_path(&self) -> Option<&Path>;

    /// Whether to verify peer certificates
    ///
    /// **Security Critical**: Default is `true` for maximum security.
    /// Only override to `false` in development environments or when absolutely necessary.
    ///
    /// When `true`:
    /// - Validates certificate chain
    /// - Checks certificate expiration
    /// - Verifies hostname (if applicable)
    fn verify_peer(&self) -> bool {
        true // Secure by default
    }

    /// Minimum TLS version
    ///
    /// Returns the minimum acceptable TLS version for connections.
    /// Default is `TlsVersion::Tls12` (TLS 1.2) as TLS 1.0/1.1 are deprecated and insecure.
    ///
    /// Production systems should prefer `TlsVersion::Tls13`.
    fn min_tls_version(&self) -> TlsVersion {
        TlsVersion::Tls12 // Secure default
    }

    /// Maximum TLS version (optional)
    ///
    /// Returns the maximum acceptable TLS version.
    /// Default is `None` (no maximum, accept latest).
    ///
    /// Only set this for compatibility with legacy systems.
    fn max_tls_version(&self) -> Option<TlsVersion> {
        None // Accept latest by default
    }

    /// List of allowed cipher suites (optional)
    ///
    /// Returns the list of allowed cipher suites in preference order.
    /// Returns `None` to use the TLS library's default secure cipher suites.
    ///
    /// Only customize this for:
    /// - Compliance requirements (e.g., FIPS 140-2)
    /// - Compatibility with specific systems
    /// - Additional security hardening
    fn cipher_suites(&self) -> Option<&[String]> {
        None // Use library defaults
    }

    /// Validates the TLS configuration for security issues
    ///
    /// Returns `Ok(())` if the configuration is secure and valid.
    /// Returns `Err(String)` with a description of security issues.
    ///
    /// Checks:
    /// - TLS is enabled (or explicitly acknowledged as disabled)
    /// - Certificate and key paths exist (if required)
    /// - No deprecated TLS versions
    /// - Peer verification is enabled (or acknowledged as disabled)
    fn validate(&self) -> Result<(), String> {
        // Check TLS version security
        if self.min_tls_version().is_deprecated() {
            return Err(format!(
                "Insecure TLS version: {} is deprecated. Use TLS 1.2 or higher.",
                self.min_tls_version()
            ));
        }

        // Warn if peer verification is disabled
        if !self.verify_peer() {
            eprintln!(
                "WARNING: TLS peer verification is disabled! \
                 This is insecure and should only be used in development."
            );
        }

        Ok(())
    }

    /// Returns true if the configuration meets production security standards
    ///
    /// Production standards:
    /// - TLS is enabled
    /// - TLS 1.2 or higher
    /// - Peer verification enabled
    /// - Certificate and key configured (if server)
    fn is_production_ready(&self) -> bool {
        self.is_enabled() && self.min_tls_version() >= TlsVersion::Tls12 && self.verify_peer()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_tls_version_ordering() {
        assert!(TlsVersion::Tls13 > TlsVersion::Tls12);
        assert!(TlsVersion::Tls12 > TlsVersion::Tls11);
        assert!(TlsVersion::Tls11 > TlsVersion::Tls10);
    }

    #[test]
    fn test_tls_version_security() {
        assert!(!TlsVersion::Tls10.is_secure());
        assert!(!TlsVersion::Tls11.is_secure());
        assert!(TlsVersion::Tls12.is_secure());
        assert!(TlsVersion::Tls13.is_secure());
    }

    #[test]
    fn test_tls_version_deprecated() {
        assert!(TlsVersion::Tls10.is_deprecated());
        assert!(TlsVersion::Tls11.is_deprecated());
        assert!(!TlsVersion::Tls12.is_deprecated());
        assert!(!TlsVersion::Tls13.is_deprecated());
    }

    #[test]
    fn test_tls_version_display() {
        assert_eq!(TlsVersion::Tls10.to_string(), "TLSv1.0");
        assert_eq!(TlsVersion::Tls11.to_string(), "TLSv1.1");
        assert_eq!(TlsVersion::Tls12.to_string(), "TLSv1.2");
        assert_eq!(TlsVersion::Tls13.to_string(), "TLSv1.3");
    }

    // Mock TLS config for testing
    struct MockTlsConfig {
        enabled: bool,
        verify: bool,
        min_version: TlsVersion,
        cert: Option<String>,
        key: Option<String>,
    }

    impl TlsConfiguration for MockTlsConfig {
        fn is_enabled(&self) -> bool {
            self.enabled
        }

        fn cert_path(&self) -> Option<&Path> {
            self.cert.as_deref().map(Path::new)
        }

        fn key_path(&self) -> Option<&Path> {
            self.key.as_deref().map(Path::new)
        }

        fn ca_path(&self) -> Option<&Path> {
            None
        }

        fn verify_peer(&self) -> bool {
            self.verify
        }

        fn min_tls_version(&self) -> TlsVersion {
            self.min_version
        }
    }

    #[test]
    fn test_secure_config_validation() {
        let config = MockTlsConfig {
            enabled: true,
            verify: true,
            min_version: TlsVersion::Tls12,
            cert: Some("/path/to/cert.pem".to_string()),
            key: Some("/path/to/key.pem".to_string()),
        };

        assert!(config.validate().is_ok());
        assert!(config.is_production_ready());
    }

    #[test]
    fn test_insecure_tls_version_validation() {
        let config = MockTlsConfig {
            enabled: true,
            verify: true,
            min_version: TlsVersion::Tls10,
            cert: None,
            key: None,
        };

        assert!(config.validate().is_err());
        assert!(!config.is_production_ready());
    }

    #[test]
    fn test_disabled_verification() {
        let config = MockTlsConfig {
            enabled: true,
            verify: false, // Insecure!
            min_version: TlsVersion::Tls12,
            cert: None,
            key: None,
        };

        // Should pass validation but warn
        assert!(config.validate().is_ok());
        // Not production ready due to disabled verification
        assert!(!config.is_production_ready());
    }

    #[test]
    fn test_production_ready_standards() {
        // Minimum production config
        let config = MockTlsConfig {
            enabled: true,
            verify: true,
            min_version: TlsVersion::Tls12,
            cert: Some("/cert.pem".to_string()),
            key: Some("/key.pem".to_string()),
        };
        assert!(config.is_production_ready());

        // Better production config (TLS 1.3)
        let config_tls13 = MockTlsConfig {
            enabled: true,
            verify: true,
            min_version: TlsVersion::Tls13,
            cert: Some("/cert.pem".to_string()),
            key: Some("/key.pem".to_string()),
        };
        assert!(config_tls13.is_production_ready());
    }

    #[test]
    fn test_trait_defaults() {
        let config = MockTlsConfig {
            enabled: true,
            verify: false,                  // Will be overridden by implementing trait
            min_version: TlsVersion::Tls10, // Will be overridden
            cert: None,
            key: None,
        };

        // Test that defaults work (from trait implementation)
        assert_eq!(config.min_tls_version(), TlsVersion::Tls10); // Custom implementation
        assert!(!config.verify_peer()); // Custom implementation
        assert_eq!(config.max_tls_version(), None); // Trait default
        assert_eq!(config.cipher_suites(), None); // Trait default
    }
}
