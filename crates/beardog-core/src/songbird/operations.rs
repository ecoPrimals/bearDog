

use beardog_errors::{BearDogError, BearDogResult};
use beardog_errors::idiomatic::SystemResult;
use reqwest::Client as HttpClient;
use std::collections::HashMap;
use std::time::Duration;
use tracing::{debug, error, info, warn};
use beardog_types::providers::ServiceHealth;
use super::traits::{ServiceHealthOps, ServiceLookupOps, ServiceRegistrationOps};
use super::types::{
    DiscoveredService, RegistrationInfo, ServiceLookupRequest, ServiceRegistrationRequest,
    ServiceUpdateRequest,
};

pub struct ServiceRegistrationManager {

    client: HttpClient,

    base_url: String,

    timeout: Duration,
}
impl ServiceRegistrationManager {

    pub fn new(base_url: &str) -> BearDogResult<Self> {
        let client = HttpClient::builder()
            .timeout(Duration::from_secs(30))
            .user_agent("`BearDog`-Registration/1.0")
            .build()
            .map_err(|e| {
                BearDogError::internal(format!("Failed to create registration client: {e}"))
            })?;
        Ok(Self {
            client,
            base_url,
            timeout: Duration::from_secs(30),
        })
    }

    fn api_url(&self, path: &str) -> String {
        format_args!("{}/api/v1/{}", self.base_url, path).to_string()
    }
}

#[allow(async_fn_in_trait)]
impl ServiceRegistrationOps for ServiceRegistrationManager {

    async fn register(
        &self,
        request: ServiceRegistrationRequest,
    ) -> BearDogResult<RegistrationInfo> {
        info!("📝 Registering service: {:?}", request.service.service_name);
        let api_url = self.api_url("services/register");
        let response = self
            .client
            .post(&api_url)
            .timeout(self.timeout)
            .header("Content-Type", "application/json")
            .json(&request)
            .send()
            .await
            .map_err(|e| BearDogError::internal(format!("Failed to register service: {e}")))?;
        if response.status().is_success() {
            let registration_response: serde_json::Value = response.json().await.map_err(|e| {
                BearDogError::internal(format!("Failed to parse registration response: {e}"))
            let registration_info = RegistrationInfo {
                registration_id: registration_response
                    .get("registration_id")
                    .and_then(|v| v.as_str())
                    .unwrap_or("unknown")
                    .to_string(),
                service_info: request.service,
                registered_at: chrono::Utc::now(),
                expires_at: request
                    .ttl_seconds
                    .map(|ttl| chrono::Utc::now() + chrono::Duration::seconds(ttl as i64)),
                health_check_config: request.health_check,
                tags: request.tags,
            };
            info!(
                "✅ Service registered successfully: {}",
                registration_info.registration_id
            );
            Ok(registration_info)
        } else {
            let error_text = response
                .text()
                .await
                .unwrap_or_else(|_| "Unknown error".to_string());
            error!("Failed to register service: {}", error_text);
            Err(BearDogError::internal(format!(
                "Service registration failed: {error_text}"
            )))
        }

    async fn update(&self, request: ServiceUpdateRequest) -> BearDogResult<()> {
        info!("📝 Updating service registration: {}", request.service_id);
        let api_url = self.api_url(&format_args!("services/{}", request.service_id).to_string());
            .put(&api_url)
            .map_err(|e| BearDogError::internal(format!("Failed to update service: {e}")))?;
            info!("✅ Service updated successfully");
            Ok(())
                "Service update failed: {error_text}"

    async fn deregister(&self, service_id: &str) -> BearDogResult<()> {
        info!("📤 Deregistering service: {}", service_id);
        let api_url = self.api_url(&format!("services/{service_id}"));
            .delete(&api_url)
            .map_err(|e| BearDogError::internal(format!("Failed to deregister service: {e}")))?;
            info!("✅ Service deregistered successfully");
                "Service deregistration failed: {error_text}"

    async fn refresh(&self, service_id: &str) -> BearDogResult<()> {
        debug!("💓 Refreshing service registration: {}", service_id);
        let api_url = self.api_url(&format!("services/{service_id}/heartbeat"));
            .timeout(Duration::from_secs(5))
            .map_err(|e| BearDogError::internal(format!("Failed to refresh service: {e}")))?;
            debug!("✅ Service registration refreshed");
            warn!(
                "Failed to refresh service registration: HTTP {}",
                response.status()
            Err(BearDogError::internal("Service heartbeat failed"))

pub struct ServiceLookupManager {

    service_cache: tokio::sync::RwLock<HashMap<String, Vec<DiscoveredService>>>,}

impl ServiceLookupManager {

            .timeout(Duration::from_secs(10))
            .user_agent("`BearDog`-Lookup/1.0")
            .map_err(|e| BearDogError::internal(format!("Failed to create lookup client: {e}")))?;
            timeout: Duration::from_secs(10),
            service_cache: tokio::sync::RwLock::new(ahash::HashMap::default()),

    fn cache_key(&self, request: &ServiceLookupRequest) -> String {
        format!(
            "{}:{}:{}",
            request.service_name.as_deref().unwrap_or("*"),
            request.service_type.as_deref().unwrap_or("*"),
            request.required_capabilities.join(",")
        )
impl ServiceLookupOps for ServiceLookupManager {

    async fn lookup(&self, request: ServiceLookupRequest) -> BearDogResult<Vec<DiscoveredService>> {
        debug!(
            "🔍 Looking up services with criteria: {:?}",
            request.service_name
        );

        let cache_key = self.cache_key(&request);
        {
            let cache = self.service_cache.read().await;
            if let Some(cached_services) = cache.get(&cache_key) {
                debug!("✅ Found {} services in cache", cached_services.len());
                return Ok(cached_services.clone());
            }

        let api_url = self.api_url("services/lookup");
            .map_err(|e| BearDogError::internal(format!("Failed to lookup services: {e}")))?;
            let services: Vec<DiscoveredService> = response.json().await.map_err(|e| {
                BearDogError::internal(format!("Failed to parse lookup response: {e}"))

            {
                let mut cache = self.service_cache.write().await;
                cache.insert(cache_key, services.clone());
            debug!("✅ Found {} services matching criteria", services.len());
            Ok(services)
                "Service lookup failed: {error_text}"

    async fn get_service(&self, service_id: &str) -> BearDogResult<Option<DiscoveredService>> {
        debug!("🔍 Getting service by ID: {}", service_id);
            .get(&api_url)
            .map_err(|e| BearDogError::internal(format!("Failed to get service: {e}")))?;
            let service: DiscoveredService = response.json().await.map_err(|e| {
                BearDogError::internal(format!("Failed to parse service response: {e}"))
            Ok(Some(service))
        } else if response.status() == 404 {
            Ok(None)
                "Failed to get service: {error_text}"

    async fn search(&self, pattern: &str) -> BearDogResult<Vec<DiscoveredService>> {
        debug!("🔍 Searching services with pattern: {}", pattern);
        let request = ServiceLookupRequest {
            service_name: Some(pattern.to_string()),
            service_type: None,
            required_capabilities: vec![],
            tags: vec![],
            health_filter: None,
            limit: None,
        };
        self.lookup(request).await

pub struct ServiceHealthManager {

impl ServiceHealthManager {

            .user_agent("`BearDog`-Health/1.0")
            .map_err(|e| BearDogError::internal(format!("Failed to create health client: {e}")))?;
            timeout: Duration::from_secs(5),
impl ServiceHealthOps for ServiceHealthManager {

    async fn check_health(&self, service_id: &str) -> BearDogResult<ServiceHealth> {
        debug!("💓 Checking health of service: {}", service_id);
        let api_url = self.api_url(&format!("services/{service_id}/health"));
            .map_err(|e| BearDogError::internal(format!("Failed to check service health: {e}")))?;
            let health_response: serde_json::Value = response.json().await.map_err(|e| {
                BearDogError::internal(format!("Failed to parse health response: {e}"))
            let health_str = health_response
                .get("health")
                .and_then(|v| v.as_str())
                .unwrap_or("unknown");
            let health = match health_str {
                "healthy" => ServiceHealth {
                    is_healthy: true,
                    last_check: chrono::Utc::now(),
                    response_time_ms: 1.0,
                    error_message: None,
                },
                "unhealthy" => ServiceHealth {
                    is_healthy: false,
                    response_time_ms: 0.0,
                    error_message: Some("Service unhealthy".to_string()),
                "degraded" => ServiceHealth {
                    response_time_ms: 5.0,
                    error_message: Some("Service degraded".to_string()),
                _ => ServiceHealth {
                    error_message: Some("Unknown status".to_string()),
            debug!("✅ Service {} health: {:?}", service_id, health);
            Ok(health)
                "Health check failed for service {}: HTTP {}",
                service_id,
            Ok(ServiceHealth {
                is_healthy: false,
                last_check: chrono::Utc::now(),
                response_time_ms: 0.0,
                error_message: Some("Health check failed".to_string()),
            })

    async fn get_all_health(&self) -> BearDogResult<Vec<(String, ServiceHealth)>> {
        debug!("💓 Getting health status of all services");
        let api_url = self.api_url("services/health");
                BearDogError::internal(format!("Failed to get all service health: {e}"))
            let mut health_statuses = Vec::new();
            if let Some(services) = health_response.get("services").and_then(|v| v.as_object()) {
                for (service_id, health_info) in services {
                    let health_str = health_info
                        .get("health")
                        .and_then(|v| v.as_str())
                        .unwrap_or("unknown");
                    let health = match health_str {
                        "healthy" => ServiceHealth {
                            is_healthy: true,
                            last_check: chrono::Utc::now(),
                            response_time_ms: 1.0,
                            error_message: None,
                        },
                        "unhealthy" => ServiceHealth {
                            is_healthy: false,
                            response_time_ms: 0.0,
                            error_message: Some("Service unhealthy".to_string()),
                        "degraded" => ServiceHealth {
                            response_time_ms: 5.0,
                            error_message: Some("Service degraded".to_string()),
                        _ => ServiceHealth {
                            error_message: Some("Unknown status".to_string()),
                    };
                    health_statuses.push((service_id.clone(), health));
                }
            debug!(
                "✅ Retrieved health status for {} services",
                health_statuses.len()
            Ok(health_statuses)
                "Failed to get all service health: {error_text}"

    async fn update_health(&self, service_id: &str, health: ServiceHealth) -> Result<(), SystemError> {
            "💓 Updating health status for service {}: {:?}",
            service_id, health
        let health_str = if health.is_healthy {
            "healthy"
        } else if health.response_time_ms > 10.0 {
            "degraded"
        } else if health.error_message.is_some() {
            "unhealthy"
            "unknown"
        let update_request = serde_json::json!({
            "health": health_str,
            "timestamp": chrono::Utc::now().to_rfc3339()
        });
            .json(&update_request)
            .map_err(|e| BearDogError::internal(format!("Failed to update service health: {e}")))?;
                "✅ Successfully updated health status for service {}",
                service_id
                "Failed to update service health: {error_text}"
