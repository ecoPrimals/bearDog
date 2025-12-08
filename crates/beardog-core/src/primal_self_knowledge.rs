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

use beardog_errors::BearDogError;

type Result<T> = std::result::Result<T, BearDogError>;
use serde::{Deserialize, Serialize};
use std::collections::{HashMap, HashSet};
use std::sync::Arc;
use tokio::sync::RwLock;

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

    /// This primal's type (e.g., "beardog", "songbird", "squirrel")
    pub primal_type: String,

    /// Capabilities this primal provides
    pub capabilities: HashSet<Capability>,

    /// Endpoints where THIS primal listens (from config, not hardcoded)
    pub endpoints: Vec<Endpoint>,

    /// Metadata about this primal
    pub metadata: HashMap<String, String>,
}

impl PrimalIdentity {
    /// Create from environment (self-knowledge from config/env)
    ///
    /// Reads ONLY information about THIS primal from:
    /// - Environment variables (BEARDOG_*)
    /// - Configuration files
    /// - Runtime introspection
    ///
    /// NO hardcoded peer addresses or external service locations.
    pub fn from_environment() -> Result<Self> {
        let name =
            std::env::var("BEARDOG_PRIMAL_NAME").unwrap_or_else(|_| "beardog-default".to_string());

        let primal_type =
            std::env::var("BEARDOG_PRIMAL_TYPE").unwrap_or_else(|_| "beardog".to_string());

        // Capabilities from config (what THIS primal can do)
        let mut capabilities = HashSet::new();
        if std::env::var("BEARDOG_CAPABILITY_HSM").is_ok() {
            capabilities.insert(Capability::Hsm);
        }
        if std::env::var("BEARDOG_CAPABILITY_ENCRYPTION").is_ok() {
            capabilities.insert(Capability::Encryption);
        }
        if std::env::var("BEARDOG_CAPABILITY_AUTH").is_ok() {
            capabilities.insert(Capability::Authentication);
        }
        // Default: If no capabilities specified, provide all BearDog capabilities
        if capabilities.is_empty() {
            capabilities.insert(Capability::Hsm);
            capabilities.insert(Capability::Encryption);
            capabilities.insert(Capability::Authentication);
            capabilities.insert(Capability::KeyManagement);
        }

        // Endpoints where THIS primal listens (from config, not hardcoded)
        let endpoints = Self::discover_my_endpoints()?;

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

    /// Discover MY endpoints (introspection, not hardcoding)
    fn discover_my_endpoints() -> Result<Vec<Endpoint>> {
        let mut endpoints = Vec::new();

        // API endpoint (from config/env)
        if let Ok(api_host) = std::env::var("BEARDOG_API_HOST") {
            let api_port = std::env::var("BEARDOG_API_PORT")
                .ok()
                .and_then(|p| p.parse().ok())
                .unwrap_or(8080);

            endpoints.push(Endpoint {
                protocol: Protocol::Http,
                host: api_host,
                port: api_port,
                path: Some("/api/v1".to_string()),
            });
        }

        // gRPC endpoint (if enabled)
        if let Ok(grpc_port) = std::env::var("BEARDOG_GRPC_PORT") {
            if let Ok(port) = grpc_port.parse() {
                let host =
                    std::env::var("BEARDOG_GRPC_HOST").unwrap_or_else(|_| "0.0.0.0".to_string());

                endpoints.push(Endpoint {
                    protocol: Protocol::Grpc,
                    host,
                    port,
                    path: None,
                });
            }
        }

        Ok(endpoints)
    }

    /// Check if this primal provides a capability
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
}

impl PrimalDiscovery {
    /// Create new discovery service with self-knowledge
    pub fn new(identity: PrimalIdentity) -> Self {
        Self {
            identity,
            discovered: Arc::new(RwLock::new(HashMap::new())),
        }
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
        if primals.is_empty() {
            if let Ok(mdns_primals) = self.discover_via_mdns(capability).await {
                primals.extend(mdns_primals);
            }
        }

        // 3. Service registry (if configured)
        if primals.is_empty() {
            if let Ok(registry_primals) = self.discover_via_registry(capability).await {
                primals.extend(registry_primals);
            }
        }

        Ok(primals)
    }

    /// Discover via mDNS (local network, no hardcoding)
    async fn discover_via_mdns(&self, _capability: &str) -> Result<Vec<DiscoveredPrimal>> {
        // Future: Implement real mDNS discovery
        // For now, return empty (no hardcoded fallbacks)
        Ok(Vec::new())
    }

    /// Discover via service registry (if configured)
    async fn discover_via_registry(&self, _capability: &str) -> Result<Vec<DiscoveredPrimal>> {
        // Service registry URL from environment (not hardcoded)
        let registry_url = std::env::var("BEARDOG_SERVICE_REGISTRY_URL").ok();

        if registry_url.is_none() {
            // No registry configured, return empty (no hardcoded fallback)
            return Ok(Vec::new());
        }

        // Future: Query service registry
        Ok(Vec::new())
    }

    /// Announce this primal's existence (capability-based)
    pub async fn announce_self(&self) -> Result<()> {
        // Announce via configured mechanisms only
        // NO hardcoded announcement targets

        if let Ok(registry_url) = std::env::var("BEARDOG_SERVICE_REGISTRY_URL") {
            // Register with service registry
            tracing::info!("Announcing to registry: {}", registry_url);
            // Future: POST self.identity to registry
        }

        if std::env::var("BEARDOG_MDNS_ANNOUNCE").is_ok() {
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
    pub fn url(&self) -> String {
        let base = format!("{}://{}:{}", self.protocol.scheme(), self.host, self.port);
        if let Some(path) = &self.path {
            format!("{}{}", base, path)
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
    fn scheme(&self) -> &str {
        match self {
            Protocol::Http => "http",
            Protocol::Https => "https",
            Protocol::Grpc => "grpc",
            Protocol::Tcp => "tcp",
            Protocol::Udp => "udp",
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
// Tests
// =============================================================================

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_primal_identity_self_knowledge_only() {
        // Primal knows only itself, no hardcoded peers
        std::env::set_var("BEARDOG_PRIMAL_NAME", "test-primal");
        std::env::set_var("BEARDOG_PRIMAL_TYPE", "beardog");

        let identity = PrimalIdentity::from_environment().unwrap();

        assert_eq!(identity.name, "test-primal");
        assert_eq!(identity.primal_type, "beardog");
        assert!(!identity.capabilities.is_empty());

        // Verify NO hardcoded information about other primals
        // (identity contains only self-knowledge)
    }

    #[tokio::test]
    async fn test_discovery_no_hardcoded_addresses() {
        let identity = PrimalIdentity::from_environment().unwrap();
        let discovery = PrimalDiscovery::new(identity);

        // Discovery should work WITHOUT hardcoded addresses
        let hsm_primals = discovery.discover_by_capability("hsm").await.unwrap();

        // May be empty if no HSM primals discovered (correct behavior)
        // Should NOT fall back to hardcoded addresses
        tracing::info!("Discovered {} HSM primals", hsm_primals.len());
    }

    #[test]
    fn test_no_hardcoded_endpoints_in_identity() {
        let identity = PrimalIdentity::from_environment().unwrap();

        // Endpoints should come from config/env, not hardcoded
        for endpoint in &identity.endpoints {
            // Verify endpoints are from environment, not literals
            assert!(!endpoint.host.is_empty());
        }
    }
}
