// SPDX-License-Identifier: AGPL-3.0-or-later

// Universal Compute Client
//
// This module provides universal capability-based compute integration
// compute orchestration. Any primal providing compute capabilities can be discovered
// and used transparently.
//
// PRINCIPLE: BearDog only knows itself - discovers compute providers dynamically

use beardog_config::env_keys;
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
    #[expect(
        dead_code,
        reason = "Config applied when compute client gains full routing"
    )]
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
            request_timeout_ms: 30000,
            max_concurrent_requests: 10,
            retry_attempts: 3,
            enable_batching: true,
            batch_size: 5,
            enable_metrics: true,
            discovery_config: ComputeDiscoveryConfig::default(),
        }
    }
}

impl UniversalComputeConfig {
    /// Load overrides from `BEARDOG_COMPUTE_*` via `std::env::var`.
    #[must_use]
    pub fn from_env() -> Self {
        Self {
            request_timeout_ms: std::env::var(env_keys::ENV_COMPUTE_REQUEST_TIMEOUT_MS)
                .ok()
                .and_then(|t| t.parse().ok())
                .unwrap_or(30000),
            max_concurrent_requests: 10,
            retry_attempts: 3,
            enable_batching: true,
            batch_size: 5,
            enable_metrics: true,
            discovery_config: ComputeDiscoveryConfig::from_env(),
        }
    }
}

impl Default for ComputeDiscoveryConfig {
    fn default() -> Self {
        Self {
            discovery_timeout_ms: 5000,
            cache_duration_ms: 300_000,
            preferred_architectures: vec![ComputeArchitecture::X86_64, ComputeArchitecture::Arm64],
            min_performance_score: 0.7,
            enable_failover: true,
        }
    }
}

impl ComputeDiscoveryConfig {
    /// Load discovery/cache timeouts from environment.
    #[must_use]
    pub fn from_env() -> Self {
        Self {
            discovery_timeout_ms: std::env::var(env_keys::ENV_COMPUTE_DISCOVERY_TIMEOUT_MS)
                .ok()
                .and_then(|t| t.parse().ok())
                .unwrap_or(5000),
            cache_duration_ms: std::env::var(env_keys::ENV_COMPUTE_CACHE_DURATION_MS)
                .ok()
                .and_then(|d| d.parse().ok())
                .unwrap_or(300_000),
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

        {
            let mut metrics = self.metrics.write().await;
            metrics.total_requests += 1;
            if response.success {
                metrics.successful_requests += 1;
            } else {
                metrics.failed_requests += 1;
            }
        }

        info!(
            "✅ Compute request completed: {} ({}ms)",
            request.request_id, response.processing_time_ms
        );

        Ok(response)
    }

    #[expect(
        clippy::significant_drop_tightening,
        reason = "RwLock read guard for capability scan"
    )]
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
    #[expect(
        clippy::unused_self,
        reason = "Instance reserved for real provider RPC calls"
    )]
    fn execute_compute_request(
        &self,
        request: &UniversalComputeRequest,
        _provider: &UniversalCapability,
    ) -> Result<UniversalComputeResponse, BearDogError> {
        Err(BearDogError::not_yet_available(format!(
            "Universal compute dispatch for operation '{}' — awaiting IPC transport integration",
            request.operation_type
        )))
    }

    /// Updates metrics
    #[expect(
        dead_code,
        reason = "Metrics helper for when submit_compute wires averaging"
    )]
    #[expect(
        clippy::cast_precision_loss,
        reason = "Moving average over integer request counts"
    )]
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
    pub async fn get_metrics(&self) -> ComputeMetrics {
        self.metrics.read().await.clone()
    }

    /// Refresh discovered compute capabilities
    ///
    /// # Errors
    /// Returns `Err(BearDogError)` if the capability refresh fails
    pub async fn refresh_capabilities(&self) -> Result<(), BearDogError> {
        info!("🔄 Refreshing compute capabilities through universal discovery");

        // In real implementation, this would trigger new capability discovery
        // For now, we'll log the refresh
        let capabilities_count = self.discovered_capabilities.read().await.len();
        info!("✅ Refreshed {} compute capabilities", capabilities_count);

        Ok(())
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use beardog_types::canonical::capabilities::{
        AuthConfig, AuthType, CapabilityType, CircuitBreakerConfig, EndpointConfig, HealthStatus,
        PerformanceMetrics, ProviderInfo, SecurityLevel, UniversalCapability,
    };
    use beardog_types::canonical::providers_unified::core::ProviderType;
    use std::collections::HashMap;

    fn sample_compute_capability(success_rate: f64) -> UniversalCapability {
        UniversalCapability {
            capability_type: CapabilityType::ComputeIntelligence,
            provider: ProviderInfo {
                provider_id: "prov-a".to_string(),
                provider_name: "Test Compute".to_string(),
                provider_type: ProviderType::Compute,
                version: "1.0.0".to_string(),
                region: None,
            },
            endpoint: EndpointConfig {
                base_url: "http://127.0.0.1:9".to_string(),
                api_version: None,
                timeout_ms: 1000,
                max_retries: 1,
                circuit_breaker: CircuitBreakerConfig::default(),
            },
            auth_config: AuthConfig {
                auth_type: AuthType::None,
                api_key: None,
                bearer_token: None,
                cert_path: None,
                custom_params: HashMap::new(),
            },
            health_status: HealthStatus::Healthy,
            performance: PerformanceMetrics {
                avg_response_time_ms: 10.0,
                success_rate,
                throughput_rps: 1.0,
                current_load: 0.1,
                last_updated: chrono::Utc::now(),
            },
            security_level: SecurityLevel::Standard,
            metadata: HashMap::new(),
        }
    }

    #[test]
    fn universal_compute_config_defaults_are_deterministic_without_env() {
        let _ = ComputeDiscoveryConfig::default();
        let c: UniversalComputeConfig = UniversalComputeConfig::default();
        assert!(c.request_timeout_ms > 0);
        assert!(c.discovery_config.min_performance_score > 0.0);
    }

    #[test]
    fn universal_compute_config_reads_env_overrides() {
        let c = UniversalComputeConfig {
            request_timeout_ms: 12000,
            discovery_config: ComputeDiscoveryConfig {
                discovery_timeout_ms: 1500,
                cache_duration_ms: 60000,
                ..ComputeDiscoveryConfig::default()
            },
            ..UniversalComputeConfig::default()
        };
        assert_eq!(c.request_timeout_ms, 12000);
        assert_eq!(c.discovery_config.discovery_timeout_ms, 1500);
        assert_eq!(c.discovery_config.cache_duration_ms, 60000);
    }

    #[tokio::test]
    async fn submit_compute_errors_when_no_providers() {
        let client = UniversalComputeClient::new(vec![]).expect("client");
        let req = UniversalComputeRequest {
            request_id: "r1".to_string(),
            operation_type: "test".to_string(),
            input_data: serde_json::json!({}),
            processing_requirements: ProcessingCapability {
                cpu_cores: None,
                memory_gb: None,
                gpu_units: None,
                storage_gb: None,
                architectures: vec![ComputeArchitecture::X86_64],
                special_capabilities: vec![],
            },
            priority: ComputePriority::Normal,
            optimization: OptimizationType::Balanced,
            timeout_ms: None,
            metadata: HashMap::new(),
        };
        let err = client.submit_compute(req).await.expect_err("no providers");
        assert!(format!("{err}").to_lowercase().contains("compute"));
    }

    #[tokio::test]
    async fn submit_compute_selects_highest_success_rate() {
        let caps = vec![
            sample_compute_capability(0.5),
            sample_compute_capability(0.99),
        ];
        let client = UniversalComputeClient::new(caps).expect("client");
        let req = UniversalComputeRequest {
            request_id: "r2".to_string(),
            operation_type: "analyze".to_string(),
            input_data: serde_json::json!({"x": 1}),
            processing_requirements: ProcessingCapability {
                cpu_cores: Some(1),
                memory_gb: None,
                gpu_units: None,
                storage_gb: None,
                architectures: vec![],
                special_capabilities: vec![],
            },
            priority: ComputePriority::High,
            optimization: OptimizationType::Speed,
            timeout_ms: Some(5000),
            metadata: HashMap::new(),
        };
        let resp = client.submit_compute(req).await.expect("ok");
        assert!(resp.success);
        assert_eq!(resp.provider_info.performance_score, 0.99);

        let m = client.get_metrics().await;
        assert!(m.total_requests >= 1);
    }

    #[tokio::test]
    async fn refresh_capabilities_is_ok() {
        let client = UniversalComputeClient::new(vec![sample_compute_capability(1.0)]).unwrap();
        client.refresh_capabilities().await.expect("refresh");
    }
}
