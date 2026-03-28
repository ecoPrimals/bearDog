// SPDX-License-Identifier: AGPL-3.0-only

//! Orchestrating evolution engine: population initialization and generational loop.

use super::engines::{
    CrossoverEngine, FitnessEvaluator, MutationEngine, PopulationManager, SelectionEngine,
};
use super::metrics::EvolutionMetrics;
use super::types::{
    EvolutionConfig, GenerationSnapshot, GeneticIndividual, GeneticSignature, PerformanceMetrics,
};
use beardog_errors::BearDogError;
use chrono::Utc;
use std::sync::Arc;
use std::sync::atomic::Ordering;
use uuid::Uuid;

/// Genetic evolution engine
#[derive(Debug, Clone)]
pub struct GeneticEvolutionEngine {
    /// Evolution configuration
    pub evolution_config: EvolutionConfig,

    /// Population manager
    population_manager: Arc<PopulationManager>,

    /// Fitness evaluator
    fitness_evaluator: Arc<FitnessEvaluator>,

    /// Mutation engine
    mutation_engine: Arc<MutationEngine>,

    /// Crossover engine
    crossover_engine: Arc<CrossoverEngine>,

    /// Selection engine
    selection_engine: Arc<SelectionEngine>,

    /// Evolution metrics
    evolution_metrics: Arc<EvolutionMetrics>,
}

impl GeneticEvolutionEngine {
    /// Create new evolution engine
    pub fn new(config: EvolutionConfig) -> Self {
        Self {
            evolution_config: config.clone(),
            population_manager: Arc::new(PopulationManager::new(config.population_size)),
            fitness_evaluator: Arc::new(FitnessEvaluator::new()),
            mutation_engine: Arc::new(MutationEngine::new()),
            crossover_engine: Arc::new(CrossoverEngine::new()),
            selection_engine: Arc::new(SelectionEngine::new()),
            evolution_metrics: Arc::new(EvolutionMetrics::new()),
        }
    }

    /// Initialize population
    pub fn initialize_population(
        &self,
        signatures: Vec<GeneticSignature>,
    ) -> Result<Vec<GeneticIndividual>, BearDogError> {
        let population: Vec<GeneticIndividual> = signatures
            .into_iter()
            .map(|sig| GeneticIndividual {
                id: Uuid::new_v4().to_string(),
                genetic_signature: sig,
                fitness_score: 0.0,
                age: 0,
                parent_ids: Vec::new(),
                mutation_history: Vec::new(),
                performance_metrics: PerformanceMetrics::default(),
                specialization_traits: Vec::new(),
            })
            .collect();

        self.population_manager.set_population(population.clone());
        Ok(population)
    }

    /// Run one generation
    pub fn evolve_generation(&self) -> Result<GenerationSnapshot, BearDogError> {
        let population = self.population_manager.get_population();
        let generation = u32::try_from(
            self.evolution_metrics
                .total_generations
                .fetch_add(1, Ordering::Relaxed),
        )
        .unwrap_or(u32::MAX);

        // Calculate fitness for all individuals
        let evaluated: Vec<_> = population
            .into_iter()
            .map(|mut ind| {
                ind.fitness_score = self.fitness_evaluator.evaluate_fitness(&ind).unwrap_or(0.0);
                ind.age += 1;
                ind
            })
            .collect();

        // Selection
        #[expect(
            clippy::cast_possible_truncation,
            clippy::cast_sign_loss,
            clippy::cast_precision_loss,
            reason = "elitism count from population size times f64 percentage; bounded by evaluator"
        )]
        let num_elite =
            (evaluated.len() as f64 * self.evolution_config.elitism_percentage) as usize;
        let elite_indices = self
            .selection_engine
            .select_survivors(&evaluated, num_elite)?;

        // Keep elite individuals
        let mut next_gen: Vec<_> = elite_indices
            .iter()
            .map(|&i| evaluated[i].clone())
            .collect();

        // Generate offspring
        while next_gen.len() < self.evolution_config.population_size {
            let parent_indices = self.selection_engine.select_parents(&evaluated, 2)?;
            if parent_indices.len() >= 2 {
                let offspring = self
                    .crossover_engine
                    .crossover(&evaluated[parent_indices[0]], &evaluated[parent_indices[1]])?;
                for mut child in offspring {
                    self.mutation_engine
                        .apply_mutation(&mut child, self.evolution_config.mutation_rate)?;
                    next_gen.push(child);
                    if next_gen.len() >= self.evolution_config.population_size {
                        break;
                    }
                }
            }
        }

        // Calculate statistics
        #[expect(
            clippy::cast_precision_loss,
            reason = "display/metric conversion, precision loss acceptable"
        )]
        let avg_fitness =
            next_gen.iter().map(|i| i.fitness_score).sum::<f64>() / next_gen.len() as f64;
        let best_fitness = next_gen
            .iter()
            .map(|i| i.fitness_score)
            .fold(0.0f64, f64::max);

        self.population_manager.set_population(next_gen);

        Ok(GenerationSnapshot {
            generation,
            timestamp: Utc::now(),
            population_size: self.evolution_config.population_size,
            average_fitness: avg_fitness,
            best_fitness,
            diversity_index: self.compute_diversity_index(),
            convergence_rate: 0.0,
        })
    }

    /// Compute population diversity as the ratio of unique fitness values to population size.
    ///
    /// Returns a value in \[0.0, 1.0\] where 1.0 means every individual has a distinct fitness.
    fn compute_diversity_index(&self) -> f64 {
        let population = self.population_manager.get_population();
        if population.is_empty() {
            return 0.0;
        }
        let mut unique: std::collections::HashSet<u64> = std::collections::HashSet::new();
        for ind in &population {
            unique.insert(ind.fitness_score.to_bits());
        }
        #[expect(
            clippy::cast_precision_loss,
            reason = "display/metric, precision loss acceptable"
        )]
        let diversity = unique.len() as f64 / population.len() as f64;
        diversity
    }

    /// Get best individual
    pub fn get_best_individual(&self) -> Option<GeneticIndividual> {
        let population = self.population_manager.get_population();
        population.into_iter().max_by(|a, b| {
            a.fitness_score
                .partial_cmp(&b.fitness_score)
                .unwrap_or(std::cmp::Ordering::Equal)
        })
    }
}

impl Default for GeneticEvolutionEngine {
    fn default() -> Self {
        Self::new(EvolutionConfig::default())
    }
}

#[cfg(test)]
mod tests {
    use super::super::engines::SelectionEngine;
    use super::super::types::GeneticSignature;
    use super::*;

    #[test]
    fn test_evolution_config_default() {
        let config = EvolutionConfig::default();
        assert_eq!(config.population_size, 100);
        assert_eq!(config.mutation_rate, 0.05);
        assert!(config.diversity_preservation);
    }

    #[test]
    fn test_genetic_individual_default() {
        let individual = GeneticIndividual::default();
        assert_eq!(individual.fitness_score, 0.0);
        assert_eq!(individual.age, 0);
    }

    #[test]
    fn test_performance_metrics_default() {
        let metrics = PerformanceMetrics::default();
        assert_eq!(metrics.authorization_success_rate, 0.5);
        assert_eq!(metrics.operation_efficiency, 0.5);
    }

    #[test]
    fn test_genetic_evolution_engine_creation() {
        let config = EvolutionConfig::default();
        let engine = GeneticEvolutionEngine::new(config);
        assert_eq!(engine.evolution_config.population_size, 100);
    }

    #[test]
    fn test_population_initialization() {
        let config = EvolutionConfig {
            population_size: 10,
            ..Default::default()
        };
        let engine = GeneticEvolutionEngine::new(config);

        let signatures = vec![GeneticSignature::default(); 10];
        let population = engine
            .initialize_population(signatures)
            .expect("population init with matching size and signatures");
        assert_eq!(population.len(), 10);
    }

    #[test]
    fn test_selection_engine() {
        let engine = SelectionEngine::new();
        let population = vec![
            GeneticIndividual {
                fitness_score: 0.8,
                ..Default::default()
            },
            GeneticIndividual {
                fitness_score: 0.5,
                ..Default::default()
            },
            GeneticIndividual {
                fitness_score: 0.3,
                ..Default::default()
            },
        ];

        let survivors = engine
            .select_survivors(&population, 2)
            .expect("selection with k <= population len");
        assert_eq!(survivors.len(), 2);
        assert_eq!(survivors[0], 0); // Highest fitness first
    }

    #[test]
    fn genetic_evolution_engine_default_matches_new_default_config() {
        let a = GeneticEvolutionEngine::default();
        let b = GeneticEvolutionEngine::new(EvolutionConfig::default());
        assert_eq!(
            a.evolution_config.population_size,
            b.evolution_config.population_size
        );
    }

    #[test]
    fn evolve_generation_updates_metrics_and_best_fitness() {
        let config = EvolutionConfig {
            population_size: 4,
            elitism_percentage: 0.25,
            ..Default::default()
        };
        let engine = GeneticEvolutionEngine::new(config);
        let sigs = vec![GeneticSignature::default(); 4];
        engine.initialize_population(sigs).expect("seed population");
        let snap = engine.evolve_generation().expect("one generation");
        assert_eq!(snap.population_size, 4);
        assert!(snap.average_fitness >= 0.0);
        assert!(snap.best_fitness >= snap.average_fitness || snap.generation == 1);

        let best = engine.get_best_individual();
        assert!(best.is_some());
        let best = best.expect("best exists after evolution");
        assert!(best.fitness_score >= 0.0);
    }

    #[test]
    fn evolve_generation_increments_generation_counter() {
        let config = EvolutionConfig {
            population_size: 3,
            ..Default::default()
        };
        let engine = GeneticEvolutionEngine::new(config);
        engine
            .initialize_population(vec![GeneticSignature::default(); 3])
            .expect("init");
        let g1 = engine.evolve_generation().expect("g1");
        let g2 = engine.evolve_generation().expect("g2");
        assert!(g2.generation > g1.generation);
    }

    #[test]
    fn get_best_individual_none_when_population_empty() {
        let engine = GeneticEvolutionEngine::new(EvolutionConfig {
            population_size: 2,
            ..Default::default()
        });
        assert!(engine.get_best_individual().is_none());
    }

    #[test]
    fn diversity_index_in_snapshot_is_bounded() {
        let engine = GeneticEvolutionEngine::new(EvolutionConfig {
            population_size: 5,
            ..Default::default()
        });
        engine
            .initialize_population(vec![GeneticSignature::default(); 5])
            .expect("init");
        let snap = engine.evolve_generation().expect("evo");
        assert!(snap.diversity_index >= 0.0 && snap.diversity_index <= 1.0);
    }
}
