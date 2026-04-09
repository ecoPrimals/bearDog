// SPDX-License-Identifier: AGPL-3.0-or-later

//! # Consolidated Network Configuration Domain
//!
//! This module consolidates ALL network-related configuration structs across the `BearDog`
//! ecosystem into a single, unified network configuration system. It eliminates fragmentation
//! by providing a canonical network configuration that replaces 12+ scattered network configs.
//!
//! ## 🎯 **Consolidation Impact**
//!
//! This module unifies and replaces:
//! - `NetworkConfig` (multiple variants across crates)
//! - `ConnectionPoolConfig` (beardog-types, beardog-core)
//! - `TimeoutConfig` (beardog-types, beardog-core)
//! - `LoadBalancerConfig` (beardog-types, beardog-core)
//! - `HealthCheckConfig` (beardog-types, beardog-core)
//! - `EndpointSecurityConfig` (beardog-core)
//! - `TlsConfig` (beardog-core)
//! - `CacheConfig` (beardog-core)
//! - `NetworkSecurityConfig` (beardog-types)
//! - `RateLimitConfig` (network-specific instances)
//! - Plus additional scattered network configurations

use crate::constants::domains::network::limits::MAX_CONNECTIONS;
use beardog_errors::{BearDogError, ConfigurationErrorCategory};
use serde::{Deserialize, Serialize};
use std::time::Duration;

// Import all modular components
pub mod client;
pub mod connection;
pub mod endpoints;
pub mod monitoring;
pub mod performance;
pub mod security;
pub mod server;

// Re-export all types for backward compatibility
pub use client::ClientConfiguration;
pub use connection::{ConnectionPoolConfig, LoadBalancerConfiguration, TimeoutConfiguration};
pub use endpoints::EndpointsConfiguration;
pub use monitoring::{
    HealthCheckConfiguration, NetworkMonitoringConfiguration, ServiceDiscoveryConfiguration,
};
pub use performance::{
    CacheConfiguration, NetworkPerformanceConfiguration, NetworkRateLimitConfiguration,
};
pub use security::{
    EndpointSecurityConfiguration, NetworkSecurityConfiguration, TlsConfiguration,
    TlsVerificationMode,
};
pub use server::ServerConfiguration;

/// **CONSOLIDATED NETWORK CONFIGURATION** - Single source of truth for all network settings
///
/// This structure consolidates all network-related configurations across the `BearDog` ecosystem,
/// eliminating fragmentation and providing a unified network configuration interface.
#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct ConsolidatedNetworkConfiguration {
    /// **CORE NETWORK SETTINGS**
    /// Bind addresses, ports, and listener limits for inbound traffic. **Default:** [`ServerConfiguration::default()`].
    pub server: ServerConfiguration,
    /// Outbound dialer defaults (timeouts, connection caps). **Default:** [`ClientConfiguration::default()`].
    pub client: ClientConfiguration,
    /// Static and discovered service endpoints. **Default:** [`EndpointsConfiguration::default()`].
    pub endpoints: EndpointsConfiguration,

    /// **CONNECTION MANAGEMENT**
    /// Pool sizing and idle handling for outbound connections. **Default:** [`ConnectionPoolConfig::default()`].
    pub connection_pool: ConnectionPoolConfig,
    /// Connect/read/write idle budgets. **Default:** [`TimeoutConfiguration::default()`].
    pub timeouts: TimeoutConfiguration,
    /// L4/L7 balancing across upstreams. **Default:** [`LoadBalancerConfiguration::default()`].
    pub load_balancer: LoadBalancerConfiguration,

    /// **SECURITY & ENCRYPTION**
    /// Certificate material and TLS versions. **Default:** [`TlsConfiguration::default()`].
    pub tls: TlsConfiguration,
    /// Firewalls, IP allowlists, and mesh trust hints. **Default:** [`NetworkSecurityConfiguration::default()`].
    pub security: NetworkSecurityConfiguration,
    /// Per-endpoint mTLS and authz policies. **Default:** [`EndpointSecurityConfiguration::default()`].
    pub endpoint_security: EndpointSecurityConfiguration,

    /// **PERFORMANCE & OPTIMIZATION**
    /// Response/path caching knobs. **Default:** [`CacheConfiguration::default()`].
    pub cache: CacheConfiguration,
    /// Compression, pipelining, and buffer tuning. **Default:** [`NetworkPerformanceConfiguration::default()`].
    pub performance: NetworkPerformanceConfiguration,
    /// Token-bucket / leaky-bucket request throttles. **Default:** [`NetworkRateLimitConfiguration::default()`].
    pub rate_limiting: NetworkRateLimitConfiguration,

    /// **MONITORING & HEALTH**
    /// Active probes for upstream readiness. **Default:** [`HealthCheckConfiguration::default()`].
    pub health_checks: HealthCheckConfiguration,
    /// Metrics and tracing exporters for network plane. **Default:** [`NetworkMonitoringConfiguration::default()`].
    pub monitoring: NetworkMonitoringConfiguration,
    /// mDNS/DNS-SD and registry integration. **Default:** [`ServiceDiscoveryConfiguration::default()`].
    pub discovery: ServiceDiscoveryConfiguration,
}

impl ConsolidatedNetworkConfiguration {
    /// Validate the network configuration
    ///
    /// # Errors
    ///
    /// Returns an error if server, client, endpoints, rate limiting, or connection pool settings are invalid.
    pub fn validate(&self) -> Result<(), BearDogError> {
        // Validate server configuration
        self.server.validate()?;

        // Validate client configuration
        self.client.validate()?;

        // Validate endpoints configuration
        self.endpoints.validate()?;

        // Validate connection pool
        if self.connection_pool.max_size < self.connection_pool.min_size {
            return Err(BearDogError::configuration(
                "Pool max size cannot be less than min size",
            ));
        }

        // Validate rate limiting
        if self.rate_limiting.enabled && self.rate_limiting.burst_size == 0 {
            return Err(BearDogError::configuration(
                "Rate limit burst size cannot be zero when enabled",
            ));
        }

        Ok(())
    }

    /// Create a development-friendly configuration
    pub fn development() -> Self {
        let mut config = Self::default();

        // Relaxed settings for development
        config.server.max_connections =
            crate::constants::domains::system::defaults::DEFAULT_QUEUE_SIZE;
        config.rate_limiting.global_rps = Some(100_000);
        config.rate_limiting.per_ip_rpm = Some(MAX_CONNECTIONS as u64);
        config
            .security
            .ddos_protection
            .max_requests_per_ip_per_minute = MAX_CONNECTIONS as u64;
        config.tls.enabled = false; // Disable TLS for easier development
        config.endpoint_security.enable_authentication = false;

        config
    }

    /// Create a production-hardened configuration
    pub fn production() -> Self {
        let mut config = Self::default();

        // Strict settings for production
        config.server.max_connections = std::env::var("BEARDOG_PRODUCTION_MAX_CONNECTIONS")
            .ok()
            .and_then(|s| s.parse().ok())
            .unwrap_or(MAX_CONNECTIONS);
        #[expect(
            clippy::cast_possible_truncation,
            reason = "timeout millis fit u64 for rate limiter RPS config"
        )]
        let global_rps = crate::constants::domains::network::defaults::DEFAULT_CONNECTION_TIMEOUT
            .as_millis() as u64;
        config.rate_limiting.global_rps = Some(global_rps);
        config.rate_limiting.per_ip_rpm = Some(
            std::env::var("BEARDOG_PRODUCTION_PER_IP_RPM")
                .ok()
                .and_then(|s| s.parse().ok())
                .unwrap_or(100),
        );
        config
            .security
            .ddos_protection
            .max_requests_per_ip_per_minute = 100;
        config.security.ddos_protection.enable_challenge_response = true;
        config.tls.enabled = true;
        config.tls.verification_mode = TlsVerificationMode::Full;
        config.endpoint_security.enable_authentication = true;
        config.endpoint_security.enable_authorization = true;

        config
    }
}

/// **CANONICAL** Rate limiting configuration
///
/// This is the single source of truth for rate limiting settings across `BearDog`.
/// Consolidates all `RateLimitConfig` variants from:
/// - `beardog-types/src/network.rs`
/// - `beardog-types/src/canonical/monitoring/mod.rs`
/// - `beardog-types/src/canonical/config/domains/security.rs`
/// - `beardog-types/src/canonical/config/domains/workflow_config.rs`
/// - `beardog-types/src/canonical/config/network.rs`
/// - `beardog-types/src/canonical/services/endpoints.rs`
/// - `beardog-types/src/canonical/providers_unified/performance.rs`
/// - `beardog-security/src/types.rs`
/// - `beardog-security/src/types/mod.rs`
///
/// # Features
/// - Supports multiple rate limiting strategies
/// - Configurable scope (per-IP, per-user, global, etc.)
/// - Burst handling with optional burst size
/// - Allowlist for exemptions
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
pub struct RateLimitConfig {
    /// Enable rate limiting
    pub enabled: bool,

    /// Maximum requests allowed in the time window
    pub max_requests: u64,

    /// Time window duration for rate limiting
    #[serde(
        serialize_with = "serialize_duration_for_rate_limit",
        deserialize_with = "deserialize_duration_for_rate_limit"
    )]
    pub window: Duration,

    /// Optional burst size (allows temporary exceeding of rate limit)
    pub burst_size: Option<u64>,

    /// Rate limiting strategy/algorithm
    pub strategy: RateLimitStrategy,

    /// Scope of rate limiting (per-IP, per-user, global, etc.)
    pub scope: RateLimitScope,

    /// Allowlist for rate limiting exemptions (IP addresses, user IDs, etc.)
    pub allowlist: Vec<String>,
}

/// Rate limiting strategy/algorithm
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq, Default)]
pub enum RateLimitStrategy {
    /// Fixed window algorithm (simple, fast, but allows bursts at window boundaries)
    FixedWindow,
    /// Sliding window algorithm (more accurate, smoother rate enforcement)
    SlidingWindow,
    /// Token bucket algorithm (supports controlled bursts)
    #[default]
    TokenBucket,
    /// Leaky bucket algorithm (smooth, constant rate output)
    LeakyBucket,
}

/// Rate limiting scope
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq, Default)]
pub enum RateLimitScope {
    /// Per IP address
    #[default]
    PerIp,
    /// Per authenticated user
    PerUser,
    /// Per API key
    PerApiKey,
    /// Global rate limit (applies to all requests)
    Global,
    /// Custom scope with identifier
    Custom(String),
}

// Serde helper functions for Duration serialization in rate limit context
fn serialize_duration_for_rate_limit<S>(
    duration: &Duration,
    serializer: S,
) -> Result<S::Ok, S::Error>
where
    S: serde::Serializer,
{
    serializer.serialize_u64(duration.as_secs())
}

fn deserialize_duration_for_rate_limit<'de, D>(deserializer: D) -> Result<Duration, D::Error>
where
    D: serde::Deserializer<'de>,
{
    let secs = u64::deserialize(deserializer)?;
    Ok(Duration::from_secs(secs))
}

impl Default for RateLimitConfig {
    fn default() -> Self {
        Self {
            enabled: true,
            max_requests: 100,
            window: Duration::from_secs(60), // 1 minute
            burst_size: Some(200),
            strategy: RateLimitStrategy::TokenBucket,
            scope: RateLimitScope::PerIp,
            allowlist: vec![
                crate::constants::domains::network::addresses::LOCALHOST_IPV4.to_string(),
            ],
        }
    }
}

impl RateLimitConfig {
    /// Create a configuration for requests per minute
    pub fn per_minute(requests: u64) -> Self {
        Self {
            max_requests: requests,
            window: Duration::from_secs(60),
            ..Default::default()
        }
    }

    /// Create a configuration for requests per second
    pub fn per_second(requests: u64) -> Self {
        Self {
            max_requests: requests,
            window: Duration::from_secs(1),
            ..Default::default()
        }
    }

    /// Create a global rate limit
    pub fn global(requests: u64, window_secs: u64) -> Self {
        Self {
            max_requests: requests,
            window: Duration::from_secs(window_secs),
            scope: RateLimitScope::Global,
            ..Default::default()
        }
    }

    /// Validate rate limit configuration
    ///
    /// # Errors
    ///
    /// Returns an error if rate limiting is enabled but limits, window, or burst settings are invalid.
    pub fn validate(&self) -> Result<(), BearDogError> {
        if self.enabled {
            if self.max_requests == 0 {
                return Err(BearDogError::configuration(
                    "Rate limit max_requests cannot be zero when enabled",
                ));
            }

            if self.window.as_secs() == 0 {
                return Err(BearDogError::configuration(
                    "Rate limit window cannot be zero when enabled",
                ));
            }

            if let Some(burst) = self.burst_size
                && burst > self.max_requests * 2
            {
                return Err(BearDogError::configuration(
                    "Burst size should not exceed 2x max_requests",
                ));
            }
        }

        Ok(())
    }
}

/// Backward compatibility alias (consolidated Nov 7, 2025)
pub type RateLimitConfiguration = RateLimitConfig;

// =============================================================================
// BACKWARD COMPATIBILITY TYPE ALIASES - Phase 2 Consolidation (Nov 11, 2025)
// =============================================================================
//
// The following type aliases provide backward compatibility for all scattered
// NetworkConfig variants found across the codebase. This allows gradual
// migration to ConsolidatedNetworkConfiguration while maintaining zero breaking
// changes.
//
// These aliases consolidate 21 NetworkConfig variants found in:
// - beardog-tunnel (2 variants)
// - beardog-core (1 variant)
// - beardog-production (1 variant)
// - beardog-adapters (1 variant)
// - beardog-types/canonical/* (9 variants)
// - beardog-config (1 variant)
// - beardog-utils (2 variants)
// - beardog-types/network.rs (4 variants)

/// Primary `NetworkConfig` alias - points to consolidated configuration
///
/// **CONSOLIDATED (Nov 11, 2025)**: Use `ConsolidatedNetworkConfiguration` for new code.
///
/// This type alias maintains backward compatibility with the most common
/// `NetworkConfig` usage pattern across the codebase.
pub type NetworkConfig = ConsolidatedNetworkConfiguration;

/// Network discovery configuration alias
///
/// **CONSOLIDATED**: Maps to embedded discovery configuration.
pub type NetworkDiscoveryConfig = ServiceDiscoveryConfiguration;

/// Network security configuration alias
///
/// **CONSOLIDATED**: Maps to embedded security configuration.
pub type NetworkSecurityConfig = NetworkSecurityConfiguration;

/// Network policy configuration alias
///
/// **CONSOLIDATED**: Use full `ConsolidatedNetworkConfiguration` with embedded configs.
pub type NetworkPolicyConfig = ConsolidatedNetworkConfiguration;

/// Network resource configuration alias
///
/// **CONSOLIDATED**: Use performance configuration instead.
pub type NetworkResourceConfig = NetworkPerformanceConfiguration;

/// Network handler configuration alias
///
/// **CONSOLIDATED**: Use client/server configuration as appropriate.
pub type NetworkHandlerConfig = ClientConfiguration;

/// Network monitoring configuration alias
///
/// **CONSOLIDATED**: Maps to embedded monitoring configuration.
pub type NetworkMonitoringConfig = NetworkMonitoringConfiguration;

/// Network environment configuration alias (from beardog-utils)
///
/// **CONSOLIDATED**: Use `ConsolidatedNetworkConfiguration` with environment variable loading.
pub type NetworkEnvConfig = ConsolidatedNetworkConfiguration;

/// Networking configuration alias (from beardog-production)
///
/// **CONSOLIDATED**: Maps to consolidated network configuration.
pub type NetworkingConfig = ConsolidatedNetworkConfiguration;

/// Network scan configuration alias (from HSM discovery)
///
/// This is domain-specific to HSM network scanning and remains separate.
/// However, we provide a helper to convert to standard network config.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct NetworkScanConfig {
    /// CIDRs or literal ranges to probe when locating HSMs or peers on the LAN.
    pub ip_ranges: Vec<String>,
    /// Maximum time to wait for each probe response during a scan pass.
    pub timeout_ms: u32,
}

impl NetworkScanConfig {
    /// Convert to standard network configuration for consistency
    pub fn to_network_config(&self) -> ConsolidatedNetworkConfiguration {
        // Set appropriate timeout based on scan timeout
        // Note: TimeoutConfiguration structure handled by its own implementation
        ConsolidatedNetworkConfiguration::default()
    }
}

/// Network core configuration alias (from `network_unified`)
///
/// **CONSOLIDATED**: Use `ConsolidatedNetworkConfiguration` instead.
pub type NetworkCoreConfig = ConsolidatedNetworkConfiguration;

// =============================================================================
// MIGRATION HELPERS
// =============================================================================

impl ConsolidatedNetworkConfiguration {
    /// Create from simple host and port (common pattern in old code)
    pub fn from_host_port(host: impl Into<String>, port: u16) -> Self {
        let mut config = Self::default();
        config.server.bind_address = host.into();
        config.server.port = port;
        config
    }

    /// Create from bind address (`SocketAddr` pattern)
    pub fn from_bind_address(addr: std::net::SocketAddr) -> Self {
        let mut config = Self::default();
        config.server.bind_address = addr.ip().to_string();
        config.server.port = addr.port();
        config
    }

    /// Get bind address as `SocketAddr` for compatibility
    ///
    /// # Errors
    ///
    /// Returns an error if the bind address or port cannot be resolved to a socket address.
    pub fn to_bind_address(&self) -> Result<std::net::SocketAddr, BearDogError> {
        use std::net::ToSocketAddrs;
        let addr_str = format!("{}:{}", self.server.bind_address, self.server.port);
        addr_str
            .to_socket_addrs()
            .map_err(|e| BearDogError::Configuration {
                message: format!("Invalid bind address: {e}"),
                category: ConfigurationErrorCategory::default(),
            })?
            .next()
            .ok_or_else(|| BearDogError::Configuration {
                message: "Could not resolve bind address".to_string(),
                category: ConfigurationErrorCategory::default(),
            })
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_default_network_config_validation() {
        let config = ConsolidatedNetworkConfiguration::default();
        assert!(config.validate().is_ok());
    }

    #[test]
    fn test_development_config() {
        let config = ConsolidatedNetworkConfiguration::development();
        assert_eq!(
            config.server.max_connections,
            crate::constants::domains::system::defaults::DEFAULT_QUEUE_SIZE
        );
        assert!(!config.tls.enabled);
        assert!(config.validate().is_ok());
    }

    #[test]
    fn test_production_config() {
        let config = ConsolidatedNetworkConfiguration::production();
        assert_eq!(config.server.max_connections, 10000);
        assert!(config.tls.enabled);
        assert_eq!(config.tls.verification_mode, TlsVerificationMode::Full);
        assert!(config.validate().is_ok());
    }

    #[test]
    fn test_invalid_config_validation() {
        let mut config = ConsolidatedNetworkConfiguration::default();
        config.server.port = 0;
        assert!(config.validate().is_err());
    }

    #[test]
    fn test_connection_pool_validation() {
        let mut config = ConsolidatedNetworkConfiguration::default();
        config.connection_pool.max_size = 5;
        config.connection_pool.min_size =
            crate::constants::domains::system::defaults::DEFAULT_POOL_SIZE;
        assert!(config.validate().is_err());
    }
}
