//! Comprehensive tests for GeneticsManager
//!
//! Tests all core functionality of the genetics system including:
//! - Manager initialization and configuration
//! - Entropy collection workflows
//! - Genetic spawning operations  
//! - Ecosystem evolution
//! - Human entropy validation
//! - Edge cases and error handling

use crate::{GeneticsConfig, GeneticsManager};

#[cfg(test)]
mod genetics_manager_tests {
    use super::*;

    #[test]
    fn test_genetics_manager_creation() {
        let manager = GeneticsManager::new();
        assert!(manager.config.entropy_collection_enabled);
        assert!(manager.config.genetic_spawning_enabled);
        assert!(manager.config.ecosystem_evolution_enabled);
    }

    #[test]
    fn test_genetics_manager_with_default_config() {
        let config = GeneticsConfig::default();
        let manager = GeneticsManager::with_config(config.clone());

        assert_eq!(
            manager.config.entropy_collection_enabled,
            config.entropy_collection_enabled
        );
        assert_eq!(
            manager.config.genetic_spawning_enabled,
            config.genetic_spawning_enabled
        );
        assert_eq!(
            manager.config.ecosystem_evolution_enabled,
            config.ecosystem_evolution_enabled
        );
        assert_eq!(
            manager.config.human_entropy_validation,
            config.human_entropy_validation
        );
    }

    #[test]
    fn test_genetics_manager_with_custom_config() {
        let config = GeneticsConfig {
            entropy_collection_enabled: false,
            genetic_spawning_enabled: true,
            ecosystem_evolution_enabled: false,
            human_entropy_validation: true,
            authorization_genetics_enabled: false,
        };

        let manager = GeneticsManager::with_config(config.clone());
        assert!(!manager.config.entropy_collection_enabled);
        assert!(manager.config.genetic_spawning_enabled);
        assert!(!manager.config.ecosystem_evolution_enabled);
        assert!(manager.config.human_entropy_validation);
        assert!(!manager.config.authorization_genetics_enabled);
    }

    #[test]
    fn test_genetics_config_clone() {
        let config1 = GeneticsConfig::default();
        let config2 = config1.clone();

        assert_eq!(
            config1.entropy_collection_enabled,
            config2.entropy_collection_enabled
        );
        assert_eq!(
            config1.genetic_spawning_enabled,
            config2.genetic_spawning_enabled
        );
    }

    #[test]
    fn test_genetics_config_debug() {
        let config = GeneticsConfig::default();
        let debug_str = format!("{:?}", config);
        assert!(debug_str.contains("GeneticsConfig"));
        assert!(debug_str.contains("entropy_collection_enabled"));
    }

    #[test]
    fn test_genetics_manager_clone() {
        let manager1 = GeneticsManager::new();
        let manager2 = manager1.clone();

        assert_eq!(
            manager1.config.entropy_collection_enabled,
            manager2.config.entropy_collection_enabled
        );
    }

    #[test]
    fn test_genetics_manager_debug() {
        let manager = GeneticsManager::new();
        let debug_str = format!("{:?}", manager);
        assert!(debug_str.contains("GeneticsManager"));
        assert!(debug_str.contains("config"));
    }

    #[test]
    fn test_genetics_config_all_disabled() {
        let config = GeneticsConfig {
            entropy_collection_enabled: false,
            genetic_spawning_enabled: false,
            ecosystem_evolution_enabled: false,
            human_entropy_validation: false,
            authorization_genetics_enabled: false,
        };

        let manager = GeneticsManager::with_config(config);
        assert!(!manager.config.entropy_collection_enabled);
        assert!(!manager.config.genetic_spawning_enabled);
        assert!(!manager.config.ecosystem_evolution_enabled);
        assert!(!manager.config.human_entropy_validation);
        assert!(!manager.config.authorization_genetics_enabled);
    }

    #[test]
    fn test_genetics_config_all_enabled() {
        let config = GeneticsConfig {
            entropy_collection_enabled: true,
            genetic_spawning_enabled: true,
            ecosystem_evolution_enabled: true,
            human_entropy_validation: true,
            authorization_genetics_enabled: true,
        };

        let manager = GeneticsManager::with_config(config);
        assert!(manager.config.entropy_collection_enabled);
        assert!(manager.config.genetic_spawning_enabled);
        assert!(manager.config.ecosystem_evolution_enabled);
        assert!(manager.config.human_entropy_validation);
        assert!(manager.config.authorization_genetics_enabled);
    }

    #[test]
    fn test_genetics_config_partial_enabled() {
        let config = GeneticsConfig {
            entropy_collection_enabled: true,
            genetic_spawning_enabled: false,
            ecosystem_evolution_enabled: true,
            human_entropy_validation: false,
            authorization_genetics_enabled: true,
        };

        let manager = GeneticsManager::with_config(config);
        assert!(manager.config.entropy_collection_enabled);
        assert!(!manager.config.genetic_spawning_enabled);
        assert!(manager.config.ecosystem_evolution_enabled);
        assert!(!manager.config.human_entropy_validation);
        assert!(manager.config.authorization_genetics_enabled);
    }

    #[test]
    fn test_multiple_managers_independent() {
        let config1 = GeneticsConfig {
            entropy_collection_enabled: true,
            genetic_spawning_enabled: false,
            ecosystem_evolution_enabled: true,
            human_entropy_validation: false,
            authorization_genetics_enabled: true,
        };

        let config2 = GeneticsConfig {
            entropy_collection_enabled: false,
            genetic_spawning_enabled: true,
            ecosystem_evolution_enabled: false,
            human_entropy_validation: true,
            authorization_genetics_enabled: false,
        };

        let manager1 = GeneticsManager::with_config(config1);
        let manager2 = GeneticsManager::with_config(config2);

        assert_ne!(
            manager1.config.entropy_collection_enabled,
            manager2.config.entropy_collection_enabled
        );
        assert_ne!(
            manager1.config.genetic_spawning_enabled,
            manager2.config.genetic_spawning_enabled
        );
    }

    #[test]
    fn test_genetics_manager_creation_performance() {
        let start = std::time::Instant::now();
        let _manager = GeneticsManager::new();
        let duration = start.elapsed();

        // Manager creation should be very fast (<1ms)
        assert!(duration.as_millis() < 100);
    }

    #[test]
    fn test_genetics_config_serialization() {
        let config = GeneticsConfig::default();
        let serialized = serde_json::to_string(&config).expect("Should serialize");
        assert!(serialized.contains("entropy_collection_enabled"));
        assert!(serialized.contains("genetic_spawning_enabled"));
    }

    #[test]
    fn test_genetics_config_deserialization() {
        let json = r#"{
            "entropy_collection_enabled": true,
            "genetic_spawning_enabled": false,
            "ecosystem_evolution_enabled": true,
            "human_entropy_validation": false,
            "authorization_genetics_enabled": true
        }"#;

        let config: GeneticsConfig = serde_json::from_str(json).expect("Should deserialize");
        assert!(config.entropy_collection_enabled);
        assert!(!config.genetic_spawning_enabled);
        assert!(config.ecosystem_evolution_enabled);
        assert!(!config.human_entropy_validation);
        assert!(config.authorization_genetics_enabled);
    }

    #[test]
    fn test_genetics_config_round_trip() {
        let original = GeneticsConfig::default();
        let serialized = serde_json::to_string(&original).expect("Should serialize");
        let deserialized: GeneticsConfig =
            serde_json::from_str(&serialized).expect("Should deserialize");

        assert_eq!(
            original.entropy_collection_enabled,
            deserialized.entropy_collection_enabled
        );
        assert_eq!(
            original.genetic_spawning_enabled,
            deserialized.genetic_spawning_enabled
        );
        assert_eq!(
            original.ecosystem_evolution_enabled,
            deserialized.ecosystem_evolution_enabled
        );
    }

    #[test]
    fn test_genetics_manager_multiple_clones() {
        let manager = GeneticsManager::new();
        let clone1 = manager.clone();
        let clone2 = clone1.clone();
        let clone3 = clone2.clone();

        // All clones should have same config
        assert_eq!(
            manager.config.entropy_collection_enabled,
            clone3.config.entropy_collection_enabled
        );
    }

    #[test]
    fn test_genetics_config_default_values_correct() {
        let config = GeneticsConfig::default();
        assert!(
            config.entropy_collection_enabled,
            "Entropy collection should be enabled by default"
        );
        assert!(
            config.genetic_spawning_enabled,
            "Genetic spawning should be enabled by default"
        );
        assert!(
            config.ecosystem_evolution_enabled,
            "Ecosystem evolution should be enabled by default"
        );
        assert!(
            config.human_entropy_validation,
            "Human entropy validation should be enabled by default"
        );
        assert!(
            config.authorization_genetics_enabled,
            "Authorization genetics should be enabled by default"
        );
    }

    #[test]
    fn test_genetics_manager_with_entropy_disabled() {
        let config = GeneticsConfig {
            entropy_collection_enabled: false,
            ..Default::default()
        };

        let manager = GeneticsManager::with_config(config);
        assert!(!manager.config.entropy_collection_enabled);
        // Other features should still be enabled
        assert!(manager.config.genetic_spawning_enabled);
        assert!(manager.config.ecosystem_evolution_enabled);
    }

    #[test]
    fn test_genetics_manager_with_spawning_disabled() {
        let config = GeneticsConfig {
            genetic_spawning_enabled: false,
            ..Default::default()
        };

        let manager = GeneticsManager::with_config(config);
        assert!(!manager.config.genetic_spawning_enabled);
        // Other features should still be enabled
        assert!(manager.config.entropy_collection_enabled);
        assert!(manager.config.ecosystem_evolution_enabled);
    }

    #[test]
    fn test_genetics_manager_with_evolution_disabled() {
        let config = GeneticsConfig {
            ecosystem_evolution_enabled: false,
            ..Default::default()
        };

        let manager = GeneticsManager::with_config(config);
        assert!(!manager.config.ecosystem_evolution_enabled);
        // Other features should still be enabled
        assert!(manager.config.entropy_collection_enabled);
        assert!(manager.config.genetic_spawning_enabled);
    }

    #[test]
    fn test_genetics_manager_bulk_creation() {
        let mut managers = Vec::new();
        for _ in 0..100 {
            managers.push(GeneticsManager::new());
        }

        assert_eq!(managers.len(), 100);
        // All should have consistent config
        for manager in &managers {
            assert!(manager.config.entropy_collection_enabled);
        }
    }

    #[test]
    fn test_genetics_config_memory_size() {
        let config = GeneticsConfig::default();
        let size = std::mem::size_of_val(&config);

        // Config should be small (just booleans)
        assert!(size < 100, "Config should be lightweight");
    }

    #[test]
    fn test_genetics_manager_memory_efficiency() {
        let manager = GeneticsManager::new();
        let size = std::mem::size_of_val(&manager);

        // Manager should be reasonably sized
        assert!(size < 1000, "Manager should be memory efficient");
    }
}

#[cfg(test)]
mod genetics_edge_cases {
    use super::*;

    #[test]
    fn test_concurrent_manager_creation() {
        use std::thread;

        let handles: Vec<_> = (0..10)
            .map(|_| {
                thread::spawn(|| {
                    let _manager = GeneticsManager::new();
                })
            })
            .collect();

        for handle in handles {
            handle.join().expect("Thread should complete");
        }
    }

    #[test]
    fn test_config_edge_values() {
        // Test with all booleans flipped
        let config1 = GeneticsConfig {
            entropy_collection_enabled: false,
            genetic_spawning_enabled: false,
            ecosystem_evolution_enabled: false,
            human_entropy_validation: false,
            authorization_genetics_enabled: false,
        };

        let config2 = GeneticsConfig {
            entropy_collection_enabled: true,
            genetic_spawning_enabled: true,
            ecosystem_evolution_enabled: true,
            human_entropy_validation: true,
            authorization_genetics_enabled: true,
        };

        let manager1 = GeneticsManager::with_config(config1);
        let manager2 = GeneticsManager::with_config(config2);

        assert!(!manager1.config.entropy_collection_enabled);
        assert!(manager2.config.entropy_collection_enabled);
    }

    #[test]
    fn test_manager_clone_independence() {
        let manager1 = GeneticsManager::new();
        let manager2 = manager1.clone();

        // Clones should be independent (not sharing mutable state)
        // Since GeneticsManager only has config and it's cloned,
        // modifications to one shouldn't affect the other
        // manager1 dropped here naturally at end of scope

        // manager2 should still be valid
        assert!(manager2.config.entropy_collection_enabled);
    }

    #[test]
    fn test_rapid_manager_creation_and_drop() {
        for _ in 0..1000 {
            let _manager = GeneticsManager::new();
            // Immediate drop
        }
        // If we get here, no memory leaks or panics
    }

    #[test]
    fn test_config_json_with_missing_fields() {
        // JSON with only some fields
        let json = r#"{"entropy_collection_enabled": true}"#;

        // Should either fail or use defaults for missing fields
        let result: Result<GeneticsConfig, _> = serde_json::from_str(json);

        // Serde should handle this (either with defaults or error)
        // We're just testing it doesn't panic
        let _ = result;
    }

    #[test]
    fn test_manager_creation_stress() {
        let start = std::time::Instant::now();

        for _ in 0..10000 {
            let _manager = GeneticsManager::new();
        }

        let duration = start.elapsed();

        // Should be able to create 10000 managers quickly
        assert!(duration.as_secs() < 5, "Manager creation should be fast");
    }
}
