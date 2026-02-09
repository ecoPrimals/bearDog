//! Health check handler
//!
//! Provides universal health/status/ping endpoints that work across all primals.
//! These methods are essential for service discovery, load balancing, and monitoring.

use super::utils::get_primal_name;
use super::MethodHandler;
use crate::btsp_provider::BeardogBtspProvider;
use async_trait::async_trait;
use chrono::Utc;
use std::sync::Arc;
use tracing::info;

/// Handler for health check methods
///
/// Supports multiple method names for compatibility:
/// - `ping` - Simple connectivity test
/// - `health` - Health status
/// - `status` - Service status
/// - `check` - Generic check
///
/// All return the same response with service metadata.
pub struct HealthHandler;

#[async_trait]
impl MethodHandler for HealthHandler {
    fn methods(&self) -> Vec<&'static str> {
        vec!["ping", "health", "status", "check"]
    }

    async fn handle(
        &self,
        _method: &str,
        _params: Option<&serde_json::Value>,
        _btsp_provider: &Arc<BeardogBtspProvider>,
    ) -> Result<serde_json::Value, String> {
        info!("🏥 Health check requested");

        Ok(serde_json::json!({
            "status": "healthy",
            "primal": get_primal_name(),
            "version": env!("CARGO_PKG_VERSION"),
            "protocol": "JSON-RPC",
            "timestamp": Utc::now().to_rfc3339(),
        }))
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[tokio::test]
    async fn test_health_handler_methods() {
        let handler = HealthHandler;
        let methods = handler.methods();

        assert_eq!(methods.len(), 4);
        assert!(methods.contains(&"ping"));
        assert!(methods.contains(&"health"));
        assert!(methods.contains(&"status"));
        assert!(methods.contains(&"check"));
    }

    #[tokio::test]
    async fn test_health_check_response() {
        let handler = HealthHandler;

        // Use safe mock provider (health handler doesn't actually use it)
        let btsp_provider = crate::test_helpers::mocks::create_minimal_beardog_provider().await;

        let result = handler.handle("ping", None, &btsp_provider).await;

        assert!(result.is_ok());
        let response = result.unwrap();

        assert_eq!(response["status"], "healthy");
        assert_eq!(response["primal"], "beardog");
        assert_eq!(response["protocol"], "JSON-RPC");
        assert!(response["version"].is_string());
        assert!(response["timestamp"].is_string());
    }

    #[tokio::test]
    async fn test_all_method_names() {
        let handler = HealthHandler;
        let btsp_provider = crate::test_helpers::mocks::create_minimal_beardog_provider().await;

        for method in &["ping", "health", "status", "check"] {
            let result = handler.handle(method, None, &btsp_provider).await;
            assert!(result.is_ok(), "Method {} should succeed", method);
        }
    }
}
