// SPDX-License-Identifier: AGPL-3.0-or-later

use super::super::MethodHandler;
use super::*;
use crate::btsp_provider::BeardogBtspProvider;
use ed25519_dalek::{Signer, SigningKey};
use std::sync::Arc;

/// Generate a real Ed25519 signature over `terms_hash` and return
/// `(signature_hex, public_key_hex)` for use in accept params.
fn sign_as_acceptor(terms_hash: &str) -> (String, String) {
    let key = SigningKey::from_bytes(&[0x42; 32]);
    let sig = key.sign(terms_hash.as_bytes());
    (
        hex::encode(sig.to_bytes()),
        hex::encode(key.verifying_key().as_bytes()),
    )
}

/// Helper: propose a bond -> return (`proposal_id`, `terms_hash`).
async fn propose_bond(
    handler: &IonicBondHandler,
    provider: &Arc<BeardogBtspProvider>,
    proposer: &str,
    target: &str,
) -> (String, String) {
    let params = serde_json::json!({ "proposer": proposer, "target": target });
    let result = handler
        .handle("crypto.ionic_bond.propose", Some(&params), provider)
        .await
        .expect("propose");
    (
        result["proposal_id"].as_str().unwrap().to_string(),
        result["terms_hash"].as_str().unwrap().to_string(),
    )
}

/// Helper: accept a bond with real Ed25519 signature -> return `bond_id`.
async fn accept_bond(
    handler: &IonicBondHandler,
    provider: &Arc<BeardogBtspProvider>,
    proposal_id: &str,
    terms_hash: &str,
    acceptor: &str,
) -> String {
    let (sig, pubkey) = sign_as_acceptor(terms_hash);
    let params = serde_json::json!({
        "proposal_id": proposal_id,
        "acceptor": acceptor,
        "acceptor_signature": sig,
        "acceptor_public_key": pubkey,
    });
    let result = handler
        .handle("crypto.ionic_bond.accept", Some(&params), provider)
        .await
        .expect("accept");
    result["bond"]["bond_id"].as_str().unwrap().to_string()
}

#[tokio::test]
async fn propose_accept_verify_lifecycle() {
    let handler = IonicBondHandler::new();
    let provider = crate::test_helpers::mocks::create_minimal_beardog_provider().await;

    let propose_params = serde_json::json!({
        "proposer": "tower_a",
        "target": "tower_b",
        "trust_model": "dual_tower_enclave",
        "encryption_tier": "aead",
        "allowed_capabilities": ["science.pkpd.simulate", "compute.dispatch.submit"]
    });

    let propose_result = handler
        .handle(
            "crypto.ionic_bond.propose",
            Some(&propose_params),
            &provider,
        )
        .await
        .expect("propose");

    let proposal_id = propose_result["proposal_id"].as_str().unwrap().to_string();
    let terms_hash = propose_result["terms_hash"].as_str().unwrap().to_string();

    assert!(!proposal_id.is_empty());
    assert!(!terms_hash.is_empty());

    let bond_id = accept_bond(&handler, &provider, &proposal_id, &terms_hash, "tower_b").await;

    let verify_params = serde_json::json!({ "bond_id": bond_id });
    let verify_result = handler
        .handle("crypto.ionic_bond.verify", Some(&verify_params), &provider)
        .await
        .expect("verify");

    assert_eq!(verify_result["valid"], true);
    assert_eq!(verify_result["state"], "active");
    assert!(verify_result["bond"]["proposer_public_key"].is_string());
    assert!(verify_result["bond"]["acceptor_public_key"].is_string());
}

#[tokio::test]
async fn accept_rejects_invalid_signature() {
    let handler = IonicBondHandler::new();
    let provider = crate::test_helpers::mocks::create_minimal_beardog_provider().await;

    let (proposal_id, _terms_hash) = propose_bond(&handler, &provider, "a", "b").await;

    let key = SigningKey::from_bytes(&[0x42; 32]);
    let wrong_sig = hex::encode([0xAA; 64]);
    let pubkey = hex::encode(key.verifying_key().as_bytes());

    let params = serde_json::json!({
        "proposal_id": proposal_id,
        "acceptor": "b",
        "acceptor_signature": wrong_sig,
        "acceptor_public_key": pubkey,
    });

    let result = handler
        .handle("crypto.ionic_bond.accept", Some(&params), &provider)
        .await;

    assert!(result.is_err(), "should reject invalid signature");
    assert!(
        result.unwrap_err().contains("verification failed"),
        "error should mention verification"
    );
}

#[tokio::test]
async fn revoke_prevents_verification() {
    let handler = IonicBondHandler::new();
    let provider = crate::test_helpers::mocks::create_minimal_beardog_provider().await;

    let (proposal_id, terms_hash) = propose_bond(&handler, &provider, "a", "b").await;
    let bond_id = accept_bond(&handler, &provider, &proposal_id, &terms_hash, "b").await;

    let revoke_params = serde_json::json!({
        "bond_id": bond_id,
        "revoker": "a"
    });
    handler
        .handle("crypto.ionic_bond.revoke", Some(&revoke_params), &provider)
        .await
        .expect("revoke");

    let verify_params = serde_json::json!({ "bond_id": bond_id });
    let verify_result = handler
        .handle("crypto.ionic_bond.verify", Some(&verify_params), &provider)
        .await
        .expect("verify");

    assert_eq!(verify_result["valid"], false);
    assert_eq!(verify_result["state"], "revoked");
}

#[tokio::test]
async fn list_filters_by_domain() {
    let handler = IonicBondHandler::new();
    let provider = crate::test_helpers::mocks::create_minimal_beardog_provider().await;

    for (proposer, target) in [("x", "y"), ("a", "b")] {
        let (pid, hash) = propose_bond(&handler, &provider, proposer, target).await;
        accept_bond(&handler, &provider, &pid, &hash, target).await;
    }

    let list_params = serde_json::json!({ "domain": "x" });
    let list_result = handler
        .handle("crypto.ionic_bond.list", Some(&list_params), &provider)
        .await
        .expect("list");

    assert_eq!(list_result["bonds"].as_array().unwrap().len(), 1);
}

#[tokio::test]
async fn methods_list() {
    let handler = IonicBondHandler::new();
    let methods = handler.methods();
    assert_eq!(methods.len(), 8);
    assert!(methods.contains(&"crypto.ionic_bond.propose"));
    assert!(methods.contains(&"crypto.ionic_bond.accept"));
    assert!(methods.contains(&"crypto.ionic_bond.seal"));
    assert!(methods.contains(&"crypto.ionic_bond.verify"));
    assert!(methods.contains(&"crypto.sign_contract"));
    assert!(methods.contains(&"crypto.verify_contract"));
}

#[tokio::test]
async fn sealed_bond_stores_terms_hash() {
    let handler = IonicBondHandler::new();
    let provider = crate::test_helpers::mocks::create_minimal_beardog_provider().await;

    let (proposal_id, terms_hash) = propose_bond(&handler, &provider, "alpha", "beta").await;
    let bond_id = accept_bond(&handler, &provider, &proposal_id, &terms_hash, "beta").await;

    let verify_params = serde_json::json!({ "bond_id": bond_id });
    let result = handler
        .handle("crypto.ionic_bond.verify", Some(&verify_params), &provider)
        .await
        .expect("verify");

    assert_eq!(result["valid"], true);
    let bond_terms = result["bond"]["terms_hash"].as_str().unwrap();
    assert_eq!(bond_terms, terms_hash, "sealed bond must carry terms_hash");
}

#[tokio::test]
async fn verify_detects_tampered_proposer_signature() {
    let handler = IonicBondHandler::new();
    let provider = crate::test_helpers::mocks::create_minimal_beardog_provider().await;

    let (proposal_id, terms_hash) = propose_bond(&handler, &provider, "p", "q").await;
    let bond_id = accept_bond(&handler, &provider, &proposal_id, &terms_hash, "q").await;

    {
        let mut bonds = handler.bonds.write().await;
        let bond = bonds.get_mut(&bond_id).unwrap();
        bond.proposer_signature = Some(hex::encode([0xDE; 64]));
    }

    let verify_params = serde_json::json!({ "bond_id": bond_id });
    let result = handler
        .handle("crypto.ionic_bond.verify", Some(&verify_params), &provider)
        .await
        .expect("verify should succeed but report invalid");

    assert_eq!(result["valid"], false);
    assert!(
        result["error"]
            .as_str()
            .unwrap()
            .contains("signature verification failed"),
    );
}

#[tokio::test]
async fn verify_detects_tampered_acceptor_signature() {
    let handler = IonicBondHandler::new();
    let provider = crate::test_helpers::mocks::create_minimal_beardog_provider().await;

    let (proposal_id, terms_hash) = propose_bond(&handler, &provider, "r", "s").await;
    let bond_id = accept_bond(&handler, &provider, &proposal_id, &terms_hash, "s").await;

    {
        let mut bonds = handler.bonds.write().await;
        let bond = bonds.get_mut(&bond_id).unwrap();
        bond.acceptor_signature = Some(hex::encode([0xBB; 64]));
    }

    let verify_params = serde_json::json!({ "bond_id": bond_id });
    let result = handler
        .handle("crypto.ionic_bond.verify", Some(&verify_params), &provider)
        .await
        .expect("verify should succeed but report invalid");

    assert_eq!(result["valid"], false);
}

#[tokio::test]
async fn full_lifecycle_propose_accept_list_revoke() {
    let handler = IonicBondHandler::new();
    let provider = crate::test_helpers::mocks::create_minimal_beardog_provider().await;

    let (pid, hash) = propose_bond(&handler, &provider, "tower_a", "tower_b").await;
    let bond_id = accept_bond(&handler, &provider, &pid, &hash, "tower_b").await;

    let list_all = handler
        .handle("crypto.ionic_bond.list", None, &provider)
        .await
        .expect("list all");
    assert_eq!(list_all["bonds"].as_array().unwrap().len(), 1);

    let verify = handler
        .handle(
            "crypto.ionic_bond.verify",
            Some(&serde_json::json!({ "bond_id": bond_id })),
            &provider,
        )
        .await
        .expect("verify");
    assert_eq!(verify["valid"], true);
    assert_eq!(verify["state"], "active");

    handler
        .handle(
            "crypto.ionic_bond.revoke",
            Some(&serde_json::json!({ "bond_id": bond_id, "revoker": "tower_a" })),
            &provider,
        )
        .await
        .expect("revoke");

    let verify_after = handler
        .handle(
            "crypto.ionic_bond.verify",
            Some(&serde_json::json!({ "bond_id": bond_id })),
            &provider,
        )
        .await
        .expect("verify after revoke");
    assert_eq!(verify_after["valid"], false);
    assert_eq!(verify_after["state"], "revoked");

    let list_active = handler
        .handle(
            "crypto.ionic_bond.list",
            Some(&serde_json::json!({ "state": "active" })),
            &provider,
        )
        .await
        .expect("list active");
    assert!(list_active["bonds"].as_array().unwrap().is_empty());
}

#[tokio::test]
async fn sign_contract_returns_valid_signature() {
    let handler = IonicBondHandler::new();
    let provider = crate::test_helpers::mocks::create_minimal_beardog_provider().await;

    let params = serde_json::json!({
        "signer": "hotSpring",
        "terms": {
            "type": "gpu_lease",
            "provider": "tower_a",
            "consumer": "tower_b",
            "duration_hours": 24,
            "capabilities": ["compute.dispatch.submit"]
        },
        "context": "gpu_lease"
    });

    let result = handler
        .handle("crypto.sign_contract", Some(&params), &provider)
        .await
        .expect("sign_contract");

    assert!(!result["terms_hash"].as_str().unwrap().is_empty());
    assert!(!result["signature"].as_str().unwrap().is_empty());
    assert!(!result["public_key"].as_str().unwrap().is_empty());
    assert!(!result["signed_at"].as_str().unwrap().is_empty());

    let sig_hex = result["signature"].as_str().unwrap();
    assert_eq!(sig_hex.len(), 128, "Ed25519 signature = 64 bytes = 128 hex");
    let pk_hex = result["public_key"].as_str().unwrap();
    assert_eq!(pk_hex.len(), 64, "Ed25519 public key = 32 bytes = 64 hex");
}

#[tokio::test]
async fn sign_then_verify_contract_roundtrip() {
    let handler = IonicBondHandler::new();
    let provider = crate::test_helpers::mocks::create_minimal_beardog_provider().await;

    let terms = serde_json::json!({
        "federation": "cern_grid",
        "parties": ["family_a", "family_b"],
        "scope": "data_egress_fence"
    });

    let sign_result = handler
        .handle(
            "crypto.sign_contract",
            Some(&serde_json::json!({
                "signer": "family_a",
                "terms": terms,
            })),
            &provider,
        )
        .await
        .expect("sign");

    let verify_result = handler
        .handle(
            "crypto.verify_contract",
            Some(&serde_json::json!({
                "terms_hash": sign_result["terms_hash"],
                "signature": sign_result["signature"],
                "public_key": sign_result["public_key"],
            })),
            &provider,
        )
        .await
        .expect("verify");

    assert_eq!(verify_result["valid"], true);
    assert!(
        verify_result
            .get("error")
            .and_then(|e| e.as_str())
            .is_none()
    );
}

#[tokio::test]
async fn verify_contract_rejects_tampered_signature() {
    let handler = IonicBondHandler::new();
    let provider = crate::test_helpers::mocks::create_minimal_beardog_provider().await;

    let sign_result = handler
        .handle(
            "crypto.sign_contract",
            Some(&serde_json::json!({
                "signer": "tower_a",
                "terms": { "scope": "test" },
            })),
            &provider,
        )
        .await
        .expect("sign");

    let verify_result = handler
        .handle(
            "crypto.verify_contract",
            Some(&serde_json::json!({
                "terms_hash": sign_result["terms_hash"],
                "signature": hex::encode([0xDE; 64]),
                "public_key": sign_result["public_key"],
            })),
            &provider,
        )
        .await
        .expect("verify");

    assert_eq!(verify_result["valid"], false);
    assert!(
        verify_result["error"]
            .as_str()
            .unwrap()
            .contains("verification failed")
    );
}

#[tokio::test]
async fn sign_contract_deterministic_terms_hash() {
    let handler = IonicBondHandler::new();
    let provider = crate::test_helpers::mocks::create_minimal_beardog_provider().await;

    let terms = serde_json::json!({
        "z_field": "last",
        "a_field": "first",
        "m_field": [1, 2, 3]
    });

    let r1 = handler
        .handle(
            "crypto.sign_contract",
            Some(&serde_json::json!({ "signer": "a", "terms": terms })),
            &provider,
        )
        .await
        .expect("sign 1");

    let terms_reordered = serde_json::json!({
        "m_field": [1, 2, 3],
        "a_field": "first",
        "z_field": "last"
    });

    let r2 = handler
        .handle(
            "crypto.sign_contract",
            Some(&serde_json::json!({ "signer": "a", "terms": terms_reordered })),
            &provider,
        )
        .await
        .expect("sign 2");

    assert_eq!(
        r1["terms_hash"].as_str().unwrap(),
        r2["terms_hash"].as_str().unwrap(),
        "canonical JSON must produce identical hashes regardless of key order"
    );
}

#[tokio::test]
async fn methods_list_includes_contract_signing() {
    let handler = IonicBondHandler::new();
    let methods = handler.methods();
    assert!(methods.contains(&"crypto.sign_contract"));
    assert!(methods.contains(&"crypto.verify_contract"));
    assert_eq!(methods.len(), 8);
}

#[tokio::test]
async fn propose_accept_seal_lifecycle() {
    let handler = IonicBondHandler::new();
    let provider = crate::test_helpers::mocks::create_minimal_beardog_provider().await;

    let (pid, hash) = propose_bond(&handler, &provider, "tower_a", "tower_b").await;
    let bond_id = accept_bond(&handler, &provider, &pid, &hash, "tower_b").await;

    let seal_result = handler
        .handle(
            "crypto.ionic_bond.seal",
            Some(&serde_json::json!({ "bond_id": bond_id, "sealer": "tower_a" })),
            &provider,
        )
        .await
        .expect("seal");

    assert_eq!(seal_result["sealed"], true);
    assert_eq!(seal_result["bond"]["state"], "sealed");
    assert!(seal_result["error"].is_null());

    let verify_result = handler
        .handle(
            "crypto.ionic_bond.verify",
            Some(&serde_json::json!({ "bond_id": bond_id })),
            &provider,
        )
        .await
        .expect("verify sealed bond");

    assert_eq!(verify_result["valid"], true);
    assert_eq!(verify_result["state"], "sealed");
}

#[tokio::test]
async fn seal_rejects_unauthorized_sealer() {
    let handler = IonicBondHandler::new();
    let provider = crate::test_helpers::mocks::create_minimal_beardog_provider().await;

    let (pid, hash) = propose_bond(&handler, &provider, "a", "b").await;
    let bond_id = accept_bond(&handler, &provider, &pid, &hash, "b").await;

    let result = handler
        .handle(
            "crypto.ionic_bond.seal",
            Some(&serde_json::json!({ "bond_id": bond_id, "sealer": "intruder" })),
            &provider,
        )
        .await;

    assert!(result.is_err());
    assert!(
        result
            .unwrap_err()
            .contains("neither proposer nor acceptor")
    );
}

#[tokio::test]
async fn seal_rejects_revoked_bond() {
    let handler = IonicBondHandler::new();
    let provider = crate::test_helpers::mocks::create_minimal_beardog_provider().await;

    let (pid, hash) = propose_bond(&handler, &provider, "a", "b").await;
    let bond_id = accept_bond(&handler, &provider, &pid, &hash, "b").await;

    handler
        .handle(
            "crypto.ionic_bond.revoke",
            Some(&serde_json::json!({ "bond_id": bond_id, "revoker": "a" })),
            &provider,
        )
        .await
        .expect("revoke");

    let seal_result = handler
        .handle(
            "crypto.ionic_bond.seal",
            Some(&serde_json::json!({ "bond_id": bond_id, "sealer": "a" })),
            &provider,
        )
        .await
        .expect("seal of revoked bond should not error but report failure");

    assert_eq!(seal_result["sealed"], false);
    assert!(
        seal_result["error"]
            .as_str()
            .unwrap()
            .contains("must be Active")
    );
}

#[tokio::test]
async fn seal_detects_tampered_signature() {
    let handler = IonicBondHandler::new();
    let provider = crate::test_helpers::mocks::create_minimal_beardog_provider().await;

    let (pid, hash) = propose_bond(&handler, &provider, "x", "y").await;
    let bond_id = accept_bond(&handler, &provider, &pid, &hash, "y").await;

    {
        let mut bonds = handler.bonds.write().await;
        let bond = bonds.get_mut(&bond_id).unwrap();
        bond.acceptor_signature = Some(hex::encode([0xCC; 64]));
    }

    let seal_result = handler
        .handle(
            "crypto.ionic_bond.seal",
            Some(&serde_json::json!({ "bond_id": bond_id, "sealer": "x" })),
            &provider,
        )
        .await
        .expect("seal should succeed but report failure");

    assert_eq!(seal_result["sealed"], false);
    assert!(
        seal_result["error"]
            .as_str()
            .unwrap()
            .contains("signature verification failed")
    );
}

#[tokio::test]
async fn sealed_bond_can_be_revoked() {
    let handler = IonicBondHandler::new();
    let provider = crate::test_helpers::mocks::create_minimal_beardog_provider().await;

    let (pid, hash) = propose_bond(&handler, &provider, "a", "b").await;
    let bond_id = accept_bond(&handler, &provider, &pid, &hash, "b").await;

    handler
        .handle(
            "crypto.ionic_bond.seal",
            Some(&serde_json::json!({ "bond_id": bond_id, "sealer": "b" })),
            &provider,
        )
        .await
        .expect("seal");

    handler
        .handle(
            "crypto.ionic_bond.revoke",
            Some(&serde_json::json!({ "bond_id": bond_id, "revoker": "b" })),
            &provider,
        )
        .await
        .expect("revoke sealed bond");

    let verify = handler
        .handle(
            "crypto.ionic_bond.verify",
            Some(&serde_json::json!({ "bond_id": bond_id })),
            &provider,
        )
        .await
        .expect("verify");

    assert_eq!(verify["valid"], false);
    assert_eq!(verify["state"], "revoked");
}
