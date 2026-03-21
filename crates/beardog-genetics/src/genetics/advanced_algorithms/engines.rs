// SPDX-License-Identifier: AGPL-3.0-only

//! Population, fitness, mutation, crossover, and selection engines.

use super::metrics::DiversityMetrics;
use super::types::{GenerationSnapshot, GeneticIndividual, MutationRecord};
use beardog_errors::BearDogError;
use rand::Rng;
use std::collections::HashMap;
use std::sync::Arc;
use std::sync::atomic::{AtomicU64, Ordering};
use uuid::Uuid;

/// Population manager
#[derive(Debug)]
pub struct PopulationManager {
    /// Current population
    current_population: parking_lot::RwLock<Vec<GeneticIndividual>>,

    /// Population history
    population_history: parking_lot::RwLock<Vec<GenerationSnapshot>>,

    /// Diversity metrics
    diversity_metrics: Arc<DiversityMetrics>,
}

impl PopulationManager {
    /// Create new manager
    pub fn new(capacity: usize) -> Self {
        Self {
            current_population: parking_lot::RwLock::new(Vec::with_capacity(capacity)),
            population_history: parking_lot::RwLock::new(Vec::new()),
            diversity_metrics: Arc::new(DiversityMetrics::new()),
        }
    }

    /// Get population size
    pub fn population_size(&self) -> usize {
        self.current_population.read().len()
    }

    /// Get current population
    pub fn get_population(&self) -> Vec<GeneticIndividual> {
        self.current_population.read().clone()
    }

    /// Set population
    pub fn set_population(&self, population: Vec<GeneticIndividual>) {
        *self.current_population.write() = population;
    }
}

impl Clone for PopulationManager {
    fn clone(&self) -> Self {
        Self {
            current_population: parking_lot::RwLock::new(self.current_population.read().clone()),
            population_history: parking_lot::RwLock::new(self.population_history.read().clone()),
            diversity_metrics: self.diversity_metrics.clone(),
        }
    }
}

/// Fitness evaluator
#[derive(Debug)]
pub struct FitnessEvaluator {
    /// Weights by criterion
    weights: HashMap<String, f64>,

    /// Adaptive weights (stored as fixed-point)
    adaptive_weights: Arc<AtomicU64>,
}

impl FitnessEvaluator {
    /// Create new evaluator
    pub fn new() -> Self {
        Self {
            weights: HashMap::with_capacity(16),
            adaptive_weights: Arc::new(AtomicU64::new(0)),
        }
    }

    /// Evaluate fitness
    pub fn evaluate_fitness(&self, individual: &GeneticIndividual) -> Result<f64, BearDogError> {
        let base_fitness = individual.genetic_signature.quality_score;
        let performance_bonus = individual.performance_metrics.authorization_success_rate * 0.2;
        Ok((base_fitness + performance_bonus).min(1.0))
    }
}

impl Clone for FitnessEvaluator {
    fn clone(&self) -> Self {
        Self {
            weights: self.weights.clone(),
            adaptive_weights: Arc::new(AtomicU64::new(
                self.adaptive_weights.load(Ordering::Relaxed),
            )),
        }
    }
}

impl Default for FitnessEvaluator {
    fn default() -> Self {
        Self::new()
    }
}

/// Mutation engine
#[derive(Debug)]
pub struct MutationEngine {
    /// Adaptive rates
    adaptive_rates: HashMap<String, f64>,

    /// Mutation history
    mutation_history: parking_lot::RwLock<Vec<MutationRecord>>,
}

impl MutationEngine {
    /// Create new engine
    pub fn new() -> Self {
        Self {
            adaptive_rates: HashMap::with_capacity(16),
            mutation_history: parking_lot::RwLock::new(Vec::new()),
        }
    }

    /// Apply mutation
    pub fn apply_mutation(
        &self,
        individual: &mut GeneticIndividual,
        rate: f64,
    ) -> Result<bool, BearDogError> {
        let mut rng = rand::thread_rng();
        if rng.gen_range(0.0..1.0) < rate {
            // Apply random mutation
            individual.fitness_score *= rng.gen_range(0.0_f64..1.0).mul_add(0.2, 0.9);
            individual.mutation_history.push(MutationRecord::default());
            Ok(true)
        } else {
            Ok(false)
        }
    }
}

impl Clone for MutationEngine {
    fn clone(&self) -> Self {
        Self {
            adaptive_rates: self.adaptive_rates.clone(),
            mutation_history: parking_lot::RwLock::new(self.mutation_history.read().clone()),
        }
    }
}

impl Default for MutationEngine {
    fn default() -> Self {
        Self::new()
    }
}

/// Crossover engine
#[derive(Debug)]
pub struct CrossoverEngine {
    /// Compatibility matrix
    compatibility_matrix: HashMap<(String, String), f64>,
}

impl CrossoverEngine {
    /// Create new engine
    pub fn new() -> Self {
        Self {
            compatibility_matrix: HashMap::with_capacity(16),
        }
    }

    /// Perform crossover
    pub fn crossover(
        &self,
        parent1: &GeneticIndividual,
        parent2: &GeneticIndividual,
    ) -> Result<Vec<GeneticIndividual>, BearDogError> {
        // Simple single-point crossover
        let mut child1 = parent1.clone();
        let mut child2 = parent2.clone();

        child1.id = Uuid::new_v4().to_string();
        child2.id = Uuid::new_v4().to_string();
        child1.parent_ids = vec![parent1.id.clone(), parent2.id.clone()];
        child2.parent_ids = vec![parent1.id.clone(), parent2.id.clone()];
        child1.age = 0;
        child2.age = 0;

        // Swap some traits
        std::mem::swap(
            &mut child1.performance_metrics,
            &mut child2.performance_metrics,
        );

        Ok(vec![child1, child2])
    }
}

impl Clone for CrossoverEngine {
    fn clone(&self) -> Self {
        Self {
            compatibility_matrix: self.compatibility_matrix.clone(),
        }
    }
}

impl Default for CrossoverEngine {
    fn default() -> Self {
        Self::new()
    }
}

/// Selection engine
#[derive(Debug)]
pub struct SelectionEngine {
    /// Current selection method
    current_method: String,

    /// Selection pressure adjustment
    pressure_adjustment: f64,
}

impl SelectionEngine {
    /// Create new engine
    pub fn new() -> Self {
        Self {
            current_method: "tournament".to_string(),
            pressure_adjustment: 1.0,
        }
    }

    /// Select parents using tournament selection
    pub fn select_parents(
        &self,
        population: &[GeneticIndividual],
        num_parents: usize,
    ) -> Result<Vec<usize>, BearDogError> {
        if population.is_empty() {
            return Ok(Vec::new());
        }

        let mut parents = Vec::with_capacity(num_parents);
        let tournament_size = 3;

        let mut rng = rand::thread_rng();
        for _ in 0..num_parents {
            let mut best_idx = rng.gen_range(0..population.len());
            let mut best_fitness = population[best_idx].fitness_score;

            for _ in 1..tournament_size {
                let idx = rng.gen_range(0..population.len());
                if population[idx].fitness_score > best_fitness {
                    best_fitness = population[idx].fitness_score;
                    best_idx = idx;
                }
            }
            parents.push(best_idx);
        }

        Ok(parents)
    }

    /// Select survivors
    pub fn select_survivors(
        &self,
        population: &[GeneticIndividual],
        num_survivors: usize,
    ) -> Result<Vec<usize>, BearDogError> {
        let mut indices: Vec<usize> = (0..population.len()).collect();
        indices.sort_by(|&a, &b| {
            population[b]
                .fitness_score
                .partial_cmp(&population[a].fitness_score)
                .unwrap_or(std::cmp::Ordering::Equal)
        });
        Ok(indices.into_iter().take(num_survivors).collect())
    }
}

impl Clone for SelectionEngine {
    fn clone(&self) -> Self {
        Self {
            current_method: self.current_method.clone(),
            pressure_adjustment: self.pressure_adjustment,
        }
    }
}

impl Default for SelectionEngine {
    fn default() -> Self {
        Self::new()
    }
}
