// SPDX-License-Identifier: AGPL-3.0-or-later

//! Logging domain types for [`super::SystemDomainConfig`].

use beardog_config::env_keys;
use beardog_errors::BearDogError;
use serde::{Deserialize, Serialize};
use std::collections::HashMap;

/// **CANONICAL** Logging configuration
///
/// This is the single source of truth for logging settings across `BearDog`.
/// Consolidates all `LoggingConfig` variants from:
/// - `beardog-types/src/canonical/config/type_aliases.rs`
/// - `beardog-types/src/canonical/monitoring_unified/logging.rs`
/// - `beardog-types/src/canonical/providers_unified/monitoring.rs`
/// - `beardog-types/src/canonical/providers/base.rs` (`LoggingConfiguration`)
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

impl LoggingConfig {
    /// Create `LoggingConfig` with hardcoded defaults
    pub fn with_defaults() -> Self {
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

    /// Create `LoggingConfig` from environment variables
    pub fn from_env() -> Self {
        let level = std::env::var(env_keys::ENV_LOG_LEVEL)
            .ok()
            .and_then(|s| match s.to_lowercase().as_str() {
                "trace" => Some(LogLevel::Trace),
                "debug" => Some(LogLevel::Debug),
                "info" => Some(LogLevel::Info),
                "warn" => Some(LogLevel::Warn),
                "error" => Some(LogLevel::Error),
                _ => None,
            })
            .unwrap_or(LogLevel::Info);

        Self {
            enabled: true,
            level,
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

impl Default for LoggingConfig {
    fn default() -> Self {
        Self::with_defaults()
    }
}

impl LogLevel {
    /// Default log level
    pub const DEFAULT: Self = Self::Info;

    /// Create `LogLevel` with defaults
    pub const fn with_defaults() -> Self {
        Self::DEFAULT
    }
}

impl Default for LogLevel {
    fn default() -> Self {
        Self::with_defaults()
    }
}

impl LogFormat {
    /// Default log format
    pub const DEFAULT: Self = Self::Json;

    /// Create `LogFormat` with defaults
    pub const fn with_defaults() -> Self {
        Self::DEFAULT
    }
}

impl Default for LogFormat {
    fn default() -> Self {
        Self::with_defaults()
    }
}

impl LogRotationConfig {
    /// Default maximum log file size in MB
    pub const DEFAULT_MAX_SIZE_MB: u64 = 100;

    /// Default maximum number of log files to keep
    pub const DEFAULT_MAX_FILES: u32 = 10;

    /// Create `LogRotationConfig` with hardcoded defaults
    ///
    /// This method is deterministic and safe for concurrent use.
    /// No environment variables are read.
    pub const fn with_defaults() -> Self {
        Self {
            max_size_mb: Self::DEFAULT_MAX_SIZE_MB,
            max_files: Self::DEFAULT_MAX_FILES,
            frequency: LogRotationFrequency::Daily,
        }
    }

    /// Create `LogRotationConfig` from environment variables
    ///
    /// Reads configuration from environment, falling back to defaults.
    ///
    /// # Environment Variables
    /// - `BEARDOG_SYSTEM_LOG_MAX_SIZE_MB`: Max log file size in MB (default: 100)
    /// - `BEARDOG_SYSTEM_LOG_MAX_FILES`: Max number of log files (default: 10)
    pub fn from_env() -> Self {
        Self::from_env_provider(|k| std::env::var(k).ok())
    }

    /// Load from a custom environment provider (e.g. tests); production uses [`Self::from_env`].
    pub fn from_env_provider(get: impl Fn(&str) -> Option<String>) -> Self {
        Self {
            max_size_mb: get("BEARDOG_SYSTEM_LOG_MAX_SIZE_MB")
                .and_then(|s| s.parse().ok())
                .unwrap_or(Self::DEFAULT_MAX_SIZE_MB),
            max_files: get("BEARDOG_SYSTEM_LOG_MAX_FILES")
                .and_then(|s| s.parse().ok())
                .unwrap_or(Self::DEFAULT_MAX_FILES),
            frequency: LogRotationFrequency::Daily,
        }
    }
}

impl Default for LogRotationConfig {
    fn default() -> Self {
        Self::with_defaults()
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
    ///
    /// # Errors
    ///
    /// Returns an error if logging is enabled without targets or rotation limits are zero.
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
