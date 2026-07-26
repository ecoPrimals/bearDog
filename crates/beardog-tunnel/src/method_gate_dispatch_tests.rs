// SPDX-License-Identifier: AGPL-3.0-or-later

//! Tests for [`super::super::method_gate::dispatch_auth_method`] routing.

use super::super::*;
use super::method_gate_test_helpers::*;

#[test]
fn dispatch_routes_auth_methods() {
    let gate = test_gate(EnforcementMode::Permissive);
    let mut caller = CallerContext::loopback();
    assert!(dispatch_auth_method("auth.check", &gate, &mut caller, None).is_some());
    assert!(dispatch_auth_method("auth.mode", &gate, &mut caller, None).is_some());
    assert!(dispatch_auth_method("auth.peer_info", &gate, &mut caller, None).is_some());
}

#[test]
fn dispatch_routes_ionic_methods() {
    let gate = test_gate(EnforcementMode::Permissive);
    let mut caller = CallerContext::loopback();
    assert!(dispatch_auth_method("identity.create", &gate, &mut caller, None).is_some());

    let issue_params = serde_json::json!({"subject": "alice"});
    assert!(
        dispatch_auth_method("auth.issue_ionic", &gate, &mut caller, Some(&issue_params)).is_some()
    );

    let token_result =
        dispatch_auth_method("auth.issue_ionic", &gate, &mut caller, Some(&issue_params)).unwrap();
    let verify_params = serde_json::json!({"token": token_result["token"]});
    assert!(
        dispatch_auth_method(
            "auth.verify_ionic",
            &gate,
            &mut caller,
            Some(&verify_params)
        )
        .is_some()
    );
}

#[test]
fn dispatch_routes_public_key() {
    let gate = test_gate(EnforcementMode::Permissive);
    let mut caller = CallerContext::loopback();
    let result = dispatch_auth_method("auth.public_key", &gate, &mut caller, None);
    assert!(result.is_some());
    let val = result.unwrap();
    assert_eq!(val["algorithm"], "Ed25519");
    assert!(val["did"].as_str().unwrap().starts_with("did:key:z6Mk"));
    assert!(val["public_key"].as_str().is_some());
    assert!(val["public_key_hex"].as_str().is_some());
}

#[test]
fn dispatch_routes_issue_session() {
    let gate = test_gate(EnforcementMode::Permissive);
    let mut caller = CallerContext::loopback();
    let session_params = serde_json::json!({"purpose": "jupyterhub", "user": "researcher"});
    let result = dispatch_auth_method(
        "auth.issue_session",
        &gate,
        &mut caller,
        Some(&session_params),
    );
    assert!(result.is_some());
    let val = result.unwrap();
    assert_eq!(val["purpose"], "jupyterhub");
    assert_eq!(val["subject"], "researcher");
    assert!(val["token"].as_str().is_some());
}

#[test]
fn dispatch_returns_none_for_non_auth() {
    let gate = test_gate(EnforcementMode::Permissive);
    let mut caller = CallerContext::loopback();
    assert!(dispatch_auth_method("crypto.sign_ed25519", &gate, &mut caller, None).is_none());
}

#[test]
fn is_gate_handled_method_correct() {
    assert!(is_gate_handled_method("auth.check"));
    assert!(is_gate_handled_method("auth.mode"));
    assert!(is_gate_handled_method("auth.peer_info"));
    assert!(is_gate_handled_method("auth.issue_ionic"));
    assert!(is_gate_handled_method("auth.issue_session"));
    assert!(is_gate_handled_method("auth.verify_ionic"));
    assert!(is_gate_handled_method("auth.public_key"));
    assert!(is_gate_handled_method("auth.trust_issuer"));
    assert!(is_gate_handled_method("auth.trusted_issuers"));
    assert!(is_gate_handled_method("identity.create"));
    assert!(!is_gate_handled_method("crypto.sign"));
}
