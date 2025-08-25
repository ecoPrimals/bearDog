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


/// # Tunnel Configuration - Canonical
///
/// **CANONICAL MODERNIZATION** ✅
/// This module consolidates tunnel-specific configurations into the canonical
/// configuration system, replacing the scattered BStpConfig with unified types.
///
/// ## Migration from BStpConfig
/// - `BStpConfig` → `UnifiedTunnelConfig`
/// - All tunnel configs now use canonical types
/// - Backward compatibility maintained through re-exports

use serde::{Deserialize, Serialize};
use std::time::Duration;

/// Unified tunnel configuration - replaces BStpConfig
/// 
/// **CANONICAL REPLACEMENT** for `BStpConfig` with improved organization
/// and integration with the unified configuration system.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct UnifiedTunnelConfig {
    /// Performance optimization settings
    pub performance: TunnelPerformanceConfig,
    /// Key management configuration
    pub key_management: TunnelKeyManagementConfig,
    /// Gaming-specific optimizations
    pub gaming: GamingOptimizationConfig,
    /// Genetic healing system configuration
    pub genetic_healing: GeneticHealingConfig,
    /// Monitoring and metrics configuration
    pub monitoring: TunnelMonitoringConfig,
    /// Alert threshold configuration
    pub alert_thresholds: TunnelAlertThresholds,
    /// Whether to prefer human entropy for key generation
    pub prefer_human_entropy: bool,
}

/// Tunnel performance optimization configuration
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct TunnelPerformanceConfig {
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
    /// Performance sampling interval
    pub sampling_interval: Duration,
    /// Enable automatic performance tuning
    pub auto_tuning: bool,
    /// Target latency for auto-tuning
    pub target_latency: Duration,
}

/// Tunnel key management configuration
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct TunnelKeyManagementConfig {
    /// Use hardware-backed keys when available
    pub use_hardware_keys: bool,
    /// Key rotation interval
    pub key_rotation_interval: Duration,
    /// Enable key derivation from human entropy
    pub enable_human_entropy: bool,
    /// Key strength in bits
    pub key_strength: u32,
    /// Enable key escrow for recovery
    pub enable_key_escrow: bool,
    /// Key storage encryption algorithm
    pub storage_encryption: String,
    /// Enable key attestation
    pub enable_attestation: bool,
    /// Maximum key age before forced rotation
    pub max_key_age: Duration,
}

/// Gaming optimization configuration
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct GamingOptimizationConfig {
    /// Enable ultra-low latency mode
    pub ultra_low_latency: bool,
    /// Enable predictive key scheduling
    pub predictive_keying: bool,
    /// Enable jitter elimination
    pub jitter_elimination: bool,
    /// Gaming packet priority
    pub packet_priority: u8,
    /// Enable gaming-specific encryption optimizations
    pub optimized_encryption: bool,
    /// Maximum acceptable gaming latency
    pub max_gaming_latency: Duration,
    /// Enable gaming performance monitoring
    pub enable_gaming_metrics: bool,
    /// Gaming session timeout
    pub session_timeout: Duration,
}

/// Genetic healing system configuration
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct GeneticHealingConfig {
    /// Enable genetic healing
    pub enable_healing: bool,
    /// Enable predictive healing
    pub enable_prediction: bool,
    /// Healing response time
    pub response_time: Duration,
    /// Maximum healing attempts
    pub max_healing_attempts: u32,
    /// Enable genetic learning
    pub enable_learning: bool,
    /// Learning rate for genetic algorithms
    pub learning_rate: f64,
    /// Enable automatic threat adaptation
    pub enable_adaptation: bool,
    /// Genetic healing confidence threshold
    pub confidence_threshold: f64,
}

/// Tunnel monitoring configuration
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct TunnelMonitoringConfig {
    /// Enable detailed monitoring
    pub enable_detailed_monitoring: bool,
    /// Monitoring interval
    pub monitoring_interval: Duration,
    /// Enable performance profiling
    pub enable_profiling: bool,
    /// Profiling data retention period
    pub profiling_retention: Duration,
    /// Enable alerting
    pub enable_alerting: bool,
    /// Alert notification interval
    pub alert_interval: Duration,
    /// Enable metric aggregation
    pub enable_aggregation: bool,
    /// Aggregation time window
    pub aggregation_window: Duration,
}

/// Tunnel alert thresholds
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct TunnelAlertThresholds {
    /// Maximum CPU usage (0.0-1.0)
    pub max_cpu_usage: f64,
    /// Maximum memory usage (0.0-1.0)
    pub max_memory_usage: f64,
    /// Maximum network latency (milliseconds)
    pub max_network_latency: u64,
    /// Maximum error rate (0.0-1.0)
    pub max_error_rate: f64,
    /// Minimum throughput (operations/sec)
    pub min_throughput: u64,
    /// Maximum disk usage (0.0-1.0)
    pub max_disk_usage: f64,
    /// Maximum concurrent connections
    pub max_connections: u32,
    /// Maximum queue depth
    pub max_queue_depth: u32,
    /// Maximum response time (milliseconds)
    pub max_response_time: u64,
    /// Minimum availability (0.0-1.0)
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
    /// Load configuration from environment variables
    pub fn from_env() -> Self {
        let mut config = Self::default();
        
        // Gaming mode optimizations
        if std::env::var("BEARDOG_GAMING_MODE").is_ok() {
            config.gaming.ultra_low_latency = true;
            config.performance.max_encryption_latency = Duration::from_micros(50);
            config.performance.max_decryption_latency = Duration::from_micros(50);
            config.gaming.predictive_keying = true;
            config.gaming.jitter_elimination = true;
        }
        
        // Security mode optimizations
        if std::env::var("BEARDOG_SECURITY_MODE").is_ok() {
            config.genetic_healing.enable_healing = true;
            config.key_management.use_hardware_keys = true;
            config.key_management.key_rotation_interval = Duration::from_secs(1800); // 30 min
            config.key_management.enable_attestation = true;
        }
        
        config
    }

    /// Create configuration optimized for competitive gaming
    pub fn competitive_gaming() -> Self {
        let mut config = Self::default();
        
        // Ultra-low latency settings
        config.performance.max_encryption_latency = Duration::from_micros(50);
        config.performance.max_decryption_latency = Duration::from_micros(50);
        config.performance.target_latency = Duration::from_micros(100);
        
        // Gaming optimizations
        config.gaming.ultra_low_latency = true;
        config.gaming.predictive_keying = true;
        config.gaming.jitter_elimination = true;
        config.gaming.optimized_encryption = true;
        config.gaming.max_gaming_latency = Duration::from_millis(5);
        
        // Genetic healing for adaptive security
        config.genetic_healing.enable_healing = true;
        config.genetic_healing.enable_prediction = true;
        config.genetic_healing.response_time = Duration::from_millis(50);
        
        // High-performance monitoring
        config.monitoring.enable_detailed_monitoring = true;
        config.monitoring.monitoring_interval = Duration::from_millis(100);
        
        config
    }

    /// Create configuration for maximum security
    pub fn maximum_security() -> Self {
        let mut config = Self::default();
        
        // Hardware security requirements
        config.key_management.use_hardware_keys = true;
        config.key_management.enable_attestation = true;
        config.key_management.key_strength = 384; // Higher security
        config.key_management.key_rotation_interval = Duration::from_secs(900); // 15 min
        
        // Enhanced genetic healing
        config.genetic_healing.enable_healing = true;
        config.genetic_healing.enable_prediction = true;
        config.genetic_healing.enable_learning = true;
        config.genetic_healing.enable_adaptation = true;
        config.genetic_healing.confidence_threshold = 0.95;
        
        // Comprehensive monitoring
        config.monitoring.enable_detailed_monitoring = true;
        config.monitoring.enable_profiling = true;
        config.monitoring.enable_alerting = true;
        
        // Strict alert thresholds
        config.alert_thresholds.max_error_rate = 0.01;
        config.alert_thresholds.min_availability = 0.99;
        config.alert_thresholds.max_response_time = 100;
        
        config
    }
}

// BACKWARD COMPATIBILITY - Re-export as BStpConfig for existing code
/// Backward compatibility alias for BStpConfig
/// 
/// **DEPRECATION NOTICE**: Use `UnifiedTunnelConfig` instead.
/// This alias is provided for backward compatibility during migration.
pub type BStpConfig = UnifiedTunnelConfig;

// Re-export individual config types for backward compatibility
pub type PerformanceConfig = TunnelPerformanceConfig;
pub type KeyManagementConfig = TunnelKeyManagementConfig;
pub type GamingConfig = GamingOptimizationConfig;
pub type MonitoringConfig = TunnelMonitoringConfig;
pub type AlertThresholds = TunnelAlertThresholds;
