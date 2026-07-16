// SPDX-License-Identifier: AGPL-3.0-or-later

//! NUCLEUS purpose-key operations and signed registration.
//!
//! Implements the NUCLEUS two-tier crypto model:
//! - `crypto.derive_purpose_key` — derive purpose-specific keys from a parent key
//! - `crypto.derive_public_key` — derive Ed25519 public key for a purpose from `FAMILY_SEED`
//! - `crypto.sign_registration` — sign `ipc.register` payloads for verifiable registrations
//! - `crypto.encrypt` / `crypto.decrypt` with `purpose` param — purpose-key envelope operations

use base64::Engine;
use base64::engine::general_purpose::STANDARD as BASE64;
use beardog_config::env_keys;
use beardog_core::crypto_service::algorithms::hashing;
use chacha20poly1305::{
    ChaCha20Poly1305,
    aead::{Aead, AeadCore, KeyInit, OsRng},
};
use serde_json::{Value, json};
use tracing::info;

use crate::unix_socket_ipc::handlers::crypto::handle_sign_ed25519;
use ed25519_dalek::SigningKey;

/// Derive a purpose-specific key from a parent key using the NUCLEUS convention.
///
/// Wire: `crypto.derive_purpose_key`
///
/// ```text
/// purpose_key = HMAC-SHA256(key, hex("purpose-v1:" + purpose))
/// ```
///
/// # Parameters
///
/// - `key`: Base64-encoded parent key (typically a family key)
/// - `purpose`: Purpose string (e.g. `"storage"`, `"dag"`, `"security"`)
///
/// # Returns
///
/// - `key`: Base64-encoded derived 32-byte purpose key
/// - `purpose`: Echo of the purpose string
/// - `method`: `"HMAC-SHA256-purpose-v1"`
pub async fn handle_derive_purpose_key(
    params: Option<&Value>,
) -> Result<Value, super::super::HandlerError> {
    let params = params.ok_or("Missing params for crypto.derive_purpose_key")?;

    let key_b64 = params
        .get("key")
        .and_then(|v| v.as_str())
        .ok_or("Missing required parameter: key (base64-encoded parent key)")?;

    let purpose = params
        .get("purpose")
        .and_then(|v| v.as_str())
        .ok_or("Missing required parameter: purpose")?;

    let key = BASE64
        .decode(key_b64)
        .map_err(|e| format!("Invalid base64 key: {e}"))?;

    let msg = hex::encode(format!("purpose-v1:{purpose}"));
    let msg_bytes = msg.as_bytes();

    let derived = hashing::hmac_sha256(&key, msg_bytes)
        .map_err(|e| format!("HMAC-SHA256 purpose derivation failed: {e}"))?;

    let derived_b64 = BASE64.encode(&derived);

    info!("✅ Derived purpose key for '{}' (32 bytes)", purpose);

    Ok(json!({
        "key": derived_b64,
        "purpose": purpose,
        "method": "HMAC-SHA256-purpose-v1",
    }))
}

/// Derive a purpose-specific Ed25519 public key from `FAMILY_SEED`.
///
/// Wire: `crypto.derive_public_key`
///
/// Combines purpose-key derivation with Ed25519 keypair generation:
/// 1. `purpose_key = HMAC-SHA256(FAMILY_SEED, hex("purpose-v1:" + purpose))`
/// 2. `signing_key = Ed25519::from_bytes(purpose_key)`
/// 3. Return the verifying (public) key.
///
/// # Parameters
///
/// - `purpose`: Purpose string (e.g. `"coordination"`, `"storage"`, `"inference"`)
///
/// # Returns
///
/// - `public_key`: Base64-encoded Ed25519 public key (32 bytes)
/// - `algorithm`: `"Ed25519"`
/// - `purpose`: Echo of the purpose string
/// - `derivation`: `"HMAC-SHA256-purpose-v1 → Ed25519"`
pub async fn handle_derive_public_key(
    params: Option<&Value>,
) -> Result<Value, super::super::HandlerError> {
    let params = params.ok_or(
        "Missing params for crypto.derive_public_key — expected: {\"purpose\": \"<string>\"}",
    )?;

    let purpose = params
        .get("purpose")
        .and_then(|v| v.as_str())
        .ok_or("Missing required parameter: purpose (e.g. \"coordination\", \"storage\")")?;

    let purpose_key = resolve_purpose_key(purpose)?;

    let signing_key = SigningKey::from_bytes(&purpose_key);
    let public_key = signing_key.verifying_key();
    let public_key_b64 = BASE64.encode(public_key.as_bytes());

    info!(
        "🔑 Derived public key for purpose '{}' (Ed25519, 32 bytes)",
        purpose
    );

    Ok(json!({
        "public_key": public_key_b64,
        "algorithm": "Ed25519",
        "purpose": purpose,
        "derivation": "HMAC-SHA256-purpose-v1 → Ed25519",
    }))
}

/// Sign an `ipc.register` payload so consumers can verify authentic registrations.
///
/// Wire: `crypto.sign_registration`
///
/// Canonicalizes the registration fields (`primal_id`, `capabilities`, `endpoint`)
/// into a deterministic byte string and signs it with Ed25519.
///
/// # Parameters
///
/// - `primal_id`: Primal identifier
/// - `capabilities`: JSON array of capability strings
/// - `endpoint`: UDS endpoint path
/// - `key_id` (optional): Ed25519 key identifier (defaults to `"default_signing_key"`)
///
/// # Returns
///
/// - `signature`: Base64-encoded Ed25519 signature
/// - `public_key`: Base64-encoded Ed25519 public key (for verification)
/// - `canonical`: The canonical string that was signed
/// - `algorithm`: `"Ed25519"`
pub async fn handle_sign_registration(
    params: Option<&Value>,
) -> Result<Value, super::super::HandlerError> {
    let params = params.ok_or("Missing params for crypto.sign_registration")?;

    let primal_id = params
        .get("primal_id")
        .and_then(|v| v.as_str())
        .ok_or("Missing required parameter: primal_id")?;

    let capabilities = params
        .get("capabilities")
        .and_then(|v| v.as_array())
        .ok_or("Missing required parameter: capabilities (JSON array)")?;

    let endpoint = params
        .get("endpoint")
        .and_then(|v| v.as_str())
        .ok_or("Missing required parameter: endpoint")?;

    let key_id = params
        .get("key_id")
        .and_then(|v| v.as_str())
        .unwrap_or("default_signing_key");

    let mut caps: Vec<&str> = capabilities.iter().filter_map(|v| v.as_str()).collect();
    caps.sort_unstable();

    let canonical = format!(
        "ipc.register-v1:primal_id={},capabilities=[{}],endpoint={}",
        primal_id,
        caps.join(","),
        endpoint
    );

    let msg_b64 = BASE64.encode(canonical.as_bytes());
    let sign_params = json!({
        "message": msg_b64,
        "key_id": key_id,
        "purpose": "registration",
    });

    let sig_result = handle_sign_ed25519(Some(&sign_params)).await?;

    info!(
        "✅ Signed registration for '{}' ({} capabilities)",
        primal_id,
        caps.len()
    );

    Ok(json!({
        "signature": sig_result.get("signature").and_then(|v| v.as_str()),
        "public_key": sig_result.get("public_key").and_then(|v| v.as_str()),
        "canonical": canonical,
        "algorithm": "Ed25519",
    }))
}

/// Resolve a NUCLEUS purpose key from `FAMILY_SEED` / `BEARDOG_FAMILY_SEED`.
///
/// Same HMAC-SHA256 convention as `handle_derive_purpose_key`:
/// `purpose_key = HMAC-SHA256(family_seed, hex("purpose-v1:" + purpose))`
fn resolve_purpose_key(purpose: &str) -> Result<[u8; 32], super::super::HandlerError> {
    let family_seed = load_family_seed()?;

    let msg = hex::encode(format!("purpose-v1:{purpose}"));
    let derived = hashing::hmac_sha256(&family_seed, msg.as_bytes())
        .map_err(|e| format!("HMAC-SHA256 purpose derivation failed: {e}"))?;

    let key: [u8; 32] = derived
        .as_slice()
        .try_into()
        .map_err(|_| "Purpose key derivation produced unexpected length".to_string())?;

    Ok(key)
}

fn load_family_seed() -> Result<Vec<u8>, super::super::HandlerError> {
    if let Ok(seed) = beardog_errors::process_env::var(env_keys::ENV_FAMILY_SEED_PREFIXED)
        && !seed.is_empty()
    {
        return Ok(seed.into_bytes());
    }
    if let Ok(seed) = beardog_errors::process_env::var(env_keys::ENV_FAMILY_SEED)
        && !seed.is_empty()
    {
        return Ok(seed.into_bytes());
    }
    Err(
        "Purpose-based encrypt/decrypt requires FAMILY_SEED or BEARDOG_FAMILY_SEED env var"
            .to_string()
            .into(),
    )
}

/// Encrypt with a NUCLEUS purpose key, returning the standard envelope.
///
/// Wire: `crypto.encrypt` with `purpose` param
///
/// # Parameters
///
/// - `data`: Base64-encoded plaintext
/// - `purpose`: Purpose string (e.g. `"storage"`, `"inference"`)
///
/// # Returns
///
/// NUCLEUS standard envelope: `{"v":1,"ct":"<b64>","n":"<b64>","alg":"chacha20-poly1305"}`
pub async fn handle_purpose_encrypt(
    params: Option<&Value>,
) -> Result<Value, super::super::HandlerError> {
    let params = params.ok_or("Missing params for crypto.encrypt with purpose")?;

    let data_b64 = params
        .get("data")
        .and_then(|v| v.as_str())
        .or_else(|| params.get("plaintext").and_then(|v| v.as_str()))
        .ok_or("Missing required parameter: data (base64-encoded plaintext)")?;

    let purpose = params
        .get("purpose")
        .and_then(|v| v.as_str())
        .ok_or("Missing required parameter: purpose")?;

    let plaintext = BASE64
        .decode(data_b64)
        .map_err(|e| format!("Invalid base64 data: {e}"))?;

    let key = resolve_purpose_key(purpose)?;
    let cipher = ChaCha20Poly1305::new(&key.into());
    let nonce = ChaCha20Poly1305::generate_nonce(OsRng);

    // AEAD output = ciphertext || tag (standard NUCLEUS envelope format)
    let ciphertext = cipher
        .encrypt(&nonce, plaintext.as_ref())
        .map_err(|e| format!("ChaCha20-Poly1305 encryption failed: {e}"))?;

    info!(
        "✅ Purpose-key encrypt: purpose='{}', {} bytes → {} bytes",
        purpose,
        plaintext.len(),
        ciphertext.len()
    );

    Ok(json!({
        "v": 1,
        "ct": BASE64.encode(&ciphertext),
        "n": BASE64.encode(nonce),
        "alg": "chacha20-poly1305",
        "purpose": purpose,
    }))
}

/// Decrypt a NUCLEUS purpose-key envelope.
///
/// Wire: `crypto.decrypt` with `purpose` param
///
/// # Parameters
///
/// Accepts either the NUCLEUS envelope or flat params:
/// - `purpose`: Purpose string (required)
/// - `ct` or `ciphertext`: Base64-encoded ciphertext
/// - `n` or `nonce`: Base64-encoded nonce
///
/// # Returns
///
/// - `plaintext`: Base64-encoded decrypted data
/// - `algorithm`: `"chacha20-poly1305"`
pub async fn handle_purpose_decrypt(
    params: Option<&Value>,
) -> Result<Value, super::super::HandlerError> {
    let params = params.ok_or("Missing params for crypto.decrypt with purpose")?;

    let purpose = params
        .get("purpose")
        .and_then(|v| v.as_str())
        .ok_or("Missing required parameter: purpose")?;

    let ct_b64 = params
        .get("ct")
        .or_else(|| params.get("ciphertext"))
        .and_then(|v| v.as_str())
        .ok_or("Missing required parameter: ct (base64-encoded ciphertext)")?;

    let nonce_b64 = params
        .get("n")
        .or_else(|| params.get("nonce"))
        .and_then(|v| v.as_str())
        .ok_or("Missing required parameter: n (base64-encoded nonce)")?;

    let ciphertext = BASE64
        .decode(ct_b64)
        .map_err(|e| format!("Invalid base64 ciphertext: {e}"))?;

    let nonce_bytes = BASE64
        .decode(nonce_b64)
        .map_err(|e| format!("Invalid base64 nonce: {e}"))?;

    if nonce_bytes.len() != 12 {
        return Err(format!(
            "Invalid nonce length: expected 12, got {}",
            nonce_bytes.len()
        )
        .into());
    }

    let key = resolve_purpose_key(purpose)?;
    let cipher = ChaCha20Poly1305::new(&key.into());
    let nonce = chacha20poly1305::Nonce::from_slice(&nonce_bytes);

    let plaintext = cipher
        .decrypt(nonce, ciphertext.as_ref())
        .map_err(|e| format!("ChaCha20-Poly1305 decryption failed: {e}"))?;

    info!(
        "✅ Purpose-key decrypt: purpose='{}', {} bytes → {} bytes",
        purpose,
        ciphertext.len(),
        plaintext.len()
    );

    Ok(json!({
        "plaintext": BASE64.encode(&plaintext),
        "algorithm": "chacha20-poly1305",
        "purpose": purpose,
    }))
}

/// Compute a stable, non-reversible fingerprint of the primal's identity seed.
///
/// Wire: `crypto.seed_fingerprint`
///
/// The fingerprint is `BLAKE3(HMAC-SHA256(family_seed, "seed-fingerprint-v1"))`,
/// truncated to 16 bytes and hex-encoded. This allows Tower atomic validation
/// to verify that a primal's seed is consistent without exposing the seed itself.
///
/// # Parameters
///
/// - `algorithm` (optional): fingerprint hash algorithm, default `"blake3"`
///
/// # Returns
///
/// - `fingerprint`: hex-encoded seed fingerprint (32 chars)
/// - `algorithm`: `"BLAKE3-over-HMAC-SHA256"`
/// - `version`: `"seed-fingerprint-v1"`
///
/// # Errors
///
/// Returns an error if `FAMILY_SEED` / `BEARDOG_FAMILY_SEED` is not set.
pub async fn handle_seed_fingerprint(
    params: Option<&Value>,
) -> Result<Value, super::super::HandlerError> {
    let _ = params;

    let seed = load_family_seed()?;

    let hmac_output = hashing::hmac_sha256(&seed, b"seed-fingerprint-v1")
        .map_err(|e| format!("HMAC for seed fingerprint failed: {e}"))?;

    let fingerprint_full = blake3::hash(&hmac_output);
    let fingerprint_hex = hex::encode(&fingerprint_full.as_bytes()[..16]);

    info!("Seed fingerprint computed (32 hex chars)");

    Ok(json!({
        "fingerprint": fingerprint_hex,
        "algorithm": "BLAKE3-over-HMAC-SHA256",
        "version": "seed-fingerprint-v1",
    }))
}

#[cfg(test)]
#[path = "purpose_key_tests.rs"]
mod tests;
