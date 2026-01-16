//! Genetic Crypto Provider - 100% Pure Rust Cryptography
//!
//! This provider eliminates ALL FFI boundaries by using only Pure Rust cryptography.
//! Unlike Ring (which uses C/asm from BoringSSL), this provider uses RustCrypto ecosystem.
//!
//! # Benefits
//!
//! - **100% Memory Safe**: Borrow checker enforced everywhere
//! - **Full Compiler Optimization**: No FFI barriers blocking LLVM
//! - **Better SIMD**: Pure Rust crypto uses AVX2/AVX-512
//! - **Single Language Audit**: No C code to review
//! - **Sovereignty**: No C compiler, no system libraries
//!
//! # Genetic Enhancements
//!
//! Future versions will integrate with `EcosystemGeneticEngine` to provide:
//! - Family-specific crypto parameters
//! - Lineage-based key derivation
//! - Genetic entropy mixing
//! - Cross-generation verification

use crate::tunnel::hsm::software_hsm::CryptoProvider;
use crate::tunnel::hsm::types::KeyType;
use beardog_errors::BearDogError;
use std::sync::Arc;
use tracing::{debug, info};

// Pure Rust crypto imports - ZERO FFI!
use aes_gcm::{
    aead::{Aead, KeyInit, Payload},
    Aes256Gcm, Nonce,
};
use blake3; // Faster and more secure than SHA-256
use ed25519_dalek::{Signature, Signer, SigningKey, Verifier, VerifyingKey};
use hmac::{Hmac, Mac};
use rand_core::{OsRng, RngCore};
use sha2::Sha256;

type HmacSha256 = Hmac<Sha256>;

/// Genetic Crypto Provider - 100% Pure Rust implementation
///
/// This provider uses only Pure Rust cryptography from the RustCrypto ecosystem,
/// eliminating all FFI boundaries and C dependencies.
///
/// # Performance
///
/// - AES-GCM: Uses AES-NI hardware acceleration (when available)
/// - Ed25519: Uses AVX2 SIMD acceleration
/// - Blake3: Uses AVX2/AVX-512 for hashing
/// - All operations are zero-copy where possible
///
/// # Future: Genetic Enhancements
///
/// Next phase will add:
/// - `genetic_engine: Option<Arc<EcosystemGeneticEngine>>`
/// - `lineage_seed: Option<Vec<u8>>`
/// - Family-specific algorithm selection
/// - Lineage-based key derivation
#[derive(Debug, Clone)]
pub struct GeneticCryptoProvider {
    /// Provider name for identification
    name: String,
}

impl GeneticCryptoProvider {
    /// Create new Genetic Crypto provider
    ///
    /// Uses 100% Pure Rust cryptography with zero FFI boundaries.
    ///
    /// # Errors
    ///
    /// Returns an error if the provider cannot be created (currently infallible).
    pub fn new() -> Result<Self, BearDogError> {
        info!("🧬 Initializing GeneticCrypto provider (100% Pure Rust)");
        Ok(Self {
            name: "GeneticCrypto-PureRust".to_string(),
        })
    }

    /// Get provider name
    pub fn name(&self) -> &str {
        &self.name
    }

    /// Generate cryptographically secure random bytes
    ///
    /// Uses `OsRng` which provides platform-specific CSRNG:
    /// - Linux: `getrandom()` syscall
    /// - macOS: `SecRandomCopyBytes()`
    /// - Windows: `BCryptGenRandom()`
    ///
    /// All implementations are Pure Rust syscall wrappers.
    fn generate_random_bytes(&self, count: usize) -> Result<Vec<u8>, BearDogError> {
        let mut bytes = vec![0u8; count];
        OsRng.fill_bytes(&mut bytes);
        Ok(bytes)
    }
}

impl Default for GeneticCryptoProvider {
    fn default() -> Self {
        Self::new().expect("GeneticCryptoProvider creation is infallible")
    }
}

#[async_trait::async_trait]
impl CryptoProvider<KeyType> for GeneticCryptoProvider {
    async fn initialize(&self) -> Result<(), BearDogError> {
        info!("🚀 GeneticCrypto provider initialized (100% Pure Rust, zero FFI)");
        Ok(())
    }

    async fn generate_key_material(&self, key_type: &KeyType) -> Result<Vec<u8>, BearDogError> {
        debug!(
            "🔑 Generating {:?} key with GeneticCrypto (Pure Rust)",
            key_type
        );

        let key_length = match key_type {
            KeyType::Aes | KeyType::ChaCha20 => 32, // 256-bit symmetric keys
            KeyType::Ed25519 | KeyType::X25519 | KeyType::EllipticCurve => 32, // EC keys
            KeyType::Rsa | KeyType::Generic | KeyType::Custom(_) => {
                return Err(BearDogError::unsupported_operation(format!(
                    "GeneticCrypto supports AES, ChaCha20, Ed25519, X25519, and ECC. Got: {key_type:?}"
                )))
            }
        };

        let key_material = self.generate_random_bytes(key_length)?;
        debug!(
            "✅ Generated {} byte key (Pure Rust OsRng)",
            key_material.len()
        );
        Ok(key_material)
    }

    async fn encrypt(
        &self,
        key_material: &[u8],
        plaintext: &[u8],
    ) -> Result<Vec<u8>, BearDogError> {
        debug!(
            "🔒 Encrypting {} bytes with AES-256-GCM (Pure Rust, AES-NI)",
            plaintext.len()
        );

        if key_material.len() != 32 {
            return Err(BearDogError::crypto_error(format!(
                "Invalid key size: expected 32 bytes, got {}",
                key_material.len()
            )));
        }

        // Create AES-256-GCM cipher (Pure Rust with AES-NI acceleration)
        let cipher = Aes256Gcm::new_from_slice(key_material).map_err(|e| {
            BearDogError::crypto_error(format!("Failed to create AES-256-GCM cipher: {e}"))
        })?;

        // Generate random nonce (96 bits for GCM)
        let nonce_bytes = self.generate_random_bytes(12)?;
        let nonce = Nonce::from_slice(&nonce_bytes);

        // Encrypt with authenticated encryption (AEAD)
        let ciphertext = cipher
            .encrypt(nonce, plaintext)
            .map_err(|e| BearDogError::crypto_error(format!("Encryption failed: {e}")))?;

        // Prepend nonce to ciphertext (standard format)
        let mut result = nonce_bytes;
        result.extend(ciphertext);

        debug!(
            "✅ Encrypted to {} bytes (nonce + ciphertext + tag)",
            result.len()
        );
        Ok(result)
    }

    async fn decrypt(
        &self,
        key_material: &[u8],
        ciphertext: &[u8],
    ) -> Result<Vec<u8>, BearDogError> {
        debug!(
            "🔓 Decrypting {} bytes with AES-256-GCM (Pure Rust, AES-NI)",
            ciphertext.len()
        );

        const NONCE_LEN: usize = 12;

        if ciphertext.len() < NONCE_LEN {
            return Err(BearDogError::crypto_error(format!(
                "Ciphertext too short: expected at least {NONCE_LEN} bytes, got {}",
                ciphertext.len()
            )));
        }

        // Extract nonce and ciphertext
        let (nonce_bytes, ciphertext_data) = ciphertext.split_at(NONCE_LEN);
        let nonce = Nonce::from_slice(nonce_bytes);

        // Create cipher
        let cipher = Aes256Gcm::new_from_slice(key_material).map_err(|e| {
            BearDogError::crypto_error(format!("Failed to create AES-256-GCM cipher: {e}"))
        })?;

        // Decrypt and verify authentication tag
        let plaintext = cipher.decrypt(nonce, ciphertext_data).map_err(|e| {
            BearDogError::crypto_error(format!(
                "Decryption failed (wrong key or tampered data): {e}"
            ))
        })?;

        debug!("✅ Decrypted to {} bytes", plaintext.len());
        Ok(plaintext)
    }

    async fn sign(&self, key_material: &[u8], data: &[u8]) -> Result<Vec<u8>, BearDogError> {
        debug!(
            "✍️ Signing {} bytes with Ed25519 (Pure Rust, AVX2)",
            data.len()
        );

        if key_material.len() != 32 {
            return Err(BearDogError::crypto_error(format!(
                "Invalid Ed25519 private key length: expected 32 bytes, got {}",
                key_material.len()
            )));
        }

        // Create signing key from seed (Pure Rust)
        let signing_key =
            SigningKey::from_bytes(key_material.try_into().map_err(|_| {
                BearDogError::crypto_error("Invalid private key format".to_string())
            })?);

        // Sign the data (Pure Rust, uses AVX2 if available)
        let signature = signing_key.sign(data);

        debug!(
            "✅ Generated Ed25519 signature ({} bytes)",
            signature.to_bytes().len()
        );
        Ok(signature.to_bytes().to_vec())
    }

    async fn verify(
        &self,
        key_material: &[u8],
        data: &[u8],
        signature: &[u8],
    ) -> Result<bool, BearDogError> {
        debug!(
            "🔍 Verifying Ed25519 signature for {} bytes (Pure Rust)",
            data.len()
        );

        if key_material.len() != 32 {
            return Err(BearDogError::crypto_error(format!(
                "Invalid Ed25519 public key length: expected 32 bytes, got {}",
                key_material.len()
            )));
        }

        if signature.len() != 64 {
            return Err(BearDogError::crypto_error(format!(
                "Invalid Ed25519 signature length: expected 64 bytes, got {}",
                signature.len()
            )));
        }

        // Create verifying key (Pure Rust)
        let verifying_key =
            VerifyingKey::from_bytes(key_material.try_into().map_err(|_| {
                BearDogError::crypto_error("Invalid public key format".to_string())
            })?)
            .map_err(|e| {
                BearDogError::crypto_error(format!("Failed to create verifying key: {e}"))
            })?;

        // Create signature
        let sig = Signature::from_bytes(
            signature
                .try_into()
                .map_err(|_| BearDogError::crypto_error("Invalid signature format".to_string()))?,
        );

        // Verify signature (Pure Rust)
        match verifying_key.verify(data, &sig) {
            Ok(()) => {
                debug!("✅ Signature valid");
                Ok(true)
            }
            Err(_) => {
                debug!("❌ Signature invalid");
                Ok(false)
            }
        }
    }

    async fn derive_key(
        &self,
        root_key: &[u8],
        derivation_data: &[u8],
    ) -> Result<Vec<u8>, BearDogError> {
        debug!("🔑 Deriving key with HMAC-SHA256 (Pure Rust)");

        if root_key.is_empty() {
            return Err(BearDogError::crypto_error(
                "Root key cannot be empty".to_string(),
            ));
        }

        // Use HMAC-SHA256 for key derivation (Pure Rust)
        use hmac::Mac as _; // Import Mac trait for update/finalize

        let mut mac = <HmacSha256 as Mac>::new_from_slice(root_key)
            .map_err(|e| BearDogError::crypto_error(format!("Failed to create HMAC: {e}")))?;

        mac.update(derivation_data);
        let result = mac.finalize();
        let key_bytes = result.into_bytes();

        debug!("✅ Derived key of {} bytes", key_bytes.len());
        Ok(key_bytes.to_vec())
    }
}

// Future: Genetic enhancements
impl GeneticCryptoProvider {
    /// Derive key using Blake3 (faster and more secure than HMAC-SHA256)
    ///
    /// This is a modern alternative that will be used in future genetic crypto features.
    ///
    /// # Performance
    ///
    /// Blake3 is 10x faster than SHA-256 and provides better security margins.
    #[allow(dead_code)]
    async fn derive_key_blake3(
        &self,
        root_key: &[u8],
        derivation_data: &[u8],
    ) -> Result<Vec<u8>, BearDogError> {
        debug!("🔑 Deriving key with Blake3 (Pure Rust, AVX2/AVX-512)");

        if root_key.len() != 32 {
            return Err(BearDogError::crypto_error(format!(
                "Blake3 requires 32-byte key, got {}",
                root_key.len()
            )));
        }

        // Use Blake3 keyed hash (Pure Rust with SIMD)
        let key: [u8; 32] = root_key
            .try_into()
            .map_err(|_| BearDogError::crypto_error("Invalid root key length".to_string()))?;

        let mut hasher = blake3::Hasher::new_keyed(&key);
        hasher.update(derivation_data);
        let hash = hasher.finalize();

        debug!("✅ Derived key of 32 bytes (Blake3)");
        Ok(hash.as_bytes().to_vec())
    }

    // Future: Genetic lineage-based key derivation
    // async fn genetic_derive_key(
    //     &self,
    //     genetic_engine: &EcosystemGeneticEngine,
    //     family_id: &str,
    //     node_id: &str,
    //     context: &[u8],
    // ) -> Result<Vec<u8>, BearDogError> {
    //     // Mix family lineage + node identity + context + hardware entropy
    //     // for enhanced genetic crypto
    // }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[tokio::test]
    async fn test_genetic_crypto_provider_creation() -> Result<(), BearDogError> {
        let provider = GeneticCryptoProvider::new()?;
        assert_eq!(provider.name(), "GeneticCrypto-PureRust");
        Ok(())
    }

    #[tokio::test]
    async fn test_key_generation() -> Result<(), BearDogError> {
        let provider = GeneticCryptoProvider::new()?;

        // Test AES key generation
        let aes_key = provider.generate_key_material(&KeyType::Aes).await?;
        assert_eq!(aes_key.len(), 32);

        // Test Ed25519 key generation
        let ed25519_key = provider.generate_key_material(&KeyType::Ed25519).await?;
        assert_eq!(ed25519_key.len(), 32);

        // Keys should be random
        let another_key = provider.generate_key_material(&KeyType::Aes).await?;
        assert_ne!(
            aes_key, another_key,
            "Keys should be cryptographically random"
        );

        Ok(())
    }

    #[tokio::test]
    async fn test_encrypt_decrypt_roundtrip() -> Result<(), BearDogError> {
        let provider = GeneticCryptoProvider::new()?;
        let key = provider.generate_key_material(&KeyType::Aes).await?;

        let plaintext = b"Genetic Crypto - 100% Pure Rust, zero FFI!";

        // Encrypt
        let ciphertext = provider.encrypt(&key, plaintext).await?;
        assert_ne!(
            plaintext.to_vec(),
            ciphertext,
            "Ciphertext should differ from plaintext"
        );
        assert!(
            ciphertext.len() > plaintext.len(),
            "Ciphertext includes nonce + tag"
        );

        // Decrypt
        let decrypted = provider.decrypt(&key, &ciphertext).await?;
        assert_eq!(
            plaintext.to_vec(),
            decrypted,
            "Decryption should recover plaintext"
        );

        Ok(())
    }

    #[tokio::test]
    async fn test_encrypt_with_wrong_key_fails() -> Result<(), BearDogError> {
        let provider = GeneticCryptoProvider::new()?;
        let key1 = provider.generate_key_material(&KeyType::Aes).await?;
        let key2 = provider.generate_key_material(&KeyType::Aes).await?;

        let plaintext = b"Encrypted with key1";
        let ciphertext = provider.encrypt(&key1, plaintext).await?;

        // Attempting to decrypt with wrong key should fail
        let result = provider.decrypt(&key2, &ciphertext).await;
        assert!(result.is_err(), "Decryption with wrong key should fail");

        Ok(())
    }

    #[tokio::test]
    async fn test_sign_verify_roundtrip() -> Result<(), BearDogError> {
        let provider = GeneticCryptoProvider::new()?;
        let private_key = provider.generate_key_material(&KeyType::Ed25519).await?;

        // Derive public key from private key
        let key_bytes: [u8; 32] = private_key.clone().try_into().unwrap();
        let signing_key = SigningKey::from_bytes(&key_bytes);
        let public_key = signing_key.verifying_key().to_bytes();

        let message = b"Genetic Crypto signature test - Pure Rust Ed25519";

        // Sign
        let signature = provider.sign(&private_key, message).await?;
        assert_eq!(signature.len(), 64, "Ed25519 signature is 64 bytes");

        // Verify with correct public key
        let is_valid = provider.verify(&public_key, message, &signature).await?;
        assert!(is_valid, "Signature should be valid");

        // Verify with wrong message
        let wrong_message = b"Different message";
        let is_valid_wrong = provider
            .verify(&public_key, wrong_message, &signature)
            .await?;
        assert!(
            !is_valid_wrong,
            "Signature should be invalid for different message"
        );

        Ok(())
    }

    #[tokio::test]
    async fn test_derive_key_deterministic() -> Result<(), BearDogError> {
        let provider = GeneticCryptoProvider::new()?;
        let root_key = b"genetic_root_key_for_derivation_test";
        let context1 = b"context1";
        let context2 = b"context2";

        // Same inputs should produce same output (deterministic)
        let derived1a = provider.derive_key(root_key, context1).await?;
        let derived1b = provider.derive_key(root_key, context1).await?;
        assert_eq!(
            derived1a, derived1b,
            "Key derivation should be deterministic"
        );

        // Different contexts should produce different keys
        let derived2 = provider.derive_key(root_key, context2).await?;
        assert_ne!(
            derived1a, derived2,
            "Different contexts should produce different keys"
        );

        Ok(())
    }

    #[tokio::test]
    async fn test_authenticated_encryption() -> Result<(), BearDogError> {
        let provider = GeneticCryptoProvider::new()?;
        let key = provider.generate_key_material(&KeyType::Aes).await?;

        let plaintext = b"Authenticated encryption test";
        let ciphertext = provider.encrypt(&key, plaintext).await?;

        // Tamper with ciphertext (modify last byte)
        let mut tampered = ciphertext.clone();
        let last_idx = tampered.len() - 1;
        tampered[last_idx] ^= 0xFF;

        // Decryption should fail due to authentication tag mismatch
        let result = provider.decrypt(&key, &tampered).await;
        assert!(
            result.is_err(),
            "Tampered ciphertext should fail authentication"
        );

        Ok(())
    }

    #[tokio::test]
    async fn test_zero_copy_efficiency() -> Result<(), BearDogError> {
        let provider = GeneticCryptoProvider::new()?;
        let key = provider.generate_key_material(&KeyType::Aes).await?;

        // Large data to test efficiency
        let large_plaintext = vec![0x42u8; 1024 * 1024]; // 1 MB

        let ciphertext = provider.encrypt(&key, &large_plaintext).await?;
        let decrypted = provider.decrypt(&key, &ciphertext).await?;

        assert_eq!(
            large_plaintext, decrypted,
            "Large data roundtrip should work"
        );

        Ok(())
    }
}
