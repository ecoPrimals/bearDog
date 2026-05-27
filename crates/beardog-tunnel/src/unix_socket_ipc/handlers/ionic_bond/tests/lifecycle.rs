// SPDX-License-Identifier: AGPL-3.0-or-later

use super::super::super::MethodHandler;
use super::super::*;
use super::helpers::{accept_bond, propose_bond, sign_as_acceptor};
use base64::Engine;
use base64::engine::general_purpose::STANDARD as BASE64;
use ed25519_dalek::SigningKey;

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
    let wrong_sig = BASE64.encode([0xAA; 64]);
    let pubkey = BASE64.encode(key.verifying_key().as_bytes());

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
    assert_eq!(methods.len(), 12);
    assert!(methods.contains(&"crypto.ionic_bond.propose"));
    assert!(methods.contains(&"crypto.ionic_bond.accept"));
    assert!(methods.contains(&"crypto.ionic_bond.seal"));
    assert!(methods.contains(&"crypto.ionic_bond.verify"));
    assert!(methods.contains(&"crypto.ionic_bond.verify_proposal"));
    assert!(methods.contains(&"crypto.sign_contract"));
    assert!(methods.contains(&"crypto.verify_contract"));
    assert!(methods.contains(&"crypto.contract.propose"));
    assert!(methods.contains(&"crypto.contract.countersign"));
    assert!(methods.contains(&"crypto.contract.verify"));
}

#[tokio::test]
async fn verify_proposal_valid() {
    let handler = IonicBondHandler::new();
    let provider = crate::test_helpers::mocks::create_minimal_beardog_provider().await;

    let (proposal_id, terms_hash) = propose_bond(&handler, &provider, "gate_a", "gate_b").await;

    let params = serde_json::json!({ "proposal_id": proposal_id });
    let result = handler
        .handle(
            "crypto.ionic_bond.verify_proposal",
            Some(&params),
            &provider,
        )
        .await
        .expect("verify_proposal");

    assert_eq!(result["valid"], true);
    assert_eq!(result["proposal_id"], proposal_id);
    assert_eq!(result["terms_hash"], terms_hash);
    assert_eq!(result["proposer"], "gate_a");
    assert_eq!(result["target"], "gate_b");
    assert!(result["proposer_signature"].as_str().is_some());
    assert!(result["proposer_public_key"].as_str().is_some());
    assert!(result["error"].is_null());
}

#[tokio::test]
async fn verify_proposal_not_found() {
    let handler = IonicBondHandler::new();
    let provider = crate::test_helpers::mocks::create_minimal_beardog_provider().await;

    let params = serde_json::json!({ "proposal_id": "nonexistent" });
    let result = handler
        .handle(
            "crypto.ionic_bond.verify_proposal",
            Some(&params),
            &provider,
        )
        .await
        .expect("verify_proposal returns ok with valid=false");

    assert_eq!(result["valid"], false);
}

#[tokio::test]
async fn propose_returns_public_key() {
    let handler = IonicBondHandler::new();
    let provider = crate::test_helpers::mocks::create_minimal_beardog_provider().await;

    let propose_params = serde_json::json!({ "proposer": "alpha", "target": "beta" });
    let result = handler
        .handle(
            "crypto.ionic_bond.propose",
            Some(&propose_params),
            &provider,
        )
        .await
        .expect("propose");

    assert!(result["proposer_public_key"].is_string());
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
        bond.proposer_signature = Some(BASE64.encode([0xDE; 64]));
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
        bond.acceptor_signature = Some(BASE64.encode([0xBB; 64]));
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
