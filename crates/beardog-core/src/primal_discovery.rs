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
    UniversalPrimalAuthority { registry_addr: String },

    /// Multicast DNS (mDNS) service discovery
    Mdns { service_type: String },

    /// DNS Service Discovery (DNS-SD)
    DnsSd { domain: String },

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

    /// Cached discoveries
    cache: HashMap<String, DiscoveredPrimal>,

    /// Cache TTL
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
    /// Create discovery engine from environment
    pub fn from_env() -> Result<Self, BearDogError> {
        info!("🔍 Initializing primal discovery from environment...");

        let method = Self::detect_discovery_method()?;
        debug!("Discovery method: {:?}", method);

        let cache_ttl = env::var("DISCOVERY_CACHE_TTL_SECS")
            .ok()
            .and_then(|s| s.parse().ok())
            .map(Duration::from_secs)
            .unwrap_or(Duration::from_secs(300)); // 5 minutes default

        Ok(Self {
            method,
            cache: HashMap::new(),
            cache_ttl,
        })
    }

    /// Detect discovery method from environment
    fn detect_discovery_method() -> Result<DiscoveryMethod, BearDogError> {
        match env::var("PRIMAL_DISCOVERY_METHOD").ok().as_deref() {
            Some("env") | Some("environment") => {
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
                "Unknown discovery method: {}",
                other
            ))),
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
    async fn discover_from_env(
        &mut self,
        query: &DiscoveryQuery,
    ) -> Result<Vec<DiscoveredPrimal>, BearDogError> {
        debug!("Discovering from environment variables");

        let mut discovered = Vec::new();

        // If specific name requested, check PRIMAL_<NAME>_ADDR
        if let Some(name) = &query.name {
            let env_key = format!("PRIMAL_{}_ADDR", name.to_uppercase());
            if let Ok(addr) = env::var(&env_key) {
                let endpoint = Endpoint::parse(&addr)?;
                info!("Found {} at {} (from {})", name, addr, env_key);

                // Also check for capabilities: PRIMAL_<NAME>_CAPABILITIES
                let caps_key = format!("PRIMAL_{}_CAPABILITIES", name.to_uppercase());
                let capabilities = Self::parse_capabilities_from_env(&caps_key);

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
            for (key, value) in env::vars() {
                if key.starts_with("PRIMAL_") && key.ends_with("_ADDR") {
                    let name = key
                        .strip_prefix("PRIMAL_")
                        .and_then(|s| s.strip_suffix("_ADDR"))
                        .unwrap_or("unknown");

                    if let Ok(endpoint) = Endpoint::parse(&value) {
                        info!("Found {} at {} (from {})", name, value, key);

                        // Also check for capabilities: PRIMAL_<NAME>_CAPABILITIES
                        let caps_key = format!("PRIMAL_{}_CAPABILITIES", name.to_uppercase());
                        let capabilities = Self::parse_capabilities_from_env(&caps_key);

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
        let discovered = if !query.capabilities.is_empty() {
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
        } else {
            discovered
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
    fn parse_capabilities_from_env(env_key: &str) -> Vec<SimpleCapability> {
        env::var(env_key)
            .ok()
            .map(|caps_str| {
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
            })
            .unwrap_or_default()
    }

    /// Discover from UPA registry (stub for now)
    async fn discover_from_upa(
        &mut self,
        _query: &DiscoveryQuery,
        registry_addr: &str,
    ) -> Result<Vec<DiscoveredPrimal>, BearDogError> {
        warn!(
            "UPA discovery not yet implemented (registry: {})",
            registry_addr
        );
        // TODO: Implement UPA client discovery
        // This will be implemented when UPA registry is ready
        Ok(Vec::new())
    }

    /// Discover from mDNS (stub for now)
    async fn discover_from_mdns(
        &mut self,
        _query: &DiscoveryQuery,
        service_type: &str,
    ) -> Result<Vec<DiscoveredPrimal>, BearDogError> {
        warn!(
            "mDNS discovery not yet implemented (service: {})",
            service_type
        );
        // TODO: Implement mDNS discovery
        // This will use the existing mDNS code in beardog-discovery
        Ok(Vec::new())
    }

    /// Discover from DNS-SD (stub for now)
    async fn discover_from_dns_sd(
        &mut self,
        _query: &DiscoveryQuery,
        domain: &str,
    ) -> Result<Vec<DiscoveredPrimal>, BearDogError> {
        warn!("DNS-SD discovery not yet implemented (domain: {})", domain);
        // TODO: Implement DNS-SD discovery
        Ok(Vec::new())
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
        // Clean up first
        std::env::remove_var("PRIMAL_SONGBIRD_ADDR");
        std::env::remove_var("PRIMAL_DISCOVERY_METHOD");

        std::env::set_var("PRIMAL_DISCOVERY_METHOD", "env");
        std::env::set_var("PRIMAL_SONGBIRD_ADDR", "127.0.0.1:9100");

        let mut discovery = PrimalDiscovery::from_env().unwrap();
        let query = DiscoveryQuery::by_name("Songbird");
        let primals = discovery.discover(query).await.unwrap();

        assert_eq!(
            primals.len(),
            1,
            "Expected 1 primal, got {}. Primals: {:?}",
            primals.len(),
            primals
        );
        assert_eq!(primals[0].name, "Songbird");
        assert_eq!(primals[0].endpoints.len(), 1);

        std::env::remove_var("PRIMAL_SONGBIRD_ADDR");
        std::env::remove_var("PRIMAL_DISCOVERY_METHOD");
    }

    #[tokio::test]
    async fn test_discover_from_env_scan_all() {
        std::env::set_var("PRIMAL_DISCOVERY_METHOD", "env");
        std::env::set_var("PRIMAL_SONGBIRD_ADDR", "127.0.0.1:9100");
        std::env::set_var("PRIMAL_BEARDOG_ADDR", "127.0.0.1:8900");

        let mut discovery = PrimalDiscovery::from_env().unwrap();
        // Query without capability filter to test scanning all primals
        let query = DiscoveryQuery {
            name: None,
            capabilities: Vec::new(),
            timeout: std::time::Duration::from_secs(5),
        };
        let primals = discovery.discover(query).await.unwrap();

        assert!(primals.len() >= 2);

        std::env::remove_var("PRIMAL_SONGBIRD_ADDR");
        std::env::remove_var("PRIMAL_BEARDOG_ADDR");
        std::env::remove_var("PRIMAL_DISCOVERY_METHOD");
    }

    #[test]
    fn test_discovery_method_detection_env() {
        std::env::set_var("PRIMAL_DISCOVERY_METHOD", "env");

        let discovery = PrimalDiscovery::from_env().unwrap();
        assert!(matches!(discovery.method, DiscoveryMethod::Environment));

        std::env::remove_var("PRIMAL_DISCOVERY_METHOD");
    }

    #[test]
    fn test_discovery_method_detection_upa() {
        std::env::set_var("PRIMAL_DISCOVERY_METHOD", "upa");
        std::env::set_var("UPA_REGISTRY_ADDR", "127.0.0.1:7000");

        let discovery = PrimalDiscovery::from_env().unwrap();
        assert!(matches!(
            discovery.method,
            DiscoveryMethod::UniversalPrimalAuthority { .. }
        ));

        std::env::remove_var("PRIMAL_DISCOVERY_METHOD");
        std::env::remove_var("UPA_REGISTRY_ADDR");
    }

    #[test]
    fn test_discovery_method_detection_mdns() {
        std::env::set_var("PRIMAL_DISCOVERY_METHOD", "mdns");

        let discovery = PrimalDiscovery::from_env().unwrap();
        assert!(matches!(discovery.method, DiscoveryMethod::Mdns { .. }));

        std::env::remove_var("PRIMAL_DISCOVERY_METHOD");
    }

    #[test]
    fn test_discovery_method_detection_multi_default() {
        std::env::remove_var("PRIMAL_DISCOVERY_METHOD");

        let discovery = PrimalDiscovery::from_env().unwrap();
        assert!(matches!(discovery.method, DiscoveryMethod::Multi(_)));
    }

    #[tokio::test]
    async fn test_discovered_primal_trust_score() {
        std::env::set_var("PRIMAL_DISCOVERY_METHOD", "env");
        std::env::set_var("PRIMAL_TRUSTED_ADDR", "127.0.0.1:9999");

        let mut discovery = PrimalDiscovery::from_env().unwrap();
        let query = DiscoveryQuery::by_name("Trusted");
        let primals = discovery.discover(query).await.unwrap();

        assert_eq!(primals.len(), 1);
        assert_eq!(primals[0].trust_score, Some(1.0)); // Explicit config = trusted

        std::env::remove_var("PRIMAL_TRUSTED_ADDR");
        std::env::remove_var("PRIMAL_DISCOVERY_METHOD");
    }
}
