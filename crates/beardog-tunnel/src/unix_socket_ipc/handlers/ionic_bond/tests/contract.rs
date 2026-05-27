// SPDX-License-Identifier: AGPL-3.0-or-later

use super::super::super::MethodHandler;
use super::super::*;
use base64::Engine;
use base64::engine::general_purpose::STANDARD as BASE64;

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

    let sig_b64 = result["signature"].as_str().unwrap();
    let sig_bytes = BASE64
        .decode(sig_b64)
        .expect("signature must be valid base64");
    assert_eq!(sig_bytes.len(), 64, "Ed25519 signature = 64 bytes");
    let pk_b64 = result["public_key"].as_str().unwrap();
    let pk_bytes = BASE64
        .decode(pk_b64)
        .expect("public_key must be valid base64");
    assert_eq!(pk_bytes.len(), 32, "Ed25519 public key = 32 bytes");
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
                "signature": BASE64.encode([0xDE; 64]),
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
    assert!(methods.contains(&"crypto.contract.propose"));
    assert!(methods.contains(&"crypto.contract.countersign"));
    assert!(methods.contains(&"crypto.contract.verify"));
    assert_eq!(methods.len(), 12);
}

#[tokio::test]
async fn sign_contract_with_ttl_returns_expires_at() {
    let handler = IonicBondHandler::new();
    let provider = crate::test_helpers::mocks::create_minimal_beardog_provider().await;

    let params = serde_json::json!({
        "signer": "tower_a",
        "terms": {"type": "gpu_lease", "hours": 4},
        "context": "gpu_lease",
        "ttl_seconds": 3600
    });

    let result = handler
        .handle("crypto.sign_contract", Some(&params), &provider)
        .await
        .expect("sign with TTL");

    assert!(
        result["expires_at"].as_str().is_some(),
        "expires_at present"
    );
    assert!(!result["terms_hash"].as_str().unwrap().is_empty());
    assert!(!result["signed_at"].as_str().unwrap().is_empty());
}

#[tokio::test]
async fn sign_contract_without_ttl_has_no_expires_at() {
    let handler = IonicBondHandler::new();
    let provider = crate::test_helpers::mocks::create_minimal_beardog_provider().await;

    let params = serde_json::json!({
        "signer": "tower_a",
        "terms": {"type": "data_egress", "gb": 10}
    });

    let result = handler
        .handle("crypto.sign_contract", Some(&params), &provider)
        .await
        .expect("sign without TTL");

    assert!(
        result.get("expires_at").map_or(true, |v| v.is_null()),
        "no expires_at without TTL"
    );
}

#[tokio::test]
async fn verify_contract_with_future_expiry_is_valid() {
    let handler = IonicBondHandler::new();
    let provider = crate::test_helpers::mocks::create_minimal_beardog_provider().await;

    let params = serde_json::json!({
        "signer": "tower_a",
        "terms": {"type": "lease"},
        "ttl_seconds": 3600
    });

    let signed = handler
        .handle("crypto.sign_contract", Some(&params), &provider)
        .await
        .expect("sign");

    let verify_params = serde_json::json!({
        "terms_hash": signed["terms_hash"],
        "signature": signed["signature"],
        "public_key": signed["public_key"],
        "expires_at": signed["expires_at"]
    });

    let verified = handler
        .handle("crypto.verify_contract", Some(&verify_params), &provider)
        .await
        .expect("verify");

    assert_eq!(verified["valid"], true);
    assert_eq!(verified["expired"], false);
}

#[tokio::test]
async fn verify_contract_with_past_expiry_is_invalid() {
    let handler = IonicBondHandler::new();
    let provider = crate::test_helpers::mocks::create_minimal_beardog_provider().await;

    let params = serde_json::json!({
        "signer": "tower_a",
        "terms": {"type": "lease"}
    });

    let signed = handler
        .handle("crypto.sign_contract", Some(&params), &provider)
        .await
        .expect("sign");

    let past = chrono::Utc::now() - chrono::Duration::hours(1);
    let verify_params = serde_json::json!({
        "terms_hash": signed["terms_hash"],
        "signature": signed["signature"],
        "public_key": signed["public_key"],
        "expires_at": past.to_rfc3339()
    });

    let verified = handler
        .handle("crypto.verify_contract", Some(&verify_params), &provider)
        .await
        .expect("verify");

    assert_eq!(verified["valid"], false);
    assert_eq!(verified["expired"], true);
    assert!(
        verified["error"]
            .as_str()
            .unwrap()
            .contains("ionic lease expired")
    );
}

#[tokio::test]
async fn verify_contract_without_expiry_ignores_lease() {
    let handler = IonicBondHandler::new();
    let provider = crate::test_helpers::mocks::create_minimal_beardog_provider().await;

    let params = serde_json::json!({
        "signer": "tower_a",
        "terms": {"type": "permanent"}
    });

    let signed = handler
        .handle("crypto.sign_contract", Some(&params), &provider)
        .await
        .expect("sign");

    let verify_params = serde_json::json!({
        "terms_hash": signed["terms_hash"],
        "signature": signed["signature"],
        "public_key": signed["public_key"]
    });

    let verified = handler
        .handle("crypto.verify_contract", Some(&verify_params), &provider)
        .await
        .expect("verify");

    assert_eq!(verified["valid"], true);
    assert!(
        verified.get("expired").map_or(true, |v| v.is_null()),
        "no expired field without expiry"
    );
}
