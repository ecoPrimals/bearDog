// SPDX-License-Identifier: AGPL-3.0-or-later

// Module documentation
//
// This module provides functionality for the BearDog ecosystem.

use beardog_errors::BearDogError;
use rand::Rng;
use std::sync::Arc;
use tokio::sync::RwLock;
use tracing::info;

///
/// Uses evolutionary algorithms to optimize system parameters and configurations
#[derive(Debug, Clone)]
pub struct GeneticOptimizer {
    config: GeneticOptimizerConfig,
    optimization_state: Arc<RwLock<OptimizationState>>,
    performance_history: Arc<RwLock<Vec<PerformanceMetric>>>,
}

/// Configuration for the genetic algorithm optimizer
///
/// Controls the behavior of the genetic optimization algorithm including
/// population size, mutation/crossover rates, and convergence criteria.
#[derive(Debug, Clone, Copy)]
pub struct GeneticOptimizerConfig {
    /// Number of individuals in each generation
    pub population_size: usize,
    /// Rate of mutation (0.0 to 1.0)
    /// The mutation rate value
    pub mutation_rate: f64,
    /// Rate of crossover between individuals (0.0 to 1.0)
    /// The crossover rate value
    pub crossover_rate: f64,
    /// Maximum number of generations to run
    /// Number of `max_generations`
    pub max_generations: usize,
    /// The convergence threshold value
    pub convergence_threshold: f64,
}

impl Default for GeneticOptimizerConfig {
    fn default() -> Self {
        Self {
            population_size: 50,
            mutation_rate: 0.01,
            crossover_rate: 0.8,
            max_generations: 100,
            convergence_threshold: 0.001,
        }
    }
}

/// Current state of the genetic optimization process
#[derive(Debug, Clone, Copy, Default)]
pub struct OptimizationState {
    /// Current generation number in the optimization process
    /// Number of `current_generation`
    pub current_generation: usize,
    /// Best fitness score achieved so far
    /// The best fitness value
    pub best_fitness: f64,
    /// Number of consecutive generations without significant improvement
    /// Number of convergence
    pub convergence_count: usize,
    /// Whether the optimization has converged to a stable solution
    /// Whether `is_converged` is enabled
    pub is_converged: bool,
}

/// Performance metrics captured during genetic optimization
#[derive(Debug, Clone, Copy)]
pub struct PerformanceMetric {
    /// Timestamp when this metric was recorded
    pub timestamp: chrono::DateTime<chrono::Utc>,
    /// Fitness score achieved at this point
    /// The fitness score value
    pub fitness_score: f64,
    /// Generation number when this metric was recorded
    /// Number of generation
    pub generation: usize,
    /// Rate of improvement compared to previous generation
    /// The improvement rate value
    pub improvement_rate: f64,
}

impl GeneticOptimizer {
    /// Create a new genetic optimizer with default configuration
    /// Creates a new instance
    #[must_use]
    pub fn new() -> Self {
        Self::with_config(GeneticOptimizerConfig::default())
    }

    /// Create a new genetic optimizer with custom configuration
    /// Creates instance with config
    #[must_use]
    pub fn with_config(config: GeneticOptimizerConfig) -> Self {
        Self {
            config,
            optimization_state: Arc::new(RwLock::new(OptimizationState::default())),
            performance_history: Arc::new(RwLock::new(Vec::new())),
        }
    }

    /// Initialize the genetic optimizer with default state
    ///
    /// # Errors
    /// Returns an error if initialization fails.
    pub async fn initialize(&self) -> Result<(), BearDogError> {
        info!(
            "🧬 Initializing GeneticOptimizer with population size: {}",
            self.config.population_size
        );

        // Initialize optimization state
        {
            let mut state = self.optimization_state.write().await;
            state.current_generation = 0;
            state.best_fitness = 0.0;
            state.convergence_count = 0;
            state.is_converged = false;
        } // Drop state lock early

        info!("✅ GeneticOptimizer initialized successfully");
        Ok(())
    }

    /// Run the genetic optimization algorithm
    ///
    /// # Errors
    /// Returns an error if population initialization fails or optimization encounters an error.
    pub async fn optimize(
        &self,
        fitness_function: impl Fn(&[f64]) -> f64 + Send + Sync,
    ) -> Result<Vec<f64>, BearDogError> {
        info!("🧬 Starting genetic optimization process");

        // Initialize population
        let mut population = self.initialize_population();
        let mut best_solution = population[0].clone();
        let mut best_fitness = fitness_function(&best_solution);

        for generation in 0..self.config.max_generations {
            // Evaluate fitness for all individuals
            let fitness_scores: Vec<f64> = population
                .iter()
                .map(|individual| fitness_function(individual))
                .collect();

            // Find best individual in current generation
            if let Some((best_idx, &current_best_fitness)) = fitness_scores
                .iter()
                .enumerate()
                .max_by(|(_, a), (_, b)| a.partial_cmp(b).unwrap_or(std::cmp::Ordering::Equal))
                && current_best_fitness > best_fitness
            {
                best_fitness = current_best_fitness;
                best_solution.clone_from(&population[best_idx]);
            }

            // Update optimization state
            {
                let mut state = self.optimization_state.write().await;
                state.current_generation = generation;
                state.best_fitness = best_fitness;

                // Check for convergence
                if generation > 0 {
                    let improvement = best_fitness - state.best_fitness;
                    if improvement.abs() < self.config.convergence_threshold {
                        state.convergence_count += 1;
                        if state.convergence_count >= 10 {
                            state.is_converged = true;
                            info!(
                                "🎯 Genetic optimization converged at generation {}",
                                generation
                            );
                            break;
                        }
                    } else {
                        state.convergence_count = 0;
                    }
                }
            }

            // Record performance metric
            {
                let mut history = self.performance_history.write().await;
                let improvement_rate = if generation > 0 {
                    history
                        .last()
                        .map_or(0.0, |last_metric| best_fitness - last_metric.fitness_score)
                } else {
                    0.0
                };

                history.push(PerformanceMetric {
                    timestamp: chrono::Utc::now(),
                    fitness_score: best_fitness,
                    generation,
                    improvement_rate,
                });
            }

            // Create next generation
            population = self.create_next_generation(&population, &fitness_scores);
        }

        info!(
            "✅ Genetic optimization completed with fitness: {:.6}",
            best_fitness
        );
        Ok(best_solution)
    }

    /// Initialize the population with random individuals
    ///
    /// Creates the initial population of candidate solutions with random
    /// values in the optimization space.
    fn initialize_population(&self) -> Vec<Vec<f64>> {
        let mut rng = rand::rng();

        let mut population = Vec::with_capacity(self.config.population_size);
        for _ in 0..self.config.population_size {
            let individual: Vec<f64> = (0..10) // 10-dimensional optimization space
                .map(|_| rng.random_range(-1.0..1.0))
                .collect();
            population.push(individual);
        }

        population
    }

    /// Create the next generation through selection, crossover, and mutation
    ///
    /// Applies genetic operators to evolve the population toward better solutions.
    fn create_next_generation(
        &self,
        population: &[Vec<f64>],
        fitness_scores: &[f64],
    ) -> Vec<Vec<f64>> {
        let mut rng = rand::rng();
        let mut next_generation = Vec::with_capacity(self.config.population_size);

        // Selection, crossover, and mutation
        for _ in 0..self.config.population_size {
            // Tournament selection
            let parent1_idx = Self::tournament_selection(fitness_scores, &mut rng);
            let parent2_idx = Self::tournament_selection(fitness_scores, &mut rng);

            let parent1 = &population[parent1_idx];
            let parent2 = &population[parent2_idx];

            // Crossover
            let mut offspring = if rng.random::<f64>() < self.config.crossover_rate {
                Self::crossover(parent1, parent2, &mut rng)
            } else {
                parent1.clone()
            };

            // Mutation
            if rng.random::<f64>() < self.config.mutation_rate {
                Self::mutate(&mut offspring, &mut rng);
            }

            next_generation.push(offspring);
        }

        next_generation
    }

    fn tournament_selection(fitness_scores: &[f64], rng: &mut impl Rng) -> usize {
        let tournament_size = 3;
        let mut best_idx = rng.random_range(0..fitness_scores.len());
        let mut best_fitness = fitness_scores[best_idx];

        for _ in 1..tournament_size {
            let idx = rng.random_range(0..fitness_scores.len());
            if fitness_scores[idx] > best_fitness {
                best_fitness = fitness_scores[idx];
                best_idx = idx;
            }
        }

        best_idx
    }

    fn crossover(parent1: &[f64], parent2: &[f64], rng: &mut impl Rng) -> Vec<f64> {
        let crossover_point = rng.random_range(1..parent1.len());
        let mut offspring = Vec::with_capacity(parent1.len());

        for i in 0..parent1.len() {
            if i < crossover_point {
                offspring.push(parent1[i]);
            } else {
                offspring.push(parent2[i]);
            }
        }

        offspring
    }

    fn mutate(individual: &mut [f64], rng: &mut impl Rng) {
        for gene in individual.iter_mut() {
            if rng.random::<f64>() < 0.1 {
                // 10% chance to mutate each gene
                *gene += rng.random_range(-0.1..0.1);
                *gene = gene.clamp(-1.0, 1.0);
            }
        }
    }

    /// Get the current optimization state
    ///
    /// Returns a snapshot of the current genetic algorithm state including
    /// generation number, best fitness, and convergence information.
    pub async fn get_optimization_state(&self) -> OptimizationState {
        *self.optimization_state.read().await
    }

    /// Get the complete performance history
    ///
    /// Returns all performance metrics collected during the optimization process,
    /// including fitness scores and improvement rates for each generation.
    pub async fn get_performance_history(&self) -> Vec<PerformanceMetric> {
        self.performance_history.read().await.clone()
    }
}

impl Default for GeneticOptimizer {
    fn default() -> Self {
        Self::new()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_genetic_optimizer_config_default() {
        let config = GeneticOptimizerConfig::default();
        assert_eq!(config.population_size, 50);
        assert!((config.mutation_rate - 0.01).abs() < 1e-10);
        assert!((config.crossover_rate - 0.8).abs() < 1e-10);
        assert_eq!(config.max_generations, 100);
        assert!((config.convergence_threshold - 0.001).abs() < 1e-10);
    }

    #[test]
    fn test_optimization_state_default() {
        let state = OptimizationState::default();
        assert_eq!(state.current_generation, 0);
        assert_eq!(state.best_fitness, 0.0);
        assert_eq!(state.convergence_count, 0);
        assert!(!state.is_converged);
    }

    #[test]
    fn test_performance_metric_creation() {
        let metric = PerformanceMetric {
            timestamp: chrono::Utc::now(),
            fitness_score: 0.95,
            generation: 5,
            improvement_rate: 0.01,
        };
        assert!((metric.fitness_score - 0.95).abs() < 1e-10);
        assert_eq!(metric.generation, 5);
    }

    #[test]
    fn test_genetic_optimizer_new() {
        let optimizer = GeneticOptimizer::new();
        assert!(std::mem::size_of_val(&optimizer) > 0);
    }

    #[test]
    fn test_genetic_optimizer_with_config() {
        let config = GeneticOptimizerConfig {
            population_size: 20,
            mutation_rate: 0.05,
            crossover_rate: 0.9,
            max_generations: 50,
            convergence_threshold: 0.0001,
        };
        let optimizer = GeneticOptimizer::with_config(config);
        assert!(std::mem::size_of_val(&optimizer) > 0);
    }

    #[tokio::test]
    async fn test_genetic_optimizer_initialize() {
        let optimizer = GeneticOptimizer::new();
        let result = optimizer.initialize().await;
        assert!(result.is_ok());
    }

    #[tokio::test]
    async fn test_genetic_optimizer_optimize() {
        let config = GeneticOptimizerConfig {
            population_size: 10,
            mutation_rate: 0.1,
            crossover_rate: 0.8,
            max_generations: 5,
            convergence_threshold: 0.001,
        };
        let optimizer = GeneticOptimizer::with_config(config);
        let () = optimizer
            .initialize()
            .await
            .expect("genetic optimizer initialize");
        // Simple fitness: sum of squares (maximize)
        let result = optimizer
            .optimize(|x| x.iter().map(|v| v * v).sum::<f64>())
            .await;
        assert!(result.is_ok());
        let solution = result.expect("genetic optimizer optimize");
        assert_eq!(solution.len(), 10);
    }

    #[tokio::test]
    async fn test_get_optimization_state() {
        let optimizer = GeneticOptimizer::new();
        let () = optimizer
            .initialize()
            .await
            .expect("genetic optimizer initialize");
        let state = optimizer.get_optimization_state().await;
        assert_eq!(state.current_generation, 0);
    }

    #[tokio::test]
    async fn test_get_performance_history() {
        let config = GeneticOptimizerConfig {
            population_size: 5,
            mutation_rate: 0.1,
            crossover_rate: 0.8,
            max_generations: 2,
            convergence_threshold: 0.001,
        };
        let optimizer = GeneticOptimizer::with_config(config);
        let () = optimizer
            .initialize()
            .await
            .expect("genetic optimizer initialize");
        let _ = optimizer
            .optimize(|x| x.iter().sum::<f64>())
            .await
            .expect("genetic optimizer optimize");
        let history = optimizer.get_performance_history().await;
        assert!(!history.is_empty());
    }
}
