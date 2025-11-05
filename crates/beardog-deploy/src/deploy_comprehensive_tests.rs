//! Comprehensive Unit Tests for BearDog Deploy
//!
//! Created: October 27, 2025
//! Purpose: Expand test coverage for beardog-deploy core functionality
//!
//! Tests cover:
//! - DeploymentConfig creation and defaults
//! - DeploymentManager lifecycle
//! - Configuration validation
//! - Serialization/deserialization
//! - Edge cases and error scenarios

use super::*;

// ============================================================================
// DeploymentConfig Tests
// ============================================================================

#[test]
fn test_deployment_config_default() {
    let config = DeploymentConfig::default();

    assert_eq!(config.environment, "development");
    assert_eq!(config.region, "local");
    assert_eq!(config.instance_count, 1);
    assert!(config.monitoring_enabled);
}

#[test]
fn test_deployment_config_custom() {
    let config = DeploymentConfig {
        environment: "production".to_string(),
        region: "us-east-1".to_string(),
        instance_count: 10,
        monitoring_enabled: false,
    };

    assert_eq!(config.environment, "production");
    assert_eq!(config.region, "us-east-1");
    assert_eq!(config.instance_count, 10);
    assert!(!config.monitoring_enabled);
}

#[test]
fn test_deployment_config_staging() {
    let config = DeploymentConfig {
        environment: "staging".to_string(),
        region: "us-west-2".to_string(),
        instance_count: 3,
        monitoring_enabled: true,
    };

    assert_eq!(config.environment, "staging");
    assert_eq!(config.instance_count, 3);
}

#[test]
fn test_deployment_config_clone() {
    let config1 = DeploymentConfig::default();
    let config2 = config1.clone();

    assert_eq!(config1.environment, config2.environment);
    assert_eq!(config1.region, config2.region);
    assert_eq!(config1.instance_count, config2.instance_count);
    assert_eq!(config1.monitoring_enabled, config2.monitoring_enabled);
}

#[test]
fn test_deployment_config_debug_format() {
    let config = DeploymentConfig::default();
    let debug_str = format!("{:?}", config);

    assert!(debug_str.contains("DeploymentConfig"));
    assert!(debug_str.contains("development"));
    assert!(debug_str.contains("local"));
}

#[test]
fn test_deployment_config_serialization() {
    let config = DeploymentConfig {
        environment: "test".to_string(),
        region: "eu-west-1".to_string(),
        instance_count: 5,
        monitoring_enabled: true,
    };

    let serialized = serde_json::to_string(&config).expect("Serialization should succeed");
    let deserialized: DeploymentConfig =
        serde_json::from_str(&serialized).expect("Deserialization should succeed");

    assert_eq!(config.environment, deserialized.environment);
    assert_eq!(config.region, deserialized.region);
    assert_eq!(config.instance_count, deserialized.instance_count);
    assert_eq!(config.monitoring_enabled, deserialized.monitoring_enabled);
}

#[test]
fn test_deployment_config_empty_environment() {
    let config = DeploymentConfig {
        environment: "".to_string(),
        region: "local".to_string(),
        instance_count: 1,
        monitoring_enabled: true,
    };

    assert!(config.environment.is_empty());
}

#[test]
fn test_deployment_config_zero_instances() {
    let config = DeploymentConfig {
        // TEST_CATEGORY: unit
        // TEST_DOMAIN: core
        // TEST_PRIORITY: normal
        environment: "test".to_string(),
        region: "local".to_string(),
        instance_count: 0,
        monitoring_enabled: true,
    };

    assert_eq!(config.instance_count, 0);
    // TEST_CATEGORY: unit
    // TEST_DOMAIN: core
    // TEST_PRIORITY: normal
}

#[test]
fn test_deployment_config_large_instance_count() {
    let config = DeploymentConfig {
        environment: "production".to_string(),
        region: "global".to_string(),
        instance_count: 1000,
        monitoring_enabled: true,
    };

    assert_eq!(config.instance_count, 1000);
    // TEST_CATEGORY: unit
    // TEST_DOMAIN: core
    // TEST_PRIORITY: normal
}

#[test]
fn test_deployment_config_various_environments() {
    let environments = vec!["dev", "test", "staging", "preprod", "production"];

    for env in environments {
        let config = DeploymentConfig {
            environment: env.to_string(),
            region: "local".to_string(),
            // TEST_CATEGORY: unit
            // TEST_DOMAIN: core
            // TEST_PRIORITY: normal
            instance_count: 1,
            monitoring_enabled: true,
        };
        assert_eq!(config.environment, env);
    }
}

#[test]
// TEST_CATEGORY: unit
// TEST_DOMAIN: core
// TEST_PRIORITY: normal
fn test_deployment_config_various_regions() {
    let regions = vec!["us-east-1", "us-west-2", "eu-west-1", "ap-southeast-1"];

    for region in regions {
        let config = DeploymentConfig {
            environment: "production".to_string(),
            region: region.to_string(),
            // TEST_CATEGORY: unit
            // TEST_DOMAIN: core
            // TEST_PRIORITY: normal
            instance_count: 1,
            monitoring_enabled: true,
        };
        assert_eq!(config.region, region);
    }
}

#[test]
fn test_deployment_config_monitoring_disabled() {
    let config = DeploymentConfig {
        environment: "test".to_string(),
        region: "local".to_string(),
        instance_count: 1,
        monitoring_enabled: false,
    };

    // TEST_CATEGORY: unit
    // TEST_DOMAIN: core
    // TEST_PRIORITY: normal
    assert!(!config.monitoring_enabled);
}

// ============================================================================
// DeploymentManager Tests
// ============================================================================

#[test]
fn test_deployment_manager_new() {
    // TEST_CATEGORY: unit
    // TEST_DOMAIN: core
    // TEST_PRIORITY: normal
    let config = DeploymentConfig::default();
    let manager = DeploymentManager::new(config.clone());

    // Manager created successfully
    assert_eq!(manager.config.environment, config.environment);
}

#[test]
fn test_deployment_manager_with_custom_config() {
    // TEST_CATEGORY: unit
    // TEST_DOMAIN: core
    // TEST_PRIORITY: normal
    let config = DeploymentConfig {
        environment: "production".to_string(),
        region: "us-east-1".to_string(),
        instance_count: 5,
        monitoring_enabled: true,
    };

    let manager = DeploymentManager::new(config.clone());
    assert_eq!(manager.config.environment, "production");
    // TEST_CATEGORY: unit
    // TEST_DOMAIN: core
    // TEST_PRIORITY: normal
    assert_eq!(manager.config.instance_count, 5);
}

#[test]
fn test_deployment_manager_clone() {
    let config = DeploymentConfig::default();
    let manager1 = DeploymentManager::new(config);
    let manager2 = manager1.clone();

    assert_eq!(manager1.config.environment, manager2.config.environment);
    assert_eq!(manager1.config.region, manager2.config.region);
}
// TEST_CATEGORY: unit
// TEST_DOMAIN: core
// TEST_PRIORITY: normal

#[test]
fn test_deployment_manager_debug_format() {
    let config = DeploymentConfig::default();
    let manager = DeploymentManager::new(config);
    let debug_str = format!("{:?}", manager);

    assert!(debug_str.contains("DeploymentManager"));
}

// ============================================================================
// DeploymentManager Initialize Tests
// TEST_CATEGORY: unit
// TEST_DOMAIN: core
// TEST_PRIORITY: normal
// ============================================================================

#[test]
fn test_deployment_manager_initialize_success() {
    let config = DeploymentConfig::default();
    let manager = DeploymentManager::new(config);

    let result = manager.initialize();
    assert!(result.is_ok());
}

#[test]
fn test_deployment_manager_initialize_with_production_config() {
    // TEST_CATEGORY: unit
    // TEST_DOMAIN: core
    // TEST_PRIORITY: normal
    let config = DeploymentConfig {
        environment: "production".to_string(),
        region: "us-west-2".to_string(),
        instance_count: 10,
        monitoring_enabled: true,
    };
    // TEST_CATEGORY: unit
    // TEST_DOMAIN: core
    // TEST_PRIORITY: normal
    let manager = DeploymentManager::new(config);

    let result = manager.initialize();
    assert!(result.is_ok());
}

#[test]
fn test_deployment_manager_initialize_empty_environment_fails() {
    let config = DeploymentConfig {
        environment: "".to_string(),
        region: "local".to_string(),
        // TEST_CATEGORY: unit
        // TEST_DOMAIN: core
        // TEST_PRIORITY: normal
        instance_count: 1,
        monitoring_enabled: true,
    };
    let manager = DeploymentManager::new(config);

    let result = manager.initialize();
    assert!(result.is_err());
    // TEST_CATEGORY: unit
    // TEST_DOMAIN: core
    // TEST_PRIORITY: normal

    let error = result.unwrap_err();
    let error_msg = format!("{:?}", error);
    assert!(error_msg.contains("Environment") || error_msg.contains("empty"));
}

#[test]
fn test_deployment_manager_initialize_multiple_times() {
    let config = DeploymentConfig::default();
    let manager = DeploymentManager::new(config);
    // TEST_CATEGORY: unit
    // TEST_DOMAIN: core
    // TEST_PRIORITY: normal

    // Initialize multiple times should work
    assert!(manager.initialize().is_ok());
    assert!(manager.initialize().is_ok());
    assert!(manager.initialize().is_ok());
}
// TEST_CATEGORY: unit
// TEST_DOMAIN: core
// TEST_PRIORITY: normal

#[test]
fn test_deployment_manager_initialize_with_monitoring_disabled() {
    let config = DeploymentConfig {
        environment: "test".to_string(),
        region: "local".to_string(),
        instance_count: 1,
        monitoring_enabled: false,
    };
    let manager = DeploymentManager::new(config);

    // TEST_CATEGORY: unit
    // TEST_DOMAIN: core
    // TEST_PRIORITY: important
    let result = manager.initialize();
    assert!(result.is_ok());
}

#[test]
fn test_deployment_manager_initialize_with_zero_instances() {
    let config = DeploymentConfig {
        environment: "dev".to_string(),
        region: "local".to_string(),
        instance_count: 0,
        monitoring_enabled: true,
    };
    let manager = DeploymentManager::new(config);

    // Should still initialize (instance_count validation not implemented)
    // TEST_CATEGORY: unit
    // TEST_DOMAIN: core
    // TEST_PRIORITY: normal
    let result = manager.initialize();
    assert!(result.is_ok());
}

// ============================================================================
// Integration Tests
// ============================================================================

// TEST_CATEGORY: unit
// TEST_DOMAIN: core
// TEST_PRIORITY: normal
#[test]
fn test_full_deployment_lifecycle_development() {
    let config = DeploymentConfig::default();
    let manager = DeploymentManager::new(config);

    // Should initialize successfully
    assert!(manager.initialize().is_ok());
}

#[test]
fn test_full_deployment_lifecycle_production() {
    // TEST_CATEGORY: unit
    // TEST_DOMAIN: core
    // TEST_PRIORITY: normal
    let config = DeploymentConfig {
        environment: "production".to_string(),
        region: "us-east-1".to_string(),
        instance_count: 20,
        monitoring_enabled: true,
    };
    let manager = DeploymentManager::new(config);

    assert!(manager.initialize().is_ok());
}

#[test]
fn test_deployment_manager_with_different_configurations() {
    let configs = vec![
        DeploymentConfig {
            environment: "dev".to_string(),
            // TEST_CATEGORY: unit
            // TEST_DOMAIN: core
            // TEST_PRIORITY: normal
            region: "local".to_string(),
            instance_count: 1,
            monitoring_enabled: false,
        },
        DeploymentConfig {
            environment: "staging".to_string(),
            // TEST_CATEGORY: unit
            // TEST_DOMAIN: core
            // TEST_PRIORITY: normal
            region: "us-west-1".to_string(),
            instance_count: 3,
            monitoring_enabled: true,
        },
        DeploymentConfig {
            environment: "production".to_string(),
            region: "eu-central-1".to_string(),
            instance_count: 10,
            monitoring_enabled: true,
        },
        // TEST_CATEGORY: unit
        // TEST_DOMAIN: core
        // TEST_PRIORITY: normal
    ];

    for config in configs {
        let manager = DeploymentManager::new(config);
        assert!(manager.initialize().is_ok());
    }
}

// ============================================================================
// Edge Cases and Error Scenarios
// ============================================================================

#[test]
fn test_deployment_config_special_characters_in_environment() {
    let config = DeploymentConfig {
        environment: "test-env_123".to_string(),
        region: "local".to_string(),
        instance_count: 1,
        monitoring_enabled: true,
    };

    assert_eq!(config.environment, "test-env_123");
}

#[test]
fn test_deployment_config_long_environment_name() {
    let long_name = "a".repeat(100);
    let config = DeploymentConfig {
        environment: long_name.clone(),
        region: "local".to_string(),
        // TEST_CATEGORY: unit
        // TEST_DOMAIN: core
        // TEST_PRIORITY: normal
        instance_count: 1,
        monitoring_enabled: true,
    };

    assert_eq!(config.environment, long_name);
}

#[test]
fn test_deployment_config_long_region_name() {
    // TEST_CATEGORY: unit
    // TEST_DOMAIN: core
    // TEST_PRIORITY: normal
    let long_region = "region-".to_string() + &"x".repeat(50);
    let config = DeploymentConfig {
        environment: "test".to_string(),
        region: long_region.clone(),
        instance_count: 1,
        monitoring_enabled: true,
    };

    assert_eq!(config.region, long_region);
}
// TEST_CATEGORY: unit
// TEST_DOMAIN: core
// TEST_PRIORITY: normal

#[test]
fn test_deployment_config_max_instance_count() {
    let config = DeploymentConfig {
        environment: "test".to_string(),
        region: "local".to_string(),
        instance_count: u32::MAX,
        monitoring_enabled: true,
    };

    // TEST_CATEGORY: unit
    // TEST_DOMAIN: core
    // TEST_PRIORITY: normal
    assert_eq!(config.instance_count, u32::MAX);
}

#[test]
fn test_deployment_config_json_roundtrip() {
    let config = DeploymentConfig {
        environment: "test-env".to_string(),
        region: "test-region".to_string(),
        instance_count: 7,
        // TEST_CATEGORY: unit
        // TEST_DOMAIN: core
        // TEST_PRIORITY: normal
        monitoring_enabled: false,
    };

    let json = serde_json::to_string(&config).expect("Serialization should succeed");
    let parsed: DeploymentConfig =
        serde_json::from_str(&json).expect("Deserialization should succeed");

    assert_eq!(config.environment, parsed.environment);
    assert_eq!(config.region, parsed.region);
    assert_eq!(config.instance_count, parsed.instance_count);
    assert_eq!(config.monitoring_enabled, parsed.monitoring_enabled);
}

#[test]
fn test_deployment_config_json_pretty() {
    let config = DeploymentConfig::default();
    // TEST_CATEGORY: unit
    // TEST_DOMAIN: core
    // TEST_PRIORITY: normal

    let json = serde_json::to_string_pretty(&config).expect("Pretty serialization should succeed");
    assert!(json.contains("development"));
    assert!(json.contains("local"));
    assert!(json.contains("instance_count"));
}

// TEST_CATEGORY: unit
// TEST_DOMAIN: core
// TEST_PRIORITY: normal
#[test]
fn test_deployment_manager_const_new() {
    // Test that new() is const and can be used in const contexts
    const fn create_manager() -> DeploymentManager {
        let config = DeploymentConfig {
            environment: String::new(),
            region: String::new(),
            instance_count: 1,
            monitoring_enabled: true,
        };
        DeploymentManager::new(config)
    }

    let _manager = create_manager();
    // Successfully created in const context
}
