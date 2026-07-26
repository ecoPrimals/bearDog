// SPDX-License-Identifier: AGPL-3.0-or-later

//! System Domain Configuration
//!
//! Core system configuration including logging, threading, resource management,
//! and application lifecycle settings.

use beardog_config::env_keys;
use beardog_errors::BearDogError;
use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use std::time::Duration;

/// System domain configuration
#[derive(Debug, Clone, Serialize, Deserialize, Default)]
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

impl ApplicationConfig {
    /// Default application name
    pub const DEFAULT_NAME: &'static str = "BearDog";

    /// Default application description
    pub const DEFAULT_DESCRIPTION: &'static str = "BearDog Security Provider";

    /// Create `ApplicationConfig` with defaults (no fixed product instance identity).
    ///
    /// Version and instance ID are taken from `BEARDOG_APP_VERSION` and `BEARDOG_INSTANCE_ID`
    /// when set; otherwise version is the crate version at compile time and instance ID is
    /// `instance-{pid}` for stable local defaults without embedding a named deployment.
    #[must_use]
    pub fn with_defaults() -> Self {
        Self {
            name: Self::DEFAULT_NAME.to_string(),
            version: std::env::var(env_keys::ENV_APP_VERSION)
                .unwrap_or_else(|_| env!("CARGO_PKG_VERSION").to_string()),
            instance_id: std::env::var(env_keys::ENV_INSTANCE_ID)
                .unwrap_or_else(|_| format!("instance-{}", std::process::id())),
            description: Self::DEFAULT_DESCRIPTION.to_string(),
            features: HashMap::new(),
        }
    }

    /// Create `ApplicationConfig` from environment variables and system detection
    ///
    /// Reads configuration from environment, falling back to detected values.
    ///
    /// # Environment Variables
    /// - `BEARDOG_APP_NAME`: Application name override
    /// - `BEARDOG_APP_DESCRIPTION`: Application description override
    /// - `BEARDOG_APP_VERSION`: Version string (defaults to compile-time crate version)
    /// - `BEARDOG_INSTANCE_ID`: Instance identifier (defaults to a new UUID)
    ///
    /// # System Detection
    /// - Version: `BEARDOG_APP_VERSION` or `CARGO_PKG_VERSION` at compile time
    /// - Instance ID: `BEARDOG_INSTANCE_ID` or random UUID
    #[must_use]
    pub fn from_env() -> Self {
        Self::from_env_provider(|k| std::env::var(k).ok())
    }

    /// Load from a custom environment provider (e.g. tests); production uses [`Self::from_env`].
    pub fn from_env_provider(get: impl Fn(&str) -> Option<String>) -> Self {
        Self {
            name: get(env_keys::ENV_APP_NAME).unwrap_or_else(|| Self::DEFAULT_NAME.to_string()),
            version: get(env_keys::ENV_APP_VERSION)
                .unwrap_or_else(|| env!("CARGO_PKG_VERSION").to_string()),
            instance_id: get(env_keys::ENV_INSTANCE_ID)
                .unwrap_or_else(|| uuid::Uuid::new_v4().to_string()),
            description: get(env_keys::ENV_APP_DESCRIPTION)
                .unwrap_or_else(|| Self::DEFAULT_DESCRIPTION.to_string()),
            features: HashMap::new(),
        }
    }
}

#[path = "system_logging.rs"]
mod system_logging;

#[allow(
    clippy::wildcard_imports,
    reason = "facade re-export: single public surface for system logging config"
)]
pub use system_logging::*;

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

impl ThreadingConfig {
    /// Default worker threads
    pub const DEFAULT_WORKER_THREADS: usize = 4;

    /// Default blocking threads
    pub const DEFAULT_BLOCKING_THREADS: usize =
        crate::constants::domains::system::defaults::DEFAULT_CACHE_SIZE;

    /// Create `ThreadingConfig` with hardcoded defaults
    ///
    /// This method is deterministic and safe for concurrent use.
    /// No environment variables or system queries are performed.
    #[must_use]
    pub const fn with_defaults() -> Self {
        Self {
            worker_threads: Self::DEFAULT_WORKER_THREADS,
            blocking_threads: Self::DEFAULT_BLOCKING_THREADS,
            stack_size: None,
            enable_tls_optimization: true,
        }
    }

    /// Create `ThreadingConfig` from environment variables and system detection
    ///
    /// Reads configuration from environment, falling back to system detection.
    ///
    /// # Environment Variables
    /// - `BEARDOG_WORKER_THREADS`: Number of worker threads (default: detected from system)
    /// - `BEARDOG_BLOCKING_THREADS`: Number of blocking threads (default: `DEFAULT_CACHE_SIZE`)
    #[must_use]
    pub fn from_env() -> Self {
        Self::from_env_provider(|k| std::env::var(k).ok())
    }

    /// Load from a custom environment provider (e.g. tests); production uses [`Self::from_env`].
    pub fn from_env_provider(get: impl Fn(&str) -> Option<String>) -> Self {
        let worker_threads = get(env_keys::ENV_WORKER_THREADS)
            .and_then(|v| v.parse().ok())
            .unwrap_or_else(|| {
                std::thread::available_parallelism()
                    .map(std::num::NonZeroUsize::get)
                    .unwrap_or(Self::DEFAULT_WORKER_THREADS)
            });

        let blocking_threads = get(env_keys::ENV_BLOCKING_THREADS)
            .and_then(|v| v.parse().ok())
            .unwrap_or(Self::DEFAULT_BLOCKING_THREADS);

        Self {
            worker_threads,
            blocking_threads,
            stack_size: None,
            enable_tls_optimization: true,
        }
    }
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

impl Default for ApplicationConfig {
    fn default() -> Self {
        Self::with_defaults()
    }
}

// LoggingConfig and LogRotationConfig Default implementations moved to their type definitions above

impl Default for ThreadingConfig {
    fn default() -> Self {
        Self::with_defaults()
    }
}

impl ResourceConfig {
    /// Default maximum file descriptors
    pub const DEFAULT_MAX_FILE_DESCRIPTORS: u64 = 65536;

    /// Default maximum connections
    pub const DEFAULT_MAX_CONNECTIONS: usize = 10000;

    /// Default monitoring interval in seconds
    pub const DEFAULT_MONITORING_INTERVAL_SECS: u64 = 60;

    /// Create `ResourceConfig` with hardcoded defaults
    ///
    /// This method is deterministic and safe for concurrent use.
    /// No environment variables are read.
    #[must_use]
    pub const fn with_defaults() -> Self {
        Self {
            max_memory_bytes: None,
            max_file_descriptors: Some(Self::DEFAULT_MAX_FILE_DESCRIPTORS),
            max_connections: Self::DEFAULT_MAX_CONNECTIONS,
            monitoring_interval: Duration::from_secs(Self::DEFAULT_MONITORING_INTERVAL_SECS),
        }
    }

    /// Create `ResourceConfig` from environment variables
    ///
    /// Reads configuration from environment, falling back to defaults.
    ///
    /// # Environment Variables
    /// - `BEARDOG_MAX_FILE_DESCRIPTORS`: Maximum file descriptors (default: 65536)
    /// - `BEARDOG_SYSTEM_MAX_CONNECTIONS`: Maximum connections (default: 10000)
    /// - `BEARDOG_SYSTEM_MONITORING_INTERVAL_SECS`: Monitoring interval (default: 60)
    #[must_use]
    pub fn from_env() -> Self {
        Self::from_env_provider(|k| std::env::var(k).ok())
    }

    /// Load from a custom environment provider (e.g. tests); production uses [`Self::from_env`].
    pub fn from_env_provider(get: impl Fn(&str) -> Option<String>) -> Self {
        Self {
            max_memory_bytes: None,
            max_file_descriptors: Some(
                get(env_keys::ENV_MAX_FILE_DESCRIPTORS)
                    .and_then(|f| f.parse().ok())
                    .unwrap_or(Self::DEFAULT_MAX_FILE_DESCRIPTORS),
            ),
            max_connections: get(env_keys::ENV_SYSTEM_MAX_CONNECTIONS)
                .and_then(|c| c.parse().ok())
                .unwrap_or(Self::DEFAULT_MAX_CONNECTIONS),
            monitoring_interval: Duration::from_secs(
                get(env_keys::ENV_SYSTEM_MONITORING_INTERVAL_SECS)
                    .and_then(|s| s.parse().ok())
                    .unwrap_or(Self::DEFAULT_MONITORING_INTERVAL_SECS),
            ),
        }
    }
}

impl Default for ResourceConfig {
    fn default() -> Self {
        Self::with_defaults()
    }
}

impl EnvironmentConfig {
    /// Default environment type
    pub const DEFAULT_ENVIRONMENT_TYPE: &'static str = "development";

    /// Create `EnvironmentConfig` with hardcoded defaults
    ///
    /// This method is deterministic and safe for concurrent use.
    /// No environment variables are read.
    #[must_use]
    pub fn with_defaults() -> Self {
        Self {
            environment_type: Self::DEFAULT_ENVIRONMENT_TYPE.to_string(),
            variables: HashMap::new(),
            overrides: HashMap::new(),
        }
    }

    /// Create `EnvironmentConfig` from environment variables
    ///
    /// Reads configuration from environment, falling back to defaults.
    ///
    /// # Environment Variables
    /// - `BEARDOG_ENVIRONMENT`: Environment type (default: "development")
    #[must_use]
    pub fn from_env() -> Self {
        Self::from_env_provider(|k| std::env::var(k).ok())
    }

    /// Load from a custom environment provider (e.g. tests); production uses [`Self::from_env`].
    pub fn from_env_provider(get: impl Fn(&str) -> Option<String>) -> Self {
        Self {
            environment_type: get(env_keys::ENV_ENVIRONMENT)
                .unwrap_or_else(|| Self::DEFAULT_ENVIRONMENT_TYPE.to_string()),
            variables: HashMap::new(),
            overrides: HashMap::new(),
        }
    }
}

impl Default for EnvironmentConfig {
    fn default() -> Self {
        Self::with_defaults()
    }
}

impl SystemDomainConfig {
    /// Load system configuration from environment variables
    ///
    /// # Errors
    ///
    /// Returns an error if numeric environment variables (e.g. worker threads, max connections) fail to parse.
    pub fn from_env() -> Result<Self, BearDogError> {
        let mut config = Self::default();

        // Load environment type
        if let Ok(env_type) = std::env::var(env_keys::ENV_ENVIRONMENT) {
            config.environment.environment_type = env_type;
        }

        // Load logging configuration
        if let Ok(log_level) = std::env::var(env_keys::ENV_LOG_LEVEL) {
            config.logging.level = match log_level.to_lowercase().as_str() {
                "trace" => LogLevel::Trace,
                "debug" => LogLevel::Debug,
                "info" => LogLevel::Info,
                "warn" | "warning" => LogLevel::Warn,
                "error" => LogLevel::Error,
                _ => LogLevel::Info, // Default to Info if invalid
            };
        }

        if let Ok(log_format) = std::env::var(env_keys::ENV_LOG_FORMAT) {
            config.logging.format = match log_format.to_lowercase().as_str() {
                "json" => LogFormat::Json,
                "text" => LogFormat::Text,
                "structured" => LogFormat::Structured,
                "compact" => LogFormat::Compact,
                _ => LogFormat::Json, // Default to JSON if invalid
            };
        }

        // Load threading configuration
        if let Ok(worker_threads) = std::env::var(env_keys::ENV_WORKER_THREADS) {
            config.threading.worker_threads = worker_threads
                .parse()
                .map_err(|_| BearDogError::validation("Invalid BEARDOG_WORKER_THREADS value"))?;
        }

        // Load resource limits
        if let Ok(max_connections) = std::env::var(env_keys::ENV_MAX_CONNECTIONS) {
            config.resources.max_connections = max_connections
                .parse()
                .map_err(|_| BearDogError::validation("Invalid BEARDOG_MAX_CONNECTIONS value"))?;
        }

        Ok(config)
    }

    /// Validate system configuration
    ///
    /// # Errors
    ///
    /// Returns an error if threading, resource limits, or logging configuration is invalid.
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
#[path = "system_tests.rs"]
mod tests;
