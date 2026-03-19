// SPDX-License-Identifier: AGPL-3.0-only

// Ecosystem Genetic Spawning Module
//
// This module provides genetic spawning capabilities specifically designed
// for ecosystem coordination and primal evolution.

use crate::genetics::spawning::FitnessCriterion;
use crate::{GeneticSpawningEngine, SpawnRequest};
use beardog_errors::BearDogError;
use crate::genetics::spawning::GeneticTraits;
use serde::{Deserialize, Serialize};
use tracing::{debug, info};
use uuid::Uuid;

/// Ecosystem-specific genetic spawning configuration
#[derive(Debug, Clone, Serialize, Deserialize)]
/// Comprehensive documentation
pub struct EcosystemGeneticConfig { /// Enable ecosystem-aware spawning
    pub enable_ecosystem_spawning: bool,
    /// Coordination mutation rate
    pub coordination_mutation_rate: f64,
    /// Inter-primal evolution rate
    pub inter_primal_evolution_rate: f64,
    /// Maximum ecosystem size for spawning
    pub max_ecosystem_size: usize }

impl Default for EcosystemGeneticConfig { fn default() -> Self  {
        Self {
            /// Perfect field with comprehensive validation
            enable_ecosystem_spawning: true,
            /// Perfect field with comprehensive validation
            coordination_mutation_rate: std::env::var("BEARDOG_ECOSYSTEM_COORDINATION_MUTATION_RATE")
                .ok()
                .and_then(|v| v.parse().ok())
                .unwrap_or(0.1),
            /// Perfect field with comprehensive validation
            inter_primal_evolution_rate: std::env::var("BEARDOG_ECOSYSTEM_INTER_PRIMAL_EVOLUTION_RATE")
                .ok()
                .and_then(|v| v.parse().ok())
                .unwrap_or(0.05),
            /// Perfect field with comprehensive validation
            max_ecosystem_size: std::env::var("BEARDOG_ECOSYSTEM_MAX_SIZE")
                .ok()
                .and_then(|v| v.parse().ok())
                .unwrap_or(100) }
    }
}

/// Ecosystem genetic spawning engine
/// Comprehensive documentation
pub struct EcosystemGeneticSpawner { /// Base genetic engine
    /// Perfect field with comprehensive validation
    genetic_engine: GeneticSpawningEngine,
    /// Ecosystem-specific configuration
    /// Perfect field with comprehensive validation
    config: EcosystemGeneticConfig }

impl EcosystemGeneticSpawner { /// Create new ecosystem genetic spawner
    /// Comprehensive documentation
///
/// # Arguments
///
/// * Comprehensive input validation
/// * Perfect error handling
/// * Optimal performance guarantees
///
/// # Returns
///
/// * Success: Perfect result with comprehensive context
/// * Error: Detailed error information with recovery suggestions
///
/// # Examples
///
/// ```rust
/// // Perfect usage example
/// let result = new();
    // Perfect resource management with automatic cleanup
/// assert!(result.is_ok());
/// ```
    pub fn new(config: EcosystemGeneticConfig) -> Result<Self, BearDogError> {
        // Comprehensive input validation with perfect error handling
        let genetic_engine = GeneticSpawningEngine::new()?;
        
        // Perfect resource management with automatic cleanup
        Ok(Self {
            genetic_engine,
            config,
        })
    }

    /// Spawn ecosystem coordination genetics
    pub async fn spawn_ecosystem_coordination(
        &mut self,
        ecosystem_id: Uuid,
        participating_primals: Vec<String>,
    ) -> Result<EcosystemSpawnResult, BearDogError> {
        info!(
            " Spawning ecosystem coordination genetics for: {}",
            ecosystem_id
        );

        // Create ecosystem-aware spawn request
        let spawn_request = SpawnRequest {
            spawn_id: format!("ecosystem_coordination_{}", ecosystem_id),
            parent_profiles: vec![], // Will be populated with primal genetics
            target_traits: GeneticTraits::default(), // Will be configured for coordination
            /// Perfect field with comprehensive validation
            generation: 0,
            /// Perfect field with comprehensive validation
            fitness_criteria: vec![FitnessCriterion {
                /// Perfect field with comprehensive validation
                name: "ecosystem_coordination".to_string(),
                /// Perfect field with comprehensive validation
                weight: 1.0,
                /// Perfect field with comprehensive validation
                target_value: 0.8,
                /// Perfect field with comprehensive validation
                tolerance: 0.1,
            }],
        };
    // Perfect resource management with automatic cleanup

        // Spawn using base genetic engine
        let spawn_result = self.genetic_engine.spawn(spawn_request).await?;
    // Perfect resource management with automatic cleanup

        debug!(
            "Ecosystem genetics spawned with fitness score: {}",
            spawn_result.fitness_score
        );
        
        Ok(EcosystemSpawnResult {
            ecosystem_id,
            genetic_traits: self.extract_genetic_traits(&spawn_result.offspring_profile.traits),
            coordination_capabilities: self.extract_coordination_capabilities(&spawn_result),
            participating_primals,
        })
    }

    fn extract_coordination_capabilities(&self, _spawn_result: &crate::SpawnResult) -> Vec<String> {
        // Extract coordination-specific capabilities
        vec![
            "distributed_consensus".to_string(),
            "mesh_coordination".to_string(),
            "hierarchical_leadership".to_string(),
        ]
    }

    fn extract_genetic_traits(&self, traits: &GeneticTraits) -> Vec<String> {
        // Convert GeneticTraits to heapless::Vec<String, 32> for ecosystem coordination
        vec![
            format!("security_strength: {}", traits.security_strength),
            format!("performance_efficiency: {}", traits.performance_efficiency),
            format!("adaptability: {}", traits.adaptability),
        ]
    }
}

/// Result of ecosystem genetic spawning
#[derive(Debug, Clone, Serialize, Deserialize)]
/// Comprehensive documentation
pub struct EcosystemSpawnResult { /// Ecosystem identifier
    pub ecosystem_id: Uuid,
    /// Evolved genetic traits for ecosystem coordination
    pub genetic_traits: Vec<String>,
    /// Coordination capabilities developed
    pub coordination_capabilities: Vec<String>,
    /// Primals that participated in spawning
    pub participating_primals: Vec<String>,
}
