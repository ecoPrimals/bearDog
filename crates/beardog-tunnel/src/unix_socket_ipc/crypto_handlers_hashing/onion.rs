// SPDX-License-Identifier: AGPL-3.0-or-later

//! Tor v3 onion address derivation and identity generation.

use base64::Engine;
use base64::engine::general_purpose::STANDARD as BASE64;
use beardog_errors::BearDogError;
use serde_json::{Value, json};
use sha2::Digest;
use sha3::Sha3_256;

/// # Errors
///
/// Returns an error if hashing fails.
/// Handle `beardog.crypto.derive_onion_address` - Tor v3 onion address derivation
pub fn handle_derive_onion_address(params: &Value) -> Result<Value, BearDogError> {
    const TOR_V3_VERSION: u8 = 0x03;

    let pubkey_b64 = params
        .get("public_key")
        .and_then(|v| v.as_str())
        .ok_or_else(|| BearDogError::invalid_input("Missing 'public_key' parameter"))?;

    let public_key = BASE64
        .decode(pubkey_b64)
        .map_err(|e| BearDogError::invalid_input(&format!("Invalid base64 public_key: {e}")))?;

    if public_key.len() != 32 {
        return Err(BearDogError::invalid_input(&format!(
            "Ed25519 public key must be 32 bytes, got {}",
            public_key.len()
        )));
    }

    let mut hasher = Sha3_256::new();
    hasher.update(b".onion checksum");
    hasher.update(&public_key);
    hasher.update([TOR_V3_VERSION]);
    let hash = hasher.finalize();
    let checksum = &hash[0..2];

    let mut onion_bytes = Vec::with_capacity(35);
    onion_bytes.extend_from_slice(&public_key);
    onion_bytes.extend_from_slice(checksum);
    onion_bytes.push(TOR_V3_VERSION);

    let onion_base32 = data_encoding::BASE32_NOPAD
        .encode(&onion_bytes)
        .to_lowercase();

    let onion_address = format!("{onion_base32}.onion");

    Ok(json!({
        "onion_address": onion_address,
        "public_key": pubkey_b64,
        "checksum": hex::encode(checksum),
        "version": u32::from(TOR_V3_VERSION)
    }))
}

/// # Errors
///
/// Returns an error if key derivation fails.
/// Handle `beardog.crypto.generate_onion_identity` - Generate Tor v3 onion identity
pub async fn handle_generate_onion_identity(
    params: Option<&Value>,
) -> Result<Value, crate::unix_socket_ipc::handlers::HandlerError> {
    use ed25519_dalek::SigningKey;
    use rand::RngCore;

    let purpose = params
        .and_then(|p| p.get("purpose"))
        .and_then(|v| v.as_str())
        .unwrap_or("hidden_service");

    let mut seed = [0u8; 32];
    rand::rng().fill_bytes(&mut seed);
    let signing_key = SigningKey::from_bytes(&seed);
    let verifying_key = signing_key.verifying_key();

    let secret_bytes = signing_key.to_bytes();
    let public_bytes = verifying_key.to_bytes();

    let onion_params = json!({
        "public_key": BASE64.encode(public_bytes)
    });
    let onion_result = handle_derive_onion_address(&onion_params)
        .map_err(|e| format!("Failed to derive onion address: {e}"))?;

    let onion_address = onion_result
        .get("onion_address")
        .and_then(|v| v.as_str())
        .ok_or("Failed to get onion address")?;

    let mut full_secret = Vec::with_capacity(64);
    full_secret.extend_from_slice(&secret_bytes);
    full_secret.extend_from_slice(&public_bytes);

    Ok(json!({
        "public_key": BASE64.encode(public_bytes),
        "secret_key": BASE64.encode(&full_secret),
        "onion_address": onion_address,
        "version": 3,
        "purpose": purpose
    }))
}
