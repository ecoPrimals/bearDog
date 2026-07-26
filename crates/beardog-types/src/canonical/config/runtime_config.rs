// SPDX-License-Identifier: AGPL-3.0-or-later

//! Runtime Configuration Management
//!
//! Centralized configuration for network endpoints, ports, and other runtime values
//! that were previously hardcoded. All values can be overridden via environment variables.
//!
//! # Environment-Driven Configuration
//!
//! This module provides runtime configuration that eliminates hardcoded values
//! by reading from environment variables with sensible defaults.
//!
//! # Examples
//!
//! ```rust
//! use beardog_types::canonical::config::runtime_config::RuntimeNetworkConfig;
//!
//! // Loads from environment, falls back to defaults
//! let config = RuntimeNetworkConfig::from_env();
//! let api_url = config.api_url();
//! ```
//!
//! # Environment Variables
//!
//! All configuration can be overridden via environment variables:
//!
//! - `BEARDOG_API_HOST` - API server hostname (default: 127.0.0.1)
//! - `BEARDOG_API_PORT` - API server port (default: 8080)
//! - `BEARDOG_METRICS_PORT` - Metrics port (default: 9190)
//! - `BEARDOG_HEALTH_PORT` - Health check port (default: 8081)
//! - `BEARDOG_WS_PORT` - WebSocket port (default: 3000)
//! - `BEARDOG_GRPC_PORT` - gRPC port (default: 50051)
//! - `BEARDOG_ENABLE_TLS` - Enable TLS (default: true)

use beardog_config::env_keys;
use serde::{Deserialize, Serialize};

/// Runtime network configuration with environment variable overrides
///
/// Provides network configuration for `BearDog` services with all values
/// configurable via environment variables. Eliminates hardcoded network
/// addresses and ports for flexible deployment.
///
/// # Environment Variables
///
/// | Variable | Default | Description |
/// |----------|---------|-------------|
/// | `BEARDOG_API_HOST` | `127.0.0.1` | API server hostname |
/// | `BEARDOG_API_PORT` | `8080` | API server port |
/// | `BEARDOG_METRICS_PORT` | `9190` | Prometheus metrics port |
/// | `BEARDOG_HEALTH_PORT` | `8081` | Health check port |
/// | `BEARDOG_WS_PORT` | `3000` | WebSocket port |
/// | `BEARDOG_GRPC_PORT` | `50051` | gRPC port |
/// | `BEARDOG_TIMEOUT_SECONDS` | `30` | Connection timeout |
/// | `BEARDOG_MAX_CONNECTIONS` | `1000` | Max concurrent connections |
/// | `BEARDOG_ENABLE_TLS` | `true` | Enable TLS/SSL |
///
/// # Examples
///
/// ## Using Environment Variables
///
/// ```bash
/// export BEARDOG_API_HOST=api.example.com
/// export BEARDOG_API_PORT=443
/// export BEARDOG_ENABLE_TLS=true
/// ```
///
/// ```rust
/// use beardog_types::canonical::config::runtime_config::RuntimeNetworkConfig;
///
/// // Without environment variables set, uses safe defaults from NetworkConfig
/// let config = RuntimeNetworkConfig::from_env();
/// // The default host comes from NetworkConfig which uses "127.0.0.1"
/// assert_eq!(config.api_url(), "https://127.0.0.1:8080");
/// ```
///
/// ## Using Defaults
///
/// ```rust
/// use beardog_types::canonical::config::runtime_config::RuntimeNetworkConfig;
///
/// // Without environment variables, uses safe defaults from NetworkConfig
/// let config = RuntimeNetworkConfig::default();
/// assert_eq!(config.api_host, "127.0.0.1");
/// assert_eq!(config.api_port, 8080);
/// ```
///
/// # Production Deployment
///
/// For production, always set:
/// - `BEARDOG_API_HOST` to your domain name
/// - `BEARDOG_ENABLE_TLS=true` for security
/// - Appropriate ports for your infrastructure
/// - `BEARDOG_MAX_CONNECTIONS` based on available resources
///
/// # Security Considerations
///
/// - **TLS**: Always enable in production (`BEARDOG_ENABLE_TLS=true`)
/// - **Network Isolation**: Use firewall rules to restrict access
/// - **Port Selection**: Avoid well-known ports if running as non-root
/// - **Connection Limits**: Set `max_connections` to prevent exhaustion
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct RuntimeNetworkConfig {
    /// Discovery endpoint URL for service discovery
    ///
    /// Full URL to the service discovery endpoint. Can be set via
    /// `BEARDOG_DISCOVERY_ENDPOINT` environment variable.
    pub discovery_endpoint: String,

    /// API server hostname or IP address
    ///
    /// The hostname or IP address where the API server listens.
    /// Set via `BEARDOG_API_HOST` (default: 127.0.0.1).
    pub api_host: String,

    /// API server port
    ///
    /// The port where the API server listens.
    /// Set via `BEARDOG_API_PORT` (default: 8080).
    pub api_port: u16,

    /// Prometheus metrics endpoint port
    ///
    /// Port for exposing Prometheus-compatible metrics.
    /// Set via `BEARDOG_METRICS_PORT` (default: 9190).
    pub metrics_port: u16,

    /// Health check endpoint port
    ///
    /// Port for health check and readiness probes.
    /// Set via `BEARDOG_HEALTH_PORT` (default: 8081).
    pub health_port: u16,

    /// WebSocket server port
    ///
    /// Port for WebSocket connections.
    /// Set via `BEARDOG_WS_PORT` (default: 3000).
    pub ws_port: u16,

    /// gRPC server port
    ///
    /// Port for gRPC service endpoints.
    /// Set via `BEARDOG_GRPC_PORT` (default: 50051).
    pub grpc_port: u16,

    /// Admin/management port
    ///
    /// Port for administrative operations and management UI.
    /// Set via `BEARDOG_ADMIN_PORT` (default: 8082).
    pub admin_port: u16,

    /// Database port (`PostgreSQL`)
    ///
    /// Port for `PostgreSQL` database connections.
    /// Set via `BEARDOG_DATABASE_PORT` (default: 5432).
    pub database_port: u16,

    /// Service discovery port (Consul)
    ///
    /// Port for Consul service discovery.
    /// Set via `BEARDOG_CONSUL_PORT` (default: 8500).
    pub consul_port: u16,

    /// Redis cache port
    ///
    /// Port for Redis caching and pub/sub.
    /// Set via `BEARDOG_REDIS_PORT` (default: 6379).
    pub redis_port: u16,

    /// Connection timeout in seconds
    ///
    /// Maximum time to wait for connection establishment.
    /// Set via `BEARDOG_TIMEOUT_SECONDS` (default: 30).
    pub timeout_seconds: u64,

    /// Maximum concurrent connections
    ///
    /// Maximum number of simultaneous connections allowed.
    /// Set via `BEARDOG_MAX_CONNECTIONS` (default: 1000).
    pub max_connections: usize,

    /// Enable TLS/SSL encryption
    ///
    /// Whether to use HTTPS/WSS instead of HTTP/WS.
    /// Set via `BEARDOG_ENABLE_TLS` (default: true for production safety).
    pub enable_tls: bool,
}

impl Default for RuntimeNetworkConfig {
    fn default() -> Self {
        // Constants must be declared before any other statements
        use beardog_config::domains::network_ports::{
            DEFAULT_ADMIN_PORT, DEFAULT_CONSUL_PORT, DEFAULT_DATABASE_PORT, DEFAULT_GRPC_PORT,
            DEFAULT_REDIS_PORT,
        };

        const DEFAULT_TIMEOUT_SECONDS: u64 = 30;
        const DEFAULT_MAX_CONNECTIONS: usize = 1000;

        // Default values - all configurable via environment variables
        // Use NetworkConfig for consistent defaults where possible
        let network_defaults = super::network::NetworkConfig::default();
        let default_api_host = network_defaults.default_host.as_str();

        let default_api_port = network_defaults.service_ports.api_port;
        let default_metrics_port = network_defaults.service_ports.metrics_port;
        let default_health_port = network_defaults.service_ports.health_port;
        let default_ws_port = network_defaults.service_ports.websocket_port;
        let default_grpc_port = DEFAULT_GRPC_PORT;
        let default_admin_port = DEFAULT_ADMIN_PORT;
        let default_database_port = DEFAULT_DATABASE_PORT;
        let default_consul_port = DEFAULT_CONSUL_PORT;
        let default_redis_port = DEFAULT_REDIS_PORT;

        Self {
            discovery_endpoint: std::env::var(env_keys::ENV_DISCOVERY_ENDPOINT).unwrap_or_else(
                |_| format!("http://{default_api_host}:{default_api_port}/discover"),
            ),
            api_host: std::env::var(env_keys::ENV_API_HOST)
                .unwrap_or_else(|_| default_api_host.to_string()),
            api_port: std::env::var(env_keys::ENV_API_PORT)
                .ok()
                .and_then(|p| p.parse().ok())
                .unwrap_or(default_api_port),
            metrics_port: std::env::var(env_keys::ENV_METRICS_PORT)
                .ok()
                .and_then(|p| p.parse().ok())
                .unwrap_or(default_metrics_port),
            health_port: std::env::var(env_keys::ENV_HEALTH_PORT)
                .ok()
                .and_then(|p| p.parse().ok())
                .unwrap_or(default_health_port),
            ws_port: std::env::var(env_keys::ENV_WS_PORT)
                .ok()
                .and_then(|p| p.parse().ok())
                .unwrap_or(default_ws_port),
            grpc_port: std::env::var(env_keys::ENV_GRPC_PORT)
                .ok()
                .and_then(|p| p.parse().ok())
                .unwrap_or(default_grpc_port),
            admin_port: std::env::var(env_keys::ENV_ADMIN_PORT)
                .ok()
                .and_then(|p| p.parse().ok())
                .unwrap_or(default_admin_port),
            database_port: std::env::var(env_keys::ENV_DATABASE_PORT)
                .ok()
                .and_then(|p| p.parse().ok())
                .unwrap_or(default_database_port),
            consul_port: std::env::var(env_keys::ENV_CONSUL_PORT)
                .ok()
                .and_then(|p| p.parse().ok())
                .unwrap_or(default_consul_port),
            redis_port: std::env::var(env_keys::ENV_REDIS_PORT)
                .ok()
                .and_then(|p| p.parse().ok())
                .unwrap_or(default_redis_port),
            timeout_seconds: std::env::var(env_keys::ENV_TIMEOUT_SECONDS)
                .ok()
                .and_then(|t| t.parse().ok())
                .unwrap_or(DEFAULT_TIMEOUT_SECONDS),
            max_connections: std::env::var(env_keys::ENV_MAX_CONNECTIONS)
                .ok()
                .and_then(|m| m.parse().ok())
                .unwrap_or(DEFAULT_MAX_CONNECTIONS),
            enable_tls: std::env::var(env_keys::ENV_ENABLE_TLS)
                .ok()
                .and_then(|e| e.parse().ok())
                .unwrap_or(true),
        }
    }
}

impl RuntimeNetworkConfig {
    /// Create configuration from environment variables
    #[must_use]
    pub fn from_env() -> Self {
        Self::default()
    }

    /// Get full API URL
    #[must_use]
    pub fn api_url(&self) -> String {
        let protocol = if self.enable_tls { "https" } else { "http" };
        format!("{}://{}:{}", protocol, self.api_host, self.api_port)
    }

    /// Get metrics URL
    #[must_use]
    pub fn metrics_url(&self) -> String {
        format!("http://{}:{}/metrics", self.api_host, self.metrics_port)
    }

    /// Get health check URL
    #[must_use]
    pub fn health_url(&self) -> String {
        format!("http://{}:{}/health", self.api_host, self.health_port)
    }

    /// Get WebSocket URL
    #[must_use]
    pub fn ws_url(&self) -> String {
        let protocol = if self.enable_tls { "wss" } else { "ws" };
        format!("{}://{}:{}", protocol, self.api_host, self.ws_port)
    }

    /// Get gRPC endpoint
    #[must_use]
    pub fn grpc_endpoint(&self) -> String {
        format!("{}:{}", self.api_host, self.grpc_port)
    }
}

/// Runtime HSM (Hardware Security Module) configuration
///
/// Provides configuration paths and settings for various HSM types including
/// PKCS#11 devices, TPM (Trusted Platform Module), and software HSM.
///
/// # Supported HSM Types
///
/// - **PKCS#11**: Industry-standard cryptographic token interface
/// - **TPM**: Trusted Platform Module (hardware security chip)
/// - **Software HSM**: Pure software implementation for development/testing
///
/// # Environment Variables
///
/// | Variable | Default | Description |
/// |----------|---------|-------------|
/// | `BEARDOG_PKCS11_LIBRARY` | `/usr/lib/softhsm/libsofthsm2.so` | PKCS#11 library path |
/// | `BEARDOG_TPM_DEVICE` | `/dev/tpm0` | TPM device path |
/// | `BEARDOG_HSM_STORAGE` | `/var/lib/beardog/hsm` | Software HSM storage |
/// | `BEARDOG_ENABLE_HARDWARE_HSM` | `false` | Enable hardware HSM |
///
/// # Examples
///
/// ## Using `SoftHSM` (Development)
///
/// ```bash
/// export BEARDOG_PKCS11_LIBRARY=/usr/lib/softhsm/libsofthsm2.so
/// export BEARDOG_ENABLE_HARDWARE_HSM=false
/// ```
///
/// ## Using Hardware HSM (Production)
///
/// ```bash
/// export BEARDOG_PKCS11_LIBRARY=/usr/lib/libpkcs11.so
/// export BEARDOG_ENABLE_HARDWARE_HSM=true
/// ```
///
/// ## Using TPM
///
/// ```bash
/// export BEARDOG_TPM_DEVICE=/dev/tpm0
/// export BEARDOG_ENABLE_HARDWARE_HSM=true
/// ```
///
/// # Security Considerations
///
/// - **Production**: Always use hardware HSM (`enable_hardware_hsm=true`)
/// - **Development**: Software HSM acceptable for testing
/// - **PKCS#11 Library**: Ensure library is from trusted source
/// - **TPM**: Verify TPM is enabled in BIOS/UEFI
/// - **Storage**: Protect software HSM storage directory (0700 permissions)
///
/// # See Also
///
/// - `beardog_tunnel::hsm` - HSM implementation
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct RuntimeHsmConfig {
    /// PKCS#11 cryptographic token library path
    ///
    /// Path to the PKCS#11 shared library (.so/.dll).
    /// Common locations:
    /// - `SoftHSM`: `/usr/lib/softhsm/libsofthsm2.so`
    /// - Hardware: `/usr/lib/libpkcs11.so`
    ///
    /// Set via `BEARDOG_PKCS11_LIBRARY`.
    pub pkcs11_library_path: String,

    /// TPM (Trusted Platform Module) device path
    ///
    /// File system path to the TPM character device.
    /// Typically `/dev/tpm0` on Linux systems.
    ///
    /// Set via `BEARDOG_TPM_DEVICE` (default: `/dev/tpm0`).
    pub tpm_device_path: String,

    /// Software HSM storage directory
    ///
    /// Directory for storing software HSM keys and state.
    /// Should have restrictive permissions (0700) in production.
    ///
    /// Set via `BEARDOG_HSM_STORAGE` (default: `/var/lib/beardog/hsm`).
    pub software_hsm_storage: String,

    /// Enable hardware HSM usage
    ///
    /// When true, prefer hardware HSM over software implementation.
    /// Should be enabled in production for security.
    ///
    /// Set via `BEARDOG_ENABLE_HARDWARE_HSM` (default: false).
    pub enable_hardware_hsm: bool,
}

impl Default for RuntimeHsmConfig {
    fn default() -> Self {
        Self {
            pkcs11_library_path: std::env::var(env_keys::ENV_PKCS11_LIBRARY)
                .unwrap_or_else(|_| "/usr/lib/softhsm/libsofthsm2.so".to_string()),
            tpm_device_path: std::env::var(env_keys::ENV_TPM_DEVICE)
                .unwrap_or_else(|_| "/dev/tpm0".to_string()),
            software_hsm_storage: std::env::var(env_keys::ENV_HSM_STORAGE)
                .unwrap_or_else(|_| "/var/lib/beardog/hsm".to_string()),
            enable_hardware_hsm: std::env::var(env_keys::ENV_ENABLE_HARDWARE_HSM)
                .ok()
                .and_then(|e| e.parse().ok())
                .unwrap_or(false),
        }
    }
}

impl RuntimeHsmConfig {
    /// Create configuration from environment variables
    #[must_use]
    pub fn from_env() -> Self {
        Self::default()
    }
}

/// Complete runtime configuration for `BearDog` services
///
/// Combines network and HSM configuration with environment detection.
/// All settings are configurable via environment variables for flexible deployment.
///
/// # Components
///
/// - **Network**: API endpoints, ports, timeouts, and TLS settings
/// - **HSM**: Hardware security module paths and configuration
/// - **Environment**: Deployment environment (development/staging/production)
///
/// # Examples
///
/// ## Load Complete Configuration
///
/// ```rust
/// use beardog_types::canonical::config::runtime_config::RuntimeConfig;
///
/// let config = RuntimeConfig::from_env();
///
/// // Check environment
/// if config.is_production() {
///     assert!(config.network.enable_tls, "TLS required in production");
///     assert!(config.hsm.enable_hardware_hsm, "Hardware HSM required in production");
/// }
/// ```
///
/// ## Environment-Specific Logic
///
/// ```rust
/// use beardog_types::canonical::config::runtime_config::RuntimeConfig;
///
/// let config = RuntimeConfig::from_env();
///
/// match config.environment.as_str() {
///     "production" => println!("Running in production mode"),
///     "staging" => println!("Running in staging mode"),
///     "development" => println!("Running in development mode"),
///     _ => println!("Unknown environment"),
/// }
/// ```
///
/// # Environment Variable
///
/// Set via `BEARDOG_ENVIRONMENT` (default: "development"):
/// - `development` - Local development with relaxed security
/// - `staging` - Pre-production testing with production-like settings
/// - `production` - Production deployment with strict security
///
/// # Production Checklist
///
/// For production deployments, ensure:
/// - ✅ `BEARDOG_ENVIRONMENT=production`
/// - ✅ `BEARDOG_ENABLE_TLS=true`
/// - ✅ `BEARDOG_ENABLE_HARDWARE_HSM=true`
/// - ✅ `BEARDOG_API_HOST` set to production domain
/// - ✅ All secrets managed securely (not in environment)
///
/// # See Also
///
/// - [`RuntimeNetworkConfig`] - Network configuration details
/// - [`RuntimeHsmConfig`] - HSM configuration details
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct RuntimeConfig {
    /// Network configuration
    ///
    /// Contains all network-related settings: API endpoints, ports,
    /// timeouts, and TLS configuration.
    pub network: RuntimeNetworkConfig,

    /// HSM configuration
    ///
    /// Contains hardware security module paths and settings for
    /// PKCS#11, TPM, and software HSM.
    pub hsm: RuntimeHsmConfig,

    /// Current deployment environment
    ///
    /// One of: "development", "staging", or "production".
    /// Affects security settings and operational behavior.
    ///
    /// Set via `BEARDOG_ENVIRONMENT` (default: "development").
    pub environment: String,
}

impl Default for RuntimeConfig {
    fn default() -> Self {
        Self {
            network: RuntimeNetworkConfig::default(),
            hsm: RuntimeHsmConfig::default(),
            environment: std::env::var(env_keys::ENV_ENVIRONMENT)
                .unwrap_or_else(|_| "development".to_string()),
        }
    }
}

impl RuntimeConfig {
    /// Create configuration from environment variables
    #[must_use]
    pub fn from_env() -> Self {
        Self::default()
    }

    /// Check if running in production
    #[must_use]
    pub fn is_production(&self) -> bool {
        self.environment.to_lowercase() == "production"
    }

    /// Check if running in development
    #[must_use]
    pub fn is_development(&self) -> bool {
        self.environment.to_lowercase() == "development"
    }

    /// Check if running in staging
    #[must_use]
    pub fn is_staging(&self) -> bool {
        self.environment.to_lowercase() == "staging"
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_default_network_config() {
        use beardog_config::domains::network_ports::{DEFAULT_API_PORT, DEFAULT_METRICS_PORT};

        let config = RuntimeNetworkConfig::default();
        assert_eq!(config.api_port, DEFAULT_API_PORT);
        assert_eq!(config.metrics_port, DEFAULT_METRICS_PORT);
        assert!(config.enable_tls);
    }

    #[test]
    fn test_api_url_generation() {
        let config = RuntimeNetworkConfig {
            api_host: "example.com".to_string(),
            api_port: 8443,
            enable_tls: true,
            ..Default::default() // TEST_CATEGORY: unit
                                 // TEST_DOMAIN: types
                                 // TEST_PRIORITY: normal
        };
        assert_eq!(config.api_url(), "https://example.com:8443");
    }

    #[test]
    // TEST_CATEGORY: unit
    // TEST_DOMAIN: types
    // TEST_PRIORITY: normal
    fn test_api_url_without_tls() {
        use beardog_config::domains::network_ports::DEFAULT_API_PORT;

        let config = RuntimeNetworkConfig {
            api_host: "localhost".to_string(),
            api_port: DEFAULT_API_PORT,
            enable_tls: false,
            ..Default::default()
        };
        assert_eq!(
            config.api_url(),
            format!("http://localhost:{DEFAULT_API_PORT}")
        );
        // TEST_CATEGORY: unit
        // TEST_DOMAIN: types
        // TEST_PRIORITY: normal
    }

    #[test]
    fn test_environment_detection() {
        let config = RuntimeConfig {
            environment: "production".to_string(),
            ..Default::default()
        };
        // TEST_CATEGORY: unit
        // TEST_DOMAIN: types
        // TEST_PRIORITY: normal
        assert!(config.is_production());
        assert!(!config.is_development());
    }

    #[test]
    fn test_hsm_config_defaults() {
        let config = RuntimeHsmConfig::default();
        // TEST_CATEGORY: unit
        // TEST_DOMAIN: types
        // TEST_PRIORITY: normal
        assert!(config.pkcs11_library_path.contains("softhsm"));
        assert_eq!(config.tpm_device_path, "/dev/tpm0");
        assert!(!config.enable_hardware_hsm);
    }

    // TEST_CATEGORY: unit
    // TEST_DOMAIN: types
    // TEST_PRIORITY: normal
    #[test]
    fn test_url_helpers() {
        use beardog_config::domains::network_ports::{
            DEFAULT_API_PORT, DEFAULT_DISCOVERY_PORT, DEFAULT_HEALTH_PORT,
        };

        const WS_PORT: u16 = 3000;
        const GRPC_PORT: u16 = 50051;

        let config = RuntimeNetworkConfig {
            api_host: "localhost".to_string(),
            api_port: DEFAULT_API_PORT,
            metrics_port: DEFAULT_DISCOVERY_PORT,
            health_port: DEFAULT_HEALTH_PORT,
            ws_port: WS_PORT,
            grpc_port: GRPC_PORT,
            enable_tls: false,
            ..Default::default()
        };

        assert_eq!(
            config.api_url(),
            format!("http://localhost:{DEFAULT_API_PORT}")
        );
        assert_eq!(
            config.metrics_url(),
            format!("http://localhost:{DEFAULT_DISCOVERY_PORT}/metrics")
        );
        assert_eq!(
            config.health_url(),
            format!("http://localhost:{DEFAULT_HEALTH_PORT}/health")
        );
        assert_eq!(config.ws_url(), format!("ws://localhost:{WS_PORT}"));
        assert_eq!(config.grpc_endpoint(), format!("localhost:{GRPC_PORT}"));
    }
}
