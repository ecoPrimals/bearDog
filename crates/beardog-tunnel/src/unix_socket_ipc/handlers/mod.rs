//! Modular handler architecture for JSON-RPC methods
//!
//! This module implements a trait-based handler registry pattern that allows
//! for extensible, testable, and maintainable JSON-RPC method handlers.
//!
//! # Architecture
//!
//! Instead of a giant match statement with inline handlers, we use:
//! - `MethodHandler` trait: Common interface for all handlers
//! - `HandlerRegistry`: Routes requests to appropriate handlers
//! - Individual handler modules: Focused, testable implementations
//!
//! # Benefits
//!
//! - **Extensibility**: Add new handlers by implementing trait
//! - **Testability**: Each handler can be unit tested independently
//! - **Maintainability**: Small focused modules (< 500 lines each)
//! - **Loose Coupling**: Handlers don't know about each other
//! - **Discoverability**: Registry pattern makes all handlers visible
//!
//! # Example
//!
//! ```rust,ignore
//! use beardog_tunnel::unix_socket_ipc::handlers::HandlerRegistry;
//! use std::sync::Arc;
//!
//! // Create registry with all handlers
//! let registry = HandlerRegistry::new();
//!
//! // Route a request (async context required)
//! let result = registry.route("health", None, &btsp_provider).await;
//! ```

use crate::btsp_provider::BeardogBtspProvider;
use async_trait::async_trait;
use std::sync::Arc;

pub mod btsp;
pub mod capabilities;
pub mod crypto; // Refactored crypto handlers module (domain-based organization)
pub mod crypto_handler; // Crypto RPC handler (routes to crypto module)
pub mod encryption;
pub mod federation;
pub mod graph_security;
pub mod health;
pub mod security;

/// Trait for JSON-RPC method handlers
///
/// Implement this trait to create a new handler that can be registered
/// with the `HandlerRegistry`. Each handler is responsible for handling
/// one or more related JSON-RPC methods.
///
/// # Example
///
/// ```rust,ignore
/// use async_trait::async_trait;
///
/// struct MyHandler;
///
/// #[async_trait]
/// impl MethodHandler for MyHandler {
///     fn methods(&self) -> Vec<&'static str> {
///         vec!["my.method1", "my.method2"]
///     }
///
///     async fn handle(
///         &self,
///         method: &str,
///         params: Option<&serde_json::Value>,
///         btsp_provider: &Arc<BeardogBtspProvider>,
///     ) -> Result<serde_json::Value, String> {
///         match method {
///             "my.method1" => Ok(serde_json::json!({"result": "ok"})),
///             "my.method2" => Ok(serde_json::json!({"result": "ok2"})),
///             _ => Err(format!("Unknown method: {}", method)),
///         }
///     }
/// }
/// ```
#[async_trait]
pub trait MethodHandler: Send + Sync {
    /// Get the methods this handler can handle
    ///
    /// Returns a list of method names (including namespace) that this
    /// handler supports. The registry uses this for routing.
    fn methods(&self) -> Vec<&'static str>;

    /// Handle a JSON-RPC request
    ///
    /// # Arguments
    /// * `method` - The JSON-RPC method name (e.g., "crypto.sign_ed25519")
    /// * `params` - Optional parameters for the method
    /// * `btsp_provider` - The BTSP provider for accessing capabilities
    ///
    /// # Returns
    /// The result as a JSON value, or an error message
    async fn handle(
        &self,
        method: &str,
        params: Option<&serde_json::Value>,
        btsp_provider: &Arc<BeardogBtspProvider>,
    ) -> Result<serde_json::Value, String>;
}

/// Registry of all JSON-RPC method handlers
///
/// The registry maintains a list of all registered handlers and routes
/// incoming requests to the appropriate handler based on the method name.
///
/// # Architecture
///
/// The registry uses dynamic dispatch (`Arc<dyn MethodHandler>`) to allow
/// different handler types to coexist in a single collection. This is a
/// zero-cost abstraction at runtime (single vtable lookup per request).
///
/// # Thread Safety
///
/// The registry is `Send + Sync` and can be safely shared across threads.
/// All handlers must also be `Send + Sync`.
pub struct HandlerRegistry {
    handlers: Vec<Arc<dyn MethodHandler>>,
}

impl HandlerRegistry {
    /// Create a new registry with all handlers registered
    ///
    /// This instantiates all handlers and registers them with the registry.
    /// New handlers should be added to this list.
    ///
    /// # Arguments
    ///
    /// * `identity` - Primal identity (family and node) for handlers that need it
    pub fn new(identity: Arc<beardog_types::primal_identity::PrimalIdentity>) -> Self {
        Self {
            handlers: vec![
                Arc::new(health::HealthHandler),
                Arc::new(capabilities::CapabilitiesHandler::new(identity.clone())),
                Arc::new(security::SecurityHandler::new(identity.clone())),
                Arc::new(btsp::BtspHandler),
                Arc::new(crypto_handler::CryptoHandler),
                Arc::new(federation::FederationHandler::new(identity.clone())),
                Arc::new(encryption::EncryptionHandler),
                Arc::new(graph_security::GraphSecurityHandler),
                // All handlers now extracted to modular architecture!
                // Legacy handler will only be used for HTTP fallback
            ],
        }
    }

    /// Route a request to the appropriate handler
    ///
    /// Iterates through all registered handlers and delegates to the first
    /// handler that claims to support the given method.
    ///
    /// # Arguments
    /// * `method` - The JSON-RPC method name
    /// * `params` - Optional parameters
    /// * `btsp_provider` - The BTSP provider
    ///
    /// # Returns
    /// The result from the handler, or an error if no handler is found
    ///
    /// # Performance
    ///
    /// O(n) where n is the number of handlers. In practice, n is small (< 20)
    /// and the lookup is very fast. If performance becomes an issue, we can
    /// use a HashMap for O(1) lookup.
    pub async fn route(
        &self,
        method: &str,
        params: Option<&serde_json::Value>,
        btsp_provider: &Arc<BeardogBtspProvider>,
    ) -> Result<serde_json::Value, String> {
        // Try each handler in order
        for handler in &self.handlers {
            if handler.methods().iter().any(|m| *m == method) {
                return handler.handle(method, params, btsp_provider).await;
            }
        }

        // No handler found (JSON-RPC 2.0 error message)
        Err(format!("Method not found: {}", method))
    }
}

#[cfg(test)]
impl Default for HandlerRegistry {
    /// Create a test registry with default test identity
    ///
    /// Only available in tests. Production code must provide explicit identity.
    fn default() -> Self {
        let identity = Arc::new(beardog_types::primal_identity::PrimalIdentity::for_test(
            "test-family",
            "test-node",
        ));
        Self::new(identity)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    /// Test handler for testing the registry
    struct TestHandler;

    #[async_trait]
    impl MethodHandler for TestHandler {
        fn methods(&self) -> Vec<&'static str> {
            vec!["test.method"]
        }

        async fn handle(
            &self,
            method: &str,
            _params: Option<&serde_json::Value>,
            _btsp_provider: &Arc<BeardogBtspProvider>,
        ) -> Result<serde_json::Value, String> {
            match method {
                "test.method" => Ok(serde_json::json!({"test": "ok"})),
                _ => Err(format!("Unknown method: {}", method)),
            }
        }
    }

    /// Helper to create a test BTSP provider
    fn create_test_provider() -> Arc<BeardogBtspProvider> {
        // This would need proper initialization in a real test
        // For now, we'll skip testing that requires a real provider
        unimplemented!("Test provider creation needs HSM setup")
    }

    #[test]
    fn test_registry_creation() {
        use beardog_types::primal_identity::PrimalIdentity;
        use std::sync::Arc;

        let identity = Arc::new(PrimalIdentity::for_test("test", "node1"));
        let registry = HandlerRegistry::new(identity);
        assert!(!registry.handlers.is_empty());
    }

    #[test]
    fn test_handler_methods() {
        let handler = TestHandler;
        let methods = handler.methods();
        assert_eq!(methods, vec!["test.method"]);
    }

    // Integration tests will be added as we extract more handlers
}
