// SPDX-License-Identifier: AGPL-3.0-or-later

//! Capabilities handler
//!
//! Provides self-description and identity endpoints for service discovery.
//! Every primal should expose these methods to enable capability-based discovery.
//!
//! Implements Wire Standard Level 2 per `CAPABILITY_WIRE_STANDARD.md`:
//! - `capabilities.list` returns `{primal, version, methods, provided_capabilities}`
//! - `identity.get` returns `{primal, version, domain, license}`

use super::utils::{IdentityHints, get_primal_name_with};
use super::{HandlerRegistry, MethodHandler};
use crate::btsp_provider::BeardogBtspProvider;
use async_trait::async_trait;
use beardog_types::primal_identity::PrimalIdentity;
use std::sync::Arc;
use tracing::info;

/// Handler for capabilities and identity methods.
///
/// ## Canonical Methods (use these)
///
/// - `capabilities.list` — list all provided capabilities (Wire Standard L2)
/// - `capability.list` — alias of `capabilities.list`
/// - `identity.get` — primal self-identification (Wire Standard L2)
/// - `discover_capabilities` — detailed capability discovery with metadata
///
/// ## Deprecated Flat Aliases (will be removed in v1.0)
///
/// - `capabilities` — use `capabilities.list`
/// - `get_capabilities` — use `capabilities.list`
/// - `primal.capabilities` — use `capabilities.list`
/// - `identity` — use `identity.get`
/// - `whoami` — use `identity.get`
/// - `get_identity` — use `identity.get`
///
/// All responses include genetic lineage (`family_id`, `node_id`) discovered
/// from environment variables at runtime (no hardcoding).
pub struct CapabilitiesHandler {
    identity: Arc<PrimalIdentity>,
    primal_hints: IdentityHints,
    registry: Arc<HandlerRegistry>,
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
            "identity.get",
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
            "capabilities.list" | "capability.list" | "primal.capabilities" => {
                self.handle_capabilities().await
            }
            // Deprecated flat aliases — remove in v1.0
            "capabilities" | "get_capabilities" => self.handle_capabilities().await,
            "discover_capabilities" => self.handle_discover_capabilities().await,
            // Wire Standard L2: canonical identity endpoint
            "identity.get" => self.handle_identity_get().await,
            // Deprecated flat aliases — remove in v1.0
            "identity" | "whoami" | "get_identity" => self.handle_identity().await,
            _ => Err(format!("Method not found: {method}")),
        }
    }
}

impl CapabilitiesHandler {
    /// Create a new `CapabilitiesHandler` with registry access for method enumeration.
    ///
    /// The registry reference enables the flat `methods` array in `capabilities.list`
    /// responses per `CAPABILITY_WIRE_STANDARD.md` Level 2.
    pub fn new(identity: Arc<PrimalIdentity>, registry: Arc<HandlerRegistry>) -> Self {
        Self {
            identity,
            primal_hints: IdentityHints::from_env(),
            registry,
        }
    }

    /// Tests / DI: explicit primal name hints (no `PRIMAL_NAME` env mutation).
    pub fn with_hints(
        identity: Arc<PrimalIdentity>,
        primal_hints: IdentityHints,
        registry: Arc<HandlerRegistry>,
    ) -> Self {
        Self {
            identity,
            primal_hints,
            registry,
        }
    }

    /// Collect the flat, fully-qualified method list from the handler registry.
    ///
    /// Filters to `domain.operation` dotted names only (excludes deprecated flat
    /// aliases like `"capabilities"` or `"get_identity"`). This is the primary
    /// routing signal for biomeOS per `CAPABILITY_WIRE_STANDARD.md`.
    async fn wire_standard_methods(&self) -> Vec<String> {
        self.registry
            .all_methods()
            .await
            .into_iter()
            .filter(|m| m.contains('.'))
            .collect()
    }

    /// Handle `capabilities.list` — Wire Standard Level 2 response.
    ///
    /// Returns `{primal, version, methods, provided_capabilities, ...}`.
    /// The `methods` flat array is the primary routing signal for biomeOS.
    /// The `provided_capabilities` grouping is retained for structured routing.
    ///
    /// # Errors
    ///
    /// Returns an error string if method enumeration fails.
    async fn handle_capabilities(&self) -> Result<serde_json::Value, String> {
        let family_id = self.identity.family_id();
        let node_id = self.identity.node_id();
        let methods = self.wire_standard_methods().await;

        info!(
            "Capabilities requested — {} methods advertised",
            methods.len()
        );

        Ok(serde_json::json!({
            "primal": get_primal_name_with(&self.primal_hints),
            "version": env!("CARGO_PKG_VERSION"),
            "methods": methods,
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
            "consumed_capabilities": [],
            "cost_estimates": {
                "crypto.sign_ed25519":             { "cpu": "low",    "latency_ms": 1 },
                "crypto.verify_ed25519":           { "cpu": "low",    "latency_ms": 1 },
                "crypto.blake3_hash":              { "cpu": "low",    "latency_ms": 1 },
                "crypto.hmac_sha256":              { "cpu": "low",    "latency_ms": 1 },
                "crypto.chacha20_poly1305_encrypt": { "cpu": "low",   "latency_ms": 1 },
                "crypto.chacha20_poly1305_decrypt": { "cpu": "low",   "latency_ms": 1 },
                "crypto.x25519_generate_ephemeral": { "cpu": "low",   "latency_ms": 1 },
                "crypto.x25519_derive_secret":      { "cpu": "low",   "latency_ms": 1 },
                "btsp.session.create":             { "cpu": "medium", "latency_ms": 2 },
                "btsp.session.verify":             { "cpu": "medium", "latency_ms": 2 },
                "security.evaluate":               { "cpu": "medium", "latency_ms": 5 },
                "graph.authorize_modification":    { "cpu": "medium", "latency_ms": 5 },
                "tls.derive_secrets":              { "cpu": "medium", "latency_ms": 2 },
            },
            "operation_dependencies": {
                "btsp.session.verify": ["btsp.session.create"],
                "btsp.negotiate":      ["btsp.session.verify"],
            },
            "protocols": ["tarpc", "json-rpc", "http"],
            "transport": ["uds", "tcp"],
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

    /// Handle `identity.get` — Wire Standard Level 2 identity endpoint.
    ///
    /// Returns `{primal, version, domain, license}` per `CAPABILITY_WIRE_STANDARD.md` §4.
    async fn handle_identity_get(&self) -> Result<serde_json::Value, String> {
        info!("identity.get requested (Wire Standard L2)");

        Ok(serde_json::json!({
            "primal": get_primal_name_with(&self.primal_hints),
            "version": env!("CARGO_PKG_VERSION"),
            "domain": "crypto",
            "license": "AGPL-3.0-or-later",
        }))
    }

    /// Handle legacy identity request (deprecated — use `identity.get`).
    ///
    /// Returns the primal's identity including family and node IDs,
    /// plus an encryption tag for discovery/federation.
    async fn handle_identity(&self) -> Result<serde_json::Value, String> {
        let family_id = self.identity.family_id();
        let node_id = self.identity.node_id();
        let encryption_tag = self.identity.encryption_tag();

        info!(
            "Identity requested (legacy) — family: {}, node: {}, encryption_tag: {}",
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

    fn test_handler() -> CapabilitiesHandler {
        let identity = Arc::new(PrimalIdentity::for_test("test-family", "test-node"));
        let registry = HandlerRegistry::default();
        CapabilitiesHandler::with_hints(identity, test_hints(), registry)
    }

    #[tokio::test]
    async fn test_capabilities_handler_methods() {
        let handler = test_handler();
        let methods = handler.methods();

        assert_eq!(methods.len(), 10);
        assert!(methods.contains(&"capabilities"));
        assert!(methods.contains(&"get_capabilities"));
        assert!(methods.contains(&"discover_capabilities"));
        assert!(methods.contains(&"capabilities.list"));
        assert!(methods.contains(&"capability.list"));
        assert!(methods.contains(&"primal.capabilities"));
        assert!(methods.contains(&"identity"));
        assert!(methods.contains(&"identity.get"));
        assert!(methods.contains(&"whoami"));
        assert!(methods.contains(&"get_identity"));
    }

    #[tokio::test]
    async fn test_capabilities_response_has_methods_array() {
        let handler = test_handler();
        let btsp_provider = crate::test_helpers::mocks::create_minimal_beardog_provider().await;

        let result = handler
            .handle("capabilities.list", None, &btsp_provider)
            .await;

        assert!(result.is_ok());
        let response = result.expect("capabilities handler in test");

        assert_eq!(response["primal"], "beardog");
        assert!(response["version"].is_string());
        assert!(response["provided_capabilities"].is_array());
        assert!(response["family_id"].is_string());
        assert!(response["node_id"].is_string());
        assert!(
            response["btsp_enabled"]
                .as_bool()
                .expect("btsp_enabled should be bool in test")
        );

        let methods = response["methods"]
            .as_array()
            .expect("Wire Standard L2: capabilities.list must include flat methods array");
        assert!(!methods.is_empty(), "methods array must not be empty");

        let method_strs: Vec<&str> = methods
            .iter()
            .map(|v| v.as_str().expect("method entry should be string in test"))
            .collect();

        assert!(
            method_strs.iter().all(|m| m.contains('.')),
            "all methods must be fully-qualified dotted names"
        );
        assert!(method_strs.contains(&"crypto.blake3_hash"));
        assert!(method_strs.contains(&"crypto.sign_ed25519"));
        assert!(method_strs.contains(&"health.liveness"));
        assert!(method_strs.contains(&"capabilities.list"));
        assert!(method_strs.contains(&"identity.get"));
    }

    #[tokio::test]
    async fn test_identity_get_wire_standard() {
        let handler = test_handler();
        let btsp_provider = crate::test_helpers::mocks::create_minimal_beardog_provider().await;

        let result = handler.handle("identity.get", None, &btsp_provider).await;

        assert!(result.is_ok());
        let response = result.expect("identity.get handler in test");

        assert_eq!(response["primal"], "beardog");
        assert!(response["version"].is_string());
        assert_eq!(response["domain"], "crypto");
        assert_eq!(response["license"], "AGPL-3.0-or-later");
    }

    #[tokio::test]
    async fn test_legacy_identity_response() {
        let handler = test_handler();
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
        let handler = test_handler();
        let btsp_provider = crate::test_helpers::mocks::create_minimal_beardog_provider().await;

        let result = handler
            .handle("discover_capabilities", None, &btsp_provider)
            .await;

        assert!(result.is_ok());
        let response = result.expect("capabilities handler in test");

        let caps = response["capabilities"]
            .as_array()
            .expect("capabilities should be array in test");
        assert!(caps.len() >= 12, "Expected at least 12 capabilities");

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
        let handler = test_handler();
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
        let handler = test_handler();
        let btsp_provider = crate::test_helpers::mocks::create_minimal_beardog_provider().await;

        for method in &["identity", "identity.get", "whoami", "get_identity"] {
            let result = handler.handle(method, None, &btsp_provider).await;
            assert!(result.is_ok(), "Method {} should succeed", method);
        }
    }
}
