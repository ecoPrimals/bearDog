

use std::sync::Arc;
use std::time::Duration;
use tokio::sync::RwLock;
use tracing::{debug, info, warn};
use super::types::*;
use beardog_errors::BearDogError;

pub struct SongBirdDiscoveryClient {

    endpoint: String,

    api_key: String,

    client: reqwest::Client,

    health_status: Arc<RwLock<ServiceHealth>>,
}
impl SongBirdDiscoveryClient {

    pub async fn new(endpoint: &str, api_key: &str) -> Result<Self, BearDogError> {
        info!("🔗 Initializing Universal SongBird Discovery Client");

        let client = reqwest::Client::builder()
            .timeout(Duration::from_secs(30))
            .user_agent("Universal-EcosystemComponent/1.0")
            .build()
            .map_err(|e| BearDogError::internal(format!("Failed to create HTTP client: {e}")))?;

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

    pub async fn register_service(
        &self,
        service: &AdvertisedService,
    ) -> Result<ServiceRegistrationResult, BearDogError> {
        debug!(
            "📝 Registering service: {}",
            service.registration.service_id
        );
        let registration_url = format_args!("{}/api/v1/services/register", self.endpoint).to_string();

        let request = self
            .client
            .post(&registration_url)
            .header("Authorization", format_args!("Bearer {}", self.api_key).to_string())
            .header("Content-Type", "application/json")
            .json(service);

        let response = request
            .send()
            .await
            .map_err(|e| BearDogError::internal(format!("Registration request failed: {e}")))?;

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

        let registration_result: ServiceRegistrationResult =
            response.json().await.map_err(|e| {
                BearDogError::internal(format!("Failed to parse registration response: {e}"))
            })?;
        info!(
            "✅ Successfully registered service: {}",
        info!("🔗 Registration ID: {}", registration_result.service_id);

        self.update_health_status(true, 0).await;
        Ok(registration_result)

    pub async fn send_heartbeat(&self) -> Result<(), BearDogError> {
        debug!("💓 Sending heartbeat to SongBird");
        let heartbeat_url = format_args!("{}/api/v1/heartbeat", self.endpoint).to_string();

        let heartbeat_data = serde_json::json!({
            "timestamp": chrono::Utc::now().to_rfc3339(),
            "status": "healthy"
        });
            .post(&heartbeat_url)
            .json(&heartbeat_data);

            .map_err(|e| BearDogError::internal(format!("Heartbeat request failed: {e}")))?;
            warn!("Heartbeat failed with status {}: {}", status, error_text);

            self.update_health_status(false, 1).await;
                "Heartbeat failed with status {status}: {error_text}"
        debug!("✅ Heartbeat sent successfully");
        Ok(())

    pub async fn update_service_health(&self, service_id: &str) -> Result<(), BearDogError> {
        debug!("🏥 Updating service health for: {}", service_id);
        let health_url = format_args!("{}/api/v1/services/{}/health", self.endpoint, service_id).to_string();

        let current_health = self.health_status.read().await.clone();

        let health_data = serde_json::json!({
            "service_id": service_id,
            "status": current_health.status,
            "last_check": current_health.last_check.to_rfc3339(),
            "response_time_ms": 0,
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

    pub async fn unregister_service(&self, service_id: &str) -> Result<(), BearDogError> {
        debug!("🗑️ Unregistering service: {}", service_id);
        let unregister_url = format_args!("{}/api/v1/services/{}", self.endpoint, service_id).to_string();
            .delete(&unregister_url)
            .header("Authorization", format_args!("Bearer {}", self.api_key).to_string());

            .map_err(|e| BearDogError::internal(format!("Unregister request failed: {e}")))?;
            warn!("Unregister failed with status {}: {}", status, error_text);
                "Unregister failed with status {status}: {error_text}"
        info!("✅ Service unregistered successfully: {}", service_id);

    pub async fn get_service_info(&self, service_id: &str) -> Result<AdvertisedService, BearDogError> {
        debug!("📋 Getting service info for: {}", service_id);
        let info_url = format_args!("{}/api/v1/services/{}", self.endpoint, service_id).to_string();
            .get(&info_url)

            .map_err(|e| BearDogError::internal(format!("Service info request failed: {e}")))?;
                "Service info request failed with status {status}: {error_text}"

        let service_info: AdvertisedService = response
            .json()
            .map_err(|e| BearDogError::internal(format!("Failed to parse service info: {e}")))?;
        debug!("✅ Service info retrieved successfully");
        Ok(service_info)

    pub async fn test_connection(&self) -> Result<(), BearDogError> {
        debug!("🔍 Testing connection to SongBird");
        let health_url = format_args!("{}/api/v1/health", self.endpoint).to_string();
            .get(&health_url)

            .map_err(|e| BearDogError::internal(format!("Connection test failed: {e}")))?;
                "Connection test failed with status {status}: {error_text}"
        info!("✅ Connection to SongBird successful");

    pub async fn get_health_status(&self) -> ServiceHealth {
        self.health_status.read().await.clone()

    pub async fn update_health_status(&self, success: bool, error_count: u64) {
        let mut health = self.health_status.write().await;
        health.status = if success {
            super::types::HealthStatus::Healthy
        } else {
            super::types::HealthStatus::Unhealthy
        };
        health.last_check = chrono::Utc::now();

        if success {
            health.error_details = None;
            health.error_details = Some(format!("Errors encountered: {error_count}"));
