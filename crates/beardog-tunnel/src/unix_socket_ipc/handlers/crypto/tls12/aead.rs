// SPDX-License-Identifier: AGPL-3.0-or-later

//! TLS 1.2 AES-GCM AEAD (128-bit and 256-bit keys).

use base64::Engine;
use base64::engine::general_purpose::STANDARD as BASE64;
use serde_json::Value;
use tracing::{debug, info};

// =============================================================================
// AES-GCM AEAD (128-bit and 256-bit)
// =============================================================================

/// # Errors
///
/// Returns an error if encryption fails.
/// Handle `crypto.aead.aes_128_gcm.encrypt` method
///
/// Encrypts data using AES-128-GCM AEAD.
///
/// # Parameters
///
/// - `key`: Base64-encoded AES key (16 bytes for AES-128)
/// - `nonce`: Base64-encoded nonce (12 bytes standard for GCM)
/// - `plaintext`: Base64-encoded plaintext
/// - `aad`: Base64-encoded additional authenticated data (optional)
///
/// # Returns
///
/// - `ciphertext`: Base64-encoded ciphertext
/// - `tag`: Base64-encoded authentication tag (16 bytes)
/// - `algorithm`: "AES-128-GCM"
///
/// # Security
///
/// ⚠️ **NEVER reuse a nonce with the same key!** Use random 12-byte nonces.
pub async fn handle_aes_128_gcm_encrypt(params: Option<&Value>) -> Result<Value, String> {
    let params = params.ok_or("Missing parameters for AES-128-GCM encryption")?;

    let key_b64 = params
        .get("key")
        .and_then(|v| v.as_str())
        .ok_or("Missing 'key' parameter")?;

    let nonce_b64 = params
        .get("nonce")
        .and_then(|v| v.as_str())
        .ok_or("Missing 'nonce' parameter")?;

    let plaintext_b64 = params
        .get("plaintext")
        .and_then(|v| v.as_str())
        .ok_or("Missing 'plaintext' parameter")?;

    let aad_b64 = params.get("aad").and_then(|v| v.as_str());

    debug!("🔒 AES-128-GCM encryption");

    // Decode inputs
    let key_bytes = BASE64
        .decode(key_b64)
        .map_err(|e| format!("Invalid key base64: {e}"))?;

    if key_bytes.len() != 16 {
        return Err(format!(
            "Invalid AES-128 key length: {} (expected 16)",
            key_bytes.len()
        ));
    }

    let nonce_bytes = BASE64
        .decode(nonce_b64)
        .map_err(|e| format!("Invalid nonce base64: {e}"))?;

    if nonce_bytes.len() != 12 {
        return Err(format!(
            "Invalid GCM nonce length: {} (expected 12)",
            nonce_bytes.len()
        ));
    }

    let plaintext_bytes = BASE64
        .decode(plaintext_b64)
        .map_err(|e| format!("Invalid plaintext base64: {e}"))?;

    let aad_bytes = if let Some(aad) = aad_b64 {
        BASE64
            .decode(aad)
            .map_err(|e| format!("Invalid AAD base64: {e}"))?
    } else {
        Vec::new()
    };

    // Perform encryption
    use aes_gcm::{Aes128Gcm, KeyInit, Nonce, aead::Aead};

    let key = aes_gcm::Key::<Aes128Gcm>::from_slice(&key_bytes);
    let cipher = Aes128Gcm::new(key);
    let nonce = Nonce::from_slice(&nonce_bytes);

    let ciphertext = cipher
        .encrypt(
            nonce,
            aes_gcm::aead::Payload {
                msg: &plaintext_bytes,
                aad: &aad_bytes,
            },
        )
        .map_err(|e| format!("AES-128-GCM encryption failed: {e}"))?;

    // Split ciphertext and tag (GCM appends 16-byte tag)
    let tag_offset = ciphertext.len() - 16;
    let ciphertext_only = &ciphertext[..tag_offset];
    let tag = &ciphertext[tag_offset..];

    let ciphertext_b64 = BASE64.encode(ciphertext_only);
    let tag_b64 = BASE64.encode(tag);

    info!(
        "✅ AES-128-GCM encryption complete ({} bytes → {} bytes)",
        plaintext_bytes.len(),
        ciphertext_only.len()
    );

    Ok(serde_json::json!({
        "ciphertext": ciphertext_b64,
        "tag": tag_b64,
        "algorithm": "AES-128-GCM",
    }))
}

/// # Errors
///
/// Returns an error if decryption fails.
/// Handle `crypto.aead.aes_128_gcm.decrypt` method
///
/// Decrypts and verifies data using AES-128-GCM AEAD.
///
/// # Parameters
///
/// - `key`: Base64-encoded AES key (16 bytes)
/// - `nonce`: Base64-encoded nonce (12 bytes)
/// - `ciphertext`: Base64-encoded ciphertext
/// - `tag`: Base64-encoded authentication tag (16 bytes)
/// - `aad`: Base64-encoded additional authenticated data (optional)
///
/// # Returns
///
/// - `plaintext`: Base64-encoded plaintext
/// - `verified`: Always `true` (decryption fails if tag invalid)
/// - `algorithm`: "AES-128-GCM"
pub async fn handle_aes_128_gcm_decrypt(params: Option<&Value>) -> Result<Value, String> {
    let params = params.ok_or("Missing parameters for AES-128-GCM decryption")?;

    let key_b64 = params
        .get("key")
        .and_then(|v| v.as_str())
        .ok_or("Missing 'key' parameter")?;

    let nonce_b64 = params
        .get("nonce")
        .and_then(|v| v.as_str())
        .ok_or("Missing 'nonce' parameter")?;

    let ciphertext_b64 = params
        .get("ciphertext")
        .and_then(|v| v.as_str())
        .ok_or("Missing 'ciphertext' parameter")?;

    let tag_b64 = params
        .get("tag")
        .and_then(|v| v.as_str())
        .ok_or("Missing 'tag' parameter")?;

    let aad_b64 = params.get("aad").and_then(|v| v.as_str());

    debug!("🔓 AES-128-GCM decryption");

    // Decode inputs
    let key_bytes = BASE64
        .decode(key_b64)
        .map_err(|e| format!("Invalid key base64: {e}"))?;

    let nonce_bytes = BASE64
        .decode(nonce_b64)
        .map_err(|e| format!("Invalid nonce base64: {e}"))?;

    let ciphertext_bytes = BASE64
        .decode(ciphertext_b64)
        .map_err(|e| format!("Invalid ciphertext base64: {e}"))?;

    let tag_bytes = BASE64
        .decode(tag_b64)
        .map_err(|e| format!("Invalid tag base64: {e}"))?;

    let aad_bytes = if let Some(aad) = aad_b64 {
        BASE64
            .decode(aad)
            .map_err(|e| format!("Invalid AAD base64: {e}"))?
    } else {
        Vec::new()
    };

    // Concatenate ciphertext + tag (required by aes-gcm crate)
    let mut ciphertext_with_tag = ciphertext_bytes;
    ciphertext_with_tag.extend_from_slice(&tag_bytes);

    // Perform decryption
    use aes_gcm::{Aes128Gcm, KeyInit, Nonce, aead::Aead};

    let key = aes_gcm::Key::<Aes128Gcm>::from_slice(&key_bytes);
    let cipher = Aes128Gcm::new(key);
    let nonce = Nonce::from_slice(&nonce_bytes);

    let plaintext = cipher
        .decrypt(
            nonce,
            aes_gcm::aead::Payload {
                msg: &ciphertext_with_tag,
                aad: &aad_bytes,
            },
        )
        .map_err(|e| format!("AES-128-GCM decryption/verification failed: {e}"))?;

    let plaintext_b64 = BASE64.encode(&plaintext);

    info!(
        "✅ AES-128-GCM decryption verified ({} bytes)",
        plaintext.len()
    );

    Ok(serde_json::json!({
        "plaintext": plaintext_b64,
        "verified": true,
        "algorithm": "AES-128-GCM",
    }))
}

/// # Errors
///
/// Returns an error if encryption fails.
/// Handle `crypto.aead.aes_256_gcm.encrypt` method
///
/// Encrypts data using AES-256-GCM AEAD.
///
/// # Parameters
///
/// Same as `handle_aes_128_gcm_encrypt` but with 32-byte key.
///
/// # Returns
///
/// Same as `handle_aes_128_gcm_encrypt` with algorithm "AES-256-GCM".
pub async fn handle_aes_256_gcm_encrypt(params: Option<&Value>) -> Result<Value, String> {
    let params = params.ok_or("Missing parameters for AES-256-GCM encryption")?;

    let key_b64 = params
        .get("key")
        .and_then(|v| v.as_str())
        .ok_or("Missing 'key' parameter")?;

    let nonce_b64 = params
        .get("nonce")
        .and_then(|v| v.as_str())
        .ok_or("Missing 'nonce' parameter")?;

    let plaintext_b64 = params
        .get("plaintext")
        .and_then(|v| v.as_str())
        .ok_or("Missing 'plaintext' parameter")?;

    let aad_b64 = params.get("aad").and_then(|v| v.as_str());

    debug!("🔒 AES-256-GCM encryption");

    // Decode inputs
    let key_bytes = BASE64
        .decode(key_b64)
        .map_err(|e| format!("Invalid key base64: {e}"))?;

    if key_bytes.len() != 32 {
        return Err(format!(
            "Invalid AES-256 key length: {} (expected 32)",
            key_bytes.len()
        ));
    }

    let nonce_bytes = BASE64
        .decode(nonce_b64)
        .map_err(|e| format!("Invalid nonce base64: {e}"))?;

    if nonce_bytes.len() != 12 {
        return Err(format!(
            "Invalid GCM nonce length: {} (expected 12)",
            nonce_bytes.len()
        ));
    }

    let plaintext_bytes = BASE64
        .decode(plaintext_b64)
        .map_err(|e| format!("Invalid plaintext base64: {e}"))?;

    let aad_bytes = if let Some(aad) = aad_b64 {
        BASE64
            .decode(aad)
            .map_err(|e| format!("Invalid AAD base64: {e}"))?
    } else {
        Vec::new()
    };

    // Perform encryption
    use aes_gcm::{Aes256Gcm, KeyInit, Nonce, aead::Aead};

    let key = aes_gcm::Key::<Aes256Gcm>::from_slice(&key_bytes);
    let cipher = Aes256Gcm::new(key);
    let nonce = Nonce::from_slice(&nonce_bytes);

    let ciphertext = cipher
        .encrypt(
            nonce,
            aes_gcm::aead::Payload {
                msg: &plaintext_bytes,
                aad: &aad_bytes,
            },
        )
        .map_err(|e| format!("AES-256-GCM encryption failed: {e}"))?;

    // Split ciphertext and tag
    let tag_offset = ciphertext.len() - 16;
    let ciphertext_only = &ciphertext[..tag_offset];
    let tag = &ciphertext[tag_offset..];

    let ciphertext_b64 = BASE64.encode(ciphertext_only);
    let tag_b64 = BASE64.encode(tag);

    info!(
        "✅ AES-256-GCM encryption complete ({} bytes → {} bytes)",
        plaintext_bytes.len(),
        ciphertext_only.len()
    );

    Ok(serde_json::json!({
        "ciphertext": ciphertext_b64,
        "tag": tag_b64,
        "algorithm": "AES-256-GCM",
    }))
}

/// # Errors
///
/// Returns an error if decryption fails.
/// Handle `crypto.aead.aes_256_gcm.decrypt` method
///
/// Decrypts and verifies data using AES-256-GCM AEAD.
///
/// # Parameters
///
/// Same as `handle_aes_128_gcm_decrypt` but with 32-byte key.
///
/// # Returns
///
/// Same as `handle_aes_128_gcm_decrypt` with algorithm "AES-256-GCM".
pub async fn handle_aes_256_gcm_decrypt(params: Option<&Value>) -> Result<Value, String> {
    let params = params.ok_or("Missing parameters for AES-256-GCM decryption")?;

    let key_b64 = params
        .get("key")
        .and_then(|v| v.as_str())
        .ok_or("Missing 'key' parameter")?;

    let nonce_b64 = params
        .get("nonce")
        .and_then(|v| v.as_str())
        .ok_or("Missing 'nonce' parameter")?;

    let ciphertext_b64 = params
        .get("ciphertext")
        .and_then(|v| v.as_str())
        .ok_or("Missing 'ciphertext' parameter")?;

    let tag_b64 = params
        .get("tag")
        .and_then(|v| v.as_str())
        .ok_or("Missing 'tag' parameter")?;

    let aad_b64 = params.get("aad").and_then(|v| v.as_str());

    debug!("🔓 AES-256-GCM decryption");

    // Decode inputs
    let key_bytes = BASE64
        .decode(key_b64)
        .map_err(|e| format!("Invalid key base64: {e}"))?;

    let nonce_bytes = BASE64
        .decode(nonce_b64)
        .map_err(|e| format!("Invalid nonce base64: {e}"))?;

    let ciphertext_bytes = BASE64
        .decode(ciphertext_b64)
        .map_err(|e| format!("Invalid ciphertext base64: {e}"))?;

    let tag_bytes = BASE64
        .decode(tag_b64)
        .map_err(|e| format!("Invalid tag base64: {e}"))?;

    let aad_bytes = if let Some(aad) = aad_b64 {
        BASE64
            .decode(aad)
            .map_err(|e| format!("Invalid AAD base64: {e}"))?
    } else {
        Vec::new()
    };

    // Concatenate ciphertext + tag
    let mut ciphertext_with_tag = ciphertext_bytes;
    ciphertext_with_tag.extend_from_slice(&tag_bytes);

    // Perform decryption
    use aes_gcm::{Aes256Gcm, KeyInit, Nonce, aead::Aead};

    let key = aes_gcm::Key::<Aes256Gcm>::from_slice(&key_bytes);
    let cipher = Aes256Gcm::new(key);
    let nonce = Nonce::from_slice(&nonce_bytes);

    let plaintext = cipher
        .decrypt(
            nonce,
            aes_gcm::aead::Payload {
                msg: &ciphertext_with_tag,
                aad: &aad_bytes,
            },
        )
        .map_err(|e| format!("AES-256-GCM decryption/verification failed: {e}"))?;

    let plaintext_b64 = BASE64.encode(&plaintext);

    info!(
        "✅ AES-256-GCM decryption verified ({} bytes)",
        plaintext.len()
    );

    Ok(serde_json::json!({
        "plaintext": plaintext_b64,
        "verified": true,
        "algorithm": "AES-256-GCM",
    }))
}
