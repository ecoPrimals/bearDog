//! # Genetic Optimization for Gaming Crypto
//!
//! This module provides genetic algorithm-based optimization
//! for cryptographic parameter tuning in gaming applications.

use serde::{Deserialize, Serialize};

// ============================================================
// Genetic Parameters
// ============================================================

/// Parameters for genetic algorithm optimization
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct GeneticParameters {
    /// Population size
    pub population_size: usize,

    /// Mutation rate (0.0 - 1.0)
    pub mutation_rate: f64,

    /// Crossover rate (0.0 - 1.0)
    pub crossover_rate: f64,

    /// Number of generations to evolve
    pub generations: usize,

    /// Percentage of elite individuals to preserve
    pub elite_percentage: f64,
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

// ============================================================
// Evolution Result
// ============================================================

/// Result of genetic evolution
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct EvolutionResult {
    /// Best fitness achieved
    pub best_fitness: f64,

    /// Number of generations evolved
    pub generations_evolved: usize,

    /// Improvement percentage over initial
    pub improvement_percentage: f64,

    /// Optimized parameters
    pub optimized_params: Vec<u8>,

    /// Convergence generation (when improvement plateaued)
    pub convergence_generation: Option<usize>,
}

impl Default for EvolutionResult {
    fn default() -> Self {
        Self {
            best_fitness: 0.85,
            generations_evolved: 50,
            improvement_percentage: 15.0,
            optimized_params: vec![],
            convergence_generation: None,
        }
    }
}

// ============================================================
// Genetic Optimizer
// ============================================================

/// Genetic optimizer for crypto parameters
#[derive(Debug, Clone)]
pub struct GeneticOptimizer {
    /// Genetic parameters
    parameters: GeneticParameters,

    /// Current best fitness
    best_fitness: f64,

    /// Generation counter
    current_generation: usize,
}

impl GeneticOptimizer {
    /// Create a new genetic optimizer
    pub fn new(parameters: GeneticParameters) -> Self {
        Self {
            parameters,
            best_fitness: 0.0,
            current_generation: 0,
        }
    }

    /// Evolve the population
    pub fn evolve(&mut self, initial_params: &[u8]) -> EvolutionResult {
        let mut best_params = initial_params.to_vec();
        let initial_fitness = self.evaluate_fitness(&best_params);
        self.best_fitness = initial_fitness;

        let mut convergence_gen = None;
        let mut no_improvement_count = 0;

        for gen in 0..self.parameters.generations {
            self.current_generation = gen;

            // Simulate evolution (simplified)
            let mutated = self.mutate(&best_params);
            let fitness = self.evaluate_fitness(&mutated);

            if fitness > self.best_fitness {
                self.best_fitness = fitness;
                best_params = mutated;
                no_improvement_count = 0;
            } else {
                no_improvement_count += 1;
                if no_improvement_count > 10 && convergence_gen.is_none() {
                    convergence_gen = Some(gen);
                }
            }
        }

        let improvement = if initial_fitness > 0.0 {
            ((self.best_fitness - initial_fitness) / initial_fitness) * 100.0
        } else {
            0.0
        };

        EvolutionResult {
            best_fitness: self.best_fitness,
            generations_evolved: self.parameters.generations,
            improvement_percentage: improvement,
            optimized_params: best_params,
            convergence_generation: convergence_gen,
        }
    }

    /// Evaluate fitness of parameters
    fn evaluate_fitness(&self, params: &[u8]) -> f64 {
        // Simplified fitness function
        if params.is_empty() {
            return 0.0;
        }

        let sum: u64 = params.iter().map(|&b| b as u64).sum();
        let avg = sum as f64 / params.len() as f64;

        // Normalize to 0.0 - 1.0
        avg / 255.0
    }

    /// Mutate parameters
    fn mutate(&self, params: &[u8]) -> Vec<u8> {
        use rand::Rng;
        let mut rng = rand::thread_rng();
        let mut result = params.to_vec();

        for byte in &mut result {
            if rng.gen::<f64>() < self.parameters.mutation_rate {
                *byte = rng.gen();
            }
        }

        result
    }

    /// Get current best fitness
    pub fn best_fitness(&self) -> f64 {
        self.best_fitness
    }

    /// Get current generation
    pub fn current_generation(&self) -> usize {
        self.current_generation
    }
}

impl Default for GeneticOptimizer {
    fn default() -> Self {
        Self::new(GeneticParameters::default())
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_genetic_parameters_default() {
        let params = GeneticParameters::default();
        assert_eq!(params.population_size, 100);
        assert_eq!(params.mutation_rate, 0.05);
        assert_eq!(params.generations, 50);
    }

    #[test]
    fn test_evolution_result_default() {
        let result = EvolutionResult::default();
        assert_eq!(result.best_fitness, 0.85);
        assert_eq!(result.generations_evolved, 50);
    }

    #[test]
    fn test_genetic_optimizer() {
        let mut optimizer = GeneticOptimizer::default();
        let initial_params = vec![128u8; 32];
        let result = optimizer.evolve(&initial_params);

        assert!(result.best_fitness > 0.0);
        assert!(!result.optimized_params.is_empty());
    }

    #[test]
    fn test_fitness_evaluation() {
        let optimizer = GeneticOptimizer::default();

        let empty_fitness = optimizer.evaluate_fitness(&[]);
        assert_eq!(empty_fitness, 0.0);

        let max_fitness = optimizer.evaluate_fitness(&[255, 255, 255, 255]);
        assert!((max_fitness - 1.0).abs() < 0.01);
    }
}
