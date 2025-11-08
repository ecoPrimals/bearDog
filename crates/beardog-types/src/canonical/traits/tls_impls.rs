//! # TLS Configuration Trait Implementations
//!
//! This module provides `TlsConfiguration` trait implementations for various
//! TLS config structs across the codebase.
//!
//! ## Implemented Configs
//!
//! 1. **Discovery TLS** (`beardog-core::TlsConfig`) - Service discovery TLS
//! 2. **Provider TLS** (`beardog-types::TlsConfig`) - Provider connection TLS  
//! 3. **Network Security TLS** (`TlsConfiguration` struct) - Network-level TLS

use super::{TlsConfiguration as TlsConfigTrait, TlsVersion};
use std::path::Path;

// Re-export the structs for convenience
pub use crate::canonical::config::domains::network::security::TlsConfiguration as NetworkSecurityTls;
pub use crate::canonical::providers_unified::connection::{TlsConfig as ProviderTlsConfig, TlsVersion as ProviderTlsVersion};

/// Implementation for Network Security TLS Configuration
impl TlsConfigTrait for NetworkSecurityTls {
    fn is_enabled(&self) -> bool {
        self.enabled
    }

    fn cert_path(&self) -> Option<&Path> {
        self.cert_path.as_deref().map(Path::new)
    }

    fn key_path(&self) -> Option<&Path> {
        self.key_path.as_deref().map(Path::new)
    }

    fn ca_path(&self) -> Option<&Path> {
        None // Not supported in this config
    }

    fn verify_peer(&self) -> bool {
        use crate::canonical::config::domains::network::security::TlsVerificationMode;
        match self.verification_mode {
            TlsVerificationMode::None => false,
            TlsVerificationMode::Basic | TlsVerificationMode::Full => true,
        }
    }

    fn min_tls_version(&self) -> TlsVersion {
        // This config doesn't specify version, use secure default
        TlsVersion::Tls12
    }
}

/// Implementation for Provider TLS Configuration
impl TlsConfigTrait for ProviderTlsConfig {
    fn is_enabled(&self) -> bool {
        self.enabled
    }

    fn cert_path(&self) -> Option<&Path> {
        self.cert_path.as_deref().map(Path::new)
    }

    fn key_path(&self) -> Option<&Path> {
        self.key_path.as_deref().map(Path::new)
    }

    fn ca_path(&self) -> Option<&Path> {
        self.ca_path.as_deref().map(Path::new)
    }

    fn verify_peer(&self) -> bool {
        self.verify_certs
    }

    fn min_tls_version(&self) -> TlsVersion {
        match self.version {
            ProviderTlsVersion::V1_2 => TlsVersion::Tls12,
            ProviderTlsVersion::V1_3 => TlsVersion::Tls13,
        }
    }

    fn cipher_suites(&self) -> Option<&[String]> {
        if self.cipher_suites.is_empty() {
            None
        } else {
            Some(&self.cipher_suites)
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::canonical::config::domains::network::security::TlsVerificationMode;

    #[test]
    fn test_network_security_tls_implementation() {
        let config = NetworkSecurityTls {
            enabled: true,
            verification_mode: TlsVerificationMode::Full,
            cert_path: Some("/path/to/cert.pem".to_string()),
            key_path: Some("/path/to/key.pem".to_string()),
        };

        assert!(config.is_enabled());
        assert!(config.verify_peer());
        assert_eq!(config.min_tls_version(), TlsVersion::Tls12);
        assert_eq!(
            config.cert_path().map(|p| p.to_str().unwrap()),
            Some("/path/to/cert.pem")
        );
        assert!(config.is_production_ready());
    }

    #[test]
    fn test_network_security_tls_no_verification() {
        let config = NetworkSecurityTls {
            enabled: true,
            verification_mode: TlsVerificationMode::None,
            cert_path: Some("/cert.pem".to_string()),
            key_path: Some("/key.pem".to_string()),
        };

        assert!(config.is_enabled());
        assert!(!config.verify_peer());
        assert!(!config.is_production_ready()); // Not production ready without verification
    }

    #[test]
    fn test_provider_tls_implementation() {
        let config = ProviderTlsConfig {
            enabled: true,
            version: ProviderTlsVersion::V1_3,
            cert_path: Some("/path/to/cert.pem".to_string()),
            key_path: Some("/path/to/key.pem".to_string()),
            ca_path: Some("/path/to/ca.pem".to_string()),
            verify_certs: true,
            verify_hostname: true,
            cipher_suites: vec!["TLS_AES_256_GCM_SHA384".to_string()],
        };

        assert!(config.is_enabled());
        assert!(config.verify_peer());
        assert_eq!(config.min_tls_version(), TlsVersion::Tls13);
        assert_eq!(
            config.ca_path().map(|p| p.to_str().unwrap()),
            Some("/path/to/ca.pem")
        );
        assert_eq!(config.cipher_suites().map(|s| s.len()), Some(1));
        assert!(config.is_production_ready());
    }

    #[test]
    fn test_provider_tls_tls12() {
        let config = ProviderTlsConfig {
            enabled: true,
            version: ProviderTlsVersion::V1_2,
            cert_path: Some("/cert.pem".to_string()),
            key_path: Some("/key.pem".to_string()),
            ca_path: None,
            verify_certs: true,
            verify_hostname: false,
            cipher_suites: vec![],
        };

        assert_eq!(config.min_tls_version(), TlsVersion::Tls12);
        assert_eq!(config.cipher_suites(), None); // Empty vec returns None
        assert!(config.is_production_ready());
    }

    #[test]
    fn test_provider_tls_disabled_verification() {
        let config = ProviderTlsConfig {
            enabled: true,
            version: ProviderTlsVersion::V1_3,
            cert_path: None,
            key_path: None,
            ca_path: None,
            verify_certs: false,
            verify_hostname: false,
            cipher_suites: vec![],
        };

        assert!(config.is_enabled());
        assert!(!config.verify_peer());
        assert!(!config.is_production_ready());
    }

    #[test]
    fn test_polymorphic_usage() {
        // Test that both configs work with generic functions

        fn check_tls_security<T: TlsConfigTrait>(config: &T) -> bool {
            config.is_enabled()
                && config.verify_peer()
                && config.min_tls_version() >= TlsVersion::Tls12
        }

        let network_config = NetworkSecurityTls {
            enabled: true,
            verification_mode: TlsVerificationMode::Full,
            cert_path: Some("/cert.pem".to_string()),
            key_path: Some("/key.pem".to_string()),
        };

        let provider_config = ProviderTlsConfig {
            enabled: true,
            version: ProviderTlsVersion::V1_3,
            cert_path: Some("/cert.pem".to_string()),
            key_path: Some("/key.pem".to_string()),
            ca_path: Some("/ca.pem".to_string()),
            verify_certs: true,
            verify_hostname: true,
            cipher_suites: vec![],
        };

        assert!(check_tls_security(&network_config));
        assert!(check_tls_security(&provider_config));
    }

    #[test]
    fn test_validation() {
        let secure_config = ProviderTlsConfig {
            enabled: true,
            version: ProviderTlsVersion::V1_3,
            cert_path: Some("/cert.pem".to_string()),
            key_path: Some("/key.pem".to_string()),
            ca_path: None,
            verify_certs: true,
            verify_hostname: true,
            cipher_suites: vec![],
        };

        assert!(secure_config.validate().is_ok());
    }
}

