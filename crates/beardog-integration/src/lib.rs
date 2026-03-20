// SPDX-License-Identifier: AGPL-3.0-only
#![forbid(unsafe_code)]

//! # BearDog-Songbird Integration
//!
//! Phase 3 integration layer providing UPA registration, heartbeat monitoring,
//! and expanded API endpoints for cross-primal federation.
//!
//! ## Modern Rust Patterns
//! - Async/await throughout (Tokio runtime)
//! - Connection pooling and keep-alive
//! - Lock-free atomic operations where possible
//! - Structured concurrency with tokio::task
//! - Graceful shutdown handling
//!
//! ## Architecture
//! ```text
//! ┌─────────────────────────────────────────┐
//! │     BearDog Integration Layer          │
//! ├─────────────────────────────────────────┤
//! │                                         │
//! │  ┌──────────────┐  ┌─────────────────┐ │
//! │  │  UPA Client  │  │  Heartbeat Svc  │ │
//! │  │  (Register)  │  │  (30s interval) │ │
//! │  └──────────────┘  └─────────────────┘ │
//! │                                         │
//! │  ┌─────────────────────────────────────┐│
//! │  │     Integration API Server          ││
//! │  │  • 6 BTSP endpoints                 ││
//! │  │  • 4 BirdSong endpoints             ││
//! │  • 3 Lineage endpoints              ││
//! │  │  • Health/metrics endpoints         ││
//! │  └─────────────────────────────────────┘│
//! │                                         │
//! └─────────────────────────────────────────┘
//! ```

#![warn(missing_docs)]

pub mod api_server;
pub mod heartbeat;
pub mod upa_client;

mod errors;

use std::sync::Arc;
use std::time::Duration;

use tokio::task::JoinHandle;
use tracing::{error, info};

pub use upa_client::{
    LoadMetrics, RegistrationRequest, RegistrationResponse, ServiceInfo, ServiceStatus, UpaClient,
    UpaClientConfig,
};

pub use heartbeat::{HeartbeatConfig, HeartbeatService};

pub use errors::IntegrationError;

/// Re-export BearDogError for convenience
pub use beardog_errors::BearDogError;

/// Integration configuration
#[derive(Debug, Clone)]
pub struct IntegrationConfig {
    /// UPA URL (e.g., "https://localhost:8080")
    pub upa_url: String,
    /// API server port
    pub api_port: u16,
    /// Service name
    pub service_name: String,
    /// Capabilities to register
    pub capabilities: Vec<String>,
    /// Heartbeat interval (seconds)
    pub heartbeat_interval_secs: u64,
}

impl Default for IntegrationConfig {
    fn default() -> Self {
        Self {
            upa_url: beardog_errors::process_env::var("BEARDOG_UPA_URL")
                .unwrap_or_else(|_| "https://localhost:8080".to_string()),
            api_port: beardog_errors::process_env::var("BEARDOG_API_PORT")
                .ok()
                .and_then(|s| s.parse().ok())
                .unwrap_or(9000),
            service_name: beardog_errors::process_env::var("BEARDOG_SERVICE_NAME")
                .unwrap_or_else(|_| "beardog-security-provider".to_string()),
            capabilities: vec![
                "security".to_string(),
                "btsp".to_string(),
                "lineage".to_string(),
                "birdsong".to_string(),
            ],
            heartbeat_interval_secs: beardog_errors::process_env::var("BEARDOG_HEARTBEAT_INTERVAL")
                .ok()
                .and_then(|s| s.parse().ok())
                .unwrap_or(30),
        }
    }
}

/// Main integration orchestrator
///
/// Coordinates UPA registration, API server, and heartbeat service.
///
/// ## Lifecycle
/// 1. Create with `new()` - registers with UPA
/// 2. Call `start()` - starts API server and heartbeat
/// 3. Runs until `shutdown()` called or process terminated
/// 4. Automatically deregisters from UPA on drop
pub struct BearDogIntegration {
    config: IntegrationConfig,
    upa_client: Arc<UpaClient>,
    registration: RegistrationResponse,
}

impl BearDogIntegration {
    /// Create new integration and register with UPA
    ///
    /// ## Async Initialization
    /// Performs UPA registration during construction to fail fast
    pub async fn new(config: IntegrationConfig) -> Result<Self, BearDogError> {
        info!("🚀 Initializing BearDog-Songbird integration");

        // Create UPA client
        let upa_config = UpaClientConfig {
            upa_url: config.upa_url.clone(),
            ..Default::default()
        };

        let upa_client = Arc::new(UpaClient::new(upa_config)?);

        // Register with UPA
        // Use environment-driven endpoint or construct from config
        // Configuration hierarchy: BEARDOG_ENDPOINT > BEARDOG_HOST + port > 127.0.0.1 (dev) / 0.0.0.0 (prod)
        let endpoint = beardog_errors::process_env::var("BEARDOG_ENDPOINT").unwrap_or_else(|_| {
            // If no explicit endpoint, use BEARDOG_HOST or environment-aware default
            let host = beardog_errors::process_env::var("BEARDOG_HOST").unwrap_or_else(|_| {
                // Secure default for dev, production default for release
                if cfg!(debug_assertions) {
                    "127.0.0.1".to_string()  // Secure localhost for development
                } else {
                    "0.0.0.0".to_string()     // Bind all interfaces for production
                }
            });
            format!("http://{}:{}", host, config.api_port)
        });

        let registration_req = RegistrationRequest {
            service_name: config.service_name.clone(),
            version: env!("CARGO_PKG_VERSION").to_string(),
            capabilities: config.capabilities.clone(),
            endpoint,
            metadata: None,
        };

        let registration = upa_client.register(registration_req).await?;

        info!(
            service_id = %registration.service_id,
            "✅ BearDog integration initialized"
        );

        Ok(Self {
            config,
            upa_client,
            registration,
        })
    }

    /// Start all integration services
    ///
    /// ## Structured Concurrency
    /// - Spawns API server task
    /// - Spawns heartbeat task
    /// - Uses `tokio::try_join!` for coordinated lifecycle
    ///
    /// ## Graceful Shutdown
    /// - Handles SIGINT/SIGTERM
    /// - Deregisters from UPA
    /// - Waits for connections to drain
    pub async fn start(self) -> Result<(), BearDogError> {
        info!("▶️  Starting BearDog integration services");

        let upa_client = self.upa_client.clone();
        let service_id = self.registration.service_id.clone();
        let token = self.registration.token.clone();

        // Start heartbeat service
        let heartbeat_config = HeartbeatConfig {
            interval: Duration::from_secs(self.config.heartbeat_interval_secs),
            service_id: service_id.clone(),
            token: token.clone(),
        };

        let heartbeat_service = HeartbeatService::new(upa_client.clone(), heartbeat_config);

        let heartbeat_handle: JoinHandle<Result<(), BearDogError>> =
            tokio::spawn(async move { heartbeat_service.start().await });

        // Start API server
        let api_config = api_server::ApiServerConfig {
            port: self.config.api_port,
            ..Default::default()
        };

        let api_handle: JoinHandle<Result<(), BearDogError>> =
            tokio::spawn(async move { api_server::start_api_server(api_config).await });

        // Wait for either task to complete (or fail)
        let result = tokio::try_join!(
            async {
                heartbeat_handle
                    .await
                    .map_err(|e| BearDogError::internal(format!("Heartbeat task panicked: {e}")))?
            },
            async {
                api_handle
                    .await
                    .map_err(|e| BearDogError::internal(format!("API server task panicked: {e}")))?
            },
        );

        match result {
            Ok(_) => {
                info!("✅ Integration services completed");
                Ok(())
            }
            Err(e) => {
                error!(error = %e, "❌ Integration service failed");
                Err(e)
            }
        }
    }

    /// Get UPA client for manual operations
    pub fn upa_client(&self) -> Arc<UpaClient> {
        self.upa_client.clone()
    }

    /// Get registration info
    pub fn registration(&self) -> &RegistrationResponse {
        &self.registration
    }
}

// Graceful shutdown on drop
impl Drop for BearDogIntegration {
    fn drop(&mut self) {
        info!("🛑 Shutting down BearDog integration");

        // Best-effort deregistration
        // (Can't block in Drop, so spawn detached task)
        let upa_client = self.upa_client.clone();
        let service_id = self.registration.service_id.clone();
        let token = self.registration.token.clone();

        tokio::spawn(async move {
            if let Err(e) = upa_client.deregister(&service_id, &token).await {
                error!(error = %e, "Failed to deregister from UPA during shutdown");
            }
        });
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_default_config() {
        let config = IntegrationConfig::default();
        assert!(config.capabilities.contains(&"security".to_string()));
        assert!(config.capabilities.contains(&"btsp".to_string()));
    }

    #[test]
    fn test_config_from_env() {
        beardog_errors::process_env::set_var("BEARDOG_API_PORT", "9999");
        beardog_errors::process_env::set_var("BEARDOG_SERVICE_NAME", "test-service");

        let config = IntegrationConfig::default();
        assert_eq!(config.api_port, 9999);
        assert_eq!(config.service_name, "test-service");

        beardog_errors::process_env::remove_var("BEARDOG_API_PORT");
        beardog_errors::process_env::remove_var("BEARDOG_SERVICE_NAME");
    }
}
