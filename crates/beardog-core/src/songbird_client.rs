//! Universal Service Mesh Integration for BearDog
//!
//! Provides service mesh agnostic communication patterns that work with any
//! service mesh primal (Songbird, future alternatives, etc.). Uses capability-based
//! discovery to find and use the best available service mesh in the ecosystem.

use async_trait::async_trait;
use beardog_errors::{BearDogError, BearDogResult};
use reqwest::Client as HttpClient;
use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use std::sync::Arc;
use std::time::Duration;
use tokio::sync::RwLock;
use tracing::{debug, error, info, warn};
use uuid::Uuid;

use crate::universal_primal_provider::{
    PrimalIdentity, PrimalMetadata, PrimalService, PrimalType, ServiceContext, ServiceHealth,
};

/// Universal service mesh client that can work with any service mesh primal
pub struct UniversalServiceMeshClient {
    /// HTTP client for service mesh communication
    client: HttpClient,
    /// Active service mesh information
    active_mesh: Arc<RwLock<Option<ServiceMeshInfo>>>,
    /// BearDog registration information
    registration: Arc<RwLock<Option<RegistrationInfo>>>,
    /// Service discovery cache
    service_cache: Arc<RwLock<HashMap<String, Vec<DiscoveredService>>>>,
    /// Available service meshes discovered in ecosystem
    available_meshes: Arc<RwLock<Vec<ServiceMeshInfo>>>,
    /// Request timeout
    timeout: Duration,
}

/// Information about a discovered service mesh primal
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ServiceMeshInfo {
    /// Service mesh primal name (e.g., "Songbird", "NewMesh", etc.)
    pub name: String,
    /// Service mesh endpoint URL
    pub endpoint: String,
    /// Service mesh capabilities
    pub capabilities: Vec<String>,
    /// API version supported
    pub api_version: String,
    /// Health status of the service mesh
    pub health: ServiceHealth,
    /// Priority/preference score (higher = preferred)
    pub priority_score: u32,
    /// Last health check timestamp
    pub last_health_check: chrono::DateTime<chrono::Utc>,
}

/// BearDog registration information with active service mesh
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct RegistrationInfo {
    /// Unique registration ID assigned by service mesh
    pub registration_id: String,
    /// BearDog node identifier
    pub node_id: String,
    /// Service mesh that handled the registration
    pub service_mesh_name: String,
    /// Registration timestamp
    pub registered_at: chrono::DateTime<chrono::Utc>,
    /// Registration status
    pub status: RegistrationStatus,
}

/// Registration status with Songbird
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub enum RegistrationStatus {
    /// Initial registration pending
    Pending,
    /// Successfully registered and active
    Active,
    /// Registration failed
    Failed,
    /// Temporarily suspended
    Suspended,
    /// Deregistered from service mesh
    Deregistered,
}

/// Discovered service from Songbird service mesh
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct DiscoveredService {
    /// Service identifier
    pub service_id: String,
    /// Primal providing the service
    pub primal_name: String,
    /// Primal type
    pub primal_type: PrimalType,
    /// Service endpoint
    pub endpoint: ServiceEndpoint,
    /// Service capabilities
    pub capabilities: Vec<String>,
    /// Service health status
    pub health: ServiceHealth,
    /// Last seen timestamp
    pub last_seen: chrono::DateTime<chrono::Utc>,
}

/// Service endpoint configuration
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ServiceEndpoint {
    /// Protocol (http, https, grpc)
    pub protocol: String,
    /// Host address
    pub host: String,
    /// Port number
    pub port: u16,
    /// Path or route
    pub path: String,
}

/// Service request to be routed through Songbird
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ServiceRequest {
    /// Request identifier
    pub request_id: Uuid,
    /// Target service identifier
    pub service_id: String,
    /// Target primal name
    pub target_primal: String,
    /// Request operation
    pub operation: String,
    /// Request payload
    pub payload: serde_json::Value,
    /// Request metadata
    pub metadata: HashMap<String, String>,
    /// Source primal identity
    pub source: PrimalIdentity,
}

/// Service response from Songbird routing
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ServiceResponse {
    /// Original request identifier
    pub request_id: Uuid,
    /// Response status
    pub status: ResponseStatus,
    /// Response payload
    pub payload: serde_json::Value,
    /// Response metadata
    pub metadata: HashMap<String, String>,
    /// Processing time in milliseconds
    pub processing_time_ms: u64,
}

/// Response status from service calls
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub enum ResponseStatus {
    /// Request processed successfully
    Success,
    /// Request failed with error
    Error { code: String, message: String },
    /// Service not found
    NotFound,
    /// Service temporarily unavailable
    Unavailable,
    /// Request timeout
    Timeout,
}

/// Universal service mesh client trait - works with any service mesh primal
#[async_trait]
pub trait UniversalServiceMesh: Send + Sync {
    /// Discover available service mesh primals in the ecosystem
    async fn discover_service_meshes(&self) -> BearDogResult<Vec<ServiceMeshInfo>>;

    /// Select and connect to the best available service mesh
    async fn connect_to_best_mesh(&self) -> BearDogResult<ServiceMeshInfo>;

    /// Register BearDog with active service mesh
    async fn register(
        &self,
        metadata: &PrimalMetadata,
        services: &[PrimalService],
    ) -> BearDogResult<RegistrationInfo>;

    /// Update BearDog's service catalog
    async fn update_services(&self, services: &[PrimalService]) -> BearDogResult<()>;

    /// Discover services by capability across the mesh
    async fn discover_services(&self, capability: &str) -> BearDogResult<Vec<DiscoveredService>>;

    /// Send service request through mesh routing
    async fn send_service_request(&self, request: ServiceRequest)
        -> BearDogResult<ServiceResponse>;

    /// Report health status to active service mesh
    async fn report_health(&self, health: ServiceHealth) -> BearDogResult<()>;

    /// Deregister from active service mesh
    async fn deregister(&self) -> BearDogResult<()>;

    /// Get current registration info
    async fn get_registration(&self) -> Option<RegistrationInfo>;

    /// Get information about the active service mesh
    async fn get_active_mesh(&self) -> Option<ServiceMeshInfo>;

    /// Failover to alternative service mesh if current one fails
    async fn failover_to_alternative(&self) -> BearDogResult<ServiceMeshInfo>;
}

impl UniversalServiceMeshClient {
    /// Create a new universal service mesh client
    pub fn new() -> BearDogResult<Self> {
        let client = HttpClient::builder()
            .timeout(Duration::from_secs(30))
            .user_agent("BearDog/1.0")
            .build()
            .map_err(|e| BearDogError::internal(&format!("Failed to create HTTP client: {}", e)))?;

        Ok(Self {
            client,
            active_mesh: Arc::new(RwLock::new(None)),
            registration: Arc::new(RwLock::new(None)),
            service_cache: Arc::new(RwLock::new(HashMap::new())),
            available_meshes: Arc::new(RwLock::new(Vec::new())),
            timeout: Duration::from_secs(30),
        })
    }

    /// Create client with custom timeout
    pub fn with_timeout(timeout: Duration) -> BearDogResult<Self> {
        let mut client = Self::new()?;
        client.timeout = timeout;
        Ok(client)
    }

    /// Get API URL for current active mesh
    async fn api_url(&self, path: &str) -> BearDogResult<String> {
        let mesh = self.active_mesh.read().await;
        let mesh_info = mesh
            .as_ref()
            .ok_or_else(|| BearDogError::internal("No active service mesh"))?;
        Ok(format!("{}/api/v1/{}", mesh_info.endpoint, path))
    }

    /// Create request context for Songbird calls
    fn create_context(&self, operation: &str) -> ServiceContext {
        ServiceContext {
            request_id: Uuid::new_v4(),
            source: PrimalIdentity {
                name: "BearDog".to_string(),
                primal_type: PrimalType::BearDog,
                node_id: "beardog-node".to_string(),
            },
            security: crate::universal_primal_provider::SecurityContext {
                auth_token: None,
                client_cert: None,
                clearance_level: 5, // High security clearance for BearDog
                encryption_required: true,
            },
            metadata: {
                let mut meta = HashMap::new();
                meta.insert("operation".to_string(), operation.to_string());
                meta.insert("primal_type".to_string(), "BearDog".to_string());
                meta.insert("version".to_string(), env!("CARGO_PKG_VERSION").to_string());
                meta
            },
        }
    }
}

#[async_trait]
impl UniversalServiceMesh for UniversalServiceMeshClient {
    /// Discover available service mesh primals in the ecosystem
    async fn discover_service_meshes(&self) -> BearDogResult<Vec<ServiceMeshInfo>> {
        info!("🔍 Discovering available service mesh primals in ecosystem");

        // Universal discovery request - look for primals with service mesh capabilities
        let discovery_request = serde_json::json!({
            "capability_type": "service_mesh",
            "required_capabilities": [
                "service_discovery",
                "request_routing",
                "health_monitoring"
            ],
            "primal_types": ["songbird", "universal_mesh", "custom_mesh"],
            "exclude_self": true,
            "include_health": true
        });

        // Use broadcast discovery to find service meshes
        // This would typically use UDP multicast or known discovery endpoints
        let mut discovered_meshes = Vec::new();

        // Try common service mesh discovery patterns
        let discovery_endpoints = vec![
            "http://localhost:3000",    // Songbird default
            "http://localhost:8080",    // Alternative port
            "http://service-mesh:3000", // Container name
            "https://mesh.local",       // Local mesh
        ];

        for endpoint in discovery_endpoints {
            match self.probe_service_mesh(endpoint).await {
                Ok(mesh_info) => {
                    info!(
                        "✅ Discovered service mesh: {} at {}",
                        mesh_info.name, endpoint
                    );
                    discovered_meshes.push(mesh_info);
                }
                Err(e) => {
                    debug!("⚠️ No service mesh at {}: {}", endpoint, e);
                }
            }
        }

        // Update available meshes cache
        {
            let mut meshes = self.available_meshes.write().await;
            *meshes = discovered_meshes.clone();
        }

        info!("🌍 Found {} service mesh primals", discovered_meshes.len());
        Ok(discovered_meshes)
    }

    /// Select and connect to the best available service mesh
    async fn connect_to_best_mesh(&self) -> BearDogResult<ServiceMeshInfo> {
        // Discover available meshes if we haven't already
        let meshes = {
            let current_meshes = self.available_meshes.read().await;
            if current_meshes.is_empty() {
                drop(current_meshes);
                self.discover_service_meshes().await?
            } else {
                current_meshes.clone()
            }
        };

        if meshes.is_empty() {
            return Err(BearDogError::internal(
                "No service mesh primals found in ecosystem",
            ));
        }

        // Select the best mesh based on priority score and health
        let best_mesh = meshes
            .into_iter()
            .filter(|m| m.health == ServiceHealth::Healthy)
            .max_by_key(|m| m.priority_score)
            .ok_or_else(|| BearDogError::internal("No healthy service mesh available"))?;

        // Set as active mesh
        {
            let mut active = self.active_mesh.write().await;
            *active = Some(best_mesh.clone());
        }

        info!(
            "🎯 Selected service mesh: {} (priority: {})",
            best_mesh.name, best_mesh.priority_score
        );
        Ok(best_mesh)
    }

    /// Register BearDog with active service mesh
    async fn register(
        &self,
        metadata: &PrimalMetadata,
        services: &[PrimalService],
    ) -> BearDogResult<RegistrationInfo> {
        // Ensure we have an active service mesh
        if self.active_mesh.read().await.is_none() {
            self.connect_to_best_mesh().await?;
        }

        let mesh_info = self
            .active_mesh
            .read()
            .await
            .clone()
            .ok_or_else(|| BearDogError::internal("No active service mesh"))?;

        info!(
            "📝 Registering BearDog with {} at {}",
            mesh_info.name, mesh_info.endpoint
        );

        let registration_request = serde_json::json!({
            "primal_metadata": metadata,
            "services": services,
            "registration_type": "primary_security_provider",
            "capabilities": [
                "security.encryption",
                "security.authentication",
                "security.authorization",
                "security.threat_detection",
                "security.compliance",
                "security.key_management"
            ],
            "health": ServiceHealth::Healthy,
            "context": self.create_context("register")
        });

        let api_url = self.api_url("primals/register").await?;
        let response = self
            .client
            .post(&api_url)
            .timeout(self.timeout)
            .header("Content-Type", "application/json")
            .header("X-Primal-Type", "BearDog")
            .header("X-Registration-Type", "primary_security_provider")
            .header("X-Service-Mesh", &mesh_info.name)
            .json(&registration_request)
            .send()
            .await
            .map_err(|e| {
                BearDogError::internal(&format!("Failed to send registration request: {}", e))
            })?;

        if response.status().is_success() {
            let registration_response: serde_json::Value = response.json().await.map_err(|e| {
                BearDogError::internal(&format!("Failed to parse registration response: {}", e))
            })?;

            let registration_info = RegistrationInfo {
                registration_id: registration_response
                    .get("registration_id")
                    .and_then(|v| v.as_str())
                    .unwrap_or_else(|| "unknown")
                    .to_string(),
                node_id: format!("beardog-{}", Uuid::new_v4()),
                service_mesh_name: mesh_info.name.clone(),
                registered_at: chrono::Utc::now(),
                status: RegistrationStatus::Active,
            };

            // Store registration info
            let mut reg = self.registration.write().await;
            *reg = Some(registration_info.clone());

            info!(
                "✅ Successfully registered BearDog with {} (ID: {})",
                mesh_info.name, registration_info.registration_id
            );
            Ok(registration_info)
        } else {
            let error_text = response
                .text()
                .await
                .unwrap_or_else(|_| "Unknown error".to_string());
            error!("Failed to register with Songbird: {}", error_text);
            Err(BearDogError::internal(&format!(
                "Registration failed: {}",
                error_text
            )))
        }
    }

    /// Update BearDog's service catalog
    async fn update_services(&self, services: &[PrimalService]) -> BearDogResult<()> {
        let registration = self.registration.read().await;
        let reg_info = registration
            .as_ref()
            .ok_or_else(|| BearDogError::internal("Not registered with Songbird"))?;

        debug!(
            "Updating BearDog service catalog with {} services",
            services.len()
        );

        let update_request = serde_json::json!({
            "registration_id": reg_info.registration_id,
            "services": services,
            "context": self.create_context("update_services")
        });

        let response = self
            .client
            .put(&self.api_url("primals/services").await?)
            .timeout(self.timeout)
            .header("Content-Type", "application/json")
            .header("X-Registration-Id", &reg_info.registration_id)
            .json(&update_request)
            .send()
            .await
            .map_err(|e| BearDogError::internal(&format!("Failed to update services: {}", e)))?;

        if response.status().is_success() {
            info!("Successfully updated BearDog service catalog");
            Ok(())
        } else {
            let error_text = response
                .text()
                .await
                .unwrap_or_else(|_| "Unknown error".to_string());
            Err(BearDogError::internal(&format!(
                "Failed to update services: {}",
                error_text
            )))
        }
    }

    /// Discover services by capability
    async fn discover_services(&self, capability: &str) -> BearDogResult<Vec<DiscoveredService>> {
        debug!("Discovering services with capability: {}", capability);

        // Check cache first
        {
            let cache = self.service_cache.read().await;
            if let Some(cached_services) = cache.get(capability) {
                // Return cached results if they're fresh (less than 5 minutes old)
                let now = chrono::Utc::now();
                if cached_services
                    .iter()
                    .any(|s| now.signed_duration_since(s.last_seen).num_minutes() < 5)
                {
                    debug!("Returning cached services for capability: {}", capability);
                    return Ok(cached_services.clone());
                }
            }
        }

        let discovery_request = serde_json::json!({
            "capability": capability,
            "exclude_self": true,
            "include_health": true,
            "context": self.create_context("discover_services")
        });

        let response = self
            .client
            .post(&self.api_url("discovery/services").await?)
            .timeout(self.timeout)
            .header("Content-Type", "application/json")
            .json(&discovery_request)
            .send()
            .await
            .map_err(|e| BearDogError::internal(&format!("Failed to discover services: {}", e)))?;

        if response.status().is_success() {
            let discovery_response: serde_json::Value = response.json().await.map_err(|e| {
                BearDogError::internal(&format!("Failed to parse discovery response: {}", e))
            })?;

            let services: Vec<DiscoveredService> = discovery_response
                .get("services")
                .and_then(|v| v.as_array())
                .map(|arr| {
                    arr.iter()
                        .filter_map(|service| serde_json::from_value(service.clone()).ok())
                        .collect()
                })
                .unwrap_or_default();

            // Update cache
            {
                let mut cache = self.service_cache.write().await;
                cache.insert(capability.to_string(), services.clone());
            }

            info!(
                "Discovered {} services with capability: {}",
                services.len(),
                capability
            );
            Ok(services)
        } else {
            let error_text = response
                .text()
                .await
                .unwrap_or_else(|_| "Unknown error".to_string());
            warn!("Failed to discover services: {}", error_text);
            Ok(Vec::new()) // Return empty vec instead of error for graceful degradation
        }
    }

    /// Send service request through Songbird routing
    async fn send_service_request(
        &self,
        request: ServiceRequest,
    ) -> BearDogResult<ServiceResponse> {
        debug!(
            "Sending service request {} to {} via Songbird",
            request.request_id, request.target_primal
        );

        let routing_request = serde_json::json!({
            "request": request,
            "routing": {
                "strategy": "fastest_available",
                "timeout_ms": 30000,
                "retry_count": 2
            },
            "context": self.create_context("route_request")
        });

        let start_time = std::time::Instant::now();

        let response = self
            .client
            .post(&self.api_url("routing/request").await?)
            .timeout(self.timeout)
            .header("Content-Type", "application/json")
            .header("X-Request-Id", &request.request_id.to_string())
            .json(&routing_request)
            .send()
            .await
            .map_err(|e| {
                BearDogError::internal(&format!("Failed to send service request: {}", e))
            })?;

        let processing_time = start_time.elapsed().as_millis() as u64;

        if response.status().is_success() {
            let response_data: serde_json::Value = response.json().await.map_err(|e| {
                BearDogError::internal(&format!("Failed to parse service response: {}", e))
            })?;

            let service_response = ServiceResponse {
                request_id: request.request_id,
                status: ResponseStatus::Success,
                payload: response_data
                    .get("payload")
                    .cloned()
                    .unwrap_or(serde_json::Value::Null),
                metadata: response_data
                    .get("metadata")
                    .and_then(|v| serde_json::from_value(v.clone()).ok())
                    .unwrap_or_default(),
                processing_time_ms: processing_time,
            };

            debug!(
                "Service request completed successfully in {}ms",
                processing_time
            );
            Ok(service_response)
        } else {
            // Get status before consuming response with .text()
            let status_code = response.status().to_string();
            let error_text = response
                .text()
                .await
                .unwrap_or_else(|_| "Unknown error".to_string());

            let service_response = ServiceResponse {
                request_id: request.request_id,
                status: ResponseStatus::Error {
                    code: status_code,
                    message: error_text,
                },
                payload: serde_json::Value::Null,
                metadata: HashMap::new(),
                processing_time_ms: processing_time,
            };

            warn!("Service request failed: {:?}", service_response.status);
            Ok(service_response) // Return response instead of error for proper handling
        }
    }

    /// Report health status to Songbird
    async fn report_health(&self, health: ServiceHealth) -> BearDogResult<()> {
        let registration = self.registration.read().await;
        let reg_info = registration
            .as_ref()
            .ok_or_else(|| BearDogError::internal("Not registered with Songbird"))?;

        let health_request = serde_json::json!({
            "registration_id": reg_info.registration_id,
            "health": health,
            "timestamp": chrono::Utc::now(),
            "context": self.create_context("report_health")
        });

        let response = self
            .client
            .post(&self.api_url("health/report").await?)
            .timeout(Duration::from_secs(10)) // Shorter timeout for health reports
            .header("Content-Type", "application/json")
            .header("X-Registration-Id", &reg_info.registration_id)
            .json(&health_request)
            .send()
            .await
            .map_err(|e| BearDogError::internal(&format!("Failed to report health: {}", e)))?;

        if response.status().is_success() {
            debug!("Successfully reported health status: {:?}", health);
            Ok(())
        } else {
            let error_text = response
                .text()
                .await
                .unwrap_or_else(|_| "Unknown error".to_string());
            warn!("Failed to report health: {}", error_text);
            // Don't return error for health reports - just log it
            Ok(())
        }
    }

    /// Deregister from Songbird service mesh
    async fn deregister(&self) -> BearDogResult<()> {
        let registration = self.registration.read().await;
        let reg_info = registration
            .as_ref()
            .ok_or_else(|| BearDogError::internal("Not registered with Songbird"))?;

        info!("Deregistering BearDog from Songbird");

        let deregister_request = serde_json::json!({
            "registration_id": reg_info.registration_id,
            "reason": "graceful_shutdown",
            "context": self.create_context("deregister")
        });

        let response = self
            .client
            .delete(&self.api_url(&format!("primals/{}", reg_info.registration_id)).await?)
            .timeout(self.timeout)
            .header("Content-Type", "application/json")
            .header("X-Registration-Id", &reg_info.registration_id)
            .json(&deregister_request)
            .send()
            .await
            .map_err(|e| BearDogError::internal(&format!("Failed to deregister: {}", e)))?;

        if response.status().is_success() {
            // Clear registration
            let mut reg = self.registration.write().await;
            if let Some(mut info) = reg.take() {
                info.status = RegistrationStatus::Deregistered;
                *reg = Some(info);
            }

            info!("Successfully deregistered BearDog from Songbird");
            Ok(())
        } else {
            let error_text = response
                .text()
                .await
                .unwrap_or_else(|_| "Unknown error".to_string());
            warn!("Failed to deregister from Songbird: {}", error_text);
            // Don't return error - we're shutting down anyway
            Ok(())
        }
    }

    /// Get current registration info
    async fn get_registration(&self) -> Option<RegistrationInfo> {
        self.registration.read().await.clone()
    }

    /// Get information about the active service mesh
    async fn get_active_mesh(&self) -> Option<ServiceMeshInfo> {
        self.active_mesh.read().await.clone()
    }

    /// Failover to alternative service mesh if current one fails
    async fn failover_to_alternative(&self) -> BearDogResult<ServiceMeshInfo> {
        warn!("🔄 Initiating service mesh failover");

        // Clear current active mesh
        {
            let mut active = self.active_mesh.write().await;
            *active = None;
        }

        // Clear registration as it's tied to the failed mesh
        {
            let mut reg = self.registration.write().await;
            *reg = None;
        }

        // Discover and connect to alternative mesh
        let new_mesh = self.connect_to_best_mesh().await?;

        info!(
            "✅ Failed over to alternative service mesh: {}",
            new_mesh.name
        );
        Ok(new_mesh)
    }
}

impl UniversalServiceMeshClient {
    /// Probe a potential service mesh endpoint to see if it's available
    async fn probe_service_mesh(&self, endpoint: &str) -> BearDogResult<ServiceMeshInfo> {
        let probe_url = format!("{}/api/v1/mesh/info", endpoint);

        let response = self
            .client
            .get(&probe_url)
            .timeout(Duration::from_secs(5)) // Short timeout for probing
            .header("X-Probe-Source", "BearDog")
            .send()
            .await
            .map_err(|e| BearDogError::internal(&format!("Probe failed: {}", e)))?;

        if response.status().is_success() {
            let mesh_info: serde_json::Value = response.json().await.map_err(|e| {
                BearDogError::internal(&format!("Failed to parse probe response: {}", e))
            })?;

            let service_mesh = ServiceMeshInfo {
                name: mesh_info
                    .get("name")
                    .and_then(|v| v.as_str())
                    .unwrap_or("unknown")
                    .to_string(),
                endpoint: endpoint.to_string(),
                capabilities: mesh_info
                    .get("capabilities")
                    .and_then(|v| v.as_array())
                    .map(|arr| {
                        arr.iter()
                            .filter_map(|s| s.as_str().map(|s| s.to_string()))
                            .collect()
                    })
                    .unwrap_or_default(),
                api_version: mesh_info
                    .get("api_version")
                    .and_then(|v| v.as_str())
                    .unwrap_or("v1")
                    .to_string(),
                health: ServiceHealth::Healthy,
                priority_score: self.calculate_mesh_priority(&mesh_info),
                last_health_check: chrono::Utc::now(),
            };

            Ok(service_mesh)
        } else {
            Err(BearDogError::internal(&format!(
                "Service mesh probe returned status: {}",
                response.status()
            )))
        }
    }

    /// Calculate priority score for a service mesh based on its capabilities
    fn calculate_mesh_priority(&self, mesh_info: &serde_json::Value) -> u32 {
        let mut score = 100; // Base score

        // Prefer Songbird as it's our primary mesh (for now)
        if mesh_info.get("name").and_then(|v| v.as_str()) == Some("Songbird") {
            score += 50;
        }

        // Add points for supported capabilities
        if let Some(capabilities) = mesh_info.get("capabilities").and_then(|v| v.as_array()) {
            for capability in capabilities {
                if let Some(cap_str) = capability.as_str() {
                    match cap_str {
                        "load_balancing" => score += 20,
                        "circuit_breaking" => score += 15,
                        "distributed_tracing" => score += 10,
                        "metrics_collection" => score += 10,
                        "security_policies" => score += 25,
                        _ => score += 5,
                    }
                }
            }
        }

        // Consider API version
        if mesh_info.get("api_version").and_then(|v| v.as_str()) >= Some("v2") {
            score += 10;
        }

        score
    }
}

impl Default for RegistrationStatus {
    fn default() -> Self {
        Self::Pending
    }
}

impl Default for ServiceEndpoint {
    fn default() -> Self {
        Self {
            protocol: "https".to_string(),
            host: "localhost".to_string(),
            port: 443,
            path: "/".to_string(),
        }
    }
}
