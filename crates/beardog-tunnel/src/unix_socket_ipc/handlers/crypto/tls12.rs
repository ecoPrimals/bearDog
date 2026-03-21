// SPDX-License-Identifier: AGPL-3.0-only

//! TLS 1.2 cryptographic operations
//!
//! This module provides TLS 1.2-specific cryptographic operations for Songbird
//! and other primals that need backward compatibility with older systems.
//!
//! # Overview
//!
//! TLS 1.2 uses different algorithms than TLS 1.3:
//! - **Key Exchange**: ECDHE with NIST P-256/P-384 curves (vs X25519)
//! - **Bulk Encryption**: AES-GCM AEAD (vs ChaCha20-Poly1305)
//! - **Key Derivation**: TLS 1.2 PRF (vs HKDF)
//! - **Signatures**: ECDSA with NIST curves (vs Ed25519)
//!
//! # Architecture: Tower Atomic Pattern
//!
//! BearDog provides the cryptographic "atoms" via JSON-RPC:
//! - Songbird orchestrates the TLS 1.2 handshake
//! - BearDog executes the crypto operations
//! - Zero crypto code duplication across primals
//! - Pure Rust for both (ecoBin compliant)
//!
//! # Methods (Semantic Naming)
//!
//! ## ECDHE Key Exchange
//! - `crypto.ecdhe.p256.generate` - Generate P-256 ephemeral keypair
//! - `crypto.ecdhe.p256.compute_shared` - Compute ECDH shared secret (P-256)
//! - `crypto.ecdhe.p384.generate` - Generate P-384 ephemeral keypair
//! - `crypto.ecdhe.p384.compute_shared` - Compute ECDH shared secret (P-384)
//!
//! ## AES-GCM AEAD
//! - `crypto.aead.aes_128_gcm.encrypt` - AES-128-GCM encryption
//! - `crypto.aead.aes_128_gcm.decrypt` - AES-128-GCM decryption
//! - `crypto.aead.aes_256_gcm.encrypt` - AES-256-GCM encryption
//! - `crypto.aead.aes_256_gcm.decrypt` - AES-256-GCM decryption
//!
//! ## TLS 1.2 PRF
//! - `crypto.kdf.tls12_prf` - TLS 1.2 Pseudorandom Function (key expansion)
//!
//! # Security
//!
//! - **NIST Curves**: P-256 (128-bit security), P-384 (192-bit security)
//! - **AES-GCM**: AEAD with 128-bit or 256-bit keys, 12-byte nonces, 16-byte tags
//! - **TLS 1.2 PRF**: HMAC-SHA256 or HMAC-SHA384 based key expansion
//! - **Pure Rust**: 100% RustCrypto, zero C dependencies
//!
//! # Usage
//!
//! ```rust,ignore
//! // Generate P-256 ephemeral keypair for ECDHE
//! let keypair = handle_ecdhe_p256_generate(None).await?;
//!
//! // Compute shared secret
//! let shared = handle_ecdhe_p256_compute_shared(params).await?;
//!
//! // Derive TLS 1.2 keys
//! let keys = handle_tls12_prf(params).await?;
//!
//! // Encrypt with AES-128-GCM
//! let ciphertext = handle_aes_128_gcm_encrypt(params).await?;
//! ```
//!
//! # References
//!
//! - [RFC 5246](https://www.rfc-editor.org/rfc/rfc5246.html) - TLS 1.2 Protocol
//! - [RFC 5288](https://www.rfc-editor.org/rfc/rfc5288.html) - AES-GCM for TLS 1.2
//! - [RFC 4492](https://www.rfc-editor.org/rfc/rfc4492.html) - ECC for TLS
//! - [RFC 5869](https://www.rfc-editor.org/rfc/rfc5869.html) - HKDF (TLS 1.3)
//!
//! # Created
//!
//! January 27, 2026 - Deep Debt Evolution Session\
//! Requested by: Songbird team for TLS 1.2 backward compatibility

use base64::Engine;
use base64::engine::general_purpose::STANDARD as BASE64;
use serde_json::Value;
use tracing::{debug, info};

// =============================================================================
// ECDHE with NIST P-256 (secp256r1)
// =============================================================================

/// Handle crypto.ecdhe.p256.generate method
///
/// Generates an ephemeral P-256 (secp256r1) keypair for ECDHE key exchange.
///
/// # Parameters
///
/// - `purpose`: Optional purpose string (for logging)
///
/// # Returns
///
/// - `public_key`: Base64-encoded P-256 public key (compressed, 33 bytes)
/// - `secret_key`: Base64-encoded P-256 secret key (32 bytes)
/// - `algorithm`: "P-256" (NIST secp256r1)
pub async fn handle_ecdhe_p256_generate(params: Option<&Value>) -> Result<Value, String> {
    let purpose = params
        .and_then(|p| p.get("purpose"))
        .and_then(|v| v.as_str())
        .unwrap_or("tls12_ecdhe");

    debug!(
        "🔑 Generating ephemeral P-256 keypair (purpose: {})",
        purpose
    );

    use p256::elliptic_curve::{SecretKey, rand_core::RngCore};
    use rand::rngs::OsRng;
    use zeroize::Zeroizing;

    // Generate a random 32-byte secret
    let mut private_key_bytes = Zeroizing::new([0u8; 32]);
    OsRng.fill_bytes(&mut *private_key_bytes);

    // Create secret key from bytes
    let secret_key: SecretKey<p256::NistP256> = SecretKey::from_slice(&private_key_bytes[..])
        .map_err(|e| format!("Failed to create P-256 secret key: {e}"))?;

    // Derive public key from secret
    let public_key = secret_key.public_key();
    let public_key_bytes = public_key.to_sec1_bytes();

    // Encode keys
    let private_b64 = BASE64.encode(&private_key_bytes[..]);
    let public_b64 = BASE64.encode(&public_key_bytes);

    info!(
        "✅ Ephemeral P-256 keypair generated ({} bytes public)",
        public_key_bytes.len()
    );

    Ok(serde_json::json!({
        "public_key": public_b64,
        "secret_key": private_b64,
        "algorithm": "P-256",
        "curve": "secp256r1",
    }))
}

/// Handle crypto.ecdhe.p256.compute_shared method
///
/// Computes ECDH shared secret using P-256.
///
/// # Parameters
///
/// - `our_secret`: Base64-encoded P-256 secret key (32 bytes)
/// - `their_public`: Base64-encoded P-256 public key (33 or 65 bytes)
///
/// # Returns
///
/// - `shared_secret`: Base64-encoded shared secret (32 bytes)
/// - `algorithm`: "P-256"
pub async fn handle_ecdhe_p256_compute_shared(params: Option<&Value>) -> Result<Value, String> {
    let params = params.ok_or("Missing parameters for P-256 ECDH")?;

    let our_secret_b64 = params
        .get("our_secret")
        .and_then(|v| v.as_str())
        .ok_or("Missing 'our_secret' parameter")?;

    let their_public_b64 = params
        .get("their_public")
        .and_then(|v| v.as_str())
        .ok_or("Missing 'their_public' parameter")?;

    debug!("🔐 Computing P-256 ECDH shared secret");

    // Decode keys
    let our_secret_bytes = BASE64
        .decode(our_secret_b64)
        .map_err(|e| format!("Invalid our_secret base64: {e}"))?;

    let their_public_bytes = BASE64
        .decode(their_public_b64)
        .map_err(|e| format!("Invalid their_public base64: {e}"))?;

    // Parse keys
    use p256::PublicKey;
    use p256::elliptic_curve::SecretKey;

    let secret_key: SecretKey<p256::NistP256> = SecretKey::from_slice(&our_secret_bytes)
        .map_err(|e| format!("Invalid P-256 secret key: {e}"))?;

    let peer_public_key = PublicKey::from_sec1_bytes(&their_public_bytes)
        .map_err(|e| format!("Invalid P-256 public key: {e}"))?;

    // Perform ECDH
    use p256::ecdh::diffie_hellman;
    let shared_secret = diffie_hellman(secret_key.to_nonzero_scalar(), peer_public_key.as_affine());

    // Encode shared secret
    let shared_secret_b64 = BASE64.encode(shared_secret.raw_secret_bytes());

    info!("✅ P-256 ECDH shared secret computed (32 bytes)");

    Ok(serde_json::json!({
        "shared_secret": shared_secret_b64,
        "algorithm": "P-256",
    }))
}

// =============================================================================
// ECDHE with NIST P-384 (secp384r1)
// =============================================================================

/// Handle crypto.ecdhe.p384.generate method
///
/// Generates an ephemeral P-384 (secp384r1) keypair for ECDHE key exchange.
///
/// # Parameters
///
/// - `purpose`: Optional purpose string (for logging)
///
/// # Returns
///
/// - `public_key`: Base64-encoded P-384 public key (compressed, 49 bytes)
/// - `secret_key`: Base64-encoded P-384 secret key (48 bytes)
/// - `algorithm`: "P-384" (NIST secp384r1)
pub async fn handle_ecdhe_p384_generate(params: Option<&Value>) -> Result<Value, String> {
    let purpose = params
        .and_then(|p| p.get("purpose"))
        .and_then(|v| v.as_str())
        .unwrap_or("tls12_ecdhe");

    debug!(
        "🔑 Generating ephemeral P-384 keypair (purpose: {})",
        purpose
    );

    use p384::elliptic_curve::{SecretKey, rand_core::RngCore};
    use rand::rngs::OsRng;
    use zeroize::Zeroizing;

    // Generate a random 48-byte secret (P-384)
    let mut private_key_bytes = Zeroizing::new([0u8; 48]);
    OsRng.fill_bytes(&mut *private_key_bytes);

    // Create secret key from bytes
    let secret_key: SecretKey<p384::NistP384> = SecretKey::from_slice(&private_key_bytes[..])
        .map_err(|e| format!("Failed to create P-384 secret key: {e}"))?;

    // Derive public key from secret
    let public_key = secret_key.public_key();
    let public_key_bytes = public_key.to_sec1_bytes();

    // Encode keys
    let private_b64 = BASE64.encode(&private_key_bytes[..]);
    let public_b64 = BASE64.encode(&public_key_bytes);

    info!(
        "✅ Ephemeral P-384 keypair generated ({} bytes public)",
        public_key_bytes.len()
    );

    Ok(serde_json::json!({
        "public_key": public_b64,
        "secret_key": private_b64,
        "algorithm": "P-384",
        "curve": "secp384r1",
    }))
}

/// Handle crypto.ecdhe.p384.compute_shared method
///
/// Computes ECDH shared secret using P-384.
///
/// # Parameters
///
/// - `our_secret`: Base64-encoded P-384 secret key (48 bytes)
/// - `their_public`: Base64-encoded P-384 public key (49 or 97 bytes)
///
/// # Returns
///
/// - `shared_secret`: Base64-encoded shared secret (48 bytes)
/// - `algorithm`: "P-384"
pub async fn handle_ecdhe_p384_compute_shared(params: Option<&Value>) -> Result<Value, String> {
    let params = params.ok_or("Missing parameters for P-384 ECDH")?;

    let our_secret_b64 = params
        .get("our_secret")
        .and_then(|v| v.as_str())
        .ok_or("Missing 'our_secret' parameter")?;

    let their_public_b64 = params
        .get("their_public")
        .and_then(|v| v.as_str())
        .ok_or("Missing 'their_public' parameter")?;

    debug!("🔐 Computing P-384 ECDH shared secret");

    // Decode keys
    let our_secret_bytes = BASE64
        .decode(our_secret_b64)
        .map_err(|e| format!("Invalid our_secret base64: {e}"))?;

    let their_public_bytes = BASE64
        .decode(their_public_b64)
        .map_err(|e| format!("Invalid their_public base64: {e}"))?;

    // Parse keys
    use p384::PublicKey;
    use p384::elliptic_curve::SecretKey;

    let secret_key: SecretKey<p384::NistP384> = SecretKey::from_slice(&our_secret_bytes)
        .map_err(|e| format!("Invalid P-384 secret key: {e}"))?;

    let peer_public_key = PublicKey::from_sec1_bytes(&their_public_bytes)
        .map_err(|e| format!("Invalid P-384 public key: {e}"))?;

    // Perform ECDH
    use p384::ecdh::diffie_hellman;
    let shared_secret = diffie_hellman(secret_key.to_nonzero_scalar(), peer_public_key.as_affine());

    // Encode shared secret
    let shared_secret_b64 = BASE64.encode(shared_secret.raw_secret_bytes());

    info!("✅ P-384 ECDH shared secret computed (48 bytes)");

    Ok(serde_json::json!({
        "shared_secret": shared_secret_b64,
        "algorithm": "P-384",
    }))
}

// =============================================================================
// AES-GCM AEAD (128-bit and 256-bit)
// =============================================================================

/// Handle crypto.aead.aes_128_gcm.encrypt method
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

/// Handle crypto.aead.aes_128_gcm.decrypt method
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

/// Handle crypto.aead.aes_256_gcm.encrypt method
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

/// Handle crypto.aead.aes_256_gcm.decrypt method
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

// =============================================================================
// TLS 1.2 PRF (Pseudorandom Function)
// =============================================================================

/// Handle crypto.kdf.tls12_prf method
///
/// TLS 1.2 Pseudorandom Function for key expansion (RFC 5246 Section 5).
///
/// # Parameters
///
/// - `secret`: Base64-encoded secret (e.g., premaster secret or master secret)
/// - `label`: ASCII label string (e.g., "master secret", "key expansion")
/// - `seed`: Base64-encoded seed data (e.g., client_random + server_random)
/// - `output_len`: Desired output length in bytes
/// - `hash`: Hash algorithm ("sha256" or "sha384", default "sha256")
///
/// # Returns
///
/// - `output`: Base64-encoded PRF output
/// - `algorithm`: "TLS12-PRF-SHA256" or "TLS12-PRF-SHA384"
///
/// # TLS 1.2 PRF Definition
///
/// ```text
/// PRF(secret, label, seed) = P_hash(secret, label + seed)
///
/// P_hash(secret, seed) = HMAC_hash(secret, A(1) + seed) +
///                        HMAC_hash(secret, A(2) + seed) +
///                        HMAC_hash(secret, A(3) + seed) + ...
///
/// where:
/// A(0) = seed
/// A(i) = HMAC_hash(secret, A(i-1))
/// ```
pub async fn handle_tls12_prf(params: Option<&Value>) -> Result<Value, String> {
    let params = params.ok_or("Missing parameters for TLS 1.2 PRF")?;

    let secret_b64 = params
        .get("secret")
        .and_then(|v| v.as_str())
        .ok_or("Missing 'secret' parameter")?;

    let label = params
        .get("label")
        .and_then(|v| v.as_str())
        .ok_or("Missing 'label' parameter")?;

    let seed_b64 = params
        .get("seed")
        .and_then(|v| v.as_str())
        .ok_or("Missing 'seed' parameter")?;

    #[expect(
        clippy::cast_possible_truncation,
        reason = "TLS 1.2 PRF output length from parameters"
    )]
    let output_len = params
        .get("output_len")
        .and_then(serde_json::Value::as_u64)
        .ok_or("Missing or invalid 'output_len' parameter")? as usize;

    let hash_alg = params
        .get("hash")
        .and_then(|v| v.as_str())
        .unwrap_or("sha256");

    debug!(
        "🔑 TLS 1.2 PRF: label='{}', hash={}, output_len={}",
        label, hash_alg, output_len
    );

    // Decode inputs
    let secret = BASE64
        .decode(secret_b64)
        .map_err(|e| format!("Invalid secret base64: {e}"))?;

    let seed = BASE64
        .decode(seed_b64)
        .map_err(|e| format!("Invalid seed base64: {e}"))?;

    // Compute PRF
    let output = match hash_alg {
        "sha256" => tls12_prf_sha256(&secret, label.as_bytes(), &seed, output_len)?,
        "sha384" => tls12_prf_sha384(&secret, label.as_bytes(), &seed, output_len)?,
        _ => {
            return Err(format!(
                "Unsupported hash algorithm: {hash_alg} (use 'sha256' or 'sha384')"
            ));
        }
    };

    let output_b64 = BASE64.encode(&output);

    info!(
        "✅ TLS 1.2 PRF complete ({} bytes output, {})",
        output_len,
        hash_alg.to_uppercase()
    );

    Ok(serde_json::json!({
        "output": output_b64,
        "algorithm": format!("TLS12-PRF-{}", hash_alg.to_uppercase()),
    }))
}

/// TLS 1.2 PRF with SHA-256
fn tls12_prf_sha256(
    secret: &[u8],
    label: &[u8],
    seed: &[u8],
    output_len: usize,
) -> Result<Vec<u8>, String> {
    use hmac::{Hmac, Mac};
    use sha2::Sha256;

    type HmacSha256 = Hmac<Sha256>;

    // Combine label + seed
    let mut label_and_seed = Vec::with_capacity(label.len() + seed.len());
    label_and_seed.extend_from_slice(label);
    label_and_seed.extend_from_slice(seed);

    // P_hash expansion
    let mut output = Vec::with_capacity(output_len);
    let mut a = label_and_seed.clone(); // A(0) = seed

    while output.len() < output_len {
        // A(i) = HMAC(secret, A(i-1))
        let mut mac = HmacSha256::new_from_slice(secret)
            .map_err(|e| format!("HMAC initialization failed: {e}"))?;
        mac.update(&a);
        a = mac.finalize().into_bytes().to_vec();

        // P_hash = HMAC(secret, A(i) + seed)
        let mut mac = HmacSha256::new_from_slice(secret)
            .map_err(|e| format!("HMAC initialization failed: {e}"))?;
        mac.update(&a);
        mac.update(&label_and_seed);
        let hmac_output = mac.finalize().into_bytes();

        output.extend_from_slice(&hmac_output);
    }

    output.truncate(output_len);
    Ok(output)
}

/// TLS 1.2 PRF with SHA-384
fn tls12_prf_sha384(
    secret: &[u8],
    label: &[u8],
    seed: &[u8],
    output_len: usize,
) -> Result<Vec<u8>, String> {
    use hmac::{Hmac, Mac};
    use sha2::Sha384;

    type HmacSha384 = Hmac<Sha384>;

    // Combine label + seed
    let mut label_and_seed = Vec::with_capacity(label.len() + seed.len());
    label_and_seed.extend_from_slice(label);
    label_and_seed.extend_from_slice(seed);

    // P_hash expansion
    let mut output = Vec::with_capacity(output_len);
    let mut a = label_and_seed.clone(); // A(0) = seed

    while output.len() < output_len {
        // A(i) = HMAC(secret, A(i-1))
        let mut mac = HmacSha384::new_from_slice(secret)
            .map_err(|e| format!("HMAC initialization failed: {e}"))?;
        mac.update(&a);
        a = mac.finalize().into_bytes().to_vec();

        // P_hash = HMAC(secret, A(i) + seed)
        let mut mac = HmacSha384::new_from_slice(secret)
            .map_err(|e| format!("HMAC initialization failed: {e}"))?;
        mac.update(&a);
        mac.update(&label_and_seed);
        let hmac_output = mac.finalize().into_bytes();

        output.extend_from_slice(&hmac_output);
    }

    output.truncate(output_len);
    Ok(output)
}

// =============================================================================
// Tests
// =============================================================================

#[cfg(test)]
#[path = "tls12_tests.rs"]
mod tests;
