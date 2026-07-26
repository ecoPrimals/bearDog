// SPDX-License-Identifier: AGPL-3.0-or-later

//! Tests for auth method handlers and trust exchange.

use super::super::*;
use super::method_gate_test_helpers::*;

#[test]
fn trust_issuer_emits_event() {
    use crate::auth_event_bus::AuthEventKind;

    let gate = test_gate(EnforcementMode::Permissive);
    let sk = crate::unix_socket_ipc::handlers::primal_signing::derive_primal_signing_key(
        "remote-gate",
        "remote-node",
    );
    let vk = sk.verifying_key();
    let did = crate::trusted_issuer_registry::did_from_verifying_key(&vk);
    let pk_b64 = base64::Engine::encode(&base64::engine::general_purpose::STANDARD, vk.as_bytes());

    let params = serde_json::json!({
        "public_key": pk_b64,
        "did": did,
        "gate_id": "remote-gate",
        "trust_method": "family_seed",
    });

    let caller = CallerContext::from_unix();
    let result = crate::trust_handlers::handle_auth_trust_issuer(
        gate.trusted_issuers(),
        gate.auth_events(),
        gate.primal_name(),
        &caller,
        Some(&params),
    );
    assert_eq!(result["registered"], true);

    let events = gate.auth_events().poll_since(0);
    assert_eq!(events.len(), 1);
    match &events[0].kind {
        AuthEventKind::TrustIssuerRegistered {
            issuer_did,
            trust_method,
            ..
        } => {
            assert_eq!(issuer_did, &did);
            assert_eq!(trust_method, "family_seed");
        }
        other => panic!("expected TrustIssuerRegistered, got {other:?}"),
    }
}

#[test]
fn auth_events_poll_returns_empty_initially() {
    let gate = test_gate(EnforcementMode::Permissive);
    let result = crate::trust_handlers::handle_auth_events_poll(gate.auth_events(), None);
    assert_eq!(result["count"], 0);
    assert!(result["events"].as_array().expect("array").is_empty());
}

#[test]
fn exchange_trust_registers_and_returns_local_key() {
    let gate = test_gate(EnforcementMode::Permissive);

    let remote_sk = crate::unix_socket_ipc::handlers::primal_signing::derive_primal_signing_key(
        "remote-gate",
        "remote-node",
    );
    let remote_vk = remote_sk.verifying_key();
    let pk_b64 = base64::Engine::encode(
        &base64::engine::general_purpose::STANDARD,
        remote_vk.as_bytes(),
    );

    let params = serde_json::json!({
        "public_key": pk_b64,
        "gate_id": "remote-gate",
        "family_id": "test-family",
    });

    let mut caller = CallerContext::from_unix();
    caller.btsp_family_verified = true;

    let result = crate::trust_handlers::handle_auth_exchange_trust(
        gate.trusted_issuers(),
        gate.auth_events(),
        gate.primal_name(),
        gate.node_id(),
        &caller,
        Some(&params),
    );

    assert_eq!(result["registered"], true);
    assert!(result["local_public_key"].is_string());
    assert!(
        result["local_did"]
            .as_str()
            .expect("did")
            .starts_with("did:key:z6Mk")
    );
    assert_eq!(result["trust_method"], "family_seed");
    assert_eq!(result["local_gate_id"], NODE);

    let events = gate.auth_events().poll_since(0);
    assert_eq!(events.len(), 2);
}

#[test]
fn exchange_trust_rejects_unauthenticated() {
    let gate = test_gate(EnforcementMode::Permissive);
    let caller = CallerContext::from_unix();

    let params = serde_json::json!({
        "public_key": "AAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAA=",
    });

    let result = crate::trust_handlers::handle_auth_exchange_trust(
        gate.trusted_issuers(),
        gate.auth_events(),
        gate.primal_name(),
        gate.node_id(),
        &caller,
        Some(&params),
    );

    assert_eq!(result["registered"], false);
    assert!(result["error"].as_str().expect("err").contains("BTSP"));
}

#[test]
fn exchange_trust_derives_did_from_key() {
    let gate = test_gate(EnforcementMode::Permissive);

    let remote_sk = crate::unix_socket_ipc::handlers::primal_signing::derive_primal_signing_key(
        "auto-gate",
        "auto-node",
    );
    let remote_vk = remote_sk.verifying_key();
    let pk_b64 = base64::Engine::encode(
        &base64::engine::general_purpose::STANDARD,
        remote_vk.as_bytes(),
    );
    let expected_did = crate::trusted_issuer_registry::did_from_verifying_key(&remote_vk);

    let params = serde_json::json!({ "public_key": pk_b64 });

    let mut caller = CallerContext::from_unix();
    caller.btsp_family_verified = true;

    let result = crate::trust_handlers::handle_auth_exchange_trust(
        gate.trusted_issuers(),
        gate.auth_events(),
        gate.primal_name(),
        gate.node_id(),
        &caller,
        Some(&params),
    );

    assert_eq!(result["registered"], true);
    assert_eq!(result["remote_did"], expected_did);
}

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
        btsp_family_verified: false,
        peer_id: None,
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
        btsp_family_verified: false,
        peer_id: None,
    };
    let result = handle_auth_peer_info(&caller);
    assert_eq!(result["available"], true);
    assert_eq!(result["uid"], 1000);
    assert_eq!(result["pid"], 1234);
}

#[test]
fn trust_issuer_rejects_unauthenticated_remote() {
    let gate = test_gate(EnforcementMode::Permissive);

    let sk = crate::unix_socket_ipc::handlers::primal_signing::derive_primal_signing_key(
        "remote-gate",
        "remote-node",
    );
    let vk = sk.verifying_key();
    let did = crate::trusted_issuer_registry::did_from_verifying_key(&vk);
    let pk_b64 = base64::Engine::encode(&base64::engine::general_purpose::STANDARD, vk.as_bytes());

    let params = serde_json::json!({
        "public_key": pk_b64,
        "did": did,
        "gate_id": "remote-gate",
    });

    let caller = CallerContext::remote();
    let result = crate::trust_handlers::handle_auth_trust_issuer(
        gate.trusted_issuers(),
        gate.auth_events(),
        gate.primal_name(),
        &caller,
        Some(&params),
    );

    assert_eq!(result["registered"], false);
    assert!(
        result["error"]
            .as_str()
            .expect("error")
            .contains("requires"),
        "should indicate auth requirement"
    );
    assert!(gate.trusted_issuers().is_empty());
}

#[test]
fn trust_issuer_accepts_btsp_verified() {
    let gate = test_gate(EnforcementMode::Permissive);

    let sk = crate::unix_socket_ipc::handlers::primal_signing::derive_primal_signing_key(
        "remote-gate",
        "remote-node",
    );
    let vk = sk.verifying_key();
    let did = crate::trusted_issuer_registry::did_from_verifying_key(&vk);
    let pk_b64 = base64::Engine::encode(&base64::engine::general_purpose::STANDARD, vk.as_bytes());

    let params = serde_json::json!({
        "public_key": pk_b64,
        "did": did,
        "gate_id": "remote-gate",
    });

    let mut caller = CallerContext::remote();
    caller.btsp_family_verified = true;

    let result = crate::trust_handlers::handle_auth_trust_issuer(
        gate.trusted_issuers(),
        gate.auth_events(),
        gate.primal_name(),
        &caller,
        Some(&params),
    );

    assert_eq!(result["registered"], true);
    assert_eq!(gate.trusted_issuers().len(), 1);
}
