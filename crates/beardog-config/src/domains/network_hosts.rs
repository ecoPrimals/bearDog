//! Network Host Configuration
//!
//! Centralized configuration for all network hosts/IPs used by BearDog.
//! This eliminates hardcoded IP addresses and hostnames throughout the codebase.
//!
//! # Design Philosophy
//!
//! - **Configuration over Hardcoding**: All hosts configurable via ENV or config file
//! - **Secure Defaults**: localhost for development, 0.0.0.0 for binding
//! - **Environment-First**: `BEARDOG_*` environment variables take precedence
//! - **Platform Agnostic**: Works across development, staging, and production
//!
//! # Usage
//!
//! ```no_run
//! use beardog_config::domains::network_hosts::{DEFAULT_HOST, DEFAULT_BIND_HOST};
//!
//! // Use constants directly
//! let host = DEFAULT_HOST; // "localhost"
//! let bind = DEFAULT_BIND_HOST; // "0.0.0.0"
//!
//! // Or use NetworkHostsConfig for environment-based configuration
//! use beardog_config::domains::network_hosts::NetworkHostsConfig;
//! let config = NetworkHostsConfig::from_env();
//! println!("API Host: {}", config.api_host);
//! ```

use serde::{Deserialize, Serialize};
use std::env;

// ============================================================================
// Default Host Constants
// ============================================================================

/// Default localhost address for client connections
///
/// Use for connecting TO services. Prefer this over "127.0.0.1" for better
/// IPv6 compatibility (localhost resolves to ::1 on IPv6 systems).
pub const DEFAULT_HOST: &str = "localhost";

/// Default bind address for server listening
///
/// Use for binding services to listen on all interfaces.
/// 0.0.0.0 means "listen on all IPv4 interfaces"
pub const DEFAULT_BIND_HOST: &str = "0.0.0.0";

/// Default IPv4 loopback address
///
/// Use when you specifically need the IPv4 loopback interface.
/// Prefer DEFAULT_HOST for client connections.
pub const DEFAULT_LOOPBACK_IPV4: &str = "127.0.0.1";

/// Default IPv6 loopback address
///
/// Use when you specifically need the IPv6 loopback interface.
pub const DEFAULT_LOOPBACK_IPV6: &str = "::1";

/// Default IPv6 bind address
///
/// Use for binding services to listen on all IPv6 interfaces.
pub const DEFAULT_BIND_HOST_IPV6: &str = "::";

// ============================================================================
// Common Service Hosts
// ============================================================================

/// Default PostgreSQL host
pub const DEFAULT_POSTGRES_HOST: &str = "localhost";

/// Default Redis host
pub const DEFAULT_REDIS_HOST: &str = "localhost";

/// Default Grafana host
pub const DEFAULT_GRAFANA_HOST: &str = "localhost";

/// Default Jaeger host
pub const DEFAULT_JAEGER_HOST: &str = "localhost";

/// Default external API host (for production)
///
/// Override with BEARDOG_EXTERNAL_HOST environment variable
pub const DEFAULT_EXTERNAL_HOST: &str = "localhost";

// ============================================================================
// Network Hosts Configuration
// ============================================================================

/// Network hosts configuration
///
/// All network hosts/IPs used by BearDog components. Configurable via
/// environment variables or config file, with secure defaults as fallback.
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
pub struct NetworkHostsConfig {
    /// Primary API host
    ///
    /// Host for API server binding. Defaults to "0.0.0.0" (all interfaces).
    /// Override with `BEARDOG_API_HOST` environment variable.
    #[serde(default = "default_api_host")]
    pub api_host: String,

    /// Client connection host
    ///
    /// Host for clients to connect to. Defaults to "localhost".
    /// Override with `BEARDOG_CLIENT_HOST` environment variable.
    #[serde(default = "default_client_host")]
    pub client_host: String,

    /// Discovery service host
    ///
    /// Host for service discovery. Defaults to "localhost".
    /// Override with `BEARDOG_DISCOVERY_HOST` environment variable.
    #[serde(default = "default_discovery_host")]
    pub discovery_host: String,

    /// Database host
    ///
    /// PostgreSQL database host. Defaults to "localhost".
    /// Override with `BEARDOG_DATABASE_HOST` environment variable.
    #[serde(default = "default_database_host")]
    pub database_host: String,

    /// Redis cache host
    ///
    /// Redis server host. Defaults to "localhost".
    /// Override with `BEARDOG_REDIS_HOST` environment variable.
    #[serde(default = "default_redis_host")]
    pub redis_host: String,

    /// Metrics/monitoring host
    ///
    /// Prometheus/Grafana host. Defaults to "localhost".
    /// Override with `BEARDOG_METRICS_HOST` environment variable.
    #[serde(default = "default_metrics_host")]
    pub metrics_host: String,

    /// External/public host
    ///
    /// Public-facing hostname for production. Defaults to "localhost".
    /// Override with `BEARDOG_EXTERNAL_HOST` environment variable.
    #[serde(default = "default_external_host")]
    pub external_host: String,
}

// ============================================================================
// Default Functions (with Environment Variable Support)
// ============================================================================

fn default_api_host() -> String {
    env::var("BEARDOG_API_HOST").unwrap_or_else(|_| DEFAULT_BIND_HOST.to_string())
}

fn default_client_host() -> String {
    env::var("BEARDOG_CLIENT_HOST").unwrap_or_else(|_| DEFAULT_HOST.to_string())
}

fn default_discovery_host() -> String {
    env::var("BEARDOG_DISCOVERY_HOST").unwrap_or_else(|_| DEFAULT_HOST.to_string())
}

fn default_database_host() -> String {
    env::var("BEARDOG_DATABASE_HOST").unwrap_or_else(|_| DEFAULT_POSTGRES_HOST.to_string())
}

fn default_redis_host() -> String {
    env::var("BEARDOG_REDIS_HOST").unwrap_or_else(|_| DEFAULT_REDIS_HOST.to_string())
}

fn default_metrics_host() -> String {
    env::var("BEARDOG_METRICS_HOST").unwrap_or_else(|_| DEFAULT_GRAFANA_HOST.to_string())
}

fn default_external_host() -> String {
    env::var("BEARDOG_EXTERNAL_HOST").unwrap_or_else(|_| DEFAULT_EXTERNAL_HOST.to_string())
}

// ============================================================================
// Implementation
// ============================================================================

impl Default for NetworkHostsConfig {
    fn default() -> Self {
        Self::with_defaults()
    }
}

impl NetworkHostsConfig {
    /// Creates configuration with secure defaults
    ///
    /// Reads from environment variables if set, otherwise uses documented defaults.
    ///
    /// # Example
    ///
    /// ```rust
    /// use beardog_config::domains::network_hosts::NetworkHostsConfig;
    ///
    /// let hosts = NetworkHostsConfig::with_defaults();
    /// assert_eq!(hosts.client_host, "localhost");
    /// ```
    #[must_use]
    pub fn with_defaults() -> Self {
        Self {
            api_host: default_api_host(),
            client_host: default_client_host(),
            discovery_host: default_discovery_host(),
            database_host: default_database_host(),
            redis_host: default_redis_host(),
            metrics_host: default_metrics_host(),
            external_host: default_external_host(),
        }
    }

    /// Loads configuration from environment variables
    ///
    /// This is equivalent to `with_defaults()` but makes the intent explicit.
    ///
    /// # Example
    ///
    /// ```no_run
    /// use beardog_config::domains::network_hosts::NetworkHostsConfig;
    ///
    /// let hosts = NetworkHostsConfig::from_env();
    /// // Reads BEARDOG_API_HOST, BEARDOG_CLIENT_HOST, etc.
    /// ```
    #[must_use]
    pub fn from_env() -> Self {
        Self::default()
    }

    /// Get bind address for a service
    ///
    /// Returns the appropriate bind address (0.0.0.0 for servers, localhost for clients)
    pub fn bind_address(&self) -> &str {
        &self.api_host
    }

    /// Get client connection address
    ///
    /// Returns the address clients should use to connect
    pub fn connection_address(&self) -> &str {
        &self.client_host
    }

    /// Validates that all hosts are valid
    ///
    /// Ensures hosts are not empty and contain valid characters
    pub fn validate(&self) -> Result<(), String> {
        let hosts = [
            ("api_host", &self.api_host),
            ("client_host", &self.client_host),
            ("discovery_host", &self.discovery_host),
            ("database_host", &self.database_host),
            ("redis_host", &self.redis_host),
            ("metrics_host", &self.metrics_host),
            ("external_host", &self.external_host),
        ];

        for (name, host) in hosts {
            if host.is_empty() {
                return Err(format!("{} cannot be empty", name));
            }

            // Basic validation: no whitespace, no control characters
            if host.chars().any(|c| c.is_whitespace() || c.is_control()) {
                return Err(format!("{} contains invalid characters: {}", name, host));
            }
        }

        Ok(())
    }
}

// ============================================================================
// Tests
// ============================================================================

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_default_constants() {
        assert_eq!(DEFAULT_HOST, "localhost");
        assert_eq!(DEFAULT_BIND_HOST, "0.0.0.0");
        assert_eq!(DEFAULT_LOOPBACK_IPV4, "127.0.0.1");
        assert_eq!(DEFAULT_LOOPBACK_IPV6, "::1");
    }

    #[test]
    fn test_default_config() {
        let config = NetworkHostsConfig::with_defaults();

        assert_eq!(config.api_host, DEFAULT_BIND_HOST);
        assert_eq!(config.client_host, DEFAULT_HOST);
        assert_eq!(config.discovery_host, DEFAULT_HOST);
        assert_eq!(config.database_host, DEFAULT_POSTGRES_HOST);
        assert_eq!(config.redis_host, DEFAULT_REDIS_HOST);
        assert_eq!(config.metrics_host, DEFAULT_GRAFANA_HOST);
    }

    #[test]
    fn test_validation_passes_for_defaults() {
        let config = NetworkHostsConfig::with_defaults();
        assert!(config.validate().is_ok());
    }

    #[test]
    fn test_validation_fails_for_empty() {
        let mut config = NetworkHostsConfig::with_defaults();
        config.api_host = String::new();

        let result = config.validate();
        assert!(result.is_err());
        assert!(result.unwrap_err().contains("api_host"));
    }

    #[test]
    fn test_validation_fails_for_whitespace() {
        let mut config = NetworkHostsConfig::with_defaults();
        config.client_host = "local host".to_string(); // Space in middle

        let result = config.validate();
        assert!(result.is_err());
    }

    #[test]
    fn test_bind_address() {
        let config = NetworkHostsConfig::with_defaults();
        assert_eq!(config.bind_address(), DEFAULT_BIND_HOST);
    }

    #[test]
    fn test_connection_address() {
        let config = NetworkHostsConfig::with_defaults();
        assert_eq!(config.connection_address(), DEFAULT_HOST);
    }

    #[test]
    fn test_serialization() {
        let config = NetworkHostsConfig::with_defaults();

        let json = serde_json::to_string(&config).expect("Should serialize");
        let deserialized: NetworkHostsConfig =
            serde_json::from_str(&json).expect("Should deserialize");

        assert_eq!(config, deserialized);
    }

    #[test]
    fn test_from_env_equals_default() {
        let from_env = NetworkHostsConfig::from_env();
        let with_defaults = NetworkHostsConfig::with_defaults();

        assert_eq!(from_env, with_defaults);
    }

    #[test]
    fn test_clone() {
        let config = NetworkHostsConfig::with_defaults();
        let cloned = config.clone();

        assert_eq!(config, cloned);
    }

    #[test]
    fn test_debug_format() {
        let config = NetworkHostsConfig::with_defaults();
        let debug_str = format!("{:?}", config);

        assert!(debug_str.contains("NetworkHostsConfig"));
        assert!(debug_str.contains("api_host"));
    }

    #[test]
    fn test_ipv6_constants() {
        assert_eq!(DEFAULT_LOOPBACK_IPV6, "::1");
        assert_eq!(DEFAULT_BIND_HOST_IPV6, "::");
    }

    #[test]
    fn test_service_host_constants() {
        assert_eq!(DEFAULT_POSTGRES_HOST, "localhost");
        assert_eq!(DEFAULT_REDIS_HOST, "localhost");
        assert_eq!(DEFAULT_GRAFANA_HOST, "localhost");
        assert_eq!(DEFAULT_JAEGER_HOST, "localhost");
    }

    #[test]
    fn test_validation_fails_for_control_characters() {
        let mut config = NetworkHostsConfig::with_defaults();
        config.database_host = "local\nhost".to_string(); // Newline character

        let result = config.validate();
        assert!(result.is_err());
        assert!(result.unwrap_err().contains("database_host"));
    }

    #[test]
    fn test_validation_fails_for_tab_character() {
        let mut config = NetworkHostsConfig::with_defaults();
        config.redis_host = "local\thost".to_string(); // Tab character

        let result = config.validate();
        assert!(result.is_err());
        assert!(result.unwrap_err().contains("redis_host"));
    }

    #[test]
    fn test_multiple_validation_errors_reported() {
        let mut config = NetworkHostsConfig::with_defaults();
        config.metrics_host = String::new(); // Empty

        let result = config.validate();
        assert!(result.is_err());
        assert!(result.unwrap_err().contains("metrics_host"));
    }

    #[test]
    fn test_external_host_validation() {
        let mut config = NetworkHostsConfig::with_defaults();
        config.external_host = String::new();

        let result = config.validate();
        assert!(result.is_err());
        assert!(result.unwrap_err().contains("external_host"));
    }

    #[test]
    fn test_discovery_host_validation() {
        let mut config = NetworkHostsConfig::with_defaults();
        config.discovery_host = "host with spaces".to_string();

        let result = config.validate();
        assert!(result.is_err());
        assert!(result.unwrap_err().contains("discovery_host"));
    }

    #[test]
    fn test_partial_eq_implementation() {
        let config1 = NetworkHostsConfig::with_defaults();
        let config2 = NetworkHostsConfig::with_defaults();
        let mut config3 = NetworkHostsConfig::with_defaults();
        config3.api_host = "different".to_string();

        assert_eq!(config1, config2);
        assert_ne!(config1, config3);
    }

    #[test]
    fn test_valid_ipv4_addresses() {
        let mut config = NetworkHostsConfig::with_defaults();
        config.api_host = "192.168.1.1".to_string();
        config.database_host = "10.0.0.1".to_string();

        assert!(config.validate().is_ok());
    }

    #[test]
    fn test_valid_ipv6_addresses() {
        let mut config = NetworkHostsConfig::with_defaults();
        config.api_host = "::1".to_string();
        config.client_host = "2001:db8::1".to_string();

        assert!(config.validate().is_ok());
    }

    #[test]
    fn test_valid_domain_names() {
        let mut config = NetworkHostsConfig::with_defaults();
        config.external_host = "example.com".to_string();
        config.api_host = "api.example.com".to_string();

        assert!(config.validate().is_ok());
    }

    #[test]
    fn test_serde_with_custom_values() {
        let mut config = NetworkHostsConfig::with_defaults();
        config.api_host = "192.168.1.1".to_string();
        config.external_host = "example.com".to_string();

        let json = serde_json::to_string(&config).expect("Should serialize");
        let deserialized: NetworkHostsConfig =
            serde_json::from_str(&json).expect("Should deserialize");

        assert_eq!(config, deserialized);
        assert_eq!(deserialized.api_host, "192.168.1.1");
        assert_eq!(deserialized.external_host, "example.com");
    }
}
