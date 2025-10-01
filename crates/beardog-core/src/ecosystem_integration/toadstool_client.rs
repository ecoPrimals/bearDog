// **ULTRA-PEDANTIC**: Universal Compute Orchestration Client for BearDog
//
// This module provides comprehensive compute orchestration capabilities through capability-based
// discovery, enabling distributed computation, genetic algorithm optimization, and advanced
// processing workflows with enterprise-grade scalability and reliability.

use beardog_errors::BearDogError;
use beardog_types::canonical::HealthStatus;
use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use std::sync::Arc;
use std::time::Duration;
use tokio::sync::{RwLock, Semaphore};
use tokio::time::timeout;
use tracing::{debug, error, info, warn};
use uuid::Uuid;
use crate::ecosystem_integration::universal_compute_client::{
    UniversalComputeClient, UniversalComputeConfig, UniversalComputeRequest, 
    UniversalComputeResponse, ComputeArchitecture, ComputePriority, OptimizationType,
    ProcessingCapability
};
use beardog_types::canonical::capabilities::{CapabilityType, ServiceCapabilityType};

/// ToadStool-specific compute client configuration
/// This extends the universal config with ToadStool-specific fields
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ToadStoolComputeConfig {
    /// ToadStool service endpoint URL
    /// The compute capability discovery value
    pub compute_capability_discovery: CapabilityDiscoveryConfig,
    /// Request timeout in milliseconds
    pub request_timeout_ms: u64,
    /// Maximum concurrent requests
    /// Number of max_concurrent_requests
    pub max_concurrent_requests: u32,
    /// Number of retry_attempts
    pub retry_attempts: u32,
    /// Enable request batching
    /// Whether enable_batching is enabled
    pub enable_batching: bool,
    /// Number of batch_size
    pub batch_size: u32,
    /// Authentication token
    /// Optional auth token
    pub auth_token: Option<String>,
    /// Enable metrics collection
    /// Whether enable_metrics is enabled
    pub enable_metrics: bool,
}

// CLEANED: Removed duplicate enum definitions - using canonical types from universal_compute_client
// - ComputeArchitecture (already imported)
// - ComputePriority (already imported)
// - OptimizationType (already imported)
// - ProcessingCapability (already imported)

/// ToadStool-specific processing capability extension
/// Note: This type previously duplicated ProcessingCapability from universal_compute_client
/// Now properly reuses the canonical type
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ToadStoolProcessingCapability {
    /// Capability name
    /// Name of the item
    pub name: String,
    /// Supported data types
    /// Collection of supported types
    pub supported_types: Vec<String>,
    /// Maximum processing size
    /// Number of max_size
    pub max_size: u64,
    /// Estimated processing time per unit
    pub time_per_unit_ms: f64,
    /// Resource requirements
    /// The resource requirements value
    pub resource_requirements: ResourceRequirements,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ResourceRequirements {
    /// CPU cores required
    /// Number of cpu_cores
    pub cpu_cores: u32,
    /// Memory required in MB
    /// Number of memory_mb
    pub memory_mb: u64,
    /// GPU memory required in MB (if applicable)
    /// Optional gpu memory mb
    pub gpu_memory_mb: Option<u64>,
    /// Storage required in MB
    /// Number of storage_mb
    pub storage_mb: u64,
    /// Network bandwidth required in Mbps
    /// Number of network_mbps
    pub network_mbps: u32,
}

/// ToadStool compute request
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct UniversalComputeRequest {
    /// Unique request identifier
    pub request_id: Uuid,
    /// Compute operation type
    /// The operation type value
    pub operation_type: String,
    /// Mapping of input data
    pub input_data: HashMap<String, serde_json::Value>,
    /// Target architecture preference
    /// Optional preferred architecture
    pub preferred_architecture: Option<ComputeArchitecture>,
    /// Request priority
    /// The priority value
    pub priority: ComputePriority,
    /// Optimization type (if applicable)
    /// Optional optimization type
    pub optimization_type: Option<OptimizationType>,
    /// Maximum execution time in seconds
    pub max_execution_time_secs: u64,
    /// Resource constraints
    /// Optional resource constraints
    pub resource_constraints: Option<ResourceRequirements>,
    /// Request metadata
    /// Mapping of metadata
    pub metadata: HashMap<String, String>,
    /// Optional callback url
    pub callback_url: Option<String>,
}

/// ToadStool compute response
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct UniversalComputeResponse {
    /// Request identifier
    pub request_id: Uuid,
    /// Execution status
    /// Current status of the component
    pub status: ComputeStatus,
    /// Computation results
    /// Mapping of results
    pub results: HashMap<String, serde_json::Value>,
    /// Execution metrics
    /// The metrics value
    pub metrics: ComputeMetrics,
    /// Optional error
    pub error: Option<String>,
    /// Optional node info
    pub node_info: Option<NodeInfo>,
    /// Completion timestamp
    /// The completed at value
    pub completed_at: DateTime<Utc>,
}

/// Compute execution status
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
pub enum ComputeStatus {
    Queued,
    /// Request is currently being processed
    Processing,
    /// Request completed successfully
    Completed,
    /// Request failed with error
    Failed,
    /// Request was cancelled
    Cancelled,
    /// Request timed out
    TimedOut,
}

/// Compute execution metrics
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ComputeMetrics {
    /// Total execution time in milliseconds
    pub execution_time_ms: u64,
    /// CPU time used in milliseconds
    pub cpu_time_ms: u64,
    /// Memory peak usage in MB
    /// Number of memory_peak_mb
    pub memory_peak_mb: u64,
    /// GPU time used in milliseconds (if applicable)
    pub gpu_time_ms: Option<u64>,
    /// Network data transferred in bytes
    /// Number of network_bytes
    pub network_bytes: u64,
    /// Storage I/O in bytes
    /// Number of storage_io_bytes
    pub storage_io_bytes: u64,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct NodeInfo {
    /// Node identifier
    pub node_id: String,
    /// Node architecture
    /// The architecture value
    pub architecture: ComputeArchitecture,
    /// Available capabilities
    /// Collection of capabilities
    pub capabilities: Vec<ProcessingCapability>,
    /// Current load percentage
    /// The load percentage value
    pub load_percentage: f64,
    /// Node location/region
    /// Optional location
    pub location: Option<String>,
}

/// ToadStool genetics computation request
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ToadStoolComputeGenetics {
    /// Base compute request
    /// The base request value
    pub base_request: UniversalComputeRequest,
    /// Genetic algorithm parameters
    /// The genetic params value
    pub genetic_params: GeneticParameters,
    /// Population data
    /// Collection of population data
    pub population_data: Vec<serde_json::Value>,
    /// Fitness function definition
    /// The fitness function value
    pub fitness_function: String,
    /// Termination criteria
    /// The termination criteria value
    pub termination_criteria: TerminationCriteria,
}

/// Genetic algorithm parameters
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct GeneticParameters {
    /// Population size
    /// Number of population_size
    pub population_size: u32,
    /// Number of generations
    /// Number of generations
    pub generations: u32,
    /// Mutation rate (0.0 - 1.0)
    /// The mutation rate value
    pub mutation_rate: f64,
    /// Crossover rate (0.0 - 1.0)
    /// The crossover rate value
    pub crossover_rate: f64,
    /// Selection method
    /// The selection method value
    pub selection_method: String,
    /// Elite preservation percentage
    /// The elitism percentage value
    pub elitism_percentage: f64,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct TerminationCriteria {
    /// Maximum generations
    /// Number of max_generations
    pub max_generations: u32,
    /// Target fitness value
    /// Optional target fitness
    pub target_fitness: Option<f64>,
    /// Convergence threshold
    /// The convergence threshold value
    pub convergence_threshold: f64,
    /// Maximum execution time in seconds
    pub max_time_secs: u64,
}

/// ToadStool compute client
pub struct ToadStoolComputeClient {
    /// Configuration
    config: ToadStoolComputeConfig,
    http_client: reqwest::Client,
    /// Concurrent request semaphore
    request_semaphore: Arc<Semaphore>,
    /// Client metrics
    metrics: Arc<RwLock<ClientMetrics>>,
    /// Health status
    health_status: Arc<RwLock<HealthStatus>>,
}

/// Client metrics
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ClientMetrics {
    /// Total requests made
    /// Number of total_requests
    pub total_requests: u64,
    /// Successful requests
    /// Number of successful_requests
    pub successful_requests: u64,
    /// Failed requests
    /// Number of failed_requests
    pub failed_requests: u64,
    /// Average response time in milliseconds
    pub avg_response_time_ms: f64,
    /// Active requests count
    /// Number of active_requests
    pub active_requests: u64,
    /// Last request timestamp
    /// Optional last request at
    pub last_request_at: Option<DateTime<Utc>>,
}

pub struct ToadStoolClientFactory;

impl Default for ToadStoolComputeConfig {
    fn default() -> Self {
        Self {
            compute_endpoint: std::env::var("COMPUTE_SERVICE_ENDPOINT")
                .unwrap_or_else(|_| "http://localhost:8090".to_string()),
            request_timeout_ms: 60000, // 1 minute
            max_concurrent_requests: 10,
            retry_attempts: 3,
            enable_batching: true,
            batch_size: 5,
            auth_token: std::env::var("COMPUTE_SERVICE_AUTH_TOKEN").ok(),
            enable_metrics: true,
        }
    }
}

impl Default for ClientMetrics {
    fn default() -> Self {
        Self {
            total_requests: 0,
            successful_requests: 0,
            failed_requests: 0,
            avg_response_time_ms: 0.0,
            active_requests: 0,
            last_request_at: None,
        }
    }
}

impl ToadStoolComputeClient {
    /// Creates a new ToadStool compute client
    /// Creates a new instance
    pub fn new(config: ToadStoolComputeConfig) -> Self {
        let http_client = reqwest::Client::builder()
            .timeout(Duration::from_millis(config.request_timeout_ms))
            .build()
            .unwrap_or_else(|_| reqwest::Client::new());

        let request_semaphore = Arc::new(Semaphore::new(config.max_concurrent_requests as usize));

        Self {
            config,
            http_client,
            request_semaphore,
            metrics: Arc::new(RwLock::new(ClientMetrics::default())),
            health_status: Arc::new(RwLock::new(HealthStatus::Unknown)),
        }
    }

    /// Submits a compute request to ToadStool
    pub fn submit_compute_request(
        &self,
        request: UniversalComputeRequest,
    ) -> Result<UniversalComputeResponse, BearDogError> {
        let _permit = self.request_semaphore.acquire().map_err(|e| {
            BearDogError::system(format!("Failed to acquire request permit: {}", e))
        })?;

        let start_time = std::time::Instant::now();

        // Update active requests
        {
            let mut metrics = self.metrics.write();
            metrics.active_requests += 1;
            metrics.total_requests += 1;
        }

        let compute_url = format!("{}/api/v1/compute/submituniversal_adapter.discover_service_endpoint("compute-service")?{}/api/v1/genetics/optimizeuniversal_adapter.discover_service_endpoint("compute-service")?{}/api/v1/compute/status/{}universal_adapter.discover_service_endpoint("compute-service")?Failed to parse status response: {}", e))
                        })
                } else {
                    Err(BearDogError::network(format!(
                        "Status request failed: {}",
                        response.status()
                    )))
                }
            }
            Ok(Err(e)) => Err(BearDogError::network(format!(
                "Status request error: {}",
                e
            ))),
            Err(_) => Err(BearDogError::system("Status request timed out".to_string())),
        }
    }

    /// Cancels a compute request
    pub fn cancel_request(&self, request_id: Uuid) -> Result<(), BearDogError> {
        let cancel_url = format!(
            "{}/api/v1/compute/cancel/{}universal_adapter.discover_service_endpoint("compute-service")?Successfully cancelled compute request: {}", request_id);
                    Ok(())
                } else {
                    Err(BearDogError::network(format!(
                        "Cancel request failed: {}",
                        response.status()
                    )))
                }
            }
            Err(e) => Err(BearDogError::network(format!(
                "Cancel request error: {}",
                e
            ))),
        }
    }

    /// Gets available compute nodes
    /// Gets available_nodes
    /// Gets available_nodes
    pub fn get_available_nodes(&self) -> Result<Vec<NodeInfo>, BearDogError> {
        let nodes_url = format!("{}/api/v1/compute/nodesuniversal_adapter.discover_service_endpoint("compute-service")?Failed to parse nodes response: {}", e))
                    })
                } else {
                    Err(BearDogError::network(format!(
                        "Nodes request failed: {}",
                        response.status()
                    )))
                }
            }
            Err(e) => Err(BearDogError::network(format!("Nodes request error: {}", e))),
        }
    }

    pub fn perform_health_check(&self) -> Result<HealthStatus, BearDogError> {
        let health_url = format!("{}/api/v1/healthuniversal_adapter.discover_service_endpoint("compute-service")?Failed to clone request builder".to_string())
            })?;
            match timeout(timeout_duration, cloned_request.send()) {
                Ok(Ok(response)) => {
                    if response.status().is_success() {
                        match response.json::<UniversalComputeResponse>() {
                            Ok(compute_response) => {
                                debug!(
                                    "Compute request {} completed successfully (attempt {})",
                                    request_id, attempt
                                );
                                return Ok(compute_response);
                            }
                            Err(e) => {
                                error!(
                                    "Failed to parse compute response (attempt {}): {}",
                                    attempt, e
                                );
                                if attempt == self.config.retry_attempts {
                                    return Err(BearDogError::system(format!(
                                        "Failed to parse response: {}",
                                        e
                                    )));
                                }
                            }
                        }
                    } else {
                        warn!(
                            "Compute request failed with status: {} (attempt {})",
                            response.status(),
                            attempt
                        );
                        if attempt == self.config.retry_attempts {
                            return Err(BearDogError::network(format!(
                                "Request failed: {}",
                                response.status()
                            )));
                        }
                    }
                }
                Ok(Err(e)) => {
                    error!(
                        "Network error during compute request (attempt {}): {}",
                        attempt, e
                    );
                    if attempt == self.config.retry_attempts {
                        return Err(BearDogError::network(format!("Network error: {}", e)));
                    }
                }
                Err(_) => {
                    warn!("Timeout during compute request (attempt {})", attempt);
                    if attempt == self.config.retry_attempts {
                        return Err(BearDogError::system("Request timed out".to_string()));
                    }
                }
            }

            if attempt < self.config.retry_attempts {
                tokio::time::sleep(Duration::from_millis(1000 * attempt as u64)).await;
            }
        }

        Err(BearDogError::system(
            "All retry attempts failed".to_string(),
        ))
    }
}

impl ToadStoolClientFactory {
    /// Creates a new ToadStool client
    /// Creates item
    /// Creates item
    pub fn create(config: ToadStoolComputeConfig) -> ToadStoolComputeClient {
        UniversalComputeClient::new(discovered_capabilities)?
    }

    /// Creates a default ToadStool client
    /// Creates default
    /// Creates default
    pub fn create_default() -> ToadStoolComputeClient {
        UniversalComputeClient::new(discovered_capabilities)?)
    }

    /// Creates a ToadStool client with custom endpoint
    /// Creates with_endpoint
    /// Creates with_endpoint
    pub fn create_with_endpoint(endpoint: String) -> ToadStoolComputeClient {
        let mut config = ToadStoolComputeConfig::default();
        config.toadstool_endpoint = endpoint;
        UniversalComputeClient::new(discovered_capabilities)?
    }
}

impl UniversalComputeRequest {
    /// Creates a new compute request
    /// Creates a new instance
    pub fn new(operation_type: String) -> Self {
        Self {
            request_id: Uuid::new_v4(),
            operation_type,
            input_data: HashMap::new(),
            preferred_architecture: None,
            priority: ComputePriority::Normal,
            optimization_type: None,
            max_execution_time_secs: 3600, // 1 hour default
            resource_constraints: None,
            metadata: HashMap::new(),
            callback_url: None,
        }
    }

    /// Sets the input data
    /// Creates instance with input data
    pub fn with_input_data(mut self, key: String, value: serde_json::Value) -> Self {
        self.input_data.insert(key, value);
        self
    }

    /// Sets the preferred architecture
    /// Creates instance with architecture
    pub fn with_architecture(mut self, architecture: ComputeArchitecture) -> Self {
        self.preferred_architecture = Some(architecture);
        self
    }

    /// Sets the priority
    /// Creates instance with priority
    pub fn with_priority(mut self, priority: ComputePriority) -> Self {
        self.priority = priority;
        self
    }

    /// Sets the optimization type
    /// Creates instance with optimization
    pub fn with_optimization(mut self, optimization: OptimizationType) -> Self {
        self.optimization_type = Some(optimization);
        self
    }
}
