// SPDX-License-Identifier: AGPL-3.0-or-later

//! BLAKE3 cryptographic hash handler.

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
pub async fn handle_blake3_hash(
    params: Option<&Value>,
) -> Result<Value, super::super::super::HandlerError> {
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

    let hash_b64 = base64::engine::general_purpose::STANDARD.encode(&hash);

    info!("✅ BLAKE3 hash computed ({} bytes)", hash.len());

    Ok(serde_json::json!({
        "hash": hash_b64,
        "algorithm": "BLAKE3",
    }))
}

#[cfg(test)]
mod tests {
    use super::*;
    use base64::engine::general_purpose::STANDARD as BASE64;
    use serde_json::json;

    #[tokio::test]
    async fn test_blake3_hash_basic() {
        let data = BASE64.encode(b"Hello, BearDog!");
        let params = json!({ "data": data });

        let result = handle_blake3_hash(Some(&params)).await;
        assert!(result.is_ok());

        let value = result.expect("blake3 hash should succeed");
        assert_eq!(value["algorithm"], "BLAKE3");
        assert!(value["hash"].is_string());

        let h = value["hash"].as_str().expect("hash should be string");
        let hash = BASE64.decode(h).expect("hash base64");
        assert_eq!(hash.len(), 32);
    }

    #[tokio::test]
    async fn test_blake3_hash_empty_data() {
        let data = BASE64.encode(b"");
        let params = json!({ "data": data });

        let result = handle_blake3_hash(Some(&params)).await;
        assert!(result.is_ok());

        let value = result.expect("blake3 empty hash should succeed");
        let h = value["hash"].as_str().expect("hash string");
        let hash = BASE64.decode(h).expect("hash base64");
        assert_eq!(hash.len(), 32);
    }

    #[tokio::test]
    async fn test_blake3_hash_deterministic() {
        let data = BASE64.encode(b"Test data for BLAKE3");
        let params = json!({ "data": data });

        let result1 = handle_blake3_hash(Some(&params))
            .await
            .expect("blake3 first");
        let result2 = handle_blake3_hash(Some(&params))
            .await
            .expect("blake3 second");

        assert_eq!(result1["hash"], result2["hash"]);
    }

    #[tokio::test]
    async fn test_blake3_hash_missing_params() {
        let result = handle_blake3_hash(None).await;
        assert!(result.is_err());
        assert!(result.unwrap_err().contains("Missing params"));
    }

    #[tokio::test]
    async fn test_blake3_hash_missing_data() {
        let params = json!({});
        let result = handle_blake3_hash(Some(&params)).await;
        assert!(result.is_err());
        assert!(result.unwrap_err().contains("data"));
    }

    #[tokio::test]
    async fn test_blake3_hash_invalid_base64() {
        let params = json!({ "data": "!!!not-valid-base64!!!" });
        let result = handle_blake3_hash(Some(&params)).await;
        assert!(result.is_err());
        assert!(result.unwrap_err().contains("Invalid base64"));
    }
}
