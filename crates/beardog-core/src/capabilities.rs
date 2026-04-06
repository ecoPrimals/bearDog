// SPDX-License-Identifier: AGPL-3.0-or-later

//! BearDog Capability Manifest
//!
//! Defines what BearDog provides to the ecosystem without coupling to specific primals
//!
//! **Design Principle**: BearDog has self-knowledge only
//! - Advertises: "I provide encryption, trust evaluation, security"
//! - Does NOT know: "I connect to Songbird" or "ToadStool needs me"
//! - biomeOS handles routing based on capabilities

use serde::{Deserialize, Serialize};
use std::collections::HashMap;

use beardog_types::constants::domains::network::ipc_discovery;

/// `BearDog`'s capabilities advertised to the ecosystem
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct BearDogCapabilities {
    /// Primal identity
    pub primal_id: String,

    /// Family this primal belongs to
    pub family_id: Option<String>,

    /// Node ID (unique per tower)
    pub node_id: String,

    /// Capabilities this primal provides
    pub provides: Vec<Capability>,

    /// Capabilities this primal requires (optional)
    pub requires: Vec<Capability>,

    /// IPC endpoints this primal exposes
    pub endpoints: Vec<IpcEndpoint>,

    /// Metadata (version, build info, etc)
    pub metadata: HashMap<String, String>,
}

/// Generic capability descriptor
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
#[serde(tag = "type", rename_all = "snake_case")]
pub enum Capability {
    /// Encryption capability
    Encryption {
        /// Supported encryption algorithms (e.g., "AES-256-GCM", "ChaCha20-Poly1305")
        algorithms: Vec<String>,
        /// Supported key types (e.g., "symmetric", "asymmetric")
        key_types: Vec<String>,
    },

    /// Trust evaluation capability
    TrustEvaluation {
        /// Supported trust evaluation models (e.g., "hierarchical", "web-of-trust")
        trust_models: Vec<String>,
    },

    /// Key management capability
    KeyManagement {
        /// Supported HSM types (e.g., "software", "yubikey", "nitrokey")
        hsm_types: Vec<String>,
    },

    /// Signature generation/verification
    Signatures {
        /// Supported signature algorithms (e.g., "Ed25519", "ECDSA-P256")
        algorithms: Vec<String>,
    },

    /// Discovery capability (for other primals)
    Discovery {
        /// Supported discovery protocols (e.g., "mdns", "dns-sd", "manual")
        protocols: Vec<String>,
    },

    /// Storage capability (for other primals)
    Storage {
        /// Supported storage types (e.g., "sled", "memory", "filesystem")
        storage_types: Vec<String>,
    },

    /// Compute capability (for other primals)
    Compute {
        /// Supported compute types (e.g., "local", "distributed")
        compute_types: Vec<String>,
    },

    /// Custom capability (extensible)
    Custom {
        /// Capability name
        name: String,
        /// Capability version
        version: String,
        /// Custom properties specific to this capability
        properties: HashMap<String, String>,
    },
}

/// IPC endpoint descriptor
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(tag = "type", rename_all = "snake_case")]
pub enum IpcEndpoint {
    /// Unix domain socket
    UnixSocket {
        /// Filesystem path to Unix domain socket
        path: String,
        /// File permissions (octal, e.g., 0o600)
        permissions: u32,
    },

    /// HTTP API endpoint
    Http {
        /// Bind address (IP:port, e.g., "127.0.0.1:8080")
        bind_addr: String,
        /// Whether TLS is enabled
        tls: bool,
    },

    /// Shared memory segment
    SharedMemory {
        /// Shared memory key identifier
        key: String,
        /// Size of shared memory segment in bytes
        size_bytes: usize,
    },
}

/// Capability request from another primal
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CapabilityRequest {
    /// Requesting primal ID
    pub from_primal: String,

    /// Requested capability
    pub capability: Capability,

    /// Request-specific parameters
    pub params: HashMap<String, serde_json::Value>,

    /// Request ID for correlation
    pub request_id: String,
}

/// Capability response
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CapabilityResponse {
    /// Request ID this responds to
    pub request_id: String,

    /// Success or error
    pub status: ResponseStatus,

    /// Response data
    pub data: Option<serde_json::Value>,

    /// Error message if failed
    pub error: Option<String>,
}

/// Response status for capability requests
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum ResponseStatus {
    /// Request completed successfully
    Success,
    /// Request failed with an error
    Error,
    /// Requested capability is not available
    NotAvailable,
}

impl BearDogCapabilities {
    /// Create `BearDog`'s capability manifest
    #[must_use]
    pub fn new(family_id: Option<String>, node_id: String) -> Self {
        let mut metadata = HashMap::new();
        metadata.insert("version".to_string(), env!("CARGO_PKG_VERSION").to_string());
        metadata.insert("primal_type".to_string(), "security".to_string());
        metadata.insert("wire_format".to_string(), "ndjson".to_string());
        metadata.insert("protocol".to_string(), "jsonrpc-2.0".to_string());

        // Self-knowledge pattern: discover primal name from environment
        let primal_name = beardog_errors::process_env::var("PRIMAL_NAME")
            .or_else(|_| beardog_errors::process_env::var("BEARDOG_NAME"))
            .unwrap_or_else(|_| "beardog".to_string());

        let socket_path = ipc_discovery::biomeos_ipc_socket_dir_from_env()
            .join(format!(
                "{}-{}.sock",
                primal_name,
                family_id.as_deref().unwrap_or("default")
            ))
            .display()
            .to_string();

        Self {
            primal_id: primal_name,
            family_id,
            node_id,
            provides: Self::default_capabilities(),
            requires: vec![
                // BearDog optionally benefits from discovery, but doesn't require it
                Capability::Discovery {
                    protocols: vec!["any".to_string()],
                },
            ],
            endpoints: vec![
                IpcEndpoint::UnixSocket {
                    path: socket_path,
                    permissions: 0o600,
                },
                {
                    use beardog_config::global::BEARDOG_CONFIG;
                    IpcEndpoint::Http {
                        // Use config-based address and port (no hardcoding)
                        bind_addr: format!(
                            "{}:{}",
                            BEARDOG_CONFIG.network.api.bind_address,
                            BEARDOG_CONFIG.network.ports.api_port
                        ),
                        tls: BEARDOG_CONFIG.network.api.tls_enabled,
                    }
                },
            ],
            metadata,
        }
    }

    /// Default capabilities `BearDog` provides
    fn default_capabilities() -> Vec<Capability> {
        vec![
            Capability::Encryption {
                algorithms: vec!["ChaCha20Poly1305".to_string(), "AES-256-GCM".to_string()],
                key_types: vec!["X25519".to_string(), "Ed25519".to_string()],
            },
            Capability::TrustEvaluation {
                trust_models: vec![
                    "family_based".to_string(),
                    "progressive_trust".to_string(),
                    "genetic_lineage".to_string(),
                ],
            },
            Capability::KeyManagement {
                hsm_types: vec![
                    "software".to_string(),
                    "hardware".to_string(),
                    "android_strongbox".to_string(),
                    "ios_secure_enclave".to_string(),
                ],
            },
            Capability::Signatures {
                algorithms: vec!["Ed25519".to_string(), "ECDSA-P256".to_string()],
            },
        ]
    }

    /// Check if this primal provides a capability
    #[must_use]
    pub fn provides_capability(&self, cap: &Capability) -> bool {
        self.provides
            .iter()
            .any(|c| Self::capability_matches(c, cap))
    }

    /// Fuzzy match capabilities (e.g., "any encryption" matches "`ChaCha20`")
    fn capability_matches(provided: &Capability, requested: &Capability) -> bool {
        use Capability::{
            Compute, Custom, Discovery, Encryption, KeyManagement, Signatures, Storage,
            TrustEvaluation,
        };
        match (provided, requested) {
            (Encryption { .. }, Encryption { .. }) => true,
            (TrustEvaluation { .. }, TrustEvaluation { .. }) => true,
            (KeyManagement { .. }, KeyManagement { .. }) => true,
            (Signatures { .. }, Signatures { .. }) => true,
            (Discovery { .. }, Discovery { .. }) => true,
            (Storage { .. }, Storage { .. }) => true,
            (Compute { .. }, Compute { .. }) => true,
            (Custom { name: n1, .. }, Custom { name: n2, .. }) => n1 == n2,
            _ => false,
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    /// Helper to get expected primal name from environment or default
    fn expected_primal_name() -> String {
        beardog_errors::process_env::var("PRIMAL_NAME")
            .or_else(|_| beardog_errors::process_env::var("BEARDOG_NAME"))
            .unwrap_or_else(|_| "beardog".to_string())
    }

    #[test]
    fn test_capability_manifest_creation() {
        let caps = BearDogCapabilities::new(Some("test_family".to_string()), "node1".to_string());

        // Primal ID is discovered from environment or defaults to "beardog"
        assert_eq!(caps.primal_id, expected_primal_name());
        assert_eq!(caps.family_id, Some("test_family".to_string()));
        assert_eq!(caps.node_id, "node1");
        assert!(!caps.provides.is_empty());
    }

    #[test]
    fn test_provides_encryption() {
        let caps = BearDogCapabilities::new(None, "node1".to_string());

        let encryption_request = Capability::Encryption {
            algorithms: vec!["any".to_string()],
            key_types: vec!["any".to_string()],
        };

        assert!(caps.provides_capability(&encryption_request));
    }

    #[test]
    fn test_provides_trust_evaluation() {
        let caps = BearDogCapabilities::new(None, "node1".to_string());

        let trust_request = Capability::TrustEvaluation {
            trust_models: vec!["any".to_string()],
        };

        assert!(caps.provides_capability(&trust_request));
    }

    #[test]
    fn test_does_not_provide_storage() {
        let caps = BearDogCapabilities::new(None, "node1".to_string());

        let storage_request = Capability::Storage {
            storage_types: vec!["any".to_string()],
        };

        assert!(!caps.provides_capability(&storage_request));
    }

    #[test]
    fn test_serialization() {
        let caps = BearDogCapabilities::new(Some("test".to_string()), "node1".to_string());

        let json = serde_json::to_string_pretty(&caps).unwrap();
        // Primal name comes from environment or defaults
        let expected_name = expected_primal_name();
        assert!(json.contains(&expected_name));
        assert!(json.contains("encryption"));

        let deserialized: BearDogCapabilities = serde_json::from_str(&json).unwrap();
        assert_eq!(deserialized.primal_id, expected_name);
    }
}

// Comprehensive test suite
#[cfg(test)]
#[path = "capabilities_tests.rs"]
mod capabilities_tests;
