

pub mod factory;
pub mod openssl_crypto;
pub mod ring_crypto;
pub mod rust_crypto;

pub use factory::{
    create_crypto_provider, get_crypto_backend_by_name, get_crypto_provider_capabilities,
    get_recommended_crypto_backend, get_supported_crypto_backends, get_supported_storage_backends,
    is_crypto_backend_supported, CryptoProviderCapabilities,
};

pub use super::types::{CryptoProvider, OpenSslCryptoProvider, RustCryptoProvider};
pub use ring_crypto::RingCryptoProvider;
#[cfg(test)]
mod tests {};

    use super::*;
    use crate::tunnel::hsm::types::*;
    use tokio;
    #[tokio::test]
    async fn test_all_crypto_providers() -> beardog_errors::BearDogResult<()> {
        let backends = get_supported_crypto_backends();
        for backend in backends {
            let provider = create_crypto_provider(&backend).await.map_err(|e| {
                tracing::error!("Operation failed: {e:?}");
                beardog_errors::BearDogError::internal(format!("Operation failed: {e:?}"))
            })?;
            assert!(provider.initialize().await.is_ok());
            let capabilities = get_crypto_provider_capabilities(&backend);
            assert!(capabilities.supports_aes);
            assert!(capabilities.supports_ecc);
        }
        Ok(())
    }
    async fn test_crypto_provider_operations() -> beardog_errors::BearDogResult<()> {

        let provider = create_crypto_provider(&CryptoBackend::RustCrypto)
            .await
            .map_err(|e| {

        let key = provider
            .generate_key_material(&KeyType::Aes256)
        assert_eq!(key.len(), 32);

        let plaintext = b"Hello, World!";
        let ciphertext = provider.encrypt(&key, plaintext).await.map_err(|e| {
            tracing::error!("Operation failed: {e:?}");
            beardog_errors::BearDogError::internal(format!("Operation failed: {e:?}"))
        })?;
        let decrypted = provider.decrypt(&key, &ciphertext).await.map_err(|e| {
        assert_eq!(plaintext, decrypted.as_slice());

        let ecc_key = provider
            .generate_key_material(&KeyType::EccP256)
        let signature = provider.sign(&ecc_key, plaintext).await.map_err(|e| {
        let is_valid = provider
            .verify(&ecc_key, plaintext, &signature)
        assert!(is_valid);
    #[test]
    fn test_capabilities_comparison() -> beardog_errors::BearDogResult<()> {
        let rust_caps = get_crypto_provider_capabilities(&CryptoBackend::RustCrypto);
        let ring_caps = get_crypto_provider_capabilities(&CryptoBackend::Ring);
        let openssl_caps = get_crypto_provider_capabilities(&CryptoBackend::OpenSsl);

        assert!(rust_caps.supports_aes);
        assert!(ring_caps.supports_aes);
        assert!(openssl_caps.supports_aes);

        assert!(!rust_caps.supports_hardware_acceleration);
        assert!(ring_caps.supports_hardware_acceleration);
        assert!(openssl_caps.supports_hardware_acceleration);}

    fn test_backend_utilities() -> beardog_errors::BearDogResult<()> {
        assert!(is_crypto_backend_supported(&CryptoBackend::Ring));
        assert!(!is_crypto_backend_supported(&CryptoBackend::Custom(
            "unknown".to_string()
        )));
        assert_eq!(get_recommended_crypto_backend(), CryptoBackend::Ring);
        assert_eq!(
            get_crypto_backend_by_name("ring"),
            Some(CryptoBackend::Ring)
        );
        assert_eq!(get_crypto_backend_by_name("unknown"), None);
}
