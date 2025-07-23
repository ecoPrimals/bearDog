//! # Crypto Provider Factory
//!
//! This module provides factory functions for creating crypto providers and querying their capabilities.
//! It acts as a unified interface for working with different crypto backends.

use super::ring_crypto::RingCryptoProvider;
use crate::tunnel::hsm::software_hsm::types::{
    CryptoProvider, OpenSslCryptoProvider, RustCryptoProvider,
};
use crate::tunnel::hsm::types::config::CryptoBackend;
use crate::tunnel::hsm::types::tier::KeyStorageType;
// use async_trait::async_trait;
use beardog_errors::{BearDogError, BearDogResult};
// use std::sync::Arc;

/// Create a crypto provider based on the specified backend
pub async fn create_crypto_provider(
    backend: &CryptoBackend,
) -> BearDogResult<Box<dyn CryptoProvider>> {
    match backend {
        CryptoBackend::RustCrypto => {
            let provider = RustCryptoProvider::new().await?;
            Ok(Box::new(provider))
        }
        CryptoBackend::Ring => {
            let provider = RingCryptoProvider::new()?;
            Ok(Box::new(provider))
        }
        CryptoBackend::OpenSsl => {
            let provider = OpenSslCryptoProvider::new().await?;
            Ok(Box::new(provider))
        }
        CryptoBackend::Hardware => Err(BearDogError::UnsupportedOperation {
            operation: "Hardware crypto backend not supported in software HSM".to_string(),
        }),
        CryptoBackend::Custom(name) => Err(BearDogError::UnsupportedOperation {
            operation: format!("Unsupported crypto backend: {name}"),
        }),
    }
}

/// Get supported crypto backends
pub fn get_supported_crypto_backends() -> Vec<CryptoBackend> {
    vec![
        CryptoBackend::RustCrypto,
        CryptoBackend::Ring,
        CryptoBackend::OpenSsl,
    ]
}

/// Get supported storage backends
pub fn get_supported_storage_backends() -> Vec<KeyStorageType> {
    vec![
        KeyStorageType::EncryptedFile,
        KeyStorageType::InMemory,
        KeyStorageType::Database,
    ]
}

/// Get crypto provider capabilities
pub fn get_crypto_provider_capabilities(backend: &CryptoBackend) -> CryptoProviderCapabilities {
    match backend {
        CryptoBackend::RustCrypto => CryptoProviderCapabilities {
            supports_aes: true,
            supports_chacha20: true,
            supports_ecc: true,
            supports_rsa: true,
            supports_hardware_acceleration: false,
        },
        CryptoBackend::Ring => CryptoProviderCapabilities {
            supports_aes: true,
            supports_chacha20: true,
            supports_ecc: true,
            supports_rsa: true,
            supports_hardware_acceleration: true,
        },
        CryptoBackend::OpenSsl => CryptoProviderCapabilities {
            supports_aes: true,
            supports_chacha20: true,
            supports_ecc: true,
            supports_rsa: true,
            supports_hardware_acceleration: true,
        },
        CryptoBackend::Custom(_) => CryptoProviderCapabilities {
            supports_aes: false,
            supports_chacha20: false,
            supports_ecc: false,
            supports_rsa: false,
            supports_hardware_acceleration: false,
        },
        CryptoBackend::Hardware => CryptoProviderCapabilities {
            supports_aes: true,
            supports_chacha20: true,
            supports_ecc: true,
            supports_rsa: true,
            supports_hardware_acceleration: true,
        },
    }
}

/// Crypto provider capabilities
#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
pub struct CryptoProviderCapabilities {
    /// Whether the provider supports AES encryption
    pub supports_aes: bool,
    /// Whether the provider supports ChaCha20 encryption
    pub supports_chacha20: bool,
    /// Whether the provider supports Elliptic Curve Cryptography
    pub supports_ecc: bool,
    /// Whether the provider supports RSA encryption
    pub supports_rsa: bool,
    /// Whether the provider supports hardware acceleration
    pub supports_hardware_acceleration: bool,
}

/// Check if a crypto backend is supported
pub fn is_crypto_backend_supported(backend: &CryptoBackend) -> bool {
    matches!(
        backend,
        CryptoBackend::RustCrypto | CryptoBackend::Ring | CryptoBackend::OpenSsl
    )
}

/// Get the recommended crypto backend based on performance and security
pub fn get_recommended_crypto_backend() -> CryptoBackend {
    // Ring is generally considered the fastest and most secure
    CryptoBackend::Ring
}

/// Get crypto backend by name
pub fn get_crypto_backend_by_name(name: &str) -> Option<CryptoBackend> {
    match name.to_lowercase().as_str() {
        "rust" | "rustcrypto" | "rust-crypto" => Some(CryptoBackend::RustCrypto),
        "ring" => Some(CryptoBackend::Ring),
        "openssl" | "ssl" => Some(CryptoBackend::OpenSsl),
        _ => None,
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use tokio;

    #[tokio::test]
    async fn test_create_rust_crypto_provider() {
        let provider = create_crypto_provider(&CryptoBackend::RustCrypto)
            .await
            .unwrap();
        assert!(provider.initialize().await.is_ok());
    }

    #[tokio::test]
    async fn test_create_ring_crypto_provider() {
        let provider = create_crypto_provider(&CryptoBackend::Ring).await.unwrap();
        assert!(provider.initialize().await.is_ok());
    }

    #[tokio::test]
    async fn test_create_openssl_crypto_provider() {
        let provider = create_crypto_provider(&CryptoBackend::OpenSsl)
            .await
            .unwrap();
        assert!(provider.initialize().await.is_ok());
    }

    #[tokio::test]
    async fn test_create_custom_crypto_provider() {
        let result = create_crypto_provider(&CryptoBackend::Custom("unknown".to_string())).await;
        assert!(result.is_err());
    }

    #[test]
    fn test_get_supported_crypto_backends() {
        let backends = get_supported_crypto_backends();
        assert_eq!(backends.len(), 3);
        assert!(backends.contains(&CryptoBackend::RustCrypto));
        assert!(backends.contains(&CryptoBackend::Ring));
        assert!(backends.contains(&CryptoBackend::OpenSsl));
    }

    #[test]
    fn test_get_crypto_provider_capabilities() {
        let rust_caps = get_crypto_provider_capabilities(&CryptoBackend::RustCrypto);
        assert!(rust_caps.supports_aes);
        assert!(rust_caps.supports_ecc);
        assert!(!rust_caps.supports_hardware_acceleration);

        let ring_caps = get_crypto_provider_capabilities(&CryptoBackend::Ring);
        assert!(ring_caps.supports_aes);
        assert!(ring_caps.supports_ecc);
        assert!(ring_caps.supports_hardware_acceleration);

        let openssl_caps = get_crypto_provider_capabilities(&CryptoBackend::OpenSsl);
        assert!(openssl_caps.supports_aes);
        assert!(openssl_caps.supports_ecc);
        assert!(openssl_caps.supports_hardware_acceleration);
    }

    #[test]
    fn test_is_crypto_backend_supported() {
        assert!(is_crypto_backend_supported(&CryptoBackend::RustCrypto));
        assert!(is_crypto_backend_supported(&CryptoBackend::Ring));
        assert!(is_crypto_backend_supported(&CryptoBackend::OpenSsl));
        assert!(!is_crypto_backend_supported(&CryptoBackend::Custom(
            "unknown".to_string()
        )));
    }

    #[test]
    fn test_get_recommended_crypto_backend() {
        let recommended = get_recommended_crypto_backend();
        assert_eq!(recommended, CryptoBackend::Ring);
    }

    #[test]
    fn test_get_crypto_backend_by_name() {
        assert_eq!(
            get_crypto_backend_by_name("rust"),
            Some(CryptoBackend::RustCrypto)
        );
        assert_eq!(
            get_crypto_backend_by_name("ring"),
            Some(CryptoBackend::Ring)
        );
        assert_eq!(
            get_crypto_backend_by_name("openssl"),
            Some(CryptoBackend::OpenSsl)
        );
        assert_eq!(get_crypto_backend_by_name("unknown"), None);
    }
}
