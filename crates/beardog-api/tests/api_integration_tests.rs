//! Integration tests for BearDog API endpoints
//!
//! These tests verify the API server's behavior, routing, and response formats.

use axum::{
    body::Body,
    http::{Method, Request, StatusCode},
};
use beardog_api::create_router;
use beardog_core::core::BearDogCore;
use beardog_types::canonical::config::unified::UnifiedBearDogConfig;
use serde_json::Value;
use std::sync::Arc;
use tower::ServiceExt;

/// Helper function to create a test BearDogCore instance
fn create_test_core() -> Arc<BearDogCore> {
    let config = UnifiedBearDogConfig::default();
    Arc::new(BearDogCore::new(config))
}

/// Helper function to parse JSON response body
async fn parse_json_body(body: axum::body::Body) -> Value {
    let bytes = axum::body::to_bytes(body, usize::MAX)
        .await
        .expect("Failed to read body bytes");
    serde_json::from_slice(&bytes).expect("Failed to parse JSON")
}

#[tokio::test]
async fn test_health_endpoint_returns_200() {
    let core = create_test_core();
    let app = create_router(core);

    let request = Request::builder()
        .method(Method::GET)
        .uri("/health")
        .body(Body::empty())
        .expect("Failed to build health request");

    let response = app
        .oneshot(request)
        .await
        .expect("Failed to get health response");

    assert_eq!(response.status(), StatusCode::OK);
}

#[tokio::test]
async fn test_health_endpoint_returns_valid_json() {
    let core = create_test_core();
    let app = create_router(core);

    let request = Request::builder()
        .method(Method::GET)
        .uri("/health")
        .body(Body::empty())
        .expect("Failed to build health request");

    let response = app
        .oneshot(request)
        .await
        .expect("Failed to get health response");

    let body = response.into_body();
    let json = parse_json_body(body).await;

    assert!(json.get("success").is_some());
    assert!(json.get("data").is_some());
    assert!(json.get("timestamp").is_some());
}

#[tokio::test]
async fn test_health_endpoint_success_true() {
    let core = create_test_core();
    let app = create_router(core);

    let request = Request::builder()
        .method(Method::GET)
        .uri("/health")
        .body(Body::empty())
        .expect("Failed to build health request");

    let response = app
        .oneshot(request)
        .await
        .expect("Failed to get health response");

    let body = response.into_body();
    let json = parse_json_body(body).await;

    assert_eq!(json["success"], true);
}

#[tokio::test]
async fn test_health_endpoint_contains_version() {
    let core = create_test_core();
    let app = create_router(core);

    let request = Request::builder()
        .method(Method::GET)
        .uri("/health")
        .body(Body::empty())
        .expect("Failed to build health request");

    let response = app
        .oneshot(request)
        .await
        .expect("Failed to get health response");

    let body = response.into_body();
    let json = parse_json_body(body).await;

    assert!(json["data"]["version"].is_string());
}

#[tokio::test]
async fn test_health_endpoint_status_healthy() {
    let core = create_test_core();
    let app = create_router(core);

    let request = Request::builder()
        .method(Method::GET)
        .uri("/health")
        .body(Body::empty())
        .expect("Failed to build health request");

    let response = app
        .oneshot(request)
        .await
        .expect("Failed to get health response");

    let body = response.into_body();
    let json = parse_json_body(body).await;

    assert_eq!(json["data"]["status"], "healthy");
}

#[tokio::test]
async fn test_status_endpoint_returns_200() {
    let core = create_test_core();
    let app = create_router(core);

    let request = Request::builder()
        .method(Method::GET)
        .uri("/status")
        .body(Body::empty())
        .expect("Failed to build status request");

    let response = app
        .oneshot(request)
        .await
        .expect("Failed to get status response");

    assert_eq!(response.status(), StatusCode::OK);
}

#[tokio::test]
async fn test_status_endpoint_returns_valid_json() {
    let core = create_test_core();
    let app = create_router(core);

    let request = Request::builder()
        .method(Method::GET)
        .uri("/status")
        .body(Body::empty())
        .expect("Failed to build status request");

    let response = app
        .oneshot(request)
        .await
        .expect("Failed to get status response");

    let body = response.into_body();
    let json = parse_json_body(body).await;

    assert!(json.get("success").is_some());
    assert!(json.get("data").is_some());
    assert!(json.get("timestamp").is_some());
}

#[tokio::test]
async fn test_status_endpoint_contains_connections() {
    let core = create_test_core();
    let app = create_router(core);

    let request = Request::builder()
        .method(Method::GET)
        .uri("/status")
        .body(Body::empty())
        .expect("Failed to build status request");

    let response = app
        .oneshot(request)
        .await
        .expect("Failed to get status response");

    let body = response.into_body();
    let json = parse_json_body(body).await;

    assert!(json["data"]["connections"].is_number());
}

#[tokio::test]
async fn test_status_endpoint_contains_memory_usage() {
    let core = create_test_core();
    let app = create_router(core);

    let request = Request::builder()
        .method(Method::GET)
        .uri("/status")
        .body(Body::empty())
        .expect("Failed to build status request");

    let response = app
        .oneshot(request)
        .await
        .expect("Failed to get status response");

    let body = response.into_body();
    let json = parse_json_body(body).await;

    assert!(json["data"]["memory_usage"].is_number());
}

#[tokio::test]
async fn test_status_endpoint_status_operational() {
    let core = create_test_core();
    let app = create_router(core);

    let request = Request::builder()
        .method(Method::GET)
        .uri("/status")
        .body(Body::empty())
        .expect("Failed to build status request");

    let response = app
        .oneshot(request)
        .await
        .expect("Failed to get status response");

    let body = response.into_body();
    let json = parse_json_body(body).await;

    assert_eq!(json["data"]["status"], "operational");
}

#[tokio::test]
async fn test_unknown_endpoint_returns_404() {
    let core = create_test_core();
    let app = create_router(core);

    let request = Request::builder()
        .method(Method::GET)
        .uri("/nonexistent")
        .body(Body::empty())
        .expect("Failed to build request");

    let response = app.oneshot(request).await.expect("Failed to get response");

    assert_eq!(response.status(), StatusCode::NOT_FOUND);
}

#[tokio::test]
async fn test_router_creation_succeeds() {
    let core = create_test_core();
    let _router = create_router(core);
    // If we got here, router creation succeeded
}

#[tokio::test]
async fn test_multiple_health_requests() {
    let core = create_test_core();

    for _ in 0..5 {
        let app = create_router(core.clone());
        let request = Request::builder()
            .method(Method::GET)
            .uri("/health")
            .body(Body::empty())
            .expect("Failed to build request");

        let response = app.oneshot(request).await.expect("Failed to get response");

        assert_eq!(response.status(), StatusCode::OK);
    }
}

#[tokio::test]
async fn test_health_and_status_endpoints_together() {
    let core = create_test_core();

    // Test health
    let app = create_router(core.clone());
    let health_request = Request::builder()
        .method(Method::GET)
        .uri("/health")
        .body(Body::empty())
        .expect("Failed to build health request");

    let health_response = app
        .oneshot(health_request)
        .await
        .expect("Failed to get health response");

    assert_eq!(health_response.status(), StatusCode::OK);

    // Test status
    let app = create_router(core);
    let status_request = Request::builder()
        .method(Method::GET)
        .uri("/status")
        .body(Body::empty())
        .expect("Failed to build status request");

    let status_response = app
        .oneshot(status_request)
        .await
        .expect("Failed to get status response");

    assert_eq!(status_response.status(), StatusCode::OK);
}
