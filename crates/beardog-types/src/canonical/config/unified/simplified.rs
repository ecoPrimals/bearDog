//! Simplified Configuration System
//!
//! This module provides a lightweight alternative to the full unified configuration
//! for simple deployments that don't need the complete feature set.

use beardog_errors::{BearDogError, BearDogResult};
use serde::{Deserialize, Serialize};
use std::collections::HashMap;

/// **SIMPLIFIED CONFIGURATION** - Lightweight alternative for simple deployments
///
/// This provides a simplified configuration structure for projects that don't need
/// the full complexity of `UnifiedBearDogConfig`. It includes only the most essential
/// configuration domains with sensible defaults.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SimplifiedBearDogConfig {
    /// Version string
    pub version: String,
    /// Environment designation
    pub environment: String,
    /// Unique instance identifier
    pub instance_id: String,
    /// Network settings
    pub network: NetworkSettings,
    /// Security settings
    pub security: SecuritySettings,
    /// Database settings
    pub database: DatabaseSettings,
    /// Monitoring settings
    pub monitoring: MonitoringSettings,
    /// Feature flags
    pub features: HashMap<String, bool>,
    /// Performance settings
    pub performance: PerformanceSettings,
}

/// Simplified network configuration
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct NetworkSettings {
    /// Bind address
    pub bind_address: String,
    /// Port number
    pub port: u16,
    /// Maximum connections
    pub max_connections: usize,
    /// Timeout in seconds
    pub timeout_seconds: u64,
    /// Enable TLS
    pub enable_tls: bool,
}

impl Default for NetworkSettings {
    fn default() -> Self {
        Self {
            bind_address: "127.0.0.1".to_string(),
            port: 8080,
            max_connections: 1000,
            timeout_seconds: 30,
            enable_tls: true,
        }
    }
}

/// Simplified security configuration
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SecuritySettings {
    /// Session timeout in seconds
    pub session_timeout_seconds: u64,
    /// Maximum login attempts
    pub max_login_attempts: u32,
    /// Enable multi-factor authentication
    pub enable_mfa: bool,
    /// Hash rounds for password hashing
    pub hash_rounds: u32,
    /// Audit retention in days
    pub audit_retention_days: u32,
}

impl Default for SecuritySettings {
    fn default() -> Self {
        Self {
            session_timeout_seconds: 3600,
            max_login_attempts: 5,
            enable_mfa: true,
            hash_rounds: 12,
            audit_retention_days: 365,
        }
    }
}

/// Simplified database configuration
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct DatabaseSettings {
    /// Connection string
    pub connection_string: String,
    /// Connection pool size
    pub pool_size: u32,
    /// Timeout in seconds
    pub timeout_seconds: u64,
    /// Enable encryption
    pub enable_encryption: bool,
}

impl Default for DatabaseSettings {
    fn default() -> Self {
        Self {
            connection_string: "sqlite://beardog.db".to_string(),
            pool_size: 10,
            timeout_seconds: 30,
            enable_encryption: true,
        }
    }
}

/// Simplified monitoring configuration
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct MonitoringSettings {
    /// Enable metrics collection
    pub enable_metrics: bool,
    /// Metrics collection interval in seconds
    pub metrics_interval_seconds: u64,
    /// Log level
    pub log_level: String,
    /// Health check interval in seconds
    pub health_check_interval_seconds: u64,
}

impl Default for MonitoringSettings {
    fn default() -> Self {
        Self {
            enable_metrics: true,
            metrics_interval_seconds: 60,
            log_level: "info".to_string(),
            health_check_interval_seconds: 30,
        }
    }
}

/// Simplified performance configuration
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PerformanceSettings {
    /// Maximum memory in MB
    pub max_memory_mb: usize,
    /// Worker thread count
    pub worker_threads: usize,
    /// Cache size in MB
    pub cache_size_mb: usize,
    /// Enable optimization
    pub enable_optimization: bool,
}

impl Default for PerformanceSettings {
    fn default() -> Self {
        Self {
            max_memory_mb: 512,
            worker_threads: 4,
            cache_size_mb: 64,
            enable_optimization: true,
        }
    }
}

impl Default for SimplifiedBearDogConfig {
    fn default() -> Self {
        let mut features = HashMap::new();
        features.insert("security_enhanced".to_string(), true);
        features.insert("performance_optimized".to_string(), true);
        features.insert("monitoring_enabled".to_string(), true);

        Self {
            version: env!("CARGO_PKG_VERSION").to_string(),
            environment: "development".to_string(),
            instance_id: format!("beardog-{}", std::process::id()),
            network: NetworkSettings::default(),
            security: SecuritySettings::default(),
            database: DatabaseSettings::default(),
            monitoring: MonitoringSettings::default(),
            features,
            performance: PerformanceSettings::default(),
        }
    }
}

impl SimplifiedBearDogConfig {
    /// Create a new simplified configuration
    #[must_use]
    pub fn new() -> Self {
        Self::default()
    }

    /// Validate the configuration
    pub fn validate(&self) -> BearDogResult<()> {
        use crate::canonical::config::r#trait::validation::{validate_port, validate_range};

        validate_port(self.network.port, "network.port")?;

        if self.network.max_connections == 0 {
            return Err(BearDogError::configuration(
                "network.max_connections must be greater than 0",
            ));
        }

        validate_range(self.security.hash_rounds, 4, 31, "security.hash_rounds")?;

        if self.database.pool_size == 0 {
            return Err(BearDogError::configuration(
                "database.pool_size must be greater than 0",
            ));
        }

        Ok(())
    }

    /// Load from environment variables
    pub fn from_env() -> BearDogResult<Self> {
        let mut config = Self::default();

        if let Ok(port) = std::env::var("BEARDOG_PORT") {
            if let Ok(port_num) = port.parse::<u16>() {
                config.network.port = port_num;
            }
        }

        if let Ok(log_level) = std::env::var("BEARDOG_LOG_LEVEL") {
            config.monitoring.log_level = log_level;
        }

        if let Ok(environment) = std::env::var("BEARDOG_ENVIRONMENT") {
            config.environment = environment;
        }

        Ok(config)
    }

    /// Apply configuration overrides
    #[must_use]
    pub fn with_overrides(mut self, overrides: HashMap<String, String>) -> Self {
        for (key, value) in overrides {
            match key.as_str() {
                "network.port" => {
                    if let Ok(port) = value.parse::<u16>() {
                        self.network.port = port;
                    }
                }
                "network.max_connections" => {
                    if let Ok(connections) = value.parse::<usize>() {
                        self.network.max_connections = connections;
                    }
                }
                "security.enable_mfa" => {
                    if let Ok(enable) = value.parse::<bool>() {
                        self.security.enable_mfa = enable;
                    }
                }
                "monitoring.log_level" => {
                    self.monitoring.log_level = value;
                }
                _ => {
                    if let Ok(bool_value) = value.parse::<bool>() {
                        self.features.insert(key, bool_value);
                    }
                }
            }
        }
        self
    }

    /// Get configuration summary
    #[must_use]
    pub fn summary(&self) -> HashMap<String, String> {
        let mut summary = HashMap::new();
        summary.insert("version".to_string(), self.version.clone());
        summary.insert("environment".to_string(), self.environment.clone());
        summary.insert("network_port".to_string(), self.network.port.to_string());
        summary.insert(
            "security_mfa".to_string(),
            self.security.enable_mfa.to_string(),
        );
        summary.insert(
            "database_pool_size".to_string(),
            self.database.pool_size.to_string(),
        );
        summary.insert(
            "monitoring_enabled".to_string(),
            self.monitoring.enable_metrics.to_string(),
        );
        summary
    }
}
