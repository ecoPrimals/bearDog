// SPDX-License-Identifier: AGPL-3.0-or-later

//! Provider-side performance tuning: caching, compression, buffers, and rate limits.

use crate::canonical::traits::CacheStrategy;
use crate::constants::buffers;
use crate::constants::defaults;
use crate::constants::time;
use beardog_config::env_keys;
use serde::{Deserialize, Serialize};
use std::time::Duration;

/// Aggregated tuning block referenced from [`super::CanonicalProviderConfig`].
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PerformanceConfig {
    /// Maximum concurrent requests
    /// Number of `max_concurrent_requests`
    pub max_concurrent_requests: u32,

    /// Request rate limiting
    /// The rate limiting value
    pub rate_limiting: super::super::config::domains::network::RateLimitConfig,

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
            rate_limiting: super::super::config::domains::network::RateLimitConfig::default(),
            caching: CachingConfig::default(),
            compression: CompressionConfig::default(),
            buffer: BufferConfig::default(),
            monitoring: PerformanceMonitoringConfig::default(),
        }
    }
}

/// Rate limiting configuration (DEPRECATED - use canonical)
///
/// **MIGRATION**: Use `super::super::config::domains::network::RateLimitConfig` instead.
///
/// This type alias will be removed in v3.3.0.
#[deprecated(
    since = "3.1.0",
    note = "Use super::super::config::domains::network::RateLimitConfig instead"
)]
pub type RateLimitConfig = super::super::config::domains::network::RateLimitConfig;

/// Rate limiting algorithms (DEPRECATED - use canonical `RateLimitStrategy`)
///
/// **MIGRATION**: Use `super::super::config::domains::network::RateLimitStrategy` instead.
///
/// This type alias will be removed in v3.3.0.
#[deprecated(
    since = "3.1.0",
    note = "Use super::super::config::domains::network::RateLimitStrategy instead"
)]
pub type RateLimitAlgorithm = super::super::config::domains::network::RateLimitStrategy;

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
            max_entries: defaults::DEFAULT_MAX_ENTRIES,
            ttl: Duration::from_secs(time::SECONDS_PER_HOUR),
            eviction_policy: EvictionPolicy::Lru,
        }
    }
}

// Implement CacheStrategy trait for provider unified caching configuration
impl CacheStrategy for CachingConfig {
    fn max_entries(&self) -> usize {
        if !self.enabled {
            return 0;
        }
        self.max_entries
    }

    fn ttl(&self) -> Duration {
        self.ttl
    }

    fn eviction_policy(&self) -> crate::canonical::traits::cache::EvictionPolicy {
        use crate::canonical::traits::cache::EvictionPolicy as TraitPolicy;
        match self.eviction_policy {
            EvictionPolicy::Lru => TraitPolicy::Lru,
            EvictionPolicy::Lfu => TraitPolicy::Lfu,
            EvictionPolicy::Fifo => TraitPolicy::Fifo,
            EvictionPolicy::Random => TraitPolicy::Random,
        }
    }

    fn is_enabled(&self) -> bool {
        self.enabled
    }

    fn validate(&self) -> Result<(), String> {
        if !self.enabled {
            return Ok(());
        }
        if self.max_entries == 0 {
            return Err("max_entries must be > 0 when caching is enabled".to_string());
        }
        if self.ttl.is_zero() {
            return Err("TTL cannot be zero".to_string());
        }
        Ok(())
    }

    fn is_production_ready(&self) -> bool {
        if !self.enabled {
            return true;
        }
        self.max_entries >= 100 && // At least 100 entries
        self.max_entries <= 1_000_000 && // At most 1M entries
        self.ttl >= Duration::from_secs(60) && // At least 1 minute
        self.ttl <= Duration::from_secs(time::SECONDS_PER_DAY) && // At most 1 day
        self.validate().is_ok()
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
    /// Plug-in cache backend identified by name.
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
            read_buffer_size: buffers::DEFAULT_SIZE,
            write_buffer_size: buffers::DEFAULT_SIZE,
            pool_size: 100,
        }
    }
}

/// Scrapes latency/error signals and raises alerts when [`PerformanceThresholds`] are breached.
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

/// SLO-style ceilings used by [`PerformanceMonitoringConfig`].
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PerformanceThresholds {
    /// Maximum response time (milliseconds)
    /// **Default:** `5000` (`BEARDOG_PROVIDER_MAX_RESPONSE_TIME_MS`).
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
            max_response_time_ms: defaults::DEFAULT_MAX_RESPONSE_TIME_MS,
            max_error_rate: 5.0,
            max_cpu_usage: 80.0,
            max_memory_usage: 1_000_000_000,
            max_concurrent_connections: 1000,
        }
    }
}

/// Rate-limits alert noise via cooldown and escalation counts.
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

impl PerformanceConfig {
    /// Load provider performance settings from environment variables, falling back to [`Default::default`].
    #[must_use]
    pub fn from_env() -> Self {
        Self {
            max_concurrent_requests: std::env::var(env_keys::ENV_PROVIDER_MAX_CONCURRENT_REQUESTS)
                .ok()
                .and_then(|v| v.parse().ok())
                .unwrap_or(100),
            rate_limiting: super::super::config::domains::network::RateLimitConfig::default(),
            caching: CachingConfig::from_env(),
            compression: CompressionConfig::from_env(),
            buffer: BufferConfig::from_env(),
            monitoring: PerformanceMonitoringConfig::from_env(),
        }
    }
}

impl CachingConfig {
    /// Load cache settings from environment variables, falling back to [`Default::default`].
    #[must_use]
    pub fn from_env() -> Self {
        Self {
            max_entries: std::env::var(env_keys::ENV_PROVIDER_CACHE_MAX_ENTRIES)
                .ok()
                .and_then(|v| v.parse().ok())
                .unwrap_or(defaults::DEFAULT_MAX_ENTRIES),
            ttl: Duration::from_secs(
                std::env::var(env_keys::ENV_PROVIDER_CACHE_TTL_SECS)
                    .ok()
                    .and_then(|v| v.parse().ok())
                    .unwrap_or(time::SECONDS_PER_HOUR),
            ),
            ..Default::default()
        }
    }
}

impl CompressionConfig {
    /// Load compression settings from environment variables, falling back to [`Default::default`].
    #[must_use]
    pub fn from_env() -> Self {
        Self {
            level: std::env::var(env_keys::ENV_COMPRESSION_LEVEL)
                .ok()
                .and_then(|v| v.parse().ok())
                .unwrap_or(6),
            min_size: std::env::var(env_keys::ENV_COMPRESSION_MIN_SIZE)
                .ok()
                .and_then(|v| v.parse().ok())
                .unwrap_or(1024),
            ..Default::default()
        }
    }
}

impl BufferConfig {
    /// Load buffer pool settings from environment variables, falling back to [`Default::default`].
    #[must_use]
    pub fn from_env() -> Self {
        Self {
            read_buffer_size: std::env::var(env_keys::ENV_READ_BUFFER_SIZE)
                .ok()
                .and_then(|v| v.parse().ok())
                .unwrap_or(buffers::DEFAULT_SIZE),
            write_buffer_size: std::env::var(env_keys::ENV_WRITE_BUFFER_SIZE)
                .ok()
                .and_then(|v| v.parse().ok())
                .unwrap_or(buffers::DEFAULT_SIZE),
            pool_size: std::env::var(env_keys::ENV_BUFFER_POOL_SIZE)
                .ok()
                .and_then(|v| v.parse().ok())
                .unwrap_or(100),
        }
    }
}

impl PerformanceMonitoringConfig {
    /// Load performance monitoring settings from environment variables, falling back to [`Default::default`].
    #[must_use]
    pub fn from_env() -> Self {
        Self {
            metrics_interval: Duration::from_secs(
                std::env::var(env_keys::ENV_PROVIDER_METRICS_INTERVAL_SECS)
                    .ok()
                    .and_then(|v| v.parse().ok())
                    .unwrap_or(60),
            ),
            thresholds: PerformanceThresholds::from_env(),
            alerting: PerformanceAlertingConfig::from_env(),
            ..Default::default()
        }
    }
}

impl PerformanceThresholds {
    /// Load SLO thresholds from environment variables, falling back to [`Default::default`].
    #[must_use]
    pub fn from_env() -> Self {
        Self {
            max_response_time_ms: std::env::var(env_keys::ENV_PROVIDER_MAX_RESPONSE_TIME_MS)
                .ok()
                .and_then(|v| v.parse().ok())
                .unwrap_or(defaults::DEFAULT_MAX_RESPONSE_TIME_MS),
            max_error_rate: std::env::var(env_keys::ENV_PROVIDER_MAX_ERROR_RATE)
                .ok()
                .and_then(|v| v.parse().ok())
                .unwrap_or(5.0),
            max_cpu_usage: std::env::var(env_keys::ENV_PROVIDER_MAX_CPU_USAGE)
                .ok()
                .and_then(|v| v.parse().ok())
                .unwrap_or(80.0),
            max_memory_usage: std::env::var(env_keys::ENV_PROVIDER_MAX_MEMORY_USAGE)
                .ok()
                .and_then(|v| v.parse().ok())
                .unwrap_or(1_000_000_000),
            max_concurrent_connections: std::env::var(
                env_keys::ENV_PROVIDER_MAX_CONCURRENT_CONNECTIONS,
            )
            .ok()
            .and_then(|v| v.parse().ok())
            .unwrap_or(1000),
        }
    }
}

impl PerformanceAlertingConfig {
    /// Load performance alerting settings from environment variables, falling back to [`Default::default`].
    #[must_use]
    pub fn from_env() -> Self {
        Self {
            cooldown_period: Duration::from_secs(
                std::env::var(env_keys::ENV_PROVIDER_ALERT_COOLDOWN_SECS)
                    .ok()
                    .and_then(|v| v.parse().ok())
                    .unwrap_or(300),
            ),
            escalation_threshold: std::env::var(env_keys::ENV_PROVIDER_ALERT_ESCALATION_THRESHOLD)
                .ok()
                .and_then(|v| v.parse().ok())
                .unwrap_or(3),
            ..Default::default()
        }
    }
}

#[cfg(test)]
mod performance_coverage_tests {
    use super::*;
    use crate::canonical::traits::CacheStrategy;

    #[test]
    fn performance_config_default_round_trips_json() {
        let p = PerformanceConfig::default();
        let v = serde_json::to_value(&p).expect("ser");
        let back: PerformanceConfig = serde_json::from_value(v).expect("de");
        assert_eq!(back.max_concurrent_requests, p.max_concurrent_requests);
    }

    #[test]
    fn caching_config_validate_when_disabled_always_ok() {
        let mut c = CachingConfig::default();
        c.enabled = false;
        c.max_entries = 0;
        c.ttl = Duration::ZERO;
        assert!(c.validate().is_ok());
        assert_eq!(c.max_entries(), 0);
        assert!(!c.is_enabled());
    }

    #[test]
    fn caching_config_validate_errors_on_zero_entries_when_enabled() {
        let mut c = CachingConfig::default();
        c.enabled = true;
        c.max_entries = 0;
        assert!(c.validate().is_err());
    }

    #[test]
    fn caching_config_validate_errors_on_zero_ttl_when_enabled() {
        let mut c = CachingConfig::default();
        c.enabled = true;
        c.ttl = Duration::ZERO;
        assert!(c.validate().is_err());
    }

    #[test]
    fn caching_config_eviction_maps_all_variants() {
        use crate::canonical::traits::cache::EvictionPolicy as TraitPolicy;

        for (policy, expect) in [
            (EvictionPolicy::Lru, TraitPolicy::Lru),
            (EvictionPolicy::Lfu, TraitPolicy::Lfu),
            (EvictionPolicy::Fifo, TraitPolicy::Fifo),
            (EvictionPolicy::Random, TraitPolicy::Random),
        ] {
            let c = CachingConfig {
                eviction_policy: policy,
                ..Default::default()
            };
            assert_eq!(c.eviction_policy(), expect);
        }
    }

    #[test]
    fn caching_config_is_production_ready_matches_expectations() {
        let good = CachingConfig::default();
        assert!(good.is_production_ready());

        let mut bad = CachingConfig::default();
        bad.max_entries = 5;
        assert!(!bad.is_production_ready());
    }

    #[test]
    fn performance_thresholds_and_alerting_defaults_are_sensible() {
        let t = PerformanceThresholds::default();
        assert!(t.max_response_time_ms > 0);
        assert!(t.max_error_rate >= 0.0);
        let a = PerformanceAlertingConfig::default();
        assert!(a.cooldown_period > Duration::ZERO);
        assert!(a.escalation_threshold > 0);
    }

    #[test]
    fn compression_and_buffer_defaults() {
        let c = CompressionConfig::default();
        assert!(c.min_size > 0);
        let b = BufferConfig::default();
        assert!(b.read_buffer_size > 0 && b.pool_size > 0);
    }
}
