// SPDX-License-Identifier: AGPL-3.0-or-later

//! Tor relay cell encryption (`ChaCha20` counter mode).

use base64::Engine;
use base64::engine::general_purpose::STANDARD as BASE64;
use beardog_errors::BearDogError;
use serde_json::{Value, json};

use super::primitives::chacha20_counter_mode;

/// Handle `beardog.crypto.tor_cell_encrypt` - Encrypt Tor relay cell.
///
/// # Errors
///
/// Returns an error on invalid parameters or encryption failure.
pub async fn handle_tor_cell_encrypt(params: Option<&Value>) -> Result<Value, BearDogError> {
    let params = params.ok_or_else(|| BearDogError::invalid_input("Missing parameters"))?;

    let key_b64 = params
        .get("key")
        .and_then(|v| v.as_str())
        .ok_or_else(|| BearDogError::invalid_input("Missing 'key' parameter"))?;

    let key = BASE64
        .decode(key_b64)
        .map_err(|e| BearDogError::invalid_input(&format!("Invalid base64 key: {e}")))?;

    if key.len() != 32 {
        return Err(BearDogError::invalid_input(&format!(
            "key must be 32 bytes, got {}",
            key.len()
        )));
    }

    let counter = params
        .get("counter")
        .and_then(serde_json::Value::as_u64)
        .ok_or_else(|| BearDogError::invalid_input("Missing 'counter' parameter"))?;

    let data_b64 = params
        .get("data")
        .and_then(|v| v.as_str())
        .ok_or_else(|| BearDogError::invalid_input("Missing 'data' parameter"))?;

    let mut data = BASE64
        .decode(data_b64)
        .map_err(|e| BearDogError::invalid_input(&format!("Invalid base64 data: {e}")))?;

    chacha20_counter_mode(&key, counter, &mut data)?;

    Ok(json!({
        "ciphertext": BASE64.encode(&data),
        "next_counter": counter + 1,
        "algorithm": "chacha20-counter"
    }))
}

/// Handle `beardog.crypto.tor_cell_decrypt` - Decrypt Tor relay cell.
///
/// # Errors
///
/// Returns an error on invalid parameters or decryption failure.
pub async fn handle_tor_cell_decrypt(params: Option<&Value>) -> Result<Value, BearDogError> {
    let params = params.ok_or_else(|| BearDogError::invalid_input("Missing parameters"))?;

    let key_b64 = params
        .get("key")
        .and_then(|v| v.as_str())
        .ok_or_else(|| BearDogError::invalid_input("Missing 'key' parameter"))?;

    let key = BASE64
        .decode(key_b64)
        .map_err(|e| BearDogError::invalid_input(&format!("Invalid base64 key: {e}")))?;

    if key.len() != 32 {
        return Err(BearDogError::invalid_input(&format!(
            "key must be 32 bytes, got {}",
            key.len()
        )));
    }

    let counter = params
        .get("counter")
        .and_then(serde_json::Value::as_u64)
        .ok_or_else(|| BearDogError::invalid_input("Missing 'counter' parameter"))?;

    let ciphertext_b64 = params
        .get("ciphertext")
        .and_then(|v| v.as_str())
        .ok_or_else(|| BearDogError::invalid_input("Missing 'ciphertext' parameter"))?;

    let mut data = BASE64
        .decode(ciphertext_b64)
        .map_err(|e| BearDogError::invalid_input(&format!("Invalid base64 ciphertext: {e}")))?;

    chacha20_counter_mode(&key, counter, &mut data)?;

    Ok(json!({
        "plaintext": BASE64.encode(&data),
        "next_counter": counter + 1,
        "algorithm": "chacha20-counter"
    }))
}
