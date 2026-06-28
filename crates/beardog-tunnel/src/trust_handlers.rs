// SPDX-License-Identifier: AGPL-3.0-or-later

//! JSON-RPC handlers for cross-gate trust management.
//!
//! These handlers manage the trust lifecycle between gates in the covalent mesh:
//!
//! - `auth.trust_issuer` — register a remote gate's Ed25519 key as trusted
//! - `auth.trusted_issuers` — list all registered trusted issuers
//! - `auth.exchange_trust` — bidirectional key exchange for mesh auto-join
//! - `auth.events.poll` — poll trust lifecycle events
//!
//! Extracted from `ionic_token_handlers` to keep token lifecycle (issuance,
//! verification) separate from trust topology management.

use crate::auth_event_bus::{AuthEvent, AuthEventBus, AuthEventKind};
use crate::method_gate::{CallerContext, ConnectionOrigin};
use crate::trusted_issuer_registry::{TrustedIssuerRegistry, did_from_verifying_key};
use crate::unix_socket_ipc::handlers::primal_signing::derive_primal_signing_key;
use base64::Engine;
use ed25519_dalek::VerifyingKey;
use serde_json::Value;

const B64: &base64::engine::GeneralPurpose = &base64::engine::general_purpose::STANDARD;

/// Derive the primal's own DID from its identity key.
fn primal_did(primal_name: &str, node_id: &str) -> String {
    let sk = derive_primal_signing_key(primal_name, node_id);
    let vk = sk.verifying_key();
    did_from_verifying_key(&vk)
}

// ── auth.trust_issuer ──────────────────────────────────────────────────

/// Handle `auth.trust_issuer` — register a remote gate's public key as trusted.
///
/// # Params
///
/// - `public_key` (string, required): base64-encoded Ed25519 public key
/// - `did` (string, required): issuer DID (`did:key:z6Mk...`)
/// - `gate_id` (string, optional): remote gate's `NODE_ID`
/// - `family_id` (string, optional): remote gate's `FAMILY_ID`
/// - `trust_method` (string, optional): `"family_seed"` | `"contract_exchange"` | `"manual"`
#[must_use]
pub fn handle_auth_trust_issuer(
    registry: &TrustedIssuerRegistry,
    event_bus: &AuthEventBus,
    source_gate: &str,
    caller: &CallerContext,
    params: Option<&Value>,
) -> Value {
    if !caller.btsp_family_verified
        && caller.validated_claims.is_none()
        && caller.origin != ConnectionOrigin::Unix
    {
        return serde_json::json!({
            "registered": false,
            "error": "auth.trust_issuer requires BTSP-authenticated channel, valid ionic token, or co-resident UDS",
        });
    }

    let Some(pk_b64) = params
        .and_then(|p| p.get("public_key"))
        .and_then(Value::as_str)
    else {
        return serde_json::json!({ "registered": false, "error": "missing required parameter: public_key" });
    };
    let Some(did) = params.and_then(|p| p.get("did")).and_then(Value::as_str) else {
        return serde_json::json!({ "registered": false, "error": "missing required parameter: did" });
    };

    let pk_bytes = match B64.decode(pk_b64) {
        Ok(b) => b,
        Err(e) => {
            return serde_json::json!({ "registered": false, "error": format!("invalid base64: {e}") });
        }
    };
    let arr: [u8; 32] = match pk_bytes.try_into() {
        Ok(a) => a,
        Err(_) => {
            return serde_json::json!({ "registered": false, "error": "public_key must be 32 bytes" });
        }
    };
    let vk = match VerifyingKey::from_bytes(&arr) {
        Ok(k) => k,
        Err(e) => {
            return serde_json::json!({ "registered": false, "error": format!("invalid Ed25519 key: {e}") });
        }
    };

    let gate_id = params
        .and_then(|p| p.get("gate_id"))
        .and_then(Value::as_str)
        .map(String::from);
    let family_id = params
        .and_then(|p| p.get("family_id"))
        .and_then(Value::as_str)
        .map(String::from);

    let method = match params
        .and_then(|p| p.get("trust_method"))
        .and_then(Value::as_str)
    {
        Some("family_seed") => crate::trusted_issuer_registry::TrustMethod::FamilySeed,
        Some("contract_exchange") => crate::trusted_issuer_registry::TrustMethod::ContractExchange,
        _ => crate::trusted_issuer_registry::TrustMethod::Manual,
    };

    match registry.register(did, vk, gate_id.clone(), family_id.clone(), method) {
        Ok(newly_registered) => {
            if newly_registered {
                let fingerprint = hex::encode(&vk.as_bytes()[..16]);
                event_bus.emit(AuthEvent {
                    kind: AuthEventKind::TrustIssuerRegistered {
                        issuer_did: did.to_owned(),
                        issuer_fingerprint: fingerprint,
                        trust_method: method.as_str().to_owned(),
                    },
                    source_gate: source_gate.to_owned(),
                    timestamp: chrono::Utc::now().timestamp(),
                });
            }
            serde_json::json!({
                "registered": newly_registered,
                "did": did,
                "gate_id": gate_id,
                "family_id": family_id,
                "trust_method": method.as_str(),
                "total_trusted_issuers": registry.len(),
            })
        }
        Err(e) => serde_json::json!({
            "registered": false,
            "error": e.to_string(),
        }),
    }
}

// ── auth.events.poll ──────────────────────────────────────────────────

/// Handle `auth.events.poll` — return auth events since a given timestamp.
///
/// # Parameters
///
/// - `since_timestamp` (integer, optional): Unix seconds. Defaults to 0 (all events).
///
/// # Returns
///
/// - `events`: array of auth events matching the FRAGO wire format
/// - `count`: number of events returned
#[must_use]
pub fn handle_auth_events_poll(event_bus: &AuthEventBus, params: Option<&Value>) -> Value {
    let since = params
        .and_then(|p| p.get("since_timestamp"))
        .and_then(Value::as_i64)
        .unwrap_or(0);

    let events = event_bus.poll_since(since);
    let count = events.len();

    serde_json::json!({
        "events": events,
        "count": count,
        "since_timestamp": since,
    })
}

// ── auth.exchange_trust ───────────────────────────────────────────────

/// Handle `auth.exchange_trust` — bidirectional trust exchange for mesh auto-join.
///
/// Requires a BTSP-authenticated channel (proves family seed membership)
/// OR a valid ionic token. When called:
///
/// 1. Auto-registers the caller's Ed25519 public key as a trusted issuer
/// 2. Returns this gate's Ed25519 public key + DID so the caller can
///    register it on their end
///
/// # Parameters
///
/// - `public_key` (string, required): base64-encoded Ed25519 public key of the remote gate
/// - `did` (string, optional): remote gate's `did:key:z6Mk...` — derived from `public_key` if absent
/// - `gate_id` (string, optional): remote gate's node ID
/// - `family_id` (string, optional): remote gate's family ID
///
/// # Returns
///
/// - `registered`: whether the remote key was newly registered
/// - `local_public_key`: this gate's Ed25519 public key (base64)
/// - `local_did`: this gate's DID
/// - `local_gate_id`: this gate's node ID
///
/// # Errors
///
/// Returns error if caller is not BTSP-authenticated and has no valid ionic token.
#[must_use]
pub fn handle_auth_exchange_trust(
    registry: &TrustedIssuerRegistry,
    event_bus: &AuthEventBus,
    primal_name: &str,
    node_id: &str,
    caller: &CallerContext,
    params: Option<&Value>,
) -> Value {
    if !caller.btsp_family_verified && caller.validated_claims.is_none() {
        return serde_json::json!({
            "error": "auth.exchange_trust requires BTSP-authenticated channel or valid ionic token",
            "registered": false,
        });
    }

    let Some(pk_b64) = params
        .and_then(|p| p.get("public_key"))
        .and_then(Value::as_str)
    else {
        return serde_json::json!({
            "error": "missing required parameter: public_key",
            "registered": false,
        });
    };

    let pk_bytes = match B64.decode(pk_b64) {
        Ok(b) => b,
        Err(e) => {
            return serde_json::json!({ "registered": false, "error": format!("invalid base64: {e}") });
        }
    };
    let arr: [u8; 32] = match pk_bytes.try_into() {
        Ok(a) => a,
        Err(_) => {
            return serde_json::json!({ "registered": false, "error": "public_key must be 32 bytes" });
        }
    };
    let remote_vk = match VerifyingKey::from_bytes(&arr) {
        Ok(k) => k,
        Err(e) => {
            return serde_json::json!({ "registered": false, "error": format!("invalid Ed25519 key: {e}") });
        }
    };

    let remote_did = params
        .and_then(|p| p.get("did"))
        .and_then(Value::as_str)
        .map_or_else(|| did_from_verifying_key(&remote_vk), String::from);

    let remote_gate_id = params
        .and_then(|p| p.get("gate_id"))
        .and_then(Value::as_str)
        .map(String::from);
    let remote_family_id = params
        .and_then(|p| p.get("family_id"))
        .and_then(Value::as_str)
        .map(String::from);

    let trust_method = if caller.btsp_family_verified {
        crate::trusted_issuer_registry::TrustMethod::FamilySeed
    } else {
        crate::trusted_issuer_registry::TrustMethod::ContractExchange
    };

    let registered = match registry.register(
        &remote_did,
        remote_vk,
        remote_gate_id,
        remote_family_id,
        trust_method,
    ) {
        Ok(newly_registered) => {
            if newly_registered {
                let fingerprint = hex::encode(&remote_vk.as_bytes()[..16]);
                event_bus.emit(AuthEvent {
                    kind: AuthEventKind::TrustIssuerRegistered {
                        issuer_did: remote_did.clone(),
                        issuer_fingerprint: fingerprint,
                        trust_method: trust_method.as_str().to_owned(),
                    },
                    source_gate: primal_name.to_owned(),
                    timestamp: chrono::Utc::now().timestamp(),
                });
            }
            newly_registered
        }
        Err(e) => {
            return serde_json::json!({
                "registered": false,
                "error": e.to_string(),
            });
        }
    };

    let local_sk = derive_primal_signing_key(primal_name, node_id);
    let local_vk = local_sk.verifying_key();
    let local_did = primal_did(primal_name, node_id);

    serde_json::json!({
        "registered": registered,
        "remote_did": remote_did,
        "trust_method": trust_method.as_str(),
        "total_trusted_issuers": registry.len(),
        "local_public_key": B64.encode(local_vk.as_bytes()),
        "local_did": local_did,
        "local_gate_id": node_id,
    })
}

// ── auth.trusted_issuers ───────────────────────────────────────────────

/// Handle `auth.trusted_issuers` — list all registered trusted issuers.
#[must_use]
pub fn handle_auth_trusted_issuers(registry: &TrustedIssuerRegistry) -> Value {
    let issuers: Vec<Value> = registry
        .list()
        .into_iter()
        .map(|info| {
            serde_json::json!({
                "did": info.did,
                "gate_id": info.gate_id,
                "family_id": info.family_id,
                "trust_method": info.trust_method.as_str(),
                "registered_at": info.registered_at,
            })
        })
        .collect();

    serde_json::json!({
        "issuers": issuers,
        "count": issuers.len(),
    })
}
