// SPDX-License-Identifier: AGPL-3.0-or-later

//! Asymmetric cryptography operations
//!
//! This module provides asymmetric cryptographic operations for ecoPrimals,
//! including digital signatures and key exchange using modern elliptic curves.
//!
//! # Overview
//!
//! Asymmetric cryptography uses key pairs (public + private) for:
//! - **Digital Signatures**: Ed25519 (`EdDSA` on Curve25519)
//! - **Key Exchange**: X25519 (ECDH on Curve25519)
//!
//! All operations use Pure Rust implementations from the `RustCrypto` ecosystem,
//! with zero C dependencies for universal cross-compilation.
//!
//! # Algorithms
//!
//! ## Ed25519 (Digital Signatures)
//!
//! - **Curve**: Curve25519 (Edwards form)
//! - **Signature Size**: 64 bytes
//! - **Public Key Size**: 32 bytes
//! - **Private Key Size**: 32 bytes (seed)
//! - **Security Level**: ~128-bit
//! - **Speed**: Very fast (~60,000 signs/sec, ~40,000 verifies/sec)
//!
//! ### Methods
//!
//! - [`handle_sign_ed25519`] - Sign a message with Ed25519
//! - [`handle_verify_ed25519`] - Verify an Ed25519 signature
//!
//! ## X25519 (Key Exchange)
//!
//! - **Curve**: Curve25519 (Montgomery form)
//! - **Shared Secret Size**: 32 bytes
//! - **Public Key Size**: 32 bytes
//! - **Private Key Size**: 32 bytes
//! - **Security Level**: ~128-bit
//! - **Speed**: Very fast (~10,000 exchanges/sec)
//!
//! ### Methods
//!
//! - [`handle_x25519_generate_ephemeral`] - Generate ephemeral X25519 keypair
//! - [`handle_x25519_derive_secret`] - Derive shared secret via ECDH
//!
//! # Usage
//!
//! All handlers are re-exported from the parent `crypto` module:
//!
//! ```rust,ignore
//! // NOTE: These handlers are internal and called via JSON-RPC
//! use crate::unix_socket_ipc::handlers::crypto::*;
//!
//! // Generate Ed25519 signature
//! let signature = handle_sign_ed25519(params).await?;
//!
//! // Verify Ed25519 signature
//! let is_valid = handle_verify_ed25519(params).await?;
//!
//! // Generate X25519 ephemeral keypair
//! let keypair = handle_x25519_generate_ephemeral(None).await?;
//!
//! // Derive X25519 shared secret
//! let shared_secret = handle_x25519_derive_secret(params).await?;
//! ```
//!
//! # References
//!
//! - RFC 8032 (EdDSA/Ed25519): <https://www.rfc-editor.org/rfc/rfc8032.html>
//! - RFC 7748 (X25519): <https://www.rfc-editor.org/rfc/rfc7748.html>
//! - Curve25519: <https://cr.yp.to/ecdh.html>

use base64::Engine;
use beardog_core::crypto_service::algorithms::asymmetric;
use ed25519_dalek::{Signer, SigningKey, VerifyingKey};
use rand::RngCore;
use serde_json::Value;
use tracing::{debug, info};
use x25519_dalek::{PublicKey as X25519PublicKey, StaticSecret};

// Import shared utility functions
use super::utils::derive_key_from_id;

/// Handle Ed25519 signature operations via JSON-RPC.
///
/// Supports two key-sourcing modes:
///
/// 1. **Direct key**: caller provides `secret_key` (base64-encoded 32-byte Ed25519
///    signing key seed). This is the recommended path for signing with a previously
///    generated keypair (e.g. from `crypto.ed25519_generate_keypair`).
///
/// 2. **Derived key**: caller provides `key_id` + optional `purpose`. The signing key
///    is deterministically derived from `BEARDOG_MASTER_KEY` via BLAKE3. Useful for
///    primal-identity signing where the key doesn't leave bearDog.
///
/// If `secret_key` is present, `key_id`/`purpose` are ignored.
///
/// # Errors
///
/// Returns an error if key decoding, derivation, or signing fails.
pub async fn handle_sign_ed25519(
    params: Option<&Value>,
) -> Result<Value, super::super::HandlerError> {
    let params = params.ok_or("Missing params for crypto.sign_ed25519")?;

    let message_b64 = params
        .get("message")
        .and_then(|v| v.as_str())
        .ok_or("Missing required parameter: message")?;

    let message = base64::engine::general_purpose::STANDARD
        .decode(message_b64)
        .map_err(|e| format!("Invalid base64 message: {e}"))?;

    let b64 = &base64::engine::general_purpose::STANDARD;

    // Mode 1: direct secret_key provided by caller
    if let Some(secret_key_b64) = params.get("secret_key").and_then(|v| v.as_str()) {
        let seed_bytes = b64
            .decode(secret_key_b64)
            .map_err(|e| format!("Invalid base64 secret_key: {e}"))?;
        let seed: [u8; 32] = seed_bytes
            .as_slice()
            .try_into()
            .map_err(|_| "secret_key must be exactly 32 bytes".to_string())?;

        let signing_key = SigningKey::from_bytes(&seed);
        let verifying_key: VerifyingKey = (&signing_key).into();

        debug!("🔐 Signing {} bytes with Ed25519 (direct key)", message.len());

        let signature = signing_key.sign(&message).to_bytes().to_vec();

        let signature_b64 = b64.encode(&signature);
        let public_key_b64 = b64.encode(verifying_key.as_bytes());

        info!("✅ Ed25519 signature generated ({} bytes, direct key)", signature.len());

        return Ok(serde_json::json!({
            "signature": signature_b64,
            "algorithm": "Ed25519",
            "public_key": public_key_b64,
        }));
    }

    // Mode 2: derive key from key_id + purpose
    let key_id = params
        .get("key_id")
        .and_then(|v| v.as_str())
        .unwrap_or("default_signing_key");

    let purpose = params
        .get("purpose")
        .and_then(|v| v.as_str())
        .unwrap_or("general");

    debug!(
        "🔐 Signing {} bytes with Ed25519 (key_id: {}, purpose: {})",
        message.len(),
        key_id,
        purpose
    );

    let seed = derive_key_from_id(key_id, purpose)?;
    let (secret_key, public_key) = asymmetric::generate_ed25519_from_seed(&seed)
        .map_err(|e| format!("Failed to generate Ed25519 keypair: {e}"))?;

    let signature = asymmetric::sign_ed25519(&message, &secret_key)
        .map_err(|e| format!("Ed25519 signing failed: {e}"))?;

    let signature_b64 = b64.encode(&signature);
    let public_key_b64 = b64.encode(public_key);

    info!("✅ Ed25519 signature generated ({} bytes)", signature.len());

    Ok(serde_json::json!({
        "signature": signature_b64,
        "algorithm": "Ed25519",
        "key_id": key_id,
        "public_key": public_key_b64,
    }))
}

/// Retrieve the Ed25519 public key for a given `key_id` without signing.
///
/// # Parameters
///
/// - `key_id`: Key identifier (optional, defaults to `"default_signing_key"`)
/// - `purpose`: Purpose string (optional, defaults to `"general"`)
///
/// # Returns
///
/// - `public_key`: Base64-encoded Ed25519 public key (32 bytes)
/// - `algorithm`: `"Ed25519"`
/// - `key_id`: The key identifier used
///
/// # Errors
///
/// Returns an error if key derivation fails.
pub async fn handle_public_key(
    params: Option<&Value>,
) -> Result<Value, super::super::HandlerError> {
    let key_id = params
        .and_then(|p| p.get("key_id"))
        .and_then(|v| v.as_str())
        .unwrap_or("default_signing_key");

    let purpose = params
        .and_then(|p| p.get("purpose"))
        .and_then(|v| v.as_str())
        .unwrap_or("general");

    let seed = derive_key_from_id(key_id, purpose)?;
    let (_secret_key, public_key) = asymmetric::generate_ed25519_from_seed(&seed)
        .map_err(|e| format!("Failed to derive Ed25519 public key: {e}"))?;

    let public_key_b64 = base64::engine::general_purpose::STANDARD.encode(public_key);

    Ok(serde_json::json!({
        "public_key": public_key_b64,
        "algorithm": "Ed25519",
        "key_id": key_id,
    }))
}

/// Derive a `did:key` identifier from a `BearDog` Ed25519 signing key.
///
/// Returns a W3C DID using the `did:key` method (multicodec Ed25519 prefix `0xed01`
/// + 32-byte public key, base58btc-encoded with `z` multibase prefix).
///
/// # Parameters
///
/// - `key_id`: Key identifier (optional, defaults to `"default_signing_key"`)
/// - `purpose`: Purpose string (optional, defaults to `"general"`)
///
/// # Returns
///
/// - `did`: The `did:key:z6Mk...` string
/// - `public_key`: Base64-encoded Ed25519 public key (32 bytes)
/// - `algorithm`: `"Ed25519"`
/// - `key_id`: The key identifier used
///
/// # Errors
///
/// Returns an error if key derivation fails.
pub async fn handle_did_from_key(
    params: Option<&Value>,
) -> Result<Value, super::super::HandlerError> {
    let key_id = params
        .and_then(|p| p.get("key_id"))
        .and_then(|v| v.as_str())
        .unwrap_or("default_signing_key");

    let purpose = params
        .and_then(|p| p.get("purpose"))
        .and_then(|v| v.as_str())
        .unwrap_or("general");

    let seed = derive_key_from_id(key_id, purpose)?;
    let (_secret_key, public_key) = asymmetric::generate_ed25519_from_seed(&seed)
        .map_err(|e| format!("Failed to derive Ed25519 public key: {e}"))?;

    // did:key multicodec: 0xed = Ed25519 public key, varint-encoded as [0xed, 0x01]
    let mut multicodec_bytes = Vec::with_capacity(34);
    multicodec_bytes.push(0xed);
    multicodec_bytes.push(0x01);
    multicodec_bytes.extend_from_slice(&public_key);

    // Multibase: 'z' prefix = base58btc
    let did = format!("did:key:z{}", bs58::encode(&multicodec_bytes).into_string());
    let public_key_b64 = base64::engine::general_purpose::STANDARD.encode(public_key);

    info!("🆔 Crypto: did_from_key → {did}");

    Ok(serde_json::json!({
        "did": did,
        "public_key": public_key_b64,
        "algorithm": "Ed25519",
        "key_id": key_id,
    }))
}

/// Decode an evidence/payload string according to the `ATTESTATION_ENCODING_STANDARD.md`
/// encoding values: `base64` (default), `hex`, `base64url`, `utf8`, `none`.
fn decode_with_encoding(
    encoded: &str,
    encoding: &str,
    field_name: &str,
) -> Result<Vec<u8>, String> {
    match encoding {
        "base64" => base64::engine::general_purpose::STANDARD
            .decode(encoded)
            .map_err(|e| format!("Invalid base64 {field_name}: {e}")),
        "base64url" => base64::engine::general_purpose::URL_SAFE
            .decode(encoded)
            .map_err(|e| format!("Invalid base64url {field_name}: {e}")),
        "hex" => {
            let hex_str = encoded.strip_prefix("0x").unwrap_or(encoded);
            hex::decode(hex_str).map_err(|e| format!("Invalid hex {field_name}: {e}"))
        }
        "utf8" => Ok(encoded.as_bytes().to_vec()),
        "none" => Ok(Vec::new()),
        other => Err(format!(
            "Unsupported encoding '{other}' for {field_name} \
             (expected: base64, base64url, hex, utf8, none)"
        )),
    }
}

/// Handle `crypto.verify_ed25519` / `crypto.ed25519.verify` method.
///
/// Verifies an Ed25519 signature. Accepts encoding hints per
/// `ATTESTATION_ENCODING_STANDARD.md` `WireWitnessRef` wire type,
/// so callers do not need to normalize witness evidence before verification.
///
/// # Parameters
///
/// - `message`: Encoded message that was signed
/// - `signature`: Encoded Ed25519 signature (64 bytes decoded)
/// - `public_key`: Encoded Ed25519 public key (32 bytes decoded)
/// - `encoding` *(optional)*: Default encoding for all fields.
///   One of `base64` (default), `hex`, `base64url`, `utf8`, `none`.
/// - `message_encoding` *(optional)*: Per-field override for message.
/// - `signature_encoding` *(optional)*: Per-field override for signature.
/// - `public_key_encoding` *(optional)*: Per-field override for public key.
///
/// Per-field hints take precedence over the shared `encoding` field,
/// allowing callers to mix encodings (e.g. hex public key with base64
/// signature from different witness sources).  Resolves BD-01.
///
/// # Returns
///
/// - `valid`: Boolean indicating if signature is valid
/// - `algorithm`: `"Ed25519"`
///
/// # Errors
///
/// Returns an error if decoding or verification fails.
pub async fn handle_verify_ed25519(
    params: Option<&Value>,
) -> Result<Value, super::super::HandlerError> {
    let params = params.ok_or("Missing params for crypto.verify_ed25519")?;

    let default_encoding = params
        .get("encoding")
        .and_then(|v| v.as_str())
        .unwrap_or("base64");

    let msg_encoding = params
        .get("message_encoding")
        .and_then(|v| v.as_str())
        .unwrap_or(default_encoding);

    let sig_encoding = params
        .get("signature_encoding")
        .and_then(|v| v.as_str())
        .unwrap_or(default_encoding);

    let pk_encoding = params
        .get("public_key_encoding")
        .and_then(|v| v.as_str())
        .unwrap_or(default_encoding);

    let message_enc = params
        .get("message")
        .and_then(|v| v.as_str())
        .ok_or("Missing required parameter: message")?;

    let signature_enc = params
        .get("signature")
        .and_then(|v| v.as_str())
        .ok_or("Missing required parameter: signature")?;

    let public_key_enc = params
        .get("public_key")
        .and_then(|v| v.as_str())
        .ok_or("Missing required parameter: public_key")?;

    let message = decode_with_encoding(message_enc, msg_encoding, "message")?;
    let signature = decode_with_encoding(signature_enc, sig_encoding, "signature")?;
    let public_key = decode_with_encoding(public_key_enc, pk_encoding, "public_key")?;

    debug!(
        "Verifying Ed25519 signature ({} bytes message, {} bytes signature, \
         encoding: {default_encoding}, overrides: msg={msg_encoding} sig={sig_encoding} pk={pk_encoding})",
        message.len(),
        signature.len()
    );

    let valid = asymmetric::verify_ed25519(&message, &signature, &public_key)
        .map_err(|e| format!("Ed25519 verification failed: {e}"))?;

    info!("Ed25519 signature verification: {valid}");

    Ok(serde_json::json!({
        "valid": valid,
        "algorithm": "Ed25519",
    }))
}

/// # Errors
///
/// Returns an error if key generation fails in the underlying HSM provider.
/// Handle `crypto.ed25519_generate_keypair` method
///
/// Generates a new Ed25519 keypair for signing/identity.
/// Used for onion identity generation in the ecosystem.
///
/// # Parameters
///
/// - `purpose`: Purpose string (optional, for logging)
///
/// # Returns
///
/// - `public_key`: Base64-encoded Ed25519 public key (32 bytes)
/// - `secret_key`: Base64-encoded Ed25519 secret key (32 bytes)
/// - `algorithm`: "Ed25519"
pub async fn handle_ed25519_generate_keypair(
    params: Option<&Value>,
) -> Result<Value, super::super::HandlerError> {
    let purpose = params
        .and_then(|p| p.get("purpose"))
        .and_then(|v| v.as_str())
        .unwrap_or("identity");

    debug!("🔑 Generating Ed25519 keypair (purpose: {})", purpose);

    // Generate a random 32-byte seed
    let mut seed_bytes = [0u8; 32];
    rand::rng().fill_bytes(&mut seed_bytes);

    // Create signing key from the seed
    let signing_key = SigningKey::from_bytes(&seed_bytes);
    let verifying_key: VerifyingKey = (&signing_key).into();

    // Encode keys
    let public_key_b64 = base64::engine::general_purpose::STANDARD.encode(verifying_key.as_bytes());
    let secret_key_b64 = base64::engine::general_purpose::STANDARD.encode(signing_key.as_bytes());

    info!("✅ Ed25519 keypair generated (purpose: {})", purpose);

    Ok(serde_json::json!({
        "public_key": public_key_b64,
        "secret_key": secret_key_b64,
        "algorithm": "Ed25519",
    }))
}

/// # Errors
///
/// Returns an error if key derivation fails.
/// Handle `crypto.x25519_generate_ephemeral` method
///
/// Generates an ephemeral X25519 keypair for key exchange.
///
/// # Parameters
///
/// - `purpose`: Purpose string (optional, for logging)
///
/// # Returns
///
/// - `public_key`: Base64-encoded X25519 public key (32 bytes)
/// - `secret_key`: Base64-encoded X25519 secret key (32 bytes)
pub async fn handle_x25519_generate_ephemeral(
    params: Option<&Value>,
) -> Result<Value, super::super::HandlerError> {
    let purpose = params
        .and_then(|p| p.get("purpose"))
        .and_then(|v| v.as_str())
        .unwrap_or("key_exchange");

    debug!(
        "🔑 Generating ephemeral X25519 keypair (purpose: {})",
        purpose
    );

    // Generate a random 32-byte secret
    let mut secret_bytes = [0u8; 32];
    rand::rng().fill_bytes(&mut secret_bytes);

    // Create keypair from the secret
    let secret = StaticSecret::from(secret_bytes);
    let public = X25519PublicKey::from(&secret);

    // Encode keys
    let public_key_b64 = base64::engine::general_purpose::STANDARD.encode(public.as_bytes());
    let secret_key_b64 = base64::engine::general_purpose::STANDARD.encode(secret_bytes);

    info!("✅ Ephemeral X25519 keypair generated");

    Ok(serde_json::json!({
        "public_key": public_key_b64,
        "secret_key": secret_key_b64,
        "algorithm": "X25519",
    }))
}

/// Semantic `crypto.generate_keypair` with HSM backend selection.
///
/// Routes key generation through the `HsmProviderRegistry` when a hardware
/// backend is requested. On platforms with Android StrongBox/Titan M2, the
/// registry selects the hardware provider; otherwise falls back to software.
///
/// `titan_m2` is an alias for `strongbox` — on Pixel devices, `StrongBox` IS
/// backed by Titan M2 via the same Android Keystore API.
///
/// # Errors
///
/// Returns an error when the requested HSM backend is unavailable or unknown.
pub async fn handle_generate_keypair_with_hsm(
    params: Option<&Value>,
) -> Result<Value, super::super::HandlerError> {
    use crate::tunnel::hsm::providers::registry::HsmProviderRegistry;
    use beardog_traits::hsm::HsmKeyProvider;
    use beardog_types::hsm::{HsmAlgorithm, KeyGenParams, SelectionPreference};

    let hsm_backend = params
        .and_then(|p| p.get("hsm_backend"))
        .and_then(|v| v.as_str());

    let algorithm_str = params
        .and_then(|p| p.get("algorithm"))
        .and_then(|v| v.as_str())
        .unwrap_or("X25519");

    let algorithm = match algorithm_str {
        "X25519" | "x25519" => HsmAlgorithm::X25519,
        "Ed25519" | "ed25519" => HsmAlgorithm::Ed25519,
        "AES-256-GCM" | "aes256gcm" => HsmAlgorithm::Aes256Gcm,
        "ChaCha20-Poly1305" | "chacha20poly1305" => HsmAlgorithm::ChaCha20Poly1305,
        "ECDSA-P256" | "ecdsa_p256" => HsmAlgorithm::EcdsaP256,
        _ => return Err(format!("unsupported algorithm: {algorithm_str}").into()),
    };

    match hsm_backend {
        None | Some("software") => handle_x25519_generate_ephemeral(params).await,

        Some(backend @ ("strongbox" | "titan_m2")) => {
            let registry = HsmProviderRegistry::discover().await;
            let preference = SelectionPreference::PreferHardware;

            match registry.select(preference) {
                Ok(provider) => {
                    let label = params
                        .and_then(|p| p.get("label"))
                        .and_then(|v| v.as_str())
                        .map(String::from);
                    let gen_params = KeyGenParams {
                        algorithm,
                        label,
                        extractable: false,
                    };

                    match provider.generate_key(&gen_params).await {
                        Ok(handle) => Ok(serde_json::json!({
                            "key_id": handle.key_id,
                            "algorithm": algorithm_str,
                            "hardware_backed": handle.hardware_backed,
                            "hsm_backend": backend,
                            "provider_id": provider.provider_id(),
                            "created_at_ms": handle.created_at_ms,
                        })),
                        Err(e) => Err(serde_json::to_string(&serde_json::json!({
                            "error": "hsm_key_generation_failed",
                            "message": e.to_string(),
                            "hsm_backend_requested": backend,
                            "provider_id": provider.provider_id(),
                        }))
                        .unwrap_or_else(|e| format!("hsm_key_generation_failed: {e}"))
                        .into()),
                    }
                }
                Err(e) => Err(serde_json::to_string(&serde_json::json!({
                    "error": "hsm_backend_not_available",
                    "message": e.to_string(),
                    "hsm_backend_requested": backend,
                    "available_backends": registry.iter()
                        .map(|p| p.provider_id())
                        .collect::<Vec<_>>(),
                }))
                .unwrap_or_else(|e| format!("hsm_backend_not_available: {e}"))
                .into()),
            }
        }

        Some(other) => Err(serde_json::to_string(&serde_json::json!({
            "error": "hsm_backend_unknown",
            "message": format!("Unknown hsm_backend: {other}"),
            "hsm_backend_requested": other,
            "available_backends": ["software", "strongbox", "titan_m2"],
        }))
        .unwrap_or_else(|e| format!("hsm_backend_unknown: {e}"))
        .into()),
    }
}

/// # Errors
///
/// Returns an error if key derivation fails.
/// Handle `crypto.x25519_derive_secret` method
///
/// Derives a shared secret using X25519 Diffie-Hellman.
///
/// # Parameters
///
/// - `our_secret`: Base64-encoded our X25519 secret key (32 bytes)
/// - `their_public`: Base64-encoded their X25519 public key (32 bytes)
///
/// # Returns
///
/// - `shared_secret`: Base64-encoded shared secret (32 bytes)
pub async fn handle_x25519_derive_secret(
    params: Option<&Value>,
) -> Result<Value, super::super::HandlerError> {
    let params = params.ok_or("Missing params for crypto.x25519_derive_secret")?;

    // Extract parameters
    let our_secret_b64 = params
        .get("our_secret")
        .and_then(|v| v.as_str())
        .ok_or("Missing required parameter: our_secret")?;

    let their_public_b64 = params
        .get("their_public")
        .and_then(|v| v.as_str())
        .ok_or("Missing required parameter: their_public")?;

    // Decode parameters
    let our_secret_bytes = base64::engine::general_purpose::STANDARD
        .decode(our_secret_b64)
        .map_err(|e| format!("Invalid base64 our_secret: {e}"))?;

    let their_public_bytes = base64::engine::general_purpose::STANDARD
        .decode(their_public_b64)
        .map_err(|e| format!("Invalid base64 their_public: {e}"))?;

    // Convert to fixed-size arrays
    let our_secret: [u8; 32] = our_secret_bytes
        .as_slice()
        .try_into()
        .map_err(|_| "our_secret must be 32 bytes".to_string())?;

    let their_public: [u8; 32] = their_public_bytes
        .as_slice()
        .try_into()
        .map_err(|_| "their_public must be 32 bytes".to_string())?;

    debug!("🤝 Deriving X25519 shared secret");

    // Create secret key from bytes
    let secret_key = StaticSecret::from(our_secret);
    let public = X25519PublicKey::from(their_public);

    let shared_secret = secret_key.diffie_hellman(&public);

    // Encode shared secret
    let shared_secret_b64 =
        base64::engine::general_purpose::STANDARD.encode(shared_secret.as_bytes());

    info!("✅ X25519 shared secret derived");

    Ok(serde_json::json!({
        "shared_secret": shared_secret_b64,
        "algorithm": "X25519",
    }))
}

#[cfg(test)]
#[path = "asymmetric_tests.rs"]
mod tests;
