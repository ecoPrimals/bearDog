// Performance Configuration
//
// Provider performance tuning, rate limiting, caching, and optimization settings.

use serde::{Deserialize, Serialize};
use std::time::Duration;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PerformanceConfig {
    /// Maximum concurrent requests
    /// Number of `max_concurrent_requests`
    pub max_concurrent_requests: u32,

    /// Request rate limiting
    /// The rate limiting value
    pub rate_limiting: RateLimitConfig,

    /// Caching configuration
    /// The caching value
    pub caching: CachingConfig,

    /// Compression configuration
    /// The compression value
    pub compression: CompressionConfig,

    /// Buffer configuration
    /// The buffer value
    pub buffer: BufferConfig,

    /// The monitoring value
    pub monitoring: PerformanceMonitoringConfig,
}

impl Default for PerformanceConfig {
    fn default() -> Self {
        Self {
            max_concurrent_requests: 100,
            rate_limiting: RateLimitConfig::default(),
            caching: CachingConfig::default(),
            compression: CompressionConfig::default(),
            buffer: BufferConfig::default(),
            monitoring: PerformanceMonitoringConfig::default(),
        }
    }
}

/// Rate limiting configuration
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct RateLimitConfig {
    /// Rate limiting enabled
    /// Whether feature is enabled
    pub enabled: bool,

    /// Requests per second
    /// The requests per second value
    pub requests_per_second: f64,

    /// Burst capacity
    /// Number of `burst_capacity`
    pub burst_capacity: u32,

    /// Rate limit window
    /// The window value
    pub window: Duration,

    /// Rate limit algorithm
    /// The algorithm value
    pub algorithm: RateLimitAlgorithm,
}

impl Default for RateLimitConfig {
    fn default() -> Self {
        Self {
            enabled: true,
            requests_per_second: 100.0,
            burst_capacity: 200,
            window: Duration::from_secs(60),
            algorithm: RateLimitAlgorithm::TokenBucket,
        }
    }
}

/// Rate limiting algorithms
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub enum RateLimitAlgorithm {
    /// `TokenBucket` variant
    TokenBucket,
    /// `LeakyBucket` variant
    LeakyBucket,
    /// `FixedWindow` variant
    FixedWindow,
    /// `SlidingWindow` variant
    SlidingWindow,
}

/// Caching configuration
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CachingConfig {
    /// Caching enabled
    /// Whether feature is enabled
    pub enabled: bool,

    /// Cache type
    /// The cache type value
    pub cache_type: CacheType,

    /// Cache size (in entries)
    /// Number of `max_entries`
    pub max_entries: usize,

    /// Cache TTL (time to live)
    /// The ttl value
    pub ttl: Duration,

    /// Cache eviction policy
    /// The eviction policy value
    pub eviction_policy: EvictionPolicy,
}

impl Default for CachingConfig {
    fn default() -> Self {
        Self {
            enabled: true,
            cache_type: CacheType::Memory,
            max_entries: 10000,
            ttl: Duration::from_secs(3600),
            eviction_policy: EvictionPolicy::Lru,
        }
    }
}

/// Cache types
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
/// Types of cache
pub enum CacheType {
    /// Memory variant
    Memory,
    /// Redis variant
    Redis,
    /// Memcached variant
    Memcached,
    Custom(String),
}

/// Cache eviction policies
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub enum EvictionPolicy {
    /// Lru variant
    Lru,
    /// Lfu variant
    Lfu,
    /// Fifo variant
    Fifo,
    /// Random variant
    Random,
}

/// Compression configuration
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CompressionConfig {
    /// Compression enabled
    /// Whether feature is enabled
    pub enabled: bool,

    /// Compression algorithm
    /// The algorithm value
    pub algorithm: CompressionAlgorithm,

    /// Compression level (1-9)
    /// Number of level
    pub level: u8,

    /// Minimum size to compress (bytes)
    /// Number of `min_size`
    pub min_size: usize,
}

impl Default for CompressionConfig {
    fn default() -> Self {
        Self {
            enabled: true,
            algorithm: CompressionAlgorithm::Gzip,
            level: 6,
            min_size: 1024,
        }
    }
}

/// Compression algorithms
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub enum CompressionAlgorithm {
    /// Gzip variant
    Gzip,
    /// Deflate variant
    Deflate,
    /// Brotli variant
    Brotli,
    /// Zstd variant
    Zstd,
}

/// Buffer configuration
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct BufferConfig {
    /// Read buffer size (bytes)
    /// Number of `read_buffer_size`
    pub read_buffer_size: usize,

    /// Write buffer size (bytes)
    /// Number of `write_buffer_size`
    pub write_buffer_size: usize,

    /// Buffer pool size
    /// Number of `pool_size`
    pub pool_size: usize,
}

impl Default for BufferConfig {
    fn default() -> Self {
        Self {
            read_buffer_size: 8192,
            write_buffer_size: 8192,
            pool_size: 100,
        }
    }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PerformanceMonitoringConfig {
    /// Whether feature is enabled
    pub enabled: bool,

    /// Metrics collection interval
    /// The metrics interval value
    pub metrics_interval: Duration,

    /// The thresholds value
    pub thresholds: PerformanceThresholds,

    /// Alerting configuration
    /// The alerting value
    pub alerting: PerformanceAlertingConfig,
}

impl Default for PerformanceMonitoringConfig {
    fn default() -> Self {
        Self {
            enabled: true,
            metrics_interval: Duration::from_secs(60),
            thresholds: PerformanceThresholds::default(),
            alerting: PerformanceAlertingConfig::default(),
        }
    }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PerformanceThresholds {
    /// Maximum response time (milliseconds)
    pub max_response_time_ms: u64,

    /// Maximum error rate (percentage)
    /// The max error rate value
    pub max_error_rate: f64,

    /// Maximum CPU usage (percentage)
    /// The max cpu usage value
    pub max_cpu_usage: f64,

    /// Maximum memory usage (bytes)
    /// Number of `max_memory_usage`
    pub max_memory_usage: u64,

    /// Maximum concurrent connections
    /// Number of `max_concurrent_connections`
    pub max_concurrent_connections: u32,
}

impl Default for PerformanceThresholds {
    fn default() -> Self {
        Self {
            max_response_time_ms: 5000,
            max_error_rate: 5.0,
            max_cpu_usage: 80.0,
            max_memory_usage: 1_000_000_000, // 1GB
            max_concurrent_connections: 1000,
        }
    }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PerformanceAlertingConfig {
    /// Alerting enabled
    /// Whether feature is enabled
    pub enabled: bool,

    /// Alert cooldown period
    /// The cooldown period value
    pub cooldown_period: Duration,

    /// Alert escalation threshold
    /// Number of `escalation_threshold`
    pub escalation_threshold: u32,
}

impl Default for PerformanceAlertingConfig {
    fn default() -> Self {
        Self {
            enabled: true,
            cooldown_period: Duration::from_secs(300),
            escalation_threshold: 3,
        }
    }
}
