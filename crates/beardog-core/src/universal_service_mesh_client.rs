

// Module documentation
//
// This module provides functionality for the BearDog ecosystem.


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

#[derive(Debug, Clone)]
    /// The endpoint value
    pub endpoint: String,

    /// Collection of capabilities
    pub capabilities: Vec<String>,

    /// The api version value
    pub api_version: String,

    /// The health value
    pub health: HealthStatus,

    /// The quality score value
    pub quality_score: f64,

    /// Optional last success
    pub last_success: Option<chrono::DateTime<chrono::Utc>>,

#[derive(Debug, Clone)]
    /// The health check interval value
    pub health_check_interval: Duration,

    /// Number of max_concurrent_requests
    pub max_concurrent_requests: usize,

    /// Whether enable_failover is enabled
    pub enable_failover: bool,

    /// The quality threshold value
    pub quality_threshold: f64,
}

impl Default for UniversalMeshConfig {
    fn default() -> Self {
        Self {
            discovery_timeout: Duration::from_secs(
                std::env::var("BEARDOG_MESH_DISCOVERY_TIMEOUT_SECS")
                    .ok()
                    .and_then(|s| s.parse().ok())
                    .unwrap_or(30)
            ),
            health_check_interval: Duration::from_secs(
                std::env::var("BEARDOG_MESH_HEALTH_CHECK_INTERVAL_SECS")
                    .ok()
                    .and_then(|s| s.parse().ok())
                    .unwrap_or(100)
            ),
            enable_failover: true,
            quality_threshold: 0.8,
        }
    }
}

pub struct BearDogRegistration {


    pub registration_id: String,


    pub instance_id: String,

    /// The endpoints value
    pub endpoints: ServiceEndpoints,

    /// The registered at value
    pub registered_at: chrono::DateTime<chrono::Utc>,

pub struct DiscoveredService {


    pub service_id: String,

    /// Name of the service
    pub service_name: String,

    /// The service type value
    pub service_type: String,

    /// Collection of endpoints
    pub endpoints: Vec<String>,

    /// The discovered at value
    pub discovered_at: chrono::DateTime<chrono::Utc>,

pub struct UniversalServiceRequest {


    pub request_id: String,

    /// The target service type value
    pub target_service_type: String,

    /// The payload value
    pub payload: serde_json::Value,

    /// Mapping of metadata
    pub metadata: HashMap<String, String>,


    pub timeout: Option<Duration>,

pub struct UniversalServiceResponse {


    pub processing_time: Duration,

    /// The source primal value
    pub source_primal: String,}

impl UniversalServiceMeshClient {

/// New operation.
    /// Creates a new instance
    pub fn new(config: UniversalMeshConfig) -> Self {
        let client = HttpClient::builder()
            .timeout(config.discovery_timeout)
            .build()
            .map_err(|e| {
    tracing::error!("Operation failed ({}): {:?}", "Failed to create HTTP client", e);
    beardog_errors::BearDogError::internal(format!("Error: {:?}", "Failed to create HTTP client", e))
})?;
            client,
            active_mesh: Arc::new(RwLock::new(None)),
            registration: Arc::new(RwLock::new(None)),
            service_cache: Arc::new(RwLock::new(std::collections::HashMap::default())),
            available_meshes: Arc::new(RwLock::new(Vec::new(config.discovery_timeout,
            config,

/// Discover Service Meshes operation.
///
/// # Errors
/// Returns an error if the operation fails.
    pub fn discover_service_meshes(&self) -> Result<Vec<ServiceMeshCapability>, BearDogError> {
        info!("🔍 Discovering service mesh capabilities in ecosystem");
        let mut discovered_meshes = Vec::new({} at {}",
                        capability.primal_id, capability.endpoint
                    );
                    discovered_meshes.push({}", endpoint, e);
            }

        {
            let mut available = self.available_meshes.write({}", discovered_meshes[0].primal_id);
        Ok(discovered_meshes)

    /// Gets discovery_endpoints
    fn get_discovery_endpoints(&self) -> Result<Vec<String>, BearDogError> {
        let mut endpoints = Vec::new();

        // Priority 1: Explicit discovery URL from environment
        if let Ok(discovery_url) = std::env::var("ECOSYSTEM_DISCOVERY_URL") {
            endpoints.push(discovery_url);
        }

        // Priority 2: Capability-based discovery using DNS-SD
        if let Ok(discovery_result) = self.discover_via_dns_sd() {
            endpoints.extend(discovery_result);
        }

        // Priority 3: Container orchestration discovery
        if let Ok(orchestration_endpoints) = self.discover_via_container_orchestration() {
            endpoints.extend(orchestration_endpoints);
        }

        // Priority 4: Environment-based host/port discovery (fallback)
        let base_host = std::env::var("ECOSYSTEM_HOST").unwrap_or_else(|_| {
            // Try to detect if we're in a container environment
            if std::path::Path::new("/.dockerenv").exists() {
                "host.docker.internal".to_string()
            } else if std::env::var("CONTAINER_ORCHESTRATION_HOST").is_ok() || std::env::var("KUBERNETES_SERVICE_HOST").is_ok() {
                "beardog-discovery.default.svc.cluster.local".to_string()
            } else {
                use beardog_types::canonical::config::network::NetworkConfig;
                let network_config = NetworkConfig::default();
                std::env::var("BEARDOG_LOCAL_HOST").unwrap_or_else(|_| network_config.default_host.clone())
            }
        });

        let discovery_ports = std::env::var("ECOSYSTEM_DISCOVERY_PORTS")
            .unwrap_or_else(|_| "8080,8081,8082,8083".to_string());
        
        for port in discovery_ports.split(',') {
            if let Ok(port_num) = port.trim().parse::<u16>() {
                endpoints.push(format!("http://{}:{}", base_host, port_num));
            }
        }

        if endpoints.is_empty() {
            return Err(BearDogError::configuration("No discovery endpoints configured. Set ECOSYSTEM_DISCOVERY_URL, ensure DNS-SD is available, or set ECOSYSTEM_DISCOVERY_PORTS"));
        }

        Ok(endpoints)
    }

    /// Discover services using DNS Service Discovery (RFC 6763)
    fn discover_via_dns_sd(&self) -> Result<Vec<String>, BearDogError> {
        use std::process::Command;
        
        let output = Command::new("dig")
            .args(&["+short", "-t", "SRV", "_beardog._tcp.local"])
            .output();

        match output {
            Ok(result) if result.status.success() => {
                let srv_records = String::from_utf8_lossy(&result.stdout);
                let mut endpoints = Vec::new();
                
                for line in srv_records.lines() {
                    if let Some(endpoint) = self.parse_srv_record(line) {
                        endpoints.push(endpoint);
                    }
                }
                Ok(endpoints)
            }
            _ => {
                debug!("DNS-SD discovery not available or failed");
                Ok(Vec::new())
            }
        }
    }

    /// Discover services via container orchestration (universal capability)
    fn discover_via_container_orchestration(&self) -> Result<Vec<String>, BearDogError> {
        // Check for container orchestration environment indicators (universal)
        if std::env::var("CONTAINER_ORCHESTRATION_HOST").is_err() && std::env::var("KUBERNETES_SERVICE_HOST").is_err() {
            return Ok(Vec::new());
        }

        // Use universal discovery patterns instead of hardcoded vendor names
        let service_name = std::env::var("BEARDOG_SERVICE_NAME")
            .unwrap_or_else(|_| "beardog-discovery".to_string());
        let namespace = std::env::var("ORCHESTRATION_NAMESPACE")
            .or_else(|_| std::env::var("BEARDOG_NAMESPACE"))
            .unwrap_or_else(|_| "default".to_string());

        // Universal service endpoint pattern (works with k8s, docker swarm, nomad, etc.)
        let orchestration_endpoint = format!("{}.{}.svc.cluster.local:8080", service_name, namespace);
        Ok(vec![format!("http://{}", orchestration_endpoint)])
    }

    /// Parses srv_record
    fn parse_srv_record(&self, record: &str) -> Option<String> {
        // Parse SRV record format: priority weight port target
        let parts: Vec<&str> = record.split_whitespace().collect();
        if parts.len() >= 4 {
            let port = parts[2];
            let target = parts[3].trim_end_matches('.');
            Some(format!("http://{}:{}", target, port))
        } else {
            None
        }
    }


    fn probe_service_mesh_capability(&self, endpoint: &str) -> Result<ServiceMeshCapability, BearDogError> {
        debug!("🔍 Probing service mesh capability at: {}", endpoint);

        let capability_url = format!("{}/ecosystem/capabilities", endpoint);
        let response = self
            .client
            .get(&capability_url)
            .timeout(self.config.discovery_timeout)
            .send()
            .map_err(|e| BearDogError::NetworkError {
                message: format!("Failed to probe {}: {}", endpoint, e),
            })?;
        if !response.status().is_success() {
            return Err(BearDogError::NetworkError {
                message: format!("Capability probe failed with status: {}", response.status()),
            });
        let capability_data: serde_json::Value = response
            .json()
            .map_err(|e| BearDogError::ParseError {
                message: format!("Failed to parse capability response: {}", e),

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
                    .filter_map(|v| v.as_str().map(std::string::ToString::to_string))
                    .collect()
            })
            .unwrap_or_default();
        let api_version = capability_data
            .get("api_version")
            .unwrap_or("v1")

        if !capabilities.iter().any(|cap| cap.contains("service_mesh") || cap.contains("routing")) {
            return Err(BearDogError::Capability {
                message: format!("Primal {} does not offer service mesh capabilities", primal_id),
        Ok(ServiceMeshCapability {
            primal_id,
            endpoint: endpoint.to_string(),            // Will be calculated based on performance
            last_success: Some(chrono::Utc::now()),
        })


    fn select_best_mesh(&self, meshes: &[ServiceMeshCapability]) -> Result<Option<ServiceMeshCapability>, BearDogError> {
        if meshes.is_empty(Vec<(f64, ServiceMeshCapability)> = Vec::new();
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
            scored_meshes.push(&(score, mesh));

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

/// Register With Ecosystem operation.
///
/// # Errors
/// Returns an error if the operation fails.
    pub fn register_with_ecosystem(&self) -> Result<BearDogRegistration, BearDogError> {
        let active_mesh = {
            let mesh_guard = self.active_mesh.read();
            mesh_guard.clone().ok_or_else(|| BearDogError::State {
                message: "No active service mesh available for registration".to_string();
        let registration_request = self.create_registration_request(format!("Registration request failed: {}", e),
            let error_text = response.text(format!("Registration failed: {}", error_text),
        let registration_response: serde_json::Value = response
                message: format!("Failed to parse registration response: {}", e),
        let registration = BearDogRegistration {
            registration_id: registration_response
                .get("registration_id")
                .and_then(|v| v.as_str())
                .unwrap_or_else(|| &Uuid::new_v4().to_string())
                .to_string(),
            instance_id: Uuid::new_v4().to_string(),
            capabilities: vec![
                "security".to_string(),
                "encryption ".to_string(),
                "key_management".to_string(),
                "authentication".to_string(),
            ],
            endpoints: ServiceEndpoints {
                primary: std::env::var(Some(format!("{}/health", 
                    std::env::var(Some(format!("{}/metrics", 
                    std::env::var("BEARDOG_ENDPOINT")
                        .unwrap_or_else(|_| adapter.discover_capability_endpoint(required_capability)?.to_string()))),
                additional: std::collections::HashMap::default(),
            },
            registered_at: chrono::Utc::now(&registration.registration_id,
                instance_id: &registration.instance_id,
                registered_at: registration.registered_at,
                expires_at: None,
                metadata: std::collections::HashMap::default(),
        info!("✅ Successfully registered BearDog with ecosystem");
        Ok(registration)

    /// Creates registration_request
    fn create_registration_request(&self) -> Result<serde_json::Value, BearDogError> {
        Ok(serde_json::json!({
            "primal_id": "beardog ",
            "instance_id": Uuid::new_v4("BearDog Security Services",
            "service_version": env!("CARGO_PKG_VERSION"),
            "capabilities": [
                "security",
                "encryption ", 
                "key_management",
                "authentication",
                "hsm_integration",
                "zero_trust"
            "endpoints": {
                "primary": std::env::var(format!("{}/health", 
                        .unwrap_or_else(format!("{}/metrics",
                        .unwrap_or_else(|_| adapter.discover_capability_endpoint(required_capability)?.to_string()))
            "metadata": {
                "deployment_environment": std::env::var("DEPLOYMENT_ENV").unwrap_or_else(|_| "development".to_string()),
                "instance_type": "security_provider"
        }))

/// Send Service Request operation.
///
/// # Errors
/// Returns an error if the operation fails.
    pub fn send_service_request(&self, request: UniversalServiceRequest) -> Result<UniversalServiceResponse, BearDogError> {
                message: "No active service mesh available for request routing".to_string(),
        debug!(
            "📤 Sending service request {} to {} via {}",
            request.request_id, request.target_service_type, active_mesh.primal_id
        );
        let routing_url = format!("{}/ecosystem/route", active_mesh.endpoint);
        let start_time = std::time::Instant::now(format!("Service request failed: {}", e),
        let processing_time = start_time.elapsed();
            return Err(BearDogError::ServiceError {
                message: format!("Service request failed: {}", error_text),
        let response_data: serde_json::Value = response
                message: format!("Failed to parse service response: {}", e),
        Ok(UniversalServiceResponse {
            request_id: request.request_id.clone(),
            payload: response_data.get("payload").cloned().unwrap_or(serde_json::Value::Null),
            metadata: response_data
                .get("metadata")
                .and_then(&|v| serde_json::from_value(active_mesh.primal_id,

/// Discover Services operation.
///
/// # Errors
/// Returns an error if the operation fails.
    pub fn discover_services(&self, service_type: Option<&str>) -> Result<Vec<DiscoveredService>, BearDogError> {
                message: "No active service mesh available for service discovery".to_string(),
                message: format!("Service discovery failed with status: {}", response.status(serde_json::Value = response
                message: format!("Failed to parse services response: {}", e),
        let services = services_data
            .get("services")
                message: "Invalid services response format".to_string(),
        let mut discovered_services = Vec::new();
        for service in services {
            if let Ok(discovered_service) = self.parse_discovered_service(service) {
                discovered_services.push(discovered_service);

            let mut cache = self.service_cache.write();
            cache.insert(svc_type.to_string(), discovered_services.clone());
        info!("🔍 Discovered {} services", discovered_services.len());
        Ok(discovered_services)

    /// Parses discovered_service
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
                        .filter_map(|v| v.as_str().map(std::string::ToString::to_string))
                        .collect()
                })
            capabilities: service_data
                .get(HealthStatus::Healthy, // Default, will be updated by health checks
            discovered_at: chrono::Utc::now(),

/// Get Active Mesh operation.
    /// Gets active_mesh
    /// Gets active_mesh
    pub fn get_active_mesh(&self) -> Option<ServiceMeshCapability> {
        let mesh_guard = self.active_mesh.read();
        mesh_guard.clone()

/// Is Registered operation.
    /// Checks if registered
    /// Checks if registered
    pub fn is_registered(&self) -> bool {
        let reg_guard = self.registration.read();
        reg_guard.is_some()
} 
