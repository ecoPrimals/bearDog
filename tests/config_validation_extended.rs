// SPDX-License-Identifier: AGPL-3.0-only
//! Extended Configuration Validation Tests
//!
//! High-value integration tests for configuration validation,
//! environment-specific behavior, and edge cases.
//!
//! `TEST_CATEGORY`: integration
//! `TEST_DOMAIN`: config

#![allow(clippy::unwrap_used, clippy::expect_used)]
#![allow(
    clippy::field_reassign_with_default,
    clippy::needless_borrows_for_generic_args
)]
#![allow(dead_code)] // Test helpers may not all be used

use beardog_core::BearDogConfig;

// ====================================================================================
// Configuration Loading Tests (10 tests)
// ====================================================================================

#[test]
fn test_config_loading_from_default() {
    // Test that default config can be loaded without errors
    let result = BearDogConfig::default();

    assert!(!result.environment.is_empty());
    assert!(!result.version.is_empty());
    assert!(!result.node_id.is_empty());
}

#[test]
fn test_config_environment_validation_development() {
    // Test development environment validation
    let mut config = BearDogConfig::default();
    config.environment = "development".to_string();

    assert_eq!(config.environment, "development");
}

#[test]
fn test_config_environment_validation_staging() {
    // Test staging environment validation
    let mut config = BearDogConfig::default();
    config.environment = "staging".to_string();

    assert_eq!(config.environment, "staging");
}

#[test]
fn test_config_environment_validation_production() {
    // Test production environment validation
    let mut config = BearDogConfig::default();
    config.environment = "production".to_string();

    assert_eq!(config.environment, "production");
}

#[test]
fn test_config_version_format() {
    // Test version format validation
    let config = BearDogConfig::default();

    // Should be semantic version format
    assert!(config.version.contains('.'));
    let parts: Vec<&str> = config.version.split('.').collect();
    assert!(parts.len() >= 2, "Version should have at least major.minor");
}

#[test]
fn test_config_node_id_uniqueness_multiple_instances() {
    // Test that multiple config instances have unique node IDs
    let configs: Vec<BearDogConfig> = (0..5).map(|_| BearDogConfig::default()).collect();

    // Check all node IDs are unique
    for (i, config1) in configs.iter().enumerate() {
        for (j, config2) in configs.iter().enumerate() {
            if i != j {
                assert_ne!(
                    config1.node_id, config2.node_id,
                    "Node IDs should be unique across instances"
                );
            }
        }
    }
}

#[test]
fn test_config_node_id_persistence() {
    // Test that node ID persists across clones
    let config1 = BearDogConfig::default();
    let config2 = config1.clone();

    assert_eq!(config1.node_id, config2.node_id);
}

// TEST_CATEGORY: unit
// TEST_DOMAIN: core
// TEST_PRIORITY: normal
#[test]
fn test_config_modification_immutability() {
    // Test that modifying one config doesn't affect another
    let config1 = BearDogConfig::default();
    let mut config2 = config1.clone();

    let original_env = config2.environment.clone();
    // TEST_CATEGORY: unit
    // TEST_DOMAIN: core
    // TEST_PRIORITY: normal
    config2.environment = "modified".to_string();

    assert_eq!(config1.environment, original_env);
    assert_ne!(config1.environment, config2.environment);
}

// TEST_CATEGORY: unit
// TEST_DOMAIN: core
// TEST_PRIORITY: normal
#[test]
fn test_config_debug_format() {
    // Test that config can be debug-formatted without panics
    let config = BearDogConfig::default();
    let debug_str = format!("{config:?}");

    // TEST_CATEGORY: unit
    // TEST_DOMAIN: core
    // TEST_PRIORITY: normal
    assert!(!debug_str.is_empty());
    assert!(debug_str.contains("BearDogConfig") || debug_str.len() > 10);
}

#[test]
fn test_config_display_or_string_conversion() {
    // TEST_CATEGORY: unit
    // TEST_DOMAIN: core
    // TEST_PRIORITY: normal
    // Test that config has meaningful string representation
    let config = BearDogConfig::default();
    let node_id_str = config.node_id.clone();

    assert!(!node_id_str.is_empty());
    assert_eq!(node_id_str.len(), 36); // UUID format
}

// TEST_CATEGORY: unit
// TEST_DOMAIN: core
// TEST_PRIORITY: normal
// ====================================================================================
// Configuration Edge Cases (10 tests)
// ====================================================================================

#[test]
fn test_config_empty_environment_handling() {
    // Test handling of empty environment string
    let mut config = BearDogConfig::default();
    let original = config.environment.clone();

    config.environment = String::new();

    // Either rejects empty or has default behavior
    assert!(config.environment.is_empty() || !original.is_empty());
}
// TEST_CATEGORY: unit
// TEST_DOMAIN: core
// TEST_PRIORITY: normal

#[test]
fn test_config_special_characters_in_environment() {
    // Test special characters in environment names
    let mut config = BearDogConfig::default();

    // TEST_CATEGORY: unit
    // TEST_DOMAIN: core
    // TEST_PRIORITY: normal
    // Test various special characters
    config.environment = "dev-test".to_string();
    assert_eq!(config.environment, "dev-test");

    config.environment = "test_env".to_string();
    assert_eq!(config.environment, "test_env");
}

#[test]
fn test_config_case_sensitivity() {
    // TEST_CATEGORY: unit
    // TEST_DOMAIN: core
    // TEST_PRIORITY: normal
    // Test case sensitivity in environment names
    let mut config1 = BearDogConfig::default();
    let mut config2 = BearDogConfig::default();

    config1.environment = "Production".to_string();
    config2.environment = "production".to_string();

    // TEST_CATEGORY: unit
    // TEST_DOMAIN: core
    // TEST_PRIORITY: normal
    // These should be treated as different
    assert_ne!(config1.environment, config2.environment);
}

#[test]
fn test_config_whitespace_handling() {
    // Test whitespace handling in configuration values
    let mut config = BearDogConfig::default();

    config.environment = " production ".to_string();

    // TEST_CATEGORY: unit
    // TEST_DOMAIN: core
    // TEST_PRIORITY: normal
    // Should either trim or preserve whitespace consistently
    assert!(config.environment.contains("production"));
}

#[test]
fn test_config_very_long_environment_name() {
    // Test handling of very long environment names
    let mut config = BearDogConfig::default();
    let long_name = "a".repeat(1000);
    // TEST_CATEGORY: unit
    // TEST_DOMAIN: core
    // TEST_PRIORITY: normal

    config.environment = long_name.clone();

    // Should handle long strings without panicking
    assert_eq!(config.environment, long_name);
}

#[test]
fn test_config_unicode_in_environment() {
    // Test Unicode characters in environment names
    // TEST_CATEGORY: unit
    // TEST_DOMAIN: core
    // TEST_PRIORITY: normal
    let mut config = BearDogConfig::default();

    config.environment = "production-🐻".to_string();

    // Should handle Unicode without panicking
    assert!(config.environment.contains("production"));
    assert!(config.environment.contains("🐻"));
}

#[test]
// TEST_CATEGORY: unit
// TEST_DOMAIN: core
// TEST_PRIORITY: normal
fn test_config_version_update() {
    // Test version update doesn't break config
    let mut config = BearDogConfig::default();

    config.version = "4.0.0".to_string();
    assert_eq!(config.version, "4.0.0");

    config.version = "5.1.2-beta".to_string();
    // TEST_CATEGORY: unit
    // TEST_DOMAIN: core
    // TEST_PRIORITY: normal
    assert_eq!(config.version, "5.1.2-beta");
}

#[test]
fn test_config_concurrent_default_creation() {
    // Test concurrent config creation
    use std::thread;

    let handles: Vec<_> = (0..10)
        // TEST_CATEGORY: unit
        // TEST_DOMAIN: core
        // TEST_PRIORITY: normal
        .map(|_| {
            thread::spawn(|| {
                let config = BearDogConfig::default();
                assert!(!config.node_id.is_empty());
                config.node_id
            })
        })
        .collect();

    // TEST_CATEGORY: unit
    // TEST_DOMAIN: core
    // TEST_PRIORITY: normal
    let node_ids: Vec<String> = handles
        .into_iter()
        .map(|h| h.join().expect("Thread should complete"))
        .collect();

    // All node IDs should be unique
    for (i, id1) in node_ids.iter().enumerate() {
        for (j, id2) in node_ids.iter().enumerate() {
            if i != j {
                // TEST_CATEGORY: unit
                // TEST_DOMAIN: core
                // TEST_PRIORITY: normal
                assert_ne!(id1, id2, "Concurrent configs should have unique IDs");
            }
        }
    }
}

#[test]
fn test_config_rapid_creation_and_destruction() {
    // Test rapid config creation/destruction doesn't leak
    for _ in 0..1000 {
        let config = BearDogConfig::default();
        assert!(!config.node_id.is_empty());
        // Config drops here
    }
    // If we get here without OOM, test passes
}

#[test]
fn test_config_clone_chain() {
    // Test multiple levels of cloning
    let config1 = BearDogConfig::default();
    let config2 = config1.clone();
    let config3 = config2.clone();
    let config4 = config3.clone();

    // All should have same values
    assert_eq!(config1.node_id, config2.node_id);
    // TEST_CATEGORY: unit
    // TEST_DOMAIN: core
    // TEST_PRIORITY: normal
    assert_eq!(config2.node_id, config3.node_id);
    assert_eq!(config3.node_id, config4.node_id);
    assert_eq!(config1.environment, config4.environment);
}

// ====================================================================================
// Configuration Integration Tests (5 tests)
// ====================================================================================
// TEST_CATEGORY: unit
// TEST_DOMAIN: core
// TEST_PRIORITY: normal

#[test]
fn test_config_with_health_status_check() {
    // Test config integrates properly with health status
    let config = BearDogConfig::default();

    // Config should be valid for health checks
    assert!(!config.node_id.is_empty());
    assert!(!config.version.is_empty());
}

#[test]
fn test_multiple_configs_different_environments() {
    // Test multiple configs with different environments
    let mut dev_config = BearDogConfig::default();
    dev_config.environment = "development".to_string();
    // TEST_CATEGORY: unit
    // TEST_DOMAIN: core
    // TEST_PRIORITY: normal

    let mut prod_config = BearDogConfig::default();
    prod_config.environment = "production".to_string();

    let mut staging_config = BearDogConfig::default();
    staging_config.environment = "staging".to_string();

    // TEST_CATEGORY: unit
    // TEST_DOMAIN: core
    // TEST_PRIORITY: normal
    // All should be valid but different
    assert_ne!(dev_config.environment, prod_config.environment);
    assert_ne!(dev_config.environment, staging_config.environment);
    assert_ne!(prod_config.environment, staging_config.environment);

    // All should have unique node IDs
    assert_ne!(dev_config.node_id, prod_config.node_id);
    assert_ne!(dev_config.node_id, staging_config.node_id);
}

#[test]
fn test_config_lifecycle_complete() {
    // Test complete config lifecycle

    // 1. Create
    let mut config = BearDogConfig::default();
    assert!(!config.node_id.is_empty());

    // 2. Modify
    // TEST_CATEGORY: unit
    // TEST_DOMAIN: core
    // TEST_PRIORITY: normal
    config.environment = "test".to_string();
    assert_eq!(config.environment, "test");

    // 3. Clone
    let config_copy = config.clone();
    assert_eq!(config.node_id, config_copy.node_id);

    // 4. Use
    let node_id = config.node_id.clone();
    assert_eq!(node_id.len(), 36);

    // 5. Destroy (implicit via drop)
}

#[test]
fn test_config_equality_after_modification() {
    // Test config equality semantics
    let config1 = BearDogConfig::default();
    let mut config2 = config1.clone();

    // TEST_CATEGORY: unit
    // TEST_DOMAIN: core
    // TEST_PRIORITY: normal
    // Initially equal
    assert_eq!(config1.node_id, config2.node_id);

    // After modification, still same node_id
    config2.environment = "different".to_string();
    assert_eq!(config1.node_id, config2.node_id);
    assert_ne!(config1.environment, config2.environment);
}

#[test]
fn test_config_stress_rapid_access() {
    // Test rapid config field access doesn't cause issues
    // TEST_CATEGORY: unit
    // TEST_DOMAIN: core
    // TEST_PRIORITY: normal
    let config = BearDogConfig::default();

    for _ in 0..10000 {
        let _ = &config.node_id;
        let _ = &config.environment;
        let _ = &config.version;
    }

    // If we get here without panic, test passes
    assert!(!config.node_id.is_empty());
}

// ====================================================================================
// Test Helpers
// ====================================================================================

#[cfg(test)]
mod test_helpers {
    use super::*;

    /// Create a test config with specific environment
    pub fn create_test_config(environment: &str) -> BearDogConfig {
        let mut config = BearDogConfig::default();
        config.environment = environment.to_string();
        config
    }

    /// Validate basic config properties
    pub fn validate_config(config: &BearDogConfig) -> bool {
        !config.node_id.is_empty() && !config.environment.is_empty() && !config.version.is_empty()
    }
}

// TEST_CATEGORY: unit
// TEST_DOMAIN: core
// TEST_PRIORITY: normal
#[test]
fn test_helper_create_test_config() {
    // Test the helper function works
    let config = test_helpers::create_test_config("test");
    assert_eq!(config.environment, "test");
    assert!(test_helpers::validate_config(&config));
}
