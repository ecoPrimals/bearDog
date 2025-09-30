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

// Import all modular components
pub mod server;
pub mod client;
pub mod endpoints;
pub mod connection;
pub mod security;
pub mod performance;
pub mod monitoring;

// Re-export all types for backward compatibility
pub use server::*;
pub use client::*;
pub use endpoints::*;
pub use connection::*;
pub use security::*;
pub use performance::*;
pub use monitoring::*;

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
    pub connection_pool: ConnectionPoolConfiguration,
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
            connection_pool: ConnectionPoolConfiguration::default(),
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
            return Err(BearDogError::configuration("Pool max size cannot be less than min size"));
        }
        
        // Validate rate limiting
        if self.rate_limiting.enabled && self.rate_limiting.burst_size == 0 {
            return Err(BearDogError::configuration("Rate limit burst size cannot be zero when enabled"));
        }
        
        Ok(())
    }
    
    /// Create a development-friendly configuration
    pub fn development() -> Self {
        let mut config = Self::default();
        
        // Relaxed settings for development
        config.server.max_connections = beardog_types::constants::domains::system::defaults::DEFAULT_QUEUE_SIZE;
        config.rate_limiting.global_rps = Some(100000);
        config.rate_limiting.per_ip_rpm = Some(10000);
        config.security.ddos_protection.max_requests_per_ip_per_minute = 10000;
        config.tls.enabled = false; // Disable TLS for easier development
        config.endpoint_security.enable_authentication = false;
        
        config
    }
    
    /// Create a production-hardened configuration
    pub fn production() -> Self {
        let mut config = Self::default();
        
        // Strict settings for production
        config.server.max_connections = 10000;
        config.rate_limiting.global_rps = Some(beardog_types::constants::domains::network::defaults::DEFAULT_CONNECTION_TIMEOUT.as_millis() as u64);
        config.rate_limiting.per_ip_rpm = Some(100);
        config.security.ddos_protection.max_requests_per_ip_per_minute = 100;
        config.security.ddos_protection.enable_challenge_response = true;
        config.tls.enabled = true;
        config.tls.verification_mode = TlsVerificationMode::Full;
        config.endpoint_security.enable_authentication = true;
        config.endpoint_security.enable_authorization = true;
        
        config
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
        assert_eq!(config.server.max_connections, beardog_types::constants::domains::system::defaults::DEFAULT_QUEUE_SIZE);
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
        config.connection_pool.min_size = beardog_types::constants::domains::system::defaults::DEFAULT_POOL_SIZE;
        assert!(config.validate().is_err());
    }
} 