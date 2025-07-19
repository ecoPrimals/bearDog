//! Parallel Processing Configuration for Genetic Algorithms
//!
//! This module handles parallel processing configurations for genetic algorithms.

use serde::{Deserialize, Serialize};
use std::time::Duration;

/// Genetic parallel processing configuration
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct GeneticParallelProcessingConfig {
    /// Enable parallel processing
    pub enabled: bool,
    /// Number of worker threads
    pub worker_threads: u32,
    /// Population partitioning strategy
    pub partitioning_strategy: PopulationPartitioningStrategy,
    /// Parallel evaluation configuration
    pub parallel_evaluation: ParallelEvaluationConfig,
    /// Synchronization strategy
    pub synchronization: GeneticSynchronizationConfig,
    /// Load balancing configuration
    pub load_balancing: GeneticLoadBalancingConfig,
}

/// Population partitioning strategy
#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum PopulationPartitioningStrategy {
    /// Fixed-size partitions
    FixedSize { partition_size: u32 },
    /// Dynamic partitioning based on fitness
    DynamicFitness { threshold: f64 },
    /// Island model partitioning
    IslandModel { island_count: u32 },
    /// Hierarchical partitioning
    Hierarchical { levels: u32 },
}

/// Parallel evaluation configuration
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ParallelEvaluationConfig {
    /// Enable parallel evaluation
    pub enabled: bool,
    /// Batch size for parallel evaluation
    pub batch_size: u32,
    /// Evaluation timeout
    pub timeout: Duration,
    /// Caching configuration
    pub caching: EvaluationCachingConfig,
    /// Maximum concurrent evaluations
    pub max_concurrent: u32,
}

/// Evaluation caching configuration
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct EvaluationCachingConfig {
    /// Enable caching
    pub enabled: bool,
    /// Maximum cache size
    pub max_cache_size: u32,
    /// Cache expiry time
    pub cache_expiry: Duration,
    /// Cache key strategy
    pub key_strategy: CacheKeyStrategy,
}

/// Cache key strategy
#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum CacheKeyStrategy {
    /// Hash-based key
    Hash,
    /// Phenotype-based key
    Phenotype,
    /// Genotype-based key
    Genotype,
}

/// Genetic synchronization configuration
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct GeneticSynchronizationConfig {
    /// Synchronization strategy
    pub strategy: GeneticSynchronizationStrategy,
    /// Migration configuration
    pub migration: MigrationConfig,
    /// Convergence detection
    pub convergence_detection: ConvergenceDetectionConfig,
}

/// Genetic synchronization strategy
#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum GeneticSynchronizationStrategy {
    /// Synchronous execution
    Synchronous,
    /// Asynchronous execution
    Asynchronous,
    /// Semi-synchronous execution
    SemiSynchronous { sync_interval: u32 },
}

/// Migration configuration
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct MigrationConfig {
    /// Enable migration
    pub enabled: bool,
    /// Migration interval
    pub interval: u32,
    /// Migration rate
    pub rate: f64,
    /// Migration topology
    pub topology: MigrationTopology,
    /// Selection strategy
    pub selection: MigrationSelectionStrategy,
}

/// Migration topology
#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum MigrationTopology {
    /// Ring topology
    Ring,
    /// Star topology
    Star,
    /// Complete graph topology
    Complete,
    /// Random topology
    Random { connection_probability: f64 },
}

/// Migration selection strategy
#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum MigrationSelectionStrategy {
    /// Best individuals
    Best,
    /// Random individuals
    Random,
    /// Tournament selection
    Tournament { tournament_size: u32 },
    /// Roulette wheel selection
    RouletteWheel,
}

/// Convergence detection configuration
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ConvergenceDetectionConfig {
    /// Enable convergence detection
    pub enabled: bool,
    /// Convergence threshold
    pub threshold: f64,
    /// Convergence window
    pub window: u32,
    /// Convergence metric
    pub metric: ConvergenceMetric,
    /// Action on convergence
    pub action: ConvergenceAction,
}

/// Convergence metric
#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum ConvergenceMetric {
    /// Fitness variance
    FitnessVariance,
    /// Population diversity
    PopulationDiversity,
    /// Improvement rate
    ImprovementRate,
    /// Stagnation count
    StagnationCount,
}

/// Convergence action
#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum ConvergenceAction {
    /// Stop evolution
    Stop,
    /// Restart with new population
    Restart,
    /// Increase mutation rate
    IncreaseMutation,
    /// Migrate from other populations
    Migrate,
}

/// Genetic load balancing configuration
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct GeneticLoadBalancingConfig {
    /// Enable load balancing
    pub enabled: bool,
    /// Load balancing strategy
    pub strategy: GeneticLoadBalancingStrategy,
    /// Work stealing configuration
    pub work_stealing: WorkStealingConfig,
    /// Load threshold
    pub load_threshold: f64,
}

/// Genetic load balancing strategy
#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum GeneticLoadBalancingStrategy {
    /// Round-robin assignment
    RoundRobin,
    /// Weighted round-robin
    WeightedRoundRobin,
    /// Least loaded first
    LeastLoaded,
    /// Work stealing
    WorkStealing,
}

/// Work stealing configuration
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct WorkStealingConfig {
    /// Enable work stealing
    pub enabled: bool,
    /// Steal threshold
    pub steal_threshold: f64,
    /// Work stealing strategy
    pub strategy: WorkStealingStrategy,
}

/// Work stealing strategy
#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum WorkStealingStrategy {
    /// Random stealing
    Random,
    /// Least loaded target
    LeastLoaded,
    /// Round-robin stealing
    RoundRobin,
}

impl Default for GeneticParallelProcessingConfig {
    fn default() -> Self {
        Self {
            enabled: true,
            worker_threads: 4,
            partitioning_strategy: PopulationPartitioningStrategy::FixedSize {
                partition_size: 100,
            },
            parallel_evaluation: ParallelEvaluationConfig::default(),
            synchronization: GeneticSynchronizationConfig::default(),
            load_balancing: GeneticLoadBalancingConfig::default(),
        }
    }
}

impl Default for ParallelEvaluationConfig {
    fn default() -> Self {
        Self {
            enabled: true,
            batch_size: 32,
            timeout: Duration::from_secs(60),
            caching: EvaluationCachingConfig::default(),
            max_concurrent: 100,
        }
    }
}

impl Default for EvaluationCachingConfig {
    fn default() -> Self {
        Self {
            enabled: true,
            max_cache_size: 1000,
            cache_expiry: Duration::from_secs(3600),
            key_strategy: CacheKeyStrategy::Hash,
        }
    }
}

impl Default for GeneticSynchronizationConfig {
    fn default() -> Self {
        Self {
            strategy: GeneticSynchronizationStrategy::Synchronous,
            migration: MigrationConfig::default(),
            convergence_detection: ConvergenceDetectionConfig::default(),
        }
    }
}

impl Default for MigrationConfig {
    fn default() -> Self {
        Self {
            enabled: false,
            interval: 10,
            rate: 0.1,
            topology: MigrationTopology::Ring,
            selection: MigrationSelectionStrategy::Best,
        }
    }
}

impl Default for ConvergenceDetectionConfig {
    fn default() -> Self {
        Self {
            enabled: true,
            threshold: 0.001,
            window: 20,
            metric: ConvergenceMetric::FitnessVariance,
            action: ConvergenceAction::Stop,
        }
    }
}

impl Default for GeneticLoadBalancingConfig {
    fn default() -> Self {
        Self {
            enabled: true,
            strategy: GeneticLoadBalancingStrategy::RoundRobin,
            work_stealing: WorkStealingConfig::default(),
            load_threshold: 0.8,
        }
    }
}

impl Default for WorkStealingConfig {
    fn default() -> Self {
        Self {
            enabled: false,
            steal_threshold: 0.5,
            strategy: WorkStealingStrategy::Random,
        }
    }
}

impl GeneticParallelProcessingConfig {
    /// Create production parallel processing configuration
    pub fn production() -> Self {
        Self {
            enabled: true,
            worker_threads: 8,
            partitioning_strategy: PopulationPartitioningStrategy::IslandModel { island_count: 4 },
            parallel_evaluation: ParallelEvaluationConfig::production(),
            synchronization: GeneticSynchronizationConfig::production(),
            load_balancing: GeneticLoadBalancingConfig::production(),
        }
    }

    /// Create development parallel processing configuration
    pub fn development() -> Self {
        Self {
            enabled: false,
            worker_threads: 2,
            partitioning_strategy: PopulationPartitioningStrategy::FixedSize { partition_size: 50 },
            parallel_evaluation: ParallelEvaluationConfig::development(),
            synchronization: GeneticSynchronizationConfig::development(),
            load_balancing: GeneticLoadBalancingConfig::development(),
        }
    }
}

impl ParallelEvaluationConfig {
    /// Create production parallel evaluation configuration
    pub fn production() -> Self {
        Self {
            enabled: true,
            batch_size: 64,
            timeout: Duration::from_secs(30),
            caching: EvaluationCachingConfig::production(),
            max_concurrent: 200,
        }
    }

    /// Create development parallel evaluation configuration
    pub fn development() -> Self {
        Self {
            enabled: false,
            batch_size: 16,
            timeout: Duration::from_secs(120),
            caching: EvaluationCachingConfig::development(),
            max_concurrent: 50,
        }
    }
}

impl EvaluationCachingConfig {
    /// Create production evaluation caching configuration
    pub fn production() -> Self {
        Self {
            enabled: true,
            max_cache_size: 10000,
            cache_expiry: Duration::from_secs(1800),
            key_strategy: CacheKeyStrategy::Hash,
        }
    }

    /// Create development evaluation caching configuration
    pub fn development() -> Self {
        Self {
            enabled: false,
            max_cache_size: 100,
            cache_expiry: Duration::from_secs(600),
            key_strategy: CacheKeyStrategy::Phenotype,
        }
    }
}

impl GeneticSynchronizationConfig {
    /// Create production synchronization configuration
    pub fn production() -> Self {
        Self {
            strategy: GeneticSynchronizationStrategy::SemiSynchronous { sync_interval: 5 },
            migration: MigrationConfig::production(),
            convergence_detection: ConvergenceDetectionConfig::production(),
        }
    }

    /// Create development synchronization configuration
    pub fn development() -> Self {
        Self {
            strategy: GeneticSynchronizationStrategy::Synchronous,
            migration: MigrationConfig::development(),
            convergence_detection: ConvergenceDetectionConfig::development(),
        }
    }
}

impl MigrationConfig {
    /// Create production migration configuration
    pub fn production() -> Self {
        Self {
            enabled: true,
            interval: 5,
            rate: 0.2,
            topology: MigrationTopology::Complete,
            selection: MigrationSelectionStrategy::Tournament { tournament_size: 3 },
        }
    }

    /// Create development migration configuration
    pub fn development() -> Self {
        Self {
            enabled: false,
            interval: 20,
            rate: 0.05,
            topology: MigrationTopology::Ring,
            selection: MigrationSelectionStrategy::Best,
        }
    }
}

impl ConvergenceDetectionConfig {
    /// Create production convergence detection configuration
    pub fn production() -> Self {
        Self {
            enabled: true,
            threshold: 0.0001,
            window: 50,
            metric: ConvergenceMetric::ImprovementRate,
            action: ConvergenceAction::IncreaseMutation,
        }
    }

    /// Create development convergence detection configuration
    pub fn development() -> Self {
        Self {
            enabled: false,
            threshold: 0.01,
            window: 10,
            metric: ConvergenceMetric::FitnessVariance,
            action: ConvergenceAction::Stop,
        }
    }
}

impl GeneticLoadBalancingConfig {
    /// Create production load balancing configuration
    pub fn production() -> Self {
        Self {
            enabled: true,
            strategy: GeneticLoadBalancingStrategy::WorkStealing,
            work_stealing: WorkStealingConfig::production(),
            load_threshold: 0.9,
        }
    }

    /// Create development load balancing configuration
    pub fn development() -> Self {
        Self {
            enabled: false,
            strategy: GeneticLoadBalancingStrategy::RoundRobin,
            work_stealing: WorkStealingConfig::development(),
            load_threshold: 0.7,
        }
    }
}

impl WorkStealingConfig {
    /// Create production work stealing configuration
    pub fn production() -> Self {
        Self {
            enabled: true,
            steal_threshold: 0.3,
            strategy: WorkStealingStrategy::LeastLoaded,
        }
    }

    /// Create development work stealing configuration
    pub fn development() -> Self {
        Self {
            enabled: false,
            steal_threshold: 0.5,
            strategy: WorkStealingStrategy::Random,
        }
    }
}
