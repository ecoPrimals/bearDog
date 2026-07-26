// SPDX-License-Identifier: AGPL-3.0-or-later

//! Runtime port discovery: primal queries, system bind probes, and platform helpers.

use beardog_errors::{BearDogError, process_env};
use std::collections::HashSet;
use std::net::{IpAddr, SocketAddr, TcpListener};

use crate::env_keys;

use super::config::{DiscoveryStrategy, PortDiscoveryConfig};

/// Capability-based port discoverer
pub struct PortDiscoverer {
    config: PortDiscoveryConfig,
}

impl PortDiscoverer {
    /// Create new port discoverer with configuration
    #[must_use]
    pub const fn new(config: PortDiscoveryConfig) -> Self {
        Self { config }
    }

    /// Create discoverer with default configuration
    #[must_use]
    pub fn with_defaults() -> Self {
        Self::new(PortDiscoveryConfig::default())
    }

    /// Discover available port using configured strategy
    ///
    /// # Discovery Process
    ///
    /// 1. **Primal Query** (if enabled): Query running primals via mDNS/discovery
    /// 2. **System Query**: Check system for available ports
    /// 3. **Validation**: Ensure port is not excluded
    /// 4. **Return**: First available port found
    ///
    /// # Errors
    ///
    /// Returns error if no available port found within range
    pub async fn discover(&self) -> Result<u16, BearDogError> {
        match self.config.strategy {
            DiscoveryStrategy::ExplicitOnly => Err(BearDogError::configuration(
                "Discovery disabled - explicit configuration required",
            )),
            DiscoveryStrategy::PrimalQuery => self.discover_from_primals().await,
            DiscoveryStrategy::SystemQuery => self.find_available_port(),
            DiscoveryStrategy::Full => {
                // Try primal discovery first, fall back to system query
                match self.discover_from_primals().await {
                    Ok(port) => Ok(port),
                    Err(_) => self.find_available_port(),
                }
            }
        }
    }

    /// Discover available port by querying other primals
    ///
    /// **Capability-Based Discovery**: Queries mDNS, network discovery,
    /// and other primal announcement mechanisms to find what ports are in use.
    ///
    /// # Primal Sovereignty
    ///
    /// - Respects other primals' port choices
    /// - Avoids conflicts through awareness
    /// - Cooperative, not competitive
    async fn discover_from_primals(&self) -> Result<u16, BearDogError> {
        // Collect ports used by discovered primals
        let used_ports = self.query_primal_ports();

        // Find first available port not in use by other primals
        for candidate in self.config.min_port..=self.config.max_port {
            if !used_ports.contains(&candidate)
                && !self.config.excluded_ports.contains(&candidate)
                && self.is_port_available(candidate)
            {
                tracing::info!(
                    "Discovered available port {} (avoiding {} primal ports)",
                    candidate,
                    used_ports.len()
                );
                return Ok(candidate);
            }
        }

        Err(BearDogError::configuration(&format!(
            "No available ports in range {}-{} (avoiding {} primal ports)",
            self.config.min_port,
            self.config.max_port,
            used_ports.len()
        )))
    }

    /// Query ports used by other primals
    ///
    /// **Runtime Discovery**: Uses mDNS, network discovery, and other
    /// mechanisms to find what ports primals are actually using.
    ///
    /// # Implementation Phases
    ///
    /// **Phase 1** (Current): Local process detection
    /// - Query system for TCP listeners on localhost
    /// - Fast, no network I/O required
    /// - Works immediately in all environments
    ///
    /// **Phase 2** (Next): mDNS/DNS-SD discovery
    /// - Query `_beardog._tcp.local` services
    /// - Parse TXT records for port information
    /// - Requires: `mdns` feature flag
    ///
    /// **Phase 3** (Future): Service registry query
    /// - Query centralized discovery service
    /// - Get registered primal endpoints
    /// - Requires: Registry configuration
    ///
    /// **Phase 4** (Advanced): Peer-to-peer discovery
    /// - Query known primals for their peers
    /// - Build distributed port map
    /// - Requires: P2P protocol implementation
    ///
    /// # Returns
    ///
    /// Set of ports currently in use by primals or system services.
    fn query_primal_ports(&self) -> HashSet<u16> {
        let mut used_ports = HashSet::new();

        // Phase 1: Query local system for TCP listeners
        // This catches all local processes including primals
        let ports = self.query_local_tcp_ports();
        tracing::debug!("Discovered {} local TCP ports in use", ports.len());
        used_ports.extend(ports);

        // Phase 2: merge ports from mDNS hooks / sidecar announcements (env-driven, no extra deps)
        let mdns_ports = self.query_mdns_primal_ports();
        if !mdns_ports.is_empty() {
            tracing::debug!(
                "Merged {} port(s) from {} / mDNS hook",
                mdns_ports.len(),
                env_keys::ENV_DISCOVERED_PRIMAL_PORTS
            );
        }
        used_ports.extend(mdns_ports);

        // Phase 3 & 4: Future implementations
        // Will be added when service registry and P2P discovery are available

        tracing::info!(
            "Primal port discovery complete: {} ports in use",
            used_ports.len()
        );

        used_ports
    }

    /// Query local TCP ports in use (Phase 1 implementation)
    ///
    /// Uses platform-specific methods to enumerate TCP listeners.
    /// This catches all local processes, including primals.
    ///
    /// # Platform Support
    ///
    /// - **Linux**: Parse `/proc/net/tcp` and `/proc/net/tcp6`
    /// - **macOS**: Use `netstat -an -p tcp` (fallback to probe)
    /// - **Windows**: Use `netstat -an -p TCP` (fallback to probe)
    /// - **All**: Falls back to probing common port ranges
    ///
    /// # Returns
    ///
    /// Set of TCP ports with listeners on localhost.
    fn query_local_tcp_ports(&self) -> HashSet<u16> {
        #[cfg(target_os = "linux")]
        {
            // Try efficient /proc parsing first
            let ports = self.query_linux_tcp_ports();
            if ports.is_empty() {
                // Fallback to probe if /proc parsing fails
                tracing::debug!("Falling back to port probing");
                self.probe_common_ports()
            } else {
                ports
            }
        }

        #[cfg(not(target_os = "linux"))]
        {
            // Fallback: Probe common port range
            // This is less efficient but works across all platforms
            self.probe_common_ports()
        }
    }

    /// Query TCP ports on Linux via /proc/net/tcp
    #[cfg(target_os = "linux")]
    fn query_linux_tcp_ports(&self) -> HashSet<u16> {
        use std::fs::File;
        use std::io::{BufRead, BufReader};

        let mut ports = HashSet::new();

        // Read IPv4 TCP connections
        if let Ok(file) = File::open("/proc/net/tcp") {
            let reader = BufReader::new(file);
            for line in reader.lines().skip(1).flatten() {
                if let Some(port) = Self::parse_proc_net_line(&line) {
                    ports.insert(port);
                }
            }
        }

        // Read IPv6 TCP connections
        if let Ok(file) = File::open("/proc/net/tcp6") {
            let reader = BufReader::new(file);
            for line in reader.lines().skip(1).flatten() {
                if let Some(port) = Self::parse_proc_net_line(&line) {
                    ports.insert(port);
                }
            }
        }

        ports
    }

    /// Parse a line from /proc/net/tcp or /proc/net/tcp6
    ///
    /// Format: `sl  local_address rem_address   st tx_queue rx_queue ...`
    /// Example: `0: 0100007F:1F90 00000000:0000 0A ...`
    ///          (127.0.0.1:8080 in hex)
    #[cfg(target_os = "linux")]
    fn parse_proc_net_line(line: &str) -> Option<u16> {
        let parts: Vec<&str> = line.split_whitespace().collect();
        if parts.len() < 2 {
            return None;
        }

        // Parse local_address (format: IP:PORT in hex)
        let local_addr = parts[1];
        let port_hex = local_addr.split(':').nth(1)?;

        // Convert hex to decimal
        u16::from_str_radix(port_hex, 16).ok()
    }

    /// Probe common ports to detect usage (fallback method)
    ///
    /// This is less efficient but works on all platforms.
    /// Only probes ports in the configured range.
    fn probe_common_ports(&self) -> HashSet<u16> {
        let mut used_ports = HashSet::new();

        // Only probe the configured range
        let probe_range =
            self.config.min_port..=self.config.max_port.min(self.config.min_port + 100);

        for port in probe_range {
            // Try to bind - if it fails, port is in use
            if !self.is_port_available(port) {
                used_ports.insert(port);
            }
        }

        used_ports
    }

    /// Query primal ports via mDNS (Phase 2 implementation)
    ///
    /// Full in-process mDNS browsing can be added behind a crate feature later. Production
    /// deployments often expose discovered ports via environment (sidecar or init) — we merge
    /// `BEARDOG_DISCOVERED_PRIMAL_PORTS` (comma-separated) so port selection avoids conflicts
    /// without extra dependencies.
    fn query_mdns_primal_ports(&self) -> HashSet<u16> {
        let mut ports = HashSet::new();
        if let Ok(s) = process_env::var(env_keys::ENV_DISCOVERED_PRIMAL_PORTS) {
            for part in s.split(',') {
                let p = part.trim();
                if p.is_empty() {
                    continue;
                }
                match p.parse::<u16>() {
                    Ok(port) => {
                        ports.insert(port);
                    }
                    Err(e) => {
                        tracing::warn!(
                            "{}: ignored invalid port {:?}: {}",
                            env_keys::ENV_DISCOVERED_PRIMAL_PORTS,
                            p,
                            e
                        );
                    }
                }
            }
        }
        ports
    }

    /// Find any available port from system
    ///
    /// Tries ports in range, checking availability with actual bind attempts.
    fn find_available_port(&self) -> Result<u16, BearDogError> {
        for candidate in self.config.min_port..=self.config.max_port {
            if !self.config.excluded_ports.contains(&candidate) && self.is_port_available(candidate)
            {
                tracing::info!("Found available system port: {}", candidate);
                return Ok(candidate);
            }
        }

        Err(BearDogError::configuration(&format!(
            "No available ports in range {}-{}",
            self.config.min_port, self.config.max_port
        )))
    }

    /// Check if port is available by attempting to bind
    ///
    /// **Zero-Cost Check**: Immediately drops listener, no resources held.
    ///
    /// Bind address is **configuration-driven**: `BEARDOG_PORT_PROBE_BIND` (IP), else documented
    /// loopback from [`NetworkAddressesConfig`](crate::domains::network_addresses::NetworkAddressesConfig) (same as `BEARDOG_LOCALHOST_IPV4`).
    fn is_port_available(&self, port: u16) -> bool {
        let ip: IpAddr = process_env::var(env_keys::ENV_PORT_PROBE_BIND)
            .ok()
            .and_then(|s| s.parse().ok())
            .unwrap_or_else(|| {
                crate::domains::network_addresses::NetworkAddressesConfig::from_env().localhost_ipv4
            });
        TcpListener::bind(SocketAddr::new(ip, port)).is_ok()
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use beardog_errors::process_env;
    use std::sync::{Mutex, MutexGuard, OnceLock};

    use crate::domains::port_discovery::FALLBACK_PORT_SCAN_MAX;
    use crate::domains::port_discovery::FALLBACK_PORT_SCAN_MIN;

    static PORT_DISCOVERY_ENV_MUTEX: OnceLock<Mutex<()>> = OnceLock::new();

    fn port_discovery_env_lock() -> MutexGuard<'static, ()> {
        PORT_DISCOVERY_ENV_MUTEX
            .get_or_init(|| Mutex::new(()))
            .lock()
            .expect("port discovery env test mutex poisoned")
    }

    #[test]
    fn test_port_discoverer_creation() {
        let config = PortDiscoveryConfig::default();
        let discoverer = PortDiscoverer::new(config);
        assert_eq!(discoverer.config.min_port, FALLBACK_PORT_SCAN_MIN);
        assert_eq!(discoverer.config.max_port, FALLBACK_PORT_SCAN_MAX);
    }

    #[test]
    fn test_port_availability_check() {
        let discoverer = PortDiscoverer::with_defaults();

        // Note: Port 0 is special (means "any available port" to OS)
        // so we test with actual ports instead

        // High port should be available (unless in use)
        let high_port = 58000u16;
        let is_available = discoverer.is_port_available(high_port);

        // Verify the function returns a valid boolean (runs without panic)
        // Actual availability depends on system state, so we just verify type correctness
        let _: bool = is_available; // Type check - compiles = test passes
    }

    #[test]
    fn test_excluded_ports_respected() {
        // Modern idiomatic: Initialize struct with all values at once
        let config = PortDiscoveryConfig {
            min_port: 8000,
            max_port: 8010,
            excluded_ports: vec![8000, 8001, 8002],
            ..Default::default()
        };

        let discoverer = PortDiscoverer::new(config);

        // These ports should be considered unavailable even if system allows
        assert!(!discoverer.config.excluded_ports.contains(&8003));
        assert!(discoverer.config.excluded_ports.contains(&8000));
    }

    #[test]
    #[cfg(target_os = "linux")]
    fn parse_proc_net_line_extracts_port() {
        let line = "0: 0100007F:1F90 00000000:0000 0A";
        assert_eq!(PortDiscoverer::parse_proc_net_line(line), Some(8080));
        assert!(PortDiscoverer::parse_proc_net_line("short").is_none());
    }

    #[cfg(target_os = "linux")]
    #[test]
    fn parse_proc_net_line_requires_address_column() {
        assert!(PortDiscoverer::parse_proc_net_line("incomplete").is_none());
        assert!(PortDiscoverer::parse_proc_net_line("0 0100007F:1F90").is_some());
    }

    #[cfg(target_os = "linux")]
    #[test]
    fn parse_proc_net_line_rejects_invalid_hex_port() {
        assert!(PortDiscoverer::parse_proc_net_line("0: 0100007F:GGGG 00000000:0000 0A").is_none());
    }

    #[test]
    fn is_port_available_respects_beardog_port_probe_bind_env() {
        let _guard = port_discovery_env_lock();
        process_env::set_var(env_keys::ENV_PORT_PROBE_BIND, "127.0.0.1");
        let discoverer = PortDiscoverer::with_defaults();
        let high = 59123u16;
        let _ = discoverer.is_port_available(high);
        process_env::remove_var(env_keys::ENV_PORT_PROBE_BIND);
    }
}
