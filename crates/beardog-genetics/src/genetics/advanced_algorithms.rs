//! # Advanced Genetic Algorithms
//!
//! This module provides advanced genetic algorithm implementations for
//! evolutionary optimization in the BearDog ecosystem.

use crate::genetics::biome_genetics::{BiomeIdentity, GeneticSignature, TrustLevel};
use crate::genetics::entropy_hierarchy::EntropyClass;
use beardog_errors::BearDogError;
use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use std::sync::atomic::{AtomicU64, Ordering};
use std::sync::Arc;
use uuid::Uuid;

// ============================================================
// Configuration Types
// ============================================================

/// Evolution configuration
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct EvolutionConfig {
    /// Population size
    pub population_size: usize,

    /// Mutation rate (0.0 - 1.0)
    pub mutation_rate: f64,

    /// Crossover rate (0.0 - 1.0)
    pub crossover_rate: f64,

    /// Elitism percentage (0.0 - 1.0)
    pub elitism_percentage: f64,

    /// Maximum generations
    pub max_generations: u32,

    /// Fitness threshold for convergence
    pub fitness_threshold: f64,

    /// Enable diversity preservation
    pub diversity_preservation: bool,

    /// Enable adaptive parameters
    pub adaptive_parameters: bool,
}

impl Default for EvolutionConfig {
    fn default() -> Self {
        Self {
            population_size: std::env::var("BEARDOG_GENETICS_POPULATION_SIZE")
                .ok()
                .and_then(|v| v.parse().ok())
                .unwrap_or(100),
            mutation_rate: std::env::var("BEARDOG_GENETICS_MUTATION_RATE")
                .ok()
                .and_then(|v| v.parse().ok())
                .unwrap_or(0.05),
            crossover_rate: std::env::var("BEARDOG_GENETICS_CROSSOVER_RATE")
                .ok()
                .and_then(|v| v.parse().ok())
                .unwrap_or(0.8),
            elitism_percentage: std::env::var("BEARDOG_GENETICS_ELITISM_PERCENTAGE")
                .ok()
                .and_then(|v| v.parse().ok())
                .unwrap_or(0.1),
            max_generations: std::env::var("BEARDOG_GENETICS_MAX_GENERATIONS")
                .ok()
                .and_then(|v| v.parse().ok())
                .unwrap_or(1000),
            fitness_threshold: std::env::var("BEARDOG_GENETICS_FITNESS_THRESHOLD")
                .ok()
                .and_then(|v| v.parse().ok())
                .unwrap_or(0.95),
            diversity_preservation: true,
            adaptive_parameters: std::env::var("BEARDOG_GENETICS_ADAPTIVE_PARAMETERS")
                .ok()
                .and_then(|v| v.parse().ok())
                .unwrap_or(true),
        }
    }
}

// ============================================================
// Individual and Population Types
// ============================================================

/// Genetic individual
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct GeneticIndividual {
    /// Unique ID
    pub id: String,

    /// Genetic signature
    pub genetic_signature: GeneticSignature,

    /// Fitness score (0.0 - 1.0)
    pub fitness_score: f64,

    /// Age in generations
    pub age: u32,

    /// Parent IDs
    pub parent_ids: Vec<String>,

    /// Mutation history
    pub mutation_history: Vec<MutationRecord>,

    /// Performance metrics
    pub performance_metrics: PerformanceMetrics,

    /// Specialization traits
    pub specialization_traits: Vec<String>,
}

impl Default for GeneticIndividual {
    fn default() -> Self {
        Self {
            id: Uuid::new_v4().to_string(),
            genetic_signature: GeneticSignature::default(),
            fitness_score: 0.0,
            age: 0,
            parent_ids: Vec::new(),
            mutation_history: Vec::new(),
            performance_metrics: PerformanceMetrics::default(),
            specialization_traits: Vec::new(),
        }
    }
}

/// Generation snapshot
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct GenerationSnapshot {
    /// Generation number
    pub generation: u32,

    /// Timestamp
    pub timestamp: DateTime<Utc>,

    /// Population size
    pub population_size: usize,

    /// Average fitness
    pub average_fitness: f64,

    /// Best fitness
    pub best_fitness: f64,

    /// Diversity index
    pub diversity_index: f64,

    /// Convergence rate
    pub convergence_rate: f64,
}

impl Default for GenerationSnapshot {
    fn default() -> Self {
        Self {
            generation: 0,
            timestamp: Utc::now(),
            population_size: 0,
            average_fitness: 0.0,
            best_fitness: 0.0,
            diversity_index: 0.0,
            convergence_rate: 0.0,
        }
    }
}

/// Mutation record
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct MutationRecord {
    /// Mutation type
    pub mutation_type: MutationType,

    /// Timestamp
    pub timestamp: DateTime<Utc>,

    /// Fitness impact
    pub fitness_impact: f64,

    /// Success rate
    pub success_rate: f64,
}

impl Default for MutationRecord {
    fn default() -> Self {
        Self {
            mutation_type: MutationType::Random,
            timestamp: Utc::now(),
            fitness_impact: 0.0,
            success_rate: 0.0,
        }
    }
}

/// Mutation type
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub enum MutationType {
    /// Random mutation
    Random,
    /// Entropy quality adjustment
    EntropyQualityAdjustment,
    /// Trust level shift
    TrustLevelShift,
    /// Specialization change
    SpecializationChange,
    /// Performance optimization
    PerformanceOptimization,
}

impl Default for MutationType {
    fn default() -> Self {
        Self::Random
    }
}

/// Performance metrics
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PerformanceMetrics {
    /// Authorization success rate
    pub authorization_success_rate: f64,

    /// Operation efficiency
    pub operation_efficiency: f64,

    /// Collaboration score
    pub collaboration_score: f64,

    /// Security rating
    pub security_rating: f64,

    /// Adaptability index
    pub adaptability_index: f64,

    /// Resource utilization
    pub resource_utilization: f64,
}

impl Default for PerformanceMetrics {
    fn default() -> Self {
        Self {
            authorization_success_rate: 0.5,
            operation_efficiency: 0.5,
            collaboration_score: 0.5,
            security_rating: 0.5,
            adaptability_index: 0.5,
            resource_utilization: 0.5,
        }
    }
}

// ============================================================
// Fitness Types
// ============================================================

/// Fitness criterion
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct FitnessCriterion {
    /// Criterion name
    pub name: String,

    /// Weight (0.0 - 1.0)
    pub weight: f64,

    /// Evaluation function identifier
    pub evaluation_function: String,

    /// Target value
    pub target_value: f64,

    /// Tolerance
    pub tolerance: f64,
}

impl Default for FitnessCriterion {
    fn default() -> Self {
        Self {
            name: String::new(),
            weight: 1.0,
            evaluation_function: String::new(),
            target_value: 1.0,
            tolerance: 0.1,
        }
    }
}

// ============================================================
// Optimization Types
// ============================================================

/// Optimization goal
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct OptimizationGoal {
    /// Goal name
    pub name: String,

    /// Target value
    pub target_value: f64,

    /// Optimization strategy
    pub optimization_strategy: String,

    /// Constraints
    pub constraints: Vec<OptimizationConstraint>,
}

impl Default for OptimizationGoal {
    fn default() -> Self {
        Self {
            name: String::new(),
            target_value: 1.0,
            optimization_strategy: "maximize".to_string(),
            constraints: Vec::new(),
        }
    }
}

/// Optimization constraint
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct OptimizationConstraint {
    /// Constraint name
    pub name: String,

    /// Minimum value
    pub min_value: Option<f64>,

    /// Maximum value
    pub max_value: Option<f64>,

    /// Fixed value
    pub fixed_value: Option<f64>,
}

impl Default for OptimizationConstraint {
    fn default() -> Self {
        Self {
            name: String::new(),
            min_value: None,
            max_value: None,
            fixed_value: None,
        }
    }
}

// ============================================================
// Engine Components
// ============================================================

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
        if fastrand::f64() < rate {
            // Apply random mutation
            individual.fitness_score *= 0.9 + fastrand::f64() * 0.2;
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

        for _ in 0..num_parents {
            let mut best_idx = fastrand::usize(..population.len());
            let mut best_fitness = population[best_idx].fitness_score;

            for _ in 1..tournament_size {
                let idx = fastrand::usize(..population.len());
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

// ============================================================
// Metrics Types
// ============================================================

/// Evolution metrics
#[derive(Debug)]
pub struct EvolutionMetrics {
    /// Total generations
    pub total_generations: AtomicU64,

    /// Successful mutations
    pub successful_mutations: AtomicU64,

    /// Successful crossovers
    pub successful_crossovers: AtomicU64,

    /// Convergence events
    pub convergence_events: AtomicU64,

    /// Diversity events
    pub diversity_events: AtomicU64,

    /// Fitness improvements
    pub fitness_improvements: AtomicU64,
}

impl EvolutionMetrics {
    /// Create new metrics
    pub fn new() -> Self {
        Self {
            total_generations: AtomicU64::new(0),
            successful_mutations: AtomicU64::new(0),
            successful_crossovers: AtomicU64::new(0),
            convergence_events: AtomicU64::new(0),
            diversity_events: AtomicU64::new(0),
            fitness_improvements: AtomicU64::new(0),
        }
    }
}

impl Clone for EvolutionMetrics {
    fn clone(&self) -> Self {
        Self {
            total_generations: AtomicU64::new(self.total_generations.load(Ordering::Relaxed)),
            successful_mutations: AtomicU64::new(self.successful_mutations.load(Ordering::Relaxed)),
            successful_crossovers: AtomicU64::new(
                self.successful_crossovers.load(Ordering::Relaxed),
            ),
            convergence_events: AtomicU64::new(self.convergence_events.load(Ordering::Relaxed)),
            diversity_events: AtomicU64::new(self.diversity_events.load(Ordering::Relaxed)),
            fitness_improvements: AtomicU64::new(self.fitness_improvements.load(Ordering::Relaxed)),
        }
    }
}

impl Default for EvolutionMetrics {
    fn default() -> Self {
        Self::new()
    }
}

/// Diversity metrics
#[derive(Debug)]
pub struct DiversityMetrics {
    /// Genetic diversity
    pub genetic_diversity: AtomicU64,

    /// Phenotypic diversity
    pub phenotypic_diversity: AtomicU64,

    /// Behavioral diversity
    pub behavioral_diversity: AtomicU64,

    /// Specialization spread
    pub specialization_spread: AtomicU64,
}

impl DiversityMetrics {
    /// Create new metrics
    pub fn new() -> Self {
        Self {
            genetic_diversity: AtomicU64::new(0),
            phenotypic_diversity: AtomicU64::new(0),
            behavioral_diversity: AtomicU64::new(0),
            specialization_spread: AtomicU64::new(0),
        }
    }
}

impl Clone for DiversityMetrics {
    fn clone(&self) -> Self {
        Self {
            genetic_diversity: AtomicU64::new(self.genetic_diversity.load(Ordering::Relaxed)),
            phenotypic_diversity: AtomicU64::new(self.phenotypic_diversity.load(Ordering::Relaxed)),
            behavioral_diversity: AtomicU64::new(self.behavioral_diversity.load(Ordering::Relaxed)),
            specialization_spread: AtomicU64::new(
                self.specialization_spread.load(Ordering::Relaxed),
            ),
        }
    }
}

impl Default for DiversityMetrics {
    fn default() -> Self {
        Self::new()
    }
}

// ============================================================
// Main Engine
// ============================================================

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
        let generation = self
            .evolution_metrics
            .total_generations
            .fetch_add(1, Ordering::Relaxed) as u32;

        // Calculate fitness for all individuals
        let mut evaluated: Vec<_> = population
            .into_iter()
            .map(|mut ind| {
                ind.fitness_score = self.fitness_evaluator.evaluate_fitness(&ind).unwrap_or(0.0);
                ind.age += 1;
                ind
            })
            .collect();

        // Selection
        let num_elite = (evaluated.len() as f64 * self.evolution_config.elitism_percentage) as usize;
        let elite_indices = self
            .selection_engine
            .select_survivors(&evaluated, num_elite)?;

        // Keep elite individuals
        let mut next_gen: Vec<_> = elite_indices.iter().map(|&i| evaluated[i].clone()).collect();

        // Generate offspring
        while next_gen.len() < self.evolution_config.population_size {
            let parent_indices = self.selection_engine.select_parents(&evaluated, 2)?;
            if parent_indices.len() >= 2 {
                let offspring = self.crossover_engine.crossover(
                    &evaluated[parent_indices[0]],
                    &evaluated[parent_indices[1]],
                )?;
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
        let avg_fitness = next_gen.iter().map(|i| i.fitness_score).sum::<f64>() / next_gen.len() as f64;
        let best_fitness = next_gen
            .iter()
            .map(|i| i.fitness_score)
            .fold(0.0f64, |a, b| a.max(b));

        self.population_manager.set_population(next_gen);

        Ok(GenerationSnapshot {
            generation,
            timestamp: Utc::now(),
            population_size: self.evolution_config.population_size,
            average_fitness: avg_fitness,
            best_fitness,
            diversity_index: 0.5, // Placeholder
            convergence_rate: 0.0,
        })
    }

    /// Get best individual
    pub fn get_best_individual(&self) -> Option<GeneticIndividual> {
        let population = self.population_manager.get_population();
        population
            .into_iter()
            .max_by(|a, b| a.fitness_score.partial_cmp(&b.fitness_score).unwrap())
    }
}

impl Default for GeneticEvolutionEngine {
    fn default() -> Self {
        Self::new(EvolutionConfig::default())
    }
}

#[cfg(test)]
mod tests {
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
        let population = engine.initialize_population(signatures).unwrap();
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

        let survivors = engine.select_survivors(&population, 2).unwrap();
        assert_eq!(survivors.len(), 2);
        assert_eq!(survivors[0], 0); // Highest fitness first
    }
}
