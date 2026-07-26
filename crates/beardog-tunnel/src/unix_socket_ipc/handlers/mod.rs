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
use beardog_ipc::{DispatchOutcome, IpcErrorPhase, OrchestratorRegistryClient};
use std::collections::HashMap;
use std::sync::Arc;

/// Strip filesystem paths, OS error details, and internal state from error
/// messages before they cross the IPC boundary.  Full details are logged
/// server-side via `tracing::error!`.
fn sanitize_error_message(raw: &str) -> String {
    let mut out = String::with_capacity(raw.len());
    let mut chars = raw.chars().peekable();

    while let Some(ch) = chars.next() {
        if ch == '/'
            && chars
                .peek()
                .is_some_and(|c| c.is_alphanumeric() || *c == '.')
        {
            out.push_str("<path>");
            for c in chars.by_ref() {
                if c == ' ' || c == ':' || c == '\'' || c == '"' || c == ')' {
                    out.push(c);
                    break;
                }
            }
        } else {
            out.push(ch);
        }
    }

    out.replace("(os error", "(os error)")
        .replace("(os error))", "(os error)")
}

/// Typed error for JSON-RPC method handlers.
///
/// Replaces raw `String` errors with structured variants that map directly
/// to JSON-RPC error codes, while maintaining backward compatibility via
/// `From<String>` (maps to [`HandlerError::Application`]).
///
/// # Migration
///
/// Existing handlers returning `Err("some message".to_string())` compile
/// unchanged — the `?` operator and `From<String>` bridge transparently.
/// New handlers should prefer [`HandlerError::InvalidParams`] or
/// [`HandlerError::Domain`] for richer error taxonomy.
#[derive(Debug, PartialEq, thiserror::Error)]
pub enum HandlerError {
    /// The requested method does not exist. Maps to JSON-RPC `-32601`.
    #[error("Method not found: {0}")]
    MethodNotFound(String),

    /// Required parameters are missing or malformed. Maps to JSON-RPC `-32602`.
    #[error("Invalid params: {0}")]
    InvalidParams(String),

    /// Application-level error (legacy `String` bridge). Maps to JSON-RPC `-32000`.
    #[error("{0}")]
    Application(String),

    /// Structured domain error from the `BearDogError` taxonomy.
    #[error(transparent)]
    Domain(#[from] beardog_errors::BearDogError),
}

impl From<String> for HandlerError {
    fn from(s: String) -> Self {
        Self::Application(s)
    }
}

impl From<&str> for HandlerError {
    fn from(s: &str) -> Self {
        Self::Application(s.to_owned())
    }
}

impl HandlerError {
    /// Check if the error message contains a substring (test convenience).
    #[must_use]
    pub fn contains(&self, pat: &str) -> bool {
        self.to_string().contains(pat)
    }

    /// Check if the error message is empty (test convenience).
    #[must_use]
    pub fn is_empty(&self) -> bool {
        self.to_string().is_empty()
    }
}

impl HandlerError {
    /// Map to a JSON-RPC error code.
    #[must_use]
    pub const fn json_rpc_code(&self) -> i32 {
        match self {
            Self::MethodNotFound(_) => -32601,
            Self::InvalidParams(_) => -32602,
            Self::Application(_) | Self::Domain(_) => -32000,
        }
    }

    /// Map to a JSON-RPC error response.
    ///
    /// For [`Application`](Self::Application) errors, heuristically detects
    /// missing/invalid parameter messages from legacy `String`-returning
    /// handlers and maps them to `-32602` (Invalid Params) for backward
    /// compatibility with existing callers.
    #[must_use]
    pub fn into_json_rpc_error(self) -> crate::unix_socket_ipc::types::JsonRpcError {
        use crate::unix_socket_ipc::types::JsonRpcError;
        match self {
            Self::MethodNotFound(msg) => JsonRpcError::method_not_found(msg),
            Self::InvalidParams(msg) => JsonRpcError::invalid_params(msg),
            Self::Application(ref msg)
                if msg.contains("Missing")
                    || msg.contains("Invalid params")
                    || msg.contains("required parameter")
                    || msg.contains("Missing params") =>
            {
                let Self::Application(msg) = self else {
                    unreachable!()
                };
                JsonRpcError::invalid_params(msg)
            }
            Self::Application(msg) => {
                tracing::error!(raw = %msg, "handler application error");
                JsonRpcError::internal_error(sanitize_error_message(&msg))
            }
            Self::Domain(err) => {
                tracing::error!(raw = %err, "handler domain error");
                JsonRpcError::internal_error(sanitize_error_message(&err.to_string()))
            }
        }
    }

    /// Map to a dispatch error phase.
    #[must_use]
    pub const fn error_phase(&self) -> IpcErrorPhase {
        match self {
            Self::MethodNotFound(_) | Self::InvalidParams(_) => IpcErrorPhase::Dispatch,
            Self::Application(_) | Self::Domain(_) => IpcErrorPhase::Application,
        }
    }
}

/// Convenience alias for handler return types.
pub type HandlerResult = Result<serde_json::Value, HandlerError>;

// Dark Forest Beacon Genetics (Phase 1 - Feb 2026)
pub mod beacon;

// Shared utilities (self-knowledge pattern, etc.)
pub mod utils;

// Existing handlers
pub mod btsp;
pub mod capabilities;
pub mod capability_call; // capability.call routing dispatcher (federation bridge)
pub mod crypto; // Refactored crypto handlers module (domain-based organization)
pub mod crypto_handler; // Crypto RPC handler (routes to crypto module)
pub mod encryption;
pub mod federation;
pub mod fido2;
pub mod graph_security;
pub mod health;
pub mod introspection; // Primal introspection (primal.info, rpc.methods)
pub mod ionic_bond;
pub mod primal_signing; // Unified primal Ed25519 identity (shared by announcements + ionic bonds)
pub mod relay; // Relay authorization (lineage-gated, for coordinated punch)
pub mod secrets; // Encrypted secret storage (family-scoped, ChaCha20-Poly1305)
pub mod security;

/// Trait for JSON-RPC method handlers.
///
/// Implement this trait to create a new handler that can be registered
/// with the [`HandlerRegistry`]. Each handler is responsible for handling
/// one or more related JSON-RPC methods.
///
/// # Example
///
/// ```rust,ignore
/// use std::sync::Arc;
///
/// struct MyHandler;
///
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
///     ) -> HandlerResult {
///         match method {
///             "my.method1" => Ok(serde_json::json!({"result": "ok"})),
///             "my.method2" => Ok(serde_json::json!({"result": "ok2"})),
///             _ => Err(HandlerError::MethodNotFound(method.to_owned())),
///         }
///     }
/// }
/// ```
#[expect(
    async_fn_in_trait,
    reason = "JSON-RPC handlers are async; object-safe trait for registry"
)]
pub trait MethodHandler: Send + Sync {
    /// Get the methods this handler can handle.
    ///
    /// Returns a list of method names (including namespace) that this
    /// handler supports. The registry uses this for routing.
    fn methods(&self) -> Vec<&'static str>;

    /// Handle a JSON-RPC request.
    ///
    /// # Arguments
    /// * `method` - The JSON-RPC method name (e.g., "`crypto.sign_ed25519`")
    /// * `params` - Optional parameters for the method
    /// * `btsp_provider` - The BTSP provider for accessing capabilities
    ///
    /// # Errors
    ///
    /// Returns [`HandlerError`] on missing params, unknown method, or
    /// domain-specific failures.
    async fn handle(
        &self,
        method: &str,
        params: Option<&serde_json::Value>,
        btsp_provider: &Arc<BeardogBtspProvider>,
    ) -> HandlerResult;
}

/// Concrete enum of all [`MethodHandler`] implementations held by [`HandlerRegistry`].
#[expect(
    missing_docs,
    reason = "enum variants mirror handler modules; names are self-explanatory"
)]
pub enum MethodHandlerKind {
    Health(health::HealthHandler),
    Security(security::SecurityHandler),
    Btsp(btsp::BtspHandler),
    IonicBond(ionic_bond::IonicBondHandler),
    Crypto(crypto_handler::CryptoHandler),
    Federation(federation::FederationHandler),
    Encryption(encryption::EncryptionHandler),
    GraphSecurity(graph_security::GraphSecurityHandler),
    Beacon(beacon::BeaconHandler),
    Secrets(secrets::SecretsHandler),
    Relay(relay::RelayHandler),
    Fido2(fido2::Fido2Handler),
    Capabilities(capabilities::CapabilitiesHandler),
    CapabilityCall(capability_call::CapabilityCallHandler),
    Introspection(introspection::IntrospectionHandler),
}

impl MethodHandler for MethodHandlerKind {
    fn methods(&self) -> Vec<&'static str> {
        match self {
            Self::Health(h) => h.methods(),
            Self::Security(h) => h.methods(),
            Self::Btsp(h) => h.methods(),
            Self::IonicBond(h) => h.methods(),
            Self::Crypto(h) => h.methods(),
            Self::Federation(h) => h.methods(),
            Self::Encryption(h) => h.methods(),
            Self::GraphSecurity(h) => h.methods(),
            Self::Beacon(h) => h.methods(),
            Self::Secrets(h) => h.methods(),
            Self::Relay(h) => h.methods(),
            Self::Fido2(h) => h.methods(),
            Self::Capabilities(h) => h.methods(),
            Self::CapabilityCall(h) => h.methods(),
            Self::Introspection(h) => h.methods(),
        }
    }

    async fn handle(
        &self,
        method: &str,
        params: Option<&serde_json::Value>,
        btsp_provider: &Arc<BeardogBtspProvider>,
    ) -> HandlerResult {
        match self {
            Self::Health(h) => h.handle(method, params, btsp_provider).await,
            Self::Security(h) => h.handle(method, params, btsp_provider).await,
            Self::Btsp(h) => h.handle(method, params, btsp_provider).await,
            Self::IonicBond(h) => h.handle(method, params, btsp_provider).await,
            Self::Crypto(h) => h.handle(method, params, btsp_provider).await,
            Self::Federation(h) => h.handle(method, params, btsp_provider).await,
            Self::Encryption(h) => h.handle(method, params, btsp_provider).await,
            Self::GraphSecurity(h) => h.handle(method, params, btsp_provider).await,
            Self::Beacon(h) => h.handle(method, params, btsp_provider).await,
            Self::Secrets(h) => h.handle(method, params, btsp_provider).await,
            Self::Relay(h) => h.handle(method, params, btsp_provider).await,
            Self::Fido2(h) => h.handle(method, params, btsp_provider).await,
            Self::Capabilities(h) => h.handle(method, params, btsp_provider).await,
            Self::CapabilityCall(h) => h.handle(method, params, btsp_provider).await,
            Self::Introspection(h) => h.handle(method, params, btsp_provider).await,
        }
    }
}

/// Registry of all JSON-RPC method handlers
///
/// The registry maintains a list of all registered handlers and routes
/// incoming requests to the appropriate handler based on the method name.
///
/// # Architecture
///
/// The registry stores [`MethodHandlerKind`] variants so all handlers share one
/// concrete collection type while keeping static dispatch per handler.
///
/// # Thread Safety
///
/// The registry is `Send + Sync` and can be safely shared across threads.
/// All handlers must also be `Send + Sync`.
pub struct HandlerRegistry {
    handlers: tokio::sync::RwLock<Vec<MethodHandlerKind>>,
    /// O(1) method name → handler index lookup, built once after all handlers
    /// are registered. Falls back to linear scan if unset (should never happen).
    method_map: std::sync::OnceLock<HashMap<&'static str, usize>>,
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
    #[must_use]
    pub fn new(identity: Arc<beardog_types::primal_identity::PrimalIdentity>) -> Arc<Self> {
        let client = OrchestratorRegistryClient::new();
        let persistence = ionic_bond::CapabilityDiscoveryBondPersistence::new(client);
        Self::with_bond_persistence(
            identity,
            Arc::new(ionic_bond::BondPersistenceBackend::CapabilityDiscovery(
                persistence,
            )),
        )
    }

    /// Create a registry with a custom bond persistence backend.
    ///
    /// Production NUCLEUS deployments should pass a
    /// [`CapabilityDiscoveryBondPersistence`](ionic_bond::CapabilityDiscoveryBondPersistence)
    /// to persist bonds via discovered `bonding.ledger.*` RPCs.
    pub fn with_bond_persistence(
        identity: Arc<beardog_types::primal_identity::PrimalIdentity>,
        bond_persistence: Arc<ionic_bond::BondPersistenceBackend>,
    ) -> Arc<Self> {
        // Two-phase construction: handlers that need a back-reference to the
        // registry (CapabilitiesHandler, IntrospectionHandler) are added in Phase 2.

        // Phase 1: handlers that do NOT need registry access
        let registry = Arc::new(Self {
            handlers: tokio::sync::RwLock::new(vec![
                MethodHandlerKind::Health(health::HealthHandler::new()),
                MethodHandlerKind::Security(security::SecurityHandler::new(identity.clone())),
                MethodHandlerKind::Btsp(btsp::BtspHandler::new()),
                MethodHandlerKind::IonicBond(ionic_bond::IonicBondHandler::with_persistence(
                    bond_persistence,
                )),
                MethodHandlerKind::Crypto(crypto_handler::CryptoHandler),
                MethodHandlerKind::Federation(federation::FederationHandler::new(identity.clone())),
                MethodHandlerKind::Encryption(encryption::EncryptionHandler),
                MethodHandlerKind::GraphSecurity(graph_security::GraphSecurityHandler),
                MethodHandlerKind::Beacon(beacon::BeaconHandler::new()),
                MethodHandlerKind::Secrets(secrets::SecretsHandler::new(
                    identity.clone(),
                    Arc::new(crate::credential_store::CredentialStoreBackend::platform_default()),
                )),
                MethodHandlerKind::Relay(relay::RelayHandler::new(identity.clone())),
                MethodHandlerKind::Fido2(fido2::Fido2Handler::new()),
            ]),
            method_map: std::sync::OnceLock::new(),
        });

        // Phase 2: handlers that need registry access for method enumeration.
        // Creates intentional Arc cycles — these handlers call registry.all_methods().
        let capabilities_handler =
            capabilities::CapabilitiesHandler::new(identity, registry.clone());
        let capability_call_handler = capability_call::CapabilityCallHandler::new(registry.clone());
        let introspection = introspection::IntrospectionHandler::new(registry.clone());

        match registry.handlers.try_write() {
            Ok(mut handlers) => {
                handlers.push(MethodHandlerKind::Capabilities(capabilities_handler));
                handlers.push(MethodHandlerKind::CapabilityCall(capability_call_handler));
                handlers.push(MethodHandlerKind::Introspection(introspection));
            }
            Err(_) => {
                tracing::error!(
                    "UNEXPECTED: Failed to acquire write lock during HandlerRegistry initialization. \
                     Capabilities and introspection handlers will not be available. This is a bug."
                );
            }
        }

        // Phase 3: build O(1) method→handler dispatch index.
        if let Ok(handlers) = registry.handlers.try_read() {
            let mut map = HashMap::with_capacity(128);
            for (idx, handler) in handlers.iter().enumerate() {
                for method in handler.methods() {
                    map.insert(method, idx);
                }
            }
            let _ = registry.method_map.set(map);
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
    /// O(1) via pre-built method→handler `HashMap` (constructed once at init).
    /// Falls back to O(n) linear scan only if the map was never built (should
    /// not happen in normal operation).
    /// # Errors
    ///
    /// Returns [`HandlerError`] if the method is unknown or the handler fails.
    pub async fn route(
        &self,
        method: &str,
        params: Option<&serde_json::Value>,
        btsp_provider: &Arc<BeardogBtspProvider>,
    ) -> HandlerResult {
        // Backward-compat bridge: bare crypto names → namespaced equivalents,
        // plus bonding.* aliases used by primalSpring graphs and dispatch.
        let method = match method {
            "x25519_generate_ephemeral" => "crypto.x25519_generate_ephemeral",
            "x25519_derive_secret" => "crypto.x25519_derive_secret",
            "sign_ed25519" => "crypto.sign_ed25519",
            "verify_ed25519" => "crypto.verify_ed25519",
            "chacha20_poly1305_encrypt" => "crypto.chacha20_poly1305_encrypt",
            "chacha20_poly1305_decrypt" => "crypto.chacha20_poly1305_decrypt",
            "hmac_sha256" => "crypto.hmac_sha256",
            "blake3_hash" => "crypto.blake3_hash",
            "bonding.propose" => "crypto.ionic_bond.propose",
            "bonding.accept" => "crypto.ionic_bond.accept",
            "bonding.status" => "crypto.ionic_bond.list",
            "bonding.terminate" => "crypto.ionic_bond.revoke",
            "bonding.modify_scope" => "crypto.ionic_bond.seal",
            other => other,
        };

        let handlers = self.handlers.read().await;

        // O(1) dispatch via pre-built index
        if let Some(map) = self.method_map.get() {
            if let Some(&idx) = map.get(method) {
                return handlers[idx].handle(method, params, btsp_provider).await;
            }
            return Err(HandlerError::MethodNotFound(method.to_owned()));
        }

        // Fallback: linear scan (only if method_map was never built)
        for handler in handlers.iter() {
            if handler.methods().contains(&method) {
                return handler.handle(method, params, btsp_provider).await;
            }
        }
        Err(HandlerError::MethodNotFound(method.to_owned()))
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
        match self.route(method, params, btsp_provider).await {
            Ok(v) => DispatchOutcome::Success(v),
            Err(ref e) => DispatchOutcome::Failure {
                phase: e.error_phase(),
                code: i64::from(e.json_rpc_code()),
                message: e.to_string(),
            },
        }
    }

    /// Get all methods from all handlers
    ///
    /// Returns a sorted list of all method names exposed by all handlers.
    /// Used by introspection handler for `rpc.methods`.
    pub async fn all_methods(&self) -> Vec<&'static str> {
        let handlers = self.handlers.read().await;
        let mut methods = Vec::with_capacity(128);
        for handler in handlers.iter() {
            methods.extend(handler.methods());
        }
        methods.sort_unstable();
        methods.dedup();
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

    struct TestHandler;

    impl MethodHandler for TestHandler {
        fn methods(&self) -> Vec<&'static str> {
            vec!["test.method"]
        }

        async fn handle(
            &self,
            method: &str,
            _params: Option<&serde_json::Value>,
            _btsp_provider: &Arc<BeardogBtspProvider>,
        ) -> HandlerResult {
            match method {
                "test.method" => Ok(serde_json::json!({"test": "ok"})),
                _ => Err(HandlerError::MethodNotFound(method.to_owned())),
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
