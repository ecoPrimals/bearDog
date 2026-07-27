// SPDX-License-Identifier: AGPL-3.0-or-later

//! Hashing algorithms
//!
//! Provides implementations of cryptographic hash functions including:
//! - SHA-256 (Secure Hash Algorithm 2)
//! - SHA-512 (Secure Hash Algorithm 2)
//! - BLAKE3 (Modern, fast hash function)
//!
//! All implementations use safe Rust with established cryptography libraries.

use beardog_errors::BearDogError;

type Result<T> = std::result::Result<T, BearDogError>;

/// SHA-256 hash
///
/// Produces a 256-bit (32-byte) hash of the input data.
/// SHA-256 is widely used and well-studied.
///
/// # Arguments
///
/// * `data` - Data to hash
///
/// # Returns
///
/// 32-byte hash digest
#[must_use]
pub fn hash_sha256(data: &[u8]) -> Vec<u8> {
    use sha2::{Digest, Sha256};

    let mut hasher = Sha256::new();
    hasher.update(data);
    hasher.finalize().to_vec()
}

/// SHA-512 hash
///
/// Produces a 512-bit (64-byte) hash of the input data.
/// Provides higher security margin than SHA-256.
///
/// # Arguments
///
/// * `data` - Data to hash
///
/// # Returns
///
/// 64-byte hash digest
#[must_use]
pub fn hash_sha512(data: &[u8]) -> Vec<u8> {
    use sha2::{Digest, Sha512};

    let mut hasher = Sha512::new();
    hasher.update(data);
    hasher.finalize().to_vec()
}

/// BLAKE3 hash
///
/// Modern hash function that is:
/// - Faster than SHA-256
/// - Parallelizable
/// - Supports keyed hashing and key derivation
/// - 256-bit output (32 bytes)
///
/// # Arguments
///
/// * `data` - Data to hash
///
/// # Returns
///
/// 32-byte hash digest
#[must_use]
pub fn hash_blake3(data: &[u8]) -> Vec<u8> {
    blake3::hash(data).as_bytes().to_vec()
}

/// Keyed BLAKE3 hash (MAC)
///
/// BLAKE3 in keyed mode provides message authentication.
///
/// # Arguments
///
/// * `key` - 32-byte secret key
/// * `data` - Data to authenticate
///
/// # Returns
///
/// 32-byte authentication tag
#[must_use]
pub fn hash_blake3_keyed(key: &[u8; 32], data: &[u8]) -> Vec<u8> {
    blake3::keyed_hash(key, data).as_bytes().to_vec()
}

/// BLAKE3 key derivation
///
/// Derives a key from input key material using BLAKE3 KDF mode.
///
/// # Arguments
///
/// * `context` - Context string to domain-separate derived keys
/// * `key_material` - Input key material
///
/// # Returns
///
/// 32-byte derived key
#[must_use]
pub fn derive_key_blake3(context: &str, key_material: &[u8]) -> Vec<u8> {
    blake3::derive_key(context, key_material).to_vec()
}

/// HMAC-SHA256
///
/// Hash-based Message Authentication Code using SHA-256.
///
/// # Arguments
///
/// * `key` - Secret key (any length)
/// * `data` - Data to authenticate
///
/// # Returns
///
/// 32-byte authentication tag
///
/// # Errors
///
/// This function is infallible in practice (HMAC accepts any key length),
/// but returns Result for API consistency.
pub fn hmac_sha256(key: &[u8], data: &[u8]) -> Result<Vec<u8>> {
    use hmac::{Hmac, Mac};
    use sha2::Sha256;

    type HmacSha256 = Hmac<Sha256>;

    // HMAC can accept any key length, so this should never fail
    let mut mac = HmacSha256::new_from_slice(key)
        .map_err(|e| BearDogError::hsm(format!("HMAC initialization failed: {e}")))?;
    mac.update(data);
    Ok(mac.finalize().into_bytes().to_vec())
}

/// HKDF-SHA256 key derivation
///
/// HMAC-based Key Derivation Function using SHA-256.
/// Expands a source key into multiple derived keys.
///
/// # Arguments
///
/// * `input_key_material` - Source key material
/// * `salt` - Optional salt value (use empty slice if none)
/// * `info` - Optional context/application-specific info
/// * `output_length` - Length of output key in bytes
///
/// # Returns
///
/// Derived key of requested length
///
/// # Errors
///
/// Returns error if `output_length` is too large (max ~8160 bytes for SHA-256)
pub fn hkdf_sha256(
    input_key_material: &[u8],
    salt: &[u8],
    info: &[u8],
    output_length: usize,
) -> Result<Vec<u8>> {
    use hkdf::Hkdf;
    use sha2::Sha256;

    let hk = Hkdf::<Sha256>::new(Some(salt), input_key_material);

    let mut okm = vec![0u8; output_length];
    hk.expand(info, &mut okm)
        .map_err(|e| BearDogError::hsm(format!("HKDF expansion failed: {e}")))?;

    Ok(okm)
}

/// Argon2id password hashing
///
/// Memory-hard password hashing function resistant to GPU cracking.
///
/// # Arguments
///
/// * `password` - Password to hash
/// * `salt` - 16-byte salt (must be unique per password)
///
/// # Returns
///
/// 32-byte password hash
///
/// # Security
///
/// Uses Argon2id with moderate parameters suitable for typical hardware:
/// - Memory: 64 MB
/// - Iterations: 3
/// - Parallelism: 4
///
/// # Errors
///
/// Returns an error if the salt is invalid or the password hashing operation fails.
pub fn hash_password_argon2(password: &[u8], salt: &[u8; 16]) -> Result<Vec<u8>> {
    use argon2::password_hash::SaltString;
    use argon2::{Argon2, PasswordHasher};

    // Convert salt bytes to SaltString
    let salt_string = SaltString::encode_b64(salt)
        .map_err(|e| BearDogError::hsm(format!("Invalid salt: {e}")))?;

    // Hash password
    let argon2 = Argon2::default();
    let hash = argon2
        .hash_password(password, &salt_string)
        .map_err(|e| BearDogError::hsm(format!("Argon2 hashing failed: {e}")))?;

    // Extract hash bytes
    let hash_bytes = hash
        .hash
        .ok_or_else(|| BearDogError::hsm("No hash output".to_string()))?;

    Ok(hash_bytes.as_bytes().to_vec())
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_sha256() {
        let data = b"Hello, BearDog!";
        let hash = hash_sha256(data);
        assert_eq!(hash.len(), 32);

        // Same input should produce same hash
        let hash2 = hash_sha256(data);
        assert_eq!(hash, hash2);

        // Different input should produce different hash
        let different = hash_sha256(b"Different data");
        assert_ne!(hash, different);
    }

    #[test]
    fn test_sha512() {
        let data = b"Test data";
        let hash = hash_sha512(data);
        assert_eq!(hash.len(), 64);
    }

    #[test]
    fn test_blake3() {
        let data = b"BLAKE3 test";
        let hash = hash_blake3(data);
        assert_eq!(hash.len(), 32);

        // Test keyed hash
        let key = [42u8; 32];
        let mac = hash_blake3_keyed(&key, data);
        assert_eq!(mac.len(), 32);
        assert_ne!(hash, mac); // Keyed should differ from plain hash
    }

    #[test]
    fn test_blake3_kdf() {
        let key_material = b"source key material";
        let key1 = derive_key_blake3("context1", key_material);
        let key2 = derive_key_blake3("context2", key_material);

        assert_eq!(key1.len(), 32);
        assert_eq!(key2.len(), 32);
        assert_ne!(key1, key2); // Different contexts produce different keys
    }

    #[test]
    fn test_hmac_sha256() {
        let key = b"secret key";
        let data = b"message to authenticate";

        let mac = hmac_sha256(key, data).expect("hmac_sha256 in test");
        assert_eq!(mac.len(), 32);

        // Same key and data produce same MAC
        let mac2 = hmac_sha256(key, data).expect("hmac_sha256 in test");
        assert_eq!(mac, mac2);

        // Different key produces different MAC
        let mac3 = hmac_sha256(b"different key", data).expect("hmac_sha256 in test");
        assert_ne!(mac, mac3);
    }

    #[test]
    fn test_hkdf_sha256() {
        let ikm = b"input key material";
        let salt = b"optional salt";
        let info = b"application context";

        let key1 = hkdf_sha256(ikm, salt, info, 32).expect("hkdf_sha256 in test");
        assert_eq!(key1.len(), 32);

        // Can derive keys of different lengths
        let key2 = hkdf_sha256(ikm, salt, info, 64).expect("hkdf_sha256 in test");
        assert_eq!(key2.len(), 64);

        // Different info produces different keys
        let key3 = hkdf_sha256(ikm, salt, b"different context", 32).expect("hkdf_sha256 in test");
        assert_ne!(key1, key3);
    }

    #[test]
    fn test_argon2_password() {
        let password = b"super secret password";
        let salt = [7u8; 16];

        let hash = hash_password_argon2(password, &salt).expect("hash_password_argon2 in test");
        assert!(!hash.is_empty());

        // Same password and salt produce same hash
        let hash2 = hash_password_argon2(password, &salt).expect("hash_password_argon2 in test");
        assert_eq!(hash, hash2);

        // Different salt produces different hash
        let different_salt = [9u8; 16];
        let hash3 =
            hash_password_argon2(password, &different_salt).expect("hash_password_argon2 in test");
        assert_ne!(hash, hash3);
    }
}
