// Configuration types for tunnel module

// Module documentation
//
// This module provides functionality for the BearDog ecosystem.

use serde::{Deserialize, Serialize};
use std::time::Duration;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PerformanceConfig {
    /// The max decryption latency value
    pub max_decryption_latency: Duration,
    pub max_session_setup_time: Duration,
    /// Number of min_gaming_throughput
    pub min_gaming_throughput: u64,
    /// Whether enable_monitoring is enabled
    pub enable_monitoring: bool,
    /// The metrics interval value
    pub metrics_interval: Duration,
    /// Whether enable_prediction is enabled
    pub enable_prediction: bool,
    /// Number of memory_limit_mb
    pub memory_limit_mb: u64,
    /// Number of max_concurrent_sessions
    pub max_concurrent_sessions: u32,
    /// Whether enable_auto_scaling is enabled
    pub enable_auto_scaling: bool,
    /// The cpu threshold value
    pub cpu_threshold: f64,
    /// The memory threshold value
    pub memory_threshold: f64,
    pub bandwidth_limit_mbps: u64,
    /// Whether enable_compression is enabled
    pub enable_compression: bool,
    /// Number of compression_level
    pub compression_level: u32,
    /// Whether enable_caching is enabled
    pub enable_caching: bool,
    /// Number of cache_size_mb
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

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SecurityConfig {
    /// The key storage path value
    pub key_storage_path: String,
    /// Number of key_escrow_threshold
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

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct GamingConfig {
    pub anti_cheat_provider: String,
    /// Number of max_latency_ms
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

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ResilienceConfig {
    /// Whether enable_circuit_breaker is enabled
    pub enable_circuit_breaker: bool,
    /// Number of failure_threshold
    pub failure_threshold: u32,
    pub recovery_timeout: Duration,
    /// Number of max_retries
    pub max_retries: u32,
    /// The retry delay value
    pub retry_delay: Duration,
    /// Whether enable_health_checks is enabled
    pub enable_health_checks: bool,
    /// The health check interval value
    pub health_check_interval: Duration,
    pub health_check_timeout: Duration,
    /// Whether enable_graceful_shutdown is enabled
    pub enable_graceful_shutdown: bool,
    pub shutdown_timeout: Duration,
    /// Whether enable_auto_recovery is enabled
    pub enable_auto_recovery: bool,
    /// The recovery delay value
    pub recovery_delay: Duration,
    /// Number of max_recovery_attempts
    pub max_recovery_attempts: u32,
    /// Whether enable_backup_systems is enabled
    pub enable_backup_systems: bool,
    /// The backup sync interval value
    pub backup_sync_interval: Duration,
    /// Whether enable_disaster_recovery is enabled
    pub enable_disaster_recovery: bool,
    pub disaster_recovery_timeout: Duration,
    /// Whether enable_self_healing is enabled
    pub enable_self_healing: bool,
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

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct MonitoringConfig {
    /// Whether enable_metrics is enabled
    pub enable_metrics: bool,
    /// Number of metrics_port
    pub metrics_port: u16,
    /// Whether enable_tracing is enabled
    pub enable_tracing: bool,
    /// The trace sample rate value
    pub trace_sample_rate: f64,
    /// Whether enable_logging is enabled
    pub enable_logging: bool,
    /// The log level value
    pub log_level: String,
    /// Whether enable_health_endpoint is enabled
    pub enable_health_endpoint: bool,
    /// Number of health_endpoint_port
    pub health_endpoint_port: u16,
    /// Whether enable_profiling is enabled
    pub enable_profiling: bool,
    /// Number of profiling_port
    pub profiling_port: u16,
    /// Whether enable_alerts is enabled
    pub enable_alerts: bool,
    /// The alert thresholds value
    pub alert_thresholds: AlertThresholds,
    /// The aggregation window value
    pub aggregation_window: Duration,
}

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

impl Default for MonitoringConfig {
    fn default() -> Self {
        Self {
            enable_metrics: true,
            metrics_port: 9090,
            enable_tracing: true,
            trace_sample_rate: 0.1,
            enable_logging: true,
            log_level: "info".to_string(),
            enable_health_endpoint: true,
            health_endpoint_port: 8080,
            enable_profiling: false,
            profiling_port: 6060,
            enable_alerts: true,
            alert_thresholds: AlertThresholds::default(),
            aggregation_window: Duration::from_secs(300),
        }
    }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct UnifiedProcessorConfig {
    /// Number of session_key_length
    pub session_key_length: usize,
    /// Number of key_derivation_rounds
    pub key_derivation_rounds: u32,
    /// The key rotation interval value
    pub key_rotation_interval: Duration,
    /// Whether use_hardware_keys is enabled
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

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct TunnelConfig {
    pub performance: PerformanceConfig,
    /// The security value
    pub security: SecurityConfig,
    /// The gaming value
    pub gaming: GamingConfig,
    /// The resilience value
    pub resilience: ResilienceConfig,
    /// The monitoring value
    pub monitoring: MonitoringConfig,
}

impl Default for TunnelConfig {
    fn default() -> Self {
        Self {
            performance: PerformanceConfig::default(),
            security: SecurityConfig::default(),
            gaming: GamingConfig::default(),
            resilience: ResilienceConfig::default(),
            monitoring: MonitoringConfig::default(),
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_config_serialization() {
        let config = TunnelConfig::default();
        let serialized = serde_json::to_string(&config).map_err(|e| {
            tracing::error!("JSON serialization failed ({}): {}", "Should serialize", e);
            e
        });
        assert!(serialized.is_ok());

        let deserialized: Result<TunnelConfig, _> = serde_json::from_str(&serialized.unwrap())
            .map_err(|e| {
                tracing::error!("JSON parsing failed ({}): {}", "Should deserialize", e);
                e
            });
        assert!(deserialized.is_ok());
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
}
