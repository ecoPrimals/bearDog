// SPDX-License-Identifier: AGPL-3.0-only

//! Network Address Configuration
//!
//! Centralized configuration for **self-knowledge** bind/connect addresses used by BearDog.
//! Aligns with `ZERO_HARDCODING_SPECIFICATION` and capability-based discovery: literals here are
//! **documented fallbacks only**—production must set env vars, config files, or discovery-derived
//! values. Peer / other-primal endpoints are **never** defined here; use runtime discovery.
//!
//! # Design Philosophy
//!
//! - **Configuration over Hardcoding**: All fallbacks overridable via ENV or config file
//! - **Fallback loopback**: Default bind/API strings favor loopback for local dev only
//! - **Environment-First**: `BEARDOG_*` environment variables take precedence over fallbacks
//! - **Platform Agnostic**: Works across development, staging, and production
//!
//! # Usage
//!
//! ```rust
//! use beardog_config::global::BEARDOG_CONFIG;
//!
//! // Access addresses from global config
//! let api_host = &BEARDOG_CONFIG.network.addresses.api_host;
//! let bind_address = &BEARDOG_CONFIG.network.addresses.bind_address;
//! ```
//!
//! # Environment Variables
//!
//! - `BEARDOG_API_HOST` - API server hostname (default: 127.0.0.1)
//! - `BEARDOG_BIND_ADDRESS` - Server bind address (default: 127.0.0.1)
//! - `BEARDOG_EXTERNAL_HOST` - External/public hostname (default: localhost)
//! - `BEARDOG_MULTICAST_ADDRESS` - Multicast address for discovery (fallback: see `DEFAULT_MULTICAST_ADDRESS`)
//! - `BEARDOG_LOCALHOST_IPV4` / `BEARDOG_LOCALHOST_IPV6` / `BEARDOG_WILDCARD_IPV4` - Override standard IP literals used in config (optional)

use serde::{Deserialize, Serialize};
use std::net::{IpAddr, Ipv4Addr};

/// Network addresses configuration
///
/// All network addresses and hostnames used by BearDog components. Configurable via
/// environment variables or config file, with secure defaults as fallback.
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
pub struct NetworkAddressesConfig {
    /// API server hostname
    ///
    /// Default hostname for API server. Defaults to 127.0.0.1 (localhost).
    /// Override with `BEARDOG_API_HOST` environment variable.
    #[serde(default = "default_api_host")]
    pub api_host: String,

    /// Server bind address
    ///
    /// Address to bind server sockets to. Defaults to 127.0.0.1 (localhost only).
    /// Use 0.0.0.0 for all interfaces in production.
    /// Override with `BEARDOG_BIND_ADDRESS` environment variable.
    #[serde(default = "default_bind_address")]
    pub bind_address: String,

    /// External/public hostname
    ///
    /// Public-facing hostname for service registration. Defaults to "localhost".
    /// Override with `BEARDOG_EXTERNAL_HOST` environment variable.
    #[serde(default = "default_external_host")]
    pub external_host: String,

    /// Multicast address for discovery
    ///
    /// IPv4 multicast address for service discovery. Defaults to 239.255.0.1.
    /// Override with `BEARDOG_MULTICAST_ADDRESS` environment variable.
    #[serde(default = "default_multicast_address")]
    pub multicast_address: String,

    /// Localhost IPv4 address
    ///
    /// **Fallback**: IANA IPv4 loopback. Override with `BEARDOG_LOCALHOST_IPV4` or config.
    #[serde(default = "default_localhost_ipv4")]
    pub localhost_ipv4: IpAddr,

    /// Localhost IPv6 address
    ///
    /// **Fallback**: IANA IPv6 loopback. Override with `BEARDOG_LOCALHOST_IPV6` or config.
    #[serde(default = "default_localhost_ipv6")]
    pub localhost_ipv6: IpAddr,

    /// Wildcard IPv4 address
    ///
    /// **Fallback**: unspecified IPv4 (all interfaces). Override with `BEARDOG_WILDCARD_IPV4` or config.
    #[serde(default = "default_wildcard_ipv4")]
    pub wildcard_ipv4: IpAddr,
}

// ---------------------------------------------------------------------------
// Documented fallbacks (ZERO_HARDCODING: single source; never the only option in production)
// ---------------------------------------------------------------------------

/// **Fallback** API host when `BEARDOG_API_HOST` and config omit a value (loopback, dev-oriented).
pub const DEFAULT_API_HOST: &str = "127.0.0.1";

/// **Fallback** bind address when `BEARDOG_BIND_ADDRESS` and config omit a value (loopback, dev-oriented).
pub const DEFAULT_BIND_ADDRESS: &str = "127.0.0.1";

/// **Fallback** external hostname when `BEARDOG_EXTERNAL_HOST` is unset.
pub const DEFAULT_EXTERNAL_HOST: &str = "localhost";

/// **Fallback** multicast group when `BEARDOG_MULTICAST_ADDRESS` is unset (discovery transport; not a peer address).
pub const DEFAULT_MULTICAST_ADDRESS: &str = "239.255.0.1";

/// **Fallback** loopback IPv4 string (self-knowledge / parsing); prefer config or `localhost_ipv4`.
pub const LOCALHOST_IPV4: &str = "127.0.0.1";

/// **Fallback** loopback IPv6 string (self-knowledge / parsing).
pub const LOCALHOST_IPV6: &str = "::1";

/// **Fallback** IPv4 “all interfaces” string for production-style bind when explicitly chosen.
pub const WILDCARD_IPV4: &str = "0.0.0.0";

fn default_api_host() -> String {
    std::env::var("BEARDOG_API_HOST").unwrap_or_else(|_| DEFAULT_API_HOST.to_string())
}

fn default_bind_address() -> String {
    std::env::var("BEARDOG_BIND_ADDRESS").unwrap_or_else(|_| DEFAULT_BIND_ADDRESS.to_string())
}

fn default_external_host() -> String {
    std::env::var("BEARDOG_EXTERNAL_HOST").unwrap_or_else(|_| DEFAULT_EXTERNAL_HOST.to_string())
}

fn default_multicast_address() -> String {
    std::env::var("BEARDOG_MULTICAST_ADDRESS")
        .unwrap_or_else(|_| DEFAULT_MULTICAST_ADDRESS.to_string())
}

fn parse_ip_env(key: &str, fallback: IpAddr) -> IpAddr {
    std::env::var(key)
        .ok()
        .and_then(|s| s.parse().ok())
        .unwrap_or(fallback)
}

fn default_localhost_ipv4() -> IpAddr {
    parse_ip_env("BEARDOG_LOCALHOST_IPV4", IpAddr::V4(Ipv4Addr::LOCALHOST))
}

fn default_localhost_ipv6() -> IpAddr {
    parse_ip_env(
        "BEARDOG_LOCALHOST_IPV6",
        IpAddr::V6(std::net::Ipv6Addr::LOCALHOST),
    )
}

fn default_wildcard_ipv4() -> IpAddr {
    parse_ip_env("BEARDOG_WILDCARD_IPV4", IpAddr::V4(Ipv4Addr::UNSPECIFIED))
}

impl Default for NetworkAddressesConfig {
    fn default() -> Self {
        Self::with_defaults()
    }
}

impl NetworkAddressesConfig {
    /// Creates configuration with secure defaults
    ///
    /// Reads from environment variables if set, otherwise uses documented defaults.
    /// Default bind address uses the **fallback** loopback string (see `DEFAULT_BIND_ADDRESS`).
    ///
    /// # Example
    ///
    /// ```rust
    /// use beardog_config::NetworkAddressesConfig;
    /// use beardog_config::domains::network_addresses::DEFAULT_API_HOST;
    ///
    /// let addresses = NetworkAddressesConfig::with_defaults();
    /// assert_eq!(addresses.api_host, DEFAULT_API_HOST);
    /// ```
    #[must_use]
    pub fn with_defaults() -> Self {
        Self {
            api_host: default_api_host(),
            bind_address: default_bind_address(),
            external_host: default_external_host(),
            multicast_address: default_multicast_address(),
            localhost_ipv4: default_localhost_ipv4(),
            localhost_ipv6: default_localhost_ipv6(),
            wildcard_ipv4: default_wildcard_ipv4(),
        }
    }

    /// Loads configuration from environment variables
    ///
    /// This is equivalent to `with_defaults()` but makes the intent explicit.
    ///
    /// # Example
    ///
    /// ```rust
    /// use beardog_config::NetworkAddressesConfig;
    ///
    /// let addresses = NetworkAddressesConfig::from_env();
    /// // Reads BEARDOG_API_HOST, BEARDOG_BIND_ADDRESS, etc.
    /// ```
    #[must_use]
    pub fn from_env() -> Self {
        Self::default()
    }

    /// Creates production-ready configuration
    ///
    /// Suitable for production environments:
    /// - Bind address: **fallback** all-interfaces IPv4 (`WILDCARD_IPV4`) unless `BEARDOG_BIND_ADDRESS` is set
    /// - API host: resolved from environment or kept as configured
    /// - External host: resolved from environment
    ///
    /// # Example
    ///
    /// ```rust
    /// use beardog_config::NetworkAddressesConfig;
    /// use beardog_config::domains::network_addresses::WILDCARD_IPV4;
    ///
    /// let addresses = NetworkAddressesConfig::for_production();
    /// assert_eq!(addresses.bind_address, WILDCARD_IPV4);
    /// ```
    #[must_use]
    pub fn for_production() -> Self {
        Self {
            api_host: default_api_host(),
            bind_address: std::env::var("BEARDOG_BIND_ADDRESS")
                .unwrap_or_else(|_| WILDCARD_IPV4.to_string()),
            external_host: default_external_host(),
            multicast_address: default_multicast_address(),
            localhost_ipv4: default_localhost_ipv4(),
            localhost_ipv6: default_localhost_ipv6(),
            wildcard_ipv4: default_wildcard_ipv4(),
        }
    }

    /// Validates that all addresses are valid
    ///
    /// Ensures addresses are properly formatted and parseable.
    pub fn validate(&self) -> Result<(), String> {
        // Validate API host (can be hostname or IP)
        if self.api_host.is_empty() {
            return Err("api_host cannot be empty".to_string());
        }

        // Validate bind address (must be valid IP)
        if self.bind_address.parse::<IpAddr>().is_err() {
            return Err(format!(
                "bind_address ({}) is not a valid IP address",
                self.bind_address
            ));
        }

        // Validate external host
        if self.external_host.is_empty() {
            return Err("external_host cannot be empty".to_string());
        }

        // Validate multicast address
        if self.multicast_address.parse::<IpAddr>().is_err() {
            return Err(format!(
                "multicast_address ({}) is not a valid IP address",
                self.multicast_address
            ));
        }

        Ok(())
    }

    /// Get bind address for all interfaces (0.0.0.0)
    ///
    /// Convenience method for production deployments.
    #[must_use]
    pub fn wildcard_bind(&self) -> String {
        self.wildcard_ipv4.to_string()
    }

    /// Get localhost bind address (127.0.0.1)
    ///
    /// Convenience method for development/testing.
    #[must_use]
    pub fn localhost_bind(&self) -> String {
        self.localhost_ipv4.to_string()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_default_addresses() {
        let config = NetworkAddressesConfig::with_defaults();

        // Should use secure defaults
        assert_eq!(config.api_host, DEFAULT_API_HOST);
        assert_eq!(config.bind_address, DEFAULT_BIND_ADDRESS);
        assert_eq!(config.external_host, DEFAULT_EXTERNAL_HOST);
    }

    #[test]
    fn test_localhost_addresses() {
        let config = NetworkAddressesConfig::with_defaults();

        assert_eq!(config.localhost_ipv4, IpAddr::V4(Ipv4Addr::LOCALHOST));
        assert_eq!(
            config.localhost_ipv6,
            IpAddr::V6(std::net::Ipv6Addr::LOCALHOST)
        );
    }

    #[test]
    fn test_wildcard_address() {
        let config = NetworkAddressesConfig::with_defaults();

        assert_eq!(config.wildcard_ipv4, IpAddr::V4(Ipv4Addr::UNSPECIFIED));
        assert_eq!(config.wildcard_bind(), WILDCARD_IPV4);
    }

    #[test]
    fn test_production_config() {
        let config = NetworkAddressesConfig::for_production();

        // Production should bind to all interfaces
        assert_eq!(config.bind_address, WILDCARD_IPV4);
    }

    #[test]
    fn test_validation_passes_for_defaults() {
        let config = NetworkAddressesConfig::with_defaults();
        assert!(config.validate().is_ok());
    }

    #[test]
    fn test_validation_fails_for_invalid_bind() {
        let mut config = NetworkAddressesConfig::with_defaults();
        config.bind_address = "invalid".to_string();

        assert!(config.validate().is_err());
    }

    #[test]
    fn test_validation_fails_for_empty_host() {
        let mut config = NetworkAddressesConfig::with_defaults();
        config.api_host = String::new();

        assert!(config.validate().is_err());
    }

    #[test]
    fn test_serialization() {
        let config = NetworkAddressesConfig::with_defaults();

        let json = serde_json::to_string(&config).expect("Should serialize");
        let deserialized: NetworkAddressesConfig =
            serde_json::from_str(&json).expect("Should deserialize");

        assert_eq!(config, deserialized);
    }

    #[test]
    fn test_from_env_equals_default() {
        // When no env vars set, from_env should equal with_defaults
        let from_env = NetworkAddressesConfig::from_env();
        let with_defaults = NetworkAddressesConfig::with_defaults();

        assert_eq!(from_env, with_defaults);
    }

    #[test]
    fn test_localhost_bind_helper() {
        let config = NetworkAddressesConfig::with_defaults();
        assert_eq!(config.localhost_bind(), LOCALHOST_IPV4);
    }

    #[test]
    fn test_wildcard_bind_helper() {
        let config = NetworkAddressesConfig::with_defaults();
        assert_eq!(config.wildcard_bind(), "0.0.0.0");
    }

    #[test]
    fn test_clone() {
        let config = NetworkAddressesConfig::with_defaults();
        let cloned = config.clone();

        assert_eq!(config, cloned);
    }

    #[test]
    fn test_debug_format() {
        let config = NetworkAddressesConfig::with_defaults();
        let debug_str = format!("{:?}", config);

        assert!(debug_str.contains("NetworkAddressesConfig"));
        assert!(debug_str.contains("api_host"));
    }
}
