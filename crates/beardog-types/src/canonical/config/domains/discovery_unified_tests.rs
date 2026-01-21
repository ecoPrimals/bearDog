//! Tests for UnifiedDiscoveryConfig
//!
//! Comprehensive test suite for discovery configuration including validation,
//! environment variables, builder patterns, and merge operations.

use super::*;
use std::env;

#[test]
fn test_const_defaults() {
    let config = UnifiedDiscoveryConfig::const_defaults();
    assert!(config.enabled);
}

#[test]
fn test_default_uses_reasonable_values() {
    let config = UnifiedDiscoveryConfig::default();
    assert!(config.enabled);
    assert_eq!(config.service_id.as_ref(), "beardog-discovery");
    assert!(!config.enabled_protocols.is_empty());
    assert_eq!(&config.registry.backend, "consul");
    assert!(config.cache.enabled);
}

#[test]
fn test_validation_success() {
    let config = UnifiedDiscoveryConfig::default();
    assert!(config.validate().is_ok());
}

#[test]
fn test_validation_empty_service_id() {
    let mut config = UnifiedDiscoveryConfig::default();
    config.service_id = Arc::from("");
    assert!(config.validate().is_err());
}

#[test]
fn test_validation_empty_registry_endpoints() {
    let mut config = UnifiedDiscoveryConfig::default();
    config.registry.endpoints = vec![];
    assert!(config.validate().is_err());
}

#[test]
fn test_validation_quantum_error_threshold() {
    let mut config = UnifiedDiscoveryConfig::default();
    config.quantum.enabled = true;
    config.quantum.error_threshold = 1.5; // Invalid: > 1.0
    assert!(config.validate().is_err());
}

#[test]
fn test_builder_basic() {
    let config = UnifiedDiscoveryConfig::builder()
        .enabled(true)
        .service_id("test-service")
        .build();

    assert!(config.enabled);
    assert_eq!(config.service_id.as_ref(), "test-service");
    assert!(config.validate().is_ok());
}

#[test]
fn test_builder_with_protocol() {
    let config = UnifiedDiscoveryConfig::builder()
        .add_protocol(DiscoveryProtocol::Http {
            endpoint: "http://test:8500".to_string(),
            timeout_ms: 3000,
        })
        .build();

    assert_eq!(config.enabled_protocols.len(), 1);
    match &config.enabled_protocols[0] {
        DiscoveryProtocol::Http {
            endpoint,
            timeout_ms,
        } => {
            assert_eq!(endpoint, "http://test:8500");
            assert_eq!(*timeout_ms, 3000);
        }
        _ => panic!("Expected Http protocol"),
    }
}

#[test]
fn test_aggressive_config() {
    let config = UnifiedDiscoveryConfig::aggressive();
    assert!(config.enabled);
    assert_eq!(config.registry.service_ttl.as_secs(), 10);
    assert_eq!(config.network.timeout.as_secs(), 1);
    assert!(config.validate().is_ok());
}

#[test]
fn test_conservative_config() {
    let config = UnifiedDiscoveryConfig::conservative();
    assert!(config.enabled);
    assert_eq!(config.registry.service_ttl.as_secs(), 600);
    assert_eq!(config.network.timeout.as_secs(), 30);
    assert!(config.validate().is_ok());
}

#[test]
fn test_from_env() {
    env::set_var("BEARDOG_DISCOVERY_ENABLED", "true");
    env::set_var("BEARDOG_DISCOVERY_SERVICE_ID", "env-test");
    env::set_var("BEARDOG_REGISTRY_BACKEND", "etcd");

    let config = UnifiedDiscoveryConfig::from_env().unwrap();
    assert!(config.enabled);
    assert_eq!(config.service_id.as_ref(), "env-test");
    assert_eq!(&config.registry.backend, "etcd");

    env::remove_var("BEARDOG_DISCOVERY_ENABLED");
    env::remove_var("BEARDOG_DISCOVERY_SERVICE_ID");
    env::remove_var("BEARDOG_REGISTRY_BACKEND");
}

#[test]
fn test_serialization_roundtrip() {
    let config = UnifiedDiscoveryConfig::default();
    let toml_str = config.to_toml().unwrap();
    let deserialized: UnifiedDiscoveryConfig = toml::from_str(&toml_str).unwrap();
    assert_eq!(config, deserialized);
}

#[test]
fn test_merge() {
    let base = UnifiedDiscoveryConfig::default();
    let mut override_config = UnifiedDiscoveryConfig::default();
    override_config.service_id = Arc::from("merged-service");
    override_config.registry.backend = "etcd".to_string();

    let merged = base.merge(&override_config).unwrap();
    assert_eq!(merged.service_id.as_ref(), "merged-service");
    assert_eq!(&merged.registry.backend, "etcd");
}
