// SPDX-License-Identifier: AGPL-3.0-or-later

//! Tests for [`crate::method_gate`] — classification, gate enforcement,
//! auth handlers, and dispatch routing.

use super::*;

const PRIMAL: &str = "beardog";
const NODE: &str = "test-node";

fn test_gate(mode: EnforcementMode) -> MethodGate {
    MethodGate::new(mode, PRIMAL, NODE)
}

fn issue_test_token(scopes: &[&str], ttl: i64) -> String {
    use crate::ionic_token::issue_ionic_token;
    use crate::unix_socket_ipc::handlers::primal_signing::derive_primal_signing_key;
    let sk = derive_primal_signing_key(PRIMAL, NODE);
    let scope_strings: Vec<String> = scopes.iter().map(|s| (*s).to_owned()).collect();
    issue_ionic_token(&sk, "did:key:z6MkTest", "testuser", &scope_strings, ttl)
}

// ── classify_method ──

#[test]
fn health_methods_are_public() {
    assert_eq!(classify_method("health.check"), MethodAccessLevel::Public);
    assert_eq!(
        classify_method("health.liveness"),
        MethodAccessLevel::Public
    );
    assert_eq!(
        classify_method("health.readiness"),
        MethodAccessLevel::Public
    );
}

#[test]
fn identity_get_is_public() {
    assert_eq!(classify_method("identity.get"), MethodAccessLevel::Public);
}

#[test]
fn identity_create_is_public() {
    assert_eq!(
        classify_method("identity.create"),
        MethodAccessLevel::Public
    );
}

#[test]
fn capabilities_list_is_public() {
    assert_eq!(
        classify_method("capabilities.list"),
        MethodAccessLevel::Public
    );
    assert_eq!(
        classify_method("capability.list"),
        MethodAccessLevel::Public
    );
}

#[test]
fn auth_introspection_is_public() {
    assert_eq!(classify_method("auth.check"), MethodAccessLevel::Public);
    assert_eq!(classify_method("auth.mode"), MethodAccessLevel::Public);
    assert_eq!(classify_method("auth.peer_info"), MethodAccessLevel::Public);
}

#[test]
fn ionic_auth_methods_are_public() {
    assert_eq!(
        classify_method("auth.issue_ionic"),
        MethodAccessLevel::Public
    );
    assert_eq!(
        classify_method("auth.issue_session"),
        MethodAccessLevel::Public
    );
    assert_eq!(
        classify_method("auth.verify_ionic"),
        MethodAccessLevel::Public
    );
}

#[test]
fn lifecycle_status_is_public() {
    assert_eq!(
        classify_method("lifecycle.status"),
        MethodAccessLevel::Public
    );
}

#[test]
fn crypto_methods_are_protected() {
    assert_eq!(
        classify_method("crypto.sign_ed25519"),
        MethodAccessLevel::Protected
    );
    assert_eq!(
        classify_method("crypto.sign_contract"),
        MethodAccessLevel::Protected
    );
}

#[test]
fn ionic_bond_methods_are_protected() {
    assert_eq!(
        classify_method("crypto.ionic_bond.propose"),
        MethodAccessLevel::Protected
    );
}

#[test]
fn empty_method_is_protected() {
    assert_eq!(classify_method(""), MethodAccessLevel::Protected);
}

#[test]
fn legacy_identity_aliases_are_protected() {
    assert_eq!(classify_method("identity"), MethodAccessLevel::Protected);
    assert_eq!(classify_method("whoami"), MethodAccessLevel::Protected);
}

// ── CallerContext ──

#[test]
fn unix_context_has_no_peer() {
    let ctx = CallerContext::from_unix();
    assert!(ctx.peer.is_none());
    assert!(ctx.bearer_token.is_none());
    assert!(ctx.validated_claims.is_none());
    assert_eq!(ctx.origin, ConnectionOrigin::Unix);
}

#[test]
fn loopback_context() {
    let ctx = CallerContext::loopback();
    assert_eq!(ctx.origin, ConnectionOrigin::Loopback);
}

#[test]
fn remote_context() {
    let ctx = CallerContext::remote();
    assert_eq!(ctx.origin, ConnectionOrigin::Remote);
}

// ── EnforcementMode ──

#[test]
fn enforcement_mode_as_str() {
    assert_eq!(EnforcementMode::Permissive.as_str(), "permissive");
    assert_eq!(EnforcementMode::Enforced.as_str(), "enforced");
}

// ── MethodGate::check ──

#[test]
fn public_method_always_passes() {
    let gate = test_gate(EnforcementMode::Enforced);
    let mut caller = CallerContext::loopback();
    assert!(gate.check("health.check", &mut caller).is_ok());
    assert!(gate.check("identity.get", &mut caller).is_ok());
    assert!(gate.check("identity.create", &mut caller).is_ok());
    assert!(gate.check("capabilities.list", &mut caller).is_ok());
    assert!(gate.check("auth.check", &mut caller).is_ok());
    assert!(gate.check("auth.issue_ionic", &mut caller).is_ok());
    assert!(gate.check("auth.issue_session", &mut caller).is_ok());
    assert!(gate.check("auth.verify_ionic", &mut caller).is_ok());
}

#[test]
fn protected_method_passes_in_permissive_mode() {
    let gate = test_gate(EnforcementMode::Permissive);
    let mut caller = CallerContext::loopback();
    assert!(gate.check("crypto.sign_ed25519", &mut caller).is_ok());
}

#[test]
fn protected_method_rejected_in_enforced_mode_without_token() {
    let gate = test_gate(EnforcementMode::Enforced);
    let mut caller = CallerContext::loopback();
    let result = gate.check("crypto.sign_ed25519", &mut caller);
    assert!(result.is_err());
    let err = result.unwrap_err();
    assert_eq!(err.code, JsonRpcError::PERMISSION_DENIED);
    assert!(err.message.contains("crypto.sign_ed25519"));
}

#[test]
fn protected_method_passes_with_valid_ionic_token() {
    let gate = test_gate(EnforcementMode::Enforced);
    let token = issue_test_token(&["crypto.*"], 3600);
    let mut caller = CallerContext {
        bearer_token: Some(token),
        peer: None,
        origin: ConnectionOrigin::Unix,
        validated_claims: None,
    };
    assert!(gate.check("crypto.sign_ed25519", &mut caller).is_ok());
    assert!(caller.validated_claims.is_some());
    assert_eq!(caller.validated_claims.as_ref().unwrap().sub, "testuser");
}

#[test]
fn insufficient_scope_rejected_in_enforced_mode() {
    let gate = test_gate(EnforcementMode::Enforced);
    let token = issue_test_token(&["health.*"], 3600);
    let mut caller = CallerContext {
        bearer_token: Some(token),
        peer: None,
        origin: ConnectionOrigin::Unix,
        validated_claims: None,
    };
    let result = gate.check("crypto.sign_ed25519", &mut caller);
    assert!(result.is_err());
    assert_eq!(result.unwrap_err().code, JsonRpcError::PERMISSION_DENIED);
}

#[test]
fn insufficient_scope_allowed_in_permissive_mode() {
    let gate = test_gate(EnforcementMode::Permissive);
    let token = issue_test_token(&["health.*"], 3600);
    let mut caller = CallerContext {
        bearer_token: Some(token),
        peer: None,
        origin: ConnectionOrigin::Unix,
        validated_claims: None,
    };
    assert!(gate.check("crypto.sign_ed25519", &mut caller).is_ok());
    assert!(caller.validated_claims.is_some());
}

#[test]
fn expired_token_rejected_in_enforced_mode() {
    let gate = test_gate(EnforcementMode::Enforced);
    let token = issue_test_token(&["*"], -10);
    let mut caller = CallerContext {
        bearer_token: Some(token),
        peer: None,
        origin: ConnectionOrigin::Unix,
        validated_claims: None,
    };
    let result = gate.check("crypto.sign_ed25519", &mut caller);
    assert!(result.is_err());
    assert_eq!(result.unwrap_err().code, JsonRpcError::UNAUTHORIZED);
}

#[test]
fn expired_token_allowed_in_permissive_mode() {
    let gate = test_gate(EnforcementMode::Permissive);
    let token = issue_test_token(&["*"], -10);
    let mut caller = CallerContext {
        bearer_token: Some(token),
        peer: None,
        origin: ConnectionOrigin::Unix,
        validated_claims: None,
    };
    assert!(gate.check("crypto.sign_ed25519", &mut caller).is_ok());
}

#[test]
fn bogus_token_rejected_in_enforced_mode() {
    let gate = test_gate(EnforcementMode::Enforced);
    let mut caller = CallerContext {
        bearer_token: Some("not-a-real-token".to_owned()),
        peer: None,
        origin: ConnectionOrigin::Unix,
        validated_claims: None,
    };
    let result = gate.check("crypto.sign_ed25519", &mut caller);
    assert!(result.is_err());
    assert_eq!(result.unwrap_err().code, JsonRpcError::UNAUTHORIZED);
}

#[test]
fn gate_error_includes_method_in_data() {
    let gate = test_gate(EnforcementMode::Enforced);
    let mut caller = CallerContext::loopback();
    let err = gate.check("crypto.blake3_hash", &mut caller).unwrap_err();
    let method_in_data = err
        .data
        .as_ref()
        .and_then(|d| d.get("method"))
        .and_then(serde_json::Value::as_str);
    assert_eq!(method_in_data, Some("crypto.blake3_hash"));
}

// ── auth method handlers ──

#[test]
fn auth_check_unauthenticated() {
    let caller = CallerContext::loopback();
    let result = handle_auth_check(&caller);
    assert_eq!(result["authenticated"], false);
    assert_eq!(result["origin"], "loopback");
    assert!(result.get("claims").is_none());
}

#[test]
fn auth_check_authenticated_with_claims() {
    let gate = test_gate(EnforcementMode::Permissive);
    let token = issue_test_token(&["*"], 3600);
    let mut caller = CallerContext {
        bearer_token: Some(token),
        peer: None,
        origin: ConnectionOrigin::Unix,
        validated_claims: None,
    };
    gate.check("crypto.sign", &mut caller).unwrap();
    let result = handle_auth_check(&caller);
    assert_eq!(result["authenticated"], true);
    assert_eq!(result["claims"]["sub"], "testuser");
}

#[test]
fn auth_mode_response() {
    let gate = test_gate(EnforcementMode::Permissive);
    let result = handle_auth_mode(&gate);
    assert_eq!(result["mode"], "permissive");
    assert_eq!(result["env_var"], "BEARDOG_AUTH_MODE");
}

#[test]
fn auth_peer_info_no_creds() {
    let caller = CallerContext::loopback();
    let result = handle_auth_peer_info(&caller);
    assert_eq!(result["available"], false);
}

#[test]
fn auth_peer_info_with_creds() {
    let caller = CallerContext {
        bearer_token: None,
        peer: Some(PeerCredentials {
            pid: Some(1234),
            uid: 1000,
        }),
        origin: ConnectionOrigin::Unix,
        validated_claims: None,
    };
    let result = handle_auth_peer_info(&caller);
    assert_eq!(result["available"], true);
    assert_eq!(result["uid"], 1000);
    assert_eq!(result["pid"], 1234);
}

// ── dispatch_auth_method ──

#[test]
fn dispatch_routes_auth_methods() {
    let gate = test_gate(EnforcementMode::Permissive);
    let caller = CallerContext::loopback();
    assert!(dispatch_auth_method("auth.check", &gate, &caller, None).is_some());
    assert!(dispatch_auth_method("auth.mode", &gate, &caller, None).is_some());
    assert!(dispatch_auth_method("auth.peer_info", &gate, &caller, None).is_some());
}

#[test]
fn dispatch_routes_ionic_methods() {
    let gate = test_gate(EnforcementMode::Permissive);
    let caller = CallerContext::loopback();
    assert!(dispatch_auth_method("identity.create", &gate, &caller, None).is_some());

    let issue_params = serde_json::json!({"subject": "alice"});
    assert!(
        dispatch_auth_method("auth.issue_ionic", &gate, &caller, Some(&issue_params)).is_some()
    );

    let token_result =
        dispatch_auth_method("auth.issue_ionic", &gate, &caller, Some(&issue_params)).unwrap();
    let verify_params = serde_json::json!({"token": token_result["token"]});
    assert!(
        dispatch_auth_method("auth.verify_ionic", &gate, &caller, Some(&verify_params)).is_some()
    );
}

#[test]
fn dispatch_routes_issue_session() {
    let gate = test_gate(EnforcementMode::Permissive);
    let caller = CallerContext::loopback();
    let session_params = serde_json::json!({"purpose": "jupyterhub", "user": "researcher"});
    let result = dispatch_auth_method("auth.issue_session", &gate, &caller, Some(&session_params));
    assert!(result.is_some());
    let val = result.unwrap();
    assert_eq!(val["purpose"], "jupyterhub");
    assert_eq!(val["subject"], "researcher");
    assert!(val["token"].as_str().is_some());
}

#[test]
fn dispatch_returns_none_for_non_auth() {
    let gate = test_gate(EnforcementMode::Permissive);
    let caller = CallerContext::loopback();
    assert!(dispatch_auth_method("crypto.sign_ed25519", &gate, &caller, None).is_none());
}

#[test]
fn is_gate_handled_method_correct() {
    assert!(is_gate_handled_method("auth.check"));
    assert!(is_gate_handled_method("auth.mode"));
    assert!(is_gate_handled_method("auth.peer_info"));
    assert!(is_gate_handled_method("auth.issue_ionic"));
    assert!(is_gate_handled_method("auth.issue_session"));
    assert!(is_gate_handled_method("auth.verify_ionic"));
    assert!(is_gate_handled_method("identity.create"));
    assert!(!is_gate_handled_method("crypto.sign"));
}
