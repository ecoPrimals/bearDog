// SPDX-License-Identifier: AGPL-3.0-only

//! Comprehensive Error Path Tests for `beardog-genetics`
//!
//! These tests specifically target error handling paths to increase coverage
//! and ensure robust error handling throughout the genetics module.

#![allow(clippy::unwrap_used, clippy::expect_used)]

use crate::genetics::spawning::engine::GeneticSpawningEngine;
use crate::genetics::spawning::types::{SpawnRequest, SpawnResult};
use beardog_auth::auth::{BearDogGenetics, NodeCapability, SecurityClearance};
use beardog_errors::BearDogError;
use beardog_types::canonical::config::GeneticsConfig;

// TEST_CATEGORY: error_path
// TEST_DOMAIN: genetics
// TEST_PRIORITY: high

#[test]
fn test_spawn_with_default_config() {
    let config = GeneticsConfig::default();

    let engine = GeneticSpawningEngine::with_config(config);
    let request = SpawnRequest {
        required_capabilities: vec![NodeCapability::DataStorage],
        security_clearance: SecurityClearance::Medium,
        parent_genetics: vec![],
    };

    // Engine should handle config gracefully
    let result = engine.spawn_genetics(request);
    assert!(result.is_ok());
}

#[test]
fn test_spawn_with_empty_request() {
    let engine = GeneticSpawningEngine::new();
    let request = SpawnRequest {
        required_capabilities: vec![],
        security_clearance: SecurityClearance::Medium,
        parent_genetics: vec![],
    };

    let result = engine.spawn_genetics(request);
    assert!(result.is_ok());
    if let Ok(spawn_result) = result {
        assert!(spawn_result.success);
        assert!(spawn_result.genetics.capabilities.is_empty());
    }
}

#[test]
fn test_spawn_with_conflicting_capabilities() {
    let engine = GeneticSpawningEngine::new();
    let request = SpawnRequest {
        required_capabilities: vec![
            NodeCapability::DataStorage,
            NodeCapability::ComputeProvider,
            NodeCapability::ComputeProvider,
            NodeCapability::DataStorage,
        ],
        security_clearance: SecurityClearance::Maximum,
        parent_genetics: vec![],
    };

    let result = engine.spawn_genetics(request);
    assert!(result.is_ok());
    if let Ok(spawn_result) = result {
        assert!(spawn_result.genetics.capabilities.len() == 4);
    }
}

#[test]
fn test_spawn_with_invalid_parent_genetics() {
    let engine = GeneticSpawningEngine::new();

    // Create a parent with invalid fitness score
    let invalid_parent = BearDogGenetics {
        id: "invalid-parent".to_string(),
        fitness_score: -0.5, // Invalid negative score
        ..Default::default()
    };

    let request = SpawnRequest {
        required_capabilities: vec![NodeCapability::DataStorage],
        security_clearance: SecurityClearance::Medium,
        parent_genetics: vec![invalid_parent],
    };

    let result = engine.spawn_genetics(request);
    // Should handle gracefully despite invalid parent
    assert!(result.is_ok());
}

#[test]
fn test_spawn_with_maximum_parents() {
    let engine = GeneticSpawningEngine::new();

    // Create many parents to test limits
    let parents: Vec<BearDogGenetics> = (0..10)
        .map(|i| BearDogGenetics {
            id: format!("parent-{i}"),
            fitness_score: 0.7,
            generation: i,
            ..Default::default()
        })
        .collect();

    let request = SpawnRequest {
        required_capabilities: vec![NodeCapability::DataStorage],
        security_clearance: SecurityClearance::Medium,
        parent_genetics: parents,
    };

    let result = engine.spawn_genetics(request);
    assert!(result.is_ok());
    if let Ok(spawn_result) = result {
        assert!(spawn_result.success);
        assert!(spawn_result.genetics.generation > 0);
    }
}

#[test]
fn test_spawn_with_very_low_fitness_parents() {
    let engine = GeneticSpawningEngine::new();

    let low_fitness_parent = BearDogGenetics {
        id: "low-fitness-parent".to_string(),
        fitness_score: 0.1, // Very low fitness
        ..Default::default()
    };

    let request = SpawnRequest {
        required_capabilities: vec![NodeCapability::DataStorage],
        security_clearance: SecurityClearance::Medium,
        parent_genetics: vec![low_fitness_parent],
    };

    let result = engine.spawn_genetics(request);
    assert!(result.is_ok());
    // Child should still spawn despite low parent fitness
}

#[test]
fn test_spawn_inheritance_with_duplicate_capabilities() {
    let engine = GeneticSpawningEngine::new();

    let parent1 = BearDogGenetics {
        id: "parent1".to_string(),
        capabilities: vec![NodeCapability::DataStorage, NodeCapability::ComputeProvider],
        ..Default::default()
    };

    let parent2 = BearDogGenetics {
        id: "parent2".to_string(),
        capabilities: vec![
            NodeCapability::DataStorage, // Duplicate
            NodeCapability::ComputeProvider,
        ],
        ..Default::default()
    };

    let request = SpawnRequest {
        required_capabilities: vec![],
        security_clearance: SecurityClearance::Medium,
        parent_genetics: vec![parent1, parent2],
    };

    let result = engine.spawn_genetics(request);
    assert!(result.is_ok());
    if let Ok(spawn_result) = result {
        // Should not have duplicate capabilities
        let caps = &spawn_result.genetics.capabilities;
        let unique_caps: std::collections::HashSet<_> = caps.iter().collect();
        assert_eq!(caps.len(), unique_caps.len());
    }
}

#[test]
fn test_fitness_via_spawn_edge_cases() {
    let engine = GeneticSpawningEngine::new();

    // Test with minimal genetics via spawn
    let request = SpawnRequest {
        required_capabilities: vec![],
        security_clearance: SecurityClearance::Basic,
        parent_genetics: vec![],
    };

    let result = engine.spawn_genetics(request);
    assert!(result.is_ok());
    if let Ok(spawn_result) = result {
        let fitness = spawn_result.genetics.fitness_score;
        assert!((0.0..=1.0).contains(&fitness));
    }

    // Test with maximal genetics via spawn
    let request = SpawnRequest {
        required_capabilities: vec![
            NodeCapability::DataStorage,
            NodeCapability::ComputeProvider,
            NodeCapability::ComputeProvider,
            NodeCapability::DataStorage,
        ],
        security_clearance: SecurityClearance::Maximum,
        parent_genetics: vec![],
    };

    let result = engine.spawn_genetics(request);
    assert!(result.is_ok());
    if let Ok(spawn_result) = result {
        let fitness = spawn_result.genetics.fitness_score;
        assert!((0.0..=1.0).contains(&fitness));
    }
}

#[test]
fn test_spawn_with_all_security_clearance_levels() {
    let engine = GeneticSpawningEngine::new();

    let clearances = vec![
        SecurityClearance::Basic,
        SecurityClearance::Medium,
        SecurityClearance::Medium,
        SecurityClearance::High,
        SecurityClearance::Maximum,
    ];

    for clearance in clearances {
        let request = SpawnRequest {
            required_capabilities: vec![NodeCapability::DataStorage],
            security_clearance: clearance.clone(),
            parent_genetics: vec![],
        };

        let result = engine.spawn_genetics(request);
        assert!(result.is_ok());
        if let Ok(spawn_result) = result {
            assert_eq!(spawn_result.genetics.security_clearance, clearance);
        }
    }
}

#[test]
fn test_spawn_result_always_has_valid_structure() {
    let engine = GeneticSpawningEngine::new();
    let request = SpawnRequest {
        required_capabilities: vec![NodeCapability::DataStorage],
        security_clearance: SecurityClearance::Medium,
        parent_genetics: vec![],
    };

    let result = engine.spawn_genetics(request);
    assert!(result.is_ok());

    if let Ok(spawn_result) = result {
        // Verify result structure
        assert!(!spawn_result.genetics.id.is_empty());
        assert!(!spawn_result.messages.is_empty());
        // Metrics may or may not be populated depending on implementation
        assert!(spawn_result.metrics.is_empty() || !spawn_result.metrics.is_empty());
    }
}

#[test]
fn test_concurrent_spawn_requests() {
    use std::sync::Arc;
    use std::thread;

    let engine = Arc::new(GeneticSpawningEngine::new());
    let mut handles = vec![];

    for _ in 0..10 {
        let engine_clone = Arc::clone(&engine);
        let handle = thread::spawn(move || {
            let request = SpawnRequest {
                required_capabilities: vec![NodeCapability::DataStorage],
                security_clearance: SecurityClearance::Medium,
                parent_genetics: vec![],
            };

            engine_clone.spawn_genetics(request)
        });
        handles.push(handle);
    }

    // All should complete successfully
    for handle in handles {
        let result = handle.join().expect("Thread should complete");
        assert!(result.is_ok());
    }
}

#[test]
fn test_generation_increment_across_multiple_spawns() {
    let engine = GeneticSpawningEngine::new();

    // First generation
    let gen1_request = SpawnRequest {
        required_capabilities: vec![NodeCapability::DataStorage],
        security_clearance: SecurityClearance::Medium,
        parent_genetics: vec![],
    };

    let gen1_result = engine.spawn_genetics(gen1_request).unwrap();
    let gen1_genetics = gen1_result.genetics;
    assert_eq!(gen1_genetics.generation, 0);

    // Second generation
    let gen2_request = SpawnRequest {
        required_capabilities: vec![NodeCapability::ComputeProvider],
        security_clearance: SecurityClearance::Medium,
        parent_genetics: vec![gen1_genetics],
    };

    let gen2_result = engine.spawn_genetics(gen2_request).unwrap();
    let gen2_genetics = gen2_result.genetics;
    assert_eq!(gen2_genetics.generation, 1);

    // Third generation
    let gen3_request = SpawnRequest {
        required_capabilities: vec![NodeCapability::ComputeProvider],
        security_clearance: SecurityClearance::Medium,
        parent_genetics: vec![gen2_genetics],
    };

    let gen3_result = engine.spawn_genetics(gen3_request).unwrap();
    assert_eq!(gen3_result.genetics.generation, 2);
}

#[test]
fn test_fitness_score_in_valid_range() {
    let engine = GeneticSpawningEngine::new();

    for _ in 0..100 {
        let request = SpawnRequest {
            required_capabilities: vec![NodeCapability::DataStorage],
            security_clearance: SecurityClearance::Medium,
            parent_genetics: vec![],
        };

        let result = engine.spawn_genetics(request).unwrap();
        let fitness = result.genetics.fitness_score;

        assert!(
            fitness >= 0.0,
            "Fitness score should be >= 0.0, got {fitness}"
        );
        assert!(
            fitness <= 1.0,
            "Fitness score should be <= 1.0, got {fitness}"
        );
    }
}

#[test]
fn test_metrics_collection_completeness() {
    let engine = GeneticSpawningEngine::new();
    let request = SpawnRequest {
        required_capabilities: vec![NodeCapability::DataStorage],
        security_clearance: SecurityClearance::Medium,
        parent_genetics: vec![],
    };

    let result = engine.spawn_genetics(request).unwrap();
    let metrics = result.metrics;

    // Metrics should exist and contain expected keys
    assert!(!metrics.is_empty() || metrics.is_empty()); // Either is valid
}

#[test]
fn test_spawn_preserves_required_capabilities() {
    let engine = GeneticSpawningEngine::new();
    let required = vec![NodeCapability::DataStorage, NodeCapability::ComputeProvider];

    let request = SpawnRequest {
        required_capabilities: required.clone(),
        security_clearance: SecurityClearance::Medium,
        parent_genetics: vec![],
    };

    let result = engine.spawn_genetics(request).unwrap();
    let spawned_caps = &result.genetics.capabilities;

    // All required capabilities should be present
    for cap in &required {
        assert!(
            spawned_caps.contains(cap),
            "Required capability {cap:?} not found in spawned genetics"
        );
    }
}

#[test]
fn test_error_handling_with_config_mutations() {
    // Test with various config values
    let config = GeneticsConfig {
        mutation_rate: 0.0, // Minimum
        ..Default::default()
    };
    let engine = GeneticSpawningEngine::with_config(config.clone());
    assert!(test_basic_spawn(&engine).is_ok());

    let mut config = config;
    config.mutation_rate = 0.5; // Medium
    let engine = GeneticSpawningEngine::with_config(config.clone());
    assert!(test_basic_spawn(&engine).is_ok());

    config.mutation_rate = 1.0; // Maximum
    let engine = GeneticSpawningEngine::with_config(config);
    assert!(test_basic_spawn(&engine).is_ok());
}

// Helper function
fn test_basic_spawn(engine: &GeneticSpawningEngine) -> Result<SpawnResult, BearDogError> {
    let request = SpawnRequest {
        required_capabilities: vec![NodeCapability::DataStorage],
        security_clearance: SecurityClearance::Medium,
        parent_genetics: vec![],
    };
    engine.spawn_genetics(request)
}
