// SPDX-License-Identifier: AGPL-3.0-or-later

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

mod strategies;
mod types;

#[cfg(test)]
pub use strategies::parse_capabilities_str;
pub use types::{DiscoveredPrimal, DiscoveryMethod, DiscoveryQuery, PrimalDiscovery};

use beardog_errors::BearDogError;
use beardog_types::constants::domains::network::ipc_discovery as ipc;
use beardog_types::constants::domains::system::intervals::PRIMAL_DISCOVERY_CACHE_TTL;
use std::collections::HashMap;
use std::time::Duration;
use tracing::{debug, info};

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
    ///
    /// # Errors
    ///
    /// Returns [`beardog_errors::BearDogError`] when `detect_discovery_method` rejects the configured method.
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
                let registry_addr = ipc::resolve_upa_registry_endpoint_from_env();
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
                let registry_addr = ipc::resolve_upa_registry_endpoint_from_env();
                info!(
                    "Using multi-method discovery (environment + platform sockets → UPA at {})",
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
    ///
    /// # Errors
    ///
    /// Returns an error if discovery fails.
    #[cfg(test)]
    pub fn discover_with_env(
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
            other => {
                tracing::warn!(
                    "discover_with_env does not support {:?}; use discover() for UPA, mDNS, or DNS-SD",
                    other
                );
                Err(BearDogError::not_implemented(&format!(
                    "discover_with_env does not support {other:?}; use discover() instead"
                )))
            }
        }
    }

    /// Discover primals matching query
    ///
    /// # Errors
    ///
    /// Returns [`BearDogError`] when environment, UPA, mDNS, DNS-SD, or multi-method discovery fails.
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
}

#[cfg(test)]
#[path = "../primal_discovery_tests.rs"]
mod tests;
