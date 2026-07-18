// SPDX-License-Identifier: AGPL-3.0-or-later

//! `beardog.fido2.entropy` — single-tap hardware entropy harvest.

use serde_json::Value;
#[cfg(feature = "ctap2")]
use serde_json::json;
#[cfg(feature = "ctap2")]
use tracing::info;

/// Harvest hardware entropy from a FIDO2 device.
///
/// Generates a random challenge, requests a `GetAssertion`, and mixes the
/// resulting signature bytes (which contain the authenticator's hardware RNG
/// nonce) with the challenge via BLAKE3 to produce uniform entropy.
///
/// Requires physical presence (user must touch the security key).
///
/// # Parameters
///
/// - `rp_id` (string, required): relying party identifier
/// - `credential_id` (string, required): base64-encoded credential ID from registration
/// - `device_path` (string, optional): HID device path
///
/// # Returns
///
/// ```json
/// {
///   "entropy": "<base64, 32 bytes>",
///   "source": "fido2_hardware",
///   "tier": 2,
///   "user_present": true
/// }
/// ```
pub async fn handle_fido2_entropy(
    params: Option<&Value>,
) -> Result<Value, super::super::HandlerError> {
    let params = params.ok_or("Missing params for beardog.fido2.entropy")?;

    let rp_id = params
        .get("rp_id")
        .and_then(|v| v.as_str())
        .ok_or("Missing required parameter: rp_id")?;

    let credential_id = params
        .get("credential_id")
        .and_then(|v| v.as_str())
        .ok_or("Missing required parameter: credential_id")?;

    let _device_path = params.get("device_path").and_then(|v| v.as_str());

    #[cfg(feature = "ctap2")]
    {
        use base64::Engine;
        use base64::engine::general_purpose::STANDARD as BASE64;

        let device_path = super::helpers::resolve_device_path(_device_path).await?;

        let credential_id_bytes = BASE64
            .decode(credential_id)
            .map_err(|e| format!("Invalid base64 credential_id: {e}"))?;

        let challenge: [u8; 32] = rand::random();

        let provider = super::helpers::create_ctap2_provider(&device_path, rp_id).await?;

        let signature_bytes = provider
            .authenticate_with_credential(&credential_id_bytes, &challenge)
            .await
            .map_err(|e| format!("Entropy harvest failed: {e}"))?;

        let entropy = blake3::keyed_hash(
            blake3::hash(b"beardog_fido2_entropy_v1").as_bytes(),
            &[challenge.as_slice(), signature_bytes.as_slice()].concat(),
        );

        info!(
            rp_id,
            entropy_len = 32,
            sig_source_len = signature_bytes.len(),
            "FIDO2 hardware entropy harvested (Tier 2)"
        );

        Ok(json!({
            "entropy": BASE64.encode(entropy.as_bytes()),
            "source": "fido2_hardware",
            "tier": 2,
            "user_present": true,
        }))
    }

    #[cfg(not(feature = "ctap2"))]
    {
        let _ = (rp_id, credential_id);
        Err(
            "FIDO2 feature not enabled — rebuild bearDog with --features ctap2 to use \
             hardware entropy from security keys"
                .to_string()
                .into(),
        )
    }
}
