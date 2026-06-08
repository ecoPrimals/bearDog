// SPDX-License-Identifier: AGPL-3.0-or-later

//! HMAC-SHA256 compute and constant-time verify.

use base64::Engine;
use beardog_core::crypto_service::algorithms::hashing;
use serde_json::Value;
use tracing::{debug, info};

/// Handle `crypto.hmac_sha256` — compute an HMAC-SHA256 authentication tag.
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
/// Returns an error if parameters are missing or base64 decoding fails.
pub async fn handle_hmac_sha256(
    params: Option<&Value>,
) -> Result<Value, super::super::super::HandlerError> {
    let params = params.ok_or("Missing params for crypto.hmac_sha256")?;

    let key_b64 = params
        .get("key")
        .and_then(|v| v.as_str())
        .ok_or("Missing required parameter: key")?;

    let data_b64 = params
        .get("data")
        .and_then(|v| v.as_str())
        .ok_or("Missing required parameter: data")?;

    let key = base64::engine::general_purpose::STANDARD
        .decode(key_b64)
        .map_err(|e| format!("Invalid base64 key: {e}"))?;

    let data = base64::engine::general_purpose::STANDARD
        .decode(data_b64)
        .map_err(|e| format!("Invalid base64 data: {e}"))?;

    debug!("🔐 Computing HMAC-SHA256 for {} bytes", data.len());

    let mac = hashing::hmac_sha256(&key, &data)
        .map_err(|e| format!("HMAC-SHA256 computation failed: {e}"))?;

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
pub async fn handle_hmac_verify(
    params: Option<&Value>,
) -> Result<Value, super::super::super::HandlerError> {
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

#[cfg(test)]
mod tests {
    use super::*;
    use base64::engine::general_purpose::STANDARD as BASE64;
    use serde_json::json;

    #[tokio::test]
    async fn test_hmac_sha256_basic() {
        let key = BASE64.encode(b"secret-key-32-bytes-long-0000000");
        let data = BASE64.encode(b"Message to authenticate");
        let params = json!({ "key": key, "data": data });
        let result = handle_hmac_sha256(Some(&params)).await;
        assert!(result.is_ok());
        let value = result.expect("hmac-sha256 should succeed");
        assert_eq!(value["algorithm"], "HMAC-SHA256");
        let m = value["mac"].as_str().expect("mac string");
        let mac = BASE64.decode(m).expect("mac base64");
        assert_eq!(mac.len(), 32);
    }

    #[tokio::test]
    async fn test_hmac_sha256_deterministic() {
        let key = BASE64.encode(b"test-key");
        let data = BASE64.encode(b"test-data");
        let params = json!({ "key": key, "data": data });
        let result1 = handle_hmac_sha256(Some(&params)).await.expect("hmac first");
        let result2 = handle_hmac_sha256(Some(&params))
            .await
            .expect("hmac second");
        assert_eq!(result1["mac"], result2["mac"]);
    }

    #[tokio::test]
    async fn test_hmac_sha256_different_keys_different_macs() {
        let data = BASE64.encode(b"same data");
        let params1 = json!({ "key": BASE64.encode(b"key1"), "data": data });
        let params2 = json!({ "key": BASE64.encode(b"key2"), "data": data });
        let result1 = handle_hmac_sha256(Some(&params1)).await.expect("hmac key1");
        let result2 = handle_hmac_sha256(Some(&params2)).await.expect("hmac key2");
        assert_ne!(result1["mac"], result2["mac"]);
    }

    #[tokio::test]
    async fn test_hmac_sha256_missing_key() {
        let data = BASE64.encode(b"data");
        let params = json!({ "data": data });
        let result = handle_hmac_sha256(Some(&params)).await;
        assert!(result.is_err());
        assert!(result.unwrap_err().contains("key"));
    }

    #[tokio::test]
    async fn test_hmac_sha256_missing_data() {
        let key = BASE64.encode(b"key");
        let params = json!({ "key": key });
        let result = handle_hmac_sha256(Some(&params)).await;
        assert!(result.is_err());
        assert!(result.unwrap_err().contains("data"));
    }

    #[tokio::test]
    async fn test_hmac_verify_correct_tag() {
        let key = BASE64.encode(b"secret-key-for-hmac");
        let data = BASE64.encode(b"message to authenticate");
        let mac_result = handle_hmac_sha256(Some(&json!({"key": key, "data": data})))
            .await
            .expect("hmac compute");
        let mac = mac_result["mac"].as_str().expect("mac");
        let params = json!({"key": key, "data": data, "mac": mac});
        let result = handle_hmac_verify(Some(&params)).await.expect("verify");
        assert_eq!(result["valid"], true);
        assert_eq!(result["algorithm"], "HMAC-SHA256");
    }

    #[tokio::test]
    async fn test_hmac_verify_wrong_tag() {
        let key = BASE64.encode(b"key");
        let data = BASE64.encode(b"data");
        let bad_mac = BASE64.encode(&[0u8; 32]);
        let params = json!({"key": key, "data": data, "mac": bad_mac});
        let result = handle_hmac_verify(Some(&params)).await.expect("verify");
        assert_eq!(result["valid"], false);
    }

    #[tokio::test]
    async fn test_hmac_verify_missing_params() {
        assert!(handle_hmac_verify(None).await.is_err());
        let partial = json!({"key": BASE64.encode(b"k"), "data": BASE64.encode(b"d")});
        let err = handle_hmac_verify(Some(&partial)).await.unwrap_err();
        assert!(err.contains("mac"));
    }
}
