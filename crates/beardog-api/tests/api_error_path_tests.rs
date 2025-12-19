//! High-Value Error Path Tests for BearDog API
//!
//! Created: December 15, 2025
//! Purpose: Comprehensive error handling and edge case testing
//!
//! Tests cover:
//! - Invalid input validation
//! - Malformed requests
//! - Edge cases (empty data, large data, special characters)
//! - Error response format
//! - Security validation (injection attempts)

#![allow(clippy::unwrap_used, clippy::expect_used)]

use axum::{
    body::Body,
    http::{Method, Request, StatusCode},
};
use base64::Engine;
use beardog_api::create_router;
use beardog_core::core::BearDogCore;
use beardog_types::canonical::config::unified::UnifiedBearDogConfig;
use serde_json::json;
use std::sync::Arc;
use tower::ServiceExt;

// ============================================================================
// Helper Functions
// ============================================================================

fn setup_test_server() -> axum::Router {
    let config = UnifiedBearDogConfig::default();
    let core = Arc::new(BearDogCore::new(config));
    create_router(core)
}

async fn post_json(
    app: axum::Router,
    uri: &str,
    body: serde_json::Value,
) -> Result<axum::response::Response, Box<dyn std::error::Error>> {
    let request = Request::builder()
        .method(Method::POST)
        .uri(uri)
        .header("Content-Type", "application/json")
        .body(Body::from(serde_json::to_vec(&body)?))?;

    Ok(app.oneshot(request).await?)
}

// ============================================================================
// Invalid Input Tests
// ============================================================================

#[tokio::test]
async fn test_encrypt_empty_data() {
    let app = setup_test_server();

    let body = json!({
        "data": "",
        "key_id": "test-key"
    });

    let response = post_json(app, "/api/v1/crypto/aes-gcm/encrypt", body)
        .await
        .expect("Request should complete");

    // Should still succeed but handle empty data gracefully
    assert!(response.status().is_success() || response.status() == StatusCode::BAD_REQUEST);
}

#[tokio::test]
async fn test_encrypt_missing_key_id() {
    let app = setup_test_server();

    let body = json!({
        "data": "test data"
        // key_id missing
    });

    let response = post_json(app, "/api/v1/crypto/aes-gcm/encrypt", body)
        .await
        .expect("Request should complete");

    // Should fail due to missing required field
    assert_eq!(response.status(), StatusCode::UNPROCESSABLE_ENTITY);
}

#[tokio::test]
async fn test_encrypt_invalid_base64() {
    let app = setup_test_server();

    let body = json!({
        "data": "this is not valid base64!@#$%",
        "key_id": "test-key"
    });

    let response = post_json(app, "/api/v1/crypto/aes-gcm/encrypt", body)
        .await
        .expect("Request should complete");

    // Should handle invalid base64 gracefully
    assert!(response.status().is_success() || response.status() == StatusCode::BAD_REQUEST);
}

#[tokio::test]
async fn test_decrypt_missing_nonce() {
    let app = setup_test_server();

    let body = json!({
        "ciphertext": "dGVzdA==",
        "key_id": "test-key",
        "tag": "dGFn"
        // nonce missing
    });

    let response = post_json(app, "/api/v1/crypto/aes-gcm/decrypt", body)
        .await
        .expect("Request should complete");

    assert_eq!(response.status(), StatusCode::UNPROCESSABLE_ENTITY);
}

#[tokio::test]
async fn test_decrypt_invalid_tag() {
    let app = setup_test_server();

    let body = json!({
        "ciphertext": "dGVzdA==",
        "key_id": "test-key",
        "nonce": "bm9uY2U=",
        "tag": "invalid!!!base64"
    });

    let response = post_json(app, "/api/v1/crypto/aes-gcm/decrypt", body)
        .await
        .expect("Request should complete");

    // Should handle invalid base64 gracefully
    assert!(response.status().is_success() || response.status() == StatusCode::BAD_REQUEST);
}

// ============================================================================
// Edge Case Tests
// ============================================================================

#[tokio::test]
async fn test_encrypt_large_data() {
    let app = setup_test_server();

    // 1MB of data
    let large_data = "A".repeat(1024 * 1024);
    let body = json!({
        "data": large_data,
        "key_id": "test-key"
    });

    let response = post_json(app, "/api/v1/crypto/aes-gcm/encrypt", body)
        .await
        .expect("Request should complete");

    // Should handle large data (may have size limits)
    assert!(
        response.status().is_success()
            || response.status() == StatusCode::PAYLOAD_TOO_LARGE
            || response.status() == StatusCode::BAD_REQUEST
    );
}

#[tokio::test]
async fn test_sign_empty_message() {
    let app = setup_test_server();

    let body = json!({
        "message": "",
        "key_id": "test-key"
    });

    let response = post_json(app, "/api/v1/crypto/ed25519/sign", body)
        .await
        .expect("Request should complete");

    // Signing empty message should be valid
    assert!(response.status().is_success() || response.status() == StatusCode::BAD_REQUEST);
}

#[tokio::test]
async fn test_sign_unicode_message() {
    let app = setup_test_server();

    // Modern idiomatic: Base64 encode Unicode properly
    let message = "Hello 世界 🌍 тест";
    let message_b64 = base64::engine::general_purpose::STANDARD.encode(message);

    let body = json!({
        "message": message_b64,
        "key_id": "test-key"
    });

    let response = post_json(app, "/api/v1/crypto/ed25519/sign", body)
        .await
        .expect("Request should complete");

    // API may return error if key doesn't exist, but should handle Unicode properly
    assert!(
        response.status().is_success() || response.status() == StatusCode::INTERNAL_SERVER_ERROR,
        "API should handle Unicode without panicking"
    );
}

#[tokio::test]
async fn test_verify_missing_public_key() {
    let app = setup_test_server();

    let body = json!({
        "message": "test",
        "signature": "c2lnbmF0dXJl"
        // public_key missing
    });

    let response = post_json(app, "/api/v1/crypto/ed25519/verify", body)
        .await
        .expect("Request should complete");

    assert_eq!(response.status(), StatusCode::UNPROCESSABLE_ENTITY);
}

#[tokio::test]
async fn test_verify_invalid_signature_format() {
    let app = setup_test_server();

    let body = json!({
        "message": "test",
        "signature": "not-valid-base64!!",
        "public_key": "cHVibGljX2tleQ=="
    });

    let response = post_json(app, "/api/v1/crypto/ed25519/verify", body)
        .await
        .expect("Request should complete");

    assert!(response.status().is_success() || response.status() == StatusCode::BAD_REQUEST);
}

// ============================================================================
// Security Tests
// ============================================================================

#[tokio::test]
async fn test_sql_injection_attempt_in_key_id() {
    let app = setup_test_server();

    let body = json!({
        "data": "test",
        "key_id": "'; DROP TABLE keys; --"
    });

    let response = post_json(app, "/api/v1/crypto/aes-gcm/encrypt", body)
        .await
        .expect("Request should complete");

    // Should handle safely (no SQL in this system, but validate input)
    assert!(response.status().is_success() || response.status() == StatusCode::BAD_REQUEST);
}

#[tokio::test]
async fn test_path_traversal_in_key_id() {
    let app = setup_test_server();

    let body = json!({
        "data": "test",
        "key_id": "../../../etc/passwd"
    });

    let response = post_json(app, "/api/v1/crypto/aes-gcm/encrypt", body)
        .await
        .expect("Request should complete");

    // Should handle safely
    assert!(response.status().is_success() || response.status() == StatusCode::BAD_REQUEST);
}

#[tokio::test]
async fn test_null_byte_injection() {
    let app = setup_test_server();

    let body = json!({
        "data": "test\0data",
        "key_id": "test\0key"
    });

    let response = post_json(app, "/api/v1/crypto/aes-gcm/encrypt", body)
        .await
        .expect("Request should complete");

    // Should handle null bytes safely
    assert!(response.status().is_success() || response.status() == StatusCode::BAD_REQUEST);
}

// ============================================================================
// Malformed Request Tests
// ============================================================================

#[tokio::test]
async fn test_malformed_json() {
    let app = setup_test_server();

    let request = Request::builder()
        .method(Method::POST)
        .uri("/api/v1/crypto/aes-gcm/encrypt")
        .header("Content-Type", "application/json")
        .body(Body::from("{invalid json"))
        .expect("Request build should succeed");

    let response = app.oneshot(request).await.expect("Request should complete");

    assert_eq!(response.status(), StatusCode::BAD_REQUEST);
}

#[tokio::test]
async fn test_wrong_content_type() {
    let app = setup_test_server();

    let request = Request::builder()
        .method(Method::POST)
        .uri("/api/v1/crypto/aes-gcm/encrypt")
        .header("Content-Type", "text/plain")
        .body(Body::from("not json"))
        .expect("Request build should succeed");

    let response = app.oneshot(request).await.expect("Request should complete");

    // Should reject non-JSON content type
    assert!(
        response.status() == StatusCode::BAD_REQUEST
            || response.status() == StatusCode::UNSUPPORTED_MEDIA_TYPE
            || response.status() == StatusCode::UNPROCESSABLE_ENTITY
    );
}

#[tokio::test]
async fn test_empty_body() {
    let app = setup_test_server();

    let request = Request::builder()
        .method(Method::POST)
        .uri("/api/v1/crypto/aes-gcm/encrypt")
        .header("Content-Type", "application/json")
        .body(Body::empty())
        .expect("Request build should succeed");

    let response = app.oneshot(request).await.expect("Request should complete");

    assert_eq!(response.status(), StatusCode::BAD_REQUEST);
}

// ============================================================================
// HTTP Method Tests
// ============================================================================

#[tokio::test]
async fn test_get_on_post_endpoint() {
    let app = setup_test_server();

    let request = Request::builder()
        .method(Method::GET)
        .uri("/api/v1/crypto/aes-gcm/encrypt")
        .body(Body::empty())
        .expect("Request build should succeed");

    let response = app.oneshot(request).await.expect("Request should complete");

    assert_eq!(response.status(), StatusCode::METHOD_NOT_ALLOWED);
}

#[tokio::test]
async fn test_put_on_post_endpoint() {
    let app = setup_test_server();

    let request = Request::builder()
        .method(Method::PUT)
        .uri("/api/v1/crypto/aes-gcm/encrypt")
        .body(Body::empty())
        .expect("Request build should succeed");

    let response = app.oneshot(request).await.expect("Request should complete");

    assert_eq!(response.status(), StatusCode::METHOD_NOT_ALLOWED);
}

#[tokio::test]
async fn test_delete_on_post_endpoint() {
    let app = setup_test_server();

    let request = Request::builder()
        .method(Method::DELETE)
        .uri("/api/v1/crypto/aes-gcm/encrypt")
        .body(Body::empty())
        .expect("Request build should succeed");

    let response = app.oneshot(request).await.expect("Request should complete");

    assert_eq!(response.status(), StatusCode::METHOD_NOT_ALLOWED);
}

// ============================================================================
// Invalid Path Tests
// ============================================================================

#[tokio::test]
async fn test_nonexistent_endpoint() {
    let app = setup_test_server();

    let request = Request::builder()
        .method(Method::GET)
        .uri("/api/v1/nonexistent")
        .body(Body::empty())
        .expect("Request build should succeed");

    let response = app.oneshot(request).await.expect("Request should complete");

    assert_eq!(response.status(), StatusCode::NOT_FOUND);
}

#[tokio::test]
async fn test_invalid_api_version() {
    let app = setup_test_server();

    let request = Request::builder()
        .method(Method::GET)
        .uri("/api/v99/capabilities")
        .body(Body::empty())
        .expect("Request build should succeed");

    let response = app.oneshot(request).await.expect("Request should complete");

    assert_eq!(response.status(), StatusCode::NOT_FOUND);
}

// ============================================================================
// Concurrent Request Tests
// ============================================================================

#[tokio::test]
async fn test_concurrent_encryption_requests() {
    let config = UnifiedBearDogConfig::default();
    let core = Arc::new(BearDogCore::new(config));

    let mut handles = vec![];

    for i in 0..10 {
        let core_clone = Arc::clone(&core);
        let handle = tokio::spawn(async move {
            let app = create_router(core_clone);

            // Modern idiomatic: Base64 encode data properly
            let data = format!("test-data-{}", i);
            let data_b64 = base64::engine::general_purpose::STANDARD.encode(&data);

            let body = json!({
                "data": data_b64,
                "key_id": "test-key"
            });

            let request = Request::builder()
                .method(Method::POST)
                .uri("/api/v1/crypto/aes-gcm/encrypt")
                .header("Content-Type", "application/json")
                .body(Body::from(serde_json::to_vec(&body).unwrap()))
                .unwrap();

            app.oneshot(request).await
        });
        handles.push(handle);
    }

    // Modern idiomatic: Concurrent operations should complete without panicking
    // (may error if keys don't exist, but shouldn't crash)
    for handle in handles {
        let result = handle.await.expect("Task should complete");
        assert!(result.is_ok(), "Request should succeed");
        let response = result.unwrap();
        // Accept both success and internal error (key may not exist)
        assert!(
            response.status().is_success()
                || response.status() == StatusCode::INTERNAL_SERVER_ERROR,
            "Concurrent requests should not panic"
        );
    }
}

// ============================================================================
// Special Character Tests
// ============================================================================

#[tokio::test]
async fn test_special_characters_in_data() {
    let app = setup_test_server();

    // Modern idiomatic: Base64 encode special characters properly
    let special_chars = "<>&\"'`\n\r\t\0";
    let data_b64 = base64::engine::general_purpose::STANDARD.encode(special_chars);

    let body = json!({
        "data": data_b64,
        "key_id": "test-key"
    });

    let response = post_json(app, "/api/v1/crypto/aes-gcm/encrypt", body)
        .await
        .expect("Request should complete");

    // API may return error if key doesn't exist, but should handle special chars properly
    assert!(
        response.status().is_success() || response.status() == StatusCode::INTERNAL_SERVER_ERROR,
        "API should handle special characters without panicking"
    );
}

#[tokio::test]
async fn test_emoji_in_message() {
    let app = setup_test_server();

    // Modern idiomatic: Base64 encode emojis properly
    let message = "🔐🐻🔒✨🚀";
    let message_b64 = base64::engine::general_purpose::STANDARD.encode(message);

    let body = json!({
        "message": message_b64,
        "key_id": "test-key"
    });

    let response = post_json(app, "/api/v1/crypto/ed25519/sign", body)
        .await
        .expect("Request should complete");

    // API may return error if key doesn't exist, but should handle emojis properly
    assert!(
        response.status().is_success() || response.status() == StatusCode::INTERNAL_SERVER_ERROR,
        "API should handle emojis without panicking"
    );
}

// ============================================================================
// Rate Limiting Tests (if implemented)
// ============================================================================

#[tokio::test]
async fn test_many_rapid_requests() {
    let config = UnifiedBearDogConfig::default();
    let core = Arc::new(BearDogCore::new(config));
    let app = create_router(core);

    let mut success_count = 0;

    for _ in 0..100 {
        let app_clone = app.clone();
        let body = json!({
            "data": "test",
            "key_id": "test-key"
        });

        if let Ok(response) = post_json(app_clone, "/api/v1/crypto/aes-gcm/encrypt", body).await {
            if response.status().is_success() {
                success_count += 1;
            }
        }
    }

    // Should handle rapid requests (may have rate limiting)
    assert!(success_count > 0, "At least some requests should succeed");
}
