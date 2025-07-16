//! Universal SongBird Client
//!
//! **Universal HTTP client for SongBird communication**
//!
//! This module provides a universal HTTP client for communicating with SongBird's
//! discovery and orchestration platform. It handles service registration,
//! heartbeats, and health updates in a domain-agnostic way.

use std::sync::Arc;
use std::time::Duration;
use tokio::sync::RwLock;
use tracing::{debug, info, warn};

use super::types::*;
use crate::{BearDogError, BearDogResult};

/// Universal SongBird Discovery Client
///
/// HTTP client for communicating with SongBird's discovery and orchestration
/// platform. Designed to work with any ecosystem component.
pub struct SongBirdDiscoveryClient {
    /// SongBird orchestrator endpoint
    endpoint: String,

    /// API credentials for authentication
    api_key: String,

    /// HTTP client for making requests
    client: reqwest::Client,

    /// Client health status
    health_status: Arc<RwLock<ServiceHealth>>,
}

impl SongBirdDiscoveryClient {
    /// Create a new universal SongBird client
    pub async fn new(endpoint: String, api_key: String) -> BearDogResult<Self> {
        info!("🔗 Initializing Universal SongBird Discovery Client");

        // Create HTTP client with universal configuration
        let client = reqwest::Client::builder()
            .timeout(Duration::from_secs(30))
            .user_agent("Universal-EcosystemComponent/1.0")
            .build()
            .map_err(|e| BearDogError::internal(format!("Failed to create HTTP client: {e}")))?;

        // Initialize health status
        let health_status = Arc::new(RwLock::new(ServiceHealth {
            status: super::types::HealthStatus::Unknown,
            last_check: chrono::Utc::now(),
            metrics: PerformanceMetrics {
                cpu_percent: 0.0,
                memory_percent: 0.0,
                latency_ms: 0,
                requests_per_second: 0.0,
                error_rate_percent: 0.0,
            },
            error_details: None,
        }));

        Ok(Self {
            endpoint,
            api_key,
            client,
            health_status,
        })
    }

    /// Register a service with SongBird
    pub async fn register_service(
        &self,
        service: &AdvertisedService,
    ) -> BearDogResult<ServiceRegistrationResult> {
        debug!("📝 Registering service: {}", service.registration.service_id);

        let registration_url = format!("{}/api/v1/services/register", self.endpoint);

        // Create registration request
        let request = self
            .client
            .post(&registration_url)
            .header("Authorization", format!("Bearer {}", self.api_key))
            .header("Content-Type", "application/json")
            .json(service);

        // Send registration request
        let response = request
            .send()
            .await
            .map_err(|e| BearDogError::internal(format!("Registration request failed: {e}")))?;

        // Check response status
        let status = response.status();
        if !status.is_success() {
            let error_text = response
                .text()
                .await
                .unwrap_or_else(|_| "Unknown error".to_string());
            return Err(BearDogError::internal(format!(
                "Registration failed with status {status}: {error_text}"
            )));
        }

        // Parse registration result
        let registration_result: ServiceRegistrationResult =
            response.json().await.map_err(|e| {
                BearDogError::internal(format!("Failed to parse registration response: {e}"))
            })?;

        info!("✅ Successfully registered service: {}", service.registration.service_id);
        info!(
            "🔗 Registration ID: {}",
            registration_result.service_id
        );

        // Update health status
        self.update_health_status(true, 0).await;

        Ok(registration_result)
    }

    /// Send heartbeat to SongBird
    pub async fn send_heartbeat(&self) -> BearDogResult<()> {
        debug!("💓 Sending heartbeat to SongBird");

        let heartbeat_url = format!("{}/api/v1/heartbeat", self.endpoint);

        // Create heartbeat request
        let heartbeat_data = serde_json::json!({
            "timestamp": chrono::Utc::now().to_rfc3339(),
            "status": "healthy"
        });

        let request = self
            .client
            .post(&heartbeat_url)
            .header("Authorization", format!("Bearer {}", self.api_key))
            .header("Content-Type", "application/json")
            .json(&heartbeat_data);

        // Send heartbeat
        let response = request
            .send()
            .await
            .map_err(|e| BearDogError::internal(format!("Heartbeat request failed: {e}")))?;

        // Check response status
        let status = response.status();
        if !status.is_success() {
            let error_text = response
                .text()
                .await
                .unwrap_or_else(|_| "Unknown error".to_string());
            warn!("Heartbeat failed with status {}: {}", status, error_text);

            // Update health status with error
            self.update_health_status(false, 1).await;

            return Err(BearDogError::internal(format!(
                "Heartbeat failed with status {status}: {error_text}"
            )));
        }

        debug!("✅ Heartbeat sent successfully");

        // Update health status
        self.update_health_status(true, 0).await;

        Ok(())
    }

    /// Update service health with SongBird
    pub async fn update_service_health(&self, service_id: &str) -> BearDogResult<()> {
        debug!("🏥 Updating service health for: {}", service_id);

        let health_url = format!("{}/api/v1/services/{}/health", self.endpoint, service_id);

        // Get current health status
        let current_health = self.health_status.read().await.clone();

        // Create health update request
        let health_data = serde_json::json!({
            "service_id": service_id,
            "status": current_health.status,
            "last_check": current_health.last_check.to_rfc3339(),
            "response_time_ms": 0,
            "error_count": 0,
            "uptime_percentage": 99.0
        });

        let request = self
            .client
            .put(&health_url)
            .header("Authorization", format!("Bearer {}", self.api_key))
            .header("Content-Type", "application/json")
            .json(&health_data);

        // Send health update
        let response = request
            .send()
            .await
            .map_err(|e| BearDogError::internal(format!("Health update request failed: {e}")))?;

        // Check response status
        let status = response.status();
        if !status.is_success() {
            let error_text = response
                .text()
                .await
                .unwrap_or_else(|_| "Unknown error".to_string());
            warn!(
                "Health update failed with status {}: {}",
                status, error_text
            );

            return Err(BearDogError::internal(format!(
                "Health update failed with status {status}: {error_text}"
            )));
        }

        debug!("✅ Service health updated successfully");

        Ok(())
    }

    /// Unregister service from SongBird
    pub async fn unregister_service(&self, service_id: &str) -> BearDogResult<()> {
        debug!("🗑️ Unregistering service: {}", service_id);

        let unregister_url = format!("{}/api/v1/services/{}", self.endpoint, service_id);

        let request = self
            .client
            .delete(&unregister_url)
            .header("Authorization", format!("Bearer {}", self.api_key));

        // Send unregister request
        let response = request
            .send()
            .await
            .map_err(|e| BearDogError::internal(format!("Unregister request failed: {e}")))?;

        // Check response status
        let status = response.status();
        if !status.is_success() {
            let error_text = response
                .text()
                .await
                .unwrap_or_else(|_| "Unknown error".to_string());
            warn!("Unregister failed with status {}: {}", status, error_text);

            return Err(BearDogError::internal(format!(
                "Unregister failed with status {status}: {error_text}"
            )));
        }

        info!("✅ Service unregistered successfully: {}", service_id);

        Ok(())
    }

    /// Get service information from SongBird
    pub async fn get_service_info(&self, service_id: &str) -> BearDogResult<AdvertisedService> {
        debug!("📋 Getting service info for: {}", service_id);

        let info_url = format!("{}/api/v1/services/{}", self.endpoint, service_id);

        let request = self
            .client
            .get(&info_url)
            .header("Authorization", format!("Bearer {}", self.api_key));

        // Send info request
        let response = request
            .send()
            .await
            .map_err(|e| BearDogError::internal(format!("Service info request failed: {e}")))?;

        // Check response status
        let status = response.status();
        if !status.is_success() {
            let error_text = response
                .text()
                .await
                .unwrap_or_else(|_| "Unknown error".to_string());
            return Err(BearDogError::internal(format!(
                "Service info request failed with status {status}: {error_text}"
            )));
        }

        // Parse service info
        let service_info: AdvertisedService = response
            .json()
            .await
            .map_err(|e| BearDogError::internal(format!("Failed to parse service info: {e}")))?;

        debug!("✅ Service info retrieved successfully");

        Ok(service_info)
    }

    /// Test connection to SongBird
    pub async fn test_connection(&self) -> BearDogResult<()> {
        debug!("🔍 Testing connection to SongBird");

        let health_url = format!("{}/api/v1/health", self.endpoint);

        let request = self
            .client
            .get(&health_url)
            .header("Authorization", format!("Bearer {}", self.api_key));

        // Send health check request
        let response = request
            .send()
            .await
            .map_err(|e| BearDogError::internal(format!("Connection test failed: {e}")))?;

        // Check response status
        let status = response.status();
        if !status.is_success() {
            let error_text = response
                .text()
                .await
                .unwrap_or_else(|_| "Unknown error".to_string());
            return Err(BearDogError::internal(format!(
                "Connection test failed with status {status}: {error_text}"
            )));
        }

        info!("✅ Connection to SongBird successful");

        // Update health status
        self.update_health_status(true, 0).await;

        Ok(())
    }

    /// Get current health status
    pub async fn get_health_status(&self) -> ServiceHealth {
        self.health_status.read().await.clone()
    }

    /// Update health status
    async fn update_health_status(&self, success: bool, error_count: u64) {
        let mut health = self.health_status.write().await;

        health.status = if success {
            super::types::HealthStatus::Healthy
        } else {
            super::types::HealthStatus::Unhealthy
        };

        health.last_check = chrono::Utc::now();
        // Update error details based on success rate
        if success {
            health.error_details = None;
        } else {
            health.error_details = Some(format!("Errors encountered: {}", error_count));
        }
    }
}
