// BearDog - Enterprise Security Ecosystem
// Copyright (C) 2025 EcoPrimals
//
// This program is free software: you can redistribute it and/or modify
// it under the terms of the GNU Affero General Public License as published by
// the Free Software Foundation, either version 3 of the License, or
// (at your option) any later version.
//
// This program is distributed in the hope that it will be useful,
// but WITHOUT ANY WARRANTY; without even the implied warranty of
// MERCHANTABILITY or FITNESS FOR A PARTICULAR PURPOSE. See the
// GNU Affero General Public License for more details.
//
// You should have received a copy of the GNU Affero General Public License
// along with this program. If not, see <https://www.gnu.org/licenses/>.


/// Universal Service Mesh Client
///
/// **PRIMAL-AGNOSTIC SERVICE MESH INTEGRATION - MODERNIZED ✅**
/// This module provides service mesh communication that works with ANY service mesh
/// primal in the ecosystem, following the principle that "primals only know themselves."
/// It uses the universal adapter pattern to discover and communicate with available
/// service mesh capabilities without hardcoding specific primal names.
/// 
/// **ZERO-COST ASYNC OPTIMIZATION** - Native async methods eliminate boxing overhead
/// This module now uses native async fn in trait definitions (available in Rust 1.75+)
/// which eliminates the Box<dyn Future> allocation overhead from async_trait.

use beardog_errors::{BearDogError, BearDogResult};
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
/// Universal service mesh client that discovers and uses available mesh capabilities
pub struct UniversalServiceMeshClient {
    /// HTTP client for mesh communication
    client: HttpClient,
    /// Currently active service mesh (discovered dynamically)
    active_mesh: Arc<RwLock<Option<ServiceMeshCapability>>>,
    /// BearDog's registration with the ecosystem
    registration: Arc<RwLock<Option<EcosystemRegistration>>>,
    /// Service discovery cache
    service_cache: Arc<RwLock<HashMap<String, Vec<DiscoveredService>>>>,
    /// Available service mesh capabilities in ecosystem
    available_meshes: Arc<RwLock<Vec<ServiceMeshCapability>>>,
    /// Request timeout
    timeout: Duration,
    /// Client configuration
    config: UniversalMeshConfig,
}
/// Service mesh capability information (primal-agnostic)
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ServiceMeshCapability {
    /// Primal ecosystem ID (discovered dynamically)
    pub primal_id: String,
    /// Service mesh endpoint URL
    pub endpoint: String,
    /// Available capabilities
    pub capabilities: Vec<String>,
    /// API version supported
    pub api_version: String,
    /// Health status
    pub health: HealthStatus,
    /// Quality metrics
    pub quality_score: f64,
    /// Last successful interaction
    pub last_success: Option<chrono::DateTime<chrono::Utc>>,
/// Universal mesh client configuration
#[derive(Debug, Clone)]
pub struct UniversalMeshConfig {
    /// Discovery timeout
    pub discovery_timeout: Duration,
    /// Health check interval
    pub health_check_interval: Duration,
    /// Maximum concurrent requests
    pub max_concurrent_requests: usize,
    /// Enable automatic failover
    pub enable_failover: bool,
    /// Quality threshold for mesh selection
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
/// BearDog's registration information in the ecosystem
pub struct BearDogRegistration {
    /// Registration ID
    pub registration_id: String,
    /// BearDog instance ID
    pub instance_id: String,
    /// Registered capabilities
    /// Service endpoints
    pub endpoints: ServiceEndpoints,
    /// Registration timestamp
    pub registered_at: chrono::DateTime<chrono::Utc>,
/// Service discovered through the universal mesh
pub struct DiscoveredService {
    /// Service identifier
    pub service_id: String,
    /// Service name
    pub service_name: String,
    /// Service type/category
    pub service_type: String,
    /// Available endpoints
    pub endpoints: Vec<String>,
    /// Service capabilities
    /// Discovery timestamp
    pub discovered_at: chrono::DateTime<chrono::Utc>,
/// Universal service request (primal-agnostic)
pub struct UniversalServiceRequest {
    /// Request ID
    pub request_id: String,
    /// Target service type
    pub target_service_type: String,
    /// Request payload
    pub payload: serde_json::Value,
    /// Request metadata
    pub metadata: HashMap<String, String>,
    /// Timeout for this request
    pub timeout: Option<Duration>,
/// Universal service response (primal-agnostic)
pub struct UniversalServiceResponse {
    /// Request ID this responds to
    /// Response payload
    /// Response metadata
    /// Processing time
    pub processing_time: Duration,
    /// Source primal that handled the request
    pub source_primal: String,}


impl UniversalServiceMeshClient {
    /// Create new universal service mesh client}


    pub fn new(config: UniversalMeshConfig) -> Self {
        let client = HttpClient::builder()
            .timeout(config.discovery_timeout)
            .build()
            .map_err(|e| {
    tracing::error!("Operation failed ({}): {:?}", "Failed to create HTTP client", e);
    beardog_errors::BearDogError::internal(format!("Operation failed ({}): {:?}", "Failed to create HTTP client", e))
})?;
            client,
            active_mesh: Arc::new(RwLock::new(None)),
            registration: Arc::new(RwLock::new(None)),
            service_cache: Arc::new(RwLock::new(HashMap::new())),
            available_meshes: Arc::new(RwLock::new(Vec::new())),
            timeout: config.discovery_timeout,
            config,
    /// Discover available service mesh capabilities in the ecosystem
    pub async fn discover_service_meshes(&self) -> BearDogResult<Vec<ServiceMeshCapability>> {
        info!("🔍 Discovering service mesh capabilities in ecosystem");
        let mut discovered_meshes = Vec::new();
        // Use environment-based discovery (no hardcoded primal names)
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
        // Update available meshes
        {
            let mut available = self.available_meshes.write().await;
            *available = discovered_meshes.clone();
        // Select best mesh based on quality
        if let Some(best_mesh) = self.select_best_mesh(&discovered_meshes).await? {
            let mut active = self.active_mesh.write().await;
            *active = Some(best_mesh);
            info!("🎯 Selected service mesh: {}", discovered_meshes[0].primal_id);
        Ok(discovered_meshes)
    /// Get discovery endpoints from environment (no hardcoded values)
    async fn get_discovery_endpoints(&self) -> BearDogResult<Vec<String>> {
        let mut endpoints = Vec::new();
        // Environment-based discovery
        if let Ok(discovery_url) = std::env::var("ECOSYSTEM_DISCOVERY_URL") {
            endpoints.push(discovery_url);
        // Standard ecosystem ports (convention-based, not primal-specific)
        let base_host = std::env::var("ECOSYSTEM_HOST").unwrap_or_else(|_| "localhost".to_string());
        let discovery_ports = std::env::var("ECOSYSTEM_DISCOVERY_PORTS")
            .unwrap_or_else(|_| "8080,8081,8082,8083".to_string());
        for port in discovery_ports.split(',') {
            if let Ok(port_num) = port.trim().parse::<u16>() {
                endpoints.push(format!("http://{}:{}", base_host, port_num));
        if endpoints.is_empty() {
            return Err(BearDogError::configuration("No discovery endpoints configured. Set ECOSYSTEM_DISCOVERY_URL or ECOSYSTEM_DISCOVERY_PORTS".to_string(),
            ));
        Ok(endpoints)
    /// Probe an endpoint for service mesh capabilities
    async fn probe_service_mesh_capability(&self, endpoint: &str) -> BearDogResult<ServiceMeshCapability> {
        debug!("🔍 Probing service mesh capability at: {}", endpoint);
        // Standard capability discovery endpoint (universal pattern)
        let capability_url = format!("{}/ecosystem/capabilities", endpoint);
        let response = self
            .client
            .get(&capability_url)
            .timeout(self.config.discovery_timeout)
            .send()
            .await
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
        // Extract capability information (universal format)
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
        // Check if this primal offers service mesh capabilities
        if !capabilities.iter().any(|cap| cap.contains("service_mesh") || cap.contains("routing")) {
            return Err(BearDogError::Capability {
                message: format!("Primal {} does not offer service mesh capabilities", primal_id),
        Ok(ServiceMeshCapability {
            primal_id,
            endpoint: endpoint.to_string(),
            capabilities,
            api_version,
            health: HealthStatus::Healthy, // Will be updated by health checks
            quality_score: 1.0,            // Will be calculated based on performance
            last_success: Some(chrono::Utc::now()),
        })
    /// Select the best available service mesh based on quality metrics
    async fn select_best_mesh(&self, meshes: &[ServiceMeshCapability]) -> BearDogResult<Option<ServiceMeshCapability>> {
        if meshes.is_empty() {
            return Ok(None);
        // Score meshes based on capabilities, health, and performance
        let mut scored_meshes: Vec<(f64, ServiceMeshCapability)> = Vec::new();
        for mesh in meshes {
            let mut score = 0.0;
            // Capability score (more capabilities = higher score)
            score += mesh.capabilities.len() as f64 * 0.1;
            // Health score
            score += match mesh.health {
                HealthStatus::Healthy => 1.0,
                HealthStatus::Degraded => 0.5,
                HealthStatus::Unhealthy => 0.0,
            };
            // Recency score (recent success = higher score)
            if let Some(last_success) = mesh.last_success {
                let age = chrono::Utc::now().signed_duration_since(last_success);
                let age_hours = age.num_hours() as f64;
                score += (24.0 - age_hours.min(24.0)) / 24.0; // Decay over 24 hours
            // Quality score
            score += mesh.quality_score;
            scored_meshes.push((score, mesh.clone()));
        // Sort by score (highest first)
        scored_meshes.sort_by(|a, b| b.0.partial_cmp(&a.0).unwrap_or(std::cmp::Ordering::Equal));
        // Return the highest scoring mesh that meets quality threshold
        for (score, mesh) in scored_meshes {
            if score >= self.config.quality_threshold {
                info!(
                    "🎯 Selected service mesh: {} (score: {:.2})",
                    mesh.primal_id, score
                );
                return Ok(Some(mesh));
        warn!("⚠️ No service mesh meets quality threshold {:.2}", self.config.quality_threshold);
        Ok(None)
    /// Register BearDog with the active service mesh
    pub async fn register_with_ecosystem(&self) -> BearDogResult<BearDogRegistration> {
        let active_mesh = {
            let mesh_guard = self.active_mesh.read().await;
            mesh_guard.clone().ok_or_else(|| BearDogError::State {
                message: "No active service mesh available for registration".to_string(),
        };
        info!("🔗 Registering BearDog with ecosystem via: {}", active_mesh.primal_id);
        let registration_request = self.create_registration_request().await?;
        let registration_url = format!("{}/ecosystem/register", active_mesh.endpoint);
            .post(&registration_url)
            .json(&registration_request)
            .timeout(self.timeout)
                message: format!("Registration request failed: {}", e),
            let error_text = response.text().await.unwrap_or_else(|_| "Unknown error".to_string());
                message: format!("Registration failed: {}", error_text),
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
                "encryption".to_string(),
                "key_management".to_string(),
                "authentication".to_string(),
            ],
            endpoints: ServiceEndpoints {
                primary: std::env::var("BEARDOG_ENDPOINT")
                    .unwrap_or_else(|_| "http://localhost:8080".to_string()),
                health: Some(format!("{}/health", 
                    std::env::var("BEARDOG_ENDPOINT")
                        .unwrap_or_else(|_| "http://localhost:8080".to_string()))),
                metrics: Some(format!("{}/metrics", 
                additional: HashMap::new(),
            },
            registered_at: chrono::Utc::now(),
        // Store registration
            let mut reg_guard = self.registration.write().await;
            *reg_guard = Some(EcosystemRegistration {
                registration_id: registration.registration_id.clone(),
                instance_id: registration.instance_id.clone(),
                registered_at: registration.registered_at,
                expires_at: None,
                metadata: HashMap::new(),
        info!("✅ Successfully registered BearDog with ecosystem");
        Ok(registration)
    /// Create registration request for BearDog
    async fn create_registration_request(&self) -> BearDogResult<serde_json::Value> {
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
                "health": format!("{}/health", 
                        .unwrap_or_else(|_| "http://localhost:8080".to_string())),
                "metrics": format!("{}/metrics",
                        .unwrap_or_else(|_| "http://localhost:8080".to_string()))
            "metadata": {
                "deployment_environment": std::env::var("DEPLOYMENT_ENV").unwrap_or_else(|_| "development".to_string()),
                "instance_type": "security_provider"
        }))
    /// Send a service request through the universal mesh
    pub async fn send_service_request(&self, request: UniversalServiceRequest) -> BearDogResult<UniversalServiceResponse> {
                message: "No active service mesh available for request routing".to_string(),
        debug!(
            "📤 Sending service request {} to {} via {}",
            request.request_id, request.target_service_type, active_mesh.primal_id
        );
        let routing_url = format!("{}/ecosystem/route", active_mesh.endpoint);
        let start_time = std::time::Instant::now();
            .post(&routing_url)
            .json(&request)
            .timeout(request.timeout.unwrap_or(self.timeout))
                message: format!("Service request failed: {}", e),
        let processing_time = start_time.elapsed();
            return Err(BearDogError::ServiceError {
                message: format!("Service request failed: {}", error_text),
        let response_data: serde_json::Value = response
                message: format!("Failed to parse service response: {}", e),
        Ok(UniversalServiceResponse {
            request_id: request.request_id,
            payload: response_data.get("payload").cloned().unwrap_or(serde_json::Value::Null),
            metadata: response_data
                .get("metadata")
                .and_then(|v| serde_json::from_value(v.clone()).ok())
                .unwrap_or_default(),
            processing_time,
            source_primal: active_mesh.primal_id,
    /// Discover services available through the mesh
    pub async fn discover_services(&self, service_type: Option<&str>) -> BearDogResult<Vec<DiscoveredService>> {
                message: "No active service mesh available for service discovery".to_string(),
        let mut discovery_url = format!("{}/ecosystem/services", active_mesh.endpoint);
        if let Some(svc_type) = service_type {
            discovery_url.push_str(&format!("?type={}", svc_type));
            .get(&discovery_url)
                message: format!("Service discovery failed: {}", e),
                message: format!("Service discovery failed with status: {}", response.status()),
        let services_data: serde_json::Value = response
                message: format!("Failed to parse services response: {}", e),
        let services = services_data
            .get("services")
                message: "Invalid services response format".to_string(),
        let mut discovered_services = Vec::new();
        for service in services {
            if let Ok(discovered_service) = self.parse_discovered_service(service) {
                discovered_services.push(discovered_service);
        // Update cache
            let mut cache = self.service_cache.write().await;
            cache.insert(svc_type.to_string(), discovered_services.clone());
        info!("🔍 Discovered {} services", discovered_services.len());
        Ok(discovered_services)
    /// Parse a discovered service from mesh response
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
    /// Get current active service mesh information
    pub async fn get_active_mesh(&self) -> Option<ServiceMeshCapability> {
        let mesh_guard = self.active_mesh.read().await;
        mesh_guard.clone()
    /// Check if registered with ecosystem}


    pub async fn is_registered(&self) -> bool {
        let reg_guard = self.registration.read().await;
        reg_guard.is_some()
} 
