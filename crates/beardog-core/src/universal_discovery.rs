//! # Universal Capability Discovery System
//!
//! This module implements a universal, agnostic capability discovery system that allows
//! BearDog to find and communicate with ANY ecosystem modules based purely on capabilities,
//! without hardcoding specific primal names or types.
//!
//! Key Principles:
//! - ZERO hardcoded primal names (no "toadstool", "nestgate", etc.)
//! - Pure capability-based discovery
//! - Universal module communication patterns
//! - Dynamic ecosystem adaptation

use async_trait::async_trait;
use beardog_errors::{BearDogError, BearDogResult};
use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use std::sync::Arc;
use tracing::{debug, info, warn};
use uuid::Uuid;

/// Universal capability types that can be requested
#[derive(Debug, Clone, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub enum CapabilityType {
    // Computation capabilities
    ComputeOptimization,
    GeneticAlgorithms,
    PerformanceAcceleration,
    ParallelProcessing,
    QuantumComputing,

    // Security capabilities
    Encryption,
    Decryption,
    Authentication,
    Authorization,
    ThreatDetection,
    ComplianceAuditing,
    KeyManagement,

    // Storage capabilities
    FileSystem,
    ObjectStorage,
    VolumeManagement,
    DataReplication,
    BackupServices,

    // Network capabilities
    ServiceDiscovery,
    LoadBalancing,
    NetworkRouting,
    MessageRouting,
    RealTimeStreaming,

    // AI capabilities
    ModelInference,
    AgentFramework,
    NaturalLanguageProcessing,
    MachineLearning,
    PatternRecognition,

    // System capabilities
    ResourceManagement,
    ProcessOrchestration,
    ConfigManagement,
    HealthMonitoring,
    MetricsCollection,

    // Specialized capabilities
    Gaming {
        low_latency: bool,
        simd_acceleration: bool,
    },
    GeneticHealing {
        adaptive: bool,
        evolutionary: bool,
    },
    Custom(String),
}

impl CapabilityType {
    /// Convert capability to string for ecosystem communication
    pub fn as_capability_string(&self) -> String {
        match self {
            CapabilityType::ComputeOptimization => "compute.optimization".to_string(),
            CapabilityType::GeneticAlgorithms => "compute.genetic_algorithms".to_string(),
            CapabilityType::PerformanceAcceleration => {
                "compute.performance_acceleration".to_string()
            }
            CapabilityType::ParallelProcessing => "compute.parallel_processing".to_string(),
            CapabilityType::QuantumComputing => "compute.quantum".to_string(),

            CapabilityType::Encryption => "security.encryption".to_string(),
            CapabilityType::Decryption => "security.decryption".to_string(),
            CapabilityType::Authentication => "security.authentication".to_string(),
            CapabilityType::Authorization => "security.authorization".to_string(),
            CapabilityType::ThreatDetection => "security.threat_detection".to_string(),
            CapabilityType::ComplianceAuditing => "security.compliance_auditing".to_string(),
            CapabilityType::KeyManagement => "security.key_management".to_string(),

            CapabilityType::FileSystem => "storage.filesystem".to_string(),
            CapabilityType::ObjectStorage => "storage.object_storage".to_string(),
            CapabilityType::VolumeManagement => "storage.volume_management".to_string(),
            CapabilityType::DataReplication => "storage.data_replication".to_string(),
            CapabilityType::BackupServices => "storage.backup_services".to_string(),

            CapabilityType::ServiceDiscovery => "network.service_discovery".to_string(),
            CapabilityType::LoadBalancing => "network.load_balancing".to_string(),
            CapabilityType::NetworkRouting => "network.routing".to_string(),
            CapabilityType::MessageRouting => "network.message_routing".to_string(),
            CapabilityType::RealTimeStreaming => "network.real_time_streaming".to_string(),

            CapabilityType::ModelInference => "ai.model_inference".to_string(),
            CapabilityType::AgentFramework => "ai.agent_framework".to_string(),
            CapabilityType::NaturalLanguageProcessing => {
                "ai.natural_language_processing".to_string()
            }
            CapabilityType::MachineLearning => "ai.machine_learning".to_string(),
            CapabilityType::PatternRecognition => "ai.pattern_recognition".to_string(),

            CapabilityType::ResourceManagement => "system.resource_management".to_string(),
            CapabilityType::ProcessOrchestration => "system.process_orchestration".to_string(),
            CapabilityType::ConfigManagement => "system.config_management".to_string(),
            CapabilityType::HealthMonitoring => "system.health_monitoring".to_string(),
            CapabilityType::MetricsCollection => "system.metrics_collection".to_string(),

            CapabilityType::Gaming {
                low_latency,
                simd_acceleration,
            } => {
                format!(
                    "gaming.optimization:low_latency={},simd={}",
                    low_latency, simd_acceleration
                )
            }
            CapabilityType::GeneticHealing {
                adaptive,
                evolutionary,
            } => {
                format!(
                    "genetic.healing:adaptive={},evolutionary={}",
                    adaptive, evolutionary
                )
            }
            CapabilityType::Custom(name) => format!("custom.{}", name.to_lowercase()),
        }
    }
}

/// Information about discovered module instances
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ModuleInstance {
    /// Anonymous module instance ID
    pub instance_id: String,

    /// Capabilities provided by this module
    pub capabilities: Vec<CapabilityType>,

    /// Module endpoint for communication
    pub endpoint: String,

    /// Current health status
    pub health_status: ModuleHealthStatus,

    /// Performance characteristics
    pub performance: ModulePerformanceMetrics,

    /// Quality metrics
    pub quality_metrics: ModuleQualityMetrics,

    /// Additional metadata
    pub metadata: HashMap<String, String>,
}

/// Module health status
#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum ModuleHealthStatus {
    Healthy,
    Degraded,
    Unhealthy,
    Unknown,
}

/// Performance metrics for discovered modules
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ModulePerformanceMetrics {
    pub average_response_time_ms: f64,
    pub throughput_ops_per_second: u64,
    pub resource_efficiency: f64,
    pub availability_percentage: f64,
}

/// Quality metrics for discovered modules
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ModuleQualityMetrics {
    pub accuracy: f64,
    pub reliability: f64,
    pub consistency: f64,
    pub fault_tolerance: f64,
}

/// Universal module request format
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct UniversalModuleRequest {
    pub request_id: Uuid,
    pub source_module: String,
    pub target_capability: CapabilityType,
    pub operation: String,
    pub payload: serde_json::Value,
    pub quality_requirements: QualityRequirements,
    pub performance_requirements: PerformanceRequirements,
    pub metadata: HashMap<String, String>,
    pub timestamp: DateTime<Utc>,
}

/// Quality requirements for module operations
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct QualityRequirements {
    pub minimum_accuracy: f64,
    pub minimum_reliability: f64,
    pub minimum_availability: f64,
    pub fault_tolerance_required: bool,
}

/// Performance requirements for module operations
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PerformanceRequirements {
    pub max_response_time_ms: u64,
    pub minimum_throughput: u64,
    pub resource_constraints: ResourceConstraints,
}

/// Resource constraints for operations
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ResourceConstraints {
    pub max_cpu_usage: Option<f64>,
    pub max_memory_mb: Option<u64>,
    pub max_network_bandwidth: Option<u64>,
    pub priority: OperationPriority,
}

/// Operation priority levels
#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum OperationPriority {
    Low,
    Medium,
    High,
    Critical,
    RealTime,
}

/// Universal module response format
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct UniversalModuleResponse {
    pub request_id: Uuid,
    pub responding_module: String,
    pub success: bool,
    pub payload: serde_json::Value,
    pub actual_performance: ModulePerformanceMetrics,
    pub actual_quality: ModuleQualityMetrics,
    pub error: Option<String>,
    pub metadata: HashMap<String, String>,
    pub timestamp: DateTime<Utc>,
}

/// Universal capability discovery trait
#[async_trait]
pub trait UniversalCapabilityDiscovery: Send + Sync {
    /// Discover modules by their capabilities
    async fn discover_by_capabilities(
        &self,
        capabilities: &[CapabilityType],
        quality_requirements: Option<QualityRequirements>,
        performance_requirements: Option<PerformanceRequirements>,
    ) -> BearDogResult<Vec<ModuleInstance>>;

    /// Send request to module by capability
    async fn request_by_capability(
        &self,
        request: UniversalModuleRequest,
    ) -> BearDogResult<UniversalModuleResponse>;

    /// Broadcast request to all modules with capability
    async fn broadcast_by_capability(
        &self,
        request: UniversalModuleRequest,
    ) -> BearDogResult<Vec<UniversalModuleResponse>>;

    /// Get health status of modules with capability
    async fn check_capability_health(
        &self,
        capability: CapabilityType,
    ) -> BearDogResult<Vec<(String, ModuleHealthStatus)>>;
}

/// Universal capability discovery implementation
pub struct EcosystemCapabilityDiscovery {
    /// Ecosystem registry client (abstract - could be Songbird, biomeOS, etc.)
    registry_client: Arc<dyn EcosystemRegistryClient>,

    /// Cache of discovered modules
    module_cache: Arc<tokio::sync::RwLock<HashMap<CapabilityType, Vec<ModuleInstance>>>>,

    /// Cache TTL in seconds
    cache_ttl_seconds: u64,
}

/// Abstract ecosystem registry client
#[async_trait]
pub trait EcosystemRegistryClient: Send + Sync {
    async fn find_modules_with_capabilities(
        &self,
        capabilities: &[String],
    ) -> BearDogResult<Vec<ModuleInstance>>;

    async fn send_module_request(
        &self,
        target_module: &str,
        request: UniversalModuleRequest,
    ) -> BearDogResult<UniversalModuleResponse>;

    async fn broadcast_module_request(
        &self,
        target_capability: &str,
        request: UniversalModuleRequest,
    ) -> BearDogResult<Vec<UniversalModuleResponse>>;
}

impl EcosystemCapabilityDiscovery {
    /// Create new universal capability discovery system
    pub fn new(registry_client: Arc<dyn EcosystemRegistryClient>) -> Self {
        Self {
            registry_client,
            module_cache: Arc::new(tokio::sync::RwLock::new(HashMap::new())),
            cache_ttl_seconds: 300, // 5 minutes default
        }
    }

    /// Filter modules by quality requirements
    fn filter_by_quality(
        &self,
        modules: Vec<ModuleInstance>,
        requirements: &QualityRequirements,
    ) -> Vec<ModuleInstance> {
        modules
            .into_iter()
            .filter(|module| {
                module.quality_metrics.accuracy >= requirements.minimum_accuracy
                    && module.quality_metrics.reliability >= requirements.minimum_reliability
                    && module.performance.availability_percentage
                        >= requirements.minimum_availability
                    && (!requirements.fault_tolerance_required
                        || module.quality_metrics.fault_tolerance > 0.8)
            })
            .collect()
    }

    /// Filter modules by performance requirements
    fn filter_by_performance(
        &self,
        modules: Vec<ModuleInstance>,
        requirements: &PerformanceRequirements,
    ) -> Vec<ModuleInstance> {
        modules
            .into_iter()
            .filter(|module| {
                module.performance.average_response_time_ms
                    <= requirements.max_response_time_ms as f64
                    && module.performance.throughput_ops_per_second
                        >= requirements.minimum_throughput
            })
            .collect()
    }

    /// Rank modules by suitability
    fn rank_modules(&self, modules: Vec<ModuleInstance>) -> Vec<ModuleInstance> {
        let mut ranked_modules = modules;

        // Sort by composite score: health, performance, quality
        ranked_modules.sort_by(|a, b| {
            let score_a = self.calculate_module_score(a);
            let score_b = self.calculate_module_score(b);
            score_b
                .partial_cmp(&score_a)
                .unwrap_or(std::cmp::Ordering::Equal)
        });

        ranked_modules
    }

    /// Calculate composite score for module ranking
    fn calculate_module_score(&self, module: &ModuleInstance) -> f64 {
        let health_weight = 0.3;
        let performance_weight = 0.4;
        let quality_weight = 0.3;

        let health_score = match module.health_status {
            ModuleHealthStatus::Healthy => 1.0,
            ModuleHealthStatus::Degraded => 0.7,
            ModuleHealthStatus::Unhealthy => 0.3,
            ModuleHealthStatus::Unknown => 0.5,
        };

        let performance_score = (module.performance.resource_efficiency * 0.4)
            + (module.performance.availability_percentage / 100.0 * 0.4)
            + (1.0 / (1.0 + module.performance.average_response_time_ms / 1000.0) * 0.2);

        let quality_score = (module.quality_metrics.accuracy * 0.3)
            + (module.quality_metrics.reliability * 0.3)
            + (module.quality_metrics.consistency * 0.2)
            + (module.quality_metrics.fault_tolerance * 0.2);

        (health_score * health_weight)
            + (performance_score * performance_weight)
            + (quality_score * quality_weight)
    }
}

#[async_trait]
impl UniversalCapabilityDiscovery for EcosystemCapabilityDiscovery {
    async fn discover_by_capabilities(
        &self,
        capabilities: &[CapabilityType],
        quality_requirements: Option<QualityRequirements>,
        performance_requirements: Option<PerformanceRequirements>,
    ) -> BearDogResult<Vec<ModuleInstance>> {
        info!(
            "🔍 Discovering modules with capabilities: {:?}",
            capabilities
        );

        // Convert capabilities to strings
        let capability_strings: Vec<String> = capabilities
            .iter()
            .map(|cap| cap.as_capability_string())
            .collect();

        // Query ecosystem registry
        let mut discovered_modules = self
            .registry_client
            .find_modules_with_capabilities(&capability_strings)
            .await?;

        debug!(
            "Found {} modules with requested capabilities",
            discovered_modules.len()
        );

        // Apply quality filters if specified
        if let Some(quality_reqs) = quality_requirements {
            discovered_modules = self.filter_by_quality(discovered_modules, &quality_reqs);
            debug!(
                "After quality filtering: {} modules",
                discovered_modules.len()
            );
        }

        // Apply performance filters if specified
        if let Some(perf_reqs) = performance_requirements {
            discovered_modules = self.filter_by_performance(discovered_modules, &perf_reqs);
            debug!(
                "After performance filtering: {} modules",
                discovered_modules.len()
            );
        }

        // Rank by suitability
        discovered_modules = self.rank_modules(discovered_modules);

        info!(
            "✅ Discovered {} suitable modules",
            discovered_modules.len()
        );
        Ok(discovered_modules)
    }

    async fn request_by_capability(
        &self,
        request: UniversalModuleRequest,
    ) -> BearDogResult<UniversalModuleResponse> {
        // First discover modules with the required capability
        let modules = self
            .discover_by_capabilities(
                &[request.target_capability.clone()],
                Some(request.quality_requirements.clone()),
                Some(request.performance_requirements.clone()),
            )
            .await?;

        if modules.is_empty() {
            return Err(BearDogError::External {
                message: format!(
                    "No modules found with capability: {:?}",
                    request.target_capability
                ),
            });
        }

        // Use the best-ranked module
        let target_module = &modules[0];

        info!(
            "📨 Sending request to module: {}",
            target_module.instance_id
        );

        // Send request to the selected module
        self.registry_client
            .send_module_request(&target_module.instance_id, request)
            .await
    }

    async fn broadcast_by_capability(
        &self,
        request: UniversalModuleRequest,
    ) -> BearDogResult<Vec<UniversalModuleResponse>> {
        info!(
            "📢 Broadcasting request to all modules with capability: {:?}",
            request.target_capability
        );

        let capability_string = request.target_capability.as_capability_string();

        self.registry_client
            .broadcast_module_request(&capability_string, request)
            .await
    }

    async fn check_capability_health(
        &self,
        capability: CapabilityType,
    ) -> BearDogResult<Vec<(String, ModuleHealthStatus)>> {
        let modules = self
            .discover_by_capabilities(&[capability], None, None)
            .await?;

        Ok(modules
            .into_iter()
            .map(|module| (module.instance_id, module.health_status))
            .collect())
    }
}

/// Default quality requirements for standard operations
impl Default for QualityRequirements {
    fn default() -> Self {
        Self {
            minimum_accuracy: 0.9,
            minimum_reliability: 0.95,
            minimum_availability: 0.99,
            fault_tolerance_required: false,
        }
    }
}

/// Default performance requirements for standard operations
impl Default for PerformanceRequirements {
    fn default() -> Self {
        Self {
            max_response_time_ms: 5000, // 5 seconds
            minimum_throughput: 10,     // 10 ops/sec minimum
            resource_constraints: ResourceConstraints {
                max_cpu_usage: Some(80.0),        // 80%
                max_memory_mb: Some(1024),        // 1GB
                max_network_bandwidth: Some(100), // 100 Mbps
                priority: OperationPriority::Medium,
            },
        }
    }
}

/// High-performance requirements for gaming and real-time operations
impl PerformanceRequirements {
    pub fn high_performance() -> Self {
        Self {
            max_response_time_ms: 100, // 100ms max
            minimum_throughput: 1000,  // 1000 ops/sec
            resource_constraints: ResourceConstraints {
                max_cpu_usage: Some(50.0),
                max_memory_mb: Some(512),
                max_network_bandwidth: Some(1000), // 1 Gbps
                priority: OperationPriority::RealTime,
            },
        }
    }

    pub fn gaming_optimized() -> Self {
        Self {
            max_response_time_ms: 16, // 60 FPS = 16ms frame time
            minimum_throughput: 2000, // 2000 ops/sec for gaming
            resource_constraints: ResourceConstraints {
                max_cpu_usage: Some(60.0),
                max_memory_mb: Some(2048), // 2GB for gaming
                max_network_bandwidth: Some(1000),
                priority: OperationPriority::RealTime,
            },
        }
    }
}

/// High-quality requirements for critical security operations
impl QualityRequirements {
    pub fn high_security() -> Self {
        Self {
            minimum_accuracy: 0.99,
            minimum_reliability: 0.999,
            minimum_availability: 0.9999,
            fault_tolerance_required: true,
        }
    }
}
