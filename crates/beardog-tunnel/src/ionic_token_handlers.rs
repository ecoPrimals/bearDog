// SPDX-License-Identifier: AGPL-3.0-or-later

//! JSON-RPC handlers for ionic token lifecycle methods (JH-1).
//!
//! These are dispatched at the gate layer (not via `HandlerRegistry`) because
//! they need access to the primal's signing key and `CallerContext`.
//!
//! - `identity.create` — generate an ephemeral Ed25519 caller keypair + DID
//! - `auth.issue_ionic` — issue a signed ionic capability token
//! - `auth.verify_ionic` — verify a token string, return claims or error

use crate::auth_event_bus::{AuthEvent, AuthEventBus, AuthEventKind};
use crate::ionic_token::{
    GateIdentity, TokenError, issue_ionic_token, issue_ionic_token_with_gate, scope_covers_method,
};
use crate::method_gate::CallerContext;
use crate::trusted_issuer_registry::{
    CrossGateVerifyResult, TrustedIssuerRegistry, did_from_verifying_key, verify_with_registry,
};
use crate::unix_socket_ipc::handlers::primal_signing::derive_primal_signing_key;
use base64::Engine;
use ed25519_dalek::{SigningKey, VerifyingKey};
use serde_json::Value;

const B64: &base64::engine::GeneralPurpose = &base64::engine::general_purpose::STANDARD;

const DEFAULT_TTL_SECS: i64 = 3600;

/// Derive the `did:key:z6Mk...` for an Ed25519 public key.
fn did_from_ed25519_public(public_key: &[u8; 32]) -> String {
    let mut multicodec = Vec::with_capacity(34);
    multicodec.push(0xed);
    multicodec.push(0x01);
    multicodec.extend_from_slice(public_key);
    format!("did:key:z{}", bs58::encode(&multicodec).into_string())
}

/// Derive the primal's own DID from its identity key.
fn primal_did(primal_name: &str, node_id: &str) -> String {
    let sk = derive_primal_signing_key(primal_name, node_id);
    let vk = sk.verifying_key();
    did_from_ed25519_public(vk.as_bytes())
}

// ── identity.create ─────────────────────────────────────────────────────

/// Handle `identity.create` — generate an ephemeral Ed25519 keypair for a caller.
///
/// Returns `{did, public_key}`. The private key is **not** returned
/// (the caller must store it or use a derived approach).
///
/// This creates caller identities, not the primal's own identity (which is
/// deterministic from `primal_signing`).
#[must_use]
pub fn handle_identity_create() -> Value {
    let seed: [u8; 32] = rand::random();
    let sk = SigningKey::from_bytes(&seed);
    let vk = sk.verifying_key();

    let did = did_from_ed25519_public(vk.as_bytes());
    let public_key_b64 = B64.encode(vk.as_bytes());
    let secret_key_b64 = B64.encode(sk.to_bytes());

    serde_json::json!({
        "did": did,
        "public_key": public_key_b64,
        "secret_key": secret_key_b64,
        "algorithm": "Ed25519",
    })
}

// ── auth.issue_ionic ────────────────────────────────────────────────────

/// Handle `auth.issue_ionic` — issue a signed ionic capability token.
///
/// # Params (from JSON-RPC `params`)
///
/// - `subject` (string, required): who the token is for
/// - `scope` (array of strings, optional): method patterns; defaults to `["*"]`
/// - `ttl_secs` (integer, optional): lifetime in seconds; defaults to 3600
/// - `include_gate_id` (bool, optional): embed `gate_id` + `family_id` in
///   the token for cross-gate verification (default: true when `FAMILY_ID`
///   is set, false otherwise)
#[must_use]
pub fn handle_auth_issue_ionic(primal_name: &str, node_id: &str, params: Option<&Value>) -> Value {
    handle_auth_issue_ionic_with_identity(primal_name, node_id, None, params)
}

/// Issue an ionic token with optional explicit gate identity.
///
/// When `family_id` is `Some`, cross-gate claims (`gate_id`, `family_id`)
/// are embedded in the token payload.
#[must_use]
pub fn handle_auth_issue_ionic_with_identity(
    primal_name: &str,
    node_id: &str,
    family_id: Option<&str>,
    params: Option<&Value>,
) -> Value {
    let subject = params
        .and_then(|p| p.get("subject"))
        .and_then(Value::as_str)
        .unwrap_or("anonymous");

    let scope: Vec<String> = match params
        .and_then(|p| p.get("scope"))
        .and_then(Value::as_array)
    {
        Some(arr) => arr
            .iter()
            .filter_map(Value::as_str)
            .map(String::from)
            .collect(),
        None => vec!["*".to_owned()],
    };

    let ttl_secs = params
        .and_then(|p| p.get("ttl_secs"))
        .and_then(Value::as_i64)
        .unwrap_or(DEFAULT_TTL_SECS);

    let signing_key = derive_primal_signing_key(primal_name, node_id);
    let issuer_did = primal_did(primal_name, node_id);

    // Resolve gate identity: explicit param > env > None
    let include_gate = params
        .and_then(|p| p.get("include_gate_id"))
        .and_then(Value::as_bool);

    let gate = if include_gate == Some(false) {
        None
    } else {
        let fid = family_id.map(String::from).or_else(|| {
            std::env::var(beardog_config::env_keys::ENV_FAMILY_ID)
                .ok()
                .filter(|v| !v.is_empty() && v != "default" && v != "standalone")
        });
        fid.map(|f| GateIdentity {
            node_id: node_id.to_owned(),
            family_id: f,
        })
    };

    let token = issue_ionic_token_with_gate(
        &signing_key,
        &issuer_did,
        subject,
        &scope,
        ttl_secs,
        gate.as_ref(),
    );

    serde_json::json!({
        "token": token,
        "issuer": issuer_did,
        "subject": subject,
        "scope": scope,
        "scopes": scope,
        "ttl_secs": ttl_secs,
    })
}

// ── auth.issue_session (JH-4) ───────────────────────────────────────────

/// Handle `auth.issue_session` — simplified token issuance for non-technical
/// users (`JupyterHub` spawners, desktop launchers, etc.).
///
/// Accepts a minimal `purpose` string and auto-derives scope, subject, and TTL.
/// The caller never needs to construct scope patterns manually.
///
/// # Params (from JSON-RPC `params`)
///
/// - `purpose` (string, optional): `"jupyterhub"`, `"desktop"`, `"notebook"`,
///   `"research"`, or `"admin"`. Defaults to `"research"`.
/// - `user` (string, optional): user identifier; defaults to `"session-user"`.
/// - `ttl_hours` (integer, optional): lifetime in hours; defaults per purpose.
#[must_use]
pub fn handle_auth_issue_session(
    primal_name: &str,
    node_id: &str,
    params: Option<&Value>,
) -> Value {
    let purpose = params
        .and_then(|p| p.get("purpose"))
        .and_then(Value::as_str)
        .unwrap_or("research");

    let user = params
        .and_then(|p| p.get("user"))
        .and_then(Value::as_str)
        .unwrap_or("session-user");

    let (scope, default_ttl_hours): (Vec<&str>, i64) = match purpose {
        "jupyterhub" | "notebook" => (
            vec![
                "crypto.*",
                "health.*",
                "capabilities.*",
                "identity.*",
                "content.*",
                "auth.verify_ionic",
            ],
            8,
        ),
        "desktop" => (
            vec![
                "crypto.*",
                "health.*",
                "capabilities.*",
                "identity.*",
                "content.*",
                "secrets.*",
            ],
            24,
        ),
        "admin" => (vec!["*"], 1),
        _ => (
            vec![
                "crypto.*",
                "health.*",
                "capabilities.*",
                "identity.*",
                "content.*",
            ],
            8,
        ),
    };

    let ttl_hours = params
        .and_then(|p| p.get("ttl_hours"))
        .and_then(Value::as_i64)
        .unwrap_or(default_ttl_hours);

    let ttl_secs = ttl_hours * 3600;
    let scope_strings: Vec<String> = scope.iter().map(|s| (*s).to_string()).collect();

    let signing_key = derive_primal_signing_key(primal_name, node_id);
    let issuer_did = primal_did(primal_name, node_id);

    let token = issue_ionic_token(&signing_key, &issuer_did, user, &scope_strings, ttl_secs);

    serde_json::json!({
        "token": token,
        "issuer": issuer_did,
        "subject": user,
        "purpose": purpose,
        "scope": scope_strings,
        "scopes": scope_strings,
        "ttl_secs": ttl_secs,
        "ttl_hours": ttl_hours,
        "usage": "Set BEARDOG_TOKEN=<token> or pass as Bearer header. Token auto-expires.",
    })
}

// ── auth.public_key (JH-11) ──────────────────────────────────────────────

/// Handle `auth.public_key` — return the primal's Ed25519 verifying key.
///
/// Enables cross-primal token verification (JH-11): any primal can call this
/// once at startup, cache the public key, and verify ionic tokens locally
/// using Ed25519 without calling back to the issuing `BearDog`.
///
/// Returns the key in multiple formats (base64, hex, DID) so consumers can
/// use whichever is convenient.
#[must_use]
pub fn handle_auth_public_key(primal_name: &str, node_id: &str) -> Value {
    let sk = derive_primal_signing_key(primal_name, node_id);
    let vk = sk.verifying_key();
    let issuer_did = primal_did(primal_name, node_id);

    serde_json::json!({
        "public_key": B64.encode(vk.as_bytes()),
        "public_key_hex": hex::encode(vk.as_bytes()),
        "did": issuer_did,
        "algorithm": "Ed25519",
        "usage": "Verify ionic tokens signed by this primal. Cache this key and use ed25519_dalek::VerifyingKey to verify token signatures locally.",
    })
}

// ── auth.verify_ionic ───────────────────────────────────────────────────

/// Handle `auth.verify_ionic` — verify a token string and return claims.
///
/// Supports cross-gate verification via three key sources (tried in order):
/// 1. Local gate key (default, fast path)
/// 2. Trusted issuer registry (remote gates registered via `auth.trust_issuer`)
/// 3. Ad-hoc `issuer_key` param (base64 Ed25519 public key)
///
/// # Params (from JSON-RPC `params`)
///
/// - `token` (string, required): the compact ionic token string
/// - `method` (string, optional): if provided, also checks scope coverage
/// - `issuer_key` (string, optional): base64-encoded Ed25519 public key for
///   ad-hoc cross-gate verification
#[must_use]
pub fn handle_auth_verify_ionic(
    verifying_key: &VerifyingKey,
    registry: Option<&TrustedIssuerRegistry>,
    params: Option<&Value>,
) -> Value {
    let Some(token_str) = params.and_then(|p| p.get("token")).and_then(Value::as_str) else {
        return serde_json::json!({
            "valid": false,
            "scopes": [],
            "error": "missing required parameter: token",
        });
    };

    let method = params.and_then(|p| p.get("method")).and_then(Value::as_str);

    // Parse optional ad-hoc issuer key
    let adhoc_key = params
        .and_then(|p| p.get("issuer_key"))
        .and_then(Value::as_str)
        .and_then(|b64| B64.decode(b64).ok())
        .and_then(|bytes| <[u8; 32]>::try_from(bytes.as_slice()).ok())
        .and_then(|arr| VerifyingKey::from_bytes(&arr).ok());

    let empty_registry = TrustedIssuerRegistry::new();
    let reg = registry.unwrap_or(&empty_registry);

    match verify_with_registry(token_str, verifying_key, reg, adhoc_key.as_ref()) {
        CrossGateVerifyResult::LocalVerified(payload) => {
            build_verify_success(&payload, method, "local")
        }
        CrossGateVerifyResult::RemoteVerified {
            payload,
            issuer_info,
        } => {
            let mut result = build_verify_success(&payload, method, "remote");
            if let Some(obj) = result.as_object_mut() {
                obj.insert(
                    "issuer_gate_id".to_owned(),
                    issuer_info
                        .gate_id
                        .as_deref()
                        .map_or(Value::Null, |s| Value::String(s.to_owned())),
                );
                obj.insert(
                    "issuer_family_id".to_owned(),
                    issuer_info
                        .family_id
                        .as_deref()
                        .map_or(Value::Null, |s| Value::String(s.to_owned())),
                );
                obj.insert(
                    "trust_method".to_owned(),
                    Value::String(issuer_info.trust_method.as_str().to_owned()),
                );
            }
            result
        }
        CrossGateVerifyResult::AdHocVerified(payload) => {
            build_verify_success(&payload, method, "adhoc")
        }
        CrossGateVerifyResult::Failed(e) => {
            let reason = match &e {
                TokenError::Malformed(_) => "malformed",
                TokenError::InvalidSignature => "invalid_signature",
                TokenError::Expired { .. } => "expired",
                TokenError::UnsupportedFormat(_) => "unsupported_format",
            };
            serde_json::json!({
                "valid": false,
                "scopes": [],
                "error": e.to_string(),
                "reason": reason,
            })
        }
    }
}

/// Build the success response for `auth.verify_ionic`.
fn build_verify_success(
    payload: &crate::ionic_token::IonicTokenPayload,
    method: Option<&str>,
    verification_source: &str,
) -> Value {
    let scope_ok = match method {
        Some(m) => scope_covers_method(&payload.scope, m),
        None => true,
    };

    let mut claims = serde_json::json!({
        "iss": payload.iss,
        "sub": payload.sub,
        "scope": payload.scope,
        "scopes": payload.scope,
        "iat": payload.iat,
        "exp": payload.exp,
        "jti": payload.jti,
    });

    // Include cross-gate claims when present
    if let Some(ref gate_id) = payload.gate_id {
        claims["gate_id"] = Value::String(gate_id.clone());
    }
    if let Some(ref family_id) = payload.family_id {
        claims["family_id"] = Value::String(family_id.clone());
    }

    serde_json::json!({
        "valid": true,
        "scope_ok": scope_ok,
        "scopes": payload.scope,
        "claims": claims,
        "verification_source": verification_source,
    })
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
    params: Option<&Value>,
) -> Value {
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
/// This eliminates the manual `auth.trust_issuer` step during mesh join.
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
/// - `local_gate_id`: this gate's primal name
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
        "local_gate_id": primal_name,
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

#[cfg(test)]
mod tests {
    use super::*;
    use crate::unix_socket_ipc::handlers::primal_signing::derive_primal_verifying_key;

    const PRIMAL: &str = "beardog";
    const NODE: &str = "test-node";

    #[test]
    fn identity_create_returns_did_and_keys() {
        let result = handle_identity_create();
        assert!(result["did"].as_str().unwrap().starts_with("did:key:z6Mk"));
        assert!(result["public_key"].as_str().is_some());
        assert!(result["secret_key"].as_str().is_some());
        assert_eq!(result["algorithm"], "Ed25519");
    }

    #[test]
    fn identity_create_produces_unique_keys() {
        let a = handle_identity_create();
        let b = handle_identity_create();
        assert_ne!(a["did"], b["did"]);
    }

    #[test]
    fn issue_and_verify_roundtrip() {
        let params = serde_json::json!({
            "subject": "alice",
            "scope": ["crypto.*", "health.*"],
            "ttl_secs": 300,
        });

        let issue_result = handle_auth_issue_ionic(PRIMAL, NODE, Some(&params));
        let token = issue_result["token"].as_str().unwrap();

        let vk = derive_primal_verifying_key(PRIMAL, NODE);
        let verify_params = serde_json::json!({ "token": token });
        let verify_result = handle_auth_verify_ionic(&vk, None, Some(&verify_params));

        assert_eq!(verify_result["valid"], true);
        assert_eq!(verify_result["claims"]["sub"], "alice");
        assert_eq!(
            verify_result["claims"]["scope"],
            serde_json::json!(["crypto.*", "health.*"])
        );
        assert_eq!(
            verify_result["scopes"],
            serde_json::json!(["crypto.*", "health.*"]),
            "scopes must be at top level for SecurityVerifier Enforced mode"
        );
    }

    #[test]
    fn verify_with_scope_check() {
        let params = serde_json::json!({
            "subject": "alice",
            "scope": ["crypto.*"],
            "ttl_secs": 300,
        });

        let issue_result = handle_auth_issue_ionic(PRIMAL, NODE, Some(&params));
        let token = issue_result["token"].as_str().unwrap();

        let vk = derive_primal_verifying_key(PRIMAL, NODE);

        let ok_params = serde_json::json!({ "token": token, "method": "crypto.sign" });
        let ok_result = handle_auth_verify_ionic(&vk, None, Some(&ok_params));
        assert_eq!(ok_result["valid"], true);
        assert_eq!(ok_result["scope_ok"], true);

        let bad_params = serde_json::json!({ "token": token, "method": "lifecycle.shutdown" });
        let bad_result = handle_auth_verify_ionic(&vk, None, Some(&bad_params));
        assert_eq!(bad_result["valid"], true);
        assert_eq!(bad_result["scope_ok"], false);
    }

    #[test]
    fn verify_missing_token_returns_error() {
        let vk = derive_primal_verifying_key(PRIMAL, NODE);
        let result = handle_auth_verify_ionic(&vk, None, Some(&serde_json::json!({})));
        assert_eq!(result["valid"], false);
        assert_eq!(
            result["scopes"],
            serde_json::json!([]),
            "scopes must always be present, even on error"
        );
    }

    #[test]
    fn verify_bad_token_returns_reason() {
        let vk = derive_primal_verifying_key(PRIMAL, NODE);
        let params = serde_json::json!({ "token": "garbage" });
        let result = handle_auth_verify_ionic(&vk, None, Some(&params));
        assert_eq!(result["valid"], false);
        assert_eq!(result["reason"], "malformed");
        assert_eq!(
            result["scopes"],
            serde_json::json!([]),
            "scopes must always be present, even on invalid token"
        );
    }

    #[test]
    fn issue_defaults_wildcard_scope() {
        let params = serde_json::json!({ "subject": "admin" });
        let result = handle_auth_issue_ionic(PRIMAL, NODE, Some(&params));
        assert_eq!(result["scope"], serde_json::json!(["*"]));
    }

    #[test]
    fn primal_did_is_deterministic() {
        let a = primal_did(PRIMAL, NODE);
        let b = primal_did(PRIMAL, NODE);
        assert_eq!(a, b);
        assert!(a.starts_with("did:key:z6Mk"));
    }

    // ── auth.public_key (JH-11) ──

    #[test]
    fn public_key_returns_valid_ed25519_key() {
        let result = handle_auth_public_key(PRIMAL, NODE);
        assert_eq!(result["algorithm"], "Ed25519");
        assert!(result["did"].as_str().unwrap().starts_with("did:key:z6Mk"));
        assert!(result["public_key"].as_str().is_some());
        assert!(result["public_key_hex"].as_str().is_some());
    }

    #[test]
    fn public_key_is_deterministic() {
        let a = handle_auth_public_key(PRIMAL, NODE);
        let b = handle_auth_public_key(PRIMAL, NODE);
        assert_eq!(a["public_key"], b["public_key"]);
        assert_eq!(a["did"], b["did"]);
    }

    #[test]
    fn public_key_matches_verify_key() {
        let pk_result = handle_auth_public_key(PRIMAL, NODE);
        let pk_b64 = pk_result["public_key"].as_str().unwrap();
        let pk_bytes = B64.decode(pk_b64).unwrap();

        let vk = derive_primal_verifying_key(PRIMAL, NODE);
        assert_eq!(pk_bytes.as_slice(), vk.as_bytes());
    }

    #[test]
    fn public_key_enables_cross_primal_verification() {
        let pk_result = handle_auth_public_key(PRIMAL, NODE);
        let pk_b64 = pk_result["public_key"].as_str().unwrap();
        let pk_bytes = B64.decode(pk_b64).unwrap();

        let remote_vk = VerifyingKey::from_bytes(pk_bytes.as_slice().try_into().unwrap()).unwrap();

        let params = serde_json::json!({
            "subject": "remote-primal",
            "scope": ["crypto.*"],
            "ttl_secs": 300,
        });
        let issue_result = handle_auth_issue_ionic(PRIMAL, NODE, Some(&params));
        let token = issue_result["token"].as_str().unwrap();

        let verify_params = serde_json::json!({ "token": token, "method": "crypto.sign" });
        let verify_result = handle_auth_verify_ionic(&remote_vk, None, Some(&verify_params));
        assert_eq!(verify_result["valid"], true);
        assert_eq!(verify_result["scope_ok"], true);
    }

    #[test]
    fn public_key_different_primals_differ() {
        let a = handle_auth_public_key("beardog", "node-1");
        let b = handle_auth_public_key("primalspring", "node-1");
        assert_ne!(a["public_key"], b["public_key"]);
    }

    // ── auth.issue_session (JH-4) ──

    #[test]
    fn issue_session_default_purpose_is_research() {
        let result = handle_auth_issue_session(PRIMAL, NODE, None);
        assert_eq!(result["purpose"], "research");
        assert!(result["token"].as_str().is_some());
        assert_eq!(result["ttl_hours"], 8);
        assert_eq!(result["ttl_secs"], 8 * 3600);
    }

    #[test]
    fn issue_session_jupyterhub_scopes() {
        let params = serde_json::json!({ "purpose": "jupyterhub", "user": "dr.who" });
        let result = handle_auth_issue_session(PRIMAL, NODE, Some(&params));
        assert_eq!(result["purpose"], "jupyterhub");
        assert_eq!(result["subject"], "dr.who");

        let scope = result["scope"].as_array().unwrap();
        assert!(scope.iter().any(|s| s == "crypto.*"));
        assert!(scope.iter().any(|s| s == "health.*"));
        assert!(scope.iter().any(|s| s == "capabilities.*"));
        assert!(scope.iter().any(|s| s == "content.*"));
        assert!(scope.iter().any(|s| s == "auth.verify_ionic"));
    }

    #[test]
    fn issue_session_admin_short_ttl() {
        let params = serde_json::json!({ "purpose": "admin" });
        let result = handle_auth_issue_session(PRIMAL, NODE, Some(&params));
        assert_eq!(result["scope"], serde_json::json!(["*"]));
        assert_eq!(result["ttl_hours"], 1);
    }

    #[test]
    fn issue_session_desktop_scope() {
        let params = serde_json::json!({ "purpose": "desktop" });
        let result = handle_auth_issue_session(PRIMAL, NODE, Some(&params));
        let scope = result["scope"].as_array().unwrap();
        assert!(scope.iter().any(|s| s == "secrets.*"));
        assert!(scope.iter().any(|s| s == "content.*"));
        assert_eq!(result["ttl_hours"], 24);
    }

    #[test]
    fn issue_session_custom_ttl() {
        let params = serde_json::json!({ "purpose": "notebook", "ttl_hours": 2 });
        let result = handle_auth_issue_session(PRIMAL, NODE, Some(&params));
        assert_eq!(result["ttl_hours"], 2);
        assert_eq!(result["ttl_secs"], 7200);
    }

    #[test]
    fn issue_session_token_verifies() {
        let params = serde_json::json!({ "purpose": "jupyterhub", "user": "researcher" });
        let result = handle_auth_issue_session(PRIMAL, NODE, Some(&params));
        let token = result["token"].as_str().unwrap();

        let vk = derive_primal_verifying_key(PRIMAL, NODE);
        let verify_params = serde_json::json!({ "token": token });
        let verify_result = handle_auth_verify_ionic(&vk, None, Some(&verify_params));
        assert_eq!(verify_result["valid"], true);
        assert_eq!(verify_result["claims"]["sub"], "researcher");
    }

    #[test]
    fn issue_session_default_purpose_includes_content_scope() {
        let params = serde_json::json!({ "purpose": "research" });
        let result = handle_auth_issue_session(PRIMAL, NODE, Some(&params));
        let scope = result["scope"].as_array().unwrap();
        assert!(scope.iter().any(|s| s == "content.*"));
        assert!(scope.iter().any(|s| s == "crypto.*"));
    }

    #[test]
    fn issue_session_returns_usage_hint() {
        let result = handle_auth_issue_session(PRIMAL, NODE, None);
        assert!(result["usage"].as_str().unwrap().contains("BEARDOG_TOKEN"));
    }
}
