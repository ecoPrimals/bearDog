// SPDX-License-Identifier: AGPL-3.0-or-later

//! Tests for [`super::super::method_gate::MethodGate::check`] enforcement.

use super::super::*;
use super::method_gate_test_helpers::*;

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
    assert!(gate.check("auth.public_key", &mut caller).is_ok());
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
        btsp_family_verified: false,
        peer_id: None,
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
        btsp_family_verified: false,
        peer_id: None,
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
        btsp_family_verified: false,
        peer_id: None,
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
        btsp_family_verified: false,
        peer_id: None,
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
        btsp_family_verified: false,
        peer_id: None,
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
        btsp_family_verified: false,
        peer_id: None,
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

// ── Co-resident trust ──

#[test]
fn co_resident_same_uid_bypasses_enforced_gate() {
    let gate = test_gate(EnforcementMode::Enforced);
    let server_uid = gate.server_uid.unwrap_or(1000);
    let mut caller = CallerContext {
        bearer_token: None,
        peer: Some(PeerCredentials {
            uid: server_uid,
            pid: Some(12345),
        }),
        origin: ConnectionOrigin::Unix,
        validated_claims: None,
        btsp_family_verified: false,
        peer_id: None,
    };
    assert!(
        gate.check("crypto.x25519_generate_ephemeral", &mut caller)
            .is_ok(),
        "same-UID UDS caller should bypass enforced gate"
    );
}

#[test]
fn different_uid_rejected_by_enforced_gate() {
    let gate = test_gate(EnforcementMode::Enforced);
    let different_uid = gate.server_uid.map_or(9999, |u| u + 1);
    let mut caller = CallerContext {
        bearer_token: None,
        peer: Some(PeerCredentials {
            uid: different_uid,
            pid: Some(12345),
        }),
        origin: ConnectionOrigin::Unix,
        validated_claims: None,
        btsp_family_verified: false,
        peer_id: None,
    };
    assert!(
        gate.check("crypto.x25519_generate_ephemeral", &mut caller)
            .is_err(),
        "different-UID UDS caller should still be rejected"
    );
}

#[test]
fn tcp_caller_not_bypassed_even_with_same_uid() {
    let gate = test_gate(EnforcementMode::Enforced);
    let server_uid = gate.server_uid.unwrap_or(1000);
    let mut caller = CallerContext {
        bearer_token: None,
        peer: Some(PeerCredentials {
            uid: server_uid,
            pid: Some(12345),
        }),
        origin: ConnectionOrigin::Remote,
        validated_claims: None,
        btsp_family_verified: false,
        peer_id: None,
    };
    assert!(
        gate.check("crypto.sign_ed25519", &mut caller).is_err(),
        "TCP caller should not get co-resident trust even with matching UID"
    );
}

// ── try_verify_bearer ──

#[test]
fn try_verify_bearer_populates_claims_for_valid_token() {
    let gate = test_gate(EnforcementMode::Enforced);
    let token = issue_test_token(&["auth.*"], 300);

    let mut caller = CallerContext::remote();
    caller.bearer_token = Some(token);
    assert!(caller.validated_claims.is_none());

    gate.try_verify_bearer("auth.trust_issuer", &mut caller);
    assert!(
        caller.validated_claims.is_some(),
        "valid token with matching scope should populate claims"
    );
}

#[test]
fn try_verify_bearer_ignores_insufficient_scope() {
    let gate = test_gate(EnforcementMode::Enforced);
    let token = issue_test_token(&["crypto.*"], 300);

    let mut caller = CallerContext::remote();
    caller.bearer_token = Some(token);

    gate.try_verify_bearer("auth.trust_issuer", &mut caller);
    assert!(
        caller.validated_claims.is_none(),
        "token with wrong scope should not populate claims for auth.trust_issuer"
    );
}

#[test]
fn try_verify_bearer_skips_when_no_token() {
    let gate = test_gate(EnforcementMode::Enforced);
    let mut caller = CallerContext::remote();

    gate.try_verify_bearer("auth.trust_issuer", &mut caller);
    assert!(caller.validated_claims.is_none());
}

#[test]
fn try_verify_bearer_skips_when_claims_already_set() {
    let gate = test_gate(EnforcementMode::Enforced);
    let token = issue_test_token(&["auth.*"], 300);

    let mut caller = CallerContext::remote();
    caller.bearer_token = Some(token);

    gate.try_verify_bearer("auth.trust_issuer", &mut caller);
    assert!(caller.validated_claims.is_some());

    let first_sub = caller.validated_claims.as_ref().map(|c| c.sub.clone());

    gate.try_verify_bearer("auth.trust_issuer", &mut caller);
    assert_eq!(
        caller.validated_claims.as_ref().map(|c| c.sub.clone()),
        first_sub,
        "should not re-verify when claims already set"
    );
}
