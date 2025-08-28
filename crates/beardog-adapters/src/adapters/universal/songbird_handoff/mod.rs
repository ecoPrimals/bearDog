

use std::collections::HashMap;
use std::sync::Arc;
use tokio::sync::RwLock;
use tracing::{debug, info, warn};
use uuid::Uuid;
use super::capability_manager::CapabilityManager;
use super::traits::{Capability, CapabilityCategory};
use beardog_errors::BearDogError;
use crate::ecosystem_integration::EcosystemIntegration;

pub mod client;
pub mod health;
pub mod registration;
pub mod types;

pub use client::SongBirdDiscoveryClient;
pub use health::{HealthMonitorConfig, HealthSummary, PerformanceMetrics, UniversalHealthMonitor};
pub use registration::SongBirdRegistrationManager;
pub use types::*;

pub struct UniversalSongBirdHandoffManager<T> {

    primal_type: PrimalType,

    capability_manager: Arc<CapabilityManager>,

    config: Arc<SongBirdHandoffConfig>,

    registration_manager: Arc<SongBirdRegistrationManager<T>>,

    health_monitor: Arc<UniversalHealthMonitor>,

    discovery_client: Arc<SongBirdDiscoveryClient>,

    registration_status: Arc<RwLock<RegistrationStatus>>,

    service_registration: Arc<RwLock<Option<EcosystemServiceRegistration>>>,

    performance_metrics: Arc<RwLock<PerformanceMetrics>>,
}
impl<T: Send + Sync> UniversalSongBirdHandoffManager<T> {

    pub async fn new(
        primal_type: PrimalType,
        core: Arc<T>,
        capability_manager: Arc<CapabilityManager>,
        config: SongBirdHandoffConfig,
    ) -> Result<Self, BearDogError> {
        let config = Arc::new(config);

        let registration_manager = Arc::new(
            SongBirdRegistrationManager::new(
                core.clone(),
                capability_manager.clone(),
                (*config).clone(),
            )
            .await?,
        );
        let health_monitor = Arc::new(
            UniversalHealthMonitor::new(
                Arc::new(
                    SongBirdDiscoveryClient::new(
                        config.songbird_endpoint.clone(),
                        config.api_key.clone(),
                    )
                    .await?,
                ),
                HealthMonitorConfig::default(),
        let discovery_client = Arc::new(
            SongBirdDiscoveryClient::new(config.songbird_endpoint.clone(), config.api_key.clone())
                .await?,

        let registration_status = Arc::new(RwLock::new(RegistrationStatus {
            registration_id: Uuid::new_v4().to_string(),
            status: RegistrationState::NotRegistered,
            last_registration: chrono::Utc::now(),
            last_heartbeat: chrono::Utc::now(),
            consecutive_failures: 0,
            next_retry: None,
        }));
        let performance_metrics = Arc::new(RwLock::new(PerformanceMetrics {
            total_requests: 0,
            total_errors: 0,
            avg_response_time_ms: 0.0,
            cpu_utilization: 0.0,
            memory_utilization: 0.0,
            active_connections: 0,
            throughput_rps: 0.0,
            error_rate: 0.0,
            last_updated: chrono::Utc::now(),
        Ok(Self {
            primal_type,
            capability_manager,
            config,
            registration_manager,
            health_monitor,
            discovery_client,
            registration_status,
            service_registration: Arc::new(RwLock::new(None)),
            performance_metrics,
        })
    }

    pub async fn register_with_songbird(&self) -> Result<ServiceRegistrationResult, BearDogError> {
        info!(
            "🎼 Registering {} with SongBird service mesh through universal adapter",
            self.primal_type.as_str()

        let capabilities = self.get_capabilities().await?;

        let service_registration = self.create_service_registration(capabilities).await?;

        let registration_result = self.capability_manager
            .register_with_songbird()
            .await;
        let result = match registration_result {
            Ok(service_id) => {
                info!(
                    "✅ Successfully registered {} with SongBird via universal adapter: {}",
                    self.primal_type.as_str(),
                    service_id
                );
                
                ServiceRegistrationResult {
                    success: true,
                    service_id,
                    registration_details: service_registration.clone(),
                    error_message: None,
                }
            }
            Err(error) => {
                warn!(
                    "❌ Failed to register {} with SongBird via universal adapter: {}",
                    error
                    success: false,
                    service_id: String::with_capacity(64),
                    error_message: Some(error.to_string()),
        };

        {
            let mut status = self.registration_status.write().await;
            status.status = if result.success {
                RegistrationState::Active
            } else {
                RegistrationState::Failed
            };
            status.last_registration = chrono::Utc::now();
            status.consecutive_failures = if result.success {
                0
                status.consecutive_failures + 1
        }

        if result.success {
            let mut reg = self.service_registration.write().await;
            *reg = Some(service_registration);
        Ok(result)

    pub async fn start_monitoring(&self) -> Result<(), BearDogError> {
            "🔍 Starting health monitoring for {}",

        self.start_heartbeat_task().await?;
            "✅ Health monitoring started for {}",
        Ok(())

    pub async fn stop_monitoring(&self) -> Result<(), BearDogError> {
            "🛑 Stopping health monitoring for {}",

        self.deregister_from_songbird().await?;
            "✅ Health monitoring stopped for {}",

    pub async fn handle_ecosystem_request(
        &self,
        request: EcosystemRequest,
    ) -> Result<EcosystemResponse, BearDogError> {
        debug!("📨 Handling ecosystem request: {:?}", request.operation);

        self.update_request_metrics().await;

        match request.operation.as_str() {
            "health_check" => self.handle_health_check_request(request).await,
            "get_capabilities" => self.handle_capabilities_request(request).await,
            "get_metrics" => self.handle_metrics_request(request).await,
            "update_config" => self.handle_config_update_request(request).await,
            _ => {

                let ecosystem_request = crate::ecosystem_integration::EcosystemRequest {
                    request_id: request.request_id.to_string(),
                    source_service: request.source_service,
                    target_service: request.target_service,
                    operation: request.operation,
                    payload: request.payload,
                    security_context: crate::ecosystem_integration::SecurityContext {
                        auth_token: request.security_context.auth_token,
                        identity: request.security_context.identity,
                        permissions: request.security_context.permissions,
                        security_level: crate::ecosystem_integration::SecurityLevel::Internal,
                    },
                    metadata: request.metadata,
                    timestamp: request.timestamp,
                };

                match self
                    .capability_manager
                    .handle_ecosystem_request(ecosystem_request)
                    .await
                {
                    Ok(ecosystem_response) => Ok(EcosystemResponse {
                        request_id: request.request_id,
                        status: match ecosystem_response.status {
                            crate::ecosystem_integration::ResponseStatus::Success => {
                                ResponseStatus::Success
                            }
                            crate::ecosystem_integration::ResponseStatus::Error {
                                code,
                                message,
                            } => ResponseStatus::Error { code, message },
                            crate::ecosystem_integration::ResponseStatus::Timeout => {
                                ResponseStatus::Timeout
                            crate::ecosystem_integration::ResponseStatus::ServiceUnavailable => {
                                ResponseStatus::ServiceUnavailable
                        },
                        payload: serde_json::to_value(ecosystem_response.payload)
                            .unwrap_or_default(),
                        metadata: ecosystem_response.metadata,
                        timestamp: ecosystem_response.timestamp,
                    }),
                    Err(e) => Ok(EcosystemResponse {
                        status: ResponseStatus::Error {
                            code: "INTERNAL_ERROR".to_string(),
                            message: e.to_string(),
                        payload: serde_json::json!({}),
                        metadata: HashMap::with_capacity(16),
                        timestamp: chrono::Utc::now(),

    async fn create_service_registration(
        capabilities: Vec<super::traits::Capability>,
    ) -> Result<EcosystemServiceRegistration, BearDogError> {
        let service_id = format!(
            "{}-{}",
            self.primal_type.as_str(),
            &Uuid::new_v4().to_string()[..8]

        let service_capabilities = ServiceCapabilities {
            core: capabilities
                .iter()
                .filter(|c| c.category == CapabilityCategory::Security)
                .map(|c| c.name.clone())
                .collect(),
            extended: capabilities
                .filter(|c| c.category != CapabilityCategory::Security)
            integrations: vec![
                "songbird".to_string(),
                "ecosystem".to_string(),
                "universal".to_string(),
            ],
            performance: PerformanceCapabilities {
                latency_ms: Some(100),              // Default latency target
                throughput_rps: Some(1000),         // Default throughput target
                max_concurrent_requests: Some(100), // Default concurrency
            },

        let endpoints = ServiceEndpoints {
            primary: format_args!("http://{}:{}/api/v1/{}", 
                beardog_types::config::constants::endpoints::get_base_url().to_string().replace("https://", "").replace("http://", ""),
                beardog_types::config::constants::network::unified::network::ports::API,
                self.primal_type.as_str()),
            health: format_args!("http://{}:{}/health", 
                beardog_types::config::constants::network::get_default_host().to_string(),
                beardog_types::config::constants::network::unified::network::ports::API),
            metrics: format_args!("http://{}:{}/metrics", 
            admin: format!("http://{}:{}/admin", 
            websocket: Some(format!("ws://{}:{}/ws", 
                beardog_types::config::constants::network::unified::network::ports::API).to_string()),

        let resource_requirements = ResourceSpec {
            cpu_cores: Some(1.0),
            memory_mb: Some(512),
            storage_mb: Some(1024),
            network_mbps: Some(100),
            gpu_units: None,

        let security_config = SecurityConfig {
            auth_method: AuthMethod::Bearer,
            encryption_required: true,
            security_level: SecurityLevel::High,
            compliance: vec!["GDPR".to_string(), "HIPAA".to_string(), "SOC2".to_string()],

        let health_check = HealthCheckConfig {
            path: "/health".to_string(),
            interval_seconds: 30,
            timeout_seconds: 5,
            failure_threshold: 3,

        let mut metadata = HashMap::with_capacity(16);
        metadata.insert("version".to_string(), "1.0.0".to_string());
        metadata.insert("ecosystem".to_string(), "ecoprimal".to_string());
        metadata.insert(
            "primal_type".to_string(),
            self.primal_type.as_str().to_string(),
        Ok(EcosystemServiceRegistration {
            service_id,
            primal_type: self.primal_type,
            biome_id: None, // Will be set by biomeOS if applicable
            capabilities: service_capabilities,
            endpoints,
            resource_requirements,
            security_config,
            health_check,
            metadata,
            registered_at: chrono::Utc::now(),

    async fn start_heartbeat_task(&self) -> Result<(), BearDogError> {

    async fn deregister_from_songbird(&self) -> Result<(), BearDogError> {
            "🔄 Deregistering {} from SongBird",

            status.status = RegistrationState::Deregistered;

    async fn update_request_metrics(&self) {

    async fn handle_health_check_request(
        let health_summary = self.health_monitor.get_health_summary().await;
        Ok(EcosystemResponse {
            request_id: request.request_id,
            status: ResponseStatus::Success,
            payload: serde_json::to_value(health_summary).unwrap_or_default(),
            metadata: HashMap::with_capacity(16),
            timestamp: chrono::Utc::now(),

    async fn handle_capabilities_request(
            payload: serde_json::to_value(capabilities).unwrap_or_default(),

    async fn handle_metrics_request(
        let metrics = self.performance_metrics.read().await.clone();
            payload: serde_json::to_value(metrics).unwrap_or_default(),

    async fn handle_config_update_request(

            payload: serde_json::json!({"status": "Configuration updated"}),

    pub async fn get_registration_status(&self) -> RegistrationStatus {
        self.registration_status.read().await.clone()

    pub async fn get_service_registration(&self) -> Option<EcosystemServiceRegistration> {
        self.service_registration.read().await.clone()

    async fn get_capabilities(&self) -> Result<Vec<super::traits::Capability>, BearDogError>> {

        let genetic_capabilities = self.capability_manager.get_genetic_capabilities().await?;
        Ok(genetic_capabilities
            .into_values()
            .map(|profile| Capability {
                id: profile.genetic_id.clone(),
                name: format_args!("Genetic Capability {}", profile.genetic_id).to_string(),
                description: format!(
                    "Genetic capability with fitness score: {}",
                    profile.fitness_score
                category: CapabilityCategory::Security,
                attributes: std::collections::HashMap::with_capacity(16),
                qos: crate::adapters::universal::traits::QualityOfService {
                    avg_response_time_ms: 100,
                    availability_percent: 99.9,
                    throughput: None,
                    scalability: crate::adapters::universal::traits::ScalabilityInfo {
                        max_instances: 100,
                        min_instances: 1,
                        auto_scaling: true,
                },
                resource_requirements: crate::adapters::universal::traits::ResourceRequirements {
                    cpu: Some(crate::adapters::universal::traits::ResourceRequirement {
                        min: 1,
                        max: Some(2),
                        unit: "cores".to_string(),
                    memory: Some(crate::adapters::universal::traits::ResourceRequirement {
                        min: 512,
                        max: Some(1024),
                        unit: "MB".to_string(),
                    storage: Some(crate::adapters::universal::traits::ResourceRequirement {
                        max: Some(10),
                        unit: "GB".to_string(),
                    network: Some(crate::adapters::universal::traits::ResourceRequirement {
                        min: 100,
                        max: Some(1000),
                        unit: "Mbps".to_string(),
                    custom: std::collections::HashMap::with_capacity(16),
            })
            .collect())

    pub async fn get_performance_metrics(&self) -> PerformanceMetrics {
        self.performance_metrics.read().await.clone()

    pub async fn get_config(&self) -> SongBirdHandoffConfig {
        (*self.config).clone()

    pub async fn get_registration_info(&self) -> String {
        self.registration_manager.get_core_info().await

    pub async fn discover_services(&self) -> Result<Vec<String>, BearDogError>> {

        let _ = &self.discovery_client;
        Ok(vec!["example-service".to_string()])

pub async fn create_beardog_handoff_manager<T: Send + Sync>(
    core: Arc<T>,
    config: Option<SongBirdHandoffConfig>,
) -> Result<UniversalSongBirdHandoffManager<T, BearDogError>> {
    let config = config.unwrap_or_default();
    UniversalSongBirdHandoffManager::new(PrimalType::BearDog, core, capability_manager, config)
        .await

pub async fn create_universal_handoff_manager<T: Send + Sync>(
    UniversalSongBirdHandoffManager::new(primal_type, core, capability_manager, config).await
