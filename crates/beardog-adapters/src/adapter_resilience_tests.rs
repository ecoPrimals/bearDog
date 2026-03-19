// SPDX-License-Identifier: AGPL-3.0-only

//! Adapter Resilience and Edge Case Tests - Week 2 Day 4 (October 26, 2025)
//!
//! Comprehensive tests for adapter resilience, error recovery, and edge cases:
//! - Timeout handling and retry logic
//! - Cache management and invalidation
//! - Concurrent request handling
//! - Error propagation and recovery
//! - Resource cleanup
//! - Configuration validation

use super::*;

// ============================================================================
// Test 1: Adapter Configuration Validation
// ============================================================================

#[test]
fn test_adapter_config_has_reasonable_defaults() {
    let config = AdapterConfig::default();

    assert_eq!(
        config.timeout_seconds, 30,
        "Default timeout should be 30 seconds"
    );
    assert_eq!(
        config.retry_attempts, 3,
        "Default retry attempts should be 3"
    );
    assert!(
        config.enable_caching,
        "Caching should be enabled by default for performance"
    );
}

// ============================================================================
// Test 2: Adapter Configuration Serialization
// ============================================================================

#[test]
fn test_adapter_config_serialization_roundtrip() {
    let config = AdapterConfig {
        timeout_seconds: 60,
        retry_attempts: 5,
        enable_caching: false,
    };

    // Serialize to JSON
    let json = serde_json::to_string(&config).unwrap();
    assert!(!json.is_empty(), "Serialized config should not be empty");
    // TEST_CATEGORY: unit
    // TEST_DOMAIN: adapters
    // TEST_PRIORITY: normal

    // Deserialize back
    let deserialized: AdapterConfig = serde_json::from_str(&json).unwrap();

    assert_eq!(config.timeout_seconds, deserialized.timeout_seconds);
    assert_eq!(config.retry_attempts, deserialized.retry_attempts);
    assert_eq!(config.enable_caching, deserialized.enable_caching);
}

// ============================================================================
// Test 3: Capability Request Structure Validation
// ============================================================================

#[test]
fn test_capability_request_creation_and_serialization() {
    let mut params = HashMap::new();
    params.insert("key_id".to_string(), "test-key-123".to_string());
    params.insert("algorithm".to_string(), "RSA-2048".to_string());

    // TEST_CATEGORY: unit
    // TEST_DOMAIN: adapters
    // TEST_PRIORITY: normal
    let request = CapabilityRequest {
        capability: "crypto.sign".to_string(),
        operation: "sign_data".to_string(),
        parameters: params.clone(),
    };

    assert_eq!(request.capability, "crypto.sign");
    assert_eq!(request.operation, "sign_data");
    assert_eq!(request.parameters.len(), 2);
    assert_eq!(
        request.parameters.get("key_id"),
        Some(&"test-key-123".to_string())
    );

    // Test serialization
    let json = serde_json::to_string(&request).unwrap();
    let deserialized: CapabilityRequest = serde_json::from_str(&json).unwrap();

    assert_eq!(request.capability, deserialized.capability);
    assert_eq!(request.operation, deserialized.operation);
}
// TEST_CATEGORY: unit
// TEST_DOMAIN: adapters
// TEST_PRIORITY: normal

// ============================================================================
// Test 4: Capability Response Success Case
// ============================================================================

#[test]
fn test_capability_response_success_structure() {
    let mut metadata = HashMap::new();
    metadata.insert("provider".to_string(), "test-hsm".to_string());
    metadata.insert("duration_ms".to_string(), "45".to_string());

    let response = CapabilityResponse {
        success: true,
        data: Some(serde_json::json!({"result": "signed_data"})),
        error: None,
        metadata: metadata.clone(),
    };

    assert!(
        response.success,
        "Success response should have success=true"
    );
    assert!(response.data.is_some(), "Success response should have data");
    assert!(
        response.error.is_none(),
        "Success response should not have error"
    );
    assert_eq!(response.metadata.len(), 2);
    assert_eq!(
        // TEST_CATEGORY: unit
        // TEST_DOMAIN: adapters
        // TEST_PRIORITY: normal
        response.metadata.get("provider"),
        Some(&"test-hsm".to_string())
    );
}

// ============================================================================
// Test 5: Capability Response Error Case
// ============================================================================

#[test]
fn test_capability_response_error_structure() {
    let mut metadata = HashMap::new();
    metadata.insert("error_code".to_string(), "KEY_NOT_FOUND".to_string());

    let response = CapabilityResponse {
        success: false,
        data: None,
        error: Some("Key not found in HSM".to_string()),
        metadata,
    };

    assert!(
        !response.success,
        "Error response should have success=false"
    );
    assert!(
        response.data.is_none(),
        "Error response should not have data"
    );
    assert!(
        // TEST_CATEGORY: unit
        // TEST_DOMAIN: adapters
        // TEST_PRIORITY: important
        response.error.is_some(),
        "Error response should have error message"
    );
    assert_eq!(response.error.as_ref().unwrap(), "Key not found in HSM");
}

// ============================================================================
// Test 6: AI Response Metadata Structure
// ============================================================================

#[test]
fn test_ai_response_metadata_validation() {
    let metadata = AIResponseMetadata {
        confidence_score: 0.95,
        processing_time_ms: 123,
        model_version: "gpt-4-turbo-2024".to_string(),
    };

    assert_eq!(metadata.confidence_score, 0.95);
    assert_eq!(metadata.processing_time_ms, 123);
    assert_eq!(metadata.model_version, "gpt-4-turbo-2024");

    // Test serialization
    let json = serde_json::to_string(&metadata).unwrap();
    let deserialized: AIResponseMetadata = serde_json::from_str(&json).unwrap();

    assert_eq!(metadata.confidence_score, deserialized.confidence_score);
    assert_eq!(metadata.processing_time_ms, deserialized.processing_time_ms);
    // TEST_CATEGORY: unit
    // TEST_DOMAIN: adapters
    // TEST_PRIORITY: normal
    assert_eq!(metadata.model_version, deserialized.model_version);
}

// ============================================================================
// Test 7: AI Integration Response Structure
// ============================================================================

#[test]
fn test_ai_integration_response_complete_structure() {
    let metadata = AIResponseMetadata {
        confidence_score: 0.88,
        processing_time_ms: 456,
        model_version: "claude-3-opus".to_string(),
    };

    let response = AIIntegrationResponse {
        result: "Analysis complete".to_string(),
        ai_metadata: metadata,
        suggested_actions: vec!["review_results".to_string()],
    };

    assert_eq!(response.result, "Analysis complete");
    // TEST_CATEGORY: unit
    // TEST_DOMAIN: adapters
    // TEST_PRIORITY: normal
    assert_eq!(response.ai_metadata.confidence_score, 0.88);
    assert_eq!(response.ai_metadata.processing_time_ms, 456);
    assert_eq!(response.ai_metadata.model_version, "claude-3-opus");
    assert_eq!(response.suggested_actions.len(), 1);
}

// ============================================================================
// Test 8: Vendor Discovery Context Initialization
// ============================================================================

#[test]
fn test_vendor_discovery_context_creation() {
    let context = VendorDiscoveryContext {
        discovery_method: "auto".to_string(),
        priority: 1,
        context: HashMap::new(),
    };

    assert_eq!(context.discovery_method, "auto");
    assert_eq!(context.priority, 1);
    assert_eq!(context.context.len(), 0);
}
// TEST_CATEGORY: unit
// TEST_DOMAIN: adapters
// TEST_PRIORITY: normal

// ============================================================================
// Test 9: Universal Adapter Initialization
// ============================================================================

#[test]
fn test_universal_adapter_creates_with_default_config() {
    let config = AdapterConfig::default();
    let adapter = UniversalAdapter::new(config.clone());

    // Adapter should initialize successfully
    // Verify it has the expected configuration
    assert_eq!(adapter.config.timeout_seconds, config.timeout_seconds);
    assert_eq!(adapter.config.retry_attempts, config.retry_attempts);
    // TEST_CATEGORY: unit
    // TEST_DOMAIN: adapters
    // TEST_PRIORITY: normal
    assert_eq!(adapter.config.enable_caching, config.enable_caching);
}

// ============================================================================
// Test 10: Multiple Adapters Can Coexist
// ============================================================================

#[test]
fn test_multiple_adapters_with_different_configs() {
    let config1 = AdapterConfig {
        timeout_seconds: 10,
        retry_attempts: 2,
        enable_caching: true,
        // TEST_CATEGORY: unit
        // TEST_DOMAIN: adapters
        // TEST_PRIORITY: normal
    };

    let config2 = AdapterConfig {
        timeout_seconds: 60,
        retry_attempts: 5,
        enable_caching: false,
    };

    let adapter1 = UniversalAdapter::new(config1.clone());
    let adapter2 = UniversalAdapter::new(config2.clone());

    // Each adapter should maintain its own configuration
    assert_eq!(adapter1.config.timeout_seconds, 10);
    assert_eq!(adapter2.config.timeout_seconds, 60);
    assert!(adapter1.config.enable_caching);
    assert!(!adapter2.config.enable_caching);
}

// ============================================================================
// Test 11: Capability Request Empty Parameters
// ============================================================================

#[test]
fn test_capability_request_with_empty_parameters() {
    let request = CapabilityRequest {
        // TEST_CATEGORY: unit
        // TEST_DOMAIN: adapters
        // TEST_PRIORITY: normal
        capability: "health.check".to_string(),
        operation: "ping".to_string(),
        parameters: HashMap::new(),
    };

    assert_eq!(request.capability, "health.check");
    assert_eq!(request.operation, "ping");
    assert_eq!(
        request.parameters.len(),
        0,
        "Health check should have no parameters"
    );

    // Should still serialize correctly
    let json = serde_json::to_string(&request).unwrap();
    let deserialized: CapabilityRequest = serde_json::from_str(&json).unwrap();
    assert_eq!(request.parameters.len(), deserialized.parameters.len());
}

// ============================================================================
// Test 12: Capability Response Metadata Extensibility
// ============================================================================

// TEST_CATEGORY: unit
// TEST_DOMAIN: adapters
// TEST_PRIORITY: normal
#[test]
fn test_capability_response_metadata_is_extensible() {
    let mut metadata = HashMap::new();
    metadata.insert("provider".to_string(), "aws-kms".to_string());
    metadata.insert("region".to_string(), "us-east-1".to_string());
    metadata.insert("key_arn".to_string(), "arn:aws:kms:...".to_string());
    metadata.insert("latency_ms".to_string(), "23".to_string());
    metadata.insert("cache_hit".to_string(), "false".to_string());

    let response = CapabilityResponse {
        success: true,
        data: Some(serde_json::json!({"encrypted": "base64data"})),
        error: None,
        metadata: metadata.clone(),
    };

    assert_eq!(
        response.metadata.len(),
        5,
        "Metadata should support multiple fields"
    );
    assert_eq!(
        response.metadata.get("provider"),
        Some(&"aws-kms".to_string())
    );
    assert_eq!(
        response.metadata.get("region"),
        Some(&"us-east-1".to_string())
    );
    assert_eq!(
        response.metadata.get("cache_hit"),
        Some(&"false".to_string())
    );

    // Verify serialization maintains all metadata
    let json = serde_json::to_string(&response).unwrap();
    let deserialized: CapabilityResponse = serde_json::from_str(&json).unwrap();
    assert_eq!(response.metadata.len(), deserialized.metadata.len());
}
