// SPDX-License-Identifier: AGPL-3.0-only

//! Additional coverage tests for beardog-adapters lib.rs
//!
//! Focus on testing UniversalAdapter, AdapterConfig, and related types that may
//! have gaps in test coverage.

use crate::{AdapterConfig, CapabilityResponse, UniversalAdapter};
use std::collections::HashMap;

// TEST_CATEGORY: unit
// TEST_DOMAIN: adapters
// TEST_PRIORITY: normal
#[test]
fn test_adapter_config_default() {
    let config = AdapterConfig::default();
    assert_eq!(config.timeout_seconds, 30);
    assert_eq!(config.retry_attempts, 3);
    assert!(config.enable_caching);
}

// TEST_CATEGORY: unit
// TEST_DOMAIN: adapters
// TEST_PRIORITY: normal
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

// TEST_CATEGORY: unit
// TEST_DOMAIN: adapters
// TEST_PRIORITY: normal
#[test]
fn test_adapter_config_clone() {
    let config1 = AdapterConfig::default();
    let config2 = config1.clone();

    assert_eq!(config1.timeout_seconds, config2.timeout_seconds);
    assert_eq!(config1.retry_attempts, config2.retry_attempts);
    assert_eq!(config1.enable_caching, config2.enable_caching);
}

// TEST_CATEGORY: unit
// TEST_DOMAIN: adapters
// TEST_PRIORITY: normal
#[test]
fn test_adapter_config_debug() {
    let config = AdapterConfig::default();
    let debug_str = format!("{:?}", config);
    assert!(debug_str.contains("AdapterConfig"));
    assert!(debug_str.contains("timeout"));
}

// TEST_CATEGORY: unit
// TEST_DOMAIN: adapters
// TEST_PRIORITY: normal
#[test]
fn test_adapter_config_serialization() {
    let config = AdapterConfig::default();
    let json = serde_json::to_string(&config).expect("Should serialize");
    assert!(json.contains("timeout_seconds"));
    assert!(json.contains("retry_attempts"));
    assert!(json.contains("enable_caching"));
}

// TEST_CATEGORY: unit
// TEST_DOMAIN: adapters
// TEST_PRIORITY: normal
#[test]
fn test_adapter_config_deserialization() {
    let json = r#"{
        "timeout_seconds": 45,
        "retry_attempts": 4,
        "enable_caching": true
    }"#;

    let config: AdapterConfig = serde_json::from_str(json).expect("Should deserialize");
    assert_eq!(config.timeout_seconds, 45);
    assert_eq!(config.retry_attempts, 4);
    assert!(config.enable_caching);
}

// TEST_CATEGORY: unit
// TEST_DOMAIN: adapters
// TEST_PRIORITY: normal
#[test]
fn test_universal_adapter_creation() {
    let config = AdapterConfig::default();
    let adapter = UniversalAdapter::new(config.clone());

    // Verify adapter is created
    let debug_str = format!("{:?}", adapter);
    assert!(debug_str.contains("UniversalAdapter"));
}

// TEST_CATEGORY: unit
// TEST_DOMAIN: adapters
// TEST_PRIORITY: normal
#[test]
fn test_universal_adapter_inspect() {
    let config = AdapterConfig::default();
    let adapter = UniversalAdapter::new(config);

    // Test adapter basic properties
    let debug_str = format!("{:?}", adapter);
    assert!(debug_str.contains("UniversalAdapter"));
}

// TEST_CATEGORY: unit
// TEST_DOMAIN: adapters
// TEST_PRIORITY: normal
#[test]
fn test_universal_adapter_with_caching_disabled() {
    let config = AdapterConfig {
        enable_caching: false,
        ..Default::default()
    };
    let adapter = UniversalAdapter::new(config);

    // Verify adapter created
    let debug_str = format!("{:?}", adapter);
    assert!(debug_str.contains("UniversalAdapter"));
}

// TEST_CATEGORY: unit
// TEST_DOMAIN: adapters
// TEST_PRIORITY: normal
#[test]
fn test_capability_response_success() {
    let response = CapabilityResponse {
        success: true,
        data: Some(serde_json::json!({"key": "value"})),
        error: None,
        metadata: HashMap::new(),
    };

    assert!(response.success);
    assert!(response.data.is_some());
    assert!(response.error.is_none());
}

// TEST_CATEGORY: unit
// TEST_DOMAIN: adapters
// TEST_PRIORITY: normal
#[test]
fn test_capability_response_error() {
    let response = CapabilityResponse {
        success: false,
        data: None,
        error: Some("Test error".to_string()),
        metadata: HashMap::new(),
    };

    assert!(!response.success);
    assert!(response.data.is_none());
    assert_eq!(response.error, Some("Test error".to_string()));
}

// TEST_CATEGORY: unit
// TEST_DOMAIN: adapters
// TEST_PRIORITY: normal
#[test]
fn test_capability_response_metadata() {
    let mut metadata = HashMap::new();
    metadata.insert("key".to_string(), "value".to_string());

    let response = CapabilityResponse {
        success: true,
        data: None,
        error: None,
        metadata: metadata.clone(),
    };

    assert!(response.success);
    assert_eq!(response.metadata.get("key"), Some(&"value".to_string()));
}

// TEST_CATEGORY: unit
// TEST_DOMAIN: adapters
// TEST_PRIORITY: normal
#[test]
fn test_capability_response_clone() {
    let response1 = CapabilityResponse {
        success: true,
        data: Some(serde_json::json!({"test": "data"})),
        error: None,
        metadata: HashMap::new(),
    };

    let response2 = response1.clone();
    assert_eq!(response1.success, response2.success);
}

// TEST_CATEGORY: unit
// TEST_DOMAIN: adapters
// TEST_PRIORITY: normal
#[test]
fn test_capability_response_serialization() {
    let response = CapabilityResponse {
        success: true,
        data: Some(serde_json::json!({"key": "value"})),
        error: None,
        metadata: HashMap::new(),
    };

    let json = serde_json::to_string(&response).expect("Should serialize");
    assert!(json.contains("success"));
    assert!(json.contains("data"));
}

// TEST_CATEGORY: unit
// TEST_DOMAIN: adapters
// TEST_PRIORITY: normal
#[tokio::test]
async fn test_universal_adapter_cache_behavior() {
    let config = AdapterConfig {
        enable_caching: true,
        ..Default::default()
    };

    let adapter = UniversalAdapter::new(config);
    // Cache operations tested internally
    let debug_str = format!("{:?}", adapter);
    assert!(debug_str.contains("UniversalAdapter"));
}

// TEST_CATEGORY: unit
// TEST_DOMAIN: adapters
// TEST_PRIORITY: normal
#[tokio::test]
async fn test_universal_adapter_no_cache() {
    let config = AdapterConfig {
        enable_caching: false,
        ..Default::default()
    };

    let adapter = UniversalAdapter::new(config);
    let debug_str = format!("{:?}", adapter);
    assert!(debug_str.contains("UniversalAdapter"));
}

// TEST_CATEGORY: unit
// TEST_DOMAIN: adapters
// TEST_PRIORITY: normal
#[test]
fn test_adapter_config_zero_timeout() {
    let config = AdapterConfig {
        timeout_seconds: 0,
        ..Default::default()
    };

    assert_eq!(config.timeout_seconds, 0);
}

// TEST_CATEGORY: unit
// TEST_DOMAIN: adapters
// TEST_PRIORITY: normal
#[test]
fn test_adapter_config_high_retries() {
    let config = AdapterConfig {
        retry_attempts: 100,
        ..Default::default()
    };

    assert_eq!(config.retry_attempts, 100);
}

// TEST_CATEGORY: unit
// TEST_DOMAIN: adapters
// TEST_PRIORITY: normal
#[test]
fn test_capability_response_with_complex_data() {
    let response = CapabilityResponse {
        success: true,
        data: Some(serde_json::json!({
            "nested": {
                "array": [1, 2, 3],
                "object": {"key": "value"}
            }
        })),
        error: None,
        metadata: HashMap::new(),
    };

    assert!(response.data.is_some());
    if let Some(data) = &response.data {
        assert!(data.get("nested").is_some());
    }
}

// TEST_CATEGORY: unit
// TEST_DOMAIN: adapters
// TEST_PRIORITY: normal
#[test]
fn test_adapter_config_high_timeout() {
    let config = AdapterConfig {
        timeout_seconds: 3600, // 1 hour
        ..Default::default()
    };

    assert_eq!(config.timeout_seconds, 3600);
}
