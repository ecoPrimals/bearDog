// SPDX-License-Identifier: AGPL-3.0-only

// Genetics Types and Configurations
//
// This module provides genetics-related types and configurations for the BearDog ecosystem.

use serde::{Deserialize, Serialize};
// Removed unused HashMap import

/// Genetic algorithm configuration
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct GeneticConfig {
    /// Population Size
    /// Number of `population_size`
    pub population_size: usize,
    /// Mutation Rate
    /// The mutation rate value
    pub mutation_rate: f64,
    /// Crossover Rate
    /// The crossover rate value
    pub crossover_rate: f64,
    /// Elite Percentage
    /// The elite percentage value
    pub elite_percentage: f64,
    /// Max Generations
    /// Number of `max_generations`
    pub max_generations: usize,
}

/// Genetic optimization parameters
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct GeneticOptimization {
    /// Target Fitness
    /// The target fitness value
    pub target_fitness: f64,
    /// Convergence Threshold
    /// The convergence threshold value
    pub convergence_threshold: f64,
    /// Diversity Threshold
    /// The diversity threshold value
    pub diversity_threshold: f64,
    /// Adaptive Parameters
    /// Whether `adaptive_parameters` is enabled
    pub adaptive_parameters: bool,
}

/// Genetic algorithm result
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct GeneticResult {
    /// Best Fitness
    /// The best fitness value
    pub best_fitness: f64,
    /// Generation
    /// Number of generation
    pub generation: usize,
    /// Converged
    /// Whether converged is enabled
    pub converged: bool,
    /// Execution Time Ms
    pub execution_time_ms: u64,
}

impl Default for GeneticConfig {
    fn default() -> Self {
        Self {
            population_size: 100,
            mutation_rate: 0.01,
            crossover_rate: 0.8,
            elite_percentage: 0.1,
            max_generations: 1000,
        }
    }
}

impl Default for GeneticOptimization {
    fn default() -> Self {
        Self {
            target_fitness: 0.95,
            convergence_threshold: 0.001,
            diversity_threshold: 0.1,
            adaptive_parameters: true,
        }
    }
}

// Re-export canonical genetics types
pub use crate::canonical::genetics::*;
