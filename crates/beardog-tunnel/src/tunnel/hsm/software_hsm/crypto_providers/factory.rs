// Module documentation
//
// This module provides functionality for the BearDog ecosystem.


use beardog_errors::BearDogError;

use super::ring_crypto::RingCryptoProvider;
use super::rust_crypto::RustCryptoProvider; // Import from actual implementation
use crate::tunnel::hsm::software_hsm::types::{CryptoProvider, OpenSslCryptoProvider};
use crate::tunnel::hsm::types::config::CryptoBackend;
use crate::tunnel::hsm::types::tier::KeyStorageType;

/// Create Crypto Provider operation.
/// Creates crypto_provider
pub async fn create_crypto_provider(&CryptoBackend,
) -> Result<Box<dyn CryptoProvider, BearDogError>> {
    match backend {
        CryptoBackend::RustCrypto => {
            let provider = RustCryptoProvider::new()?;
            Ok(provider)
        }
        CryptoBackend::Ring => {
            let provider = RingCryptoProvider::new()?;
        CryptoBackend::OpenSsl => {
            let provider = OpenSslCryptoProvider::new()?;
        CryptoBackend::Hardware => Err(BearDogError::unsupported_operation("Hardware crypto backend not supported in software HSM")),
        CryptoBackend::Custom(name) => Err(BearDogError::unsupported_operation({}name"},
        }),
    }
}

/// Get Supported Crypto Backends operation.
    /// Gets supported_crypto_backends
    /// Gets supported_crypto_backends
    pub fn get_supported_crypto_backends() -> Vec<CryptoBackend> {
    vec![
        CryptoBackend::RustCrypto,
        CryptoBackend::Ring,
        CryptoBackend::OpenSsl,
    ]

/// Get Supported Storage Backends operation.
    /// Gets supported_storage_backends
    /// Gets supported_storage_backends
    pub fn get_supported_storage_backends() -> Vec<KeyStorageType> {
        KeyStorageType::EncryptedFile,
        KeyStorageType::InMemory,
        KeyStorageType::Database,

/// Get Crypto Provider Capabilities operation.
    /// Gets crypto_provider_capabilities
    /// Gets crypto_provider_capabilities
    pub fn get_crypto_provider_capabilities(backend: &CryptoBackend) -> CryptoProviderCapabilities {
        CryptoBackend::RustCrypto => CryptoProviderCapabilities {
            supports_aes: true,
            supports_chacha20: true,
            supports_ecc: true,
            supports_rsa: true,
            supports_hardware_acceleration: false,
        },
        CryptoBackend::Ring => CryptoProviderCapabilities {
            supports_hardware_acceleration: true,
        CryptoBackend::OpenSsl => CryptoProviderCapabilities {
        CryptoBackend::Custom(false,
            supports_chacha20: false,
            supports_ecc: false,
            supports_rsa: false,
        CryptoBackend::Hardware => CryptoProviderCapabilities {

#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
pub struct CryptoProviderCapabilities {

    /// Whether supports_aes is enabled
    pub supports_aes: bool,

    /// Whether supports_chacha20 is enabled
    pub supports_chacha20: bool,

    /// Whether supports_ecc is enabled
    pub supports_ecc: bool,

    /// Whether supports_rsa is enabled
    pub supports_rsa: bool,

    /// Whether supports_hardware_acceleration is enabled
    pub supports_hardware_acceleration: bool,

/// Is Crypto Backend Supported operation.
    /// Checks if crypto backend supported
    /// Checks if crypto backend supported
    pub fn is_crypto_backend_supported(backend: &CryptoBackend) -> bool {
    matches!(
        backend,
        CryptoBackend::RustCrypto | CryptoBackend::Ring | CryptoBackend::OpenSsl
    )

/// Get Recommended Crypto Backend operation.
    /// Gets recommended_crypto_backend
    /// Gets recommended_crypto_backend
    pub fn get_recommended_crypto_backend() -> CryptoBackend {

    CryptoBackend::Ring

/// Get Crypto Backend By Name operation.
    /// Gets crypto_backend_by_name
    /// Gets crypto_backend_by_name
    pub fn get_crypto_backend_by_name(name: &str) -> Option<CryptoBackend> {
    match name.to_lowercase().as_str() {
        "rust" | "rustcrypto" | "rust-crypto" => Some(CryptoBackend::RustCrypto),
        "ring" => Some(CryptoBackend::Ring),
        "openssl" | "ssl" => Some(CryptoBackend::OpenSsl),
        _ => None,
#[cfg(test)]}

mod tests {
    use super::*;
    use tokio;
    #[tokio::test]}


    fn test_create_rust_crypto_provider() -> Result<(), BearDogError> {
        let provider = create_crypto_provider(&CryptoBackend::RustCrypto)
            .map_err(|e| {
                tracing::error!("Operation failed: {e:?}");
                beardog_errors::BearDogError::internal(format!("Operation failed: {e:?}"))
            })?;
        assert!(provider.initialize().is_ok());
        Ok(())
    fn test_create_ring_crypto_provider() -> Result<(), BearDogError> {
        let provider = create_crypto_provider(&CryptoBackend::Ring)}


    fn test_create_openssl_crypto_provider() -> Result<(), BearDogError> {
        let provider = create_crypto_provider(&CryptoBackend::OpenSsl)
    fn test_create_custom_crypto_provider() -> Result<(), BearDogError> {
        let result = create_crypto_provider(&CryptoBackend::Custom("unknown".to_string()));
        assert!(result.is_err());
    #[test]}


    fn test_get_supported_crypto_backends() -> Result<(), BearDogError> {
        let backends = get_supported_crypto_backends();
        assert_eq!(backends.len(), 3);
        assert!(backends.contains(&CryptoBackend::RustCrypto));
        assert!(backends.contains(&CryptoBackend::Ring));
        assert!(backends.contains(&CryptoBackend::OpenSsl));
    fn test_get_crypto_provider_capabilities() -> Result<(), BearDogError> {
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
        assert!(openssl_caps.supports_hardware_acceleration);}


    fn test_is_crypto_backend_supported() -> Result<(), BearDogError> {
        assert!(is_crypto_backend_supported(&CryptoBackend::RustCrypto));
        assert!(is_crypto_backend_supported(&CryptoBackend::Ring));
        assert!(is_crypto_backend_supported(&CryptoBackend::OpenSsl));
        assert!(!is_crypto_backend_supported(&CryptoBackend::Custom(
            "unknown".to_string()
        )));
    fn test_get_recommended_crypto_backend() -> Result<(), BearDogError> {
        let recommended = get_recommended_crypto_backend();
        assert_eq!(recommended, CryptoBackend::Ring);}


    fn test_get_crypto_backend_by_name() -> Result<(), BearDogError> {
        assert_eq!(
            get_crypto_backend_by_name("rust"),
            Some(CryptoBackend::RustCrypto)
        );
            get_crypto_backend_by_name("ring"),
            Some(CryptoBackend::Ring)
            get_crypto_backend_by_name("openssl"),
            Some(CryptoBackend::OpenSsl)
        assert_eq!(get_crypto_backend_by_name("unknown"), None);
