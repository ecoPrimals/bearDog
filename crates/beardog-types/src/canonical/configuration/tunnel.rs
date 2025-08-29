use serde::{Deserialize, Serialize};
use std::time::Duration;
use std::collections::HashMap;

/// Canonical tunnel configuration - consolidates all tunnel-related configs
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct TunnelConfig {
    pub performance: TunnelPerformanceConfig,
    pub key_management: TunnelKeyManagementConfig,
    pub gaming: TunnelGamingConfig,
    pub hsm_manager: TunnelHsmManagerConfig,
    pub monitoring: TunnelMonitoringConfig,
    pub security: TunnelSecurityConfig,
}

impl Default for TunnelConfig {
    fn default() -> Self {
        Self {
            performance: TunnelPerformanceConfig::default(),
            key_management: TunnelKeyManagementConfig::default(),
            gaming: TunnelGamingConfig::default(),
            hsm_manager: TunnelHsmManagerConfig::default(),
            monitoring: TunnelMonitoringConfig::default(),
            security: TunnelSecurityConfig::default(),
        }
    }
}

/// Performance configuration for tunnel operations
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct TunnelPerformanceConfig {
    pub max_encryption_latency: Duration,
    pub max_decryption_latency: Duration,
    pub max_session_setup_time: Duration,
    pub min_gaming_throughput: u64,
    pub enable_monitoring: bool,
    pub metrics_interval: Duration,
    pub memory_limit_mb: u64,
    pub max_concurrent_sessions: u32,
    pub enable_auto_scaling: bool,
    pub cpu_threshold: f64,
    pub memory_threshold: f64,
    pub bandwidth_limit_mbps: u64,
    pub enable_compression: bool,
    pub compression_level: u32,
    pub enable_caching: bool,
    pub cache_size_mb: u64,
    pub cache_ttl: Duration,
}

impl Default for TunnelPerformanceConfig {
    fn default() -> Self {
        Self {
            max_encryption_latency: Duration::from_micros(100),
            max_decryption_latency: Duration::from_micros(100),
            max_session_setup_time: Duration::from_millis(10),
            min_gaming_throughput: 1_000_000_000, // 1 Gbps
            enable_monitoring: true,
            metrics_interval: Duration::from_millis(100),
            memory_limit_mb: 1024, // 1 GB
            max_concurrent_sessions: 100,
            enable_auto_scaling: true,
            cpu_threshold: 0.8,
            memory_threshold: 0.8,
            bandwidth_limit_mbps: 1_000_000, // 1 Gbps
            enable_compression: true,
            compression_level: 6,
            enable_caching: true,
            cache_size_mb: 1024, // 1 GB
            cache_ttl: Duration::from_secs(3600), // 1 hour
        }
    }
}

/// Key management configuration for tunnel operations
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct TunnelKeyManagementConfig {
    pub use_hardware_keys: bool,
    pub key_derivation_rounds: u32,
    pub session_key_length: u32,
    pub key_rotation_interval: Duration,
    pub backup_key_count: u32,
    pub key_storage_path: String,
    pub enable_key_escrow: bool,
    pub key_escrow_threshold: u32,
}

impl Default for TunnelKeyManagementConfig {
    fn default() -> Self {
        Self {
            use_hardware_keys: true,
            key_derivation_rounds: 100_000,
            session_key_length: 32, // 256 bits
            key_rotation_interval: Duration::from_secs(3600), // 1 hour
            backup_key_count: 3,
            key_storage_path: "/var/lib/beardog/tunnel/keys".to_string(),
            enable_key_escrow: false,
            key_escrow_threshold: 10,
        }
    }
}

/// Gaming-specific configuration for ultra-low latency
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct TunnelGamingConfig {
    pub profile_name: String,
    pub ultra_low_latency: bool,
    pub predictive_keying: bool,
    pub jitter_elimination: bool,
    pub bandwidth_optimization: bool,
    pub prefer_hardware_crypto: bool,
    pub enable_batch_processing: bool,
    pub target_fps: u32,
    pub max_input_lag_ms: u32,
    pub enable_traffic_priority: bool,
    pub traffic_priority_level: u32,
    pub enable_anti_cheat: bool,
    pub anti_cheat_provider: String,
    pub enable_state_sync: bool,
    pub state_sync_interval_ms: u32,
    pub enable_spectator_mode: bool,
    pub max_spectators: u32,
}

impl Default for TunnelGamingConfig {
    fn default() -> Self {
        Self {
            profile_name: "ultra_performance".to_string(),
            ultra_low_latency: true,
            predictive_keying: true,
            jitter_elimination: true,
            bandwidth_optimization: true,
            prefer_hardware_crypto: true,
            enable_batch_processing: false, // Prioritize latency
            target_fps: 240,
            max_input_lag_ms: 1,
            enable_traffic_priority: true,
            traffic_priority_level: 7,
            enable_anti_cheat: true,
            anti_cheat_provider: "beardog_native".to_string(),
            enable_state_sync: true,
            state_sync_interval_ms: 16, // 60 FPS
            enable_spectator_mode: false,
            max_spectators: 0,
        }
    }
}

/// HSM manager configuration
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct TunnelHsmManagerConfig {
    pub hsm_configs: Vec<TunnelHsmConfig>,
    pub health_config: TunnelHealthConfig,
    pub failover_config: TunnelFailoverConfig,
    pub performance_config: TunnelHsmPerformanceConfig,
}

impl Default for TunnelHsmManagerConfig {
    fn default() -> Self {
        Self {
            hsm_configs: vec![],
            health_config: TunnelHealthConfig::default(),
            failover_config: TunnelFailoverConfig::default(),
            performance_config: TunnelHsmPerformanceConfig::default(),
        }
    }
}

/// Health check configuration for tunnel HSM
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct TunnelHealthConfig {
    pub check_interval: Duration,
    pub failure_threshold: u32,
    pub recovery_threshold: u32,
    pub timeout: Duration,
}

impl Default for TunnelHealthConfig {
    fn default() -> Self {
        Self {
            check_interval: Duration::from_secs(30),
            failure_threshold: 3,
            recovery_threshold: 2,
            timeout: Duration::from_secs(5),
        }
    }
}

/// Failover configuration for tunnel operations
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct TunnelFailoverConfig {
    pub enabled: bool,
    pub max_retries: u32,
    pub retry_delay: Duration,
    pub circuit_breaker_threshold: u32,
    pub circuit_breaker_timeout: Duration,
}

impl Default for TunnelFailoverConfig {
    fn default() -> Self {
        Self {
            enabled: true,
            max_retries: 3,
            retry_delay: Duration::from_millis(500),
            circuit_breaker_threshold: 5,
            circuit_breaker_timeout: Duration::from_secs(60),
        }
    }
}

/// HSM-specific performance configuration
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct TunnelHsmPerformanceConfig {
    pub enable_load_balancing: bool,
    pub enable_caching: bool,
    pub max_concurrent_operations: u32,
    pub operation_timeout: Duration,
}

impl Default for TunnelHsmPerformanceConfig {
    fn default() -> Self {
        Self {
            enable_load_balancing: true,
            enable_caching: true,
            max_concurrent_operations: 100,
            operation_timeout: Duration::from_secs(10),
        }
    }
}

/// Individual HSM configuration
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct TunnelHsmConfig {
    pub tier: TunnelHsmTier,
    pub connection_config: TunnelConnectionConfig,
    pub auth_config: TunnelAuthConfig,
    pub vendor_specific: HashMap<String, String>,
}

/// HSM tier classification for tunnel operations
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub enum TunnelHsmTier {
    Smartphone,
    Software,
    Hardware,
    Hybrid,
}

impl std::fmt::Display for TunnelHsmTier {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            TunnelHsmTier::Smartphone => write!(f, "Smartphone"),
            TunnelHsmTier::Software => write!(f, "Software"),
            TunnelHsmTier::Hardware => write!(f, "Hardware"),
            TunnelHsmTier::Hybrid => write!(f, "Hybrid"),
        }
    }
}

/// Connection configuration for tunnel HSM
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct TunnelConnectionConfig {
    pub endpoint: String,
    pub timeout: Duration,
    pub max_retries: u32,
    pub use_tls: bool,
}

impl Default for TunnelConnectionConfig {
    fn default() -> Self {
        Self {
            endpoint: "localhost:8443".to_string(),
            timeout: Duration::from_secs(30),
            max_retries: 3,
            use_tls: true,
        }
    }
}

/// Authentication configuration for tunnel operations
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct TunnelAuthConfig {
    pub method: TunnelAuthMethod,
    pub credentials: HashMap<String, String>,
    pub session_timeout: Duration,
}

/// Authentication methods for tunnel operations
#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum TunnelAuthMethod {
    Certificate { cert_path: String, key_path: String },
    Token { token: String },
    ApiKey { key: String },
    Mutual { client_cert: String, ca_cert: String },
}

impl Default for TunnelAuthConfig {
    fn default() -> Self {
        Self {
            method: TunnelAuthMethod::Certificate {
                cert_path: "/etc/beardog/tunnel/client.crt".to_string(),
                key_path: "/etc/beardog/tunnel/client.key".to_string(),
            },
            credentials: HashMap::new(),
            session_timeout: Duration::from_secs(3600),
        }
    }
}

/// Monitoring configuration for tunnel operations
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct TunnelMonitoringConfig {
    pub enable_metrics: bool,
    pub metrics_interval: Duration,
    pub enable_tracing: bool,
    pub log_level: String,
    pub alert_thresholds: TunnelAlertThresholds,
}

impl Default for TunnelMonitoringConfig {
    fn default() -> Self {
        Self {
            enable_metrics: true,
            metrics_interval: Duration::from_secs(60),
            enable_tracing: true,
            log_level: "info".to_string(),
            alert_thresholds: TunnelAlertThresholds::default(),
        }
    }
}

/// Alert thresholds for tunnel monitoring
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct TunnelAlertThresholds {
    pub max_cpu_usage: f64,
    pub max_memory_usage: f64,
    pub max_network_latency: u32,
    pub max_error_rate: f64,
    pub min_throughput: u64,
    pub max_disk_usage: f64,
    pub max_connections: u32,
    pub max_queue_depth: u32,
    pub max_response_time: u32,
    pub min_availability: f64,
}

impl Default for TunnelAlertThresholds {
    fn default() -> Self {
        Self {
            max_cpu_usage: 0.9,
            max_memory_usage: 0.9,
            max_network_latency: 100,
            max_error_rate: 0.05,
            min_throughput: 1_000_000,
            max_disk_usage: 0.9,
            max_connections: 1000,
            max_queue_depth: 100,
            max_response_time: 500,
            min_availability: 0.99, // Higher standard for tunnels
        }
    }
}

/// Security configuration for tunnel operations
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct TunnelSecurityConfig {
    pub encryption_algorithm: String,
    pub key_size: u32,
    pub enable_perfect_forward_secrecy: bool,
    pub session_key_rotation: Duration,
    pub enable_quantum_resistance: bool,
    pub audit_logging: bool,
    pub intrusion_detection: bool,
}

impl Default for TunnelSecurityConfig {
    fn default() -> Self {
        Self {
            encryption_algorithm: "ChaCha20Poly1305".to_string(),
            key_size: 256,
            enable_perfect_forward_secrecy: true,
            session_key_rotation: Duration::from_secs(300), // 5 minutes
            enable_quantum_resistance: true,
            audit_logging: true,
            intrusion_detection: true,
        }
    }
} 