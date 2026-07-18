// SPDX-License-Identifier: AGPL-3.0-or-later

use super::*;
use crate::ionic_token::{TokenError, issue_ionic_token};
use crate::unix_socket_ipc::handlers::primal_signing::{
    derive_primal_signing_key, derive_primal_verifying_key,
};
use base64::Engine;
use beardog_config::env_keys::ENV_TRUSTED_ISSUERS;

const GATE_A_PRIMAL: &str = "beardog";
const GATE_A_NODE: &str = "southgate-node-1";
const GATE_B_PRIMAL: &str = "beardog";
const GATE_B_NODE: &str = "eastgate-node-1";

fn gate_did(primal: &str, node: &str) -> String {
    let vk = derive_primal_verifying_key(primal, node);
    did_from_verifying_key(&vk)
}

fn issue_token_with_did(primal: &str, node: &str, subject: &str) -> String {
    let sk = derive_primal_signing_key(primal, node);
    let did = gate_did(primal, node);
    issue_ionic_token(&sk, &did, subject, &["*".to_owned()], 3600)
}

#[test]
fn local_token_verifies_without_registry() {
    let local_vk = derive_primal_verifying_key(GATE_A_PRIMAL, GATE_A_NODE);
    let token = issue_token_with_did(GATE_A_PRIMAL, GATE_A_NODE, "alice");
    let registry = TrustedIssuerRegistry::new();

    let result = verify_with_registry(&token, &local_vk, &registry, None);
    assert!(matches!(result, CrossGateVerifyResult::LocalVerified(_)));
}

#[test]
fn remote_token_fails_without_registry() {
    let local_vk = derive_primal_verifying_key(GATE_A_PRIMAL, GATE_A_NODE);
    let token = issue_token_with_did(GATE_B_PRIMAL, GATE_B_NODE, "bob");
    let registry = TrustedIssuerRegistry::new();

    let result = verify_with_registry(&token, &local_vk, &registry, None);
    assert!(matches!(
        result,
        CrossGateVerifyResult::Failed(TokenError::InvalidSignature)
    ));
}

#[test]
fn remote_token_verifies_with_registered_issuer() {
    let local_vk = derive_primal_verifying_key(GATE_A_PRIMAL, GATE_A_NODE);
    let remote_vk = derive_primal_verifying_key(GATE_B_PRIMAL, GATE_B_NODE);
    let remote_did = gate_did(GATE_B_PRIMAL, GATE_B_NODE);
    let token = issue_token_with_did(GATE_B_PRIMAL, GATE_B_NODE, "bob");

    let registry = TrustedIssuerRegistry::new();
    registry
        .register(
            &remote_did,
            remote_vk,
            Some("eastgate-node-1".to_owned()),
            Some("family-alpha".to_owned()),
            TrustMethod::FamilySeed,
        )
        .expect("register should succeed");

    let result = verify_with_registry(&token, &local_vk, &registry, None);
    match result {
        CrossGateVerifyResult::RemoteVerified {
            payload,
            issuer_info,
        } => {
            assert_eq!(payload.sub, "bob");
            assert_eq!(issuer_info.gate_id.as_deref(), Some("eastgate-node-1"));
            assert_eq!(issuer_info.trust_method, TrustMethod::FamilySeed);
        }
        other => panic!("expected RemoteVerified, got {other:?}"),
    }
}

#[test]
fn adhoc_key_verifies_unregistered_issuer() {
    let local_vk = derive_primal_verifying_key(GATE_A_PRIMAL, GATE_A_NODE);
    let remote_vk = derive_primal_verifying_key(GATE_B_PRIMAL, GATE_B_NODE);
    let token = issue_token_with_did(GATE_B_PRIMAL, GATE_B_NODE, "charlie");
    let registry = TrustedIssuerRegistry::new();

    let result = verify_with_registry(&token, &local_vk, &registry, Some(&remote_vk));
    assert!(matches!(result, CrossGateVerifyResult::AdHocVerified(_)));
}

#[test]
fn register_validates_did_key_binding() {
    let registry = TrustedIssuerRegistry::new();
    let vk = derive_primal_verifying_key(GATE_B_PRIMAL, GATE_B_NODE);
    let correct_did = gate_did(GATE_B_PRIMAL, GATE_B_NODE);

    // Mismatched DID should fail
    let result = registry.register("did:key:wrong", vk, None, None, TrustMethod::Manual);
    assert!(result.is_err());

    // Correct DID should succeed
    let result = registry.register(&correct_did, vk, None, None, TrustMethod::Manual);
    assert_eq!(result.unwrap(), true);

    // Idempotent — second register returns false
    let result = registry.register(&correct_did, vk, None, None, TrustMethod::Manual);
    assert_eq!(result.unwrap(), false);
    assert_eq!(registry.len(), 1);
}

#[test]
fn list_returns_all_issuers() {
    let registry = TrustedIssuerRegistry::new();
    let vk_b = derive_primal_verifying_key(GATE_B_PRIMAL, GATE_B_NODE);
    let vk_c = derive_primal_verifying_key("beardog", "westgate-node-1");
    let did_b = gate_did(GATE_B_PRIMAL, GATE_B_NODE);
    let did_c = gate_did("beardog", "westgate-node-1");

    registry
        .register(
            &did_b,
            vk_b,
            Some("eastgate-node-1".to_owned()),
            None,
            TrustMethod::FamilySeed,
        )
        .unwrap();
    registry
        .register(
            &did_c,
            vk_c,
            Some("westgate-node-1".to_owned()),
            None,
            TrustMethod::ContractExchange,
        )
        .unwrap();

    let issuers = registry.list();
    assert_eq!(issuers.len(), 2);
}

#[test]
fn remove_issuer() {
    let registry = TrustedIssuerRegistry::new();
    let vk = derive_primal_verifying_key(GATE_B_PRIMAL, GATE_B_NODE);
    let did = gate_did(GATE_B_PRIMAL, GATE_B_NODE);
    registry
        .register(&did, vk, None, None, TrustMethod::Manual)
        .unwrap();
    assert_eq!(registry.len(), 1);
    assert!(registry.remove(&did));
    assert!(registry.is_empty());
    assert!(!registry.remove(&did));
}

#[test]
fn cross_gate_roundtrip_full_flow() {
    let gate_a_vk = derive_primal_verifying_key(GATE_A_PRIMAL, GATE_A_NODE);
    let gate_b_sk = derive_primal_signing_key(GATE_B_PRIMAL, GATE_B_NODE);
    let gate_b_vk = derive_primal_verifying_key(GATE_B_PRIMAL, GATE_B_NODE);
    let gate_b_did = gate_did(GATE_B_PRIMAL, GATE_B_NODE);

    let token = crate::ionic_token::issue_ionic_token_with_gate(
        &gate_b_sk,
        &gate_b_did,
        "cross-gate-user",
        &["crypto.*".to_owned()],
        3600,
        Some(&crate::ionic_token::GateIdentity {
            node_id: GATE_B_NODE.to_owned(),
            family_id: "family-alpha".to_owned(),
        }),
    );

    let registry = TrustedIssuerRegistry::new();
    registry
        .register(
            &gate_b_did,
            gate_b_vk,
            Some(GATE_B_NODE.to_owned()),
            Some("family-alpha".to_owned()),
            TrustMethod::FamilySeed,
        )
        .expect("register gate B");

    let result = verify_with_registry(&token, &gate_a_vk, &registry, None);
    match result {
        CrossGateVerifyResult::RemoteVerified {
            payload,
            issuer_info,
        } => {
            assert_eq!(payload.sub, "cross-gate-user");
            assert_eq!(payload.gate_id.as_deref(), Some(GATE_B_NODE));
            assert_eq!(payload.family_id.as_deref(), Some("family-alpha"));
            assert_eq!(issuer_info.trust_method, TrustMethod::FamilySeed);
            assert_eq!(payload.iss, gate_b_did);
        }
        other => panic!("expected RemoteVerified, got {other:?}"),
    }
}

#[test]
fn iss_mismatch_prevents_remote_verify() {
    let local_vk = derive_primal_verifying_key(GATE_A_PRIMAL, GATE_A_NODE);
    let remote_sk = derive_primal_signing_key(GATE_B_PRIMAL, GATE_B_NODE);
    let remote_vk = derive_primal_verifying_key(GATE_B_PRIMAL, GATE_B_NODE);
    let remote_did = gate_did(GATE_B_PRIMAL, GATE_B_NODE);

    // Issue token with a WRONG iss (key signs fine but iss won't match registry DID)
    let token = issue_ionic_token(
        &remote_sk,
        "did:key:forged-iss",
        "mallory",
        &["*".to_owned()],
        3600,
    );

    let registry = TrustedIssuerRegistry::new();
    registry
        .register(&remote_did, remote_vk, None, None, TrustMethod::FamilySeed)
        .unwrap();

    // Sig is valid for the key but iss doesn't match → should NOT verify as remote
    let result = verify_with_registry(&token, &local_vk, &registry, None);
    assert!(
        matches!(
            result,
            CrossGateVerifyResult::Failed(TokenError::InvalidSignature)
        ),
        "iss mismatch should prevent RemoteVerified"
    );
}

#[test]
fn did_from_key_roundtrip() {
    let vk = derive_primal_verifying_key("beardog", "test-node");
    let did = did_from_verifying_key(&vk);
    assert!(did.starts_with("did:key:z6Mk"));
    assert!(did_matches_key(&did, &vk));
    assert!(!did_matches_key("did:key:z6MkWrong", &vk));
}

#[test]
#[serial_test::serial]
fn seed_from_env_registers_issuers() {
    use base64::Engine;

    let vk_a = derive_primal_verifying_key("gate-a", "node-a");
    let vk_b = derive_primal_verifying_key("gate-b", "node-b");
    let b64_a = base64::engine::general_purpose::STANDARD.encode(vk_a.as_bytes());
    let b64_b = base64::engine::general_purpose::STANDARD.encode(vk_b.as_bytes());

    let env_val = format!("{b64_a}:gate-a:fam1,{b64_b}:gate-b:fam2");
    beardog_errors::process_env::set_var(ENV_TRUSTED_ISSUERS, &env_val);

    let registry = TrustedIssuerRegistry::new();
    let seeded = registry.seed_from_env().expect("seed should succeed");
    assert_eq!(seeded, 2);
    assert_eq!(registry.len(), 2);

    let did_a = did_from_verifying_key(&vk_a);
    let (stored_vk, info) = registry.get(&did_a).expect("gate-a should be registered");
    assert_eq!(stored_vk.as_bytes(), vk_a.as_bytes());
    assert_eq!(info.gate_id.as_deref(), Some("gate-a"));
    assert_eq!(info.family_id.as_deref(), Some("fam1"));

    beardog_errors::process_env::remove_var(ENV_TRUSTED_ISSUERS);
}

#[test]
#[serial_test::serial]
fn seed_from_env_skips_when_unset() {
    beardog_errors::process_env::remove_var(ENV_TRUSTED_ISSUERS);
    let registry = TrustedIssuerRegistry::new();
    let seeded = registry.seed_from_env().expect("should succeed with 0");
    assert_eq!(seeded, 0);
    assert!(registry.is_empty());
}

#[test]
#[serial_test::serial]
fn seed_from_env_rejects_invalid_base64() {
    beardog_errors::process_env::set_var(ENV_TRUSTED_ISSUERS, "not-valid-base64!!!");
    let registry = TrustedIssuerRegistry::new();
    let err = registry
        .seed_from_env()
        .expect_err("should reject bad base64");
    assert!(err.to_string().contains("base64"), "error: {err}");
    beardog_errors::process_env::remove_var(ENV_TRUSTED_ISSUERS);
}

#[test]
#[serial_test::serial]
fn seed_from_env_rejects_wrong_key_length() {
    let short_key = base64::engine::general_purpose::STANDARD.encode(b"tooshort");
    beardog_errors::process_env::set_var(ENV_TRUSTED_ISSUERS, &short_key);
    let registry = TrustedIssuerRegistry::new();
    let err = registry
        .seed_from_env()
        .expect_err("should reject short key");
    assert!(err.to_string().contains("32 bytes"), "error: {err}");
    beardog_errors::process_env::remove_var(ENV_TRUSTED_ISSUERS);
}

#[test]
#[serial_test::serial]
fn seed_from_env_handles_optional_fields() {
    use base64::Engine;

    let vk = derive_primal_verifying_key("solo", "node");
    let b64 = base64::engine::general_purpose::STANDARD.encode(vk.as_bytes());

    beardog_errors::process_env::set_var(ENV_TRUSTED_ISSUERS, &b64);
    let registry = TrustedIssuerRegistry::new();
    let seeded = registry.seed_from_env().expect("should succeed");
    assert_eq!(seeded, 1);

    let did = did_from_verifying_key(&vk);
    let (_, info) = registry.get(&did).expect("should be registered");
    assert!(info.gate_id.is_none());
    assert!(info.family_id.is_none());

    beardog_errors::process_env::remove_var(ENV_TRUSTED_ISSUERS);
}
