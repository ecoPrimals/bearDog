// SPDX-License-Identifier: AGPL-3.0-or-later

//! Hashing and MAC operations
//!
//! This module provides cryptographic hashing and message authentication code
//! (MAC) operations for ecoPrimals.
//!
//! # Encoding Contract (LD-01)
//!
//! All hashing methods expect the `data` parameter as a **standard Base64**
//! string (RFC 4648 §4, `+/=` alphabet). `BearDog` decodes the Base64 to raw
//! bytes, hashes those bytes, and returns the digest as Base64. Callers that
//! pass raw UTF-8 or hex-encoded data directly will get incorrect hashes.
//!
//! ```json
//! { "method": "crypto.hash", "params": { "data": "<base64-encoded bytes>" } }
//! → { "hash": "<base64-encoded BLAKE3 digest>", "algorithm": "BLAKE3" }
//! ```
//!
//! For Ed25519 sign/verify, per-field encoding hints (`message_encoding`,
//! `signature_encoding`, `public_key_encoding`) override this default — see
//! `ATTESTATION_ENCODING_STANDARD.md` v2.0 (BD-01, resolved in Wave 33).
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
//! - **Use Case**: Primary hash for `BearDog` (faster than SHA-256)
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
use beardog_core::crypto_service::algorithms::hashing;
use serde_json::Value;
use sha2::{Digest, Sha256, Sha384};
use tracing::{debug, info};

/// # Errors
///
/// Returns an error if hashing fails.
/// Handle `crypto.hash_for_cipher` method
///
/// Cipher-aware hashing for TLS 1.3 - selects hash algorithm based on cipher suite.
///
/// # TLS 1.3 Cipher Suites (RFC 8446)
///
/// - **0x1301** (`TLS_AES_128_GCM_SHA256)`: Uses SHA-256 (32 bytes)
/// - **0x1302** (`TLS_AES_256_GCM_SHA384)`: Uses SHA-384 (48 bytes)
/// - **0x1303** (`TLS_CHACHA20_POLY1305_SHA256)`: Uses SHA-256 (32 bytes)
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
/// This method enables callers to hash data without knowing which algorithm to use.
/// `BearDog` owns the crypto decisions; callers pass `cipher_suite`.
pub async fn handle_hash_for_cipher(params: Option<&Value>) -> Result<Value, String> {
    let params = params.ok_or("Missing params for crypto.hash_for_cipher")?;

    // Extract parameters
    let data_b64 = params
        .get("data")
        .and_then(|v| v.as_str())
        .ok_or("Missing required parameter: data")?;

    #[expect(
        clippy::cast_possible_truncation,
        reason = "IANA TLS cipher suite identifier"
    )]
    let cipher_suite = params
        .get("cipher_suite")
        .and_then(serde_json::Value::as_u64)
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
                "Unsupported TLS 1.3 cipher suite: 0x{cipher_suite:04x}. Supported: 0x1301 (AES-128-GCM), 0x1302 (AES-256-GCM), 0x1303 (ChaCha20-Poly1305)"
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
/// Handle BLAKE3 hash operations via JSON-RPC.
///
/// # Wire Contract (LD-01)
///
/// The `data` param **must** be standard Base64 (RFC 4648 §4). `BearDog`
/// decodes to raw bytes, hashes with BLAKE3, and returns the digest as
/// Base64. Passing raw UTF-8 or hex directly yields incorrect hashes.
///
/// # Errors
///
/// Returns an error if `data` is missing or not valid Base64.
pub async fn handle_blake3_hash(params: Option<&Value>) -> Result<Value, String> {
    let params = params.ok_or(
        "Missing params for crypto.blake3_hash — expected: {\"data\": \"<standard-base64>\"}",
    )?;

    let data_b64 = params
        .get("data")
        .and_then(|v| v.as_str())
        .ok_or("Missing required parameter: data (standard base64 encoded string)")?;

    let data = base64::engine::general_purpose::STANDARD
        .decode(data_b64)
        .map_err(|e| {
            format!(
                "Invalid base64 in 'data' field: {e} — use standard base64 (RFC 4648, +/= alphabet)"
            )
        })?;

    debug!("🔨 Hashing {} bytes with BLAKE3", data.len());

    let hash = hashing::hash_blake3(&data);

    // Encode hash
    let hash_b64 = base64::engine::general_purpose::STANDARD.encode(&hash);

    info!("✅ BLAKE3 hash computed ({} bytes)", hash.len());

    Ok(serde_json::json!({
        "hash": hash_b64,
        "algorithm": "BLAKE3",
    }))
}

/// # Errors
///
/// Returns an error if hashing fails.
/// Handle `crypto.hmac_sha256` method
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

/// Handle `crypto.hmac_verify` — verify an HMAC tag using constant-time comparison.
///
/// Params: `key` (base64), `data` (base64), `mac` (base64 expected MAC).
///
/// Returns `{valid: bool, algorithm: "HMAC-SHA256"}`.
///
/// # Errors
///
/// Returns an error if parameters are missing or base64 decoding fails.
pub async fn handle_hmac_verify(params: Option<&Value>) -> Result<Value, String> {
    let params = params.ok_or("Missing params for crypto.hmac_verify")?;

    let key_b64 = params
        .get("key")
        .and_then(|v| v.as_str())
        .ok_or("Missing required parameter: key")?;

    let data_b64 = params
        .get("data")
        .and_then(|v| v.as_str())
        .ok_or("Missing required parameter: data")?;

    let expected_b64 = params
        .get("mac")
        .and_then(|v| v.as_str())
        .ok_or("Missing required parameter: mac")?;

    let key = base64::engine::general_purpose::STANDARD
        .decode(key_b64)
        .map_err(|e| format!("Invalid base64 key: {e}"))?;

    let data = base64::engine::general_purpose::STANDARD
        .decode(data_b64)
        .map_err(|e| format!("Invalid base64 data: {e}"))?;

    let expected = base64::engine::general_purpose::STANDARD
        .decode(expected_b64)
        .map_err(|e| format!("Invalid base64 mac: {e}"))?;

    let computed = hashing::hmac_sha256(&key, &data)
        .map_err(|e| format!("HMAC-SHA256 computation failed: {e}"))?;

    let valid: bool =
        subtle::ConstantTimeEq::ct_eq(computed.as_slice(), expected.as_slice()).into();

    info!(valid, "HMAC-SHA256 verification");

    Ok(serde_json::json!({
        "valid": valid,
        "algorithm": "HMAC-SHA256",
    }))
}

/// Handle `crypto.hkdf_sha256` — HKDF-SHA256 extract-and-expand.
///
/// Params:
/// - `ikm` (base64, required): input key material
/// - `salt` (base64, optional): salt for extract step
/// - `info` (base64 or UTF-8 string, optional): context/info for expand step
/// - `length` (integer, optional): output key length in bytes (default 32, max 255*32)
///
/// Returns `{okm: "<base64>", algorithm: "HKDF-SHA256", length: N}`.
///
/// # Errors
///
/// Returns an error if parameters are missing or HKDF expand fails.
pub async fn handle_hkdf_sha256(params: Option<&Value>) -> Result<Value, String> {
    use hkdf::Hkdf;
    use sha2::Sha256;

    let params = params.ok_or("Missing params for crypto.hkdf_sha256")?;

    let ikm_b64 = params
        .get("ikm")
        .and_then(|v| v.as_str())
        .ok_or("Missing required parameter: ikm")?;

    let ikm = base64::engine::general_purpose::STANDARD
        .decode(ikm_b64)
        .map_err(|e| format!("Invalid base64 ikm: {e}"))?;

    let salt = params
        .get("salt")
        .and_then(|v| v.as_str())
        .map(|s| {
            base64::engine::general_purpose::STANDARD
                .decode(s)
                .map_err(|e| format!("Invalid base64 salt: {e}"))
        })
        .transpose()?;

    let info = params
        .get("info")
        .map(|v| {
            if let Some(s) = v.as_str() {
                base64::engine::general_purpose::STANDARD
                    .decode(s)
                    .unwrap_or_else(|_| s.as_bytes().to_vec())
            } else {
                Vec::new()
            }
        })
        .unwrap_or_default();

    #[allow(
        clippy::cast_possible_truncation,
        reason = "HKDF length capped at MAX_HKDF_LENGTH (8160), fits in usize on all targets"
    )]
    let length = params
        .get("length")
        .and_then(serde_json::Value::as_u64)
        .unwrap_or(32)
        .min(255 * 32) as usize;

    let hkdf = Hkdf::<Sha256>::new(salt.as_deref(), &ikm);
    let mut okm = vec![0u8; length];
    hkdf.expand(&info, &mut okm)
        .map_err(|e| format!("HKDF-SHA256 expand failed: {e}"))?;

    let okm_b64 = base64::engine::general_purpose::STANDARD.encode(&okm);

    info!(length, "HKDF-SHA256 derived key material");

    Ok(serde_json::json!({
        "okm": okm_b64,
        "algorithm": "HKDF-SHA256",
        "length": length,
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

        let value = result.expect("blake3 hash should succeed");
        assert_eq!(value["algorithm"], "BLAKE3");
        assert!(value["hash"].is_string());

        // Decode and verify hash length (32 bytes)
        let h = value["hash"].as_str().expect("hash should be string");
        let hash = BASE64.decode(h).expect("hash base64");
        assert_eq!(hash.len(), 32);
    }

    #[tokio::test]
    async fn test_blake3_hash_empty_data() {
        let data = BASE64.encode(b"");
        let params = json!({ "data": data });

        let result = handle_blake3_hash(Some(&params)).await;
        assert!(result.is_ok());

        let value = result.expect("blake3 empty hash should succeed");
        let h = value["hash"].as_str().expect("hash string");
        let hash = BASE64.decode(h).expect("hash base64");
        assert_eq!(hash.len(), 32);
    }

    #[tokio::test]
    async fn test_blake3_hash_deterministic() {
        let data = BASE64.encode(b"Test data for BLAKE3");
        let params = json!({ "data": data });

        let result1 = handle_blake3_hash(Some(&params))
            .await
            .expect("blake3 first");
        let result2 = handle_blake3_hash(Some(&params))
            .await
            .expect("blake3 second");

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

        let value = result.expect("hmac-sha256 should succeed");
        assert_eq!(value["algorithm"], "HMAC-SHA256");
        assert!(value["mac"].is_string());

        // Decode and verify MAC length (32 bytes)
        let m = value["mac"].as_str().expect("mac string");
        let mac = BASE64.decode(m).expect("mac base64");
        assert_eq!(mac.len(), 32);
    }

    #[tokio::test]
    async fn test_hmac_sha256_deterministic() {
        let key = BASE64.encode(b"test-key");
        let data = BASE64.encode(b"test-data");
        let params = json!({ "key": key, "data": data });

        let result1 = handle_hmac_sha256(Some(&params)).await.expect("hmac first");
        let result2 = handle_hmac_sha256(Some(&params))
            .await
            .expect("hmac second");

        assert_eq!(result1["mac"], result2["mac"]);
    }

    #[tokio::test]
    async fn test_hmac_sha256_different_keys_different_macs() {
        let data = BASE64.encode(b"same data");

        let params1 = json!({ "key": BASE64.encode(b"key1"), "data": data });
        let params2 = json!({ "key": BASE64.encode(b"key2"), "data": data });

        let result1 = handle_hmac_sha256(Some(&params1)).await.expect("hmac key1");
        let result2 = handle_hmac_sha256(Some(&params2)).await.expect("hmac key2");

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

        let value = result.expect("hash_for_cipher 0x1301");
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

        let value = result.expect("hash_for_cipher 0x1302");
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

        let value = result.expect("hash_for_cipher 0x1303");
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

    // ========================================================================
    // HMAC VERIFY TESTS
    // ========================================================================

    #[tokio::test]
    async fn test_hmac_verify_correct_tag() {
        let key = BASE64.encode(b"secret-key-for-hmac");
        let data = BASE64.encode(b"message to authenticate");
        let mac_result = handle_hmac_sha256(Some(&json!({"key": key, "data": data})))
            .await
            .expect("hmac compute");
        let mac = mac_result["mac"].as_str().expect("mac");
        let params = json!({"key": key, "data": data, "mac": mac});
        let result = handle_hmac_verify(Some(&params)).await.expect("verify");
        assert_eq!(result["valid"], true);
        assert_eq!(result["algorithm"], "HMAC-SHA256");
    }

    #[tokio::test]
    async fn test_hmac_verify_wrong_tag() {
        let key = BASE64.encode(b"key");
        let data = BASE64.encode(b"data");
        let bad_mac = BASE64.encode(&[0u8; 32]);
        let params = json!({"key": key, "data": data, "mac": bad_mac});
        let result = handle_hmac_verify(Some(&params)).await.expect("verify");
        assert_eq!(result["valid"], false);
    }

    #[tokio::test]
    async fn test_hmac_verify_missing_params() {
        assert!(handle_hmac_verify(None).await.is_err());
        let partial = json!({"key": BASE64.encode(b"k"), "data": BASE64.encode(b"d")});
        let err = handle_hmac_verify(Some(&partial)).await.unwrap_err();
        assert!(err.contains("mac"));
    }

    // ========================================================================
    // HKDF-SHA256 TESTS
    // ========================================================================

    #[tokio::test]
    async fn test_hkdf_sha256_default_length() {
        let ikm = BASE64.encode(b"input key material");
        let params = json!({"ikm": ikm});
        let result = handle_hkdf_sha256(Some(&params)).await.expect("hkdf");
        assert_eq!(result["algorithm"], "HKDF-SHA256");
        assert_eq!(result["length"], 32);
        let okm = BASE64
            .decode(result["okm"].as_str().expect("okm"))
            .expect("b64");
        assert_eq!(okm.len(), 32);
    }

    #[tokio::test]
    async fn test_hkdf_sha256_custom_length() {
        let params = json!({"ikm": BASE64.encode(b"key"), "length": 48});
        let result = handle_hkdf_sha256(Some(&params)).await.expect("hkdf");
        assert_eq!(result["length"], 48);
        let okm = BASE64
            .decode(result["okm"].as_str().expect("okm"))
            .expect("b64");
        assert_eq!(okm.len(), 48);
    }

    #[tokio::test]
    async fn test_hkdf_sha256_with_salt_and_info() {
        let params = json!({
            "ikm": BASE64.encode(b"shared_secret"),
            "salt": BASE64.encode(b"random_salt"),
            "info": BASE64.encode(b"btsp-v1-phase3"),
            "length": 32
        });
        let result = handle_hkdf_sha256(Some(&params)).await.expect("hkdf");
        assert_eq!(result["length"], 32);
    }

    #[tokio::test]
    async fn test_hkdf_sha256_deterministic() {
        let params = json!({"ikm": BASE64.encode(b"k"), "salt": BASE64.encode(b"s"), "info": BASE64.encode(b"i")});
        let r1 = handle_hkdf_sha256(Some(&params)).await.expect("1");
        let r2 = handle_hkdf_sha256(Some(&params)).await.expect("2");
        assert_eq!(r1["okm"], r2["okm"]);
    }

    #[tokio::test]
    async fn test_hkdf_sha256_different_salt_different_output() {
        let p1 = json!({"ikm": BASE64.encode(b"k"), "salt": BASE64.encode(b"salt1")});
        let p2 = json!({"ikm": BASE64.encode(b"k"), "salt": BASE64.encode(b"salt2")});
        let r1 = handle_hkdf_sha256(Some(&p1)).await.expect("1");
        let r2 = handle_hkdf_sha256(Some(&p2)).await.expect("2");
        assert_ne!(r1["okm"], r2["okm"]);
    }

    #[tokio::test]
    async fn test_hkdf_sha256_missing_ikm_errors() {
        let params = json!({"salt": BASE64.encode(b"s")});
        let err = handle_hkdf_sha256(Some(&params)).await.unwrap_err();
        assert!(err.contains("ikm"));
    }

    #[tokio::test]
    async fn test_hkdf_sha256_missing_params_errors() {
        assert!(handle_hkdf_sha256(None).await.is_err());
    }

    #[tokio::test]
    async fn test_hkdf_sha256_matches_barrucuda_pattern() {
        let session_key = [42u8; 32];
        let client_nonce = [1u8; 16];
        let server_nonce = [2u8; 16];
        let mut salt = Vec::new();
        salt.extend_from_slice(&client_nonce);
        salt.extend_from_slice(&server_nonce);

        let params = json!({
            "ikm": BASE64.encode(session_key),
            "salt": BASE64.encode(&salt),
            "info": BASE64.encode(b"btsp-v1-phase3"),
            "length": 32
        });
        let result = handle_hkdf_sha256(Some(&params)).await.expect("hkdf");

        use hkdf::Hkdf;
        use sha2::Sha256;
        let hkdf = Hkdf::<Sha256>::new(Some(&salt), &session_key);
        let mut expected = [0u8; 32];
        hkdf.expand(b"btsp-v1-phase3", &mut expected)
            .expect("expand");

        let okm = BASE64
            .decode(result["okm"].as_str().expect("okm"))
            .expect("b64");
        assert_eq!(okm, expected);
    }
}
