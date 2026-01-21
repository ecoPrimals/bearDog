//! Health check handler
//!
//! Provides universal health/status/ping endpoints that work across all primals.
//! These methods are essential for service discovery, load balancing, and monitoring.

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
            "primal": "beardog",
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

        // Create a mock provider (not actually used by health handler)
        // In a real test, we'd need proper initialization
        let btsp_provider = Arc::new(unsafe {
            // SAFETY: Health handler doesn't actually access the provider
            // This is a test-only hack to avoid complex HSM setup
            std::mem::zeroed()
        });

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
        let btsp_provider = Arc::new(unsafe { std::mem::zeroed() });

        for method in &["ping", "health", "status", "check"] {
            let result = handler.handle(method, None, &btsp_provider).await;
            assert!(result.is_ok(), "Method {} should succeed", method);
        }
    }
}

