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
//! ```rust,no_run
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
use serde_json::Value;
use tracing::{debug, info};

pub async fn handle_chacha20_poly1305_encrypt(params: Option<&Value>) -> Result<Value, String> {
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

    // Use BearDog's crypto service
    use beardog_core::crypto_service::algorithms::symmetric;

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

/// Handle crypto.chacha20_poly1305_decrypt method
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
pub async fn handle_chacha20_poly1305_decrypt(params: Option<&Value>) -> Result<Value, String> {
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

    // Use BearDog's crypto service
    use beardog_core::crypto_service::algorithms::symmetric;

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
