//! Genetic Algorithm Optimization Configuration
//!
//! This module provides comprehensive genetic algorithm optimization configuration for BearDog,
//! including population management, evolution strategies, fitness evaluation, and performance optimization.

use serde::{Deserialize, Serialize};

pub mod evolution;
pub mod fitness;
pub mod operators;
pub mod parallel;
pub mod performance;
pub mod population;

pub use evolution::*;
pub use fitness::*;
pub use operators::*;
pub use parallel::*;
pub use performance::*;
pub use population::*;

/// Genetic algorithm optimization configuration
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct GeneticOptimizationConfig {
    /// Enable genetic optimization
    pub enabled: bool,
    /// Parallel processing configuration
    pub parallel_processing: GeneticParallelProcessingConfig,
    /// Population management configuration
    pub population_management: PopulationManagementConfig,
    /// Evolution strategy configuration
    pub evolution_strategies: EvolutionStrategiesConfig,
    /// Fitness evaluation configuration
    pub fitness_evaluation: FitnessEvaluationConfig,
    /// Genetic operators configuration
    pub genetic_operators: GeneticOperatorsConfig,
    /// Performance optimization configuration
    pub performance_optimization: GeneticPerformanceOptimizationConfig,
}

impl Default for GeneticOptimizationConfig {
    fn default() -> Self {
        Self {
            enabled: true,
            parallel_processing: GeneticParallelProcessingConfig::default(),
            population_management: PopulationManagementConfig::default(),
            evolution_strategies: EvolutionStrategiesConfig::default(),
            fitness_evaluation: FitnessEvaluationConfig::default(),
            genetic_operators: GeneticOperatorsConfig::default(),
            performance_optimization: GeneticPerformanceOptimizationConfig::default(),
        }
    }
}

impl GeneticOptimizationConfig {
    /// Create a production genetic optimization configuration
    pub fn production() -> Self {
        Self {
            enabled: true,
            parallel_processing: GeneticParallelProcessingConfig::production(),
            population_management: PopulationManagementConfig::production(),
            evolution_strategies: EvolutionStrategiesConfig::production(),
            fitness_evaluation: FitnessEvaluationConfig::production(),
            genetic_operators: GeneticOperatorsConfig::production(),
            performance_optimization: GeneticPerformanceOptimizationConfig::production(),
        }
    }

    /// Create a development genetic optimization configuration
    pub fn development() -> Self {
        Self {
            enabled: false,
            parallel_processing: GeneticParallelProcessingConfig::development(),
            population_management: PopulationManagementConfig::development(),
            evolution_strategies: EvolutionStrategiesConfig::development(),
            fitness_evaluation: FitnessEvaluationConfig::development(),
            genetic_operators: GeneticOperatorsConfig::development(),
            performance_optimization: GeneticPerformanceOptimizationConfig::development(),
        }
    }

    /// Validate the genetic optimization configuration
    pub fn validate(&self) -> Result<(), String> {
        if self.enabled {
            if self.parallel_processing.worker_threads == 0 {
                return Err("Worker threads must be greater than 0".to_string());
            }

            if self.population_management.initial_population_size == 0 {
                return Err("Initial population size must be greater than 0".to_string());
            }

            if self.evolution_strategies.max_generations == 0 {
                return Err("Max generations must be greater than 0".to_string());
            }
        }

        Ok(())
    }

    /// Get estimated memory usage in bytes
    pub fn estimated_memory_usage(&self) -> u64 {
        if !self.enabled {
            return 0;
        }

        let population_size = self.population_management.initial_population_size as u64;
        let worker_threads = self.parallel_processing.worker_threads as u64;

        // Rough estimate: population size * individual size + thread overhead
        population_size * 1024 + worker_threads * 4096
    }
}
