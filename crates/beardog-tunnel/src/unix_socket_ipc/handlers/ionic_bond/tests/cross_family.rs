// SPDX-License-Identifier: AGPL-3.0-or-later

use super::super::super::MethodHandler;
use super::super::*;
use super::helpers::sign_as_countersigner;
use base64::Engine;
use base64::engine::general_purpose::STANDARD as BASE64;

#[tokio::test]
async fn cross_family_contract_propose_countersign_lifecycle() {
    let handler = IonicBondHandler::new();
    let provider = crate::test_helpers::mocks::create_minimal_beardog_provider().await;

    let terms = serde_json::json!({
        "type": "gpu_lease",
        "provider_family": "family_alpha",
        "consumer_family": "family_beta",
        "duration_hours": 24,
    });

    let propose_result = handler
        .handle(
            "crypto.contract.propose",
            Some(&serde_json::json!({
                "proposer": "family_alpha",
                "terms": terms,
                "context": "gpu_lease",
            })),
            &provider,
        )
        .await
        .expect("propose");

    let contract_id = propose_result["contract_id"].as_str().unwrap();
    let terms_hash = propose_result["terms_hash"].as_str().unwrap();
    assert!(!contract_id.is_empty());
    assert!(!terms_hash.is_empty());
    assert!(propose_result["proposer_signature"].as_str().is_some());
    assert!(propose_result["proposer_public_key"].as_str().is_some());

    let (cs_sig, cs_pk) = sign_as_countersigner(terms_hash);

    let countersign_result = handler
        .handle(
            "crypto.contract.countersign",
            Some(&serde_json::json!({
                "contract_id": contract_id,
                "countersigner": "family_beta",
                "countersigner_signature": cs_sig,
                "countersigner_public_key": cs_pk,
            })),
            &provider,
        )
        .await
        .expect("countersign");

    let contract = &countersign_result["contract"];
    assert_eq!(contract["proposer"], "family_alpha");
    assert_eq!(contract["countersigner"], "family_beta");
    assert_eq!(contract["terms_hash"], terms_hash);
    assert_eq!(contract["context"], "gpu_lease");
    assert!(contract["sealed_at"].as_str().is_some());
    assert!(contract["proposer_signature"].as_str().is_some());
    assert!(contract["countersigner_signature"].as_str().is_some());
}

#[tokio::test]
async fn cross_family_contract_verify_both_signatures() {
    let handler = IonicBondHandler::new();
    let provider = crate::test_helpers::mocks::create_minimal_beardog_provider().await;

    let propose_result = handler
        .handle(
            "crypto.contract.propose",
            Some(&serde_json::json!({
                "proposer": "hotSpring",
                "terms": { "scope": "GAP-HS-005" },
            })),
            &provider,
        )
        .await
        .expect("propose");

    let terms_hash = propose_result["terms_hash"].as_str().unwrap();
    let (cs_sig, cs_pk) = sign_as_countersigner(terms_hash);

    let countersign_result = handler
        .handle(
            "crypto.contract.countersign",
            Some(&serde_json::json!({
                "contract_id": propose_result["contract_id"],
                "countersigner": "coldSpring",
                "countersigner_signature": cs_sig,
                "countersigner_public_key": cs_pk,
            })),
            &provider,
        )
        .await
        .expect("countersign");

    let contract = &countersign_result["contract"];

    let verify_result = handler
        .handle(
            "crypto.contract.verify",
            Some(&serde_json::json!({
                "terms_hash": contract["terms_hash"],
                "proposer_signature": contract["proposer_signature"],
                "proposer_public_key": contract["proposer_public_key"],
                "countersigner_signature": contract["countersigner_signature"],
                "countersigner_public_key": contract["countersigner_public_key"],
            })),
            &provider,
        )
        .await
        .expect("verify");

    assert_eq!(verify_result["valid"], true);
    assert!(
        verify_result
            .get("failed_party")
            .is_none_or(|v| v.is_null()),
        "failed_party should be absent or null on success"
    );
    assert!(
        verify_result.get("error").is_none_or(|v| v.is_null()),
        "error should be absent or null on success"
    );
}

#[tokio::test]
async fn cross_family_contract_verify_rejects_tampered_countersigner_sig() {
    let handler = IonicBondHandler::new();
    let provider = crate::test_helpers::mocks::create_minimal_beardog_provider().await;

    let propose_result = handler
        .handle(
            "crypto.contract.propose",
            Some(&serde_json::json!({
                "proposer": "wetSpring",
                "terms": { "provenance": "cross-spring" },
            })),
            &provider,
        )
        .await
        .expect("propose");

    let terms_hash = propose_result["terms_hash"].as_str().unwrap();
    let (cs_sig, cs_pk) = sign_as_countersigner(terms_hash);

    let countersign_result = handler
        .handle(
            "crypto.contract.countersign",
            Some(&serde_json::json!({
                "contract_id": propose_result["contract_id"],
                "countersigner": "drySpring",
                "countersigner_signature": cs_sig,
                "countersigner_public_key": cs_pk,
            })),
            &provider,
        )
        .await
        .expect("countersign");

    let contract = &countersign_result["contract"];

    let verify_result = handler
        .handle(
            "crypto.contract.verify",
            Some(&serde_json::json!({
                "terms_hash": contract["terms_hash"],
                "proposer_signature": contract["proposer_signature"],
                "proposer_public_key": contract["proposer_public_key"],
                "countersigner_signature": BASE64.encode([0xFF; 64]),
                "countersigner_public_key": contract["countersigner_public_key"],
            })),
            &provider,
        )
        .await
        .expect("verify");

    assert_eq!(verify_result["valid"], false);
    assert_eq!(verify_result["failed_party"], "countersigner");
}

#[tokio::test]
async fn cross_family_contract_countersign_rejects_invalid_sig() {
    let handler = IonicBondHandler::new();
    let provider = crate::test_helpers::mocks::create_minimal_beardog_provider().await;

    let propose_result = handler
        .handle(
            "crypto.contract.propose",
            Some(&serde_json::json!({
                "proposer": "healthSpring",
                "terms": { "dual_tower": "ionic" },
            })),
            &provider,
        )
        .await
        .expect("propose");

    let (_, cs_pk) = sign_as_countersigner("irrelevant");

    let result = handler
        .handle(
            "crypto.contract.countersign",
            Some(&serde_json::json!({
                "contract_id": propose_result["contract_id"],
                "countersigner": "adversary",
                "countersigner_signature": BASE64.encode([0xAA; 64]),
                "countersigner_public_key": cs_pk,
            })),
            &provider,
        )
        .await;

    assert!(result.is_err());
    assert!(
        result
            .unwrap_err()
            .contains("Countersigner signature invalid"),
        "should reject invalid countersigner signature"
    );
}

#[tokio::test]
async fn cross_family_contract_countersign_rejects_unknown_id() {
    let handler = IonicBondHandler::new();
    let provider = crate::test_helpers::mocks::create_minimal_beardog_provider().await;

    let result = handler
        .handle(
            "crypto.contract.countersign",
            Some(&serde_json::json!({
                "contract_id": "nonexistent",
                "countersigner": "someone",
                "countersigner_signature": BASE64.encode([0; 64]),
                "countersigner_public_key": BASE64.encode([0; 32]),
            })),
            &provider,
        )
        .await;

    assert!(result.is_err());
    assert!(result.unwrap_err().contains("No pending contract"));
}

#[tokio::test]
async fn cross_family_contract_with_ttl() {
    let handler = IonicBondHandler::new();
    let provider = crate::test_helpers::mocks::create_minimal_beardog_provider().await;

    let propose_result = handler
        .handle(
            "crypto.contract.propose",
            Some(&serde_json::json!({
                "proposer": "tower_a",
                "terms": { "scope": "time_bounded" },
                "ttl_seconds": 3600,
            })),
            &provider,
        )
        .await
        .expect("propose");

    assert!(propose_result["expires_at"].as_str().is_some());
    let terms_hash = propose_result["terms_hash"].as_str().unwrap();
    let (cs_sig, cs_pk) = sign_as_countersigner(terms_hash);

    let countersign_result = handler
        .handle(
            "crypto.contract.countersign",
            Some(&serde_json::json!({
                "contract_id": propose_result["contract_id"],
                "countersigner": "tower_b",
                "countersigner_signature": cs_sig,
                "countersigner_public_key": cs_pk,
            })),
            &provider,
        )
        .await
        .expect("countersign within TTL");

    assert_eq!(countersign_result["contract"]["proposer"], "tower_a");
}
