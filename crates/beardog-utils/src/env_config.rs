//! Environment-based configuration utilities
//!
//! This module provides utilities for loading configuration from environment variables
//! with sensible defaults and validation.

use std::env;
use std::net::{IpAddr, SocketAddr};
use std::str::FromStr;
use std::time::Duration;

/// Get an environment variable or return a default value
pub fn get_env_or_default(key: &str, default: &str) -> String {
    env::var(key).unwrap_or_else(|_| default.to_string())
}

/// Get an environment variable as a typed value or return a default
pub fn get_env_as<T>(key: &str, default: T) -> T
where
    T: FromStr,
{
    env::var(key)
        .ok()
        .and_then(|s| s.parse::<T>().ok())
        .unwrap_or(default)
}

/// Get a required environment variable or panic with a helpful message
pub fn get_env_required(key: &str) -> String {
    env::var(key).unwrap_or_else(|_| {
        panic!(
            "Required environment variable {} is not set. \
             Please set it in your .env file or environment. \
             See .env.example for reference.",
            key
        )
    })
}

/// Configuration for network endpoints
#[derive(Debug, Clone)]
pub struct NetworkConfig {
    pub host: String,
    pub port: u16,
}

impl NetworkConfig {
    /// Create from environment variables with prefix
    pub fn from_env(prefix: &str) -> Self {
        let host_key = format!("{}_HOST", prefix);
        let port_key = format!("{}_PORT", prefix);

        Self {
            host: get_env_or_default(&host_key, "127.0.0.1"),
            port: get_env_as(&port_key, 8080),
        }
    }

    /// Get as SocketAddr
    pub fn to_socket_addr(&self) -> Result<SocketAddr, std::net::AddrParseError> {
        let ip: IpAddr = self.host.parse()?;
        Ok(SocketAddr::new(ip, self.port))
    }

    /// Get as URL string
    pub fn to_url(&self, scheme: &str) -> String {
        format!("{}://{}:{}", scheme, self.host, self.port)
    }
}

/// Configuration for timeouts
#[derive(Debug, Clone)]
pub struct TimeoutConfig {
    pub connect_timeout: Duration,
    pub request_timeout: Duration,
    pub idle_timeout: Duration,
}

impl TimeoutConfig {
    /// Create from environment variables with prefix
    pub fn from_env(prefix: &str) -> Self {
        let connect_key = format!("{}_CONNECT_TIMEOUT_MS", prefix);
        let request_key = format!("{}_REQUEST_TIMEOUT_MS", prefix);
        let idle_key = format!("{}_IDLE_TIMEOUT_MS", prefix);

        Self {
            connect_timeout: Duration::from_millis(get_env_as(&connect_key, 5000)),
            request_timeout: Duration::from_millis(get_env_as(&request_key, 30000)),
            idle_timeout: Duration::from_millis(get_env_as(&idle_key, 60000)),
        }
    }
}

/// Discovery endpoint configuration
#[derive(Debug, Clone)]
pub struct DiscoveryConfig {
    pub endpoint: String,
    pub timeout_secs: u64,
    pub retry_attempts: u32,
}

impl DiscoveryConfig {
    /// Load from environment or use defaults
    pub fn from_env() -> Self {
        Self {
            endpoint: get_env_or_default("BEARDOG_DISCOVERY_ENDPOINT", "http://127.0.0.1:8080"),
            timeout_secs: get_env_as("BEARDOG_DISCOVERY_TIMEOUT_SECS", 30),
            retry_attempts: get_env_as("BEARDOG_DISCOVERY_RETRY_ATTEMPTS", 3),
        }
    }
}

/// HSM configuration
#[derive(Debug, Clone)]
pub struct HsmConfig {
    pub provider: String,
    pub timeout_ms: u64,
    pub retry_attempts: u32,
}

impl HsmConfig {
    /// Load from environment or use defaults
    pub fn from_env() -> Self {
        Self {
            provider: get_env_or_default("BEARDOG_HSM_PROVIDER", "software"),
            timeout_ms: get_env_as("BEARDOG_HSM_TIMEOUT_MS", 5000),
            retry_attempts: get_env_as("BEARDOG_HSM_RETRY_ATTEMPTS", 3),
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_get_env_or_default() {
        let result = get_env_or_default("NONEXISTENT_VAR_TEST", "default_value");
        assert_eq!(result, "default_value");
    }

    #[test]
    fn test_get_env_as_with_default() {
        let result: u16 = get_env_as("NONEXISTENT_PORT_TEST", 8080);
        assert_eq!(result, 8080);
    }

    #[test]
    fn test_network_config_defaults() {
        let config = NetworkConfig::from_env("TEST_SERVICE");
        assert_eq!(config.host, "127.0.0.1");
        assert_eq!(config.port, 8080);
    }

    #[test]
    fn test_network_config_to_url() {
        let config = NetworkConfig {
            host: "127.0.0.1".to_string(),
            port: 9000,
        };
        assert_eq!(config.to_url("http"), "http://127.0.0.1:9000");
    }

    #[test]
    fn test_discovery_config_defaults() {
        let config = DiscoveryConfig::from_env();
        assert_eq!(config.endpoint, "http://127.0.0.1:8080");
        assert_eq!(config.timeout_secs, 30);
        assert_eq!(config.retry_attempts, 3);
    }

    #[test]
    fn test_hsm_config_defaults() {
        let config = HsmConfig::from_env();
        assert_eq!(config.provider, "software");
        assert_eq!(config.timeout_ms, 5000);
        assert_eq!(config.retry_attempts, 3);
    }
}
