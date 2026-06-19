// SPDX-License-Identifier: AGPL-3.0-or-later

use base64::Engine;
use beardog_core::crypto_service::algorithms::hashing;
use serde_json::Value;
use tracing::{debug, info};

/// Handle BLAKE3 hash operations via JSON-RPC.
///
/// # Wire Contract (LD-01)
///
/// The `data` param **must** be standard Base64 (RFC 4648 §4). `BearDog`
/// decodes to raw bytes, hashes with BLAKE3, and returns the digest as
/// Base64. Passing raw UTF-8 or hex directly yields incorrect hashes.
///
/// # Errors
///
/// Returns an error if `data` is missing or not valid Base64.
pub async fn handle_blake3_hash(params: Option<&Value>) -> Result<Value, String> {
    let params = params.ok_or(
        "Missing params for crypto.blake3_hash — expected: {\"data\": \"<standard-base64>\"}",
    )?;

    let data_b64 = params
        .get("data")
        .and_then(|v| v.as_str())
        .ok_or("Missing required parameter: data (standard base64 encoded string)")?;

    let data = base64::engine::general_purpose::STANDARD
        .decode(data_b64)
        .map_err(|e| {
            format!(
                "Invalid base64 in 'data' field: {e} — use standard base64 (RFC 4648, +/= alphabet)"
            )
        })?;

    debug!("🔨 Hashing {} bytes with BLAKE3", data.len());

    let hash = hashing::hash_blake3(&data);

    // Encode hash
    let hash_b64 = base64::engine::general_purpose::STANDARD.encode(&hash);

    info!("✅ BLAKE3 hash computed ({} bytes)", hash.len());

    Ok(serde_json::json!({
        "hash": hash_b64,
        "algorithm": "BLAKE3",
    }))
}
