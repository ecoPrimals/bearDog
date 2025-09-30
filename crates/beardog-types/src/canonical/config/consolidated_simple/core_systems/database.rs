//! Database configuration
//!
//! This module contains database-related configuration types.

use serde::{Deserialize, Serialize};
use std::time::Duration;

/// Database configuration
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct DatabaseConfig {
    /// Enable database
    pub enabled: bool,
    /// Connection settings
    pub connection: DatabaseConnectionConfig,
    /// Pooling settings
    pub pooling: DatabasePoolingConfig,
    /// Performance settings
    pub performance: DatabasePerformanceConfig,
    /// Security settings
    pub security: DatabaseSecurityConfig,
    /// Backup settings
    pub backup: DatabaseBackupConfig,
}

/// Database connection configuration
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct DatabaseConnectionConfig {
    /// Connection URL
    pub url: String,
    /// Connection timeout
    pub timeout: Duration,
}

/// Database pooling configuration
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct DatabasePoolingConfig {
    /// Maximum connections
    pub max_connections: u32,
    /// Minimum connections
    pub min_connections: u32,
}

/// Database performance configuration
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct DatabasePerformanceConfig {
    /// Query timeout
    pub query_timeout: Duration,
    /// Enable query caching
    pub enable_caching: bool,
}

/// Database security configuration
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct DatabaseSecurityConfig {
    /// Enable encryption
    pub enable_encryption: bool,
    /// SSL mode
    pub ssl_mode: String,
}

/// Database backup configuration
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct DatabaseBackupConfig {
    /// Enable backups
    pub enabled: bool,
    /// Backup interval
    pub interval: Duration,
}

impl Default for DatabaseConfig {
    fn default() -> Self {
        Self {
            enabled: false,
            connection: DatabaseConnectionConfig::default(),
            pooling: DatabasePoolingConfig::default(),
            performance: DatabasePerformanceConfig::default(),
            security: DatabaseSecurityConfig::default(),
            backup: DatabaseBackupConfig::default(),
        }
    }
}

impl Default for DatabaseConnectionConfig {
    fn default() -> Self {
        Self {
            url: "sqlite://./data/beardog.db".to_string(),
            timeout: Duration::from_secs(30),
        }
    }
}

impl Default for DatabasePoolingConfig {
    fn default() -> Self {
        Self {
            max_connections: 10,
            min_connections: 1,
        }
    }
}

impl Default for DatabasePerformanceConfig {
    fn default() -> Self {
        Self {
            query_timeout: Duration::from_secs(30),
            enable_caching: true,
        }
    }
}

impl Default for DatabaseSecurityConfig {
    fn default() -> Self {
        Self {
            enable_encryption: true,
            ssl_mode: "require".to_string(),
        }
    }
}

impl Default for DatabaseBackupConfig {
    fn default() -> Self {
        Self {
            enabled: false,
            interval: Duration::from_secs(86400), // Daily
        }
    }
} 