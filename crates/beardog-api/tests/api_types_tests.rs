//! Unit tests for API types and data structures
//!
//! These tests verify the API's type system, serialization, and response formats.

use beardog_api::{ApiResponse, HealthResponse, StatusResponse};

#[test]
fn test_api_response_success_creation() {
    let data = "test data";
    let response = ApiResponse::success(data);

    assert!(response.success);
    assert_eq!(response.data, Some("test data"));
    assert_eq!(response.error, None);
}

#[test]
fn test_api_response_error_creation() {
    let response: ApiResponse<()> = ApiResponse::error("Test error".to_string());

    assert!(!response.success);
    assert_eq!(response.data, None);
    assert_eq!(response.error, Some("Test error".to_string()));
}

#[test]
fn test_api_response_timestamp_present() {
    let response = ApiResponse::success("test");

    // Timestamp should be recent (within last 5 seconds)
    let now = chrono::Utc::now();
    let diff = now - response.timestamp;
    assert!(diff.num_seconds() < 5);
}

#[test]
fn test_health_response_creation() {
    let health = HealthResponse {
        status: "healthy".to_string(),
        version: "1.0.0".to_string(),
    };

    assert_eq!(health.status, "healthy");
    assert_eq!(health.version, "1.0.0");
}

#[test]
fn test_health_response_serialization() {
    let health = HealthResponse {
        status: "healthy".to_string(),
        version: "1.0.0".to_string(),
    };

    let json = serde_json::to_string(&health).expect("Failed to serialize");
    assert!(json.contains("healthy"));
    assert!(json.contains("1.0.0"));
}

#[test]
fn test_health_response_deserialization() {
    let json = r#"{"status":"healthy","version":"1.0.0"}"#;
    let health: HealthResponse = serde_json::from_str(json).expect("Failed to deserialize");

    // TEST_CATEGORY: integration
    // TEST_DOMAIN: core
    // TEST_PRIORITY: normal
    assert_eq!(health.status, "healthy");
    assert_eq!(health.version, "1.0.0");
}

#[test]
fn test_status_response_creation() {
    let status = StatusResponse {
        // TEST_CATEGORY: integration
        // TEST_DOMAIN: core
        // TEST_PRIORITY: important
        status: "operational".to_string(),
        connections: 42,
        memory_usage: 1024,
    };

    assert_eq!(status.status, "operational");
    // TEST_CATEGORY: integration
    // TEST_DOMAIN: core
    // TEST_PRIORITY: normal
    assert_eq!(status.connections, 42);
    assert_eq!(status.memory_usage, 1024);
}

#[test]
fn test_status_response_serialization() {
    let status = StatusResponse {
        // TEST_CATEGORY: integration
        // TEST_DOMAIN: core
        // TEST_PRIORITY: normal
        status: "operational".to_string(),
        connections: 42,
        memory_usage: 1024,
    };

    let json = serde_json::to_string(&status).expect("Failed to serialize");
    assert!(json.contains("operational"));
    assert!(json.contains("42"));
    // TEST_CATEGORY: integration
    // TEST_DOMAIN: core
    // TEST_PRIORITY: normal
    assert!(json.contains("1024"));
}

#[test]
fn test_status_response_deserialization() {
    let json = r#"{"status":"operational","connections":42,"memory_usage":1024}"#;
    let status: StatusResponse = serde_json::from_str(json).expect("Failed to deserialize");

    assert_eq!(status.status, "operational");
    // TEST_CATEGORY: integration
    // TEST_DOMAIN: core
    // TEST_PRIORITY: normal
    assert_eq!(status.connections, 42);
    assert_eq!(status.memory_usage, 1024);
}

#[test]
fn test_api_response_with_health_data() {
    // TEST_CATEGORY: integration
    // TEST_DOMAIN: core
    // TEST_PRIORITY: normal
    let health = HealthResponse {
        status: "healthy".to_string(),
        version: "1.0.0".to_string(),
    };
    let response = ApiResponse::success(health);

    assert!(response.success);
    assert!(response.data.is_some());
    let data = response.data.unwrap();
    assert_eq!(data.status, "healthy");
    // TEST_CATEGORY: integration
    // TEST_DOMAIN: core
    // TEST_PRIORITY: normal
    assert_eq!(data.version, "1.0.0");
}

#[test]
fn test_api_response_with_status_data() {
    let status = StatusResponse {
        status: "operational".to_string(),
        connections: 10,
        memory_usage: 2048,
    };
    let response = ApiResponse::success(status);
    // TEST_CATEGORY: integration
    // TEST_DOMAIN: core
    // TEST_PRIORITY: normal

    assert!(response.success);
    assert!(response.data.is_some());
    let data = response.data.unwrap();
    assert_eq!(data.status, "operational");
    assert_eq!(data.connections, 10);
    assert_eq!(data.memory_usage, 2048);
    // TEST_CATEGORY: integration
    // TEST_DOMAIN: core
    // TEST_PRIORITY: normal
}

#[test]
fn test_api_response_full_serialization() {
    let health = HealthResponse {
        status: "healthy".to_string(),
        version: "1.0.0".to_string(),
    };
    let response = ApiResponse::success(health);

    let json = serde_json::to_string(&response).expect("Failed to serialize");
    assert!(json.contains("success"));
    // TEST_CATEGORY: integration
    // TEST_DOMAIN: core
    // TEST_PRIORITY: normal
    assert!(json.contains("healthy"));
    assert!(json.contains("1.0.0"));
    assert!(json.contains("timestamp"));
}

#[test]
fn test_api_response_error_serialization() {
    let response: ApiResponse<()> = ApiResponse::error("Test error message".to_string());

    let json = serde_json::to_string(&response).expect("Failed to serialize");
    assert!(json.contains("success"));
    assert!(json.contains("error"));
    assert!(json.contains("Test error message"));
}
// TEST_CATEGORY: integration
// TEST_DOMAIN: core
// TEST_PRIORITY: normal

#[test]
fn test_multiple_api_responses() {
    let responses = vec![
        ApiResponse::success("test1"),
        ApiResponse::success("test2"),
        ApiResponse::success("test3"),
    ];

    for response in responses {
        assert!(response.success);
        assert!(response.data.is_some());
        // TEST_CATEGORY: integration
        // TEST_DOMAIN: core
        // TEST_PRIORITY: important
    }
}

#[test]
fn test_api_response_clone() {
    let response = ApiResponse::success("test data");
    let cloned = response.clone();
    // TEST_CATEGORY: integration
    // TEST_DOMAIN: core
    // TEST_PRIORITY: normal

    assert_eq!(response.success, cloned.success);
    assert_eq!(response.data, cloned.data);
    assert_eq!(response.error, cloned.error);
    assert_eq!(response.timestamp, cloned.timestamp);
}

#[test]
fn test_health_response_clone() {
    let health = HealthResponse {
        status: "healthy".to_string(),
        // TEST_CATEGORY: integration
        // TEST_DOMAIN: core
        // TEST_PRIORITY: normal
        version: "1.0.0".to_string(),
    };
    let cloned = health.clone();

    assert_eq!(health.status, cloned.status);
    assert_eq!(health.version, cloned.version);
}

// TEST_CATEGORY: integration
// TEST_DOMAIN: core
// TEST_PRIORITY: normal
#[test]
fn test_status_response_clone() {
    let status = StatusResponse {
        status: "operational".to_string(),
        connections: 42,
        memory_usage: 1024,
    };
    let cloned = status.clone();

    // TEST_CATEGORY: integration
    // TEST_DOMAIN: core
    // TEST_PRIORITY: normal
    assert_eq!(status.status, cloned.status);
    assert_eq!(status.connections, cloned.connections);
    assert_eq!(status.memory_usage, cloned.memory_usage);
}

#[test]
fn test_api_response_with_large_error_message() {
    let large_error = "A".repeat(1000);
    let response: ApiResponse<()> = ApiResponse::error(large_error.clone());

    assert!(!response.success);
    // TEST_CATEGORY: integration
    // TEST_DOMAIN: core
    // TEST_PRIORITY: important
    assert_eq!(response.error, Some(large_error));
}

#[test]
fn test_status_response_with_zero_connections() {
    let status = StatusResponse {
        // TEST_CATEGORY: integration
        // TEST_DOMAIN: core
        // TEST_PRIORITY: normal
        status: "idle".to_string(),
        connections: 0,
        memory_usage: 512,
    };

    assert_eq!(status.connections, 0);
}

// TEST_CATEGORY: integration
// TEST_DOMAIN: core
// TEST_PRIORITY: normal
#[test]
fn test_status_response_with_high_memory() {
    let status = StatusResponse {
        status: "operational".to_string(),
        connections: 100,
        memory_usage: u64::MAX,
    };

    assert_eq!(status.memory_usage, u64::MAX);
}
