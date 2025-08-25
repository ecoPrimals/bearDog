// BearDog - Enterprise Security Ecosystem
// Copyright (C) 2025 EcoPrimals
//
// This program is free software: you can redistribute it and/or modify
// it under the terms of the GNU Affero General Public License as published by
// the Free Software Foundation, either version 3 of the License, or
// (at your option) any later version.
//
// This program is distributed in the hope that it will be useful,
// but WITHOUT ANY WARRANTY; without even the implied warranty of
// MERCHANTABILITY or FITNESS FOR A PARTICULAR PURPOSE. See the
// GNU Affero General Public License for more details.
//
// You should have received a copy of the GNU Affero General Public License
// along with this program. If not, see <https://www.gnu.org/licenses/>.


/// # Performance Configuration
///
/// **CANONICAL PERFORMANCE CONFIGURATION TYPES**
/// This module contains all performance-related configuration structures,
/// extracted from the large configuration.rs file to maintain modularity.
use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use std::time::Duration;

/// **CANONICAL PERFORMANCE CONFIGURATION** - Consolidates all performance-related configs
#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct PerformanceConfig {
    /// Thread pool configuration
    pub thread_pool: ThreadPoolConfig,
    /// Memory management configuration  
    pub memory: MemoryConfig,
    /// SIMD crypto configuration
    pub simd_crypto: SimdCryptoConfig,
    /// Caching configuration
    pub caching: CacheConfig,
    /// Load balancing configuration
    pub load_balancing: LoadBalancingConfig,
}
/// **CANONICAL THREAD POOL CONFIGURATION** - Consolidates ThreadPoolConfig variants
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

/// Thread priority levels
#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum ThreadPriority {
    Low,
    Normal,
    High,
    RealTime,
}

/// **CANONICAL THREAD AFFINITY CONFIGURATION**
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ThreadAffinityConfig {
    /// Whether thread affinity is enabled
    pub enabled: bool,
    /// Affinity strategy
    pub strategy: AffinityStrategy,
}
/// **CANONICAL AFFINITY STRATEGY** - CPU affinity configuration
#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum AffinityStrategy {
    None,
    Core,
    Socket,
    Numa,
}

impl Default for AffinityStrategy {
    fn default() -> Self {
        Self::None
    }
}
/// **CANONICAL MEMORY CONFIGURATION** - Consolidates memory management settings
#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct MemoryConfig {
    /// Memory allocator configuration
    pub allocator: AllocatorConfig,
    /// Object pooling configuration
    pub object_pooling: ObjectPoolingConfig,
    /// Memory monitoring configuration
    pub monitoring: MemoryMonitoringConfig,
}

/// **CANONICAL ALLOCATOR CONFIGURATION**
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct AllocatorConfig {
    /// Allocator type
    pub allocator_type: AllocatorType,
    /// Memory arena configuration
    pub arena: ArenaConfig,
}

/// Memory allocator types
#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum AllocatorType {
    System,
    Jemalloc,
    Mimalloc,
    Custom,
}

/// **CANONICAL ARENA CONFIGURATION**
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ArenaConfig {
    /// Arena size in bytes
    pub size: usize,
    /// Number of arenas
    pub count: u32,
    /// Arena growth strategy
    pub growth_strategy: ArenaGrowthStrategy,
}

/// Arena growth strategies
#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum ArenaGrowthStrategy {
    Fixed,
    Linear,
    Exponential,
}


#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ObjectPoolingConfig {
    /// Whether object pooling is enabled
    pub enabled: bool,
    /// Pool configurations by type
    pub pools: HashMap<String, PoolConfig>,
    /// Global pool settings
    pub global_settings: GlobalPoolSettings,
}

/// **CANONICAL POOL CONFIGURATION**
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PoolConfig {
    /// Initial pool size
    pub initial_size: u32,
    /// Maximum pool size
    pub max_size: u32,
    /// Pool growth increment
    pub growth_increment: u32,
    /// Object lifetime in pool
    pub object_lifetime: Duration,
}

/// **CANONICAL GLOBAL POOL SETTINGS**
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct GlobalPoolSettings {
    /// Pool cleanup interval
    pub cleanup_interval: Duration,
    /// Health check interval
    pub health_check_interval: Duration,
}
/// **CANONICAL MEMORY MONITORING CONFIGURATION**
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct MemoryMonitoringConfig {
    /// Whether monitoring is enabled
    pub enabled: bool,
    /// Monitoring interval
    pub interval: Duration,
    /// Memory usage thresholds
    pub thresholds: MemoryThresholds,
}

/// **CANONICAL MEMORY THRESHOLDS**
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct MemoryThresholds {
    /// Warning threshold (percentage)
    pub warning: f64,
    /// Critical threshold (percentage)
    pub critical: f64,
    /// Maximum heap size
    pub max_heap_size: Option<usize>,
}

/// **CANONICAL CORE MONITORING CONFIGURATION**
pub struct CoreMonitoringConfig {
    /// Enable core monitoring
    pub enabled: bool,
    /// Collection of metrics to monitor
    pub metrics: Vec<String>,
    /// Alert thresholds
    pub thresholds: std::collections::HashMap<String, f64>,
}

/// **CANONICAL SIMD CRYPTO CONFIGURATION** - SIMD optimization settings
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SimdCryptoConfig {
    /// Whether SIMD is enabled
    pub enabled: bool,
    /// Vector processing configuration
    pub vector_processing: VectorProcessingConfig,
    /// Parallel crypto configuration
    pub parallel_crypto: ParallelCryptoConfig,
}

/// **CANONICAL VECTOR PROCESSING CONFIGURATION**
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct VectorProcessingConfig {
    /// Vector size
    pub vector_size: u32,
    /// Instruction set configuration
    pub instruction_sets: InstructionSetConfig,
    /// Performance benchmarking
    pub benchmarking: CryptoBenchmarkingConfig,
}

/// **CANONICAL INSTRUCTION SET CONFIGURATION**
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct InstructionSetConfig {
    /// Available instruction sets
    pub available: Vec<InstructionSet>,
    /// Preferred instruction set
    pub preferred: Option<InstructionSet>,
    /// Fallback instruction set
    pub fallback: InstructionSet,
}
/// Instruction set types
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub enum InstructionSet {
    SSE2,
    SSE3,
    SSE4,
    Avx,
    Avx2,
    Avx512,
    Neon,
    Generic,
}

impl Default for InstructionSet {
    fn default() -> Self {
        Self::Generic
    }
}

/// **CANONICAL CRYPTO BENCHMARKING CONFIGURATION**
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CryptoBenchmarkingConfig {
    /// Whether benchmarking is enabled
    pub enabled: bool,
    /// Benchmark interval
    pub interval: Duration,
    /// Benchmark iterations
    pub iterations: u32,
}

/// **CANONICAL PARALLEL CRYPTO CONFIGURATION**
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ParallelCryptoConfig {
    /// Thread pool configuration (inherits from canonical)
    pub thread_pool_size: u32,
    /// Batch processing configuration
    pub batch_processing: BatchProcessingConfig,
    /// Synchronization configuration
    pub synchronization: SynchronizationConfig,
    /// Load balancing configuration (inherits from canonical)
    pub load_balancing: bool,
}
/// **CANONICAL BATCH PROCESSING CONFIGURATION**
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct BatchProcessingConfig {
    /// Batch size
    pub batch_size: u32,
    /// Processing strategy
    pub strategy: BatchProcessingStrategy,
    /// Queue size
    pub queue_size: u32,
    /// Processing timeout
    pub timeout: Duration,
}

/// Batch processing strategies
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub enum BatchProcessingStrategy {
    Dynamic,
    Adaptive,
    Fixed,
}

/// **CANONICAL SYNCHRONIZATION CONFIGURATION**
#[derive(Debug, Clone, Serialize, Deserialize)]
#[derive(Default)]
pub struct SynchronizationConfig {
    /// Synchronization primitive
    pub primitive: SynchronizationPrimitive,
    /// Work distribution strategy
    pub work_distribution: WorkDistributionStrategy,
}

/// Synchronization primitives
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub enum SynchronizationPrimitive {
    Mutex,
    RwLock,
    Atomic,
    Channel,
}

/// Work distribution strategies
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub enum WorkDistributionStrategy {
    RoundRobin,
    Random,
    LeastLoaded,
    WorkStealing,
}

/// **CANONICAL CACHE CONFIGURATION** - Caching system settings
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CacheConfig {
    /// Cache type
    pub cache_type: CacheType,
    /// Cache size limits
    pub size_limits: CacheSizeLimits,
    /// Cache performance settings
    pub performance: CachePerformanceConfig,
}

/// Cache types
#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum CacheType {
    Memory,
    Redis,
    Memcached,
    Hybrid,
}

impl Default for CacheType {
    fn default() -> Self {
        Self::Memory
    }
}

/// **CANONICAL CACHE SIZE LIMITS**
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CacheSizeLimits {
    /// Maximum memory usage in bytes
    pub max_memory_bytes: u64,
    /// Maximum number of entries
    pub max_entries: u64,
}

/// **CANONICAL CACHE PERFORMANCE CONFIGURATION**
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CachePerformanceConfig {
    /// Maximum cache size
    pub max_size: u64,
    /// Eviction policy
    pub eviction_policy: EvictionPolicy,
}

/// Cache eviction policies
#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum EvictionPolicy {
    Lru,
    Lfu,
    Fifo,
    Random,
}

impl Default for EvictionPolicy {
    fn default() -> Self {
        Self::Lru
    }
}

/// **CANONICAL LOAD BALANCING CONFIGURATION** - Load balancing settings
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct LoadBalancingConfig {
    /// Load balancing algorithm
    pub algorithm: LoadBalancingAlgorithm,
    /// Health check configuration
    pub health_check: HealthCheckConfig,
    /// Load threshold
    pub load_threshold: f64,
}

/// Load balancing algorithms
#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum LoadBalancingAlgorithm {
    LeastLoaded,
    WeightedRoundRobin,
}

impl Default for LoadBalancingAlgorithm {
    fn default() -> Self {
        Self::LeastLoaded
    }
}

/// **CANONICAL HEALTH CHECK CONFIGURATION**
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct HealthCheckConfig {
    /// Health check timeout in seconds
    pub timeout_secs: u32,
    /// Failure threshold
    pub failure_threshold: u32,
    /// Recovery threshold
    pub recovery_threshold: u32,
    /// Maximum retry attempts (for backward compatibility)
    pub retries: u32,
}

/// **CANONICAL FAILOVER CONFIGURATION** - Failover and redundancy settings
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct FailoverConfig {
    /// Whether failover is enabled
    pub enabled: bool,
    /// Failover strategy
    pub strategy: FailoverStrategy,
    /// Maximum failover attempts
    pub max_attempts: u32,
    /// Failover timeout
    pub timeout: Duration,
}

/// Failover strategies
#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum FailoverStrategy {
    Immediate,
    Gradual,
    CircuitBreaker,
}

impl Default for FailoverStrategy {
    fn default() -> Self {
        Self::Immediate
    }
}

// ============================================================================
// DEFAULT IMPLEMENTATIONS

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

impl Default for AllocatorConfig {
    fn default() -> Self {
        Self {
            allocator_type: AllocatorType::System,
            arena: ArenaConfig::default(),
        }
    }
}

impl Default for ArenaConfig {
    fn default() -> Self {
        Self {
            size: 1024 * 1024, // 1MB
            count: 4,
            growth_strategy: ArenaGrowthStrategy::Linear,
        }
    }
}


impl Default for ObjectPoolingConfig {
    fn default() -> Self {
        Self {
            enabled: true,
            pools: HashMap::new(),
            global_settings: GlobalPoolSettings::default(),
        }
    }
}

impl Default for GlobalPoolSettings {
    fn default() -> Self {
        Self {
            cleanup_interval: Duration::from_secs(60),
            health_check_interval: Duration::from_secs(30),
        }
    }
}


impl Default for MemoryMonitoringConfig {
    fn default() -> Self {
        Self {
            enabled: true,
            interval: Duration::from_secs(10),
            thresholds: MemoryThresholds::default(),
        }
    }
}

impl Default for MemoryThresholds {
    fn default() -> Self {
        Self {
            warning: 75.0,
            critical: 90.0,
            max_heap_size: None,
        }
    }
}


impl Default for SimdCryptoConfig {
    fn default() -> Self {
        Self {
            enabled: true,
            vector_processing: VectorProcessingConfig::default(),
            parallel_crypto: ParallelCryptoConfig::default(),
        }
    }
}

impl Default for VectorProcessingConfig {
    fn default() -> Self {
        Self {
            vector_size: 32,
            instruction_sets: InstructionSetConfig::default(),
            benchmarking: CryptoBenchmarkingConfig::default(),
        }
    }
}


impl Default for InstructionSetConfig {
    fn default() -> Self {
        Self {
            available: vec![InstructionSet::Generic],
            preferred: None,
            fallback: InstructionSet::Generic,
        }
    }
}

impl Default for CryptoBenchmarkingConfig {
    fn default() -> Self {
        Self {
            enabled: false,
            interval: Duration::from_secs(60),
            iterations: 100,
        }
    }
}

impl Default for ParallelCryptoConfig {
    fn default() -> Self {
        Self {
            thread_pool_size: 4,
            batch_processing: BatchProcessingConfig::default(),
            synchronization: SynchronizationConfig::default(),
            load_balancing: true,
        }
    }
}


impl Default for BatchProcessingConfig {
    fn default() -> Self {
        Self {
            batch_size: 1000,
            strategy: BatchProcessingStrategy::Fixed,
            queue_size: 10000,
            timeout: Duration::from_secs(30),
        }
    }
}



impl Default for CacheConfig {
    fn default() -> Self {
        Self {
            cache_type: CacheType::Memory,
            size_limits: CacheSizeLimits::default(),
            performance: CachePerformanceConfig::default(),
        }
    }
}

impl Default for CacheSizeLimits {
    fn default() -> Self {
        Self {
            max_memory_bytes: 1024 * 1024 * 100, // 100MB
            max_entries: 10000,
        }
    }
}


impl Default for CachePerformanceConfig {
    fn default() -> Self {
        Self {
            max_size: 1024 * 1024 * 100,    // 100MB
            eviction_policy: EvictionPolicy::Lru,
        }
    }
}

impl Default for LoadBalancingConfig {
    fn default() -> Self {
        Self {
            algorithm: LoadBalancingAlgorithm::LeastLoaded,
            health_check: HealthCheckConfig::default(),
            load_threshold: 0.8,
        }
    }
}


impl Default for HealthCheckConfig {
    fn default() -> Self {
        Self {
            timeout_secs: 5,
            failure_threshold: 3,
            recovery_threshold: 2,
            retries: 3,
        }
    }
}

impl Default for SynchronizationPrimitive {
    fn default() -> Self {
        Self::Mutex
    }
}

impl Default for WorkDistributionStrategy {
    fn default() -> Self {
        Self::RoundRobin
    }
}

impl Default for BatchProcessingStrategy {
    fn default() -> Self {
        Self::Dynamic
    }
}
