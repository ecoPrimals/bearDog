// SPDX-License-Identifier: AGPL-3.0-only

//! Hashing and MAC operations
//!
//! This module provides cryptographic hashing and message authentication code
//! (MAC) operations for ecoPrimals.
//!
//! # Overview
//!
//! Hashing operations:
//! - **Integrity**: Verify data hasn't been corrupted
//! - **Content-Addressed Storage**: Use hash as identifier
//! - **Key Derivation**: Derive keys from passwords/secrets
//! - **Digital Signatures**: Hash messages before signing
//!
//! Message Authentication Codes (MACs):
//! - **Authentication**: Verify message sender
//! - **Integrity**: Detect tampering
//! - **Key-Based**: Requires shared secret key
//!
//! # Algorithms
//!
//! ## Blake3
//!
//! - **Type**: Cryptographic hash function
//! - **Output Size**: 32 bytes (256-bit, default)
//! - **Speed**: Extremely fast (~3 GB/s single-core, highly parallelizable)
//! - **Security**: 128-bit collision resistance
//! - **Use Case**: Primary hash for BearDog (faster than SHA-256)
//!
//! ### Methods
//!
//! - [`handle_blake3_hash`] - Compute Blake3 hash
//!
//! ## HMAC-SHA256
//!
//! - **Type**: Hash-based Message Authentication Code
//! - **Hash**: SHA-256
//! - **Key Size**: Variable (recommend 32 bytes)
//! - **Output Size**: 32 bytes (256-bit)
//! - **Security**: Keyed MAC for authentication
//! - **Use Case**: TLS Finished messages, API authentication
//!
//! ### Methods
//!
//! - [`handle_hmac_sha256`] - Compute HMAC-SHA256
//!
//! # Usage
//!
//! All handlers are re-exported from the parent `crypto` module:
//!
//! ```rust,ignore
//! // NOTE: These handlers are internal and called via JSON-RPC
//! use crate::unix_socket_ipc::handlers::crypto::*;
//!
//! // Compute Blake3 hash
//! let hash = handle_blake3_hash(params).await?;
//!
//! // Compute HMAC-SHA256
//! let mac = handle_hmac_sha256(params).await?;
//! ```
//!
//! # Security Notes
//!
//! - **Don't Hash Passwords**: Use Argon2id, bcrypt, or scrypt instead
//! - **Use MACs for Authentication**: Plain hashes don't authenticate
//! - **Key Management**: Protect HMAC keys like encryption keys
//!
//! # References
//!
//! - Blake3: <https://github.com/BLAKE3-team/BLAKE3-specs/blob/master/blake3.pdf>
//! - RFC 2104 (HMAC): <https://www.rfc-editor.org/rfc/rfc2104.html>
//! - FIPS 180-4 (SHA-256): <https://csrc.nist.gov/publications/detail/fips/180/4/final>

use base64::Engine;
use serde_json::Value;
use sha2::{Digest, Sha256, Sha384};
use tracing::{debug, info};

/// Handle crypto.hash_for_cipher method
///
/// Cipher-aware hashing for TLS 1.3 - selects hash algorithm based on cipher suite.
///
/// # TLS 1.3 Cipher Suites (RFC 8446)
///
/// - **0x1301** (TLS_AES_128_GCM_SHA256): Uses SHA-256 (32 bytes)
/// - **0x1302** (TLS_AES_256_GCM_SHA384): Uses SHA-384 (48 bytes)
/// - **0x1303** (TLS_CHACHA20_POLY1305_SHA256): Uses SHA-256 (32 bytes)
///
/// # Parameters
///
/// - `data` (base64): Data to hash
/// - `cipher_suite` (number): TLS 1.3 cipher suite ID
///
/// # Returns
///
/// - `hash` (base64): Hash output (32 or 48 bytes depending on cipher)
/// - `algorithm` (string): Hash algorithm used ("SHA-256" or "SHA-384")
/// - `cipher_suite` (number): Echo of input cipher suite
///
/// # Example
///
/// ```json
/// {
///   "method": "crypto.hash_for_cipher",
///   "params": {
///     "data": "base64_encoded_data",
///     "cipher_suite": 4866
///   }
/// }
/// ```
///
/// # TRUE PRIMAL Pattern
///
/// This method enables Songbird to hash data without knowing which algorithm to use.
/// BearDog owns the crypto decisions, Songbird just passes cipher_suite.
pub async fn handle_hash_for_cipher(params: Option<&Value>) -> Result<Value, String> {
    let params = params.ok_or("Missing params for crypto.hash_for_cipher")?;

    // Extract parameters
    let data_b64 = params
        .get("data")
        .and_then(|v| v.as_str())
        .ok_or("Missing required parameter: data")?;

    let cipher_suite = params
        .get("cipher_suite")
        .and_then(|v| v.as_u64())
        .ok_or("Missing required parameter: cipher_suite")? as u16;

    // Decode data
    let data = base64::engine::general_purpose::STANDARD
        .decode(data_b64)
        .map_err(|e| format!("Invalid base64 data: {e}"))?;

    debug!(
        "🔨 Hashing {} bytes for cipher suite 0x{:04x}",
        data.len(),
        cipher_suite
    );

    // Select hash algorithm based on cipher suite (RFC 8446)
    let (hash, algorithm) = match cipher_suite {
        0x1301 => {
            // TLS_AES_128_GCM_SHA256
            info!("  → Using SHA-256 for cipher suite 0x1301 (TLS_AES_128_GCM_SHA256)");
            (Sha256::digest(&data).to_vec(), "SHA-256")
        }
        0x1302 => {
            // TLS_AES_256_GCM_SHA384
            info!("  → Using SHA-384 for cipher suite 0x1302 (TLS_AES_256_GCM_SHA384)");
            (Sha384::digest(&data).to_vec(), "SHA-384")
        }
        0x1303 => {
            // TLS_CHACHA20_POLY1305_SHA256
            info!("  → Using SHA-256 for cipher suite 0x1303 (TLS_CHACHA20_POLY1305_SHA256)");
            (Sha256::digest(&data).to_vec(), "SHA-256")
        }
        _ => {
            return Err(format!(
                "Unsupported TLS 1.3 cipher suite: 0x{:04x}. Supported: 0x1301 (AES-128-GCM), 0x1302 (AES-256-GCM), 0x1303 (ChaCha20-Poly1305)",
                cipher_suite
            ));
        }
    };

    // Encode hash
    let hash_b64 = base64::engine::general_purpose::STANDARD.encode(&hash);

    info!(
        "✅ Hash computed: {} bytes using {} (cipher 0x{:04x})",
        hash.len(),
        algorithm,
        cipher_suite
    );

    Ok(serde_json::json!({
        "hash": hash_b64,
        "algorithm": algorithm,
        "hash_length": hash.len(),
        "cipher_suite": cipher_suite
    }))
}
/// Handle BLAKE3 hash operations via JSON-RPC

pub async fn handle_blake3_hash(params: Option<&Value>) -> Result<Value, String> {
    let params = params.ok_or("Missing params for crypto.blake3_hash")?;

    // Extract parameters
    let data_b64 = params
        .get("data")
        .and_then(|v| v.as_str())
        .ok_or("Missing required parameter: data")?;

    // Decode data
    let data = base64::engine::general_purpose::STANDARD
        .decode(data_b64)
        .map_err(|e| format!("Invalid base64 data: {e}"))?;

    debug!("🔨 Hashing {} bytes with BLAKE3", data.len());

    // Use BearDog's crypto service
    use beardog_core::crypto_service::algorithms::hashing;

    let hash = hashing::hash_blake3(&data);

    // Encode hash
    let hash_b64 = base64::engine::general_purpose::STANDARD.encode(&hash);

    info!("✅ BLAKE3 hash computed ({} bytes)", hash.len());

    Ok(serde_json::json!({
        "hash": hash_b64,
        "algorithm": "BLAKE3",
    }))
}

/// Handle crypto.hmac_sha256 method
///
/// Computes HMAC-SHA256 authentication tag.
///
/// # Parameters
///
/// - `key`: Base64-encoded secret key
/// - `data`: Base64-encoded data to authenticate
///
/// # Returns
///
/// - `mac`: Base64-encoded HMAC-SHA256 tag (32 bytes)
pub async fn handle_hmac_sha256(params: Option<&Value>) -> Result<Value, String> {
    let params = params.ok_or("Missing params for crypto.hmac_sha256")?;

    // Extract parameters
    let key_b64 = params
        .get("key")
        .and_then(|v| v.as_str())
        .ok_or("Missing required parameter: key")?;

    let data_b64 = params
        .get("data")
        .and_then(|v| v.as_str())
        .ok_or("Missing required parameter: data")?;

    // Decode parameters
    let key = base64::engine::general_purpose::STANDARD
        .decode(key_b64)
        .map_err(|e| format!("Invalid base64 key: {e}"))?;

    let data = base64::engine::general_purpose::STANDARD
        .decode(data_b64)
        .map_err(|e| format!("Invalid base64 data: {e}"))?;

    debug!("🔐 Computing HMAC-SHA256 for {} bytes", data.len());

    // Use BearDog's crypto service
    use beardog_core::crypto_service::algorithms::hashing;

    let mac = hashing::hmac_sha256(&key, &data)
        .map_err(|e| format!("HMAC-SHA256 computation failed: {e}"))?;

    // Encode MAC
    let mac_b64 = base64::engine::general_purpose::STANDARD.encode(&mac);

    info!("✅ HMAC-SHA256 computed ({} bytes)", mac.len());

    Ok(serde_json::json!({
        "mac": mac_b64,
        "algorithm": "HMAC-SHA256",
    }))
}

#[cfg(test)]
mod tests {
    use super::*;
    use base64::engine::general_purpose::STANDARD as BASE64;
    use serde_json::json;

    // ========================================================================
    // BLAKE3 HASH TESTS
    // ========================================================================

    #[tokio::test]
    async fn test_blake3_hash_basic() {
        let data = BASE64.encode(b"Hello, BearDog!");
        let params = json!({ "data": data });

        let result = handle_blake3_hash(Some(&params)).await;
        assert!(result.is_ok());

        let value = result.unwrap();
        assert_eq!(value["algorithm"], "BLAKE3");
        assert!(value["hash"].is_string());

        // Decode and verify hash length (32 bytes)
        let hash = BASE64.decode(value["hash"].as_str().unwrap()).unwrap();
        assert_eq!(hash.len(), 32);
    }

    #[tokio::test]
    async fn test_blake3_hash_empty_data() {
        let data = BASE64.encode(b"");
        let params = json!({ "data": data });

        let result = handle_blake3_hash(Some(&params)).await;
        assert!(result.is_ok());

        let value = result.unwrap();
        let hash = BASE64.decode(value["hash"].as_str().unwrap()).unwrap();
        assert_eq!(hash.len(), 32);
    }

    #[tokio::test]
    async fn test_blake3_hash_deterministic() {
        let data = BASE64.encode(b"Test data for BLAKE3");
        let params = json!({ "data": data });

        let result1 = handle_blake3_hash(Some(&params)).await.unwrap();
        let result2 = handle_blake3_hash(Some(&params)).await.unwrap();

        assert_eq!(result1["hash"], result2["hash"]);
    }

    #[tokio::test]
    async fn test_blake3_hash_missing_params() {
        let result = handle_blake3_hash(None).await;
        assert!(result.is_err());
        assert!(result.unwrap_err().contains("Missing params"));
    }

    #[tokio::test]
    async fn test_blake3_hash_missing_data() {
        let params = json!({});
        let result = handle_blake3_hash(Some(&params)).await;
        assert!(result.is_err());
        assert!(result.unwrap_err().contains("data"));
    }

    #[tokio::test]
    async fn test_blake3_hash_invalid_base64() {
        let params = json!({ "data": "!!!not-valid-base64!!!" });
        let result = handle_blake3_hash(Some(&params)).await;
        assert!(result.is_err());
        assert!(result.unwrap_err().contains("Invalid base64"));
    }

    // ========================================================================
    // HMAC-SHA256 TESTS
    // ========================================================================

    #[tokio::test]
    async fn test_hmac_sha256_basic() {
        let key = BASE64.encode(b"secret-key-32-bytes-long-0000000");
        let data = BASE64.encode(b"Message to authenticate");
        let params = json!({ "key": key, "data": data });

        let result = handle_hmac_sha256(Some(&params)).await;
        assert!(result.is_ok());

        let value = result.unwrap();
        assert_eq!(value["algorithm"], "HMAC-SHA256");
        assert!(value["mac"].is_string());

        // Decode and verify MAC length (32 bytes)
        let mac = BASE64.decode(value["mac"].as_str().unwrap()).unwrap();
        assert_eq!(mac.len(), 32);
    }

    #[tokio::test]
    async fn test_hmac_sha256_deterministic() {
        let key = BASE64.encode(b"test-key");
        let data = BASE64.encode(b"test-data");
        let params = json!({ "key": key, "data": data });

        let result1 = handle_hmac_sha256(Some(&params)).await.unwrap();
        let result2 = handle_hmac_sha256(Some(&params)).await.unwrap();

        assert_eq!(result1["mac"], result2["mac"]);
    }

    #[tokio::test]
    async fn test_hmac_sha256_different_keys_different_macs() {
        let data = BASE64.encode(b"same data");

        let params1 = json!({ "key": BASE64.encode(b"key1"), "data": data });
        let params2 = json!({ "key": BASE64.encode(b"key2"), "data": data });

        let result1 = handle_hmac_sha256(Some(&params1)).await.unwrap();
        let result2 = handle_hmac_sha256(Some(&params2)).await.unwrap();

        assert_ne!(result1["mac"], result2["mac"]);
    }

    #[tokio::test]
    async fn test_hmac_sha256_missing_key() {
        let data = BASE64.encode(b"data");
        let params = json!({ "data": data });

        let result = handle_hmac_sha256(Some(&params)).await;
        assert!(result.is_err());
        assert!(result.unwrap_err().contains("key"));
    }

    #[tokio::test]
    async fn test_hmac_sha256_missing_data() {
        let key = BASE64.encode(b"key");
        let params = json!({ "key": key });

        let result = handle_hmac_sha256(Some(&params)).await;
        assert!(result.is_err());
        assert!(result.unwrap_err().contains("data"));
    }

    // ========================================================================
    // HASH FOR CIPHER TESTS (TLS 1.3)
    // ========================================================================

    #[tokio::test]
    async fn test_hash_for_cipher_sha256() {
        let data = BASE64.encode(b"TLS 1.3 data");
        let params = json!({ "data": data, "cipher_suite": 0x1301 });

        let result = handle_hash_for_cipher(Some(&params)).await;
        assert!(result.is_ok());

        let value = result.unwrap();
        assert_eq!(value["algorithm"], "SHA-256");
        assert_eq!(value["cipher_suite"], 0x1301);
        assert_eq!(value["hash_length"], 32);
    }

    #[tokio::test]
    async fn test_hash_for_cipher_sha384() {
        let data = BASE64.encode(b"TLS 1.3 data");
        let params = json!({ "data": data, "cipher_suite": 0x1302 });

        let result = handle_hash_for_cipher(Some(&params)).await;
        assert!(result.is_ok());

        let value = result.unwrap();
        assert_eq!(value["algorithm"], "SHA-384");
        assert_eq!(value["cipher_suite"], 0x1302);
        assert_eq!(value["hash_length"], 48);
    }

    #[tokio::test]
    async fn test_hash_for_cipher_chacha20() {
        let data = BASE64.encode(b"TLS 1.3 data");
        let params = json!({ "data": data, "cipher_suite": 0x1303 });

        let result = handle_hash_for_cipher(Some(&params)).await;
        assert!(result.is_ok());

        let value = result.unwrap();
        assert_eq!(value["algorithm"], "SHA-256");
        assert_eq!(value["cipher_suite"], 0x1303);
        assert_eq!(value["hash_length"], 32);
    }

    #[tokio::test]
    async fn test_hash_for_cipher_unsupported() {
        let data = BASE64.encode(b"data");
        let params = json!({ "data": data, "cipher_suite": 0x9999 });

        let result = handle_hash_for_cipher(Some(&params)).await;
        assert!(result.is_err());
        assert!(result.unwrap_err().contains("Unsupported"));
    }

    #[tokio::test]
    async fn test_hash_for_cipher_missing_cipher_suite() {
        let data = BASE64.encode(b"data");
        let params = json!({ "data": data });

        let result = handle_hash_for_cipher(Some(&params)).await;
        assert!(result.is_err());
        assert!(result.unwrap_err().contains("cipher_suite"));
    }
}
