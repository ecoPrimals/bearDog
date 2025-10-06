// **PEDANTIC TYPE ALIASES**
//
// This module provides comprehensive type aliases to resolve all missing type references
// during the unified configuration system integration.
//
// Note: Some types in this module are intentionally deprecated with clear migration paths.

#![allow(deprecated)]

use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use std::time::Duration;

// ================================================================================================
// HSM CONFIGURATION ALIASES
// ================================================================================================

/// HSM key management configuration
#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct HsmKeyManagementConfig {
    /// Key Rotation Interval
    /// The key rotation interval value
    pub key_rotation_interval: Duration,
    /// Backup Enabled
    /// Whether backup is enabled
    pub backup_enabled: bool,
    /// Hardware Backed
    /// Whether `hardware_backed` is enabled
    pub hardware_backed: bool,
    /// Key Derivation Algorithm
    /// The key derivation algorithm value
    pub key_derivation_algorithm: String,
}

/// HSM security policy configuration
#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct HsmSecurityPolicyConfig {
    pub enforce_hardware_backing: bool,
    /// Require Attestation
    /// Whether `require_attestation` is enabled
    pub require_attestation: bool,
    /// Audit All Operations
    /// Whether `audit_all_operations` is enabled
    pub audit_all_operations: bool,
    /// Allowed Algorithms
    /// Collection of allowed algorithms
    pub allowed_algorithms: Vec<String>,
}

/// HSM backup configuration
#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct HsmBackupConfig {
    /// Enabled
    /// Whether feature is enabled
    pub enabled: bool,
    /// Backup Interval
    /// The backup interval value
    pub backup_interval: Duration,
    /// Encryption Enabled
    /// Whether encryption is enabled
    pub encryption_enabled: bool,
    /// Remote Backup Url
    /// Optional remote backup url
    pub remote_backup_url: Option<String>,
}

// ================================================================================================
// DATABASE CONFIGURATION ALIASES
// ================================================================================================

/// Database backup configuration
#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct DatabaseBackupConfig {
    /// Enabled
    /// Whether feature is enabled
    pub enabled: bool,
    /// Backup Interval
    /// The backup interval value
    pub backup_interval: Duration,
    /// Retention Days
    /// Number of `retention_days`
    pub retention_days: u32,
    /// Compression Enabled
    /// Whether compression is enabled
    pub compression_enabled: bool,
    /// Encryption Enabled
    /// Whether encryption is enabled
    pub encryption_enabled: bool,
}

// ================================================================================================
// MONITORING CONFIGURATION ALIASES
// ================================================================================================

/// Metrics collection configuration
#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct MetricsCollectionConfig {
    /// Enabled
    /// Whether feature is enabled
    pub enabled: bool,
    /// Collection Interval
    /// The collection interval value
    pub collection_interval: Duration,
    /// Retention Period
    /// The retention period value
    pub retention_period: Duration,
    /// Export Formats
    pub export_formats: Vec<String>,
}

/// Logging configuration
#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct LoggingConfig {
    /// Level
    /// The level value
    pub level: String,
    /// Output Format
    pub output_format: String,
    /// File Rotation
    /// Whether `file_rotation` is enabled
    pub file_rotation: bool,
    /// Max File Size Mb
    /// Number of `max_file_size_mb`
    pub max_file_size_mb: u64,
}

/// Tracing configuration
#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct TracingConfig {
    /// Enabled
    /// Whether feature is enabled
    pub enabled: bool,
    /// Sampling Rate
    /// The sampling rate value
    pub sampling_rate: f64,
    /// Export Endpoint
    /// Optional export endpoint
    pub export_endpoint: Option<String>,
    /// Trace Id Format
    pub trace_id_format: String,
}

/// Generic health check configuration (DEPRECATED)
///
/// This generic health check config is being phased out in favor of
/// domain-specific configurations. Use the appropriate domain config instead:
/// - Production: `canonical::config::production::operations::HealthCheckConfig`
/// - Discovery: `canonical::config::discovery::HealthCheckConfig`
/// - Network: `canonical::network::HealthCheckConfig`
/// - Services: `canonical::services::endpoints::HealthCheckConfig`
/// - AI: `beardog_core::ai::hybrid_intelligence::types::HealthCheckConfig`
/// Health check configuration
///
/// **DEPRECATED**: Use `super::domains::network::monitoring::HealthCheckConfiguration` instead.
///
/// For specialized health checks, see:
/// - Production: `super::production::operations::HealthCheckConfig`
/// - Discovery: `super::discovery::HealthCheckConfig`
/// - Services: `canonical::services::endpoints::HealthCheckConfig`
///
/// This type alias will be removed in v3.3.0.
#[deprecated(
    since = "3.1.0",
    note = "Use canonical::config::domains::network::monitoring::HealthCheckConfiguration instead"
)]
#[allow(deprecated)]
pub type HealthCheckConfig = super::domains::network::monitoring::HealthCheckConfiguration;

/// Alerting configuration
#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct AlertingConfig {
    /// Enabled
    /// Whether feature is enabled
    pub enabled: bool,
    /// Notification Channels
    /// Collection of notification channels
    pub notification_channels: Vec<String>,
    /// Severity Levels
    /// Mapping of severity levels
    pub severity_levels: HashMap<String, u8>,
    /// Rate Limiting
    /// Whether `rate_limiting` is enabled
    pub rate_limiting: bool,
}

#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct PerformanceMonitoringConfig {
    /// Enabled
    /// Whether feature is enabled
    pub enabled: bool,
    /// Cpu Monitoring
    /// Whether `cpu_monitoring` is enabled
    pub cpu_monitoring: bool,
    /// Memory Monitoring
    /// Whether `memory_monitoring` is enabled
    pub memory_monitoring: bool,
    /// Network Monitoring
    /// Whether `network_monitoring` is enabled
    pub network_monitoring: bool,
    /// Disk Monitoring
    /// Whether `disk_monitoring` is enabled
    pub disk_monitoring: bool,
}

/// Audit logging configuration
#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct AuditLoggingConfig {
    /// Enabled
    /// Whether feature is enabled
    pub enabled: bool,
    /// Log All Operations
    /// Whether `log_all_operations` is enabled
    pub log_all_operations: bool,
    /// Retention Days
    /// Number of `retention_days`
    pub retention_days: u32,
    /// Encryption Enabled
    /// Whether encryption is enabled
    pub encryption_enabled: bool,
    /// Tamper Detection
    /// Whether `tamper_detection` is enabled
    pub tamper_detection: bool,
}

// ================================================================================================
// GENETICS CONFIGURATION ALIASES
// ================================================================================================

/// Entropy collection configuration
#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct EntropyCollectionConfig {
    /// Enabled
    /// Whether feature is enabled
    pub enabled: bool,
    /// Collection Sources
    /// Collection of collection sources
    pub collection_sources: Vec<String>,
    /// Quality Threshold
    /// The quality threshold value
    pub quality_threshold: f64,
    /// Buffer Size
    /// Number of `buffer_size`
    pub buffer_size: usize,
}

/// Human entropy configuration
#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct HumanEntropyConfig {
    /// Enabled
    /// Whether feature is enabled
    pub enabled: bool,
    /// Collection Methods
    /// Collection of collection methods
    pub collection_methods: Vec<String>,
    /// Quality Analysis
    /// Whether `quality_analysis` is enabled
    pub quality_analysis: bool,
    /// Privacy Protection
    /// Whether `privacy_protection` is enabled
    pub privacy_protection: bool,
}

/// Genetic algorithm configuration
#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct GeneticAlgorithmConfig {
    /// Enabled
    /// Whether feature is enabled
    pub enabled: bool,
    /// Population Size
    /// Number of `population_size`
    pub population_size: usize,
    /// Mutation Rate
    /// The mutation rate value
    pub mutation_rate: f64,
    /// Crossover Rate
    /// The crossover rate value
    pub crossover_rate: f64,
    /// Selection Method
    /// The selection method value
    pub selection_method: String,
}

/// SIMD optimization configuration
#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct SimdOptimizationConfig {
    /// Enabled
    /// Whether feature is enabled
    pub enabled: bool,
    /// Instruction Sets
    /// Collection of instruction sets
    pub instruction_sets: Vec<String>,
    /// Auto Detection
    /// Whether `auto_detection` is enabled
    pub auto_detection: bool,
    /// Fallback Enabled
    /// Whether fallback is enabled
    pub fallback_enabled: bool,
}

/// Entropy hierarchy configuration
#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct EntropyHierarchyConfig {
    /// Levels
    /// Number of levels
    pub levels: u32,
    /// Quality Requirements
    /// Mapping of quality requirements
    pub quality_requirements: HashMap<String, f64>,
    /// Mixing Algorithms
    /// Collection of mixing algorithms
    pub mixing_algorithms: Vec<String>,
    /// Verification Enabled
    /// Whether verification is enabled
    pub verification_enabled: bool,
}

// ================================================================================================
// ENVIRONMENT AND DEPLOYMENT ALIASES
// ================================================================================================

/// Environment enumeration
#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub enum Environment {
    #[default]
    /// Represents development variant
    Development,
    /// Testing variant
    Testing,
    /// Staging variant
    Staging,
    /// Production variant
    Production,
    /// Custom environment with user-defined name
    Custom(String),
}

/// Deployment mode enumeration
#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub enum DeploymentMode {
    /// Standalone deployment on a single node
    #[default]
    /// Represents standalone variant
    Standalone,
    /// Distributed variant
    Distributed,
    /// Cloud variant
    Cloud,
    /// Edge variant
    Edge,
    /// Hybrid variant
    Hybrid,
}

#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct UnifiedVersionInfo {
    /// Major
    /// Number of major
    pub major: u32,
    /// Minor
    /// Number of minor
    pub minor: u32,
    /// Patch
    /// Number of patch
    pub patch: u32,
    /// Pre Release
    /// Optional pre release
    pub pre_release: Option<String>,
    /// Build Metadata
    /// Optional build metadata
    pub build_metadata: Option<String>,
}

/// Rollout configuration
#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct RolloutConfig {
    /// Enabled
    /// Whether feature is enabled
    pub enabled: bool,
    /// Percentage
    /// The percentage value
    pub percentage: f64,
    /// Target Groups
    /// Collection of target groups
    pub target_groups: Vec<String>,
    /// Rollback Threshold
    /// The rollback threshold value
    pub rollback_threshold: f64,
}

// Additional enums needed
#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub enum LogLevel {
    #[default]
    /// Represents info variant
    Info,
    /// Debug variant
    Debug,
    /// Warn variant
    Warn,
    /// Error variant
    Error,
    /// Trace variant
    Trace,
}

#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub enum PasswordSource {
    /// Read password from environment variables (default)
    #[default]
    /// Represents environment variant
    Environment,
    /// Read password from specified file path
    File(String),
    /// Retrieve password from specified vault location
    Vault(String),
    /// Generated variant
    Generated,
}
