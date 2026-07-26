// SPDX-License-Identifier: AGPL-3.0-or-later

//! Tests for [`super::super::method_gate::classify_method`] and related types.

use super::super::*;

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
fn capability_call_is_protected() {
    assert_eq!(
        classify_method("capability.call"),
        MethodAccessLevel::Protected
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
    assert_eq!(
        classify_method("auth.public_key"),
        MethodAccessLevel::Public
    );
    assert_eq!(
        classify_method("auth.trusted_issuers"),
        MethodAccessLevel::Public
    );
}

#[test]
fn trust_issuer_is_protected() {
    assert_eq!(
        classify_method("auth.trust_issuer"),
        MethodAccessLevel::Protected
    );
}

#[test]
fn phase35_verify_ed25519_is_protected() {
    assert_eq!(
        classify_method("crypto.verify.ed25519"),
        MethodAccessLevel::Protected
    );
    assert_eq!(
        classify_method("crypto.sign.ed25519"),
        MethodAccessLevel::Protected
    );
}

#[test]
fn auth_events_poll_is_protected() {
    assert_eq!(
        classify_method("auth.events.poll"),
        MethodAccessLevel::Protected
    );
}

#[test]
fn auth_events_poll_is_gate_handled() {
    assert!(is_gate_handled_method("auth.events.poll"));
}

#[test]
fn exchange_trust_is_gate_handled() {
    assert!(is_gate_handled_method("auth.exchange_trust"));
}

#[test]
fn exchange_trust_is_protected() {
    assert_eq!(
        classify_method("auth.exchange_trust"),
        MethodAccessLevel::Protected
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
