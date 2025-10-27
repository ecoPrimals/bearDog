//! Adapter Integration Comprehensive Tests
//! Created: October 26, 2025
//! Purpose: Week 2 Day 4 - Comprehensive adapter integration, error handling, and edge cases

use crate::{
    AIIntegrationResponse, AIResponseMetadata, AdapterConfig, CapabilityRequest,
    CapabilityResponse, UniversalAdapter, VendorDiscoveryContext,
};
use std::collections::HashMap;

// ============================================================================
// Test 1: AdapterConfig Default Values
// ============================================================================

#[test]
fn test_adapter_config_defaults() {
    let config = AdapterConfig::default();

    assert_eq!(config.timeout_seconds, 30, "Default timeout should be 30s");
    assert_eq!(
        config.retry_attempts, 3,
        "Default retry attempts should be 3"
    );
    assert!(
        config.enable_caching,
        "Caching should be enabled by default"
    );
}

// ============================================================================
// Test 2: AdapterConfig Custom Values
// ============================================================================

#[test]
fn test_adapter_config_custom_values() {
    let config = AdapterConfig {
        timeout_seconds: 60,
        retry_attempts: 5,
        enable_caching: false,
    };

    assert_eq!(config.timeout_seconds, 60);
    assert_eq!(config.retry_attempts, 5);
    assert!(!config.enable_caching);
}

// ============================================================================
// Test 3: AdapterConfig Serialization
// ============================================================================

#[test]
fn test_adapter_config_serialization() {
    let config = AdapterConfig::default();

    let serialized = serde_json::to_string(&config).expect("Should serialize");
    let deserialized: AdapterConfig =
        serde_json::from_str(&serialized).expect("Should deserialize");

    assert_eq!(config.timeout_seconds, deserialized.timeout_seconds);
    assert_eq!(config.retry_attempts, deserialized.retry_attempts);
    assert_eq!(config.enable_caching, deserialized.enable_caching);
}

// ============================================================================
// Test 4: CapabilityRequest Construction
// ============================================================================

#[test]
fn test_capability_request_construction() {
    let mut params = HashMap::new();
    params.insert("key1".to_string(), "value1".to_string());
    params.insert("key2".to_string(), "value2".to_string());

    let request = CapabilityRequest {
        capability: "encryption".to_string(),
        operation: "encrypt".to_string(),
        parameters: params.clone(),
    };

    assert_eq!(request.capability, "encryption");
    assert_eq!(request.operation, "encrypt");
    assert_eq!(request.parameters.len(), 2);
    assert_eq!(request.parameters.get("key1"), Some(&"value1".to_string()));
}

// ============================================================================
// Test 5: CapabilityResponse Success Case
// ============================================================================

#[test]
fn test_capability_response_success() {
    let mut metadata = HashMap::new();
    metadata.insert("provider".to_string(), "test_provider".to_string());

    let response = CapabilityResponse {
        success: true,
        data: Some(serde_json::json!({"result": "encrypted_data"})),
        error: None,
        metadata,
    };

    assert!(response.success);
    assert!(response.data.is_some());
    assert!(response.error.is_none());
    assert_eq!(
        response.metadata.get("provider"),
        Some(&"test_provider".to_string())
    );
}

// ============================================================================
// Test 6: CapabilityResponse Error Case
// ============================================================================

#[test]
fn test_capability_response_error() {
    let response = CapabilityResponse {
        success: false,
        data: None,
        error: Some("Operation failed".to_string()),
        metadata: HashMap::new(),
    };

    assert!(!response.success);
    assert!(response.data.is_none());
    assert_eq!(response.error, Some("Operation failed".to_string()));
}

// ============================================================================
// Test 7: AIResponseMetadata Default Values
// ============================================================================

#[test]
fn test_ai_response_metadata_defaults() {
    let metadata = AIResponseMetadata::default();

    assert_eq!(metadata.confidence_score, 1.0);
    assert_eq!(metadata.processing_time_ms, 0);
    assert_eq!(metadata.model_version, "v1.0.0");
}

// ============================================================================
// Test 8: AIIntegrationResponse Construction
// ============================================================================

#[test]
fn test_ai_integration_response_with_suggestions() {
    let suggestions = vec![
        "action1".to_string(),
        "action2".to_string(),
        "action3".to_string(),
    ];

    let response = AIIntegrationResponse {
        result: "analysis_complete".to_string(),
        ai_metadata: AIResponseMetadata {
            confidence_score: 0.95,
            processing_time_ms: 150,
            model_version: "v2.0.0".to_string(),
        },
        suggested_actions: suggestions.clone(),
    };

    assert_eq!(response.result, "analysis_complete");
    assert_eq!(response.ai_metadata.confidence_score, 0.95);
    assert_eq!(response.ai_metadata.processing_time_ms, 150);
    assert_eq!(response.suggested_actions.len(), 3);
    assert_eq!(response.suggested_actions[0], "action1");
}

// ============================================================================
// Test 9: VendorDiscoveryContext Default Values
// ============================================================================

#[test]
fn test_vendor_discovery_context_defaults() {
    let context = VendorDiscoveryContext::default();

    assert_eq!(context.discovery_method, "capability_based");
    assert_eq!(context.priority, 1);
    // Capacity may vary based on HashMap implementation
    assert!(
        context.context.capacity() >= 16,
        "Context should have at least 16 capacity"
    );
    assert!(context.context.is_empty());
}

// ============================================================================
// Test 10: VendorDiscoveryContext With Custom Data
// ============================================================================

#[test]
fn test_vendor_discovery_context_with_data() {
    let mut ctx_data = HashMap::new();
    ctx_data.insert("region".to_string(), "us-west-2".to_string());
    ctx_data.insert("environment".to_string(), "production".to_string());

    let context = VendorDiscoveryContext {
        discovery_method: "network_scan".to_string(),
        priority: 5,
        context: ctx_data,
    };

    assert_eq!(context.discovery_method, "network_scan");
    assert_eq!(context.priority, 5);
    assert_eq!(context.context.len(), 2);
    assert_eq!(
        context.context.get("region"),
        Some(&"us-west-2".to_string())
    );
}

// ============================================================================
// Test 11: UniversalAdapter Creation
// ============================================================================

#[test]
fn test_universal_adapter_creation() {
    let config = AdapterConfig::default();
    let _adapter = UniversalAdapter::new(config);

    // Adapter should be created successfully
    // This tests that the constructor doesn't panic
    // No assertion needed - test passes if constructor doesn't panic
}

// ============================================================================
// Test 12: AdapterConfig Clone and Debug
// ============================================================================

#[test]
fn test_adapter_config_clone_and_debug() {
    let config = AdapterConfig {
        timeout_seconds: 45,
        retry_attempts: 4,
        enable_caching: true,
    };

    // Test Clone
    let cloned = config.clone();
    assert_eq!(config.timeout_seconds, cloned.timeout_seconds);
    assert_eq!(config.retry_attempts, cloned.retry_attempts);
    assert_eq!(config.enable_caching, cloned.enable_caching);

    // Test Debug
    let debug_str = format!("{:?}", config);
    assert!(debug_str.contains("AdapterConfig"));
    assert!(debug_str.contains("45"));
    assert!(debug_str.contains("4"));
}
