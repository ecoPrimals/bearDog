//! Error path tests for generic crypto API endpoints
//!
//! Tests error handling for encrypt/decrypt operations

use axum::{
    body::Body,
    http::{Request, StatusCode},
};
use base64::{engine::general_purpose::STANDARD, Engine};
use beardog_api::{
    create_router,
    endpoints::{EncryptRequest, GenericDecryptRequest},
};
use beardog_core::BearDogCore;
use std::sync::Arc;
use tower::ServiceExt;

/// Helper to create test router
fn create_test_router() -> axum::Router {
    let core = Arc::new(BearDogCore::with_default_config().expect("Failed to create BearDogCore"));
    create_router(core)
}

// ============================================================================
// Encryption Error Tests
// ============================================================================

#[tokio::test]
async fn test_encrypt_invalid_base64() {
    let app = create_test_router();

    let request_body = EncryptRequest {
        plaintext: "not!!!valid!!!base64".to_string(),
        key_id: None,
        algorithm: "auto".to_string(),
    };

    let request = Request::builder()
        .method("POST")
        .uri("/api/v1/encrypt")
        .header("content-type", "application/json")
        .body(Body::from(serde_json::to_string(&request_body).unwrap()))
        .unwrap();

    let response = app.oneshot(request).await.unwrap();
    assert_eq!(response.status(), StatusCode::BAD_REQUEST);
}

#[tokio::test]
async fn test_encrypt_empty_plaintext() {
    let app = create_test_router();

    let request_body = EncryptRequest {
        plaintext: STANDARD.encode(""), // Empty data
        key_id: None,
        algorithm: "auto".to_string(),
    };

    let request = Request::builder()
        .method("POST")
        .uri("/api/v1/encrypt")
        .header("content-type", "application/json")
        .body(Body::from(serde_json::to_string(&request_body).unwrap()))
        .unwrap();

    let response = app.oneshot(request).await.unwrap();
    // Should either succeed (encrypting empty data) or return appropriate status
    assert!(response.status().is_success() || response.status() == StatusCode::BAD_REQUEST);
}

#[tokio::test]
async fn test_encrypt_large_data() {
    let app = create_test_router();

    let large_data = vec![0u8; 10 * 1024 * 1024]; // 10 MB

    let request_body = EncryptRequest {
        plaintext: STANDARD.encode(&large_data),
        key_id: None,
        algorithm: "auto".to_string(),
    };

    let request = Request::builder()
        .method("POST")
        .uri("/api/v1/encrypt")
        .header("content-type", "application/json")
        .body(Body::from(serde_json::to_string(&request_body).unwrap()))
        .unwrap();

    let response = app.oneshot(request).await.unwrap();
    // Should handle large data or return appropriate error
    assert!(
        response.status().is_success()
            || response.status() == StatusCode::PAYLOAD_TOO_LARGE
            || response.status() == StatusCode::INTERNAL_SERVER_ERROR
    );
}

#[tokio::test]
async fn test_encrypt_with_aes_algorithm() {
    let app = create_test_router();

    let request_body = EncryptRequest {
        plaintext: STANDARD.encode(b"test data"),
        key_id: None,
        algorithm: "aes-256-gcm".to_string(),
    };

    let request = Request::builder()
        .method("POST")
        .uri("/api/v1/encrypt")
        .header("content-type", "application/json")
        .body(Body::from(serde_json::to_string(&request_body).unwrap()))
        .unwrap();

    let response = app.oneshot(request).await.unwrap();
    assert_eq!(response.status(), StatusCode::OK);
}

#[tokio::test]
async fn test_encrypt_with_chacha_algorithm() {
    let app = create_test_router();

    let request_body = EncryptRequest {
        plaintext: STANDARD.encode(b"test data"),
        key_id: None,
        algorithm: "chacha20-poly1305".to_string(),
    };

    let request = Request::builder()
        .method("POST")
        .uri("/api/v1/encrypt")
        .header("content-type", "application/json")
        .body(Body::from(serde_json::to_string(&request_body).unwrap()))
        .unwrap();

    let response = app.oneshot(request).await.unwrap();
    assert_eq!(response.status(), StatusCode::OK);
}

#[tokio::test]
async fn test_encrypt_with_auto_algorithm() {
    let app = create_test_router();

    let request_body = EncryptRequest {
        plaintext: STANDARD.encode(b"test data"),
        key_id: None,
        algorithm: "auto".to_string(),
    };

    let request = Request::builder()
        .method("POST")
        .uri("/api/v1/encrypt")
        .header("content-type", "application/json")
        .body(Body::from(serde_json::to_string(&request_body).unwrap()))
        .unwrap();

    let response = app.oneshot(request).await.unwrap();
    assert_eq!(response.status(), StatusCode::OK);
}

#[tokio::test]
async fn test_encrypt_with_invalid_algorithm() {
    let app = create_test_router();

    let request_body = EncryptRequest {
        plaintext: STANDARD.encode(b"test data"),
        key_id: None,
        algorithm: "invalid-algo".to_string(),
    };

    let request = Request::builder()
        .method("POST")
        .uri("/api/v1/encrypt")
        .header("content-type", "application/json")
        .body(Body::from(serde_json::to_string(&request_body).unwrap()))
        .unwrap();

    let response = app.oneshot(request).await.unwrap();
    // Should fall back to default algorithm
    assert_eq!(response.status(), StatusCode::OK);
}

// ============================================================================
// Decryption Error Tests
// ============================================================================

#[tokio::test]
async fn test_decrypt_invalid_ciphertext_base64() {
    let app = create_test_router();

    let request_body = GenericDecryptRequest {
        ciphertext: "invalid!!!base64".to_string(),
        nonce: STANDARD.encode(b"123456789012"), // Valid nonce
        tag: STANDARD.encode(b"1234567890123456"), // Valid tag
        algorithm: "aes-256-gcm".to_string(),
        key_id: "test-key".to_string(),
    };

    let request = Request::builder()
        .method("POST")
        .uri("/api/v1/decrypt")
        .header("content-type", "application/json")
        .body(Body::from(serde_json::to_string(&request_body).unwrap()))
        .unwrap();

    let response = app.oneshot(request).await.unwrap();
    assert_eq!(response.status(), StatusCode::BAD_REQUEST);
}

#[tokio::test]
async fn test_decrypt_invalid_nonce_base64() {
    let app = create_test_router();

    let request_body = GenericDecryptRequest {
        ciphertext: STANDARD.encode(b"some ciphertext"),
        nonce: "invalid!!!base64".to_string(),
        tag: STANDARD.encode(b"1234567890123456"), // Valid tag
        algorithm: "aes-256-gcm".to_string(),
        key_id: "test-key".to_string(),
    };

    let request = Request::builder()
        .method("POST")
        .uri("/api/v1/decrypt")
        .header("content-type", "application/json")
        .body(Body::from(serde_json::to_string(&request_body).unwrap()))
        .unwrap();

    let response = app.oneshot(request).await.unwrap();
    assert_eq!(response.status(), StatusCode::BAD_REQUEST);
}

#[tokio::test]
async fn test_decrypt_with_nonexistent_key() {
    let app = create_test_router();

    let request_body = GenericDecryptRequest {
        ciphertext: STANDARD.encode(b"some ciphertext"),
        nonce: STANDARD.encode(b"123456789012"),
        tag: STANDARD.encode(b"1234567890123456"),
        algorithm: "aes-256-gcm".to_string(),
        key_id: "nonexistent-key-id".to_string(),
    };

    let request = Request::builder()
        .method("POST")
        .uri("/api/v1/decrypt")
        .header("content-type", "application/json")
        .body(Body::from(serde_json::to_string(&request_body).unwrap()))
        .unwrap();

    let response = app.oneshot(request).await.unwrap();
    // Should either succeed (deterministic key derivation) or fail with appropriate error
    assert!(
        response.status().is_success() || response.status() == StatusCode::INTERNAL_SERVER_ERROR
    );
}

#[tokio::test]
async fn test_decrypt_wrong_algorithm() {
    let app = create_test_router();

    // First encrypt with AES
    let encrypt_request = EncryptRequest {
        plaintext: STANDARD.encode(b"test data"),
        key_id: Some("decrypt-test-key".to_string()),
        algorithm: "aes-256-gcm".to_string(),
    };

    let request = Request::builder()
        .method("POST")
        .uri("/api/v1/encrypt")
        .header("content-type", "application/json")
        .body(Body::from(serde_json::to_string(&encrypt_request).unwrap()))
        .unwrap();

    let encrypt_response = app.clone().oneshot(request).await.unwrap();
    assert_eq!(encrypt_response.status(), StatusCode::OK);

    let encrypt_body = axum::body::to_bytes(encrypt_response.into_body(), usize::MAX)
        .await
        .unwrap();
    let encrypt_result: beardog_api::ApiResponse<beardog_api::endpoints::EncryptResponse> =
        serde_json::from_slice(&encrypt_body).unwrap();
    let encrypted_data = encrypt_result.data.unwrap();

    // Try to decrypt with wrong algorithm
    let decrypt_request = GenericDecryptRequest {
        ciphertext: encrypted_data.ciphertext,
        nonce: encrypted_data.nonce,
        tag: encrypted_data.tag,
        algorithm: "chacha20-poly1305".to_string(), // Wrong algorithm!
        key_id: encrypted_data.key_id,
    };

    let request = Request::builder()
        .method("POST")
        .uri("/api/v1/decrypt")
        .header("content-type", "application/json")
        .body(Body::from(serde_json::to_string(&decrypt_request).unwrap()))
        .unwrap();

    let response = app.oneshot(request).await.unwrap();
    // Should fail due to algorithm mismatch
    assert!(response.status().is_server_error() || !response.status().is_success());
}

// ============================================================================
// Round-trip Tests
// ============================================================================

#[tokio::test]
async fn test_encrypt_decrypt_roundtrip() {
    let app = create_test_router();

    let original_data = b"confidential data";

    // Encrypt
    let encrypt_request = EncryptRequest {
        plaintext: STANDARD.encode(original_data),
        key_id: Some("roundtrip-key".to_string()),
        algorithm: "aes-256-gcm".to_string(),
    };

    let request = Request::builder()
        .method("POST")
        .uri("/api/v1/encrypt")
        .header("content-type", "application/json")
        .body(Body::from(serde_json::to_string(&encrypt_request).unwrap()))
        .unwrap();

    let encrypt_response = app.clone().oneshot(request).await.unwrap();
    assert_eq!(encrypt_response.status(), StatusCode::OK);

    let encrypt_body = axum::body::to_bytes(encrypt_response.into_body(), usize::MAX)
        .await
        .unwrap();
    let encrypt_result: beardog_api::ApiResponse<beardog_api::endpoints::EncryptResponse> =
        serde_json::from_slice(&encrypt_body).unwrap();
    let encrypted_data = encrypt_result.data.unwrap();

    // Decrypt
    let decrypt_request = GenericDecryptRequest {
        ciphertext: encrypted_data.ciphertext,
        nonce: encrypted_data.nonce,
        tag: encrypted_data.tag,
        algorithm: encrypted_data.algorithm,
        key_id: encrypted_data.key_id,
    };

    let request = Request::builder()
        .method("POST")
        .uri("/api/v1/decrypt")
        .header("content-type", "application/json")
        .body(Body::from(serde_json::to_string(&decrypt_request).unwrap()))
        .unwrap();

    let decrypt_response = app.oneshot(request).await.unwrap();
    assert_eq!(decrypt_response.status(), StatusCode::OK);

    let decrypt_body = axum::body::to_bytes(decrypt_response.into_body(), usize::MAX)
        .await
        .unwrap();
    let decrypt_result: beardog_api::ApiResponse<beardog_api::endpoints::GenericDecryptResponse> =
        serde_json::from_slice(&decrypt_body).unwrap();
    let decrypted_data = decrypt_result.data.unwrap();

    // Verify
    let recovered_plaintext = STANDARD.decode(&decrypted_data.plaintext).unwrap();
    assert_eq!(recovered_plaintext, original_data);
    assert!(decrypted_data.verified);
}

#[tokio::test]
async fn test_encrypt_decrypt_multiple_messages() {
    let app = create_test_router();

    let messages = [
        b"message 1".to_vec(),
        b"message 2".to_vec(),
        b"message 3".to_vec(),
    ];

    for (i, msg) in messages.iter().enumerate() {
        let key_id = format!("multi-msg-key-{}", i);

        // Encrypt
        let encrypt_request = EncryptRequest {
            plaintext: STANDARD.encode(msg),
            key_id: Some(key_id.clone()),
            algorithm: "auto".to_string(),
        };

        let request = Request::builder()
            .method("POST")
            .uri("/api/v1/encrypt")
            .header("content-type", "application/json")
            .body(Body::from(serde_json::to_string(&encrypt_request).unwrap()))
            .unwrap();

        let response = app.clone().oneshot(request).await.unwrap();
        assert_eq!(response.status(), StatusCode::OK);

        let body = axum::body::to_bytes(response.into_body(), usize::MAX)
            .await
            .unwrap();
        let encrypt_result: beardog_api::ApiResponse<beardog_api::endpoints::EncryptResponse> =
            serde_json::from_slice(&body).unwrap();

        assert!(encrypt_result.success);
        assert_eq!(encrypt_result.data.as_ref().unwrap().key_id, key_id);
    }
}

// ============================================================================
// Concurrent Operations Tests
// ============================================================================

#[tokio::test]
async fn test_concurrent_encryption() {
    let app = create_test_router();

    let mut handles = vec![];

    for i in 0..10 {
        let app_clone = app.clone();
        let handle = tokio::spawn(async move {
            let encrypt_request = EncryptRequest {
                plaintext: STANDARD.encode(format!("concurrent message {}", i).as_bytes()),
                key_id: Some(format!("concurrent-key-{}", i)),
                algorithm: "auto".to_string(),
            };

            let request = Request::builder()
                .method("POST")
                .uri("/api/v1/encrypt")
                .header("content-type", "application/json")
                .body(Body::from(serde_json::to_string(&encrypt_request).unwrap()))
                .unwrap();

            let response = app_clone.oneshot(request).await.unwrap();
            response.status()
        });

        handles.push(handle);
    }

    let results = futures::future::join_all(handles).await;

    for result in results {
        let status = result.unwrap();
        assert_eq!(
            status,
            StatusCode::OK,
            "Concurrent encryption should succeed"
        );
    }
}
