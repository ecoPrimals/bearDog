// SPDX-License-Identifier: AGPL-3.0-or-later

//! Comprehensive tests for AI learning configurations
//!
//! These tests ensure learning configuration types are correctly implemented
//! and handle various scenarios.

use super::learning::*;
use approx::assert_relative_eq;

#[cfg(test)]
#[expect(
    clippy::disallowed_methods,
    reason = "learning tests use approx macros and intentional unwraps for fixtures"
)]
// ═══════════════════════════════════════════════════════════════════
// LearningAlgorithmType Tests
// ═══════════════════════════════════════════════════════════════════
#[test]
fn test_learning_algorithm_type_variants() {
    // Test all enum variants exist and are constructible
    let types = [
        LearningAlgorithmType::Supervised,
        LearningAlgorithmType::Unsupervised,
        LearningAlgorithmType::Reinforcement,
        LearningAlgorithmType::SemiSupervised,
        LearningAlgorithmType::Transfer,
        LearningAlgorithmType::Meta,
        LearningAlgorithmType::Online,
        LearningAlgorithmType::Federated,
    ];

    assert_eq!(types.len(), 8);
}

#[test]
fn test_learning_algorithm_type_clone() {
    let original = LearningAlgorithmType::Supervised;
    let cloned = original;

    // Should be copyable
    assert!(matches!(cloned, LearningAlgorithmType::Supervised));
}

#[test]
fn test_learning_algorithm_type_debug() {
    let algo = LearningAlgorithmType::Reinforcement;
    let debug_str = format!("{algo:?}");
    assert!(debug_str.contains("Reinforcement"));
}

#[test]
fn test_learning_algorithm_type_serialization() {
    let algo = LearningAlgorithmType::Meta;
    let serialized = serde_json::to_string(&algo).expect("Should serialize");
    let deserialized: LearningAlgorithmType =
        serde_json::from_str(&serialized).expect("Should deserialize");

    assert!(matches!(deserialized, LearningAlgorithmType::Meta));
}

// ═══════════════════════════════════════════════════════════════════
// OnlineLearningConfig Tests
// ═══════════════════════════════════════════════════════════════════

#[test]
fn test_online_learning_config_default() {
    let config = OnlineLearningConfig::default();

    // Verify default values
    assert!(!config.enabled); // Default is disabled
    assert_relative_eq!(config.learning_rate, 0.001, epsilon = 1e-6);
    assert_eq!(config.batch_size, 32);
    assert_eq!(config.update_frequency, 100);
}

#[test]
fn test_online_learning_config_custom() {
    let config = OnlineLearningConfig {
        enabled: true,
        learning_rate: 0.01,
        batch_size: 64,
        update_frequency: 50,
    };

    assert!(config.enabled);
    assert_relative_eq!(config.learning_rate, 0.01, epsilon = 1e-6);
    assert_eq!(config.batch_size, 64);
    assert_eq!(config.update_frequency, 50);
}

#[test]
fn test_online_learning_config_clone() {
    let config = OnlineLearningConfig {
        enabled: true,
        learning_rate: 0.005,
        batch_size: 128,
        update_frequency: 200,
    };

    let cloned = config;
    assert_relative_eq!(cloned.learning_rate, 0.005, epsilon = 1e-6);
    assert_eq!(cloned.batch_size, 128);
}

#[test]
fn test_online_learning_config_debug() {
    let config = OnlineLearningConfig::default();
    let debug_str = format!("{config:?}");
    assert!(debug_str.contains("OnlineLearningConfig"));
}

#[test]
fn test_online_learning_config_serialization() {
    let config = OnlineLearningConfig {
        enabled: true,
        learning_rate: 0.002,
        batch_size: 64,
        update_frequency: 150,
    };

    let serialized = serde_json::to_string(&config).expect("Should serialize");
    let deserialized: OnlineLearningConfig =
        serde_json::from_str(&serialized).expect("Should deserialize");

    assert!(deserialized.enabled);
    assert_relative_eq!(deserialized.learning_rate, 0.002, epsilon = 1e-6);
    assert_eq!(deserialized.batch_size, 64);
}

#[test]
fn test_online_learning_config_various_learning_rates() {
    let rates = vec![0.0001, 0.001, 0.01, 0.1, 1.0];

    for rate in rates {
        let config = OnlineLearningConfig {
            enabled: true,
            learning_rate: rate,
            batch_size: 32,
            update_frequency: 100,
        };

        assert_relative_eq!(config.learning_rate, rate, epsilon = 1e-6);
    }
}

#[test]
fn test_online_learning_config_various_batch_sizes() {
    let sizes = vec![1, 8, 16, 32, 64, 128, 256];

    for size in sizes {
        let config = OnlineLearningConfig {
            enabled: true,
            learning_rate: 0.001,
            batch_size: size,
            update_frequency: 100,
        };

        assert_eq!(config.batch_size, size);
    }
}

// ═══════════════════════════════════════════════════════════════════
// LearningRateAdaptation Tests
// ═══════════════════════════════════════════════════════════════════

#[test]
fn test_learning_rate_adaptation_variants() {
    let strategies = [
        LearningRateAdaptation::Fixed,
        LearningRateAdaptation::Adaptive,
        LearningRateAdaptation::Decay,
        LearningRateAdaptation::PerformanceBased,
    ];

    assert_eq!(strategies.len(), 4);
}

#[test]
fn test_learning_rate_adaptation_clone() {
    let strategy = LearningRateAdaptation::Adaptive;
    let cloned = strategy;

    assert_eq!(strategy, cloned);
}

#[test]
fn test_learning_rate_adaptation_partial_eq() {
    assert_eq!(LearningRateAdaptation::Fixed, LearningRateAdaptation::Fixed);
    assert_ne!(
        LearningRateAdaptation::Fixed,
        LearningRateAdaptation::Adaptive
    );
}

#[test]
fn test_learning_rate_adaptation_debug() {
    let strategy = LearningRateAdaptation::Decay;
    let debug_str = format!("{strategy:?}");
    assert!(debug_str.contains("Decay"));
}

#[test]
fn test_learning_rate_adaptation_serialization() {
    let strategy = LearningRateAdaptation::PerformanceBased;
    let serialized = serde_json::to_string(&strategy).expect("Should serialize");
    let deserialized: LearningRateAdaptation =
        serde_json::from_str(&serialized).expect("Should deserialize");

    assert_eq!(deserialized, LearningRateAdaptation::PerformanceBased);
}

// ═══════════════════════════════════════════════════════════════════
// Integration Tests
// ═══════════════════════════════════════════════════════════════════

#[test]
fn test_online_learning_disabled_by_default() {
    let config = OnlineLearningConfig::default();

    // Online learning should be disabled by default for safety
    assert!(
        !config.enabled,
        "Online learning should be disabled by default"
    );
}

#[test]
fn test_online_learning_sensible_defaults() {
    let config = OnlineLearningConfig::default();

    // Verify defaults are sensible
    assert!(
        config.learning_rate > 0.0 && config.learning_rate < 1.0,
        "Learning rate should be in (0, 1)"
    );
    assert!(config.batch_size >= 1, "Batch size should be positive");
    assert!(
        config.update_frequency >= 1,
        "Update frequency should be positive"
    );
}

#[test]
fn test_learning_algorithm_type_copy_trait() {
    let algo1 = LearningAlgorithmType::Supervised;
    let algo2 = algo1; // Copy
    let algo3 = algo1; // Copy again

    // All should be usable after copies
    assert!(matches!(algo1, LearningAlgorithmType::Supervised));
    assert!(matches!(algo2, LearningAlgorithmType::Supervised));
    assert!(matches!(algo3, LearningAlgorithmType::Supervised));
}

#[test]
fn test_learning_rate_adaptation_copy_trait() {
    let strategy1 = LearningRateAdaptation::Adaptive;
    let strategy2 = strategy1; // Copy
    let strategy3 = strategy1; // Copy again

    // All should be usable after copies
    assert_eq!(strategy1, LearningRateAdaptation::Adaptive);
    assert_eq!(strategy2, LearningRateAdaptation::Adaptive);
    assert_eq!(strategy3, LearningRateAdaptation::Adaptive);
}

#[test]
fn test_all_types_have_required_traits() {
    // This test ensures all types implement required traits
    // by trying to use them in various contexts

    // LearningAlgorithmType
    let algo = LearningAlgorithmType::Meta;
    let _ = format!("{algo:?}"); // Debug
    let _ = algo; // Copy

    // OnlineLearningConfig
    let config = OnlineLearningConfig::default();
    let _ = format!("{config:?}"); // Debug
    let _ = config; // Copy

    // LearningRateAdaptation
    let strategy = LearningRateAdaptation::PerformanceBased;
    let _ = format!("{strategy:?}"); // Debug
    let _ = strategy; // Copy
    let _ = strategy == LearningRateAdaptation::PerformanceBased; // PartialEq
}

#[test]
fn test_online_learning_config_high_batch_size() {
    let config = OnlineLearningConfig {
        enabled: true,
        learning_rate: 0.001,
        batch_size: 1024,
        update_frequency: 10,
    };

    assert_eq!(config.batch_size, 1024);
}

#[test]
fn test_online_learning_config_low_update_frequency() {
    let config = OnlineLearningConfig {
        enabled: true,
        learning_rate: 0.001,
        batch_size: 32,
        update_frequency: 1,
    };

    assert_eq!(config.update_frequency, 1);
}

#[test]
fn test_online_learning_config_high_update_frequency() {
    let config = OnlineLearningConfig {
        enabled: true,
        learning_rate: 0.001,
        batch_size: 32,
        update_frequency: 10000,
    };

    assert_eq!(config.update_frequency, 10000);
}

// ═══════════════════════════════════════════════════════════════════
// Edge Case Tests
// ═══════════════════════════════════════════════════════════════════

#[test]
fn test_online_learning_config_zero_learning_rate() {
    // While not recommended, should be possible to set 0 learning rate
    let config = OnlineLearningConfig {
        enabled: false,
        learning_rate: 0.0,
        batch_size: 32,
        update_frequency: 100,
    };

    assert_relative_eq!(config.learning_rate, 0.0, epsilon = 1e-6);
}

#[test]
fn test_online_learning_config_high_learning_rate() {
    // While unusual, should handle high learning rates
    let config = OnlineLearningConfig {
        enabled: true,
        learning_rate: 10.0,
        batch_size: 32,
        update_frequency: 100,
    };

    assert_relative_eq!(config.learning_rate, 10.0, epsilon = 1e-6);
}

#[test]
fn test_serde_roundtrip_all_types() {
    // LearningAlgorithmType
    for algo_type in [
        LearningAlgorithmType::Supervised,
        LearningAlgorithmType::Unsupervised,
        LearningAlgorithmType::Reinforcement,
        LearningAlgorithmType::SemiSupervised,
        LearningAlgorithmType::Transfer,
        LearningAlgorithmType::Meta,
        LearningAlgorithmType::Online,
        LearningAlgorithmType::Federated,
    ] {
        let serialized = serde_json::to_string(&algo_type).expect("Should serialize");
        let _deserialized: LearningAlgorithmType =
            serde_json::from_str(&serialized).expect("Should deserialize");
    }

    // LearningRateAdaptation
    for strategy in [
        LearningRateAdaptation::Fixed,
        LearningRateAdaptation::Adaptive,
        LearningRateAdaptation::Decay,
        LearningRateAdaptation::PerformanceBased,
    ] {
        let serialized = serde_json::to_string(&strategy).expect("Should serialize");
        let _deserialized: LearningRateAdaptation =
            serde_json::from_str(&serialized).expect("Should deserialize");
    }
}

// ── learning_optimization-heavy serde coverage ───────────────────────────────

use std::collections::HashMap;
use std::time::Duration;

#[test]
fn prediction_horizon_and_update_frequency_json_roundtrip() {
    let h = PredictionHorizon::Custom(Duration::from_millis(500));
    let s = serde_json::to_string(&h).expect("ser");
    let back: PredictionHorizon = serde_json::from_str(&s).expect("de");
    assert_eq!(back, h);

    for uf in [
        UpdateFrequency::PerSample,
        UpdateFrequency::PerBatch,
        UpdateFrequency::TimeInterval(Duration::from_secs(3)),
        UpdateFrequency::PerformanceThreshold(0.88),
    ] {
        let js = serde_json::to_string(&uf).expect("ser");
        let back: UpdateFrequency = serde_json::from_str(&js).expect("de");
        assert_eq!(
            serde_json::to_string(&back).expect("re-ser"),
            serde_json::to_string(&uf).expect("orig-ser")
        );
    }
}

#[test]
fn ensemble_and_hyperparameter_config_json_roundtrip() {
    let cfg = EnsembleConfig {
        base_models: vec![BaseModel {
            id: "bm".into(),
            model_type: ModelType::NeuralNetwork,
            configuration: HashMap::from([("layers".into(), serde_json::json!(4))]),
            weight: 1.0,
        }],
        ensemble_method: EnsembleMethod::Stacking,
        model_weights: None,
        diversity_measures: vec![DiversityMeasure::KohaviWolpert],
    };
    let json = serde_json::to_string(&cfg).expect("ser");
    let back: EnsembleConfig = serde_json::from_str(&json).expect("de");
    assert_eq!(back.ensemble_method, cfg.ensemble_method);

    let ho = HyperparameterOptimization {
        method: HyperparameterOptimizationMethod::Hyperband,
        max_trials: 12,
        timeout_secs: 120,
        objective_metric: "accuracy".into(),
        optimization_direction: OptimizationDirection::Maximize,
    };
    let j2 = serde_json::to_string(&ho).expect("ser");
    let back_ho: HyperparameterOptimization = serde_json::from_str(&j2).expect("de");
    assert_eq!(back_ho.method, ho.method);
}

#[test]
fn neural_architecture_search_json_roundtrip() {
    let nas = NeuralArchitectureSearch {
        search_space: SearchSpace {
            layer_types: vec![LayerType::Convolutional],
            depth_range: (1, 4),
            width_range: (8, 128),
            activation_functions: vec![ActivationFunction::Swish],
        },
        search_strategy: NasSearchStrategy::RandomSearch,
        performance_estimation: PerformanceEstimation {
            method: PerformanceEstimationMethod::LearningCurveExtrapolation,
            early_stopping: None,
            resource_constraints: ResourceConstraints {
                max_training_time: Duration::from_secs(300),
                max_memory_gb: 4.0,
                max_gpu_count: 0,
                max_cpu_cores: 8,
            },
        },
    };
    let j = serde_json::to_string(&nas).expect("ser");
    let back: NeuralArchitectureSearch = serde_json::from_str(&j).expect("de");
    assert_eq!(back.search_strategy, nas.search_strategy);
}
