//! Integration tests for JSON-RPC 2.0 API
//!
//! Tests verify that:
//! 1. JSON-RPC 2.0 protocol is correctly implemented
//! 2. All crypto methods work via JSON-RPC
//! 3. Encrypt/decrypt round-trips via RPC
//! 4. Sign/verify round-trips via RPC
//! 5. Error handling matches JSON-RPC spec

use axum::{
    body::Body,
    http::{Request, StatusCode},
};
use base64::{engine::general_purpose::STANDARD, Engine};
use beardog_api::create_router;
use beardog_core::core::BearDogCore;
use beardog_types::canonical::config::unified::UnifiedBearDogConfig;
use serde_json::json;
use std::sync::Arc;
use tower::ServiceExt; // for oneshot

#[tokio::test]
async fn test_jsonrpc_encrypt_decrypt_roundtrip() {
    // Setup
    let config = UnifiedBearDogConfig::default();
    let core = Arc::new(BearDogCore::new(config));
    let app = create_router(core);

    // Original plaintext
    let plaintext = b"Hello via JSON-RPC!";
    let plaintext_b64 = STANDARD.encode(plaintext);

    // 1. Encrypt via JSON-RPC
    let encrypt_request = Request::builder()
        .method("POST")
        .uri("/rpc")
        .header("content-type", "application/json")
        .body(Body::from(
            serde_json::to_string(&json!({
                "jsonrpc": "2.0",
                "method": "beardog.encrypt",
                "params": {
                    "data": plaintext_b64,
                    "key_id": "test-key-jsonrpc",
                    "algorithm": "aes-256-gcm"
                },
                "id": 1
            }))
            .expect("Failed to serialize"),
        ))
        .expect("Failed to build request");

    let response = app
        .clone()
        .oneshot(encrypt_request)
        .await
        .expect("Failed to get response");

    assert_eq!(response.status(), StatusCode::OK);

    let body = axum::body::to_bytes(response.into_body(), usize::MAX)
        .await
        .expect("Failed to read response body");
    let rpc_response: serde_json::Value =
        serde_json::from_slice(&body).expect("Failed to parse JSON");

    // Verify JSON-RPC response structure
    assert_eq!(rpc_response["jsonrpc"], "2.0");
    assert_eq!(rpc_response["id"], 1);
    assert!(rpc_response["error"].is_null());
    assert!(!rpc_response["result"].is_null());

    let result = &rpc_response["result"];
    let ciphertext = result["ciphertext"].as_str().expect("Missing ciphertext");
    let nonce = result["nonce"].as_str().expect("Missing nonce");
    let tag = result["tag"].as_str().expect("Missing tag");

    // 2. Decrypt via JSON-RPC
    let decrypt_request = Request::builder()
        .method("POST")
        .uri("/rpc")
        .header("content-type", "application/json")
        .body(Body::from(
            serde_json::to_string(&json!({
                "jsonrpc": "2.0",
                "method": "beardog.decrypt",
                "params": {
                    "ciphertext": ciphertext,
                    "nonce": nonce,
                    "tag": tag,
                    "key_id": "test-key-jsonrpc",
                    "algorithm": "aes-256-gcm"
                },
                "id": 2
            }))
            .expect("Failed to serialize"),
        ))
        .expect("Failed to build request");

    let response = app
        .oneshot(decrypt_request)
        .await
        .expect("Failed to get response");

    assert_eq!(response.status(), StatusCode::OK);

    let body = axum::body::to_bytes(response.into_body(), usize::MAX)
        .await
        .expect("Failed to read response body");
    let rpc_response: serde_json::Value =
        serde_json::from_slice(&body).expect("Failed to parse JSON");

    // Verify response
    assert_eq!(rpc_response["jsonrpc"], "2.0");
    assert_eq!(rpc_response["id"], 2);
    assert!(rpc_response["error"].is_null());

    let result = &rpc_response["result"];
    let decrypted_b64 = result["plaintext"].as_str().expect("Missing plaintext");
    let decrypted = STANDARD.decode(decrypted_b64).expect("Invalid base64");

    // Verify roundtrip
    assert_eq!(decrypted, plaintext);
}

#[tokio::test]
async fn test_jsonrpc_sign_verify_roundtrip() {
    // Setup
    let config = UnifiedBearDogConfig::default();
    let core = Arc::new(BearDogCore::new(config));
    let app = create_router(core);

    // Original message
    let message = b"JSON-RPC signed message";
    let message_b64 = STANDARD.encode(message);

    // 1. Sign via JSON-RPC
    let sign_request = Request::builder()
        .method("POST")
        .uri("/rpc")
        .header("content-type", "application/json")
        .body(Body::from(
            serde_json::to_string(&json!({
                "jsonrpc": "2.0",
                "method": "beardog.sign",
                "params": {
                    "message": message_b64,
                    "key_id": "test-sign-key-rpc",
                    "algorithm": "ed25519"
                },
                "id": 1
            }))
            .expect("Failed to serialize"),
        ))
        .expect("Failed to build request");

    let response = app
        .clone()
        .oneshot(sign_request)
        .await
        .expect("Failed to get response");

    assert_eq!(response.status(), StatusCode::OK);

    let body = axum::body::to_bytes(response.into_body(), usize::MAX)
        .await
        .expect("Failed to read response body");
    let rpc_response: serde_json::Value =
        serde_json::from_slice(&body).expect("Failed to parse JSON");

    // Verify JSON-RPC response
    assert_eq!(rpc_response["jsonrpc"], "2.0");
    assert_eq!(rpc_response["id"], 1);
    assert!(rpc_response["error"].is_null());

    let result = &rpc_response["result"];
    let signature = result["signature"].as_str().expect("Missing signature");
    let algorithm = result["algorithm"].as_str().expect("Missing algorithm");

    assert_eq!(algorithm, "ed25519");

    // Get public key (derive same way as service using HKDF)
    use beardog_core::crypto_service::algorithms::asymmetric::generate_ed25519_from_seed;
    use beardog_core::crypto_service::algorithms::hashing::hkdf_sha256;

    // Match the crypto service's derive_signing_key logic
    let key_id = "test-sign-key-rpc";
    let salt = b"beardog-signing-key-derivation-v1";
    let info = format!("beardog:sign:{}", key_id);
    let derived =
        hkdf_sha256(key_id.as_bytes(), salt, info.as_bytes(), 32).expect("HKDF derivation failed");

    let mut key_seed = [0u8; 32];
    key_seed.copy_from_slice(&derived[..32]);

    let (_secret_key, public_key) =
        generate_ed25519_from_seed(&key_seed).expect("Ed25519 key generation failed");
    let public_key_b64 = STANDARD.encode(public_key);

    // 2. Verify via JSON-RPC
    let verify_request = Request::builder()
        .method("POST")
        .uri("/rpc")
        .header("content-type", "application/json")
        .body(Body::from(
            serde_json::to_string(&json!({
                "jsonrpc": "2.0",
                "method": "beardog.verify",
                "params": {
                    "message": message_b64,
                    "signature": signature,
                    "public_key": public_key_b64,
                    "algorithm": "ed25519"
                },
                "id": 2
            }))
            .expect("Failed to serialize"),
        ))
        .expect("Failed to build request");

    let response = app
        .oneshot(verify_request)
        .await
        .expect("Failed to get response");

    assert_eq!(response.status(), StatusCode::OK);

    let body = axum::body::to_bytes(response.into_body(), usize::MAX)
        .await
        .expect("Failed to read response body");
    let rpc_response: serde_json::Value =
        serde_json::from_slice(&body).expect("Failed to parse JSON");

    // Verify response
    assert_eq!(rpc_response["jsonrpc"], "2.0");
    assert_eq!(rpc_response["id"], 2);
    assert!(rpc_response["error"].is_null());

    let result = &rpc_response["result"];
    let valid = result["valid"].as_bool().expect("Missing valid field");

    // Should be valid
    assert!(valid);
}

#[tokio::test]
async fn test_jsonrpc_method_not_found() {
    // Setup
    let config = UnifiedBearDogConfig::default();
    let core = Arc::new(BearDogCore::new(config));
    let app = create_router(core);

    // Call non-existent method
    let request = Request::builder()
        .method("POST")
        .uri("/rpc")
        .header("content-type", "application/json")
        .body(Body::from(
            serde_json::to_string(&json!({
                "jsonrpc": "2.0",
                "method": "beardog.nonexistent",
                "params": {},
                "id": 1
            }))
            .expect("Failed to serialize"),
        ))
        .expect("Failed to build request");

    let response = app.oneshot(request).await.expect("Failed to get response");

    assert_eq!(response.status(), StatusCode::OK);

    let body = axum::body::to_bytes(response.into_body(), usize::MAX)
        .await
        .expect("Failed to read response body");
    let rpc_response: serde_json::Value =
        serde_json::from_slice(&body).expect("Failed to parse JSON");

    // Verify error response
    assert_eq!(rpc_response["jsonrpc"], "2.0");
    assert_eq!(rpc_response["id"], 1);
    assert!(rpc_response["result"].is_null());
    assert!(!rpc_response["error"].is_null());

    let error = &rpc_response["error"];
    assert_eq!(error["code"], -32601); // METHOD_NOT_FOUND
    assert!(error["message"].as_str().unwrap().contains("not found"));
}

#[tokio::test]
async fn test_jsonrpc_invalid_params() {
    // Setup
    let config = UnifiedBearDogConfig::default();
    let core = Arc::new(BearDogCore::new(config));
    let app = create_router(core);

    // Call with invalid parameters
    let request = Request::builder()
        .method("POST")
        .uri("/rpc")
        .header("content-type", "application/json")
        .body(Body::from(
            serde_json::to_string(&json!({
                "jsonrpc": "2.0",
                "method": "beardog.encrypt",
                "params": {
                    "invalid_field": "value"
                },
                "id": 1
            }))
            .expect("Failed to serialize"),
        ))
        .expect("Failed to build request");

    let response = app.oneshot(request).await.expect("Failed to get response");

    assert_eq!(response.status(), StatusCode::OK);

    let body = axum::body::to_bytes(response.into_body(), usize::MAX)
        .await
        .expect("Failed to read response body");
    let rpc_response: serde_json::Value =
        serde_json::from_slice(&body).expect("Failed to parse JSON");

    // Verify error response
    assert_eq!(rpc_response["jsonrpc"], "2.0");
    assert_eq!(rpc_response["id"], 1);
    assert!(rpc_response["result"].is_null());
    assert!(!rpc_response["error"].is_null());

    let error = &rpc_response["error"];
    assert_eq!(error["code"], -32602); // INVALID_PARAMS
}

#[tokio::test]
async fn test_jsonrpc_capabilities() {
    // Setup
    let config = UnifiedBearDogConfig::default();
    let core = Arc::new(BearDogCore::new(config));
    let app = create_router(core);

    // Call capabilities method
    let request = Request::builder()
        .method("POST")
        .uri("/rpc")
        .header("content-type", "application/json")
        .body(Body::from(
            serde_json::to_string(&json!({
                "jsonrpc": "2.0",
                "method": "beardog.capabilities",
                "params": {},
                "id": 1
            }))
            .expect("Failed to serialize"),
        ))
        .expect("Failed to build request");

    let response = app.oneshot(request).await.expect("Failed to get response");

    assert_eq!(response.status(), StatusCode::OK);

    let body = axum::body::to_bytes(response.into_body(), usize::MAX)
        .await
        .expect("Failed to read response body");
    let rpc_response: serde_json::Value =
        serde_json::from_slice(&body).expect("Failed to parse JSON");

    // Verify response
    assert_eq!(rpc_response["jsonrpc"], "2.0");
    assert_eq!(rpc_response["id"], 1);
    assert!(rpc_response["error"].is_null());

    let result = &rpc_response["result"];

    // Verify capabilities structure (ServiceCapabilities)
    // Fields: service_name, version, supported_algorithms, features, max_data_size
    assert!(!result.is_null());
    assert!(!result["service_name"].is_null());
    assert!(!result["version"].is_null());
    assert!(result["supported_algorithms"].is_array());
}

#[tokio::test]
async fn test_jsonrpc_health() {
    // Setup
    let config = UnifiedBearDogConfig::default();
    let core = Arc::new(BearDogCore::new(config));
    let app = create_router(core);

    // Call health method
    let request = Request::builder()
        .method("POST")
        .uri("/rpc")
        .header("content-type", "application/json")
        .body(Body::from(
            serde_json::to_string(&json!({
                "jsonrpc": "2.0",
                "method": "beardog.health",
                "params": {},
                "id": 1
            }))
            .expect("Failed to serialize"),
        ))
        .expect("Failed to build request");

    let response = app.oneshot(request).await.expect("Failed to get response");

    assert_eq!(response.status(), StatusCode::OK);

    let body = axum::body::to_bytes(response.into_body(), usize::MAX)
        .await
        .expect("Failed to read response body");
    let rpc_response: serde_json::Value =
        serde_json::from_slice(&body).expect("Failed to parse JSON");

    // Verify response
    assert_eq!(rpc_response["jsonrpc"], "2.0");
    assert_eq!(rpc_response["id"], 1);
    assert!(rpc_response["error"].is_null());

    let result = &rpc_response["result"];

    // Verify health structure
    assert!(result["healthy"].is_boolean());
    assert!(result["uptime_seconds"].is_number());
}
