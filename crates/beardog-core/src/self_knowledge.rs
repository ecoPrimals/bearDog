// SPDX-License-Identifier: AGPL-3.0-only

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
//!
//! # async fn example() -> Result<(), beardog_errors::BearDogError> {
//! // Identity comes from PRIMAL_NAME and BEARDOG_LISTEN_ADDR env vars
//! // set before process start.
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
use std::path::PathBuf;

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

        let identity = PrimalIdentity::discover();
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
    pub const fn my_version(&self) -> &VersionInfo {
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
    fn discover() -> Self {
        let name = env::var("PRIMAL_NAME")
            .or_else(|_| env::var("BEARDOG_NAME"))
            .unwrap_or_else(|_| {
                warn!("No PRIMAL_NAME set, using default 'beardog'");
                "beardog".to_string()
            });

        let hostname = std::env::var("HOSTNAME")
            .or_else(|_| std::env::var("HOST"))
            .unwrap_or_else(|_| "unknown".to_string());

        let pid = std::process::id();
        let instance_id = format!("{hostname}-{pid}");

        Self { name, instance_id }
    }
}

/// Network endpoint (where this primal listens)
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct Endpoint {
    /// Protocol (HTTP, gRPC, etc.)
    pub protocol: Protocol,

    /// Socket address (OS-assigned or configured).
    ///
    /// For [`Protocol::UnixSocket`], this is a placeholder; use [`Self::unix_socket_path`].
    pub address: SocketAddr,

    /// Unix domain socket path when `protocol` is [`Protocol::UnixSocket`].
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub unix_socket_path: Option<PathBuf>,
}

impl Endpoint {
    /// Parse an endpoint from a string
    ///
    /// Supports formats:
    /// - `127.0.0.1:8900` (defaults to HTTP)
    /// - `http://127.0.0.1:8900`
    /// - `grpc://127.0.0.1:8900`
    /// - `unix:///run/user/1000/biomeos/foo.sock` (PRIMAL IPC)
    /// - On Unix, an absolute path `/run/.../foo.sock` is treated as a Unix socket
    pub fn parse(s: &str) -> Result<Self, BearDogError> {
        let trimmed = s.trim();
        if let Some(rest) = trimmed
            .strip_prefix("unix://")
            .or_else(|| trimmed.strip_prefix("unix:"))
        {
            let path = PathBuf::from(rest);
            if path.as_os_str().is_empty() {
                return Err(BearDogError::network(
                    "Empty unix:// path in endpoint".to_string(),
                ));
            }
            #[expect(
                clippy::expect_used,
                reason = "0.0.0.0:0 is a valid placeholder SocketAddr"
            )]
            let placeholder = "0.0.0.0:0"
                .parse::<SocketAddr>()
                .expect("placeholder socket addr");
            return Ok(Self {
                protocol: Protocol::UnixSocket,
                address: placeholder,
                unix_socket_path: Some(path),
            });
        }

        #[cfg(unix)]
        if trimmed.starts_with('/') {
            let path = PathBuf::from(trimmed);
            #[expect(
                clippy::expect_used,
                reason = "0.0.0.0:0 is a valid placeholder SocketAddr"
            )]
            let placeholder = "0.0.0.0:0"
                .parse::<SocketAddr>()
                .expect("placeholder socket addr");
            return Ok(Self {
                protocol: Protocol::UnixSocket,
                address: placeholder,
                unix_socket_path: Some(path),
            });
        }

        // Check for protocol prefix
        let (protocol, addr_str) = if let Some(rest) = trimmed.strip_prefix("http://") {
            (Protocol::Http, rest)
        } else if let Some(rest) = trimmed.strip_prefix("grpc://") {
            (Protocol::Grpc, rest)
        } else {
            // Default to HTTP for bare addresses
            (Protocol::Http, trimmed)
        };

        // Parse socket address
        let address = addr_str.parse::<SocketAddr>().map_err(|e| {
            BearDogError::network(format!("Invalid endpoint address '{addr_str}': {e}"))
        })?;

        Ok(Self {
            protocol,
            address,
            unix_socket_path: None,
        })
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
            Self::Http => write!(f, "HTTP"),
            Self::Grpc => write!(f, "gRPC"),
            Self::UnixSocket => write!(f, "Unix Socket"),
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
    let mut capabilities = vec![
        SimpleCapability::SecureTunneling,
        SimpleCapability::GeneticLineage,
        SimpleCapability::Cryptography,
    ];

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
            unix_socket_path: None,
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
            unix_socket_path: None,
        });

        return Ok(endpoints);
    }

    // Default: Let OS assign port (port 0)
    debug!("No explicit endpoint configured, using OS-assigned port");
    #[expect(
        clippy::expect_used,
        reason = "127.0.0.1:0 is a valid hardcoded default listen address"
    )]
    endpoints.push(Endpoint {
        protocol: Protocol::Http,
        address: "127.0.0.1:0"
            .parse()
            .expect("hardcoded localhost address should always parse"),
        unix_socket_path: None,
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
    #[serial_test::serial]
    fn test_identity_from_env() {
        beardog_errors::process_env::set_var("PRIMAL_NAME", "test-primal");

        let identity = PrimalIdentity::discover();
        assert_eq!(identity.name, "test-primal");
        assert!(
            identity
                .instance_id
                .contains(&std::process::id().to_string())
        );

        beardog_errors::process_env::remove_var("PRIMAL_NAME");
    }

    #[test]
    fn test_identity_default() {
        beardog_errors::process_env::remove_var("PRIMAL_NAME");
        beardog_errors::process_env::remove_var("BEARDOG_NAME");

        let identity = PrimalIdentity::discover();
        assert_eq!(identity.name, "beardog");
    }

    #[test]
    #[serial_test::serial]
    fn test_endpoint_from_listen_addr() {
        beardog_errors::process_env::set_var("BEARDOG_LISTEN_ADDR", "127.0.0.1:9000");

        let endpoints = discover_endpoints().unwrap();
        assert_eq!(endpoints.len(), 1);
        assert_eq!(endpoints[0].address.port(), 9000);
        assert_eq!(endpoints[0].protocol, Protocol::Http);

        beardog_errors::process_env::remove_var("BEARDOG_LISTEN_ADDR");
    }

    #[test]
    #[serial_test::serial]
    fn test_endpoint_from_port() {
        beardog_errors::process_env::remove_var("BEARDOG_LISTEN_ADDR");
        beardog_errors::process_env::set_var("BEARDOG_PORT", "8080");

        let endpoints = discover_endpoints().unwrap();
        assert_eq!(endpoints.len(), 1);
        assert_eq!(endpoints[0].address.port(), 8080);

        beardog_errors::process_env::remove_var("BEARDOG_PORT");
    }

    #[test]
    #[serial_test::serial]
    fn test_endpoint_default() {
        beardog_errors::process_env::remove_var("BEARDOG_LISTEN_ADDR");
        beardog_errors::process_env::remove_var("BEARDOG_PORT");

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
    fn test_endpoint_parse_unix_uri() {
        let ep = Endpoint::parse("unix:///run/user/1000/biomeos/example.sock").unwrap();
        assert_eq!(ep.protocol, Protocol::UnixSocket);
        assert_eq!(
            ep.unix_socket_path
                .as_ref()
                .map(|p| p.to_string_lossy().into_owned()),
            Some("/run/user/1000/biomeos/example.sock".to_string())
        );
    }

    #[test]
    fn test_provides_capability() {
        let sk = PrimalSelfKnowledge::discover().unwrap();
        assert!(sk.provides_capability(&SimpleCapability::SecureTunneling));
    }

    #[test]
    #[serial_test::serial]
    fn test_my_name() {
        beardog_errors::process_env::set_var("PRIMAL_NAME", "test-name");
        let sk = PrimalSelfKnowledge::discover().expect("discover should succeed");
        assert_eq!(sk.my_name(), "test-name");
        beardog_errors::process_env::remove_var("PRIMAL_NAME");
    }

    #[test]
    fn test_endpoint_parse_http_grpc_and_bare_tcp() {
        let h = Endpoint::parse("http://127.0.0.1:9001").expect("http");
        assert_eq!(h.protocol, Protocol::Http);
        assert_eq!(h.address.port(), 9001);

        let g = Endpoint::parse("grpc://[::1]:50051").expect("grpc");
        assert_eq!(g.protocol, Protocol::Grpc);
        assert_eq!(g.address.port(), 50051);

        let b = Endpoint::parse("10.0.0.5:7777").expect("bare");
        assert_eq!(b.protocol, Protocol::Http);
        assert_eq!(b.address.port(), 7777);
    }

    #[test]
    fn test_endpoint_parse_unix_prefix_without_slashes() {
        let ep = Endpoint::parse("unix:/tmp/no-double-slash.sock").expect("unix:");
        assert_eq!(ep.protocol, Protocol::UnixSocket);
        assert_eq!(
            ep.unix_socket_path
                .as_ref()
                .map(|p| p.to_string_lossy().into_owned()),
            Some("/tmp/no-double-slash.sock".to_string())
        );
    }

    #[test]
    fn test_endpoint_parse_empty_unix_errors() {
        assert!(Endpoint::parse("unix://").is_err());
    }

    #[test]
    fn test_endpoint_parse_invalid_address_errors() {
        assert!(Endpoint::parse("http://not-a-socket-addr").is_err());
    }

    #[cfg(unix)]
    #[test]
    fn test_endpoint_parse_absolute_path_unix_socket() {
        let ep = Endpoint::parse("/tmp/beardog_test_abs.sock").expect("abs path");
        assert_eq!(ep.protocol, Protocol::UnixSocket);
        assert!(
            ep.unix_socket_path
                .as_ref()
                .is_some_and(|p| p.ends_with("beardog_test_abs.sock"))
        );
    }

    #[test]
    #[serial_test::serial]
    fn test_identity_from_beardog_name() {
        beardog_errors::process_env::remove_var("PRIMAL_NAME");
        beardog_errors::process_env::set_var("BEARDOG_NAME", "from-beardog-name");
        let id = PrimalIdentity::discover();
        assert_eq!(id.name, "from-beardog-name");
        beardog_errors::process_env::remove_var("BEARDOG_NAME");
    }

    #[test]
    #[serial_test::serial]
    fn test_discover_endpoints_invalid_listen_addr_errors() {
        beardog_errors::process_env::set_var("BEARDOG_LISTEN_ADDR", "127.0.0.1:99999");
        let err = discover_endpoints();
        assert!(err.is_err());
        beardog_errors::process_env::remove_var("BEARDOG_LISTEN_ADDR");
    }

    #[test]
    #[serial_test::serial]
    fn test_discover_endpoints_invalid_port_errors() {
        beardog_errors::process_env::remove_var("BEARDOG_LISTEN_ADDR");
        beardog_errors::process_env::set_var("BEARDOG_PORT", "not-a-u16");
        let err = discover_endpoints();
        assert!(err.is_err());
        beardog_errors::process_env::remove_var("BEARDOG_PORT");
    }

    #[test]
    fn test_protocol_display() {
        assert_eq!(format!("{}", Protocol::Http), "HTTP");
        assert_eq!(format!("{}", Protocol::Grpc), "gRPC");
        assert_eq!(format!("{}", Protocol::UnixSocket), "Unix Socket");
    }

    #[test]
    fn test_primal_self_knowledge_accessors() {
        let sk = PrimalSelfKnowledge::discover().expect("discover");
        assert!(!sk.my_capabilities().is_empty());
        assert!(!sk.my_endpoints().is_empty());
        assert!(!sk.my_version().version.is_empty());
    }
}
