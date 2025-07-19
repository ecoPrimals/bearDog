use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use std::time::Duration;

/// Memory optimization configuration
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct MemoryOptimizationConfig {
    /// Object pooling configuration
    pub object_pooling: ObjectPoolingConfig,
    /// Memory monitoring configuration
    pub monitoring: MemoryMonitoringConfig,
    /// Garbage collection tuning
    pub gc_tuning: GcTuningConfig,
    /// Memory allocation strategies
    pub allocation_strategies: AllocationStrategiesConfig,
    /// Cache configuration
    pub cache: MemoryCacheConfig,
}

/// Object pooling configuration
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ObjectPoolingConfig {
    /// Enable object pooling
    pub enabled: bool,
    /// Pool configurations for different object types
    pub pools: HashMap<String, PoolConfig>,
    /// Global pool settings
    pub global_settings: GlobalPoolSettings,
}

/// Configuration for individual object pools
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PoolConfig {
    /// Initial pool size
    pub initial_size: u32,
    /// Maximum pool size
    pub max_size: u32,
    /// Minimum pool size
    pub min_size: u32,
    /// Pool growth increment
    pub growth_increment: u32,
    /// Pool shrink threshold
    pub shrink_threshold: f64,
    /// Object lifetime in pool
    pub object_lifetime: Duration,
    /// Enable pool monitoring
    pub enable_monitoring: bool,
}

/// Global pool settings
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct GlobalPoolSettings {
    /// Pool cleanup interval
    pub cleanup_interval: Duration,
    /// Pool health check interval
    pub health_check_interval: Duration,
    /// Enable pool statistics
    pub enable_statistics: bool,
    /// Pool statistics collection interval
    pub statistics_interval: Duration,
}

/// Memory monitoring configuration
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct MemoryMonitoringConfig {
    /// Enable memory monitoring
    pub enabled: bool,
    /// Memory usage threshold for alerts
    pub alert_threshold_percent: f64,
    /// Critical memory threshold
    pub critical_threshold_percent: f64,
    /// Memory check interval
    pub check_interval: Duration,
    /// Enable memory profiling
    pub enable_profiling: bool,
    /// Memory leak detection
    pub leak_detection: LeakDetectionConfig,
}

/// Memory leak detection configuration
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct LeakDetectionConfig {
    /// Enable leak detection
    pub enabled: bool,
    /// Leak detection interval
    pub detection_interval: Duration,
    /// Memory growth threshold for leak detection
    pub growth_threshold_percent: f64,
    /// Number of checks before confirming leak
    pub confirmation_checks: u32,
}

/// Garbage collection tuning configuration
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct GcTuningConfig {
    /// Enable GC tuning
    pub enabled: bool,
    /// GC trigger threshold
    pub trigger_threshold_percent: f64,
    /// GC target utilization
    pub target_utilization_percent: f64,
    /// GC pause target
    pub pause_target_ms: u64,
    /// Enable concurrent GC
    pub enable_concurrent_gc: bool,
}

/// Memory allocation strategies
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct AllocationStrategiesConfig {
    /// Enable copy-on-write optimization
    pub enable_cow: bool,
    /// Enable arena allocation
    pub enable_arena: bool,
    /// Enable bump allocation
    pub enable_bump: bool,
    /// Memory alignment
    pub memory_alignment: u32,
    /// Pre-allocation strategies
    pub pre_allocation: PreAllocationConfig,
}

/// Pre-allocation configuration
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PreAllocationConfig {
    /// Enable pre-allocation
    pub enabled: bool,
    /// Pre-allocation sizes for different object types
    pub sizes: HashMap<String, u32>,
    /// Pre-allocation growth rate
    pub growth_rate: f64,
}

/// Memory cache configuration
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct MemoryCacheConfig {
    /// L1 cache configuration (CPU cache-friendly)
    pub l1_cache: L1CacheConfig,
    /// L2 cache configuration (larger, slower)
    pub l2_cache: L2CacheConfig,
    /// Cache warming configuration
    pub cache_warming: CacheWarmingConfig,
}

/// L1 cache configuration
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct L1CacheConfig {
    /// Enable L1 cache
    pub enabled: bool,
    /// Cache size in KB
    pub size_kb: u32,
    /// Cache line size
    pub line_size: u32,
    /// Cache associativity
    pub associativity: u32,
    /// Cache replacement policy
    pub replacement_policy: CacheReplacementPolicy,
}

/// L2 cache configuration
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct L2CacheConfig {
    /// Enable L2 cache
    pub enabled: bool,
    /// Cache size in MB
    pub size_mb: u32,
    /// Cache line size
    pub line_size: u32,
    /// Cache associativity
    pub associativity: u32,
    /// Cache replacement policy
    pub replacement_policy: CacheReplacementPolicy,
}

/// Cache replacement policy
#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum CacheReplacementPolicy {
    /// Least Recently Used
    Lru,
    /// Least Frequently Used
    Lfu,
    /// First In, First Out
    Fifo,
    /// Random replacement
    Random,
    /// Adaptive replacement cache
    Arc,
}

/// Cache warming configuration
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CacheWarmingConfig {
    /// Enable cache warming
    pub enabled: bool,
    /// Warming strategies
    pub strategies: Vec<WarmingStrategy>,
    /// Warming interval
    pub warming_interval: Duration,
    /// Warming batch size
    pub batch_size: u32,
}

/// Cache warming strategy
#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum WarmingStrategy {
    /// Warm most frequently accessed objects
    MostFrequent,
    /// Warm recently accessed objects
    MostRecent,
    /// Warm based on predicted access patterns
    Predictive,
    /// Warm critical objects
    Critical,
}

impl Default for ObjectPoolingConfig {
    fn default() -> Self {
        let mut pools = HashMap::new();

        // Configure pools for common BearDog objects
        pools.insert(
            "BearDogGenetics".to_string(),
            PoolConfig {
                initial_size: 50,
                max_size: 500,
                min_size: 10,
                growth_increment: 25,
                shrink_threshold: 0.3,
                object_lifetime: Duration::from_secs(300),
                enable_monitoring: true,
            },
        );

        pools.insert(
            "EncryptedData".to_string(),
            PoolConfig {
                initial_size: 100,
                max_size: 1000,
                min_size: 20,
                growth_increment: 50,
                shrink_threshold: 0.2,
                object_lifetime: Duration::from_secs(60),
                enable_monitoring: true,
            },
        );

        pools.insert(
            "CryptoKey".to_string(),
            PoolConfig {
                initial_size: 25,
                max_size: 250,
                min_size: 5,
                growth_increment: 12,
                shrink_threshold: 0.4,
                object_lifetime: Duration::from_secs(600),
                enable_monitoring: true,
            },
        );

        pools.insert(
            "SecurityContext".to_string(),
            PoolConfig {
                initial_size: 30,
                max_size: 300,
                min_size: 10,
                growth_increment: 15,
                shrink_threshold: 0.3,
                object_lifetime: Duration::from_secs(180),
                enable_monitoring: true,
            },
        );

        Self {
            enabled: true,
            pools,
            global_settings: GlobalPoolSettings::default(),
        }
    }
}

impl Default for GlobalPoolSettings {
    fn default() -> Self {
        Self {
            cleanup_interval: Duration::from_secs(30),
            health_check_interval: Duration::from_secs(60),
            enable_statistics: true,
            statistics_interval: Duration::from_secs(10),
        }
    }
}

impl Default for MemoryMonitoringConfig {
    fn default() -> Self {
        Self {
            enabled: true,
            alert_threshold_percent: 80.0,
            critical_threshold_percent: 95.0,
            check_interval: Duration::from_secs(10),
            enable_profiling: true,
            leak_detection: LeakDetectionConfig::default(),
        }
    }
}

impl Default for LeakDetectionConfig {
    fn default() -> Self {
        Self {
            enabled: true,
            detection_interval: Duration::from_secs(300),
            growth_threshold_percent: 10.0,
            confirmation_checks: 3,
        }
    }
}

impl Default for GcTuningConfig {
    fn default() -> Self {
        Self {
            enabled: true,
            trigger_threshold_percent: 70.0,
            target_utilization_percent: 60.0,
            pause_target_ms: 10,
            enable_concurrent_gc: true,
        }
    }
}

impl Default for AllocationStrategiesConfig {
    fn default() -> Self {
        let mut sizes = HashMap::new();
        sizes.insert("Vector".to_string(), 256);
        sizes.insert("HashMap".to_string(), 64);
        sizes.insert("String".to_string(), 128);
        sizes.insert("Buffer".to_string(), 4096);

        Self {
            enable_cow: true,
            enable_arena: true,
            enable_bump: true,
            memory_alignment: 64,
            pre_allocation: PreAllocationConfig {
                enabled: true,
                sizes,
                growth_rate: 1.5,
            },
        }
    }
}

impl Default for L1CacheConfig {
    fn default() -> Self {
        Self {
            enabled: true,
            size_kb: 64,
            line_size: 64,
            associativity: 8,
            replacement_policy: CacheReplacementPolicy::Lru,
        }
    }
}

impl Default for L2CacheConfig {
    fn default() -> Self {
        Self {
            enabled: true,
            size_mb: 8,
            line_size: 64,
            associativity: 16,
            replacement_policy: CacheReplacementPolicy::Arc,
        }
    }
}

impl Default for CacheWarmingConfig {
    fn default() -> Self {
        Self {
            enabled: true,
            strategies: vec![WarmingStrategy::MostFrequent, WarmingStrategy::Critical],
            warming_interval: Duration::from_secs(30),
            batch_size: 100,
        }
    }
}

impl Default for MemoryOptimizationConfig {
    fn default() -> Self {
        Self {
            object_pooling: ObjectPoolingConfig::default(),
            monitoring: MemoryMonitoringConfig::default(),
            gc_tuning: GcTuningConfig::default(),
            allocation_strategies: AllocationStrategiesConfig::default(),
            cache: MemoryCacheConfig::default(),
        }
    }
}

impl Default for MemoryCacheConfig {
    fn default() -> Self {
        Self {
            l1_cache: L1CacheConfig::default(),
            l2_cache: L2CacheConfig::default(),
            cache_warming: CacheWarmingConfig::default(),
        }
    }
}

impl MemoryOptimizationConfig {
    /// Create production-optimized memory configuration
    pub fn production() -> Self {
        let mut config = Self::default();

        // Aggressive object pooling
        config.object_pooling.enabled = true;
        for pool in config.object_pooling.pools.values_mut() {
            pool.max_size *= 2;
            pool.initial_size *= 2;
            pool.growth_increment *= 2;
        }

        // Comprehensive monitoring
        config.monitoring.enabled = true;
        config.monitoring.alert_threshold_percent = 85.0;
        config.monitoring.critical_threshold_percent = 95.0;
        config.monitoring.check_interval = Duration::from_secs(5);
        config.monitoring.enable_profiling = true;
        config.monitoring.leak_detection.enabled = true;

        // Aggressive GC tuning
        config.gc_tuning.enabled = true;
        config.gc_tuning.trigger_threshold_percent = 60.0;
        config.gc_tuning.target_utilization_percent = 50.0;
        config.gc_tuning.pause_target_ms = 5;
        config.gc_tuning.enable_concurrent_gc = true;

        // All allocation strategies
        config.allocation_strategies.enable_cow = true;
        config.allocation_strategies.enable_arena = true;
        config.allocation_strategies.enable_bump = true;
        config.allocation_strategies.memory_alignment = 64;
        config.allocation_strategies.pre_allocation.enabled = true;

        // Larger caches
        config.cache.l1_cache.size_kb = 128;
        config.cache.l2_cache.size_mb = 16;
        config.cache.cache_warming.enabled = true;

        config
    }

    /// Create development-optimized memory configuration
    pub fn development() -> Self {
        let mut config = Self::default();

        // Moderate object pooling
        config.object_pooling.enabled = true;

        // Detailed monitoring for debugging
        config.monitoring.enabled = true;
        config.monitoring.alert_threshold_percent = 70.0;
        config.monitoring.check_interval = Duration::from_secs(5);
        config.monitoring.enable_profiling = true;
        config.monitoring.leak_detection.enabled = true;
        config.monitoring.leak_detection.detection_interval = Duration::from_secs(60);

        // Conservative GC tuning
        config.gc_tuning.enabled = true;
        config.gc_tuning.trigger_threshold_percent = 75.0;
        config.gc_tuning.pause_target_ms = 20;

        // Smaller caches for development
        config.cache.l1_cache.size_kb = 32;
        config.cache.l2_cache.size_mb = 4;
        config.cache.cache_warming.enabled = false;

        config
    }

    /// Validate configuration
    pub fn validate(&self) -> Result<(), String> {
        if self.monitoring.alert_threshold_percent >= self.monitoring.critical_threshold_percent {
            return Err(
                "alert_threshold_percent must be less than critical_threshold_percent".to_string(),
            );
        }

        if self.gc_tuning.trigger_threshold_percent >= self.gc_tuning.target_utilization_percent {
            return Err(
                "trigger_threshold_percent must be less than target_utilization_percent"
                    .to_string(),
            );
        }

        for (name, pool) in &self.object_pooling.pools {
            if pool.min_size > pool.max_size {
                return Err(format!("Pool '{name}': min_size must be <= max_size"));
            }
            if pool.initial_size > pool.max_size {
                return Err(format!("Pool '{name}': initial_size must be <= max_size"));
            }
        }

        Ok(())
    }
}

/// Memory optimization utilities
pub mod utils {
    /// Calculate optimal pool size based on usage patterns
    pub fn calculate_optimal_pool_size(
        current_size: u32,
        usage_rate: f64,
        growth_rate: f64,
    ) -> u32 {
        let target_size = (current_size as f64 * usage_rate * growth_rate) as u32;
        target_size.clamp(1, 10000) // Reasonable bounds
    }

    /// Estimate memory usage for object type
    pub fn estimate_object_memory_usage(object_type: &str) -> u32 {
        match object_type {
            "BearDogGenetics" => 2048, // ~2KB per genetics object
            "EncryptedData" => 1024,   // ~1KB per encrypted data
            "CryptoKey" => 512,        // ~512B per key
            "SecurityContext" => 256,  // ~256B per context
            _ => 128,                  // Default estimate
        }
    }

    /// Calculate memory fragmentation ratio
    pub fn calculate_fragmentation_ratio(allocated: u64, used: u64) -> f64 {
        if allocated == 0 {
            return 0.0;
        }
        1.0 - (used as f64 / allocated as f64)
    }
}
