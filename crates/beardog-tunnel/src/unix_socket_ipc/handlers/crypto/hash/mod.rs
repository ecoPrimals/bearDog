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

mod blake3;
mod cipher;
mod hkdf;
mod hmac;

pub use blake3::handle_blake3_hash;
pub use cipher::handle_hash_for_cipher;
pub use hkdf::handle_hkdf_sha256;
pub use hmac::{handle_hmac_sha256, handle_hmac_verify};

#[cfg(test)]
mod tests {
    use super::*;
    use base64::Engine as _;
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
        use ::hkdf::Hkdf;
        use ::sha2::Sha256;

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
