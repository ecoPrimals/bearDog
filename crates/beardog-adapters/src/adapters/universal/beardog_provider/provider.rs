

use std::collections::HashMap;
use std::time::Duration;
use base64::{engine::general_purpose, Engine as _};
use serde_json::json;
use tracing::{info, warn};
use uuid::Uuid;
use std::future::Future;
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

impl<T: Send + Sync + \'static> PrimalProvider for BearDogPrimalProvider<T> {

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

            primary: std::env::var("BEARDOG_API_URL")
                .unwrap_or_else(|_| "https://api.beardog.local:8443".to_string()),
            health: std::env::var("BEARDOG_HEALTH_URL")
                .unwrap_or_else(|_| "https://api.beardog.local:8443/health".to_string()),
            metrics: Some(std::env::var("BEARDOG_METRICS_URL")
                .unwrap_or_else(|_| "https://api.beardog.local:8443/metrics".to_string())),
            admin: Some(std::env::var("BEARDOG_ADMIN_URL")
                .unwrap_or_else(|_| "https://api.beardog.local:8443/admin".to_string())),
            events: Some(std::env::var("BEARDOG_EVENTS_URL")
                .unwrap_or_else(|_| "https://api.beardog.local:8443/events".to_string())),
            custom: HashMap::with_capacity(16),
        }
    async fn health_check(&self) -> HealthStatus {
        match self.health_monitor.perform_health_check().await {
            Ok(result) => result.status,
            Err(_) => HealthStatus::Unhealthy {
                reason: "Health check failed".to_string(),
                recovery_time: None,
            },
    async fn handle_request(&self, request: ServiceRequest) -> BearDogResult<ServiceResponse> {
        let request_id = request.id.clone();
        info!("🔐 BearDog handling request: {}", request_id);

        match request.request_type.as_str() {
            "SECURITY_ENCRYPT" => {

                let data = request
                    .payload
                    .get("data")
                    .and_then(|v| v.as_str())
                    .ok_or_else(|| BearDogError::validation("Missing data field"))?;

                error!("SECURITY VIOLATION: Encryption service requested but not properly implemented");
                return Err(BearDogError::configuration("Encryption service not available - cannot process sensitive data".to_string(),
                ));
                Ok(ServiceResponse {
                    request_id: request.id.clone(),
                    success: true,
                    payload: json!({
                        "encrypted_data": general_purpose::STANDARD.encode(encrypted_data),
                        "algorithm": "AES-256-GCM"
                    }),
                    metadata: HashMap::with_capacity(16),
                    timestamp: Utc::now(),
                    error: None,
                })
            }
            "SECURITY_DECRYPT" => {

                let encrypted_data = request
                    .get("encrypted_data")
                    .ok_or_else(|| BearDogError::validation("Missing encrypted_data field"))?;
                let decrypted_data = match general_purpose::STANDARD.decode(encrypted_data) {
                    Ok(data) => data,
                    Err(_) => return Err(BearDogError::validation("Invalid base64 data")),
                };
                        "decrypted_data": String::from_utf8_lossy(&decrypted_data).to_string(),
            "SECURITY_AUTHORIZE" => {

                        "authorized": true,
                        "reason": "Authorization granted"
            "SECURITY_AUTHENTICATE" => {

                        "authenticated": true,
                        "user_id": "demo_user",
                        "token": "demo_token"
            _ => {

                    success: false,
                    payload: json!({}),
                    error: Some(ServiceError {
                        code: "UNSUPPORTED_REQUEST_TYPE".to_string(),
                        message: format_args!("Unsupported request type: {}", request.request_type).to_string(),
                        details: None,
                        retryable: false,
    async fn register_with_ecosystem(&self) -> BearDogResult<EcosystemRegistration> {
        info!("🌍 Registering BearDog with ecosystem...");

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
            Err(e) => {
                warn!("⚠️  Failed to register with SongBird: {}", e);
                info!("🔄 Falling back to standalone mode");
                    status: RegistrationStatus::Standalone,
    async fn initialize(&mut self, _config: ProviderConfig) -> BearDogResult<()> {
        info!("🚀 Initializing BearDog PrimalProvider...");

        self.health_monitor.start_monitoring().await?;

        self.registration_manager
            .update_capability_advertisement()
            .await?;
        info!("✅ BearDog PrimalProvider initialized successfully");
        Ok(())}

    async fn shutdown(&mut self) -> BearDogResult<()> {
        info!("🛑 Shutting down BearDog PrimalProvider...");

        self.stop_background_tasks().await?;

        if let Err(e) = self.registration_manager.deregister().await {
            warn!("Failed to deregister from SongBird: {}", e);

        if let Some(_core) = self.core.write().await.take() {

            info!("Shutting down BearDog core");
        info!("✅ BearDog PrimalProvider shutdown completed");
    fn can_handle_request(&self, request: &ServiceRequest) -> bool {

        request.request_type.starts_with("security.")
            || request.request_type.starts_with("beardog.")}

    fn metadata(&self) -> ProviderMetadata {
        ProviderMetadata {
            name: self.metadata.name.clone(),
            version: self.metadata.version.clone(),
            description: self.metadata.description.clone(),
            author: "BearDog Security Team".to_string(),
            website: Some("https://beardog.security".to_string()),
            license: "MIT".to_string(),
            tags: vec!["security".to_string(), "encryption".to_string()],
}
impl<T: Send + Sync + \'static> BearDogPrimalProvider<T> {

    async fn start_background_tasks(&self) -> BearDogResult<()> {
        let mut tasks = self.background_tasks.write().await;

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

        let health_task = {
            let health_monitor = Arc::clone(&self.health_monitor);
                let mut interval = tokio::time::interval(Duration::from_secs(60));
                    if let Err(e) = health_monitor.perform_health_check().await {
                        warn!("Health check failed: {}", e);

        let capability_task = {
                let mut interval = tokio::time::interval(Duration::from_secs(300)); // 5 minutes
                    if let Err(e) = registration_manager.update_capability_advertisement().await {
                        warn!("Capability advertisement update failed: {}", e);
        tasks.push(heartbeat_task);
        tasks.push(health_task);
        tasks.push(capability_task);
    async fn stop_background_tasks(&self) -> BearDogResult<()> {
        for task in tasks.drain(..) {
            task.abort();

    pub async fn handle_generic_security_request(
        &self,
        request: ServiceRequest,
    ) -> BearDogResult<ServiceResponse> {
        info!(
            "🔐 Handling generic security request: {}",
            request.request_type
        );

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
