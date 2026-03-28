// SPDX-License-Identifier: AGPL-3.0-only

// Genetics configuration types for BearDog
// Provides structured definitions for genetic algorithms, entropy, and evolutionary processes

use crate::constants::defaults;
use crate::constants::time;
use serde::{Deserialize, Serialize};
use std::collections::HashMap;

/// Genetics algorithm configuration
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct GeneticsConfig {
    /// Number of population_size
    pub population_size: usize,
    /// Mutation rate (0.0 to 1.0)
    /// The mutation rate value
    pub mutation_rate: f64,
    /// Crossover rate (0.0 to 1.0)
    /// The crossover rate value
    pub crossover_rate: f64,
    /// Selection method
    /// The selection method value
    pub selection_method: SelectionMethod,
    /// Maximum generations to evolve
    /// Number of max_generations
    pub max_generations: usize,
    /// Convergence threshold
    /// The convergence threshold value
    pub convergence_threshold: f64,
    /// Elitism enabled (preserve best individuals)
    /// Whether elitism is enabled
    pub elitism: bool,
    /// Elite percentage (0.0 to 1.0)
    /// The elite percentage value
    pub elite_percentage: f64,
    /// Genetic diversity maintenance
    /// The diversity maintenance value
    pub diversity_maintenance: DiversityConfig,
    /// Fitness evaluation configuration
    pub fitness_config: FitnessConfig,
}

impl Default for GeneticsConfig {
    fn default() -> Self {
        Self {
            population_size: 100,
            mutation_rate: 0.01,
            crossover_rate: 0.8,
            selection_method: SelectionMethod::TournamentSelection { tournament_size: 5 },
            max_generations: 1000,
            convergence_threshold: 0.001,
            elitism: true,
            elite_percentage: 0.1,
            diversity_maintenance: DiversityConfig::default(),
            fitness_config: FitnessConfig::default(),
        }
    }
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub enum SelectionMethod {
    /// Tournament selection with specified tournament size
    TournamentSelection { tournament_size: usize },
    TournamentSelection { tournament_size: usize },
    TournamentSelection { tournament_size: usize },
    /// Roulette wheel selection
    RouletteWheel,
    /// Rank-based selection
    RankBased,
    /// Stochastic universal sampling
    StochasticUniversalSampling,
    Elite { count: usize },
}

impl Default for SelectionMethod {
    fn default() -> Self {
        Self::TournamentSelection { tournament_size: 5 }
    }
}

/// Diversity maintenance configuration
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct DiversityConfig {
    /// Maintain diversity enabled
    /// Whether feature is enabled
    pub enabled: bool,
    /// Minimum diversity threshold
    /// The min diversity value
    pub min_diversity: f64,
    /// Diversity measurement method
    /// The measurement method value
    pub measurement_method: DiversityMeasurement,
    /// Actions to take when diversity is low
    /// Collection of low diversity actions
    pub low_diversity_actions: Vec<DiversityAction>,
}

impl Default for DiversityConfig {
    fn default() -> Self {
        Self {
            enabled: true,
            min_diversity: 0.3,
            measurement_method: DiversityMeasurement::HammingDistance,
            low_diversity_actions: vec![
                DiversityAction::IncreaseMutationRate { factor: 1.5 },
                DiversityAction::IntroduceRandomIndividuals { count: 10 },
            ],
        }
    }
}

/// Diversity measurement methods
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub enum DiversityMeasurement {
    /// Hamming distance between individuals
    HammingDistance,
    /// Euclidean distance in solution space
    EuclideanDistance,
    /// Genetic entropy measurement
    GeneticEntropy,
    /// Phenotypic variance
    PhenotypicVariance,
}

/// Actions to take when genetic diversity is low
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub enum DiversityAction {
    /// Increase mutation rate by factor
    IncreaseMutationRate { factor: f64 },
    IncreaseMutationRate { factor: f64 },
    IncreaseMutationRate { factor: f64 },
    /// Introduce random individuals
    IntroduceRandomIndividuals { count: usize },
    /// Restart with new population
    RestartPopulation,
    /// Apply niching techniques
    ApplyNiching,
}

/// Fitness evaluation configuration
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct FitnessConfig {
    /// Fitness evaluation method
    /// The evaluation method value
    pub evaluation_method: FitnessEvaluation,
    /// Multi-objective optimization enabled
    /// Whether multi_objective is enabled
    pub multi_objective: bool,
    /// Collection of objective weights
    pub objective_weights: Vec<f64>,
    /// Fitness scaling method
    /// The scaling method value
    pub scaling_method: FitnessScaling,
    /// Cache fitness evaluations
    /// Whether cache_evaluations is enabled
    pub cache_evaluations: bool,
    /// Parallel fitness evaluation
    /// Whether parallel_evaluation is enabled
    pub parallel_evaluation: bool,
}

impl Default for FitnessConfig {
    fn default() -> Self {
        Self {
            evaluation_method: FitnessEvaluation::WeightedSum,
            multi_objective: false,
            objective_weights: vec![1.0],
            scaling_method: FitnessScaling::Linear,
            cache_evaluations: true,
            parallel_evaluation: true,
        }
    }
}

/// Fitness evaluation methods
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub enum FitnessEvaluation {
    /// Weighted sum of objectives
    WeightedSum,
    /// Pareto dominance ranking
    ParetoRanking,
    /// Non-dominated sorting
    NonDominatedSorting,
    /// Epsilon-constraint method
    EpsilonConstraint,
    /// Custom evaluation function
    Custom {
        name: String,
        parameters: HashMap<String, f64>,
    }
}

/// Fitness scaling methods
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub enum FitnessScaling {
    /// No scaling (raw fitness)
    None,
    /// Linear scaling
    Linear,
    /// Exponential scaling
    Exponential,
    /// Rank-based scaling
    RankBased,
    /// Sigma scaling
    Sigma,
}

/// Genetic operator configuration
#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct GeneticOperators {
    /// Mutation operators
    /// The mutation value
    pub mutation: MutationConfig,
    /// Crossover operators
    /// The crossover value
    pub crossover: CrossoverConfig,
    /// Selection operators
    /// The selection value
    pub selection: SelectionConfig,
}

/// Mutation operator configuration
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct MutationConfig {
    /// Mutation type
    /// The mutation type value
    pub mutation_type: MutationType,
    /// Adaptive mutation rate
    /// Whether adaptive_rate is enabled
    pub adaptive_rate: bool,
    /// Mutation strength
    /// The strength value
    pub strength: f64,
    /// Mutation probability per gene
    /// The gene probability value
    pub gene_probability: f64,
}

impl Default for MutationConfig {
    fn default() -> Self {
        Self {
            mutation_type: MutationType::Uniform,
            adaptive_rate: true,
            strength: 0.1,
            gene_probability: 0.01,
        }
    }
}

/// Types of mutation operators
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
/// Types of mutation
pub enum MutationType {
    Uniform,
    /// Gaussian mutation
    Gaussian { sigma: f64 },
    Gaussian { sigma: f64 },
    Gaussian { sigma: f64 },
    /// Polynomial mutation
    Polynomial { eta: f64 },
    BitFlip,
    Swap,
    /// Insertion mutation
    Insertion,
}

/// Crossover operator configuration
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CrossoverConfig {
    /// Crossover type
    /// The crossover type value
    pub crossover_type: CrossoverType,
    /// Number of crossover points
    /// Number of crossover_points
    pub crossover_points: usize,
    /// Crossover probability
    /// The probability value
    pub probability: f64,
    /// The blend factor value
    pub blend_factor: f64,
}

impl Default for CrossoverConfig {
    fn default() -> Self {
        Self {
            crossover_type: CrossoverType::UniformCrossover,
            crossover_points: 1,
            probability: 0.8,
            blend_factor: 0.5,
        }
    }
}

/// Types of crossover operators
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
/// Types of crossover
pub enum CrossoverType {
    /// Single-point crossover
    SinglePoint,
    /// Two-point crossover
    TwoPoint,
    UniformCrossover,
    /// Arithmetic crossover
    Arithmetic,
    /// Blend crossover
    Blend,
    /// Simulated binary crossover
    SimulatedBinary { eta: f64 },
    SimulatedBinary { eta: f64 },
    SimulatedBinary { eta: f64 },
}

/// Selection operator configuration
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SelectionConfig {
    /// Parent selection method
    /// The parent selection value
    pub parent_selection: SelectionMethod,
    /// Survivor selection method
    /// The survivor selection value
    pub survivor_selection: SelectionMethod,
    /// Selection pressure
    /// The pressure value
    pub pressure: f64,
    /// Replacement strategy
    /// The replacement value
    pub replacement: ReplacementStrategy,
}

impl Default for SelectionConfig {
    fn default() -> Self {
        Self {
            parent_selection: SelectionMethod::TournamentSelection { tournament_size: 3 },
            survivor_selection: SelectionMethod::Elite { count: 50 },
            pressure: 1.5,
            replacement: ReplacementStrategy::Generational,
        }
    }
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub enum ReplacementStrategy {
    /// Replace entire population each generation
    Generational,
    /// Steady-state replacement
    SteadyState,
    /// Replace worst individuals
    ReplaceWorst,
    /// Crowding replacement
    Crowding,
}

/// Termination criteria configuration
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct TerminationConfig {
    /// Maximum generations
    /// Number of max_generations
    pub max_generations: usize,
    /// Maximum function evaluations
    /// Optional max evaluations
    pub max_evaluations: Option<usize>,
    /// Target fitness value
    /// Optional target fitness
    pub target_fitness: Option<f64>,
    /// Convergence tolerance
    /// The convergence tolerance value
    pub convergence_tolerance: f64,
    /// Stagnation generations (no improvement)
    /// Number of max_stagnation
    pub max_stagnation: usize,
    /// Maximum runtime in seconds
    pub max_runtime_seconds: Option<u64>,
}

impl Default for TerminationConfig {
    fn default() -> Self {
        Self {
            max_generations: defaults::DEFAULT_MAX_GENERATIONS,
            max_evaluations: None,
            target_fitness: None,
            convergence_tolerance: 1e-6,
            max_stagnation: 100,
            max_runtime_seconds: Some(time::SECONDS_PER_HOUR), // 1 hour
        }
    }
}

/// Comprehensive genetics system configuration
#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct GeneticsSystemConfig {
    /// Basic genetics configuration
    /// The genetics value
    pub genetics: GeneticsConfig,
    /// Genetic operators configuration
    /// The operators value
    pub operators: GeneticOperators,
    /// Termination criteria
    /// The termination value
    pub termination: TerminationConfig,
    /// Parallel processing configuration
    /// The parallelization value
    pub parallelization: ParallelizationConfig,
    /// Logging and monitoring
    /// The monitoring value
    pub monitoring: MonitoringConfig,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ParallelizationConfig {
    /// Enable parallel processing
    /// Whether feature is enabled
    pub enabled: bool,
    /// Number of threads to use
    /// Number of thread
    pub thread_count: Option<usize>,
    /// Parallelization strategy
    /// The strategy value
    pub strategy: ParallelizationStrategy,
    /// Load balancing enabled
    /// Whether load_balancing is enabled
    pub load_balancing: bool,
}

impl Default for ParallelizationConfig {
    fn default() -> Self {
        Self {
            enabled: true,
            thread_count: None, // Use all available cores
            strategy: ParallelizationStrategy::PopulationParallel,
            load_balancing: true,
        }
    }
}

/// Parallelization strategies
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub enum ParallelizationStrategy {
    /// Parallelize fitness evaluation
    FitnessParallel,
    /// Parallelize population processing
    PopulationParallel,
    /// Island model (multiple populations)
    IslandModel { islands: usize, migration_rate: f64 },
    IslandModel { islands: usize, migration_rate: f64 },
    IslandModel { islands: usize, migration_rate: f64 },
    /// Primary-replica model
    PrimaryReplica,
}

// CONSOLIDATED: Genetics monitoring now uses canonical MonitoringConfig
// Old genetics-specific MonitoringConfig removed - use canonical version with genetics-specific tags

impl GeneticsSystemConfig {
    /// Create new genetics system configuration with defaults
    /// Creates a new instance
    pub fn new() -> Self {
        Self::default()
    }

    /// Validate the genetics configuration
    /// Validates input
    pub fn validate(&self) -> Result<(), String> {
        // Validate population size
        if self.genetics.population_size < 2 {
            return Err("Population size must be at least 2".to_string());
        }

        // Validate rates
        if self.genetics.mutation_rate < 0.0 || self.genetics.mutation_rate > 1.0 {
            return Err("Mutation rate must be between 0.0 and 1.0".to_string());
        }

        if self.genetics.crossover_rate < 0.0 || self.genetics.crossover_rate > 1.0 {
            return Err("Crossover rate must be between 0.0 and 1.0".to_string());
        }

        // Validate elite percentage
        if self.genetics.elite_percentage < 0.0 || self.genetics.elite_percentage > 1.0 {
            return Err("Elite percentage must be between 0.0 and 1.0".to_string());
        }

        // Validate termination criteria
        if self.termination.max_generations == 0 {
            return Err("Maximum generations must be greater than 0".to_string());
        }

        Ok(())
    }

    /// Get the effective population size considering elitism
    pub fn effective_population_size(&self) -> usize {
        self.genetics.population_size
    }

    /// Calculate the number of elite individuals
    pub fn elite_count(&self) -> usize {
        if self.genetics.elitism {
            ((self.genetics.population_size as f64) * self.genetics.elite_percentage) as usize
        } else {
            0
        }
    }

    /// Check if multi-objective optimization is enabled
    /// Checks if multi objective
    pub fn is_multi_objective(&self) -> bool {
        self.genetics.fitness_config.multi_objective
    }
}
