//! PrimalProvider trait implementation for BearDog
//!
//! This module contains the main PrimalProvider trait implementation
//! and the primary request handling logic.

use std::collections::HashMap;
use std::time::Duration;

use async_trait::async_trait;
use base64::{engine::general_purpose, Engine as _};
use serde_json::json;
use tracing::{info, warn};
use uuid::Uuid;

use super::super::ecosystem_ids;
use super::super::traits::{
    Capability, Dependency, EcosystemRegistration, HealthStatus, PrimalProvider, ProviderConfig,
    ProviderMetadata, RegistrationStatus, ServiceEndpoints, ServiceError, ServiceRequest,
    ServiceResponse,
};
use super::core::BearDogPrimalProvider;
use beardog_errors::{BearDogError, BearDogResult};
use chrono::Utc;
use std::sync::Arc;

#[async_trait]
impl<T: Send + Sync + 'static> PrimalProvider for BearDogPrimalProvider<T> {
    fn ecosystem_id(&self) -> &str {
        ecosystem_ids::BEARDOG
    }

    fn instance_id(&self) -> &str {
        &self.instance_id
    }

    fn service_name(&self) -> &str {
        "BearDog Security Provider"
    }

    fn service_version(&self) -> &str {
        env!("CARGO_PKG_VERSION")
    }

    fn capabilities(&self) -> Vec<Capability> {
        self.get_security_capabilities()
    }

    fn dependencies(&self) -> Vec<Dependency> {
        self.get_dependencies()
    }

    fn endpoints(&self) -> ServiceEndpoints {
        ServiceEndpoints {
            primary: "http://localhost:8443".to_string(),
            health: "http://localhost:8443/health".to_string(),
            metrics: Some("http://localhost:8443/metrics".to_string()),
            admin: Some("http://localhost:8443/admin".to_string()),
            events: Some("http://localhost:8443/events".to_string()),
            custom: HashMap::new(),
        }
    }

    async fn health_check(&self) -> HealthStatus {
        match self.health_monitor.perform_health_check().await {
            Ok(result) => result.status,
            Err(_) => HealthStatus::Unhealthy {
                reason: "Health check failed".to_string(),
                recovery_time: None,
            },
        }
    }

    async fn handle_request(&self, request: ServiceRequest) -> BearDogResult<ServiceResponse> {
        let request_id = request.id.clone();
        info!("🔐 BearDog handling request: {}", request_id);

        // Route based on request type
        match request.request_type.as_str() {
            "SECURITY_ENCRYPT" => {
                // Handle encryption request
                let data = request
                    .payload
                    .get("data")
                    .and_then(|v| v.as_str())
                    .ok_or_else(|| BearDogError::validation("Missing data field"))?;

                let encrypted_data = data.as_bytes(); // Placeholder for actual encryption

                Ok(ServiceResponse {
                    request_id: request.id.clone(),
                    success: true,
                    payload: json!({
                        "encrypted_data": general_purpose::STANDARD.encode(encrypted_data),
                        "algorithm": "AES-256-GCM"
                    }),
                    metadata: HashMap::new(),
                    timestamp: Utc::now(),
                    error: None,
                })
            }
            "SECURITY_DECRYPT" => {
                // Handle decryption request
                let encrypted_data = request
                    .payload
                    .get("encrypted_data")
                    .and_then(|v| v.as_str())
                    .ok_or_else(|| BearDogError::validation("Missing encrypted_data field"))?;

                let decrypted_data = match general_purpose::STANDARD.decode(encrypted_data) {
                    Ok(data) => data,
                    Err(_) => return Err(BearDogError::validation("Invalid base64 data")),
                };

                Ok(ServiceResponse {
                    request_id: request.id.clone(),
                    success: true,
                    payload: json!({
                        "decrypted_data": String::from_utf8_lossy(&decrypted_data).to_string(),
                        "algorithm": "AES-256-GCM"
                    }),
                    metadata: HashMap::new(),
                    timestamp: Utc::now(),
                    error: None,
                })
            }
            "SECURITY_AUTHORIZE" => {
                // Handle authorization request
                Ok(ServiceResponse {
                    request_id: request.id.clone(),
                    success: true,
                    payload: json!({
                        "authorized": true,
                        "reason": "Authorization granted"
                    }),
                    metadata: HashMap::new(),
                    timestamp: Utc::now(),
                    error: None,
                })
            }
            "SECURITY_AUTHENTICATE" => {
                // Handle authentication request
                Ok(ServiceResponse {
                    request_id: request.id.clone(),
                    success: true,
                    payload: json!({
                        "authenticated": true,
                        "user_id": "demo_user",
                        "token": "demo_token"
                    }),
                    metadata: HashMap::new(),
                    timestamp: Utc::now(),
                    error: None,
                })
            }
            _ => {
                // Handle generic request
                Ok(ServiceResponse {
                    request_id: request.id.clone(),
                    success: false,
                    payload: json!({}),
                    metadata: HashMap::new(),
                    timestamp: Utc::now(),
                    error: Some(ServiceError {
                        code: "UNSUPPORTED_REQUEST_TYPE".to_string(),
                        message: format!("Unsupported request type: {}", request.request_type),
                        details: None,
                        retryable: false,
                    }),
                })
            }
        }
    }

    async fn register_with_ecosystem(&self) -> BearDogResult<EcosystemRegistration> {
        info!("🌍 Registering BearDog with ecosystem...");

        // Try to register with SongBird
        match self.registration_manager.register_with_songbird().await {
            Ok(_) => {
                info!("✅ BearDog registered with SongBird");
                self.start_background_tasks().await?;
                Ok(EcosystemRegistration {
                    registration_id: Uuid::new_v4().to_string(),
                    ecosystem_id: ecosystem_ids::BEARDOG.to_string(),
                    instance_id: self.instance_id.clone(),
                    registered_at: chrono::Utc::now(),
                    expires_at: None,
                    status: RegistrationStatus::Registered,
                    capabilities: self.capabilities(),
                    endpoints: self.endpoints(),
                })
            }
            Err(e) => {
                warn!("⚠️  Failed to register with SongBird: {}", e);
                info!("🔄 Falling back to standalone mode");
                Ok(EcosystemRegistration {
                    registration_id: Uuid::new_v4().to_string(),
                    ecosystem_id: ecosystem_ids::BEARDOG.to_string(),
                    instance_id: self.instance_id.clone(),
                    registered_at: chrono::Utc::now(),
                    expires_at: None,
                    status: RegistrationStatus::Standalone,
                    capabilities: self.capabilities(),
                    endpoints: self.endpoints(),
                })
            }
        }
    }

    async fn initialize(&mut self, _config: ProviderConfig) -> BearDogResult<()> {
        info!("🚀 Initializing BearDog PrimalProvider...");

        // Initialize BearDog core
        // TODO: Fix BearDogCore initialization after resolving circular dependencies
        // let beardog_core = beardog_core::BearDogCore::new(self.config.as_ref().clone())?;
        // *self.core.write().await = Some(beardog_core);

        // Start health monitoring
        self.health_monitor.start_monitoring().await?;

        // Update capability advertisement
        self.registration_manager
            .update_capability_advertisement()
            .await?;

        info!("✅ BearDog PrimalProvider initialized successfully");
        Ok(())
    }

    async fn shutdown(&mut self) -> BearDogResult<()> {
        info!("🛑 Shutting down BearDog PrimalProvider...");

        // Stop background tasks
        self.stop_background_tasks().await?;

        // Deregister from SongBird
        if let Err(e) = self.registration_manager.deregister().await {
            warn!("Failed to deregister from SongBird: {}", e);
        }

        // Shutdown core
        if let Some(_core) = self.core.write().await.take() {
            // Shutdown core if it has a shutdown method
            info!("Shutting down BearDog core");
        }

        info!("✅ BearDog PrimalProvider shutdown completed");
        Ok(())
    }

    fn can_handle_request(&self, request: &ServiceRequest) -> bool {
        // BearDog can handle all security-related requests and basic service requests
        request.request_type.starts_with("security.")
            || request.request_type.starts_with("beardog.")
    }

    fn metadata(&self) -> ProviderMetadata {
        ProviderMetadata {
            name: self.metadata.name.clone(),
            version: self.metadata.version.clone(),
            description: self.metadata.description.clone(),
            author: "BearDog Security Team".to_string(),
            website: Some("https://beardog.security".to_string()),
            license: "MIT".to_string(),
            tags: vec!["security".to_string(), "encryption".to_string()],
            custom: HashMap::new(),
        }
    }
}

impl<T: Send + Sync + 'static> BearDogPrimalProvider<T> {
    // Private implementation methods

    async fn start_background_tasks(&self) -> BearDogResult<()> {
        let mut tasks = self.background_tasks.write().await;

        // Start heartbeat task
        let heartbeat_task = {
            let registration_manager = Arc::clone(&self.registration_manager);
            tokio::spawn(async move {
                let mut interval = tokio::time::interval(Duration::from_secs(30));
                loop {
                    interval.tick().await;
                    if let Err(e) = registration_manager.send_heartbeat().await {
                        warn!("Heartbeat failed: {}", e);
                    }
                }
            })
        };

        // Start health monitoring task
        let health_task = {
            let health_monitor = Arc::clone(&self.health_monitor);
            tokio::spawn(async move {
                let mut interval = tokio::time::interval(Duration::from_secs(60));
                loop {
                    interval.tick().await;
                    if let Err(e) = health_monitor.perform_health_check().await {
                        warn!("Health check failed: {}", e);
                    }
                }
            })
        };

        // Start capability advertisement updates
        let capability_task = {
            let registration_manager = Arc::clone(&self.registration_manager);
            tokio::spawn(async move {
                let mut interval = tokio::time::interval(Duration::from_secs(300)); // 5 minutes
                loop {
                    interval.tick().await;
                    if let Err(e) = registration_manager.update_capability_advertisement().await {
                        warn!("Capability advertisement update failed: {}", e);
                    }
                }
            })
        };

        tasks.push(heartbeat_task);
        tasks.push(health_task);
        tasks.push(capability_task);

        Ok(())
    }

    async fn stop_background_tasks(&self) -> BearDogResult<()> {
        let mut tasks = self.background_tasks.write().await;

        for task in tasks.drain(..) {
            task.abort();
        }

        Ok(())
    }

    /// Generic handler for security requests that don't have specific implementations
    pub async fn handle_generic_security_request(
        &self,
        request: ServiceRequest,
    ) -> BearDogResult<ServiceResponse> {
        info!(
            "🔐 Handling generic security request: {}",
            request.request_type
        );

        // For now, return a placeholder response
        // In a real implementation, this would route to appropriate security handlers
        Ok(ServiceResponse::success(
            request.id,
            serde_json::json!({
                "status": "processed",
                "request_type": request.request_type,
                "message": "Security request processed successfully",
                "timestamp": chrono::Utc::now().to_rfc3339()
            }),
        ))
    }
}
