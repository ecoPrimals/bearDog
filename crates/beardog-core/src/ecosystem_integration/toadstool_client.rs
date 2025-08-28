

use super::ecosystem_genetic_spawner::{
    EcosystemPrimalClient, GeneticTrait, TraitCategory, EcosystemCapability,
    ComputeResourceAllocation, EcosystemGeneticBlueprint,
};
use beardog_errors::BearDogError;
use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use std::time::Duration;
use tokio::time::timeout;
use tracing::{debug, info, warn, error};
use uuid::Uuid;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ToadStoolClientConfig {

    pub endpoint: String,

    pub timeout_ms: u64,

    pub auth_token: String,

    pub retry_attempts: u32,

    pub client_id: String,
}

impl Default for ToadStoolClientConfig {
    fn default() -> Self {
        Self {
            endpoint: "http://localhost:9090".to_string(),
            timeout_ms: 30000,
            auth_token: "beardog_toadstool_integration".to_string(),
            retry_attempts: 3,
            client_id: format_args!("beardog-{}", Uuid::new_v4().to_string()),
        }
    }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ToadStoolComputeGenetics {

    pub supported_architectures: Vec<ComputeArchitecture>,

    pub processing_capabilities: Vec<ProcessingCapability>,

    pub optimization_traits: Vec<OptimizationTrait>,

    pub platform_compatibility: Vec<PlatformCompatibility>,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub enum ComputeArchitecture {
    X86_64,
    ARM64,
    RISC_V,
    GPU_CUDA,
    GPU_OpenCL,
    FPGA,
    Quantum,
    Microcontroller,
    EdgeCompute,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub enum ProcessingCapability {
    HighPerformanceComputing,
    RealTimeProcessing,
    BatchProcessing,
    StreamProcessing,
    ParallelProcessing,
    DistributedComputing,
    EdgeProcessing,
    QuantumComputing,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct OptimizationTrait {
    pub optimization_type: OptimizationType,
    pub efficiency_score: f64,
    pub resource_savings: f64,
    pub performance_boost: f64,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub enum OptimizationType {
    CPUOptimization,
    MemoryOptimization,
    NetworkOptimization,
    StorageOptimization,
    PowerOptimization,
    CostOptimization,
    LatencyOptimization,
    ThroughputOptimization,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub enum PlatformCompatibility {
    Linux,
    Windows,
    MacOS,
    Android,
    iOS,
    EmbeddedSystems,
    CloudNative,
    EdgeDevices,
    IoTDevices,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ToadStoolComputeRequest {
    pub request_id: String,
    pub compute_requirements: ComputeRequirements,
    pub duration_hours: u32,
    pub priority: ComputePriority,
    pub genetic_spawning_context: Option<GeneticSpawningContext>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ComputeRequirements {
    pub cpu_cores: u32,
    pub memory_gb: u64,
    pub gpu_units: u32,
    pub storage_gb: u64,
    pub network_bandwidth_mbps: u32,
    pub required_architectures: Vec<ComputeArchitecture>,
    pub required_capabilities: Vec<ProcessingCapability>,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub enum ComputePriority {
    Low,
    Normal,
    High,
    Critical,
    Emergency,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct GeneticSpawningContext {
    pub spawning_operation_id: String,
    pub security_genetics_id: String,
    pub hybrid_capabilities: Vec<String>,
    pub cross_primal_requirements: HashMap<String, serde_json::Value>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ToadStoolComputeResponse {
    pub allocation_id: String,
    pub allocated_resources: ComputeResourceAllocation,
    pub allocation_status: AllocationStatus,
    pub estimated_ready_time: Option<chrono::DateTime<chrono::Utc>>,
    pub cost_estimate: CostEstimate,
    pub genetic_compatibility_score: f64,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub enum AllocationStatus {
    Pending,
    Allocated,
    Ready,
    InUse,
    Released,
    Failed,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CostEstimate {
    pub hourly_cost: f64,
    pub total_estimated_cost: f64,
    pub currency: String,
    pub cost_breakdown: HashMap<String, f64>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ToadStoolHybridComponent {
    pub component_id: String,
    pub component_type: HybridComponentType,
    pub compute_allocation: ComputeResourceAllocation,
    pub genetic_traits: Vec<GeneticTrait>,
    pub performance_metrics: ToadStoolPerformanceMetrics,
    pub status: ComponentStatus,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub enum HybridComponentType {
    ComputeEngine,
    ResourceOrchestrator,
    PerformanceOptimizer,
    LoadBalancer,
    TaskScheduler,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ToadStoolPerformanceMetrics {
    pub compute_utilization: f64,
    pub memory_utilization: f64,
    pub network_throughput_mbps: f64,
    pub task_completion_rate: f64,
    pub average_response_time_ms: f64,
    pub error_rate: f64,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub enum ComponentStatus {
    Initializing,
    Ready,
    Active,
    Degraded,
    Failed,
    Terminated,
}

#[derive(Debug)]
pub struct ToadStoolComputeClient {
    config: ToadStoolClientConfig,
    http_client: reqwest::Client,
    genetics_cache: tokio::sync::RwLock<Option<ToadStoolComputeGenetics>>,
    active_allocations: tokio::sync::RwLock<HashMap<String, ToadStoolComputeResponse>>,
}

impl ToadStoolComputeClient {

    pub fn new(config: ToadStoolClientConfig) -> Self {
        Self {
            config,
            http_client: reqwest::Client::builder()
                .timeout(Duration::from_millis(30000))
                .build()
                .unwrap_or_else(|_| reqwest::Client::new()),
            genetics_cache: tokio::sync::RwLock::new(None),
            active_allocations: tokio::sync::RwLock::new(ahash::HashMap::default()),
        }
    }

    pub async fn request_compute_allocation(
        &self,
        request: ToadStoolComputeRequest,
    ) -> Result<ToadStoolComputeResponse, BearDogError> {
        let allocation_url = format_args!("{}/api/v1/compute/allocate", self.config.endpoint).to_string();
        let timeout_duration = Duration::from_millis(self.config.timeout_ms);

        let payload = serde_json::json!({
            "request_id": request.request_id,
            "compute_requirements": request.compute_requirements,
            "duration_hours": request.duration_hours,
            "priority": request.priority,
            "client_id": self.config.client_id,
            "genetic_spawning_context": request.genetic_spawning_context
        });

        for attempt in 1..=self.config.retry_attempts {
            match timeout(
                timeout_duration,
                self.http_client
                    .post(&allocation_url)
                    .header("Authorization", format_args!("Bearer {}", self.config.auth_token).to_string())
                    .json(&payload)
                    .send(),
            ).await {
                Ok(Ok(response)) => {
                    if response.status().is_success() {
                        match response.json::<ToadStoolComputeResponse>().await {
                            Ok(allocation_response) => {
                                info!("🍄 ToadStool compute allocation successful: {}", allocation_response.allocation_id);

                                {
                                    let mut allocations = self.active_allocations.write().await;
                                    allocations.insert(allocation_response.allocation_id.clone(), allocation_response.clone());
                                }
                                
                                return Ok(allocation_response);
                            }
                            Err(e) => {
                                warn!("Failed to parse ToadStool allocation response (attempt {}): {}", attempt, e);
                            }
                        }
                    } else {
                        warn!("ToadStool allocation failed with status: {} (attempt {})", response.status(), attempt);
                    }
                }
                Ok(Err(e)) => {
                    warn!("HTTP error during ToadStool allocation (attempt {}): {}", attempt, e);
                }
                Err(_) => {
                    warn!("Timeout during ToadStool allocation (attempt {})", attempt);
                }
            }

            if attempt < self.config.retry_attempts {
                tokio::time::sleep(Duration::from_millis(1000 * attempt as u64)).await;
            }
        }

        Err(BearDogError::network_error(
            "Failed to allocate ToadStool compute resources after all retries".to_string(),
        ))
    }

    pub async fn release_compute_allocation(&self, allocation_id: &str) -> Result<(), BearDogError> {
        let release_url = format_args!("{}/api/v1/compute/release/{}", self.config.endpoint, allocation_id).to_string();
        let timeout_duration = Duration::from_millis(self.config.timeout_ms);

        match timeout(
            timeout_duration,
            self.http_client
                .delete(&release_url)
                .header("Authorization", format_args!("Bearer {}", self.config.auth_token).to_string())
                .send(),
        ).await {
            Ok(Ok(response)) => {
                if response.status().is_success() {
                    info!("🍄 ToadStool compute allocation released: {}", allocation_id);

                    {
                        let mut allocations = self.active_allocations.write().await;
                        allocations.remove(allocation_id);
                    }
                    
                    Ok(())
                } else {
                    Err(BearDogError::network_error(format!(
                        "Failed to release ToadStool allocation: {}",
                        response.status()
                    )))
                }
            }
            Ok(Err(e)) => Err(BearDogError::network_error(format!(
                "HTTP error releasing ToadStool allocation: {}",
                e
            ))),
            Err(_) => Err(BearDogError::network_error(
                "Timeout releasing ToadStool allocation".to_string(),
            )),
        }
    }

    pub async fn get_compute_genetics(&self) -> Result<ToadStoolComputeGenetics, BearDogError> {

        {
            let cache = self.genetics_cache.read().await;
            if let Some(genetics) = cache.as_ref() {
                return Ok(genetics.clone());
            }
        }

        let genetics_url = format_args!("{}/api/v1/genetics/compute", self.config.endpoint).to_string();
        let timeout_duration = Duration::from_millis(self.config.timeout_ms);

        match timeout(
            timeout_duration,
            self.http_client
                .get(&genetics_url)
                .header("Authorization", format_args!("Bearer {}", self.config.auth_token).to_string())
                .send(),
        ).await {
            Ok(Ok(response)) => {
                if response.status().is_success() {
                    match response.json::<ToadStoolComputeGenetics>().await {
                        Ok(genetics) => {
                            info!("🍄 Retrieved ToadStool compute genetics");

                            {
                                let mut cache = self.genetics_cache.write().await;
                                *cache = Some(genetics.clone());
                            }
                            
                            Ok(genetics)
                        }
                        Err(e) => Err(BearDogError::network_error(format!(
                            "Failed to parse ToadStool genetics: {}",
                            e
                        ))),
                    }
                } else {
                    Err(BearDogError::network_error(format!(
                        "ToadStool genetics request failed: {}",
                        response.status()
                    )))
                }
            }
            Ok(Err(e)) => Err(BearDogError::network_error(format!(
                "HTTP error getting ToadStool genetics: {}",
                e
            ))),
            Err(_) => Err(BearDogError::network_error(
                "Timeout getting ToadStool genetics".to_string(),
            )),
        }
    }

    fn convert_to_genetic_traits(&self, genetics: &ToadStoolComputeGenetics) -> Vec<GeneticTrait> {
        let mut traits = Vec::new();

        for arch in &genetics.supported_architectures {
            traits.push(GeneticTrait {
                trait_id: format_args!("compute_arch_{:?}", arch).to_string(),
                trait_name: format_args!("{:?} Architecture Support", arch).to_string(),
                category: TraitCategory::Compute,
                strength: 0.8,
                dominance: 0.7,
                required_capabilities: vec![EcosystemCapability::ComputeGenetics],
                trait_config: HashMap::from([
                    ("architecture".to_string(), serde_json::json!(arch)),
                ]),
            });
        }

        for capability in &genetics.processing_capabilities {
            traits.push(GeneticTrait {
                trait_id: format_args!("processing_{:?}", capability).to_string(),
                trait_name: format_args!("{:?} Processing", capability).to_string(),
                category: TraitCategory::Compute,
                strength: 0.9,
                dominance: 0.8,
                required_capabilities: vec![EcosystemCapability::ComputeGenetics],
                trait_config: HashMap::from([
                    ("processing_capability".to_string(), serde_json::json!(capability)),
                ]),
            });
        }

        for optimization in &genetics.optimization_traits {
            traits.push(GeneticTrait {
                trait_id: format_args!("optimization_{:?}", optimization.optimization_type).to_string(),
                trait_name: format_args!("{:?} Optimization", optimization.optimization_type).to_string(),
                category: TraitCategory::Compute,
                strength: optimization.efficiency_score,
                dominance: optimization.performance_boost,
                required_capabilities: vec![EcosystemCapability::ComputeGenetics],
                trait_config: HashMap::from([
                    ("optimization_type".to_string(), serde_json::json!(optimization.optimization_type)),
                    ("efficiency_score".to_string(), serde_json::json!(optimization.efficiency_score)),
                    ("resource_savings".to_string(), serde_json::json!(optimization.resource_savings)),
                    ("performance_boost".to_string(), serde_json::json!(optimization.performance_boost)),
                ]),
            });
        }

        traits
    }

    pub async fn get_active_allocations(&self) -> Result<Vec<ToadStoolComputeResponse>, BearDogError> {
        let allocations = self.active_allocations.read().await;
        Ok(allocations.values().cloned().collect())
    }
}

impl EcosystemPrimalClient for ToadStoolComputeClient {
    fn get_primal_id(&self) -> &str {
        "toadstool"
    }

    async fn get_genetic_traits(&self) -> Result<Vec<GeneticTrait>, BearDogError> {
        let genetics = self.get_compute_genetics().await?;
        let traits = self.convert_to_genetic_traits(&genetics);
        debug!("🍄 ToadStool provided {} genetic traits", traits.len());
        Ok(traits)
    }

    async fn allocate_resources(&self, requirements: &serde_json::Value) -> Result<serde_json::Value, BearDogError> {

        let compute_request: ToadStoolComputeRequest = serde_json::from_value(requirements.clone())
            .map_err(|e| BearDogError::invalid_input(&format_args!("Invalid ToadStool compute requirements: {}", e).to_string()))?;

        let response = self.request_compute_allocation(compute_request).await?;
        Ok(serde_json::to_value(response)?)
    }

    async fn create_hybrid_component(&self, blueprint: &EcosystemGeneticBlueprint) -> Result<serde_json::Value, BearDogError> {

        let toadstool_traits = blueprint.hybrid_traits.iter()
            .filter(|trait_| trait_.category == TraitCategory::Compute)
            .cloned()
            .collect::<Vec<_>>();

        let component = ToadStoolHybridComponent {
            component_id: Uuid::new_v4().to_string(),
            component_type: HybridComponentType::ComputeEngine,
            compute_allocation: ComputeResourceAllocation {
                cpu_cores: 4,
                memory_gb: 8,
                gpu_units: 1,
                storage_gb: 100,
                network_bandwidth_mbps: 1000,
            },
            genetic_traits: toadstool_traits,
            performance_metrics: ToadStoolPerformanceMetrics {
                compute_utilization: 0.0,
                memory_utilization: 0.0,
                network_throughput_mbps: 0.0,
                task_completion_rate: 0.0,
                average_response_time_ms: 0.0,
                error_rate: 0.0,
            },
            status: ComponentStatus::Ready,
        };

        info!("🍄 Created ToadStool hybrid component: {}", component.component_id);
        Ok(serde_json::to_value(component)?)
    }

    async fn health_check(&self) -> Result<bool, BearDogError> {
        let health_url = format_args!("{}/api/v1/health", self.config.endpoint).to_string();
        let timeout_duration = Duration::from_millis(5000); // Shorter timeout for health checks

        match timeout(
            timeout_duration,
            self.http_client.get(&health_url).send(),
        ).await {
            Ok(Ok(response)) => {
                let is_healthy = response.status().is_success();
                if is_healthy {
                    debug!("✅ ToadStool health check passed");
                } else {
                    warn!("⚠️ ToadStool health check failed: {}", response.status());
                }
                Ok(is_healthy)
            }
            Ok(Err(e)) => {
                warn!("❌ ToadStool health check HTTP error: {}", e);
                Ok(false)
            }
            Err(_) => {
                warn!("❌ ToadStool health check timeout");
                Ok(false)
            }
        }
    }

    async fn get_resource_utilization(&self) -> Result<HashMap<String, f64>, BearDogError> {
        let mut utilization = ahash::HashMap::default();

        let allocations = self.active_allocations.read().await;
        let mut total_cpu_cores = 0u32;
        let mut total_memory_gb = 0u64;
        let mut total_gpu_units = 0u32;
        
        for allocation in allocations.values() {
            total_cpu_cores += allocation.allocated_resources.cpu_cores;
            total_memory_gb += allocation.allocated_resources.memory_gb;
            total_gpu_units += allocation.allocated_resources.gpu_units;
        }
        
        utilization.insert("cpu_cores".to_string(), total_cpu_cores as f64);
        utilization.insert("memory_gb".to_string(), total_memory_gb as f64);
        utilization.insert("gpu_units".to_string(), total_gpu_units as f64);
        utilization.insert("active_allocations".to_string(), allocations.len() as f64);
        
        Ok(utilization)
    }
}

pub struct ToadStoolClientFactory;

impl ToadStoolClientFactory {

    pub fn create_default() -> ToadStoolComputeClient {
        ToadStoolComputeClient::new(ToadStoolClientConfig::default())
    }

    pub fn create_with_config(config: ToadStoolClientConfig) -> ToadStoolComputeClient {
        ToadStoolComputeClient::new(config)
    }

    pub fn create_for_development() -> ToadStoolComputeClient {
        let mut config = ToadStoolClientConfig::default();
        config.endpoint = "http://localhost:9090".to_string();
        config.timeout_ms = 10000;
        ToadStoolComputeClient::new(config)
    }

    pub fn create_for_production(endpoint: &str, auth_token: &str) -> ToadStoolComputeClient {
        let mut config = ToadStoolClientConfig::default();
        config.endpoint = endpoint;
        config.auth_token = auth_token;
        config.timeout_ms = 5000; // Shorter timeout for production
        ToadStoolComputeClient::new(config)
    }
} 