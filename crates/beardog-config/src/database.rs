use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use std::path::PathBuf;
use std::time::Duration;

/// Optimized database configuration with performance tuning
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct OptimizedDatabaseConfig {
    /// Database connection URL
    pub url: String,
    /// Connection pool configuration
    pub pool: ConnectionPoolConfig,
    /// Query performance optimization
    pub query_optimization: QueryOptimizationConfig,
    /// Caching configuration
    pub caching: DatabaseCachingConfig,
    /// Performance monitoring
    pub monitoring: DatabaseMonitoringConfig,
    /// SSL/TLS configuration
    pub ssl: Option<SslConfig>,
    /// Database-specific optimizations
    pub database_specific: DatabaseSpecificConfig,
}

/// Advanced connection pool configuration
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ConnectionPoolConfig {
    /// Maximum number of connections in the pool
    pub max_connections: u32,
    /// Minimum number of connections to maintain
    pub min_connections: u32,
    /// Connection acquire timeout
    pub acquire_timeout: Duration,
    /// Connection idle timeout
    pub idle_timeout: Duration,
    /// Connection maximum lifetime
    pub max_lifetime: Duration,
    /// Connection health check interval
    pub health_check_interval: Duration,
    /// Enable connection multiplexing
    pub enable_multiplexing: bool,
    /// Connection retry configuration
    pub retry_config: RetryConfig,
}

/// Query optimization configuration
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct QueryOptimizationConfig {
    /// Enable prepared statement caching
    pub enable_prepared_statements: bool,
    /// Maximum number of cached prepared statements
    pub max_prepared_statements: u32,
    /// Query result caching
    pub enable_query_caching: bool,
    /// Query timeout
    pub query_timeout: Duration,
    /// Enable query parallelization
    pub enable_parallel_queries: bool,
    /// Batch size for bulk operations
    pub batch_size: u32,
    /// Enable query performance monitoring
    pub enable_query_monitoring: bool,
}

/// Database caching configuration
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct DatabaseCachingConfig {
    /// Enable result caching
    pub enable_result_caching: bool,
    /// Cache size in MB
    pub cache_size_mb: u32,
    /// Cache TTL for different operation types
    pub cache_ttl: HashMap<String, Duration>,
    /// Enable write-through caching
    pub enable_write_through: bool,
    /// Cache eviction policy
    pub eviction_policy: CacheEvictionPolicy,
    /// Enable cache warming
    pub enable_cache_warming: bool,
}

/// Database monitoring configuration
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct DatabaseMonitoringConfig {
    /// Enable performance metrics collection
    pub enable_metrics: bool,
    /// Slow query threshold
    pub slow_query_threshold: Duration,
    /// Enable query logging
    pub enable_query_logging: bool,
    /// Connection pool metrics
    pub enable_pool_metrics: bool,
    /// Metrics collection interval
    pub metrics_interval: Duration,
}

/// SSL/TLS configuration
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SslConfig {
    /// Enable SSL/TLS
    pub enabled: bool,
    /// SSL certificate path
    pub cert_path: Option<PathBuf>,
    /// SSL key path
    pub key_path: Option<PathBuf>,
    /// SSL CA path
    pub ca_path: Option<PathBuf>,
    /// SSL mode (require, prefer, allow, disable)
    pub mode: String,
}

/// Database-specific optimizations
#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct DatabaseSpecificConfig {
    /// PostgreSQL specific settings
    pub postgresql: PostgresConfig,
    /// SQLite specific settings
    pub sqlite: SqliteConfig,
    /// MySQL specific settings
    pub mysql: MySqlConfig,
}

/// PostgreSQL optimization settings
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PostgresConfig {
    /// Shared buffers (percentage of RAM)
    pub shared_buffers_percent: f32,
    /// Work memory per query
    pub work_mem_mb: u32,
    /// WAL buffer size
    pub wal_buffers_mb: u32,
    /// Checkpoint completion target
    pub checkpoint_completion_target: f32,
    /// Enable parallel workers
    pub enable_parallel_workers: bool,
    /// Max parallel workers
    pub max_parallel_workers: u32,
    /// Enable JIT compilation
    pub enable_jit: bool,
}

/// SQLite optimization settings
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SqliteConfig {
    /// Journal mode (WAL, DELETE, TRUNCATE, PERSIST, MEMORY, OFF)
    pub journal_mode: String,
    /// Synchronous mode (OFF, NORMAL, FULL, EXTRA)
    pub synchronous_mode: String,
    /// Cache size (number of pages)
    pub cache_size: i32,
    /// Page size in bytes
    pub page_size: u32,
    /// Enable memory-mapped I/O
    pub enable_mmap: bool,
    /// Memory map size
    pub mmap_size: u64,
    /// Enable foreign keys
    pub enable_foreign_keys: bool,
}

/// MySQL optimization settings
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct MySqlConfig {
    /// InnoDB buffer pool size
    pub innodb_buffer_pool_size_mb: u32,
    /// InnoDB log file size
    pub innodb_log_file_size_mb: u32,
    /// InnoDB flush method
    pub innodb_flush_method: String,
    /// Enable query cache
    pub enable_query_cache: bool,
    /// Query cache size
    pub query_cache_size_mb: u32,
    /// Max connections
    pub max_connections: u32,
}

/// Connection retry configuration
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct RetryConfig {
    /// Maximum number of retry attempts
    pub max_attempts: u32,
    /// Initial delay between retries
    pub initial_delay: Duration,
    /// Maximum delay between retries
    pub max_delay: Duration,
    /// Delay multiplier for exponential backoff
    pub delay_multiplier: f64,
    /// Enable jitter to prevent thundering herd
    pub enable_jitter: bool,
}

/// Cache eviction policy
#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum CacheEvictionPolicy {
    /// Least Recently Used
    Lru,
    /// Least Frequently Used
    Lfu,
    /// First In, First Out
    Fifo,
    /// Time-based expiration
    TimeBasedExpiration,
    /// Adaptive replacement cache
    Arc,
}

impl Default for OptimizedDatabaseConfig {
    fn default() -> Self {
        Self {
            url: std::env::var("BEARDOG_DATABASE_URL")
                .unwrap_or_else(|_| "postgresql://localhost/beardog".to_string()),
            pool: ConnectionPoolConfig::default(),
            query_optimization: QueryOptimizationConfig::default(),
            caching: DatabaseCachingConfig::default(),
            monitoring: DatabaseMonitoringConfig::default(),
            ssl: None,
            database_specific: DatabaseSpecificConfig::default(),
        }
    }
}

impl Default for ConnectionPoolConfig {
    fn default() -> Self {
        Self {
            max_connections: 100,
            min_connections: 10,
            acquire_timeout: Duration::from_secs(30),
            idle_timeout: Duration::from_secs(300),
            max_lifetime: Duration::from_secs(1800),
            health_check_interval: Duration::from_secs(30),
            enable_multiplexing: true,
            retry_config: RetryConfig::default(),
        }
    }
}

impl Default for QueryOptimizationConfig {
    fn default() -> Self {
        Self {
            enable_prepared_statements: true,
            max_prepared_statements: 1000,
            enable_query_caching: true,
            query_timeout: Duration::from_secs(30),
            enable_parallel_queries: true,
            batch_size: 1000,
            enable_query_monitoring: true,
        }
    }
}

impl Default for DatabaseCachingConfig {
    fn default() -> Self {
        let mut cache_ttl = HashMap::new();
        cache_ttl.insert("SELECT".to_string(), Duration::from_secs(300));
        cache_ttl.insert("INSERT".to_string(), Duration::from_secs(0));
        cache_ttl.insert("UPDATE".to_string(), Duration::from_secs(0));
        cache_ttl.insert("DELETE".to_string(), Duration::from_secs(0));

        Self {
            enable_result_caching: true,
            cache_size_mb: 256,
            cache_ttl,
            enable_write_through: true,
            eviction_policy: CacheEvictionPolicy::Lru,
            enable_cache_warming: true,
        }
    }
}

impl Default for DatabaseMonitoringConfig {
    fn default() -> Self {
        Self {
            enable_metrics: true,
            slow_query_threshold: Duration::from_secs(1),
            enable_query_logging: true,
            enable_pool_metrics: true,
            metrics_interval: Duration::from_secs(10),
        }
    }
}

impl Default for PostgresConfig {
    fn default() -> Self {
        Self {
            shared_buffers_percent: 0.25,
            work_mem_mb: 64,
            wal_buffers_mb: 16,
            checkpoint_completion_target: 0.9,
            enable_parallel_workers: true,
            max_parallel_workers: 4,
            enable_jit: true,
        }
    }
}

impl Default for SqliteConfig {
    fn default() -> Self {
        Self {
            journal_mode: "WAL".to_string(),
            synchronous_mode: "NORMAL".to_string(),
            cache_size: 2000,
            page_size: 4096,
            enable_mmap: true,
            mmap_size: 268435456, // 256MB
            enable_foreign_keys: true,
        }
    }
}

impl Default for MySqlConfig {
    fn default() -> Self {
        Self {
            innodb_buffer_pool_size_mb: 512,
            innodb_log_file_size_mb: 256,
            innodb_flush_method: "O_DIRECT".to_string(),
            enable_query_cache: true,
            query_cache_size_mb: 64,
            max_connections: 200,
        }
    }
}

impl Default for RetryConfig {
    fn default() -> Self {
        Self {
            max_attempts: 3,
            initial_delay: Duration::from_millis(100),
            max_delay: Duration::from_secs(5),
            delay_multiplier: 2.0,
            enable_jitter: true,
        }
    }
}

impl OptimizedDatabaseConfig {
    /// Create production-optimized database configuration
    pub fn production() -> Self {
        let mut config = Self::default();

        // Production-specific optimizations
        config.pool.max_connections = 200;
        config.pool.min_connections = 20;
        config.pool.acquire_timeout = Duration::from_secs(10);
        config.pool.idle_timeout = Duration::from_secs(600);
        config.pool.max_lifetime = Duration::from_secs(3600);

        // Enable all optimizations
        config.query_optimization.enable_prepared_statements = true;
        config.query_optimization.enable_query_caching = true;
        config.query_optimization.enable_parallel_queries = true;
        config.query_optimization.batch_size = 5000;

        // Aggressive caching
        config.caching.enable_result_caching = true;
        config.caching.cache_size_mb = 512;
        config.caching.enable_write_through = true;
        config.caching.enable_cache_warming = true;

        // Comprehensive monitoring
        config.monitoring.enable_metrics = true;
        config.monitoring.slow_query_threshold = Duration::from_millis(500);
        config.monitoring.enable_query_logging = true;
        config.monitoring.enable_pool_metrics = true;

        config
    }

    /// Create development-optimized database configuration
    pub fn development() -> Self {
        let mut config = Self::default();

        // Development-specific optimizations
        config.pool.max_connections = 50;
        config.pool.min_connections = 5;
        config.pool.acquire_timeout = Duration::from_secs(30);

        // Moderate caching
        config.caching.cache_size_mb = 128;
        config.caching.enable_cache_warming = false;

        // Detailed monitoring for debugging
        config.monitoring.slow_query_threshold = Duration::from_millis(100);
        config.monitoring.enable_query_logging = true;

        config
    }

    /// Validate configuration
    pub fn validate(&self) -> Result<(), String> {
        if self.pool.max_connections <= self.pool.min_connections {
            return Err("max_connections must be greater than min_connections".to_string());
        }

        if self.pool.acquire_timeout.as_secs() == 0 {
            return Err("acquire_timeout must be greater than 0".to_string());
        }

        if self.query_optimization.batch_size == 0 {
            return Err("batch_size must be greater than 0".to_string());
        }

        if self.caching.cache_size_mb == 0 {
            return Err("cache_size_mb must be greater than 0".to_string());
        }

        Ok(())
    }
}
