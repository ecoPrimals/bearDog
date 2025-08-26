

use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use std::time::Duration;

#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct PerformanceConfig {

    pub thread_pool: ThreadPoolConfig,

    pub memory: MemoryConfig,

    pub simd_crypto: SimdCryptoConfig,

    pub caching: CacheConfig,

    pub load_balancing: LoadBalancingConfig,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ThreadPoolConfig {

    pub max_threads: u32,

    pub min_threads: u32,

    pub thread_priority: ThreadPriority,

    pub thread_affinity: ThreadAffinityConfig,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum ThreadPriority {
    Low,
    Normal,
    High,
    RealTime,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ThreadAffinityConfig {

    pub enabled: bool,

    pub strategy: AffinityStrategy,
}

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

#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct MemoryConfig {

    pub allocator: AllocatorConfig,

    pub object_pooling: ObjectPoolingConfig,

    pub monitoring: MemoryMonitoringConfig,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct AllocatorConfig {

    pub allocator_type: AllocatorType,

    pub arena: ArenaConfig,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum AllocatorType {
    System,
    Jemalloc,
    Mimalloc,
    Custom,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ArenaConfig {

    pub size: usize,

    pub count: u32,

    pub growth_strategy: ArenaGrowthStrategy,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum ArenaGrowthStrategy {
    Fixed,
    Linear,
    Exponential,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ObjectPoolingConfig {

    pub enabled: bool,

    pub pools: HashMap<String, PoolConfig>,

    pub global_settings: GlobalPoolSettings,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PoolConfig {

    pub initial_size: u32,

    pub max_size: u32,

    pub growth_increment: u32,

    pub object_lifetime: Duration,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct GlobalPoolSettings {

    pub cleanup_interval: Duration,

    pub health_check_interval: Duration,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct MemoryMonitoringConfig {

    pub enabled: bool,

    pub interval: Duration,

    pub thresholds: MemoryThresholds,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct MemoryThresholds {

    pub warning: f64,

    pub critical: f64,

    pub max_heap_size: Option<usize>,
}

pub struct CoreMonitoringConfig {

    pub enabled: bool,

    pub metrics: Vec<String>,

    pub thresholds: std::collections::HashMap<String, f64>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SimdCryptoConfig {

    pub enabled: bool,

    pub vector_processing: VectorProcessingConfig,

    pub parallel_crypto: ParallelCryptoConfig,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct VectorProcessingConfig {

    pub vector_size: u32,

    pub instruction_sets: InstructionSetConfig,

    pub benchmarking: CryptoBenchmarkingConfig,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct InstructionSetConfig {

    pub available: Vec<InstructionSet>,

    pub preferred: Option<InstructionSet>,

    pub fallback: InstructionSet,
}

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

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CryptoBenchmarkingConfig {

    pub enabled: bool,

    pub interval: Duration,

    pub iterations: u32,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ParallelCryptoConfig {

    pub thread_pool_size: u32,

    pub batch_processing: BatchProcessingConfig,

    pub synchronization: SynchronizationConfig,

    pub load_balancing: bool,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct BatchProcessingConfig {

    pub batch_size: u32,

    pub strategy: BatchProcessingStrategy,

    pub queue_size: u32,

    pub timeout: Duration,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub enum BatchProcessingStrategy {
    Dynamic,
    Adaptive,
    Fixed,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[derive(Default)]
pub struct SynchronizationConfig {

    pub primitive: SynchronizationPrimitive,

    pub work_distribution: WorkDistributionStrategy,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub enum SynchronizationPrimitive {
    Mutex,
    RwLock,
    Atomic,
    Channel,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub enum WorkDistributionStrategy {
    RoundRobin,
    Random,
    LeastLoaded,
    WorkStealing,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CacheConfig {

    pub cache_type: CacheType,

    pub size_limits: CacheSizeLimits,

    pub performance: CachePerformanceConfig,
}

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

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CacheSizeLimits {

    pub max_memory_bytes: u64,

    pub max_entries: u64,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CachePerformanceConfig {

    pub max_size: u64,

    pub eviction_policy: EvictionPolicy,
}

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

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct LoadBalancingConfig {

    pub algorithm: LoadBalancingAlgorithm,

    pub health_check: HealthCheckConfig,

    pub load_threshold: f64,
}

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

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct HealthCheckConfig {

    pub timeout_secs: u32,

    pub failure_threshold: u32,

    pub recovery_threshold: u32,

    pub retries: u32,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct FailoverConfig {

    pub enabled: bool,

    pub strategy: FailoverStrategy,

    pub max_attempts: u32,

    pub timeout: Duration,
}

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
            pools: HashMap::with_capacity(16),
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
