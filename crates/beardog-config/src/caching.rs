use serde::{Deserialize, Serialize};
use std::time::Duration;

/// Distributed caching configuration
#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct DistributedCachingConfig {
    /// Multi-tier caching configuration
    pub multi_tier: MultiTierCachingConfig,
    /// Redis cluster configuration
    pub redis_cluster: RedisClusterConfig,
    /// Cache warming configuration
    pub cache_warming: CacheWarmingConfig,
    /// Cache synchronization configuration
    pub synchronization: CacheSynchronizationConfig,
    /// Performance optimization configuration
    pub performance: CachePerformanceConfig,
}

/// Multi-tier caching configuration
#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct MultiTierCachingConfig {
    /// L1 cache (in-memory, fastest)
    pub l1_cache: L1CacheConfig,
    /// L2 cache (Redis, fast)
    pub l2_cache: L2CacheConfig,
    /// L3 cache (persistent, slower)
    pub l3_cache: L3CacheConfig,
    /// Cache promotion strategy
    pub promotion_strategy: CachePromotionStrategy,
    /// Cache eviction coordination
    pub eviction_coordination: EvictionCoordinationConfig,
}

/// L1 cache configuration (in-memory)
#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct L1CacheConfig {
    /// Enable L1 cache
    pub enabled: bool,
    /// Cache size in MB
    pub size_mb: u32,
    /// Maximum entries
    pub max_entries: u32,
    /// Entry TTL
    pub entry_ttl: Duration,
    /// Eviction policy
    pub eviction_policy: EvictionPolicy,
    /// Enable SIMD optimization
    pub enable_simd: bool,
}

/// L2 cache configuration (Redis)
#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct L2CacheConfig {
    /// Enable L2 cache
    pub enabled: bool,
    /// Redis configuration
    pub redis: RedisConfig,
    /// Cache size in MB
    pub size_mb: u32,
    /// Entry TTL
    pub entry_ttl: Duration,
    /// Compression configuration
    pub compression: CompressionConfig,
    /// Partitioning strategy
    pub partitioning: PartitioningStrategy,
}

/// L3 cache configuration (persistent)
#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct L3CacheConfig {
    /// Enable L3 cache
    pub enabled: bool,
    /// Storage backend
    pub storage_backend: StorageBackend,
    /// Cache size in GB
    pub size_gb: u32,
    /// Entry TTL
    pub entry_ttl: Duration,
    /// Compression enabled
    pub enable_compression: bool,
    /// Encryption enabled
    pub enable_encryption: bool,
}

/// Redis cluster configuration
#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct RedisClusterConfig {
    /// Enable Redis cluster
    pub enabled: bool,
    /// Cluster nodes
    pub nodes: Vec<RedisNodeConfig>,
    /// Cluster configuration
    pub cluster_config: ClusterConfig,
    /// Failover configuration
    pub failover: FailoverConfig,
    /// Monitoring configuration
    pub monitoring: RedisMonitoringConfig,
}

/// Redis node configuration
#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct RedisNodeConfig {
    /// Node host
    pub host: String,
    /// Node port
    pub port: u16,
    /// Node role
    pub role: RedisNodeRole,
    /// Node priority
    pub priority: u8,
    /// Node region
    pub region: Option<String>,
    /// Node availability zone
    pub availability_zone: Option<String>,
}

/// Redis node role
#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub enum RedisNodeRole {
    /// Master node
    #[default]
    Master,
    /// Replica node
    Replica,
    /// Sentinel node
    Sentinel,
}

/// Redis cluster configuration
#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct ClusterConfig {
    /// Number of hash slots
    pub hash_slots: u16,
    /// Replication factor
    pub replication_factor: u8,
    /// Cluster timeout
    pub cluster_timeout: Duration,
    /// Enable cluster mode
    pub enable_cluster_mode: bool,
    /// Cluster announcement
    pub cluster_announce: bool,
}

/// Redis failover configuration
#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct FailoverConfig {
    /// Enable automatic failover
    pub enabled: bool,
    /// Failover timeout
    pub timeout: Duration,
    /// Retry attempts
    pub retry_attempts: u32,
    /// Retry delay
    pub retry_delay: Duration,
    /// Health check interval
    pub health_check_interval: Duration,
}

/// Redis monitoring configuration
#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct RedisMonitoringConfig {
    /// Enable monitoring
    pub enabled: bool,
    /// Metrics collection interval
    pub metrics_interval: Duration,
    /// Enable slow query logging
    pub enable_slow_query_log: bool,
    /// Slow query threshold
    pub slow_query_threshold: Duration,
    /// Enable memory monitoring
    pub enable_memory_monitoring: bool,
}

/// Cache warming configuration
#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct CacheWarmingConfig {
    /// Enable cache warming
    pub enabled: bool,
    /// Warming strategies
    pub strategies: Vec<WarmingStrategy>,
    /// Warming schedule
    pub schedule: WarmingSchedule,
    /// Predictive warming
    pub predictive_warming: PredictiveWarmingConfig,
    /// Performance optimization
    pub performance: WarmingPerformanceConfig,
}

/// Cache warming strategy
#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub enum WarmingStrategy {
    /// No warming strategy
    #[default]
    None,
    /// Warm most frequently accessed items
    MostFrequent {
        /// Number of items to warm
        item_count: u32,
        /// Frequency threshold
        frequency_threshold: f64,
    },
    /// Warm recently accessed items
    MostRecent {
        /// Time window for recent items
        time_window: Duration,
        /// Number of items to warm
        item_count: u32,
    },
    /// Warm based on access patterns
    PatternBased {
        /// Pattern analysis window
        analysis_window: Duration,
        /// Prediction confidence threshold
        confidence_threshold: f64,
    },
    /// Warm critical items
    Critical {
        /// Critical item patterns
        patterns: Vec<String>,
        /// Priority level
        priority: u8,
    },
    /// Warm based on business rules
    BusinessRules {
        /// Rule definitions
        rules: Vec<WarmingRule>,
    },
}

/// Cache warming rule
#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct WarmingRule {
    /// Rule name
    pub name: String,
    /// Rule condition
    pub condition: String,
    /// Items to warm
    pub items: Vec<String>,
    /// Rule priority
    pub priority: u8,
    /// Rule enabled
    pub enabled: bool,
}

/// Cache warming schedule
#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct WarmingSchedule {
    /// Enable scheduled warming
    pub enabled: bool,
    /// Warming interval
    pub interval: Duration,
    /// Warming time windows
    pub time_windows: Vec<TimeWindow>,
    /// Warming batch size
    pub batch_size: u32,
    /// Warming concurrency
    pub concurrency: u32,
}

/// Time window for cache warming
#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct TimeWindow {
    /// Start time (24-hour format)
    pub start_time: String,
    /// End time (24-hour format)
    pub end_time: String,
    /// Days of week (0=Sunday, 6=Saturday)
    pub days_of_week: Vec<u8>,
    /// Time zone
    pub timezone: String,
}

/// Predictive cache warming configuration
#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct PredictiveWarmingConfig {
    /// Enable predictive warming
    pub enabled: bool,
    /// Machine learning model configuration
    pub ml_model: MLModelConfig,
    /// Prediction horizon
    pub prediction_horizon: Duration,
    /// Confidence threshold
    pub confidence_threshold: f64,
    /// Feature engineering
    pub feature_engineering: FeatureEngineeringConfig,
}

/// Machine learning model configuration
#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct MLModelConfig {
    /// Model type
    pub model_type: MLModelType,
    /// Model path
    pub model_path: String,
    /// Model update interval
    pub update_interval: Duration,
    /// Training data retention
    pub training_data_retention: Duration,
    /// Model validation
    pub validation: ModelValidationConfig,
}

/// Machine learning model type
#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub enum MLModelType {
    /// Linear regression
    #[default]
    LinearRegression,
    /// Decision tree
    DecisionTree,
    /// Random forest
    RandomForest,
    /// Neural network
    NeuralNetwork,
    /// Time series forecasting
    TimeSeriesForecasting,
    /// Custom model
    Custom { algorithm: String },
}

/// Model validation configuration
#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct ModelValidationConfig {
    /// Validation method
    pub method: ValidationMethod,
    /// Validation threshold
    pub threshold: f64,
    /// Validation interval
    pub interval: Duration,
    /// Enable A/B testing
    pub enable_ab_testing: bool,
}

/// Validation method
#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub enum ValidationMethod {
    /// No validation
    #[default]
    None,
    /// Cross-validation
    CrossValidation { folds: u32 },
    /// Hold-out validation
    HoldOut { test_ratio: f64 },
    /// Time series validation
    TimeSeries { window_size: Duration },
}

/// Feature engineering configuration
#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct FeatureEngineeringConfig {
    /// Enable feature engineering
    pub enabled: bool,
    /// Feature extraction methods
    pub extraction_methods: Vec<FeatureExtractionMethod>,
    /// Feature selection
    pub selection: FeatureSelectionConfig,
    /// Feature normalization
    pub normalization: FeatureNormalizationConfig,
}

/// Feature extraction method
#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub enum FeatureExtractionMethod {
    /// Statistical features
    #[default]
    Statistical,
    /// Frequency-based features
    FrequencyBased,
    /// Pattern-based features
    PatternBased,
    /// User behavior features
    UserBehavior,
    /// System metrics features
    SystemMetrics,
}

/// Feature selection configuration
#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct FeatureSelectionConfig {
    /// Selection method
    pub method: FeatureSelectionMethod,
    /// Number of features to select
    pub feature_count: u32,
    /// Selection threshold
    pub threshold: f64,
}

/// Feature selection method
#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub enum FeatureSelectionMethod {
    /// Correlation-based selection
    #[default]
    Correlation,
    /// Information gain
    InformationGain,
    /// Chi-square test
    ChiSquare,
    /// Recursive feature elimination
    RecursiveElimination,
}

/// Feature normalization configuration
#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct FeatureNormalizationConfig {
    /// Normalization method
    pub method: NormalizationMethod,
    /// Enable scaling
    pub enable_scaling: bool,
    /// Scaling range
    pub scaling_range: (f64, f64),
}

/// Normalization method
#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub enum NormalizationMethod {
    /// Z-score normalization
    #[default]
    ZScore,
    /// Robust scaling
    Robust,
    /// Unit vector scaling
    UnitVector,
}

/// Cache warming performance configuration
#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct WarmingPerformanceConfig {
    /// Enable parallel warming
    pub enable_parallel: bool,
    /// Warming thread count
    pub thread_count: u32,
    /// Warming batch size
    pub batch_size: u32,
    /// Warming rate limit
    pub rate_limit: Option<RateLimitConfig>,
    /// Enable SIMD optimization
    pub enable_simd: bool,
}

/// Rate limit configuration
#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct RateLimitConfig {
    /// Requests per second
    pub requests_per_second: u32,
    /// Burst size
    pub burst_size: u32,
    /// Rate limit window
    pub window: Duration,
}

/// Cache synchronization configuration
#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct CacheSynchronizationConfig {
    /// Enable synchronization
    pub enabled: bool,
    /// Synchronization strategy
    pub strategy: SynchronizationStrategy,
    /// Conflict resolution
    pub conflict_resolution: ConflictResolutionStrategy,
    /// Synchronization interval
    pub interval: Duration,
    /// Enable change notifications
    pub enable_notifications: bool,
}

/// Cache synchronization strategy
#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub enum SynchronizationStrategy {
    /// Eventually consistent
    #[default]
    EventuallyConsistent,
    /// Strong consistency
    StrongConsistency,
    /// Weak consistency
    WeakConsistency,
    /// Causal consistency
    CausalConsistency,
}

/// Conflict resolution strategy
#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub enum ConflictResolutionStrategy {
    /// Last writer wins
    #[default]
    LastWriterWins,
    /// First writer wins
    FirstWriterWins,
    /// Timestamp-based
    TimestampBased,
    /// Version-based
    VersionBased,
    /// Custom resolver
    Custom { resolver: String },
}

/// Cache performance optimization configuration
#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct CachePerformanceConfig {
    /// Enable SIMD optimization
    pub enable_simd: bool,
    /// SIMD configuration
    pub simd: SIMDConfig,
    /// Compression configuration
    pub compression: CompressionConfig,
    /// Serialization configuration
    pub serialization: SerializationConfig,
    /// Network optimization
    pub network: NetworkOptimizationConfig,
}

/// SIMD optimization configuration
#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct SIMDConfig {
    /// Enable SIMD
    pub enabled: bool,
    /// SIMD instruction set
    pub instruction_set: SIMDInstructionSet,
    /// SIMD operations
    pub operations: Vec<SIMDOperation>,
    /// SIMD vectorization
    pub vectorization: VectorizationConfig,
}

/// SIMD instruction set
#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub enum SIMDInstructionSet {
    /// SSE2 instruction set
    #[default]
    SSE2,
    /// SSE3
    SSE3,
    /// SSE4.1
    SSE4_1,
    /// SSE4.2
    SSE4_2,
    /// AVX (Advanced Vector Extensions)
    AVX,
    /// AVX2
    AVX2,
    /// AVX512
    AVX512,
    /// ARM NEON
    NEON,
}

/// SIMD operation
#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub enum SIMDOperation {
    /// Vector addition
    #[default]
    Add,
    /// Compression
    Compression,
    /// Encryption
    Encryption,
    /// Serialization
    Serialization,
    /// Comparison
    Comparison,
    /// Sorting
    Sorting,
}

/// Vectorization configuration
#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct VectorizationConfig {
    /// Vector size
    pub vector_size: u32,
    /// Enable auto-vectorization
    pub enable_auto_vectorization: bool,
    /// Vectorization threshold
    pub threshold: u32,
    /// Alignment requirements
    pub alignment: u32,
}

/// Compression configuration
#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct CompressionConfig {
    /// Enable compression
    pub enabled: bool,
    /// Compression algorithm
    pub algorithm: CompressionAlgorithm,
    /// Compression level
    pub level: u8,
    /// Compression threshold
    pub threshold: u32,
    /// Enable adaptive compression
    pub adaptive: bool,
}

/// Compression algorithm
#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub enum CompressionAlgorithm {
    /// No compression
    #[default]
    None,
    /// Gzip
    Gzip,
    /// Deflate
    Deflate,
    /// Brotli
    Brotli,
    /// LZ4
    LZ4,
    /// Zstd
    Zstd,
    /// Snappy
    Snappy,
}

/// Serialization configuration
#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct SerializationConfig {
    /// Serialization format
    pub format: SerializationFormat,
    /// Enable schema evolution
    pub enable_schema_evolution: bool,
    /// Compression integration
    pub compression_integration: bool,
    /// Enable binary optimization
    pub enable_binary_optimization: bool,
}

/// Serialization format
#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub enum SerializationFormat {
    /// JSON format
    #[default]
    Json,
    /// MessagePack
    MessagePack,
    /// Protocol Buffers
    ProtocolBuffers,
    /// Apache Avro
    Avro,
    /// Apache Thrift
    Thrift,
    /// Binary
    Binary,
}

/// Network optimization configuration
#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct NetworkOptimizationConfig {
    /// Enable connection pooling
    pub enable_connection_pooling: bool,
    /// Connection pool size
    pub pool_size: u32,
    /// Enable pipelining
    pub enable_pipelining: bool,
    /// Pipeline depth
    pub pipeline_depth: u32,
    /// Enable multiplexing
    pub enable_multiplexing: bool,
    /// Multiplexing factor
    pub multiplexing_factor: u32,
}

/// Storage backend for L3 cache
#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub enum StorageBackend {
    /// In-memory storage
    #[default]
    Memory,
    /// File system
    FileSystem { path: String },
    /// Database
    Database { url: String },
    /// Object storage
    ObjectStorage { bucket: String, region: String },
    /// Distributed file system
    DistributedFS { cluster: String },
}

/// Cache promotion strategy
#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub enum CachePromotionStrategy {
    /// Promote on access
    #[default]
    OnAccess,
    /// Promote on frequency
    OnFrequency { threshold: u32 },
    /// Promote on recency
    OnRecency { window: Duration },
    /// Promote on size
    OnSize { threshold: u32 },
    /// Adaptive promotion
    Adaptive,
}

/// Eviction coordination configuration
#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct EvictionCoordinationConfig {
    /// Enable coordination
    pub enabled: bool,
    /// Coordination strategy
    pub strategy: EvictionCoordinationStrategy,
    /// Notification method
    pub notification_method: NotificationMethod,
    /// Coordination timeout
    pub timeout: Duration,
}

/// Eviction coordination strategy
#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub enum EvictionCoordinationStrategy {
    /// Broadcast eviction notifications
    #[default]
    Broadcast,
    /// Selective eviction
    Selective,
    /// Hierarchical eviction
    Hierarchical,
    /// Independent eviction
    Independent,
}

/// Notification method
#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub enum NotificationMethod {
    /// Message queue notifications
    #[default]
    MessageQueue,
    /// HTTP webhook
    HttpWebhook { url: String },
    /// Redis pub/sub
    RedisPubSub { channel: String },
    /// In-memory notification
    InMemory,
}

/// Eviction policy
#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub enum EvictionPolicy {
    /// Least Recently Used
    #[default]
    LRU,
    /// Least Frequently Used
    LFU,
    /// First In, First Out
    FIFO,
    /// Time-based expiration
    TTL,
    /// Size-based eviction
    Size,
    /// Adaptive eviction
    Adaptive,
}

/// Partitioning strategy
#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub enum PartitioningStrategy {
    /// Hash-based partitioning
    #[default]
    Hash,
    /// Range-based partitioning
    Range,
    /// Directory-based partitioning
    Directory,
    /// Consistent hashing
    ConsistentHashing,
}

/// Redis configuration
#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct RedisConfig {
    /// Redis URL
    pub url: String,
    /// Connection pool size
    pub pool_size: u32,
    /// Connection timeout
    pub connection_timeout: Duration,
    /// Command timeout
    pub command_timeout: Duration,
    /// Enable TLS
    pub enable_tls: bool,
    /// Authentication
    pub auth: Option<RedisAuth>,
}

/// Redis authentication
#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct RedisAuth {
    /// Username
    pub username: Option<String>,
    /// Password
    pub password: String,
}

impl DistributedCachingConfig {
    /// Create production-optimized caching configuration
    pub fn production() -> Self {
        let mut config = Self::default();

        // Production L1 cache
        config.multi_tier.l1_cache.size_mb = 128;
        config.multi_tier.l1_cache.max_entries = 50000;
        config.multi_tier.l1_cache.enable_simd = true;

        // Production L2 cache
        config.multi_tier.l2_cache.size_mb = 2048;
        config.multi_tier.l2_cache.compression.enabled = true;
        config.multi_tier.l2_cache.compression.algorithm = CompressionAlgorithm::Zstd;

        // Production L3 cache
        config.multi_tier.l3_cache.size_gb = 50;
        config.multi_tier.l3_cache.enable_compression = true;
        config.multi_tier.l3_cache.enable_encryption = true;

        // Enable Redis cluster
        config.redis_cluster.enabled = true;
        config.redis_cluster.cluster_config.enable_cluster_mode = true;
        config.redis_cluster.cluster_config.replication_factor = 2;

        // Aggressive cache warming
        config.cache_warming.enabled = true;
        config.cache_warming.strategies = vec![
            WarmingStrategy::MostFrequent {
                item_count: 5000,
                frequency_threshold: 0.9,
            },
            WarmingStrategy::MostRecent {
                time_window: Duration::from_secs(1800),
                item_count: 2000,
            },
            WarmingStrategy::PatternBased {
                analysis_window: Duration::from_secs(3600),
                confidence_threshold: 0.85,
            },
        ];

        // Enable predictive warming
        config.cache_warming.predictive_warming.enabled = true;
        config.cache_warming.predictive_warming.ml_model.model_type = MLModelType::RandomForest;

        // Performance optimizations
        config.performance.enable_simd = true;
        config.performance.simd.instruction_set = SIMDInstructionSet::AVX512;
        config.performance.compression.algorithm = CompressionAlgorithm::Zstd;
        config.performance.serialization.format = SerializationFormat::ProtocolBuffers;

        config
    }

    /// Create development-optimized caching configuration
    pub fn development() -> Self {
        let mut config = Self::default();

        // Development L1 cache
        config.multi_tier.l1_cache.size_mb = 32;
        config.multi_tier.l1_cache.max_entries = 5000;

        // Development L2 cache
        config.multi_tier.l2_cache.size_mb = 128;
        config.multi_tier.l2_cache.compression.enabled = false;

        // Development L3 cache
        config.multi_tier.l3_cache.size_gb = 2;
        config.multi_tier.l3_cache.enable_compression = false;
        config.multi_tier.l3_cache.enable_encryption = false;

        // Disable Redis cluster
        config.redis_cluster.enabled = false;

        // Simple cache warming
        config.cache_warming.strategies = vec![WarmingStrategy::MostFrequent {
            item_count: 100,
            frequency_threshold: 0.5,
        }];

        // Disable predictive warming
        config.cache_warming.predictive_warming.enabled = false;

        // Basic performance optimizations
        config.performance.enable_simd = false;
        config.performance.compression.enabled = false;
        config.performance.serialization.format = SerializationFormat::Json;

        config
    }

    /// Validate configuration
    pub fn validate(&self) -> Result<(), String> {
        if self.multi_tier.l1_cache.size_mb == 0 {
            return Err("L1 cache size must be greater than 0".to_string());
        }

        if self.multi_tier.l2_cache.size_mb == 0 {
            return Err("L2 cache size must be greater than 0".to_string());
        }

        if self.multi_tier.l3_cache.size_gb == 0 {
            return Err("L3 cache size must be greater than 0".to_string());
        }

        if self.cache_warming.enabled && self.cache_warming.strategies.is_empty() {
            return Err("Cache warming enabled but no strategies configured".to_string());
        }

        if self.redis_cluster.enabled && self.redis_cluster.nodes.is_empty() {
            return Err("Redis cluster enabled but no nodes configured".to_string());
        }

        Ok(())
    }
}

/// Distributed caching utilities
pub mod utils {
    use super::*;

    /// Calculate optimal cache sizes based on system resources
    pub fn calculate_optimal_cache_sizes(
        total_memory_gb: u64,
        cache_memory_ratio: f64,
    ) -> (u32, u32, u32) {
        let cache_memory_gb = (total_memory_gb as f64 * cache_memory_ratio) as u64;
        let cache_memory_mb = cache_memory_gb * 1024;

        // L1: 10% of cache memory
        let l1_size_mb = ((cache_memory_mb as f64 * 0.1) as u32).max(32);

        // L2: 30% of cache memory
        let l2_size_mb = ((cache_memory_mb as f64 * 0.3) as u32).max(128);

        // L3: 60% of cache memory (in GB)
        let l3_size_gb = ((cache_memory_gb as f64 * 0.6) as u32).max(1);

        (l1_size_mb, l2_size_mb, l3_size_gb)
    }

    /// Estimate cache hit ratio based on configuration
    pub fn estimate_cache_hit_ratio(config: &DistributedCachingConfig) -> f64 {
        let mut base_ratio = 0.5f64;

        // L1 cache contribution
        if config.multi_tier.l1_cache.enabled {
            base_ratio += 0.2;
        }

        // L2 cache contribution
        if config.multi_tier.l2_cache.enabled {
            base_ratio += 0.15;
        }

        // L3 cache contribution
        if config.multi_tier.l3_cache.enabled {
            base_ratio += 0.1;
        }

        // Cache warming contribution
        if config.cache_warming.enabled {
            base_ratio += 0.05;
        }

        // Predictive warming contribution
        if config.cache_warming.predictive_warming.enabled {
            base_ratio += 0.05;
        }

        base_ratio.min(0.95)
    }

    /// Calculate cache memory usage
    pub fn calculate_memory_usage(config: &DistributedCachingConfig) -> u64 {
        let mut total_mb = 0;

        if config.multi_tier.l1_cache.enabled {
            total_mb += config.multi_tier.l1_cache.size_mb;
        }

        if config.multi_tier.l2_cache.enabled {
            total_mb += config.multi_tier.l2_cache.size_mb;
        }

        if config.multi_tier.l3_cache.enabled {
            total_mb += config.multi_tier.l3_cache.size_gb * 1024;
        }

        total_mb as u64
    }
}
