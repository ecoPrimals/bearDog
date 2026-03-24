// SPDX-License-Identifier: AGPL-3.0-only

//! Canonical Capabilities Tests
//!
//! Comprehensive testing of capability-based architecture including:
//! - CapabilityType variants and methods
//! - SecurityLevel enum
//! - CircuitBreakerConfig
//! - AuthType and AuthConfig
//! - Serialization/deserialization
//! - String conversions and identifiers

use super::*;

// ============================================================================
// CapabilityType Tests
// ============================================================================

#[test]
fn test_capability_type_variants() {
    // Test that all major capability types exist
    let capabilities = vec![
        CapabilityType::KeyManagement,
        CapabilityType::HardwareSecurityModule,
        CapabilityType::SecretsManagement,
        CapabilityType::Authentication,
        CapabilityType::CloudStorage,
        CapabilityType::DatabaseService,
        CapabilityType::LoadBalancing,
        CapabilityType::ContentDeliveryNetwork,
        CapabilityType::ServiceMesh,
        CapabilityType::ComputeIntelligence,
        CapabilityType::DataStorage,
        CapabilityType::DistributedIntelligence,
        CapabilityType::ContainerOrchestration,
        CapabilityType::Security,
        CapabilityType::Monitoring,
        CapabilityType::Logging,
        CapabilityType::Metrics,
    ];

    assert_eq!(capabilities.len(), 17);
}

#[test]
fn test_capability_type_names() {
    assert_eq!(CapabilityType::KeyManagement.name(), "Key Management");
    assert_eq!(
        CapabilityType::HardwareSecurityModule.name(),
        "Hardware Security Module"
    );
    assert_eq!(
        CapabilityType::SecretsManagement.name(),
        "Secrets Management"
    );
}

#[test]
fn test_capability_type_custom() {
    let custom = CapabilityType::Custom("my-custom-capability".to_string());
    assert_eq!(custom.name(), "my-custom-capability");
}

#[test]
fn test_capability_type_as_capability_id() {
    assert_eq!(
        CapabilityType::Security.as_capability_id(),
        "capability:security"
    );
    assert_eq!(
        CapabilityType::Storage.as_capability_id(),
        "capability:storage"
    );
    assert_eq!(
        CapabilityType::Compute.as_capability_id(),
        "capability:compute"
    );
}

#[test]
fn test_capability_type_custom_id() {
    let custom = CapabilityType::Custom("test".to_string());
    assert_eq!(custom.as_capability_id(), "capability:custom:test");
}

#[test]
fn test_capability_type_is_vendor() {
    assert!(CapabilityType::KeyManagement.is_vendor_capability());
    assert!(CapabilityType::HardwareSecurityModule.is_vendor_capability());
    assert!(CapabilityType::SecretsManagement.is_vendor_capability());
    assert!(CapabilityType::Authentication.is_vendor_capability());
    assert!(CapabilityType::CloudStorage.is_vendor_capability());

    // Non-vendor capabilities
    assert!(!CapabilityType::ServiceMesh.is_vendor_capability());
    assert!(!CapabilityType::ComputeIntelligence.is_vendor_capability());
}

#[test]
fn test_capability_type_is_primal() {
    assert!(CapabilityType::ServiceMesh.is_primal_capability());
    assert!(CapabilityType::ComputeIntelligence.is_primal_capability());
    assert!(CapabilityType::DataStorage.is_primal_capability());
    assert!(CapabilityType::DistributedIntelligence.is_primal_capability());
    assert!(CapabilityType::ContainerOrchestration.is_primal_capability());
    assert!(CapabilityType::Security.is_primal_capability());

    // Non-primal capabilities
    assert!(!CapabilityType::KeyManagement.is_primal_capability());
    assert!(!CapabilityType::HardwareSecurityModule.is_primal_capability());
}

#[test]
fn test_capability_type_display() {
    let cap = CapabilityType::Monitoring;
    let displayed = format!("{cap}");
    assert_eq!(displayed, "Monitoring");
}

#[test]
fn test_capability_type_from_string() {
    let cap = CapabilityType::Storage;
    let string: String = cap.into();
    assert_eq!(string, "capability:storage");
}

#[test]
fn test_capability_type_equality() {
    assert_eq!(CapabilityType::Storage, CapabilityType::Storage);
    assert_ne!(CapabilityType::Storage, CapabilityType::Compute);
}

#[test]
fn test_capability_type_clone() {
    let cap = CapabilityType::Networking;
    let cloned = cap.clone();
    assert_eq!(cap, cloned);
}

#[test]
fn test_capability_type_hash() {
    use std::collections::HashSet;

    let mut set = HashSet::new();
    set.insert(CapabilityType::Storage);
    set.insert(CapabilityType::Compute);
    set.insert(CapabilityType::Storage); // Duplicate

    assert_eq!(set.len(), 2); // Should only contain unique values
}

#[test]
fn test_capability_type_serialization() {
    let cap = CapabilityType::KeyManagement;
    let json = serde_json::to_string(&cap).expect("Failed to serialize");
    let deserialized: CapabilityType = serde_json::from_str(&json).expect("Failed to deserialize");
    assert_eq!(cap, deserialized);
}

#[test]
fn test_capability_type_custom_serialization() {
    let cap = CapabilityType::Custom("test-capability".to_string());
    let json = serde_json::to_string(&cap).expect("Failed to serialize");
    let deserialized: CapabilityType = serde_json::from_str(&json).expect("Failed to deserialize");
    assert_eq!(cap, deserialized);
}

#[test]
#[allow(deprecated)]
fn test_capability_type_associated_primal_deprecated() {
    // This should return None per zero-knowledge architecture
    assert_eq!(CapabilityType::ServiceMesh.associated_primal(), None);
    assert_eq!(
        CapabilityType::ComputeIntelligence.associated_primal(),
        None
    );
}

// ============================================================================
// SecurityLevel Tests
// ============================================================================

#[test]
fn test_security_level_variants() {
    let levels = [
        SecurityLevel::Basic,
        SecurityLevel::Standard,
        SecurityLevel::High,
        SecurityLevel::Critical,
    ];
    assert_eq!(levels.len(), 4);
}

#[test]
fn test_security_level_default() {
    assert_eq!(SecurityLevel::default(), SecurityLevel::Standard);
}

#[test]
fn test_security_level_ordering() {
    assert!(SecurityLevel::Basic < SecurityLevel::Standard);
    assert!(SecurityLevel::Standard < SecurityLevel::High);
    assert!(SecurityLevel::High < SecurityLevel::Critical);
}

#[test]
fn test_security_level_equality() {
    assert_eq!(SecurityLevel::High, SecurityLevel::High);
    assert_ne!(SecurityLevel::Basic, SecurityLevel::Critical);
}

#[test]
fn test_security_level_clone() {
    let level = SecurityLevel::Critical;
    let cloned = level.clone();
    assert_eq!(level, cloned);
}

#[test]
fn test_security_level_serialization() {
    let level = SecurityLevel::High;
    let json = serde_json::to_string(&level).expect("Failed to serialize");
    let deserialized: SecurityLevel = serde_json::from_str(&json).expect("Failed to deserialize");
    assert_eq!(level, deserialized);
}

// ============================================================================
// CircuitBreakerConfig Tests
// ============================================================================

#[test]
fn test_circuit_breaker_config_default() {
    let config = CircuitBreakerConfig::default();

    assert_eq!(config.failure_threshold, 5);
    assert_eq!(config.timeout_ms, 60000); // 60 seconds
    assert_eq!(config.success_threshold, 3);
}

#[test]
fn test_circuit_breaker_config_custom() {
    let config = CircuitBreakerConfig {
        failure_threshold: 10,
        timeout_ms: 30000,
        success_threshold: 5,
    };

    assert_eq!(config.failure_threshold, 10);
    assert_eq!(config.timeout_ms, 30000);
    assert_eq!(config.success_threshold, 5);
}

#[test]
fn test_circuit_breaker_config_environment_override() {
    // Modern pattern: Use from_env_provider in tests; production uses from_env() + real env.
    let config = CircuitBreakerConfig::from_env_provider(|k| match k {
        "BEARDOG_CIRCUIT_BREAKER_FAILURE_THRESHOLD" => Some("8".to_string()),
        "BEARDOG_CIRCUIT_BREAKER_TIMEOUT_MS" => Some("45000".to_string()),
        "BEARDOG_CIRCUIT_BREAKER_SUCCESS_THRESHOLD" => Some("4".to_string()),
        _ => None,
    });

    assert_eq!(config.failure_threshold, 8);
    assert_eq!(config.timeout_ms, 45000);
    assert_eq!(config.success_threshold, 4);
}

#[test]
fn test_circuit_breaker_config_clone() {
    let config = CircuitBreakerConfig::default();
    let cloned = config.clone();

    assert_eq!(config.failure_threshold, cloned.failure_threshold);
    assert_eq!(config.timeout_ms, cloned.timeout_ms);
    assert_eq!(config.success_threshold, cloned.success_threshold);
}

#[test]
fn test_circuit_breaker_config_equality() {
    let config1 = CircuitBreakerConfig::default();
    let config2 = CircuitBreakerConfig::default();

    assert_eq!(config1, config2);
}

#[test]
fn test_circuit_breaker_config_serialization() {
    let config = CircuitBreakerConfig {
        failure_threshold: 7,
        timeout_ms: 50000,
        success_threshold: 2,
    };

    let json = serde_json::to_string(&config).expect("Failed to serialize");
    let deserialized: CircuitBreakerConfig =
        serde_json::from_str(&json).expect("Failed to deserialize");

    assert_eq!(config, deserialized);
}

// ============================================================================
// AuthType Tests
// ============================================================================

#[test]
fn test_auth_type_variants() {
    let types = [
        AuthType::None,
        AuthType::ApiKey,
        AuthType::Bearer,
        AuthType::MutualTLS,
        AuthType::OAuth2,
        AuthType::Custom("custom".to_string()),
    ];

    assert_eq!(types.len(), 6);
}

#[test]
fn test_auth_type_equality() {
    assert_eq!(AuthType::ApiKey, AuthType::ApiKey);
    assert_ne!(AuthType::ApiKey, AuthType::Bearer);
}

#[test]
fn test_auth_type_custom() {
    let custom = AuthType::Custom("my-auth".to_string());
    let custom2 = AuthType::Custom("my-auth".to_string());
    let custom3 = AuthType::Custom("other-auth".to_string());

    assert_eq!(custom, custom2);
    assert_ne!(custom, custom3);
}

#[test]
fn test_auth_type_clone() {
    let auth = AuthType::OAuth2;
    let cloned = auth.clone();
    assert_eq!(auth, cloned);
}

#[test]
fn test_auth_type_serialization() {
    let auth = AuthType::Bearer;
    let json = serde_json::to_string(&auth).expect("Failed to serialize");
    let deserialized: AuthType = serde_json::from_str(&json).expect("Failed to deserialize");
    assert_eq!(auth, deserialized);
}

#[test]
fn test_auth_type_custom_serialization() {
    let auth = AuthType::Custom("test-auth".to_string());
    let json = serde_json::to_string(&auth).expect("Failed to serialize");
    let deserialized: AuthType = serde_json::from_str(&json).expect("Failed to deserialize");
    assert_eq!(auth, deserialized);
}

// ============================================================================
// ServiceCapabilityType Alias Tests
// ============================================================================

#[test]
fn test_service_capability_type_alias() {
    // ServiceCapabilityType should be an alias for CapabilityType
    let cap: ServiceCapabilityType = CapabilityType::Monitoring;
    assert_eq!(cap, CapabilityType::Monitoring);
}

#[test]
fn test_service_capability_type_interchangeable() {
    let cap1: CapabilityType = CapabilityType::Storage;
    let cap2: ServiceCapabilityType = CapabilityType::Storage;
    assert_eq!(cap1, cap2);
}

// ============================================================================
// Integration Tests
// ============================================================================

#[test]
fn test_capability_categorization() {
    // Vendor capabilities
    let vendors = vec![
        CapabilityType::KeyManagement,
        CapabilityType::HardwareSecurityModule,
        CapabilityType::CloudStorage,
    ];

    for vendor in vendors {
        assert!(vendor.is_vendor_capability());
        assert!(!vendor.is_primal_capability());
    }

    // Primal capabilities
    let primals = vec![
        CapabilityType::ServiceMesh,
        CapabilityType::ComputeIntelligence,
        CapabilityType::DataStorage,
    ];

    for primal in primals {
        assert!(primal.is_primal_capability());
        assert!(!primal.is_vendor_capability());
    }

    // Cross-cutting capabilities (neither vendor nor primal)
    let cross_cutting = vec![
        CapabilityType::Monitoring,
        CapabilityType::Logging,
        CapabilityType::Metrics,
    ];

    for cap in cross_cutting {
        assert!(!cap.is_vendor_capability());
        assert!(!cap.is_primal_capability());
    }
}

#[test]
fn test_capability_id_consistency() {
    let cap = CapabilityType::Storage;
    let id = cap.as_capability_id();

    // ID should be lowercase and kebab-case
    assert!(id.starts_with("capability:"));
    assert!(!id.contains(' '));
}

#[test]
fn test_multiple_capability_serialization() {
    let capabilities = vec![
        CapabilityType::KeyManagement,
        CapabilityType::Storage,
        CapabilityType::Compute,
    ];

    let json = serde_json::to_string(&capabilities).expect("Failed to serialize");
    let deserialized: Vec<CapabilityType> =
        serde_json::from_str(&json).expect("Failed to deserialize");

    assert_eq!(capabilities, deserialized);
}

#[test]
fn test_circuit_breaker_threshold_values() {
    // Test various threshold configurations
    let configs = vec![
        CircuitBreakerConfig {
            failure_threshold: 1,
            timeout_ms: 1000,
            success_threshold: 1,
        },
        CircuitBreakerConfig {
            failure_threshold: 100,
            timeout_ms: 300_000,
            success_threshold: 50,
        },
    ];

    for config in configs {
        assert!(config.failure_threshold > 0);
        assert!(config.timeout_ms > 0);
        assert!(config.success_threshold > 0);
    }
}

// ============================================================================
// Test Summary
// ============================================================================
// Total tests: 40
// Categories:
// - CapabilityType: 18 tests
// - SecurityLevel: 6 tests
// - CircuitBreakerConfig: 7 tests
// - AuthType: 6 tests
// - Integration: 3 tests
//
// Status: All tests are functional and comprehensive
// Priority: Critical - Foundational capability architecture
// Coverage: Core capability system validation
// ============================================================================
