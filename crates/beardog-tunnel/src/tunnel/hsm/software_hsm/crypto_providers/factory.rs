use std::sync::Arc;

use beardog_errors::BearDogError;

use super::genetic_crypto::GeneticCryptoProvider;
// OpenSslCryptoProvider removed - evolved to 100% Pure Rust
// RingCryptoProvider removed - evolved to RustCrypto (100% Pure Rust, no C deps!)
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
        CryptoBackend::GeneticCrypto => {
            let provider = GeneticCryptoProvider::new()?;
            Ok(Arc::new(provider))
        }
        CryptoBackend::RustCrypto => {
            let provider = RustCryptoProvider::new().await?;
            Ok(Arc::new(provider))
        }
        CryptoBackend::Ring => {
            // Ring evolved to RustCrypto (100% Pure Rust, no C dependencies!)
            tracing::warn!("Ring backend deprecated - evolved to RustCrypto (100% Pure Rust, ARM-ready!)");
            let provider = RustCryptoProvider::new().await?;
            Ok(Arc::new(provider))
        }
        CryptoBackend::OpenSsl => {
            // OpenSSL evolved to RustCrypto (100% Pure Rust sovereignty)
            tracing::warn!("OpenSSL backend evolved to RustCrypto (100% Pure Rust, ARM-ready!)");
            let provider = RustCryptoProvider::new().await?;
            Ok(Arc::new(provider))
        }
    }
}

/// Get Supported Crypto Backends operation.
///
/// **TRUE PRIMAL**: Only 100% Pure Rust backends supported! 🦀
///
/// - GeneticCrypto: RECOMMENDED (100% Pure Rust, hardware acceleration)
/// - RustCrypto: Standard (100% Pure Rust, ARM-ready, no C compiler needed)
pub fn get_supported_crypto_backends() -> Vec<CryptoBackend> {
    vec![
        CryptoBackend::GeneticCrypto, // RECOMMENDED: 100% Pure Rust + Hardware Acceleration
        CryptoBackend::RustCrypto,    // 100% Pure Rust, ARM Cross-Compile Ready!
        // Ring removed - evolved to RustCrypto (100% Pure Rust, no C deps!)
        // OpenSsl removed - evolved to RustCrypto (100% Pure Rust sovereignty)
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
///
/// **TRUE PRIMAL**: Only 100% Pure Rust backends! 🦀
pub fn get_crypto_provider_capabilities(backend: &CryptoBackend) -> CryptoProviderCapabilities {
    match backend {
        CryptoBackend::GeneticCrypto => CryptoProviderCapabilities {
            supports_aes: true,
            supports_chacha20: false, // Not yet implemented (use AES-256-GCM)
            supports_ecc: true,
            supports_rsa: false,                  // Not yet implemented
            supports_hardware_acceleration: true, // Uses AES-NI, AVX2
        },
        CryptoBackend::RustCrypto => CryptoProviderCapabilities {
            supports_aes: true,
            supports_chacha20: true,
            supports_ecc: true,
            supports_rsa: true,
            supports_hardware_acceleration: false,
        },
        // Ring evolved to RustCrypto (backward compatibility)
        CryptoBackend::Ring => CryptoProviderCapabilities {
            supports_aes: true,
            supports_chacha20: true,
            supports_ecc: true,
            supports_rsa: true,
            supports_hardware_acceleration: false, // RustCrypto capabilities
        },
        // OpenSSL evolved to RustCrypto (backward compatibility)
        CryptoBackend::OpenSsl => CryptoProviderCapabilities {
            supports_aes: true,
            supports_chacha20: true,
            supports_ecc: true,
            supports_rsa: true,
            supports_hardware_acceleration: false, // RustCrypto capabilities
        },
        // Vendor-agnostic: Custom and Hardware variants removed
    }
}

/// Check if Crypto Backend is Supported.
///
/// **TRUE PRIMAL**: Only 100% Pure Rust backends! 🦀
///
/// - GeneticCrypto: ✅ RECOMMENDED (100% Pure Rust)
/// - RustCrypto: ✅ Standard (100% Pure Rust)
/// - Ring: ✅ Supported (auto-fallback to RustCrypto for ARM compatibility)
/// - OpenSsl: ✅ Supported (auto-fallback to RustCrypto for Pure Rust sovereignty)
pub fn is_crypto_backend_supported(backend: &CryptoBackend) -> bool {
    // All backends supported - Ring and OpenSsl auto-fallback to RustCrypto! 🦀
    matches!(
        backend,
        CryptoBackend::GeneticCrypto | CryptoBackend::RustCrypto | CryptoBackend::Ring | CryptoBackend::OpenSsl
    )
}

/// Get Recommended Crypto Backend.
///
/// **UPDATED**: Now recommends GeneticCrypto (100% Pure Rust, zero FFI)
///
/// GeneticCrypto provides:
/// - 100% memory safe (borrow checker enforced)
/// - Zero FFI boundaries (full compiler optimization)
/// - Hardware acceleration (AES-NI, AVX2)
/// - Single language audit (no C code)
/// - Genetic crypto enhancements (future)
pub fn get_recommended_crypto_backend() -> CryptoBackend {
    CryptoBackend::GeneticCrypto // Changed from Ring!
}

/// Get Crypto Backend By Name.
///
/// **TRUE PRIMAL**: Evolves legacy requests to 100% Pure Rust! 🦀
pub fn get_crypto_backend_by_name(name: &str) -> Option<CryptoBackend> {
    match name.to_lowercase().as_str() {
        "genetic" | "geneticcrypto" | "genetic-crypto" => Some(CryptoBackend::GeneticCrypto),
        "rust" | "rustcrypto" | "rust-crypto" => Some(CryptoBackend::RustCrypto),
        // Ring evolved to RustCrypto (100% Pure Rust, ARM-ready!)
        "ring" => {
            tracing::warn!(
                "Ring backend deprecated - evolved to RustCrypto (100% Pure Rust, ARM-ready!)"
            );
            Some(CryptoBackend::Ring) // Auto-fallback to RustCrypto in create_crypto_provider
        }
        // OpenSSL evolved to RustCrypto (100% Pure Rust sovereignty)
        "openssl" | "ssl" => {
            tracing::warn!(
                "OpenSSL backend evolved to RustCrypto (100% Pure Rust, ARM-ready!)"
            );
            Some(CryptoBackend::OpenSsl) // Auto-fallback to RustCrypto in create_crypto_provider
        }
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
    async fn test_create_ring_crypto_provider_fallback() -> Result<(), BearDogError> {
        // Ring evolved to RustCrypto (100% Pure Rust, ARM-ready!)
        let provider = create_crypto_provider(&CryptoBackend::Ring).await?;
        assert!(provider.initialize().await.is_ok());
        // Should succeed - auto-fallback to RustCrypto
        Ok(())
    }

    #[tokio::test]
    async fn test_create_openssl_crypto_provider_fallback() -> Result<(), BearDogError> {
        // OpenSSL evolved to RustCrypto (100% Pure Rust sovereignty!)
        let provider = create_crypto_provider(&CryptoBackend::OpenSsl).await?;
        assert!(provider.initialize().await.is_ok());
        // Should succeed - auto-fallback to RustCrypto
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
        assert!(backends.contains(&CryptoBackend::RustCrypto));
        assert!(backends.contains(&CryptoBackend::GeneticCrypto));
        // Ring evolved to RustCrypto - only 100% Pure Rust backends listed! 🦀
        assert!(!backends.contains(&CryptoBackend::Ring), "Ring evolved to RustCrypto");
        assert!(!backends.contains(&CryptoBackend::OpenSsl), "OpenSsl evolved to RustCrypto");
        assert_eq!(
            backends.len(),
            2,
            "Should have exactly 2 pure Rust backends (GeneticCrypto, RustCrypto)"
        );
    }

    #[test]
    fn test_get_crypto_provider_capabilities() {
        let rust_caps = get_crypto_provider_capabilities(&CryptoBackend::RustCrypto);
        assert!(rust_caps.supports_aes);
        assert!(rust_caps.supports_ecc);
        assert!(!rust_caps.supports_hardware_acceleration);

        // Ring evolved to RustCrypto - capabilities reflect RustCrypto now
        let ring_caps = get_crypto_provider_capabilities(&CryptoBackend::Ring);
        assert!(ring_caps.supports_aes);
        assert!(ring_caps.supports_ecc);
        assert!(!ring_caps.supports_hardware_acceleration, "Ring uses RustCrypto now (100% Pure Rust)");

        // OpenSSL evolved to RustCrypto - capabilities reflect RustCrypto now
        let openssl_caps = get_crypto_provider_capabilities(&CryptoBackend::OpenSsl);
        assert!(openssl_caps.supports_aes);
        assert!(openssl_caps.supports_ecc);
        assert!(!openssl_caps.supports_hardware_acceleration, "OpenSSL uses RustCrypto now (100% Pure Rust)");
    }

    #[test]
    fn test_is_crypto_backend_supported() {
        assert!(is_crypto_backend_supported(&CryptoBackend::RustCrypto));
        assert!(is_crypto_backend_supported(&CryptoBackend::GeneticCrypto));
        // Ring and OpenSSL supported with auto-fallback to RustCrypto (backward compatibility) 🦀
        assert!(
            is_crypto_backend_supported(&CryptoBackend::Ring),
            "Ring supported with fallback to RustCrypto (100% Pure Rust!)"
        );
        assert!(
            is_crypto_backend_supported(&CryptoBackend::OpenSsl),
            "OpenSSL supported with fallback to RustCrypto (100% Pure Rust!)"
        );
    }

    #[test]
    fn test_get_recommended_crypto_backend() {
        let recommended = get_recommended_crypto_backend();
        // Should recommend GeneticCrypto (sovereignty-first, genetic lineage integrated)
        assert_eq!(
            recommended,
            CryptoBackend::GeneticCrypto,
            "GeneticCrypto is recommended for sovereignty and genetic lineage integration"
        );

        // Verify it's NOT OpenSSL (we're 100% pure Rust now!)
        assert_ne!(
            recommended,
            CryptoBackend::OpenSsl,
            "Should never recommend OpenSSL - we've evolved beyond it!"
        );
    }

    #[test]
    fn test_get_crypto_backend_by_name() {
        // 100% Pure Rust backends
        assert_eq!(
            get_crypto_backend_by_name("rust"),
            Some(CryptoBackend::RustCrypto)
        );
        assert_eq!(
            get_crypto_backend_by_name("genetic"),
            Some(CryptoBackend::GeneticCrypto)
        );

        // Ring evolved to RustCrypto (auto-fallback in create_crypto_provider)
        assert_eq!(
            get_crypto_backend_by_name("ring"),
            Some(CryptoBackend::Ring), // Returns Ring enum, but fallbacks to RustCrypto implementation
            "Ring returns Ring enum but uses RustCrypto (100% Pure Rust!)"
        );

        // OpenSSL evolved to RustCrypto (auto-fallback in create_crypto_provider)
        assert_eq!(
            get_crypto_backend_by_name("openssl"),
            Some(CryptoBackend::OpenSsl), // Returns OpenSsl enum, but fallbacks to RustCrypto implementation
            "OpenSSL returns OpenSsl enum but uses RustCrypto (100% Pure Rust!)"
        );

        assert_eq!(get_crypto_backend_by_name("unknown"), None);
    }
}
