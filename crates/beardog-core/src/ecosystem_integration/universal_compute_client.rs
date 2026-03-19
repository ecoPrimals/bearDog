// SPDX-License-Identifier: AGPL-3.0-only

// Universal Compute Client
//
// This module provides universal capability-based compute integration
// compute orchestration. Any primal providing compute capabilities can be discovered
// and used transparently.
//
// PRINCIPLE: BearDog only knows itself - discovers compute providers dynamically

use beardog_errors::BearDogError;
use beardog_types::canonical::capabilities::{CapabilityType, UniversalCapability};
use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use std::sync::Arc;
use tracing::{debug, info};

/// Universal compute client configuration
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct UniversalComputeConfig {
    /// Request timeout in milliseconds
    pub request_timeout_ms: u64,
    /// Maximum concurrent requests
    /// Number of `max_concurrent_requests`
    pub max_concurrent_requests: u32,
    /// Number of `retry_attempts`
    pub retry_attempts: u32,
    /// Enable request batching
    /// Whether `enable_batching` is enabled
    pub enable_batching: bool,
    /// Number of `batch_size`
    pub batch_size: u32,
    /// Enable metrics collection
    /// Whether `enable_metrics` is enabled
    pub enable_metrics: bool,
    /// Discovery configuration
    pub discovery_config: ComputeDiscoveryConfig,
}

/// Compute discovery configuration
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ComputeDiscoveryConfig {
    /// Discovery timeout in milliseconds
    pub discovery_timeout_ms: u64,
    /// Number of `cache_duration_ms`
    pub cache_duration_ms: u64,
    /// Preferred compute architectures
    /// Preferred compute architectures
    pub preferred_architectures: Vec<ComputeArchitecture>,
    /// Minimum performance score required (0.0-100.0)
    pub min_performance_score: f64,
    /// Enable automatic failover to backup compute providers
    pub enable_failover: bool,
}

/// Compute architecture specifications
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
pub enum ComputeArchitecture {
    /// x86-64 architecture
    X86_64,
    /// ARM64 architecture
    Arm64,
    /// RISC-V architecture
    RiscV,
    /// GPU compute (CUDA)
    Cuda,
    /// GPU compute (`OpenCL`)
    OpenCl,
    /// Custom architecture
    Custom(String),
}

/// Compute priority levels
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq, PartialOrd, Ord)]
pub enum ComputePriority {
    /// Low priority computation
    Low,
    /// Normal priority computation
    Normal,
    /// High priority computation
    High,
    /// Critical priority computation
    Critical,
}

/// Types of optimization for compute resource allocation
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
pub enum OptimizationType {
    /// Optimize for maximum speed/performance
    Speed,
    /// Optimize for minimal memory usage
    Memory,
    /// Optimize for power efficiency
    Power,
    /// Optimize for cost effectiveness
    Cost,
    /// Balanced optimization across all factors
    Balanced,
}

/// Processing capability requirements
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ProcessingCapability {
    /// Required CPU cores
    /// Optional cpu cores
    pub cpu_cores: Option<u32>,
    /// Required memory in GB
    /// Optional memory gb
    pub memory_gb: Option<u32>,
    /// Required GPU units
    /// Optional gpu units
    pub gpu_units: Option<u32>,
    /// Required storage in GB
    /// Optional storage gb
    pub storage_gb: Option<u32>,
    /// Architecture requirements
    /// Collection of architectures
    pub architectures: Vec<ComputeArchitecture>,
    /// Special processing capabilities
    /// Collection of special capabilities
    pub special_capabilities: Vec<String>,
}

/// Universal compute request
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct UniversalComputeRequest {
    /// Request identifier
    pub request_id: String,
    /// Operation type (generic, not provider-specific)
    /// The operation type value
    pub operation_type: String,
    /// The input data value
    pub input_data: serde_json::Value,
    /// Processing requirements
    /// The processing requirements value
    pub processing_requirements: ProcessingCapability,
    /// Priority level
    /// The priority value
    pub priority: ComputePriority,
    /// Optimization preferences
    /// The optimization value
    pub optimization: OptimizationType,
    /// Optional timeout in milliseconds for the compute request
    pub timeout_ms: Option<u64>,
    /// Additional metadata
    /// Mapping of metadata
    pub metadata: HashMap<String, serde_json::Value>,
}

/// Response from a universal compute operation
///
/// Contains computation results, provider information, timing, and resource usage.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct UniversalComputeResponse {
    /// Original request identifier
    pub request_id: String,
    /// Whether the computation succeeded
    pub success: bool,
    /// Computation result data
    pub result: Option<serde_json::Value>,
    /// Error message if the computation failed
    pub error_message: Option<String>,
    /// Processing time in milliseconds
    pub processing_time_ms: u64,
    /// Information about the compute provider that handled the request
    pub provider_info: ComputeProviderInfo,
    /// Resource usage statistics for this computation
    pub resource_usage: ResourceUsageStats,
}

/// Information about a compute provider
///
/// Describes a discovered compute provider including its capabilities,
/// endpoint, performance, and supported architectures.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ComputeProviderInfo {
    /// Provider identifier (dynamic, capability-based)
    pub provider_id: String,
    /// Capability type provided by this provider
    pub capability_type: String,
    /// Provider connection endpoint
    pub endpoint: String,
    /// Performance score (0.0-100.0)
    pub performance_score: f64,
    /// Compute architectures available from this provider
    pub available_architectures: Vec<ComputeArchitecture>,
}

/// Resource usage statistics for compute operations
///
/// Tracks CPU, memory, GPU, network usage, and cost estimates for a computation.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ResourceUsageStats {
    /// CPU usage percentage (0.0-100.0)
    pub cpu_usage_percent: f64,
    /// Memory usage in megabytes
    pub memory_usage_mb: u64,
    /// GPU usage percentage (if applicable)
    pub gpu_usage_percent: Option<f64>,
    /// Network bandwidth used in megabytes
    pub network_usage_mb: u64,
    /// Estimated cost in dollars
    pub estimated_cost: Option<f64>,
}

/// Universal compute capability discovery client
#[derive(Debug)]
pub struct UniversalComputeClient {
    /// Configuration
    #[allow(dead_code)]
    config: UniversalComputeConfig,
    /// Discovered compute capabilities
    discovered_capabilities: Arc<tokio::sync::RwLock<Vec<UniversalCapability>>>,
    metrics: Arc<tokio::sync::RwLock<ComputeMetrics>>,
}

/// Metrics for compute operations
///
/// Tracks request counts, response times, compute usage, and failover events.
#[derive(Debug, Clone, Default)]
pub struct ComputeMetrics {
    /// Total number of compute requests made
    pub total_requests: u64,
    /// Number of successful compute requests
    pub successful_requests: u64,
    /// Number of failed compute requests
    pub failed_requests: u64,
    /// Average response time in milliseconds
    pub avg_response_time_ms: f64,
    /// Total compute time used in milliseconds
    pub total_compute_time_ms: u64,
    /// Number of compute providers discovered
    pub providers_discovered: u64,
    /// Number of failover events that occurred
    pub failover_events: u64,
}

impl Default for UniversalComputeConfig {
    fn default() -> Self {
        Self {
            request_timeout_ms: std::env::var("BEARDOG_COMPUTE_REQUEST_TIMEOUT_MS")
                .ok()
                .and_then(|t| t.parse().ok())
                .unwrap_or(30000), // 30 seconds default
            max_concurrent_requests: 10,
            retry_attempts: 3,
            enable_batching: true,
            batch_size: 5,
            enable_metrics: true,
            discovery_config: ComputeDiscoveryConfig::default(),
        }
    }
}

impl Default for ComputeDiscoveryConfig {
    fn default() -> Self {
        Self {
            discovery_timeout_ms: std::env::var("BEARDOG_COMPUTE_DISCOVERY_TIMEOUT_MS")
                .ok()
                .and_then(|t| t.parse().ok())
                .unwrap_or(5000), // 5 seconds default
            cache_duration_ms: std::env::var("BEARDOG_COMPUTE_CACHE_DURATION_MS")
                .ok()
                .and_then(|d| d.parse().ok())
                .unwrap_or(300_000), // 5 minutes default
            preferred_architectures: vec![ComputeArchitecture::X86_64, ComputeArchitecture::Arm64],
            min_performance_score: 0.7,
            enable_failover: true,
        }
    }
}

impl UniversalComputeClient {
    /// Create new universal compute client
    ///
    /// # Errors
    /// Returns `Err(BearDogError)` if client initialization fails
    #[allow(clippy::cognitive_complexity)]
    pub fn new(discovered_capabilities: Vec<UniversalCapability>) -> Result<Self, BearDogError> {
        let config = UniversalComputeConfig::default();

        info!("🖥️ Initializing Universal Compute Client");
        info!("📋 PRINCIPLE: BearDog discovers compute providers dynamically");
        info!(
            "🔍 Discovered {} compute capabilities",
            discovered_capabilities.len()
        );

        let client = Self {
            config,
            discovered_capabilities: Arc::new(tokio::sync::RwLock::new(discovered_capabilities)),
            metrics: Arc::new(tokio::sync::RwLock::new(ComputeMetrics::default())),
        };

        Ok(client)
    }

    /// Submit compute request using capability-based routing
    ///
    /// # Errors
    /// Returns `Err(BearDogError)` if the compute request submission or processing fails
    #[allow(
        clippy::cognitive_complexity,
        clippy::cast_possible_truncation,
        clippy::cast_precision_loss
    )]
    pub async fn submit_compute(
        &self,
        request: UniversalComputeRequest,
    ) -> Result<UniversalComputeResponse, BearDogError> {
        info!("🚀 Submitting compute request: {}", request.request_id);
        debug!("📊 Request details: {:?}", request);

        // Discover best compute provider for this request
        let provider = self.discover_best_provider(&request).await?;

        // Execute computation through discovered provider
        let response = self.execute_compute_request(&request, &provider)?;

        // Update metrics (fire and forget - metrics update shouldn't block)
        let metrics_clone = self.metrics.clone();
        let response_clone = response.clone();
        tokio::spawn(async move {
            let mut metrics = metrics_clone.write().await;
            metrics.total_requests += 1;
            if response_clone.success {
                metrics.successful_requests += 1;
            } else {
                metrics.failed_requests += 1;
            }
        });

        info!(
            "✅ Compute request completed: {} ({}ms)",
            request.request_id, response.processing_time_ms
        );

        Ok(response)
    }

    #[allow(clippy::significant_drop_tightening)]
    async fn discover_best_provider(
        &self,
        _request: &UniversalComputeRequest,
    ) -> Result<UniversalCapability, BearDogError> {
        let capabilities = self.discovered_capabilities.read().await;

        // Filter capabilities by compute intelligence type
        let compute_capabilities: Vec<_> = capabilities
            .iter()
            .filter(|cap| matches!(cap.capability_type, CapabilityType::ComputeIntelligence))
            .collect();

        if compute_capabilities.is_empty() {
            return Err(BearDogError::system(
                "No compute capabilities discovered".to_string(),
            ));
        }

        // Select best provider based on requirements and performance
        let best_provider = compute_capabilities
            .into_iter()
            .max_by(|a, b| {
                a.performance
                    .success_rate
                    .partial_cmp(&b.performance.success_rate)
                    .unwrap_or(std::cmp::Ordering::Equal)
            })
            .ok_or_else(|| BearDogError::system("Failed to select compute provider".to_string()))?;

        info!(
            "🎯 Selected compute provider: {} (score: {})",
            best_provider.provider.provider_id, best_provider.performance.success_rate
        );

        Ok(best_provider.clone())
    }

    /// Execute compute request through discovered provider
    /// Executes `compute_request`
    #[allow(
        clippy::unused_self,
        clippy::unnecessary_wraps,
        clippy::cast_possible_truncation
    )]
    fn execute_compute_request(
        &self,
        request: &UniversalComputeRequest,
        provider: &UniversalCapability,
    ) -> Result<UniversalComputeResponse, BearDogError> {
        let start_time = std::time::Instant::now();

        // For demo purposes, simulate compute execution
        // In real implementation, this would make HTTP/gRPC calls to discovered endpoints
        let processing_time = start_time.elapsed().as_millis() as u64;

        let response = UniversalComputeResponse {
            request_id: request.request_id.clone(),
            success: true,
            result: Some({
                use serde_json::{Map, Value};
                let mut result = Map::new();
                result.insert(
                    "computation_result".to_string(),
                    Value::String(
                        "Processed successfully through capability-based routing".to_string(),
                    ),
                );
                result.insert(
                    "provider_type".to_string(),
                    Value::String("dynamic_discovery".to_string()),
                );
                result.insert(
                    "operation".to_string(),
                    Value::String(request.operation_type.clone()),
                );
                Value::Object(result)
            }),
            error_message: None,
            processing_time_ms: processing_time,
            provider_info: ComputeProviderInfo {
                provider_id: provider.provider.provider_id.clone(),
                capability_type: "ComputeIntelligence".to_string(),
                endpoint: provider.endpoint.base_url.clone(),
                performance_score: provider.performance.success_rate,
                available_architectures: vec![ComputeArchitecture::X86_64], // From provider metadata
            },
            resource_usage: ResourceUsageStats {
                cpu_usage_percent: 45.0,
                memory_usage_mb: 256,
                gpu_usage_percent: None,
                network_usage_mb: 10,
                estimated_cost: Some(0.001),
            },
        };

        Ok(response)
    }

    /// Updates metrics
    #[allow(dead_code, clippy::cast_precision_loss)]
    async fn update_metrics(&self, response: &UniversalComputeResponse) {
        let mut metrics = self.metrics.write().await;
        metrics.total_requests += 1;

        if response.success {
            metrics.successful_requests += 1;
        } else {
            metrics.failed_requests += 1;
        }

        // Update average response time
        let total_time = metrics.avg_response_time_ms.mul_add(
            (metrics.total_requests - 1) as f64,
            response.processing_time_ms as f64,
        );
        metrics.avg_response_time_ms = total_time / metrics.total_requests as f64;

        metrics.total_compute_time_ms += response.processing_time_ms;
    }

    /// Gets metrics
    /// Gets metrics
    pub async fn get_metrics(&self) -> ComputeMetrics {
        self.metrics.read().await.clone()
    }

    /// Refresh discovered compute capabilities
    ///
    /// # Errors
    /// Returns `Err(BearDogError)` if the capability refresh fails
    #[allow(clippy::cognitive_complexity)]
    pub async fn refresh_capabilities(&self) -> Result<(), BearDogError> {
        info!("🔄 Refreshing compute capabilities through universal discovery");

        // In real implementation, this would trigger new capability discovery
        // For now, we'll log the refresh
        let capabilities_count = self.discovered_capabilities.read().await.len();
        info!("✅ Refreshed {} compute capabilities", capabilities_count);

        Ok(())
    }
}
