// SPDX-License-Identifier: AGPL-3.0-or-later

//! Test Fixtures for Configuration
//!
//! Provides reusable test fixtures and helpers for testing code that uses
//! network configuration, eliminating hardcoded values in tests.
//!
//! # Usage
//!
//! ```rust
//! use beardog_types::canonical::config::test_fixtures::{TestNetworkConfig, test_api_url};
//!
//! fn my_test() {
//!     let config = TestNetworkConfig::local();
//!     let url = test_api_url(); // Returns a test URL
//!     // Use config in tests...
//! }
//! ```

use super::runtime_config::{RuntimeConfig, RuntimeHsmConfig, RuntimeNetworkConfig};
use std::sync::Once;

static INIT: Once = Once::new();

/// Initialize test environment once
///
/// Sets up test environment variables for consistent testing.
/// Safe to call multiple times - initialization only happens once.
///
/// NOTE: Use `TestNetworkConfig` presets instead of this function for isolated tests.
/// This function sets GLOBAL environment variables that affect all tests.
pub fn init_test_env() {
    INIT.call_once(|| {
        // Set test environment variables
        // Using sensible defaults to avoid breaking validation tests
        beardog_errors::process_env::set_var("BEARDOG_ENVIRONMENT", "test");
        beardog_errors::process_env::set_var("BEARDOG_API_HOST", "127.0.0.1");
        beardog_errors::process_env::set_var("BEARDOG_API_PORT", "8080"); // Valid port for tests
        beardog_errors::process_env::set_var("BEARDOG_ENABLE_TLS", "true"); // TLS enabled by default for security
    });
}

/// Test network configuration presets
///
/// Provides common test configurations for different scenarios.
pub struct TestNetworkConfig;

impl TestNetworkConfig {
    /// Local development test configuration
    ///
    /// Uses localhost with standard ports, TLS disabled.
    /// Suitable for unit and integration tests.
    pub fn local() -> RuntimeNetworkConfig {
        RuntimeNetworkConfig {
            discovery_endpoint: "http://127.0.0.1:8080/discover".to_string(),
            api_host: "127.0.0.1".to_string(),
            api_port: 8080,
            metrics_port: 9090,
            health_port: 8081,
            ws_port: 3000,
            grpc_port: 50051,
            admin_port: 8082,
            database_port: 5432,
            consul_port: 8500,
            redis_port: 6379,
            timeout_seconds: 5,  // Shorter for tests
            max_connections: 10, // Lower for tests
            enable_tls: false,   // Disabled for tests
        }
    }

    /// Mock external service configuration
    ///
    /// Uses mock server addresses for testing external integrations.
    pub fn mock_external() -> RuntimeNetworkConfig {
        RuntimeNetworkConfig {
            discovery_endpoint: "http://mock.example.com:8080/discover".to_string(),
            api_host: "mock.example.com".to_string(),
            api_port: 8080,
            metrics_port: 9090,
            health_port: 8081,
            ws_port: 3000,
            grpc_port: 50051,
            admin_port: 8082,
            database_port: 5432,
            consul_port: 8500,
            redis_port: 6379,
            timeout_seconds: 1, // Very short for mocks
            max_connections: 1,
            enable_tls: false,
        }
    }

    /// Production-like test configuration
    ///
    /// Mimics production settings for integration tests.
    /// Uses TLS and production-like timeouts.
    pub fn production_like() -> RuntimeNetworkConfig {
        RuntimeNetworkConfig {
            discovery_endpoint: "https://api.test.example.com/discover".to_string(),
            api_host: "api.test.example.com".to_string(),
            api_port: 443,
            metrics_port: 9090,
            health_port: 8081,
            ws_port: 443,
            grpc_port: 443,
            admin_port: 443,
            database_port: 5432,
            consul_port: 8500,
            redis_port: 6379,
            timeout_seconds: 30,
            max_connections: 100,
            enable_tls: true,
        }
    }

    /// Ephemeral test configuration with random ports
    ///
    /// Uses port 0 to let the OS assign random available ports.
    /// Ideal for parallel test execution without port conflicts.
    pub fn ephemeral() -> RuntimeNetworkConfig {
        RuntimeNetworkConfig {
            discovery_endpoint: "http://127.0.0.1:0/discover".to_string(),
            api_host: "127.0.0.1".to_string(),
            api_port: 0, // OS assigns random port
            metrics_port: 0,
            health_port: 0,
            ws_port: 0,
            grpc_port: 0,
            admin_port: 0,
            database_port: 0,
            consul_port: 0,
            redis_port: 0,
            timeout_seconds: 5,
            max_connections: 10,
            enable_tls: false,
        }
    }
}

/// Test HSM configuration presets
pub struct TestHsmConfig;

impl TestHsmConfig {
    /// Software HSM for testing
    ///
    /// Uses in-memory software HSM, no hardware required.
    pub fn software() -> RuntimeHsmConfig {
        RuntimeHsmConfig {
            pkcs11_library_path: "/usr/lib/softhsm/libsofthsm2.so".to_string(),
            tpm_device_path: "/dev/null".to_string(), // No TPM in tests
            software_hsm_storage: "/tmp/beardog-test-hsm".to_string(),
            enable_hardware_hsm: false,
        }
    }

    /// Mock hardware HSM for testing
    ///
    /// Simulates hardware HSM without actual hardware.
    pub fn mock_hardware() -> RuntimeHsmConfig {
        RuntimeHsmConfig {
            pkcs11_library_path: "/tmp/mock-pkcs11.so".to_string(),
            tpm_device_path: "/tmp/mock-tpm".to_string(),
            software_hsm_storage: "/tmp/beardog-test-hsm".to_string(),
            enable_hardware_hsm: false, // Still use software in tests
        }
    }
}

/// Complete test runtime configuration
pub struct TestRuntimeConfig;

impl TestRuntimeConfig {
    /// Standard test configuration
    ///
    /// Combines local network with software HSM.
    pub fn standard() -> RuntimeConfig {
        RuntimeConfig {
            network: TestNetworkConfig::local(),
            hsm: TestHsmConfig::software(),
            environment: "test".to_string(),
        }
    }

    /// Production-like test configuration
    ///
    /// Mimics production setup for integration testing.
    pub fn production_like() -> RuntimeConfig {
        RuntimeConfig {
            network: TestNetworkConfig::production_like(),
            hsm: TestHsmConfig::mock_hardware(),
            environment: "staging".to_string(),
        }
    }

    /// Ephemeral test configuration
    ///
    /// Uses random ports for parallel test execution.
    pub fn ephemeral() -> RuntimeConfig {
        RuntimeConfig {
            network: TestNetworkConfig::ephemeral(),
            hsm: TestHsmConfig::software(),
            environment: "test".to_string(),
        }
    }
}

// Convenience functions for common test values

/// Get test API URL
///
/// Returns a consistent test API URL for all tests.
pub fn test_api_url() -> String {
    "http://127.0.0.1:8080".to_string()
}

/// Get test API host
pub fn test_api_host() -> String {
    "127.0.0.1".to_string()
}

/// Get test API port
pub const fn test_api_port() -> u16 {
    8080
}

/// Get test metrics URL
pub fn test_metrics_url() -> String {
    "http://127.0.0.1:9090/metrics".to_string()
}

/// Get test health URL
pub fn test_health_url() -> String {
    "http://127.0.0.1:8081/health".to_string()
}

/// Get test WebSocket URL
pub fn test_ws_url() -> String {
    "ws://127.0.0.1:3000".to_string()
}

/// Get test gRPC endpoint
pub fn test_grpc_endpoint() -> String {
    "127.0.0.1:50051".to_string()
}

/// Get test discovery endpoint
pub fn test_discovery_endpoint() -> String {
    "http://127.0.0.1:8080/discover".to_string()
}

/// Get test database URL
pub fn test_database_url() -> String {
    "postgresql://test:test@127.0.0.1:5432/beardog_test".to_string()
}

/// Get test Redis URL
pub fn test_redis_url() -> String {
    "redis://127.0.0.1:6379/0".to_string()
}

/// Test helper macros
#[macro_export]
macro_rules! with_test_config {
    ($test_fn:expr) => {{
        use $crate::canonical::config::test_fixtures::init_test_env;
        init_test_env();
        $test_fn
    }};
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_local_config() {
        let config = TestNetworkConfig::local();
        assert_eq!(config.api_host, "127.0.0.1");
        assert_eq!(config.api_port, 8080);
        assert!(!config.enable_tls);
    }

    #[test]
    fn test_production_like_config() {
        let config = TestNetworkConfig::production_like();
        assert!(config.enable_tls);
        assert_eq!(config.api_port, 443);
    }

    #[test]
    fn test_ephemeral_config() {
        let config = TestNetworkConfig::ephemeral();
        assert_eq!(config.api_port, 0); // Random port
    }

    #[test]
    fn test_software_hsm() {
        let config = TestHsmConfig::software();
        assert!(!config.enable_hardware_hsm);
        assert!(config.pkcs11_library_path.contains("softhsm"));
    }

    #[test]
    fn test_standard_runtime_config() {
        let config = TestRuntimeConfig::standard();
        assert_eq!(config.environment, "test");
        assert!(!config.network.enable_tls);
        assert!(!config.hsm.enable_hardware_hsm);
    }

    #[test]
    fn test_convenience_functions() {
        assert_eq!(test_api_url(), "http://127.0.0.1:8080");
        assert_eq!(test_api_host(), "127.0.0.1");
        assert_eq!(test_api_port(), 8080);
        assert_eq!(test_metrics_url(), "http://127.0.0.1:9090/metrics");
        assert_eq!(test_health_url(), "http://127.0.0.1:8081/health");
        assert_eq!(test_ws_url(), "ws://127.0.0.1:3000");
        assert_eq!(test_grpc_endpoint(), "127.0.0.1:50051");
        assert_eq!(test_discovery_endpoint(), "http://127.0.0.1:8080/discover");
    }

    #[test]
    fn test_init_env() {
        init_test_env();
        // Should not panic if called multiple times
        init_test_env();
        init_test_env();
    }
}
