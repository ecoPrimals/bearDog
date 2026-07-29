// SPDX-License-Identifier: AGPL-3.0-or-later

//! `capability.call` routing dispatcher.
//!
//! Translates ecosystem orchestrator calls of the form:
//!
//! ```json
//! {"method": "capability.call", "params": {
//!     "capability": "tls",
//!     "operation": "sign_handshake",
//!     "args": { ... }
//! }}
//! ```
//!
//! into concrete bearDog method calls (`tls.sign_handshake`) by re-dispatching
//! through the existing `HandlerRegistry`.
//!
//! This bridges the ecosystem capability-based RPC convention to bearDog's
//! semantic method namespace, enabling cross-gate federation handshakes.

use super::{HandlerRegistry, HandlerResult, MethodHandler};
use crate::btsp_provider::BeardogBtspProvider;
use std::sync::Arc;
use tracing::info;

/// Handler for `capability.call` — routes capability-addressed requests to
/// their concrete method implementations.
pub struct CapabilityCallHandler {
    registry: Arc<HandlerRegistry>,
}

impl CapabilityCallHandler {
    /// Create a new handler with a back-reference to the registry for re-dispatch.
    pub const fn new(registry: Arc<HandlerRegistry>) -> Self {
        Self { registry }
    }

    /// Resolve `(capability, operation)` to a concrete method name.
    ///
    /// Strategy (tried in order):
    /// 1. `{capability}.{operation}` — direct namespace (covers `tls.*`, `btsp.*`, `crypto.*`)
    /// 2. `{capability}_{operation}` — underscore convention (covers `crypto.sign_ed25519`)
    fn resolve_method(capability: &str, operation: &str) -> String {
        format!("{capability}.{operation}")
    }
}

impl MethodHandler for CapabilityCallHandler {
    fn methods(&self) -> Vec<&'static str> {
        vec!["capability.call"]
    }

    async fn handle(
        &self,
        _method: &str,
        params: Option<&serde_json::Value>,
        btsp_provider: &Arc<BeardogBtspProvider>,
    ) -> HandlerResult {
        let Some(params) = params else {
            return Err(super::HandlerError::InvalidParams(
                "capability.call requires params with 'capability' and 'operation'".to_owned(),
            ));
        };

        let capability = params
            .get("capability")
            .and_then(serde_json::Value::as_str)
            .ok_or_else(|| {
                super::HandlerError::InvalidParams("missing required field: capability".to_owned())
            })?;

        let operation = params
            .get("operation")
            .and_then(serde_json::Value::as_str)
            .ok_or_else(|| {
                super::HandlerError::InvalidParams("missing required field: operation".to_owned())
            })?;

        let target_method = Self::resolve_method(capability, operation);

        if target_method == "capability.call" {
            return Err(super::HandlerError::InvalidParams(
                "capability.call cannot route to itself".to_owned(),
            ));
        }

        let args = params.get("args").cloned();

        info!(
            capability,
            operation,
            target = %target_method,
            "capability.call → routing"
        );

        let registry = self.registry.clone();
        let btsp = btsp_provider.clone();
        Box::pin(async move { registry.route(&target_method, args.as_ref(), &btsp).await }).await
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use beardog_types::primal_identity::PrimalIdentity;

    async fn make_registry() -> Arc<HandlerRegistry> {
        let identity = Arc::new(PrimalIdentity::for_test("beardog", "test-node"));
        HandlerRegistry::new(identity)
    }

    async fn make_btsp() -> Arc<BeardogBtspProvider> {
        crate::test_helpers::mocks::create_minimal_beardog_provider().await
    }

    #[test]
    fn exposes_capability_call_method() {
        let rt = tokio::runtime::Builder::new_current_thread()
            .enable_all()
            .build()
            .unwrap();
        let registry = rt.block_on(make_registry());
        let handler = CapabilityCallHandler::new(registry);
        assert_eq!(handler.methods(), vec!["capability.call"]);
    }

    #[tokio::test]
    async fn routes_tls_sign_handshake() {
        let registry = make_registry().await;
        let handler = CapabilityCallHandler::new(registry);
        let btsp = make_btsp().await;

        let params = serde_json::json!({
            "capability": "tls",
            "operation": "sign_handshake",
            "args": {
                "handshake_hash": "deadbeef"
            }
        });

        let result = handler
            .handle("capability.call", Some(&params), &btsp)
            .await;
        // Should NOT be MethodNotFound — it reaches tls.sign_handshake
        match &result {
            Err(super::super::HandlerError::MethodNotFound(_)) => {
                panic!("capability.call returned MethodNotFound — routing failed");
            }
            _ => {} // Any other result means routing worked (may fail on params, that's ok)
        }
    }

    #[tokio::test]
    async fn routes_health_liveness() {
        let registry = make_registry().await;
        let handler = CapabilityCallHandler::new(registry);
        let btsp = make_btsp().await;

        let params = serde_json::json!({
            "capability": "health",
            "operation": "liveness",
            "args": {}
        });

        let result = handler
            .handle("capability.call", Some(&params), &btsp)
            .await;
        assert!(result.is_ok(), "health.liveness should route successfully");
        let val = result.unwrap();
        assert_eq!(val["status"], "alive");
    }

    #[tokio::test]
    async fn routes_btsp_negotiate() {
        let registry = make_registry().await;
        let handler = CapabilityCallHandler::new(registry);
        let btsp = make_btsp().await;

        let params = serde_json::json!({
            "capability": "btsp",
            "operation": "negotiate",
            "args": {}
        });

        let result = handler
            .handle("capability.call", Some(&params), &btsp)
            .await;
        match &result {
            Err(super::super::HandlerError::MethodNotFound(_)) => {
                panic!("capability.call failed to route btsp.negotiate");
            }
            _ => {}
        }
    }

    #[tokio::test]
    async fn rejects_missing_params() {
        let registry = make_registry().await;
        let handler = CapabilityCallHandler::new(registry);
        let btsp = make_btsp().await;

        let result = handler.handle("capability.call", None, &btsp).await;
        assert!(result.is_err());
        let err = result.unwrap_err();
        assert!(err.to_string().contains("requires params"));
    }

    #[tokio::test]
    async fn rejects_missing_capability() {
        let registry = make_registry().await;
        let handler = CapabilityCallHandler::new(registry);
        let btsp = make_btsp().await;

        let params = serde_json::json!({"operation": "foo"});
        let result = handler
            .handle("capability.call", Some(&params), &btsp)
            .await;
        assert!(result.is_err());
        assert!(result.unwrap_err().to_string().contains("capability"));
    }

    #[tokio::test]
    async fn rejects_missing_operation() {
        let registry = make_registry().await;
        let handler = CapabilityCallHandler::new(registry);
        let btsp = make_btsp().await;

        let params = serde_json::json!({"capability": "tls"});
        let result = handler
            .handle("capability.call", Some(&params), &btsp)
            .await;
        assert!(result.is_err());
        assert!(result.unwrap_err().to_string().contains("operation"));
    }

    #[tokio::test]
    async fn rejects_recursive_call() {
        let registry = make_registry().await;
        let handler = CapabilityCallHandler::new(registry);
        let btsp = make_btsp().await;

        let params = serde_json::json!({
            "capability": "capability",
            "operation": "call",
            "args": {}
        });

        let result = handler
            .handle("capability.call", Some(&params), &btsp)
            .await;
        assert!(result.is_err());
        assert!(
            result
                .unwrap_err()
                .to_string()
                .contains("cannot route to itself")
        );
    }

    #[tokio::test]
    async fn returns_method_not_found_for_unknown_capability() {
        let registry = make_registry().await;
        let handler = CapabilityCallHandler::new(registry);
        let btsp = make_btsp().await;

        let params = serde_json::json!({
            "capability": "nonexistent",
            "operation": "fake_method",
            "args": {}
        });

        let result = handler
            .handle("capability.call", Some(&params), &btsp)
            .await;
        assert!(result.is_err());
        match result.unwrap_err() {
            super::super::HandlerError::MethodNotFound(m) => {
                assert_eq!(m, "nonexistent.fake_method");
            }
            other => panic!("Expected MethodNotFound, got: {other:?}"),
        }
    }

    #[tokio::test]
    async fn routes_crypto_sign_ed25519_with_direct_key() {
        use base64::Engine;
        use crate::unix_socket_ipc::handlers::crypto::{
            handle_ed25519_generate_keypair, handle_verify_ed25519,
        };

        let registry = make_registry().await;
        let handler = CapabilityCallHandler::new(registry);
        let btsp = make_btsp().await;

        let kp = handle_ed25519_generate_keypair(None)
            .await
            .expect("keygen");
        let sk = kp["secret_key"].as_str().expect("sk");
        let pk = kp["public_key"].as_str().expect("pk");

        let msg = base64::engine::general_purpose::STANDARD.encode(b"provenance entry");

        let params = serde_json::json!({
            "capability": "crypto",
            "operation": "sign_ed25519",
            "args": {
                "message": msg,
                "secret_key": sk,
            }
        });
        let signed = handler
            .handle("capability.call", Some(&params), &btsp)
            .await
            .expect("capability.call → crypto.sign_ed25519 must succeed");

        assert_eq!(signed["algorithm"], "Ed25519");
        assert_eq!(signed["public_key"].as_str().expect("pk"), pk);

        let verify_params = serde_json::json!({
            "message": msg,
            "signature": signed["signature"].as_str().expect("sig"),
            "public_key": pk,
        });
        let v = handle_verify_ed25519(Some(&verify_params))
            .await
            .expect("verify");
        assert_eq!(
            v["valid"], true,
            "capability.call → crypto.sign_ed25519 → verify must pass"
        );
    }

    #[test]
    fn resolve_method_joins_with_dot() {
        assert_eq!(
            CapabilityCallHandler::resolve_method("tls", "sign_handshake"),
            "tls.sign_handshake"
        );
        assert_eq!(
            CapabilityCallHandler::resolve_method("btsp", "negotiate"),
            "btsp.negotiate"
        );
        assert_eq!(
            CapabilityCallHandler::resolve_method("crypto", "sign_ed25519"),
            "crypto.sign_ed25519"
        );
    }
}
