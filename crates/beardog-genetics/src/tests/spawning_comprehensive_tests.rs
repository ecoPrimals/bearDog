// SPDX-License-Identifier: AGPL-3.0-only

//! Comprehensive Spawning Engine Tests
//!
//! Exhaustive tests for genetic spawning engine functionality

use crate::genetics::spawning::{GeneticSpawningEngine, SpawnRequest};
use beardog_auth::auth::{BearDogGenetics, NodeCapability, SecurityClearance};
use beardog_errors::BearDogError;
use beardog_types::canonical::config::GeneticsConfig;

// ============================================================================
// Engine Creation and Configuration Tests
// ============================================================================

#[tokio::test]
async fn test_spawning_engine_new() {
    let engine = GeneticSpawningEngine::new();
    // Just verify engine is created successfully
    assert!(engine.get_config().mutation_rate >= 0.0);
}

#[tokio::test]
async fn test_spawning_engine_with_custom_config() {
    let config = GeneticsConfig {
        enabled: true,
        population_size: 50,
        mutation_rate: 0.05,
        crossover_rate: 0.8,
        max_generations: 100,
    };

    let engine = GeneticSpawningEngine::with_config(config);
    assert!(engine.get_config().enabled);
    assert_eq!(engine.get_config().population_size, 50);
}

#[tokio::test]
async fn test_spawning_engine_default() {
    let engine = GeneticSpawningEngine::default();
    // Just verify engine is created successfully
    assert!(engine.get_config().crossover_rate >= 0.0);
}

// ============================================================================
// Basic Spawning Tests
// ============================================================================

#[tokio::test]
async fn test_spawn_with_single_capability() -> Result<(), BearDogError> {
    let engine = GeneticSpawningEngine::new();

    let request = SpawnRequest {
        required_capabilities: vec![NodeCapability::ComputeProvider],
        security_clearance: SecurityClearance::Basic,
        parent_genetics: vec![],
    };

    let result = engine.spawn_genetics(request)?;

    assert!(result.success);
    assert!(!result.genetics.id.is_empty());
    assert_eq!(result.genetics.capabilities.len(), 1);
    assert!(result.genetics.fitness_score > 0.0);
    assert!(!result.messages.is_empty());

    Ok(())
}

#[tokio::test]
async fn test_spawn_with_multiple_capabilities() -> Result<(), BearDogError> {
    let engine = GeneticSpawningEngine::new();

    let request = SpawnRequest {
        required_capabilities: vec![
            NodeCapability::ComputeProvider,
            NodeCapability::SecurityAnalysis,
            NodeCapability::DataStorage,
        ],
        security_clearance: SecurityClearance::High,
        parent_genetics: vec![],
    };

    let result = engine.spawn_genetics(request)?;

    assert!(result.success);
    assert_eq!(result.genetics.capabilities.len(), 3);
    assert!(result.genetics.fitness_score > 0.0);

    Ok(())
}

#[tokio::test]
async fn test_spawn_with_high_security_clearance() -> Result<(), BearDogError> {
    let engine = GeneticSpawningEngine::new();

    let request = SpawnRequest {
        required_capabilities: vec![NodeCapability::SecurityAnalysis],
        security_clearance: SecurityClearance::Maximum,
        parent_genetics: vec![],
    };

    let result = engine.spawn_genetics(request)?;

    assert!(result.success);
    assert_eq!(
        result.genetics.security_clearance,
        SecurityClearance::Maximum
    );

    Ok(())
}

// ============================================================================
// Inheritance Tests
// ============================================================================

#[tokio::test]
async fn test_spawn_with_single_parent() -> Result<(), BearDogError> {
    let engine = GeneticSpawningEngine::new();

    let parent = BearDogGenetics {
        id: "parent-1".to_string(),
        capabilities: vec![NodeCapability::DataStorage],
        security_clearance: SecurityClearance::High,
        generation: 0,
        fitness_score: 0.85,
        ..Default::default()
    };

    let request = SpawnRequest {
        required_capabilities: vec![NodeCapability::ComputeProvider],
        security_clearance: SecurityClearance::Basic,
        parent_genetics: vec![parent],
    };

    let result = engine.spawn_genetics(request)?;

    assert!(result.success);
    assert_eq!(result.genetics.generation, 1);
    assert!(result.genetics.capabilities.len() >= 2);
    assert!(result
        .genetics
        .capabilities
        .contains(&NodeCapability::DataStorage));
    assert!(result
        .genetics
        .capabilities
        .contains(&NodeCapability::ComputeProvider));

    Ok(())
}

#[tokio::test]
async fn test_spawn_with_multiple_parents() -> Result<(), BearDogError> {
    let engine = GeneticSpawningEngine::new();

    let parent1 = BearDogGenetics {
        id: "parent-1".to_string(),
        capabilities: vec![NodeCapability::DataStorage],
        generation: 0,
        fitness_score: 0.8,
        ..Default::default()
    };

    let parent2 = BearDogGenetics {
        id: "parent-2".to_string(),
        capabilities: vec![NodeCapability::SecurityAnalysis],
        generation: 0,
        fitness_score: 0.9,
        ..Default::default()
    };

    let request = SpawnRequest {
        required_capabilities: vec![NodeCapability::ComputeProvider],
        security_clearance: SecurityClearance::Basic,
        parent_genetics: vec![parent1, parent2],
    };

    let result = engine.spawn_genetics(request)?;

    assert!(result.success);
    assert_eq!(result.genetics.generation, 1);
    assert!(result.genetics.capabilities.len() >= 3);

    Ok(())
}

#[tokio::test]
async fn test_spawn_multi_generation() -> Result<(), BearDogError> {
    let engine = GeneticSpawningEngine::new();

    // Generation 0
    let gen0 = BearDogGenetics {
        id: "gen-0".to_string(),
        capabilities: vec![NodeCapability::DataStorage],
        generation: 0,
        fitness_score: 0.7,
        ..Default::default()
    };

    // Generation 1
    let request1 = SpawnRequest {
        required_capabilities: vec![NodeCapability::ComputeProvider],
        security_clearance: SecurityClearance::Basic,
        parent_genetics: vec![gen0],
    };
    let result1 = engine.spawn_genetics(request1)?;
    assert_eq!(result1.genetics.generation, 1);

    // Generation 2
    let request2 = SpawnRequest {
        required_capabilities: vec![NodeCapability::SecurityAnalysis],
        security_clearance: SecurityClearance::Basic,
        parent_genetics: vec![result1.genetics],
    };
    let result2 = engine.spawn_genetics(request2)?;
    assert_eq!(result2.genetics.generation, 2);

    Ok(())
}

// ============================================================================
// Fitness and Metrics Tests
// ============================================================================

#[tokio::test]
async fn test_fitness_score_calculation() -> Result<(), BearDogError> {
    let engine = GeneticSpawningEngine::new();

    let request = SpawnRequest {
        required_capabilities: vec![
            NodeCapability::ComputeProvider,
            NodeCapability::DataStorage,
            NodeCapability::SecurityAnalysis,
        ],
        security_clearance: SecurityClearance::High,
        parent_genetics: vec![],
    };

    let result = engine.spawn_genetics(request)?;

    assert!(result.genetics.fitness_score > 0.0);
    assert!(result.genetics.fitness_score <= 1.0);

    Ok(())
}

#[tokio::test]
async fn test_spawn_result_metrics() -> Result<(), BearDogError> {
    let engine = GeneticSpawningEngine::new();

    let request = SpawnRequest {
        required_capabilities: vec![NodeCapability::ComputeProvider],
        security_clearance: SecurityClearance::Basic,
        parent_genetics: vec![],
    };

    let result = engine.spawn_genetics(request)?;

    // Verify result has metrics (even if empty)
    assert!(result.success);

    Ok(())
}

// ============================================================================
// Edge Cases and Error Handling Tests
// ============================================================================

#[tokio::test]
async fn test_spawn_with_empty_capabilities() -> Result<(), BearDogError> {
    let engine = GeneticSpawningEngine::new();

    let request = SpawnRequest {
        required_capabilities: vec![],
        security_clearance: SecurityClearance::Basic,
        parent_genetics: vec![],
    };

    let result = engine.spawn_genetics(request)?;

    assert!(result.success);
    assert!(result.genetics.capabilities.is_empty());

    Ok(())
}

#[tokio::test]
async fn test_spawn_preserves_unique_capabilities() -> Result<(), BearDogError> {
    let engine = GeneticSpawningEngine::new();

    let parent = BearDogGenetics {
        id: "parent-1".to_string(),
        capabilities: vec![NodeCapability::ComputeProvider, NodeCapability::DataStorage],
        generation: 0,
        fitness_score: 0.8,
        ..Default::default()
    };

    let request = SpawnRequest {
        required_capabilities: vec![NodeCapability::ComputeProvider], // Duplicate
        security_clearance: SecurityClearance::Basic,
        parent_genetics: vec![parent],
    };

    let result = engine.spawn_genetics(request)?;

    assert!(result.success);
    // Should not have duplicate ComputeProvider
    let compute_count = result
        .genetics
        .capabilities
        .iter()
        .filter(|c| matches!(c, NodeCapability::ComputeProvider))
        .count();
    assert_eq!(compute_count, 1);

    Ok(())
}

#[tokio::test]
async fn test_spawn_result_always_has_messages() -> Result<(), BearDogError> {
    let engine = GeneticSpawningEngine::new();

    let request = SpawnRequest {
        required_capabilities: vec![NodeCapability::ComputeProvider],
        security_clearance: SecurityClearance::Basic,
        parent_genetics: vec![],
    };

    let result = engine.spawn_genetics(request)?;

    assert!(!result.messages.is_empty());
    assert!(result.messages[0].contains("success"));

    Ok(())
}

// ============================================================================
// Configuration Consistency Tests
// ============================================================================

#[tokio::test]
async fn test_engine_config_access() {
    let config = GeneticsConfig {
        enabled: true,
        population_size: 250,
        mutation_rate: 0.02,
        crossover_rate: 0.75,
        max_generations: 500,
    };

    let engine = GeneticSpawningEngine::with_config(config.clone());

    assert_eq!(engine.get_config().enabled, config.enabled);
    assert_eq!(engine.get_config().population_size, config.population_size);
    assert_eq!(engine.get_config().mutation_rate, config.mutation_rate);
}

#[tokio::test]
async fn test_spawning_with_all_capabilities() -> Result<(), BearDogError> {
    let engine = GeneticSpawningEngine::new();

    let all_capabilities = vec![
        NodeCapability::ComputeProvider,
        NodeCapability::DataStorage,
        NodeCapability::SecurityAnalysis,
        NodeCapability::ThreatDetection,
        NodeCapability::CryptographicAuditing,
    ];

    let request = SpawnRequest {
        required_capabilities: all_capabilities.clone(),
        security_clearance: SecurityClearance::Maximum,
        parent_genetics: vec![],
    };

    let result = engine.spawn_genetics(request)?;

    assert!(result.success);
    assert_eq!(result.genetics.capabilities.len(), all_capabilities.len());

    Ok(())
}
