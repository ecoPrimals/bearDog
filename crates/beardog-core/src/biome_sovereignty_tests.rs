// SPDX-License-Identifier: AGPL-3.0-only

//! Tests for Biome Sovereignty

#[cfg(test)]
mod tests {
    #![allow(
        unused_imports,
        clippy::float_cmp,
        clippy::useless_vec,
        clippy::needless_range_loop,
        clippy::uninlined_format_args,
        clippy::field_reassign_with_default,
        clippy::manual_range_contains,
        unused_variables,
        dead_code
    )]

    use crate::biome_sovereignty::{
        BiomeSovereigntyConfig, BiomeSovereigntyManager, EntropyPreferences, SovereigntyLevel,
    };

    #[test]
    fn test_new_sovereignty_manager() {
        let manager = BiomeSovereigntyManager::new("test-biome".to_string());

        assert_eq!(manager.biome_id, "test-biome");
        assert!(manager.config.enable_genetic_algorithms);
        assert!(manager.config.enable_mixed_lineage);
    }

    #[test]
    fn test_new_with_custom_config() {
        let config = BiomeSovereigntyConfig {
            enable_genetic_algorithms: false,
            enable_mixed_lineage: true,
            max_genetic_iterations: 500,
            entropy_preferences: EntropyPreferences::Hybrid,
        };

        let manager = BiomeSovereigntyManager::new_with_config("custom-biome".to_string(), config);

        assert_eq!(manager.biome_id, "custom-biome");
        assert!(!manager.config.enable_genetic_algorithms);
        assert!(manager.config.enable_mixed_lineage);
        assert_eq!(manager.config.max_genetic_iterations, 500);
    }

    #[test]
    fn test_initialize_with_genetic_algorithms() {
        let mut manager = BiomeSovereigntyManager::new("genetic-biome".to_string());
        manager.config.enable_genetic_algorithms = true;

        // Should not panic
        manager.initialize();
    }

    #[test]
    fn test_initialize_with_mixed_lineage() {
        let mut manager = BiomeSovereigntyManager::new("lineage-biome".to_string());
        manager.config.enable_mixed_lineage = true;

        // Should not panic
        manager.initialize();
    }

    #[test]
    fn test_initialize_with_both_features() {
        let mut manager = BiomeSovereigntyManager::new("full-biome".to_string());
        manager.config.enable_genetic_algorithms = true;
        manager.config.enable_mixed_lineage = true;

        // Should not panic
        manager.initialize();
    }

    #[test]
    fn test_initialize_with_no_features() {
        let mut manager = BiomeSovereigntyManager::new("minimal-biome".to_string());
        manager.config.enable_genetic_algorithms = false;
        manager.config.enable_mixed_lineage = false;

        // Should not panic
        manager.initialize();
    }

    #[test]
    fn test_get_sovereignty_status() {
        let manager = BiomeSovereigntyManager::new("status-biome".to_string());
        let status = manager.get_sovereignty_status();

        assert_eq!(status.biome_id, "status-biome");
        assert!(status.is_sovereign);
        assert!(status.genetic_algorithms_active);
        assert!(status.mixed_lineage_active);
    }

    #[test]
    fn test_entropy_quality_human_only() {
        let config = BiomeSovereigntyConfig {
            enable_genetic_algorithms: true,
            enable_mixed_lineage: true,
            max_genetic_iterations: 1000,
            entropy_preferences: EntropyPreferences::HumanOnly,
        };

        let manager = BiomeSovereigntyManager::new_with_config("human-entropy".to_string(), config);
        let status = manager.get_sovereignty_status();

        assert_eq!(status.entropy_quality, 0.9);
    }

    #[test]
    fn test_entropy_quality_hybrid() {
        let config = BiomeSovereigntyConfig {
            enable_genetic_algorithms: true,
            enable_mixed_lineage: true,
            max_genetic_iterations: 1000,
            entropy_preferences: EntropyPreferences::Hybrid,
        };

        let manager =
            BiomeSovereigntyManager::new_with_config("hybrid-entropy".to_string(), config);
        let status = manager.get_sovereignty_status();

        assert_eq!(status.entropy_quality, 0.8);
    }

    #[test]
    fn test_entropy_quality_machine_only() {
        let config = BiomeSovereigntyConfig {
            enable_genetic_algorithms: true,
            enable_mixed_lineage: true,
            max_genetic_iterations: 1000,
            entropy_preferences: EntropyPreferences::MachineOnly,
        };

        let manager =
            BiomeSovereigntyManager::new_with_config("machine-entropy".to_string(), config);
        let status = manager.get_sovereignty_status();

        assert_eq!(status.entropy_quality, 0.7);
    }

    #[test]
    fn test_config_default() {
        let config = BiomeSovereigntyConfig::default();

        assert!(config.enable_genetic_algorithms);
        assert!(config.enable_mixed_lineage);
        assert_eq!(config.max_genetic_iterations, 1000);
        assert!(matches!(
            config.entropy_preferences,
            EntropyPreferences::HumanOnly
        ));
    }

    #[test]
    fn test_entropy_preferences_default() {
        let pref = EntropyPreferences::default();
        assert!(matches!(pref, EntropyPreferences::HumanOnly));
    }

    #[test]
    fn test_sovereignty_level_values() {
        // Verify all enum variants exist and can be instantiated
        let levels = [
            SovereigntyLevel::Sovereign,
            SovereigntyLevel::PartiallySovereign,
            SovereigntyLevel::HighlyDependent,
        ];
        // Verify we have all three levels
        assert_eq!(levels.len(), 3);
    }

    #[test]
    fn test_manager_created_timestamp() {
        // ✅ MODERNIZED: Removed sleep - timestamps are already monotonic
        // SystemTime::now() guarantees monotonic increasing timestamps
        let manager1 = BiomeSovereigntyManager::new("time1".to_string());
        let manager2 = BiomeSovereigntyManager::new("time2".to_string());

        // Timestamps should be sequential (or equal if instant creation)
        assert!(manager2.created_at >= manager1.created_at);
    }

    #[test]
    fn test_status_last_updated_timestamp() {
        use std::thread;
        use std::time::Duration;

        let manager = BiomeSovereigntyManager::new("timestamp-test".to_string());

        let status1 = manager.get_sovereignty_status();
        // ✅ REMOVED: Unnecessary sleep - SystemTime::now() is monotonic
        // Test verifies timestamps are ordered, not that time passes
        let status2 = manager.get_sovereignty_status();

        // Timestamps should be ordered (>= allows for fast execution)
        assert!(status2.last_updated >= status1.last_updated);
    }

    #[test]
    fn test_config_serialization() {
        let config = BiomeSovereigntyConfig::default();
        let json = serde_json::to_string(&config).expect("Should serialize");
        assert!(!json.is_empty());

        let deserialized: BiomeSovereigntyConfig =
            serde_json::from_str(&json).expect("Should deserialize");
        assert_eq!(
            deserialized.max_genetic_iterations,
            config.max_genetic_iterations
        );
    }

    #[test]
    fn test_status_serialization() {
        let manager = BiomeSovereigntyManager::new("serialize-test".to_string());
        let status = manager.get_sovereignty_status();

        let json = serde_json::to_string(&status).expect("Should serialize");
        assert!(!json.is_empty());

        let deserialized: crate::biome_sovereignty::SovereigntyStatus =
            serde_json::from_str(&json).expect("Should deserialize");
        // Verify the deserialized status is valid
        assert!(!deserialized.biome_id.is_empty());
    }

    #[test]
    fn test_manager_debug_format() {
        let manager = BiomeSovereigntyManager::new("debug-test".to_string());
        let debug_str = format!("{:?}", manager);
        assert!(debug_str.contains("BiomeSovereigntyManager"));
    }

    #[test]
    fn test_config_clone() {
        let config1 = BiomeSovereigntyConfig::default();
        let config2 = config1.clone();

        assert_eq!(
            config1.max_genetic_iterations,
            config2.max_genetic_iterations
        );
        assert_eq!(
            config1.enable_genetic_algorithms,
            config2.enable_genetic_algorithms
        );
    }

    #[test]
    fn test_status_clone() {
        let manager = BiomeSovereigntyManager::new("clone-test".to_string());
        let status1 = manager.get_sovereignty_status();
        let status2 = status1.clone();

        assert_eq!(status1.biome_id, status2.biome_id);
        assert_eq!(status1.is_sovereign, status2.is_sovereign);
    }

    #[test]
    fn test_multiple_managers_independent() {
        let mut manager1 = BiomeSovereigntyManager::new("biome1".to_string());
        let mut manager2 = BiomeSovereigntyManager::new("biome2".to_string());

        manager1.config.enable_genetic_algorithms = false;
        manager2.config.enable_genetic_algorithms = true;

        manager1.initialize();
        manager2.initialize();

        let status1 = manager1.get_sovereignty_status();
        let status2 = manager2.get_sovereignty_status();

        assert_eq!(status1.biome_id, "biome1");
        assert_eq!(status2.biome_id, "biome2");
        assert!(!status1.genetic_algorithms_active);
        assert!(status2.genetic_algorithms_active);
    }

    #[test]
    fn test_entropy_preferences_copy() {
        let pref1 = EntropyPreferences::HumanOnly;
        let pref2 = pref1; // Should work because Copy

        assert!(matches!(pref1, EntropyPreferences::HumanOnly));
        assert!(matches!(pref2, EntropyPreferences::HumanOnly));
    }

    #[test]
    fn test_sovereignty_level_copy() {
        let level1 = SovereigntyLevel::Sovereign;
        let level2 = level1; // Should work because Copy

        assert!(matches!(level1, SovereigntyLevel::Sovereign));
        assert!(matches!(level2, SovereigntyLevel::Sovereign));
    }
}
