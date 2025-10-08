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
///
/// Configures how cryptographic keys are managed, rotated, and backed up
/// within the Hardware Security Module (HSM) system.
///
/// # Examples
///
/// ```rust
/// use beardog_types::canonical::config::type_aliases::HsmKeyManagementConfig;
/// use std::time::Duration;
///
/// let config = HsmKeyManagementConfig {
///     key_rotation_interval: Duration::from_secs(86400 * 30), // 30 days
///     backup_enabled: true,
///     hardware_backed: true,
///     key_derivation_algorithm: "PBKDF2-SHA256".to_string(),
/// };
/// ```
#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct HsmKeyManagementConfig {
    /// Interval between automatic key rotations
    ///
    /// Recommended: 30-90 days for production environments
    pub key_rotation_interval: Duration,

    /// Enable automatic key backups to secure storage
    pub backup_enabled: bool,

    /// Require hardware-backed key storage (TPM, Secure Enclave, etc.)
    pub hardware_backed: bool,

    /// Algorithm used for key derivation (e.g., "PBKDF2-SHA256", "Argon2id")
    pub key_derivation_algorithm: String,
}

/// HSM security policy configuration
///
/// Defines security policies and constraints for HSM operations,
/// including attestation requirements and algorithm restrictions.
///
/// # Examples
///
/// ```rust
/// use beardog_types::canonical::config::type_aliases::HsmSecurityPolicyConfig;
///
/// let policy = HsmSecurityPolicyConfig {
///     enforce_hardware_backing: true,
///     require_attestation: true,
///     audit_all_operations: true,
///     allowed_algorithms: vec![
///         "Ed25519".to_string(),
///         "AES-256-GCM".to_string(),
///     ],
/// };
/// ```
#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct HsmSecurityPolicyConfig {
    /// Enforce that all keys must be hardware-backed (no software fallback)
    pub enforce_hardware_backing: bool,

    /// Require HSM attestation for all cryptographic operations
    pub require_attestation: bool,

    /// Enable comprehensive audit logging for all HSM operations
    pub audit_all_operations: bool,

    /// Whitelist of allowed cryptographic algorithms (e.g., "Ed25519", "AES-256-GCM")
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
///
/// Configures automated database backups including frequency, retention,
/// compression, and encryption settings.
///
/// # Examples
///
/// ```rust
/// use beardog_types::canonical::config::type_aliases::DatabaseBackupConfig;
/// use std::time::Duration;
///
/// let backup = DatabaseBackupConfig {
///     enabled: true,
///     backup_interval: Duration::from_secs(3600), // Hourly backups
///     retention_days: 30,
///     compression_enabled: true,
///     encryption_enabled: true,
/// };
/// ```
#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct DatabaseBackupConfig {
    /// Enable automated database backups
    pub enabled: bool,

    /// Time interval between backup operations
    ///
    /// Recommended: 1-24 hours depending on data criticality
    pub backup_interval: Duration,

    /// Number of days to retain backup files before automatic deletion
    ///
    /// Recommended: 7-90 days depending on compliance requirements
    pub retention_days: u32,

    /// Enable compression for backup files to reduce storage costs
    pub compression_enabled: bool,

    /// Enable encryption for backup files (strongly recommended for production)
    pub encryption_enabled: bool,
}

// ================================================================================================
// MONITORING CONFIGURATION ALIASES
// ================================================================================================

/// Metrics collection configuration
///
/// Configures how system metrics are collected, stored, and exported
/// for monitoring and observability purposes.
///
/// # Examples
///
/// ```rust
/// use beardog_types::canonical::config::type_aliases::MetricsCollectionConfig;
/// use std::time::Duration;
///
/// let metrics = MetricsCollectionConfig {
///     enabled: true,
///     collection_interval: Duration::from_secs(60), // Collect every minute
///     retention_period: Duration::from_secs(86400 * 7), // Keep for 7 days
///     export_formats: vec!["prometheus".to_string(), "json".to_string()],
/// };
/// ```
#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct MetricsCollectionConfig {
    /// Enable metrics collection system-wide
    pub enabled: bool,

    /// Interval between metric collection cycles
    ///
    /// Recommended: 10-60 seconds for production monitoring
    pub collection_interval: Duration,

    /// How long to retain collected metrics before deletion
    ///
    /// Recommended: 7-30 days depending on storage capacity
    pub retention_period: Duration,

    /// Supported export formats (e.g., "prometheus", "json", "statsd")
    pub export_formats: Vec<String>,
}

/// Logging configuration
///
/// Configures application logging including log levels, output format,
/// and file rotation policies.
///
/// # Examples
///
/// ```rust
/// use beardog_types::canonical::config::type_aliases::LoggingConfig;
///
/// let logging = LoggingConfig {
///     level: "info".to_string(),
///     output_format: "json".to_string(),
///     file_rotation: true,
///     max_file_size_mb: 100,
/// };
/// ```
#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct LoggingConfig {
    /// Log level filter (e.g., "trace", "debug", "info", "warn", "error")
    pub level: String,

    /// Output format for log messages (e.g., "json", "text", "compact")
    pub output_format: String,

    /// Enable automatic log file rotation when size limit is reached
    pub file_rotation: bool,

    /// Maximum size of a single log file in megabytes before rotation
    ///
    /// Recommended: 50-200 MB for production systems
    pub max_file_size_mb: u64,
}

/// Tracing configuration
///
/// Configures distributed tracing for tracking requests across services,
/// enabling performance analysis and debugging in microservice architectures.
///
/// # Examples
///
/// ```rust
/// use beardog_types::canonical::config::type_aliases::TracingConfig;
///
/// let tracing = TracingConfig {
///     enabled: true,
///     sampling_rate: 0.1, // Sample 10% of requests
///     export_endpoint: Some("http://jaeger:14268/api/traces".to_string()),
///     trace_id_format: "w3c".to_string(),
/// };
/// ```
#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct TracingConfig {
    /// Enable distributed tracing system-wide
    pub enabled: bool,

    /// Fraction of requests to trace (0.0 = none, 1.0 = all)
    ///
    /// Recommended: 0.01-0.1 (1-10%) for production to balance overhead and visibility
    pub sampling_rate: f64,

    /// Optional endpoint for exporting traces (e.g., Jaeger, Zipkin, OTLP)
    pub export_endpoint: Option<String>,

    /// Trace ID format standard (e.g., "w3c", "b3", "jaeger")
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
///
/// Configures the alerting system for notifying operators of system issues,
/// including notification channels, severity levels, and rate limiting.
///
/// # Examples
///
/// ```rust
/// use beardog_types::canonical::config::type_aliases::AlertingConfig;
/// use std::collections::HashMap;
///
/// let mut severity_levels = HashMap::new();
/// severity_levels.insert("critical".to_string(), 1);
/// severity_levels.insert("warning".to_string(), 2);
///
/// let alerting = AlertingConfig {
///     enabled: true,
///     notification_channels: vec!["slack".to_string(), "email".to_string()],
///     severity_levels,
///     rate_limiting: true,
/// };
/// ```
#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct AlertingConfig {
    /// Enable the alerting system
    pub enabled: bool,

    /// Notification channels for alerts (e.g., "slack", "email", "pagerduty", "webhook")
    pub notification_channels: Vec<String>,

    /// Mapping of severity level names to numeric priorities (1 = highest)
    ///
    /// Example: "critical" → 1, "warning" → 2, "info" → 3
    pub severity_levels: HashMap<String, u8>,

    /// Enable rate limiting to prevent alert storms
    ///
    /// Recommended: true for production to avoid notification fatigue
    pub rate_limiting: bool,
}

/// Performance monitoring configuration
///
/// Configures which system resources to monitor for performance tracking
/// and capacity planning.
///
/// # Examples
///
/// ```rust
/// use beardog_types::canonical::config::type_aliases::PerformanceMonitoringConfig;
///
/// let perf_mon = PerformanceMonitoringConfig {
///     enabled: true,
///     cpu_monitoring: true,
///     memory_monitoring: true,
///     network_monitoring: true,
///     disk_monitoring: true,
/// };
/// ```
#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct PerformanceMonitoringConfig {
    /// Enable performance monitoring system-wide
    pub enabled: bool,

    /// Monitor CPU usage (utilization, load average, per-core stats)
    pub cpu_monitoring: bool,

    /// Monitor memory usage (RAM, swap, allocation patterns)
    pub memory_monitoring: bool,

    /// Monitor network I/O (bandwidth, packet loss, latency)
    pub network_monitoring: bool,

    /// Monitor disk I/O (read/write ops, latency, space usage)
    pub disk_monitoring: bool,
}

/// Audit logging configuration
///
/// Configures comprehensive audit logging for security, compliance,
/// and forensic analysis. Critical for regulated environments (HIPAA, PCI-DSS, SOC 2).
///
/// # Examples
///
/// ```rust
/// use beardog_types::canonical::config::type_aliases::AuditLoggingConfig;
///
/// let audit = AuditLoggingConfig {
///     enabled: true,
///     log_all_operations: true,
///     retention_days: 365, // 1 year for compliance
///     encryption_enabled: true,
///     tamper_detection: true,
/// };
/// ```
#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct AuditLoggingConfig {
    /// Enable audit logging (required for compliance environments)
    pub enabled: bool,

    /// Log all operations including read-only (comprehensive audit trail)
    ///
    /// Recommended: true for regulated environments, false for performance-sensitive systems
    pub log_all_operations: bool,

    /// Number of days to retain audit logs (must meet compliance requirements)
    ///
    /// Recommended: 90-365 days (GDPR: 90 days, SOC 2: 365 days, HIPAA: 2555 days)
    pub retention_days: u32,

    /// Encrypt audit logs at rest (strongly recommended)
    pub encryption_enabled: bool,

    /// Enable tamper detection with cryptographic signatures
    ///
    /// Recommended: true to ensure audit log integrity
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
