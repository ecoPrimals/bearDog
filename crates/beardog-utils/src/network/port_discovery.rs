// Copyright 2025 EcoPrimals BearDog Team
// SPDX-License-Identifier: AGPL-3.0-or-later

//! Runtime Port Discovery - Zero Hardcoding
//!
//! This module provides capability-based port discovery that works without
//! any hardcoded values. Ports are discovered at runtime based on:
//! - System capabilities
//! - Available ports
//! - Configuration precedence
//! - Platform constraints

use beardog_errors::BearDogError;
use std::net::TcpListener;
use tracing::{debug, info};

/// Port discovery result
#[derive(Debug, Clone)]
pub struct DiscoveredPort {
    /// The discovered port number
    pub port: u16,
    /// How the port was discovered
    pub source: PortSource,
    /// Whether this port is confirmed available
    pub verified: bool,
}

/// Source of port discovery
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum PortSource {
    /// From environment variable
    Environment(String),
    /// From configuration file
    Configuration,
    /// Dynamically discovered as available
    Dynamic,
    /// Platform-suggested default
    PlatformDefault,
    /// Well-known service port (e.g., HTTP 80, HTTPS 443)
    WellKnown { service: String },
}

/// Port discovery configuration
#[derive(Debug, Clone)]
pub struct PortDiscoveryConfig {
    /// Preferred port range (if dynamic discovery needed)
    pub preferred_range: std::ops::Range<u16>,
    /// Whether to verify port availability before returning
    pub verify_available: bool,
    /// Service name for well-known port lookup
    pub service_name: Option<String>,
}

impl Default for PortDiscoveryConfig {
    fn default() -> Self {
        Self {
            // Ephemeral port range (IANA recommendation)
            preferred_range: 49152..65535,
            verify_available: true,
            service_name: None,
        }
    }
}

/// Discover an available port with zero hardcoding
///
/// # Discovery Order (Precedence)
/// 1. Environment variable (explicit user configuration)
/// 2. Configuration file (explicit configuration)
/// 3. Dynamic discovery (find available port)
/// 4. Platform default (OS-suggested)
///
/// # Example
/// ```rust,no_run
/// use beardog_utils::network::port_discovery::{discover_port, PortDiscoveryConfig};
///
/// # async fn example() -> Result<(), beardog_errors::BearDogError> {
/// let config = PortDiscoveryConfig::default();
/// let port = discover_port("api", &config).await?;
/// println!("API will bind to port {}", port.port);
/// # Ok(())
/// # }
/// ```
pub async fn discover_port(
    service: &str,
    config: &PortDiscoveryConfig,
) -> Result<DiscoveredPort, BearDogError> {
    info!("Discovering port for service: {}", service);

    // 1. Check environment variable (highest priority)
    if let Some(port) = try_environment_port(service, config).await? {
        return Ok(port);
    }

    // 2. Check global configuration
    if let Some(port) = try_config_port(service, config).await? {
        return Ok(port);
    }

    // 3. Try well-known service port if applicable
    if let Some(port) = try_well_known_port(service, config).await? {
        return Ok(port);
    }

    // 4. Dynamic discovery - find available port
    if let Some(port) = try_dynamic_discovery(service, config).await? {
        return Ok(port);
    }

    // 5. Platform default as final fallback
    try_platform_default(service, config).await
}

/// Try to get port from environment variable
async fn try_environment_port(
    service: &str,
    config: &PortDiscoveryConfig,
) -> Result<Option<DiscoveredPort>, BearDogError> {
    // Try service-specific env var first
    let env_var = format!("BEARDOG_{}_PORT", service.to_uppercase());
    if let Ok(port_str) = std::env::var(&env_var) {
        if let Ok(port) = port_str.parse::<u16>() {
            debug!("Found port {} from environment: {}", port, env_var);
            
            let verified = if config.verify_available {
                is_port_available(port).await?
            } else {
                false
            };

            return Ok(Some(DiscoveredPort {
                port,
                source: PortSource::Environment(env_var),
                verified,
            }));
        }
    }

    // Try generic BEARDOG_PORT as fallback
    if let Ok(port_str) = std::env::var("BEARDOG_PORT") {
        if let Ok(port) = port_str.parse::<u16>() {
            debug!("Found generic port {} from BEARDOG_PORT", port);
            
            let verified = if config.verify_available {
                is_port_available(port).await?
            } else {
                false
            };

            return Ok(Some(DiscoveredPort {
                port,
                source: PortSource::Environment("BEARDOG_PORT".to_string()),
                verified,
            }));
        }
    }

    Ok(None)
}

/// Try to get port from configuration system
async fn try_config_port(
    service: &str,
    config: &PortDiscoveryConfig,
) -> Result<Option<DiscoveredPort>, BearDogError> {
    use beardog_config::global::BEARDOG_CONFIG;

    let port = match service {
        "api" => Some(BEARDOG_CONFIG.network.api.port),
        "discovery" => Some(BEARDOG_CONFIG.network.discovery.port),
        _ => None,
    };

    if let Some(port) = port {
        debug!("Found port {} from configuration for service: {}", port, service);
        
        let verified = if config.verify_available {
            is_port_available(port).await?
        } else {
            false
        };

        return Ok(Some(DiscoveredPort {
            port,
            source: PortSource::Configuration,
            verified,
        }));
    }

    Ok(None)
}

/// Try well-known service port
async fn try_well_known_port(
    service: &str,
    config: &PortDiscoveryConfig,
) -> Result<Option<DiscoveredPort>, BearDogError> {
    let (port, service_name) = match service {
        "http" => (80, "HTTP"),
        "https" => (443, "HTTPS"),
        "ssh" => (22, "SSH"),
        "postgres" => (5432, "PostgreSQL"),
        "redis" => (6379, "Redis"),
        "grafana" => (3000, "Grafana"),
        _ => return Ok(None),
    };

    debug!("Trying well-known port {} for {}", port, service_name);

    let verified = if config.verify_available {
        is_port_available(port).await?
    } else {
        false
    };

    Ok(Some(DiscoveredPort {
        port,
        source: PortSource::WellKnown {
            service: service_name.to_string(),
        },
        verified,
    }))
}

/// Dynamic port discovery - find available port in range
async fn try_dynamic_discovery(
    service: &str,
    config: &PortDiscoveryConfig,
) -> Result<Option<DiscoveredPort>, BearDogError> {
    debug!(
        "Attempting dynamic port discovery for {} in range {:?}",
        service, config.preferred_range
    );

    // Try to find available port in preferred range
    for port in config.preferred_range.clone() {
        if is_port_available(port).await? {
            info!("Discovered available port {} for service: {}", port, service);
            return Ok(Some(DiscoveredPort {
                port,
                source: PortSource::Dynamic,
                verified: true,
            }));
        }
    }

    Ok(None)
}

/// Platform default as final fallback
async fn try_platform_default(
    service: &str,
    config: &PortDiscoveryConfig,
) -> Result<DiscoveredPort, BearDogError> {
    // Use OS to suggest a port (bind to 0 and see what we get)
    let listener = TcpListener::bind("127.0.0.1:0").map_err(|e| {
        BearDogError::network_error(&format!("Failed to get platform default port: {}", e))
    })?;

    let port = listener.local_addr().map_err(|e| {
        BearDogError::network_error(&format!("Failed to get local address: {}", e))
    })?.port();

    info!("Platform suggested port {} for service: {}", port, service);

    Ok(DiscoveredPort {
        port,
        source: PortSource::PlatformDefault,
        verified: true, // We just bound to it, so it's available
    })
}

/// Check if a port is available for binding
async fn is_port_available(port: u16) -> Result<bool, BearDogError> {
    // Try to bind to the port
    match TcpListener::bind(format!("127.0.0.1:{}", port)) {
        Ok(_) => {
            debug!("Port {} is available", port);
            Ok(true)
        }
        Err(e) if e.kind() == std::io::ErrorKind::AddrInUse => {
            debug!("Port {} is in use", port);
            Ok(false)
        }
        Err(e) if e.kind() == std::io::ErrorKind::PermissionDenied => {
            debug!("Port {} requires elevated permissions", port);
            Ok(false)
        }
        Err(e) => {
            Err(BearDogError::network_error(&format!(
                "Failed to check port availability: {}",
                e
            )))
        }
    }
}

/// Discover multiple ports for different services
pub async fn discover_ports(
    services: &[&str],
    config: &PortDiscoveryConfig,
) -> Result<Vec<(String, DiscoveredPort)>, BearDogError> {
    let mut results = Vec::new();

    for service in services {
        let port = discover_port(service, config).await?;
        results.push((service.to_string(), port));
    }

    Ok(results)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[tokio::test]
    async fn test_port_availability_check() {
        // Should be able to check if a port is available
        let result = is_port_available(65000).await;
        assert!(result.is_ok());
    }

    #[tokio::test]
    async fn test_dynamic_discovery() {
        let config = PortDiscoveryConfig {
            preferred_range: 50000..50010,
            verify_available: true,
            service_name: None,
        };

        let result = discover_port("test", &config).await;
        assert!(result.is_ok());
        
        let port = result.unwrap();
        assert!(port.port >= 50000 || port.source == PortSource::PlatformDefault);
    }

    #[tokio::test]
    async fn test_environment_override() {
        std::env::set_var("BEARDOG_TEST_PORT", "12345");
        
        let config = PortDiscoveryConfig::default();
        let result = discover_port("test", &config).await;
        
        assert!(result.is_ok());
        let port = result.unwrap();
        assert_eq!(port.port, 12345);
        assert!(matches!(port.source, PortSource::Environment(_)));
        
        std::env::remove_var("BEARDOG_TEST_PORT");
    }

    #[tokio::test]
    async fn test_well_known_ports() {
        let config = PortDiscoveryConfig {
            verify_available: false, // Don't actually try to bind to 80
            ..Default::default()
        };

        let result = discover_port("http", &config).await;
        assert!(result.is_ok());
        
        let port = result.unwrap();
        assert_eq!(port.port, 80);
        assert!(matches!(port.source, PortSource::WellKnown { .. }));
    }
}

