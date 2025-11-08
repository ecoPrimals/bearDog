//! # Crypto Migration Helpers [DEPRECATED - NOT IN USE]
//!
//! **Status**: This module is NOT currently used in the codebase (Nov 2025)
//! **Action**: Marked for removal in v3.3.0 (Q2 2026)
//!
//! This module was intended to provide compatibility wrappers during crypto migration,
//! but the codebase has fully migrated to `UniversalCryptoProvider` directly.
//!
//! ## Current Status
//!
//! - ✅ Migration to UniversalCryptoProvider complete
//! - ✅ No active imports of this module
//! - ⚠️ Module kept for historical reference only
//! - 🗑️ Scheduled for removal in Q2 2026
//!
//! ## If You Need Crypto Operations
//!
//! Use the modern approach directly:
//! ```rust
//! use beardog_tunnel::tunnel::hsm::crypto::UniversalCryptoProvider;
//!
//! // Modern async approach:
//! let provider = UniversalCryptoProvider::new_rustcrypto();
//! let encrypted = provider.encrypt_symmetric(...).await?;
//! ```
//!
//! ## Migration Timeline
//!
//! 1. **Phase 1 (Complete)**: Migrated to UniversalCryptoProvider ✅
//! 2. **Phase 2 (Q1 2026)**: Mark module as deprecated
//! 3. **Phase 3 (Q2 2026)**: Remove this compatibility module

use beardog_errors::BearDogError;
use ring::rand::{SecureRandom, SystemRandom};
use sha2::{Digest, Sha256};

/// Crypto migration helper - provides sync wrappers for common crypto operations
///
/// **DEPRECATED**: This module is not in use. Migrate to UniversalCryptoProvider.
///
/// This struct provides synchronous equivalents of the deprecated crypto_utils
/// functions, but internally uses secure implementations.
#[deprecated(
    since = "3.1.0",
    note = "Not in use. Use beardog_tunnel::tunnel::hsm::crypto::UniversalCryptoProvider instead"
)]
pub struct CryptoMigration;

impl CryptoMigration {
    // =========================================================================
    // Random Number Generation
    // =========================================================================

    /// Generate cryptographically secure random bytes
    ///
    /// This is a direct replacement for `crypto_utils::secure_random_bytes()`.
    ///
    /// ## Example
    ///
    /// ```rust
    /// use beardog_utils::crypto_migration::CryptoMigration;
    ///
    /// let key = CryptoMigration::secure_random_bytes(32); // 256-bit key
    /// assert_eq!(key.len(), 32);
    /// ```
    ///
    /// ## Migration Path
    ///
    /// For async code, migrate to:
    /// ```ignore
    /// use beardog_security::crypto_utils::BearDogCrypto;
    /// let key = BearDogCrypto::generate_secure_random(32);
    /// ```
    pub fn secure_random_bytes(size: usize) -> Vec<u8> {
        let rng = SystemRandom::new();
        let mut bytes = vec![0u8; size];

        match rng.fill(&mut bytes) {
            Ok(()) => bytes,
            Err(_) => {
                // Fallback to OS random
                use rand::RngCore;
                let mut rng = rand::thread_rng();
                rng.fill_bytes(&mut bytes);
                bytes
            }
        }
    }

    /// Generate a cryptographic salt (32 bytes / 256 bits)
    ///
    /// This is a direct replacement for `crypto_utils::generate_salt()`.
    ///
    /// ## Example
    ///
    /// ```rust
    /// use beardog_utils::crypto_migration::CryptoMigration;
    ///
    /// let salt = CryptoMigration::generate_salt();
    /// assert_eq!(salt.len(), 32);
    /// ```
    pub fn generate_salt() -> Vec<u8> {
        Self::secure_random_bytes(32)
    }

    /// Generate a cryptographic nonce/IV of specified size
    ///
    /// This is a direct replacement for `crypto_utils::generate_nonce()`.
    ///
    /// ## Example
    ///
    /// ```rust
    /// use beardog_utils::crypto_migration::CryptoMigration;
    ///
    /// let nonce = CryptoMigration::generate_nonce(12); // AES-GCM nonce
    /// assert_eq!(nonce.len(), 12);
    /// ```
    pub fn generate_nonce(size: usize) -> Vec<u8> {
        Self::secure_random_bytes(size)
    }

    // =========================================================================
    // Hashing Operations
    // =========================================================================

    /// Compute SHA-256 hash of data
    ///
    /// This is a direct replacement for `crypto_utils::sha256_hash()`.
    ///
    /// ## Example
    ///
    /// ```rust
    /// use beardog_utils::crypto_migration::CryptoMigration;
    ///
    /// let data = b"Hello, BearDog!";
    /// let hash = CryptoMigration::sha256_hash(data);
    /// assert_eq!(hash.len(), 32); // SHA-256 produces 32 bytes
    /// ```
    pub fn sha256_hash(data: &[u8]) -> Vec<u8> {
        let mut hasher = Sha256::new();
        hasher.update(data);
        hasher.finalize().to_vec()
    }

    /// Compute SHA-256 hash and return as hex string
    ///
    /// This is a direct replacement for `crypto_utils::sha256_hex()`.
    ///
    /// ## Example
    ///
    /// ```rust
    /// use beardog_utils::crypto_migration::CryptoMigration;
    ///
    /// let data = b"Hello, BearDog!";
    /// let hash_hex = CryptoMigration::sha256_hex(data);
    /// assert_eq!(hash_hex.len(), 64); // 32 bytes = 64 hex chars
    /// ```
    pub fn sha256_hex(data: &[u8]) -> String {
        let hash = Self::sha256_hash(data);
        hex::encode(hash)
    }

    // =========================================================================
    // HMAC Operations
    // =========================================================================

    /// Compute HMAC-SHA256 of data with a key
    ///
    /// This is a direct replacement for `crypto_utils::hmac_sha256()`.
    ///
    /// ## Example
    ///
    /// ```rust
    /// use beardog_utils::crypto_migration::CryptoMigration;
    ///
    /// let key = b"secret-key";
    /// let data = b"message-to-authenticate";
    /// let mac = CryptoMigration::hmac_sha256(key, data)
    ///     .expect("HMAC computation failed");
    /// assert_eq!(mac.len(), 32);
    /// ```
    pub fn hmac_sha256(key: &[u8], data: &[u8]) -> Result<Vec<u8>, BearDogError> {
        use hmac::{Hmac, Mac};
        type HmacSha256 = Hmac<Sha256>;

        let mut mac = HmacSha256::new_from_slice(key)
            .map_err(|e| BearDogError::crypto_error(format!("Invalid HMAC key length: {}", e)))?;

        mac.update(data);
        Ok(mac.finalize().into_bytes().to_vec())
    }

    /// Verify HMAC-SHA256 tag
    ///
    /// This is a direct replacement for `crypto_utils::verify_hmac_sha256()`.
    ///
    /// ## Example
    ///
    /// ```rust
    /// use beardog_utils::crypto_migration::CryptoMigration;
    ///
    /// let key = b"secret-key";
    /// let data = b"message";
    /// let mac = CryptoMigration::hmac_sha256(key, data).unwrap();
    ///
    /// // Verify the MAC
    /// assert!(CryptoMigration::verify_hmac_sha256(key, data, &mac).unwrap());
    ///
    /// // Wrong MAC should fail
    /// let wrong_mac = vec![0u8; 32];
    /// assert!(!CryptoMigration::verify_hmac_sha256(key, data, &wrong_mac).unwrap());
    /// ```
    pub fn verify_hmac_sha256(key: &[u8], data: &[u8], expected_mac: &[u8]) -> Result<bool, BearDogError> {
        use hmac::{Hmac, Mac};
        type HmacSha256 = Hmac<Sha256>;

        let mut mac = HmacSha256::new_from_slice(key)
            .map_err(|e| BearDogError::crypto_error(format!("Invalid HMAC key length: {}", e)))?;

        mac.update(data);

        // Constant-time comparison
        match mac.verify_slice(expected_mac) {
            Ok(_) => Ok(true),
            Err(_) => Ok(false),
        }
    }

    // =========================================================================
    // Key Derivation
    // =========================================================================

    /// Derive key using HKDF-SHA256
    ///
    /// This provides a simplified interface for key derivation.
    ///
    /// ## Example
    ///
    /// ```rust
    /// use beardog_utils::crypto_migration::CryptoMigration;
    ///
    /// let input_key = b"master-key";
    /// let salt = b"unique-salt";
    /// let info = b"key-derivation-context";
    ///
    /// let derived = CryptoMigration::derive_key_hkdf(input_key, salt, info, 32)
    ///     .expect("Key derivation failed");
    /// assert_eq!(derived.len(), 32);
    /// ```
    pub fn derive_key_hkdf(
        input_key: &[u8],
        salt: &[u8],
        info: &[u8],
        output_length: usize,
    ) -> Result<Vec<u8>, BearDogError> {
        use hkdf::Hkdf;

        let hk = Hkdf::<Sha256>::new(Some(salt), input_key);
        let mut output = vec![0u8; output_length];

        hk.expand(info, &mut output)
            .map_err(|e| BearDogError::crypto_error(format!("HKDF expansion failed: {}", e)))?;

        Ok(output)
    }
}

// =============================================================================
// Async Migration Helpers (for full async migration)
// =============================================================================

/// Async crypto operations placeholder
///
/// For full async migration to UniversalCryptoProvider, consumers should:
/// 1. Add dependency: `beardog-tunnel = { path = "../beardog-tunnel" }`
/// 2. Use UniversalCryptoProvider directly from beardog-tunnel
///
/// ## Example (Full Async Migration)
///
/// ```ignore
/// use beardog_tunnel::tunnel::hsm::crypto::provider::UniversalCryptoProvider;
/// use beardog_tunnel::tunnel::hsm::crypto::providers::RustCryptoProvider;
///
/// let provider = RustCryptoProvider::new();
/// let encrypted = provider.encrypt_symmetric(...).await?;
/// ```
///
/// This placeholder exists to document the migration path.
pub struct AsyncCryptoMigration;

impl AsyncCryptoMigration {
    /// Placeholder - see module documentation for async migration path
    ///
    /// For async encryption, use `beardog_tunnel::hsm::crypto::provider::UniversalCryptoProvider`
    /// directly in modules that already depend on beardog-tunnel.
    #[deprecated(note = "Use UniversalCryptoProvider from beardog-tunnel directly")]
    pub fn placeholder() {
        unimplemented!("Add beardog-tunnel dependency and use UniversalCryptoProvider")
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_secure_random_bytes() {
        let bytes1 = CryptoMigration::secure_random_bytes(32);
        let bytes2 = CryptoMigration::secure_random_bytes(32);

        assert_eq!(bytes1.len(), 32);
        assert_eq!(bytes2.len(), 32);
        assert_ne!(bytes1, bytes2); // Should be different
    }

    #[test]
    fn test_salt_generation() {
        let salt = CryptoMigration::generate_salt();
        assert_eq!(salt.len(), 32);
    }

    #[test]
    fn test_nonce_generation() {
        let nonce = CryptoMigration::generate_nonce(12);
        assert_eq!(nonce.len(), 12);
    }

    #[test]
    fn test_sha256_hash() {
        let data = b"Hello, BearDog!";
        let hash = CryptoMigration::sha256_hash(data);
        assert_eq!(hash.len(), 32);

        // Same input should produce same hash
        let hash2 = CryptoMigration::sha256_hash(data);
        assert_eq!(hash, hash2);
    }

    #[test]
    fn test_sha256_hex() {
        let data = b"Hello, BearDog!";
        let hash_hex = CryptoMigration::sha256_hex(data);
        assert_eq!(hash_hex.len(), 64); // 32 bytes = 64 hex chars
        assert!(hash_hex.chars().all(|c| c.is_ascii_hexdigit()));
    }

    #[test]
    fn test_hmac_sha256() {
        let key = b"secret-key";
        let data = b"message";

        let mac = CryptoMigration::hmac_sha256(key, data).expect("HMAC failed");
        assert_eq!(mac.len(), 32);

        // Same inputs should produce same MAC
        let mac2 = CryptoMigration::hmac_sha256(key, data).expect("HMAC failed");
        assert_eq!(mac, mac2);
    }

    #[test]
    fn test_verify_hmac_sha256() {
        let key = b"secret-key";
        let data = b"message";

        let mac = CryptoMigration::hmac_sha256(key, data).unwrap();

        // Correct MAC should verify
        assert!(CryptoMigration::verify_hmac_sha256(key, data, &mac).unwrap());

        // Wrong MAC should not verify
        let wrong_mac = vec![0u8; 32];
        assert!(!CryptoMigration::verify_hmac_sha256(key, data, &wrong_mac).unwrap());

        // Wrong key should not verify
        let wrong_key = b"wrong-key";
        assert!(!CryptoMigration::verify_hmac_sha256(wrong_key, data, &mac).unwrap());
    }

    #[test]
    fn test_derive_key_hkdf() {
        let input_key = b"master-key";
        let salt = b"salt";
        let info = b"context";

        let derived1 = CryptoMigration::derive_key_hkdf(input_key, salt, info, 32).unwrap();
        assert_eq!(derived1.len(), 32);

        // Same inputs should produce same output
        let derived2 = CryptoMigration::derive_key_hkdf(input_key, salt, info, 32).unwrap();
        assert_eq!(derived1, derived2);

        // Different info should produce different output
        let derived3 = CryptoMigration::derive_key_hkdf(input_key, salt, b"different", 32).unwrap();
        assert_ne!(derived1, derived3);
    }
}

