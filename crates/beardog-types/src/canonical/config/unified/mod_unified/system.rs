//! # System Configuration Domain
//!
//! System-level configuration for `BearDog` including deployment environment,
//! logging, performance limits, and runtime settings.

use crate::canonical::config::unified_trait::BearDogConfig;
use beardog_errors::{BearDogError, BearDogResult};
use serde::{Deserialize, Serialize};
use uuid::Uuid;

/// System-level configuration
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SystemConfiguration {
    /// System name identifier
    pub name: String,
    /// System version string
    pub version: String,
    /// Deployment environment (dev/staging/prod)
    pub environment: String,
    /// Enable debug mode and verbose logging
    pub debug: bool,
    /// Logging level (trace/debug/info/warn/error)
    pub log_level: String,
    /// Unique instance identifier for this deployment
    pub instance_id: String,
    /// Maximum memory usage in MB
    pub max_memory_mb: usize,
    /// Thread pool size for async operations
    pub thread_pool_size: usize,
    /// Enable telemetry collection
    pub telemetry_enabled: bool,
}

impl Default for SystemConfiguration {
    fn default() -> Self {
        Self {
            name: "beardog".to_string(),
            version: "3.7.0".to_string(),
            environment: "development".to_string(),
            debug: false,
            log_level: "info".to_string(),
            instance_id: Uuid::new_v4().to_string(),
            max_memory_mb: 1024,
            thread_pool_size: num_cpus::get(),
            telemetry_enabled: true,
        }
    }
}

impl BearDogConfig for SystemConfiguration {
    fn validate(&self) -> BearDogResult<()> {
        if self.name.is_empty() {
            return Err(BearDogError::system("System name cannot be empty".to_string()));
        }
        
        if self.version.is_empty() {
            return Err(BearDogError::system("System version cannot be empty".to_string()));
        }

        if !["development", "staging", "production"].contains(&self.environment.as_str()) {
            return Err(BearDogError::system(format!(
                "Invalid environment: {}. Must be one of: development, staging, production",
                self.environment
            )));
        }

        if !["trace", "debug", "info", "warn", "error"].contains(&self.log_level.as_str()) {
            return Err(BearDogError::system(format!(
                "Invalid log level: {}. Must be one of: trace, debug, info, warn, error",
                self.log_level
            )));
        }

        if self.max_memory_mb < 128 {
            return Err(BearDogError::system(
                "Maximum memory must be at least 128MB".to_string()
            ));
        }

        if self.thread_pool_size == 0 {
            return Err(BearDogError::system(
                "Thread pool size must be greater than 0".to_string()
            ));
        }

        Ok(())
    }

    fn to_toml(&self) -> BearDogResult<String> {
        toml::to_string(self)
            .map_err(|e| BearDogError::system(format!("Failed to serialize system config: {e}")))
    }

    fn from_env() -> BearDogResult<Self> {
        Ok(Self {
            name: std::env::var("BEARDOG_SYSTEM_NAME").unwrap_or_else(|_| "beardog".to_string()),
            version: std::env::var("BEARDOG_VERSION").unwrap_or_else(|_| "3.7.0".to_string()),
            environment: std::env::var("BEARDOG_ENVIRONMENT").unwrap_or_else(|_| "development".to_string()),
            debug: std::env::var("BEARDOG_DEBUG")
                .ok()
                .and_then(|v| v.parse().ok())
                .unwrap_or(false),
            log_level: std::env::var("BEARDOG_LOG_LEVEL").unwrap_or_else(|_| "info".to_string()),
            instance_id: std::env::var("BEARDOG_INSTANCE_ID").unwrap_or_else(|_| Uuid::new_v4().to_string()),
            max_memory_mb: std::env::var("BEARDOG_MAX_MEMORY_MB")
                .ok()
                .and_then(|v| v.parse().ok())
                .unwrap_or(1024),
            thread_pool_size: std::env::var("BEARDOG_THREAD_POOL_SIZE")
                .ok()
                .and_then(|v| v.parse().ok())
                .unwrap_or_else(num_cpus::get),
            telemetry_enabled: std::env::var("BEARDOG_TELEMETRY_ENABLED")
                .ok()
                .and_then(|v| v.parse().ok())
                .unwrap_or(true),
        })
    }
    
    fn merge(&self, _other: &Self) -> BearDogResult<Self> { 
        Ok(self.clone()) 
    }
    
    fn domain() -> &'static str { 
        "system" 
    }
}

impl SystemConfiguration {
    /// Create a new system configuration with defaults
    pub fn new() -> Self {
        Self::default()
    }

    /// Check if running in production environment
    pub fn is_production(&self) -> bool {
        self.environment == "production"
    }

    /// Check if debug mode is enabled
    pub fn is_debug(&self) -> bool {
        self.debug || self.environment == "development"
    }

    /// Get the effective log level based on environment
    pub fn effective_log_level(&self) -> &str {
        if self.is_debug() && self.log_level == "info" {
            "debug"
        } else {
            &self.log_level
        }
    }

    /// Get system resource limits
    pub fn resource_limits(&self) -> SystemResourceLimits {
        SystemResourceLimits {
            max_memory_mb: self.max_memory_mb,
            max_threads: self.thread_pool_size,
            max_file_descriptors: if self.is_production() { 65536 } else { 1024 },
            max_connections: if self.is_production() { 10000 } else { 1000 },
        }
    }
}

/// System resource limits
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SystemResourceLimits {
    /// Maximum memory usage in MB
    pub max_memory_mb: usize,
    /// Maximum number of threads
    pub max_threads: usize,
    /// Maximum file descriptors
    pub max_file_descriptors: usize,
    /// Maximum concurrent connections
    pub max_connections: usize,
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_system_config_default() {
        let config = SystemConfiguration::default();
        assert_eq!(config.name, "beardog");
        assert_eq!(config.version, "3.7.0");
        assert_eq!(config.environment, "development");
        assert!(!config.instance_id.is_empty());
    }

    #[test]
    fn test_system_config_validation() {
        let config = SystemConfiguration::default();
        assert!(config.validate().is_ok());

        let mut invalid_config = config.clone();
        invalid_config.name = "".to_string();
        assert!(invalid_config.validate().is_err());

        let mut invalid_env = config.clone();
        invalid_env.environment = "invalid".to_string();
        assert!(invalid_env.validate().is_err());
    }

    #[test]
    fn test_system_config_environment_checks() {
        let mut config = SystemConfiguration::default();
        
        config.environment = "production".to_string();
        assert!(config.is_production());
        
        config.environment = "development".to_string();
        config.debug = false;
        assert!(config.is_debug()); // Should be true in development
    }

    #[test]
    fn test_system_config_resource_limits() {
        let config = SystemConfiguration::default();
        let limits = config.resource_limits();
        
        assert_eq!(limits.max_memory_mb, config.max_memory_mb);
        assert_eq!(limits.max_threads, config.thread_pool_size);
    }
} 