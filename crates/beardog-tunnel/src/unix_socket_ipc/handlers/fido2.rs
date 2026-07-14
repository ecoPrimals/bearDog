// SPDX-License-Identifier: AGPL-3.0-or-later

//! FIDO2/CTAP2 IPC handler — hardware-attested authentication and credential
//! management via USB security keys.
//!
//! Exposes `BearDog`'s CTAP2 infrastructure as JSON-RPC methods so downstream
//! primals (lithoSpore, etc.) can request hardware-attested signatures without
//! embedding HID or CTAP2 crate dependencies.
//!
//! # Methods
//!
//! - `beardog.fido2.discover` — enumerate connected FIDO2 devices
//! - `beardog.fido2.register` — create a credential (CTAP2 `MakeCredential`)
//! - `beardog.fido2.authenticate` — get an assertion (CTAP2 `GetAssertion`)
//!
//! # Feature Gate
//!
//! Real CTAP2 operations require the `fido2` feature. Without it, discovery
//! returns an empty list and credential operations return a clear capability
//! error guiding the caller to enable the feature.
//!
//! # Security Model
//!
//! - Physical presence is enforced by the authenticator (user touch)
//! - Private keys never leave the security key
//! - `BearDog` relays CTAP2 frames, never holds credential secrets

use super::{HandlerResult, MethodHandler};
use crate::btsp_provider::BeardogBtspProvider;
use serde_json::{Value, json};
use std::sync::Arc;
use tracing::info;

/// FIDO2/CTAP2 handler for hardware-attested authentication.
#[derive(Default)]
pub struct Fido2Handler;

impl Fido2Handler {
    /// Create a new FIDO2 handler.
    #[must_use]
    pub const fn new() -> Self {
        Self
    }
}

impl MethodHandler for Fido2Handler {
    fn methods(&self) -> Vec<&'static str> {
        vec![
            "beardog.fido2.discover",
            "beardog.fido2.register",
            "beardog.fido2.authenticate",
            "beardog.fido2.entropy",
        ]
    }

    async fn handle(
        &self,
        method: &str,
        params: Option<&Value>,
        _btsp_provider: &Arc<BeardogBtspProvider>,
    ) -> HandlerResult {
        match method {
            "beardog.fido2.discover" => handle_fido2_discover(params).await,
            "beardog.fido2.register" => handle_fido2_register(params).await,
            "beardog.fido2.authenticate" => handle_fido2_authenticate(params).await,
            "beardog.fido2.entropy" => handle_fido2_entropy(params).await,
            _ => Err(format!("Unknown FIDO2 method: {method}").into()),
        }
    }
}

/// Enumerate connected FIDO2/CTAP2 devices.
///
/// # Returns
///
/// ```json
/// {
///   "devices": [
///     {
///       "path": "/dev/hidraw3",
///       "vendor_id": 7504,
///       "product_id": 24705,
///       "product_name": "SoloKeys Solo 2",
///       "fido2": true
///     }
///   ],
///   "count": 1
/// }
/// ```
async fn handle_fido2_discover(_params: Option<&Value>) -> Result<Value, super::HandlerError> {
    #[cfg(feature = "ctap2")]
    {
        let raw_devices = beardog_hid::discover()
            .await
            .map_err(|e| format!("FIDO2 discovery failed: {e}"))?;
        let fido2_devices: Vec<Value> = raw_devices
            .iter()
            .filter(|d| beardog_hid::types::is_fido2_device(d.vendor_id, d.product_id))
            .map(|d| {
                json!({
                    "path": d.path,
                    "vendor_id": d.vendor_id.0,
                    "product_id": d.product_id.0,
                    "manufacturer": d.manufacturer,
                    "product": d.product,
                    "fido2": true,
                })
            })
            .collect();

        let count = fido2_devices.len();
        info!(count, "FIDO2 device discovery complete");

        Ok(json!({
            "devices": fido2_devices,
            "count": count,
        }))
    }

    #[cfg(not(feature = "ctap2"))]
    {
        info!("FIDO2 discovery: feature not enabled, returning empty");
        Ok(json!({
            "devices": [],
            "count": 0,
            "note": "FIDO2 feature not enabled — rebuild with --features ctap2",
        }))
    }
}

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
///
/// # Returns
///
/// ```json
/// {
///   "credential_id": "<base64>",
///   "public_key": "<base64 COSE key>",
///   "attestation_object": "<base64>",
///   "rp_id": "primals.eco"
/// }
/// ```
async fn handle_fido2_register(params: Option<&Value>) -> Result<Value, super::HandlerError> {
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

    #[cfg(feature = "ctap2")]
    {
        use base64::Engine;
        use base64::engine::general_purpose::STANDARD as BASE64;

        let device_path = resolve_device_path(_device_path).await?;

        let _user_id_bytes = BASE64
            .decode(user_id)
            .map_err(|e| format!("Invalid base64 user_id: {e}"))?;

        let provider = create_ctap2_provider(&device_path, rp_id).await?;

        use crate::tunnel::hsm::solo_v2::types::KeyType;
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
///   "authenticator_data": "<base64>",
///   "user_present": true,
///   "rp_id": "primals.eco",
///   "credential_id": "<base64>"
/// }
/// ```
async fn handle_fido2_authenticate(params: Option<&Value>) -> Result<Value, super::HandlerError> {
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

        let device_path = resolve_device_path(_device_path).await?;

        let credential_id_bytes = BASE64
            .decode(credential_id)
            .map_err(|e| format!("Invalid base64 credential_id: {e}"))?;

        let challenge_bytes = BASE64
            .decode(challenge)
            .map_err(|e| format!("Invalid base64 challenge: {e}"))?;

        let provider = create_ctap2_provider(&device_path, rp_id).await?;

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
            "FIDO2 feature not enabled — rebuild bearDog with --features fido2 to use \
             hardware security keys"
                .to_string()
                .into(),
        )
    }
}

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
async fn handle_fido2_entropy(params: Option<&Value>) -> Result<Value, super::HandlerError> {
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

        let device_path = resolve_device_path(_device_path).await?;

        let credential_id_bytes = BASE64
            .decode(credential_id)
            .map_err(|e| format!("Invalid base64 credential_id: {e}"))?;

        let challenge: [u8; 32] = rand::random();

        let provider = create_ctap2_provider(&device_path, rp_id).await?;

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
            "FIDO2 feature not enabled — rebuild bearDog with --features fido2 to use \
             hardware entropy from security keys"
                .to_string()
                .into(),
        )
    }
}

// ── Feature-gated helpers ────────────────────────────────────────────────

#[cfg(feature = "ctap2")]
async fn resolve_device_path(explicit: Option<&str>) -> Result<String, String> {
    if let Some(path) = explicit {
        return Ok(path.to_string());
    }
    let devices = beardog_hid::discover()
        .await
        .map_err(|e| format!("HID enumeration failed: {e}"))?;
    let fido2_device = devices
        .into_iter()
        .find(|d| beardog_hid::types::is_fido2_device(d.vendor_id, d.product_id))
        .ok_or(
            "No FIDO2 device found — connect a USB security key \
             (SoloKey, YubiKey, or other CTAP2 authenticator)"
                .to_string(),
        )?;
    Ok(fido2_device.path)
}

#[cfg(feature = "ctap2")]
async fn create_ctap2_provider(
    device_path: &str,
    rp_id: &str,
) -> Result<crate::tunnel::hsm::solo_v2::SoloV2Provider, String> {
    use crate::tunnel::hsm::solo_v2::SoloV2Provider;
    use crate::tunnel::hsm::solo_v2::types::{SoloV2Config, SoloV2DeviceInfo};

    let device_info = SoloV2DeviceInfo {
        device_id: device_path.to_string(),
        product_name: "FIDO2 device".to_string(),
        firmware_version: "unknown".to_string(),
        is_connected: true,
        vendor_id: 0,
        product_id: 0,
    };

    let config = SoloV2Config {
        device_id: Some(device_path.to_string()),
        relying_party_id: rp_id.to_string(),
        require_user_presence: true,
        require_user_verification: false,
        user_interaction_timeout: 30,
    };

    SoloV2Provider::with_hid_device_path(device_info, config, device_path)
        .await
        .map_err(|e| format!("Failed to open CTAP2 device at {device_path}: {e}"))
}

// ── Tests ────────────────────────────────────────────────────────────────

#[cfg(test)]
mod tests {
    use super::super::MethodHandler;
    use super::*;
    use serde_json::json;

    #[test]
    fn fido2_handler_method_list() {
        let handler = Fido2Handler::new();
        let methods = handler.methods();
        assert_eq!(methods.len(), 4);
        assert!(methods.contains(&"beardog.fido2.discover"));
        assert!(methods.contains(&"beardog.fido2.register"));
        assert!(methods.contains(&"beardog.fido2.authenticate"));
        assert!(methods.contains(&"beardog.fido2.entropy"));
    }

    #[tokio::test]
    async fn fido2_discover_returns_valid_response() {
        let result = handle_fido2_discover(None).await.expect("discover");
        assert!(result.get("devices").is_some());
        assert!(result.get("count").is_some());
        let count = result["count"].as_u64().expect("count is u64");
        let devices = result["devices"].as_array().expect("devices is array");
        assert_eq!(devices.len() as u64, count);
    }

    #[tokio::test]
    async fn fido2_register_requires_params() {
        let err = handle_fido2_register(None).await.expect_err("no params");
        assert!(err.contains("Missing params"));
    }

    #[tokio::test]
    async fn fido2_register_requires_rp_id() {
        let params = json!({"user_id": "dXNlcg==", "user_name": "test"});
        let err = handle_fido2_register(Some(&params))
            .await
            .expect_err("no rp_id");
        assert!(err.contains("rp_id"));
    }

    #[tokio::test]
    async fn fido2_register_requires_user_id() {
        let params = json!({"rp_id": "primals.eco", "user_name": "test"});
        let err = handle_fido2_register(Some(&params))
            .await
            .expect_err("no user_id");
        assert!(err.contains("user_id"));
    }

    #[tokio::test]
    async fn fido2_register_requires_user_name() {
        let params = json!({"rp_id": "primals.eco", "user_id": "dXNlcg=="});
        let err = handle_fido2_register(Some(&params))
            .await
            .expect_err("no user_name");
        assert!(err.contains("user_name"));
    }

    #[tokio::test]
    async fn fido2_authenticate_requires_params() {
        let err = handle_fido2_authenticate(None)
            .await
            .expect_err("no params");
        assert!(err.contains("Missing params"));
    }

    #[tokio::test]
    async fn fido2_authenticate_requires_rp_id() {
        let params = json!({"credential_id": "Y3JlZA==", "challenge": "Y2hhbA=="});
        let err = handle_fido2_authenticate(Some(&params))
            .await
            .expect_err("no rp_id");
        assert!(err.contains("rp_id"));
    }

    #[tokio::test]
    async fn fido2_authenticate_requires_credential_id() {
        let params = json!({"rp_id": "primals.eco", "challenge": "Y2hhbA=="});
        let err = handle_fido2_authenticate(Some(&params))
            .await
            .expect_err("no credential_id");
        assert!(err.contains("credential_id"));
    }

    #[tokio::test]
    async fn fido2_authenticate_requires_challenge() {
        let params = json!({"rp_id": "primals.eco", "credential_id": "Y3JlZA=="});
        let err = handle_fido2_authenticate(Some(&params))
            .await
            .expect_err("no challenge");
        assert!(err.contains("challenge"));
    }

    #[tokio::test]
    async fn fido2_entropy_requires_params() {
        let err = handle_fido2_entropy(None).await.expect_err("no params");
        assert!(err.contains("Missing params"));
    }

    #[tokio::test]
    async fn fido2_entropy_requires_rp_id() {
        let params = json!({"credential_id": "Y3JlZA=="});
        let err = handle_fido2_entropy(Some(&params))
            .await
            .expect_err("no rp_id");
        assert!(err.contains("rp_id"));
    }

    #[tokio::test]
    async fn fido2_entropy_requires_credential_id() {
        let params = json!({"rp_id": "primals.eco"});
        let err = handle_fido2_entropy(Some(&params))
            .await
            .expect_err("no credential_id");
        assert!(err.contains("credential_id"));
    }

    #[tokio::test]
    async fn fido2_handler_routes_discover() {
        let handler = Fido2Handler::new();
        let provider = crate::test_helpers::mocks::create_minimal_beardog_provider().await;
        let result = handler
            .handle("beardog.fido2.discover", None, &provider)
            .await
            .expect("discover via handler");
        assert!(result.get("devices").is_some());
    }

    #[tokio::test]
    async fn fido2_handler_unknown_method_errors() {
        let handler = Fido2Handler::new();
        let provider = crate::test_helpers::mocks::create_minimal_beardog_provider().await;
        let err = handler
            .handle("beardog.fido2.nonexistent", None, &provider)
            .await
            .expect_err("unknown method");
        assert!(err.contains("Unknown FIDO2 method"));
    }
}
