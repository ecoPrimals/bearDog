//! Comprehensive Unit Tests for BearDog API
//!
//! Created: October 27, 2025
//! Purpose: Expand test coverage for beardog-api core functionality
//!
//! Tests cover:
//! - ApiResponse generic wrapper
//! - HealthResponse and StatusResponse
//! - API state management
//! - Router creation
//! - Endpoint behavior
//! - Serialization/deserialization
//! - Error handling

use super::*;
use serde_json;

// ============================================================================
// ApiResponse<T> Tests
// ============================================================================

#[test]
fn test_api_response_success_creation() {
    let response = ApiResponse::success("test_data".to_string());

    assert!(response.success);
    assert_eq!(response.data, Some("test_data".to_string()));
    assert!(response.error.is_none());
}

#[test]
fn test_api_response_error_creation() {
    let response: ApiResponse<String> = ApiResponse::error("test error".to_string());

    assert!(!response.success);
    assert!(response.data.is_none());
    assert_eq!(response.error, Some("test error".to_string()));
}

#[test]
fn test_api_response_timestamp_set() {
    let before = chrono::Utc::now();
    let response = ApiResponse::success(42);
    let after = chrono::Utc::now();

    assert!(response.timestamp >= before);
    assert!(response.timestamp <= after);
}

#[test]
fn test_api_response_with_string() {
    let response = ApiResponse::success("hello".to_string());

    assert!(response.success);
    assert_eq!(response.data, Some("hello".to_string()));
}

#[test]
fn test_api_response_with_number() {
    let response = ApiResponse::success(12345);

    assert!(response.success);
    assert_eq!(response.data, Some(12345));
}

#[test]
fn test_api_response_with_struct() {
    #[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
    struct TestData {
        field: String,
    }

    let data = TestData {
        field: "value".to_string(),
    };
    let response = ApiResponse::success(data.clone());

    assert!(response.success);
    assert_eq!(response.data, Some(data));
}

#[test]
fn test_api_response_clone() {
    let response1 = ApiResponse::success(100);
    let response2 = response1.clone();

    assert_eq!(response1.success, response2.success);
    assert_eq!(response1.data, response2.data);
    assert_eq!(response1.error, response2.error);
}

#[test]
fn test_api_response_debug_format() {
    let response = ApiResponse::success(42);
    let debug_str = format!("{:?}", response);

    assert!(debug_str.contains("ApiResponse"));
    assert!(debug_str.contains("success"));
}

#[test]
fn test_api_response_success_serialization() {
    let response = ApiResponse::success("test".to_string());
    let serialized = serde_json::to_string(&response).expect("Serialization should succeed");
    let deserialized: ApiResponse<String> =
        serde_json::from_str(&serialized).expect("Deserialization should succeed");

    assert_eq!(response.success, deserialized.success);
    assert_eq!(response.data, deserialized.data);
    assert_eq!(response.error, deserialized.error);
}

#[test]
fn test_api_response_error_serialization() {
    let response: ApiResponse<String> = ApiResponse::error("error message".to_string());
    let serialized = serde_json::to_string(&response).expect("Serialization should succeed");
    let deserialized: ApiResponse<String> =
        serde_json::from_str(&serialized).expect("Deserialization should succeed");

    assert_eq!(response.success, deserialized.success);
    assert_eq!(response.error, deserialized.error);
}

#[test]
fn test_api_response_empty_error_message() {
    let response: ApiResponse<String> = ApiResponse::error("".to_string());

    assert!(!response.success);
    assert_eq!(response.error, Some("".to_string()));
}

#[test]
fn test_api_response_long_error_message() {
    let long_error = "a".repeat(1000);
    let response: ApiResponse<String> = ApiResponse::error(long_error.clone());

    assert!(!response.success);
    assert_eq!(response.error, Some(long_error));
}

#[test]
fn test_api_response_with_option() {
    let response = ApiResponse::success(Some("nested_option".to_string()));

    assert!(response.success);
    assert_eq!(response.data, Some(Some("nested_option".to_string())));
}

#[test]
fn test_api_response_with_vec() {
    let data = vec![1, 2, 3, 4, 5];
    let response = ApiResponse::success(data.clone());

    assert!(response.success);
    assert_eq!(response.data, Some(data));
}

// ============================================================================
// HealthResponse Tests
// ============================================================================

#[test]
fn test_health_response_creation() {
    let response = HealthResponse {
        status: "healthy".to_string(),
        version: "1.0.0".to_string(),
    };

    assert_eq!(response.status, "healthy");
    assert_eq!(response.version, "1.0.0");
}

#[test]
fn test_health_response_clone() {
    let response1 = HealthResponse {
        status: "healthy".to_string(),
        version: "1.0.0".to_string(),
    };
    let response2 = response1.clone();

    assert_eq!(response1.status, response2.status);
    assert_eq!(response1.version, response2.version);
}

#[test]
fn test_health_response_debug_format() {
    let response = HealthResponse {
        status: "healthy".to_string(),
        version: "1.0.0".to_string(),
    };
    let debug_str = format!("{:?}", response);

    assert!(debug_str.contains("HealthResponse"));
    assert!(debug_str.contains("healthy"));
}

#[test]
fn test_health_response_serialization() {
    let response = HealthResponse {
        status: "degraded".to_string(),
        version: "2.0.0".to_string(),
    };

    let serialized = serde_json::to_string(&response).expect("Serialization should succeed");
    let deserialized: HealthResponse =
        serde_json::from_str(&serialized).expect("Deserialization should succeed");

    assert_eq!(response.status, deserialized.status);
    assert_eq!(response.version, deserialized.version);
}

#[test]
fn test_health_response_different_statuses() {
    let statuses = vec!["healthy", "degraded", "unhealthy", "starting", "stopping"];

    for status in statuses {
        let response = HealthResponse {
            status: status.to_string(),
            version: "1.0.0".to_string(),
        };
        assert_eq!(response.status, status);
    }
}

#[test]
fn test_health_response_empty_version() {
    let response = HealthResponse {
        status: "healthy".to_string(),
        version: "".to_string(),
    };

    assert!(response.version.is_empty());
}

// ============================================================================
// StatusResponse Tests
// ============================================================================

#[test]
fn test_status_response_creation() {
    let response = StatusResponse {
        status: "operational".to_string(),
        connections: 42,
        memory_usage: 1024 * 1024,
    };

    assert_eq!(response.status, "operational");
    assert_eq!(response.connections, 42);
    assert_eq!(response.memory_usage, 1024 * 1024);
}

#[test]
fn test_status_response_zero_connections() {
    let response = StatusResponse {
        status: "idle".to_string(),
        connections: 0,
        memory_usage: 1024,
    };

    assert_eq!(response.connections, 0);
}

#[test]
fn test_status_response_high_connections() {
    let response = StatusResponse {
        status: "busy".to_string(),
        connections: 10000,
        memory_usage: 1024 * 1024 * 1024,
    };

    assert_eq!(response.connections, 10000);
}

#[test]
fn test_status_response_clone() {
    let response1 = StatusResponse {
        status: "operational".to_string(),
        connections: 100,
        memory_usage: 2048,
    };
    let response2 = response1.clone();

    assert_eq!(response1.status, response2.status);
    assert_eq!(response1.connections, response2.connections);
    assert_eq!(response1.memory_usage, response2.memory_usage);
}

#[test]
fn test_status_response_debug_format() {
    let response = StatusResponse {
        status: "operational".to_string(),
        connections: 5,
        memory_usage: 4096,
    };
    let debug_str = format!("{:?}", response);

    assert!(debug_str.contains("StatusResponse"));
    assert!(debug_str.contains("operational"));
}

#[test]
fn test_status_response_serialization() {
    let response = StatusResponse {
        status: "maintenance".to_string(),
        connections: 10,
        memory_usage: 8192,
    };

    let serialized = serde_json::to_string(&response).expect("Serialization should succeed");
    let deserialized: StatusResponse =
        serde_json::from_str(&serialized).expect("Deserialization should succeed");

    assert_eq!(response.status, deserialized.status);
    assert_eq!(response.connections, deserialized.connections);
    assert_eq!(response.memory_usage, deserialized.memory_usage);
}

#[test]
fn test_status_response_different_statuses() {
    let statuses = vec!["operational", "degraded", "maintenance", "offline"];

    for status in statuses {
        let response = StatusResponse {
            status: status.to_string(),
            connections: 1,
            memory_usage: 1024,
        };
        assert_eq!(response.status, status);
    }
}

// ============================================================================
// ApiState Tests
// ============================================================================

#[test]
fn test_api_state_creation() {
    let config = beardog_types::canonical::config::unified::UnifiedBearDogConfig::default();
    let core = Arc::new(BearDogCore::new(config));
    let state = ApiState { core: core.clone() };

    assert!(Arc::strong_count(&core) >= 2); // core + state
}

#[test]
fn test_api_state_clone() {
    let config = beardog_types::canonical::config::unified::UnifiedBearDogConfig::default();
    let core = Arc::new(BearDogCore::new(config));
    let state1 = ApiState { core: core.clone() };
    let state2 = state1.clone();

    // Both states should share the same Arc
    assert!(Arc::ptr_eq(&state1.core, &state2.core));
}

#[test]
fn test_api_state_debug_format() {
    let config = beardog_types::canonical::config::unified::UnifiedBearDogConfig::default();
    let core = Arc::new(BearDogCore::new(config));
    let state = ApiState { core };

    let debug_str = format!("{:?}", state);
    assert!(debug_str.contains("ApiState"));
}

// ============================================================================
// Router Tests
// ============================================================================

#[test]
fn test_create_router() {
    let config = beardog_types::canonical::config::unified::UnifiedBearDogConfig::default();
    let core = Arc::new(BearDogCore::new(config));
    let _router = create_router(core);

    // Router creation should not panic
}

#[test]
fn test_create_router_multiple_times() {
    let config = beardog_types::canonical::config::unified::UnifiedBearDogConfig::default();
    let core = Arc::new(BearDogCore::new(config));

    for _ in 0..5 {
        let _router = create_router(core.clone());
    }
    // Should be able to create multiple routers
}

// ============================================================================
// Utility Function Tests
// ============================================================================

#[test]
fn test_get_memory_usage_returns_value() {
    let memory = get_memory_usage();

    // Memory usage should be non-zero (process ID * 1024)
    assert!(memory > 0);
}

#[test]
fn test_get_memory_usage_consistent() {
    let memory1 = get_memory_usage();
    let memory2 = get_memory_usage();

    // Should return same value in quick succession (same process ID)
    assert_eq!(memory1, memory2);
}

// ============================================================================
// Endpoint Integration Tests
// ============================================================================

#[tokio::test]
async fn test_health_endpoint_success_response() {
    use axum::{
        body::Body,
        http::{Method, Request},
    };
    use tower::ServiceExt;

    let config = beardog_types::canonical::config::unified::UnifiedBearDogConfig::default();
    let core = Arc::new(BearDogCore::new(config));
    let app = create_router(core);

    let request = Request::builder()
        .method(Method::GET)
        .uri("/health")
        .body(Body::empty())
        .expect("Request build should succeed");

    let response = app.oneshot(request).await.expect("Request should succeed");

    assert_eq!(response.status(), StatusCode::OK);
}

#[tokio::test]
async fn test_status_endpoint_success_response() {
    use axum::{
        body::Body,
        http::{Method, Request},
    };
    use tower::ServiceExt;

    let config = beardog_types::canonical::config::unified::UnifiedBearDogConfig::default();
    let core = Arc::new(BearDogCore::new(config));
    let app = create_router(core);

    let request = Request::builder()
        .method(Method::GET)
        .uri("/status")
        .body(Body::empty())
        .expect("Request build should succeed");

    let response = app.oneshot(request).await.expect("Request should succeed");

    assert_eq!(response.status(), StatusCode::OK);
}

#[tokio::test]
async fn test_health_endpoint_response_body() {
    use axum::{
        body::{to_bytes, Body},
        http::{Method, Request},
    };
    use tower::ServiceExt;

    let config = beardog_types::canonical::config::unified::UnifiedBearDogConfig::default();
    let core = Arc::new(BearDogCore::new(config));
    let app = create_router(core);

    let request = Request::builder()
        .method(Method::GET)
        .uri("/health")
        .body(Body::empty())
        .expect("Request build should succeed");

    let response = app.oneshot(request).await.expect("Request should succeed");
    let body_bytes = to_bytes(response.into_body(), usize::MAX)
        .await
        .expect("Reading body should succeed");

    let body_str = String::from_utf8(body_bytes.to_vec()).expect("Body should be valid UTF-8");

    // Should contain success field
    assert!(body_str.contains("success"));
    assert!(body_str.contains("healthy"));
}

#[tokio::test]
async fn test_status_endpoint_response_body() {
    use axum::{
        body::{to_bytes, Body},
        http::{Method, Request},
    };
    use tower::ServiceExt;

    let config = beardog_types::canonical::config::unified::UnifiedBearDogConfig::default();
    let core = Arc::new(BearDogCore::new(config));
    let app = create_router(core);

    let request = Request::builder()
        .method(Method::GET)
        .uri("/status")
        .body(Body::empty())
        .expect("Request build should succeed");

    let response = app.oneshot(request).await.expect("Request should succeed");
    let body_bytes = to_bytes(response.into_body(), usize::MAX)
        .await
        .expect("Reading body should succeed");

    let body_str = String::from_utf8(body_bytes.to_vec()).expect("Body should be valid UTF-8");

    // Should contain success and operational fields
    assert!(body_str.contains("success"));
    assert!(body_str.contains("operational"));
    assert!(body_str.contains("connections"));
    assert!(body_str.contains("memory_usage"));
}

#[tokio::test]
async fn test_nonexistent_endpoint_returns_404() {
    use axum::{
        body::Body,
        http::{Method, Request},
    };
    use tower::ServiceExt;

    let config = beardog_types::canonical::config::unified::UnifiedBearDogConfig::default();
    let core = Arc::new(BearDogCore::new(config));
    let app = create_router(core);

    let request = Request::builder()
        .method(Method::GET)
        .uri("/nonexistent")
        .body(Body::empty())
        .expect("Request build should succeed");

    let response = app.oneshot(request).await.expect("Request should succeed");

    assert_eq!(response.status(), StatusCode::NOT_FOUND);
}

#[tokio::test]
async fn test_health_endpoint_multiple_requests() {
    use axum::{
        body::Body,
        http::{Method, Request},
    };
    use tower::ServiceExt;

    let config = beardog_types::canonical::config::unified::UnifiedBearDogConfig::default();
    let core = Arc::new(BearDogCore::new(config));

    for _ in 0..10 {
        let app = create_router(core.clone());
        let request = Request::builder()
            .method(Method::GET)
            .uri("/health")
            .body(Body::empty())
            .expect("Request build should succeed");

        let response = app.oneshot(request).await.expect("Request should succeed");
        assert_eq!(response.status(), StatusCode::OK);
    }
}

#[tokio::test]
async fn test_status_endpoint_multiple_requests() {
    use axum::{
        body::Body,
        http::{Method, Request},
    };
    use tower::ServiceExt;

    let config = beardog_types::canonical::config::unified::UnifiedBearDogConfig::default();
    let core = Arc::new(BearDogCore::new(config));

    for _ in 0..10 {
        let app = create_router(core.clone());
        let request = Request::builder()
            .method(Method::GET)
            .uri("/status")
            .body(Body::empty())
            .expect("Request build should succeed");

        let response = app.oneshot(request).await.expect("Request should succeed");
        assert_eq!(response.status(), StatusCode::OK);
    }
}
