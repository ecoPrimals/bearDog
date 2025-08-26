

use serde::{Deserialize, Serialize};
use std::time::Duration;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct UnifiedTunnelConfig {

    pub performance: TunnelPerformanceConfig,

    pub key_management: TunnelKeyManagementConfig,

    pub gaming: GamingOptimizationConfig,

    pub genetic_healing: GeneticHealingConfig,

    pub monitoring: TunnelMonitoringConfig,

    pub alert_thresholds: TunnelAlertThresholds,

    pub prefer_human_entropy: bool,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct TunnelPerformanceConfig {

    pub max_encryption_latency: Duration,

    pub max_decryption_latency: Duration,

    pub max_session_setup_time: Duration,

    pub min_gaming_throughput: u64,

    pub enable_monitoring: bool,

    pub sampling_interval: Duration,

    pub auto_tuning: bool,

    pub target_latency: Duration,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct TunnelKeyManagementConfig {

    pub use_hardware_keys: bool,

    pub key_rotation_interval: Duration,

    pub enable_human_entropy: bool,

    pub key_strength: u32,

    pub enable_key_escrow: bool,

    pub storage_encryption: String,

    pub enable_attestation: bool,

    pub max_key_age: Duration,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct GamingOptimizationConfig {

    pub ultra_low_latency: bool,

    pub predictive_keying: bool,

    pub jitter_elimination: bool,

    pub packet_priority: u8,

    pub optimized_encryption: bool,

    pub max_gaming_latency: Duration,

    pub enable_gaming_metrics: bool,

    pub session_timeout: Duration,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct GeneticHealingConfig {

    pub enable_healing: bool,

    pub enable_prediction: bool,

    pub response_time: Duration,

    pub max_healing_attempts: u32,

    pub enable_learning: bool,

    pub learning_rate: f64,

    pub enable_adaptation: bool,

    pub confidence_threshold: f64,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct TunnelMonitoringConfig {

    pub enable_detailed_monitoring: bool,

    pub monitoring_interval: Duration,

    pub enable_profiling: bool,

    pub profiling_retention: Duration,

    pub enable_alerting: bool,

    pub alert_interval: Duration,

    pub enable_aggregation: bool,

    pub aggregation_window: Duration,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct TunnelAlertThresholds {

    pub max_cpu_usage: f64,

    pub max_memory_usage: f64,

    pub max_network_latency: u64,

    pub max_error_rate: f64,

    pub min_throughput: u64,

    pub max_disk_usage: f64,

    pub max_connections: u32,

    pub max_queue_depth: u32,

    pub max_response_time: u64,

    pub min_availability: f64,
}

impl Default for UnifiedTunnelConfig {
    fn default() -> Self {
        Self {
            performance: TunnelPerformanceConfig::default(),
            key_management: TunnelKeyManagementConfig::default(),
            gaming: GamingOptimizationConfig::default(),
            genetic_healing: GeneticHealingConfig::default(),
            monitoring: TunnelMonitoringConfig::default(),
            alert_thresholds: TunnelAlertThresholds::default(),
            prefer_human_entropy: true,
        }
    }
}

impl Default for TunnelPerformanceConfig {
    fn default() -> Self {
        Self {
            max_encryption_latency: Duration::from_millis(1),
            max_decryption_latency: Duration::from_millis(1),
            max_session_setup_time: Duration::from_millis(100),
            min_gaming_throughput: 10_000,
            enable_monitoring: true,
            sampling_interval: Duration::from_millis(100),
            auto_tuning: true,
            target_latency: Duration::from_micros(500),
        }
    }
}

impl Default for TunnelKeyManagementConfig {
    fn default() -> Self {
        Self {
            use_hardware_keys: true,
            key_rotation_interval: Duration::from_secs(3600), // 1 hour
            enable_human_entropy: true,
            key_strength: 256,
            enable_key_escrow: false,
            storage_encryption: "AES-256-GCM".to_string(),
            enable_attestation: true,
            max_key_age: Duration::from_secs(86400), // 24 hours
        }
    }
}

impl Default for GamingOptimizationConfig {
    fn default() -> Self {
        Self {
            ultra_low_latency: false,
            predictive_keying: false,
            jitter_elimination: false,
            packet_priority: 7, // High priority
            optimized_encryption: true,
            max_gaming_latency: Duration::from_millis(10),
            enable_gaming_metrics: true,
            session_timeout: Duration::from_secs(7200), // 2 hours
        }
    }
}

impl Default for GeneticHealingConfig {
    fn default() -> Self {
        Self {
            enable_healing: false,
            enable_prediction: false,
            response_time: Duration::from_millis(100),
            max_healing_attempts: 3,
            enable_learning: false,
            learning_rate: 0.01,
            enable_adaptation: false,
            confidence_threshold: 0.8,
        }
    }
}

impl Default for TunnelMonitoringConfig {
    fn default() -> Self {
        Self {
            enable_detailed_monitoring: true,
            monitoring_interval: Duration::from_secs(60),
            enable_profiling: false,
            profiling_retention: Duration::from_secs(7 * 24 * 60 * 60), // 7 days
            enable_alerting: true,
            alert_interval: Duration::from_millis(100),
            enable_aggregation: true,
            aggregation_window: Duration::from_secs(5 * 60), // 5 minutes
        }
    }
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
            min_availability: 0.95,
        }
    }
}

impl UnifiedTunnelConfig {

    pub fn from_env() -> Self {
        let mut config = Self::default();

        if std::env::var("BEARDOG_GAMING_MODE").is_ok() {
            config.gaming.ultra_low_latency = true;
            config.performance.max_encryption_latency = Duration::from_micros(50);
            config.performance.max_decryption_latency = Duration::from_micros(50);
            config.gaming.predictive_keying = true;
            config.gaming.jitter_elimination = true;
        }

        if std::env::var("BEARDOG_SECURITY_MODE").is_ok() {
            config.genetic_healing.enable_healing = true;
            config.key_management.use_hardware_keys = true;
            config.key_management.key_rotation_interval = Duration::from_secs(1800); // 30 min
            config.key_management.enable_attestation = true;
        }
        
        config
    }

    pub fn competitive_gaming() -> Self {
        let mut config = Self::default();

        config.performance.max_encryption_latency = Duration::from_micros(50);
        config.performance.max_decryption_latency = Duration::from_micros(50);
        config.performance.target_latency = Duration::from_micros(100);

        config.gaming.ultra_low_latency = true;
        config.gaming.predictive_keying = true;
        config.gaming.jitter_elimination = true;
        config.gaming.optimized_encryption = true;
        config.gaming.max_gaming_latency = Duration::from_millis(5);

        config.genetic_healing.enable_healing = true;
        config.genetic_healing.enable_prediction = true;
        config.genetic_healing.response_time = Duration::from_millis(50);

        config.monitoring.enable_detailed_monitoring = true;
        config.monitoring.monitoring_interval = Duration::from_millis(100);
        
        config
    }

    pub fn maximum_security() -> Self {
        let mut config = Self::default();

        config.key_management.use_hardware_keys = true;
        config.key_management.enable_attestation = true;
        config.key_management.key_strength = 384; // Higher security
        config.key_management.key_rotation_interval = Duration::from_secs(900); // 15 min

        config.genetic_healing.enable_healing = true;
        config.genetic_healing.enable_prediction = true;
        config.genetic_healing.enable_learning = true;
        config.genetic_healing.enable_adaptation = true;
        config.genetic_healing.confidence_threshold = 0.95;

        config.monitoring.enable_detailed_monitoring = true;
        config.monitoring.enable_profiling = true;
        config.monitoring.enable_alerting = true;

        config.alert_thresholds.max_error_rate = 0.01;
        config.alert_thresholds.min_availability = 0.99;
        config.alert_thresholds.max_response_time = 100;
        
        config
    }
}

pub type BStpConfig = UnifiedTunnelConfig;

pub type PerformanceConfig = TunnelPerformanceConfig;
pub type KeyManagementConfig = TunnelKeyManagementConfig;
pub type GamingConfig = GamingOptimizationConfig;
pub type MonitoringConfig = TunnelMonitoringConfig;
pub type AlertThresholds = TunnelAlertThresholds;
