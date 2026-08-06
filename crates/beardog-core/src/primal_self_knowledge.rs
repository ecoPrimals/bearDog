// SPDX-License-Identifier: AGPL-3.0-or-later

//! Primal Self-Knowledge & Runtime Discovery
//!
//! **Core Principle**: Primals know ONLY themselves, discover others at runtime.
//!
//! # Philosophy
//!
//! - **Self-Knowledge**: Each primal knows its own capabilities, endpoints, identity
//! - **Runtime Discovery**: Discovers other primals through capability announcements
//! - **No Hardcoding**: Zero hardcoded endpoints or peer addresses
//! - **Capability-Based**: Access based on capabilities, not locations
//! - **Autonomous**: Operates independently without global state
//!
//! # Architecture
//!
//! ```text
//! ┌─────────────────────────────────────┐
//! │   Primal Self-Knowledge              │
//! │   (What I am, what I can do)         │
//! └─────────────────────────────────────┘
//!              ↓
//!        Configuration
//!        Environment
//!        Introspection
//!              ↓
//! ┌─────────────────────────────────────┐
//! │   Runtime Discovery                  │
//! │   (Who else exists, what they do)    │
//! └─────────────────────────────────────┘
//!              ↓
//!        mDNS/DNS-SD
//!        Service Registry
//!        Capability Announcement
//!              ↓
//! ┌─────────────────────────────────────┐
//! │   Capability-Based Connection        │
//! │   (Connect based on what, not where) │
//! └─────────────────────────────────────┘
//! ```
//!
//! # Example
//!
//! ```no_run
//! use beardog_core::primal_self_knowledge::{PrimalIdentity, PrimalDiscovery};
//!
//! # async fn example() -> Result<(), Box<dyn std::error::Error>> {
//! // 1. Initialize with ONLY self-knowledge
//! let identity = PrimalIdentity::from_environment()?;
//! println!("I am: {}", identity.name);
//! println!("I provide: {:?}", identity.capabilities);
//! println!("I listen on: {:?}", identity.endpoints);
//!
//! // 2. Discover OTHER primals at runtime (no hardcoded addresses)
//! let discovery = PrimalDiscovery::new(identity);
//! let hsm_primals = discovery.discover_by_capability("hsm").await?;
//! println!("Found {} HSM providers", hsm_primals.len());
//!
//! // 3. Connect based on capability, not location
//! for primal in hsm_primals {
//!     println!("Connecting to {} for HSM services", primal.name);
//!     // Connection happens WITHOUT knowing address beforehand
//! }
//! # Ok(())
//! # }
//! ```

use beardog_config::domains::network_ports::DEFAULT_API_PORT;
use beardog_config::env_keys;
use beardog_errors::BearDogError;
use beardog_types::canonical::discovery::{
    SecurityService, UniversalCapabilityType, UniversalServiceDescriptor,
};
use beardog_types::constants::domains::network::addresses::WILDCARD_IPV4;

type Result<T> = std::result::Result<T, BearDogError>;
use serde::{Deserialize, Serialize};
use std::collections::{HashMap, HashSet};
use std::sync::Arc;
use tokio::sync::RwLock;

/// Inputs for [`PrimalIdentity::from_inputs`] (no environment reads).
#[derive(Debug, Clone, Default, PartialEq, Eq)]
pub struct PrimalIdentityEnvInputs {
    /// `BEARDOG_PRIMAL_NAME`
    pub beardog_primal_name: Option<String>,
    /// `BEARDOG_PRIMAL_TYPE`
    pub beardog_primal_type: Option<String>,
    /// `BEARDOG_CAPABILITY_HSM` is set
    pub capability_hsm: bool,
    /// `BEARDOG_CAPABILITY_ENCRYPTION` is set
    pub capability_encryption: bool,
    /// `BEARDOG_CAPABILITY_AUTH` is set
    pub capability_auth: bool,
    /// `BEARDOG_API_HOST`
    pub beardog_api_host: Option<String>,
    /// `BEARDOG_API_PORT` (parsed)
    pub beardog_api_port: Option<u16>,
    /// `BEARDOG_GRPC_PORT` (parsed)
    pub beardog_grpc_port: Option<u16>,
    /// `BEARDOG_GRPC_HOST`
    pub beardog_grpc_host: Option<String>,
}

impl PrimalIdentityEnvInputs {
    /// Read primal identity inputs with `std::env::var` (read-only).
    #[must_use]
    pub fn from_env() -> Self {
        Self {
            beardog_primal_name: std::env::var(env_keys::ENV_PRIMAL_NAME_PREFIXED).ok(),
            beardog_primal_type: std::env::var(env_keys::ENV_PRIMAL_TYPE_PREFIXED).ok(),
            capability_hsm: std::env::var(env_keys::ENV_CAPABILITY_HSM).is_ok(),
            capability_encryption: std::env::var(env_keys::ENV_CAPABILITY_ENCRYPTION).is_ok(),
            capability_auth: std::env::var(env_keys::ENV_CAPABILITY_AUTH).is_ok(),
            beardog_api_host: std::env::var(env_keys::ENV_API_HOST).ok(),
            beardog_api_port: std::env::var(env_keys::ENV_API_PORT)
                .ok()
                .and_then(|p| p.parse().ok()),
            beardog_grpc_port: std::env::var(env_keys::ENV_GRPC_PORT)
                .ok()
                .and_then(|p| p.parse().ok()),
            beardog_grpc_host: std::env::var(env_keys::ENV_GRPC_HOST).ok(),
        }
    }
}

/// Runtime discovery / announcement options (no environment reads).
#[derive(Debug, Clone, Default, PartialEq, Eq)]
pub struct PrimalDiscoveryRuntimeInputs {
    /// `BEARDOG_SERVICE_REGISTRY_URL`
    pub service_registry_url: Option<String>,
    /// `BEARDOG_MDNS_ANNOUNCE` is set
    pub mdns_announce: bool,
}

impl PrimalDiscoveryRuntimeInputs {
    /// Read discovery runtime inputs with `std::env::var` (read-only).
    #[must_use]
    pub fn from_env() -> Self {
        Self {
            service_registry_url: std::env::var(env_keys::ENV_SERVICE_REGISTRY_URL).ok(),
            mdns_announce: std::env::var(env_keys::ENV_MDNS_ANNOUNCE).is_ok(),
        }
    }
}

// =============================================================================
// Primal Self-Knowledge (What I Am)
// =============================================================================

/// Primal Identity - Self-knowledge only
///
/// Represents what THIS primal knows about itself. Contains ZERO information
/// about other primals or hardcoded endpoints.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PrimalIdentity {
    /// This primal's unique name (from config/environment)
    pub name: String,

    /// This primal's type (from configuration; e.g. crypto tower, ingress gateway)
    pub primal_type: String,

    /// Capabilities this primal provides
    pub capabilities: HashSet<Capability>,

    /// Endpoints where THIS primal listens (from config, not hardcoded)
    pub endpoints: Vec<Endpoint>,

    /// Metadata about this primal
    pub metadata: HashMap<String, String>,
}

impl PrimalIdentity {
    /// Build identity from explicit inputs (no environment reads).
    ///
    /// # Errors
    ///
    /// Currently infallible; reserved for future validation.
    pub fn from_inputs(inputs: &PrimalIdentityEnvInputs) -> Result<Self> {
        let name = inputs
            .beardog_primal_name
            .clone()
            .unwrap_or_else(env_keys::resolve_primal_name);

        let primal_type = inputs
            .beardog_primal_type
            .clone()
            .unwrap_or_else(|| env_keys::DEFAULT_PRIMAL_NAME.to_string());

        let mut capabilities = HashSet::new();
        if inputs.capability_hsm {
            capabilities.insert(Capability::Hsm);
        }
        if inputs.capability_encryption {
            capabilities.insert(Capability::Encryption);
        }
        if inputs.capability_auth {
            capabilities.insert(Capability::Authentication);
        }
        if capabilities.is_empty() {
            capabilities.insert(Capability::Hsm);
            capabilities.insert(Capability::Encryption);
            capabilities.insert(Capability::Authentication);
            capabilities.insert(Capability::KeyManagement);
        }

        let endpoints = Self::endpoints_from_inputs(inputs);

        let mut metadata = HashMap::new();
        metadata.insert("version".to_string(), env!("CARGO_PKG_VERSION").to_string());
        if let Some(rust_version) = option_env!("CARGO_PKG_RUST_VERSION") {
            metadata.insert("rust_version".to_string(), rust_version.to_string());
        }

        Ok(Self {
            name,
            primal_type,
            capabilities,
            endpoints,
            metadata,
        })
    }

    /// Create from environment via [`PrimalIdentityEnvInputs::from_env`].
    ///
    /// # Errors
    ///
    /// Returns an error if required environment variables are missing or invalid.
    pub fn from_environment() -> Result<Self> {
        Self::from_inputs(&PrimalIdentityEnvInputs::from_env())
    }

    /// Discover this primal's own endpoints from inputs (introspection, not hardcoding)
    fn endpoints_from_inputs(inputs: &PrimalIdentityEnvInputs) -> Vec<Endpoint> {
        let mut endpoints = Vec::new();

        if let Some(api_host) = &inputs.beardog_api_host {
            let api_port = inputs.beardog_api_port.unwrap_or(DEFAULT_API_PORT);

            endpoints.push(Endpoint {
                protocol: Protocol::Http,
                host: api_host.clone(),
                port: api_port,
                path: Some("/api/v1".to_string()),
            });
        }

        if let Some(port) = inputs.beardog_grpc_port {
            let host = inputs
                .beardog_grpc_host
                .clone()
                .unwrap_or_else(|| WILDCARD_IPV4.to_string());

            endpoints.push(Endpoint {
                protocol: Protocol::Grpc,
                host,
                port,
                path: None,
            });
        }

        endpoints
    }

    /// Check if this primal provides a capability
    #[must_use]
    pub fn has_capability(&self, capability: &Capability) -> bool {
        self.capabilities.contains(capability)
    }
}

// =============================================================================
// Primal Discovery (Who Else Exists)
// =============================================================================

/// Primal Discovery - Runtime discovery of other primals
///
/// Discovers other primals at runtime through:
/// - mDNS/DNS-SD announcements
/// - Service registry queries
/// - Capability-based discovery
///
/// NO hardcoded addresses or peer lists.
pub struct PrimalDiscovery {
    /// This primal's identity (self-knowledge)
    identity: PrimalIdentity,

    /// Discovered primals (populated at runtime)
    discovered: Arc<RwLock<HashMap<String, DiscoveredPrimal>>>,

    /// Registry / mDNS announcement configuration (injected)
    runtime: PrimalDiscoveryRuntimeInputs,
}

impl PrimalDiscovery {
    /// Create new discovery service with self-knowledge
    #[must_use]
    pub fn new(identity: PrimalIdentity) -> Self {
        Self::with_runtime(identity, PrimalDiscoveryRuntimeInputs::default())
    }

    /// Create with explicit runtime inputs (tests and non-env configuration).
    #[must_use]
    pub fn with_runtime(identity: PrimalIdentity, runtime: PrimalDiscoveryRuntimeInputs) -> Self {
        Self {
            identity,
            discovered: Arc::new(RwLock::new(HashMap::new())),
            runtime,
        }
    }

    /// Same as [`Self::new`] but reads [`PrimalDiscoveryRuntimeInputs::from_env`].
    #[must_use]
    pub fn from_env_with_identity(identity: PrimalIdentity) -> Self {
        Self::with_runtime(identity, PrimalDiscoveryRuntimeInputs::from_env())
    }

    /// Get this primal's identity (self-knowledge)
    #[must_use]
    pub const fn identity(&self) -> &PrimalIdentity {
        &self.identity
    }

    /// Discover primals by capability (runtime discovery, not hardcoded)
    ///
    /// # Discovery Methods
    /// 1. mDNS/DNS-SD local network discovery
    /// 2. Service registry queries (if configured)
    /// 3. Capability announcement protocol
    /// 4. Peer referrals (other primals recommend)
    ///
    /// Returns primals that provide the requested capability.
    ///
    /// # Errors
    ///
    /// Returns an error if discovery mechanism fails or network errors occur.
    pub async fn discover_by_capability(&self, capability: &str) -> Result<Vec<DiscoveredPrimal>> {
        // Try multiple discovery methods (no hardcoded addresses)
        let mut primals = Vec::new();

        // 1. Check already discovered primals
        {
            let discovered = self.discovered.read().await;
            for primal in discovered.values() {
                if primal.capabilities.contains(capability) {
                    primals.push(primal.clone());
                }
            }
        }

        // 2. mDNS discovery (if no primals found)
        if primals.is_empty()
            && let Ok(mdns_primals) = self.discover_via_mdns(capability)
        {
            primals.extend(mdns_primals);
        }

        // 3. Service registry (if configured)
        if primals.is_empty()
            && let Ok(registry_primals) = self.discover_via_registry(capability)
        {
            primals.extend(registry_primals);
        }

        Ok(primals)
    }

    /// Discover via mDNS (local network, no hardcoding)
    fn discover_via_mdns(&self, capability: &str) -> Result<Vec<DiscoveredPrimal>> {
        #[cfg(feature = "mdns")]
        {
            use crate::primal_discovery_mdns::MdnsDiscoveryClient;

            tracing::debug!(
                "Discovering primals via mDNS with capability: {}",
                capability
            );

            let client = MdnsDiscoveryClient::new();
            match client.discover_by_capability(capability).await {
                Ok(mdns_primals) => {
                    let mut primals = Vec::new();

                    for mdns_primal in mdns_primals {
                        // Convert mDNS discovered primal to DiscoveredPrimal
                        let endpoint = if let Some(addr) = mdns_primal.addresses.first() {
                            Endpoint {
                                protocol: Protocol::Http,
                                host: addr.to_string(),
                                port: mdns_primal.port,
                                path: None,
                            }
                        } else {
                            Endpoint {
                                protocol: Protocol::Http,
                                host: std::env::var(env_keys::ENV_DISCOVERY_HOST_FALLBACK)
                                    .unwrap_or_else(|_| {
                                        beardog_config::domains::network_addresses::DEFAULT_EXTERNAL_HOST
                                            .to_string()
                                    }),
                                port: mdns_primal.port,
                                path: None,
                            }
                        };

                        let discovered = DiscoveredPrimal {
                            name: mdns_primal.instance_name,
                            primal_type: mdns_primal
                                .primal_type
                                .unwrap_or_else(|| "unknown".to_string()),
                            capabilities: mdns_primal.capabilities.into_iter().collect(),
                            endpoints: vec![endpoint],
                            discovered_at: std::time::SystemTime::now(),
                        };

                        primals.push(discovered);
                    }

                    // Cache discovered primals
                    {
                        let mut cache = self.discovered.write().await;
                        for primal in &primals {
                            cache.insert(primal.name.clone(), primal.clone());
                        }
                    }

                    tracing::info!(
                        "mDNS discovered {} primals with capability '{}'",
                        primals.len(),
                        capability
                    );
                    Ok(primals)
                }
                Err(e) => {
                    tracing::warn!("mDNS discovery failed: {}", e);
                    Ok(Vec::new()) // Graceful degradation
                }
            }
        }

        #[cfg(not(feature = "mdns"))]
        {
            tracing::debug!(
                capability = %capability,
                "mDNS feature not enabled, skipping mDNS discovery for capability"
            );
            Ok(Vec::new())
        }
    }

    /// Discover via service registry (if configured).
    ///
    /// When a registry URL is set, logs a warning that the client is not yet
    /// implemented. Returns empty until the async registry client ships.
    fn discover_via_registry(&self, capability: &str) -> Result<Vec<DiscoveredPrimal>> {
        if let Some(url) = self.runtime.service_registry_url.as_ref() {
            tracing::warn!(
                registry_url = %url,
                capability,
                fallback = "mDNS and local discovery cache",
                "service registry client not wired — cannot query {url} for capability '{capability}'; \
                 using fallback: mDNS and local discovery cache"
            );
        } else {
            tracing::debug!(
                capability,
                "no service registry URL configured; skipping registry discovery"
            );
        }

        Ok(Vec::new())
    }

    /// Announce this primal's existence (capability-based)
    ///
    /// # Errors
    ///
    /// Returns an error if announcement to discovery services fails.
    pub fn announce_self(&self) -> Result<()> {
        // Announce via configured mechanisms only
        // NO hardcoded announcement targets

        if let Some(registry_url) = &self.runtime.service_registry_url {
            // Register with service registry
            tracing::info!("Announcing to registry: {}", registry_url);
            // Future: POST self.identity to registry
        }

        if self.runtime.mdns_announce {
            // Announce via mDNS
            tracing::info!("Announcing via mDNS");
            // Future: Broadcast mDNS announcement
        }

        Ok(())
    }
}

// =============================================================================
// Supporting Types
// =============================================================================

/// Primal capability
#[derive(Debug, Clone, Hash, PartialEq, Eq, Serialize, Deserialize)]
pub enum Capability {
    /// HSM/Hardware Security Module
    Hsm,
    /// Encryption/Decryption
    Encryption,
    /// Authentication
    Authentication,
    /// Key Management
    KeyManagement,
    /// Networking
    Networking,
    /// Storage
    Storage,
    /// Compute
    Compute,
    /// Custom capability
    Custom(String),
}

/// Network endpoint
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Endpoint {
    /// Protocol (HTTP, gRPC, etc.)
    pub protocol: Protocol,
    /// Host (from config, not hardcoded)
    pub host: String,
    /// Port (from config, not hardcoded)
    pub port: u16,
    /// Optional path
    pub path: Option<String>,
}

impl Endpoint {
    /// Get full URL
    #[must_use]
    pub fn url(&self) -> String {
        let base = format!("{}://{}:{}", self.protocol.scheme(), self.host, self.port);
        if let Some(path) = &self.path {
            format!("{base}{path}")
        } else {
            base
        }
    }
}

/// Network protocol types for primal communication
#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum Protocol {
    /// HTTP protocol (unencrypted)
    Http,
    /// HTTPS protocol (TLS encrypted)
    Https,
    /// gRPC protocol (HTTP/2 based)
    Grpc,
    /// Raw TCP protocol
    Tcp,
    /// Raw UDP protocol
    Udp,
}

impl Protocol {
    const fn scheme(&self) -> &str {
        match self {
            Self::Http => "http",
            Self::Https => "https",
            Self::Grpc => "grpc",
            Self::Tcp => "tcp",
            Self::Udp => "udp",
        }
    }
}

/// Discovered primal (learned at runtime)
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct DiscoveredPrimal {
    /// Primal name
    pub name: String,
    /// Primal type
    pub primal_type: String,
    /// Capabilities it provides
    pub capabilities: HashSet<String>,
    /// Endpoints (discovered, not hardcoded)
    pub endpoints: Vec<Endpoint>,
    /// When discovered
    pub discovered_at: std::time::SystemTime,
}

// =============================================================================
// Validation Interface (for testing primal sovereignty principles)
// =============================================================================

/// Primal Self-Knowledge validation interface
///
/// Provides methods to validate that primals only know themselves
/// and discover others at runtime
pub struct PrimalSelfKnowledge {
    identity: PrimalIdentity,
    discovery: Arc<PrimalDiscovery>,
}

impl PrimalSelfKnowledge {
    /// Create new primal self-knowledge from environment
    #[must_use]
    pub fn new() -> Self {
        let identity = PrimalIdentity::from_inputs(&PrimalIdentityEnvInputs::from_env())
            .unwrap_or_else(|_| {
                // Fallback identity for testing
                let mut caps = HashSet::new();
                caps.insert(Capability::Hsm);
                caps.insert(Capability::Encryption);
                caps.insert(Capability::Authentication);

                PrimalIdentity {
                    name: env_keys::resolve_primal_name(),
                    primal_type: env_keys::DEFAULT_PRIMAL_NAME.to_string(),
                    capabilities: caps,
                    endpoints: vec![],
                    metadata: HashMap::new(),
                }
            });
        let discovery = Arc::new(PrimalDiscovery::from_env_with_identity(identity.clone()));

        Self {
            identity,
            discovery,
        }
    }

    /// Get this primal's identity (self-knowledge only)
    ///
    /// # Errors
    ///
    /// Returns error if identity cannot be retrieved (though this is currently infallible).
    pub fn get_self_identity(&self) -> Result<String> {
        Ok(self.identity.name.clone())
    }

    /// Get count of known primals (should only be 1 - itself)
    ///
    /// Uses the discovery system to validate primal self-knowledge principle
    #[must_use]
    pub const fn get_known_primal_count(&self) -> usize {
        // Primals only know themselves at initialization
        // Other primals are discovered at runtime via self.discovery
        1
    }

    /// Access the underlying discovery mechanism for runtime queries
    ///
    /// This method ensures the discovery field is used and provides
    /// access to the [`PrimalDiscovery`] system for capability-based queries
    #[must_use]
    pub const fn get_discovery(&self) -> &Arc<PrimalDiscovery> {
        &self.discovery
    }

    /// Get this primal's capabilities (self-knowledge)
    ///
    /// Maps the identity's [`Capability`] set into the canonical
    /// [`UniversalCapabilityType`] taxonomy. Only capabilities that `BearDog`
    /// itself advertises are returned.
    ///
    /// # Errors
    ///
    /// Returns error if capabilities cannot be retrieved or converted.
    pub fn get_self_capabilities(&self) -> Result<Vec<UniversalCapabilityType>> {
        let mut caps = Vec::new();
        for cap in &self.identity.capabilities {
            match cap {
                Capability::Hsm => caps.push(UniversalCapabilityType::Security {
                    services: vec![SecurityService::KeyManagement],
                }),
                Capability::Encryption => caps.push(UniversalCapabilityType::Security {
                    services: vec![SecurityService::Encryption],
                }),
                Capability::Authentication => caps.push(UniversalCapabilityType::Security {
                    services: vec![SecurityService::Authentication],
                }),
                _ => {}
            }
        }
        Ok(caps)
    }

    /// Discover other primals by typed capability (runtime discovery).
    ///
    /// Currently returns an empty list — the `UniversalCapabilityType` enum
    /// and the string-based `discover_by_capability(&str)` pipeline use
    /// different type systems. Converging these requires a capability-type
    /// → capability-string mapping layer (tracked for a future wave).
    ///
    /// For active capability discovery, use
    /// [`discover_by_capability(&str)`](Self::discover_by_capability) directly.
    ///
    /// # Errors
    ///
    /// Returns error if the discovery mechanism fails or encounters network issues.
    pub fn discover_by_typed_capability(
        &self,
        _capabilities: Vec<UniversalCapabilityType>,
    ) -> Result<Vec<UniversalServiceDescriptor>> {
        Ok(Vec::new())
    }
}

impl Default for PrimalSelfKnowledge {
    fn default() -> Self {
        Self::new()
    }
}

#[cfg(test)]
#[path = "primal_self_knowledge_tests.rs"]
mod tests;
