// SPDX-License-Identifier: AGPL-3.0-only

// System Domain Constants
//
// This module provides system-level constants consolidated from the large unified.rs file.
// It includes version information, compile-time constants, performance limits, and system defaults.

use std::time::Duration;

/// Released component versions, protocol versions, and build target metadata.
pub mod versions {
    /// Primary `BearDog` version from Cargo.toml
    pub const BEARDOG_VERSION: &str = env!("CARGO_PKG_VERSION");

    /// Component versions
    /// Version of the `BearDog` workflow system
    pub const WORKFLOW_SYSTEM_VERSION: &str = "3.1.0";
    /// Version of the HSM foundation layer
    pub const HSM_FOUNDATION_VERSION: &str = "2.0.0-clean";
    /// Version of the software HSM implementation
    pub const SOFTWARE_HSM_VERSION: &str = "1.0.0";
    /// Version of the universal storage system
    pub const UNIVERSAL_STORAGE_VERSION: &str = "2.0.0";
    /// Version of the `BearDog` core library
    pub const BEARDOG_CORE_VERSION: &str = "2.0.0";
    /// Version of the `BearDog` tunnel system
    pub const TUNNEL_VERSION: &str = "1.0.0";
    /// Version of the genetics/AI system
    pub const GENETICS_VERSION: &str = "1.0.0";

    /// Compatibility versions
    /// Minimum supported client version
    pub const MIN_SUPPORTED_CLIENT_VERSION: &str = "1.0.0";
    /// Minimum supported Rust toolchain for building BearDog
    pub const MIN_RUST_VERSION: &str = "1.70.0";
    /// Maximum supported `BearDog` version
    pub const MAX_SUPPORTED_VERSION: &str = "4.0.0";

    /// Protocol and API versions
    /// `BearDog` protocol version
    pub const PROTOCOL_VERSION: &str = "2.0";
    /// `BearDog` API version
    pub const API_VERSION: &str = "2.0";
    /// Configuration schema version
    pub const CONFIG_SCHEMA_VERSION: &str = "1.0.0";

    /// Target CPU architecture at compile time (`std::env::consts::ARCH`)
    pub const BUILD_TARGET: &str = std::env::consts::ARCH;
    /// Target operating system at compile time (`std::env::consts::OS`)
    pub const BUILD_OS: &str = std::env::consts::OS;
    /// Target OS family at compile time (`std::env::consts::FAMILY`)
    pub const BUILD_FAMILY: &str = std::env::consts::FAMILY;
}

/// **SYSTEM DEFAULTS** - Default system configuration values
pub mod defaults {
    use super::Duration;

    /// Memory and buffer defaults
    /// Default buffer size in bytes (4KB)
    pub const DEFAULT_BUFFER_SIZE: usize = 4096;
    /// Default cache size in entries
    pub const DEFAULT_CACHE_SIZE: usize = 512;
    /// Default connection pool size
    pub const DEFAULT_POOL_SIZE: usize = 10;
    /// Default bounded queue depth for async or worker pipelines
    pub const DEFAULT_QUEUE_SIZE: usize = 1000;

    /// Connection defaults
    /// Default maximum number of concurrent connections
    pub const DEFAULT_MAX_CONNECTIONS: usize = 1000;
    /// Default connection timeout duration
    pub const DEFAULT_CONNECTION_TIMEOUT: Duration = Duration::from_secs(30);
    /// Configuration constant: default read timeout
    pub const DEFAULT_READ_TIMEOUT: Duration = Duration::from_secs(60);
    /// Configuration constant: default write timeout
    pub const DEFAULT_WRITE_TIMEOUT: Duration = Duration::from_secs(30);
    /// Configuration constant: default idle timeout
    pub const DEFAULT_IDLE_TIMEOUT: Duration = Duration::from_secs(300);

    /// Thread and concurrency defaults
    /// Default size for generic worker / blocking thread pools
    pub const DEFAULT_THREAD_POOL_SIZE: usize = 4;
    /// Configuration constant: default max concurrent tasks
    pub const DEFAULT_MAX_CONCURRENT_TASKS: usize = 100;
    /// Configuration constant: default task timeout
    pub const DEFAULT_TASK_TIMEOUT: Duration = Duration::from_secs(300);

    /// Logging and monitoring defaults
    /// Default log verbosity (`tracing` / env filter style)
    pub const DEFAULT_LOG_LEVEL: &str = "info";
    /// Default structured log encoding (`json`, etc.)
    pub const DEFAULT_LOG_FORMAT: &str = "json";
    /// Configuration constant: default metrics interval
    pub const DEFAULT_METRICS_INTERVAL: Duration = Duration::from_secs(30);
    /// Configuration constant: default health check interval
    pub const DEFAULT_HEALTH_CHECK_INTERVAL: Duration = Duration::from_secs(30);

    /// Retry and backoff defaults
    pub const DEFAULT_MAX_RETRIES: u32 = 3;
    /// Configuration constant: default retry delay
    pub const DEFAULT_RETRY_DELAY: Duration = Duration::from_millis(100);
    /// Configuration constant: default backoff multiplier
    pub const DEFAULT_BACKOFF_MULTIPLIER: f64 = 2.0;
    /// Configuration constant: default max backoff
    pub const DEFAULT_MAX_BACKOFF: Duration = Duration::from_secs(30);

    /// File system defaults
    pub const DEFAULT_CONFIG_DIR: &str = "/etc/beardog";
    /// Configuration constant: default data dir
    pub const DEFAULT_DATA_DIR: &str = "/var/lib/beardog";
    /// Configuration constant: default log dir
    pub const DEFAULT_LOG_DIR: &str = "/var/log/beardog";
    /// Configuration constant: default cache dir
    pub const DEFAULT_CACHE_DIR: &str = "/var/cache/beardog";
    /// Configuration constant: default temp dir
    pub const DEFAULT_TEMP_DIR: &str = "/tmp/beardog";
    /// Default Unix socket path for IPC (last-resort fallback).
    pub const DEFAULT_SOCKET_PATH: &str = "/tmp/beardog.sock";
    /// Default TCP discovery file path (filename under `/tmp` search order).
    pub const DEFAULT_IPC_PORT_FILE: &str = "/tmp/beardog-ipc-port";
    /// Default software key storage directory when env and XDG paths are unset.
    pub const DEFAULT_KEY_STORAGE_DIR: &str = "/tmp/beardog/keys";

    /// Environment defaults
    pub const DEFAULT_ENVIRONMENT: &str = "production";
    /// Configuration constant: default deployment mode
    pub const DEFAULT_DEPLOYMENT_MODE: &str = "standalone";
    /// Configuration constant: default service name
    pub const DEFAULT_SERVICE_NAME: &str = "beardog ";
}

/// **SYSTEM LIMITS** - Maximum and minimum system limits
pub mod limits {
    use super::Duration;

    /// Memory limits
    pub const MAX_MEMORY_USAGE: usize = 100 * 1024 * 1024; // 100MB
    /// Configuration constant: max buffer size
    pub const MAX_BUFFER_SIZE: usize = 1024 * 1024; // 1MB
    /// Configuration constant: max cache size
    pub const MAX_CACHE_SIZE: usize = 50 * 1024 * 1024; // 50MB
    /// Configuration constant: min buffer size
    pub const MIN_BUFFER_SIZE: usize = 256;

    /// Connection limits
    pub const MAX_CONNECTIONS: usize = 10000;
    /// Configuration constant: min connections
    pub const MIN_CONNECTIONS: usize = 1;
    /// Configuration constant: max connection pool size
    pub const MAX_CONNECTION_POOL_SIZE: usize = 100;
    /// Configuration constant: min connection pool size
    pub const MIN_CONNECTION_POOL_SIZE: usize = 1;

    /// Thread limits
    pub const MAX_THREAD_POOL_SIZE: usize = 1000;
    /// Configuration constant: min thread pool size
    pub const MIN_THREAD_POOL_SIZE: usize = 1;
    /// Configuration constant: max concurrent tasks
    pub const MAX_CONCURRENT_TASKS: usize = 10000;
    /// Configuration constant: min concurrent tasks
    pub const MIN_CONCURRENT_TASKS: usize = 1;

    /// Timeout limits
    pub const MAX_TIMEOUT: Duration = Duration::from_secs(3600); // 1 hour
    /// Configuration constant: min timeout
    pub const MIN_TIMEOUT: Duration = Duration::from_millis(1);
    /// Configuration constant: max retry attempts
    pub const MAX_RETRY_ATTEMPTS: u32 = 100;
    /// Configuration constant: min retry attempts
    pub const MIN_RETRY_ATTEMPTS: u32 = 0;

    /// File system limits
    pub const MAX_FILE_SIZE: u64 = 1024 * 1024 * 1024; // 1GB
    /// Configuration constant: max log file size
    pub const MAX_LOG_FILE_SIZE: u64 = 100 * 1024 * 1024; // 100MB
    /// Configuration constant: max log files
    pub const MAX_LOG_FILES: u32 = 10;
    /// Configuration constant: max path length
    pub const MAX_PATH_LENGTH: usize = 4096;
    /// Configuration constant: max filename length
    pub const MAX_FILENAME_LENGTH: usize = 255;

    /// String and data limits
    pub const MAX_STRING_LENGTH: usize = 65536;
    /// Configuration constant: max message size
    pub const MAX_MESSAGE_SIZE: usize = 16 * 1024 * 1024; // 16MB
    /// Configuration constant: max batch size
    pub const MAX_BATCH_SIZE: usize = 1000;
    /// Configuration constant: min batch size
    pub const MIN_BATCH_SIZE: usize = 1;
}

/// **COMPILE-TIME CONSTANTS** - Constants evaluated at compile time
pub mod compile_time {
    /// Hash and cryptographic defaults
    pub const DEFAULT_HASH_ROUNDS: u32 = 12;
    /// Configuration constant: max hash rounds
    pub const MAX_HASH_ROUNDS: u32 = 31;
    /// Configuration constant: min hash rounds
    pub const MIN_HASH_ROUNDS: u32 = 4;

    /// Pre-computed table sizes
    pub const CRC32_TABLE_SIZE: usize = 256;
    /// Configuration constant: sine table size
    pub const SINE_TABLE_SIZE: usize = 360;
    /// Configuration constant: small primes count
    pub const SMALL_PRIMES_COUNT: usize = 168;
    /// Configuration constant: max small prime
    pub const MAX_SMALL_PRIME: u16 = 1000;

    /// Optimization constants
    pub const ENABLE_LOGGING: bool = true;
    /// Configuration constant: enable metrics
    pub const ENABLE_METRICS: bool = true;
    /// Configuration constant: enable tracing
    pub const ENABLE_TRACING: bool = false;
    /// Configuration constant: enable profiling
    pub const ENABLE_PROFILING: bool = false;

    /// Build-time feature flags
    pub const ENABLE_HSM_SUPPORT: bool = true;
    /// Configuration constant: enable genetics
    pub const ENABLE_GENETICS: bool = true;
    /// Enable workflow processing capabilities
    pub const ENABLE_WORKFLOWS: bool = true;
    /// Enable comprehensive system monitoring
    pub const ENABLE_MONITORING: bool = true;
}

/// Throughput, latency, and resource targets for runtime tuning.
pub mod performance {
    use super::Duration;

    /// CPU and processing
    pub const DEFAULT_CPU_CORES: usize = 4;
    /// Upper bound CPU utilization before throttling or alerts (%)
    pub const MAX_CPU_UTILIZATION: f64 = 90.0;
    /// Desired steady-state CPU utilization for schedulers (%)
    pub const TARGET_CPU_UTILIZATION: f64 = 70.0;
    /// Interval between CPU metric samples
    pub const CPU_SAMPLING_INTERVAL: Duration = Duration::from_secs(1);

    /// Memory optimization
    pub const MEMORY_PRESSURE_THRESHOLD: f64 = 85.0;
    /// Garbage collection pressure threshold
    pub const GC_PRESSURE_THRESHOLD: f64 = 80.0;
    /// Interval between memory pressure samples
    pub const MEMORY_SAMPLING_INTERVAL: Duration = Duration::from_secs(5);

    /// I/O optimization
    pub const IO_BUFFER_SIZE: usize = 8192;
    /// Read-ahead buffer size for sequential I/O
    pub const READ_AHEAD_SIZE: usize = 65536;
    /// Write buffer size before flush
    pub const WRITE_BUFFER_SIZE: usize = 65536;
    /// How often to flush buffered I/O
    pub const FLUSH_INTERVAL: Duration = Duration::from_secs(1);

    /// Default TCP_NODELAY (disable Nagle)
    pub const TCP_NODELAY: bool = true;
    /// Default SO_KEEPALIVE
    pub const TCP_KEEPALIVE: bool = true;
    /// Default socket send/receive buffer size
    pub const SOCKET_BUFFER_SIZE: usize = 65536;
    /// Typical Ethernet MTU-sized upper bound for packet sizing
    pub const MAX_PACKET_SIZE: usize = 1500;

    /// Concurrency optimization
    pub const WORK_STEALING_ENABLED: bool = true;
    /// Number of async runtime threads
    pub const ASYNC_RUNTIME_THREADS: usize = 4;
    /// Size of the blocking task thread pool
    pub const BLOCKING_POOL_SIZE: usize = 512;

    /// Target cache hit ratio for admission/eviction policies
    pub const CACHE_HIT_RATIO_TARGET: f64 = 0.95;
    /// Cache eviction threshold (90% full)
    pub const CACHE_EVICTION_THRESHOLD: f64 = 0.90;
    /// Default cache time-to-live duration
    pub const CACHE_TTL: Duration = Duration::from_secs(3600);
}

/// **SYSTEM TIMEOUTS** - Various timeout configurations
pub mod timeouts {
    use super::Duration;

    /// Connection timeouts
    pub const CONNECTION_TIMEOUT: Duration = Duration::from_secs(30);
    /// TLS handshake timeout duration
    pub const HANDSHAKE_TIMEOUT: Duration = Duration::from_secs(10);
    /// TCP or HTTP keep-alive timeout
    pub const KEEP_ALIVE_TIMEOUT: Duration = Duration::from_secs(60);
    /// Idle connection reclamation timeout
    pub const IDLE_CONNECTION_TIMEOUT: Duration = Duration::from_secs(300);

    /// Request/Response timeouts
    pub const REQUEST_TIMEOUT: Duration = Duration::from_secs(30);
    /// Standard API response timeout
    pub const RESPONSE_TIMEOUT: Duration = Duration::from_secs(30);
    /// Upper bound for long-running jobs (exports, migrations)
    pub const LONG_RUNNING_TIMEOUT: Duration = Duration::from_secs(300);
    /// Timeout for batched RPC or bulk operations
    pub const BATCH_TIMEOUT: Duration = Duration::from_secs(60);

    /// Health check timeouts
    pub const HEALTH_CHECK_TIMEOUT: Duration = Duration::from_secs(5);
    /// Kubernetes-style readiness probe budget
    pub const READINESS_TIMEOUT: Duration = Duration::from_secs(30);
    /// Kubernetes-style liveness probe budget
    pub const LIVENESS_TIMEOUT: Duration = Duration::from_secs(10);

    /// System operation timeouts
    pub const STARTUP_TIMEOUT: Duration = Duration::from_secs(120);
    /// Standard shutdown timeout duration
    pub const SHUTDOWN_TIMEOUT: Duration = Duration::from_secs(30);
    /// Graceful shutdown timeout allowing cleanup
    pub const GRACEFUL_SHUTDOWN_TIMEOUT: Duration = Duration::from_secs(60);
    /// Hard kill deadline after graceful shutdown stalls
    pub const FORCE_SHUTDOWN_TIMEOUT: Duration = Duration::from_secs(10);

    /// **LEGACY COMPATIBILITY FUNCTION** - For migration from deprecated unified constants
    #[must_use]
    #[expect(
        clippy::cast_possible_truncation,
        reason = "platform-specific size, checked at boundary"
    )]
    pub const fn default_timeout_ms() -> u64 {
        CONNECTION_TIMEOUT.as_millis() as u64
    }
}

/// **SYSTEM INTERVALS** - Regular operation intervals
pub mod intervals {
    use super::Duration;

    /// Monitoring intervals
    pub const METRICS_COLLECTION_INTERVAL: Duration = Duration::from_secs(15);
    /// Interval between health check executions
    pub const HEALTH_CHECK_INTERVAL: Duration = Duration::from_secs(30);
    /// How often log rotation is evaluated
    pub const LOG_ROTATION_INTERVAL: Duration = Duration::from_secs(3600);
    /// Background cleanup sweep interval
    pub const CLEANUP_INTERVAL: Duration = Duration::from_secs(300);

    /// Maintenance intervals
    pub const GC_INTERVAL: Duration = Duration::from_secs(60);
    /// Periodic cache eviction / TTL sweep
    pub const CACHE_CLEANUP_INTERVAL: Duration = Duration::from_secs(300);
    /// Default TTL for cached primal discovery results (when `DISCOVERY_CACHE_TTL_SECS` is unset).
    pub const PRIMAL_DISCOVERY_CACHE_TTL: Duration = Duration::from_secs(300);
    /// Stale connection pool purge interval
    pub const CONNECTION_CLEANUP_INTERVAL: Duration = Duration::from_secs(60);
    /// Temp directory scrub interval
    pub const TEMP_FILE_CLEANUP_INTERVAL: Duration = Duration::from_secs(3600);

    /// Backup and persistence intervals
    pub const CHECKPOINT_INTERVAL: Duration = Duration::from_secs(300);
    /// Scheduled backup cadence
    pub const BACKUP_INTERVAL: Duration = Duration::from_secs(3600);
    /// Cross-node or disk sync interval
    pub const SYNC_INTERVAL: Duration = Duration::from_secs(30);

    /// Heartbeat and keep-alive intervals
    pub const HEARTBEAT_INTERVAL: Duration = Duration::from_secs(30);
    /// Liveness ping interval between peers
    pub const PING_INTERVAL: Duration = Duration::from_secs(60);
    /// TCP or application keep-alive interval
    pub const KEEP_ALIVE_INTERVAL: Duration = Duration::from_secs(30);
}

/// **ERROR HANDLING CONSTANTS** - Error management configuration
pub mod errors {
    /// Maximum error message length
    pub const MAX_ERROR_MESSAGE_LENGTH: usize = 1000;

    /// Maximum error context entries
    pub const MAX_ERROR_CONTEXT_ENTRIES: usize = 10;

    /// Error code prefix
    pub const ERROR_CODE_PREFIX: &str = "BEARDOG_";

    /// Error categories
    pub const ERROR_CATEGORY_SYSTEM: &str = "SYSTEM";
    /// Security-related error bucket
    pub const ERROR_CATEGORY_SECURITY: &str = "SECURITY";
    /// Network-related error bucket
    pub const ERROR_CATEGORY_NETWORK: &str = "NETWORK";
    /// Domain or business-rule error bucket
    pub const ERROR_CATEGORY_BUSINESS: &str = "BUSINESS";
    /// Input validation error bucket
    pub const ERROR_CATEGORY_VALIDATION: &str = "VALIDATION";
}

/// **ENVIRONMENT CONSTANTS** - Environment-specific configuration
pub mod environment {
    /// Environment types
    pub const ENV_DEVELOPMENT: &str = "development";
    /// Automated test environment label
    pub const ENV_TESTING: &str = "testing";
    /// Pre-production staging label
    pub const ENV_STAGING: &str = "staging";
    /// Production deployment label
    pub const ENV_PRODUCTION: &str = "production";

    /// Environment variable prefixes
    pub const ENV_PREFIX: &str = "BEARDOG_";
    /// Prefix for layered config overrides from the environment
    pub const CONFIG_ENV_PREFIX: &str = "BEARDOG_CONFIG_";
    /// Prefix for secret material injected via environment
    pub const SECRET_ENV_PREFIX: &str = "BEARDOG_SECRET_";

    /// Configuration file names
    pub const CONFIG_FILE_NAME: &str = "beardog-config.toml";
    /// Default filename for secret key material
    pub const SECRETS_FILE_NAME: &str = "beardog-secrets.toml";
    /// Single-instance lock file
    pub const LOCK_FILE_NAME: &str = "beardog.lock";
    /// PID file for service managers
    pub const PID_FILE_NAME: &str = "beardog.pid";
}

/// **APPLICATION CONSTANTS** - Application-level constants
pub mod application {
    /// Application identity
    pub const APP_NAME: &str = "BearDog";
    /// Short product blurb for banners and `--version` output
    pub const APP_DESCRIPTION: &str = "Enterprise-grade security provider for ecoPrimals ecosystem";
    /// Owning organization name
    pub const ORGANIZATION: &str = "ecoPrimals";
    /// Copyright string for binaries and docs
    pub const COPYRIGHT: &str = "Copyright (c) 2024 ecoPrimals";
    /// SPDX license identifier
    pub const LICENSE: &str = "AGPL-3.0";

    /// Application metadata
    pub const USER_AGENT: &str = concat!("BearDog/", env!("CARGO_PKG_VERSION"));
    /// HTTP server header value
    pub const SERVER_HEADER: &str = concat!("BearDog/", env!("CARGO_PKG_VERSION"));

    /// Feature flags
    pub const FEATURE_HSM_SUPPORT: bool = true;
    /// Enable genetic algorithm features
    pub const FEATURE_GENETICS: bool = true;
    /// Enable workflow processing features
    pub const FEATURE_WORKFLOWS: bool = true;
    /// Enable monitoring features
    pub const FEATURE_MONITORING: bool = true;
    /// Enable compliance checking features
    pub const FEATURE_COMPLIANCE: bool = true;
}

// Re-export commonly used constants for convenience
pub use defaults::{DEFAULT_CONNECTION_TIMEOUT, DEFAULT_MAX_CONNECTIONS};
pub use intervals::{HEALTH_CHECK_INTERVAL, METRICS_COLLECTION_INTERVAL};
pub use limits::{MAX_CONNECTIONS, MAX_MEMORY_USAGE};
pub use timeouts::{CONNECTION_TIMEOUT, REQUEST_TIMEOUT};
pub use versions::BEARDOG_VERSION;

/// Aggregated snapshot of version, performance, and security defaults for tooling.
#[derive(Debug, Clone)]
pub struct ConstantRegistry {
    /// The version info value
    pub version_info: VersionInfo,
    /// Derived thread, memory, and cache tuning hints
    pub performance_tuning: PerformanceTuning,
    /// Security settings
    /// The security settings value
    pub security_settings: SecuritySettings,
}

/// Build-time and release identity strings.
#[derive(Debug, Clone)]
pub struct VersionInfo {
    /// `BearDog` version
    /// The beardog version value
    pub beardog_version: String,
    /// Build timestamp
    pub build_timestamp: String,
    /// Git commit hash
    /// Optional git commit
    pub git_commit: Option<String>,
}

/// Suggested runtime sizing derived from [`limits`] and [`defaults`].
#[derive(Debug, Clone)]
pub struct PerformanceTuning {
    /// Number of worker threads
    /// Number of `worker_threads`
    pub worker_threads: usize,
    /// Maximum memory in MB
    /// Number of `max_memory_mb`
    pub max_memory_mb: usize,
    /// Cache size in MB
    /// Number of `cache_size_mb`
    pub cache_size_mb: usize,
}

/// Security settings configuration
#[derive(Debug, Clone)]
pub struct SecuritySettings {
    /// Maximum idle time before session invalidation
    pub session_timeout: Duration,
    /// Maximum login attempts
    /// Number of `max_login_attempts`
    pub max_login_attempts: u32,
    /// Audit retention in days
    /// Number of `audit_retention_days`
    pub audit_retention_days: u32,
}

impl Default for ConstantRegistry {
    fn default() -> Self {
        Self {
            version_info: VersionInfo {
                beardog_version: versions::BEARDOG_VERSION.to_string(),
                build_timestamp: std::env::var("BUILD_TIMESTAMP")
                    .unwrap_or_else(|_| "unknown".to_string()),
                git_commit: std::env::var("GIT_COMMIT").ok(),
            },
            performance_tuning: PerformanceTuning {
                worker_threads: defaults::DEFAULT_THREAD_POOL_SIZE,
                max_memory_mb: limits::MAX_MEMORY_USAGE / (1024 * 1024),
                cache_size_mb: defaults::DEFAULT_CACHE_SIZE / 1024,
            },
            security_settings: SecuritySettings {
                session_timeout: timeouts::CONNECTION_TIMEOUT,
                max_login_attempts: 3,
                audit_retention_days: 90,
            },
        }
    }
}

/// Re-exports for cache sizing defaults used by storage layers.
pub mod cache {
    /// Cache-related constants
    pub use super::defaults::{DEFAULT_CACHE_SIZE, DEFAULT_POOL_SIZE};
}

/// Workflow module
pub mod workflow {
    pub use super::defaults::DEFAULT_TASK_TIMEOUT;
    pub use super::timeouts::LONG_RUNNING_TIMEOUT;
}

/// Genetics module
pub mod genetics {
    pub use super::defaults::DEFAULT_MAX_CONCURRENT_TASKS;
}

/// Testing module
pub mod testing {
    pub use super::defaults::DEFAULT_MAX_RETRIES;
    pub use super::timeouts::HEALTH_CHECK_TIMEOUT;
}

/// States module
pub mod states {
    /// Entity is running and accepting work
    pub const ACTIVE: &str = "active";
    /// Entity is stopped or disabled
    pub const INACTIVE: &str = "inactive";
    /// Entity is scheduled but not yet active
    pub const PENDING: &str = "pending";
    /// Entity terminated after an error
    pub const FAILED: &str = "failed ";
}

#[cfg(test)]
mod tests {
    use super::ConstantRegistry;
    use super::application;
    use super::compile_time;
    use super::defaults;
    use super::environment;
    use super::errors;
    use super::intervals;
    use super::limits;
    use super::performance;
    use super::timeouts;
    use super::versions;
    use std::time::Duration;

    fn assert_nonempty(s: &str) {
        assert!(!s.trim().is_empty(), "expected non-empty string");
    }

    #[test]
    fn version_strings_non_empty() {
        assert_nonempty(versions::BEARDOG_VERSION);
        assert_nonempty(versions::WORKFLOW_SYSTEM_VERSION);
        assert_nonempty(versions::PROTOCOL_VERSION);
        assert_nonempty(versions::BUILD_TARGET);
        assert_nonempty(versions::BUILD_OS);
        assert_nonempty(versions::BUILD_FAMILY);
    }

    #[test]
    fn defaults_and_limits_sane() {
        assert!(defaults::DEFAULT_BUFFER_SIZE > 0);
        assert!(limits::MAX_MEMORY_USAGE >= limits::MIN_BUFFER_SIZE);
        assert!(timeouts::CONNECTION_TIMEOUT > Duration::ZERO);
        assert!(intervals::HEALTH_CHECK_INTERVAL > Duration::ZERO);
    }

    #[test]
    fn errors_and_environment_prefixes() {
        assert!(errors::ERROR_CODE_PREFIX.contains("BEARDOG"));
        assert!(environment::ENV_PREFIX.starts_with("BEARDOG"));
        assert_nonempty(environment::CONFIG_FILE_NAME);
    }

    #[test]
    fn application_metadata() {
        assert_nonempty(application::APP_NAME);
        assert_nonempty(application::USER_AGENT);
        assert!(application::FEATURE_HSM_SUPPORT);
    }

    #[test]
    fn compile_time_and_performance_flags() {
        assert!(compile_time::CRC32_TABLE_SIZE > 0);
        assert!(performance::IO_BUFFER_SIZE > 0);
        assert!(performance::MAX_CPU_UTILIZATION > performance::TARGET_CPU_UTILIZATION);
    }

    #[test]
    fn timeouts_helper_default_timeout_ms() {
        let ms = timeouts::default_timeout_ms();
        assert!(ms > 0);
    }

    #[test]
    fn constant_registry_default() {
        let r = ConstantRegistry::default();
        assert!(!r.version_info.beardog_version.is_empty());
        assert!(r.performance_tuning.worker_threads > 0);
        assert!(r.security_settings.max_login_attempts > 0);
    }

    #[test]
    fn states_and_workflow_reexports() {
        assert_nonempty(super::states::ACTIVE);
        assert!(super::workflow::DEFAULT_TASK_TIMEOUT > Duration::ZERO);
    }
}
