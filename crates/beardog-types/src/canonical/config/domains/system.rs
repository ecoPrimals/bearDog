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

/// Logging configuration
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct LoggingConfig {
    /// Log level (trace, debug, info, warn, error)
    pub level: String,
    /// Log format (json, text, structured)
    pub format: String,
    /// Log output targets
    pub targets: Vec<LogTarget>,
    /// Enable structured logging
    pub structured: bool,
    /// Log rotation settings
    pub rotation: LogRotationConfig,
}

/// Log target configuration
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct LogTarget {
    /// Target type (stdout, file, syslog, etc.)
    pub target_type: String,
    /// Target-specific configuration
    pub config: HashMap<String, String>,
}

/// Log rotation configuration
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct LogRotationConfig {
    /// Maximum file size before rotation
    pub max_size_mb: u64,
    /// Maximum number of log files to keep
    pub max_files: u32,
    /// Rotation frequency
    pub frequency: String,
}

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

impl Default for LoggingConfig {
    fn default() -> Self {
        Self {
            level: "info".to_string(),
            format: "structured".to_string(),
            targets: vec![LogTarget {
                target_type: "stdout".to_string(),
                config: HashMap::new(),
            }],
            structured: true,
            rotation: LogRotationConfig::default(),
        }
    }
}

impl Default for LogRotationConfig {
    fn default() -> Self {
        Self {
            max_size_mb: 100,
            max_files: beardog_types::constants::domains::system::defaults::DEFAULT_POOL_S as u32I as u32Z as u32E as u32,
            frequency: "daily".to_string(),
        }
    }
}

impl Default for ThreadingConfig {
    fn default() -> Self {
        Self {
            worker_threads: std::thread::available_parallelism()
                .map(|n| n.get())
                .unwrap_or(4),
            blocking_threads: beardog_types::constants::domains::system::defaults::DEFAULT_CACHE_SIZE,
            stack_size: None,
            enable_tls_optimization: true,
        }
    }
}

impl Default for ResourceConfig {
    fn default() -> Self {
        Self {
            max_memory_bytes: None,
            max_file_descriptors: Some(65536),
            max_connections: 10000,
            monitoring_interval: Duration::from_secs(60),
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
            config.logging.level = log_level;
        }

        if let Ok(log_format) = std::env::var("BEARDOG_LOG_FORMAT") {
            config.logging.format = log_format;
        }

        // Load threading configuration
        if let Ok(worker_threads) = std::env::var("BEARDOG_WORKER_THREADS") {
            config.threading.worker_threads = worker_threads.parse()
                .map_err(|_| BearDogError::validation("Invalid BEARDOG_WORKER_THREADS value"))?;
        }

        // Load resource limits
        if let Ok(max_connections) = std::env::var("BEARDOG_MAX_CONNECTIONS") {
            config.resources.max_connections = max_connections.parse()
                .map_err(|_| BearDogError::validation("Invalid BEARDOG_MAX_CONNECTIONS value"))?;
        }

        Ok(config)
    }

    /// Validate system configuration
    pub fn validate(&self) -> Result<(), BearDogError> {
        // Validate threading configuration
        if self.threading.worker_threads == 0 {
            return Err(BearDogError::validation("Worker threads must be greater than 0"));
        }

        if self.threading.blocking_threads == 0 {
            return Err(BearDogError::validation("Blocking threads must be greater than 0"));
        }

        // Validate resource limits
        if self.resources.max_connections == 0 {
            return Err(BearDogError::validation("Max connections must be greater than 0"));
        }

        // Validate log level
        match self.logging.level.as_str() {
            "trace" | "debug" | "info" | "warn" | "error" => {},
            _ => return Err(BearDogError::validation("Invalid log level")),
        }

        Ok(())
    }
} 