//! Capabilities handler
//!
//! Provides self-description and identity endpoints for service discovery.
//! Every primal should expose these methods to enable capability-based discovery.

use super::MethodHandler;
use crate::btsp_provider::BeardogBtspProvider;
use async_trait::async_trait;
use beardog_types::primal_identity::PrimalIdentity;
use std::sync::Arc;
use tracing::info;

/// Get the primal name using self-knowledge pattern.
/// Uses PRIMAL_NAME or BEARDOG_NAME env var, falls back to "beardog".
fn get_primal_name() -> String {
    std::env::var("PRIMAL_NAME")
        .or_else(|_| std::env::var("BEARDOG_NAME"))
        .unwrap_or_else(|_| "beardog".to_string())
}

/// Handler for capabilities and identity methods
///
/// Supports:
/// - `capabilities` / `get_capabilities` - Lists all provided capabilities
/// - `identity` / `whoami` / `get_identity` - Returns primal identity
///
/// All responses include genetic lineage (family_id, node_id) discovered
/// from environment variables at runtime (no hardcoding).
pub struct CapabilitiesHandler {
    identity: Arc<PrimalIdentity>,
}

#[async_trait]
impl MethodHandler for CapabilitiesHandler {
    fn methods(&self) -> Vec<&'static str> {
        vec![
            "capabilities",
            "get_capabilities",
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
            "capabilities" | "get_capabilities" => self.handle_capabilities().await,
            "identity" | "whoami" | "get_identity" => self.handle_identity().await,
            _ => Err(format!("Method not found: {}", method)),
        }
    }
}

impl CapabilitiesHandler {
    /// Create a new CapabilitiesHandler with explicit identity injection
    pub fn new(identity: Arc<PrimalIdentity>) -> Self {
        Self { identity }
    }

    /// Handle capabilities request
    ///
    /// Returns a comprehensive list of all capabilities provided by BearDog,
    /// including crypto, security, BTSP, graph security, and JWT generation.
    async fn handle_capabilities(&self) -> Result<serde_json::Value, String> {
        // Use injected identity (no environment variables!)
        let family_id = self.identity.family_id();
        let node_id = self.identity.node_id();

        info!("🎯 Capabilities requested - exposing our capabilities");

        Ok(serde_json::json!({
            "primal": get_primal_name(),
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
                    "description": "JWT secret generation for authentication systems (e.g., NestGate)"
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
                    "description": "Pure Rust cryptographic operations for Songbird TLS and other primals - Ed25519, X25519, ChaCha20-Poly1305, Blake3, HMAC"
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
                }
            ],
            "version": env!("CARGO_PKG_VERSION"),
            "protocols": ["tarpc", "json-rpc", "http"],
            "btsp_enabled": true,
            "collaborative_intelligence": true,
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
            "primal": "beardog",
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

    #[tokio::test]
    async fn test_capabilities_handler_methods() {
        let identity = Arc::new(PrimalIdentity::for_test("test-family", "test-node"));
        let handler = CapabilitiesHandler::new(identity);
        let methods = handler.methods();

        assert_eq!(methods.len(), 5);
        assert!(methods.contains(&"capabilities"));
        assert!(methods.contains(&"get_capabilities"));
        assert!(methods.contains(&"identity"));
        assert!(methods.contains(&"whoami"));
        assert!(methods.contains(&"get_identity"));
    }

    #[tokio::test]
    async fn test_capabilities_response() {
        let identity = Arc::new(PrimalIdentity::for_test("test-family", "test-node"));
        let handler = CapabilitiesHandler::new(identity);
        let btsp_provider = crate::test_helpers::mocks::create_minimal_beardog_provider().await;

        let result = handler.handle("capabilities", None, &btsp_provider).await;

        assert!(result.is_ok());
        let response = result.unwrap();

        assert_eq!(response["primal"], "beardog");
        assert!(response["provided_capabilities"].is_array());
        assert!(response["family_id"].is_string());
        assert!(response["node_id"].is_string());
        assert!(response["btsp_enabled"].as_bool().unwrap());
    }

    #[tokio::test]
    async fn test_identity_response() {
        let identity = Arc::new(PrimalIdentity::for_test("test-family", "test-node"));
        let handler = CapabilitiesHandler::new(identity);
        let btsp_provider = crate::test_helpers::mocks::create_minimal_beardog_provider().await;

        let result = handler.handle("identity", None, &btsp_provider).await;

        assert!(result.is_ok());
        let response = result.unwrap();

        assert_eq!(response["primal"], "beardog");
        assert!(response["family"].is_string());
        assert!(response["node"].is_string());
        assert!(response["encryption_tag"].is_string());
    }

    #[tokio::test]
    async fn test_all_capability_aliases() {
        let identity = Arc::new(PrimalIdentity::for_test("test-family", "test-node"));
        let handler = CapabilitiesHandler::new(identity);
        let btsp_provider = crate::test_helpers::mocks::create_minimal_beardog_provider().await;

        for method in &["capabilities", "get_capabilities"] {
            let result = handler.handle(method, None, &btsp_provider).await;
            assert!(result.is_ok(), "Method {} should succeed", method);
        }
    }

    #[tokio::test]
    async fn test_all_identity_aliases() {
        let identity = Arc::new(PrimalIdentity::for_test("test-family", "test-node"));
        let handler = CapabilitiesHandler::new(identity);
        let btsp_provider = crate::test_helpers::mocks::create_minimal_beardog_provider().await;

        for method in &["identity", "whoami", "get_identity"] {
            let result = handler.handle(method, None, &btsp_provider).await;
            assert!(result.is_ok(), "Method {} should succeed", method);
        }
    }
}
