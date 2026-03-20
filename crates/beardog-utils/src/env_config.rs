// SPDX-License-Identifier: AGPL-3.0-only

//! Environment-based configuration utilities
//!
//! This module provides utilities for loading configuration from environment variables
//! with sensible defaults and validation.
//!
//! Production code uses [`std_env_lookup`] with the real process environment.
//! Tests pass a [`HashMap`]-backed lookup via [`env_map_lookup`] to avoid global `set_var` / races.

use std::collections::HashMap;
use std::net::{IpAddr, SocketAddr};
use std::str::FromStr;
use std::time::Duration;

/// Look up a variable the same way as `std::env::var` but as `Option<String>`.
#[must_use]
pub fn std_env_lookup(key: &str) -> Option<String> {
    std::env::var(key).ok()
}

/// Build a lookup closure backed by a map (for tests).
#[must_use]
pub fn env_map_lookup(map: &HashMap<String, String>) -> impl Fn(&str) -> Option<String> + '_ {
    move |k| map.get(k).cloned()
}

/// Get an environment variable or return a default value
pub fn get_env_or_default(key: &str, default: &str) -> String {
    get_env_or_default_with(std_env_lookup, key, default)
}

/// Get an environment variable or default using a custom lookup.
pub fn get_env_or_default_with(
    get: impl Fn(&str) -> Option<String>,
    key: &str,
    default: &str,
) -> String {
    get(key).unwrap_or_else(|| default.to_string())
}

/// Get an environment variable as a typed value or return a default
pub fn get_env_as<T>(key: &str, default: T) -> T
where
    T: FromStr,
{
    get_env_as_with(std_env_lookup, key, default)
}

/// Typed env parse with a custom lookup.
pub fn get_env_as_with<T>(get: impl Fn(&str) -> Option<String>, key: &str, default: T) -> T
where
    T: FromStr,
{
    get(key)
        .and_then(|s| s.parse::<T>().ok())
        .unwrap_or(default)
}

/// Get a required environment variable, returning Result for graceful error handling
///
/// # Errors
///
/// Returns `BearDogError::configuration` if the environment variable is not set.
pub fn try_get_env_required(key: &str) -> Result<String, beardog_errors::BearDogError> {
    try_get_env_required_with(std_env_lookup, key)
}

/// Required env with custom lookup.
pub fn try_get_env_required_with(
    get: impl Fn(&str) -> Option<String>,
    key: &str,
) -> Result<String, beardog_errors::BearDogError> {
    get(key).ok_or_else(|| {
        let message = format!(
            "Required environment variable '{key}' is not set. \
             Please set it in your .env file or environment. \
             See .env.example for reference."
        );
        beardog_errors::BearDogError::configuration(&message)
    })
}

/// Configuration for network endpoints
#[derive(Debug, Clone)]
pub struct NetworkConfig {
    /// Resolved hostname or IP literal.
    pub host: String,
    /// TCP/UDP port.
    pub port: u16,
}

impl NetworkConfig {
    /// Create from environment variables with prefix
    pub fn from_env(prefix: &str) -> Self {
        use beardog_types::canonical::config::runtime_config::RuntimeNetworkConfig;

        let runtime_config = RuntimeNetworkConfig::from_env();
        Self::from_env_with_runtime(prefix, std_env_lookup, &runtime_config)
    }

    /// Create from a custom lookup and explicit runtime defaults (testable).
    pub fn from_env_with_runtime(
        prefix: &str,
        get: impl Fn(&str) -> Option<String>,
        runtime_defaults: &beardog_types::canonical::config::runtime_config::RuntimeNetworkConfig,
    ) -> Self {
        let host_key = format!("{prefix}_HOST");
        let port_key = format!("{prefix}_PORT");

        Self {
            host: get_env_or_default_with(&get, &host_key, &runtime_defaults.api_host),
            port: get_env_as_with(&get, &port_key, runtime_defaults.api_port),
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
    /// Time allowed to establish a connection.
    pub connect_timeout: Duration,
    /// End-to-end deadline for a single request/response.
    pub request_timeout: Duration,
    /// How long an idle connection may sit before teardown.
    pub idle_timeout: Duration,
}

impl TimeoutConfig {
    /// Create from environment variables with prefix
    pub fn from_env(prefix: &str) -> Self {
        Self::from_env_with(prefix, std_env_lookup)
    }

    /// Create from custom lookup (tests).
    pub fn from_env_with(prefix: &str, get: impl Fn(&str) -> Option<String>) -> Self {
        let connect_key = format!("{prefix}_CONNECT_TIMEOUT_MS");
        let request_key = format!("{prefix}_REQUEST_TIMEOUT_MS");
        let idle_key = format!("{prefix}_IDLE_TIMEOUT_MS");

        Self {
            connect_timeout: Duration::from_millis(get_env_as_with(&get, &connect_key, 5000u64)),
            request_timeout: Duration::from_millis(get_env_as_with(&get, &request_key, 30000u64)),
            idle_timeout: Duration::from_millis(get_env_as_with(&get, &idle_key, 60000u64)),
        }
    }
}

// Use canonical DiscoveryConfig instead of local definition
pub use beardog_types::canonical::config::domains::discovery::DiscoveryConfig;

/// HSM configuration
#[derive(Debug, Clone)]
pub struct HsmConfig {
    /// Backend identifier (e.g. `software`, vendor plugin name).
    pub provider: String,
    /// Per-call timeout in milliseconds.
    pub timeout_ms: u64,
    /// How many times to retry transient failures.
    pub retry_attempts: u32,
}

impl Default for HsmConfig {
    /// Pure defaults without environment variable access
    /// Concurrent-safe and suitable for testing
    fn default() -> Self {
        Self {
            provider: "software".to_string(),
            timeout_ms: 5000,
            retry_attempts: 3,
        }
    }
}

impl HsmConfig {
    /// Load from environment or use defaults
    pub fn from_env() -> Self {
        Self::from_env_with(std_env_lookup)
    }

    /// Load using custom lookup (tests).
    pub fn from_env_with(get: impl Fn(&str) -> Option<String>) -> Self {
        Self {
            provider: get_env_or_default_with(&get, "BEARDOG_HSM_PROVIDER", "software"),
            timeout_ms: get_env_as_with(&get, "BEARDOG_HSM_TIMEOUT_MS", 5000u64),
            retry_attempts: get_env_as_with(&get, "BEARDOG_HSM_RETRY_ATTEMPTS", 3u32),
        }
    }
}

#[allow(unused_imports, clippy::nonminimal_bool, dead_code, missing_docs)]
#[cfg(test)]
mod tests {
    use super::*;
    use beardog_types::canonical::config::runtime_config::RuntimeNetworkConfig;

    // Test constant - matches DEFAULT_API_PORT from beardog-config
    const TEST_DEFAULT_PORT: u16 = 8080;

    fn test_runtime_defaults() -> RuntimeNetworkConfig {
        RuntimeNetworkConfig {
            discovery_endpoint: format!("http://127.0.0.1:{TEST_DEFAULT_PORT}/discover"),
            api_host: "127.0.0.1".to_string(),
            api_port: TEST_DEFAULT_PORT,
            metrics_port: 9090,
            health_port: 8081,
            ws_port: 3000,
            grpc_port: 50051,
            admin_port: 8082,
            database_port: 5432,
            consul_port: 8500,
            redis_port: 6379,
            timeout_seconds: 30,
            max_connections: 1000,
            enable_tls: true,
        }
    }

    #[test]
    fn test_get_env_or_default() {
        let result = get_env_or_default("NONEXISTENT_VAR_TEST", "default_value");
        assert_eq!(result, "default_value");
    }

    #[test]
    fn test_get_env_as_with_default() {
        let result: u16 = get_env_as("NONEXISTENT_PORT_TEST", TEST_DEFAULT_PORT);
        assert_eq!(result, TEST_DEFAULT_PORT);
    }

    #[test]
    fn test_network_config_defaults() {
        let runtime = test_runtime_defaults();
        let config = NetworkConfig::from_env_with_runtime("TEST_SERVICE", |_| None, &runtime);
        assert_eq!(config.host, "127.0.0.1");
        assert_eq!(config.port, TEST_DEFAULT_PORT);
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
        let config = DiscoveryConfig::default();

        assert_eq!(config.timeout.as_secs(), 5);
        assert_eq!(config.max_attempts, 3);
        assert_eq!(config.max_concurrent, 10);

        assert!(config.enabled);
        assert!(config.cache_enabled);
        assert_eq!(config.endpoints.len(), 0);
    }

    #[test]
    fn test_hsm_config_defaults() {
        let config = HsmConfig::default();

        assert_eq!(config.provider, "software");
        assert_eq!(config.timeout_ms, 5000);
        assert_eq!(config.retry_attempts, 3);
    }

    #[test]
    fn test_get_env_or_default_with_env_var() {
        let mut map = HashMap::new();
        map.insert("TEST_VAR_EXISTS".to_string(), "custom_value".to_string());
        let result =
            get_env_or_default_with(env_map_lookup(&map), "TEST_VAR_EXISTS", "default_value");
        assert_eq!(result, "custom_value");
    }

    #[test]
    fn test_get_env_as_with_env_var() {
        const TEST_DISCOVERY_PORT: u16 = 9090;
        let mut map = HashMap::new();
        map.insert(
            "TEST_PORT_EXISTS".to_string(),
            TEST_DISCOVERY_PORT.to_string(),
        );
        let result: u16 =
            get_env_as_with(env_map_lookup(&map), "TEST_PORT_EXISTS", TEST_DEFAULT_PORT);
        assert_eq!(result, TEST_DISCOVERY_PORT);
    }

    #[test]
    fn test_get_env_as_with_invalid_value() {
        let mut map = HashMap::new();
        map.insert("TEST_PORT_INVALID".to_string(), "not_a_number".to_string());
        let result: u16 =
            get_env_as_with(env_map_lookup(&map), "TEST_PORT_INVALID", TEST_DEFAULT_PORT);
        assert_eq!(result, TEST_DEFAULT_PORT);
    }

    #[test]
    fn test_try_get_env_required_missing() {
        let err = try_get_env_required_with(|_| None, "REQUIRED_VAR_MISSING")
            .expect_err("missing var should err");
        let msg = err.to_string();
        assert!(
            msg.contains("REQUIRED_VAR_MISSING") && msg.contains("not set"),
            "unexpected message: {msg}"
        );
    }

    #[test]
    fn test_try_get_env_required_present() {
        let mut map = HashMap::new();
        map.insert(
            "REQUIRED_VAR_PRESENT".to_string(),
            "required_value".to_string(),
        );
        let result = try_get_env_required_with(env_map_lookup(&map), "REQUIRED_VAR_PRESENT")
            .expect("var is set");
        assert_eq!(result, "required_value");
    }

    #[test]
    fn test_network_config_from_env_custom() {
        const TEST_CUSTOM_PORT: u16 = 3000;
        let mut map = HashMap::new();
        map.insert("CUSTOM_HOST".to_string(), "192.168.1.1".to_string());
        map.insert("CUSTOM_PORT".to_string(), TEST_CUSTOM_PORT.to_string());

        let runtime = test_runtime_defaults();
        let config = NetworkConfig::from_env_with_runtime("CUSTOM", env_map_lookup(&map), &runtime);
        assert_eq!(config.host, "192.168.1.1");
        assert_eq!(config.port, TEST_CUSTOM_PORT);
    }

    #[test]
    fn test_network_config_to_socket_addr() {
        let config = NetworkConfig {
            host: "127.0.0.1".to_string(),
            port: TEST_DEFAULT_PORT,
        };

        let addr = config
            .to_socket_addr()
            .expect("127.0.0.1 should parse as a socket address");
        assert_eq!(addr.to_string(), format!("127.0.0.1:{}", TEST_DEFAULT_PORT));
    }

    #[test]
    fn test_network_config_to_socket_addr_invalid() {
        let config = NetworkConfig {
            host: "invalid-ip".to_string(),
            port: TEST_DEFAULT_PORT,
        };

        let result = config.to_socket_addr();
        assert!(result.is_err());
    }

    #[test]
    fn test_network_config_to_url_https() {
        let config = NetworkConfig {
            host: "example.com".to_string(),
            port: 443,
        };
        assert_eq!(config.to_url("https"), "https://example.com:443");
    }

    #[test]
    fn test_timeout_config_from_env_defaults() {
        let config = TimeoutConfig::from_env_with("DEFAULT", |_| None);
        assert_eq!(config.connect_timeout, Duration::from_millis(5000));
        assert_eq!(config.request_timeout, Duration::from_millis(30000));
        assert_eq!(config.idle_timeout, Duration::from_millis(60000));
    }

    #[test]
    fn test_timeout_config_from_env_custom() {
        let mut map = HashMap::new();
        map.insert("CUSTOM_CONNECT_TIMEOUT_MS".to_string(), "1000".to_string());
        map.insert("CUSTOM_REQUEST_TIMEOUT_MS".to_string(), "10000".to_string());
        map.insert("CUSTOM_IDLE_TIMEOUT_MS".to_string(), "30000".to_string());

        let config = TimeoutConfig::from_env_with("CUSTOM", env_map_lookup(&map));
        assert_eq!(config.connect_timeout, Duration::from_millis(1000));
        assert_eq!(config.request_timeout, Duration::from_millis(10000));
        assert_eq!(config.idle_timeout, Duration::from_millis(30000));
    }

    #[test]
    fn test_discovery_config_custom_values() {
        use std::time::Duration;

        let config = DiscoveryConfig {
            endpoints: vec!["http://custom:9000/api".to_string()],
            timeout: Duration::from_secs(60),
            max_attempts: 5,
            ..DiscoveryConfig::default()
        };
        assert!(
            config
                .endpoints
                .iter()
                .any(|e| e == "http://custom:9000/api")
        );
        assert_eq!(config.timeout.as_secs(), 60);
        assert_eq!(config.max_attempts, 5);
    }

    #[test]
    fn test_hsm_config_from_env_custom() {
        let mut map = HashMap::new();
        map.insert("BEARDOG_HSM_PROVIDER".to_string(), "hardware".to_string());
        map.insert("BEARDOG_HSM_TIMEOUT_MS".to_string(), "10000".to_string());
        map.insert("BEARDOG_HSM_RETRY_ATTEMPTS".to_string(), "5".to_string());

        let config = HsmConfig::from_env_with(env_map_lookup(&map));
        assert_eq!(config.provider, "hardware");
        assert_eq!(config.timeout_ms, 10000);
        assert_eq!(config.retry_attempts, 5);
    }

    #[test]
    fn test_network_config_clone() {
        let config1 = NetworkConfig {
            host: "127.0.0.1".to_string(),
            port: TEST_DEFAULT_PORT,
        };
        let config2 = config1.clone();
        assert_eq!(config1.host, config2.host);
        assert_eq!(config1.port, config2.port);
    }

    #[test]
    fn test_timeout_config_clone() {
        let config1 = TimeoutConfig {
            connect_timeout: Duration::from_secs(5),
            request_timeout: Duration::from_secs(30),
            idle_timeout: Duration::from_secs(60),
        };
        let config2 = config1.clone();
        assert_eq!(config1.connect_timeout, config2.connect_timeout);
        assert_eq!(config1.request_timeout, config2.request_timeout);
        assert_eq!(config1.idle_timeout, config2.idle_timeout);
    }

    #[test]
    fn test_discovery_config_clone() {
        let config1 =
            DiscoveryConfig::with_endpoints(vec![format!("http://test:{}", TEST_DEFAULT_PORT)]);
        let config2 = config1.clone();
        assert_eq!(config1.endpoints, config2.endpoints);
        assert_eq!(config1.timeout, config2.timeout);
        assert_eq!(config1.max_attempts, config2.max_attempts);
    }

    #[test]
    fn test_hsm_config_clone() {
        let config1 = HsmConfig {
            provider: "software".to_string(),
            timeout_ms: 5000,
            retry_attempts: 3,
        };
        let config2 = config1.clone();
        assert_eq!(config1.provider, config2.provider);
        assert_eq!(config1.timeout_ms, config2.timeout_ms);
        assert_eq!(config1.retry_attempts, config2.retry_attempts);
    }
}
