// SPDX-License-Identifier: AGPL-3.0-or-later

//! TLS 1.3 cipher-suite-aware hashing (SHA-256 / SHA-384).

use base64::Engine;
use serde_json::Value;
use sha2::{Digest, Sha256, Sha384};
use tracing::{debug, info};

/// Handle `crypto.hash_for_cipher` — select hash algorithm by TLS 1.3 cipher suite.
///
/// # TLS 1.3 Cipher Suites (RFC 8446)
///
/// - **0x1301** (`TLS_AES_128_GCM_SHA256`): SHA-256 (32 bytes)
/// - **0x1302** (`TLS_AES_256_GCM_SHA384`): SHA-384 (48 bytes)
/// - **0x1303** (`TLS_CHACHA20_POLY1305_SHA256`): SHA-256 (32 bytes)
///
/// # Errors
///
/// Returns an error if parameters are missing or the cipher suite is unsupported.
pub async fn handle_hash_for_cipher(
    params: Option<&Value>,
) -> Result<Value, super::super::super::HandlerError> {
    let params = params.ok_or("Missing params for crypto.hash_for_cipher")?;

    let data_b64 = params
        .get("data")
        .and_then(|v| v.as_str())
        .ok_or("Missing required parameter: data")?;

    #[expect(
        clippy::cast_possible_truncation,
        reason = "IANA TLS cipher suite identifier"
    )]
    let cipher_suite = params
        .get("cipher_suite")
        .and_then(serde_json::Value::as_u64)
        .ok_or("Missing required parameter: cipher_suite")? as u16;

    let data = base64::engine::general_purpose::STANDARD
        .decode(data_b64)
        .map_err(|e| format!("Invalid base64 data: {e}"))?;

    debug!(
        "🔨 Hashing {} bytes for cipher suite 0x{:04x}",
        data.len(),
        cipher_suite
    );

    let (hash, algorithm) = match cipher_suite {
        0x1301 => {
            info!("  → Using SHA-256 for cipher suite 0x1301 (TLS_AES_128_GCM_SHA256)");
            (Sha256::digest(&data).to_vec(), "SHA-256")
        }
        0x1302 => {
            info!("  → Using SHA-384 for cipher suite 0x1302 (TLS_AES_256_GCM_SHA384)");
            (Sha384::digest(&data).to_vec(), "SHA-384")
        }
        0x1303 => {
            info!("  → Using SHA-256 for cipher suite 0x1303 (TLS_CHACHA20_POLY1305_SHA256)");
            (Sha256::digest(&data).to_vec(), "SHA-256")
        }
        _ => {
            return Err(format!(
                "Unsupported TLS 1.3 cipher suite: 0x{cipher_suite:04x}. Supported: 0x1301 (AES-128-GCM), 0x1302 (AES-256-GCM), 0x1303 (ChaCha20-Poly1305)"
            )
            .into());
        }
    };

    let hash_b64 = base64::engine::general_purpose::STANDARD.encode(&hash);

    info!(
        "✅ Hash computed: {} bytes using {} (cipher 0x{:04x})",
        hash.len(),
        algorithm,
        cipher_suite
    );

    Ok(serde_json::json!({
        "hash": hash_b64,
        "algorithm": algorithm,
        "hash_length": hash.len(),
        "cipher_suite": cipher_suite
    }))
}

#[cfg(test)]
mod tests {
    use super::*;
    use base64::engine::general_purpose::STANDARD as BASE64;
    use serde_json::json;

    #[tokio::test]
    async fn test_hash_for_cipher_sha256() {
        let data = BASE64.encode(b"TLS 1.3 data");
        let params = json!({ "data": data, "cipher_suite": 0x1301 });
        let result = handle_hash_for_cipher(Some(&params)).await;
        assert!(result.is_ok());
        let value = result.expect("hash_for_cipher 0x1301");
        assert_eq!(value["algorithm"], "SHA-256");
        assert_eq!(value["cipher_suite"], 0x1301);
        assert_eq!(value["hash_length"], 32);
    }

    #[tokio::test]
    async fn test_hash_for_cipher_sha384() {
        let data = BASE64.encode(b"TLS 1.3 data");
        let params = json!({ "data": data, "cipher_suite": 0x1302 });
        let result = handle_hash_for_cipher(Some(&params)).await;
        assert!(result.is_ok());
        let value = result.expect("hash_for_cipher 0x1302");
        assert_eq!(value["algorithm"], "SHA-384");
        assert_eq!(value["cipher_suite"], 0x1302);
        assert_eq!(value["hash_length"], 48);
    }

    #[tokio::test]
    async fn test_hash_for_cipher_chacha20() {
        let data = BASE64.encode(b"TLS 1.3 data");
        let params = json!({ "data": data, "cipher_suite": 0x1303 });
        let result = handle_hash_for_cipher(Some(&params)).await;
        assert!(result.is_ok());
        let value = result.expect("hash_for_cipher 0x1303");
        assert_eq!(value["algorithm"], "SHA-256");
        assert_eq!(value["cipher_suite"], 0x1303);
        assert_eq!(value["hash_length"], 32);
    }

    #[tokio::test]
    async fn test_hash_for_cipher_unsupported() {
        let data = BASE64.encode(b"data");
        let params = json!({ "data": data, "cipher_suite": 0x9999 });
        let result = handle_hash_for_cipher(Some(&params)).await;
        assert!(result.is_err());
        assert!(result.unwrap_err().contains("Unsupported"));
    }

    #[tokio::test]
    async fn test_hash_for_cipher_missing_cipher_suite() {
        let data = BASE64.encode(b"data");
        let params = json!({ "data": data });
        let result = handle_hash_for_cipher(Some(&params)).await;
        assert!(result.is_err());
        assert!(result.unwrap_err().contains("cipher_suite"));
    }
}
