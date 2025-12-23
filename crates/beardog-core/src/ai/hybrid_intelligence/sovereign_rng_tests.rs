//! Comprehensive tests for Sovereign RNG module
//! Testing coverage for human entropy-driven randomization

use super::neural_networks::{EntropyDistribution, WeightInitialization};
use super::sovereign_rng::*;
use beardog_genetics::EntropyHierarchyManager;

// TEST_CATEGORY: unit
// TEST_DOMAIN: ai
// TEST_PRIORITY: high

#[tokio::test]
async fn test_sovereign_rng_with_default_config() {
    let entropy_manager = EntropyHierarchyManager::default();
    let config = SovereignRngConfig::default();
    let rng = SovereignRng::new(entropy_manager, config.clone());

    // Verify default configuration
    assert_eq!(config.min_entropy_tier, 2, "Default should require tier 2");
    assert!(config.cache_entropy, "Cache should be enabled by default");
    assert_eq!(
        config.cache_max_age_seconds, 300,
        "Default cache should be 5 minutes"
    );
    assert!(
        config.allow_machine_fallback,
        "Machine fallback should be allowed by default"
    );
    assert!(
        config.audit_entropy_usage,
        "Audit should be enabled by default"
    );

    // Verify stats
    let stats = rng.get_entropy_stats();
    assert_eq!(stats.cached_seeds, 0, "Should start with empty cache");
}

#[tokio::test]
async fn test_sovereign_rng_with_custom_config() {
    let entropy_manager = EntropyHierarchyManager::default();
    let config = SovereignRngConfig {
        min_entropy_tier: 3,
        cache_entropy: false,
        cache_max_age_seconds: 600,
        allow_machine_fallback: false,
        audit_entropy_usage: false,
    };
    let rng = SovereignRng::new(entropy_manager, config.clone());

    assert_eq!(config.min_entropy_tier, 3, "Custom tier should be 3");
    assert!(!config.cache_entropy, "Cache should be disabled");

    let stats = rng.get_entropy_stats();
    assert_eq!(stats.config.min_entropy_tier, 3);
}

#[tokio::test]
async fn test_weight_initialization_tier_3_human_lived_experience() {
    let entropy_manager = EntropyHierarchyManager::default();
    let config = SovereignRngConfig::default();
    let mut rng = SovereignRng::new(entropy_manager, config);

    let initializer = HumanEntropyWeightInitializer {
        entropy_tier: 3, // Highest tier - Human Lived Experience
        human_identity_id: "user_tier3".to_string(),
        distribution: EntropyDistribution::Xavier,
        layer_shape: (10, 10),
    };

    let result = rng.initialize_weights(&initializer);
    assert!(result.is_ok(), "Tier 3 initialization should succeed");

    let weights = result.unwrap();
    assert_eq!(weights.len(), 10, "Should have 10 rows");
    assert_eq!(weights[0].len(), 10, "Should have 10 columns");

    // Verify Xavier initialization properties (roughly)
    let limit = (6.0_f64 / (10.0 + 10.0)).sqrt();
    for row in &weights {
        for &weight in row {
            assert!(
                weight >= -limit * 2.0 && weight <= limit * 2.0,
                "Weights should be in reasonable range"
            );
        }
    }
}

#[tokio::test]
async fn test_weight_initialization_tier_2_human_supervised() {
    let entropy_manager = EntropyHierarchyManager::default();
    let config = SovereignRngConfig::default();
    let mut rng = SovereignRng::new(entropy_manager, config);

    let initializer = HumanEntropyWeightInitializer {
        entropy_tier: 2, // Human Supervised Machine
        human_identity_id: "user_tier2".to_string(),
        distribution: EntropyDistribution::He,
        layer_shape: (32, 64),
    };

    let result = rng.initialize_weights(&initializer);
    assert!(result.is_ok(), "Tier 2 initialization should succeed");

    let weights = result.unwrap();
    assert_eq!(weights.len(), 32, "Should have 32 rows");
    assert_eq!(weights[0].len(), 64, "Should have 64 columns");
}

#[tokio::test]
async fn test_weight_initialization_tier_1_store_bought() {
    let entropy_manager = EntropyHierarchyManager::default();
    let config = SovereignRngConfig::default();
    let mut rng = SovereignRng::new(entropy_manager, config);

    let initializer = HumanEntropyWeightInitializer {
        entropy_tier: 1, // Lowest tier - Store Bought Machine
        human_identity_id: "user_tier1".to_string(),
        distribution: EntropyDistribution::Uniform {
            min: -0.1,
            max: 0.1,
        },
        layer_shape: (5, 5),
    };

    let result = rng.initialize_weights(&initializer);
    assert!(result.is_ok(), "Tier 1 initialization should succeed");

    let weights = result.unwrap();
    assert_eq!(weights.len(), 5);
    assert_eq!(weights[0].len(), 5);

    // Verify uniform distribution properties
    for row in &weights {
        for &weight in row {
            assert!(
                (-0.2..=0.2).contains(&weight),
                "Uniform weights should be in specified range"
            );
        }
    }
}

#[test]
fn test_normal_distribution_initialization() {
    let entropy_manager = EntropyHierarchyManager::default();
    let config = SovereignRngConfig::default();
    let mut rng = SovereignRng::new(entropy_manager, config);

    let initializer = HumanEntropyWeightInitializer {
        entropy_tier: 2,
        human_identity_id: "user_normal".to_string(),
        distribution: EntropyDistribution::Normal {
            mean: 0.0,
            stddev: 0.5,
        },
        layer_shape: (20, 30),
    };

    let weights = rng.initialize_weights(&initializer).unwrap();
    assert_eq!(weights.len(), 20);
    assert_eq!(weights[0].len(), 30);

    // Check that weights vary (not all the same)
    let first_weight = weights[0][0];
    let has_variation = weights
        .iter()
        .any(|row| row.iter().any(|&w| (w - first_weight).abs() > 0.01));
    assert!(has_variation, "Weights should have variation");
}

#[test]
fn test_entropy_cache_functionality() {
    let entropy_manager = EntropyHierarchyManager::default();
    let config = SovereignRngConfig {
        min_entropy_tier: 2,
        cache_entropy: true,
        cache_max_age_seconds: 300,
        allow_machine_fallback: true,
        audit_entropy_usage: true,
    };
    let mut rng = SovereignRng::new(entropy_manager, config);

    // First initialization should cache entropy
    let initializer1 = HumanEntropyWeightInitializer {
        entropy_tier: 2,
        human_identity_id: "cache_test_user".to_string(),
        distribution: EntropyDistribution::Xavier,
        layer_shape: (10, 10),
    };

    let result1 = rng.initialize_weights(&initializer1);
    assert!(result1.is_ok());

    let stats_after_first = rng.get_entropy_stats();
    assert_eq!(
        stats_after_first.cached_seeds, 1,
        "Should have 1 cached seed"
    );

    // Second initialization with same user should use cache
    let initializer2 = HumanEntropyWeightInitializer {
        entropy_tier: 2,
        human_identity_id: "cache_test_user".to_string(),
        distribution: EntropyDistribution::He,
        layer_shape: (15, 20),
    };

    let result2 = rng.initialize_weights(&initializer2);
    assert!(result2.is_ok());

    let stats_after_second = rng.get_entropy_stats();
    assert_eq!(
        stats_after_second.cached_seeds, 1,
        "Should still have 1 cached seed"
    );

    // Different user should create new cache entry
    let initializer3 = HumanEntropyWeightInitializer {
        entropy_tier: 2,
        human_identity_id: "different_user".to_string(),
        distribution: EntropyDistribution::Xavier,
        layer_shape: (10, 10),
    };

    let result3 = rng.initialize_weights(&initializer3);
    assert!(result3.is_ok());

    let stats_after_third = rng.get_entropy_stats();
    assert_eq!(
        stats_after_third.cached_seeds, 2,
        "Should have 2 cached seeds"
    );
}

#[test]
fn test_entropy_cache_disabled() {
    let entropy_manager = EntropyHierarchyManager::default();
    let config = SovereignRngConfig {
        min_entropy_tier: 2,
        cache_entropy: false, // Disable caching
        cache_max_age_seconds: 300,
        allow_machine_fallback: true,
        audit_entropy_usage: true,
    };
    let mut rng = SovereignRng::new(entropy_manager, config);

    let initializer = HumanEntropyWeightInitializer {
        entropy_tier: 2,
        human_identity_id: "no_cache_user".to_string(),
        distribution: EntropyDistribution::Xavier,
        layer_shape: (10, 10),
    };

    let result = rng.initialize_weights(&initializer);
    assert!(result.is_ok());

    let stats = rng.get_entropy_stats();
    assert_eq!(
        stats.cached_seeds, 0,
        "Should have no cached seeds when caching disabled"
    );
}

#[test]
fn test_cache_cleanup() {
    let entropy_manager = EntropyHierarchyManager::default();
    let config = SovereignRngConfig {
        min_entropy_tier: 2,
        cache_entropy: true,
        cache_max_age_seconds: 0, // Immediate expiry for testing
        allow_machine_fallback: true,
        audit_entropy_usage: true,
    };
    let mut rng = SovereignRng::new(entropy_manager, config);

    let initializer = HumanEntropyWeightInitializer {
        entropy_tier: 2,
        human_identity_id: "cleanup_test_user".to_string(),
        distribution: EntropyDistribution::Xavier,
        layer_shape: (10, 10),
    };

    let _ = rng.initialize_weights(&initializer);

    // Cache should have 1 entry
    let stats_before = rng.get_entropy_stats();
    assert_eq!(stats_before.cached_seeds, 1);

    // ✅ MODERNIZED: Directly trigger cleanup instead of waiting for expiration
    // Cleanup should remove expired entries
    rng.cleanup_cache();

    let stats_after = rng.get_entropy_stats();
    assert_eq!(
        stats_after.cached_seeds, 0,
        "Expired cache should be cleaned up"
    );
}

#[test]
fn test_neural_network_entropy_integration() {
    let weight_init = WeightInitialization::HumanEntropyInitialization {
        required_entropy_tier: 3,
        human_identity_id: "integration_user".to_string(),
        distribution: EntropyDistribution::Xavier,
        fallback_to_machine: true,
    };

    let layer_shape = (256, 512);
    let initializer = NeuralNetworkEntropyIntegration::create_human_entropy_initializer(
        &weight_init,
        layer_shape,
    );

    assert!(
        initializer.is_some(),
        "Should create initializer from HumanEntropyInitialization"
    );

    let init = initializer.unwrap();
    assert_eq!(init.entropy_tier, 3);
    assert_eq!(init.human_identity_id, "integration_user");
    assert_eq!(init.layer_shape, (256, 512));
}

#[test]
fn test_uses_human_entropy_check() {
    let human_entropy_init = WeightInitialization::HumanEntropyInitialization {
        required_entropy_tier: 2,
        human_identity_id: "test_user".to_string(),
        distribution: EntropyDistribution::Xavier,
        fallback_to_machine: false,
    };

    assert!(
        NeuralNetworkEntropyIntegration::uses_human_entropy(&human_entropy_init),
        "Should recognize HumanEntropyInitialization"
    );

    let he_init = WeightInitialization::HeNormal;
    assert!(
        !NeuralNetworkEntropyIntegration::uses_human_entropy(&he_init),
        "HeNormal initialization should not use human entropy"
    );
}

#[test]
fn test_generate_entropy_bytes() {
    let entropy_manager = EntropyHierarchyManager::default();
    let config = SovereignRngConfig::default();
    let mut rng = SovereignRng::new(entropy_manager, config);

    let identity = beardog_genetics::HumanIdentity {
        identity_id: "entropy_bytes_user".to_string(),
        identity_hash: vec![1, 2, 3, 4],
        verification_level: beardog_genetics::VerificationLevel::Enhanced,
        verified_at: chrono::Utc::now(),
    };

    let result = rng.generate_entropy_bytes(&identity, 2, 128);
    assert!(result.is_ok(), "Should generate entropy bytes successfully");

    let entropy = result.unwrap();
    assert_eq!(entropy.len(), 128, "Should generate exactly 128 bytes");
}

#[test]
fn test_large_layer_initialization() {
    let entropy_manager = EntropyHierarchyManager::default();
    let config = SovereignRngConfig::default();
    let mut rng = SovereignRng::new(entropy_manager, config);

    let initializer = HumanEntropyWeightInitializer {
        entropy_tier: 2,
        human_identity_id: "large_layer_user".to_string(),
        distribution: EntropyDistribution::He,
        layer_shape: (512, 1024), // Large layer
    };

    let result = rng.initialize_weights(&initializer);
    assert!(result.is_ok(), "Should handle large layers");

    let weights = result.unwrap();
    assert_eq!(weights.len(), 512);
    assert_eq!(weights[0].len(), 1024);
}

#[test]
fn test_config_serialization() {
    let config = SovereignRngConfig {
        min_entropy_tier: 3,
        cache_entropy: true,
        cache_max_age_seconds: 600,
        allow_machine_fallback: false,
        audit_entropy_usage: true,
    };

    // Test that config can be serialized/deserialized
    let json = serde_json::to_string(&config).unwrap();
    let deserialized: SovereignRngConfig = serde_json::from_str(&json).unwrap();

    assert_eq!(deserialized.min_entropy_tier, 3);
    assert_eq!(deserialized.cache_max_age_seconds, 600);
    assert!(!deserialized.allow_machine_fallback);
}

#[test]
fn test_stats_serialization() {
    let stats = SovereignRngStats {
        cached_seeds: 5,
        config: SovereignRngConfig::default(),
    };

    let json = serde_json::to_string(&stats).unwrap();
    let deserialized: SovereignRngStats = serde_json::from_str(&json).unwrap();

    assert_eq!(deserialized.cached_seeds, 5);
    assert_eq!(deserialized.config.min_entropy_tier, 2);
}

#[test]
fn test_multiple_distribution_types() {
    let entropy_manager = EntropyHierarchyManager::default();
    let config = SovereignRngConfig::default();
    let mut rng = SovereignRng::new(entropy_manager, config);

    let distributions = vec![
        EntropyDistribution::Normal {
            mean: 0.0,
            stddev: 1.0,
        },
        EntropyDistribution::Uniform {
            min: -1.0,
            max: 1.0,
        },
        EntropyDistribution::Xavier,
        EntropyDistribution::He,
    ];

    for (i, dist) in distributions.into_iter().enumerate() {
        let initializer = HumanEntropyWeightInitializer {
            entropy_tier: 2,
            human_identity_id: format!("dist_test_user_{}", i),
            distribution: dist,
            layer_shape: (8, 8),
        };

        let result = rng.initialize_weights(&initializer);
        assert!(result.is_ok(), "All distribution types should work");
    }
}
