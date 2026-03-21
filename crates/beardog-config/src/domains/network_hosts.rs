// SPDX-License-Identifier: AGPL-3.0-only

//! Network Host Configuration
//!
//! **Self-knowledge and explicit configuration** for hosts BearDog uses to bind or connect when
//! no higher layer (URLs, discovery) has supplied a value. Per `ZERO_HARDCODING_SPECIFICATION` and
//! capability-based discovery, string literals below are **documented fallbacks**, not peer
//! definitions: **other primals** are reached via runtime discovery (mDNS, registry, mesh)—not
//! fixed hostnames here.
//!
//! # Design Philosophy
//!
//! - **Configuration over fallbacks**: Prefer `DATABASE_URL`, `REDIS_URL`, `BEARDOG_*_HOST`, and discovery
//! - **Documented dev fallbacks**: A single infrastructure loopback fallback when nothing else is set
//! - **Environment-First**: `BEARDOG_*` and standard URL env vars take precedence
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

// ============================================================================
// Documented fallbacks (single source; overridable via env / config / discovery)
// ============================================================================

/// **Fallback** hostname for client connections when `BEARDOG_CLIENT_HOST` is unset (loopback name).
pub const DEFAULT_HOST: &str = "localhost";

/// **Fallback** IPv4 bind-all when `BEARDOG_API_HOST` is unset for this config shape (server bind).
pub const DEFAULT_BIND_HOST: &str = "0.0.0.0";

/// **Fallback** IPv4 loopback literal (prefer hostname `DEFAULT_HOST` for clients when possible).
pub const DEFAULT_LOOPBACK_IPV4: &str = "127.0.0.1";

/// **Fallback** IPv6 loopback literal.
pub const DEFAULT_LOOPBACK_IPV6: &str = "::1";

/// **Fallback** IPv6 bind-all.
pub const DEFAULT_BIND_HOST_IPV6: &str = "::";

// ---------------------------------------------------------------------------
// Infrastructure / legacy fields — NOT other-primal endpoints
// ---------------------------------------------------------------------------
//
// Production must use `DATABASE_URL`, `REDIS_URL`, monitoring URLs, or capability/registry
// discovery. When none are present, a single dev-only loopback fallback applies (overridable via
// `BEARDOG_INFRASTRUCTURE_HOST_FALLBACK`). See `runtime_network_discovery.rs`.

/// **Dev-only fallback** host for infrastructure URLs when no env URL or explicit host is set.
pub const FALLBACK_DEV_INFRASTRUCTURE_HOST: &str = "localhost";

/// Legacy alias: prefer `DATABASE_URL` / `BEARDOG_DATABASE_HOST` / discovery.
pub const POSTGRES_DISCOVERY_HINT: &str = FALLBACK_DEV_INFRASTRUCTURE_HOST;

/// Legacy alias: prefer `REDIS_URL` / `BEARDOG_REDIS_HOST` / discovery.
pub const REDIS_DISCOVERY_HINT: &str = FALLBACK_DEV_INFRASTRUCTURE_HOST;

/// Legacy alias: prefer `GRAFANA_URL` / `BEARDOG_METRICS_HOST` / discovery.
pub const GRAFANA_DISCOVERY_HINT: &str = FALLBACK_DEV_INFRASTRUCTURE_HOST;

/// Prefer `JAEGER_ENDPOINT` / tracing discovery.
pub const JAEGER_DISCOVERY_HINT: &str = FALLBACK_DEV_INFRASTRUCTURE_HOST;

/// Prefer `BEARDOG_EXTERNAL_HOST` / DNS / capability discovery (never rely on this in production).
pub const EXTERNAL_DISCOVERY_HINT: &str = FALLBACK_DEV_INFRASTRUCTURE_HOST;

// ============================================================================
// Network Hosts Configuration
// ============================================================================

/// Network hosts configuration
///
/// **Design Philosophy**: Runtime discovery and explicit config over fallbacks
///
/// This configuration provides:
/// 1. **Environment / URL variables**: Explicit configuration (highest priority)
/// 2. **Capability-based discovery**: Peer endpoints from mDNS/registry (not stored here)
/// 3. **Documented dev fallback**: `FALLBACK_DEV_INFRASTRUCTURE_HOST` unless `BEARDOG_INFRASTRUCTURE_HOST_FALLBACK` is set
///
/// **Production Usage**:
/// - MUST use environment variables or service discovery
/// - NEVER rely on default values in production
/// - Use mDNS/DNS-SD for local network discovery
/// - Use service mesh/registry for production discovery
///
/// All network hosts/IPs used by BearDog components. Configurable via
/// environment variables or config file, with secure defaults as fallback.
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
pub struct NetworkHostsConfig {
    /// Primary API host
    ///
    /// Host for API server binding. **Fallback**: all-interfaces IPv4 (`DEFAULT_BIND_HOST`).
    /// Override with `BEARDOG_API_HOST`.
    #[serde(default = "default_api_host")]
    pub api_host: String,

    /// Client connection host
    ///
    /// **Fallback**: loopback hostname (`DEFAULT_HOST`). Override with `BEARDOG_CLIENT_HOST`.
    #[serde(default = "default_client_host")]
    pub client_host: String,

    /// Discovery service host (legacy / local tooling)
    ///
    /// **Not** a peer primal address. Use runtime capability discovery for other primals.
    /// **Fallback**: `DEFAULT_HOST`. Override with `BEARDOG_DISCOVERY_HOST`.
    #[serde(default = "default_discovery_host")]
    pub discovery_host: String,

    /// Database host
    ///
    /// Prefer `DATABASE_URL` or `BEARDOG_DATABASE_HOST`. **Fallback**: infrastructure dev host (see `FALLBACK_DEV_INFRASTRUCTURE_HOST`).
    #[serde(default = "default_database_host")]
    pub database_host: String,

    /// Redis cache host
    ///
    /// Prefer `REDIS_URL` or `BEARDOG_REDIS_HOST`. **Fallback**: infrastructure dev host.
    #[serde(default = "default_redis_host")]
    pub redis_host: String,

    /// Metrics/monitoring host
    ///
    /// Prefer `GRAFANA_URL` or `BEARDOG_METRICS_HOST`. **Fallback**: infrastructure dev host.
    #[serde(default = "default_metrics_host")]
    pub metrics_host: String,

    /// External/public host
    ///
    /// **Fallback**: infrastructure dev host. Production: set `BEARDOG_EXTERNAL_HOST` or discovery-derived name.
    #[serde(default = "default_external_host")]
    pub external_host: String,
}

// ============================================================================
// Default Functions (with Environment Variable Support)
// ============================================================================

/// Last-resort infrastructure hostname when no URL or specific `BEARDOG_*_HOST` is set.
fn infrastructure_fallback_host() -> String {
    FALLBACK_DEV_INFRASTRUCTURE_HOST.to_string()
}

fn default_api_host() -> String {
    DEFAULT_BIND_HOST.to_string()
}

fn default_client_host() -> String {
    DEFAULT_HOST.to_string()
}

fn default_discovery_host() -> String {
    DEFAULT_HOST.to_string()
}

fn default_database_host() -> String {
    infrastructure_fallback_host()
}

fn default_redis_host() -> String {
    infrastructure_fallback_host()
}

fn default_metrics_host() -> String {
    infrastructure_fallback_host()
}

fn default_external_host() -> String {
    infrastructure_fallback_host()
}

/// Extract host from URL (simple extraction, not full parsing)
fn extract_host_from_url(url: &str) -> String {
    // Simple extraction: remove protocol and path
    url.trim_start_matches("http://")
        .trim_start_matches("https://")
        .trim_start_matches("redis://")
        .trim_start_matches("postgres://")
        .trim_start_matches("postgresql://")
        .split('/')
        .next()
        .unwrap_or(url)
        .split(':')
        .next()
        .unwrap_or(url)
        .to_string()
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
    /// use beardog_config::domains::network_hosts::{NetworkHostsConfig, DEFAULT_HOST};
    ///
    /// let hosts = NetworkHostsConfig::with_defaults();
    /// assert_eq!(hosts.client_host, DEFAULT_HOST);
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
        let infra = std::env::var("BEARDOG_INFRASTRUCTURE_HOST_FALLBACK")
            .unwrap_or_else(|_| FALLBACK_DEV_INFRASTRUCTURE_HOST.to_string());

        Self {
            api_host: std::env::var("BEARDOG_API_HOST")
                .unwrap_or_else(|_| DEFAULT_BIND_HOST.to_string()),
            client_host: std::env::var("BEARDOG_CLIENT_HOST")
                .unwrap_or_else(|_| DEFAULT_HOST.to_string()),
            discovery_host: std::env::var("BEARDOG_DISCOVERY_HOST")
                .unwrap_or_else(|_| DEFAULT_HOST.to_string()),
            database_host: std::env::var("BEARDOG_DATABASE_HOST")
                .or_else(|_| std::env::var("DATABASE_URL").map(|url| extract_host_from_url(&url)))
                .unwrap_or_else(|_| infra.clone()),
            redis_host: std::env::var("BEARDOG_REDIS_HOST")
                .or_else(|_| std::env::var("REDIS_URL").map(|url| extract_host_from_url(&url)))
                .unwrap_or_else(|_| infra.clone()),
            metrics_host: std::env::var("BEARDOG_METRICS_HOST")
                .or_else(|_| std::env::var("GRAFANA_URL").map(|url| extract_host_from_url(&url)))
                .unwrap_or_else(|_| infra.clone()),
            external_host: std::env::var("BEARDOG_EXTERNAL_HOST").unwrap_or(infra),
        }
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
                return Err(format!("{name} cannot be empty"));
            }

            // Basic validation: no whitespace, no control characters
            if host.chars().any(|c| c.is_whitespace() || c.is_control()) {
                return Err(format!("{name} contains invalid characters: {host}"));
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
        assert_eq!(DEFAULT_HOST, FALLBACK_DEV_INFRASTRUCTURE_HOST);
        assert_eq!(
            DEFAULT_BIND_HOST,
            crate::domains::network_addresses::WILDCARD_IPV4
        );
        assert_eq!(
            DEFAULT_LOOPBACK_IPV4,
            crate::domains::network_addresses::LOCALHOST_IPV4
        );
        assert_eq!(
            DEFAULT_LOOPBACK_IPV6,
            crate::domains::network_addresses::LOCALHOST_IPV6
        );
    }

    #[test]
    fn test_default_config() {
        let config = NetworkHostsConfig::with_defaults();

        assert_eq!(config.api_host, DEFAULT_BIND_HOST);
        assert_eq!(config.client_host, DEFAULT_HOST);
        assert_eq!(config.discovery_host, DEFAULT_HOST);
        assert_eq!(config.database_host, FALLBACK_DEV_INFRASTRUCTURE_HOST);
        assert_eq!(config.redis_host, FALLBACK_DEV_INFRASTRUCTURE_HOST);
        assert_eq!(config.metrics_host, FALLBACK_DEV_INFRASTRUCTURE_HOST);
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
        assert_eq!(POSTGRES_DISCOVERY_HINT, FALLBACK_DEV_INFRASTRUCTURE_HOST);
        assert_eq!(REDIS_DISCOVERY_HINT, FALLBACK_DEV_INFRASTRUCTURE_HOST);
        assert_eq!(GRAFANA_DISCOVERY_HINT, FALLBACK_DEV_INFRASTRUCTURE_HOST);
        assert_eq!(JAEGER_DISCOVERY_HINT, FALLBACK_DEV_INFRASTRUCTURE_HOST);
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
