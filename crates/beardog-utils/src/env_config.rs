// SPDX-License-Identifier: AGPL-3.0-only

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

/// Get a required environment variable, returning Result for graceful error handling
///
/// # Errors
///
/// Returns `BearDogError::configuration` if the environment variable is not set.
pub fn try_get_env_required(key: &str) -> Result<String, beardog_errors::BearDogError> {
    env::var(key).map_err(|_| {
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

        // Use RuntimeNetworkConfig for defaults instead of hardcoded values
        let runtime_config = RuntimeNetworkConfig::from_env();

        let host_key = format!("{prefix}_HOST");
        let port_key = format!("{prefix}_PORT");

        Self {
            host: get_env_or_default(&host_key, &runtime_config.api_host),
            port: get_env_as(&port_key, runtime_config.api_port),
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
        let connect_key = format!("{prefix}_CONNECT_TIMEOUT_MS");
        let request_key = format!("{prefix}_REQUEST_TIMEOUT_MS");
        let idle_key = format!("{prefix}_IDLE_TIMEOUT_MS");

        Self {
            connect_timeout: Duration::from_millis(get_env_as(&connect_key, 5000)),
            request_timeout: Duration::from_millis(get_env_as(&request_key, 30000)),
            idle_timeout: Duration::from_millis(get_env_as(&idle_key, 60000)),
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
        Self {
            provider: get_env_or_default("BEARDOG_HSM_PROVIDER", "software"),
            timeout_ms: get_env_as("BEARDOG_HSM_TIMEOUT_MS", 5000),
            retry_attempts: get_env_as("BEARDOG_HSM_RETRY_ATTEMPTS", 3),
        }
    }
}

#[allow(unused_imports, clippy::nonminimal_bool, dead_code, missing_docs)]
#[cfg(test)]
mod tests {
    use super::*;
    use std::sync::Mutex;

    // Test constant - matches DEFAULT_API_PORT from beardog-config
    const TEST_DEFAULT_PORT: u16 = 8080;

    // Mutex to serialize tests that modify environment variables
    // This prevents race conditions when tests run in parallel
    static ENV_TEST_MUTEX: Mutex<()> = Mutex::new(());

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
    #[serial_test::serial] // Environment variable test - must run serially
    fn test_network_config_defaults() {
        // Clear any env vars that might interfere
        beardog_errors::process_env::remove_var("TEST_SERVICE_HOST");
        beardog_errors::process_env::remove_var("TEST_SERVICE_PORT");
        beardog_errors::process_env::remove_var("BEARDOG_API_HOST");
        beardog_errors::process_env::remove_var("BEARDOG_API_PORT");

        let config = NetworkConfig::from_env("TEST_SERVICE");
        // TEST_CATEGORY: unit
        // TEST_DOMAIN: core
        // TEST_PRIORITY: normal
        // Actual default from RuntimeNetworkConfig is 127.0.0.1 (not "localhost")
        assert_eq!(config.host, "127.0.0.1");
        assert_eq!(config.port, TEST_DEFAULT_PORT);

        // Clean up
        beardog_errors::process_env::remove_var("TEST_SERVICE_HOST");
        beardog_errors::process_env::remove_var("TEST_SERVICE_PORT");
        beardog_errors::process_env::remove_var("BEARDOG_API_HOST");
        beardog_errors::process_env::remove_var("BEARDOG_API_PORT");
    }
    // TEST_CATEGORY: unit
    // TEST_DOMAIN: core
    // TEST_PRIORITY: normal

    #[test]
    fn test_network_config_to_url() {
        // TEST_CATEGORY: unit
        // TEST_DOMAIN: core
        // TEST_PRIORITY: normal
        let config = NetworkConfig {
            host: "127.0.0.1".to_string(),
            port: 9000,
        };
        // TEST_CATEGORY: unit
        // TEST_DOMAIN: core
        // TEST_PRIORITY: normal
        assert_eq!(config.to_url("http"), "http://127.0.0.1:9000");
    }

    #[test]
    fn test_discovery_config_defaults() {
        // Modern pattern: Test Default implementation directly (no env vars)
        // This is concurrent-safe and doesn't pollute global state
        let config = DiscoveryConfig::default();

        // TEST_CATEGORY: unit
        // TEST_DOMAIN: core
        // TEST_PRIORITY: normal
        // Canonical DiscoveryConfig uses endpoints: Vec<String>, timeout: Duration, max_attempts: u32
        // Default timeout is 5 seconds (canonical default)
        assert_eq!(config.timeout.as_secs(), 5);
        assert_eq!(config.max_attempts, 3);
        assert_eq!(config.max_concurrent, 10); // canonical default

        // Verify other defaults
        assert!(config.enabled);
        assert!(config.cache_enabled);
        assert_eq!(config.endpoints.len(), 0); // No endpoints in pure default
    }

    // TEST_CATEGORY: unit
    // TEST_DOMAIN: core
    // TEST_PRIORITY: normal
    #[test]
    fn test_hsm_config_defaults() {
        // Modern pattern: Test Default implementation directly (no env vars)
        // This is concurrent-safe and doesn't pollute global state
        let config = HsmConfig::default();

        // Verify default values
        assert_eq!(config.provider, "software");
        assert_eq!(config.timeout_ms, 5000);
        assert_eq!(config.retry_attempts, 3);
    }

    #[test]
    fn test_get_env_or_default_with_env_var() {
        beardog_errors::process_env::set_var("TEST_VAR_EXISTS", "custom_value");
        let result = get_env_or_default("TEST_VAR_EXISTS", "default_value");
        assert_eq!(result, "custom_value");
        beardog_errors::process_env::remove_var("TEST_VAR_EXISTS");
    }

    #[test]
    fn test_get_env_as_with_env_var() {
        const TEST_DISCOVERY_PORT: u16 = 9090;
        beardog_errors::process_env::set_var("TEST_PORT_EXISTS", TEST_DISCOVERY_PORT.to_string());
        let result: u16 = get_env_as("TEST_PORT_EXISTS", TEST_DEFAULT_PORT);
        assert_eq!(result, TEST_DISCOVERY_PORT);
        beardog_errors::process_env::remove_var("TEST_PORT_EXISTS");
    }

    #[test]
    fn test_get_env_as_with_invalid_value() {
        beardog_errors::process_env::set_var("TEST_PORT_INVALID", "not_a_number");
        let result: u16 = get_env_as("TEST_PORT_INVALID", TEST_DEFAULT_PORT);
        assert_eq!(result, TEST_DEFAULT_PORT); // Should fall back to default
        beardog_errors::process_env::remove_var("TEST_PORT_INVALID");
    }

    #[test]
    fn test_try_get_env_required_missing() {
        beardog_errors::process_env::remove_var("REQUIRED_VAR_MISSING");
        let err = try_get_env_required("REQUIRED_VAR_MISSING").expect_err("missing var should err");
        let msg = err.to_string();
        assert!(
            msg.contains("REQUIRED_VAR_MISSING") && msg.contains("not set"),
            "unexpected message: {msg}"
        );
    }

    #[test]
    fn test_try_get_env_required_present() {
        beardog_errors::process_env::set_var("REQUIRED_VAR_PRESENT", "required_value");
        let result = try_get_env_required("REQUIRED_VAR_PRESENT").expect("var is set");
        assert_eq!(result, "required_value");
        beardog_errors::process_env::remove_var("REQUIRED_VAR_PRESENT");
    }

    #[test]
    fn test_network_config_from_env_custom() {
        const TEST_CUSTOM_PORT: u16 = 3000;
        beardog_errors::process_env::set_var("CUSTOM_HOST", "192.168.1.1");
        beardog_errors::process_env::set_var("CUSTOM_PORT", TEST_CUSTOM_PORT.to_string());

        let config = NetworkConfig::from_env("CUSTOM");
        assert_eq!(config.host, "192.168.1.1");
        assert_eq!(config.port, TEST_CUSTOM_PORT);

        beardog_errors::process_env::remove_var("CUSTOM_HOST");
        beardog_errors::process_env::remove_var("CUSTOM_PORT");
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
        let config = TimeoutConfig::from_env("DEFAULT");
        assert_eq!(config.connect_timeout, Duration::from_millis(5000));
        assert_eq!(config.request_timeout, Duration::from_millis(30000));
        assert_eq!(config.idle_timeout, Duration::from_millis(60000));
    }

    #[test]
    fn test_timeout_config_from_env_custom() {
        beardog_errors::process_env::set_var("CUSTOM_CONNECT_TIMEOUT_MS", "1000");
        beardog_errors::process_env::set_var("CUSTOM_REQUEST_TIMEOUT_MS", "10000");
        beardog_errors::process_env::set_var("CUSTOM_IDLE_TIMEOUT_MS", "30000");

        let config = TimeoutConfig::from_env("CUSTOM");
        assert_eq!(config.connect_timeout, Duration::from_millis(1000));
        assert_eq!(config.request_timeout, Duration::from_millis(10000));
        assert_eq!(config.idle_timeout, Duration::from_millis(30000));

        beardog_errors::process_env::remove_var("CUSTOM_CONNECT_TIMEOUT_MS");
        beardog_errors::process_env::remove_var("CUSTOM_REQUEST_TIMEOUT_MS");
        beardog_errors::process_env::remove_var("CUSTOM_IDLE_TIMEOUT_MS");
    }

    #[test]
    fn test_discovery_config_from_env_custom() {
        // Use a lock to ensure this test runs serially with other env-modifying tests
        let _lock = ENV_TEST_MUTEX
            .lock()
            .expect("ENV_TEST_MUTEX should not be poisoned");

        // Save current env state
        let old_endpoint = env::var("BEARDOG_DISCOVERY_ENDPOINT").ok();
        let old_timeout = env::var("BEARDOG_DISCOVERY_TIMEOUT_SECS").ok();
        let old_retry = env::var("BEARDOG_DISCOVERY_RETRY_ATTEMPTS").ok();

        beardog_errors::process_env::set_var(
            "BEARDOG_DISCOVERY_ENDPOINT",
            "http://custom:9000/api",
        );
        beardog_errors::process_env::set_var("BEARDOG_DISCOVERY_TIMEOUT_SECS", "60");
        beardog_errors::process_env::set_var("BEARDOG_DISCOVERY_MAX_ATTEMPTS", "5");

        let config = DiscoveryConfig::from_env();
        // Canonical DiscoveryConfig from_env() loads BEARDOG_DISCOVERY_ENDPOINT into endpoints vec
        assert!(
            config
                .endpoints
                .iter()
                .any(|e| e == "http://custom:9000/api")
        );
        assert_eq!(config.timeout.as_secs(), 60);
        assert_eq!(config.max_attempts, 5);

        // Restore original env state
        match old_endpoint {
            Some(val) => beardog_errors::process_env::set_var("BEARDOG_DISCOVERY_ENDPOINT", val),
            None => beardog_errors::process_env::remove_var("BEARDOG_DISCOVERY_ENDPOINT"),
        }
        match old_timeout {
            Some(val) => {
                beardog_errors::process_env::set_var("BEARDOG_DISCOVERY_TIMEOUT_SECS", val)
            }
            None => beardog_errors::process_env::remove_var("BEARDOG_DISCOVERY_TIMEOUT_SECS"),
        }
        match old_retry {
            Some(val) => {
                beardog_errors::process_env::set_var("BEARDOG_DISCOVERY_RETRY_ATTEMPTS", val)
            }
            None => beardog_errors::process_env::remove_var("BEARDOG_DISCOVERY_RETRY_ATTEMPTS"),
        }
    }

    #[test]
    fn test_hsm_config_from_env_custom() {
        // Lock mutex to prevent parallel test interference with env vars
        let _lock = ENV_TEST_MUTEX
            .lock()
            .expect("ENV_TEST_MUTEX should not be poisoned");

        // Use a guard to ensure cleanup even if test fails
        struct EnvGuard;
        impl Drop for EnvGuard {
            fn drop(&mut self) {
                beardog_errors::process_env::remove_var("BEARDOG_HSM_PROVIDER");
                beardog_errors::process_env::remove_var("BEARDOG_HSM_TIMEOUT_MS");
                beardog_errors::process_env::remove_var("BEARDOG_HSM_RETRY_ATTEMPTS");
            }
        }
        let _guard = EnvGuard;

        beardog_errors::process_env::set_var("BEARDOG_HSM_PROVIDER", "hardware");
        beardog_errors::process_env::set_var("BEARDOG_HSM_TIMEOUT_MS", "10000");
        beardog_errors::process_env::set_var("BEARDOG_HSM_RETRY_ATTEMPTS", "5");

        let config = HsmConfig::from_env();
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
