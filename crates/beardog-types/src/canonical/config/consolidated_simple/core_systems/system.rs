//! System configuration and resource management
//!
//! This module contains system-level configuration including logging,
//! resource limits, and filesystem settings.

use serde::{Deserialize, Serialize};
use std::path::PathBuf;

/// **SYSTEM CONFIGURATION** - Core system settings
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SystemConfig {
    /// Application name
    pub app_name: String,
    /// Application description
    pub app_description: String,
    /// Organization name
    pub organization: String,
    /// Logging configuration
    pub logging: LoggingConfig,
    /// Resource limits
    pub resources: ResourceConfig,
    /// File system paths
    pub filesystem: FilesystemConfig,
}

/// Logging configuration
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct LoggingConfig {
    /// Log level (trace, debug, info, warn, error)
    pub level: String,
    /// Log format (json, text)
    pub format: String,
    /// Enable structured logging
    pub structured: bool,
    /// Log file path
    pub file_path: Option<PathBuf>,
}

/// Resource configuration
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ResourceConfig {
    /// Maximum memory usage in MB
    pub max_memory_mb: usize,
    /// Maximum CPU cores to use
    pub max_cpu_cores: Option<usize>,
    /// Thread pool size
    pub thread_pool_size: Option<usize>,
    /// Enable resource monitoring
    pub enable_monitoring: bool,
}

/// Filesystem configuration
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct FilesystemConfig {
    /// Data directory path
    pub data_dir: PathBuf,
    /// Configuration directory path
    pub config_dir: PathBuf,
    /// Log directory path
    pub log_dir: PathBuf,
    /// Temporary directory path
    pub temp_dir: PathBuf,
}

impl Default for SystemConfig {
    fn default() -> Self {
        Self {
            app_name: "BearDog".to_string(),
            app_description: "Enterprise-grade security provider for ecoPrimals ecosystem".to_string(),
            organization: "ecoPrimals".to_string(),
            logging: LoggingConfig::default(),
            resources: ResourceConfig::default(),
            filesystem: FilesystemConfig::default(),
        }
    }
}

impl Default for LoggingConfig {
    fn default() -> Self {
        Self {
            level: "info".to_string(),
            format: "json".to_string(),
            structured: true,
            file_path: None,
        }
    }
}

impl Default for ResourceConfig {
    fn default() -> Self {
        Self {
            max_memory_mb: 1024, // 1GB default
            max_cpu_cores: None, // Use all available cores
            thread_pool_size: None, // Use default thread pool size
            enable_monitoring: true,
        }
    }
}

impl Default for FilesystemConfig {
    fn default() -> Self {
        Self {
            data_dir: PathBuf::from("./data"),
            config_dir: PathBuf::from("./config"),
            log_dir: PathBuf::from("./logs"),
            temp_dir: PathBuf::from("./tmp"),
        }
    }
} 