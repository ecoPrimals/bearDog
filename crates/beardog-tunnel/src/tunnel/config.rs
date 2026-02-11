// Configuration types for tunnel module

//! # Tunnel Configuration Types
//!
//! This module provides granular configuration for different aspects of tunnel operation:
//!
//! - **Performance**: Latency, throughput, resource limits
//! - **Security**: Key storage, escrow thresholds
//! - **Gaming**: Anti-cheat, latency requirements
//! - **Resilience**: Circuit breakers, retries, health checks
//! - **Monitoring**: Metrics, tracing, alerting
//!
//! ## Philosophy
//!
//! Configuration is zero-hardcoded: all values can be overridden via environment
//! variables or runtime configuration files. Sensible defaults are provided for
//! immediate use.
//!
//! ## Example
//!
//! ```rust
//! use beardog_tunnel::tunnel::config::TunnelConfig;
//!
//! // Use defaults
//! let config = TunnelConfig::default();
//!
//! // Customize for specific needs
//! let mut gaming_config = TunnelConfig::default();
//! gaming_config.performance.max_decryption_latency = std::time::Duration::from_micros(50);
//! gaming_config.gaming.max_latency_ms = 30;
//! ```

use serde::{Deserialize, Serialize};
use std::time::Duration;

/// Performance configuration for tunnel operations
///
/// Controls latency, throughput, resource limits, and performance optimizations.
///
/// # Key Settings
///
/// - **Latency**: `max_decryption_latency`, `max_session_setup_time`
/// - **Throughput**: `min_gaming_throughput`, `bandwidth_limit_mbps`
/// - **Scaling**: `enable_auto_scaling`, `cpu_threshold`, `memory_threshold`
/// - **Caching**: `enable_caching`, `cache_size_mb`, `cache_ttl`
///
/// # Defaults
///
/// Optimized for production use with reasonable limits:
/// - Max decryption latency: 100 microseconds
/// - Session setup: 500 milliseconds
/// - Min throughput: 1 MB/s
/// - 100 concurrent sessions
///
/// # Example
///
/// ```rust
/// use beardog_tunnel::tunnel::config::PerformanceConfig;
/// use std::time::Duration;
///
/// let mut config = PerformanceConfig::default();
/// // Optimize for low latency
/// config.max_decryption_latency = Duration::from_micros(50);
/// config.max_session_setup_time = Duration::from_millis(100);
/// ```
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PerformanceConfig {
    /// The max decryption latency value
    pub max_decryption_latency: Duration,
    /// Maximum time allowed for session setup
    pub max_session_setup_time: Duration,
    /// Number of `min_gaming_throughput`
    pub min_gaming_throughput: u64,
    /// Whether `enable_monitoring` is enabled
    pub enable_monitoring: bool,
    /// The metrics interval value
    pub metrics_interval: Duration,
    /// Whether `enable_prediction` is enabled
    pub enable_prediction: bool,
    /// Number of `memory_limit_mb`
    pub memory_limit_mb: u64,
    /// Number of `max_concurrent_sessions`
    pub max_concurrent_sessions: u32,
    /// Whether `enable_auto_scaling` is enabled
    pub enable_auto_scaling: bool,
    /// The cpu threshold value
    pub cpu_threshold: f64,
    /// The memory threshold value
    pub memory_threshold: f64,
    /// Maximum bandwidth limit in megabits per second
    pub bandwidth_limit_mbps: u64,
    /// Whether `enable_compression` is enabled
    pub enable_compression: bool,
    /// Number of `compression_level`
    pub compression_level: u32,
    /// Whether `enable_caching` is enabled
    pub enable_caching: bool,
    /// Number of `cache_size_mb`
    pub cache_size_mb: u64,
    /// The cache ttl value
    pub cache_ttl: Duration,
}

impl Default for PerformanceConfig {
    fn default() -> Self {
        Self {
            max_decryption_latency: Duration::from_micros(100),
            max_session_setup_time: Duration::from_millis(500),
            min_gaming_throughput: 1_000_000, // 1 MB/s
            enable_monitoring: true,
            metrics_interval: Duration::from_secs(60),
            enable_prediction: false,
            memory_limit_mb: 512,
            max_concurrent_sessions: 100,
            enable_auto_scaling: true,
            cpu_threshold: 80.0,
            memory_threshold: 85.0,
            bandwidth_limit_mbps: 1000,
            enable_compression: true,
            compression_level: 6,
            enable_caching: true,
            cache_size_mb: 128,
            cache_ttl: Duration::from_secs(300),
        }
    }
}

/// Security configuration for key management and storage
///
/// Controls how cryptographic keys are stored and protected.
///
/// # Fields
///
/// - `key_storage_path`: Filesystem path for key material (should be secure mount)
/// - `key_escrow_threshold`: Number of key shares required for escrow recovery
///
/// # Security Notes
///
/// - Key storage path should be on encrypted filesystem
/// - Escrow threshold implements Shamir's Secret Sharing
/// - Keys should never be stored in plaintext
///
/// # Example
///
/// ```rust
/// use beardog_tunnel::tunnel::config::SecurityConfig;
///
/// let mut config = SecurityConfig::default();
/// config.key_storage_path = "/mnt/secure/keys".to_string();
/// config.key_escrow_threshold = 5; // 5-of-N threshold
/// ```
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SecurityConfig {
    /// The key storage path value
    pub key_storage_path: String,
    /// Number of `key_escrow_threshold`
    pub key_escrow_threshold: u32,
}

impl Default for SecurityConfig {
    fn default() -> Self {
        Self {
            key_storage_path: String::from("/secure/keys"),
            key_escrow_threshold: 10,
        }
    }
}

/// Gaming-specific configuration
///
/// Optimizes tunnel behavior for real-time gaming use cases.
///
/// # Fields
///
/// - `anti_cheat_provider`: Name of anti-cheat service (discovery-based)
/// - `max_latency_ms`: Maximum acceptable latency for gaming traffic
///
/// # Use Case
///
/// - Multiplayer games requiring low latency
/// - Real-time competitive scenarios
/// - Latency-sensitive applications
///
/// # Example
///
/// ```rust
/// use beardog_tunnel::tunnel::config::GamingConfig;
///
/// let mut config = GamingConfig::default();
/// config.max_latency_ms = 30; // 30ms for competitive gaming
/// ```
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct GamingConfig {
    /// Anti-cheat provider name (resolved via capability discovery)
    pub anti_cheat_provider: String,
    /// Number of `max_latency_ms`
    pub max_latency_ms: u32,
}

impl Default for GamingConfig {
    fn default() -> Self {
        Self {
            anti_cheat_provider: String::from("default"),
            max_latency_ms: 50,
        }
    }
}

/// Resilience configuration for fault tolerance
///
/// Controls circuit breakers, retries, health checks, and recovery mechanisms.
///
/// # Resilience Patterns
///
/// - **Circuit Breaker**: Prevents cascade failures
/// - **Retries**: Automatic retry with exponential backoff
/// - **Health Checks**: Proactive failure detection
/// - **Graceful Shutdown**: Clean resource cleanup
/// - **Auto Recovery**: Self-healing after failures
/// - **Backup Systems**: Redundancy for critical paths
///
/// # Example
///
/// ```rust
/// use beardog_tunnel::tunnel::config::ResilienceConfig;
/// use std::time::Duration;
///
/// let mut config = ResilienceConfig::default();
/// // More aggressive circuit breaker
/// config.failure_threshold = 3;
/// config.recovery_timeout = Duration::from_secs(15);
/// ```
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ResilienceConfig {
    /// Whether `enable_circuit_breaker` is enabled
    pub enable_circuit_breaker: bool,
    /// Number of `failure_threshold`
    pub failure_threshold: u32,
    /// Timeout before recovery attempt after circuit breaker trips
    pub recovery_timeout: Duration,
    /// Number of `max_retries`
    pub max_retries: u32,
    /// The retry delay value
    pub retry_delay: Duration,
    /// Whether `enable_health_checks` is enabled
    pub enable_health_checks: bool,
    /// The health check interval value
    pub health_check_interval: Duration,
    /// Timeout for individual health check probes
    pub health_check_timeout: Duration,
    /// Whether `enable_graceful_shutdown` is enabled
    pub enable_graceful_shutdown: bool,
    /// Maximum time to wait for graceful shutdown
    pub shutdown_timeout: Duration,
    /// Whether `enable_auto_recovery` is enabled
    pub enable_auto_recovery: bool,
    /// The recovery delay value
    pub recovery_delay: Duration,
    /// Number of `max_recovery_attempts`
    pub max_recovery_attempts: u32,
    /// Whether `enable_backup_systems` is enabled
    pub enable_backup_systems: bool,
    /// The backup sync interval value
    pub backup_sync_interval: Duration,
    /// Whether `enable_disaster_recovery` is enabled
    pub enable_disaster_recovery: bool,
    /// Timeout for disaster recovery operations
    pub disaster_recovery_timeout: Duration,
    /// Whether `enable_self_healing` is enabled
    pub enable_self_healing: bool,
    /// Maximum time for self-healing operations
    pub healing_timeout: Duration,
}

impl Default for ResilienceConfig {
    fn default() -> Self {
        Self {
            enable_circuit_breaker: true,
            failure_threshold: 5,
            recovery_timeout: Duration::from_secs(30),
            max_retries: 3,
            retry_delay: Duration::from_millis(100),
            enable_health_checks: true,
            health_check_interval: Duration::from_secs(30),
            health_check_timeout: Duration::from_secs(5),
            enable_graceful_shutdown: true,
            shutdown_timeout: Duration::from_secs(30),
            enable_auto_recovery: true,
            recovery_delay: Duration::from_secs(10),
            max_recovery_attempts: 5,
            enable_backup_systems: false,
            backup_sync_interval: Duration::from_secs(300),
            enable_disaster_recovery: false,
            disaster_recovery_timeout: Duration::from_secs(600),
            enable_self_healing: true,
            healing_timeout: Duration::from_secs(60),
        }
    }
}

/// Tunnel-specific monitoring configuration
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct TunnelMonitoringConfig {
    /// Whether metrics collection is enabled
    pub enable_metrics: bool,
    /// Port for metrics endpoint
    pub metrics_port: u16,
    /// Whether distributed tracing is enabled
    pub enable_tracing: bool,
    /// Trace sampling rate (0.0 to 1.0)
    pub trace_sample_rate: f64,
    /// Whether structured logging is enabled
    pub enable_logging: bool,
    /// Log level filter (e.g. "info", "debug", "trace")
    pub log_level: String,
    /// Whether the health check endpoint is exposed
    pub enable_health_endpoint: bool,
    /// Port for health check endpoint
    pub health_endpoint_port: u16,
    /// Whether runtime profiling is enabled
    pub enable_profiling: bool,
    /// Port for profiling endpoint
    pub profiling_port: u16,
    /// Whether alerting is enabled
    pub enable_alerts: bool,
    /// Threshold configuration for alerts
    pub alert_thresholds: AlertThresholds,
    /// Window duration for metric aggregation
    pub aggregation_window: Duration,
}

/// Backward compatibility alias for [`TunnelMonitoringConfig`]
#[deprecated(since = "3.1.0", note = "Use TunnelMonitoringConfig instead")]
pub type MonitoringConfig = TunnelMonitoringConfig;

/// Threshold values that trigger monitoring alerts
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct AlertThresholds {
    /// The cpu threshold value
    pub cpu_threshold: f64,
    /// The memory threshold value
    pub memory_threshold: f64,
    /// The error rate threshold value
    pub error_rate_threshold: f64,
}

impl Default for AlertThresholds {
    fn default() -> Self {
        Self {
            cpu_threshold: 0.9,
            memory_threshold: 0.85,
            error_rate_threshold: 0.05,
        }
    }
}

impl Default for TunnelMonitoringConfig {
    fn default() -> Self {
        // ✅ MIGRATED: Load ports from global configuration instead of hardcoded values
        // This allows runtime configuration via environment variables
        use beardog_config::global::BEARDOG_CONFIG;

        Self {
            enable_metrics: true,
            metrics_port: BEARDOG_CONFIG.network.discovery.port, // Was: 9090
            enable_tracing: true,
            trace_sample_rate: 0.1,
            enable_logging: true,
            log_level: "info".to_string(),
            enable_health_endpoint: true,
            health_endpoint_port: BEARDOG_CONFIG.network.api.port, // Was: 8080
            enable_profiling: false,
            profiling_port: std::env::var("BEARDOG_PROFILING_PORT")
                .ok()
                .and_then(|s| s.parse().ok())
                .unwrap_or(6060), // Was: 6060 (no standard config for profiling yet)
            enable_alerts: true,
            alert_thresholds: AlertThresholds::default(),
            aggregation_window: Duration::from_secs(300),
        }
    }
}

/// Unified key management processor configuration
///
/// Controls cryptographic key lifecycle including generation, derivation,
/// and rotation policies.
///
/// # Key Settings
///
/// - `session_key_length`: Symmetric key length in bytes (32 for AES-256)
/// - `key_derivation_rounds`: PBKDF2/Argon2 iteration count
/// - `key_rotation_interval`: Automatic rotation period
/// - `use_hardware_keys`: Enable HSM/TPM for key protection
///
/// # Security Considerations
///
/// - Longer keys = stronger security, higher latency
/// - More derivation rounds = slower but more resistant to brute force
/// - Shorter rotation intervals = better forward secrecy
/// - Hardware keys prevent key extraction attacks
///
/// # Example
///
/// ```rust
/// use beardog_tunnel::tunnel::config::UnifiedProcessorConfig;
/// use std::time::Duration;
///
/// let mut config = UnifiedProcessorConfig::default();
/// // Maximum security
/// config.session_key_length = 64; // 512-bit keys
/// config.key_derivation_rounds = 100_000;
/// config.key_rotation_interval = Duration::from_secs(1800); // 30 min
/// ```
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct UnifiedProcessorConfig {
    /// Number of `session_key_length`
    pub session_key_length: usize,
    /// Number of `key_derivation_rounds`
    pub key_derivation_rounds: u32,
    /// The key rotation interval value
    pub key_rotation_interval: Duration,
    /// Whether `use_hardware_keys` is enabled
    pub use_hardware_keys: bool,
}

impl Default for UnifiedProcessorConfig {
    fn default() -> Self {
        Self {
            session_key_length: 32,
            key_derivation_rounds: 10000,
            key_rotation_interval: Duration::from_secs(3600), // 1 hour
            use_hardware_keys: true,
        }
    }
}

/// Complete tunnel configuration
///
/// Top-level configuration struct that aggregates all tunnel settings.
///
/// # Components
///
/// - `performance`: Latency, throughput, resource limits
/// - `security`: Key storage and escrow
/// - `gaming`: Gaming-specific optimizations
/// - `resilience`: Fault tolerance and recovery
/// - `monitoring`: Observability and alerting
///
/// # Usage
///
/// Typically loaded from configuration files or environment variables,
/// with sane defaults for immediate use.
///
/// # Example
///
/// ```rust
/// use beardog_tunnel::tunnel::config::TunnelConfig;
///
/// // Start with defaults
/// let config = TunnelConfig::default();
///
/// // Verify default settings
/// assert!(config.performance.enable_monitoring);
/// assert!(config.resilience.enable_circuit_breaker);
/// assert!(config.monitoring.enable_metrics);
/// ```
#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct TunnelConfig {
    /// Performance and resource configuration
    pub performance: PerformanceConfig,
    /// The security value
    pub security: SecurityConfig,
    /// The gaming value
    pub gaming: GamingConfig,
    /// The resilience value
    pub resilience: ResilienceConfig,
    /// The monitoring value
    pub monitoring: TunnelMonitoringConfig,
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_config_serialization() -> Result<(), Box<dyn std::error::Error>> {
        let config = TunnelConfig::default();
        let serialized = serde_json::to_string(&config).map_err(|e| {
            tracing::error!("JSON serialization failed ({}): {}", "Should serialize", e);
            e
        });
        assert!(serialized.is_ok());

        let deserialized: Result<TunnelConfig, _> =
            serde_json::from_str(&serialized?).map_err(|e| {
                tracing::error!("JSON parsing failed ({}): {}", "Should deserialize", e);
                e
            });
        assert!(deserialized.is_ok());
        Ok(())
    }

    #[test]
    fn test_performance_config_defaults() {
        let config = PerformanceConfig::default();
        assert_eq!(config.max_concurrent_sessions, 100);
        assert!(config.enable_monitoring);
        assert_eq!(config.compression_level, 6);
    }

    #[test]
    fn test_security_config_defaults() {
        let config = SecurityConfig::default();
        assert_eq!(config.key_storage_path, "/secure/keys");
        assert_eq!(config.key_escrow_threshold, 10);
    }

    #[test]
    fn test_gaming_config_defaults() {
        let config = GamingConfig::default();
        assert_eq!(config.max_latency_ms, 50);
        assert_eq!(config.anti_cheat_provider, "default");
    }

    #[test]
    fn test_resilience_config_defaults() {
        let config = ResilienceConfig::default();
        assert_eq!(config.max_retries, 3);
        assert_eq!(config.retry_delay, Duration::from_millis(100));
        assert!(config.enable_circuit_breaker);
        assert_eq!(config.failure_threshold, 5);
    }

    #[test]
    fn test_tunnel_monitoring_config_defaults() {
        let config = TunnelMonitoringConfig::default();
        assert!(config.enable_metrics);
        assert_eq!(
            config.metrics_port,
            beardog_config::domains::network_ports::DEFAULT_DISCOVERY_PORT
        );
        assert!(config.enable_tracing);
        assert!(config.enable_logging);
        assert_eq!(config.log_level, "info");
    }

    #[test]
    fn test_unified_processor_config_defaults() {
        let config = UnifiedProcessorConfig::default();
        assert_eq!(config.session_key_length, 32);
        assert_eq!(config.key_derivation_rounds, 10_000);
        assert_eq!(config.key_rotation_interval, Duration::from_secs(3600));
        assert!(config.use_hardware_keys);
    }

    #[test]
    fn test_tunnel_config_contains_all_subconfigs() {
        let config = TunnelConfig::default();

        // Verify all subconfigs are present and have defaults
        assert_eq!(config.performance.max_concurrent_sessions, 100);
        assert_eq!(config.security.key_storage_path, "/secure/keys");
        assert_eq!(config.gaming.max_latency_ms, 50);
        assert_eq!(config.resilience.max_retries, 3);
        assert!(config.monitoring.enable_metrics);
    }

    #[test]
    fn test_performance_config_monitoring_enabled_by_default() {
        let config = PerformanceConfig::default();
        assert!(config.enable_monitoring);
        assert!(config.enable_caching);
    }

    #[test]
    fn test_performance_config_thresholds() {
        let config = PerformanceConfig::default();
        // Verify thresholds are reasonable percentages (0-100)
        assert!(config.cpu_threshold >= 0.0 && config.cpu_threshold <= 100.0);
        assert!(config.memory_threshold >= 0.0 && config.memory_threshold <= 100.0);
        assert_eq!(config.cpu_threshold, 80.0);
        assert_eq!(config.memory_threshold, 85.0);
    }

    #[test]
    fn test_config_clone_and_equality() {
        let config1 = TunnelConfig::default();
        let config2 = config1.clone();

        // Verify cloning works correctly
        assert_eq!(
            config1.performance.max_concurrent_sessions,
            config2.performance.max_concurrent_sessions
        );
        assert_eq!(
            config1.security.key_storage_path,
            config2.security.key_storage_path
        );
    }
}
