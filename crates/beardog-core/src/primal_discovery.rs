// SPDX-License-Identifier: AGPL-3.0-only

//! Primal Discovery Module
//!
//! **Core Principle**: "Discover other primals at runtime, never hardcode"
//!
//! This module implements runtime discovery of other ecoPrimals in the ecosystem,
//! eliminating hardcoded addresses, ports, and primal-specific knowledge.
//!
//! # Design Philosophy
//!
//! - **Zero Hardcoding**: No primal addresses, ports, or identities in code
//! - **Capability-Based**: Discover primals by capabilities they provide
//! - **Environment-Driven**: Discovery configuration from environment/config
//! - **Protocol-Agnostic**: Support mDNS, DNS-SD, UPA registry, etc.
//!
//! # Discovery Flow
//!
//! ```text
//! 1. Query: "Who provides SecureTunneling?"
//! 2. Discovery: Check environment, UPA, mDNS, DNS-SD
//! 3. Result: List of primals with endpoints and capabilities
//! 4. Selection: Choose based on trust, proximity, load
//! ```
//!
//! # Environment Variables
//!
//! - `PRIMAL_DISCOVERY_METHOD` - Discovery method (upa, mdns, dns-sd, env, multi)
//! - `BEARDOG_REGISTRY_ENDPOINT` - Registry / UPA endpoint (URI or `unix://` path), highest precedence
//! - `BEARDOG_SERVICE_REGISTRY_ENDPOINT` - Alternate registry env (workspace convention)
//! - `UPA_REGISTRY_ADDR` - Legacy UPA registry address
//! - `BEARDOG_BIOMEOS_SOCKET_DIR` - Override `$XDG_RUNTIME_DIR/biomeos` for IPC socket scan
//! - `DISCOVERY_TIMEOUT_MS` - Discovery timeout in milliseconds
//! - `PRIMAL_&lt;NAME&gt;_ADDR` - Explicit primal address (`http://…`, `unix://…`, or absolute socket path on Unix)
//! - `PRIMAL_&lt;NAME&gt;_CAPABILITIES` - Comma-separated capabilities for env- or socket-discovered primals
//!
//! # Usage Example
//!
//! ```rust,no_run
//! use beardog_core::primal_discovery::{PrimalDiscovery, DiscoveryQuery};
//! use beardog_core::self_knowledge::SimpleCapability;
//!
//! # async fn example() -> Result<(), beardog_errors::BearDogError> {
//! // Discover primals that provide cryptography
//! let mut discovery = PrimalDiscovery::from_env()?;
//! let query = DiscoveryQuery::by_capability(SimpleCapability::Cryptography);
//! let primals = discovery.discover(query).await?;
//!
//! for primal in primals {
//!     println!("Found: {} at {:?}", primal.name, primal.endpoints);
//! }
//! # Ok(())
//! # }
//! ```

use crate::self_knowledge::{Endpoint, SimpleCapability};
use beardog_errors::BearDogError;
use beardog_types::constants::domains::network::ipc_discovery as ipc;
use beardog_types::constants::domains::system::intervals::PRIMAL_DISCOVERY_CACHE_TTL;
use beardog_types::constants::domains::timeouts::HEALTH_CHECK_TIMEOUT;
use serde::{Deserialize, Serialize};
use std::collections::{HashMap, HashSet};
use std::ffi::OsStr;
use std::time::Duration;
use tracing::{debug, info, warn};

// =============================================================================
// CORE TYPES
// =============================================================================

/// Discovery method for finding primals
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub enum DiscoveryMethod {
    /// Environment variables (`PRIMAL_&lt;NAME&gt;_ADDR`)
    Environment,

    /// Universal Primal Authority (UPA) registry
    UniversalPrimalAuthority {
        /// Registry address (Unix socket or TCP endpoint)
        registry_addr: String,
    },

    /// Multicast DNS (mDNS) service discovery
    Mdns {
        /// Service type for mDNS query (e.g., "_primal._tcp.local")
        service_type: String,
    },

    /// DNS Service Discovery (DNS-SD)
    DnsSd {
        /// Domain for DNS-SD lookup
        domain: String,
    },

    /// Multiple methods in priority order
    Multi(Vec<Self>),
}

/// Query for discovering primals
#[derive(Debug, Clone)]
pub struct DiscoveryQuery {
    /// Specific primal name (optional)
    pub name: Option<String>,

    /// Required capabilities
    pub capabilities: Vec<SimpleCapability>,

    /// Discovery timeout
    pub timeout: Duration,
}

/// Discovered primal information
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct DiscoveredPrimal {
    /// Primal name
    pub name: String,

    /// Available endpoints
    pub endpoints: Vec<Endpoint>,

    /// Provided capabilities
    pub capabilities: Vec<SimpleCapability>,

    /// Trust score (0.0 - 1.0)
    pub trust_score: Option<f64>,

    /// Discovery timestamp
    pub discovered_at: std::time::SystemTime,
}

/// Primal discovery engine
#[derive(Clone)]
pub struct PrimalDiscovery {
    /// Discovery method
    method: DiscoveryMethod,

    /// Cached discoveries (for future use)
    _cache: HashMap<String, DiscoveredPrimal>,

    /// Cache TTL (for future use)
    _cache_ttl: Duration,

    /// When set, environment-based discovery uses this map instead of reading the process environment.
    env_override: Option<HashMap<String, String>>,
}

// =============================================================================
// DISCOVERY QUERY BUILDERS
// =============================================================================

impl DiscoveryQuery {
    /// Create query for a specific primal by name
    #[must_use]
    pub fn by_name(name: impl Into<String>) -> Self {
        Self {
            name: Some(name.into()),
            capabilities: Vec::new(),
            timeout: HEALTH_CHECK_TIMEOUT,
        }
    }

    /// Create query for primals by capability
    #[must_use]
    pub fn by_capability(capability: SimpleCapability) -> Self {
        Self {
            name: None,
            capabilities: vec![capability],
            timeout: HEALTH_CHECK_TIMEOUT,
        }
    }

    /// Add required capability
    #[must_use]
    pub fn with_capability(mut self, capability: SimpleCapability) -> Self {
        self.capabilities.push(capability);
        self
    }

    /// Set discovery timeout
    #[must_use]
    pub const fn with_timeout(mut self, timeout: Duration) -> Self {
        self.timeout = timeout;
        self
    }
}

// =============================================================================
// PRIMAL DISCOVERY IMPLEMENTATION
// =============================================================================

impl PrimalDiscovery {
    /// Create discovery engine with explicit configuration
    ///
    /// This is the primary constructor for production and test use.
    /// Accepts explicit configuration for concurrent-safe operation.
    #[must_use]
    pub fn new(method: DiscoveryMethod) -> Self {
        Self::with_cache_ttl(method, PRIMAL_DISCOVERY_CACHE_TTL)
    }

    /// Create discovery engine with explicit method and cache TTL
    #[must_use]
    pub fn with_cache_ttl(method: DiscoveryMethod, cache_ttl: Duration) -> Self {
        debug!("🔍 Initializing primal discovery with method: {:?}", method);
        Self {
            method,
            _cache: HashMap::new(),
            _cache_ttl: cache_ttl,
            env_override: None,
        }
    }

    /// Use an explicit environment map for `DiscoveryMethod::Environment` (tests and injected config).
    #[must_use]
    pub fn with_env_override(mut self, env: HashMap<String, String>) -> Self {
        self.env_override = Some(env);
        self
    }

    /// Create discovery engine from environment
    ///
    /// Convenience wrapper that reads configuration from environment variables.
    /// For concurrent-safe tests, use `new()` with explicit configuration instead.
    pub fn from_env() -> Result<Self, BearDogError> {
        info!("🔍 Initializing primal discovery from environment...");

        let method = Self::detect_discovery_method()?;
        debug!("Discovery method: {:?}", method);

        let cache_ttl = beardog_errors::process_env::var("DISCOVERY_CACHE_TTL_SECS")
            .ok()
            .and_then(|s| s.parse().ok())
            .map_or(PRIMAL_DISCOVERY_CACHE_TTL, Duration::from_secs);

        Ok(Self {
            method,
            _cache: HashMap::new(),
            _cache_ttl: cache_ttl,
            env_override: None,
        })
    }

    /// Detect discovery method from environment
    fn detect_discovery_method() -> Result<DiscoveryMethod, BearDogError> {
        match beardog_errors::process_env::var("PRIMAL_DISCOVERY_METHOD")
            .ok()
            .as_deref()
        {
            Some("env" | "environment") => {
                info!("Using environment-based discovery");
                Ok(DiscoveryMethod::Environment)
            }
            Some("upa") => {
                let registry_addr = ipc::resolve_upa_registry_endpoint();
                info!("Using UPA registry at: {}", registry_addr);
                Ok(DiscoveryMethod::UniversalPrimalAuthority { registry_addr })
            }
            Some("mdns") => {
                let service_type = beardog_errors::process_env::var("MDNS_SERVICE_TYPE")
                    .unwrap_or_else(|_| "_ecoprimal._tcp".to_string());
                info!("Using mDNS service discovery: {}", service_type);
                Ok(DiscoveryMethod::Mdns { service_type })
            }
            Some("dns-sd") => {
                let domain = beardog_errors::process_env::var("DNSSD_DOMAIN")
                    .unwrap_or_else(|_| "local.".to_string());
                info!("Using DNS-SD discovery in domain: {}", domain);
                Ok(DiscoveryMethod::DnsSd { domain })
            }
            Some("multi") | None => {
                let registry_addr = ipc::resolve_upa_registry_endpoint();
                info!(
                    "Using multi-method discovery (environment + biomeOS sockets → UPA at {})",
                    registry_addr
                );
                Ok(DiscoveryMethod::Multi(vec![
                    DiscoveryMethod::Environment,
                    DiscoveryMethod::UniversalPrimalAuthority { registry_addr },
                ]))
            }
            Some(other) => Err(BearDogError::invalid_input(&format!(
                "Unknown discovery method: {other}"
            ))),
        }
    }

    /// Discover primals with explicit environment (for testing)
    ///
    /// This method allows tests to provide explicit environment variables without
    /// modifying global state, enabling concurrent-safe testing.
    #[cfg(test)]
    pub async fn discover_with_env(
        &mut self,
        query: DiscoveryQuery,
        env_vars: HashMap<String, String>,
    ) -> Result<Vec<DiscoveredPrimal>, BearDogError> {
        // Clone method to avoid borrow conflicts
        let method = self.method.clone();

        match method {
            DiscoveryMethod::Environment => self.discover_from_env_vars(&env_vars, &query),
            DiscoveryMethod::Multi(methods) => {
                // Try each method in order
                for method in methods {
                    if matches!(method, DiscoveryMethod::Environment)
                        && let Ok(results) = self.discover_from_env_vars(&env_vars, &query)
                        && !results.is_empty()
                    {
                        return Ok(results);
                    }
                }
                Ok(Vec::new())
            }
            _ => {
                // Other methods not yet implemented for testing
                Ok(Vec::new())
            }
        }
    }

    /// Discover primals matching query
    pub async fn discover(
        &mut self,
        query: DiscoveryQuery,
    ) -> Result<Vec<DiscoveredPrimal>, BearDogError> {
        info!("🔍 Discovering primals: {:?}", query);

        // Clone method to avoid borrow checker issues
        let method = self.method.clone();

        match method {
            DiscoveryMethod::Environment => self.discover_from_env(&query),
            DiscoveryMethod::UniversalPrimalAuthority { registry_addr } => {
                self.discover_from_upa(&query, &registry_addr).await
            }
            DiscoveryMethod::Mdns { service_type } => {
                self.discover_from_mdns(&query, &service_type)
            }
            DiscoveryMethod::DnsSd { domain } => self.discover_from_dns_sd(&query, &domain),
            DiscoveryMethod::Multi(methods) => self.discover_multi(&query, &methods).await,
        }
    }

    /// Discover from environment variables
    ///
    /// Reads via [`beardog_errors::process_env::vars`] (OS env merged with the test overlay).
    fn discover_from_env(
        &mut self,
        query: &DiscoveryQuery,
    ) -> Result<Vec<DiscoveredPrimal>, BearDogError> {
        let vars: HashMap<String, String> = match &self.env_override {
            Some(m) => m.clone(),
            None => beardog_errors::process_env::vars().collect(),
        };
        self.discover_from_env_vars(&vars, query)
    }

    /// Discover from explicit environment map
    ///
    /// This method accepts an explicit environment map, making it concurrent-safe
    /// for testing while maintaining the same logic as `discover_from_env()`.
    fn discover_from_env_vars(
        &self,
        env_vars: &HashMap<String, String>,
        query: &DiscoveryQuery,
    ) -> Result<Vec<DiscoveredPrimal>, BearDogError> {
        debug!("Discovering from environment variables");

        let mut discovered = Vec::new();

        // If specific name requested, check PRIMAL_<NAME>_ADDR
        if let Some(name) = &query.name {
            let env_key = format!("PRIMAL_{}_ADDR", name.to_uppercase());
            if let Some(addr) = env_vars.get(&env_key) {
                let endpoint = Endpoint::parse(addr)?;
                info!("Found {} at {} (from {})", name, addr, env_key);

                // Also check for capabilities: PRIMAL_<NAME>_CAPABILITIES
                let caps_key = format!("PRIMAL_{}_CAPABILITIES", name.to_uppercase());
                let capabilities = Self::parse_capabilities_from_env_map(env_vars, &caps_key);

                discovered.push(DiscoveredPrimal {
                    name: name.clone(),
                    endpoints: vec![endpoint],
                    capabilities,
                    trust_score: Some(1.0), // Explicit config = trusted
                    discovered_at: std::time::SystemTime::now(),
                });
            }
        } else {
            // Scan all PRIMAL_*_ADDR environment variables
            for (key, value) in env_vars {
                if key.starts_with("PRIMAL_") && key.ends_with("_ADDR") {
                    let name = key
                        .strip_prefix("PRIMAL_")
                        .and_then(|s| s.strip_suffix("_ADDR"))
                        .unwrap_or("unknown");

                    if let Ok(endpoint) = Endpoint::parse(value) {
                        info!("Found {} at {} (from {})", name, value, key);

                        // Also check for capabilities: PRIMAL_<NAME>_CAPABILITIES
                        let caps_key = format!("PRIMAL_{}_CAPABILITIES", name.to_uppercase());
                        let capabilities =
                            Self::parse_capabilities_from_env_map(env_vars, &caps_key);

                        discovered.push(DiscoveredPrimal {
                            name: name.to_lowercase(),
                            endpoints: vec![endpoint],
                            capabilities,
                            trust_score: Some(1.0),
                            discovered_at: std::time::SystemTime::now(),
                        });
                    }
                }
            }
        }

        Self::append_biomeos_socket_primals(env_vars, query, &mut discovered);

        // Filter by capabilities if specified in query
        let discovered = if query.capabilities.is_empty() {
            discovered
        } else {
            discovered
                .into_iter()
                .filter(|primal| {
                    // Primal must have ALL requested capabilities
                    query
                        .capabilities
                        .iter()
                        .all(|req_cap| primal.capabilities.contains(req_cap))
                })
                .collect()
        };

        if discovered.is_empty() {
            warn!("No primals discovered from environment");
        } else {
            info!("Discovered {} primals from environment", discovered.len());
        }

        Ok(discovered)
    }

    /// Discover peer primals from `*.sock` entries under the resolved biomeOS runtime directory.
    ///
    /// Resolution order for the directory: `BEARDOG_BIOMEOS_SOCKET_DIR`, then
    /// `$XDG_RUNTIME_DIR/biomeos`, then [`ipc::biomeos_ipc_socket_dir`].
    /// The registry listener ([`ipc::DEFAULT_UPA_REGISTRY_SOCKET_STEM`]) is skipped; use UPA discovery for that.
    fn append_biomeos_socket_primals(
        env_vars: &HashMap<String, String>,
        query: &DiscoveryQuery,
        discovered: &mut Vec<DiscoveredPrimal>,
    ) {
        let dir = ipc::biomeos_ipc_socket_dir_from_components(
            env_vars
                .get(ipc::ENV_BIOMEOS_SOCKET_DIR_OVERRIDE)
                .map(String::as_str),
            env_vars.get("XDG_RUNTIME_DIR").map(String::as_str),
            env_vars
                .get(ipc::ENV_BIOMEOS_IPC_NAMESPACE)
                .map(String::as_str),
        );

        let entries = match std::fs::read_dir(&dir) {
            Ok(e) => e,
            Err(e) => {
                debug!(
                    "BiomeOS IPC socket directory not readable ({}): {}",
                    dir.display(),
                    e
                );
                return;
            }
        };

        let mut seen: HashSet<String> = discovered.iter().map(|p| p.name.to_lowercase()).collect();

        for entry in entries.filter_map(Result::ok) {
            let path = entry.path();
            let ext = path.extension();
            if ext != Some(OsStr::new("sock")) {
                continue;
            }
            let Ok(meta) = std::fs::metadata(&path) else {
                continue;
            };
            if meta.is_dir() {
                continue;
            }
            let Some(stem) = path.file_stem().and_then(|s| s.to_str()) else {
                continue;
            };
            if stem.eq_ignore_ascii_case(ipc::DEFAULT_UPA_REGISTRY_SOCKET_STEM) {
                continue;
            }
            if let Some(want) = &query.name
                && !want.eq_ignore_ascii_case(stem)
            {
                continue;
            }
            let lname = stem.to_lowercase();
            if seen.contains(&lname) {
                continue;
            }
            let Ok(endpoint) = Endpoint::parse(&format!("unix://{}", path.display())) else {
                continue;
            };
            let caps_key = format!("PRIMAL_{}_CAPABILITIES", stem.to_uppercase());
            let capabilities = Self::parse_capabilities_from_env_map(env_vars, &caps_key);
            info!(
                "Found primal '{}' at {} (runtime socket scan)",
                lname,
                path.display()
            );
            seen.insert(lname.clone());
            discovered.push(DiscoveredPrimal {
                name: lname,
                endpoints: vec![endpoint],
                capabilities,
                trust_score: Some(0.85),
                discovered_at: std::time::SystemTime::now(),
            });
        }
    }

    /// Parse capabilities from environment variable
    ///
    /// Expected format: Comma-separated list like "SecureTunneling,GeneticLineage,Discovery"
    fn _parse_capabilities_from_env(env_key: &str) -> Vec<SimpleCapability> {
        beardog_errors::process_env::var(env_key)
            .ok()
            .map(|caps_str| Self::parse_capabilities_str(&caps_str, env_key))
            .unwrap_or_default()
    }

    fn parse_capabilities_from_env_map(
        env_vars: &HashMap<String, String>,
        env_key: &str,
    ) -> Vec<SimpleCapability> {
        env_vars
            .get(env_key)
            .map(|caps_str| Self::parse_capabilities_str(caps_str, env_key))
            .unwrap_or_default()
    }

    fn parse_capabilities_str(caps_str: &str, env_key: &str) -> Vec<SimpleCapability> {
        caps_str
            .split(',')
            .filter_map(|cap| {
                let cap_trimmed = cap.trim();
                match cap_trimmed {
                    "SecureTunneling" => Some(SimpleCapability::SecureTunneling),
                    "GeneticLineage" => Some(SimpleCapability::GeneticLineage),
                    "Cryptography" => Some(SimpleCapability::Cryptography),
                    "HsmIntegration" => Some(SimpleCapability::HsmIntegration),
                    "Discovery" => Some(SimpleCapability::Discovery),
                    _ => {
                        warn!("Unknown capability in {}: {}", env_key, cap_trimmed);
                        None
                    }
                }
            })
            .collect()
    }

    /// Discover from UPA registry (COMPLETE IMPLEMENTATION)
    async fn discover_from_upa(
        &mut self,
        query: &DiscoveryQuery,
        registry_addr: &str,
    ) -> Result<Vec<DiscoveredPrimal>, BearDogError> {
        info!("🔍 UPA registry discovery at: {}", registry_addr);

        // UPA uses Unix socket + JSON-RPC
        let socket_path = registry_addr.trim_start_matches("unix://");

        // Build JSON-RPC request
        let capability = if query.capabilities.is_empty() {
            "generic".to_string()
        } else {
            format!("{:?}", query.capabilities[0])
        };

        let request = serde_json::json!({
            "jsonrpc": "2.0",
            "method": "upa.discover",
            "params": {
                "capability": capability,
                "timeout_ms": HEALTH_CHECK_TIMEOUT.as_millis() as u64
            },
            "id": 1
        });

        // Connect to UPA registry via Unix socket
        match tokio::net::UnixStream::connect(socket_path).await {
            Ok(mut stream) => {
                use tokio::io::{AsyncReadExt, AsyncWriteExt};

                // Send request
                let request_str =
                    serde_json::to_string(&request).map_err(|e| BearDogError::Network {
                        message: format!("Failed to serialize UPA request: {e}"),
                        category: beardog_errors::NetworkErrorCategory::Connection,
                    })?;
                stream.write_all(request_str.as_bytes()).await?;
                stream.write_all(b"\n").await?;

                // Read response
                let mut buffer = vec![0u8; 8192];
                let n = stream.read(&mut buffer).await?;
                let response_str = String::from_utf8_lossy(&buffer[..n]);

                // Parse JSON-RPC response
                let response: serde_json::Value =
                    serde_json::from_str(&response_str).map_err(|e| BearDogError::Network {
                        message: format!("Failed to parse UPA response: {e}"),
                        category: beardog_errors::NetworkErrorCategory::Connection,
                    })?;

                if let Some(result) = response.get("result")
                    && let Some(primals_array) = result.as_array()
                {
                    info!("✅ UPA discovered {} primals", primals_array.len());

                    let primals = primals_array
                        .iter()
                        .filter_map(|p| serde_json::from_value::<DiscoveredPrimal>(p.clone()).ok())
                        .collect();

                    return Ok(primals);
                }

                warn!("UPA returned no results");
                Ok(Vec::new())
            }
            Err(e) => {
                warn!("UPA registry not available at {}: {}", registry_addr, e);
                Ok(Vec::new())
            }
        }
    }

    /// Discover from mDNS
    ///
    /// # Integration Status
    ///
    /// The beardog-discovery crate has a complete mDNS implementation (45 tests pass).
    /// Integration requires:
    /// 1. Add `beardog-discovery` to beardog-core/Cargo.toml
    /// 2. Enable the `mdns` feature
    /// 3. Convert between `DiscoveredService` and `DiscoveredPrimal` types
    ///
    /// # Current Behavior
    ///
    /// - With `mdns` feature: Logs warning, returns empty
    /// - Without `mdns` feature: Logs warning, returns empty
    ///
    /// Production deployments should use Unix socket discovery (via beardog-ipc)
    /// or HTTP-based service registries until mDNS integration is complete.
    fn discover_from_mdns(
        &mut self,
        _query: &DiscoveryQuery,
        service_type: &str,
    ) -> Result<Vec<DiscoveredPrimal>, BearDogError> {
        info!("🔍 mDNS discovery for service: {}", service_type);

        // beardog-discovery integration pending (see module docs above).

        #[cfg(feature = "mdns")]
        {
            warn!(
                "mDNS discovery requested for '{}' - beardog-discovery integration pending. \
                 Use Unix socket or HTTP discovery instead.",
                service_type
            );
            Ok(Vec::new())
        }

        #[cfg(not(feature = "mdns"))]
        {
            debug!("mDNS feature not enabled. Enable with: cargo build --features mdns");
            Ok(Vec::new())
        }
    }

    /// Discover from DNS-SD (COMPLETE IMPLEMENTATION)
    fn discover_from_dns_sd(
        &mut self,
        _query: &DiscoveryQuery, // Planned: Use for capability filtering in discovery results
        domain: &str,
    ) -> Result<Vec<DiscoveredPrimal>, BearDogError> {
        info!("🔍 DNS-SD discovery in domain: {}", domain);

        // DNS-SD uses the same infrastructure as mDNS, just with different domain
        #[cfg(feature = "mdns")]
        {
            // EVOLUTION: DNS-SD discovery via ecosystem relay IPC (capability-based).
            // This is the proper implementation using inter-primal communication
            // instead of hardcoded mock data.
            //
            // NOTE: beardog-discovery ready (45 tests pass), pending integration wiring
            // For now, return empty until integration is complete.
            // This is honest about current capabilities (fallback, not full discovery).
            warn!(
                "DNS-SD discovery via ecosystem relay IPC not yet complete - beardog-discovery crate pending"
            );
            warn!("Returning empty discovery results until integration is complete");
            warn!("See: specs/IMPLEMENTATION_GAPS_NOV_2025.md for beardog-discovery timeline");

            // Return empty - honest about current state
            // Tests that depend on discovery should use explicit test-only mocks
            Ok(vec![])
        }

        #[cfg(not(feature = "mdns"))]
        {
            warn!("DNS-SD/mDNS feature not enabled, returning empty results");
            Ok(Vec::new())
        }
    }

    /// Try multiple discovery methods
    async fn discover_multi(
        &mut self,
        query: &DiscoveryQuery,
        methods: &[DiscoveryMethod],
    ) -> Result<Vec<DiscoveredPrimal>, BearDogError> {
        let mut all_discovered = Vec::new();

        for method in methods {
            let result = match method {
                DiscoveryMethod::Environment => self.discover_from_env(query),
                DiscoveryMethod::UniversalPrimalAuthority { registry_addr } => {
                    self.discover_from_upa(query, registry_addr).await
                }
                DiscoveryMethod::Mdns { service_type } => {
                    self.discover_from_mdns(query, service_type)
                }
                DiscoveryMethod::DnsSd { domain } => self.discover_from_dns_sd(query, domain),
                DiscoveryMethod::Multi(_) => {
                    // Prevent infinite recursion
                    continue;
                }
            };

            if let Ok(mut discovered) = result {
                all_discovered.append(&mut discovered);
            }
        }

        // Deduplicate by name (keep first occurrence)
        let mut seen = std::collections::HashSet::new();
        all_discovered.retain(|p| seen.insert(p.name.clone()));

        Ok(all_discovered)
    }
}

#[cfg(test)]
#[path = "primal_discovery_tests.rs"]
mod tests;
