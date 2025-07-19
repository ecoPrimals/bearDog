//! Parallel Processing Configuration for SIMD Cryptography
//!
//! This module handles parallel processing configurations for SIMD crypto operations.

use serde::{Deserialize, Serialize};

/// Parallel crypto configuration
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ParallelCryptoConfig {
    /// Thread pool configuration
    pub thread_pool: ThreadPoolConfig,
    /// Batch processing configuration
    pub batch_processing: BatchProcessingConfig,
    /// Synchronization configuration
    pub synchronization: SynchronizationConfig,
    /// Load balancing configuration
    pub load_balancing: LoadBalancingConfig,
}

/// Thread pool configuration
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ThreadPoolConfig {
    /// Maximum number of threads
    pub max_threads: u32,
    /// Minimum number of threads
    pub min_threads: u32,
    /// Thread priority
    pub thread_priority: ThreadPriority,
    /// Thread affinity configuration
    pub thread_affinity: ThreadAffinityConfig,
}

/// Thread priority
#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum ThreadPriority {
    /// Low priority
    Low,
    /// Normal priority
    Normal,
    /// High priority
    High,
    /// Real-time priority
    RealTime,
}

/// Thread affinity configuration
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ThreadAffinityConfig {
    /// Enable thread affinity
    pub enabled: bool,
    /// Affinity strategy
    pub strategy: AffinityStrategy,
}

/// Affinity strategy
#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum AffinityStrategy {
    /// No affinity
    None,
    /// Core affinity
    Core,
    /// NUMA affinity
    NUMA,
    /// Cache affinity
    Cache,
}

/// Batch processing configuration
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct BatchProcessingConfig {
    /// Enable batch processing
    pub enabled: bool,
    /// Batch size
    pub batch_size: u32,
    /// Batch processing strategy
    pub strategy: BatchProcessingStrategy,
}

/// Batch processing strategy
#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum BatchProcessingStrategy {
    /// Fixed batch size
    Fixed,
    /// Dynamic batch size
    Dynamic,
    /// Adaptive batch size
    Adaptive,
}

/// Synchronization configuration
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SynchronizationConfig {
    /// Synchronization primitive
    pub primitive: SynchronizationPrimitive,
    /// Work distribution strategy
    pub work_distribution: WorkDistributionStrategy,
}

/// Synchronization primitive
#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum SynchronizationPrimitive {
    /// Mutex
    Mutex,
    /// RwLock
    RwLock,
    /// Atomic operations
    Atomic,
    /// Lock-free structures
    LockFree,
}

/// Work distribution strategy
#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum WorkDistributionStrategy {
    /// Round-robin
    RoundRobin,
    /// Work stealing
    WorkStealing,
    /// Static partitioning
    StaticPartitioning,
}

/// Load balancing configuration
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct LoadBalancingConfig {
    /// Enable load balancing
    pub enabled: bool,
    /// Load balancing algorithm
    pub algorithm: LoadBalancingAlgorithm,
    /// Load threshold
    pub load_threshold: f64,
}

/// Load balancing algorithm
#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum LoadBalancingAlgorithm {
    /// Least loaded
    LeastLoaded,
    /// Random
    Random,
    /// Weighted round-robin
    WeightedRoundRobin,
}

impl Default for ParallelCryptoConfig {
    fn default() -> Self {
        Self {
            thread_pool: ThreadPoolConfig::default(),
            batch_processing: BatchProcessingConfig::default(),
            synchronization: SynchronizationConfig::default(),
            load_balancing: LoadBalancingConfig::default(),
        }
    }
}

impl Default for ThreadPoolConfig {
    fn default() -> Self {
        Self {
            max_threads: 4,
            min_threads: 1,
            thread_priority: ThreadPriority::Normal,
            thread_affinity: ThreadAffinityConfig::default(),
        }
    }
}

impl Default for ThreadAffinityConfig {
    fn default() -> Self {
        Self {
            enabled: false,
            strategy: AffinityStrategy::None,
        }
    }
}

impl Default for BatchProcessingConfig {
    fn default() -> Self {
        Self {
            enabled: true,
            batch_size: 100,
            strategy: BatchProcessingStrategy::Fixed,
        }
    }
}

impl Default for SynchronizationConfig {
    fn default() -> Self {
        Self {
            primitive: SynchronizationPrimitive::Mutex,
            work_distribution: WorkDistributionStrategy::RoundRobin,
        }
    }
}

impl Default for LoadBalancingConfig {
    fn default() -> Self {
        Self {
            enabled: true,
            algorithm: LoadBalancingAlgorithm::LeastLoaded,
            load_threshold: 0.8,
        }
    }
}

impl ParallelCryptoConfig {
    /// Create production parallel crypto configuration
    pub fn production() -> Self {
        Self {
            thread_pool: ThreadPoolConfig::production(),
            batch_processing: BatchProcessingConfig::production(),
            synchronization: SynchronizationConfig::production(),
            load_balancing: LoadBalancingConfig::production(),
        }
    }

    /// Create development parallel crypto configuration
    pub fn development() -> Self {
        Self {
            thread_pool: ThreadPoolConfig::development(),
            batch_processing: BatchProcessingConfig::development(),
            synchronization: SynchronizationConfig::development(),
            load_balancing: LoadBalancingConfig::development(),
        }
    }
}

impl ThreadPoolConfig {
    /// Create production thread pool configuration
    pub fn production() -> Self {
        Self {
            max_threads: 16,
            min_threads: 4,
            thread_priority: ThreadPriority::High,
            thread_affinity: ThreadAffinityConfig::production(),
        }
    }

    /// Create development thread pool configuration
    pub fn development() -> Self {
        Self {
            max_threads: 2,
            min_threads: 1,
            thread_priority: ThreadPriority::Normal,
            thread_affinity: ThreadAffinityConfig::development(),
        }
    }
}

impl ThreadAffinityConfig {
    /// Create production thread affinity configuration
    pub fn production() -> Self {
        Self {
            enabled: true,
            strategy: AffinityStrategy::NUMA,
        }
    }

    /// Create development thread affinity configuration
    pub fn development() -> Self {
        Self {
            enabled: false,
            strategy: AffinityStrategy::None,
        }
    }
}

impl BatchProcessingConfig {
    /// Create production batch processing configuration
    pub fn production() -> Self {
        Self {
            enabled: true,
            batch_size: 1000,
            strategy: BatchProcessingStrategy::Adaptive,
        }
    }

    /// Create development batch processing configuration
    pub fn development() -> Self {
        Self {
            enabled: false,
            batch_size: 10,
            strategy: BatchProcessingStrategy::Fixed,
        }
    }
}

impl SynchronizationConfig {
    /// Create production synchronization configuration
    pub fn production() -> Self {
        Self {
            primitive: SynchronizationPrimitive::LockFree,
            work_distribution: WorkDistributionStrategy::WorkStealing,
        }
    }

    /// Create development synchronization configuration
    pub fn development() -> Self {
        Self {
            primitive: SynchronizationPrimitive::Mutex,
            work_distribution: WorkDistributionStrategy::RoundRobin,
        }
    }
}

impl LoadBalancingConfig {
    /// Create production load balancing configuration
    pub fn production() -> Self {
        Self {
            enabled: true,
            algorithm: LoadBalancingAlgorithm::WeightedRoundRobin,
            load_threshold: 0.9,
        }
    }

    /// Create development load balancing configuration
    pub fn development() -> Self {
        Self {
            enabled: false,
            algorithm: LoadBalancingAlgorithm::Random,
            load_threshold: 0.5,
        }
    }
}
