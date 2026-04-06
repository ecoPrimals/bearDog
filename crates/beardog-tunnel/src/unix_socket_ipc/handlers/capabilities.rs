// SPDX-License-Identifier: AGPL-3.0-or-later

//! Capabilities handler
//!
//! Provides self-description and identity endpoints for service discovery.
//! Every primal should expose these methods to enable capability-based discovery.

use super::MethodHandler;
use super::utils::{IdentityHints, get_primal_name_with};
use crate::btsp_provider::BeardogBtspProvider;
use async_trait::async_trait;
use beardog_types::primal_identity::PrimalIdentity;
use std::sync::Arc;
use tracing::info;

/// Handler for capabilities and identity methods.
///
/// ## Canonical Methods (use these)
///
/// - `capabilities.list` — list all provided capabilities
/// - `capability.list` — alias of `capabilities.list`
/// - `primal.capabilities` — alias of `capabilities.list`
/// - `discover_capabilities` — detailed capability discovery with metadata
///
/// ## Deprecated Flat Aliases (will be removed in v1.0)
///
/// - `capabilities` — use `capabilities.list`
/// - `get_capabilities` — use `capabilities.list`
/// - `identity` — use a canonical `identity.*` method when available
/// - `whoami` — use a canonical `identity.*` method when available
/// - `get_identity` — use a canonical `identity.*` method when available
///
/// All responses include genetic lineage (`family_id`, `node_id`) discovered
/// from environment variables at runtime (no hardcoding).
pub struct CapabilitiesHandler {
    identity: Arc<PrimalIdentity>,
    primal_hints: IdentityHints,
}

#[async_trait]
impl MethodHandler for CapabilitiesHandler {
    fn methods(&self) -> Vec<&'static str> {
        vec![
            "capabilities",
            "get_capabilities",
            "discover_capabilities",
            "capabilities.list",
            "capability.list",
            "primal.capabilities",
            "identity",
            "whoami",
            "get_identity",
        ]
    }

    async fn handle(
        &self,
        method: &str,
        _params: Option<&serde_json::Value>,
        _btsp_provider: &Arc<BeardogBtspProvider>,
    ) -> Result<serde_json::Value, String> {
        match method {
            // Canonical
            "capabilities.list" | "capability.list" | "primal.capabilities" => {
                self.handle_capabilities().await
            }
            // Deprecated flat aliases — remove in v1.0
            "capabilities" | "get_capabilities" => self.handle_capabilities().await,
            "discover_capabilities" => self.handle_discover_capabilities().await,
            // Deprecated flat aliases — remove in v1.0
            "identity" | "whoami" | "get_identity" => self.handle_identity().await,
            _ => Err(format!("Method not found: {method}")),
        }
    }
}

impl CapabilitiesHandler {
    /// Create a new `CapabilitiesHandler` with explicit identity injection
    pub fn new(identity: Arc<PrimalIdentity>) -> Self {
        Self {
            identity,
            primal_hints: IdentityHints::from_env(),
        }
    }

    /// Tests / DI: explicit primal name hints (no `PRIMAL_NAME` env mutation).
    pub fn with_hints(identity: Arc<PrimalIdentity>, primal_hints: IdentityHints) -> Self {
        Self {
            identity,
            primal_hints,
        }
    }

    /// Handle capabilities request
    ///
    /// Returns a comprehensive list of all capabilities provided by `BearDog`,
    /// including crypto, security, BTSP, graph security, and JWT generation.
    async fn handle_capabilities(&self) -> Result<serde_json::Value, String> {
        // Use injected identity (no environment variables!)
        let family_id = self.identity.family_id();
        let node_id = self.identity.node_id();

        info!("🎯 Capabilities requested - exposing our capabilities");

        Ok(serde_json::json!({
            "primal": get_primal_name_with(&self.primal_hints),
            "family_id": family_id,
            "node_id": node_id,
            "provided_capabilities": [
                {
                    "type": "security",
                    "version": "1.0",
                    "methods": ["evaluate", "lineage", "generate_jwt_secret"],
                    "description": "Security provider - trust evaluation, genetic lineage, and secret generation"
                },
                {
                    "type": "encryption",
                    "version": "1.0",
                    "methods": ["encrypt", "decrypt"],
                },
                {
                    "type": "trust",
                    "version": "1.0",
                    "methods": ["evaluate", "lineage"],
                },
                {
                    "type": "btsp",
                    "version": "1.0",
                    "methods": ["contact_exchange", "tunnel_establish", "tunnel_encrypt", "tunnel_decrypt", "tunnel_status", "tunnel_close"],
                    "description": "BearDog Tunnel Security Protocol - VPN-free P2P mesh via genetic lineage"
                },
                {
                    "type": "graph",
                    "version": "1.0",
                    "methods": ["authorize_modification", "validate_template", "audit_origin"],
                    "description": "Collaborative Intelligence graph security - 5-layer authorization, threat detection, and provenance"
                },
                {
                    "type": "jwt_secrets",
                    "version": "1.0",
                    "methods": ["generate_jwt_secret"],
                    "description": "JWT secret generation for authentication systems (e.g., sovereign storage primals)"
                },
                {
                    "type": "crypto",
                    "version": "1.0",
                    "methods": [
                        "sign_ed25519",
                        "verify_ed25519",
                        "x25519_generate_ephemeral",
                        "x25519_derive_secret",
                        "chacha20_poly1305_encrypt",
                        "chacha20_poly1305_decrypt",
                        "blake3_hash",
                        "hmac_sha256"
                    ],
                    "description": "Pure Rust cryptographic operations for TLS and other primals - Ed25519, X25519, ChaCha20-Poly1305, Blake3, HMAC"
                },
                {
                    "type": "tls",
                    "version": "1.0",
                    "methods": [
                        "derive_secrets",
                        "sign_handshake",
                        "verify_certificate"
                    ],
                    "description": "TLS 1.3 cryptographic operations - HKDF key derivation, Ed25519 handshake signing, X.509 certificate verification"
                },
                {
                    "type": "relay",
                    "version": "1.0",
                    "methods": ["authorize"],
                    "description": "Relay authorization - lineage-gated access control for relay-assisted coordinated punch"
                }
            ],
            "version": env!("CARGO_PKG_VERSION"),
            "protocols": ["tarpc", "json-rpc", "http"],
            "wire_format": "ndjson",
            "btsp_enabled": true,
            "collaborative_intelligence": true,
        }))
    }

    /// Handle `discover_capabilities` request
    ///
    /// Returns a flat list of capability strings for ecosystem consistency.
    /// This mirrors the conventional `discover_capabilities` format, enabling
    /// uniform capability discovery across all primals.
    async fn handle_discover_capabilities(&self) -> Result<serde_json::Value, String> {
        info!("🔍 discover_capabilities requested");

        Ok(serde_json::json!({
            "capabilities": [
                "crypto.sha256",
                "crypto.sha512",
                "crypto.sign",
                "crypto.verify",
                "crypto.key_exchange",
                "crypto.encrypt",
                "crypto.decrypt",
                "crypto.hmac",
                "jwt.provision",
                "secrets.store",
                "secrets.retrieve",
                "relay.authorize"
            ]
        }))
    }

    /// Handle identity request
    ///
    /// Returns the primal's identity including family and node IDs,
    /// plus an encryption tag for discovery/federation.
    async fn handle_identity(&self) -> Result<serde_json::Value, String> {
        // Use injected identity (no environment variables!)
        let family_id = self.identity.family_id();
        let node_id = self.identity.node_id();

        // Generate encryption tag using identity helper
        let encryption_tag = self.identity.encryption_tag();

        info!(
            "🆔 Identity requested - family: {}, node: {}, encryption_tag: {}",
            family_id, node_id, encryption_tag
        );

        Ok(serde_json::json!({
            "primal": get_primal_name_with(&self.primal_hints),
            "family": family_id,
            "node": node_id,
            "encryption_tag": encryption_tag,
            "version": env!("CARGO_PKG_VERSION"),
        }))
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::unix_socket_ipc::handlers::utils::IdentityHints;

    fn test_hints() -> IdentityHints {
        IdentityHints {
            primal_name: Some("beardog".to_string()),
            ..Default::default()
        }
    }

    #[tokio::test]
    async fn test_capabilities_handler_methods() {
        let identity = Arc::new(PrimalIdentity::for_test("test-family", "test-node"));
        let handler = CapabilitiesHandler::with_hints(identity, test_hints());
        let methods = handler.methods();

        assert_eq!(methods.len(), 9);
        assert!(methods.contains(&"capabilities"));
        assert!(methods.contains(&"get_capabilities"));
        assert!(methods.contains(&"discover_capabilities"));
        assert!(methods.contains(&"capabilities.list"));
        assert!(methods.contains(&"capability.list"));
        assert!(methods.contains(&"primal.capabilities"));
        assert!(methods.contains(&"identity"));
        assert!(methods.contains(&"whoami"));
        assert!(methods.contains(&"get_identity"));
    }

    #[tokio::test]
    async fn test_capabilities_response() {
        let identity = Arc::new(PrimalIdentity::for_test("test-family", "test-node"));
        let handler = CapabilitiesHandler::with_hints(identity, test_hints());
        let btsp_provider = crate::test_helpers::mocks::create_minimal_beardog_provider().await;

        let result = handler.handle("capabilities", None, &btsp_provider).await;

        assert!(result.is_ok());
        let response = result.expect("capabilities handler in test");

        assert_eq!(response["primal"], "beardog");
        assert!(response["provided_capabilities"].is_array());
        assert!(response["family_id"].is_string());
        assert!(response["node_id"].is_string());
        assert!(
            response["btsp_enabled"]
                .as_bool()
                .expect("btsp_enabled should be bool in test")
        );
    }

    #[tokio::test]
    async fn test_identity_response() {
        let identity = Arc::new(PrimalIdentity::for_test("test-family", "test-node"));
        let handler = CapabilitiesHandler::with_hints(identity, test_hints());
        let btsp_provider = crate::test_helpers::mocks::create_minimal_beardog_provider().await;

        let result = handler.handle("identity", None, &btsp_provider).await;

        assert!(result.is_ok());
        let response = result.expect("capabilities handler in test");

        assert_eq!(response["primal"], "beardog");
        assert!(response["family"].is_string());
        assert!(response["node"].is_string());
        assert!(response["encryption_tag"].is_string());
    }

    #[tokio::test]
    async fn test_discover_capabilities_response() {
        let identity = Arc::new(PrimalIdentity::for_test("test-family", "test-node"));
        let handler = CapabilitiesHandler::with_hints(identity, test_hints());
        let btsp_provider = crate::test_helpers::mocks::create_minimal_beardog_provider().await;

        let result = handler
            .handle("discover_capabilities", None, &btsp_provider)
            .await;

        assert!(result.is_ok());
        let response = result.expect("capabilities handler in test");

        // Must have flat capabilities array
        let caps = response["capabilities"]
            .as_array()
            .expect("capabilities should be array in test");
        assert!(caps.len() >= 12, "Expected at least 12 capabilities");

        // Verify required capabilities per ecoBin v2.0
        let cap_strs: Vec<&str> = caps
            .iter()
            .map(|v| {
                v.as_str()
                    .expect("capability entry should be string in test")
            })
            .collect();
        assert!(cap_strs.contains(&"crypto.sha256"));
        assert!(cap_strs.contains(&"crypto.sign"));
        assert!(cap_strs.contains(&"crypto.verify"));
        assert!(cap_strs.contains(&"crypto.key_exchange"));
        assert!(cap_strs.contains(&"crypto.encrypt"));
        assert!(cap_strs.contains(&"crypto.decrypt"));
        assert!(cap_strs.contains(&"crypto.hmac"));
        assert!(cap_strs.contains(&"jwt.provision"));
        assert!(cap_strs.contains(&"secrets.store"));
        assert!(cap_strs.contains(&"secrets.retrieve"));
    }

    #[tokio::test]
    async fn test_all_capability_aliases() {
        let identity = Arc::new(PrimalIdentity::for_test("test-family", "test-node"));
        let handler = CapabilitiesHandler::with_hints(identity, test_hints());
        let btsp_provider = crate::test_helpers::mocks::create_minimal_beardog_provider().await;

        for method in &[
            "capabilities",
            "get_capabilities",
            "capabilities.list",
            "capability.list",
            "primal.capabilities",
        ] {
            let result = handler.handle(method, None, &btsp_provider).await;
            assert!(result.is_ok(), "Method {} should succeed", method);
        }
    }

    #[tokio::test]
    async fn test_all_identity_aliases() {
        let identity = Arc::new(PrimalIdentity::for_test("test-family", "test-node"));
        let handler = CapabilitiesHandler::with_hints(identity, test_hints());
        let btsp_provider = crate::test_helpers::mocks::create_minimal_beardog_provider().await;

        for method in &["identity", "whoami", "get_identity"] {
            let result = handler.handle(method, None, &btsp_provider).await;
            assert!(result.is_ok(), "Method {} should succeed", method);
        }
    }
}
