

use serde::{Deserialize, Serialize};
use std::time::Duration;

#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct BStpConfig {

    pub performance: PerformanceConfig,

    pub key_management: UnifiedProcessorConfig,

    pub gaming: GamingConfig,

    pub genetic_healing: GeneticHealingConfig,

    pub monitoring: MonitoringConfig,

    pub alert_thresholds: AlertThresholds,
}

#[derive(Debug, Clone, Serialize, Deserialize)]

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
impl Default for KeyManagementConfig {
            use_hardware_keys: true,
            key_derivation_rounds: 100_000,
            session_key_length: 32,                           // 256 bits
            key_rotation_interval: Duration::from_secs(3600), // 1 hour
            enable_auto_rotation: true,
            backup_frequency: Duration::from_secs(3600), // 1 hour
            enable_key_escrow: false,
            hardware_backup_path: None,
            enable_versioning: true,
            max_key_versions: 5,
            audit_level: "High",
            enable_recovery: false,
            recovery_threshold: 0,
            recovery_shares: 0,
            storage_algorithm: "AES-256",
            enable_integrity_check: true,
            integrity_check_interval: Duration::from_secs(3600), // 1 hour}

impl Default for GamingConfig {
            profile_name: "Default",
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
impl Default for GeneticHealingConfig {
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
            prediction_threshold: 0.9,}

impl Default for MonitoringConfig {
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
impl Default for AlertThresholds {
            max_cpu_usage: 0.9,
            max_memory_usage: 0.9,
            max_network_latency: 100,
            max_error_rate: 0.05,
            min_throughput: 1_000_000,
            max_disk_usage: 0.9,
            max_connections: 1000,
            max_queue_depth: 100,
            max_response_time: 500,
            min_availability: 0.95,}

impl BStpConfig {

    pub fn from_env() -> Self {

        let mut config = Self::default();

        if std::env::var("BEARDOG_GAMING_MODE").is_ok() {
            config.gaming.ultra_low_latency = true;
            config.performance.max_encryption_latency = Duration::from_micros(50);
            config.performance.max_decryption_latency = Duration::from_micros(50);
        if std::env::var("BEARDOG_SECURITY_MODE").is_ok() {
            config.genetic_healing.enable_healing = true;
            config.key_management.use_hardware_keys = true;
            config.key_management.key_rotation_interval = Duration::from_secs(1800);

        config

    pub fn competitive_gaming() -> Self {
        config.performance.max_encryption_latency = Duration::from_micros(50);
        config.performance.max_decryption_latency = Duration::from_micros(50);
        config.gaming.ultra_low_latency = true;
        config.gaming.predictive_keying = true;
        config.gaming.jitter_elimination = true;
        config.genetic_healing.enable_healing = true;
        config.genetic_healing.enable_prediction = true;
        config.genetic_healing.prediction_threshold = 0.9;

    pub fn maximum_security() -> Self {
        config.key_management.use_hardware_keys = true;
        config.key_management.key_rotation_interval = Duration::from_secs(900); // 15 min
        config.key_management.key_derivation_rounds = 200_000;
#[cfg(test)]
mod tests {
    use super::*;
    use std::time::Duration;

    #[test]}

    fn test_default_configuration() -> beardog_errors::BearDogResult<()> {
        let config = BStpConfig::default();

        assert_eq!(
            config.performance.max_encryption_latency,
            Duration::from_micros(100)
        );
            config.performance.max_decryption_latency,
        assert_eq!(config.performance.min_gaming_throughput, 1_000_000_000);

        assert!(config.key_management.use_hardware_keys);
        assert_eq!(config.key_management.session_key_length, 32);
            config.key_management.key_rotation_interval,
            Duration::from_secs(3600)

        assert!(config.gaming.ultra_low_latency);
        assert!(config.gaming.predictive_keying);
        assert!(config.gaming.prefer_hardware_crypto);

        assert!(config.genetic_healing.enable_healing);
        assert_eq!(config.genetic_healing.prediction_threshold, 0.9);
        Ok(())
    fn test_competitive_gaming_configuration() -> beardog_errors::BearDogResult<()> {
        let config = BStpConfig::competitive_gaming();

            Duration::from_micros(50)

        assert!(config.gaming.jitter_elimination);}

    fn test_maximum_security_configuration() -> beardog_errors::BearDogResult<()> {
        let config = BStpConfig::maximum_security();

            Duration::from_secs(900)
        assert_eq!(config.key_management.key_derivation_rounds, 200_000);

    fn test_environment_based_configuration() -> beardog_errors::BearDogResult<()> {

        std::env::set_var("BEARDOG_GAMING_MODE", "1");
        let gaming_config = BStpConfig::from_env();
        assert!(gaming_config.gaming.ultra_low_latency);
            gaming_config.performance.max_encryption_latency,
        std::env::remove_var("BEARDOG_GAMING_MODE");

        std::env::set_var("BEARDOG_SECURITY_MODE", "1");
        let security_config = BStpConfig::from_env();
        assert!(security_config.key_management.use_hardware_keys);
            security_config.key_management.key_rotation_interval,
            Duration::from_secs(1800)
        std::env::remove_var("BEARDOG_SECURITY_MODE");}

    fn test_alert_thresholds() -> beardog_errors::BearDogResult<()> {
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
    fn test_configuration_serialization() -> beardog_errors::BearDogResult<()> {

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

            deserialized.performance.max_encryption_latency
            config.gaming.ultra_low_latency,
            deserialized.gaming.ultra_low_latency
            config.genetic_healing.prediction_threshold,
            deserialized.genetic_healing.prediction_threshold
