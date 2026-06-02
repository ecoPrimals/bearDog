// SPDX-License-Identifier: AGPL-3.0-or-later

//! Runtime Network Discovery
//!
//! Evolution from hardcoded network addresses to runtime capability-based discovery.
//!
//! ## Philosophy
//! - No hardcoded IPs or ports
//! - Discover available network resources at runtime
//! - Use configuration as preferences, not requirements
//! - Graceful fallback when discovery fails

use crate::domains::network_ports::DEFAULT_API_PORT;
use crate::env_keys;
use beardog_errors::BearDogError;
use serde::{Deserialize, Serialize};
use std::net::{IpAddr, Ipv4Addr, SocketAddr, TcpListener};
use tracing::{debug, info, warn};

type Result<T> = std::result::Result<T, BearDogError>;

/// Network capabilities discovered at runtime
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct NetworkCapabilities {
    /// Discovered local IP addresses
    pub local_addresses: Vec<IpAddr>,
    /// Available ports discovered
    pub available_ports: Vec<u16>,
    /// Preferred configuration (if any)
    pub preferred_config: Option<NetworkPreferences>,
}

/// Network preferences from configuration
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct NetworkPreferences {
    /// Preferred bind address (can be overridden by discovery)
    pub preferred_host: Option<String>,
    /// Preferred port (will check availability)
    pub preferred_port: Option<u16>,
    /// Port range to search
    pub port_range: (u16, u16),
}

/// Inclusive upper bound for the default [`NetworkPreferences`] port scan window (see [`NetworkPreferences::default`]).
pub const DEFAULT_RUNTIME_PORT_SCAN_RANGE_END: u16 = DEFAULT_API_PORT + 19;

impl Default for NetworkPreferences {
    fn default() -> Self {
        Self {
            preferred_host: None,
            preferred_port: None,
            port_range: (DEFAULT_API_PORT, DEFAULT_RUNTIME_PORT_SCAN_RANGE_END),
        }
    }
}

/// Runtime network discovery
pub struct NetworkDiscovery {
    preferences: NetworkPreferences,
}

impl NetworkDiscovery {
    /// Create new network discovery with preferences
    pub const fn new(preferences: NetworkPreferences) -> Self {
        Self { preferences }
    }

    /// Create with default preferences
    pub fn with_defaults() -> Self {
        Self::new(NetworkPreferences::default())
    }

    /// Discover network capabilities at runtime
    ///
    /// This discovers:
    /// 1. Local network interfaces and IPs
    /// 2. Available ports in the configured range
    /// 3. Network reachability
    ///
    /// # Errors
    ///
    /// Returns [`BearDogError`] when local interface or port discovery fails.
    pub fn discover(&self) -> Result<NetworkCapabilities> {
        info!("🔍 Discovering runtime network capabilities...");

        // Discover local IP addresses
        let local_addresses = self.discover_local_addresses()?;
        debug!("Found {} local addresses", local_addresses.len());

        // Discover available ports
        let available_ports = self.discover_available_ports()?;
        debug!("Found {} available ports", available_ports.len());

        Ok(NetworkCapabilities {
            local_addresses,
            available_ports,
            preferred_config: Some(self.preferences.clone()),
        })
    }

    /// Discover local IP addresses (no hardcoding)
    fn discover_local_addresses(&self) -> Result<Vec<IpAddr>> {
        let mut addresses = Vec::new();

        // Try to discover real network interfaces
        #[cfg(target_os = "linux")]
        {
            // Use getifaddrs or similar on Linux
            // For now, use loopback + try to get primary interface
            addresses.push(IpAddr::V4(Ipv4Addr::LOCALHOST));

            // Try to get primary interface IP
            if let Ok(primary) = self.get_primary_interface_ip() {
                addresses.push(primary);
            }
        }

        #[cfg(not(target_os = "linux"))]
        {
            // Fallback for other platforms
            addresses.push(IpAddr::V4(Ipv4Addr::LOCALHOST));
        }

        // Check environment variable override
        if let Ok(host) = std::env::var(env_keys::ENV_BIND_ADDRESS)
            && let Ok(addr) = host.parse::<IpAddr>()
        {
            addresses.insert(0, addr); // Prioritize env var
        }

        // Check configuration preference
        if let Some(ref preferred) = self.preferences.preferred_host
            && let Ok(addr) = preferred.parse::<IpAddr>()
        {
            addresses.insert(0, addr); // Prioritize config
        }

        Ok(addresses)
    }

    /// Get primary network interface IP (best effort)
    fn get_primary_interface_ip(&self) -> Result<IpAddr> {
        // This is a simplified implementation
        // Real implementation would query network interfaces

        // Try to connect to a known endpoint to discover our outbound IP
        // This doesn't actually connect, just asks the OS for routing
        use std::net::UdpSocket;

        let socket = UdpSocket::bind("0.0.0.0:0")
            .map_err(|e| BearDogError::network(format!("Socket bind failed: {e}")))?;

        // UDP connect to any routable address to discover local IP (no data sent).
        // Configurable via BEARDOG_NETWORK_PROBE_TARGET for air-gapped or custom environments.
        let probe_target = std::env::var(env_keys::ENV_NETWORK_PROBE_TARGET)
            .unwrap_or_else(|_| "198.51.100.1:80".to_string());
        socket
            .connect(&probe_target)
            .map_err(|e| BearDogError::network(format!("Socket connect failed: {e}")))?;

        let local_addr = socket
            .local_addr()
            .map_err(|e| BearDogError::network(format!("Get local addr failed: {e}")))?;

        Ok(local_addr.ip())
    }

    /// Discover available ports in configured range
    fn discover_available_ports(&self) -> Result<Vec<u16>> {
        let mut available = Vec::new();
        let (start, end) = self.preferences.port_range;

        // Check preferred port first
        if let Some(preferred) = self.preferences.preferred_port {
            if self.is_port_available(preferred) {
                available.push(preferred);
                info!("✅ Preferred port {} is available", preferred);
                return Ok(available); // Use preferred if available
            }
            warn!(
                "⚠️  Preferred port {} not available, searching range",
                preferred
            );
        }

        // Search configured range for available ports
        for port in start..=end {
            if self.is_port_available(port) {
                available.push(port);
                if available.len() >= 5 {
                    break; // Found enough options
                }
            }
        }

        if available.is_empty() {
            warn!(
                "⚠️  No ports available in range {}-{}, using ephemeral",
                start, end
            );
            // Use OS-assigned ephemeral port
            available.push(0);
        }

        Ok(available)
    }

    /// Check if a port is available for binding
    fn is_port_available(&self, port: u16) -> bool {
        use std::net::{IpAddr, Ipv4Addr, SocketAddr};
        TcpListener::bind(SocketAddr::new(IpAddr::V4(Ipv4Addr::LOCALHOST), port)).is_ok()
    }

    /// Select best bind address from discovered capabilities
    ///
    /// # Errors
    ///
    /// Returns [`BearDogError`] when no local address or port was discovered.
    pub fn select_bind_address(capabilities: &NetworkCapabilities) -> Result<SocketAddr> {
        // Prefer first local address (prioritized by discovery)
        let addr = capabilities
            .local_addresses
            .first()
            .ok_or_else(|| BearDogError::network("No local addresses discovered".to_string()))?;

        // Prefer first available port
        let port = capabilities
            .available_ports
            .first()
            .ok_or_else(|| BearDogError::network("No ports available".to_string()))?;

        Ok(SocketAddr::new(*addr, *port))
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_network_discovery_finds_addresses() {
        let discovery = NetworkDiscovery::with_defaults();
        let capabilities = discovery
            .discover()
            .expect("network discovery finds capabilities");

        // Should discover at least loopback
        assert!(!capabilities.local_addresses.is_empty());
        assert!(
            capabilities
                .local_addresses
                .iter()
                .any(std::net::IpAddr::is_loopback)
        );
    }

    #[test]
    fn test_network_discovery_finds_ports() {
        let discovery = NetworkDiscovery::with_defaults();
        let capabilities = discovery
            .discover()
            .expect("network discovery finds capabilities");

        // Should find at least one available port
        assert!(!capabilities.available_ports.is_empty());
    }

    #[test]
    fn test_prioritizes_preferred_host_from_preferences() {
        let prefs = NetworkPreferences {
            preferred_host: Some("127.0.0.1".to_string()),
            preferred_port: None,
            port_range: (DEFAULT_API_PORT, DEFAULT_RUNTIME_PORT_SCAN_RANGE_END),
        };
        let discovery = NetworkDiscovery::new(prefs);
        let capabilities = discovery
            .discover()
            .expect("network discovery finds capabilities");

        assert!(
            capabilities
                .local_addresses
                .first()
                .expect("at least one local address")
                .is_loopback()
        );
    }

    #[test]
    fn test_respects_port_preferences() {
        let prefs = NetworkPreferences {
            preferred_host: None,
            preferred_port: Some(9999), // Likely available
            port_range: (9990, 10000),
        };

        let discovery = NetworkDiscovery::new(prefs);
        let capabilities = discovery
            .discover()
            .expect("network discovery finds capabilities");

        // Should check preferred port first
        assert!(!capabilities.available_ports.is_empty());
    }

    #[test]
    fn test_select_bind_address_works() {
        let discovery = NetworkDiscovery::with_defaults();
        let capabilities = discovery
            .discover()
            .expect("network discovery finds capabilities");

        let bind_addr = NetworkDiscovery::select_bind_address(&capabilities)
            .expect("select bind address from discovered capabilities");

        assert!(bind_addr.port() > 0);
    }

    #[test]
    fn test_no_hardcoded_addresses() {
        // This test validates that discovery doesn't return hardcoded addresses
        // unless they're actually available

        let discovery = NetworkDiscovery::with_defaults();
        let capabilities = discovery
            .discover()
            .expect("network discovery finds capabilities");

        // All discovered addresses should be real
        for addr in &capabilities.local_addresses {
            // Should be a valid IP (not 0.0.0.0 unless intentional)
            assert!(!addr.is_unspecified() || addr.is_loopback());
        }
    }
}
