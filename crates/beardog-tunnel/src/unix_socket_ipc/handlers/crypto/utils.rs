//! Utility functions for crypto handlers
//!
//! This module contains shared helper functions used across multiple
//! crypto handler domains.

use beardog_core::crypto_service::algorithms::hashing;

/// Derive a 32-byte key from a key ID and purpose
///
/// Uses BLAKE3 key derivation for deterministic key generation.
/// This is used by various crypto handlers to derive signing keys,
/// encryption keys, and other cryptographic material from a key identifier.
///
/// # Parameters
///
/// - `key_id`: The key identifier (e.g., "default", "prod", "test")
/// - `purpose`: The purpose of the key (e.g., "signature", "encryption", "tls_handshake")
///
/// # Returns
///
/// A 32-byte key derived deterministically from the inputs.
///
/// # Security Note
///
/// The master key is derived from the `BEARDOG_MASTER_KEY` environment variable.
/// If not set, a default value is used (suitable for development only).
/// In production, always set a strong, random `BEARDOG_MASTER_KEY`.
///
/// # Example
///
/// ```rust,ignore
/// let key = derive_key_from_id("prod", "signature")?;
/// // Use key for signing operations
/// ```
pub(super) fn derive_key_from_id(key_id: &str, purpose: &str) -> Result<[u8; 32], String> {
    // Get master key from environment or generate deterministic key
    let master_key = std::env::var("BEARDOG_MASTER_KEY")
        .unwrap_or_else(|_| "beardog_default_master_key_v1".to_string());

    // Derive key using BLAKE3 KDF
    let context = format!("beardog_crypto_v1:{}:{}", key_id, purpose);
    let derived = hashing::derive_key_blake3(&context, master_key.as_bytes());

    let key: [u8; 32] = derived
        .as_slice()
        .try_into()
        .map_err(|_| "Key derivation failed".to_string())?;

    Ok(key)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_derive_key_from_id() {
        let key = derive_key_from_id("test_key", "test").unwrap();
        assert_eq!(key.len(), 32);

        // Same inputs should produce same key (deterministic)
        let key2 = derive_key_from_id("test_key", "test").unwrap();
        assert_eq!(key, key2);

        // Different inputs should produce different keys
        let key3 = derive_key_from_id("test_key", "different").unwrap();
        assert_ne!(key, key3);
    }
}
