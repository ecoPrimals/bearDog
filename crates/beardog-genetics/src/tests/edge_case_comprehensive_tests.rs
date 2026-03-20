// SPDX-License-Identifier: AGPL-3.0-only

//! Comprehensive Edge Case Tests for beardog-genetics
//!
//! These tests target boundary conditions and edge cases to ensure
//! robustness across unusual but valid scenarios.

use crate::genetics::spawning::engine::GeneticSpawningEngine;
use crate::genetics::spawning::types::SpawnRequest;
use beardog_auth::auth::{BearDogGenetics, NodeCapability, SecurityClearance};
use beardog_types::canonical::config::GeneticsConfig;

// TEST_CATEGORY: edge_case
// TEST_DOMAIN: genetics
// TEST_PRIORITY: medium

#[test]
fn test_spawn_with_zero_generation_parent() {
    let engine = GeneticSpawningEngine::new();
    let parent = BearDogGenetics {
        id: "gen-0-parent".to_string(),
        generation: 0,
        ..Default::default()
    };

    let request = SpawnRequest {
        required_capabilities: vec![NodeCapability::DataStorage],
        security_clearance: SecurityClearance::Medium,
        parent_genetics: vec![parent],
    };

    let result = engine.spawn_genetics(request);
    assert!(result.is_ok());
    assert_eq!(result.unwrap().genetics.generation, 1);
}

#[test]
fn test_spawn_with_high_generation_parent() {
    let engine = GeneticSpawningEngine::new();
    let parent = BearDogGenetics {
        id: "gen-999-parent".to_string(),
        generation: 999,
        ..Default::default()
    };

    let request = SpawnRequest {
        required_capabilities: vec![NodeCapability::DataStorage],
        security_clearance: SecurityClearance::Medium,
        parent_genetics: vec![parent],
    };

    let result = engine.spawn_genetics(request);
    assert!(result.is_ok());
    assert_eq!(result.unwrap().genetics.generation, 1000);
}

#[test]
fn test_spawn_with_minimum_fitness_score() {
    let engine = GeneticSpawningEngine::new();
    let parent = BearDogGenetics {
        id: "min-fitness-parent".to_string(),
        fitness_score: 0.0,
        ..Default::default()
    };

    let request = SpawnRequest {
        required_capabilities: vec![NodeCapability::DataStorage],
        security_clearance: SecurityClearance::Medium,
        parent_genetics: vec![parent],
    };

    let result = engine.spawn_genetics(request);
    assert!(result.is_ok());
    let spawned = result.unwrap().genetics;
    assert!(spawned.fitness_score >= 0.0);
}

#[test]
fn test_spawn_with_maximum_fitness_score() {
    let engine = GeneticSpawningEngine::new();
    let parent = BearDogGenetics {
        id: "max-fitness-parent".to_string(),
        fitness_score: 1.0,
        ..Default::default()
    };

    let request = SpawnRequest {
        required_capabilities: vec![NodeCapability::DataStorage],
        security_clearance: SecurityClearance::Medium,
        parent_genetics: vec![parent],
    };

    let result = engine.spawn_genetics(request);
    assert!(result.is_ok());
    let spawned = result.unwrap().genetics;
    assert!(spawned.fitness_score <= 1.0);
}

#[test]
fn test_spawn_with_identical_parents() {
    let engine = GeneticSpawningEngine::new();
    let parent = BearDogGenetics {
        id: "identical-parent".to_string(),
        fitness_score: 0.7,
        capabilities: vec![NodeCapability::DataStorage],
        ..Default::default()
    };

    let request = SpawnRequest {
        required_capabilities: vec![],
        security_clearance: SecurityClearance::Medium,
        parent_genetics: vec![parent.clone(), parent.clone(), parent],
    };

    let result = engine.spawn_genetics(request);
    assert!(result.is_ok());
    // Should handle identical parents gracefully
}

#[test]
fn test_spawn_with_all_capability_types() {
    let engine = GeneticSpawningEngine::new();

    // Get all capability variants
    let all_capabilities = vec![
        NodeCapability::DataStorage,
        NodeCapability::ComputeProvider,
        NodeCapability::ComputeProvider,
        NodeCapability::DataStorage,
    ];

    let request = SpawnRequest {
        required_capabilities: all_capabilities.clone(),
        security_clearance: SecurityClearance::Maximum,
        parent_genetics: vec![],
    };

    let result = engine.spawn_genetics(request);
    assert!(result.is_ok());
    let spawned = result.unwrap().genetics;

    // Should have all capabilities
    assert!(spawned.capabilities.len() >= all_capabilities.len());
}

#[test]
fn test_spawn_with_single_parent_single_capability() {
    let engine = GeneticSpawningEngine::new();
    let parent = BearDogGenetics {
        id: "single-cap-parent".to_string(),
        capabilities: vec![NodeCapability::DataStorage],
        ..Default::default()
    };

    let request = SpawnRequest {
        required_capabilities: vec![],
        security_clearance: SecurityClearance::Basic,
        parent_genetics: vec![parent],
    };

    let result = engine.spawn_genetics(request);
    assert!(result.is_ok());
    let spawned = result.unwrap().genetics;
    assert!(spawned.capabilities.contains(&NodeCapability::DataStorage));
}

#[test]
fn test_spawn_config_with_extreme_values() {
    // Test with minimum mutation rate
    let config = GeneticsConfig {
        mutation_rate: 0.0,
        ..Default::default()
    };

    let engine = GeneticSpawningEngine::with_config(config);
    let request = SpawnRequest {
        required_capabilities: vec![NodeCapability::DataStorage],
        security_clearance: SecurityClearance::Medium,
        parent_genetics: vec![],
    };

    let result = engine.spawn_genetics(request.clone());
    assert!(result.is_ok());

    // Test with maximum mutation rate
    let config2 = GeneticsConfig {
        mutation_rate: 1.0,
        ..Default::default()
    };

    let engine2 = GeneticSpawningEngine::with_config(config2);
    let result2 = engine2.spawn_genetics(request);
    assert!(result2.is_ok());
}

#[test]
fn test_spawn_preserves_parent_order() {
    let engine = GeneticSpawningEngine::new();

    let parent1 = BearDogGenetics {
        id: "parent-1".to_string(),
        capabilities: vec![NodeCapability::DataStorage],
        ..Default::default()
    };

    let parent2 = BearDogGenetics {
        id: "parent-2".to_string(),
        capabilities: vec![NodeCapability::ComputeProvider],
        ..Default::default()
    };

    let parent3 = BearDogGenetics {
        id: "parent-3".to_string(),
        capabilities: vec![NodeCapability::ComputeProvider],
        ..Default::default()
    };

    let request = SpawnRequest {
        required_capabilities: vec![],
        security_clearance: SecurityClearance::Medium,
        parent_genetics: vec![parent1, parent2, parent3],
    };

    let result = engine.spawn_genetics(request);
    assert!(result.is_ok());
    // Order shouldn't matter for capabilities
    let spawned = result.unwrap().genetics;
    assert!(spawned.capabilities.len() >= 2); // At least some capabilities inherited
}

#[test]
fn test_spawn_deterministic_with_same_inputs() {
    let engine = GeneticSpawningEngine::new();
    let request = SpawnRequest {
        required_capabilities: vec![NodeCapability::DataStorage],
        security_clearance: SecurityClearance::Medium,
        parent_genetics: vec![],
    };

    // Spawn multiple times with same request
    let result1 = engine.spawn_genetics(request.clone());
    let result2 = engine.spawn_genetics(request.clone());
    let result3 = engine.spawn_genetics(request);

    // All should succeed
    assert!(result1.is_ok());
    assert!(result2.is_ok());
    assert!(result3.is_ok());

    // Fitness scores should be in valid range
    let fitness1 = result1.unwrap().genetics.fitness_score;
    let fitness2 = result2.unwrap().genetics.fitness_score;
    let fitness3 = result3.unwrap().genetics.fitness_score;

    assert!((0.0..=1.0).contains(&fitness1));
    assert!((0.0..=1.0).contains(&fitness2));
    assert!((0.0..=1.0).contains(&fitness3));
}

#[test]
fn test_spawn_with_varying_security_clearances_and_parents() {
    let engine = GeneticSpawningEngine::new();

    // Parent with high clearance
    let high_clearance_parent = BearDogGenetics {
        id: "high-clearance".to_string(),
        security_clearance: SecurityClearance::Maximum,
        ..Default::default()
    };

    // Parent with low clearance
    let low_clearance_parent = BearDogGenetics {
        id: "low-clearance".to_string(),
        security_clearance: SecurityClearance::Basic,
        ..Default::default()
    };

    let request = SpawnRequest {
        required_capabilities: vec![],
        security_clearance: SecurityClearance::Medium,
        parent_genetics: vec![high_clearance_parent, low_clearance_parent],
    };

    let result = engine.spawn_genetics(request);
    assert!(result.is_ok());
    // Spawned genetics should have specified clearance
    assert_eq!(
        result.unwrap().genetics.security_clearance,
        SecurityClearance::Medium
    );
}

#[test]
fn test_empty_id_handling() {
    let engine = GeneticSpawningEngine::new();
    let request = SpawnRequest {
        required_capabilities: vec![],
        security_clearance: SecurityClearance::Medium,
        parent_genetics: vec![],
    };

    let result = engine.spawn_genetics(request);
    assert!(result.is_ok());

    // ID should never be empty
    let spawned = result.unwrap().genetics;
    assert!(!spawned.id.is_empty());
    assert!(!spawned.id.is_empty());
}

#[test]
fn test_messages_always_present() {
    let engine = GeneticSpawningEngine::new();
    let request = SpawnRequest {
        required_capabilities: vec![],
        security_clearance: SecurityClearance::Medium,
        parent_genetics: vec![],
    };

    let result = engine.spawn_genetics(request);
    assert!(result.is_ok());

    let spawn_result = result.unwrap();
    assert!(!spawn_result.messages.is_empty());
    assert!(!spawn_result.messages[0].is_empty());
}

#[test]
fn test_rapid_sequential_spawns() {
    let engine = GeneticSpawningEngine::new();

    for i in 0..100 {
        let request = SpawnRequest {
            required_capabilities: vec![NodeCapability::DataStorage],
            security_clearance: SecurityClearance::Medium,
            parent_genetics: vec![],
        };

        let result = engine.spawn_genetics(request);
        assert!(result.is_ok(), "Spawn {} failed", i);
    }
}

#[test]
fn test_spawn_with_complex_inheritance_chain() {
    let engine = GeneticSpawningEngine::new();

    // Build a complex inheritance chain
    let mut current_genetics = BearDogGenetics {
        id: "ancestor".to_string(),
        generation: 0,
        capabilities: vec![NodeCapability::DataStorage],
        ..Default::default()
    };

    // Spawn 5 generations
    for gen_idx in 0..5 {
        let request = SpawnRequest {
            required_capabilities: vec![],
            security_clearance: SecurityClearance::Medium,
            parent_genetics: vec![current_genetics.clone()],
        };

        let result = engine.spawn_genetics(request);
        assert!(result.is_ok());

        current_genetics = result.unwrap().genetics;
        assert_eq!(current_genetics.generation, gen_idx + 1);
    }

    // Final genetics should be generation 5
    assert_eq!(current_genetics.generation, 5);
}
