//! Integration tests for protocol discovery endpoints

use axum::{
    body::Body,
    http::{Request, StatusCode},
};
use beardog_api::create_router;
use beardog_core::core::BearDogCore;
use beardog_types::canonical::config::unified::UnifiedBearDogConfig;
use std::sync::Arc;
use tower::ServiceExt;

#[tokio::test]
async fn test_get_all_protocols() {
    // Setup
    let config = UnifiedBearDogConfig::default();
    let core = Arc::new(BearDogCore::new(config));
    let app = create_router(core);

    // Request
    let request = Request::builder()
        .method("GET")
        .uri("/api/v1/protocols")
        .body(Body::empty())
        .expect("Failed to build request");

    let response = app
        .clone()
        .oneshot(request)
        .await
        .expect("Failed to get response");

    assert_eq!(response.status(), StatusCode::OK);

    let body = axum::body::to_bytes(response.into_body(), usize::MAX)
        .await
        .expect("Failed to read response body");
    let json: serde_json::Value = serde_json::from_slice(&body).expect("Failed to parse JSON");

    // Verify structure
    assert!(json["success"].as_bool().unwrap());
    let data = &json["data"];

    // Should have all three protocols
    assert!(data["http"].is_object());
    assert!(data["jsonrpc"].is_object());
    assert!(data["tarpc"].is_object());

    // Verify HTTP protocol info
    assert_eq!(data["http"]["protocol_type"], "http");
    assert_eq!(data["http"]["available"], true);
    assert!(data["http"]["supported_algorithms"].is_array());

    // Verify JSON-RPC protocol info
    assert_eq!(data["jsonrpc"]["protocol_type"], "jsonrpc");
    assert_eq!(data["jsonrpc"]["available"], true);

    // Verify tarpc protocol info
    assert_eq!(data["tarpc"]["protocol_type"], "tarpc");
    assert_eq!(data["tarpc"]["available"], true);
    assert_eq!(data["tarpc"]["characteristics"]["efficiency"], 10);
}

#[tokio::test]
async fn test_get_specific_protocol() {
    // Setup
    let config = UnifiedBearDogConfig::default();
    let core = Arc::new(BearDogCore::new(config));
    let app = create_router(core);

    // Request tarpc protocol specifically
    let request = Request::builder()
        .method("GET")
        .uri("/api/v1/protocols/tarpc")
        .body(Body::empty())
        .expect("Failed to build request");

    let response = app.oneshot(request).await.expect("Failed to get response");

    assert_eq!(response.status(), StatusCode::OK);

    let body = axum::body::to_bytes(response.into_body(), usize::MAX)
        .await
        .expect("Failed to read response body");
    let json: serde_json::Value = serde_json::from_slice(&body).expect("Failed to parse JSON");

    // Verify tarpc-specific info
    let data = &json["data"];
    assert_eq!(data["name"], "tarpc Binary RPC");
    assert_eq!(data["protocol_type"], "tarpc");
    assert_eq!(data["characteristics"]["efficiency"], 10);
    assert_eq!(
        data["characteristics"]["type_safety"],
        "Strong (Rust types)"
    );
}

#[tokio::test]
async fn test_protocol_not_found() {
    // Setup
    let config = UnifiedBearDogConfig::default();
    let core = Arc::new(BearDogCore::new(config));
    let app = create_router(core);

    // Request non-existent protocol
    let request = Request::builder()
        .method("GET")
        .uri("/api/v1/protocols/nonexistent")
        .body(Body::empty())
        .expect("Failed to build request");

    let response = app.oneshot(request).await.expect("Failed to get response");

    assert_eq!(response.status(), StatusCode::NOT_FOUND);
}

#[tokio::test]
async fn test_escalation_guide() {
    // Setup
    let config = UnifiedBearDogConfig::default();
    let core = Arc::new(BearDogCore::new(config));
    let app = create_router(core);

    // Request
    let request = Request::builder()
        .method("GET")
        .uri("/api/v1/protocols/escalation/guide")
        .body(Body::empty())
        .expect("Failed to build request");

    let response = app.oneshot(request).await.expect("Failed to get response");

    assert_eq!(response.status(), StatusCode::OK);

    let body = axum::body::to_bytes(response.into_body(), usize::MAX)
        .await
        .expect("Failed to read response body");
    let json: serde_json::Value = serde_json::from_slice(&body).expect("Failed to parse JSON");

    // Verify structure
    assert!(json["success"].as_bool().unwrap());
    let data = &json["data"];
    assert!(data.is_array());

    // Should have multiple escalation paths
    let guides = data.as_array().unwrap();
    assert!(guides.len() >= 3);

    // Verify one guide structure
    let first_guide = &guides[0];
    assert!(first_guide["from"].is_string());
    assert!(first_guide["to"].is_string());
    assert!(first_guide["steps"].is_array());
    assert!(first_guide["performance_gain"].is_string());
}

#[tokio::test]
async fn test_protocol_comparison() {
    // Setup
    let config = UnifiedBearDogConfig::default();
    let core = Arc::new(BearDogCore::new(config));
    let app = create_router(core);

    // Request
    let request = Request::builder()
        .method("GET")
        .uri("/api/v1/protocols/comparison")
        .body(Body::empty())
        .expect("Failed to build request");

    let response = app.oneshot(request).await.expect("Failed to get response");

    assert_eq!(response.status(), StatusCode::OK);

    let body = axum::body::to_bytes(response.into_body(), usize::MAX)
        .await
        .expect("Failed to read response body");
    let json: serde_json::Value = serde_json::from_slice(&body).expect("Failed to parse JSON");

    // Verify structure
    assert!(json["success"].as_bool().unwrap());
    let data = &json["data"];
    assert!(data.is_array());

    // Should have comparison rows
    let comparisons = data.as_array().unwrap();
    assert!(comparisons.len() >= 5);

    // Verify comparison structure
    let first_comparison = &comparisons[0];
    assert!(first_comparison["feature"].is_string());
    assert!(first_comparison["http"].is_string());
    assert!(first_comparison["jsonrpc"].is_string());
    assert!(first_comparison["tarpc"].is_string());
    assert!(first_comparison["best"].is_string());
}
