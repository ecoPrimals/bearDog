//! System Domain Configuration
//!
//! Core system configuration including logging, threading, resource management,
//! and application lifecycle settings.

use beardog_errors::BearDogError;
use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use std::time::Duration;

/// System domain configuration
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SystemDomainConfig {
    /// Application metadata
    pub application: ApplicationConfig,
    /// Logging configuration
    pub logging: LoggingConfig,
    /// Threading and concurrency configuration
    pub threading: ThreadingConfig,
    /// Resource limits and management
    pub resources: ResourceConfig,
    /// Environment-specific settings
    pub environment: EnvironmentConfig,
}

/// Application configuration
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ApplicationConfig {
    /// Application name
    pub name: String,
    /// Application version
    pub version: String,
    /// Instance identifier
    pub instance_id: String,
    /// Application description
    pub description: String,
    /// Feature flags
    pub features: HashMap<String, bool>,
}

/// **CANONICAL** Logging configuration
///
/// This is the single source of truth for logging settings across BearDog.
/// Consolidates all LoggingConfig variants from:
/// - `beardog-types/src/canonical/config/type_aliases.rs`
/// - `beardog-types/src/canonical/monitoring_unified/logging.rs`
/// - `beardog-types/src/canonical/providers_unified/monitoring.rs`
/// - `beardog-types/src/canonical/providers/base.rs` (LoggingConfiguration)
/// - `beardog-production/src/config_management.rs`
/// - `beardog-core/src/ai/hybrid_intelligence/types.rs`
///
/// # Features
/// - Typed log levels (Trace, Debug, Info, Warn, Error)
/// - Multiple output targets (stdout, file, syslog, etc.)
/// - Structured logging support
/// - Log rotation configuration
/// - Correlation ID tracking
/// - Performance and audit logging
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
pub struct LoggingConfig {
    /// Enable logging
    pub enabled: bool,

    /// Log level
    pub level: LogLevel,

    /// Log format
    pub format: LogFormat,

    /// Log output targets
    pub targets: Vec<LogTarget>,

    /// Enable structured logging
    pub structured: bool,

    /// Log rotation settings
    pub rotation: Option<LogRotationConfig>,

    /// Enable correlation ID tracking for request tracing
    pub correlation_id_tracking: bool,

    /// Enable performance logging (timing, metrics)
    pub performance_logging: bool,

    /// Enable audit logging (security events, access logs)
    pub audit_logging: bool,
}

/// Log level enumeration
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq, PartialOrd, Ord)]
pub enum LogLevel {
    /// Trace level (most verbose)
    Trace,
    /// Debug level
    Debug,
    /// Info level (default)
    Info,
    /// Warning level
    Warn,
    /// Error level (least verbose)
    Error,
}

/// Log format enumeration
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
pub enum LogFormat {
    /// JSON format (machine-readable)
    Json,
    /// Plain text format (human-readable)
    Text,
    /// Structured format (key=value pairs)
    Structured,
    /// Compact format (minimal output)
    Compact,
}

/// Log target configuration
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
pub struct LogTarget {
    /// Target type
    pub target_type: LogTargetType,

    /// Target-specific configuration
    pub config: HashMap<String, String>,
}

/// Log target type enumeration
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
pub enum LogTargetType {
    /// Standard output
    Stdout,
    /// Standard error
    Stderr,
    /// File output
    File,
    /// Syslog
    Syslog,
    /// Remote logging service
    Remote,
    /// Database logging
    Database,
    /// Custom target
    Custom(String),
}

/// Log rotation configuration
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
pub struct LogRotationConfig {
    /// Maximum file size before rotation (in MB)
    pub max_size_mb: u64,

    /// Maximum number of log files to keep
    pub max_files: u32,

    /// Rotation frequency
    pub frequency: LogRotationFrequency,
}

/// Log rotation frequency
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
pub enum LogRotationFrequency {
    /// Rotate hourly
    Hourly,
    /// Rotate daily
    Daily,
    /// Rotate weekly
    Weekly,
    /// Rotate monthly
    Monthly,
    /// Rotate based on size only
    SizeOnly,
}

impl Default for LoggingConfig {
    fn default() -> Self {
        Self {
            enabled: true,
            level: LogLevel::Info,
            format: LogFormat::Json,
            targets: vec![LogTarget {
                target_type: LogTargetType::Stdout,
                config: HashMap::new(),
            }],
            structured: true,
            rotation: None,
            correlation_id_tracking: true,
            performance_logging: false,
            audit_logging: false,
        }
    }
}

impl Default for LogLevel {
    fn default() -> Self {
        Self::Info
    }
}

impl Default for LogFormat {
    fn default() -> Self {
        Self::Json
    }
}

impl Default for LogRotationConfig {
    fn default() -> Self {
        Self {
            max_size_mb: std::env::var("BEARDOG_SYSTEM_LOG_MAX_SIZE_MB")
                .ok()
                .and_then(|s| s.parse().ok())
                .unwrap_or(100),
            max_files: std::env::var("BEARDOG_SYSTEM_LOG_MAX_FILES")
                .ok()
                .and_then(|s| s.parse().ok())
                .unwrap_or(10),
            frequency: LogRotationFrequency::Daily,
        }
    }
}

impl LoggingConfig {
    /// Create a minimal configuration for stdout only
    pub fn stdout_only() -> Self {
        Self {
            targets: vec![LogTarget {
                target_type: LogTargetType::Stdout,
                config: HashMap::new(),
            }],
            ..Default::default()
        }
    }

    /// Create a file-based configuration with rotation
    pub fn file_with_rotation(path: &str) -> Self {
        let mut config = HashMap::new();
        config.insert("path".to_string(), path.to_string());

        Self {
            targets: vec![LogTarget {
                target_type: LogTargetType::File,
                config,
            }],
            rotation: Some(LogRotationConfig::default()),
            ..Default::default()
        }
    }

    /// Validate logging configuration
    pub fn validate(&self) -> Result<(), BearDogError> {
        if self.enabled && self.targets.is_empty() {
            return Err(BearDogError::configuration(
                "Logging enabled but no targets configured",
            ));
        }

        if let Some(rotation) = &self.rotation {
            if rotation.max_size_mb == 0 {
                return Err(BearDogError::configuration(
                    "Log rotation max_size_mb cannot be zero",
                ));
            }
            if rotation.max_files == 0 {
                return Err(BearDogError::configuration(
                    "Log rotation max_files cannot be zero",
                ));
            }
        }

        Ok(())
    }
}

/// Backward compatibility alias
#[deprecated(since = "3.1.0", note = "Use LoggingConfig instead")]
pub type LoggingConfiguration = LoggingConfig;

/// Threading configuration
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ThreadingConfig {
    /// Number of worker threads
    pub worker_threads: usize,
    /// Thread pool size for blocking operations
    pub blocking_threads: usize,
    /// Thread stack size
    pub stack_size: Option<usize>,
    /// Enable thread-local storage optimization
    pub enable_tls_optimization: bool,
}

/// Resource configuration
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ResourceConfig {
    /// Maximum memory usage (bytes)
    pub max_memory_bytes: Option<u64>,
    /// Maximum file descriptors
    pub max_file_descriptors: Option<u64>,
    /// Maximum concurrent connections
    pub max_connections: usize,
    /// Resource monitoring interval
    pub monitoring_interval: Duration,
}

/// Environment configuration
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct EnvironmentConfig {
    /// Environment type (development, testing, staging, production)
    pub environment_type: String,
    /// Environment-specific variables
    pub variables: HashMap<String, String>,
    /// Configuration overrides
    pub overrides: HashMap<String, serde_json::Value>,
}

impl Default for SystemDomainConfig {
    fn default() -> Self {
        Self {
            application: ApplicationConfig::default(),
            logging: LoggingConfig::default(),
            threading: ThreadingConfig::default(),
            resources: ResourceConfig::default(),
            environment: EnvironmentConfig::default(),
        }
    }
}

impl Default for ApplicationConfig {
    fn default() -> Self {
        Self {
            name: "BearDog".to_string(),
            version: env!("CARGO_PKG_VERSION").to_string(),
            instance_id: uuid::Uuid::new_v4().to_string(),
            description: "BearDog Security Provider".to_string(),
            features: HashMap::new(),
        }
    }
}

// LoggingConfig and LogRotationConfig Default implementations moved to their type definitions above

impl Default for ThreadingConfig {
    fn default() -> Self {
        Self {
            worker_threads: std::thread::available_parallelism()
                .map(std::num::NonZeroUsize::get)
                .unwrap_or(4),
            blocking_threads: crate::constants::domains::system::defaults::DEFAULT_CACHE_SIZE,
            stack_size: None,
            enable_tls_optimization: true,
        }
    }
}

impl Default for ResourceConfig {
    fn default() -> Self {
        Self {
            max_memory_bytes: None,
            max_file_descriptors: Some(
                std::env::var("BEARDOG_MAX_FILE_DESCRIPTORS")
                    .ok()
                    .and_then(|f| f.parse().ok())
                    .unwrap_or(65536), // 64K default
            ),
            max_connections: std::env::var("BEARDOG_SYSTEM_MAX_CONNECTIONS")
                .ok()
                .and_then(|c| c.parse().ok())
                .unwrap_or(10000), // 10K connections default
            monitoring_interval: Duration::from_secs(
                std::env::var("BEARDOG_SYSTEM_MONITORING_INTERVAL_SECS")
                    .ok()
                    .and_then(|s| s.parse().ok())
                    .unwrap_or(60),
            ),
        }
    }
}

impl Default for EnvironmentConfig {
    fn default() -> Self {
        Self {
            environment_type: std::env::var("BEARDOG_ENVIRONMENT")
                .unwrap_or_else(|_| "development".to_string()),
            variables: HashMap::new(),
            overrides: HashMap::new(),
        }
    }
}

impl SystemDomainConfig {
    /// Load system configuration from environment variables
    pub fn from_env() -> Result<Self, BearDogError> {
        let mut config = Self::default();

        // Load environment type
        if let Ok(env_type) = std::env::var("BEARDOG_ENVIRONMENT") {
            config.environment.environment_type = env_type;
        }

        // Load logging configuration
        if let Ok(log_level) = std::env::var("BEARDOG_LOG_LEVEL") {
            config.logging.level = match log_level.to_lowercase().as_str() {
                "trace" => LogLevel::Trace,
                "debug" => LogLevel::Debug,
                "info" => LogLevel::Info,
                "warn" | "warning" => LogLevel::Warn,
                "error" => LogLevel::Error,
                _ => LogLevel::Info, // Default to Info if invalid
            };
        }

        if let Ok(log_format) = std::env::var("BEARDOG_LOG_FORMAT") {
            config.logging.format = match log_format.to_lowercase().as_str() {
                "json" => LogFormat::Json,
                "text" => LogFormat::Text,
                "structured" => LogFormat::Structured,
                "compact" => LogFormat::Compact,
                _ => LogFormat::Json, // Default to JSON if invalid
            };
        }

        // Load threading configuration
        if let Ok(worker_threads) = std::env::var("BEARDOG_WORKER_THREADS") {
            config.threading.worker_threads = worker_threads
                .parse()
                .map_err(|_| BearDogError::validation("Invalid BEARDOG_WORKER_THREADS value"))?;
        }

        // Load resource limits
        if let Ok(max_connections) = std::env::var("BEARDOG_MAX_CONNECTIONS") {
            config.resources.max_connections = max_connections
                .parse()
                .map_err(|_| BearDogError::validation("Invalid BEARDOG_MAX_CONNECTIONS value"))?;
        }

        Ok(config)
    }

    /// Validate system configuration
    pub fn validate(&self) -> Result<(), BearDogError> {
        // Validate threading configuration
        if self.threading.worker_threads == 0 {
            return Err(BearDogError::validation(
                "Worker threads must be greater than 0",
            ));
        }

        if self.threading.blocking_threads == 0 {
            return Err(BearDogError::validation(
                "Blocking threads must be greater than 0",
            ));
        }

        // Validate resource limits
        if self.resources.max_connections == 0 {
            return Err(BearDogError::validation(
                "Max connections must be greater than 0",
            ));
        }

        // Validate logging configuration (delegates to LoggingConfig::validate)
        self.logging.validate()?;

        Ok(())
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_log_level_default() {
        assert_eq!(LogLevel::default(), LogLevel::Info);
    }

    #[test]
    fn test_log_level_ordering() {
        assert!(LogLevel::Trace < LogLevel::Debug);
        assert!(LogLevel::Debug < LogLevel::Info);
        assert!(LogLevel::Info < LogLevel::Warn);
        assert!(LogLevel::Warn < LogLevel::Error);
    }

    #[test]
    fn test_logging_config_default() {
        let config = LoggingConfig::default();
        assert!(config.enabled);
        assert_eq!(config.level, LogLevel::Info);
        assert_eq!(config.format, LogFormat::Json);
        assert_eq!(config.targets.len(), 1);
        assert!(config.structured);
    }

    #[test]
    fn test_logging_config_stdout_only() {
        let config = LoggingConfig::stdout_only();
        assert_eq!(config.targets.len(), 1);
        assert_eq!(config.targets[0].target_type, LogTargetType::Stdout);
    }

    #[test]
    fn test_logging_config_file_with_rotation() {
        let config = LoggingConfig::file_with_rotation("/var/log/beardog.log");
        assert_eq!(config.targets.len(), 1);
        assert_eq!(config.targets[0].target_type, LogTargetType::File);
        assert!(config.rotation.is_some());
    }

    #[test]
    fn test_logging_config_validate_success() {
        let config = LoggingConfig::default();
        assert!(config.validate().is_ok());
    }

    #[test]
    fn test_logging_config_validate_no_targets() {
        let mut config = LoggingConfig::default();
        config.targets.clear();
        assert!(config.validate().is_err());
    }

    #[test]
    fn test_system_domain_config_default() {
        let config = SystemDomainConfig::default();
        assert_eq!(config.application.name, "BearDog");
        assert!(config.logging.enabled);
        assert!(config.threading.worker_threads > 0);
    }

    #[test]
    fn test_system_domain_config_from_env() {
        let result = SystemDomainConfig::from_env();
        assert!(result.is_ok());
    }

    #[test]
    fn test_system_domain_config_validate_success() {
        let config = SystemDomainConfig::default();
        assert!(config.validate().is_ok());
    }

    #[test]
    fn test_system_domain_config_validate_zero_worker_threads() {
        let mut config = SystemDomainConfig::default();
        config.threading.worker_threads = 0;
        assert!(config.validate().is_err());
    }

    #[test]
    fn test_threading_config_default() {
        let config = ThreadingConfig::default();
        assert!(config.worker_threads > 0);
        assert!(config.enable_tls_optimization);
    }

    #[test]
    fn test_resource_config_default() {
        let config = ResourceConfig::default();
        assert!(config.max_connections > 0);
        assert!(config.monitoring_interval > Duration::from_secs(0));
    }
}
