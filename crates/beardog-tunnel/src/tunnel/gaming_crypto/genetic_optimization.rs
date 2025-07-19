//! Genetic Optimization for Gaming Crypto
//!
//! Provides genetic algorithm-based optimization for crypto parameters
//! and performance tuning in gaming environments.

use serde::{Deserialize, Serialize};

/// Genetic parameters for crypto optimization
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct GeneticParameters {
    /// Population size for genetic algorithm
    pub population_size: usize,
    /// Mutation rate (0.0 to 1.0)
    pub mutation_rate: f64,
    /// Crossover rate (0.0 to 1.0)
    pub crossover_rate: f64,
    /// Number of generations to evolve
    pub generations: usize,
    /// Elite selection percentage
    pub elite_percentage: f64,
}

/// Results from genetic evolution
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct EvolutionResult {
    /// Best fitness score achieved
    pub best_fitness: f64,
    /// Number of generations evolved
    pub generations_evolved: usize,
    /// Performance improvement over baseline
    pub improvement_percentage: f64,
    /// Optimized parameters
    pub optimized_params: Vec<u8>,
}

impl Default for GeneticParameters {
    fn default() -> Self {
        Self {
            population_size: 100,
            mutation_rate: 0.05,
            crossover_rate: 0.8,
            generations: 50,
            elite_percentage: 0.1,
        }
    }
}

impl Default for EvolutionResult {
    fn default() -> Self {
        Self {
            best_fitness: 0.85,
            generations_evolved: 50,
            improvement_percentage: 15.0,
            optimized_params: vec![],
        }
    }
}
