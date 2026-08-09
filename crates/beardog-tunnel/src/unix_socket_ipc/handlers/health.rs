// SPDX-License-Identifier: AGPL-3.0-or-later

//! Health check handler
//!
//! Provides universal health/status/ping endpoints that work across all primals.
//! These methods are essential for service discovery, load balancing, and monitoring.

use super::utils::{IdentityHints, get_primal_name_with};
use super::{HandlerError, HandlerResult, MethodHandler};
use crate::btsp_provider::BeardogBtspProvider;
use chrono::Utc;
use std::sync::Arc;
use tracing::info;

/// Handler for health check methods.
///
/// ## Canonical Methods (use these)
///
/// Per Semantic Method Naming Standard v2.1.0:
///
/// - `health.liveness` — minimal liveness probe → `{"status":"alive"}`
/// - `health.readiness` — readiness probe → includes protocol and capabilities count
/// - `health.check` — deep health check → full health with timestamp
///
/// ## Deprecated Flat Aliases (will be removed in v1.0)
///
/// - `ping` — use `health.liveness`
/// - `health` — use `health.liveness`
/// - `status` — use `health.check`
/// - `check` — use `health.check`
pub struct HealthHandler {
    identity: IdentityHints,
    capabilities_count: usize,
}

impl Default for HealthHandler {
    fn default() -> Self {
        Self::new()
    }
}

impl HealthHandler {
    /// Production: identity from environment (see [`IdentityHints::from_env`]).
    #[must_use]
    pub fn new() -> Self {
        Self {
            identity: IdentityHints::from_env(),
            capabilities_count: 0,
        }
    }

    /// Production with known capabilities count (set during registry construction).
    #[must_use]
    pub fn with_capabilities(count: usize) -> Self {
        Self {
            identity: IdentityHints::from_env(),
            capabilities_count: count,
        }
    }
    /// Tests / DI: explicit identity hints (no `PRIMAL_NAME` env mutation).
    #[must_use]
    pub const fn with_identity_hints(identity: IdentityHints) -> Self {
        Self {
            identity,
            capabilities_count: 0,
        }
    }
}

impl MethodHandler for HealthHandler {
    fn methods(&self) -> Vec<&'static str> {
        vec![
            "ping",
            "health",
            "status",
            "check",
            "health.liveness",
            "health.readiness",
            "health.check",
        ]
    }

    async fn handle(
        &self,
        method: &str,
        _params: Option<&serde_json::Value>,
        _btsp_provider: &Arc<BeardogBtspProvider>,
    ) -> HandlerResult {
        let primal = get_primal_name_with(&self.identity);
        let version = env!("CARGO_PKG_VERSION");

        match method {
            // Liveness: minimal, fast — "am I alive?"
            // "ping" and "health" are deprecated flat aliases — remove in v1.0
            "ping" | "health" | "health.liveness" => {
                info!(probe = "liveness", "Health handler");
                Ok(serde_json::json!({
                    "status": "alive",
                    "primal": primal,
                    "version": version,
                }))
            }
            // Readiness: "can I serve requests?"
            "health.readiness" => {
                info!(probe = "readiness", "Health handler");
                Ok(serde_json::json!({
                    "status": "ready",
                    "primal": primal,
                    "version": version,
                    "protocol": "JSON-RPC",
                    "capabilities_count": self.capabilities_count,
                }))
            }
            // Deep check: full health with timestamp
            "status" | "check" | "health.check" => {
                info!(probe = "deep_check", "Health handler");
                Ok(serde_json::json!({
                    "status": "healthy",
                    "primal": primal,
                    "version": version,
                    "protocol": "JSON-RPC",
                    "timestamp": Utc::now().to_rfc3339(),
                }))
            }
            other => Err(HandlerError::MethodNotFound(other.to_owned())),
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::unix_socket_ipc::handlers::utils::IdentityHints;

    #[tokio::test]
    async fn test_health_handler_methods() {
        let handler = HealthHandler::new();
        let methods = handler.methods();

        assert_eq!(methods.len(), 7);
        assert!(methods.contains(&"ping"));
        assert!(methods.contains(&"health"));
        assert!(methods.contains(&"status"));
        assert!(methods.contains(&"check"));
        assert!(methods.contains(&"health.liveness"));
        assert!(methods.contains(&"health.readiness"));
        assert!(methods.contains(&"health.check"));
    }

    #[tokio::test]
    async fn test_liveness_response() {
        let handler = HealthHandler::with_identity_hints(IdentityHints {
            primal_name: Some("beardog".to_string()),
            ..Default::default()
        });
        let btsp_provider = crate::test_helpers::mocks::create_minimal_beardog_provider().await;

        for method in &["ping", "health", "health.liveness"] {
            let result = handler
                .handle(method, None, &btsp_provider)
                .await
                .expect("liveness should succeed");
            assert_eq!(result["status"], "alive", "liveness status for {method}");
            assert_eq!(result["primal"], "beardog");
            assert!(result["version"].is_string());
            assert!(
                result.get("timestamp").is_none(),
                "liveness has no timestamp"
            );
        }
    }

    #[tokio::test]
    async fn test_readiness_response() {
        let mut handler = HealthHandler::with_identity_hints(IdentityHints {
            primal_name: Some("beardog".to_string()),
            ..Default::default()
        });
        handler.capabilities_count = 91;
        let btsp_provider = crate::test_helpers::mocks::create_minimal_beardog_provider().await;

        let result = handler
            .handle("health.readiness", None, &btsp_provider)
            .await
            .expect("readiness should succeed");
        assert_eq!(result["status"], "ready");
        assert_eq!(result["capabilities_count"], 91);
        assert_eq!(result["protocol"], "JSON-RPC");
    }

    #[tokio::test]
    async fn test_deep_check_response() {
        let handler = HealthHandler::with_identity_hints(IdentityHints {
            primal_name: Some("beardog".to_string()),
            ..Default::default()
        });
        let btsp_provider = crate::test_helpers::mocks::create_minimal_beardog_provider().await;

        for method in &["status", "check", "health.check"] {
            let result = handler
                .handle(method, None, &btsp_provider)
                .await
                .expect("deep check should succeed");
            assert_eq!(result["status"], "healthy", "deep check for {method}");
            assert!(result["timestamp"].is_string(), "deep check has timestamp");
            assert_eq!(result["protocol"], "JSON-RPC");
        }
    }

    #[tokio::test]
    async fn test_all_method_names() {
        let handler = HealthHandler::new();
        let btsp_provider = crate::test_helpers::mocks::create_minimal_beardog_provider().await;

        for method in &[
            "ping",
            "health",
            "status",
            "check",
            "health.liveness",
            "health.readiness",
            "health.check",
        ] {
            let result = handler.handle(method, None, &btsp_provider).await;
            assert!(result.is_ok(), "Method {method} should succeed");
        }
    }
}
