// SPDX-License-Identifier: AGPL-3.0-or-later

//! Universal Crypto Provider Trait
//!
//! Defines the interface all crypto providers must implement, similar to `UniversalHsmProvider`.

use super::algorithms::{
    AsymmetricAlgorithm, CryptoAlgorithm, DecryptionOptions, EncryptedData, EncryptionOptions,
    HashAlgorithm, KdfAlgorithm, Signature, SignatureAlgorithm, SigningOptions, SymmetricAlgorithm,
    VerificationOptions,
};
use super::capabilities::CryptoCapabilities;
use beardog_errors::BearDogError;
use std::future::Future;

/// Universal crypto provider trait
///
/// This trait eliminates crypto library lock-in by providing a vendor-agnostic
/// interface, similar to our Universal HSM architecture.
#[allow(async_fn_in_trait)]
pub trait UniversalCryptoProvider: Send + Sync + std::fmt::Debug {
    // ============================================================================
    // Provider Information
    // ============================================================================

    /// Get the provider name (e.g., "`RustCrypto`", "Ring", "OpenSSL")
    fn provider_name(&self) -> &str;

    /// Get the provider version
    fn provider_version(&self) -> &str;

    // ============================================================================
    // Capability Discovery (mirrors HSM pattern)
    // ============================================================================

    /// Discover what this provider can do
    fn discover_capabilities(
        &self,
    ) -> impl Future<Output = Result<CryptoCapabilities, BearDogError>> + Send;

    /// Check if this provider supports a specific algorithm
    fn supports_algorithm(&self, algorithm: &CryptoAlgorithm) -> impl Future<Output = bool> + Send;

    // ============================================================================
    // Symmetric Encryption
    // ============================================================================

    /// Encrypt data using symmetric algorithm
    fn encrypt_symmetric(
        &self,
        algorithm: SymmetricAlgorithm,
        key: &[u8],
        plaintext: &[u8],
        options: &EncryptionOptions,
    ) -> impl Future<Output = Result<EncryptedData, BearDogError>> + Send;

    /// Decrypt data using symmetric algorithm
    fn decrypt_symmetric(
        &self,
        algorithm: SymmetricAlgorithm,
        key: &[u8],
        ciphertext: &EncryptedData,
        options: &DecryptionOptions,
    ) -> impl Future<Output = Result<Vec<u8>, BearDogError>> + Send;

    // ============================================================================
    // Asymmetric Encryption
    // ============================================================================

    /// Encrypt data using asymmetric algorithm
    async fn encrypt_asymmetric(
        &self,
        algorithm: AsymmetricAlgorithm,
        public_key: &[u8],
        plaintext: &[u8],
        options: &EncryptionOptions,
    ) -> Result<EncryptedData, BearDogError>;

    /// Decrypt data using asymmetric algorithm
    async fn decrypt_asymmetric(
        &self,
        algorithm: AsymmetricAlgorithm,
        private_key: &[u8],
        ciphertext: &EncryptedData,
        options: &DecryptionOptions,
    ) -> Result<Vec<u8>, BearDogError>;

    // ============================================================================
    // Digital Signatures
    // ============================================================================

    /// Sign data
    fn sign(
        &self,
        algorithm: SignatureAlgorithm,
        private_key: &[u8],
        message: &[u8],
        options: &SigningOptions,
    ) -> impl Future<Output = Result<Signature, BearDogError>> + Send;

    /// Verify signature
    fn verify(
        &self,
        algorithm: SignatureAlgorithm,
        public_key: &[u8],
        message: &[u8],
        signature: &Signature,
        options: &VerificationOptions,
    ) -> impl Future<Output = Result<bool, BearDogError>> + Send;

    // ============================================================================
    // Hashing
    // ============================================================================

    /// Hash data
    fn hash(
        &self,
        algorithm: HashAlgorithm,
        data: &[u8],
    ) -> impl Future<Output = Result<Vec<u8>, BearDogError>> + Send;

    // ============================================================================
    // Key Derivation
    // ============================================================================

    /// Derive key
    fn derive_key(
        &self,
        algorithm: KdfAlgorithm,
        input_key: &[u8],
        salt: &[u8],
        info: &[u8],
        output_length: usize,
    ) -> impl Future<Output = Result<Vec<u8>, BearDogError>> + Send;
}

/// Helper trait for generating nonces/IVs
pub trait NonceGenerator {
    /// Generate a cryptographically random nonce of the specified size
    fn generate_nonce(&self, size: usize) -> Vec<u8>;
}

impl<T: UniversalCryptoProvider> NonceGenerator for T {
    fn generate_nonce(&self, size: usize) -> Vec<u8> {
        use rand::RngCore;
        let mut nonce = vec![0u8; size];
        rand::rng().fill_bytes(&mut nonce);
        nonce
    }
}
