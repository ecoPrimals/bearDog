// SPDX-License-Identifier: AGPL-3.0-or-later

//! Concurrent-Safe Network Configuration Module
//!
//! Network configuration for API, service discovery, and admin interfaces.
//!
//! ## Design Pattern: Explicit Environment Loading
//!
//! To ensure **concurrent safety** and **testability**, this module separates:
//! - **Static defaults** (`Default` trait) - Pure, no environment reads
//! - **Environment loading** (`from_env()`) - Explicit environment variable reads
//! - **Flexible construction** (`builder()`) - Testing without env var pollution

use crate::domains::network_addresses::NetworkAddressesConfig;
#[cfg(test)]
use crate::domains::network_ports::DEFAULT_ADMIN_PORT;
use crate::domains::network_ports::{DEFAULT_API_PORT, DEFAULT_DISCOVERY_PORT, NetworkPortsConfig};
use crate::env_keys;
use crate::error::{ConfigError, ConfigResult};
use serde::{Deserialize, Serialize};
use std::net::{IpAddr, Ipv4Addr};

/// Network configuration
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
pub struct NetworkConfig {
    /// Centralized port configuration (preferred)
    ///
    /// Use this for all new code. Individual component ports are maintained
    /// for backward compatibility during migration.
    #[serde(default)]
    pub ports: NetworkPortsConfig,

    /// Centralized address configuration (preferred)
    ///
    /// Use this for all new code. Replaces hardcoded IP addresses and hostnames.
    #[serde(default)]
    pub addresses: NetworkAddressesConfig,

    /// API server configuration
    #[serde(default)]
    pub api: ApiConfig,

    /// Service discovery configuration
    #[serde(default)]
    pub discovery: ServiceDiscoveryConfig,

    /// Admin interface configuration
    #[serde(default)]
    pub admin: AdminConfig,
}

impl NetworkConfig {
    /// Pure static defaults (no environment variable reads)
    #[must_use]
    pub fn const_defaults() -> Self {
        Self {
            ports: NetworkPortsConfig::default(),
            addresses: NetworkAddressesConfig::default(),
            api: ApiConfig::const_defaults(),
            discovery: ServiceDiscoveryConfig::const_defaults(),
            admin: AdminConfig::const_defaults(),
        }
    }

    /// Load configuration from environment variables with fallback to defaults
    #[must_use]
    pub fn from_env() -> Self {
        Self {
            ports: NetworkPortsConfig::from_env(),
            addresses: NetworkAddressesConfig::from_env(),
            api: ApiConfig::from_env(),
            discovery: ServiceDiscoveryConfig::from_env(),
            admin: AdminConfig::from_env(),
        }
    }

    /// Validate network configuration
    ///
    /// # Errors
    ///
    /// Returns [`ConfigError`] when nested port, address, API, discovery, or admin validation fails.
    pub fn validate(&self) -> ConfigResult<()> {
        self.ports.validate()?;
        self.addresses.validate()?;

        self.api.validate()?;
        self.discovery.validate()?;
        self.admin.validate()?;

        // Check for port conflicts
        if self.api.port == self.discovery.port {
            return Err(ConfigError::port_conflict(
                "API and discovery ports must be different",
            ));
        }
        if self.api.port == self.admin.port {
            return Err(ConfigError::port_conflict(
                "API and admin ports must be different",
            ));
        }
        if self.discovery.port == self.admin.port {
            return Err(ConfigError::port_conflict(
                "Discovery and admin ports must be different",
            ));
        }

        Ok(())
    }
}

impl Default for NetworkConfig {
    fn default() -> Self {
        Self::const_defaults()
    }
}

/// API server configuration
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
pub struct ApiConfig {
    /// Bind address (default: 127.0.0.1 for security)
    pub bind_address: IpAddr,

    /// API port
    pub port: u16,

    /// Enable TLS
    pub tls_enabled: bool,

    /// TLS certificate path (required if TLS enabled)
    #[serde(skip_serializing_if = "Option::is_none")]
    pub tls_cert_path: Option<String>,

    /// TLS key path (required if TLS enabled)
    #[serde(skip_serializing_if = "Option::is_none")]
    pub tls_key_path: Option<String>,

    /// Maximum concurrent connections
    pub max_connections: usize,
}

impl ApiConfig {
    /// Pure static defaults (no environment variable reads)
    #[must_use]
    pub const fn const_defaults() -> Self {
        Self {
            bind_address: IpAddr::V4(Ipv4Addr::LOCALHOST),
            port: DEFAULT_API_PORT,
            tls_enabled: false,
            tls_cert_path: None,
            tls_key_path: None,
            max_connections: 100,
        }
    }

    /// Load configuration from environment variables with fallback to defaults
    #[must_use]
    pub fn from_env() -> Self {
        let defaults = Self::const_defaults();

        Self {
            bind_address: std::env::var(env_keys::ENV_API_BIND_ADDRESS)
                .ok()
                .and_then(|s| s.parse().ok())
                .unwrap_or(defaults.bind_address),

            port: std::env::var(env_keys::ENV_API_PORT)
                .ok()
                .and_then(|s| s.parse().ok())
                .unwrap_or(defaults.port),

            tls_enabled: defaults.tls_enabled,
            tls_cert_path: None,
            tls_key_path: None,

            max_connections: std::env::var(env_keys::ENV_API_MAX_CONNECTIONS)
                .ok()
                .and_then(|s| s.parse().ok())
                .unwrap_or(defaults.max_connections),
        }
    }

    /// Create a builder for flexible configuration construction
    #[must_use]
    pub fn builder() -> ApiConfigBuilder {
        ApiConfigBuilder::new()
    }

    /// Validate API configuration
    ///
    /// # Errors
    ///
    /// Returns [`ConfigError`] when the port is invalid or TLS settings are inconsistent.
    pub fn validate(&self) -> ConfigResult<()> {
        if self.port == 0 {
            return Err(ConfigError::invalid_value(
                "network.api.port",
                "Port cannot be 0. Please specify a valid port (1-65535)",
            ));
        }

        if self.tls_enabled && (self.tls_cert_path.is_none() || self.tls_key_path.is_none()) {
            return Err(ConfigError::validation(
                "TLS is enabled but certificate or key path is missing",
            ));
        }

        if self.max_connections == 0 {
            return Err(ConfigError::invalid_value(
                "network.api.max_connections",
                "Must be greater than 0",
            ));
        }

        Ok(())
    }
}

impl Default for ApiConfig {
    fn default() -> Self {
        Self::const_defaults()
    }
}

/// Builder for `ApiConfig`
#[derive(Debug, Default)]
pub struct ApiConfigBuilder {
    bind_address: Option<IpAddr>,
    port: Option<u16>,
    tls_enabled: Option<bool>,
    tls_cert_path: Option<String>,
    tls_key_path: Option<String>,
    max_connections: Option<usize>,
}

impl ApiConfigBuilder {
    /// Starts a builder; unset fields use [`ApiConfig::const_defaults`].
    #[must_use]
    pub fn new() -> Self {
        Self::default()
    }

    /// Address the public API listener binds to.
    #[must_use]
    pub const fn bind_address(mut self, addr: IpAddr) -> Self {
        self.bind_address = Some(addr);
        self
    }

    /// TCP port for the primary API server.
    #[must_use]
    pub const fn port(mut self, port: u16) -> Self {
        self.port = Some(port);
        self
    }

    /// Whether TLS terminates on this listener (vs offloaded upstream).
    #[must_use]
    pub const fn tls_enabled(mut self, enabled: bool) -> Self {
        self.tls_enabled = Some(enabled);
        self
    }

    /// Filesystem path to the leaf certificate presented when TLS is enabled.
    #[must_use]
    pub fn tls_cert_path(mut self, path: String) -> Self {
        self.tls_cert_path = Some(path);
        self
    }

    /// Filesystem path to the private key for `tls_cert_path`.
    #[must_use]
    pub fn tls_key_path(mut self, path: String) -> Self {
        self.tls_key_path = Some(path);
        self
    }

    /// Upper bound on concurrent accepted connections for the API socket.
    #[must_use]
    pub const fn max_connections(mut self, max: usize) -> Self {
        self.max_connections = Some(max);
        self
    }

    /// Produces an [`ApiConfig`] with defaults filling any omitted settings.
    #[must_use]
    pub fn build(self) -> ApiConfig {
        let defaults = ApiConfig::const_defaults();

        ApiConfig {
            bind_address: self.bind_address.unwrap_or(defaults.bind_address),
            port: self.port.unwrap_or(defaults.port),
            tls_enabled: self.tls_enabled.unwrap_or(defaults.tls_enabled),
            tls_cert_path: self.tls_cert_path.or(defaults.tls_cert_path),
            tls_key_path: self.tls_key_path.or(defaults.tls_key_path),
            max_connections: self.max_connections.unwrap_or(defaults.max_connections),
        }
    }
}

/// Service discovery configuration
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
pub struct ServiceDiscoveryConfig {
    /// Discovery port
    pub port: u16,

    /// Discovery backends to use
    pub backends: Vec<String>,

    /// Multicast address for mDNS
    pub multicast_address: String,

    /// Discovery interval in seconds
    pub interval_secs: u64,
}

impl ServiceDiscoveryConfig {
    /// Pure static defaults (no environment variable reads)
    #[must_use]
    pub fn const_defaults() -> Self {
        Self {
            port: DEFAULT_DISCOVERY_PORT,
            backends: vec!["dns-sd".to_string(), "static".to_string()],
            multicast_address: format!("239.255.0.1:{DEFAULT_DISCOVERY_PORT}"),
            interval_secs: 60,
        }
    }

    /// Load configuration from environment variables with fallback to defaults
    #[must_use]
    pub fn from_env() -> Self {
        let defaults = Self::const_defaults();

        Self {
            port: std::env::var(env_keys::ENV_DISCOVERY_PORT)
                .ok()
                .and_then(|s| s.parse().ok())
                .unwrap_or(defaults.port),

            backends: defaults.backends,

            multicast_address: std::env::var(env_keys::ENV_MULTICAST_ADDRESS)
                .ok()
                .unwrap_or(defaults.multicast_address),

            interval_secs: std::env::var(env_keys::ENV_DISCOVERY_INTERVAL_SECS)
                .ok()
                .and_then(|s| s.parse().ok())
                .unwrap_or(defaults.interval_secs),
        }
    }

    /// Create a builder for flexible configuration construction
    #[must_use]
    pub fn builder() -> ServiceDiscoveryConfigBuilder {
        ServiceDiscoveryConfigBuilder::new()
    }

    /// Validate discovery configuration
    ///
    /// # Errors
    ///
    /// Returns [`ConfigError`] when discovery port, backends, or timing options are invalid.
    pub fn validate(&self) -> ConfigResult<()> {
        if self.port == 0 {
            return Err(ConfigError::invalid_value(
                "network.discovery.port",
                "Port cannot be 0",
            ));
        }

        if self.backends.is_empty() {
            return Err(ConfigError::validation(
                "At least one discovery backend must be specified",
            ));
        }

        if self.interval_secs == 0 {
            return Err(ConfigError::invalid_value(
                "network.discovery.interval_secs",
                "Interval must be greater than 0",
            ));
        }

        Ok(())
    }
}

impl Default for ServiceDiscoveryConfig {
    fn default() -> Self {
        Self::const_defaults()
    }
}

/// Builder for `ServiceDiscoveryConfig`
#[derive(Debug, Default)]
pub struct ServiceDiscoveryConfigBuilder {
    port: Option<u16>,
    backends: Option<Vec<String>>,
    multicast_address: Option<String>,
    interval_secs: Option<u64>,
}

impl ServiceDiscoveryConfigBuilder {
    /// Starts a builder backed by [`ServiceDiscoveryConfig::const_defaults`].
    #[must_use]
    pub fn new() -> Self {
        Self::default()
    }

    /// UDP/TCP port used for discovery traffic (mDNS, etc.).
    #[must_use]
    pub const fn port(mut self, port: u16) -> Self {
        self.port = Some(port);
        self
    }

    /// Ordered list of discovery backend identifiers (e.g. `dns-sd`, `static`).
    #[must_use]
    pub fn backends(mut self, backends: Vec<String>) -> Self {
        self.backends = Some(backends);
        self
    }

    /// Multicast group and port string for mDNS or similar (e.g. `239.255.0.1:5353`).
    #[must_use]
    pub fn multicast_address(mut self, addr: String) -> Self {
        self.multicast_address = Some(addr);
        self
    }

    /// Seconds between periodic re-announcement or refresh cycles.
    #[must_use]
    pub const fn interval_secs(mut self, secs: u64) -> Self {
        self.interval_secs = Some(secs);
        self
    }

    /// Builds [`ServiceDiscoveryConfig`] with static defaults for unset fields.
    #[must_use]
    pub fn build(self) -> ServiceDiscoveryConfig {
        let defaults = ServiceDiscoveryConfig::const_defaults();

        ServiceDiscoveryConfig {
            port: self.port.unwrap_or(defaults.port),
            backends: self.backends.unwrap_or(defaults.backends),
            multicast_address: self.multicast_address.unwrap_or(defaults.multicast_address),
            interval_secs: self.interval_secs.unwrap_or(defaults.interval_secs),
        }
    }
}

/// Admin interface configuration
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
pub struct AdminConfig {
    /// Admin interface bind address
    pub bind_address: IpAddr,

    /// Admin port
    pub port: u16,

    /// Enable admin interface
    pub enabled: bool,
}

impl AdminConfig {
    /// Pure static defaults (no environment variable reads)
    #[must_use]
    pub const fn const_defaults() -> Self {
        Self {
            bind_address: IpAddr::V4(Ipv4Addr::LOCALHOST),
            port: 9091,
            enabled: true,
        }
    }

    /// Load configuration from environment variables with fallback to defaults
    #[must_use]
    pub fn from_env() -> Self {
        let defaults = Self::const_defaults();

        Self {
            bind_address: std::env::var(env_keys::ENV_ADMIN_BIND_ADDRESS)
                .ok()
                .and_then(|s| s.parse().ok())
                .unwrap_or(defaults.bind_address),

            port: std::env::var(env_keys::ENV_ADMIN_PORT)
                .ok()
                .and_then(|s| s.parse().ok())
                .unwrap_or(defaults.port),

            enabled: std::env::var(env_keys::ENV_ADMIN_ENABLED)
                .ok()
                .and_then(|s| s.parse().ok())
                .unwrap_or(defaults.enabled),
        }
    }

    /// Create a builder for flexible configuration construction
    #[must_use]
    pub fn builder() -> AdminConfigBuilder {
        AdminConfigBuilder::new()
    }

    /// Validate admin configuration
    ///
    /// # Errors
    ///
    /// Returns [`ConfigError`] when the admin port is invalid for the enabled state.
    pub fn validate(&self) -> ConfigResult<()> {
        if self.enabled && self.port == 0 {
            return Err(ConfigError::invalid_value(
                "network.admin.port",
                "Port cannot be 0 when admin interface is enabled",
            ));
        }

        Ok(())
    }
}

impl Default for AdminConfig {
    fn default() -> Self {
        Self::const_defaults()
    }
}

/// Builder for `AdminConfig`
#[derive(Debug, Default)]
pub struct AdminConfigBuilder {
    bind_address: Option<IpAddr>,
    port: Option<u16>,
    enabled: Option<bool>,
}

impl AdminConfigBuilder {
    /// Starts a builder using [`AdminConfig::const_defaults`] for gaps.
    #[must_use]
    pub fn new() -> Self {
        Self::default()
    }

    /// Bind address for the administrative HTTP or control plane interface.
    #[must_use]
    pub const fn bind_address(mut self, addr: IpAddr) -> Self {
        self.bind_address = Some(addr);
        self
    }

    /// TCP port for the admin interface when enabled.
    #[must_use]
    pub const fn port(mut self, port: u16) -> Self {
        self.port = Some(port);
        self
    }

    /// Master switch for exposing the admin listener and routes.
    #[must_use]
    pub const fn enabled(mut self, enabled: bool) -> Self {
        self.enabled = Some(enabled);
        self
    }

    /// Materializes [`AdminConfig`].
    #[must_use]
    pub fn build(self) -> AdminConfig {
        let defaults = AdminConfig::const_defaults();

        AdminConfig {
            bind_address: self.bind_address.unwrap_or(defaults.bind_address),
            port: self.port.unwrap_or(defaults.port),
            enabled: self.enabled.unwrap_or(defaults.enabled),
        }
    }
}

#[cfg(test)]
#[path = "network_coverage_extension.rs"]
mod network_coverage_extension;

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_const_defaults() {
        let config = NetworkConfig::const_defaults();
        assert_eq!(config.api.port, DEFAULT_API_PORT);
        assert_eq!(config.discovery.port, DEFAULT_DISCOVERY_PORT);
        assert_eq!(config.admin.port, DEFAULT_ADMIN_PORT);
    }

    #[test]
    fn test_default_uses_const_defaults() {
        let default_config = NetworkConfig::default();
        let const_config = NetworkConfig::const_defaults();
        assert_eq!(default_config, const_config);
    }

    #[test]
    fn test_default_network_config() {
        let config = NetworkConfig::default();
        assert!(config.validate().is_ok());
    }

    #[test]
    fn test_port_conflict_detection() {
        let mut config = NetworkConfig::default();
        config.api.port = DEFAULT_API_PORT;
        config.discovery.port = DEFAULT_API_PORT; // Conflict!

        assert!(config.validate().is_err());
    }

    #[test]
    fn test_api_builder() {
        let config = ApiConfig::builder().port(9000).max_connections(200).build();

        assert_eq!(config.port, 9000);
        assert_eq!(config.max_connections, 200);
        assert_eq!(config.bind_address, IpAddr::V4(Ipv4Addr::LOCALHOST));
    }

    #[test]
    fn test_invalid_port_zero() {
        let config = ApiConfig::builder().port(0).build();

        assert!(config.validate().is_err());
    }

    #[test]
    fn test_tls_without_cert() {
        let config = ApiConfig::builder().tls_enabled(true).build();

        assert!(config.validate().is_err());
    }

    #[test]
    fn test_tls_with_cert_and_key() {
        let config = ApiConfig::builder()
            .tls_enabled(true)
            .tls_cert_path("/path/to/cert.pem".to_string())
            .tls_key_path("/path/to/key.pem".to_string())
            .build();

        assert!(config.validate().is_ok());
    }

    #[test]
    fn test_discovery_builder() {
        let config = ServiceDiscoveryConfig::builder()
            .port(9999)
            .interval_secs(30)
            .build();

        assert_eq!(config.port, 9999);
        assert_eq!(config.interval_secs, 30);
    }

    #[test]
    fn test_admin_builder() {
        let config = AdminConfig::builder().port(7777).enabled(false).build();

        assert_eq!(config.port, 7777);
        assert!(!config.enabled);
    }
}
