// SPDX-License-Identifier: AGPL-3.0-or-later

//! Comprehensive Unit Tests for Universal Adapter Core
//!
//! Created: October 27, 2025
//! Purpose: Expand test coverage for beardog-adapters core functionality
//!
//! Tests cover:
//! - Configuration management
//! - Request/Response handling  
//! - Caching behavior
//! - Retry logic
//! - Timeout handling
//! - Error scenarios
//! - Serialization/deserialization

use super::*;
use std::collections::HashMap;

// ============================================================================
// AdapterConfig Tests
// ============================================================================

#[test]
fn test_adapter_config_default() {
    let config = AdapterConfig::default();

    assert_eq!(config.timeout_seconds, 30);
    assert_eq!(config.retry_attempts, 3);
    assert!(config.enable_caching);
}

#[test]
fn test_adapter_config_custom() {
    let config = AdapterConfig {
        timeout_seconds: 60,
        retry_attempts: 5,
        enable_caching: false,
    };

    assert_eq!(config.timeout_seconds, 60);
    assert_eq!(config.retry_attempts, 5);
    assert!(!config.enable_caching);
}

#[test]
fn test_adapter_config_clone() {
    let config1 = AdapterConfig::default();
    let config2 = config1.clone();

    assert_eq!(config1.timeout_seconds, config2.timeout_seconds);
    assert_eq!(config1.retry_attempts, config2.retry_attempts);
    assert_eq!(config1.enable_caching, config2.enable_caching);
}

#[test]
fn test_adapter_config_serialization() {
    let config = AdapterConfig {
        timeout_seconds: 45,
        retry_attempts: 4,
        enable_caching: true,
    };

    let serialized = serde_json::to_string(&config).expect("Serialization should succeed");
    let deserialized: AdapterConfig =
        serde_json::from_str(&serialized).expect("Deserialization should succeed");

    assert_eq!(config.timeout_seconds, deserialized.timeout_seconds);
    assert_eq!(config.retry_attempts, deserialized.retry_attempts);
    assert_eq!(config.enable_caching, deserialized.enable_caching);
}

// ============================================================================
// CapabilityRequest Tests
// ============================================================================

#[test]
fn test_capability_request_creation() {
    let mut params = HashMap::new();
    params.insert("key1".to_string(), "value1".to_string());

    let request = CapabilityRequest {
        capability: "encryption".to_string(),
        operation: "encrypt".to_string(),
        parameters: params.clone(),
    };

    assert_eq!(request.capability, "encryption");
    assert_eq!(request.operation, "encrypt");
    assert_eq!(request.parameters.len(), 1);
    assert_eq!(request.parameters.get("key1"), Some(&"value1".to_string()));
}

#[test]
fn test_capability_request_empty_parameters() {
    let request = CapabilityRequest {
        capability: "test".to_string(),
        operation: "test_op".to_string(),
        parameters: HashMap::new(),
    };

    assert!(request.parameters.is_empty());
}

#[test]
fn test_capability_request_serialization() {
    let request = CapabilityRequest {
        capability: "signing".to_string(),
        operation: "sign".to_string(),
        parameters: HashMap::new(),
    };

    let serialized = serde_json::to_string(&request).expect("Serialization should succeed");
    let deserialized: CapabilityRequest =
        serde_json::from_str(&serialized).expect("Deserialization should succeed");

    assert_eq!(request.capability, deserialized.capability);
    assert_eq!(request.operation, deserialized.operation);
}

#[test]
fn test_capability_request_clone() {
    let request1 = CapabilityRequest {
        capability: "test".to_string(),
        operation: "op".to_string(),
        parameters: HashMap::new(),
    };

    let request2 = request1.clone();
    assert_eq!(request1.capability, request2.capability);
    assert_eq!(request1.operation, request2.operation);
}

// ============================================================================
// CapabilityResponse Tests
// ============================================================================

#[test]
fn test_capability_response_success() {
    let response = CapabilityResponse {
        success: true,
        data: Some(serde_json::json!({"result": "ok"})),
        error: None,
        metadata: HashMap::new(),
    };

    // TEST_CATEGORY: unit
    // TEST_DOMAIN: adapters
    // TEST_PRIORITY: normal
    assert!(response.success);
    assert!(response.data.is_some());
    assert!(response.error.is_none());
}

#[test]
// TEST_CATEGORY: unit
// TEST_DOMAIN: adapters
// TEST_PRIORITY: normal
fn test_capability_response_error() {
    let response = CapabilityResponse {
        success: false,
        data: None,
        error: Some("Test error".to_string()),
        metadata: HashMap::new(),
    };

    assert!(!response.success);
    assert!(response.data.is_none());
    // TEST_CATEGORY: unit
    // TEST_DOMAIN: adapters
    // TEST_PRIORITY: normal
    assert_eq!(response.error, Some("Test error".to_string()));
}

#[test]
fn test_capability_response_with_metadata() {
    let mut metadata = HashMap::new();
    metadata.insert("execution_time".to_string(), "100ms".to_string());
    // TEST_CATEGORY: unit
    // TEST_DOMAIN: adapters
    // TEST_PRIORITY: normal
    metadata.insert("cached".to_string(), "false".to_string());

    let response = CapabilityResponse {
        success: true,
        data: None,
        error: None,
        metadata: metadata.clone(),
    };

    assert_eq!(response.metadata.len(), 2);
    assert_eq!(
        response.metadata.get("execution_time"),
        Some(&"100ms".to_string())
    );
}

#[test]
fn test_capability_response_serialization() {
    // TEST_CATEGORY: unit
    // TEST_DOMAIN: adapters
    // TEST_PRIORITY: normal
    let response = CapabilityResponse {
        success: true,
        data: Some(serde_json::json!({"key": "value"})),
        error: None,
        metadata: HashMap::new(),
    };

    let serialized = serde_json::to_string(&response).expect("Serialization should succeed");
    let deserialized: CapabilityResponse =
        serde_json::from_str(&serialized).expect("Deserialization should succeed");

    assert_eq!(response.success, deserialized.success);
}

// TEST_CATEGORY: unit
// TEST_DOMAIN: adapters
// TEST_PRIORITY: normal
#[test]
fn test_capability_response_clone() {
    let response1 = CapabilityResponse {
        success: true,
        data: None,
        error: None,
        metadata: HashMap::new(),
    };
    // TEST_CATEGORY: unit
    // TEST_DOMAIN: adapters
    // TEST_PRIORITY: normal

    let response2 = response1.clone();
    assert_eq!(response1.success, response2.success);
}

// ============================================================================
// AIResponseMetadata Tests
// ============================================================================

#[test]
fn test_ai_response_metadata_default() {
    let metadata = AIResponseMetadata::default();

    // TEST_CATEGORY: unit
    // TEST_DOMAIN: adapters
    // TEST_PRIORITY: normal
    assert!(
        (metadata.confidence_score - 1.0).abs() < 1e-9,
        "expected confidence_score ≈ 1.0, got {}",
        metadata.confidence_score
    );
    assert_eq!(metadata.processing_time_ms, 0);
    assert_eq!(metadata.model_version, "v1.0.0");
}

#[test]
fn test_ai_response_metadata_custom() {
    let metadata = AIResponseMetadata {
        confidence_score: 0.95,
        processing_time_ms: 150,
        model_version: "v2.0.0".to_string(),
    };

    assert!(
        (metadata.confidence_score - 0.95).abs() < 1e-9,
        "expected confidence_score ≈ 0.95, got {}",
        metadata.confidence_score
    );
    // TEST_CATEGORY: unit
    // TEST_DOMAIN: adapters
    // TEST_PRIORITY: normal
    assert_eq!(metadata.processing_time_ms, 150);
    assert_eq!(metadata.model_version, "v2.0.0");
}

#[test]
fn test_ai_response_metadata_serialization() {
    let metadata = AIResponseMetadata {
        confidence_score: 0.88,
        processing_time_ms: 200,
        model_version: "v1.5.0".to_string(),
    };
    // TEST_CATEGORY: unit
    // TEST_DOMAIN: adapters
    // TEST_PRIORITY: important

    let serialized = serde_json::to_string(&metadata).expect("Serialization should succeed");
    let deserialized: AIResponseMetadata =
        serde_json::from_str(&serialized).expect("Deserialization should succeed");

    assert!(
        (metadata.confidence_score - deserialized.confidence_score).abs() < 1e-9,
        "confidence_score mismatch after round-trip"
    );
    assert_eq!(metadata.processing_time_ms, deserialized.processing_time_ms);
    assert_eq!(metadata.model_version, deserialized.model_version);
}

#[test]
// TEST_CATEGORY: unit
// TEST_DOMAIN: adapters
// TEST_PRIORITY: normal
fn test_ai_response_metadata_clone() {
    let metadata1 = AIResponseMetadata::default();
    let metadata2 = metadata1.clone();

    assert!(
        (metadata1.confidence_score - metadata2.confidence_score).abs() < 1e-9,
        "cloned confidence_score should match"
    );
    assert_eq!(metadata1.model_version, metadata2.model_version);
}

// ============================================================================
// AIIntegrationResponse Tests
// ============================================================================

#[test]
fn test_ai_integration_response_default() {
    let response = AIIntegrationResponse::default();

    assert_eq!(response.result, "success ");
    // TEST_CATEGORY: unit
    // TEST_DOMAIN: adapters
    // TEST_PRIORITY: normal
    assert!(
        (response.ai_metadata.confidence_score - 1.0).abs() < 1e-9,
        "expected confidence_score ≈ 1.0, got {}",
        response.ai_metadata.confidence_score
    );
    assert!(response.suggested_actions.is_empty());
}

#[test]
fn test_ai_integration_response_with_actions() {
    let response = AIIntegrationResponse {
        result: "analysis_complete".to_string(),
        ai_metadata: AIResponseMetadata::default(),
        suggested_actions: vec![
            "action1".to_string(),
            "action2".to_string(),
            "action3".to_string(),
            // TEST_CATEGORY: unit
            // TEST_DOMAIN: adapters
            // TEST_PRIORITY: normal
        ],
    };

    assert_eq!(response.suggested_actions.len(), 3);
    assert_eq!(response.suggested_actions[0], "action1");
}

#[test]
fn test_ai_integration_response_serialization() {
    let response = AIIntegrationResponse {
        result: "test_result".to_string(),
        ai_metadata: AIResponseMetadata::default(),
        suggested_actions: vec!["action".to_string()],
    };
    // TEST_CATEGORY: unit
    // TEST_DOMAIN: adapters
    // TEST_PRIORITY: normal

    let serialized = serde_json::to_string(&response).expect("Serialization should succeed");
    let deserialized: AIIntegrationResponse =
        serde_json::from_str(&serialized).expect("Deserialization should succeed");

    assert_eq!(response.result, deserialized.result);
    // TEST_CATEGORY: unit
    // TEST_DOMAIN: adapters
    // TEST_PRIORITY: normal
    assert_eq!(
        response.suggested_actions.len(),
        deserialized.suggested_actions.len()
    );
}

#[test]
fn test_ai_integration_response_clone() {
    let response1 = AIIntegrationResponse::default();
    let response2 = response1.clone();
    // TEST_CATEGORY: unit
    // TEST_DOMAIN: adapters
    // TEST_PRIORITY: normal

    assert_eq!(response1.result, response2.result);
}

// ============================================================================
// VendorDiscoveryContext Tests
// ============================================================================

#[test]
fn test_vendor_discovery_context_default() {
    let context = VendorDiscoveryContext::default();

    assert_eq!(context.discovery_method, "capability_based");
    assert_eq!(context.priority, 1);
    // TEST_CATEGORY: unit
    // TEST_DOMAIN: adapters
    // TEST_PRIORITY: normal
    assert!(context.context.capacity() >= 16); // HashMap reserves at least 16
}

#[test]
fn test_vendor_discovery_context_custom() {
    let mut ctx_map = HashMap::new();
    ctx_map.insert("region".to_string(), "us-west-2".to_string());

    let context = VendorDiscoveryContext {
        discovery_method: "dns_based".to_string(),
        // TEST_CATEGORY: unit
        // TEST_DOMAIN: adapters
        // TEST_PRIORITY: normal
        priority: 10,
        context: ctx_map.clone(),
    };

    assert_eq!(context.discovery_method, "dns_based");
    assert_eq!(context.priority, 10);
    // TEST_CATEGORY: unit
    // TEST_DOMAIN: adapters
    // TEST_PRIORITY: normal
    assert_eq!(context.context.len(), 1);
}

#[test]
fn test_vendor_discovery_context_serialization() {
    let context = VendorDiscoveryContext {
        discovery_method: "service_mesh".to_string(),
        priority: 5,
        context: HashMap::new(),
    };

    let serialized = serde_json::to_string(&context).expect("Serialization should succeed");
    let deserialized: VendorDiscoveryContext =
        // TEST_CATEGORY: unit
        // TEST_DOMAIN: adapters
        // TEST_PRIORITY: normal
        serde_json::from_str(&serialized).expect("Deserialization should succeed");

    assert_eq!(context.discovery_method, deserialized.discovery_method);
    assert_eq!(context.priority, deserialized.priority);
}

#[test]
fn test_vendor_discovery_context_clone() {
    let context1 = VendorDiscoveryContext::default();
    let context2 = context1.clone();

    assert_eq!(context1.discovery_method, context2.discovery_method);
    assert_eq!(context1.priority, context2.priority);
}

// ============================================================================
// TEST_CATEGORY: unit
// TEST_DOMAIN: adapters
// TEST_PRIORITY: normal
// UniversalAdapter Tests
// ============================================================================

#[test]
fn test_universal_adapter_new() {
    let config = AdapterConfig::default();
    let adapter = UniversalAdapter::new(config);

    assert!(adapter.get_capabilities().is_empty());
    // TEST_CATEGORY: unit
    // TEST_DOMAIN: adapters
    // TEST_PRIORITY: normal
}

#[test]
fn test_universal_adapter_register_capability() {
    let config = AdapterConfig::default();
    let mut adapter = UniversalAdapter::new(config);
    // TEST_CATEGORY: unit
    // TEST_DOMAIN: adapters
    // TEST_PRIORITY: normal

    adapter.register_capability(
        "encryption".to_string(),
        "http://encryption.svc".to_string(),
    );
    adapter.register_capability("signing".to_string(), "http://signing.svc".to_string());

    assert_eq!(adapter.get_capabilities().len(), 2);
    assert!(
        adapter
            .get_capabilities()
            .contains(&"encryption".to_string())
    );
    assert!(adapter.get_capabilities().contains(&"signing".to_string()));
}
// TEST_CATEGORY: unit
// TEST_DOMAIN: adapters
// TEST_PRIORITY: normal

#[test]
fn test_universal_adapter_multiple_registrations() {
    let config = AdapterConfig::default();
    let mut adapter = UniversalAdapter::new(config);

    for i in 0..10 {
        adapter.register_capability(format!("capability_{i}"), format!("http://svc{i}.local"));
    }

    // TEST_CATEGORY: unit
    // TEST_DOMAIN: adapters
    // TEST_PRIORITY: normal
    assert_eq!(adapter.get_capabilities().len(), 10);
}

#[tokio::test]
async fn test_universal_adapter_execute_unregistered_capability() {
    let mut adapter = UniversalAdapter::new(AdapterConfig::default());

    let request = CapabilityRequest {
        capability: "nonexistent".to_string(),
        operation: "test".to_string(),
        // TEST_CATEGORY: unit
        // TEST_DOMAIN: adapters
        // TEST_PRIORITY: normal
        parameters: HashMap::new(),
    };

    let response = adapter
        .execute_capability(request)
        // TEST_CATEGORY: unit
        // TEST_DOMAIN: adapters
        // TEST_PRIORITY: normal
        .await
        .expect("Should return error response");

    assert!(!response.success);
    assert!(response.error.is_some());
    assert_eq!(response.error.as_ref().unwrap(), "Capability not available");
}

#[tokio::test]
async fn test_universal_adapter_execute_registered_capability() {
    let mut adapter = UniversalAdapter::new(AdapterConfig::default());
    adapter.register_capability("test".to_string(), "http://test.com".to_string());

    let request = CapabilityRequest {
        capability: "test".to_string(),
        operation: "execute".to_string(),
        parameters: HashMap::new(),
    };

    let err = adapter
        .execute_capability(request)
        .await
        .expect_err("dispatch returns not_yet_available until IPC wired");
    assert!(err.to_string().contains("ipc.resolve"));
}
// TEST_CATEGORY: unit
// TEST_DOMAIN: adapters
// TEST_PRIORITY: normal

#[tokio::test]
async fn test_universal_adapter_caching_enabled() {
    let config = AdapterConfig {
        timeout_seconds: 30,
        retry_attempts: 3,
        enable_caching: true,
    };
    let mut adapter = UniversalAdapter::new(config);
    adapter.register_capability("test".to_string(), "http://test.com".to_string());

    let request = CapabilityRequest {
        capability: "test".to_string(),
        operation: "cached_op".to_string(),
        parameters: HashMap::new(),
    };

    // IPC dispatch not wired — both calls return not_yet_available;
    // caching behaviour will be testable once ipc.resolve is integrated
    let err1 = adapter
        .execute_capability(request.clone())
        .await
        .expect_err("dispatch returns not_yet_available until IPC wired");
    assert!(err1.to_string().contains("ipc.resolve"));

    let err2 = adapter
        .execute_capability(request)
        .await
        .expect_err("dispatch returns not_yet_available until IPC wired");
    assert!(err2.to_string().contains("ipc.resolve"));
}

#[tokio::test]
async fn test_universal_adapter_caching_disabled() {
    let config = AdapterConfig {
        timeout_seconds: 30,
        retry_attempts: 3,
        enable_caching: false,
    };
    let mut adapter = UniversalAdapter::new(config);
    adapter.register_capability("test".to_string(), "http://test.com".to_string());

    let request = CapabilityRequest {
        capability: "test".to_string(),
        operation: "no_cache_op".to_string(),
        parameters: HashMap::new(),
    };

    let err = adapter
        .execute_capability(request)
        .await
        .expect_err("dispatch returns not_yet_available until IPC wired");
    assert!(err.to_string().contains("ipc.resolve"));
}
// TEST_CATEGORY: unit
// TEST_DOMAIN: adapters
// TEST_PRIORITY: normal

#[tokio::test]
async fn test_universal_adapter_request_with_parameters() {
    let mut adapter = UniversalAdapter::new(AdapterConfig::default());
    adapter.register_capability("encryption".to_string(), "http://encrypt.svc".to_string());

    let mut params = HashMap::new();
    params.insert("algorithm".to_string(), "AES-256-GCM".to_string());
    params.insert("key_id".to_string(), "key-123".to_string());

    let request = CapabilityRequest {
        capability: "encryption".to_string(),
        operation: "encrypt".to_string(),
        parameters: params,
    };

    let err = adapter
        .execute_capability(request)
        .await
        .expect_err("dispatch returns not_yet_available until IPC wired");
    assert!(err.to_string().contains("encryption"));
    assert!(err.to_string().contains("ipc.resolve"));
}

#[tokio::test]
async fn test_universal_adapter_retry_metadata() {
    let mut adapter = UniversalAdapter::new(AdapterConfig::default());
    adapter.register_capability("test".to_string(), "http://test.com".to_string());

    let request = CapabilityRequest {
        capability: "test".to_string(),
        operation: "normal_op".to_string(),
        parameters: HashMap::new(),
    };

    let err = adapter
        .execute_capability(request)
        .await
        .expect_err("dispatch returns not_yet_available until IPC wired");
    assert!(err.to_string().contains("ipc.resolve"));
}

#[test]
fn test_universal_adapter_get_capabilities_immutable() {
    let mut adapter = UniversalAdapter::new(AdapterConfig::default());
    // TEST_CATEGORY: unit
    // TEST_DOMAIN: adapters
    // TEST_PRIORITY: normal
    adapter.register_capability("cap1".to_string(), "http://cap1".to_string());

    let caps1 = adapter.get_capabilities();
    assert_eq!(caps1.len(), 1);

    adapter.register_capability("cap2".to_string(), "http://cap2".to_string());

    let caps2 = adapter.get_capabilities();
    assert_eq!(caps2.len(), 2);
}

// ============================================================================
// Edge Cases and Error Scenarios
// ============================================================================

#[tokio::test]
async fn test_universal_adapter_empty_capability_name() {
    let mut adapter = UniversalAdapter::new(AdapterConfig::default());
    adapter.register_capability(String::new(), "http://test.com".to_string());

    let request = CapabilityRequest {
        capability: String::new(),
        operation: "test".to_string(),
        parameters: HashMap::new(),
    };

    let err = adapter
        .execute_capability(request)
        .await
        .expect_err("dispatch returns not_yet_available until IPC wired");
    assert!(err.to_string().contains("ipc.resolve"));
}
// TEST_CATEGORY: unit
// TEST_DOMAIN: adapters
// TEST_PRIORITY: normal

#[tokio::test]
async fn test_universal_adapter_special_characters_in_capability() {
    let mut adapter = UniversalAdapter::new(AdapterConfig::default());
    let special_cap = "test:cap/with-special.chars_123".to_string();
    adapter.register_capability(special_cap.clone(), "http://test.com".to_string());

    let request = CapabilityRequest {
        capability: special_cap,
        operation: "test".to_string(),
        parameters: HashMap::new(),
    };

    let err = adapter
        .execute_capability(request)
        .await
        .expect_err("dispatch returns not_yet_available until IPC wired");
    assert!(err.to_string().contains("ipc.resolve"));
}

#[tokio::test]
async fn test_universal_adapter_large_parameters() {
    let mut adapter = UniversalAdapter::new(AdapterConfig::default());
    adapter.register_capability("test".to_string(), "http://test.com".to_string());

    let mut params = HashMap::new();
    for i in 0..100 {
        params.insert(format!("key_{i}"), format!("value_{i}"));
    }

    let request = CapabilityRequest {
        capability: "test".to_string(),
        operation: "large_params".to_string(),
        parameters: params,
    };

    let err = adapter
        .execute_capability(request)
        .await
        .expect_err("dispatch returns not_yet_available until IPC wired");
    assert!(err.to_string().contains("ipc.resolve"));
}

#[test]
fn test_adapter_config_zero_timeout() {
    let config = AdapterConfig {
        timeout_seconds: 0,
        retry_attempts: 3,
        enable_caching: true,
    };

    // Should be able to create config with zero timeout (edge case)
    // TEST_CATEGORY: unit
    // TEST_DOMAIN: adapters
    // TEST_PRIORITY: normal
    assert_eq!(config.timeout_seconds, 0);
}

#[test]
fn test_adapter_config_zero_retries() {
    let config = AdapterConfig {
        timeout_seconds: 30,
        retry_attempts: 0,
        enable_caching: true,
        // TEST_CATEGORY: unit
        // TEST_DOMAIN: adapters
        // TEST_PRIORITY: normal
    };

    // Should be able to create config with zero retries
    assert_eq!(config.retry_attempts, 0);
}

#[test]
fn test_capability_response_debug_format() {
    let response = CapabilityResponse {
        // TEST_CATEGORY: unit
        // TEST_DOMAIN: adapters
        // TEST_PRIORITY: normal
        success: true,
        data: None,
        error: None,
        metadata: HashMap::new(),
    };

    let debug_str = format!("{response:?}");
    assert!(debug_str.contains("CapabilityResponse"));
    assert!(debug_str.contains("success"));
}

// TEST_CATEGORY: unit
// TEST_DOMAIN: adapters
// TEST_PRIORITY: normal
#[test]
fn test_ai_response_metadata_debug_format() {
    let metadata = AIResponseMetadata::default();
    let debug_str = format!("{metadata:?}");

    assert!(debug_str.contains("AIResponseMetadata"));
    assert!(debug_str.contains("confidence_score"));
}
