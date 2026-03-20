// SPDX-License-Identifier: AGPL-3.0-only

//! Timeout Configuration Module  
//!
//! Centralized timeout configuration to eliminate hardcoded timeout values.
//! Follows the same pattern as PathConfig for consistency.

use serde::{Deserialize, Serialize};
use std::time::Duration;

/// Centralized timeout configuration for BearDog
///
/// Provides environment-aware default timeouts for:
/// - Network operations (connection, request, response)
/// - Database operations
/// - Cache operations
/// - Health checks
/// - Workflow operations
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct TimeoutConfig {
    /// HTTP/network connection timeout
    pub connection_timeout: Duration,
    
    /// HTTP/network request timeout
    pub request_timeout: Duration,
    
    /// HTTP/network response read timeout
    pub response_timeout: Duration,
    
    /// Database query timeout
    pub database_timeout: Duration,
    
    /// Cache operation timeout
    pub cache_timeout: Duration,
    
    /// Health check timeout
    pub health_check_timeout: Duration,
    
    /// Workflow step timeout
    pub workflow_step_timeout: Duration,
    
    /// Graceful shutdown timeout
    pub shutdown_timeout: Duration,
    
    /// Lock acquisition timeout
    pub lock_timeout: Duration,
}

impl Default for TimeoutConfig {
    fn default() -> Self {
        Self::from_environment()
    }
}

impl TimeoutConfig {
    /// Create timeout configuration from environment variables
    ///
    /// Environment variables (in order of precedence):
    /// - `BEARDOG_CONNECTION_TIMEOUT_MS` - Connection timeout in milliseconds
    /// - `BEARDOG_REQUEST_TIMEOUT_MS` - Request timeout in milliseconds
    /// - `BEARDOG_RESPONSE_TIMEOUT_MS` - Response timeout in milliseconds
    /// - `BEARDOG_DATABASE_TIMEOUT_MS` - Database timeout in milliseconds
    /// - `BEARDOG_CACHE_TIMEOUT_MS` - Cache timeout in milliseconds
    /// - `BEARDOG_HEALTH_CHECK_TIMEOUT_MS` - Health check timeout in milliseconds
    /// - `BEARDOG_WORKFLOW_STEP_TIMEOUT_MS` - Workflow step timeout in milliseconds
    /// - `BEARDOG_SHUTDOWN_TIMEOUT_MS` - Shutdown timeout in milliseconds
    /// - `BEARDOG_LOCK_TIMEOUT_MS` - Lock timeout in milliseconds
    ///
    /// Falls back to sensible defaults if not set.
    pub fn from_environment() -> Self {
        Self {
            connection_timeout: Self::get_env_duration("BEARDOG_CONNECTION_TIMEOUT_MS")
                .unwrap_or_else(|| Duration::from_secs(30)),
            request_timeout: Self::get_env_duration("BEARDOG_REQUEST_TIMEOUT_MS")
                .unwrap_or_else(|| Duration::from_secs(60)),
            response_timeout: Self::get_env_duration("BEARDOG_RESPONSE_TIMEOUT_MS")
                .unwrap_or_else(|| Duration::from_secs(30)),
            database_timeout: Self::get_env_duration("BEARDOG_DATABASE_TIMEOUT_MS")
                .unwrap_or_else(|| Duration::from_secs(30)),
            cache_timeout: Self::get_env_duration("BEARDOG_CACHE_TIMEOUT_MS")
                .unwrap_or_else(|| Duration::from_secs(5)),
            health_check_timeout: Self::get_env_duration("BEARDOG_HEALTH_CHECK_TIMEOUT_MS")
                .unwrap_or_else(|| Duration::from_secs(10)),
            workflow_step_timeout: Self::get_env_duration("BEARDOG_WORKFLOW_STEP_TIMEOUT_MS")
                .unwrap_or_else(|| Duration::from_secs(300)), // 5 minutes
            shutdown_timeout: Self::get_env_duration("BEARDOG_SHUTDOWN_TIMEOUT_MS")
                .unwrap_or_else(|| Duration::from_secs(30)),
            lock_timeout: Self::get_env_duration("BEARDOG_LOCK_TIMEOUT_MS")
                .unwrap_or_else(|| Duration::from_secs(5)),
        }
    }
    
    /// Get duration from environment variable (milliseconds)
    fn get_env_duration(var_name: &str) -> Option<Duration> {
        std::env::var(var_name)
            .ok()
            .and_then(|s| s.parse::<u64>().ok())
            .map(Duration::from_millis)
    }
    
    /// Create configuration for production environment (longer timeouts)
    #[must_use]
    pub fn production() -> Self {
        Self {
            connection_timeout: Duration::from_secs(60),
            request_timeout: Duration::from_secs(120),
            response_timeout: Duration::from_secs(60),
            database_timeout: Duration::from_secs(60),
            cache_timeout: Duration::from_secs(10),
            health_check_timeout: Duration::from_secs(30),
            workflow_step_timeout: Duration::from_secs(600), // 10 minutes
            shutdown_timeout: Duration::from_secs(60),
            lock_timeout: Duration::from_secs(10),
        }
    }
    
    /// Create configuration for development environment (shorter timeouts)
    #[must_use]
    pub fn development() -> Self {
        Self {
            connection_timeout: Duration::from_secs(10),
            request_timeout: Duration::from_secs(30),
            response_timeout: Duration::from_secs(15),
            database_timeout: Duration::from_secs(15),
            cache_timeout: Duration::from_secs(3),
            health_check_timeout: Duration::from_secs(5),
            workflow_step_timeout: Duration::from_secs(120), // 2 minutes
            shutdown_timeout: Duration::from_secs(15),
            lock_timeout: Duration::from_secs(3),
        }
    }
    
    /// Create configuration for testing (very short timeouts)
    #[must_use]
    pub fn testing() -> Self {
        Self {
            connection_timeout: Duration::from_secs(5),
            request_timeout: Duration::from_secs(10),
            response_timeout: Duration::from_secs(5),
            database_timeout: Duration::from_secs(5),
            cache_timeout: Duration::from_secs(1),
            health_check_timeout: Duration::from_secs(3),
            workflow_step_timeout: Duration::from_secs(30),
            shutdown_timeout: Duration::from_secs(5),
            lock_timeout: Duration::from_secs(1),
        }
    }
    
    /// Get timeout for network operations
    #[must_use]
    pub const fn network_timeout(&self) -> Duration {
        self.request_timeout
    }
    
    /// Get timeout for database operations
    #[must_use]
    pub const fn db_timeout(&self) -> Duration {
        self.database_timeout
    }
    
    /// Get timeout for workflow operations
    #[must_use]
    pub const fn workflow_timeout(&self) -> Duration {
        self.workflow_step_timeout
    }
}

/// Global timeout configuration instance
///
/// This is lazily initialized on first access and cached for the lifetime of the application.
pub fn global_timeout_config() -> &'static TimeoutConfig {
    use std::sync::OnceLock;
    static TIMEOUT_CONFIG: OnceLock<TimeoutConfig> = OnceLock::new();
    TIMEOUT_CONFIG.get_or_init(TimeoutConfig::default)
}

#[cfg(test)]
mod tests {
    use super::*;
    
    #[test]
    fn test_timeout_config_default() {
        let config = TimeoutConfig::default();
        
        // All timeouts should be set to reasonable values
        assert!(config.connection_timeout.as_secs() > 0);
        assert!(config.request_timeout.as_secs() > 0);
        assert!(config.database_timeout.as_secs() > 0);
    }
    
    #[test]
    fn test_timeout_config_production() {
        let config = TimeoutConfig::production();
        
        // Production should have longer timeouts
        assert_eq!(config.connection_timeout, Duration::from_secs(60));
        assert_eq!(config.request_timeout, Duration::from_secs(120));
    }
    
    #[test]
    fn test_timeout_config_development() {
        let config = TimeoutConfig::development();
        
        // Development should have moderate timeouts
        assert_eq!(config.connection_timeout, Duration::from_secs(10));
        assert_eq!(config.request_timeout, Duration::from_secs(30));
    }
    
    #[test]
    fn test_timeout_config_testing() {
        let config = TimeoutConfig::testing();
        
        // Testing should have short timeouts
        assert_eq!(config.connection_timeout, Duration::from_secs(5));
        assert_eq!(config.request_timeout, Duration::from_secs(10));
    }
    
    #[test]
    fn test_timeout_config_helpers() {
        let config = TimeoutConfig::default();
        
        assert_eq!(config.network_timeout(), config.request_timeout);
        assert_eq!(config.db_timeout(), config.database_timeout);
        assert_eq!(config.workflow_timeout(), config.workflow_step_timeout);
    }
    
    #[test]
    fn test_environment_variable_override() {
        beardog_errors::process_env::set_var("BEARDOG_CONNECTION_TIMEOUT_MS", "5000");
        let config = TimeoutConfig::from_environment();
        assert_eq!(config.connection_timeout, Duration::from_millis(5000));
        beardog_errors::process_env::remove_var("BEARDOG_CONNECTION_TIMEOUT_MS");
    }
    
    #[test]
    fn test_global_timeout_config() {
        let config1 = global_timeout_config();
        let config2 = global_timeout_config();
        
        // Should return the same instance
        assert_eq!(config1.connection_timeout, config2.connection_timeout);
    }
    
    #[test]
    fn test_timeout_config_serialization() {
        let config = TimeoutConfig::default();
        let json = serde_json::to_string(&config).unwrap();
        let deserialized: TimeoutConfig = serde_json::from_str(&json).unwrap();
        
        assert_eq!(config.connection_timeout, deserialized.connection_timeout);
        assert_eq!(config.request_timeout, deserialized.request_timeout);
    }
}

