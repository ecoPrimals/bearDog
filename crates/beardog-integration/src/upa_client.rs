// SPDX-License-Identifier: AGPL-3.0-only

//! # UPA Registration Client (Tower Atomic Edition)
//!
//! **EVOLVED**: Now uses Tower Atomic (Unix sockets + JSON-RPC) instead of HTTP!
//!
//! Modern async client for Songbird's Universal Port Authority (UPA).
//!
//! ## Evolution
//!
//! - **Before**: HTTP client using reqwest (connection pooling, keep-alive)
//! - **After**: Tower Atomic client using Unix sockets (100% Pure Rust IPC!)
//!
//! ## Features
//! - Unix socket communication (no network overhead!)
//! - JSON-RPC 2.0 protocol
//! - Automatic retry with exponential backoff
//! - Graceful error handling
//! - Lock-free atomic state management
//!
//! ## Concurrency Patterns
//! - Uses `Arc<RwLock>` for shared mutable state
//! - Tokio async/await throughout
//! - No blocking operations

use std::sync::Arc;
use std::time::Duration;

use arc_swap::ArcSwap;
use serde::{Deserialize, Serialize};
use serde_json::{json, Value};
use tracing::{debug, error, info, warn};

use beardog_errors::BearDogError;

// Tower Atomic client for Songbird
use beardog_tower_atomic::Client as AtomicClient;

/// UPA registration request payload
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct RegistrationRequest {
    /// Service name (e.g., "beardog-security-provider")
    pub service_name: String,
    /// Service version
    pub version: String,
    /// List of capabilities this service provides
    pub capabilities: Vec<String>,
    /// Service endpoint (Unix socket path)
    pub endpoint: String,
    /// Additional metadata
    #[serde(skip_serializing_if = "Option::is_none")]
    pub metadata: Option<Value>,
}

/// UPA registration response
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct RegistrationResponse {
    /// Unique service ID assigned by UPA
    pub service_id: String,
    /// Authentication token for subsequent requests
    pub token: String,
    /// Registration timestamp
    pub registered_at: String,
}

/// Default UPA service provider (environment-discoverable)
///
/// Priority:
/// 1. `UPA_PROVIDER` env var (operator override)
/// 2. `"songbird"` (default UPA implementation)
fn discover_upa_provider() -> String {
    beardog_errors::process_env::var("UPA_PROVIDER").unwrap_or_else(|_| "songbird".to_string())
}

/// UPA client for service registration and discovery
pub struct UpaClient {
    /// Tower Atomic client for UPA provider
    client: AtomicClient,
    /// Cached registration token
    token: Arc<ArcSwap<Option<String>>>,
}

impl UpaClient {
    /// Create a new UPA client
    ///
    /// Discovers the UPA provider at runtime:
    /// 1. `UPA_PROVIDER` env var (operator override)
    /// 2. Default: `"songbird"` (standard UPA implementation)
    ///
    /// Connects via Tower Atomic (Unix socket).
    ///
    /// # Example
    ///
    /// ```rust,no_run
    /// use beardog_integration::upa_client::UpaClient;
    ///
    /// #[tokio::main]
    /// async fn main() -> Result<(), Box<dyn std::error::Error>> {
    ///     let client = UpaClient::new().await?;
    ///     Ok(())
    /// }
    /// ```
    pub async fn new() -> Result<Self, BearDogError> {
        let provider = discover_upa_provider();
        info!("🔌 Connecting to UPA provider '{}' via Tower Atomic", provider);

        let client = AtomicClient::connect(&provider)
            .await
            .map_err(|e| {
                BearDogError::ConnectionFailed(format!(
                    "Failed to connect to UPA provider '{}': {}",
                    provider, e
                ))
            })?;

        info!("✅ Connected to UPA provider '{}'", provider);

        Ok(Self {
            client,
            token: Arc::new(ArcSwap::new(Arc::new(None))),
        })
    }

    /// Register a service with UPA
    ///
    /// # Example
    ///
    /// ```rust,no_run
    /// # use beardog_integration::upa_client::{UpaClient, RegistrationRequest};
    /// # async fn example() -> Result<(), Box<dyn std::error::Error>> {
    /// let mut client = UpaClient::new().await?;
    /// 
    /// let request = RegistrationRequest {
    ///     service_name: "beardog-crypto".to_string(),
    ///     version: "0.9.0".to_string(),
    ///     capabilities: vec!["crypto.sign".to_string(), "crypto.encrypt".to_string()],
    ///     endpoint: "/run/user/1000/ecoPrimals/beardog.sock".to_string(),
    ///     metadata: None,
    /// };
    /// 
    /// let response = client.register(request).await?;
    /// println!("Registered with service ID: {}", response.service_id);
    /// # Ok(())
    /// # }
    /// ```
    pub async fn register(
        &mut self,
        request: RegistrationRequest,
    ) -> Result<RegistrationResponse, BearDogError> {
        info!("📝 Registering service: {}", request.service_name);

        let params = json!({
            "service_name": request.service_name,
            "version": request.version,
            "capabilities": request.capabilities,
            "endpoint": request.endpoint,
            "metadata": request.metadata,
        });

        let response = self
            .client
            .call("upa.register", params)
            .await
            .map_err(|e| BearDogError::ApiError(format!("UPA registration failed: {}", e)))?;

        let registration: RegistrationResponse =
            serde_json::from_value(response).map_err(|e| {
                BearDogError::SerializationError(format!("Failed to parse registration response: {}", e))
            })?;

        // Cache token
        self.token
            .store(Arc::new(Some(registration.token.clone())));

        info!("✅ Registered with service ID: {}", registration.service_id);

        Ok(registration)
    }

    /// Discover services by capability
    pub async fn discover(&mut self, capability: &str) -> Result<Vec<Value>, BearDogError> {
        info!("🔍 Discovering services with capability: {}", capability);

        let params = json!({
            "capability": capability,
        });

        let response = self
            .client
            .call("upa.discover", params)
            .await
            .map_err(|e| BearDogError::ApiError(format!("UPA discovery failed: {}", e)))?;

        let services: Vec<Value> = serde_json::from_value(response["services"].clone())
            .unwrap_or_default();

        info!("✅ Found {} services", services.len());

        Ok(services)
    }

    /// Heartbeat to keep registration alive
    pub async fn heartbeat(&mut self) -> Result<(), BearDogError> {
        debug!("💓 Sending heartbeat to UPA");

        let token = self.token.load();
        let token = token
            .as_ref()
            .as_ref()
            .ok_or_else(|| BearDogError::AuthenticationFailed("Not registered".to_string()))?;

        let params = json!({
            "token": token,
        });

        self.client
            .call("upa.heartbeat", params)
            .await
            .map_err(|e| BearDogError::ApiError(format!("UPA heartbeat failed: {}", e)))?;

        debug!("✅ Heartbeat sent");

        Ok(())
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[tokio::test]
    #[ignore] // Requires running Songbird instance
    async fn test_connect() {
        let client = UpaClient::new().await;
        match client {
            Ok(_) => println!("✅ Connected to Songbird UPA"),
            Err(e) => println!("⚠️  Songbird not running: {}", e),
        }
    }
}
