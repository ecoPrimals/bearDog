// SPDX-License-Identifier: AGPL-3.0-or-later

use base64::Engine;
use beardog_core::crypto_service::algorithms::hashing;
use serde_json::Value;
use tracing::{debug, info};
/// Handle `crypto.hmac_sha256` method
///
/// Computes HMAC-SHA256 authentication tag.
///
/// # Parameters
///
/// - `key`: Base64-encoded secret key
/// - `data`: Base64-encoded data to authenticate
///
/// # Returns
///
/// - `mac`: Base64-encoded HMAC-SHA256 tag (32 bytes)
///
/// # Errors
///
/// Returns `Err` if params are missing or key/data are not valid base64.
pub async fn handle_hmac_sha256(params: Option<&Value>) -> Result<Value, String> {
    let params = params.ok_or("Missing params for crypto.hmac_sha256")?;

    // Extract parameters
    let key_b64 = params
        .get("key")
        .and_then(|v| v.as_str())
        .ok_or("Missing required parameter: key")?;

    let data_b64 = params
        .get("data")
        .and_then(|v| v.as_str())
        .ok_or("Missing required parameter: data")?;

    // Decode parameters
    let key = base64::engine::general_purpose::STANDARD
        .decode(key_b64)
        .map_err(|e| format!("Invalid base64 key: {e}"))?;

    let data = base64::engine::general_purpose::STANDARD
        .decode(data_b64)
        .map_err(|e| format!("Invalid base64 data: {e}"))?;

    debug!("🔐 Computing HMAC-SHA256 for {} bytes", data.len());

    let mac = hashing::hmac_sha256(&key, &data)
        .map_err(|e| format!("HMAC-SHA256 computation failed: {e}"))?;

    // Encode MAC
    let mac_b64 = base64::engine::general_purpose::STANDARD.encode(&mac);

    info!("✅ HMAC-SHA256 computed ({} bytes)", mac.len());

    Ok(serde_json::json!({
        "mac": mac_b64,
        "algorithm": "HMAC-SHA256",
    }))
}

/// Handle `crypto.hmac_verify` — verify an HMAC tag using constant-time comparison.
///
/// Params: `key` (base64), `data` (base64), `mac` (base64 expected MAC).
///
/// Returns `{valid: bool, algorithm: "HMAC-SHA256"}`.
///
/// # Errors
///
/// Returns an error if parameters are missing or base64 decoding fails.
pub async fn handle_hmac_verify(params: Option<&Value>) -> Result<Value, String> {
    let params = params.ok_or("Missing params for crypto.hmac_verify")?;

    let key_b64 = params
        .get("key")
        .and_then(|v| v.as_str())
        .ok_or("Missing required parameter: key")?;

    let data_b64 = params
        .get("data")
        .and_then(|v| v.as_str())
        .ok_or("Missing required parameter: data")?;

    let expected_b64 = params
        .get("mac")
        .and_then(|v| v.as_str())
        .ok_or("Missing required parameter: mac")?;

    let key = base64::engine::general_purpose::STANDARD
        .decode(key_b64)
        .map_err(|e| format!("Invalid base64 key: {e}"))?;

    let data = base64::engine::general_purpose::STANDARD
        .decode(data_b64)
        .map_err(|e| format!("Invalid base64 data: {e}"))?;

    let expected = base64::engine::general_purpose::STANDARD
        .decode(expected_b64)
        .map_err(|e| format!("Invalid base64 mac: {e}"))?;

    let computed = hashing::hmac_sha256(&key, &data)
        .map_err(|e| format!("HMAC-SHA256 computation failed: {e}"))?;

    let valid: bool =
        subtle::ConstantTimeEq::ct_eq(computed.as_slice(), expected.as_slice()).into();

    info!(valid, "HMAC-SHA256 verification");

    Ok(serde_json::json!({
        "valid": valid,
        "algorithm": "HMAC-SHA256",
    }))
}
