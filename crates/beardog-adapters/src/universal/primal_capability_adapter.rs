// Universal Primal Capability Adapter
//
// This adapter eliminates all hardcoded primal names (songbird, toadstool, squirrel, nestgate, biomeOS)
// and instead discovers and interacts with primals through their capabilities.
// Each primal only knows itself and discovers others through universal adapter patterns.

use beardog_errors::{BearDogError, BearDogResult};
use beardog_types::canonical::discovery::{
    ComputeAbility, NetworkFunction, OrchestrationFeature, PerformanceRequirements,
    SecurityRequirements, SecurityService, StorageCharacteristic, UniversalCapabilityType,
    UniversalDiscoveryRequest, UniversalServiceDescriptor,
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
    ) -> BearDogResult<Vec<UniversalServiceDescriptor>>;

    /// Send request to primal with capability
    fn send_request(
        &self,
        service: &UniversalServiceDescriptor,
        request: PrimalRequest,
    ) -> BearDogResult<PrimalResponse>;
}

impl UniversalPrimalAdapter {
    /// Create new universal primal adapter
    /// Creates a new instance
    pub async fn new(
        discovery_client: Arc<dyn PrimalDiscoveryClient + Send + Sync>,
    ) -> BearDogResult<Self> {
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
    pub fn discover_compute_primals(&self) -> BearDogResult<Vec<UniversalServiceDescriptor>> {
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
    pub fn discover_network_primals(&self) -> BearDogResult<Vec<UniversalServiceDescriptor>> {
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
    pub fn discover_storage_primals(&self) -> BearDogResult<Vec<UniversalServiceDescriptor>> {
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
    ) -> BearDogResult<Vec<UniversalServiceDescriptor>> {
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
    ) -> BearDogResult<PrimalResponse> {
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
    ) -> BearDogResult<PrimalResponse> {
        let compute_capability = UniversalCapabilityType::Compute {
            abilities: vec![ComputeAbility::DataAnalysis],
        };

        self.send_capability_request(compute_capability, data)
    }

    /// Request network routing (replaces direct songbird calls)
    pub fn request_network_routing(
        &self,
        routing_config: serde_json::Value,
    ) -> BearDogResult<PrimalResponse> {
        let network_capability = UniversalCapabilityType::Network {
            functions: vec![NetworkFunction::TrafficRouting],
        };

        self.send_capability_request(network_capability, routing_config)
    }

    /// Request data storage (replaces direct nestgate calls)
    pub fn request_data_storage(
        &self,
        storage_request: serde_json::Value,
    ) -> BearDogResult<PrimalResponse> {
        let storage_capability = UniversalCapabilityType::Storage {
            characteristics: vec![StorageCharacteristic::Persistent],
        };

        self.send_capability_request(storage_capability, storage_request)
    }

    /// Request orchestration (replaces direct biomeOS calls)
    pub fn request_orchestration(
        &self,
        orchestration_request: serde_json::Value,
    ) -> BearDogResult<PrimalResponse> {
        let orchestration_capability = UniversalCapabilityType::Orchestration {
            features: vec![OrchestrationFeature::ServiceDeployment],
        };

        self.send_capability_request(orchestration_capability, orchestration_request)
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
    ) -> BearDogResult<Vec<UniversalServiceDescriptor>> {
        debug!(
            "🔍 Discovering primals with capabilities: {:?}",
            capabilities
        );

        // This would use the universal infant discovery system
        // For now, return mock services for demonstration
        let mut services = Vec::new();

        for capability in capabilities {
            let service = UniversalServiceDescriptor {
                service_id: format!("primal_{}", uuid::Uuid::new_v4()),
                capabilities: vec![capability],
                endpoint: beardog_types::canonical::discovery::ServiceEndpoint {
                    protocol: "http".to_string(),
                    host: "localhost".to_string(),
                    port: 8080,
                    path: Some("/api/v1".to_string()),
                    parameters: HashMap::new(),
                },
                auth_method: beardog_types::canonical::discovery::AuthenticationMethod::None,
                performance_profile:
                    beardog_types::canonical::discovery::PerformanceProfile::default(),
                trust_score: 0.8,
            };
            services.push(service);
        }

        Ok(services)
    }


    fn send_request(
        &self,
        service: &UniversalServiceDescriptor,
        request: PrimalRequest,
    ) -> BearDogResult<PrimalResponse> {
        debug!("📤 Sending request to primal: {}", service.service_id);

        // This would send actual HTTP/gRPC request to the primal
        // For now, return mock response
        Ok(PrimalResponse {
            success: true,
            payload: serde_json::json!({
                "result": "mock_response",
                "capability": format!("{:?}", request.required_capability)
            }),
            metadata: HashMap::new(),
            processing_time_ms: 100,
            service_id: service.service_id.clone(),
        })
    }
}
