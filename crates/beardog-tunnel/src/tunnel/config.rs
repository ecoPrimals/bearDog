use beardog_errors::BearDogError;

use serde::{Deserialize, Serialize};
use std::time::Duration;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PerformanceConfig {
    pub max_encryption_latency: Duration,
    pub max_decryption_latency: Duration,
    pub max_session_setup_time: Duration,
    pub min_gaming_throughput: u64,
    pub enable_monitoring: bool,
    pub metrics_interval: Duration,
    pub enable_prediction: bool,
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

impl Default for PerformanceConfig {
    fn default() -> Self {
        Self {
            max_encryption_latency: Duration::from_micros(100),
            max_decryption_latency: Duration::from_micros(100),
            max_session_setup_time: Duration::from_millis(10),
            min_gaming_throughput: 1_000_000_000, // 1 Gbps
            enable_monitoring: false,
            metrics_interval: Duration::from_millis(100),
            enable_prediction: false,
            memory_limit_mb: 1024, // 1 GB
            max_concurrent_sessions: 100,
            enable_auto_scaling: false,
            cpu_threshold: 0.8,
            memory_threshold: 0.8,
            bandwidth_limit_mbps: 1_000_000, // 1 Gbps
            enable_compression: false,
            compression_level: 5,
            enable_caching: false,
            cache_size_mb: 1024,                  // 1 GB
            cache_ttl: Duration::from_secs(3600), // 1 hour
        }
    }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct KeyManagementConfig {
    pub use_hardware_keys: bool,
    pub key_derivation_rounds: u32,
    pub session_key_length: u32,
    pub key_rotation_interval: Duration,
    pub backup_key_count: u32,
    pub key_storage_path: String,
    pub enable_key_escrow: bool,
    pub key_escrow_threshold: u32,
}

impl Default for KeyManagementConfig {
    fn default() -> Self {
        Self {
            use_hardware_keys: true,
            key_derivation_rounds: 100_000,
            session_key_length: 32,                           // 256 bits
            key_rotation_interval: Duration::from_secs(3600), // 1 hour
            backup_key_count: 1,
            key_storage_path: String::from("/var/lib/beardog/keys"),
            enable_key_escrow: false,
            key_escrow_threshold: 10,
        }
    }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct GamingConfig {
    pub profile_name: String,
    pub ultra_low_latency: bool,
    pub predictive_keying: bool,
    pub jitter_elimination: bool,
    pub bandwidth_optimization: bool,
    pub prefer_hardware_crypto: bool,
    pub enable_batch_processing: bool, // Prefer latency over throughput
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

impl Default for GamingConfig {
    fn default() -> Self {
        Self {
            profile_name: String::from("Default"),
            ultra_low_latency: true,
            predictive_keying: true,
            jitter_elimination: true,
            bandwidth_optimization: false,
            prefer_hardware_crypto: true,
            enable_batch_processing: false, // Prefer latency over throughput
            target_fps: 60,
            max_input_lag_ms: 100,
            enable_traffic_priority: false,
            traffic_priority_level: 0,
            enable_anti_cheat: false,
            anti_cheat_provider: String::from(""),
            enable_state_sync: false,
            state_sync_interval_ms: 100,
            enable_spectator_mode: false,
            max_spectators: 100,
        }
    }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct GeneticHealingConfig {
    pub enable_healing: bool,
    pub generation_interval: Duration,
    pub population_size: u32,
    pub mutation_rate: f64,
    pub crossover_rate: f64,
    pub fitness_interval: Duration,
    pub enable_consensus: bool,
    pub consensus_threshold: f64,
    pub max_healing_attempts: u32,
    pub healing_timeout: Duration,
    pub adaptive_mutation: bool,
    pub enable_elitism: bool,
    pub elite_percentage: f64,
    pub preserve_diversity: bool,
    pub min_diversity: f64,
    pub track_history: bool,
    pub max_history_entries: u32,
    pub enable_prediction: bool,
    pub prediction_threshold: f64,
}

impl Default for GeneticHealingConfig {
    fn default() -> Self {
        Self {
            enable_healing: true,
            generation_interval: Duration::from_secs(3600), // 1 hour
            population_size: 100,
            mutation_rate: 0.05,
            crossover_rate: 0.8,
            fitness_interval: Duration::from_secs(3600), // 1 hour
            enable_consensus: true,
            consensus_threshold: 0.7,
            max_healing_attempts: 10,
            healing_timeout: Duration::from_secs(300), // 5 minutes
            adaptive_mutation: true,
            enable_elitism: true,
            elite_percentage: 0.2,
            preserve_diversity: true,
            min_diversity: 0.5,
            track_history: true,
            max_history_entries: 100,
            enable_prediction: true,
            prediction_threshold: 0.9,
        }
    }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct MonitoringConfig {
    pub enable_monitoring: bool,
    pub enable_tracing: bool,
    pub trace_sample_rate: f64,
    pub enable_audit_logging: bool,
    pub audit_retention: Duration,
    pub enable_profiling: bool,
    pub profiling_retention: Duration,
    pub enable_alerting: bool,
    pub alert_interval: Duration,
    pub enable_aggregation: bool,
    pub aggregation_window: Duration,
}

impl Default for MonitoringConfig {
    fn default() -> Self {
        Self {
            enable_monitoring: true,
            enable_tracing: false,
            trace_sample_rate: 0.1,
            enable_audit_logging: false,
            audit_retention: Duration::from_secs(7 * 24 * 60 * 60), // 7 days
            enable_profiling: false,
            profiling_retention: Duration::from_secs(7 * 24 * 60 * 60), // 7 days
            enable_alerting: true,
            alert_interval: Duration::from_millis(100),
            enable_aggregation: true,
            aggregation_window: Duration::from_secs(5 * 60), // 5 minutes
        }
    }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct AlertThresholds {
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

impl Default for AlertThresholds {
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

#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct BStpConfig {

    pub performance: PerformanceConfig,

    pub key_management: KeyManagementConfig,

    pub gaming: GamingConfig,

    pub genetic_healing: GeneticHealingConfig,

    pub monitoring: MonitoringConfig,

    pub alert_thresholds: AlertThresholds,
}

impl BStpConfig {

    pub fn from_env() -> Self {

        let mut config = Self::default();

        if std::env::var("BEARDOG_GAMING_MODE").is_ok() {
            config.gaming.ultra_low_latency = true;
            config.performance.max_encryption_latency = Duration::from_micros(50);
            config.performance.max_decryption_latency = Duration::from_micros(50);
        }
        if std::env::var("BEARDOG_SECURITY_MODE").is_ok() {
            config.genetic_healing.enable_healing = true;
            config.key_management.use_hardware_keys = true;
            config.key_management.key_rotation_interval = Duration::from_secs(1800);
        }

        config
    }

    pub fn competitive_gaming() -> Self {
        let mut config = Self::default();
        config.performance.max_encryption_latency = Duration::from_micros(50);
        config.performance.max_decryption_latency = Duration::from_micros(50);
        config.gaming.ultra_low_latency = true;
        config.gaming.predictive_keying = true;
        config.gaming.jitter_elimination = true;
        config.genetic_healing.enable_healing = true;
        config.genetic_healing.enable_prediction = true;
        config.genetic_healing.prediction_threshold = 0.9;
        config
    }

    pub fn maximum_security() -> Self {
        let mut config = Self::default();
        config.key_management.use_hardware_keys = true;
        config.key_management.key_rotation_interval = Duration::from_secs(900); // 15 min
        config.key_management.key_derivation_rounds = 200_000;
        config
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::time::Duration;

    #[test]
    fn test_default_configuration() -> Result<(), BearDogError> {
        let config = BStpConfig::default();

        assert_eq!(
            config.performance.max_encryption_latency,
            Duration::from_micros(100)
        );
        assert_eq!(config.performance.max_decryption_latency, Duration::from_micros(100));
        assert_eq!(config.performance.min_gaming_throughput, 1_000_000_000);

        assert!(config.key_management.use_hardware_keys);
        assert_eq!(config.key_management.session_key_length, 32);
        assert_eq!(config.key_management.key_rotation_interval, Duration::from_secs(3600));

        assert!(config.gaming.ultra_low_latency);
        assert!(config.gaming.predictive_keying);
        assert!(config.gaming.prefer_hardware_crypto);

        assert!(config.genetic_healing.enable_healing);
        assert_eq!(config.genetic_healing.prediction_threshold, 0.9);
        Ok(())
    }

    #[test]
    fn test_competitive_gaming_configuration() -> Result<(), BearDogError> {
        let config = BStpConfig::competitive_gaming();

        assert_eq!(config.performance.max_encryption_latency, Duration::from_micros(50));
        assert_eq!(config.performance.max_decryption_latency, Duration::from_micros(50));
        assert!(config.gaming.jitter_elimination);
        assert!(config.gaming.ultra_low_latency);
        assert!(config.gaming.predictive_keying);
        assert!(config.gaming.prefer_hardware_crypto);
        assert!(config.genetic_healing.enable_healing);
        assert!(config.genetic_healing.enable_prediction);
        assert_eq!(config.genetic_healing.prediction_threshold, 0.9);
        Ok(())
    }

    #[test]
    fn test_maximum_security_configuration() -> Result<(), BearDogError> {
        let config = BStpConfig::maximum_security();

        assert!(config.key_management.use_hardware_keys);
        assert_eq!(config.key_management.key_rotation_interval, Duration::from_secs(900)); // 15 min
        assert_eq!(config.key_management.key_derivation_rounds, 200_000);
        Ok(())
    }

    #[test]
    fn test_environment_based_configuration() -> Result<(), BearDogError> {

        std::env::set_var("BEARDOG_GAMING_MODE", "1");
        let gaming_config = BStpConfig::from_env();
        assert!(gaming_config.gaming.ultra_low_latency);
        assert_eq!(gaming_config.performance.max_encryption_latency, Duration::from_micros(50));
        std::env::remove_var("BEARDOG_GAMING_MODE");

        std::env::set_var("BEARDOG_SECURITY_MODE", "1");
        let security_config = BStpConfig::from_env();
        assert!(security_config.key_management.use_hardware_keys);
        assert_eq!(
            security_config.key_management.key_rotation_interval,
            Duration::from_secs(1800)
        );
        std::env::remove_var("BEARDOG_SECURITY_MODE");
        Ok(())
    }

    #[test]
    fn test_alert_thresholds() -> Result<(), BearDogError> {
        let config = BStpConfig::default();
        let alerts = &config.alert_thresholds;
        assert_eq!(alerts.max_cpu_usage, 0.9);
        assert_eq!(alerts.max_memory_usage, 0.9);
        assert_eq!(alerts.max_network_latency, 100);
        assert_eq!(alerts.max_error_rate, 0.05);
        assert_eq!(alerts.min_throughput, 1_000_000);
        assert_eq!(alerts.max_disk_usage, 0.9);
        assert_eq!(alerts.max_connections, 1000);
        assert_eq!(alerts.max_queue_depth, 100);
        assert_eq!(alerts.max_response_time, 500);
        assert_eq!(alerts.min_availability, 0.95);
        Ok(())
    }

    #[test]
    fn test_configuration_serialization() -> Result<(), BearDogError> {

        let config = BStpConfig::default();
        let serialized = serde_json::to_string(&config).map_err(|e| {
            tracing::error!("Operation failed ({}): {:?}", "Should serialize", e);
            beardog_errors::BearDogError::internal(format!(
                "Operation failed ({}): {:?}",
                "Should serialize", e
            ))
        })?;
        let deserialized: BStpConfig = serde_json::from_str(&serialized).map_err(|e| {
            tracing::error!("JSON parsing failed ({}): {}", "Should deserialize", e);
            beardog_errors::BearDogError::ValidationError(format!(
                "JSON parsing error ({}): {}",
                "Should deserialize", e
            ))
        })?;

        // Basic validation assertions
        assert_eq!(config.performance.max_encryption_latency, deserialized.performance.max_encryption_latency);
        assert_eq!(config.gaming.ultra_low_latency, deserialized.gaming.ultra_low_latency);
        assert_eq!(config.genetic_healing.prediction_threshold, deserialized.genetic_healing.prediction_threshold);
        
        Ok(())
    }
}
