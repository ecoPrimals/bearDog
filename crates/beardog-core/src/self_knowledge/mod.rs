// SPDX-License-Identifier: AGPL-3.0-or-later

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
//! let self_knowledge = PrimalSelfKnowledge::discover()?;
//! println!("Starting {} on {:?}",
//!     self_knowledge.my_name(),
//!     self_knowledge.my_endpoints()
//! );
//! # Ok(())
//! # }
//! ```

mod capabilities;
mod endpoints;
mod identity;

pub use capabilities::{
    SimpleCapability, discovered_simple_capabilities, ipc_registry_capability_strings,
};
pub use endpoints::{
    Endpoint, EndpointInputs, Protocol, discover_endpoints_from_env, discover_endpoints_from_inputs,
};
pub use identity::{IdentityInputs, PrimalIdentity, VersionInfo};

use beardog_errors::BearDogError;
use tracing::info;

/// Aggregate inputs for all self-knowledge subsystems.
#[derive(Debug, Clone, Default)]
pub struct SelfKnowledgeInputs {
    /// Identity discovery inputs.
    pub identity: IdentityInputs,
    /// Endpoint discovery inputs.
    pub endpoints: EndpointInputs,
}

impl SelfKnowledgeInputs {
    /// Read all inputs from the process environment.
    #[must_use]
    pub fn from_env() -> Self {
        Self {
            identity: IdentityInputs::from_env(),
            endpoints: EndpointInputs::from_env(),
        }
    }
}

/// Primal's self-knowledge (discovered at runtime).
///
/// **Zero hardcoded values** - everything is discovered at runtime.
#[derive(Debug, Clone)]
pub struct PrimalSelfKnowledge {
    identity: PrimalIdentity,
    capabilities: Vec<SimpleCapability>,
    endpoints: Vec<Endpoint>,
    version: VersionInfo,
}

impl PrimalSelfKnowledge {
    /// Discover self-knowledge from explicit inputs (no environment reads).
    ///
    /// # Errors
    ///
    /// Returns error if endpoint addresses cannot be parsed.
    pub fn discover_from_inputs(inputs: &SelfKnowledgeInputs) -> Result<Self, BearDogError> {
        let identity = PrimalIdentity::from_inputs(&inputs.identity);
        let capabilities = capabilities::discover_capabilities();
        let endpoints = discover_endpoints_from_inputs(&inputs.endpoints)?;
        let version = VersionInfo::discover();

        info!(
            name = %identity.name,
            capabilities = %capabilities.len(),
            endpoints = %endpoints.len(),
            "Primal self-knowledge discovered"
        );

        Ok(Self {
            identity,
            capabilities,
            endpoints,
            version,
        })
    }

    /// Discover self-knowledge from the process environment.
    ///
    /// # Errors
    ///
    /// Returns error if environment variables are malformed.
    pub fn discover_from_env() -> Result<Self, BearDogError> {
        Self::discover_from_inputs(&SelfKnowledgeInputs::from_env())
    }

    /// Alias for [`discover_from_env`](Self::discover_from_env).
    ///
    /// # Errors
    ///
    /// Same as [`discover_from_env`](Self::discover_from_env).
    pub fn discover() -> Result<Self, BearDogError> {
        Self::discover_from_env()
    }

    /// This primal's name (discovered, never hardcoded).
    #[must_use]
    pub fn my_name(&self) -> &str {
        &self.identity.name
    }

    /// This primal's capabilities.
    #[must_use]
    pub fn my_capabilities(&self) -> &[SimpleCapability] {
        &self.capabilities
    }

    /// This primal's listen endpoints.
    #[must_use]
    pub fn my_endpoints(&self) -> &[Endpoint] {
        &self.endpoints
    }

    /// Build version information.
    #[must_use]
    pub const fn my_version(&self) -> &VersionInfo {
        &self.version
    }

    /// Check if this primal provides a specific capability.
    #[must_use]
    pub fn provides_capability(&self, capability: &SimpleCapability) -> bool {
        self.capabilities.contains(capability)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_self_knowledge_discovery() {
        let sk = PrimalSelfKnowledge::discover_from_inputs(&SelfKnowledgeInputs::default());
        assert!(sk.is_ok());
    }

    #[test]
    fn test_identity_from_primal_name() {
        let identity = PrimalIdentity::from_inputs(&IdentityInputs {
            primal_name: Some("test-primal".to_string()),
            ..Default::default()
        });
        assert_eq!(identity.name, "test-primal");
        assert!(
            identity
                .instance_id
                .contains(&std::process::id().to_string())
        );
    }

    #[test]
    fn test_identity_default() {
        let identity = PrimalIdentity::from_inputs(&IdentityInputs {
            primal_name: None,
            beardog_name: None,
            hostname: Some("test-host-self-knowledge".to_string()),
            host: None,
        });
        assert_eq!(identity.name, "test-host-self-knowledge");
    }

    #[test]
    fn test_endpoint_from_listen_addr() {
        let endpoints = discover_endpoints_from_inputs(&EndpointInputs {
            beardog_listen_addr: Some("127.0.0.1:0".to_string()),
            ..Default::default()
        })
        .expect("discover_endpoints_from_inputs with listen addr");
        assert_eq!(endpoints.len(), 1);
        assert_eq!(endpoints[0].address.port(), 0);
        assert_eq!(endpoints[0].protocol, Protocol::Http);
    }

    #[test]
    fn test_endpoint_from_port() {
        let endpoints = discover_endpoints_from_inputs(&EndpointInputs {
            beardog_port: Some("0".to_string()),
            ..Default::default()
        })
        .expect("discover_endpoints_from_inputs with port");
        assert_eq!(endpoints.len(), 1);
        assert_eq!(endpoints[0].address.port(), 0);
    }

    #[test]
    fn test_endpoint_default() {
        let endpoints = discover_endpoints_from_inputs(&EndpointInputs::default())
            .expect("default endpoint discovery");
        assert_eq!(endpoints.len(), 1);
        assert_eq!(endpoints[0].address.port(), 0);
    }

    #[test]
    fn test_capabilities_discovery() {
        let caps = capabilities::discover_capabilities();
        assert!(caps.contains(&SimpleCapability::SecureTunneling));
        assert!(caps.contains(&SimpleCapability::GeneticLineage));
        assert!(caps.contains(&SimpleCapability::Cryptography));
    }

    #[test]
    fn ipc_registry_capability_strings_maps_and_sorts() {
        let tags = ipc_registry_capability_strings(&[
            SimpleCapability::Cryptography,
            SimpleCapability::SecureTunneling,
        ]);
        assert!(tags.iter().any(|s| s == "crypto"));
        assert!(tags.iter().any(|s| s == "btsp"));
        assert!(tags.iter().any(|s| s == "ed25519"));
        let sorted = tags.clone();
        let mut cmp = sorted.clone();
        cmp.sort();
        assert_eq!(tags, cmp, "tags should be sorted (BTreeSet order)");
    }

    #[test]
    fn test_version_discovery() {
        let version = VersionInfo::discover();
        assert!(!version.version.is_empty());
    }

    #[test]
    fn test_endpoint_parse_unix_uri() {
        let ep = Endpoint::parse("unix:///run/user/1000/biomeos/example.sock")
            .expect("parse unix URI in test");
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
        let sk = PrimalSelfKnowledge::discover().expect("PrimalSelfKnowledge::discover in test");
        assert!(sk.provides_capability(&SimpleCapability::SecureTunneling));
    }

    #[test]
    fn test_my_name() {
        let sk = PrimalSelfKnowledge::discover_from_inputs(&SelfKnowledgeInputs {
            identity: IdentityInputs {
                primal_name: Some("test-name".to_string()),
                ..Default::default()
            },
            ..Default::default()
        })
        .expect("discover should succeed");
        assert_eq!(sk.my_name(), "test-name");
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
    fn test_identity_from_beardog_name() {
        let id = PrimalIdentity::from_inputs(&IdentityInputs {
            primal_name: None,
            beardog_name: Some("from-beardog-name".to_string()),
            ..Default::default()
        });
        assert_eq!(id.name, "from-beardog-name");
    }

    #[test]
    fn test_discover_endpoints_invalid_listen_addr_errors() {
        let err = discover_endpoints_from_inputs(&EndpointInputs {
            beardog_listen_addr: Some("127.0.0.1:99999".to_string()),
            ..Default::default()
        });
        assert!(err.is_err());
    }

    #[test]
    fn test_discover_endpoints_invalid_port_errors() {
        let err = discover_endpoints_from_inputs(&EndpointInputs {
            beardog_port: Some("not-a-u16".to_string()),
            ..Default::default()
        });
        assert!(err.is_err());
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
