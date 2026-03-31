// SPDX-License-Identifier: AGPL-3.0-only

//! Primal Introspection Handler
//!
//! Provides JSON-RPC methods for primal self-description and discovery:
//! - `primal.info` - Returns primal metadata (name, version, capabilities)
//! - `rpc.methods` - Returns list of all available JSON-RPC methods
//!
//! These methods enable runtime capability discovery by allowing other primals
//! and services to query what this primal provides without manual configuration.

use super::utils::{IdentityHints, get_primal_name_with};
use super::{HandlerRegistry, MethodHandler};
use crate::btsp_provider::BeardogBtspProvider;
use async_trait::async_trait;
use beardog_types::constants::domains::network::ipc_discovery::BEARDOG_CAPABILITY_DOMAIN;
use serde_json::{Value, json};
use std::sync::Arc;

/// Handler for primal introspection methods
pub struct IntrospectionHandler {
    /// Reference to handler registry for method listing
    registry: Arc<HandlerRegistry>,
    identity: IdentityHints,
}

impl IntrospectionHandler {
    /// Create new introspection handler (reads identity from environment).
    pub fn new(registry: Arc<HandlerRegistry>) -> Self {
        Self {
            registry,
            identity: IdentityHints::from_env(),
        }
    }

    /// Tests / DI: explicit identity hints (no `PRIMAL_NAME` env mutation).
    pub fn with_identity_hints(registry: Arc<HandlerRegistry>, identity: IdentityHints) -> Self {
        Self { registry, identity }
    }
}

#[async_trait]
impl MethodHandler for IntrospectionHandler {
    fn methods(&self) -> Vec<&'static str> {
        vec!["primal.info", "rpc.methods", "primal.capabilities"]
    }

    async fn handle(
        &self,
        method: &str,
        _params: Option<&Value>,
        _btsp_provider: &Arc<BeardogBtspProvider>,
    ) -> Result<Value, String> {
        match method {
            "primal.info" => self.handle_primal_info().await,
            "rpc.methods" => self.handle_rpc_methods().await,
            "primal.capabilities" => self.handle_primal_capabilities().await,
            _ => Err(format!("Unknown introspection method: {method}")),
        }
    }
}

impl IntrospectionHandler {
    /// Handle primal.info - Return primal metadata
    ///
    /// Returns comprehensive information about this primal including:
    /// - Name and version
    /// - Capabilities provided
    /// - Available method namespaces
    /// - Protocol version
    async fn handle_primal_info(&self) -> Result<Value, String> {
        Ok(json!({
            "name": get_primal_name_with(&self.identity),
            "version": env!("CARGO_PKG_VERSION"),
            "description": "Cryptographic heart of ecoPrimals - Pure Rust crypto service",
            "capabilities": [
                BEARDOG_CAPABILITY_DOMAIN,      // Cryptographic operations
                "security",    // Security/HSM operations
                "genetic",     // Genetic lineage operations
                "federation",  // Sub-federation key derivation
                "encryption",  // Encryption/decryption
                "secrets",     // Encrypted secret storage (family-scoped)
                "btsp"         // BTSP provider
            ],
            "protocol": {
                "jsonrpc": "2.0",
                "transport": "unix_socket"
            },
            "features": {
                "algorithms": {
                    "signatures": ["Ed25519", "ECDSA", "RSA"],
                    "key_exchange": ["X25519", "ECDHE"],
                    "aead": ["ChaCha20-Poly1305", "AES-GCM"],
                    "hashing": ["BLAKE3", "SHA-256", "SHA-384", "SHA-512", "HMAC"],
                    "kdf": ["HKDF", "TLS-PRF", "PBKDF2", "Argon2id", "Scrypt", "Bcrypt"]
                },
                "hsm": {
                    "tiers": ["software", "hardware", "cloud", "mobile"],
                    "platforms": ["Linux", "macOS", "Android", "iOS", "Windows"]
                },
                "genetic": {
                    "lineage": true,
                    "entropy_mixing": true,
                    "dark_forest": true
                }
            },
            "status": "production_ready"
        }))
    }

    /// Handle rpc.methods - Return list of available methods
    ///
    /// Returns all JSON-RPC methods this primal exposes, grouped by namespace.
    /// This enables auto-discovery of capabilities.
    async fn handle_rpc_methods(&self) -> Result<Value, String> {
        // Get all methods from registry (now async)
        let all_methods = self.registry.all_methods().await;

        // Group by namespace for readability (borrow method strings — no per-method clone)
        let mut by_namespace: std::collections::HashMap<String, Vec<&str>> =
            std::collections::HashMap::new();

        for method in &all_methods {
            let namespace = method.split('.').next().unwrap_or("unknown").to_string();

            by_namespace
                .entry(namespace)
                .or_default()
                .push(method.as_str());
        }

        Ok(json!({
            "methods": all_methods,
            "count": all_methods.len(),
            "by_namespace": by_namespace
        }))
    }

    /// Handle primal.capabilities - Return detailed capability information
    ///
    /// Returns structured information about each capability this primal provides,
    /// including methods available for each capability.
    async fn handle_primal_capabilities(&self) -> Result<Value, String> {
        let all_methods = self.registry.all_methods().await;

        // Define capability mappings
        let capabilities = json!({
            (BEARDOG_CAPABILITY_DOMAIN): {
                "description": "Core cryptographic operations",
                "methods": all_methods.iter()
                    .filter(|m| m.starts_with("crypto."))
                    .map(String::as_str)
                    .collect::<Vec<_>>(),
                "operations": [
                    "sign", "verify", "encrypt", "decrypt",
                    "hash", "hmac", "kdf", "key_exchange"
                ]
            },
            "genetic": {
                "description": "Genetic lineage and Dark Forest operations",
                "methods": all_methods.iter()
                    .filter(|m| m.starts_with("genetic."))
                    .map(String::as_str)
                    .collect::<Vec<_>>(),
                "operations": [
                    "derive_lineage_key", "mix_entropy", "verify_lineage",
                    "generate_challenge", "respond_to_challenge", "verify_challenge_response"
                ]
            },
            "federation": {
                "description": "Sub-federation key derivation",
                "methods": all_methods.iter()
                    .filter(|m| m.starts_with("federation."))
                    .map(String::as_str)
                    .collect::<Vec<_>>(),
                "operations": ["derive_subfed_key", "verify_family_member"]
            },
            "encryption": {
                "description": "High-level encryption/decryption",
                "methods": all_methods.iter()
                    .filter(|m| m.starts_with("encryption."))
                    .map(String::as_str)
                    .collect::<Vec<_>>(),
                "operations": ["encrypt", "decrypt"]
            },
            "security": {
                "description": "Security and HSM operations",
                "methods": all_methods.iter()
                    .filter(|m| m.starts_with("security."))
                    .map(String::as_str)
                    .collect::<Vec<_>>(),
                "operations": ["hsm_status", "key_management"]
            },
            "btsp": {
                "description": "BTSP provider for TLS integration",
                "methods": all_methods.iter()
                    .filter(|m| m.starts_with("btsp."))
                    .map(String::as_str)
                    .collect::<Vec<_>>(),
                "operations": ["provide_crypto_atoms"]
            },
            "graph_security": {
                "description": "Graph security and audit operations",
                "methods": all_methods.iter()
                    .filter(|m| m.starts_with("graph_security."))
                    .map(String::as_str)
                    .collect::<Vec<_>>(),
                "operations": ["audit", "validate", "collaborate"]
            }
        });

        Ok(capabilities)
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::unix_socket_ipc::handlers::utils::IdentityHints;

    #[tokio::test]
    async fn test_primal_info_structure() {
        let registry = HandlerRegistry::default();
        let handler = IntrospectionHandler::with_identity_hints(
            registry,
            IdentityHints {
                primal_name: Some("beardog".to_string()),
                ..Default::default()
            },
        );

        let result = handler.handle_primal_info().await.unwrap();

        // Validate structure
        assert!(result["name"].is_string());
        assert_eq!(result["name"], "beardog");
        assert!(result["version"].is_string());
        assert!(result["capabilities"].is_array());
        assert!(result["protocol"].is_object());
        assert!(result["features"].is_object());
    }

    #[tokio::test]
    async fn test_primal_capabilities_structure() {
        let registry = HandlerRegistry::default();
        let handler = IntrospectionHandler::new(registry);

        let result = handler.handle_primal_capabilities().await.unwrap();

        // Validate crypto capability
        assert!(result["crypto"].is_object());
        assert!(result["crypto"]["description"].is_string());
        assert!(result["crypto"]["methods"].is_array());
        assert!(result["crypto"]["operations"].is_array());

        // Validate genetic capability
        assert!(result["genetic"].is_object());
        assert!(result["genetic"]["description"].is_string());
    }

    #[tokio::test]
    async fn test_methods_list() {
        let registry = HandlerRegistry::default();
        let handler = IntrospectionHandler::new(registry);

        let methods = handler.methods();
        assert_eq!(methods.len(), 3);
        assert!(methods.contains(&"primal.info"));
        assert!(methods.contains(&"rpc.methods"));
        assert!(methods.contains(&"primal.capabilities"));
    }
}
