//! Primal Self-Knowledge Module
//!
//! **Core Principle**: "Primals only know themselves, discover others at runtime"
//!
//! This module implements the self-knowledge pattern where each primal:
//! - Discovers its own identity from environment/config (never hardcoded)
//! - Introspects its capabilities (what it can do)
//! - Discovers its endpoints (where it listens, OS-assigned or configured)
//! - Loads runtime configuration (environment-driven)
//!
//! ## Philosophy
//!
//! Traditional approach (hardcoded):
//! ```ignore
//! const MY_NAME: &str = "beardog";  // ❌ Hardcoded self-knowledge
//! const MY_PORT: u16 = 8080;        // ❌ Hardcoded port
//! const SONGBIRD_URL: &str = "..."; // ❌ Hardcoded other primal
//! ```
//!
//! Self-knowledge approach (discovered):
//! ```
//! # use beardog_core::self_knowledge::PrimalSelfKnowledge;
//! # fn main() -> Result<(), Box<dyn std::error::Error>> {
//! let self_knowledge = PrimalSelfKnowledge::discover()?;
//! println!("I am '{}'", self_knowledge.my_name());
//! println!("I listen on {:?}", self_knowledge.my_endpoints());
//! # Ok(())
//! # }
//! ```
//!
//! ## Usage
//!
//! ```no_run
//! use beardog_core::self_knowledge::PrimalSelfKnowledge;
//! use std::env;
//!
//! # async fn example() -> Result<(), beardog_errors::BearDogError> {
//! // Set identity via environment
//! env::set_var("PRIMAL_NAME", "my-beardog-instance");
//! env::set_var("BEARDOG_LISTEN_ADDR", "127.0.0.1:9000");
//!
//! // Discover self-knowledge at runtime
//! let self_knowledge = PrimalSelfKnowledge::discover()?;
//!
//! // Use self-knowledge (zero assumptions!)
//! println!("Starting {} on {:?}",
//!     self_knowledge.my_name(),
//!     self_knowledge.my_endpoints()
//! );
//! # Ok(())
//! # }
//! ```

use std::env;
use std::net::{SocketAddr, ToSocketAddrs};

use beardog_errors::BearDogError;
use serde::{Deserialize, Serialize};
use tracing::{debug, info, warn};

/// Simple capability enumeration
///
/// Represents what this primal can do. This is a simplified version
/// for the self-knowledge pattern. The full capability system is in
/// beardog-capabilities crate.
#[derive(Debug, Clone, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub enum SimpleCapability {
    /// Secure tunneling capability
    SecureTunneling,
    /// Genetic lineage generation/verification
    GeneticLineage,
    /// General cryptographic operations
    Cryptography,
    /// HSM integration
    HsmIntegration,
    /// Discovery services
    Discovery,
}

// =============================================================================
// Public API
// =============================================================================

/// Primal's self-knowledge (discovered at runtime)
///
/// This struct contains everything a primal knows about itself,
/// discovered from:
/// - Environment variables
/// - Configuration files
/// - OS introspection
/// - Capability registration
///
/// **Zero hardcoded values** - everything is discovered at runtime.
#[derive(Debug, Clone)]
pub struct PrimalSelfKnowledge {
    /// My identity (from env or config)
    identity: PrimalIdentity,

    /// My capabilities (what I can do)
    capabilities: Vec<SimpleCapability>,

    /// My endpoints (where I listen)
    endpoints: Vec<Endpoint>,

    /// My version info
    version: VersionInfo,
}

impl PrimalSelfKnowledge {
    /// Discover self-knowledge at runtime
    ///
    /// This performs zero-assumption discovery of the primal's identity,
    /// capabilities, and configuration.
    ///
    /// # Environment Variables
    ///
    /// - `PRIMAL_NAME` - This primal's name (default: "beardog")
    /// - `BEARDOG_LISTEN_ADDR` - Listen address (default: "127.0.0.1:0")
    /// - `BEARDOG_PORT` - Explicit port override (optional)
    ///
    /// # Returns
    ///
    /// Self-knowledge struct with all discovered information
    ///
    /// # Errors
    ///
    /// Returns error if:
    /// - Environment variables are malformed
    /// - Socket addresses cannot be parsed
    /// - Required configuration is missing
    pub fn discover() -> Result<Self, BearDogError> {
        info!("🔍 Discovering primal self-knowledge...");

        let identity = PrimalIdentity::discover()?;
        debug!("Identity discovered: {}", identity.name);

        let capabilities = discover_capabilities();
        debug!("Capabilities discovered: {} total", capabilities.len());

        let endpoints = discover_endpoints()?;
        debug!("Endpoints discovered: {} total", endpoints.len());

        let version = VersionInfo::discover();
        debug!("Version: {}", version.version);

        Ok(Self {
            identity,
            capabilities,
            endpoints,
            version,
        })
    }

    /// Get this primal's name (never assume it!)
    ///
    /// # Example
    ///
    /// ```no_run
    /// # use beardog_core::self_knowledge::PrimalSelfKnowledge;
    /// # fn example() -> Result<(), beardog_errors::BearDogError> {
    /// let sk = PrimalSelfKnowledge::discover()?;
    /// println!("I am: {}", sk.my_name());
    /// # Ok(())
    /// # }
    /// ```
    #[must_use]
    pub fn my_name(&self) -> &str {
        &self.identity.name
    }

    /// Get this primal's capabilities
    ///
    /// Returns the list of capabilities this primal provides.
    #[must_use]
    pub fn my_capabilities(&self) -> &[SimpleCapability] {
        &self.capabilities
    }

    /// Get this primal's listening endpoints
    ///
    /// Returns where this primal is listening for connections.
    #[must_use]
    pub fn my_endpoints(&self) -> &[Endpoint] {
        &self.endpoints
    }

    /// Get this primal's version information
    #[must_use]
    pub fn my_version(&self) -> &VersionInfo {
        &self.version
    }

    /// Check if this primal provides a specific capability
    #[must_use]
    pub fn provides_capability(&self, capability: &SimpleCapability) -> bool {
        self.capabilities.contains(capability)
    }
}

// =============================================================================
// Supporting Types
// =============================================================================

/// Primal identity (discovered, never hardcoded)
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PrimalIdentity {
    /// Primal name (from `PRIMAL_NAME` env or config)
    pub name: String,

    /// Instance ID (unique identifier for this instance)
    pub instance_id: String,
}

impl PrimalIdentity {
    /// Discover identity from environment/config
    fn discover() -> Result<Self, BearDogError> {
        // Try PRIMAL_NAME first
        let name = env::var("PRIMAL_NAME")
            .or_else(|_| env::var("BEARDOG_NAME"))
            .unwrap_or_else(|_| {
                warn!("No PRIMAL_NAME set, using default 'beardog'");
                "beardog".to_string()
            });

        // Generate unique instance ID (hostname + process ID)
        let hostname = std::env::var("HOSTNAME")
            .or_else(|_| std::env::var("HOST"))
            .unwrap_or_else(|_| "unknown".to_string());

        let pid = std::process::id();
        let instance_id = format!("{hostname}-{pid}");

        Ok(Self { name, instance_id })
    }
}

/// Network endpoint (where this primal listens)
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Endpoint {
    /// Protocol (HTTP, gRPC, etc.)
    pub protocol: Protocol,

    /// Socket address (OS-assigned or configured)
    pub address: SocketAddr,
}

impl Endpoint {
    /// Parse an endpoint from a string
    ///
    /// Supports formats:
    /// - `127.0.0.1:8900` (defaults to HTTP)
    /// - `http://127.0.0.1:8900`
    /// - `grpc://127.0.0.1:8900`
    pub fn parse(s: &str) -> Result<Self, BearDogError> {
        // Check for protocol prefix
        let (protocol, addr_str) = if let Some(rest) = s.strip_prefix("http://") {
            (Protocol::Http, rest)
        } else if let Some(rest) = s.strip_prefix("grpc://") {
            (Protocol::Grpc, rest)
        } else {
            // Default to HTTP for bare addresses
            (Protocol::Http, s)
        };

        // Parse socket address
        let address = addr_str.parse::<SocketAddr>().map_err(|e| {
            BearDogError::network(format!("Invalid endpoint address '{addr_str}': {e}"))
        })?;

        Ok(Self { protocol, address })
    }
}

/// Network protocol
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum Protocol {
    /// HTTP/HTTPS
    Http,
    /// gRPC
    Grpc,
    /// Unix domain socket
    UnixSocket,
}

impl std::fmt::Display for Protocol {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Protocol::Http => write!(f, "HTTP"),
            Protocol::Grpc => write!(f, "gRPC"),
            Protocol::UnixSocket => write!(f, "Unix Socket"),
        }
    }
}

/// Version information
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct VersionInfo {
    /// Cargo package version
    pub version: String,

    /// Git commit hash (if available)
    pub git_hash: Option<String>,

    /// Build timestamp (if available)
    pub build_time: Option<String>,
}

impl VersionInfo {
    fn discover() -> Self {
        Self {
            version: env!("CARGO_PKG_VERSION").to_string(),
            git_hash: option_env!("GIT_HASH").map(String::from),
            build_time: option_env!("BUILD_TIME").map(String::from),
        }
    }
}

// =============================================================================
// Discovery Functions
// =============================================================================

/// Discover this primal's capabilities
///
/// Introspects what capabilities this primal actually implements.
/// This is done at runtime by checking feature flags, available modules, etc.
fn discover_capabilities() -> Vec<SimpleCapability> {
    let mut capabilities = Vec::new();

    // Core capabilities (always available in BearDog)
    capabilities.push(SimpleCapability::SecureTunneling);
    capabilities.push(SimpleCapability::GeneticLineage);
    capabilities.push(SimpleCapability::Cryptography);

    // Conditional capabilities based on features
    #[cfg(feature = "hsm-integration")]
    capabilities.push(SimpleCapability::HsmIntegration);

    #[cfg(feature = "mdns")]
    capabilities.push(SimpleCapability::Discovery);

    debug!("Discovered capabilities: {:?}", capabilities);
    capabilities
}

/// Discover endpoints where this primal listens
///
/// Priority order:
/// 1. Explicit `BEARDOG_LISTEN_ADDR` environment variable
/// 2. `BEARDOG_PORT` environment variable (with 127.0.0.1)
/// 3. Configuration file
/// 4. Default: 127.0.0.1:0 (let OS assign port)
fn discover_endpoints() -> Result<Vec<Endpoint>, BearDogError> {
    let mut endpoints = Vec::new();

    // Try BEARDOG_LISTEN_ADDR first
    if let Ok(addr_str) = env::var("BEARDOG_LISTEN_ADDR") {
        debug!("Using BEARDOG_LISTEN_ADDR: {}", addr_str);

        let addr = addr_str
            .to_socket_addrs()
            .map_err(|e| {
                BearDogError::network(format!("Invalid BEARDOG_LISTEN_ADDR '{addr_str}': {e}"))
            })?
            .next()
            .ok_or_else(|| {
                BearDogError::network(format!(
                    "Could not resolve BEARDOG_LISTEN_ADDR '{addr_str}'"
                ))
            })?;

        endpoints.push(Endpoint {
            protocol: Protocol::Http,
            address: addr,
        });

        return Ok(endpoints);
    }

    // Try BEARDOG_PORT
    if let Ok(port_str) = env::var("BEARDOG_PORT") {
        let port: u16 = port_str.parse().map_err(|e| {
            BearDogError::network(format!("Invalid BEARDOG_PORT '{port_str}': {e}"))
        })?;

        debug!("Using BEARDOG_PORT: {}", port);

        endpoints.push(Endpoint {
            protocol: Protocol::Http,
            address: SocketAddr::from(([127, 0, 0, 1], port)),
        });

        return Ok(endpoints);
    }

    // Default: Let OS assign port (port 0)
    debug!("No explicit endpoint configured, using OS-assigned port");
    #[allow(clippy::expect_used)] // Hardcoded address is guaranteed valid
    endpoints.push(Endpoint {
        protocol: Protocol::Http,
        address: "127.0.0.1:0"
            .parse()
            .expect("hardcoded localhost address should always parse"),
    });

    Ok(endpoints)
}

// =============================================================================
// Tests
// =============================================================================

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_self_knowledge_discovery() {
        // Should succeed even with no env vars set
        let sk = PrimalSelfKnowledge::discover();
        assert!(sk.is_ok());
    }

    #[test]
    fn test_identity_from_env() {
        env::set_var("PRIMAL_NAME", "test-primal");

        let identity = PrimalIdentity::discover().unwrap();
        assert_eq!(identity.name, "test-primal");
        assert!(identity
            .instance_id
            .contains(&std::process::id().to_string()));

        env::remove_var("PRIMAL_NAME");
    }

    #[test]
    fn test_identity_default() {
        env::remove_var("PRIMAL_NAME");
        env::remove_var("BEARDOG_NAME");

        let identity = PrimalIdentity::discover().unwrap();
        assert_eq!(identity.name, "beardog");
    }

    #[test]
    fn test_endpoint_from_listen_addr() {
        env::set_var("BEARDOG_LISTEN_ADDR", "127.0.0.1:9000");

        let endpoints = discover_endpoints().unwrap();
        assert_eq!(endpoints.len(), 1);
        assert_eq!(endpoints[0].address.port(), 9000);
        assert_eq!(endpoints[0].protocol, Protocol::Http);

        env::remove_var("BEARDOG_LISTEN_ADDR");
    }

    #[test]
    fn test_endpoint_from_port() {
        env::remove_var("BEARDOG_LISTEN_ADDR");
        env::set_var("BEARDOG_PORT", "8080");

        let endpoints = discover_endpoints().unwrap();
        assert_eq!(endpoints.len(), 1);
        assert_eq!(endpoints[0].address.port(), 8080);

        env::remove_var("BEARDOG_PORT");
    }

    #[test]
    fn test_endpoint_default() {
        env::remove_var("BEARDOG_LISTEN_ADDR");
        env::remove_var("BEARDOG_PORT");

        let endpoints = discover_endpoints().unwrap();
        assert_eq!(endpoints.len(), 1);
        assert_eq!(endpoints[0].address.port(), 0); // OS-assigned
    }

    #[test]
    fn test_capabilities_discovery() {
        let caps = discover_capabilities();

        // Should always have core capabilities
        assert!(caps.contains(&SimpleCapability::SecureTunneling));
        assert!(caps.contains(&SimpleCapability::GeneticLineage));
        assert!(caps.contains(&SimpleCapability::Cryptography));
    }

    #[test]
    fn test_version_discovery() {
        let version = VersionInfo::discover();
        assert!(!version.version.is_empty());
    }

    #[test]
    fn test_provides_capability() {
        let sk = PrimalSelfKnowledge::discover().unwrap();
        assert!(sk.provides_capability(&SimpleCapability::SecureTunneling));
    }

    #[test]
    fn test_my_name() {
        env::set_var("PRIMAL_NAME", "test-name");
        let sk = PrimalSelfKnowledge::discover().unwrap();
        assert_eq!(sk.my_name(), "test-name");
        env::remove_var("PRIMAL_NAME");
    }
}
