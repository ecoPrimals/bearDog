// BearDog - Enterprise Security Ecosystem
// Copyright (C) 2025 EcoPrimals
//
// This program is free software: you can redistribute it and/or modify
// it under the terms of the GNU Affero General Public License as published by
// the Free Software Foundation, either version 3 of the License, or
// (at your option) any later version.
//
// This program is distributed in the hope that it will be useful,
// but WITHOUT ANY WARRANTY; without even the implied warranty of
// MERCHANTABILITY or FITNESS FOR A PARTICULAR PURPOSE. See the
// GNU Affero General Public License for more details.
//
// You should have received a copy of the GNU Affero General Public License
// along with this program. If not, see <https://www.gnu.org/licenses/>.


/// # BearDog Tunnel Configuration
///
/// This module provides configuration structures for the BearDog tunnel system.
/// It includes performance tuning, key management, gaming optimizations, genetic healing,
/// monitoring, and alerting configurations.
/// ## Configuration Profiles
/// - **Competitive Gaming**: Ultra-low latency for competitive gaming
/// - **Maximum Security**: High security with hardware key requirements
/// - **Default**: Balanced performance and security
/// ## Key Features
/// - Performance optimization settings
/// - Key management and rotation policies
/// - Gaming-specific configurations
/// - Genetic healing parameters
/// - Monitoring and alerting thresholds

use serde::{Deserialize, Serialize};
use std::time::Duration;
/// Main BearDog Tunnel Protocol (BSTP) configuration
/// Root configuration structure that contains all subsystem configurations.
/// This is the primary configuration entry point for the entire tunnel system.
#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct BStpConfig {
    /// Performance optimization settings
    pub performance: PerformanceConfig,
    /// Key management configuration
    pub key_management: KeyManagementConfig,
    /// Gaming-specific optimizations
    pub gaming: GamingConfig,
    /// Genetic healing system configuration
    pub genetic_healing: GeneticHealingConfig,
    /// Monitoring and metrics configuration
    pub monitoring: MonitoringConfig,
    /// Alert threshold configuration
    pub alert_thresholds: AlertThresholds,
}
/// Performance optimization configuration
/// Controls performance-related settings including latency targets, throughput requirements,
/// and session setup time limits. These settings directly impact the user experience.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PerformanceConfig {
    /// Maximum acceptable encryption latency
    pub max_encryption_latency: Duration,
    /// Maximum acceptable decryption latency
    pub max_decryption_latency: Duration,
    /// Maximum session setup time
    pub max_session_setup_time: Duration,
    /// Minimum required gaming throughput (packets/sec)
    pub min_gaming_throughput: u64,
    /// Enable performance monitoring
    pub enable_monitoring: bool,
    /// Performance metrics collection interval
    pub metrics_interval: Duration,
    /// Enable predictive performance optimization
    pub enable_prediction: bool,
    /// Target memory usage limit (MB)
    pub memory_limit_mb: usize,
    /// Maximum concurrent sessions
    pub max_concurrent_sessions: usize,
    /// Enable performance-based auto-scaling
    pub enable_auto_scaling: bool,
    /// CPU usage threshold for auto-scaling
    pub cpu_threshold: f64,
    /// Memory usage threshold for auto-scaling
    pub memory_threshold: f64,
    /// Network bandwidth limit (Mbps)
    pub bandwidth_limit_mbps: u64,
    /// Enable compression for data transmission
    pub enable_compression: bool,
    /// Compression level (1-9, higher = better compression but slower)
    pub compression_level: u8,
    /// Enable caching for frequently accessed data
    pub enable_caching: bool,
    /// Cache size limit (MB)
    pub cache_size_mb: usize,
    /// Cache TTL for entries
    pub cache_ttl: Duration,
/// Key management configuration
/// Controls cryptographic key generation, rotation, storage, and lifecycle management.
/// These settings are critical for maintaining security and compliance.
pub struct KeyManagementConfig {
    /// Use hardware security modules for key generation
    pub use_hardware_keys: bool,
    /// Key derivation rounds for PBKDF2
    pub key_derivation_rounds: u32,
    /// Session key length in bytes
    pub session_key_length: usize,
    /// Key rotation interval
    pub key_rotation_interval: Duration,
    /// Enable automatic key rotation
    pub enable_auto_rotation: bool,
    /// Master key backup frequency
    pub backup_frequency: Duration,
    /// Key escrow requirements
    pub enable_key_escrow: bool,
    /// Hardware key backup location
    pub hardware_backup_path: Option<String>,
    /// Enable key versioning
    pub enable_versioning: bool,
    /// Maximum key versions to retain
    pub max_key_versions: u32,
    /// Key audit logging level
    pub audit_level: String,
    /// Enable key recovery mechanisms
    pub enable_recovery: bool,
    /// Recovery key threshold (for secret sharing)
    pub recovery_threshold: u32,
    /// Total recovery key shares
    pub recovery_shares: u32,
    /// Key storage encryption algorithm
    pub storage_algorithm: String,
    /// Enable key integrity checking
    pub enable_integrity_check: bool,
    /// Key integrity check interval
    pub integrity_check_interval: Duration,
/// Gaming-specific configuration
/// Optimizations specifically designed for gaming workloads including ultra-low latency,
/// jitter elimination, and gaming-specific crypto optimizations.
pub struct GamingConfig {
    /// Gaming profile name
    pub profile_name: String,
    /// Enable ultra-low latency mode
    pub ultra_low_latency: bool,
    /// Enable predictive key generation
    pub predictive_keying: bool,
    /// Enable jitter elimination
    pub jitter_elimination: bool,
    /// Enable bandwidth optimization
    pub bandwidth_optimization: bool,
    /// Prefer hardware-accelerated crypto
    pub prefer_hardware_crypto: bool,
    /// Enable batch processing for crypto operations
    pub enable_batch_processing: bool,
    /// Target frame rate for optimization
    pub target_fps: u32,
    /// Maximum acceptable input lag (ms)
    pub max_input_lag_ms: u32,
    /// Enable gaming traffic prioritization
    pub enable_traffic_priority: bool,
    /// Gaming traffic priority level (0-7)
    pub traffic_priority_level: u8,
    /// Enable anti-cheat integration
    pub enable_anti_cheat: bool,
    /// Anti-cheat provider
    pub anti_cheat_provider: String,
    /// Enable game state synchronization
    pub enable_state_sync: bool,
    /// State sync interval (ms)
    pub state_sync_interval_ms: u32,
    /// Enable spectator mode support
    pub enable_spectator_mode: bool,
    /// Maximum spectators per session
    pub max_spectators: u32,
/// Genetic healing system configuration
/// Controls the autonomous healing system that uses genetic algorithms to optimize
/// security and performance based on network conditions and threats.
pub struct GeneticHealingConfig {
    /// Enable genetic healing system
    pub enable_healing: bool,
    /// Healing algorithm generation interval
    pub generation_interval: Duration,
    /// Population size for genetic algorithms
    pub population_size: usize,
    /// Mutation rate for genetic evolution
    pub mutation_rate: f64,
    /// Crossover rate for genetic recombination
    pub crossover_rate: f64,
    /// Fitness evaluation interval
    pub fitness_interval: Duration,
    /// Enable network consensus for healing decisions
    pub enable_consensus: bool,
    /// Consensus threshold (percentage of nodes that must agree)
    pub consensus_threshold: f64,
    /// Maximum healing attempts before fallback
    pub max_healing_attempts: u32,
    /// Healing attempt timeout
    pub healing_timeout: Duration,
    /// Enable adaptive mutation rates
    pub adaptive_mutation: bool,
    /// Enable elitism in genetic selection
    pub enable_elitism: bool,
    /// Elite selection percentage
    pub elite_percentage: f64,
    /// Enable diversity preservation
    pub preserve_diversity: bool,
    /// Minimum genetic diversity threshold
    pub min_diversity: f64,
    /// Enable healing history tracking
    pub track_history: bool,
    /// Maximum healing history entries
    pub max_history_entries: usize,
    /// Enable predictive healing
    /// Prediction confidence threshold
    pub prediction_threshold: f64,
/// Monitoring and metrics configuration
/// Controls system monitoring, metrics collection, and observability features.
pub struct MonitoringConfig {
    /// Enable comprehensive monitoring
    /// Metrics collection interval
    /// Enable distributed tracing
    pub enable_tracing: bool,
    /// Tracing sample rate (0.0 to 1.0)
    pub trace_sample_rate: f64,
    /// Enable audit logging
    pub enable_audit_logging: bool,
    /// Audit log retention period
    pub audit_retention: Duration,
    /// Enable performance profiling
    pub enable_profiling: bool,
    /// Profiling data retention period
    pub profiling_retention: Duration,
    /// Enable real-time alerting
    pub enable_alerting: bool,
    /// Alert evaluation interval
    pub alert_interval: Duration,
    /// Enable metric aggregation
    pub enable_aggregation: bool,
    /// Metric aggregation window
    pub aggregation_window: Duration,
/// Alert threshold configuration
/// Defines thresholds for various system metrics that trigger alerts when exceeded.
pub struct AlertThresholds {
    /// Maximum CPU usage before alert (percentage)
    pub max_cpu_usage: f64,
    /// Maximum memory usage before alert (percentage)
    pub max_memory_usage: f64,
    /// Maximum network latency before alert (ms)
    pub max_network_latency: u64,
    /// Maximum error rate before alert (percentage)
    pub max_error_rate: f64,
    /// Minimum throughput before alert (ops/sec)
    pub min_throughput: u64,
    /// Maximum disk usage before alert (percentage)
    pub max_disk_usage: f64,
    /// Maximum connection count before alert
    pub max_connections: u32,
    /// Maximum queue depth before alert
    pub max_queue_depth: u32,
    /// Maximum response time before alert (ms)
    pub max_response_time: u64,
    /// Minimum availability before alert (percentage)
    pub min_availability: f64,}


impl Default for PerformanceConfig {}


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
            audit_level: String::from("High"),
            enable_recovery: false,
            recovery_threshold: 0,
            recovery_shares: 0,
            storage_algorithm: String::from("AES-256"),
            enable_integrity_check: true,
            integrity_check_interval: Duration::from_secs(3600), // 1 hour}


impl Default for GamingConfig {
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
    /// Load configuration from environment variables
    pub fn from_env() -> Self {
        // In production, this would read from environment variables
        // For now, return optimized defaults
        let mut config = Self::default();
        // Override with environment-specific optimizations
        if std::env::var("BEARDOG_GAMING_MODE").is_ok() {
            config.gaming.ultra_low_latency = true;
            config.performance.max_encryption_latency = Duration::from_micros(50);
            config.performance.max_decryption_latency = Duration::from_micros(50);
        if std::env::var("BEARDOG_SECURITY_MODE").is_ok() {
            config.genetic_healing.enable_healing = true;
            config.key_management.use_hardware_keys = true;
            config.key_management.key_rotation_interval = Duration::from_secs(1800);
            // 30 min
        config
    /// Create configuration optimized for competitive gaming}


    pub fn competitive_gaming() -> Self {
        config.performance.max_encryption_latency = Duration::from_micros(50);
        config.performance.max_decryption_latency = Duration::from_micros(50);
        config.gaming.ultra_low_latency = true;
        config.gaming.predictive_keying = true;
        config.gaming.jitter_elimination = true;
        config.genetic_healing.enable_healing = true;
        config.genetic_healing.enable_prediction = true;
        config.genetic_healing.prediction_threshold = 0.9;
    /// Create configuration optimized for maximum security
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
        // Verify performance defaults
        assert_eq!(
            config.performance.max_encryption_latency,
            Duration::from_micros(100)
        );
            config.performance.max_decryption_latency,
        assert_eq!(config.performance.min_gaming_throughput, 1_000_000_000);
        // Verify key management defaults
        assert!(config.key_management.use_hardware_keys);
        assert_eq!(config.key_management.session_key_length, 32);
            config.key_management.key_rotation_interval,
            Duration::from_secs(3600)
        // Verify gaming defaults
        assert!(config.gaming.ultra_low_latency);
        assert!(config.gaming.predictive_keying);
        assert!(config.gaming.prefer_hardware_crypto);
        // Verify genetic healing defaults
        assert!(config.genetic_healing.enable_healing);
        assert_eq!(config.genetic_healing.prediction_threshold, 0.9);
        Ok(())
    fn test_competitive_gaming_configuration() -> beardog_errors::BearDogResult<()> {
        let config = BStpConfig::competitive_gaming();
        // Should have ultra-low latency targets
            Duration::from_micros(50)
        // Should prioritize performance over security
        // Gaming optimizations should be enabled
        assert!(config.gaming.jitter_elimination);}


    fn test_maximum_security_configuration() -> beardog_errors::BearDogResult<()> {
        let config = BStpConfig::maximum_security();
        // Should have frequent key rotation
            Duration::from_secs(900)
        assert_eq!(config.key_management.key_derivation_rounds, 200_000);
        // Should prioritize security over performance
    fn test_environment_based_configuration() -> beardog_errors::BearDogResult<()> {
        // Test gaming mode environment variable
        std::env::set_var("BEARDOG_GAMING_MODE", "1");
        let gaming_config = BStpConfig::from_env();
        assert!(gaming_config.gaming.ultra_low_latency);
            gaming_config.performance.max_encryption_latency,
        std::env::remove_var("BEARDOG_GAMING_MODE");
        // Test security mode environment variable
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
        // Test that configuration can be serialized and deserialized
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
        // Verify key fields match
            deserialized.performance.max_encryption_latency
            config.gaming.ultra_low_latency,
            deserialized.gaming.ultra_low_latency
            config.genetic_healing.prediction_threshold,
            deserialized.genetic_healing.prediction_threshold
