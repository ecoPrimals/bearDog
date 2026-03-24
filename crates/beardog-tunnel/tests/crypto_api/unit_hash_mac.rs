// SPDX-License-Identifier: AGPL-3.0-only
#![allow(clippy::expect_used, clippy::unwrap_used, missing_docs)]

//! Blake3 and HMAC-SHA256 unit tests.

use base64::Engine;
use beardog_tunnel::unix_socket_ipc::handlers::crypto::*;
use serde_json::json;

#[tokio::test]
async fn test_blake3_empty_data() {
    let data_b64 = base64::engine::general_purpose::STANDARD.encode(b"");

    let params = json!({"data": data_b64});

    let result = handle_blake3_hash(Some(&params)).await;
    assert!(result.is_ok(), "Empty data hashing failed");

    let hash = result.unwrap();
    assert_eq!(
        base64::engine::general_purpose::STANDARD
            .decode(hash["hash"].as_str().unwrap())
            .unwrap()
            .len(),
        32
    );
}

#[tokio::test]
async fn test_blake3_determinism() {
    let data_b64 = base64::engine::general_purpose::STANDARD.encode(b"deterministic test");

    let params = json!({"data": data_b64});

    let result1 = handle_blake3_hash(Some(&params)).await.unwrap();
    let result2 = handle_blake3_hash(Some(&params)).await.unwrap();
    let result3 = handle_blake3_hash(Some(&params)).await.unwrap();

    assert_eq!(result1["hash"], result2["hash"]);
    assert_eq!(result2["hash"], result3["hash"]);
}

#[tokio::test]
async fn test_blake3_large_data() {
    // Hash 100MB of data
    let large_data = vec![0x77; 100 * 1024 * 1024];
    let data_b64 = base64::engine::general_purpose::STANDARD.encode(&large_data);

    let params = json!({"data": data_b64});

    let result = handle_blake3_hash(Some(&params)).await;
    assert!(result.is_ok(), "Large data hashing failed");
}

#[tokio::test]
async fn test_blake3_avalanche_effect() {
    // Small change in input should produce completely different hash
    let data1_b64 = base64::engine::general_purpose::STANDARD.encode(b"test data");
    let data2_b64 = base64::engine::general_purpose::STANDARD.encode(b"test datA"); // Changed last char

    let hash1 = handle_blake3_hash(Some(&json!({"data": data1_b64})))
        .await
        .unwrap();
    let hash2 = handle_blake3_hash(Some(&json!({"data": data2_b64})))
        .await
        .unwrap();

    assert_ne!(
        hash1["hash"], hash2["hash"],
        "Hashes should be completely different (avalanche effect)"
    );
}

#[tokio::test]
async fn test_hmac_sha256_empty_data() {
    let key_b64 = base64::engine::general_purpose::STANDARD.encode(b"key");
    let data_b64 = base64::engine::general_purpose::STANDARD.encode(b"");

    let params = json!({
        "key": key_b64,
        "data": data_b64
    });

    let result = handle_hmac_sha256(Some(&params)).await;
    assert!(result.is_ok(), "Empty data HMAC failed");

    let mac = result.unwrap();
    assert_eq!(
        base64::engine::general_purpose::STANDARD
            .decode(mac["mac"].as_str().unwrap())
            .unwrap()
            .len(),
        32
    );
}

#[tokio::test]
async fn test_hmac_sha256_determinism() {
    let key_b64 = base64::engine::general_purpose::STANDARD.encode(b"secret");
    let data_b64 = base64::engine::general_purpose::STANDARD.encode(b"message");

    let params = json!({
        "key": key_b64,
        "data": data_b64
    });

    let result1 = handle_hmac_sha256(Some(&params)).await.unwrap();
    let result2 = handle_hmac_sha256(Some(&params)).await.unwrap();

    assert_eq!(result1["mac"], result2["mac"], "HMAC must be deterministic");
}

#[tokio::test]
async fn test_hmac_sha256_different_keys() {
    let data_b64 = base64::engine::general_purpose::STANDARD.encode(b"data");

    let mac1 = handle_hmac_sha256(Some(&json!({
        "key": base64::engine::general_purpose::STANDARD.encode(b"key1"),
        "data": data_b64
    })))
    .await
    .unwrap();

    let mac2 = handle_hmac_sha256(Some(&json!({
        "key": base64::engine::general_purpose::STANDARD.encode(b"key2"),
        "data": data_b64
    })))
    .await
    .unwrap();

    assert_ne!(
        mac1["mac"], mac2["mac"],
        "Different keys must produce different MACs"
    );
}
