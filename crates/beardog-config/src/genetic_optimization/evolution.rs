//! Evolution Strategies for Genetic Algorithms
//!
//! This module handles evolution strategy configurations.

use serde::{Deserialize, Serialize};

/// Evolution strategies configuration
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct EvolutionStrategiesConfig {
    /// Maximum number of generations
    pub max_generations: u32,
    /// Selection strategy
    pub selection_strategy: SelectionStrategy,
    /// Crossover strategy
    pub crossover_strategy: CrossoverStrategy,
    /// Mutation strategy
    pub mutation_strategy: MutationStrategy,
    /// Adaptive strategies enabled
    pub adaptive_strategies: bool,
    /// Multi-objective optimization enabled
    pub multi_objective: bool,
}

/// Selection strategy
#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum SelectionStrategy {
    /// Tournament selection
    Tournament,
    /// Roulette wheel selection
    RouletteWheel,
    /// Rank selection
    Rank,
    /// Elitist selection
    Elitist,
}

/// Crossover strategy
#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum CrossoverStrategy {
    /// Single-point crossover
    SinglePoint,
    /// Two-point crossover
    TwoPoint,
    /// Uniform crossover
    Uniform,
    /// Arithmetic crossover
    Arithmetic,
}

/// Mutation strategy
#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum MutationStrategy {
    /// Gaussian mutation
    Gaussian,
    /// Uniform mutation
    Uniform,
    /// Polynomial mutation
    Polynomial,
    /// Bit flip mutation
    BitFlip,
}

impl Default for EvolutionStrategiesConfig {
    fn default() -> Self {
        Self {
            max_generations: 100,
            selection_strategy: SelectionStrategy::Tournament,
            crossover_strategy: CrossoverStrategy::SinglePoint,
            mutation_strategy: MutationStrategy::Gaussian,
            adaptive_strategies: false,
            multi_objective: false,
        }
    }
}

impl EvolutionStrategiesConfig {
    /// Create production evolution strategies configuration
    pub fn production() -> Self {
        Self {
            max_generations: 1000,
            selection_strategy: SelectionStrategy::Tournament,
            crossover_strategy: CrossoverStrategy::Uniform,
            mutation_strategy: MutationStrategy::Polynomial,
            adaptive_strategies: true,
            multi_objective: true,
        }
    }

    /// Create development evolution strategies configuration
    pub fn development() -> Self {
        Self {
            max_generations: 50,
            selection_strategy: SelectionStrategy::RouletteWheel,
            crossover_strategy: CrossoverStrategy::SinglePoint,
            mutation_strategy: MutationStrategy::Gaussian,
            adaptive_strategies: false,
            multi_objective: false,
        }
    }
}
