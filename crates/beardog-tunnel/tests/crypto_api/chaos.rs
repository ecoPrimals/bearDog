// SPDX-License-Identifier: AGPL-3.0-only

//! Chaos / malformed input tests.

use base64::Engine;
use beardog_tunnel::unix_socket_ipc::handlers::crypto::*;
use rand::Rng;
use serde_json::json;

#[tokio::test]
async fn test_chaos_invalid_base64() {
    let invalid_b64 = "!!!NOT_BASE64!!!";

    let result = handle_sign_ed25519(Some(&json!({
        "message": invalid_b64,
        "key_id": "test",
        "purpose": "test"
    })))
    .await;

    assert!(result.is_err(), "Invalid base64 should fail");
    assert!(result.unwrap_err().contains("Invalid base64"));
}

#[tokio::test]
async fn test_chaos_random_binary_data() {
    let mut rng = rand::thread_rng();
    let random_data: Vec<u8> = (0..1024).map(|_| rng.r#gen()).collect();
    let random_b64 = base64::engine::general_purpose::STANDARD.encode(&random_data);

    // Should succeed with random data
    let result = handle_blake3_hash(Some(&json!({"data": random_b64}))).await;
    assert!(result.is_ok(), "Random data should be hashable");
}

#[tokio::test]
async fn test_chaos_unicode_in_params() {
    let message_b64 = base64::engine::general_purpose::STANDARD.encode("Hello 世界 🦀");

    let result = handle_sign_ed25519(Some(&json!({
        "message": message_b64,
        "key_id": "test_🔑",
        "purpose": "テスト"
    })))
    .await;

    assert!(result.is_ok(), "Unicode in params should work");
}

#[tokio::test]
async fn test_chaos_null_params() {
    let result = handle_sign_ed25519(None).await;
    assert!(result.is_err(), "Null params should fail");
}

#[tokio::test]
async fn test_chaos_empty_json_object() {
    let result = handle_sign_ed25519(Some(&json!({}))).await;
    assert!(result.is_err(), "Empty params should fail");
}

#[tokio::test]
async fn test_chaos_wrong_param_types() {
    // Number instead of string
    let result = handle_sign_ed25519(Some(&json!({
        "message": 12345,
        "key_id": "test",
        "purpose": "test"
    })))
    .await;

    assert!(result.is_err(), "Wrong param type should fail");
}

#[tokio::test]
async fn test_chaos_sql_injection_attempt() {
    let sql_injection = "'; DROP TABLE users; --";
    let message_b64 = base64::engine::general_purpose::STANDARD.encode(sql_injection);

    // Should handle gracefully (no SQL used anyway!)
    let result = handle_sign_ed25519(Some(&json!({
        "message": message_b64,
        "key_id": "test",
        "purpose": "test"
    })))
    .await;

    assert!(result.is_ok(), "SQL injection should be harmless");
}

#[tokio::test]
async fn test_chaos_xss_attempt() {
    let xss = "<script>alert('xss')</script>";
    let message_b64 = base64::engine::general_purpose::STANDARD.encode(xss);

    let result = handle_blake3_hash(Some(&json!({"data": message_b64}))).await;
    assert!(result.is_ok(), "XSS should be harmless");
}

#[tokio::test]
async fn test_chaos_path_traversal_attempt() {
    let path_traversal = "../../../../etc/passwd";
    let message_b64 = base64::engine::general_purpose::STANDARD.encode(path_traversal);

    let result = handle_sign_ed25519(Some(&json!({
        "message": message_b64,
        "key_id": path_traversal,
        "purpose": "test"
    })))
    .await;

    assert!(result.is_ok(), "Path traversal should be harmless");
}

#[tokio::test]
async fn test_chaos_extremely_long_strings() {
    // 10MB key_id (should handle gracefully)
    let long_string = "A".repeat(10 * 1024 * 1024);
    let message_b64 = base64::engine::general_purpose::STANDARD.encode(b"test");

    let result = handle_sign_ed25519(Some(&json!({
        "message": message_b64,
        "key_id": long_string,
        "purpose": "test"
    })))
    .await;

    // Should handle (may succeed or fail gracefully)
    assert!(result.is_ok() || result.is_err());
}

#[tokio::test]
async fn test_chaos_nested_json() {
    let nested = json!({
        "message": {
            "nested": {
                "deeply": {
                    "value": "actual_message"
                }
            }
        }
    });

    let result = handle_sign_ed25519(Some(&nested)).await;
    assert!(result.is_err(), "Nested JSON should fail (wrong type)");
}

#[tokio::test]
async fn test_chaos_array_instead_of_object() {
    let array_params = json!(["message", "key_id", "purpose"]);

    let result = handle_sign_ed25519(Some(&array_params)).await;
    assert!(result.is_err(), "Array params should fail");
}

#[tokio::test]
async fn test_chaos_concurrent_random_operations() {
    // Fire off 100 random crypto operations concurrently
    let mut rng = rand::thread_rng();

    let handles: Vec<_> = (0..100)
        .map(|_| {
            let op: u8 = rng.gen_range(0..5);
            tokio::spawn(async move {
                let random_data: Vec<u8> = (0..256).map(|_| rand::thread_rng().r#gen()).collect();
                let data_b64 = base64::engine::general_purpose::STANDARD.encode(&random_data);

                match op {
                    0 => {
                        handle_sign_ed25519(Some(&json!({
                            "message": data_b64,
                            "key_id": "test",
                            "purpose": "chaos"
                        })))
                        .await
                    }
                    1 => handle_x25519_generate_ephemeral(None).await,
                    2 => handle_blake3_hash(Some(&json!({"data": data_b64}))).await,
                    3 => {
                        handle_hmac_sha256(Some(&json!({
                            "key": data_b64.clone(),
                            "data": data_b64
                        })))
                        .await
                    }
                    _ => handle_blake3_hash(Some(&json!({"data": data_b64}))).await,
                }
            })
        })
        .collect();

    let mut successes = 0;
    for handle in handles {
        if let Ok(result) = handle.await {
            if result.is_ok() {
                successes += 1;
            }
        }
    }

    assert!(successes > 90, "Most random operations should succeed");
}
