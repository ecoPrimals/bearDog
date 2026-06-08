// SPDX-License-Identifier: AGPL-3.0-or-later

//! HKDF-SHA256 key derivation (RFC 5869).

use base64::Engine;
use serde_json::Value;
use tracing::info;

/// Handle `crypto.hkdf_sha256` — HKDF-SHA256 extract-and-expand.
///
/// Params:
/// - `ikm` (base64, required): input key material
/// - `salt` (base64, optional): salt for extract step
/// - `info` (base64 or UTF-8 string, optional): context/info for expand step
/// - `length` (integer, optional): output key length in bytes (default 32, max 255*32)
///
/// Returns `{okm: "<base64>", algorithm: "HKDF-SHA256", length: N}`.
///
/// # Errors
///
/// Returns an error if parameters are missing or HKDF expand fails.
pub async fn handle_hkdf_sha256(
    params: Option<&Value>,
) -> Result<Value, super::super::super::HandlerError> {
    use hkdf::Hkdf;
    use sha2::Sha256;

    let params = params.ok_or("Missing params for crypto.hkdf_sha256")?;

    let ikm_b64 = params
        .get("ikm")
        .and_then(|v| v.as_str())
        .ok_or("Missing required parameter: ikm")?;

    let ikm = base64::engine::general_purpose::STANDARD
        .decode(ikm_b64)
        .map_err(|e| format!("Invalid base64 ikm: {e}"))?;

    let salt = params
        .get("salt")
        .and_then(|v| v.as_str())
        .map(|s| {
            base64::engine::general_purpose::STANDARD
                .decode(s)
                .map_err(|e| format!("Invalid base64 salt: {e}"))
        })
        .transpose()?;

    let info = params
        .get("info")
        .map(|v| {
            if let Some(s) = v.as_str() {
                base64::engine::general_purpose::STANDARD
                    .decode(s)
                    .unwrap_or_else(|_| s.as_bytes().to_vec())
            } else {
                Vec::new()
            }
        })
        .unwrap_or_default();

    #[allow(
        clippy::cast_possible_truncation,
        reason = "HKDF length capped at MAX_HKDF_LENGTH (8160), fits in usize on all targets"
    )]
    let length = params
        .get("length")
        .and_then(serde_json::Value::as_u64)
        .unwrap_or(32)
        .min(255 * 32) as usize;

    let hkdf = Hkdf::<Sha256>::new(salt.as_deref(), &ikm);
    let mut okm = vec![0u8; length];
    hkdf.expand(&info, &mut okm)
        .map_err(|e| format!("HKDF-SHA256 expand failed: {e}"))?;

    let okm_b64 = base64::engine::general_purpose::STANDARD.encode(&okm);

    info!(length, "HKDF-SHA256 derived key material");

    Ok(serde_json::json!({
        "okm": okm_b64,
        "algorithm": "HKDF-SHA256",
        "length": length,
    }))
}

#[cfg(test)]
mod tests {
    use super::*;
    use base64::engine::general_purpose::STANDARD as BASE64;
    use serde_json::json;

    #[tokio::test]
    async fn test_hkdf_sha256_default_length() {
        let ikm = BASE64.encode(b"input key material");
        let params = json!({"ikm": ikm});
        let result = handle_hkdf_sha256(Some(&params)).await.expect("hkdf");
        assert_eq!(result["algorithm"], "HKDF-SHA256");
        assert_eq!(result["length"], 32);
        let okm = BASE64
            .decode(result["okm"].as_str().expect("okm"))
            .expect("b64");
        assert_eq!(okm.len(), 32);
    }

    #[tokio::test]
    async fn test_hkdf_sha256_custom_length() {
        let params = json!({"ikm": BASE64.encode(b"key"), "length": 48});
        let result = handle_hkdf_sha256(Some(&params)).await.expect("hkdf");
        assert_eq!(result["length"], 48);
        let okm = BASE64
            .decode(result["okm"].as_str().expect("okm"))
            .expect("b64");
        assert_eq!(okm.len(), 48);
    }

    #[tokio::test]
    async fn test_hkdf_sha256_with_salt_and_info() {
        let params = json!({
            "ikm": BASE64.encode(b"shared_secret"),
            "salt": BASE64.encode(b"random_salt"),
            "info": BASE64.encode(b"btsp-v1-phase3"),
            "length": 32
        });
        let result = handle_hkdf_sha256(Some(&params)).await.expect("hkdf");
        assert_eq!(result["length"], 32);
    }

    #[tokio::test]
    async fn test_hkdf_sha256_deterministic() {
        let params = json!({"ikm": BASE64.encode(b"k"), "salt": BASE64.encode(b"s"), "info": BASE64.encode(b"i")});
        let r1 = handle_hkdf_sha256(Some(&params)).await.expect("1");
        let r2 = handle_hkdf_sha256(Some(&params)).await.expect("2");
        assert_eq!(r1["okm"], r2["okm"]);
    }

    #[tokio::test]
    async fn test_hkdf_sha256_different_salt_different_output() {
        let p1 = json!({"ikm": BASE64.encode(b"k"), "salt": BASE64.encode(b"salt1")});
        let p2 = json!({"ikm": BASE64.encode(b"k"), "salt": BASE64.encode(b"salt2")});
        let r1 = handle_hkdf_sha256(Some(&p1)).await.expect("1");
        let r2 = handle_hkdf_sha256(Some(&p2)).await.expect("2");
        assert_ne!(r1["okm"], r2["okm"]);
    }

    #[tokio::test]
    async fn test_hkdf_sha256_missing_ikm_errors() {
        let params = json!({"salt": BASE64.encode(b"s")});
        let err = handle_hkdf_sha256(Some(&params)).await.unwrap_err();
        assert!(err.contains("ikm"));
    }

    #[tokio::test]
    async fn test_hkdf_sha256_missing_params_errors() {
        assert!(handle_hkdf_sha256(None).await.is_err());
    }

    #[tokio::test]
    async fn test_hkdf_sha256_matches_barrucuda_pattern() {
        let session_key = [42u8; 32];
        let client_nonce = [1u8; 16];
        let server_nonce = [2u8; 16];
        let mut salt = Vec::new();
        salt.extend_from_slice(&client_nonce);
        salt.extend_from_slice(&server_nonce);

        let params = json!({
            "ikm": BASE64.encode(session_key),
            "salt": BASE64.encode(&salt),
            "info": BASE64.encode(b"btsp-v1-phase3"),
            "length": 32
        });
        let result = handle_hkdf_sha256(Some(&params)).await.expect("hkdf");

        use hkdf::Hkdf;
        use sha2::Sha256;
        let hkdf = Hkdf::<Sha256>::new(Some(&salt), &session_key);
        let mut expected = [0u8; 32];
        hkdf.expand(b"btsp-v1-phase3", &mut expected)
            .expect("expand");

        let okm = BASE64
            .decode(result["okm"].as_str().expect("okm"))
            .expect("b64");
        assert_eq!(okm, expected);
    }
}
