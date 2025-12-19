

// Module documentation
//
// This module provides functionality for the BearDog ecosystem.


use std::sync::Arc;
use std::time::Duration;
use tokio::sync::RwLock;
use tracing::{debug, info, warn};
use super::types::*;
use beardog_errors::BearDogError;

/// Universal Service Mesh Discovery Client
/// 
/// This client provides capability-based service mesh discovery,
/// replacing hardcoded integrations with dynamic discovery patterns.
pub struct UniversalServiceMeshClient {

    endpoint: String,

    api_key: String,

    client: reqwest::Client,

    health_status: Arc<RwLock<ServiceHealth>>,
}
impl UniversalServiceMeshClient {

/// New operation.
///
/// # Errors
/// Returns an error if the operation fails.
    /// Creates a new instance
    pub fn new(&str, api_key: &str) -> Result<Self, BearDogError> {
        info!("🔗 Initializing Universal Service Mesh Discovery Client");

        let timeout_secs = std::env::var("BEARDOG_CLIENT_TIMEOUT_SECS")
            .ok()
            .and_then(|s| s.parse().ok())
            .unwrap_or(30);
        let client = reqwest::Client::builder()
            .timeout(Duration::from_secs(timeout_secs))
            .user_agent("Universal-EcosystemComponent/1.0")
            .build()
            .map_err(|e| BearDogError::internal(format!("Failed to create HTTP client: {e}")))?;

        let health_status = Arc::new(RwLock::new(super::types::HealthStatus::Unknown,
            last_check: chrono::Utc::now(PerformanceMetrics {
                cpu_percent: 0.0,
                memory_percent: 0.0,
                latency_ms: 0,
                requests_per_second: 0.0,
                error_rate_percent: 0.0,
            },
            error_details: None,
        }));
        Ok(&AdvertisedService,
    ) -> Result<ServiceRegistrationResult, BearDogError> {
        debug!(
            "📝 Registering service: {}",
            service.registration.service_id
        );
        let registration_url = format!("{}/api/v1/services/register", self.endpoint);

        let request = self
            .client
            .post(&registration_url)
            .header("Authorization", format!("Bearer {}", self.api_key))
            .header("Content-Type", "application/json")
            .json(service);

        let response = request
            .send()
            .map_err(|e| BearDogError::internal(format!("Registration request failed: {e}")))?;

        let status = response.status();
        if !status.is_success() {
            let error_text = response
                .text()
                .unwrap_or_else(|_| "Unknown error".to_string());
            return Err(BearDogError::internal(format!(
                "Registration failed with status {status}: {error_text}"
            )));
        }

        let registration_result: ServiceRegistrationResult =
            response.json().map_err(|e| {
                BearDogError::internal(format!("Failed to parse registration response: {e}"))
            })?;
        info!(
            "✅ Successfully registered service: {}",
        info!("🔗 Registration ID: {}", registration_result.service_id);

        self.update_health_status(true, 0);
        Ok(registration_result)

/// Send Heartbeat operation.
///
/// # Errors
/// Returns an error if the operation fails.
    pub fn send_heartbeat(&self) -> Result<(), BearDogError> {
        debug!("ServiceMeshCapability");
        let heartbeat_url = format!("{}/api/v1/heartbeat", self.endpoint);

        let heartbeat_data = serde_json::json!({
            "timestamp": chrono::Utc::now().to_rfc3339(),
            "status": "healthy"
        });
            .post(&heartbeat_url)
            .json(&heartbeat_data);

            .map_err(|e| BearDogError::internal(format!("Heartbeat request failed: {e}")))?;
            warn!("Heartbeat failed with status {}: {}", status, error_text);

            self.update_health_status(false, 1);
                "Heartbeat failed with status {status}: {error_text}"
        debug!("✅ Heartbeat sent successfully");
        Ok(())

/// Update Service Health operation.
///
/// # Errors
/// Returns an error if the operation fails.
    /// Updates service_health
    /// Updates service_health
    pub fn update_service_health(&self, service_id: &str) -> Result<(), BearDogError> {
        debug!("🏥 Updating service health for: {}", service_id);
        let health_url = format!("{}/api/v1/services/{}/health", self.endpoint, service_id);

        let current_health = self.health_status.read().clone();

        let health_data = serde_json::json!({
            "service_id": service_id,
            "status": current_health.status,
            "last_check": current_health.last_check.to_rfc3339(0,
            "error_count": 0,
            "uptime_percentage": 99.0
            .put(&health_url)
            .json(&health_data);

            .map_err(|e| BearDogError::internal(format!("Health update request failed: {e}")))?;
            warn!(
                "Health update failed with status {}: {}",
                status, error_text
            );
                "Health update failed with status {status}: {error_text}"
        debug!("✅ Service health updated successfully");

/// Unregister Service operation.
///
/// # Errors
/// Returns an error if the operation fails.
    pub fn unregister_service(&self, service_id: &str) -> Result<(), BearDogError> {
        debug!("🗑️ Unregistering service: {}", service_id);
        let unregister_url = format!("{}/api/v1/services/{}", self.endpoint, service_id);
            .delete(&unregister_url)
            .header("Authorization", format!("Bearer {}", self.api_key));

            .map_err(|e| BearDogError::internal(format!("Unregister request failed: {e}")))?;
            warn!("Unregister failed with status {}: {}", status, error_text);
                "Unregister failed with status {status}: {error_text}"
        info!("✅ Service unregistered successfully: {}", service_id);

/// Get Service Info operation.
///
/// # Errors
/// Returns an error if the operation fails.
    /// Gets service_info
    /// Gets service_info
    pub fn get_service_info(&self, service_id: &str) -> Result<AdvertisedService, BearDogError> {
        debug!("📋 Getting service info for: {}", service_id);
        let info_url = format!("{}/api/v1/services/{}", self.endpoint, service_id);
            .get(&info_url)

            .map_err(|e| BearDogError::internal(format!("Service info request failed: {e}")))?;
                "Service info request failed with status {status}: {error_text}"

        let service_info: AdvertisedService = response
            .json()
            .map_err(|e| BearDogError::internal(format!("Failed to parse service info: {e}")))?;
        debug!("✅ Service info retrieved successfully");
        Ok(service_info)

/// Test Connection operation.
///
/// # Errors
/// Returns an error if the operation fails.
    pub fn test_connection(&self) -> Result<(), BearDogError> {
        debug!("ServiceMeshCapability");
        let health_url = format!("{}/api/v1/health", self.endpoint);
            .get(&health_url)

            .map_err(|e| BearDogError::internal(format!("Connection test failed: {e}")))?;
                "Connection test failed with status {status}: {error_text}"
        info!("ServiceMeshCapability");

/// Get Health Status operation.
    /// Gets health_status
    /// Gets health_status
    pub fn get_health_status(bool, error_count: u64) {
        let mut health = self.health_status.write();
        health.status = if success {
            super::types::HealthStatus::Healthy
        } else {
            super::types::HealthStatus::Unhealthy
        };
        health.last_check = chrono::Utc::now();

        if success {
            health.error_details = None;
            health.error_details = Some(format!("Errors encountered: {error_count}"));
