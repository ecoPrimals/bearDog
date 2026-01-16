//! # Cryptographic Utilities - DEPRECATED
//!
//! ⚠️ **DEPRECATION NOTICE**: This module is being phased out in favor of the
//! canonical cryptographic operations in `beardog-security::crypto_utils`.
//!
//! ## Migration Path
//!
//! **Old** (beardog-utils):
//! ```rust
//! use beardog_utils::utils::crypto_utils::*;
//! let bytes = secure_random_bytes(32);
//! let nonce = generate_nonce(12);
//! ```
//!
//! **New** (beardog-security):
//! ```rust
//! use beardog_security::crypto_utils::BearDogCrypto;
//! let bytes = BearDogCrypto::generate_secure_random(32);
//! let nonce = BearDogCrypto::generate_secure_nonce(12);
//! ```
//!
//! ## Rationale
//!
//! Cryptographic operations should be centralized in the security crate for:
//! - Better audit trail and security review
//! - Consistent cryptographic implementation
//! - Single source of truth for crypto primitives
//! - Proper separation of concerns
//!
//! ## Removal Timeline
//!
//! Most functions in this module will be removed in **v3.3.0 (Q1 2026)**.
//! Only non-crypto utility functions (if any) will remain.

use beardog_errors::BearDogError;
use hmac::{Hmac, Mac};
use pbkdf2;
use rand::RngCore;
use sha2::{Digest, Sha256};
type HmacSha256 = Hmac<Sha256>;

/// Generate secure random bytes - DEPRECATED
///
/// ⚠️ **DEPRECATED**: Use `beardog_security::crypto_utils::BearDogCrypto::generate_secure_random` instead.
///
/// ## Migration
/// ```rust
/// // Old
/// use beardog_utils::utils::crypto_utils::secure_random_bytes;
/// let bytes = secure_random_bytes(32);
///
/// // New
/// use beardog_security::crypto_utils::BearDogCrypto;
/// let bytes = BearDogCrypto::generate_secure_random(32);
/// ```
#[deprecated(
    since = "3.0.2",
    note = "Use beardog_security::crypto_utils::BearDogCrypto::generate_secure_random instead"
)]
pub fn secure_random_bytes(size: usize) -> Vec<u8> {
    // 100% Pure Rust - using rand crate with OsRng
    let mut rng = rand::rngs::OsRng;
    let mut bytes = vec![0u8; size];
    rng.fill_bytes(&mut bytes);
    bytes
}

/// Generate cryptographic salt - DEPRECATED
///
/// ⚠️ **DEPRECATED**: Use `beardog_security::crypto_utils::BearDogCrypto::generate_secure_random(32)` instead.
#[deprecated(
    since = "3.0.2",
    note = "Use beardog_security::crypto_utils::BearDogCrypto::generate_secure_random(32) instead"
)]
pub fn generate_salt() -> Vec<u8> {
    secure_random_bytes(32) // 256-bit salt
}

/// Generate nonce - DEPRECATED
///
/// ⚠️ **DEPRECATED**: Use `beardog_security::crypto_utils::BearDogCrypto::generate_secure_nonce` instead.
#[deprecated(
    since = "3.0.2",
    note = "Use beardog_security::crypto_utils::BearDogCrypto::generate_secure_nonce instead"
)]
pub fn generate_nonce(size: usize) -> Vec<u8> {
    secure_random_bytes(size)
}

/// DEPRECATED: Use beardog_security::crypto_utils::BearDogCrypto::sha256_hash instead
#[deprecated(
    since = "3.0.1",
    note = "Use beardog_security::crypto_utils::BearDogCrypto::sha256_hash instead. \
            Crypto functions should be in beardog-security crate. Removal planned for v3.3.0 (Q1 2026)."
)]
pub fn sha256_hash(data: &[u8]) -> String {
    let mut hasher = Sha256::new();
    hasher.update(data);
    bytes_to_hex(&hasher.finalize())
}

/// HMAC-SHA256 - DEPRECATED
///
/// ⚠️ **DEPRECATED**: HMAC functionality should be migrated to beardog-security.
/// Consider using `beardog_security::crypto_utils` for cryptographic operations.
#[deprecated(
    since = "3.0.2",
    note = "HMAC operations should be in beardog-security. Will be removed in v3.3.0"
)]
pub fn hmac_sha256(key: &[u8], data: &[u8]) -> Result<String, BearDogError> {
    let mut mac = HmacSha256::new_from_slice(key).map_err(|e| BearDogError::Crypto {
        message: format!("Invalid HMAC key: {e}"),
    })?;
    mac.update(data);
    Ok(bytes_to_hex(&mac.finalize().into_bytes()))
}

/// Verify HMAC-SHA256 - DEPRECATED
///
/// ⚠️ **DEPRECATED**: HMAC functionality should be migrated to beardog-security.
#[deprecated(
    since = "3.0.2",
    note = "HMAC operations should be in beardog-security. Will be removed in v3.3.0"
)]
pub fn verify_hmac_sha256(key: &[u8], data: &[u8], signature: &str) -> Result<bool, BearDogError> {
    let computed = hmac_sha256(key, data)?;
    Ok(constant_time_compare(
        computed.as_bytes(),
        signature.as_bytes(),
    ))
}

/// Constant time comparison - DEPRECATED
///
/// ⚠️ **DEPRECATED**: Use constant-time comparison from security crate or dedicated library.
#[deprecated(
    since = "3.0.2",
    note = "Use constant-time comparison from security crate. Will be removed in v3.3.0"
)]
pub fn constant_time_compare(a: &[u8], b: &[u8]) -> bool {
    if a.len() != b.len() {
        return false;
    }
    let mut diff = 0u8;
    for (byte_a, byte_b) in a.iter().zip(b.iter()) {
        diff |= byte_a ^ byte_b;
    }
    diff == 0
}

/// Generate password - DEPRECATED
///
/// ⚠️ **DEPRECATED**: Password generation should be in beardog-security.
#[deprecated(
    since = "3.0.2",
    note = "Password generation should be in beardog-security. Will be removed in v3.3.0"
)]
pub fn generate_password(length: usize) -> String {
    const CHARSET: &[u8] =
        b"ABCDEFGHIJKLMNOPQRSTUVWXYZabcdefghijklmnopqrstuvwxyz0123456789!@#$%^&*()_+-=[]{}|;:,.<>?";
    let mut password = String::with_capacity(length);
    let random_bytes = secure_random_bytes(length);
    for byte in random_bytes {
        let idx = (byte as usize) % CHARSET.len();
        password.push(CHARSET[idx] as char);
    }
    password
}

/// Generate API key - DEPRECATED
///
/// ⚠️ **DEPRECATED**: API key generation should be in beardog-security.
#[deprecated(
    since = "3.0.2",
    note = "API key generation should be in beardog-security. Will be removed in v3.3.0"
)]
pub fn generate_api_key() -> String {
    const CHARSET: &[u8] = b"ABCDEFGHIJKLMNOPQRSTUVWXYZabcdefghijklmnopqrstuvwxyz0123456789";
    let mut key = String::with_capacity(64);
    let random_bytes = secure_random_bytes(64);
    for byte in random_bytes {
        let idx = (byte as usize) % CHARSET.len();
        key.push(CHARSET[idx] as char);
    }
    key
}

/// Zero memory - DEPRECATED
///
/// ⚠️ **DEPRECATED**: Secure memory zeroing should be in beardog-security.
#[deprecated(
    since = "3.0.2",
    note = "Secure memory operations should be in beardog-security. Will be removed in v3.3.0"
)]
pub fn zero_memory(data: &mut [u8]) {

    for byte in data.iter_mut() {
        *byte = 0;
    }
}

/// PBKDF2-HMAC-SHA256 - DEPRECATED
///
/// ⚠️ **DEPRECATED**: Use `beardog_security::crypto_utils::BearDogCrypto::derive_pbkdf2_key` instead.
///
/// ## Migration
/// ```rust
/// // Old
/// use beardog_utils::utils::crypto_utils::pbkdf2_hmac_sha256;
/// let key = pbkdf2_hmac_sha256(password, salt, iterations, output_len)?;
///
/// // New
/// use beardog_security::crypto_utils::BearDogCrypto;
/// let key = BearDogCrypto::derive_pbkdf2_key(password, salt, iterations, output_len)?;
/// ```
#[deprecated(
    since = "3.0.2",
    note = "Use beardog_security::crypto_utils::BearDogCrypto::derive_pbkdf2_key instead"
)]
pub fn pbkdf2_hmac_sha256(
    password: &[u8],
    salt: &[u8],
    iterations: u32,
    output_len: usize,
) -> Result<Vec<u8>, BearDogError> {
    // 100% Pure Rust - using pbkdf2 crate
    if iterations == 0 {
        return Err(BearDogError::Crypto {
            message: "PBKDF2 iterations must be non-zero".to_string(),
        });
    }
    let mut output = vec![0u8; output_len];
    pbkdf2::pbkdf2_hmac::<Sha256>(password, salt, iterations, &mut output);
    Ok(output)
}

/// Convert bytes to hex string
///
/// Note: This is a utility function and may remain as it's not strictly cryptographic.
pub fn bytes_to_hex(bytes: &[u8]) -> String {
    bytes.iter().map(|b| format!("{b:02x}")).collect()
}

/// Convert hex string to bytes
///
/// Note: This is a utility function and may remain as it's not strictly cryptographic.
pub fn hex_to_bytes(hex: &str) -> Result<Vec<u8>, BearDogError> {
    if hex.len() % 2 != 0 {
        return Err(BearDogError::validation("Hex string must have even length"));
    }
    let mut bytes = Vec::with_capacity(hex.len() / 2);
    for chunk in hex.as_bytes().chunks(2) {
        let hex_byte = std::str::from_utf8(chunk)
            .map_err(|_| BearDogError::validation("Invalid hex character"))?;
        let byte = u8::from_str_radix(hex_byte, 16)
            .map_err(|_| BearDogError::validation("Invalid hex digit"))?;
        bytes.push(byte);
    }
    Ok(bytes)
}

#[allow(unused_imports, clippy::nonminimal_bool, dead_code)]
#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    #[allow(deprecated)]
    fn test_sha256_hash() {
        let data = b"hello world";
        let hash = sha256_hash(data);

        // TEST_CATEGORY: unit
        // TEST_DOMAIN: core
        // TEST_PRIORITY: normal
        assert_eq!(hash.len(), 64); // 32 bytes = 64 hex chars
        assert_eq!(
            hash,
            "b94d27b9934d3e08a52e52d7da7dabfac484efe37a5380ee9088f7ace2efcde9"
        );
    }
    #[test]
    #[allow(deprecated)]
    fn test_hmac_sha256() {
        // TEST_CATEGORY: unit
        // TEST_DOMAIN: core
        // TEST_PRIORITY: normal
        let key = b"secret_key";
        let data = b"hello";
        let hmac = hmac_sha256(key, data).map_err(|e| {
    tracing::error!("Operation failed ({}): {:?}", "HMAC should succeed with valid key", e);
    beardog_errors::BearDogError::internal(format_args!("Operation failed ({}): {:?}", "HMAC should succeed with valid key", e).to_string())
})?;

        assert_eq!(hmac.len(), 64); // 32 bytes = 64 hex chars
            hmac,
            "cf1a418afaafc798df48fd804a2abf6970283afd8c40b41f818ad9b6ca4f8ca8"
    }
    // TEST_CATEGORY: unit
    // TEST_DOMAIN: core
    // TEST_PRIORITY: normal
    #[test]
    #[allow(deprecated)]
    fn test_secure_random_bytes() {
        let bytes1 = secure_random_bytes(32);
        let bytes2 = secure_random_bytes(32);

        assert_eq!(bytes1.len(), 32);
        assert_eq!(bytes2.len(), 32);
        assert_ne!(bytes1, bytes2);}


    fn test_hex_conversion() {
        let data = vec![0x01, 0x23, 0x45, 0x67, 0x89, 0xab, 0xcd, 0xef];
        let hex = bytes_to_hex(&data);
        let back = hex_to_bytes(&hex).map_err(|e| {
    tracing::error!("Operation failed: {:?}", e);
    beardog_errors::BearDogError::internal(format_args!("Operation failed: {:?}", e).to_string())
        assert_eq!(hex, "0123456789abcdef");
        assert_eq!(data, back);
    fn test_hex_conversion_invalid() {

        assert!(hex_to_bytes("invalid_hex").is_err());
        assert!(hex_to_bytes("0g").is_err()); // Invalid hex character
        assert!(hex_to_bytes("123").is_err()); // Odd length}


    fn test_constant_time_compare() {
        let a = b"hello";
        let b = b"hello";
        let c = b"world";
        assert!(constant_time_compare(a, b));
        assert!(!constant_time_compare(a, c));
        assert!(!constant_time_compare(a, b"hell")); // Different lengths
    fn test_generate_password() {
        let password1 = generate_password(16);
        let password2 = generate_password(16);
        assert_eq!(password1.len(), 16);
        assert_eq!(password2.len(), 16);
        assert_ne!(password1, password2);

        let short = generate_password(8);
        let long = generate_password(32);
        assert_eq!(short.len(), 8);
        assert_eq!(long.len(), 32);}


    fn test_password_character_set() {
        let password = generate_password(100);

        let has_upper = password.chars().any(|c| c.is_ascii_uppercase());
        let has_lower = password.chars().any(|c| c.is_ascii_lowercase());
        let has_digit = password.chars().any(|c| c.is_ascii_digit());
        let has_special = password
            .chars()
            .any(|c| "!@#$%^&*()_+-=[]{}|;:,.<>?".contains(c));
        assert!(has_upper, "Password should contain uppercase letters");
        assert!(has_lower, "Password should contain lowercase letters");
        assert!(has_digit, "Password should contain digits");
        assert!(has_special, "Password should contain special characters");
    fn test_empty_data_handling() {
        let empty_data = b"";
        let hash = sha256_hash(empty_data);
        let hmac = hmac_sha256(b"key", empty_data).map_err(|e| {
    tracing::error!("Operation failed ({}): {:?}", "HMAC should succeed with empty data", e);
    beardog_errors::BearDogError::internal(format_args!("Operation failed ({}): {:?}", "HMAC should succeed with empty data", e).to_string())

        assert_eq!(hash.len(), 64);
        assert_eq!(hmac.len(), 64);
    fn test_large_data_handling() {
        let large_data = vec![0x42; 1024 * 1024]; // 1MB of data
        let hash = sha256_hash(&large_data);
        let hmac = hmac_sha256(b"key", &large_data).map_err(|e| {
    tracing::error!("Operation failed ({}): {:?}", "HMAC should succeed with large data", e);
    beardog_errors::BearDogError::internal(format_args!("Operation failed ({}): {:?}", "HMAC should succeed with large data", e).to_string())

