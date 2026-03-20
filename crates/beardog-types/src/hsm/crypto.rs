// SPDX-License-Identifier: AGPL-3.0-only

//! HSM Cryptographic Provider Traits
//!
//! Defines the core cryptographic provider interface for HSM operations.

use crate::BearDogError;

/// Cryptographic Provider Trait
///
/// Defines the interface for cryptographic operations that can be backed
/// by different implementations (RustCrypto, OpenSSL, etc.).
///
/// # Implementations
///
/// - **RustCryptoProvider**: Pure Rust cryptography using RustCrypto libraries (✅ RECOMMENDED)
/// - **OpenSslCryptoProvider**: OpenSSL library integration (for compatibility)
///
/// # Thread Safety
///
/// All implementations must be `Send + Sync` for use in async contexts.
///
/// # Generic Parameters
///
/// - `KeyType`: The key type enumeration used by the implementation
#[async_trait::async_trait]
pub trait CryptoProvider<KeyType = ()>: Send + Sync
where
    KeyType: Send + Sync + std::fmt::Debug,
{
    /// Initialize the cryptographic provider
    ///
    /// Performs any necessary setup, such as loading libraries,
    /// initializing random number generators, or validating capabilities.
    ///
    /// # Errors
    ///
    /// Returns an error if initialization fails (e.g., missing libraries,
    /// insufficient entropy, or unsupported platform).
    async fn initialize(&self) -> Result<(), BearDogError> {
        Ok(())
    }

    /// Generate key material for the specified key type
    ///
    /// # Arguments
    ///
    /// * `key_type` - The type of key to generate (AES, ECC, RSA, etc.)
    ///
    /// # Returns
    ///
    /// Raw key material as bytes. The length depends on the key type:
    /// - AES-256: 32 bytes
    /// - ChaCha20: 32 bytes
    /// - Ed25519: 32 bytes
    /// - ECC P-256: 32 bytes
    /// - ECC P-384: 48 bytes
    /// - RSA: key_size / 8 bytes
    ///
    /// # Errors
    ///
    /// Returns an error if key generation fails due to:
    /// - Unsupported key type
    /// - Insufficient entropy
    /// - Library errors
    async fn generate_key_material(&self, key_type: &KeyType) -> Result<Vec<u8>, BearDogError> {
        let _ = key_type;
        Ok(vec![0; 32])
    }

    /// Encrypt data using the provided key material
    ///
    /// # Arguments
    ///
    /// * `key_material` - Raw key bytes
    /// * `plaintext` - Data to encrypt
    ///
    /// # Returns
    ///
    /// Encrypted ciphertext. The format is implementation-specific but typically
    /// includes a nonce/IV and authentication tag.
    ///
    /// # Errors
    ///
    /// Returns an error if encryption fails due to:
    /// - Invalid key material
    /// - Encryption operation failure
    /// - Memory allocation failure
    async fn encrypt(&self, key_material: &[u8], plaintext: &[u8])
    -> Result<Vec<u8>, BearDogError>;

    /// Decrypt data using the provided key material
    ///
    /// # Arguments
    ///
    /// * `key_material` - Raw key bytes (must match encryption key)
    /// * `ciphertext` - Encrypted data to decrypt
    ///
    /// # Returns
    ///
    /// Decrypted plaintext
    ///
    /// # Errors
    ///
    /// Returns an error if decryption fails due to:
    /// - Invalid key material
    /// - Corrupted ciphertext
    /// - Authentication failure
    /// - Decryption operation failure
    async fn decrypt(
        &self,
        key_material: &[u8],
        ciphertext: &[u8],
    ) -> Result<Vec<u8>, BearDogError>;

    /// Sign data using the provided key material
    ///
    /// # Arguments
    ///
    /// * `key_material` - Raw signing key bytes
    /// * `data` - Data to sign
    ///
    /// # Returns
    ///
    /// Signature bytes. The format depends on the key type:
    /// - Ed25519: 64 bytes
    /// - ECDSA P-256: 64 bytes
    /// - RSA: key_size / 8 bytes
    ///
    /// # Errors
    ///
    /// Returns an error if signing fails due to:
    /// - Invalid key material
    /// - Unsupported key type for signing
    /// - Signing operation failure
    async fn sign(&self, key_material: &[u8], data: &[u8]) -> Result<Vec<u8>, BearDogError>;

    /// Verify a signature using the provided key material
    ///
    /// # Arguments
    ///
    /// * `key_material` - Raw verification key bytes
    /// * `data` - Original data that was signed
    /// * `signature` - Signature to verify
    ///
    /// # Returns
    ///
    /// `true` if the signature is valid, `false` otherwise
    ///
    /// # Errors
    ///
    /// Returns an error if verification fails due to:
    /// - Invalid key material
    /// - Malformed signature
    /// - Verification operation failure
    async fn verify(
        &self,
        key_material: &[u8],
        data: &[u8],
        signature: &[u8],
    ) -> Result<bool, BearDogError>;

    /// Derive a key from root key material
    ///
    /// Uses HKDF (HMAC-based Key Derivation Function) or similar to derive
    /// a new key from existing key material.
    ///
    /// # Arguments
    ///
    /// * `root_key` - Root key material
    /// * `derivation_data` - Context-specific derivation data (salt, info, etc.)
    ///
    /// # Returns
    ///
    /// Derived key bytes (typically 32 bytes)
    ///
    /// # Errors
    ///
    /// Returns an error if key derivation fails due to:
    /// - Invalid root key
    /// - Derivation operation failure
    async fn derive_key(
        &self,
        root_key: &[u8],
        derivation_data: &[u8],
    ) -> Result<Vec<u8>, BearDogError> {
        let _ = (root_key, derivation_data);
        // Default implementation returns a fixed-size key
        // Implementations should override this with proper HKDF
        Ok(vec![0; 32])
    }
}

// Note: KeyType is defined in beardog-tunnel/types/key.rs
// We don't re-export it here to avoid duplication
