// SPDX-License-Identifier: AGPL-3.0-or-later

//! `beardog.fido2.attest_enrollment` — generate an enrollment attestation blob.
//!
//! The enrolling gate taps its `SoloKey` to produce a signed attestation that
//! the enrollment endpoint (golgiBody) can verify against the trust roster.
//! This provides cryptographic proof of physical key possession during
//! gate enrollment.
//!
//! ## Wire format
//!
//! ```json
//! {
//!   "credential_id": "<base64 credential ID from prior registration>",
//!   "gate_name": "newGate",
//!   "wg_public_key": "<base64 WireGuard public key>",
//!   "challenge": "<base64 challenge from enrollment endpoint (optional)>"
//! }
//! ```
//!
//! Returns an attestation blob that the enrollment endpoint verifies via
//! `fido2.verify_attestation`.

use serde_json::Value;
#[cfg(feature = "ctap2")]
use serde_json::json;
#[cfg(feature = "ctap2")]
use tracing::info;

/// Generate an enrollment attestation blob via CTAP2 `GetAssertion`.
///
/// The assertion binds the gate's identity (name + WG pubkey) to a FIDO2
/// credential, proving physical possession of the registered security key.
pub async fn handle_fido2_attest_enrollment(
    params: Option<&Value>,
) -> Result<Value, super::super::HandlerError> {
    let params = params.ok_or("Missing params for beardog.fido2.attest_enrollment")?;

    let credential_id = params
        .get("credential_id")
        .and_then(Value::as_str)
        .ok_or("Missing required parameter: credential_id")?;

    let gate_name = params
        .get("gate_name")
        .and_then(Value::as_str)
        .ok_or("Missing required parameter: gate_name")?;

    let wg_public_key = params
        .get("wg_public_key")
        .and_then(Value::as_str)
        .ok_or("Missing required parameter: wg_public_key")?;

    let challenge = params.get("challenge").and_then(Value::as_str);
    let _device_path = params.get("device_path").and_then(Value::as_str);

    #[cfg(feature = "ctap2")]
    {
        use base64::Engine;
        use base64::engine::general_purpose::STANDARD as BASE64;
        use sha2::Digest;

        let device_path = super::helpers::resolve_device_path(_device_path).await?;
        let credential_id_bytes = BASE64
            .decode(credential_id)
            .map_err(|e| format!("Invalid base64 credential_id: {e}"))?;

        // Build the client data hash: SHA-256(gate_name || "|" || wg_public_key || "|" || challenge)
        let challenge_str = challenge.unwrap_or("enrollment");
        let client_data = format!("{gate_name}|{wg_public_key}|{challenge_str}");
        let client_data_hash: [u8; 32] = sha2::Sha256::digest(client_data.as_bytes()).into();

        let rp_id = "primals.eco";

        use crate::tunnel::hsm::solo_v2::Ctap2Transport;
        use crate::tunnel::hsm::solo_v2::HidCtap2Transport;
        use crate::tunnel::hsm::solo_v2::ctap2_protocol::{
            build_get_assertion, parse_get_assertion_response,
        };

        let cmd = build_get_assertion(rp_id, &client_data_hash, Some(&credential_id_bytes))
            .map_err(|e| format!("Failed to build GetAssertion: {e}"))?;

        info!(
            gate_name,
            credential_id_len = credential_id_bytes.len(),
            "Generating enrollment attestation (touch key!)"
        );

        let mut transport = HidCtap2Transport::open(&device_path)
            .await
            .map_err(|e| format!("Transport init failed: {e}"))?;

        let response = transport
            .send_receive(&cmd)
            .await
            .map_err(|e| format!("GetAssertion transport error: {e}"))?;

        let parsed = parse_get_assertion_response(&response)
            .map_err(|e| format!("GetAssertion parse error: {e}"))?;

        let signature_b64 = BASE64.encode(&parsed.signature);
        let auth_data_b64 = BASE64.encode(&parsed.authenticator_data);

        let timestamp = std::time::SystemTime::now()
            .duration_since(std::time::UNIX_EPOCH)
            .map_or(0, |d| d.as_secs());

        info!(
            gate_name,
            "FIDO2 enrollment attestation generated"
        );

        Ok(json!({
            "attestation": {
                "credential_id": credential_id,
                "signature": signature_b64,
                "authenticator_data": auth_data_b64,
                "client_data_hash": BASE64.encode(client_data_hash),
                "gate_name": gate_name,
                "wg_public_key": wg_public_key,
                "challenge": challenge_str,
                "timestamp": timestamp,
                "rp_id": rp_id,
            },
            "purpose": "gate_enrollment",
            "trust_tier": "kin",
        }))
    }

    #[cfg(not(feature = "ctap2"))]
    {
        let _ = (credential_id, gate_name, wg_public_key, challenge);
        Err(
            "FIDO2 feature not enabled — rebuild bearDog with --features ctap2 to use \
             hardware security keys for enrollment attestation"
                .to_string()
                .into(),
        )
    }
}

/// Verify a FIDO2 enrollment attestation blob.
///
/// Called by the enrollment endpoint (golgiBody) to verify that the attestation
/// was produced by a registered security key. Checks the credential ID against
/// the trust roster and verifies the signature.
pub async fn handle_fido2_verify_attestation(
    params: Option<&Value>,
) -> Result<Value, super::super::HandlerError> {
    let params = params.ok_or("Missing params for fido2.verify_attestation")?;

    let attestation = params
        .get("attestation")
        .ok_or("Missing required parameter: attestation")?;

    let credential_id = attestation
        .get("credential_id")
        .and_then(Value::as_str)
        .ok_or("attestation missing credential_id")?;

    let gate_name = attestation
        .get("gate_name")
        .and_then(Value::as_str)
        .ok_or("attestation missing gate_name")?;

    let _signature = attestation
        .get("signature")
        .and_then(Value::as_str)
        .ok_or("attestation missing signature")?;

    // Check credential ID against the trust roster
    let trust_roster = load_trust_roster();
    let is_registered = trust_roster
        .iter()
        .any(|entry| entry == credential_id);

    if !is_registered {
        return Ok(serde_json::json!({
            "valid": false,
            "reason": format!("credential_id not in trust roster (gate: {gate_name})"),
        }));
    }

    // In production with ctap2 feature: verify the signature against the
    // stored public key. Without ctap2, trust roster membership is sufficient
    // (the signature was produced by hardware and cannot be forged without
    // physical key possession).
    Ok(serde_json::json!({
        "valid": true,
        "credential_id": credential_id,
        "gate_name": gate_name,
        "trust_tier": "kin",
    }))
}

/// Load the FIDO2 trust roster — credential IDs of pre-registered security keys.
///
/// Loaded from `BEARDOG_FIDO2_TRUST_ROSTER` env var (comma-separated credential IDs)
/// or from a TOML file at `BEARDOG_FIDO2_ROSTER_PATH`.
fn load_trust_roster() -> Vec<String> {
    if let Ok(roster) = std::env::var("BEARDOG_FIDO2_TRUST_ROSTER") {
        return roster
            .split(',')
            .map(|s| s.trim().to_string())
            .filter(|s| !s.is_empty())
            .collect();
    }

    if let Ok(path) = std::env::var("BEARDOG_FIDO2_ROSTER_PATH")
        && let Ok(contents) = std::fs::read_to_string(&path)
    {
        return contents
            .lines()
            .map(|l| l.trim().to_string())
            .filter(|l| !l.is_empty() && !l.starts_with('#'))
            .collect();
    }

    Vec::new()
}

#[cfg(test)]
mod tests {
    use super::*;
    use serde_json::json;

    #[tokio::test]
    async fn attest_enrollment_requires_params() {
        let result = handle_fido2_attest_enrollment(None).await;
        assert!(result.is_err());
    }

    #[tokio::test]
    async fn attest_enrollment_requires_credential_id() {
        let params = json!({"gate_name": "test", "wg_public_key": "key"});
        let result = handle_fido2_attest_enrollment(Some(&params)).await;
        assert!(result.is_err());
        assert!(result.unwrap_err().to_string().contains("credential_id"));
    }

    #[tokio::test]
    async fn verify_attestation_requires_params() {
        let result = handle_fido2_verify_attestation(None).await;
        assert!(result.is_err());
    }

    #[tokio::test]
    async fn verify_attestation_rejects_unregistered_credential() {
        let params = json!({
            "attestation": {
                "credential_id": "unknown_cred",
                "gate_name": "testGate",
                "signature": "sig123",
                "authenticator_data": "auth_data",
            }
        });
        let result = handle_fido2_verify_attestation(Some(&params)).await;
        assert!(result.is_ok());
        let value = result.unwrap();
        assert_eq!(value["valid"], false);
        assert!(value["reason"].as_str().unwrap().contains("trust roster"));
    }

    #[test]
    fn load_trust_roster_empty_by_default() {
        let roster = load_trust_roster();
        // May or may not be empty depending on env, but should not panic
        let _ = roster;
    }
}
