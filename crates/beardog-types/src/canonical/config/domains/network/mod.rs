//! # Consolidated Network Configuration Domain
//!
//! This module consolidates ALL network-related configuration structs across the BearDog
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

use beardog_errors::BearDogError;
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
pub use client::*;
pub use connection::*;
pub use endpoints::*;
pub use monitoring::*;
pub use performance::*;
pub use security::*;
pub use server::*;

/// **CONSOLIDATED NETWORK CONFIGURATION** - Single source of truth for all network settings
///
/// This structure consolidates all network-related configurations across the BearDog ecosystem,
/// eliminating fragmentation and providing a unified network configuration interface.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ConsolidatedNetworkConfiguration {
    /// **CORE NETWORK SETTINGS**
    pub server: ServerConfiguration,
    pub client: ClientConfiguration,
    pub endpoints: EndpointsConfiguration,

    /// **CONNECTION MANAGEMENT**
    pub connection_pool: ConnectionPoolConfig,
    pub timeouts: TimeoutConfiguration,
    pub load_balancer: LoadBalancerConfiguration,

    /// **SECURITY & ENCRYPTION**
    pub tls: TlsConfiguration,
    pub security: NetworkSecurityConfiguration,
    pub endpoint_security: EndpointSecurityConfiguration,

    /// **PERFORMANCE & OPTIMIZATION**
    pub cache: CacheConfiguration,
    pub performance: NetworkPerformanceConfiguration,
    pub rate_limiting: NetworkRateLimitConfiguration,

    /// **MONITORING & HEALTH**
    pub health_checks: HealthCheckConfiguration,
    pub monitoring: NetworkMonitoringConfiguration,
    pub discovery: ServiceDiscoveryConfiguration,
}

impl Default for ConsolidatedNetworkConfiguration {
    fn default() -> Self {
        Self {
            server: ServerConfiguration::default(),
            client: ClientConfiguration::default(),
            endpoints: EndpointsConfiguration::default(),
            connection_pool: ConnectionPoolConfig::default(),
            timeouts: TimeoutConfiguration::default(),
            load_balancer: LoadBalancerConfiguration::default(),
            tls: TlsConfiguration::default(),
            security: NetworkSecurityConfiguration::default(),
            endpoint_security: EndpointSecurityConfiguration::default(),
            cache: CacheConfiguration::default(),
            performance: NetworkPerformanceConfiguration::default(),
            rate_limiting: NetworkRateLimitConfiguration::default(),
            health_checks: HealthCheckConfiguration::default(),
            monitoring: NetworkMonitoringConfiguration::default(),
            discovery: ServiceDiscoveryConfiguration::default(),
        }
    }
}

impl ConsolidatedNetworkConfiguration {
    /// Validate the network configuration
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
        config.rate_limiting.per_ip_rpm = Some(10000);
        config
            .security
            .ddos_protection
            .max_requests_per_ip_per_minute = 10000;
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
            .unwrap_or(10000);
        config.rate_limiting.global_rps = Some(
            crate::constants::domains::network::defaults::DEFAULT_CONNECTION_TIMEOUT.as_millis()
                as u64,
        );
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
/// This is the single source of truth for rate limiting settings across BearDog.
/// Consolidates all RateLimitConfig variants from:
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
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
pub enum RateLimitStrategy {
    /// Fixed window algorithm (simple, fast, but allows bursts at window boundaries)
    FixedWindow,
    /// Sliding window algorithm (more accurate, smoother rate enforcement)
    SlidingWindow,
    /// Token bucket algorithm (supports controlled bursts)
    TokenBucket,
    /// Leaky bucket algorithm (smooth, constant rate output)
    LeakyBucket,
}

/// Rate limiting scope
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
pub enum RateLimitScope {
    /// Per IP address
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

impl Default for RateLimitStrategy {
    fn default() -> Self {
        Self::TokenBucket
    }
}

impl Default for RateLimitScope {
    fn default() -> Self {
        Self::PerIp
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

            if let Some(burst) = self.burst_size {
                if burst > self.max_requests * 2 {
                    return Err(BearDogError::configuration(
                        "Burst size should not exceed 2x max_requests",
                    ));
                }
            }
        }

        Ok(())
    }
}

/// Backward compatibility alias (deprecated)
#[deprecated(since = "3.1.0", note = "Use RateLimitConfig instead")]
pub type RateLimitConfiguration = RateLimitConfig;

/// Domain-specific type aliases for clarity (all point to canonical)
pub type NetworkRateLimitConfig = RateLimitConfig;
pub type SecurityRateLimitConfig = RateLimitConfig;
pub type WorkflowRateLimitConfig = RateLimitConfig;
pub type MonitoringRateLimitConfig = RateLimitConfig;

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
        assert_eq!(config.tls.enabled, false);
        assert!(config.validate().is_ok());
    }

    #[test]
    fn test_production_config() {
        let config = ConsolidatedNetworkConfiguration::production();
        assert_eq!(config.server.max_connections, 10000);
        assert_eq!(config.tls.enabled, true);
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
