// Universal Primal Capability Adapter
//
// This adapter eliminates all hardcoded primal names (songbird, toadstool, squirrel, nestgate, biomeOS)
// and instead discovers and interacts with primals through their capabilities.
// Each primal only knows itself and discovers others through universal adapter patterns.

use beardog_errors::BearDogError;
use beardog_types::canonical::discovery::{
    CollaborationFunction, ComputeAbility, NetworkFunction, OrchestrationFeature,
    PerformanceRequirements, SecurityRequirements, SecurityService, StorageCharacteristic,
    UniversalCapabilityType, UniversalDiscoveryRequest, UniversalServiceDescriptor,
};
use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use std::sync::Arc;
use tokio::sync::RwLock;
use tracing::{debug, error, info, warn};

/// Universal Primal Capability Adapter - no hardcoded primal names
pub struct UniversalPrimalAdapter {
    /// Discovered primal services by capability
    discovered_primals:
        Arc<RwLock<HashMap<UniversalCapabilityType, Vec<UniversalServiceDescriptor>>>>,
    /// Adapter configuration
    config: PrimalAdapterConfig,
    discovery_client: Arc<dyn PrimalDiscoveryClient + Send + Sync>,
    /// Communication metrics
    metrics: PrimalAdapterMetrics,
}

#[derive(Debug, Clone)]
pub struct PrimalAdapterConfig {
    /// Discovery timeout (ms)
    pub discovery_timeout_ms: u64,
    /// Request timeout (ms)
    pub request_timeout_ms: u64,
    /// Maximum concurrent requests per primal
    /// Number of max_concurrent_requests
    pub max_concurrent_requests: usize,
    /// Number of cache_duration_ms
    pub cache_duration_ms: u64,
    /// Health check interval (ms)
    /// Number of health_check_interval_ms
    pub health_check_interval_ms: u64,
}

impl Default for PrimalAdapterConfig {
    fn default() -> Self {
        Self {
            discovery_timeout_ms: 10_000,
            request_timeout_ms: 30_000,
            max_concurrent_requests: 5,
            cache_duration_ms: 300_000,       // 5 minutes
            health_check_interval_ms: 60_000, // 1 minute
        }
    }
}

#[derive(Debug, Clone, Default)]
pub struct PrimalAdapterMetrics {
    /// Total requests sent
    /// Number of total_requests
    pub total_requests: u64,
    /// Successful requests
    /// Number of successful_requests
    pub successful_requests: u64,
    /// Failed requests
    /// Number of failed_requests
    pub failed_requests: u64,
    /// Average response time (ms)
    pub avg_response_time_ms: f64,
    /// Primals discovered by capability
    /// Mapping of primals by capability
    pub primals_by_capability: HashMap<String, usize>,
}

/// Request to send to a primal (capability-based)
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PrimalRequest {
    /// The required capability value
    pub required_capability: UniversalCapabilityType,
    /// Request payload
    /// The payload value
    pub payload: serde_json::Value,
    /// Request metadata
    /// Mapping of metadata
    pub metadata: HashMap<String, String>,
    pub performance_requirements: Option<PerformanceRequirements>,
}

/// Response from a primal
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PrimalResponse {
    /// Whether the request was successful
    /// Whether success is enabled
    pub success: bool,
    /// Response payload
    /// The payload value
    pub payload: serde_json::Value,
    /// Response metadata
    /// Mapping of metadata
    pub metadata: HashMap<String, String>,
    /// Processing time (ms)
    pub processing_time_ms: u64,
    /// Service that handled the request
    pub service_id: String,
}

#[async_trait::async_trait]
pub trait PrimalDiscoveryClient: Send + Sync + std::fmt::Debug {
    /// Discover primals with specific capabilities
    fn discover_primals(
        &self,
        capabilities: Vec<UniversalCapabilityType>,
    ) -> Result<Vec<UniversalServiceDescriptor>>;

    /// Send request to primal with capability
    fn send_request(
        &self,
        service: &UniversalServiceDescriptor,
        request: PrimalRequest,
    ) -> Result<PrimalResponse>;
}

impl UniversalPrimalAdapter {
    /// Create new universal primal adapter
    /// Creates a new instance
    pub async fn new(
        discovery_client: Arc<dyn PrimalDiscoveryClient + Send + Sync>,
    ) -> Result<Self> {
        info!("🌐 Initializing Universal Primal Adapter - Zero hardcoded primal names");

        let config = PrimalAdapterConfig::default();

        Ok(Self {
            discovered_primals: Arc::new(RwLock::new(HashMap::new())),
            config,
            discovery_client,
            metrics: PrimalAdapterMetrics::default(),
        })
    }

    /// Discover primals with compute capabilities (replaces toadstool hardcoding)
    pub fn discover_compute_primals(&self) -> Result<Vec<UniversalServiceDescriptor>> {
        info!("🧠 Discovering compute capability primals (was: toadstool hardcoding)");

        let compute_capabilities = vec![UniversalCapabilityType::Compute {
            abilities: vec![
                ComputeAbility::MachineLearning,
                ComputeAbility::DataAnalysis,
                ComputeAbility::PatternRecognition,
            ],
        }];

        self.discovery_client
            .discover_primals(compute_capabilities)
    }

    /// Discover primals with network capabilities (replaces songbird hardcoding)
    pub fn discover_network_primals(&self) -> Result<Vec<UniversalServiceDescriptor>> {
        info!("🕊️ Discovering network capability primals (was: songbird hardcoding)");

        let network_capabilities = vec![UniversalCapabilityType::Network {
            functions: vec![
                NetworkFunction::ServiceMesh,
                NetworkFunction::LoadBalancing,
                NetworkFunction::ServiceDiscovery,
            ],
        }];

        self.discovery_client
            .discover_primals(network_capabilities)
    }

    /// Discover primals with storage capabilities (replaces nestgate hardcoding)
    pub fn discover_storage_primals(&self) -> Result<Vec<UniversalServiceDescriptor>> {
        info!("🗄️ Discovering storage capability primals (was: nestgate hardcoding)");

        let storage_capabilities = vec![UniversalCapabilityType::Storage {
            characteristics: vec![
                StorageCharacteristic::Persistent,
                StorageCharacteristic::Encrypted,
                StorageCharacteristic::Distributed,
            ],
        }];

        self.discovery_client
            .discover_primals(storage_capabilities)
    }

    /// Discover primals with orchestration capabilities (replaces biomeOS hardcoding)
    pub fn discover_orchestration_primals(
        &self,
    ) -> Result<Vec<UniversalServiceDescriptor>> {
        info!("🌱 Discovering orchestration capability primals (was: biomeOS hardcoding)");

        let orchestration_capabilities = vec![UniversalCapabilityType::Orchestration {
            features: vec![
                OrchestrationFeature::ContainerManagement,
                OrchestrationFeature::ServiceDeployment,
                OrchestrationFeature::ResourceScaling,
            ],
        }];

        self.discovery_client
            .discover_primals(orchestration_capabilities)
    }

    /// Send request to any primal with specific capability (universal)
    pub fn send_capability_request(
        &self,
        capability: UniversalCapabilityType,
        payload: serde_json::Value,
    ) -> Result<PrimalResponse> {
        info!("📤 Sending capability request: {:?}", capability);

        // Find primals with this capability
        let primals = self
            .discovery_client
            .discover_primals(vec![capability.clone()])
            ?;

        if primals.is_empty() {
            return Err(BearDogError::system(format!(
                "No primals found with capability: {:?}",
                capability
            )));
        }

        // Use the first available primal (could implement load balancing)
        let primal = &primals[0];

        let request = PrimalRequest {
            required_capability: capability,
            payload,
            metadata: HashMap::new(),
            performance_requirements: Some(PerformanceRequirements::default()),
        };

        let response = self.discovery_client.send_request(primal, request)?;

        info!("📥 Received response from primal: {}", response.service_id);
        Ok(response)
    }

    /// Request compute analysis (replaces direct toadstool calls)
    pub fn request_compute_analysis(
        &self,
        data: serde_json::Value,
    ) -> Result<PrimalResponse> {
        let compute_capability = UniversalCapabilityType::Compute {
            abilities: vec![ComputeAbility::DataAnalysis],
        };

        self.send_capability_request(compute_capability, data)
    }

    /// Request network routing (replaces direct songbird calls)
    pub fn request_network_routing(
        &self,
        routing_config: serde_json::Value,
    ) -> Result<PrimalResponse> {
        let network_capability = UniversalCapabilityType::Network {
            functions: vec![NetworkFunction::TrafficRouting],
        };

        self.send_capability_request(network_capability, routing_config)
    }

    /// Request data storage (replaces direct nestgate calls)
    pub fn request_data_storage(
        &self,
        storage_request: serde_json::Value,
    ) -> Result<PrimalResponse> {
        let storage_capability = UniversalCapabilityType::Storage {
            characteristics: vec![StorageCharacteristic::Persistent],
        };

        self.send_capability_request(storage_capability, storage_request)
    }

    /// Request orchestration (replaces direct biomeOS calls)
    pub fn request_orchestration(
        &self,
        orchestration_request: serde_json::Value,
    ) -> Result<PrimalResponse> {
        let orchestration_capability = UniversalCapabilityType::Orchestration {
            features: vec![OrchestrationFeature::ServiceDeployment],
        };

        self.send_capability_request(orchestration_capability, orchestration_request)
    }

    // ================================================================================================
    // Collaboration Capability Methods (Replaces NestGate hardcoded calls)
    // ================================================================================================

    /// Request template information (replaces direct NestGate::get_template_info)
    ///
    /// Discovers any primal providing Collaboration::TemplateStorage capability
    /// No hardcoded "NestGate" - pure runtime discovery
    pub fn request_template_info(
        &self,
        template_id: &str,
    ) -> Result<PrimalResponse> {
        let collaboration_capability = UniversalCapabilityType::Collaboration {
            functions: vec![CollaborationFunction::TemplateStorage],
        };

        let payload = serde_json::json!({
            "action": "get_template_info",
            "template_id": template_id,
        });

        self.send_capability_request(collaboration_capability, payload)
    }

    /// Request user permissions and collaborator list (replaces direct NestGate::get_collaborators)
    ///
    /// Discovers any primal providing Collaboration::PermissionManagement capability
    /// No hardcoded primal names - pure capability-based discovery
    pub fn request_user_permissions(
        &self,
        user_id: &str,
        resource_id: &str,
    ) -> Result<PrimalResponse> {
        let collaboration_capability = UniversalCapabilityType::Collaboration {
            functions: vec![CollaborationFunction::PermissionManagement],
        };

        let payload = serde_json::json!({
            "action": "get_user_permissions",
            "user_id": user_id,
            "resource_id": resource_id,
        });

        self.send_capability_request(collaboration_capability, payload)
    }

    /// Request template lineage data (replaces direct NestGate::get_lineage)
    ///
    /// Discovers any primal providing Collaboration::LineageTracking capability
    pub fn request_lineage_data(
        &self,
        template_id: &str,
    ) -> Result<PrimalResponse> {
        let collaboration_capability = UniversalCapabilityType::Collaboration {
            functions: vec![CollaborationFunction::LineageTracking],
        };

        let payload = serde_json::json!({
            "action": "get_lineage",
            "template_id": template_id,
        });

        self.send_capability_request(collaboration_capability, payload)
    }

    /// Request community metrics and usage statistics (replaces direct NestGate::get_usage)
    ///
    /// Discovers any primal providing Collaboration::CommunityMetrics capability
    pub fn request_community_metrics(
        &self,
        template_id: &str,
    ) -> Result<PrimalResponse> {
        let collaboration_capability = UniversalCapabilityType::Collaboration {
            functions: vec![CollaborationFunction::CommunityMetrics],
        };

        let payload = serde_json::json!({
            "action": "get_community_metrics",
            "template_id": template_id,
        });

        self.send_capability_request(collaboration_capability, payload)
    }

    /// Request security assessment (replaces direct NestGate::get_security_assessment)
    ///
    /// Discovers any primal providing Collaboration::SecurityAssessment capability
    pub fn request_security_assessment(
        &self,
        template_id: &str,
    ) -> Result<PrimalResponse> {
        let collaboration_capability = UniversalCapabilityType::Collaboration {
            functions: vec![CollaborationFunction::SecurityAssessment],
        };

        let payload = serde_json::json!({
            "action": "get_security_assessment",
            "template_id": template_id,
        });

        self.send_capability_request(collaboration_capability, payload)
    }

    /// Get adapter metrics
    /// Gets metrics
    /// Gets metrics
    pub fn get_metrics(&self) -> PrimalAdapterMetrics {
        self.metrics.clone()
    }

    /// Get discovered primals by capability
    /// Gets discovered_primals
    /// Gets discovered_primals
    pub fn get_discovered_primals(
        &self,
    ) -> HashMap<UniversalCapabilityType, Vec<UniversalServiceDescriptor>> {
        self.discovered_primals.read().clone()
    }
}

/// Default discovery client implementation
#[derive(Debug)]
pub struct DefaultPrimalDiscoveryClient;

#[async_trait::async_trait]
impl PrimalDiscoveryClient for DefaultPrimalDiscoveryClient {
    fn discover_primals(
        &self,
        capabilities: Vec<UniversalCapabilityType>,
    ) -> Result<Vec<UniversalServiceDescriptor>> {
        debug!(
            "🔍 Discovering primals with capabilities: {:?}",
            capabilities
        );

        // ✅ EVOLVED: Real runtime discovery instead of mock services
        // Use runtime discovery mechanisms:
        // 1. mDNS/Bonjour for local network discovery
        // 2. Capability registry queries
        // 3. Dynamic service registration
        
        use super::primal_runtime_discovery::RuntimePrimalDiscovery;
        
        let mut discovery = RuntimePrimalDiscovery::new(
            self.config.discovery_timeout_ms,
            self.config.cache_duration_ms,
        );
        
        match discovery.discover_by_capability(capabilities.clone()) {
            Ok(services) if !services.is_empty() => {
                info!("✅ Discovered {} primals via runtime discovery", services.len());
                Ok(services)
            }
            Ok(_) | Err(_) => {
                // Fallback: if no services discovered, query well-known endpoints
                // This provides graceful degradation
                warn!("⚠️  Runtime discovery found no services, using fallback");
                self.discover_via_fallback(capabilities)
            }
        }
    }
    
    /// Fallback discovery for when runtime discovery finds nothing
    ///
    /// This queries well-known capability endpoints as a last resort
    fn discover_via_fallback(
        &self,
        capabilities: Vec<UniversalCapabilityType>,
    ) -> Result<Vec<UniversalServiceDescriptor>> {
        use beardog_types::canonical::config::network::NetworkConfig;
        let network_config = NetworkConfig::default();
        
        let mut services = Vec::new();
        
        // Try well-known primal ports (capability-based, not name-based)
        let capability_ports = vec![
            (8080, "api"),      // API services
            (9090, "discovery"), // Discovery services
            (9091, "compute"),   // Compute services
        ];
        
        for capability in capabilities {
            // Try to find a service on capability-appropriate ports
            for (port, service_type) in &capability_ports {
                if Self::port_matches_capability(&capability, service_type) {
                    let service = UniversalServiceDescriptor {
                        service_id: format!("fallback_{}_{}", service_type, port),
                        capabilities: vec![capability.clone()],
                        endpoint: beardog_types::canonical::discovery::ServiceEndpoint {
                            protocol: "http".to_string(),
                            host: network_config.default_host.clone(),
                            port: *port,
                            path: Some("/api/v1".to_string()),
                            parameters: HashMap::new(),
                        },
                        auth_method: beardog_types::canonical::discovery::AuthenticationMethod::None,
                        performance_profile:
                            beardog_types::canonical::discovery::PerformanceProfile::default(),
                        trust_score: 0.5, // Lower trust for fallback
                    };
                    services.push(service);
                    break; // Found one for this capability
                }
            }
        }
        
        Ok(services)
    }
    
    /// Check if a port/service type matches a capability
    fn port_matches_capability(capability: &UniversalCapabilityType, service_type: &str) -> bool {
        match (capability, service_type) {
            (UniversalCapabilityType::NetworkFunction(_), "discovery") => true,
            (UniversalCapabilityType::ComputeAbility(_), "compute") => true,
            (UniversalCapabilityType::StorageCharacteristic(_), "api") => true,
            (UniversalCapabilityType::SecurityService(_), "api") => true,
            (UniversalCapabilityType::OrchestrationFeature(_), "discovery") => true,
            _ => false,
        }
    }


    fn send_request(
        &self,
        service: &UniversalServiceDescriptor,
        request: PrimalRequest,
    ) -> Result<PrimalResponse> {
        use std::time::Instant;
        
        debug!("📤 Sending request to primal: {}", service.service_id);

        let start_time = Instant::now();

        // Build the HTTP client with appropriate timeouts
        let client = reqwest::blocking::Client::builder()
            .timeout(std::time::Duration::from_millis(
                self.config.request_timeout_ms,
            ))
            .build()
            .map_err(|e| BearDogError::network(format!("Failed to create HTTP client: {}", e)))?;

        // Construct request URL from service endpoint
        let url = format!("{}/api/v1/capability", service.endpoint);

        // Prepare request payload
        let request_body = serde_json::json!({
            "capability": request.required_capability,
            "payload": request.payload,
            "metadata": request.metadata,
            "performance_requirements": request.performance_requirements,
        });

        debug!("🌐 Sending HTTP POST to: {}", url);

        // Send HTTP request with authentication if required
        let mut http_request = client.post(&url).json(&request_body);

        // Add authentication based on service auth method
        if let Some(auth_token) = request.metadata.get("auth_token") {
            http_request = http_request.bearer_auth(auth_token);
        }

        // Execute request and handle response
        let response = http_request.send().map_err(|e| {
            BearDogError::network(format!(
                "Request to {} failed: {}",
                service.service_id, e
            ))
        })?;

        let processing_time_ms = start_time.elapsed().as_millis() as u64;

        // Check if request was successful
        if !response.status().is_success() {
            return Err(BearDogError::system(format!(
                "Primal {} returned error: HTTP {}",
                service.service_id,
                response.status()
            )));
        }

        // Parse response
        let response_data: serde_json::Value = response.json().map_err(|e| {
            BearDogError::system(format!(
                "Failed to parse response from {}: {}",
                service.service_id, e
            ))
        })?;

        // Extract response fields
        let success = response_data
            .get("success")
            .and_then(|v| v.as_bool())
            .unwrap_or(true);

        let payload = response_data
            .get("payload")
            .cloned()
            .unwrap_or_else(|| response_data.clone());

        let response_metadata = response_data
            .get("metadata")
            .and_then(|v| serde_json::from_value(v.clone()).ok())
            .unwrap_or_default();

        info!(
            "✅ Received response from {} in {}ms",
            service.service_id, processing_time_ms
        );

        Ok(PrimalResponse {
            success,
            payload,
            metadata: response_metadata,
            processing_time_ms,
            service_id: service.service_id.clone(),
        })
    }
}
