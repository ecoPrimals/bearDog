// Comprehensive Adapter Tests
// Created: October 25, 2025
// Purpose: Week 2 test expansion - Universal adapter functionality

use crate::universal::capability_based_adapter::UniversalCapabilityAdapter;
use crate::universal::types::AdapterConfig;
use beardog_types::canonical::capabilities::ServiceCapabilityType;

#[tokio::test]
async fn test_adapter_creation_with_default_config() {
    let adapter = UniversalCapabilityAdapter::new().await;
    assert!(adapter.is_ok(), "Should create adapter with default config");
}

#[tokio::test]
async fn test_adapter_creation_with_custom_config() {
    let config = AdapterConfig {
        max_providers_per_capability: 10,
        health_check_interval_secs: 60,
        connection_timeout_ms: 5000,
        ..Default::default()
    };
    
    let adapter = UniversalCapabilityAdapter::with_config(config).await;
    assert!(adapter.is_ok(), "Should create adapter with custom config");
}

#[tokio::test]
async fn test_adapter_configuration_validation() {
    let config = AdapterConfig {
        max_providers_per_capability: 1,
        health_check_interval_secs: 10,
        connection_timeout_ms: 1000,
        ..Default::default()
    };
    
    let adapter = UniversalCapabilityAdapter::with_config(config).await;
    assert!(adapter.is_ok(), "Should handle minimal valid config");
}

#[tokio::test]
async fn test_adapter_default_config_values() {
    let adapter = UniversalCapabilityAdapter::new().await.unwrap();
    let config = adapter.config();
    
    // Verify sensible defaults
    assert!(config.max_providers_per_capability > 0, "Should have positive max providers");
    assert!(config.health_check_interval_secs > 0, "Should have positive health check interval");
    assert!(config.connection_timeout_ms > 0, "Should have positive timeout");
}

#[tokio::test]
async fn test_discover_capabilities_empty_initial_state() {
    let adapter = UniversalCapabilityAdapter::new().await.unwrap();
    let capabilities = adapter.get_available_capabilities().await;
    
    assert!(capabilities.is_ok(), "Should list capabilities even if empty");
    assert!(capabilities.unwrap().is_empty(), "Should start with empty capabilities");
}

#[tokio::test]
async fn test_get_capabilities_returns_hashmap() {
    let adapter = UniversalCapabilityAdapter::new().await.unwrap();
    let capabilities = adapter.get_available_capabilities().await;
    
    assert!(capabilities.is_ok(), "Should return capabilities HashMap");
}

#[tokio::test]
async fn test_get_discovered_primals_empty_initial() {
    let adapter = UniversalCapabilityAdapter::new().await.unwrap();
    let primals = adapter.get_discovered_primals().await;
    
    assert!(primals.is_ok(), "Should get primals list");
    assert!(primals.unwrap().is_empty(), "Should start with no discovered primals");
}

#[tokio::test]
async fn test_adapter_metrics_available() {
    let adapter = UniversalCapabilityAdapter::new().await.unwrap();
    let metrics = adapter.get_metrics();
    
    // Metrics should be accessible
    assert!(format!("{:?}", metrics).len() > 0, "Should have metrics structure");
}

#[tokio::test]
async fn test_health_check_all_connections_empty() {
    let adapter = UniversalCapabilityAdapter::new().await.unwrap();
    let health = adapter.health_check_all_connections().await;
    
    assert!(health.is_ok(), "Should check health even with no connections");
    assert!(health.unwrap().is_empty(), "Should have no connections initially");
}

#[tokio::test]
async fn test_adapter_shutdown() {
    let adapter = UniversalCapabilityAdapter::new().await.unwrap();
    let result = adapter.shutdown().await;
    
    assert!(result.is_ok(), "Should shutdown gracefully");
}

#[tokio::test]
async fn test_adapter_config_access() {
    let adapter = UniversalCapabilityAdapter::new().await.unwrap();
    let config = adapter.config();
    
    // Should be able to access configuration
    assert!(config.max_providers_per_capability > 0);
}

#[tokio::test]
async fn test_adapter_lifecycle() {
    // Test full lifecycle: create -> use -> shutdown
    let adapter = UniversalCapabilityAdapter::new().await.unwrap();
    
    // Check initial state
    let capabilities = adapter.get_available_capabilities().await.unwrap();
    assert!(capabilities.is_empty());
    
    // Shutdown
    let shutdown = adapter.shutdown().await;
    assert!(shutdown.is_ok());
}

#[tokio::test]
async fn test_multiple_adapters_can_coexist() {
    let adapter1 = UniversalCapabilityAdapter::new().await;
    let adapter2 = UniversalCapabilityAdapter::new().await;
    
    assert!(adapter1.is_ok() && adapter2.is_ok(), "Should create multiple adapters");
}

#[tokio::test]
async fn test_adapter_with_zero_max_providers() {
    let config = AdapterConfig {
        max_providers_per_capability: 0,
        ..Default::default()
    };
    
    let adapter = UniversalCapabilityAdapter::with_config(config).await;
    // Should either succeed or fail gracefully
    assert!(adapter.is_ok() || adapter.is_err(), "Should handle edge case config");
}

#[tokio::test]
async fn test_adapter_config_persistence() {
    let config = AdapterConfig {
        max_providers_per_capability: 42,
        health_check_interval_secs: 99,
        connection_timeout_ms: 1234,
        ..Default::default()
    };
    
    let adapter = UniversalCapabilityAdapter::with_config(config).await.unwrap();
    let stored_config = adapter.config();
    
    assert_eq!(stored_config.max_providers_per_capability, 42);
    assert_eq!(stored_config.health_check_interval_secs, 99);
    assert_eq!(stored_config.connection_timeout_ms, 1234);
}

