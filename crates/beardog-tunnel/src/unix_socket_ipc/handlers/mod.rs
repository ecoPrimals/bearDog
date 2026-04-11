// SPDX-License-Identifier: AGPL-3.0-or-later

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
use beardog_ipc::{DispatchOutcome, IpcErrorPhase};
use std::sync::Arc;

// Dark Forest Beacon Genetics (Phase 1 - Feb 2026)
pub mod beacon;

// Shared utilities (self-knowledge pattern, etc.)
pub mod utils;

// Existing handlers
pub mod btsp;
pub mod capabilities;
pub mod crypto; // Refactored crypto handlers module (domain-based organization)
pub mod crypto_handler; // Crypto RPC handler (routes to crypto module)
pub mod encryption;
pub mod federation;
pub mod graph_security;
pub mod health;
pub mod introspection; // Primal introspection (primal.info, rpc.methods)
pub mod ionic_bond;
pub mod relay; // Relay authorization (lineage-gated, for coordinated punch)
pub mod secrets; // Encrypted secret storage (family-scoped, ChaCha20-Poly1305)
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
    /// * `method` - The JSON-RPC method name (e.g., "`crypto.sign_ed25519`")
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
    handlers: tokio::sync::RwLock<Vec<Arc<dyn MethodHandler>>>,
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
    ///
    /// # Panics
    ///
    /// This function is panic-free under normal operation. The only panic path
    /// would be if lock acquisition fails during construction, which should never
    /// happen since we hold the only reference at that point.
    pub fn new(identity: Arc<beardog_types::primal_identity::PrimalIdentity>) -> Arc<Self> {
        // Two-phase construction: handlers that need a back-reference to the
        // registry (CapabilitiesHandler, IntrospectionHandler) are added in Phase 2.

        // Phase 1: handlers that do NOT need registry access
        let registry = Arc::new(Self {
            handlers: tokio::sync::RwLock::new(vec![
                Arc::new(health::HealthHandler::new()),
                Arc::new(security::SecurityHandler::new(identity.clone())),
                Arc::new(btsp::BtspHandler::new()),
                Arc::new(ionic_bond::IonicBondHandler::new()),
                Arc::new(crypto_handler::CryptoHandler),
                Arc::new(federation::FederationHandler::new(identity.clone())),
                Arc::new(encryption::EncryptionHandler),
                Arc::new(graph_security::GraphSecurityHandler),
                Arc::new(beacon::BeaconHandler::new()),
                Arc::new(secrets::SecretsHandler::new(identity.clone())),
                Arc::new(relay::RelayHandler::new(identity.clone())),
            ]),
        });

        // Phase 2: handlers that need registry access for method enumeration.
        // Creates intentional Arc cycles — these handlers call registry.all_methods().
        let capabilities_handler = Arc::new(capabilities::CapabilitiesHandler::new(
            identity,
            registry.clone(),
        ));
        let introspection = Arc::new(introspection::IntrospectionHandler::new(registry.clone()));

        match registry.handlers.try_write() {
            Ok(mut handlers) => {
                handlers.push(capabilities_handler);
                handlers.push(introspection);
            }
            Err(_) => {
                tracing::error!(
                    "UNEXPECTED: Failed to acquire write lock during HandlerRegistry initialization. \
                     Capabilities and introspection handlers will not be available. This is a bug."
                );
            }
        }

        registry
    }

    /// # Errors
    ///
    /// Returns an error if decryption fails.
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
    /// use a `HashMap` for O(1) lookup.
    pub async fn route(
        &self,
        method: &str,
        params: Option<&serde_json::Value>,
        btsp_provider: &Arc<BeardogBtspProvider>,
    ) -> Result<serde_json::Value, String> {
        // Backward-compat bridge: bare crypto names → namespaced equivalents.
        // Consuming primals may call bare names during migration to
        // capability-based routing via the Neural API.
        let method = match method {
            "x25519_generate_ephemeral" => "crypto.x25519_generate_ephemeral",
            "x25519_derive_secret" => "crypto.x25519_derive_secret",
            "sign_ed25519" => "crypto.sign_ed25519",
            "verify_ed25519" => "crypto.verify_ed25519",
            "chacha20_poly1305_encrypt" => "crypto.chacha20_poly1305_encrypt",
            "chacha20_poly1305_decrypt" => "crypto.chacha20_poly1305_decrypt",
            "hmac_sha256" => "crypto.hmac_sha256",
            "blake3_hash" => "crypto.blake3_hash",
            other => other,
        };

        // Try each handler in order
        let handlers = self.handlers.read().await;
        for handler in handlers.iter() {
            if handler.methods().contains(&method) {
                return handler.handle(method, params, btsp_provider).await;
            }
        }

        // No handler found (JSON-RPC 2.0 error message)
        Err(format!("Method not found: {method}"))
    }

    /// Route a request and return a structured [`DispatchOutcome`].
    ///
    /// Same dispatch logic as [`route`](Self::route), but returns a typed outcome
    /// that distinguishes transport/protocol/dispatch/application failures.
    pub async fn route_with_outcome(
        &self,
        method: &str,
        params: Option<&serde_json::Value>,
        btsp_provider: &Arc<BeardogBtspProvider>,
    ) -> DispatchOutcome {
        let result = self.route(method, params, btsp_provider).await;
        match result {
            Ok(v) => DispatchOutcome::Success(v),
            Err(ref msg) if msg.contains("Method not found") => DispatchOutcome::Failure {
                phase: IpcErrorPhase::Dispatch,
                code: -32601,
                message: msg.clone(),
            },
            Err(ref msg) if msg.contains("Invalid params") => DispatchOutcome::Failure {
                phase: IpcErrorPhase::Dispatch,
                code: -32602,
                message: msg.clone(),
            },
            Err(msg) => DispatchOutcome::Failure {
                phase: IpcErrorPhase::Application,
                code: -32000,
                message: msg,
            },
        }
    }

    /// Get all methods from all handlers
    ///
    /// Returns a sorted list of all method names exposed by all handlers.
    /// Used by introspection handler for `rpc.methods`.
    pub async fn all_methods(&self) -> Vec<String> {
        let handlers = self.handlers.read().await;
        let mut methods = Vec::new();
        for handler in handlers.iter() {
            methods.extend(handler.methods().iter().map(|s| (*s).to_string()));
        }
        methods.sort();
        methods
    }
}

#[cfg(test)]
impl HandlerRegistry {
    /// Create a test registry with default test identity
    ///
    /// Only available in tests. Production code must provide explicit identity.
    pub fn default() -> Arc<Self> {
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
    ///
    /// Uses the minimal test provider from `test_helpers`, which bypasses
    /// full HSM initialization while providing a valid `BeardogBtspProvider`.
    async fn create_test_provider() -> Arc<BeardogBtspProvider> {
        crate::test_helpers::mocks::create_minimal_beardog_provider().await
    }

    #[tokio::test]
    async fn test_registry_creation() {
        use beardog_types::primal_identity::PrimalIdentity;
        use std::sync::Arc;

        let identity = Arc::new(PrimalIdentity::for_test("test", "node1"));
        let registry = HandlerRegistry::new(identity);
        assert!(!registry.handlers.read().await.is_empty());
    }

    #[test]
    fn test_handler_methods() {
        let handler = TestHandler;
        let methods = handler.methods();
        assert_eq!(methods, vec!["test.method"]);
    }

    // Integration tests will be added as we extract more handlers
}
