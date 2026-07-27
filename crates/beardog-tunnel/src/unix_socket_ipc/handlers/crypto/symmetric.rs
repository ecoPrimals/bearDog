// SPDX-License-Identifier: AGPL-3.0-or-later

//! Symmetric cryptography operations
//!
//! This module provides symmetric (shared-key) cryptographic operations for
//! ecoPrimals, including authenticated encryption with associated data (AEAD).
//!
//! # Overview
//!
//! Symmetric cryptography uses a single shared key for both encryption and
//! decryption. All algorithms in this module are AEAD ciphers, which provide:
//!
//! - **Confidentiality**: Data is encrypted
//! - **Integrity**: Tampering is detected
//! - **Authentication**: Messages are authenticated
//!
//! # Algorithms
//!
//! ## ChaCha20-Poly1305
//!
//! - **Type**: Stream cipher + Poly1305 MAC
//! - **Key Size**: 32 bytes (256-bit)
//! - **Nonce Size**: 12 bytes (96-bit)
//! - **Tag Size**: 16 bytes (128-bit)
//! - **Security Level**: ~256-bit
//! - **Speed**: Very fast (~1 GB/s single-core)
//! - **Use Case**: Primary AEAD for TLS 1.3, BTSP tunnels
//!
//! ### Methods
//!
//! - [`handle_chacha20_poly1305_encrypt`] - Encrypt with ChaCha20-Poly1305
//! - [`handle_chacha20_poly1305_decrypt`] - Decrypt and verify
//!
//! # Usage
//!
//! All handlers are re-exported from the parent `crypto` module:
//!
//! ```rust,ignore
//! // NOTE: These handlers are internal and called via JSON-RPC
//! use crate::unix_socket_ipc::handlers::crypto::*;
//!
//! // Encrypt data
//! let encrypted = handle_chacha20_poly1305_encrypt(params).await?;
//!
//! // Decrypt and verify
//! let plaintext = handle_chacha20_poly1305_decrypt(params).await?;
//! ```
//!
//! # Security Notes
//!
//! - **Nonce Reuse is Fatal**: Never reuse a nonce with the same key!
//! - **Use Random Nonces**: Generate 12 random bytes for each encryption
//! - **Tag Verification**: Always verify the authentication tag before using plaintext
//! - **Key Rotation**: Rotate keys periodically (genetic key exchange)
//!
//! # References
//!
//! - RFC 8439 (ChaCha20-Poly1305): <https://www.rfc-editor.org/rfc/rfc8439.html>
//! - RFC 7539 (ChaCha20): <https://www.rfc-editor.org/rfc/rfc7539.html>

use base64::Engine;
use beardog_core::crypto_service::algorithms::symmetric;
use serde_json::Value;
use tracing::{debug, info};

/// Handle ChaCha20-Poly1305 encryption operations via JSON-RPC
///
/// # Errors
///
/// Returns an error if encryption fails.
pub async fn handle_chacha20_poly1305_encrypt(
    params: Option<&Value>,
) -> Result<Value, super::super::HandlerError> {
    let params = params.ok_or("Missing params for crypto.chacha20_poly1305_encrypt")?;

    // Extract parameters
    let plaintext_b64 = params
        .get("plaintext")
        .and_then(|v| v.as_str())
        .ok_or("Missing required parameter: plaintext")?;

    let key_b64 = params
        .get("key")
        .and_then(|v| v.as_str())
        .ok_or("Missing required parameter: key")?;

    let aad_b64 = params.get("aad").and_then(|v| v.as_str());

    // Decode parameters
    let plaintext = base64::engine::general_purpose::STANDARD
        .decode(plaintext_b64)
        .map_err(|e| format!("Invalid base64 plaintext: {e}"))?;

    let key_bytes = base64::engine::general_purpose::STANDARD
        .decode(key_b64)
        .map_err(|e| format!("Invalid base64 key: {e}"))?;

    let key: [u8; 32] = key_bytes
        .as_slice()
        .try_into()
        .map_err(|_| "key must be 32 bytes".to_string())?;

    let aad = if let Some(aad_b64) = aad_b64 {
        Some(
            base64::engine::general_purpose::STANDARD
                .decode(aad_b64)
                .map_err(|e| format!("Invalid base64 aad: {e}"))?,
        )
    } else {
        None
    };

    debug!(
        "🔒 Encrypting {} bytes with ChaCha20-Poly1305",
        plaintext.len()
    );

    let (ciphertext, nonce, tag) =
        symmetric::encrypt_chacha20_poly1305(&plaintext, &key, aad.as_deref())
            .map_err(|e| format!("ChaCha20-Poly1305 encryption failed: {e}"))?;

    // Encode results
    let ciphertext_b64 = base64::engine::general_purpose::STANDARD.encode(&ciphertext);
    let nonce_b64 = base64::engine::general_purpose::STANDARD.encode(&nonce);
    let tag_b64 = base64::engine::general_purpose::STANDARD.encode(&tag);

    info!(
        "✅ ChaCha20-Poly1305 encryption complete ({} bytes → {} bytes)",
        plaintext.len(),
        ciphertext.len()
    );

    Ok(serde_json::json!({
        "ciphertext": ciphertext_b64,
        "nonce": nonce_b64,
        "tag": tag_b64,
        "algorithm": "ChaCha20-Poly1305",
    }))
}

/// # Errors
///
/// Returns an error if decryption fails.
/// Handle `crypto.chacha20_poly1305_decrypt` method
///
/// Decrypts data with ChaCha20-Poly1305 AEAD.
///
/// # Parameters
///
/// - `ciphertext`: Base64-encoded ciphertext
/// - `key`: Base64-encoded 32-byte key
/// - `nonce`: Base64-encoded 12-byte nonce
/// - `tag`: Base64-encoded 16-byte authentication tag
/// - `aad`: Base64-encoded additional authenticated data (optional)
///
/// # Returns
///
/// - `plaintext`: Base64-encoded plaintext
pub async fn handle_chacha20_poly1305_decrypt(
    params: Option<&Value>,
) -> Result<Value, super::super::HandlerError> {
    let params = params.ok_or("Missing params for crypto.chacha20_poly1305_decrypt")?;

    // Extract parameters
    let ciphertext_b64 = params
        .get("ciphertext")
        .and_then(|v| v.as_str())
        .ok_or("Missing required parameter: ciphertext")?;

    let key_b64 = params
        .get("key")
        .and_then(|v| v.as_str())
        .ok_or("Missing required parameter: key")?;

    let nonce_b64 = params
        .get("nonce")
        .and_then(|v| v.as_str())
        .ok_or("Missing required parameter: nonce")?;

    let tag_b64 = params
        .get("tag")
        .and_then(|v| v.as_str())
        .ok_or("Missing required parameter: tag")?;

    let aad_b64 = params.get("aad").and_then(|v| v.as_str());

    // Decode parameters
    let ciphertext = base64::engine::general_purpose::STANDARD
        .decode(ciphertext_b64)
        .map_err(|e| format!("Invalid base64 ciphertext: {e}"))?;

    let key_bytes = base64::engine::general_purpose::STANDARD
        .decode(key_b64)
        .map_err(|e| format!("Invalid base64 key: {e}"))?;

    let key: [u8; 32] = key_bytes
        .as_slice()
        .try_into()
        .map_err(|_| "key must be 32 bytes".to_string())?;

    let nonce = base64::engine::general_purpose::STANDARD
        .decode(nonce_b64)
        .map_err(|e| format!("Invalid base64 nonce: {e}"))?;

    let tag = base64::engine::general_purpose::STANDARD
        .decode(tag_b64)
        .map_err(|e| format!("Invalid base64 tag: {e}"))?;

    let aad = if let Some(aad_b64) = aad_b64 {
        Some(
            base64::engine::general_purpose::STANDARD
                .decode(aad_b64)
                .map_err(|e| format!("Invalid base64 aad: {e}"))?,
        )
    } else {
        None
    };

    debug!(
        "🔓 Decrypting {} bytes with ChaCha20-Poly1305",
        ciphertext.len()
    );

    // EVOLVED: Diagnostic logging moved to diagnostics module (not removed!)
    // Enable with: cargo build --features diagnostics
    // This is zero-cost when disabled (completely inlined away)
    crate::diagnostics::crypto::log_chacha20_poly1305_decrypt(
        &key,
        &nonce,
        &ciphertext,
        &tag,
        aad.as_deref(),
    );

    let plaintext =
        symmetric::decrypt_chacha20_poly1305(&ciphertext, &nonce, &tag, &key, aad.as_deref())
            .map_err(|e| format!("ChaCha20-Poly1305 decryption failed: {e}"))?;

    // Encode result
    let plaintext_b64 = base64::engine::general_purpose::STANDARD.encode(&plaintext);

    info!(
        "✅ ChaCha20-Poly1305 decryption complete ({} bytes → {} bytes)",
        ciphertext.len(),
        plaintext.len()
    );

    Ok(serde_json::json!({
        "plaintext": plaintext_b64,
        "algorithm": "ChaCha20-Poly1305",
    }))
}

#[cfg(test)]
mod tests {
    use super::*;
    use base64::engine::general_purpose::STANDARD as BASE64;
    use serde_json::json;

    // ========================================================================
    // CHACHA20-POLY1305 ENCRYPTION TESTS
    // ========================================================================

    #[tokio::test]
    async fn test_chacha20_encrypt_basic() {
        let plaintext = BASE64.encode(b"Hello, BearDog!");
        let key = BASE64.encode(&[0u8; 32]); // 32-byte key
        let params = json!({ "plaintext": plaintext, "key": key });

        let result = handle_chacha20_poly1305_encrypt(Some(&params)).await;
        assert!(result.is_ok());

        let value = result.expect("chacha20 encrypt should succeed");
        assert_eq!(value["algorithm"], "ChaCha20-Poly1305");
        assert!(value["ciphertext"].is_string());
        assert!(value["nonce"].is_string());
        assert!(value["tag"].is_string());

        // Verify nonce is 12 bytes
        let nonce_str = value["nonce"].as_str().expect("nonce should be string");
        let nonce = BASE64
            .decode(nonce_str)
            .expect("nonce should decode as base64");
        assert_eq!(nonce.len(), 12);

        // Verify tag is 16 bytes
        let tag_str = value["tag"].as_str().expect("tag should be string");
        let tag = BASE64.decode(tag_str).expect("tag should decode as base64");
        assert_eq!(tag.len(), 16);
    }

    #[tokio::test]
    async fn test_chacha20_encrypt_with_aad() {
        let plaintext = BASE64.encode(b"Secret message");
        let key = BASE64.encode(&[0xAB; 32]);
        let aad = BASE64.encode(b"Additional authenticated data");
        let params = json!({ "plaintext": plaintext, "key": key, "aad": aad });

        let result = handle_chacha20_poly1305_encrypt(Some(&params)).await;
        assert!(result.is_ok());
    }

    #[tokio::test]
    async fn test_chacha20_encrypt_empty_plaintext() {
        let plaintext = BASE64.encode(b"");
        let key = BASE64.encode(&[0u8; 32]);
        let params = json!({ "plaintext": plaintext, "key": key });

        let result = handle_chacha20_poly1305_encrypt(Some(&params)).await;
        assert!(result.is_ok());

        let value = result.expect("encrypt empty plaintext should succeed");
        let ct_str = value["ciphertext"]
            .as_str()
            .expect("ciphertext should be string");
        let ciphertext = BASE64.decode(ct_str).expect("ciphertext base64");
        assert_eq!(ciphertext.len(), 0); // Empty plaintext = empty ciphertext
    }

    #[tokio::test]
    async fn test_chacha20_encrypt_wrong_key_length() {
        let plaintext = BASE64.encode(b"data");
        let key = BASE64.encode(&[0u8; 16]); // Wrong: 16 bytes instead of 32
        let params = json!({ "plaintext": plaintext, "key": key });

        let result = handle_chacha20_poly1305_encrypt(Some(&params)).await;
        assert!(result.is_err());
        assert!(result.unwrap_err().contains("32 bytes"));
    }

    #[tokio::test]
    async fn test_chacha20_encrypt_missing_plaintext() {
        let key = BASE64.encode(&[0u8; 32]);
        let params = json!({ "key": key });

        let result = handle_chacha20_poly1305_encrypt(Some(&params)).await;
        assert!(result.is_err());
        assert!(result.unwrap_err().contains("plaintext"));
    }

    #[tokio::test]
    async fn test_chacha20_encrypt_missing_key() {
        let plaintext = BASE64.encode(b"data");
        let params = json!({ "plaintext": plaintext });

        let result = handle_chacha20_poly1305_encrypt(Some(&params)).await;
        assert!(result.is_err());
        assert!(result.unwrap_err().contains("key"));
    }

    // ========================================================================
    // CHACHA20-POLY1305 DECRYPTION TESTS
    // ========================================================================

    #[tokio::test]
    async fn test_chacha20_encrypt_decrypt_roundtrip() {
        let original = b"Test message for roundtrip!";
        let plaintext = BASE64.encode(original);
        let key = BASE64.encode(&[0x42; 32]);

        // Encrypt
        let encrypt_params = json!({ "plaintext": plaintext, "key": key });
        let encrypted = handle_chacha20_poly1305_encrypt(Some(&encrypt_params))
            .await
            .expect("encryption failed");

        // Decrypt
        let decrypt_params = json!({
            "ciphertext": encrypted["ciphertext"],
            "key": key,
            "nonce": encrypted["nonce"],
            "tag": encrypted["tag"]
        });
        let decrypted = handle_chacha20_poly1305_decrypt(Some(&decrypt_params))
            .await
            .expect("decryption failed");

        // Verify roundtrip
        let pt_str = decrypted["plaintext"]
            .as_str()
            .expect("plaintext should be string");
        let result = BASE64.decode(pt_str).expect("plaintext base64");
        assert_eq!(result, original);
    }

    #[tokio::test]
    async fn test_chacha20_encrypt_decrypt_with_aad() {
        let original = b"Authenticated message";
        let plaintext = BASE64.encode(original);
        let key = BASE64.encode(&[0x55; 32]);
        let aad = BASE64.encode(b"context data");

        // Encrypt with AAD
        let encrypt_params = json!({ "plaintext": plaintext, "key": key, "aad": aad });
        let encrypted = handle_chacha20_poly1305_encrypt(Some(&encrypt_params))
            .await
            .expect("encryption failed");

        // Decrypt with same AAD
        let decrypt_params = json!({
            "ciphertext": encrypted["ciphertext"],
            "key": key,
            "nonce": encrypted["nonce"],
            "tag": encrypted["tag"],
            "aad": aad
        });
        let decrypted = handle_chacha20_poly1305_decrypt(Some(&decrypt_params))
            .await
            .expect("decryption failed");

        let pt_str = decrypted["plaintext"]
            .as_str()
            .expect("plaintext should be string");
        let result = BASE64.decode(pt_str).expect("plaintext base64");
        assert_eq!(result, original);
    }

    #[tokio::test]
    async fn test_chacha20_decrypt_wrong_key_fails() {
        let plaintext = BASE64.encode(b"Secret data");
        let key1 = BASE64.encode(&[0x11; 32]);
        let key2 = BASE64.encode(&[0x22; 32]); // Different key

        // Encrypt with key1
        let encrypt_params = json!({ "plaintext": plaintext, "key": key1 });
        let encrypted = handle_chacha20_poly1305_encrypt(Some(&encrypt_params))
            .await
            .expect("encryption failed");

        // Decrypt with key2 - should fail
        let decrypt_params = json!({
            "ciphertext": encrypted["ciphertext"],
            "key": key2,
            "nonce": encrypted["nonce"],
            "tag": encrypted["tag"]
        });
        let result = handle_chacha20_poly1305_decrypt(Some(&decrypt_params)).await;
        assert!(result.is_err());
    }

    #[tokio::test]
    async fn test_chacha20_decrypt_modified_ciphertext_fails() {
        let plaintext = BASE64.encode(b"Integrity test");
        let key = BASE64.encode(&[0x33; 32]);

        // Encrypt
        let encrypt_params = json!({ "plaintext": plaintext, "key": key });
        let encrypted = handle_chacha20_poly1305_encrypt(Some(&encrypt_params))
            .await
            .expect("encryption failed");

        // Modify ciphertext
        let ct_str = encrypted["ciphertext"].as_str().expect("ciphertext string");
        let mut ciphertext = BASE64.decode(ct_str).expect("ciphertext base64");
        if !ciphertext.is_empty() {
            ciphertext[0] ^= 0xFF; // Flip bits
        }
        let modified_ciphertext = BASE64.encode(&ciphertext);

        // Decrypt modified ciphertext - should fail
        let decrypt_params = json!({
            "ciphertext": modified_ciphertext,
            "key": key,
            "nonce": encrypted["nonce"],
            "tag": encrypted["tag"]
        });
        let result = handle_chacha20_poly1305_decrypt(Some(&decrypt_params)).await;
        assert!(result.is_err());
    }

    #[tokio::test]
    async fn test_chacha20_decrypt_wrong_aad_fails() {
        let plaintext = BASE64.encode(b"AAD test");
        let key = BASE64.encode(&[0x44; 32]);
        let aad1 = BASE64.encode(b"correct aad");
        let aad2 = BASE64.encode(b"wrong aad");

        // Encrypt with aad1
        let encrypt_params = json!({ "plaintext": plaintext, "key": key, "aad": aad1 });
        let encrypted = handle_chacha20_poly1305_encrypt(Some(&encrypt_params))
            .await
            .expect("encryption failed");

        // Decrypt with aad2 - should fail
        let decrypt_params = json!({
            "ciphertext": encrypted["ciphertext"],
            "key": key,
            "nonce": encrypted["nonce"],
            "tag": encrypted["tag"],
            "aad": aad2
        });
        let result = handle_chacha20_poly1305_decrypt(Some(&decrypt_params)).await;
        assert!(result.is_err());
    }

    #[tokio::test]
    async fn test_chacha20_decrypt_missing_nonce() {
        let ciphertext = BASE64.encode(b"ciphertext");
        let key = BASE64.encode(&[0u8; 32]);
        let tag = BASE64.encode(&[0u8; 16]);
        let params = json!({ "ciphertext": ciphertext, "key": key, "tag": tag });

        let result = handle_chacha20_poly1305_decrypt(Some(&params)).await;
        assert!(result.is_err());
        assert!(result.unwrap_err().contains("nonce"));
    }

    #[tokio::test]
    async fn test_chacha20_decrypt_missing_tag() {
        let ciphertext = BASE64.encode(b"ciphertext");
        let key = BASE64.encode(&[0u8; 32]);
        let nonce = BASE64.encode(&[0u8; 12]);
        let params = json!({ "ciphertext": ciphertext, "key": key, "nonce": nonce });

        let result = handle_chacha20_poly1305_decrypt(Some(&params)).await;
        assert!(result.is_err());
        assert!(result.unwrap_err().contains("tag"));
    }
}
