// SPDX-License-Identifier: AGPL-3.0-or-later

//! `beardog.fido2.authenticate` — get an assertion (CTAP2 `GetAssertion`).

use serde_json::Value;
#[cfg(feature = "ctap2")]
use serde_json::json;
#[cfg(feature = "ctap2")]
use tracing::info;

/// Authenticate with a FIDO2 device (CTAP2 `GetAssertion`).
///
/// Requires physical presence (user must touch the security key).
/// This is the primary method for hardware-attested signatures in
/// the spore chain (`liveSpore.json` witness entries).
///
/// # Parameters
///
/// - `rp_id` (string, required): relying party identifier
/// - `credential_id` (string, required): base64-encoded credential ID from registration
/// - `challenge` (string, required): base64-encoded challenge data to sign
/// - `device_path` (string, optional): HID device path
///
/// # Returns
///
/// ```json
/// {
///   "signature": "<base64>",
///   "user_present": true,
///   "rp_id": "primals.eco",
///   "credential_id": "<base64>"
/// }
/// ```
pub fn handle_fido2_authenticate(
    params: Option<&Value>,
) -> Result<Value, super::super::HandlerError> {
    let params = params.ok_or("Missing params for beardog.fido2.authenticate")?;

    let rp_id = params
        .get("rp_id")
        .and_then(|v| v.as_str())
        .ok_or("Missing required parameter: rp_id")?;

    let credential_id = params
        .get("credential_id")
        .and_then(|v| v.as_str())
        .ok_or("Missing required parameter: credential_id")?;

    let challenge = params
        .get("challenge")
        .and_then(|v| v.as_str())
        .ok_or("Missing required parameter: challenge")?;

    let _device_path = params.get("device_path").and_then(|v| v.as_str());

    #[cfg(feature = "ctap2")]
    {
        use base64::Engine;
        use base64::engine::general_purpose::STANDARD as BASE64;

        let device_path = super::helpers::resolve_device_path(_device_path).await?;

        let credential_id_bytes = BASE64
            .decode(credential_id)
            .map_err(|e| format!("Invalid base64 credential_id: {e}"))?;

        let challenge_bytes = BASE64
            .decode(challenge)
            .map_err(|e| format!("Invalid base64 challenge: {e}"))?;

        let provider = super::helpers::create_ctap2_provider(&device_path, rp_id).await?;

        let signature_bytes = provider
            .authenticate_with_credential(&credential_id_bytes, &challenge_bytes)
            .await
            .map_err(|e| format!("GetAssertion failed: {e}"))?;

        let signature_b64 = BASE64.encode(&signature_bytes);

        info!(
            rp_id,
            sig_len = signature_bytes.len(),
            "FIDO2 assertion signed (hardware-attested)"
        );

        Ok(json!({
            "signature": signature_b64,
            "user_present": true,
            "rp_id": rp_id,
            "credential_id": credential_id,
        }))
    }

    #[cfg(not(feature = "ctap2"))]
    {
        let _ = (rp_id, credential_id, challenge);
        Err(
            "FIDO2 feature not enabled — rebuild bearDog with --features ctap2 to use \
             hardware security keys"
                .to_string()
                .into(),
        )
    }
}
