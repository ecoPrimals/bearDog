// SPDX-License-Identifier: AGPL-3.0-or-later

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
pub async fn handle_hkdf_sha256(params: Option<&Value>) -> Result<Value, String> {
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
