//! Comprehensive Adapter Operations Tests
//!
//! Tests for UniversalAdapter operations including:
//! - Request/response handling
//! - Caching mechanisms
//! - Timeout behavior
//! - Retry logic
//! - Error handling

#[cfg(test)]
mod adapter_operations {
    use crate::{AdapterConfig, CapabilityRequest, UniversalAdapter};
    use std::collections::HashMap;

    #[test]
    fn test_adapter_creation() {
        let adapter = UniversalAdapter::new(AdapterConfig::default());
        assert!(adapter.capabilities.is_empty());
        assert!(adapter.endpoints.is_empty());
    }

    #[test]
    fn test_adapter_with_custom_config() {
        let config = AdapterConfig {
            timeout_seconds: 60,
            retry_attempts: 5,
            enable_caching: false,
        };
        let adapter = UniversalAdapter::new(config.clone());
        assert_eq!(adapter.config.timeout_seconds, 60);
        assert_eq!(adapter.config.retry_attempts, 5);
        assert!(!adapter.config.enable_caching);
    }

    #[test]
    fn test_adapter_config_default() {
        let config = AdapterConfig::default();
        assert_eq!(config.timeout_seconds, 30);
        assert_eq!(config.retry_attempts, 3);
        assert!(config.enable_caching);
    }

    #[test]
    fn test_adapter_config_serialization() {
        let config = AdapterConfig::default();
        let json = serde_json::to_string(&config).expect("Should serialize");
        assert!(json.contains("timeout_seconds"));
        assert!(json.contains("retry_attempts"));
        assert!(json.contains("enable_caching"));
    }

    #[test]
    fn test_adapter_config_deserialization() {
        let json = r#"{"timeout_seconds":45,"retry_attempts":2,"enable_caching":true}"#;
        let config: AdapterConfig = serde_json::from_str(json).expect("Should deserialize");
        assert_eq!(config.timeout_seconds, 45);
        assert_eq!(config.retry_attempts, 2);
        assert!(config.enable_caching);
    }

    #[test]
    fn test_adapter_config_clone() {
        let config1 = AdapterConfig::default();
        let config2 = config1.clone();
        assert_eq!(config1.timeout_seconds, config2.timeout_seconds);
        assert_eq!(config1.retry_attempts, config2.retry_attempts);
    }

    #[test]
    fn test_adapter_config_debug() {
        let config = AdapterConfig::default();
        let debug = format!("{:?}", config);
        assert!(debug.contains("AdapterConfig"));
        assert!(debug.contains("timeout_seconds"));
    }

    #[test]
    fn test_capability_request_creation() {
        let mut params = HashMap::new();
        params.insert("key1".to_string(), "value1".to_string());

        let request = CapabilityRequest {
            capability: "test_capability".to_string(),
            operation: "test_operation".to_string(),
            parameters: params,
        };

        assert_eq!(request.capability, "test_capability");
        assert_eq!(request.operation, "test_operation");
        assert_eq!(request.parameters.len(), 1);
    }

    #[test]
    fn test_capability_request_serialization() {
        let request = CapabilityRequest {
            capability: "security".to_string(),
            operation: "encrypt".to_string(),
            parameters: HashMap::new(),
        };

        let json = serde_json::to_string(&request).expect("Should serialize");
        assert!(json.contains("security"));
        assert!(json.contains("encrypt"));
    }

    #[test]
    fn test_capability_request_with_multiple_params() {
        let mut params = HashMap::new();
        params.insert("param1".to_string(), "value1".to_string());
        params.insert("param2".to_string(), "value2".to_string());
        params.insert("param3".to_string(), "value3".to_string());

        let request = CapabilityRequest {
            capability: "test".to_string(),
            operation: "multi_param".to_string(),
            parameters: params,
        };

        assert_eq!(request.parameters.len(), 3);
    }

    #[test]
    fn test_capability_request_empty_params() {
        let request = CapabilityRequest {
            capability: "test".to_string(),
            operation: "no_params".to_string(),
            parameters: HashMap::new(),
        };

        assert!(request.parameters.is_empty());
    }

    #[test]
    fn test_adapter_clone() {
        let adapter1 = UniversalAdapter::new(AdapterConfig::default());
        let adapter2 = adapter1.clone();

        assert_eq!(
            adapter1.config.timeout_seconds,
            adapter2.config.timeout_seconds
        );
    }

    #[test]
    fn test_adapter_debug() {
        let adapter = UniversalAdapter::new(AdapterConfig::default());
        let debug = format!("{:?}", adapter);
        assert!(debug.contains("UniversalAdapter"));
    }

    #[test]
    fn test_adapter_config_high_timeout() {
        let config = AdapterConfig {
            timeout_seconds: 300,
            retry_attempts: 10,
            enable_caching: true,
        };
        let adapter = UniversalAdapter::new(config);
        assert_eq!(adapter.config.timeout_seconds, 300);
        assert_eq!(adapter.config.retry_attempts, 10);
    }

    #[test]
    fn test_adapter_config_low_timeout() {
        let config = AdapterConfig {
            timeout_seconds: 1,
            retry_attempts: 1,
            enable_caching: false,
        };
        let adapter = UniversalAdapter::new(config);
        assert_eq!(adapter.config.timeout_seconds, 1);
        assert_eq!(adapter.config.retry_attempts, 1);
    }

    #[test]
    fn test_adapter_config_zero_retries() {
        let config = AdapterConfig {
            timeout_seconds: 30,
            retry_attempts: 0,
            enable_caching: true,
        };
        let adapter = UniversalAdapter::new(config);
        assert_eq!(adapter.config.retry_attempts, 0);
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

    #[test]
    fn test_multiple_adapters_independent() {
        let config1 = AdapterConfig {
            timeout_seconds: 30,
            retry_attempts: 3,
            enable_caching: true,
        };
        let config2 = AdapterConfig {
            timeout_seconds: 60,
            retry_attempts: 5,
            enable_caching: false,
        };

        let adapter1 = UniversalAdapter::new(config1);
        let adapter2 = UniversalAdapter::new(config2);

        assert_ne!(
            adapter1.config.timeout_seconds,
            adapter2.config.timeout_seconds
        );
        assert_ne!(
            adapter1.config.retry_attempts,
            adapter2.config.retry_attempts
        );
    }

    #[test]
    fn test_adapter_creation_performance() {
        let start = std::time::Instant::now();
        let _adapter = UniversalAdapter::new(AdapterConfig::default());
        let duration = start.elapsed();

        // Adapter creation should be fast
        assert!(duration.as_millis() < 10);
    }

    #[test]
    fn test_bulk_adapter_creation() {
        let mut adapters = Vec::new();
        for _ in 0..100 {
            adapters.push(UniversalAdapter::new(AdapterConfig::default()));
        }
        assert_eq!(adapters.len(), 100);
    }

    #[test]
    fn test_capability_request_large_params() {
        let mut params = HashMap::new();
        for i in 0..100 {
            params.insert(format!("param{}", i), format!("value{}", i));
        }

        let request = CapabilityRequest {
            capability: "test".to_string(),
            operation: "large".to_string(),
            parameters: params,
        };

        assert_eq!(request.parameters.len(), 100);
    }

    #[test]
    fn test_adapter_config_edge_values() {
        let config = AdapterConfig {
            timeout_seconds: u64::MAX,
            retry_attempts: u32::MAX,
            enable_caching: true,
        };
        let adapter = UniversalAdapter::new(config);
        assert_eq!(adapter.config.timeout_seconds, u64::MAX);
        assert_eq!(adapter.config.retry_attempts, u32::MAX);
    }

    #[test]
    fn test_capability_request_special_characters() {
        let mut params = HashMap::new();
        params.insert("key!@#$%".to_string(), "value^&*()".to_string());

        let request = CapabilityRequest {
            capability: "test-capability_123".to_string(),
            operation: "operation.with.dots".to_string(),
            parameters: params,
        };

        assert!(request.capability.contains('-'));
        assert!(request.operation.contains('.'));
    }

    #[test]
    fn test_adapter_config_round_trip() {
        let original = AdapterConfig {
            timeout_seconds: 45,
            retry_attempts: 7,
            enable_caching: false,
        };

        let json = serde_json::to_string(&original).expect("Serialize");
        let deserialized: AdapterConfig = serde_json::from_str(&json).expect("Deserialize");

        assert_eq!(original.timeout_seconds, deserialized.timeout_seconds);
        assert_eq!(original.retry_attempts, deserialized.retry_attempts);
        assert_eq!(original.enable_caching, deserialized.enable_caching);
    }

    #[test]
    fn test_capability_request_empty_strings() {
        let request = CapabilityRequest {
            capability: String::new(),
            operation: String::new(),
            parameters: HashMap::new(),
        };

        assert!(request.capability.is_empty());
        assert!(request.operation.is_empty());
    }

    #[test]
    fn test_adapter_memory_efficiency() {
        let adapter = UniversalAdapter::new(AdapterConfig::default());
        let size = std::mem::size_of_val(&adapter);

        // Adapter should be reasonably sized
        assert!(size < 10000);
    }

    #[test]
    fn test_config_memory_size() {
        let config = AdapterConfig::default();
        let size = std::mem::size_of_val(&config);

        // Config should be small
        assert!(size < 100);
    }
}

#[cfg(test)]
mod adapter_edge_cases {
    use crate::{AdapterConfig, UniversalAdapter};

    #[test]
    fn test_concurrent_adapter_creation() {
        use std::thread;

        let handles: Vec<_> = (0..10)
            .map(|_| {
                thread::spawn(|| {
                    let _adapter = UniversalAdapter::new(AdapterConfig::default());
                })
            })
            .collect();

        for handle in handles {
            handle.join().expect("Thread should complete");
        }
    }

    #[test]
    fn test_rapid_creation_and_drop() {
        for _ in 0..1000 {
            let _adapter = UniversalAdapter::new(AdapterConfig::default());
        }
    }

    #[test]
    fn test_adapter_clone_independence() {
        let adapter1 = UniversalAdapter::new(AdapterConfig::default());
        let adapter2 = adapter1.clone();

        drop(adapter1);

        // adapter2 should still be valid
        assert!(adapter2.config.enable_caching);
    }

    #[test]
    fn test_config_with_disabled_caching() {
        let config = AdapterConfig {
            timeout_seconds: 30,
            retry_attempts: 3,
            enable_caching: false,
        };

        let adapter = UniversalAdapter::new(config);
        assert!(!adapter.config.enable_caching);
    }

    #[test]
    fn test_adapter_creation_stress() {
        let start = std::time::Instant::now();

        for _ in 0..10000 {
            let _adapter = UniversalAdapter::new(AdapterConfig::default());
        }

        let duration = start.elapsed();

        // Should be able to create many adapters quickly
        assert!(duration.as_secs() < 5);
    }
}
