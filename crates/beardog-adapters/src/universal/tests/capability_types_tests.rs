// Capability Types Tests
//
// Testing capability type definitions and conversions

use crate::universal::capability_types::*;

#[test]
fn test_capability_request_creation() {
    let request = CapabilityRequest {
        required_capability: CapabilityType::Security,
        payload: serde_json::json!({"test": "data"}),
        metadata: std::collections::HashMap::new(),
    };

    assert_eq!(request.required_capability, CapabilityType::Security);
    assert!(!request.payload.is_null());
}

#[test]
fn test_capability_type_equality() {
    let security1 = CapabilityType::Security;
    let security2 = CapabilityType::Security;
    let compute = CapabilityType::Compute;

    assert_eq!(security1, security2);
    assert_ne!(security1, compute);
}

#[test]
fn test_capability_type_variants() {
    // Verify all capability types can be created
    let types = vec![
        CapabilityType::Security,
        CapabilityType::Compute,
        CapabilityType::Storage,
        CapabilityType::Networking,
        CapabilityType::Monitoring,
    ];

    assert_eq!(types.len(), 5);
}

#[test]
fn test_capability_response_success() {
    let response = CapabilityResponse {
        success: true,
        result: serde_json::json!({"status": "ok"}),
        metadata: std::collections::HashMap::new(),
    };

    assert!(response.success);
    assert!(!response.result.is_null());
}

#[test]
fn test_capability_response_failure() {
    let mut metadata = std::collections::HashMap::new();
    metadata.insert("error".to_string(), "capability not found".to_string());

    let response = CapabilityResponse {
        success: false,
        result: serde_json::json!(null),
        metadata,
    };

    assert!(!response.success);
    assert!(response.metadata.contains_key("error"));
}

#[test]
fn test_capability_metadata() {
    let mut metadata = std::collections::HashMap::new();
    metadata.insert("priority".to_string(), "high".to_string());
    metadata.insert("timeout_ms".to_string(), "5000".to_string());

    assert_eq!(metadata.len(), 2);
    assert_eq!(metadata.get("priority"), Some(&"high".to_string()));
}

#[test]
fn test_capability_request_with_empty_metadata() {
    let request = CapabilityRequest {
        required_capability: CapabilityType::Compute,
        payload: serde_json::json!({}),
        metadata: std::collections::HashMap::new(),
    };

    assert!(request.metadata.is_empty());
}

#[test]
fn test_capability_request_with_complex_payload() {
    let payload = serde_json::json!({
        "operation": "process",
        "data": [1, 2, 3, 4, 5],
        "options": {
            "parallel": true,
            "threads": 4
        }
    });

    let request = CapabilityRequest {
        required_capability: CapabilityType::Compute,
        payload,
        metadata: std::collections::HashMap::new(),
    };

    assert!(request.payload.is_object());
    assert!(request.payload["data"].is_array());
}

#[test]
fn test_capability_response_with_metadata() {
    let mut metadata = std::collections::HashMap::new();
    metadata.insert("execution_time_ms".to_string(), "150".to_string());
    metadata.insert("provider".to_string(), "universal".to_string());

    let response = CapabilityResponse {
        success: true,
        result: serde_json::json!({"result": "processed"}),
        metadata,
    };

    assert!(response.success);
    assert_eq!(response.metadata.len(), 2);
}

