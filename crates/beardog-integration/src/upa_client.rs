//! # UPA Registration Client
//!
//! Modern async client for Songbird's Universal Port Authority (UPA).
//!
//! ## Features
//! - Connection pooling with keep-alive
//! - Automatic retry with exponential backoff
//! - Graceful error handling
//! - Lock-free atomic state management
//!
//! ## Concurrency Patterns
//! - Uses `Arc<RwLock>` for shared mutable state
//! - Connection pooling via `reqwest::Client`
//! - Tokio async/await throughout
//! - No blocking operations

use std::sync::Arc;
use std::time::Duration;

use arc_swap::ArcSwap;
use reqwest::Client;
use serde::{Deserialize, Serialize};
use tracing::{debug, error, info, warn};

use beardog_errors::BearDogError;

/// UPA registration request payload
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct RegistrationRequest {
    /// Service name (e.g., "beardog-security-provider")
    pub service_name: String,
    /// Service version
    pub version: String,
    /// List of capabilities this service provides
    pub capabilities: Vec<String>,
    /// Service endpoint URL
    pub endpoint: String,
    /// Additional metadata
    #[serde(skip_serializing_if = "Option::is_none")]
    pub metadata: Option<serde_json::Value>,
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
    /// Suggested heartbeat interval (seconds)
    pub heartbeat_interval: u64,
}

/// UPA service discovery query
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct DiscoveryQuery {
    /// Required capabilities (AND logic)
    pub capabilities: Vec<String>,
    /// Optional: minimum version requirement
    #[serde(skip_serializing_if = "Option::is_none")]
    pub min_version: Option<String>,
}

/// Discovered service information
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ServiceInfo {
    /// Service ID
    pub service_id: String,
    /// Service name
    pub service_name: String,
    /// Service endpoint
    pub endpoint: String,
    /// Capabilities provided
    pub capabilities: Vec<String>,
    /// Service status
    pub status: ServiceStatus,
    /// Last heartbeat timestamp
    pub last_heartbeat: String,
}

/// Service status in UPA
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "lowercase")]
pub enum ServiceStatus {
    /// Service is active and responding
    Active,
    /// Service missed recent heartbeats
    Stale,
    /// Service explicitly deregistered
    Inactive,
}

/// UPA client configuration
#[derive(Debug, Clone)]
pub struct UpaClientConfig {
    /// UPA base URL
    pub upa_url: String,
    /// Request timeout
    pub timeout: Duration,
    /// Maximum retry attempts
    pub max_retries: u32,
    /// Initial retry backoff
    pub retry_backoff: Duration,
}

impl Default for UpaClientConfig {
    fn default() -> Self {
        Self {
            upa_url: std::env::var("BEARDOG_UPA_URL")
                .unwrap_or_else(|_| "https://localhost:8080".to_string()),
            timeout: Duration::from_secs(10),
            max_retries: 3,
            retry_backoff: Duration::from_millis(500),
        }
    }
}

/// Modern async UPA client with connection pooling
///
/// ## Concurrency
/// - Thread-safe via `Arc<Self>`
/// - Lock-free reads for most operations
/// - Uses `ArcSwap` for atomic state updates
/// - Connection pool managed by `reqwest::Client`
pub struct UpaClient {
    /// HTTP client with connection pooling
    client: Client,
    /// UPA base URL
    upa_url: String,
    /// Configuration
    config: UpaClientConfig,
    /// Current registration state (lock-free reads)
    registration: ArcSwap<Option<RegistrationResponse>>,
}

impl UpaClient {
    /// Create a new UPA client
    ///
    /// ## Modern Patterns
    /// - Uses connection pooling (HTTP/2 multiplexing)
    /// - Configures keep-alive
    /// - Sets reasonable timeouts
    pub fn new(config: UpaClientConfig) -> Result<Self, BearDogError> {
        let client = Client::builder()
            .timeout(config.timeout)
            .pool_max_idle_per_host(10)
            .http2_keep_alive_interval(Some(Duration::from_secs(30)))
            .http2_keep_alive_while_idle(true)
            .build()
            .map_err(|e| BearDogError::network(format!("Failed to build HTTP client: {}", e)))?;

        Ok(Self {
            client,
            upa_url: config.upa_url.clone(),
            config,
            registration: ArcSwap::from_pointee(None),
        })
    }

    /// Register service with UPA
    ///
    /// ## Idempotent
    /// Safe to call multiple times - UPA will update existing registration
    ///
    /// ## Retry Logic
    /// Automatically retries with exponential backoff on transient failures
    pub async fn register(
        &self,
        request: RegistrationRequest,
    ) -> Result<RegistrationResponse, BearDogError> {
        info!(
            service_name = %request.service_name,
            capabilities = ?request.capabilities,
            "📝 Registering with UPA"
        );

        let url = format!("{}/api/v1/services/register", self.upa_url);
        let mut attempts = 0;
        let mut backoff = self.config.retry_backoff;

        loop {
            attempts += 1;

            match self.client.post(&url).json(&request).send().await {
                Ok(response) => {
                    if response.status().is_success() {
                        match response.json::<RegistrationResponse>().await {
                            Ok(reg) => {
                                info!(
                                    service_id = %reg.service_id,
                                    heartbeat_interval = reg.heartbeat_interval,
                                    "✅ Successfully registered with UPA"
                                );

                                // Atomic update of registration state (lock-free!)
                                self.registration.store(Arc::new(Some(reg.clone())));

                                return Ok(reg);
                            }
                            Err(e) => {
                                error!(error = %e, "Failed to parse registration response");
                                return Err(BearDogError::network(format!(
                                    "Invalid response from UPA: {}",
                                    e
                                )));
                            }
                        }
                    } else {
                        let status = response.status();
                        let body = response.text().await.unwrap_or_else(|_| "".to_string());

                        error!(status = %status, body = %body, "UPA registration failed");

                        // Don't retry on client errors (4xx)
                        if status.is_client_error() {
                            return Err(BearDogError::network(format!(
                                "UPA rejected registration: {} - {}",
                                status, body
                            )));
                        }
                    }
                }
                Err(e) => {
                    warn!(
                        attempt = attempts,
                        max_retries = self.config.max_retries,
                        error = %e,
                        "UPA registration attempt failed"
                    );
                }
            }

            // Retry logic with exponential backoff
            if attempts >= self.config.max_retries {
                error!("Max retry attempts reached for UPA registration");
                return Err(BearDogError::network(format!(
                    "Failed to register with UPA after {} attempts",
                    attempts
                )));
            }

            debug!(backoff_ms = ?backoff, "Retrying UPA registration...");
            tokio::time::sleep(backoff).await;
            backoff *= 2; // Exponential backoff
        }
    }

    /// Send heartbeat to UPA
    ///
    /// ## Non-blocking
    /// Returns immediately even if UPA is unreachable
    /// Logs warnings but doesn't fail the service
    pub async fn send_heartbeat(
        &self,
        service_id: &str,
        token: &str,
        metrics: LoadMetrics,
    ) -> Result<(), BearDogError> {
        debug!(service_id = %service_id, "💓 Sending heartbeat to UPA");

        let url = format!("{}/api/v1/services/{}/heartbeat", self.upa_url, service_id);

        let response = self
            .client
            .post(&url)
            .bearer_auth(token)
            .json(&metrics)
            .send()
            .await;

        match response {
            Ok(resp) if resp.status().is_success() => {
                debug!("✅ Heartbeat acknowledged by UPA");
                Ok(())
            }
            Ok(resp) => {
                let status = resp.status();
                warn!(status = %status, "UPA heartbeat returned non-success status");
                Err(BearDogError::network(format!(
                    "Heartbeat failed with status: {}",
                    status
                )))
            }
            Err(e) => {
                // Don't fail the service, just log
                warn!(error = %e, "Failed to send heartbeat (UPA may be unavailable)");
                Err(BearDogError::network(format!("Heartbeat error: {}", e)))
            }
        }
    }

    /// Discover services by capabilities
    ///
    /// ## Concurrent
    /// Multiple discovery queries can run in parallel safely
    pub async fn discover_services(
        &self,
        query: DiscoveryQuery,
    ) -> Result<Vec<ServiceInfo>, BearDogError> {
        debug!(capabilities = ?query.capabilities, "🔍 Discovering services");

        let url = format!("{}/api/v1/services/discover", self.upa_url);

        let response = self
            .client
            .post(&url)
            .json(&query)
            .send()
            .await
            .map_err(|e| BearDogError::network(format!("Discovery request failed: {}", e)))?;

        if !response.status().is_success() {
            return Err(BearDogError::network(format!(
                "Discovery failed with status: {}",
                response.status()
            )));
        }

        let services = response
            .json::<Vec<ServiceInfo>>()
            .await
            .map_err(|e| BearDogError::network(format!("Invalid discovery response: {}", e)))?;

        info!(
            count = services.len(),
            "✅ Discovered {} service(s)",
            services.len()
        );

        Ok(services)
    }

    /// Deregister from UPA
    ///
    /// ## Graceful Shutdown
    /// Should be called on service shutdown for clean deregistration
    pub async fn deregister(&self, service_id: &str, token: &str) -> Result<(), BearDogError> {
        info!(service_id = %service_id, "👋 Deregistering from UPA");

        let url = format!("{}/api/v1/services/{}", self.upa_url, service_id);

        let response = self
            .client
            .delete(&url)
            .bearer_auth(token)
            .send()
            .await
            .map_err(|e| BearDogError::network(format!("Deregistration failed: {}", e)))?;

        if response.status().is_success() {
            info!("✅ Successfully deregistered from UPA");

            // Clear registration state
            self.registration.store(Arc::new(None));

            Ok(())
        } else {
            warn!(status = %response.status(), "Deregistration returned non-success status");
            Err(BearDogError::network(format!(
                "Deregistration failed with status: {}",
                response.status()
            )))
        }
    }

    /// Get current registration (lock-free read)
    pub fn get_registration(&self) -> Option<RegistrationResponse> {
        self.registration.load().as_ref().clone()
    }
}

/// Load metrics for heartbeat
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct LoadMetrics {
    /// CPU usage percentage (0-100)
    pub cpu_percent: f32,
    /// Memory usage percentage (0-100)
    pub memory_percent: f32,
    /// Number of active connections
    pub active_connections: u32,
}

#[cfg(test)]
mod tests {
    use super::*;

    #[tokio::test]
    async fn test_client_creation() {
        let config = UpaClientConfig::default();
        let client = UpaClient::new(config);
        assert!(client.is_ok());
    }

    #[test]
    fn test_registration_request_serialization() {
        let req = RegistrationRequest {
            service_name: "test-service".to_string(),
            version: "1.0.0".to_string(),
            capabilities: vec!["security".to_string(), "btsp".to_string()],
            endpoint: "http://localhost:9000".to_string(),
            metadata: None,
        };

        let json = serde_json::to_string(&req).unwrap();
        assert!(json.contains("test-service"));
    }

    #[test]
    fn test_service_status() {
        let status = ServiceStatus::Active;
        let json = serde_json::to_string(&status).unwrap();
        assert_eq!(json, "\"active\"");
    }
}
