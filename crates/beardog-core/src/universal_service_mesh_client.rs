

use beardog_errors::BearDogError;
use beardog_errors::idiomatic::SecurityResult;
use reqwest::Client as HttpClient;
use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use std::sync::Arc;
use std::time::Duration;
use tokio::sync::RwLock;
use tracing::{debug, error, info, warn};
use uuid::Uuid;
use beardog_adapters::adapters::universal::traits::{
    PrimalProvider, ServiceRequest, ServiceResponse, Capability, HealthStatus,
    EcosystemRegistration, ServiceEndpoints
};

pub struct UniversalServiceMeshClient {

    client: HttpClient,

    active_mesh: Arc<RwLock<Option<ServiceMeshCapability>>>,

    registration: Arc<RwLock<Option<EcosystemRegistration>>>,

    service_cache: Arc<RwLock<HashMap<String, Vec<DiscoveredService>>>>,

    available_meshes: Arc<RwLock<Vec<ServiceMeshCapability>>>,

    timeout: Duration,

    config: UniversalMeshConfig,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ServiceMeshCapability {

    pub primal_id: String,

    pub endpoint: String,

    pub capabilities: Vec<String>,

    pub api_version: String,

    pub health: HealthStatus,

    pub quality_score: f64,

    pub last_success: Option<chrono::DateTime<chrono::Utc>>,

#[derive(Debug, Clone)]
pub struct UniversalMeshConfig {

    pub discovery_timeout: Duration,

    pub health_check_interval: Duration,

    pub max_concurrent_requests: usize,

    pub enable_failover: bool,

    pub quality_threshold: f64,}

impl Default for UniversalMeshConfig {}

    fn default() -> Self {
        Self {
            discovery_timeout: Duration::from_secs(30),
            health_check_interval: Duration::from_secs(60),
            max_concurrent_requests: 100,
            enable_failover: true,
            quality_threshold: 0.8,
        }
    }

pub struct BearDogRegistration {

    pub registration_id: String,

    pub instance_id: String,

    pub endpoints: ServiceEndpoints,

    pub registered_at: chrono::DateTime<chrono::Utc>,

pub struct DiscoveredService {

    pub service_id: String,

    pub service_name: String,

    pub service_type: String,

    pub endpoints: Vec<String>,

    pub discovered_at: chrono::DateTime<chrono::Utc>,

pub struct UniversalServiceRequest {

    pub request_id: String,

    pub target_service_type: String,

    pub payload: serde_json::Value,

    pub metadata: HashMap<String, String>,

    pub timeout: Option<Duration>,

pub struct UniversalServiceResponse {

    pub processing_time: Duration,

    pub source_primal: String,}

impl UniversalServiceMeshClient {

    pub fn new(config: UniversalMeshConfig) -> Self {
        let client = HttpClient::builder()
            .timeout(config.discovery_timeout)
            .build()
            .map_err(|e| {
    tracing::error!("Operation failed ({}): {:?}", "Failed to create HTTP client", e);
    beardog_errors::BearDogError::internal(format_args!("Operation failed ({}): {:?}", "Failed to create HTTP client", e).to_string())
})?;
            client,
            active_mesh: Arc::new(RwLock::new(None)),
            registration: Arc::new(RwLock::new(None)),
            service_cache: Arc::new(RwLock::new(ahash::HashMap::default())),
            available_meshes: Arc::new(RwLock::new(Vec::new())),
            timeout: config.discovery_timeout,
            config,

    pub async fn discover_service_meshes(&self) -> Result<Vec<ServiceMeshCapability>, BearDogError> {
        info!("🔍 Discovering service mesh capabilities in ecosystem");
        let mut discovered_meshes = Vec::new();

        let discovery_endpoints = self.get_discovery_endpoints().await?;
        for endpoint in discovery_endpoints {
            match self.probe_service_mesh_capability(&endpoint).await {
                Ok(capability) => {
                    info!(
                        "✅ Discovered service mesh capability: {} at {}",
                        capability.primal_id, capability.endpoint
                    );
                    discovered_meshes.push(capability);
                }
                Err(e) => {
                    debug!("❌ Failed to probe endpoint {}: {}", endpoint, e);
            }

        {
            let mut available = self.available_meshes.write().await;
            *available = discovered_meshes.clone();

        if let Some(best_mesh) = self.select_best_mesh(&discovered_meshes).await? {
            let mut active = self.active_mesh.write().await;
            *active = Some(best_mesh);
            info!("🎯 Selected service mesh: {}", discovered_meshes[0].primal_id);
        Ok(discovered_meshes)

    async fn get_discovery_endpoints(&self) -> Result<Vec<String>, BearDogError> {
        let mut endpoints = Vec::new();

        if let Ok(discovery_url) = std::env::var("ECOSYSTEM_DISCOVERY_URL") {
            endpoints.push(discovery_url);

        let base_host = std::env::var("ECOSYSTEM_HOST").unwrap_or_else(|_| "localhost".to_string());
        let discovery_ports = std::env::var("ECOSYSTEM_DISCOVERY_PORTS")
            .unwrap_or_else(|_| "8080,8081,8082,8083".to_string());
        for port in discovery_ports.split(',') {
            if let Ok(port_num) = port.trim().parse::<u16>() {
                endpoints.push(format_args!("http://{}:{}", base_host, port_num).to_string());
        if endpoints.is_empty() {
            return Err(BearDogError::configuration("No discovery endpoints configured. Set ECOSYSTEM_DISCOVERY_URL or ECOSYSTEM_DISCOVERY_PORTS".to_string(),
            ));
        Ok(endpoints)

    async fn probe_service_mesh_capability(&self, endpoint: &str) -> Result<ServiceMeshCapability, BearDogError> {
        debug!("🔍 Probing service mesh capability at: {}", endpoint);

        let capability_url = format_args!("{}/ecosystem/capabilities", endpoint).to_string();
        let response = self
            .client
            .get(&capability_url)
            .timeout(self.config.discovery_timeout)
            .send()
            .await
            .map_err(|e| BearDogError::NetworkError {
                message: format_args!("Failed to probe {}: {}", endpoint, e).to_string(),
            })?;
        if !response.status().is_success() {
            return Err(BearDogError::NetworkError {
                message: format_args!("Capability probe failed with status: {}", response.status().to_string()),
            });
        let capability_data: serde_json::Value = response
            .json()
            .map_err(|e| BearDogError::ParseError {
                message: format_args!("Failed to parse capability response: {}", e).to_string(),

        let primal_id = capability_data
            .get("primal_id")
            .and_then(|v| v.as_str())
            .ok_or_else(|| BearDogError::ParseError {
                message: "Missing primal_id in capability response".to_string(),
            })?
            .to_string();
        let capabilities = capability_data
            .get("capabilities")
            .and_then(|v| v.as_array())
            .map(|arr| {
                arr.iter()
                    .filter_map(|v| v.as_str().map(|s| s.to_string()))
                    .collect()
            })
            .unwrap_or_default();
        let api_version = capability_data
            .get("api_version")
            .unwrap_or("v1")

        if !capabilities.iter().any(|cap| cap.contains("service_mesh") || cap.contains("routing")) {
            return Err(BearDogError::Capability {
                message: format_args!("Primal {} does not offer service mesh capabilities", primal_id).to_string(),
        Ok(ServiceMeshCapability {
            primal_id,
            endpoint: endpoint.to_string(),
            capabilities,
            api_version,
            health: HealthStatus::Healthy, // Will be updated by health checks
            quality_score: 1.0,            // Will be calculated based on performance
            last_success: Some(chrono::Utc::now()),
        })

    async fn select_best_mesh(&self, meshes: &[ServiceMeshCapability]) -> Result<Option<ServiceMeshCapability>, BearDogError> {
        if meshes.is_empty() {
            return Ok(None);

        let mut scored_meshes: Vec<(f64, ServiceMeshCapability)> = Vec::new();
        for mesh in meshes {
            let mut score = 0.0;

            score += mesh.capabilities.len() as f64 * 0.1;

            score += match mesh.health {
                HealthStatus::Healthy => 1.0,
                HealthStatus::Degraded => 0.5,
                HealthStatus::Unhealthy => 0.0,
            };

            if let Some(last_success) = mesh.last_success {
                let age = chrono::Utc::now().signed_duration_since(last_success);
                let age_hours = age.num_hours() as f64;
                score += (24.0 - age_hours.min(24.0)) / 24.0; // Decay over 24 hours

            score += mesh.quality_score;
            scored_meshes.push((score, mesh.clone()));

        scored_meshes.sort_by(|a, b| b.0.partial_cmp(&a.0).unwrap_or(std::cmp::Ordering::Equal));

        for (score, mesh) in scored_meshes {
            if score >= self.config.quality_threshold {
                info!(
                    "🎯 Selected service mesh: {} (score: {:.2})",
                    mesh.primal_id, score
                );
                return Ok(Some(mesh));
        warn!("⚠️ No service mesh meets quality threshold {:.2}", self.config.quality_threshold);
        Ok(None)

    pub async fn register_with_ecosystem(&self) -> Result<BearDogRegistration, BearDogError> {
        let active_mesh = {
            let mesh_guard = self.active_mesh.read().await;
            mesh_guard.clone().ok_or_else(|| BearDogError::State {
                message: "No active service mesh available for registration".to_string(),
        };
        info!("🔗 Registering BearDog with ecosystem via: {}", active_mesh.primal_id);
        let registration_request = self.create_registration_request().await?;
        let registration_url = format_args!("{}/ecosystem/register", active_mesh.endpoint).to_string();
            .post(&registration_url)
            .json(&registration_request)
            .timeout(self.timeout)
                message: format_args!("Registration request failed: {}", e).to_string(),
            let error_text = response.text().await.unwrap_or_else(|_| "Unknown error".to_string());
                message: format_args!("Registration failed: {}", error_text).to_string(),
        let registration_response: serde_json::Value = response
                message: format_args!("Failed to parse registration response: {}", e).to_string(),
        let registration = BearDogRegistration {
            registration_id: registration_response
                .get("registration_id")
                .and_then(|v| v.as_str())
                .unwrap_or_else(|| &Uuid::new_v4().to_string())
                .to_string(),
            instance_id: Uuid::new_v4().to_string(),
            capabilities: vec![
                "security".to_string(),
                "encryption".to_string(),
                "key_management".to_string(),
                "authentication".to_string(),
            ],
            endpoints: ServiceEndpoints {
                primary: std::env::var("BEARDOG_ENDPOINT")
                    .unwrap_or_else(|_| "http://localhost:8080".to_string()),
                health: Some(format_args!("{}/health", 
                    std::env::var("BEARDOG_ENDPOINT").to_string()
                        .unwrap_or_else(|_| "http://localhost:8080".to_string()))),
                metrics: Some(format_args!("{}/metrics", 
                    std::env::var("BEARDOG_ENDPOINT").to_string()
                        .unwrap_or_else(|_| "http://localhost:8080".to_string()))),
                additional: ahash::HashMap::default(),
            },
            registered_at: chrono::Utc::now(),

            let mut reg_guard = self.registration.write().await;
            *reg_guard = Some(EcosystemRegistration {
                registration_id: registration.registration_id.clone(),
                instance_id: registration.instance_id.clone(),
                registered_at: registration.registered_at,
                expires_at: None,
                metadata: ahash::HashMap::default(),
        info!("✅ Successfully registered BearDog with ecosystem");
        Ok(registration)

    async fn create_registration_request(&self) -> Result<serde_json::Value, BearDogError> {
        Ok(serde_json::json!({
            "primal_id": "beardog",
            "instance_id": Uuid::new_v4().to_string(),
            "service_name": "BearDog Security Services",
            "service_version": env!("CARGO_PKG_VERSION"),
            "capabilities": [
                "security",
                "encryption", 
                "key_management",
                "authentication",
                "hsm_integration",
                "zero_trust"
            "endpoints": {
                "primary": std::env::var("BEARDOG_ENDPOINT")
                "health": format_args!("{}/health", 
                        .unwrap_or_else(|_| "http://localhost:8080".to_string().to_string())),
                "metrics": format!("{}/metrics",
                        .unwrap_or_else(|_| "http://localhost:8080".to_string()))
            "metadata": {
                "deployment_environment": std::env::var("DEPLOYMENT_ENV").unwrap_or_else(|_| "development".to_string()),
                "instance_type": "security_provider"
        }))

    pub async fn send_service_request(&self, request: UniversalServiceRequest) -> Result<UniversalServiceResponse, BearDogError> {
                message: "No active service mesh available for request routing".to_string(),
        debug!(
            "📤 Sending service request {} to {} via {}",
            request.request_id, request.target_service_type, active_mesh.primal_id
        );
        let routing_url = format_args!("{}/ecosystem/route", active_mesh.endpoint).to_string();
        let start_time = std::time::Instant::now();
            .post(&routing_url)
            .json(&request)
            .timeout(request.timeout.unwrap_or(self.timeout))
                message: format_args!("Service request failed: {}", e).to_string(),
        let processing_time = start_time.elapsed();
            return Err(BearDogError::ServiceError {
                message: format_args!("Service request failed: {}", error_text).to_string(),
        let response_data: serde_json::Value = response
                message: format_args!("Failed to parse service response: {}", e).to_string(),
        Ok(UniversalServiceResponse {
            request_id: request.request_id,
            payload: response_data.get("payload").cloned().unwrap_or(serde_json::Value::Null),
            metadata: response_data
                .get("metadata")
                .and_then(|v| serde_json::from_value(v.clone()).ok())
                .unwrap_or_default(),
            processing_time,
            source_primal: active_mesh.primal_id,

    pub async fn discover_services(&self, service_type: Option<&str>) -> Result<Vec<DiscoveredService>, BearDogError> {
                message: "No active service mesh available for service discovery".to_string(),
        let mut discovery_url = format_args!("{}/ecosystem/services", active_mesh.endpoint).to_string();
        if let Some(svc_type) = service_type {
            discovery_url.push_str(&format_args!("?type={}", svc_type).to_string());
            .get(&discovery_url)
                message: format_args!("Service discovery failed: {}", e).to_string(),
                message: format_args!("Service discovery failed with status: {}", response.status().to_string()),
        let services_data: serde_json::Value = response
                message: format_args!("Failed to parse services response: {}", e).to_string(),
        let services = services_data
            .get("services")
                message: "Invalid services response format".to_string(),
        let mut discovered_services = Vec::new();
        for service in services {
            if let Ok(discovered_service) = self.parse_discovered_service(service) {
                discovered_services.push(discovered_service);

            let mut cache = self.service_cache.write().await;
            cache.insert(svc_type.to_string(), discovered_services.clone());
        info!("🔍 Discovered {} services", discovered_services.len());
        Ok(discovered_services)

    fn parse_discovered_service(&self, service_data: &serde_json::Value) -> Result<DiscoveredService, SecurityError> {
        Ok(DiscoveredService {
            service_id: service_data
                .get("service_id")
                .unwrap_or("unknown")
            service_name: service_data
                .get("service_name")
                .unwrap_or("Unknown Service")
            service_type: service_data
                .get("service_type")
            endpoints: service_data
                .get("endpoints")
                .and_then(|v| v.as_array())
                .map(|arr| {
                    arr.iter()
                        .filter_map(|v| v.as_str().map(|s| s.to_string()))
                        .collect()
                })
            capabilities: service_data
                .get("capabilities")
            health: HealthStatus::Healthy, // Default, will be updated by health checks
            discovered_at: chrono::Utc::now(),

    pub async fn get_active_mesh(&self) -> Option<ServiceMeshCapability> {
        let mesh_guard = self.active_mesh.read().await;
        mesh_guard.clone()

    pub async fn is_registered(&self) -> bool {
        let reg_guard = self.registration.read().await;
        reg_guard.is_some()
} 
