// SPDX-License-Identifier: AGPL-3.0-or-later

//! Network Port Configuration
//!
//! Centralized configuration for all network ports used by `BearDog`.
//! This eliminates hardcoded port numbers throughout the codebase.
//!
//! # Design Philosophy
//!
//! - **Configuration over Hardcoding**: All ports configurable via ENV or config file
//! - **Secure Defaults**: Non-privileged ports (>1024) with documented fallbacks
//! - **Environment-First**: `BEARDOG_*` environment variables take precedence
//! - **Platform Agnostic**: Works across development, staging, and production
//!
//! # Usage
//!
//! ```no_run
//! use beardog_config::global::BEARDOG_CONFIG;
//!
//! // Access ports from global config
//! let api_port = BEARDOG_CONFIG.network.ports.api_port;
//! let discovery_port = BEARDOG_CONFIG.network.ports.discovery_port;
//! ```
//!
//! # Environment Variables
//!
//! - `BEARDOG_API_PORT` - API server port (default: 8080)
//! - `BEARDOG_DISCOVERY_PORT` - Discovery service port (default: 9090)
//! - `BEARDOG_ADMIN_PORT` - Admin endpoint port (default: 9091)
//! - `BEARDOG_HTTPS_PORT` - HTTPS API port (default: 8443)
//! - `BEARDOG_METRICS_PORT` - Metrics/monitoring port (default: 9100)
//! - `BEARDOG_HEALTH_PORT` - Health check port (default: 8081)
//! - `BEARDOG_TCP_IPC_PORT` - TCP IPC fallback when Unix sockets unavailable (default: 9900)
//! - `BEARDOG_PROFILING_PORT` - Profiling / diagnostic HTTP (default: 6060; see `DEFAULT_PROFILING_PORT`)

use serde::{Deserialize, Serialize};

/// Network ports configuration
///
/// All network ports used by `BearDog` components. Configurable via environment
/// variables or config file, with secure defaults as fallback.
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
pub struct NetworkPortsConfig {
    /// API server port
    ///
    /// Main HTTP API endpoint port. Defaults to 8080.
    /// Override with `BEARDOG_API_PORT` environment variable.
    #[serde(default = "default_api_port")]
    pub api_port: u16,

    /// Discovery service port
    ///
    /// Service discovery and registration port. Defaults to 9090.
    /// Override with `BEARDOG_DISCOVERY_PORT` environment variable.
    #[serde(default = "default_discovery_port")]
    pub discovery_port: u16,

    /// Admin endpoint port
    ///
    /// Administrative operations port. Defaults to 9091.
    /// Override with `BEARDOG_ADMIN_PORT` environment variable.
    #[serde(default = "default_admin_port")]
    pub admin_port: u16,

    /// HTTPS API port
    ///
    /// Secure HTTP API endpoint. Defaults to 8443.
    /// Override with `BEARDOG_HTTPS_PORT` environment variable.
    #[serde(default = "default_https_port")]
    pub https_port: u16,

    /// Metrics/monitoring port
    ///
    /// Prometheus metrics and monitoring. Defaults to 9100.
    /// Override with `BEARDOG_METRICS_PORT` environment variable.
    #[serde(default = "default_metrics_port")]
    pub metrics_port: u16,

    /// Health check port
    ///
    /// Health and liveness probes. Defaults to 8081.
    /// Override with `BEARDOG_HEALTH_PORT` environment variable.
    #[serde(default = "default_health_port")]
    pub health_port: u16,

    /// TCP IPC fallback port
    ///
    /// Used when Unix sockets aren't available (Android, Windows containers).
    /// This is the TCP endpoint for JSON-RPC IPC. Defaults to 9900.
    /// Override with `BEARDOG_TCP_IPC_PORT` environment variable.
    #[serde(default = "default_tcp_ipc_port")]
    pub tcp_ipc_port: u16,
}

// Default port constants (documented fallbacks)
// These are ONLY used when no environment variable or config file value is provided

/// Default API server port (8080)
pub const DEFAULT_API_PORT: u16 = 8080;

/// Decimal string form of [`DEFAULT_API_PORT`] for URL construction (`concat!`, logging, etc.).
///
/// Must parse to the same value as [`DEFAULT_API_PORT`]; used to avoid embedding port literals in strings.
pub const DEFAULT_API_PORT_STR: &str = "8080";

/// Default UPA-style HTTPS base URL when `BEARDOG_UPA_URL` / related env vars are unset.
///
/// Semantically `https://{DEFAULT_EXTERNAL_HOST}:{DEFAULT_API_PORT}`.
/// Built with `concat!` (string literals only); unit tests assert it matches
/// `https://{DEFAULT_EXTERNAL_HOST}:{DEFAULT_API_PORT}`.
pub const DEFAULT_UPA_FALLBACK_BASE_URL: &str = concat!("https://", "localhost", ":", "8080");

/// Default discovery service port (9090)
pub const DEFAULT_DISCOVERY_PORT: u16 = 9090;

/// Default admin endpoint port (9091)
pub const DEFAULT_ADMIN_PORT: u16 = 9091;

/// Default debug/diagnostics port (9092)
/// By convention, `admin_port` + 1 for diagnostic endpoints
pub const DEFAULT_DEBUG_PORT: u16 = 9092;

/// Default HTTPS API port (8443)
pub const DEFAULT_HTTPS_PORT: u16 = 8443;

/// Default metrics port (9100 - Prometheus standard)
pub const DEFAULT_METRICS_PORT: u16 = 9100;

/// Default health check port (8081)
pub const DEFAULT_HEALTH_PORT: u16 = 8081;

/// Default Vault server port (8200 - Vault standard)
pub const DEFAULT_VAULT_PORT: u16 = 8200;

/// Default WebSocket port (8082)
pub const DEFAULT_WEBSOCKET_PORT: u16 = 8082;

/// Default compute service port (8001)
pub const DEFAULT_COMPUTE_PORT: u16 = 8001;

/// Default service mesh port (8002)
pub const DEFAULT_MESH_PORT: u16 = 8002;

/// Default AI service port (8003)
pub const DEFAULT_AI_PORT: u16 = 8003;

/// Default storage service port (8004)
pub const DEFAULT_STORAGE_PORT: u16 = 8004;

/// Default security service port (8005)
pub const DEFAULT_SECURITY_PORT: u16 = 8005;

/// Default database port (`PostgreSQL`: 5432)
pub const DEFAULT_DATABASE_PORT: u16 = 5432;

/// Default Grafana port (3000)
pub const DEFAULT_GRAFANA_PORT: u16 = 3000;

/// Default integration API server listen port (9000)
/// REST endpoints for cross-primal federation. Override with `BEARDOG_API_PORT`.
pub const DEFAULT_INTEGRATION_API_PORT: u16 = 9000;

/// Default TCP IPC fallback port (9900)
/// Used when Unix sockets aren't available (Android, containers, Windows)
pub const DEFAULT_TCP_IPC_PORT: u16 = 9900;

/// Default Jaeger port (14268)
pub const DEFAULT_JAEGER_PORT: u16 = 14268;

/// Default profiling / diagnostic HTTP port (6060, common pprof convention)
pub const DEFAULT_PROFILING_PORT: u16 = 6060;

/// Default gRPC server port (common convention when not using ALPN on 443)
pub const DEFAULT_GRPC_PORT: u16 = 50051;

/// Default Consul HTTP API port (`HashiCorp` standard)
pub const DEFAULT_CONSUL_PORT: u16 = 8500;

/// Default Redis server port (IANA assigned)
pub const DEFAULT_REDIS_PORT: u16 = 6379;

fn default_api_port() -> u16 {
    DEFAULT_API_PORT
}

fn default_discovery_port() -> u16 {
    DEFAULT_DISCOVERY_PORT
}

fn default_admin_port() -> u16 {
    DEFAULT_ADMIN_PORT
}

fn default_https_port() -> u16 {
    DEFAULT_HTTPS_PORT
}

fn default_metrics_port() -> u16 {
    DEFAULT_METRICS_PORT
}

fn default_health_port() -> u16 {
    DEFAULT_HEALTH_PORT
}

fn default_tcp_ipc_port() -> u16 {
    DEFAULT_TCP_IPC_PORT
}

impl Default for NetworkPortsConfig {
    fn default() -> Self {
        Self::with_defaults()
    }
}

impl NetworkPortsConfig {
    /// Creates configuration with documented static defaults (no environment reads).
    ///
    /// All defaults are non-privileged ports (>1024). For environment overrides, use
    /// [`Self::from_env`].
    ///
    /// # Example
    ///
    /// ```rust
    /// use beardog_config::NetworkPortsConfig;
    ///
    /// let ports = NetworkPortsConfig::with_defaults();
    /// assert_eq!(ports.api_port, 8080);
    /// ```
    #[must_use]
    pub fn with_defaults() -> Self {
        Self {
            api_port: default_api_port(),
            discovery_port: default_discovery_port(),
            admin_port: default_admin_port(),
            https_port: default_https_port(),
            metrics_port: default_metrics_port(),
            health_port: default_health_port(),
            tcp_ipc_port: default_tcp_ipc_port(),
        }
    }

    /// Loads configuration from environment variables
    ///
    /// Reads `BEARDOG_*` port variables when set; otherwise matches [`Self::with_defaults`].
    ///
    /// # Example
    ///
    /// ```no_run
    /// use beardog_config::NetworkPortsConfig;
    ///
    /// let ports = NetworkPortsConfig::from_env();
    /// // Reads BEARDOG_API_PORT, BEARDOG_DISCOVERY_PORT, etc.
    /// ```
    #[must_use]
    pub fn from_env() -> Self {
        Self {
            api_port: std::env::var("BEARDOG_API_PORT")
                .ok()
                .and_then(|v| v.parse().ok())
                .unwrap_or(DEFAULT_API_PORT),
            discovery_port: std::env::var("BEARDOG_DISCOVERY_PORT")
                .ok()
                .and_then(|v| v.parse().ok())
                .unwrap_or(DEFAULT_DISCOVERY_PORT),
            admin_port: std::env::var("BEARDOG_ADMIN_PORT")
                .ok()
                .and_then(|v| v.parse().ok())
                .unwrap_or(DEFAULT_ADMIN_PORT),
            https_port: std::env::var("BEARDOG_HTTPS_PORT")
                .ok()
                .and_then(|v| v.parse().ok())
                .unwrap_or(DEFAULT_HTTPS_PORT),
            metrics_port: std::env::var("BEARDOG_METRICS_PORT")
                .ok()
                .and_then(|v| v.parse().ok())
                .unwrap_or(DEFAULT_METRICS_PORT),
            health_port: std::env::var("BEARDOG_HEALTH_PORT")
                .ok()
                .and_then(|v| v.parse().ok())
                .unwrap_or(DEFAULT_HEALTH_PORT),
            tcp_ipc_port: std::env::var("BEARDOG_TCP_IPC_PORT")
                .ok()
                .and_then(|v| v.parse().ok())
                .unwrap_or(DEFAULT_TCP_IPC_PORT),
        }
    }

    /// Validates that all ports are in valid range
    ///
    /// Ensures ports are non-privileged (>1024) and below max (65535).
    ///
    /// # Errors
    ///
    /// Returns `Err` when any port is `0`, privileged, or conflicts with another configured port.
    pub fn validate(&self) -> Result<(), String> {
        let ports = [
            ("api_port", self.api_port),
            ("discovery_port", self.discovery_port),
            ("admin_port", self.admin_port),
            ("https_port", self.https_port),
            ("metrics_port", self.metrics_port),
            ("health_port", self.health_port),
            ("tcp_ipc_port", self.tcp_ipc_port),
        ];

        for (name, port) in ports {
            if port < 1024 {
                return Err(format!("{name} ({port}) should be non-privileged (>1024)"));
            }
            if port == 0 {
                return Err(format!("{name} cannot be 0"));
            }
        }

        // Check for port conflicts
        let port_values: Vec<u16> = ports.iter().map(|(_, p)| *p).collect();
        for (i, &port1) in port_values.iter().enumerate() {
            for &port2 in port_values.iter().skip(i + 1) {
                if port1 == port2 {
                    return Err(format!("Port conflict detected: {port1}"));
                }
            }
        }

        Ok(())
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn default_api_port_str_matches_numeric() {
        assert_eq!(
            DEFAULT_API_PORT_STR
                .parse::<u16>()
                .expect("port str parses"),
            DEFAULT_API_PORT
        );
    }

    #[test]
    fn default_upa_fallback_matches_external_host_and_api_port() {
        use crate::domains::network_addresses::DEFAULT_EXTERNAL_HOST;
        assert_eq!(
            DEFAULT_UPA_FALLBACK_BASE_URL,
            format!("https://{DEFAULT_EXTERNAL_HOST}:{DEFAULT_API_PORT}")
        );
    }

    #[test]
    fn test_default_ports() {
        let config = NetworkPortsConfig::with_defaults();

        // Should use documented defaults
        assert_eq!(config.api_port, DEFAULT_API_PORT);
        assert_eq!(config.discovery_port, DEFAULT_DISCOVERY_PORT);
        assert_eq!(config.admin_port, DEFAULT_ADMIN_PORT);
        assert_eq!(config.https_port, DEFAULT_HTTPS_PORT);
        assert_eq!(config.metrics_port, DEFAULT_METRICS_PORT);
        assert_eq!(config.health_port, DEFAULT_HEALTH_PORT);
    }

    #[test]
    fn test_ports_are_non_privileged() {
        let config = NetworkPortsConfig::with_defaults();

        assert!(config.api_port > 1024, "API port should be non-privileged");
        assert!(
            config.discovery_port > 1024,
            "Discovery port should be non-privileged"
        );
        assert!(
            config.admin_port > 1024,
            "Admin port should be non-privileged"
        );
        assert!(
            config.https_port > 1024,
            "HTTPS port should be non-privileged"
        );
        assert!(
            config.metrics_port > 1024,
            "Metrics port should be non-privileged"
        );
        assert!(
            config.health_port > 1024,
            "Health port should be non-privileged"
        );
    }

    #[test]
    fn test_ports_are_valid_range() {
        let config = NetworkPortsConfig::with_defaults();

        assert!(config.api_port < 65535, "API port should be valid");
        assert!(
            config.discovery_port < 65535,
            "Discovery port should be valid"
        );
        assert!(config.admin_port < 65535, "Admin port should be valid");
        assert!(config.https_port < 65535, "HTTPS port should be valid");
        assert!(config.metrics_port < 65535, "Metrics port should be valid");
        assert!(config.health_port < 65535, "Health port should be valid");
    }

    #[test]
    fn test_validation_passes_for_defaults() {
        let config = NetworkPortsConfig::with_defaults();
        assert!(config.validate().is_ok());
    }

    #[test]
    fn test_no_port_conflicts_in_defaults() {
        let config = NetworkPortsConfig::with_defaults();

        // Ensure all default ports are unique
        let ports = [
            config.api_port,
            config.discovery_port,
            config.admin_port,
            config.https_port,
            config.metrics_port,
            config.health_port,
        ];

        let unique_ports: std::collections::HashSet<_> = ports.iter().collect();
        assert_eq!(
            unique_ports.len(),
            ports.len(),
            "All default ports should be unique"
        );
    }

    #[test]
    fn test_serialization() {
        let config = NetworkPortsConfig::with_defaults();

        let json = serde_json::to_string(&config).expect("Should serialize");
        let deserialized: NetworkPortsConfig =
            serde_json::from_str(&json).expect("Should deserialize");

        assert_eq!(config, deserialized);
    }

    #[test]
    fn test_from_env_equals_default() {
        // When no env vars set, from_env should equal with_defaults
        let from_env = NetworkPortsConfig::from_env();
        let with_defaults = NetworkPortsConfig::with_defaults();

        assert_eq!(from_env, with_defaults);
    }

    #[test]
    fn test_clone() {
        let config = NetworkPortsConfig::with_defaults();
        let cloned = config.clone();

        assert_eq!(config, cloned);
    }

    #[test]
    fn test_debug_format() {
        let config = NetworkPortsConfig::with_defaults();
        let debug_str = format!("{config:?}");

        assert!(debug_str.contains("NetworkPortsConfig"));
        assert!(debug_str.contains("api_port"));
    }
}
