// Universal adapter types for capability-based service handoff
//
// This module provides sovereignty-compliant types that use capabilities
// instead of hardcoded primal references.

// MODERNIZATION NOTE: This file contains primal-specific references that should be migrated
// to universal adapter patterns. See migration guide: docs/guides/UNIVERSAL_ADAPTER_USAGE_GUIDE.md
// Target: Replace with capability-based discovery for vendor/primal agnosticism
use chrono::{DateTime, Utc};
use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use uuid::Uuid;

/// Import ServiceCapabilityType from canonical location
pub use beardog_types::canonical::capabilities::ServiceCapabilityType;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ServiceRegistration {
    pub service_id: Uuid,
    /// Collection of capabilities
    pub capabilities: Vec<ServiceCapabilityType>,
    pub biome_id: Option<String>,
    /// The service capabilities value
    pub service_capabilities: ServiceCapabilities,
    /// The endpoints value
    pub endpoints: ServiceEndpoints,
    /// The resource requirements value
    pub resource_requirements: ResourceSpec,
    pub security_config: SecurityConfig,
    /// The health check value
    pub health_check: HealthCheckConfig,
    /// Mapping of metadata
    pub metadata: HashMap<String, String>,
    /// The registered at value
    pub registered_at: DateTime<Utc>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ServiceCapabilities {
    /// Collection of core
    pub core: Vec<String>,
    /// Collection of extended
    pub extended: Vec<String>,
    /// Collection of integrations
    pub integrations: Vec<String>,
    pub performance: PerformanceCapabilities,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ServiceEndpoints {
    /// The health value
    pub health: String,
    /// The metrics value
    pub metrics: String,
    /// The admin value
    pub admin: String,
    /// Optional websocket
    pub websocket: Option<String>,
    /// The primary value
    pub primary: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ResourceSpec {
    /// Optional cpu cores
    pub cpu_cores: Option<f64>,
    /// Optional memory mb
    pub memory_mb: Option<u64>,
    /// Optional storage mb
    pub storage_mb: Option<u64>,
    /// Optional network mbps
    pub network_mbps: Option<u64>,
    /// Optional gpu units
    pub gpu_units: Option<u32>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SecurityContext {
    /// Optional auth token
    pub auth_token: Option<String>,
    pub identity: String,
    /// Collection of permissions
    pub permissions: Vec<String>,
    pub session_id: Option<String>,
}

pub use beardog_types::canonical::configuration::HandoffConfig;

///
///
/// 🎯 UNIVERSAL PATTERN:
///
/// ✅ IMPLEMENTATION:
/// ```rust
/// let universal_adapter = UniversalAdapter::new()?;
/// let mesh_providers = universal_adapter
///     .discover_capability(ServiceCapabilityType::ServiceMesh)
///     ?;
/// let handoff_config = ServiceMeshHandoffConfig::from_discovered_providers(mesh_providers);
/// ```
/// Universal service mesh handoff configuration (replaces hardcoded primal references)
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ServiceMeshHandoffConfig {
    /// Discovered service mesh endpoints (no hardcoded primal names)
    /// Collection of mesh endpoints
    pub mesh_endpoints: Vec<ServiceMeshEndpoint>,
    /// Discovery configuration
    pub discovery_config: MeshDiscoveryConfig,
    /// Handoff retry configuration
    pub retry_config: HandoffRetryConfig,
    pub security_config: MeshSecurityConfig,
}

/// Service mesh endpoint (discovered, not hardcoded)
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ServiceMeshEndpoint {
    /// Endpoint identifier (learned through discovery)
    pub endpoint_id: String,
    /// The base url value
    pub base_url: String,
    /// Capabilities provided by this endpoint
    /// Collection of capabilities
    pub capabilities: Vec<String>,
    /// Trust level (built through successful interactions)
    /// The trust level value
    pub trust_level: f64,
    pub performance_metrics: EndpointPerformanceMetrics,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct MeshDiscoveryConfig {
    /// Discovery interval in seconds
    /// Number of discovery_interval_secs
    pub discovery_interval_secs: u64,
    /// Maximum discovery attempts
    /// Number of max_discovery_attempts
    pub max_discovery_attempts: u32,
    /// Discovery timeout in milliseconds
    pub discovery_timeout_ms: u64,
}

/// Handoff retry configuration
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct HandoffRetryConfig {
    /// Maximum retry attempts
    /// Number of max_retries
    pub max_retries: u32,
    /// Base retry delay in milliseconds
    /// Number of base_delay_ms
    pub base_delay_ms: u64,
    /// Exponential backoff multiplier
    /// The backoff multiplier value
    pub backoff_multiplier: f64,
    /// Maximum retry delay in milliseconds
    /// Number of max_delay_ms
    pub max_delay_ms: u64,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct MeshSecurityConfig {
    /// Whether require_tls is enabled
    pub require_tls: bool,
    /// Certificate validation mode
    pub cert_validation: CertValidationMode,
    /// Authentication method
    /// The auth method value
    pub auth_method: MeshAuthMethod,
}

/// Certificate validation modes
#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum CertValidationMode {
    /// Full certificate validation
    Full,
    /// Skip hostname verification
    SkipHostname,
    None,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum MeshAuthMethod {
    /// No authentication
    None,
    /// Mutual TLS
    MutualTls { cert_path: String, key_path: String },
    MutualTls { cert_path: String, key_path: String },
    MutualTls { cert_path: String, key_path: String },
    /// Bearer token
    BearerToken { token: String },
    /// API key
    ApiKey { key: String, header: String },
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct EndpointPerformanceMetrics {
    /// Average response time in milliseconds
    pub avg_response_time_ms: f64,
    /// Success rate (0.0 to 1.0)
    /// The success rate value
    pub success_rate: f64,
    /// Throughput in requests per second
    /// The throughput rps value
    pub throughput_rps: f64,
    /// Last measured at
    /// The last measured at value
    pub last_measured_at: DateTime<Utc>,
}

impl Default for ServiceMeshHandoffConfig {
    fn default() -> Self {
        Self {
            mesh_endpoints: Vec::new(),
            discovery_config: MeshDiscoveryConfig {
                discovery_interval_secs: 30,
                max_discovery_attempts: 3,
                discovery_timeout_ms: 5000,
            },
            retry_config: HandoffRetryConfig {
                max_retries: 3,
                base_delay_ms: 100,
                backoff_multiplier: 2.0,
                max_delay_ms: 5000,
            },
            security_config: MeshSecurityConfig {
                require_tls: true,
                cert_validation: CertValidationMode::Full,
                auth_method: MeshAuthMethod::None,
            },
        }
    }
}

impl Default for EndpointPerformanceMetrics {
    fn default() -> Self {
        Self {
            avg_response_time_ms: 0.0,
            success_rate: 0.0,
            throughput_rps: 0.0,
            last_measured_at: Utc::now(),
        }
    }
}

#[deprecated = "Use ServiceMeshHandoffConfig with capability-based discovery"]
impl Default for SongBirdHandoffConfig {}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct RegistrationState {
    /// Current status of the component
    pub status: RegistrationState,
    /// The last registration value
    pub last_registration: DateTime<Utc>,
    /// The last heartbeat value
    pub last_heartbeat: DateTime<Utc>,
    /// Number of consecutive_failures
    pub consecutive_failures: u32,
    /// Optional next retry
    pub next_retry: Option<DateTime<Utc>>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct EcosystemServiceRegistration {
    /// The registration details value
    pub registration_details: EcosystemServiceRegistration,
    /// Optional error message
    pub error_message: Option<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct AdvertisedService {
    /// The registration value
    pub registration: EcosystemServiceRegistration,
    /// The health value
    pub health: ServiceHealth,
    pub load_balancer_config: LoadBalancerConfig,
    /// The orchestration value
    pub orchestration: OrchestrationMetadata,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ServiceHealth {
    /// Current status of the component
    pub status: HealthStatus,
    /// The last check value
    pub last_check: DateTime<Utc>,
    /// The metrics value
    pub metrics: PerformanceMetrics,
    /// Optional error details
    pub error_details: Option<String>,
}

pub use beardog_types::canonical::HealthStatus;
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum HealthStatus {
    /// Represents healthy variant
    Healthy,
    /// State indicating degraded
    Degraded,
    /// Represents unhealthy variant
    Unhealthy,
    /// Unknown or undefined state
    Unknown,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PerformanceMetrics {
    /// The cpu percent value
    pub cpu_percent: f64,
    /// The memory percent value
    pub memory_percent: f64,
    /// Number of latency_ms
    pub latency_ms: u64,
    /// The requests per second value
    pub requests_per_second: f64,
    /// The error rate percent value
    pub error_rate_percent: f64,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct AffinityRule {
    /// The affinity type value
    pub affinity_type: AffinityType,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
/// Types of affinity
pub enum AffinityType {
    /// Represents node affinity variant
    NodeAffinity,
    /// Represents service affinity variant
    ServiceAffinity,
    /// Represents anti affinity variant
    AntiAffinity,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ServiceEndpoint {
    /// The url value
    pub url: String,
    /// The endpoint type value
    pub endpoint_type: EndpointType,
    /// The protocol value
    pub protocol: String,
    /// Number of port
    pub port: u16,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
/// Types of endpoint
pub enum EndpointType {
    /// Represents primary variant
    Primary,
    /// Represents health variant
    Health,
    /// Represents metrics variant
    Metrics,
    /// Represents admin variant
    Admin,
    /// Represents web socket variant
    WebSocket,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct HealthSummary {
    /// Current status of the overall
    pub overall_status: HealthStatus,
    /// Mapping of components
    pub components: HashMap<String, ComponentHealth>,
    pub performance: PerformanceMetrics,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ComponentHealth {
    /// Name of the item
    pub name: String,
    /// Optional details
    pub details: Option<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct HealthMonitorConfig {
    /// Number of check_interval_seconds
    pub check_interval_seconds: u64,
    pub timeout_seconds: u64,
    /// Number of failure_threshold
    pub failure_threshold: u64,
    /// Number of recovery_threshold
    pub recovery_threshold: u64,
}
