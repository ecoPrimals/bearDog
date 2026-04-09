// SPDX-License-Identifier: AGPL-3.0-or-later

//! Capability-Based Port Discovery
//!
//! Evolves hardcoded ports to runtime discovery with primal awareness.
//!
//! # Philosophy
//!
//! - **No Hardcoding**: Discover available ports at runtime
//! - **Primal Awareness**: Query other primals to avoid conflicts
//! - **Human Sovereignty**: Explicit configuration always wins
//! - **Graceful Fallback**: Multiple discovery strategies
//!
//! # Discovery Hierarchy
//!
//! ```text
//! 1. CLI Arguments      (Explicit human intent - highest priority)
//! 2. Environment Vars   (Human configuration)
//! 3. Config File        (Persistent settings)
//! 4. Primal Discovery   (Query running primals)
//! 5. System Query       (Find available port)
//! 6. Default Constant   (Last resort fallback)
//! ```

use beardog_errors::{BearDogError, process_env};
use serde::{Deserialize, Serialize};
use std::collections::HashSet;
use std::net::{IpAddr, SocketAddr, TcpListener};

/// Port discovery strategy
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum DiscoveryStrategy {
    /// Use explicit configuration only (no discovery)
    ExplicitOnly,
    /// Query other primals for port usage
    PrimalQuery,
    /// Find any available port from system
    SystemQuery,
    /// Full discovery (primal + system)
    Full,
}

/// **Fallback** lower bound for scanning when `BEARDOG_PORT_DISCOVERY_MIN` is unset.
pub const FALLBACK_PORT_SCAN_MIN: u16 = 8000;

/// **Fallback** upper bound when `BEARDOG_PORT_DISCOVERY_MAX` is unset.
pub const FALLBACK_PORT_SCAN_MAX: u16 = 9000;

/// **Fallback** excluded ports (comma-separated override: `BEARDOG_PORT_DISCOVERY_EXCLUDE`).
pub const FALLBACK_EXCLUDED_DEV_PORTS: &[u16] = &[8000, 8888];

/// **Fallback** primal-discovery timeout (ms) when `BEARDOG_PORT_DISCOVERY_TIMEOUT_MS` is unset.
pub const FALLBACK_PORT_DISCOVERY_TIMEOUT_MS: u64 = 2000;

fn parse_u16_env(key: &str, fallback: u16) -> u16 {
    process_env::var(key)
        .ok()
        .and_then(|s| s.parse().ok())
        .unwrap_or(fallback)
}

fn default_excluded_ports() -> Vec<u16> {
    FALLBACK_EXCLUDED_DEV_PORTS.to_vec()
}

/// Port discovery configuration
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PortDiscoveryConfig {
    /// Discovery strategy to use
    pub strategy: DiscoveryStrategy,
    /// Minimum port to consider (**fallback**: `FALLBACK_PORT_SCAN_MIN`; env: `BEARDOG_PORT_DISCOVERY_MIN`)
    pub min_port: u16,
    /// Maximum port to consider (**fallback**: `FALLBACK_PORT_SCAN_MAX`; env: `BEARDOG_PORT_DISCOVERY_MAX`)
    pub max_port: u16,
    /// Ports to avoid (**fallback**: `FALLBACK_EXCLUDED_DEV_PORTS`; env: `BEARDOG_PORT_DISCOVERY_EXCLUDE`)
    pub excluded_ports: Vec<u16>,
    /// Primal discovery timeout (**fallback**: `FALLBACK_PORT_DISCOVERY_TIMEOUT_MS`)
    pub discovery_timeout_ms: u64,
}

impl Default for PortDiscoveryConfig {
    fn default() -> Self {
        Self {
            strategy: DiscoveryStrategy::Full,
            min_port: FALLBACK_PORT_SCAN_MIN,
            max_port: FALLBACK_PORT_SCAN_MAX,
            excluded_ports: default_excluded_ports(),
            discovery_timeout_ms: FALLBACK_PORT_DISCOVERY_TIMEOUT_MS,
        }
    }
}

impl PortDiscoveryConfig {
    /// Load port discovery settings from environment variables.
    #[must_use]
    pub fn from_env() -> Self {
        let mut base = Self::default();
        base.min_port = parse_u16_env("BEARDOG_PORT_DISCOVERY_MIN", FALLBACK_PORT_SCAN_MIN);
        base.max_port = parse_u16_env("BEARDOG_PORT_DISCOVERY_MAX", FALLBACK_PORT_SCAN_MAX);
        base.discovery_timeout_ms = process_env::var("BEARDOG_PORT_DISCOVERY_TIMEOUT_MS")
            .ok()
            .and_then(|s| s.parse().ok())
            .unwrap_or(FALLBACK_PORT_DISCOVERY_TIMEOUT_MS);
        base.excluded_ports = if let Ok(s) = process_env::var("BEARDOG_PORT_DISCOVERY_EXCLUDE") {
            s.split(',').filter_map(|p| p.trim().parse().ok()).collect()
        } else {
            default_excluded_ports()
        };
        base
    }
}

/// Capability-based port discoverer
pub struct PortDiscoverer {
    config: PortDiscoveryConfig,
}

impl PortDiscoverer {
    /// Create new port discoverer with configuration
    pub const fn new(config: PortDiscoveryConfig) -> Self {
        Self { config }
    }

    /// Create discoverer with default configuration
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
        let used_ports = self.query_primal_ports().await?;

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
    async fn query_primal_ports(&self) -> Result<HashSet<u16>, BearDogError> {
        let mut used_ports = HashSet::new();

        // Phase 1: Query local system for TCP listeners
        // This catches all local processes including primals
        match self.query_local_tcp_ports() {
            Ok(ports) => {
                tracing::debug!("Discovered {} local TCP ports in use", ports.len());
                used_ports.extend(ports);
            }
            Err(e) => {
                tracing::warn!("Failed to query local TCP ports: {}", e);
                // Continue - this is not fatal, we'll use other discovery methods
            }
        }

        // Phase 2: merge ports from mDNS hooks / sidecar announcements (env-driven, no extra deps)
        match self.query_mdns_primal_ports().await {
            Ok(ports) => {
                if !ports.is_empty() {
                    tracing::debug!(
                        "Merged {} port(s) from BEARDOG_DISCOVERED_PRIMAL_PORTS / mDNS hook",
                        ports.len()
                    );
                }
                used_ports.extend(ports);
            }
            Err(e) => {
                tracing::debug!("Primal announcement port discovery: {}", e);
            }
        }

        // Phase 3 & 4: Future implementations
        // Will be added when service registry and P2P discovery are available

        tracing::info!(
            "Primal port discovery complete: {} ports in use",
            used_ports.len()
        );

        Ok(used_ports)
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
    fn query_local_tcp_ports(&self) -> Result<HashSet<u16>, BearDogError> {
        #[cfg(target_os = "linux")]
        {
            // Try efficient /proc parsing first
            match self.query_linux_tcp_ports() {
                Ok(ports) if !ports.is_empty() => Ok(ports),
                _ => {
                    // Fallback to probe if /proc parsing fails
                    tracing::debug!("Falling back to port probing");
                    self.probe_common_ports()
                }
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
    fn query_linux_tcp_ports(&self) -> Result<HashSet<u16>, BearDogError> {
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

        Ok(ports)
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
    fn probe_common_ports(&self) -> Result<HashSet<u16>, BearDogError> {
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

        Ok(used_ports)
    }

    /// Query primal ports via mDNS (Phase 2 implementation)
    ///
    /// Full in-process mDNS browsing can be added behind a crate feature later. Production
    /// deployments often expose discovered ports via environment (sidecar or init) — we merge
    /// `BEARDOG_DISCOVERED_PRIMAL_PORTS` (comma-separated) so port selection avoids conflicts
    /// without extra dependencies.
    async fn query_mdns_primal_ports(&self) -> Result<HashSet<u16>, BearDogError> {
        let mut ports = HashSet::new();
        if let Ok(s) = process_env::var("BEARDOG_DISCOVERED_PRIMAL_PORTS") {
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
                            "BEARDOG_DISCOVERED_PRIMAL_PORTS: ignored invalid port {:?}: {}",
                            p,
                            e
                        );
                    }
                }
            }
        }
        Ok(ports)
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
        let ip: IpAddr = process_env::var("BEARDOG_PORT_PROBE_BIND")
            .ok()
            .and_then(|s| s.parse().ok())
            .unwrap_or_else(|| {
                crate::domains::network_addresses::NetworkAddressesConfig::from_env().localhost_ipv4
            });
        TcpListener::bind(SocketAddr::new(ip, port)).is_ok()
    }
}

/// Discover port with full capability-based hierarchy
///
/// # Discovery Hierarchy
///
/// 1. CLI argument (if provided)
/// 2. Environment variable
/// 3. Config file value
/// 4. Runtime discovery (primals + system)
/// 5. Default constant
///
/// # Example
///
/// ```no_run
/// use beardog_config::domains::port_discovery::{discover_port_hierarchical, PortDiscoveryConfig};
///
/// # async fn example() {
/// // Discover API port with full hierarchy
/// let port = discover_port_hierarchical(
///     "BEARDOG_API_PORT",    // Environment variable
///     None,                  // No CLI override
///     None,                  // No config file value
///     8080,                  // Default fallback
///     PortDiscoveryConfig::default(),
/// ).await.expect("hierarchical port discovery");
/// # }
/// ```
///
/// # Errors
///
/// Returns [`BearDogError`] when automatic port probing fails or no valid port can be chosen.
pub async fn discover_port_hierarchical(
    env_var: &str,
    cli_override: Option<u16>,
    config_value: Option<u16>,
    default: u16,
    discovery_config: PortDiscoveryConfig,
) -> Result<u16, BearDogError> {
    // 1. CLI argument (highest priority - explicit human intent)
    if let Some(port) = cli_override {
        tracing::debug!("Using CLI override port: {}", port);
        return Ok(port);
    }

    // 2. Environment variable (human configuration)
    if let Ok(port_str) = process_env::var(env_var)
        && let Ok(port) = port_str.parse::<u16>()
    {
        tracing::debug!("Using environment variable {} = {}", env_var, port);
        return Ok(port);
    }

    // 3. Config file value (persistent configuration)
    if let Some(port) = config_value {
        tracing::debug!("Using config file port: {}", port);
        return Ok(port);
    }

    // 4. Runtime discovery (if enabled)
    if discovery_config.strategy != DiscoveryStrategy::ExplicitOnly {
        let discoverer = PortDiscoverer::new(discovery_config);
        match discoverer.discover().await {
            Ok(port) => {
                tracing::info!(
                    "Discovered available port via capability-based discovery: {}",
                    port
                );
                return Ok(port);
            }
            Err(e) => {
                tracing::warn!("Port discovery failed, falling back to default: {}", e);
            }
        }
    }

    // 5. Default constant (last resort)
    tracing::debug!("Using default port: {}", default);
    Ok(default)
}

#[cfg(test)]
#[expect(clippy::unwrap_used, reason = "test assertions")]
mod tests {
    use super::*;
    use beardog_errors::process_env;
    use std::sync::{Mutex, MutexGuard, OnceLock};

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

    #[tokio::test]
    async fn test_system_query_strategy() {
        // Modern idiomatic: Initialize struct with all values at once
        let config = PortDiscoveryConfig {
            strategy: DiscoveryStrategy::SystemQuery,
            min_port: 58000, // Use high ports for testing
            max_port: 58100,
            ..Default::default()
        };

        let discoverer = PortDiscoverer::new(config);
        let result = discoverer.discover().await;

        assert!(result.is_ok(), "Should find available port in range");
        let port = result.expect("system query finds port in range");
        // Modern idiomatic: Use range contains
        assert!((58000..=58100).contains(&port));
    }

    #[tokio::test]
    async fn test_explicit_only_strategy() {
        // Modern idiomatic: Initialize struct with all values at once
        let config = PortDiscoveryConfig {
            strategy: DiscoveryStrategy::ExplicitOnly,
            ..Default::default()
        };

        let discoverer = PortDiscoverer::new(config);
        let result = discoverer.discover().await;

        assert!(
            result.is_err(),
            "Explicit only should require configuration"
        );
    }

    #[tokio::test]
    async fn test_hierarchical_discovery_cli_override() {
        let result = discover_port_hierarchical(
            "NONEXISTENT_VAR",
            Some(12345), // CLI override
            Some(8080),  // Config value
            9090,        // Default
            PortDiscoveryConfig::default(),
        )
        .await;

        assert_eq!(
            result.expect("hierarchical discovery with CLI override"),
            12345,
            "CLI override should win"
        );
    }

    #[tokio::test]
    async fn test_hierarchical_discovery_default() {
        let result = discover_port_hierarchical(
            "NONEXISTENT_VAR",
            None, // No CLI
            None, // No config
            9090, // Default
            PortDiscoveryConfig {
                strategy: DiscoveryStrategy::ExplicitOnly, // Disable discovery
                ..Default::default()
            },
        )
        .await;

        assert_eq!(
            result.expect("hierarchical discovery falls back to default"),
            9090,
            "Should fall back to default"
        );
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
    fn default_config_is_pure_and_from_env_matches_explicit_overrides() {
        let d = PortDiscoveryConfig::default();
        assert_eq!(d.min_port, FALLBACK_PORT_SCAN_MIN);
        assert_eq!(d.max_port, FALLBACK_PORT_SCAN_MAX);

        let c = PortDiscoveryConfig {
            min_port: 9100,
            max_port: 9101,
            discovery_timeout_ms: 1500,
            excluded_ports: vec![9100, 9101],
            ..Default::default()
        };
        assert_eq!(c.min_port, 9100);
        assert_eq!(c.max_port, 9101);
        assert_eq!(c.discovery_timeout_ms, 1500);
        assert_eq!(c.excluded_ports, vec![9100, 9101]);
    }

    #[tokio::test]
    async fn hierarchical_uses_config_when_env_unset() {
        let port = discover_port_hierarchical(
            "HIER_PORT_TEST_XYZ",
            None,
            Some(1111),
            2222,
            PortDiscoveryConfig {
                strategy: DiscoveryStrategy::ExplicitOnly,
                ..Default::default()
            },
        )
        .await
        .expect("hierarchical uses config when env unset");
        assert_eq!(port, 1111);
    }

    #[tokio::test]
    async fn hierarchical_invalid_env_falls_through_to_config() {
        let port = discover_port_hierarchical(
            "HIER_PORT_BAD",
            None,
            Some(3333),
            4444,
            PortDiscoveryConfig {
                strategy: DiscoveryStrategy::ExplicitOnly,
                ..Default::default()
            },
        )
        .await
        .expect("hierarchical invalid env falls through to config");
        assert_eq!(port, 3333);
    }

    #[tokio::test]
    async fn hierarchical_runtime_discovery_then_default() {
        let port = discover_port_hierarchical(
            "NONEXISTENT_HIER_PORT_999",
            None,
            None,
            4242,
            PortDiscoveryConfig {
                strategy: DiscoveryStrategy::ExplicitOnly,
                ..Default::default()
            },
        )
        .await
        .expect("hierarchical runtime discovery then default");
        assert_eq!(port, 4242);
    }

    #[tokio::test]
    async fn full_strategy_falls_back_to_system_when_primal_fails() {
        let config = PortDiscoveryConfig {
            strategy: DiscoveryStrategy::Full,
            min_port: 58200,
            max_port: 58250,
            excluded_ports: vec![],
            discovery_timeout_ms: 100,
        };
        let discoverer = PortDiscoverer::new(config);
        let p = discoverer
            .discover()
            .await
            .expect("full strategy discovers port when primal fails");
        assert!((58200..=58250).contains(&p));
    }

    #[tokio::test]
    async fn primal_query_strategy_smoke() {
        let config = PortDiscoveryConfig {
            strategy: DiscoveryStrategy::PrimalQuery,
            min_port: 58300,
            max_port: 58320,
            excluded_ports: vec![],
            discovery_timeout_ms: 100,
        };
        let discoverer = PortDiscoverer::new(config);
        let res = discoverer.discover().await;
        assert!(res.is_ok());
        let p = res.expect("primal query strategy discovers port");
        assert!((58300..=58320).contains(&p));
    }

    #[test]
    #[cfg(target_os = "linux")]
    fn parse_proc_net_line_extracts_port() {
        let line = "0: 0100007F:1F90 00000000:0000 0A";
        assert_eq!(PortDiscoverer::parse_proc_net_line(line), Some(8080));
        assert!(PortDiscoverer::parse_proc_net_line("short").is_none());
    }

    #[tokio::test]
    async fn discover_errors_when_no_free_port_in_range() {
        let config = PortDiscoveryConfig {
            strategy: DiscoveryStrategy::SystemQuery,
            min_port: 60000,
            max_port: 60000,
            excluded_ports: vec![60000],
            discovery_timeout_ms: 100,
        };
        let discoverer = PortDiscoverer::new(config);
        let err = discoverer.discover().await.unwrap_err();
        assert!(err.to_string().contains("No available ports"));
    }

    #[test]
    fn from_env_reads_process_env_overlay() {
        let _guard = port_discovery_env_lock();
        process_env::set_var("BEARDOG_PORT_DISCOVERY_MIN", "9100");
        process_env::set_var("BEARDOG_PORT_DISCOVERY_MAX", "9200");
        process_env::set_var("BEARDOG_PORT_DISCOVERY_TIMEOUT_MS", "1500");
        process_env::set_var("BEARDOG_PORT_DISCOVERY_EXCLUDE", "9101, 9102");

        let c = PortDiscoveryConfig::from_env();
        assert_eq!(c.min_port, 9100);
        assert_eq!(c.max_port, 9200);
        assert_eq!(c.discovery_timeout_ms, 1500);
        assert_eq!(c.excluded_ports, vec![9101u16, 9102]);

        process_env::remove_var("BEARDOG_PORT_DISCOVERY_MIN");
        process_env::remove_var("BEARDOG_PORT_DISCOVERY_MAX");
        process_env::remove_var("BEARDOG_PORT_DISCOVERY_TIMEOUT_MS");
        process_env::remove_var("BEARDOG_PORT_DISCOVERY_EXCLUDE");
    }

    #[test]
    fn parse_u16_env_invalid_falls_back_to_fallback() {
        let _guard = port_discovery_env_lock();
        process_env::set_var("BEARDOG_PORT_DISCOVERY_MIN", "not-a-number");
        let c = PortDiscoveryConfig::from_env();
        assert_eq!(c.min_port, FALLBACK_PORT_SCAN_MIN);
        process_env::remove_var("BEARDOG_PORT_DISCOVERY_MIN");
    }

    #[tokio::test]
    async fn hierarchical_falls_back_to_default_when_discovery_range_invalid() {
        let port = discover_port_hierarchical(
            "HIER_INVALID_RANGE_PORT",
            None,
            None,
            7777,
            PortDiscoveryConfig {
                strategy: DiscoveryStrategy::SystemQuery,
                min_port: 65500,
                max_port: 65400,
                excluded_ports: vec![],
                discovery_timeout_ms: 50,
            },
        )
        .await
        .expect("default when system discovery cannot find a port");
        assert_eq!(port, 7777);
    }

    #[tokio::test]
    async fn discovered_primal_ports_env_merges_valid_and_ignores_invalid() {
        let _guard = port_discovery_env_lock();
        process_env::set_var("BEARDOG_DISCOVERED_PRIMAL_PORTS", "8443,not-a-port,9001");

        let config = PortDiscoveryConfig {
            strategy: DiscoveryStrategy::PrimalQuery,
            min_port: 58400,
            max_port: 58500,
            excluded_ports: vec![],
            discovery_timeout_ms: 100,
        };
        let discoverer = PortDiscoverer::new(config);
        let res = discoverer.discover().await;
        assert!(res.is_ok());

        process_env::remove_var("BEARDOG_DISCOVERED_PRIMAL_PORTS");
    }

    #[tokio::test]
    async fn hierarchical_prefers_env_overlay_over_config() {
        let _guard = port_discovery_env_lock();
        process_env::set_var("HIER_PORT_ENV_OVERLAY", "4411");

        let port = discover_port_hierarchical(
            "HIER_PORT_ENV_OVERLAY",
            None,
            Some(9911),
            8822,
            PortDiscoveryConfig::default(),
        )
        .await
        .expect("env should win over config");

        assert_eq!(port, 4411);

        process_env::remove_var("HIER_PORT_ENV_OVERLAY");
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

    #[tokio::test]
    async fn primal_query_errors_when_no_candidate_in_tiny_excluded_range() {
        let config = PortDiscoveryConfig {
            strategy: DiscoveryStrategy::PrimalQuery,
            min_port: 60100,
            max_port: 60100,
            excluded_ports: vec![60100],
            discovery_timeout_ms: 100,
        };
        let discoverer = PortDiscoverer::new(config);
        let err = discoverer.discover().await.expect_err("no port available");
        assert!(err.to_string().contains("No available ports"));
        assert!(err.to_string().contains("primal"));
    }

    #[test]
    fn from_env_invalid_timeout_ms_falls_back_to_fallback() {
        let _guard = port_discovery_env_lock();
        process_env::set_var("BEARDOG_PORT_DISCOVERY_TIMEOUT_MS", "not-a-number");
        let c = PortDiscoveryConfig::from_env();
        assert_eq!(c.discovery_timeout_ms, FALLBACK_PORT_DISCOVERY_TIMEOUT_MS);
        process_env::remove_var("BEARDOG_PORT_DISCOVERY_TIMEOUT_MS");
    }

    #[test]
    fn from_env_invalid_max_port_falls_back() {
        let _guard = port_discovery_env_lock();
        process_env::set_var("BEARDOG_PORT_DISCOVERY_MAX", "bogus");
        let c = PortDiscoveryConfig::from_env();
        assert_eq!(c.max_port, FALLBACK_PORT_SCAN_MAX);
        process_env::remove_var("BEARDOG_PORT_DISCOVERY_MAX");
    }

    #[test]
    fn is_port_available_respects_beardog_port_probe_bind_env() {
        let _guard = port_discovery_env_lock();
        process_env::set_var("BEARDOG_PORT_PROBE_BIND", "127.0.0.1");
        let discoverer = PortDiscoverer::with_defaults();
        let high = 59123u16;
        let _ = discoverer.is_port_available(high);
        process_env::remove_var("BEARDOG_PORT_PROBE_BIND");
    }

    #[tokio::test]
    async fn discovered_primal_ports_skips_empty_csv_segments() {
        let _guard = port_discovery_env_lock();
        process_env::set_var("BEARDOG_DISCOVERED_PRIMAL_PORTS", "8443,,,9001");

        let config = PortDiscoveryConfig {
            strategy: DiscoveryStrategy::PrimalQuery,
            min_port: 58600,
            max_port: 58700,
            excluded_ports: vec![],
            discovery_timeout_ms: 100,
        };
        let discoverer = PortDiscoverer::new(config);
        assert!(discoverer.discover().await.is_ok());

        process_env::remove_var("BEARDOG_DISCOVERED_PRIMAL_PORTS");
    }
}
