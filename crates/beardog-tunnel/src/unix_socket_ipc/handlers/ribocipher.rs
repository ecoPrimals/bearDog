// SPDX-License-Identifier: AGPL-3.0-or-later

//! riboCipher Transport Signal Handler
//!
//! Exposes riboCipher Tier 2 (mito-obfuscated) operations as Neural API
//! capabilities. biomeOS and cross-gate consumers call these methods to
//! decode/encode mito tags without needing direct access to the family seed.
//!
//! # Methods
//!
//! - `ribocipher.decode_mito_tag` — Decode a 4-byte HMAC tag to protocol type
//! - `ribocipher.encode_mito_signal` — Build a 5-byte mito-beacon signal
//! - `ribocipher.protocol_name` — Human-readable name for a protocol byte
//! - `ribocipher.list_protocols` — List all known protocol types
//!
//! # Security
//!
//! The `family_seed` parameter is required for encode/decode operations.
//! bearDog does NOT store or cache the seed — callers must supply it per-call.
//! The seed never appears in responses or logs.

use super::{HandlerError, HandlerResult, MethodHandler};
use crate::btsp_provider::BeardogBtspProvider;
use crate::ribocipher;
use base64::Engine;
use base64::engine::general_purpose::STANDARD as BASE64_STD;
use std::sync::Arc;
use tracing::info;

/// Handler for riboCipher transport signal operations.
#[derive(Default)]
pub struct RiboCipherHandler;

impl RiboCipherHandler {
    /// Create a new riboCipher handler.
    #[must_use]
    pub fn new() -> Self {
        Self
    }
}

impl MethodHandler for RiboCipherHandler {
    fn methods(&self) -> Vec<&'static str> {
        vec![
            "ribocipher.decode_mito_tag",
            "ribocipher.encode_mito_signal",
            "ribocipher.protocol_name",
            "ribocipher.list_protocols",
        ]
    }

    async fn handle(
        &self,
        method: &str,
        params: Option<&serde_json::Value>,
        _btsp_provider: &Arc<BeardogBtspProvider>,
    ) -> HandlerResult {
        match method {
            "ribocipher.decode_mito_tag" => handle_decode_mito_tag(params),
            "ribocipher.encode_mito_signal" => handle_encode_mito_signal(params),
            "ribocipher.protocol_name" => handle_protocol_name(params),
            "ribocipher.list_protocols" => handle_list_protocols(),
            other => Err(HandlerError::MethodNotFound(other.to_owned())),
        }
    }
}

/// Decode a 4-byte mito HMAC tag to determine the protocol type.
///
/// Params: `{ "family_seed": "<base64>", "tag": "<base64 or hex>" }`
/// Returns: `{ "protocol_type": <u8>, "protocol_name": "<str>" }` or `null` if
/// no known protocol matches.
fn handle_decode_mito_tag(params: Option<&serde_json::Value>) -> HandlerResult {
    let params = params.ok_or_else(|| HandlerError::InvalidParams("params required".into()))?;

    let seed_b64 = params
        .get("family_seed")
        .and_then(serde_json::Value::as_str)
        .ok_or_else(|| HandlerError::InvalidParams("family_seed (base64) required".into()))?;
    let seed = BASE64_STD
        .decode(seed_b64)
        .map_err(|e| HandlerError::InvalidParams(format!("family_seed base64 invalid: {e}")))?;

    let tag_bytes = extract_tag_bytes(params)?;

    info!("riboCipher: decode_mito_tag");

    if let Some(proto) = ribocipher::decode_mito_tag(&seed, &tag_bytes) {
        let name = ribocipher::protocol_name(proto);
        if let Some(gossip) = beardog_ipc::gossip() {
            gossip.mito_decoded(proto, name);
        }
        Ok(serde_json::json!({
            "protocol_type": proto,
            "protocol_name": name,
            "decoded": true,
        }))
    } else {
        if let Some(gossip) = beardog_ipc::gossip() {
            gossip.spread(
                beardog_ipc::gossip::topics::MITO_REJECTED,
                serde_json::json!({ "tag_hex": hex::encode(tag_bytes) }),
            );
        }
        Ok(serde_json::json!({
            "protocol_type": null,
            "protocol_name": null,
            "decoded": false,
        }))
    }
}

/// Encode a mito-beacon signal for a given protocol type and family seed.
///
/// Params: `{ "family_seed": "<base64>", "protocol_type": <u8> }`
/// Returns: `{ "signal": "<base64>", "signal_hex": "<hex>" }`
fn handle_encode_mito_signal(params: Option<&serde_json::Value>) -> HandlerResult {
    let params = params.ok_or_else(|| HandlerError::InvalidParams("params required".into()))?;

    let seed_b64 = params
        .get("family_seed")
        .and_then(serde_json::Value::as_str)
        .ok_or_else(|| HandlerError::InvalidParams("family_seed (base64) required".into()))?;
    let seed = BASE64_STD
        .decode(seed_b64)
        .map_err(|e| HandlerError::InvalidParams(format!("family_seed base64 invalid: {e}")))?;

    let proto = extract_protocol_type(params)?;

    info!(protocol_type = proto, "riboCipher: encode_mito_signal");

    let signal = ribocipher::mito_signal(&seed, proto);

    Ok(serde_json::json!({
        "signal": BASE64_STD.encode(signal),
        "signal_hex": hex::encode(signal),
        "protocol_type": proto,
        "protocol_name": ribocipher::protocol_name(proto),
    }))
}

/// Get the human-readable name for a protocol type byte.
///
/// Params: `{ "protocol_type": <u8> }`
/// Returns: `{ "protocol_name": "<str>" }`
fn handle_protocol_name(params: Option<&serde_json::Value>) -> HandlerResult {
    let params = params.ok_or_else(|| HandlerError::InvalidParams("params required".into()))?;
    let proto = extract_protocol_type(params)?;

    Ok(serde_json::json!({
        "protocol_type": proto,
        "protocol_name": ribocipher::protocol_name(proto),
    }))
}

/// List all known protocol types with names.
///
/// No params required.
fn handle_list_protocols() -> HandlerResult {
    let protocols: Vec<serde_json::Value> = [
        ribocipher::PROTO_PROBE,
        ribocipher::PROTO_NDJSON_JSONRPC,
        ribocipher::PROTO_BTSP_BINARY,
        ribocipher::PROTO_BTSP_JSONLINE,
        ribocipher::PROTO_HTTP,
        ribocipher::PROTO_ENCRYPTED_RESUME,
        ribocipher::PROTO_DARK_FOREST_BEACON,
        ribocipher::PROTO_MESH_RELAY,
    ]
    .iter()
    .map(|&p| {
        serde_json::json!({
            "protocol_type": p,
            "protocol_name": ribocipher::protocol_name(p),
        })
    })
    .collect();

    Ok(serde_json::json!({ "protocols": protocols }))
}

/// Extract `protocol_type` (u8) from JSON params.
fn extract_protocol_type(params: &serde_json::Value) -> Result<u8, HandlerError> {
    let v = params
        .get("protocol_type")
        .and_then(serde_json::Value::as_u64)
        .ok_or_else(|| HandlerError::InvalidParams("protocol_type (u8) required".into()))?;
    u8::try_from(v)
        .map_err(|_| HandlerError::InvalidParams(format!("protocol_type out of u8 range: {v}")))
}

/// Extract a 4-byte tag from params, accepting either base64 or hex encoding.
fn extract_tag_bytes(params: &serde_json::Value) -> Result<[u8; 4], HandlerError> {
    let tag_str = params
        .get("tag")
        .and_then(serde_json::Value::as_str)
        .ok_or_else(|| HandlerError::InvalidParams("tag (base64 or hex) required".into()))?;

    // Try hex first (8 hex chars = 4 bytes)
    if tag_str.len() == 8
        && let Ok(bytes) = hex::decode(tag_str)
        && let Ok(tag) = <[u8; 4]>::try_from(bytes.as_slice())
    {
        return Ok(tag);
    }

    // Try base64
    let bytes = BASE64_STD
        .decode(tag_str)
        .map_err(|e| HandlerError::InvalidParams(format!("tag decode failed: {e}")))?;
    if bytes.len() != 4 {
        return Err(HandlerError::InvalidParams(format!(
            "tag must be exactly 4 bytes, got {}",
            bytes.len()
        )));
    }
    let mut tag = [0u8; 4];
    tag.copy_from_slice(&bytes);
    Ok(tag)
}

#[cfg(test)]
mod tests {
    use super::*;

    async fn make_btsp() -> Arc<BeardogBtspProvider> {
        crate::test_helpers::mocks::create_minimal_beardog_provider().await
    }

    #[tokio::test]
    async fn decode_round_trip() {
        let handler = RiboCipherHandler::new();
        let btsp = make_btsp().await;
        let seed = BASE64_STD.encode(b"test-family-seed-32-bytes!!!!!!!");

        let enc_params = serde_json::json!({
            "family_seed": seed,
            "protocol_type": ribocipher::PROTO_NDJSON_JSONRPC,
        });
        let enc_result = handler
            .handle("ribocipher.encode_mito_signal", Some(&enc_params), &btsp)
            .await
            .expect("encode should succeed");

        let signal_hex = enc_result["signal_hex"].as_str().unwrap();
        assert!(signal_hex.starts_with("ed"), "signal should start with 0xED");

        let tag_hex = &signal_hex[2..10];
        let dec_params = serde_json::json!({
            "family_seed": seed,
            "tag": tag_hex,
        });
        let dec_result = handler
            .handle("ribocipher.decode_mito_tag", Some(&dec_params), &btsp)
            .await
            .expect("decode should succeed");

        assert_eq!(dec_result["decoded"], true);
        assert_eq!(dec_result["protocol_type"], ribocipher::PROTO_NDJSON_JSONRPC);
        assert_eq!(dec_result["protocol_name"], "ndjson-jsonrpc");
    }

    #[tokio::test]
    async fn decode_wrong_seed_returns_not_decoded() {
        let handler = RiboCipherHandler::new();
        let btsp = make_btsp().await;
        let seed_a = BASE64_STD.encode(b"seed-alpha-32-bytes-padding!!!!");
        let seed_b = BASE64_STD.encode(b"seed-bravo-32-bytes-padding!!!!");

        let enc_params = serde_json::json!({
            "family_seed": seed_a,
            "protocol_type": ribocipher::PROTO_BTSP_BINARY,
        });
        let enc_result = handler
            .handle("ribocipher.encode_mito_signal", Some(&enc_params), &btsp)
            .await
            .unwrap();

        let signal_hex = enc_result["signal_hex"].as_str().unwrap();
        let tag_hex = &signal_hex[2..10];

        let dec_params = serde_json::json!({
            "family_seed": seed_b,
            "tag": tag_hex,
        });
        let dec_result = handler
            .handle("ribocipher.decode_mito_tag", Some(&dec_params), &btsp)
            .await
            .unwrap();

        assert_eq!(dec_result["decoded"], false);
        assert!(dec_result["protocol_type"].is_null());
    }

    #[tokio::test]
    async fn protocol_name_returns_known() {
        let handler = RiboCipherHandler::new();
        let btsp = make_btsp().await;
        let params = serde_json::json!({"protocol_type": 0x01});
        let result = handler
            .handle("ribocipher.protocol_name", Some(&params), &btsp)
            .await
            .unwrap();
        assert_eq!(result["protocol_name"], "ndjson-jsonrpc");
    }

    #[tokio::test]
    async fn list_protocols_returns_all() {
        let handler = RiboCipherHandler::new();
        let btsp = make_btsp().await;
        let result = handler
            .handle("ribocipher.list_protocols", None, &btsp)
            .await
            .unwrap();
        let protocols = result["protocols"].as_array().unwrap();
        assert_eq!(protocols.len(), 8);
        assert_eq!(protocols[0]["protocol_name"], "probe");
        assert_eq!(protocols[1]["protocol_name"], "ndjson-jsonrpc");
    }

    #[tokio::test]
    async fn unknown_method_returns_error() {
        let handler = RiboCipherHandler::new();
        let btsp = make_btsp().await;
        let result = handler
            .handle("ribocipher.nonexistent", None, &btsp)
            .await;
        assert!(result.is_err());
    }

    #[tokio::test]
    async fn decode_missing_params_returns_error() {
        let handler = RiboCipherHandler::new();
        let btsp = make_btsp().await;
        let result = handler
            .handle("ribocipher.decode_mito_tag", None, &btsp)
            .await;
        assert!(result.is_err());
    }
}
