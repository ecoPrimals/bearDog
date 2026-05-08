// SPDX-License-Identifier: AGPL-3.0-or-later

//! JSON-RPC handlers for ionic token lifecycle methods (JH-1).
//!
//! These are dispatched at the gate layer (not via `HandlerRegistry`) because
//! they need access to the primal's signing key and `CallerContext`.
//!
//! - `identity.create` — generate an ephemeral Ed25519 caller keypair + DID
//! - `auth.issue_ionic` — issue a signed ionic capability token
//! - `auth.verify_ionic` — verify a token string, return claims or error

use crate::ionic_token::{TokenError, issue_ionic_token, scope_covers_method, verify_ionic_token};
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
#[must_use]
pub fn handle_auth_issue_ionic(primal_name: &str, node_id: &str, params: Option<&Value>) -> Value {
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

    let token = issue_ionic_token(&signing_key, &issuer_did, subject, &scope, ttl_secs);

    serde_json::json!({
        "token": token,
        "issuer": issuer_did,
        "subject": subject,
        "scope": scope,
        "ttl_secs": ttl_secs,
    })
}

// ── auth.verify_ionic ───────────────────────────────────────────────────

/// Handle `auth.verify_ionic` — verify a token string and return claims.
///
/// # Params (from JSON-RPC `params`)
///
/// - `token` (string, required): the compact ionic token string
/// - `method` (string, optional): if provided, also checks scope coverage
#[must_use]
pub fn handle_auth_verify_ionic(verifying_key: &VerifyingKey, params: Option<&Value>) -> Value {
    let Some(token_str) = params.and_then(|p| p.get("token")).and_then(Value::as_str) else {
        return serde_json::json!({
            "valid": false,
            "error": "missing required parameter: token",
        });
    };

    let method = params.and_then(|p| p.get("method")).and_then(Value::as_str);

    match verify_ionic_token(token_str, verifying_key) {
        Ok(payload) => {
            let scope_ok = match method {
                Some(m) => scope_covers_method(&payload.scope, m),
                None => true,
            };

            serde_json::json!({
                "valid": true,
                "scope_ok": scope_ok,
                "claims": {
                    "iss": payload.iss,
                    "sub": payload.sub,
                    "scope": payload.scope,
                    "iat": payload.iat,
                    "exp": payload.exp,
                    "jti": payload.jti,
                },
            })
        }
        Err(e) => {
            let reason = match &e {
                TokenError::Malformed(_) => "malformed",
                TokenError::InvalidSignature => "invalid_signature",
                TokenError::Expired { .. } => "expired",
                TokenError::UnsupportedFormat(_) => "unsupported_format",
            };
            serde_json::json!({
                "valid": false,
                "error": e.to_string(),
                "reason": reason,
            })
        }
    }
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
        let verify_result = handle_auth_verify_ionic(&vk, Some(&verify_params));

        assert_eq!(verify_result["valid"], true);
        assert_eq!(verify_result["claims"]["sub"], "alice");
        assert_eq!(
            verify_result["claims"]["scope"],
            serde_json::json!(["crypto.*", "health.*"])
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
        let ok_result = handle_auth_verify_ionic(&vk, Some(&ok_params));
        assert_eq!(ok_result["valid"], true);
        assert_eq!(ok_result["scope_ok"], true);

        let bad_params = serde_json::json!({ "token": token, "method": "lifecycle.shutdown" });
        let bad_result = handle_auth_verify_ionic(&vk, Some(&bad_params));
        assert_eq!(bad_result["valid"], true);
        assert_eq!(bad_result["scope_ok"], false);
    }

    #[test]
    fn verify_missing_token_returns_error() {
        let vk = derive_primal_verifying_key(PRIMAL, NODE);
        let result = handle_auth_verify_ionic(&vk, Some(&serde_json::json!({})));
        assert_eq!(result["valid"], false);
    }

    #[test]
    fn verify_bad_token_returns_reason() {
        let vk = derive_primal_verifying_key(PRIMAL, NODE);
        let params = serde_json::json!({ "token": "garbage" });
        let result = handle_auth_verify_ionic(&vk, Some(&params));
        assert_eq!(result["valid"], false);
        assert_eq!(result["reason"], "malformed");
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
}
