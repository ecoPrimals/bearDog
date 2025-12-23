//! Integration tests for key management API endpoints
//!
//! Tests the key management API for Songbird integration.

use axum::{
    body::Body,
    http::{Request, StatusCode},
};
use beardog_api::{
    create_router,
    endpoints::{
        DeleteKeyRequest, DeleteKeyResponse, GenerateKeyRequest, GenerateKeyResponse,
        GetKeyInfoRequest, GetKeyInfoResponse,
    },
    ApiResponse,
};
use beardog_core::BearDogCore;
use std::sync::Arc;
use tower::ServiceExt;

/// Helper to create test router
fn create_test_router() -> axum::Router {
    let core = Arc::new(BearDogCore::with_default_config().expect("Failed to create BearDogCore"));
    create_router(core)
}

#[tokio::test]
async fn test_generate_key_aes_gcm() {
    let app = create_test_router();

    let request_body = GenerateKeyRequest {
        algorithm: "aes-256-gcm".to_string(),
        key_id: Some("test-key-1".to_string()),
        metadata: std::collections::HashMap::new(),
    };

    let request = Request::builder()
        .method("POST")
        .uri("/api/v1/keys/generate")
        .header("content-type", "application/json")
        .body(Body::from(serde_json::to_string(&request_body).unwrap()))
        .unwrap();

    let response = app.oneshot(request).await.unwrap();
    assert_eq!(response.status(), StatusCode::OK);

    let body = axum::body::to_bytes(response.into_body(), usize::MAX)
        .await
        .unwrap();
    let api_response: ApiResponse<GenerateKeyResponse> = serde_json::from_slice(&body).unwrap();

    assert!(api_response.success);
    assert_eq!(api_response.data.as_ref().unwrap().key_id, "test-key-1");
    assert_eq!(api_response.data.as_ref().unwrap().algorithm, "aes-256-gcm");
}

#[tokio::test]
async fn test_generate_key_chacha20() {
    let app = create_test_router();

    let request_body = GenerateKeyRequest {
        algorithm: "chacha20-poly1305".to_string(),
        key_id: None, // Auto-generate key ID
        metadata: std::collections::HashMap::new(),
    };

    let request = Request::builder()
        .method("POST")
        .uri("/api/v1/keys/generate")
        .header("content-type", "application/json")
        .body(Body::from(serde_json::to_string(&request_body).unwrap()))
        .unwrap();

    let response = app.oneshot(request).await.unwrap();
    assert_eq!(response.status(), StatusCode::OK);

    let body = axum::body::to_bytes(response.into_body(), usize::MAX)
        .await
        .unwrap();
    let api_response: ApiResponse<GenerateKeyResponse> = serde_json::from_slice(&body).unwrap();

    assert!(api_response.success);
    assert!(api_response
        .data
        .as_ref()
        .unwrap()
        .key_id
        .starts_with("key-"));
    assert_eq!(
        api_response.data.as_ref().unwrap().algorithm,
        "chacha20-poly1305"
    );
}

#[tokio::test]
async fn test_generate_key_invalid_algorithm() {
    let app = create_test_router();

    let request_body = GenerateKeyRequest {
        algorithm: "invalid-algo".to_string(),
        key_id: Some("test-key".to_string()),
        metadata: std::collections::HashMap::new(),
    };

    let request = Request::builder()
        .method("POST")
        .uri("/api/v1/keys/generate")
        .header("content-type", "application/json")
        .body(Body::from(serde_json::to_string(&request_body).unwrap()))
        .unwrap();

    let response = app.oneshot(request).await.unwrap();
    assert_eq!(response.status(), StatusCode::BAD_REQUEST);
}

#[tokio::test]
async fn test_get_key_info() {
    let app = create_test_router();

    let request_body = GetKeyInfoRequest {
        key_id: "test-key-1".to_string(),
    };

    let request = Request::builder()
        .method("POST")
        .uri("/api/v1/keys/info")
        .header("content-type", "application/json")
        .body(Body::from(serde_json::to_string(&request_body).unwrap()))
        .unwrap();

    let response = app.oneshot(request).await.unwrap();
    assert_eq!(response.status(), StatusCode::OK);

    let body = axum::body::to_bytes(response.into_body(), usize::MAX)
        .await
        .unwrap();
    let api_response: ApiResponse<GetKeyInfoResponse> = serde_json::from_slice(&body).unwrap();

    assert!(api_response.success);
    assert_eq!(api_response.data.as_ref().unwrap().key_id, "test-key-1");
    assert!(api_response.data.as_ref().unwrap().active);
}

#[tokio::test]
async fn test_delete_key() {
    let app = create_test_router();

    let request_body = DeleteKeyRequest {
        key_id: "test-key-1".to_string(),
    };

    let request = Request::builder()
        .method("POST")
        .uri("/api/v1/keys/delete")
        .header("content-type", "application/json")
        .body(Body::from(serde_json::to_string(&request_body).unwrap()))
        .unwrap();

    let response = app.oneshot(request).await.unwrap();
    assert_eq!(response.status(), StatusCode::OK);

    let body = axum::body::to_bytes(response.into_body(), usize::MAX)
        .await
        .unwrap();
    let api_response: ApiResponse<DeleteKeyResponse> = serde_json::from_slice(&body).unwrap();

    assert!(api_response.success);
    assert_eq!(api_response.data.as_ref().unwrap().key_id, "test-key-1");
    assert!(api_response.data.as_ref().unwrap().success);
}

#[tokio::test]
async fn test_key_lifecycle() {
    let app = create_test_router();

    // 1. Generate a key
    let generate_request = GenerateKeyRequest {
        algorithm: "aes-256-gcm".to_string(),
        key_id: Some("lifecycle-key".to_string()),
        metadata: {
            let mut meta = std::collections::HashMap::new();
            meta.insert("purpose".to_string(), "testing".to_string());
            meta
        },
    };

    let request = Request::builder()
        .method("POST")
        .uri("/api/v1/keys/generate")
        .header("content-type", "application/json")
        .body(Body::from(
            serde_json::to_string(&generate_request).unwrap(),
        ))
        .unwrap();

    let response = app.clone().oneshot(request).await.unwrap();
    assert_eq!(response.status(), StatusCode::OK);

    // 2. Get key info
    let info_request = GetKeyInfoRequest {
        key_id: "lifecycle-key".to_string(),
    };

    let request = Request::builder()
        .method("POST")
        .uri("/api/v1/keys/info")
        .header("content-type", "application/json")
        .body(Body::from(serde_json::to_string(&info_request).unwrap()))
        .unwrap();

    let response = app.clone().oneshot(request).await.unwrap();
    assert_eq!(response.status(), StatusCode::OK);

    // 3. Delete the key
    let delete_request = DeleteKeyRequest {
        key_id: "lifecycle-key".to_string(),
    };

    let request = Request::builder()
        .method("POST")
        .uri("/api/v1/keys/delete")
        .header("content-type", "application/json")
        .body(Body::from(serde_json::to_string(&delete_request).unwrap()))
        .unwrap();

    let response = app.oneshot(request).await.unwrap();
    assert_eq!(response.status(), StatusCode::OK);
}

#[tokio::test]
async fn test_multiple_key_generation() {
    let app = create_test_router();

    for i in 0..5 {
        let request_body = GenerateKeyRequest {
            algorithm: "aes-256-gcm".to_string(),
            key_id: Some(format!("multi-key-{}", i)),
            metadata: std::collections::HashMap::new(),
        };

        let request = Request::builder()
            .method("POST")
            .uri("/api/v1/keys/generate")
            .header("content-type", "application/json")
            .body(Body::from(serde_json::to_string(&request_body).unwrap()))
            .unwrap();

        let response = app.clone().oneshot(request).await.unwrap();
        assert_eq!(response.status(), StatusCode::OK);
    }
}
