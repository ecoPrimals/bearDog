// SPDX-License-Identifier: AGPL-3.0-or-later

//! Tor-specific KDF IPC handler.

use base64::Engine;
use base64::engine::general_purpose::STANDARD as BASE64;
use beardog_errors::BearDogError;
use serde_json::{Value, json};

use super::constants::NTOR_T_EXPAND;
use super::primitives::hkdf_expand;

/// Handle `beardog.crypto.tor_kdf` - Tor-specific key derivation.
///
/// # Errors
///
/// Returns an error on invalid seed data or KDF failure.
pub async fn handle_tor_kdf(params: Option<&Value>) -> Result<Value, BearDogError> {
    let params = params.ok_or_else(|| BearDogError::invalid_input("Missing parameters"))?;

    let seed_b64 = params
        .get("key_seed")
        .and_then(|v| v.as_str())
        .ok_or_else(|| BearDogError::invalid_input("Missing 'key_seed' parameter"))?;

    let key_seed = BASE64
        .decode(seed_b64)
        .map_err(|e| BearDogError::invalid_input(&format!("Invalid base64 key_seed: {e}")))?;

    #[expect(
        clippy::cast_possible_truncation,
        reason = "HKDF key count from JSON parameters"
    )]
    let key_count = params
        .get("key_count")
        .and_then(serde_json::Value::as_u64)
        .unwrap_or(4) as usize;

    #[expect(
        clippy::cast_possible_truncation,
        reason = "HKDF key byte length from JSON parameters"
    )]
    let key_length = params
        .get("key_length")
        .and_then(serde_json::Value::as_u64)
        .unwrap_or(20) as usize;

    let total_len = key_count * key_length;
    let expanded = hkdf_expand(&key_seed, NTOR_T_EXPAND, total_len)?;

    let keys: Vec<String> = expanded
        .chunks(key_length)
        .map(|chunk| BASE64.encode(chunk))
        .collect();

    Ok(json!({
        "keys": keys,
        "algorithm": "hkdf-sha256",
        "key_count": key_count,
        "key_length": key_length
    }))
}
