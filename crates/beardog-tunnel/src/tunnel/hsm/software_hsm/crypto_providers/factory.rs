use std::sync::Arc;

use beardog_errors::BearDogError;

use super::openssl_crypto::OpenSslCryptoProvider;
use super::ring_crypto::RingCryptoProvider;
use super::rust_crypto::RustCryptoProvider;
use crate::tunnel::hsm::types::config::CryptoBackend;
use crate::tunnel::hsm::types::tier::KeyStorageType;
use crate::tunnel::hsm::types::KeyType;
use beardog_types::hsm::CryptoProvider; // Import KeyType for generic trait

/// Create Crypto Provider operation.
///
/// # Errors
/// Returns an error if the crypto provider cannot be created.
pub async fn create_crypto_provider(
    backend: &CryptoBackend,
) -> Result<Arc<dyn CryptoProvider<KeyType>>, BearDogError> {
    match backend {
        CryptoBackend::RustCrypto => {
            let provider = RustCryptoProvider::new().await?;
            Ok(Arc::new(provider))
        }
        CryptoBackend::Ring => {
            let provider = RingCryptoProvider::new()?;
            Ok(Arc::new(provider))
        }
        CryptoBackend::OpenSsl => {
            let provider = OpenSslCryptoProvider::new().await?;
            Ok(Arc::new(provider))
        }
    }
}

/// Get Supported Crypto Backends operation.
pub fn get_supported_crypto_backends() -> Vec<CryptoBackend> {
    vec![
        CryptoBackend::RustCrypto,
        CryptoBackend::Ring,
        CryptoBackend::OpenSsl,
    ]
}

/// Get Supported Storage Backends operation.
pub fn get_supported_storage_backends() -> Vec<KeyStorageType> {
    vec![
        KeyStorageType::Memory,
        KeyStorageType::Database,
        KeyStorageType::File,
    ]
}

/// Crypto provider capabilities
#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
pub struct CryptoProviderCapabilities {
    pub supports_aes: bool,
    pub supports_chacha20: bool,
    pub supports_ecc: bool,
    pub supports_rsa: bool,
    pub supports_hardware_acceleration: bool,
}

/// Get Crypto Provider Capabilities operation.
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
        // Vendor-agnostic: Custom and Hardware variants removed
    }
}

/// Check if Crypto Backend is Supported.
pub fn is_crypto_backend_supported(backend: &CryptoBackend) -> bool {
    matches!(
        backend,
        CryptoBackend::RustCrypto | CryptoBackend::Ring | CryptoBackend::OpenSsl
    )
}

/// Get Recommended Crypto Backend.
pub fn get_recommended_crypto_backend() -> CryptoBackend {
    CryptoBackend::Ring
}

/// Get Crypto Backend By Name.
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

    #[tokio::test]
    async fn test_create_rust_crypto_provider() -> Result<(), BearDogError> {
        let provider = create_crypto_provider(&CryptoBackend::RustCrypto).await?;
        assert!(provider.initialize().await.is_ok());
        Ok(())
    }

    #[tokio::test]
    async fn test_create_ring_crypto_provider() -> Result<(), BearDogError> {
        let provider = create_crypto_provider(&CryptoBackend::Ring).await?;
        assert!(provider.initialize().await.is_ok());
        Ok(())
    }

    #[tokio::test]
    async fn test_create_openssl_crypto_provider() -> Result<(), BearDogError> {
        let provider = create_crypto_provider(&CryptoBackend::OpenSsl).await?;
        assert!(provider.initialize().await.is_ok());
        Ok(())
    }

    #[tokio::test]
    async fn test_create_rustcrypto_provider() -> Result<(), BearDogError> {
        // Vendor-agnostic: Test creating a supported provider
        let result = create_crypto_provider(&CryptoBackend::RustCrypto).await;
        assert!(result.is_ok()); // RustCrypto is supported
        Ok(())
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
        // Vendor-agnostic: Removed Custom backend test
        // assert!(!is_crypto_backend_supported(&CryptoBackend::Custom("unknown".to_string())));
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
