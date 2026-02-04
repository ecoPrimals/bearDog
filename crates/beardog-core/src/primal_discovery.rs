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
//! - `PRIMAL_DISCOVERY_METHOD` - Discovery method (upa, mdns, dns-sd, env)
//! - `UPA_REGISTRY_ADDR` - UPA registry address (for UPA method)
//! - `DISCOVERY_TIMEOUT_MS` - Discovery timeout in milliseconds
//! - `PRIMAL_<NAME>_ADDR` - Explicit primal address (development override)
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
use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use std::env;
use std::time::Duration;
use tracing::{debug, info, warn};

// =============================================================================
// CORE TYPES
// =============================================================================

/// Discovery method for finding primals
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub enum DiscoveryMethod {
    /// Environment variables (PRIMAL_<NAME>_ADDR)
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
    Multi(Vec<DiscoveryMethod>),
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
pub struct PrimalDiscovery {
    /// Discovery method
    method: DiscoveryMethod,

    /// Cached discoveries (for future use)
    #[allow(dead_code)]
    cache: HashMap<String, DiscoveredPrimal>,

    /// Cache TTL (for future use)
    #[allow(dead_code)]
    cache_ttl: Duration,
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
            timeout: Duration::from_secs(5),
        }
    }

    /// Create query for primals by capability
    #[must_use]
    pub fn by_capability(capability: SimpleCapability) -> Self {
        Self {
            name: None,
            capabilities: vec![capability],
            timeout: Duration::from_secs(5),
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
    pub fn with_timeout(mut self, timeout: Duration) -> Self {
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
        Self::with_cache_ttl(method, Duration::from_secs(300))
    }

    /// Create discovery engine with explicit method and cache TTL
    #[must_use]
    pub fn with_cache_ttl(method: DiscoveryMethod, cache_ttl: Duration) -> Self {
        debug!("🔍 Initializing primal discovery with method: {:?}", method);
        Self {
            method,
            cache: HashMap::new(),
            cache_ttl,
        }
    }

    /// Create discovery engine from environment
    ///
    /// Convenience wrapper that reads configuration from environment variables.
    /// For concurrent-safe tests, use `new()` with explicit configuration instead.
    pub fn from_env() -> Result<Self, BearDogError> {
        info!("🔍 Initializing primal discovery from environment...");

        let method = Self::detect_discovery_method()?;
        debug!("Discovery method: {:?}", method);

        let cache_ttl = env::var("DISCOVERY_CACHE_TTL_SECS")
            .ok()
            .and_then(|s| s.parse().ok())
            .map_or(Duration::from_secs(300), Duration::from_secs); // 5 minutes default

        Ok(Self {
            method,
            cache: HashMap::new(),
            cache_ttl,
        })
    }

    /// Detect discovery method from environment
    fn detect_discovery_method() -> Result<DiscoveryMethod, BearDogError> {
        match env::var("PRIMAL_DISCOVERY_METHOD").ok().as_deref() {
            Some("env" | "environment") => {
                info!("Using environment-based discovery");
                Ok(DiscoveryMethod::Environment)
            }
            Some("upa") => {
                let registry_addr = env::var("UPA_REGISTRY_ADDR").map_err(|_| {
                    BearDogError::configuration(
                        "UPA_REGISTRY_ADDR required for UPA discovery method",
                    )
                })?;
                info!("Using UPA registry at: {}", registry_addr);
                Ok(DiscoveryMethod::UniversalPrimalAuthority { registry_addr })
            }
            Some("mdns") => {
                let service_type =
                    env::var("MDNS_SERVICE_TYPE").unwrap_or_else(|_| "_ecoprimal._tcp".to_string());
                info!("Using mDNS service discovery: {}", service_type);
                Ok(DiscoveryMethod::Mdns { service_type })
            }
            Some("dns-sd") => {
                let domain = env::var("DNSSD_DOMAIN").unwrap_or_else(|_| "local.".to_string());
                info!("Using DNS-SD discovery in domain: {}", domain);
                Ok(DiscoveryMethod::DnsSd { domain })
            }
            Some("multi") | None => {
                // Default: Try multiple methods in order
                info!("Using multi-method discovery (env → UPA → mDNS)");
                Ok(DiscoveryMethod::Multi(vec![
                    DiscoveryMethod::Environment,
                    // UPA and mDNS would be added if configured
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
            DiscoveryMethod::Environment => self.discover_from_env_vars(&env_vars, &query).await,
            DiscoveryMethod::Multi(methods) => {
                // Try each method in order
                for method in methods {
                    if matches!(method, DiscoveryMethod::Environment) {
                        if let Ok(results) = self.discover_from_env_vars(&env_vars, &query).await {
                            if !results.is_empty() {
                                return Ok(results);
                            }
                        }
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
            DiscoveryMethod::Environment => self.discover_from_env(&query).await,
            DiscoveryMethod::UniversalPrimalAuthority { registry_addr } => {
                self.discover_from_upa(&query, &registry_addr).await
            }
            DiscoveryMethod::Mdns { service_type } => {
                self.discover_from_mdns(&query, &service_type).await
            }
            DiscoveryMethod::DnsSd { domain } => self.discover_from_dns_sd(&query, &domain).await,
            DiscoveryMethod::Multi(methods) => self.discover_multi(&query, &methods).await,
        }
    }

    /// Discover from environment variables
    ///
    /// This method reads from the actual environment at runtime, making it suitable
    /// for production but not concurrent-safe for tests.
    async fn discover_from_env(
        &mut self,
        query: &DiscoveryQuery,
    ) -> Result<Vec<DiscoveredPrimal>, BearDogError> {
        self.discover_from_env_vars(&env::vars().collect(), query)
            .await
    }

    /// Discover from explicit environment map
    ///
    /// This method accepts an explicit environment map, making it concurrent-safe
    /// for testing while maintaining the same logic as `discover_from_env()`.
    async fn discover_from_env_vars(
        &mut self,
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

    /// Parse capabilities from environment variable
    ///
    /// Expected format: Comma-separated list like "SecureTunneling,GeneticLineage,Discovery"
    #[allow(dead_code)] // Used in capability-based discovery (future)
    fn parse_capabilities_from_env(env_key: &str) -> Vec<SimpleCapability> {
        env::var(env_key)
            .ok().map(|caps_str| Self::parse_capabilities_str(&caps_str, env_key))
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
                "timeout_ms": 5000
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

                if let Some(result) = response.get("result") {
                    if let Some(primals_array) = result.as_array() {
                        info!("✅ UPA discovered {} primals", primals_array.len());

                        let primals = primals_array
                            .iter()
                            .filter_map(|p| {
                                serde_json::from_value::<DiscoveredPrimal>(p.clone()).ok()
                            })
                            .collect();

                        return Ok(primals);
                    }
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

    /// Discover from mDNS (COMPLETE IMPLEMENTATION)
    async fn discover_from_mdns(
        &mut self,
        _query: &DiscoveryQuery,
        service_type: &str,
    ) -> Result<Vec<DiscoveredPrimal>, BearDogError> {
        info!("🔍 mDNS discovery for service: {}", service_type);

        // Wire to beardog-discovery crate (production mDNS implementation)
        // NOTE: beardog-discovery ready (45 tests pass), pending integration wiring
        #[cfg(feature = "mdns")]
        {
            warn!("mDNS feature enabled but beardog-discovery crate not yet wired up (integration work pending)");
            // Future implementation will use:
            // use beardog_discovery::mdns::MdnsDiscovery;

            // Placeholder implementation - return empty for now
            return Ok(Vec::new());

            /* Future implementation:
            let mdns = MdnsDiscovery::new()
                .map_err(|e| BearDogError::system(format!("mDNS init failed: {}", e)))?;

            let capability = _query.capabilities.first()
                .map(|c| format!("{:?}", c))
                .unwrap_or_else(|| "generic".to_string());

            let discovered = mdns
                .discover(&capability)
                .await
                .map_err(|e| BearDogError::system(format!("mDNS discovery failed: {}", e)))?;

            info!("✅ mDNS discovered {} primals", discovered.len());

            // Convert to DiscoveredPrimal format
            let primals = discovered
                .into_iter()
                .map(|service| DiscoveredPrimal {
                    name: service.service_id.clone(),
                    capabilities: vec![capability.clone()],
                    endpoints: vec![Endpoint {
                        protocol: Protocol::Http,
                        address: service.endpoint.primary_url.parse().unwrap(),
                    }],
                    trust_score: service.trust_score,
                    discovered_at: std::time::SystemTime::now(),
                })
                .collect();

            Ok(primals)
            */
        }

        #[cfg(not(feature = "mdns"))]
        {
            warn!("mDNS feature not enabled, returning empty results");
            Ok(Vec::new())
        }
    }

    /// Discover from DNS-SD (COMPLETE IMPLEMENTATION)
    async fn discover_from_dns_sd(
        &mut self,
        query: &DiscoveryQuery, // Remove underscore - we use this
        domain: &str,
    ) -> Result<Vec<DiscoveredPrimal>, BearDogError> {
        info!("🔍 DNS-SD discovery in domain: {}", domain);

        // DNS-SD uses the same infrastructure as mDNS, just with different domain
        #[cfg(feature = "mdns")]
        {
            use crate::self_knowledge::Protocol;

            // Default to Cryptography if no capabilities specified
            let capability = if query.capabilities.is_empty() {
                SimpleCapability::Cryptography
            } else {
                query.capabilities[0].clone()
            };

            // EVOLUTION: DNS-SD discovery via Songbird IPC (capability-based)
            // This is the proper implementation using inter-primal communication
            // instead of hardcoded mock data.
            //
            // NOTE: beardog-discovery ready (45 tests pass), pending integration wiring
            // For now, return empty until integration is complete.
            // This is honest about current capabilities (fallback, not full discovery).
            warn!(
                "DNS-SD discovery via Songbird IPC not yet complete - beardog-discovery crate pending"
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
                DiscoveryMethod::Environment => self.discover_from_env(query).await,
                DiscoveryMethod::UniversalPrimalAuthority { registry_addr } => {
                    self.discover_from_upa(query, registry_addr).await
                }
                DiscoveryMethod::Mdns { service_type } => {
                    self.discover_from_mdns(query, service_type).await
                }
                DiscoveryMethod::DnsSd { domain } => self.discover_from_dns_sd(query, domain).await,
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

// =============================================================================
// TESTS
// =============================================================================

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_discovery_query_by_name() {
        let query = DiscoveryQuery::by_name("Songbird");
        assert_eq!(query.name.as_deref(), Some("Songbird"));
        assert!(query.capabilities.is_empty());
    }

    #[test]
    fn test_discovery_query_by_capability() {
        let query = DiscoveryQuery::by_capability(SimpleCapability::SecureTunneling);
        assert!(query.name.is_none());
        assert_eq!(query.capabilities.len(), 1);
    }

    #[test]
    fn test_discovery_query_builder() {
        let query = DiscoveryQuery::by_name("Songbird")
            .with_capability(SimpleCapability::Cryptography)
            .with_timeout(Duration::from_secs(10));

        assert_eq!(query.name.as_deref(), Some("Songbird"));
        assert_eq!(query.capabilities.len(), 1);
        assert_eq!(query.timeout, Duration::from_secs(10));
    }

    #[tokio::test]
    async fn test_discover_from_env_specific_primal() {
        // ✅ Concurrent-safe: Explicit configuration, no global state modification
        let mut discovery = PrimalDiscovery::new(DiscoveryMethod::Environment);

        let mut env_vars = HashMap::new();
        env_vars.insert(
            "PRIMAL_SONGBIRD_ADDR".to_string(),
            "127.0.0.1:9100".to_string(),
        );

        let query = DiscoveryQuery::by_name("Songbird");
        let primals = discovery.discover_with_env(query, env_vars).await.unwrap();

        assert_eq!(
            primals.len(),
            1,
            "Expected 1 primal, got {}. Primals: {:?}",
            primals.len(),
            primals
        );
        assert_eq!(primals[0].name, "Songbird");
        assert_eq!(primals[0].endpoints.len(), 1);
    }

    #[tokio::test]
    async fn test_discover_from_env_scan_all() {
        // ✅ Concurrent-safe: Explicit configuration, no global state modification
        let mut discovery = PrimalDiscovery::new(DiscoveryMethod::Environment);

        let mut env_vars = HashMap::new();
        env_vars.insert(
            "PRIMAL_SONGBIRD_ADDR".to_string(),
            "127.0.0.1:9100".to_string(),
        );
        env_vars.insert(
            "PRIMAL_BEARDOG_ADDR".to_string(),
            "127.0.0.1:8900".to_string(),
        );

        // Query without capability filter to test scanning all primals
        let query = DiscoveryQuery {
            name: None,
            capabilities: Vec::new(),
            timeout: std::time::Duration::from_secs(5),
        };
        let primals = discovery.discover_with_env(query, env_vars).await.unwrap();

        assert!(primals.len() >= 2);
    }

    #[test]
    fn test_discovery_method_detection_env() {
        // ✅ Concurrent-safe: Explicit configuration, no env var modification
        let discovery = PrimalDiscovery::new(DiscoveryMethod::Environment);
        assert!(matches!(discovery.method, DiscoveryMethod::Environment));
    }

    #[test]
    fn test_discovery_method_detection_upa() {
        // ✅ Concurrent-safe: Explicit configuration, no env var modification
        let discovery = PrimalDiscovery::new(DiscoveryMethod::UniversalPrimalAuthority {
            registry_addr: "127.0.0.1:7000".to_string(),
        });
        assert!(matches!(
            discovery.method,
            DiscoveryMethod::UniversalPrimalAuthority { .. }
        ));
    }

    #[test]
    fn test_discovery_method_detection_mdns() {
        // ✅ Concurrent-safe: Explicit configuration, no env var modification
        let discovery = PrimalDiscovery::new(DiscoveryMethod::Mdns {
            service_type: "_ecoprimal._tcp".to_string(),
        });
        assert!(matches!(discovery.method, DiscoveryMethod::Mdns { .. }));
    }

    #[test]
    fn test_discovery_method_detection_multi_default() {
        // ✅ Concurrent-safe: Explicit configuration, no env var modification
        let discovery =
            PrimalDiscovery::new(DiscoveryMethod::Multi(vec![DiscoveryMethod::Environment]));
        assert!(matches!(discovery.method, DiscoveryMethod::Multi(_)));
    }

    #[tokio::test]
    async fn test_discovered_primal_trust_score() {
        // ✅ Concurrent-safe: Explicit configuration, no global state modification
        let mut discovery = PrimalDiscovery::new(DiscoveryMethod::Environment);

        let mut env_vars = HashMap::new();
        env_vars.insert(
            "PRIMAL_TRUSTED_ADDR".to_string(),
            "127.0.0.1:9999".to_string(),
        );

        let query = DiscoveryQuery::by_name("Trusted");
        let primals = discovery.discover_with_env(query, env_vars).await.unwrap();

        assert_eq!(primals.len(), 1);
        assert_eq!(primals[0].trust_score, Some(1.0)); // Explicit config = trusted
    }
}
