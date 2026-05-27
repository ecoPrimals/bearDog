// SPDX-License-Identifier: AGPL-3.0-or-later

use super::super::super::MethodHandler;
use super::super::*;
use super::helpers::{accept_bond, propose_bond};
use base64::Engine;
use base64::engine::general_purpose::STANDARD as BASE64;

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
        bond.acceptor_signature = Some(BASE64.encode([0xCC; 64]));
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
