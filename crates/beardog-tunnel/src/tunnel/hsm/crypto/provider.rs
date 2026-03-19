// SPDX-License-Identifier: AGPL-3.0-only

//! Universal Crypto Provider Trait
//!
//! Defines the interface all crypto providers must implement, similar to UniversalHsmProvider.

use super::algorithms::*;
use super::capabilities::CryptoCapabilities;
use async_trait::async_trait;
use beardog_errors::BearDogError;

/// Universal crypto provider trait
///
/// This trait eliminates crypto library lock-in by providing a vendor-agnostic
/// interface, similar to our Universal HSM architecture.
#[async_trait]
pub trait UniversalCryptoProvider: Send + Sync + std::fmt::Debug {
    // ============================================================================
    // Provider Information
    // ============================================================================

    /// Get the provider name (e.g., "RustCrypto", "Ring", "OpenSSL")
    fn provider_name(&self) -> &str;

    /// Get the provider version
    fn provider_version(&self) -> &str;

    // ============================================================================
    // Capability Discovery (mirrors HSM pattern)
    // ============================================================================

    /// Discover what this provider can do
    async fn discover_capabilities(&self) -> Result<CryptoCapabilities, BearDogError>;

    /// Check if this provider supports a specific algorithm
    async fn supports_algorithm(&self, algorithm: &CryptoAlgorithm) -> bool;

    // ============================================================================
    // Symmetric Encryption
    // ============================================================================

    /// Encrypt data using symmetric algorithm
    async fn encrypt_symmetric(
        &self,
        algorithm: SymmetricAlgorithm,
        key: &[u8],
        plaintext: &[u8],
        options: &EncryptionOptions,
    ) -> Result<EncryptedData, BearDogError>;

    /// Decrypt data using symmetric algorithm
    async fn decrypt_symmetric(
        &self,
        algorithm: SymmetricAlgorithm,
        key: &[u8],
        ciphertext: &EncryptedData,
        options: &DecryptionOptions,
    ) -> Result<Vec<u8>, BearDogError>;

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
    async fn sign(
        &self,
        algorithm: SignatureAlgorithm,
        private_key: &[u8],
        message: &[u8],
        options: &SigningOptions,
    ) -> Result<Signature, BearDogError>;

    /// Verify signature
    async fn verify(
        &self,
        algorithm: SignatureAlgorithm,
        public_key: &[u8],
        message: &[u8],
        signature: &Signature,
        options: &VerificationOptions,
    ) -> Result<bool, BearDogError>;

    // ============================================================================
    // Hashing
    // ============================================================================

    /// Hash data
    async fn hash(&self, algorithm: HashAlgorithm, data: &[u8]) -> Result<Vec<u8>, BearDogError>;

    // ============================================================================
    // Key Derivation
    // ============================================================================

    /// Derive key
    async fn derive_key(
        &self,
        algorithm: KdfAlgorithm,
        input_key: &[u8],
        salt: &[u8],
        info: &[u8],
        output_length: usize,
    ) -> Result<Vec<u8>, BearDogError>;
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
        rand::thread_rng().fill_bytes(&mut nonce);
        nonce
    }
}
