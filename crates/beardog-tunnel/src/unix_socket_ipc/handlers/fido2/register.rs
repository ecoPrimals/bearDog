// SPDX-License-Identifier: AGPL-3.0-or-later

//! `beardog.fido2.register` — create a credential (CTAP2 `MakeCredential`).

use serde_json::Value;
#[cfg(feature = "ctap2")]
use serde_json::json;
#[cfg(feature = "ctap2")]
use tracing::info;

/// Create a credential on a FIDO2 device (CTAP2 `MakeCredential`).
///
/// Requires physical presence (user must touch the security key).
///
/// # Parameters
///
/// - `rp_id` (string, required): relying party identifier (e.g. `"primals.eco"`)
/// - `rp_name` (string, optional): human-readable RP name
/// - `user_id` (string, required): base64-encoded user handle
/// - `user_name` (string, required): user display name
/// - `device_path` (string, optional): HID device path (auto-selects first FIDO2 device)
/// - `pin` (string, optional): device PIN for ClientPIN-authenticated registration
pub async fn handle_fido2_register(params: Option<&Value>) -> Result<Value, super::super::HandlerError> {
    let params = params.ok_or("Missing params for beardog.fido2.register")?;

    let rp_id = params
        .get("rp_id")
        .and_then(|v| v.as_str())
        .ok_or("Missing required parameter: rp_id")?;

    let user_id = params
        .get("user_id")
        .and_then(|v| v.as_str())
        .ok_or("Missing required parameter: user_id")?;

    let user_name = params
        .get("user_name")
        .and_then(|v| v.as_str())
        .ok_or("Missing required parameter: user_name")?;

    let _rp_name = params
        .get("rp_name")
        .and_then(|v| v.as_str())
        .unwrap_or(rp_id);

    let _device_path = params.get("device_path").and_then(|v| v.as_str());
    let _pin = params.get("pin").and_then(|v| v.as_str());

    #[cfg(feature = "ctap2")]
    {
        use base64::Engine;
        use base64::engine::general_purpose::STANDARD as BASE64;
        use crate::tunnel::hsm::solo_v2::types::KeyType;

        let device_path = super::helpers::resolve_device_path(_device_path).await?;

        let _user_id_bytes = BASE64
            .decode(user_id)
            .map_err(|e| format!("Invalid base64 user_id: {e}"))?;

        if let Some(pin) = _pin {
            let result =
                ceremony_register_with_pin(&device_path, rp_id, &_user_id_bytes, user_name, pin)
                    .await
                    .map_err(|e| format!("MakeCredential (PIN ceremony): {e}"))?;

            let credential_id_b64 = BASE64.encode(&result.0);
            let public_key_b64 = BASE64.encode(&result.1);

            info!(
                rp_id,
                credential_id_len = result.0.len(),
                "FIDO2 credential registered (PIN ceremony)"
            );

            return Ok(json!({
                "credential_id": credential_id_b64,
                "public_key": public_key_b64,
                "rp_id": rp_id,
                "user_name": user_name,
            }));
        }

        let provider = super::helpers::create_ctap2_provider(&device_path, rp_id).await?;

        let key_label = format!("{rp_id}:{user_name}");
        let result = provider
            .generate_key_on_device(KeyType::Ed25519, key_label)
            .await
            .map_err(|e| format!("MakeCredential failed: {e}"))?;

        let credential_id_b64 = BASE64.encode(&result.credential_id);
        let public_key_b64 = BASE64.encode(&result.public_key);

        info!(
            rp_id,
            credential_id_len = result.credential_id.len(),
            "FIDO2 credential registered"
        );

        Ok(json!({
            "credential_id": credential_id_b64,
            "public_key": public_key_b64,
            "rp_id": rp_id,
            "user_name": user_name,
            "key_id": result.key_id,
        }))
    }

    #[cfg(not(feature = "ctap2"))]
    {
        let _ = (rp_id, user_id, user_name, _rp_name);
        Err(
            "FIDO2 feature not enabled — rebuild bearDog with --features ctap2 to use \
             hardware security keys"
                .to_string()
                .into(),
        )
    }
}

/// Full PIN-ceremony credential registration: set PIN -> get token -> `MakeCredential`.
///
/// Returns `(credential_id, public_key_cose)` on success.
#[cfg(feature = "ctap2")]
async fn ceremony_register_with_pin(
    device_path: &str,
    rp_id: &str,
    user_id: &[u8],
    user_name: &str,
    pin: &str,
) -> Result<(Vec<u8>, Vec<u8>), String> {
    use crate::tunnel::hsm::solo_v2::Ctap2Transport;
    use crate::tunnel::hsm::solo_v2::HidCtap2Transport;
    use crate::tunnel::hsm::solo_v2::ctap2_protocol::{
        build_make_credential, parse_make_credential_response,
    };
    use beardog_security::hsm::fido2::ctap2::client_pin;

    info!(device_path, "Opening HID device for PIN ceremony");

    let mut device = beardog_hid::open_device(device_path)
        .await
        .map_err(|e| format!("Failed to open HID device: {e}"))?;

    info!("Attempting to get pinUvAuthToken (assumes PIN already set)...");
    let pin_token = match client_pin::get_pin_token(&mut device, pin).await {
        Ok(token) => {
            info!("pinUvAuthToken obtained (PIN was already set)");
            token
        }
        Err(e) => {
            let msg = format!("{e}");
            if msg.contains("PIN_NOT_SET") || msg.contains("0x35") || msg.contains("PinNotSet") {
                info!("PIN not set, setting now...");
                client_pin::set_pin(&mut device, pin)
                    .await
                    .map_err(|e2| format!("Failed to set PIN: {e2}"))?;
                info!("PIN set, getting token...");
                client_pin::get_pin_token(&mut device, pin)
                    .await
                    .map_err(|e2| format!("Failed to get PIN token after set: {e2}"))?
            } else {
                return Err(format!("ClientPIN failed: {msg}"));
            }
        }
    };

    let client_data_hash = [0u8; 32];
    let pin_uv_auth_param = pin_token.authenticate(&client_data_hash);
    let pin_uv = Some((pin_uv_auth_param.as_slice(), 1_u64));

    let cmd = build_make_credential(rp_id, user_id, user_name, &client_data_hash, -8, pin_uv)
        .map_err(|e| format!("Failed to build MakeCredential: {e}"))?;

    info!("Sending MakeCredential (touch key when it blinks!)...");
    let mut transport = HidCtap2Transport::open(device_path)
        .await
        .map_err(|e| format!("Transport init failed: {e}"))?;

    let response = transport
        .send_receive(&cmd)
        .await
        .map_err(|e| format!("MakeCredential transport error: {e}"))?;

    let parsed = parse_make_credential_response(&response)
        .map_err(|e| format!("MakeCredential parse error: {e}"))?;

    Ok((parsed.credential_id, parsed.raw_cose_public_key))
}
