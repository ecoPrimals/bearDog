//! Universal Capability-Based Ecosystem Integration
//!
//! This module implements the Universal Primal Architecture Standard for capability-based
//! service discovery and integration. No hardcoded service names or types.

use async_trait::async_trait;
use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use uuid::Uuid;

use crate::{AIFirstResponse, EcosystemResult};

pub mod beardog_provider;
pub mod capability_registry;
pub mod service_discovery;

pub use beardog_provider::*;
pub use capability_registry::*;
pub use service_discovery::*;

/// Universal Service Registration following ecosystem standard
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct UniversalServiceRegistration {
    /// Unique service identifier (UUID)
    pub service_id: Uuid,

    /// Service metadata
    pub metadata: ServiceMetadata,

    /// Capabilities this service provides
    pub capabilities: Vec<ServiceCapability>,

    /// Resource requirements and limits
    pub resources: ResourceSpec,

    /// API endpoints (dynamically discovered)
    pub endpoints: Vec<ServiceEndpoint>,

    /// Integration preferences
    pub integration: IntegrationPreferences,

    /// Extension points for custom data
    pub extensions: HashMap<String, serde_json::Value>,

    /// Registration timestamp
    pub registration_timestamp: DateTime<Utc>,

    /// Service version
    pub service_version: String,

    /// Instance identifier for multi-instance support
    pub instance_id: String,

    /// Priority level for load balancing
    pub priority: u8,
}

/// Service metadata with open categorization
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ServiceMetadata {
    /// Human-readable service name
    pub name: String,

    /// Service category (extensible)
    pub category: ServiceCategory,

    /// Version information
    pub version: String,

    /// Service description
    pub description: String,

    /// Maintainer information
    pub maintainer: String,

    /// License information
    pub license: String,

    /// Tags for categorization
    pub tags: Vec<String>,
}

/// Service category - open enumeration
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(tag = "type", content = "data")]
pub enum ServiceCategory {
    /// Security-related services
    Security { subcategory: String },
    /// Storage-related services  
    Storage { subcategory: String },
    /// Compute-related services
    Compute { subcategory: String },
    /// Network-related services
    Network { subcategory: String },
    /// AI/ML-related services
    ArtificialIntelligence { subcategory: String },
    /// Orchestration services
    Orchestration { subcategory: String },
    /// Monitoring services
    Monitoring { subcategory: String },
    /// Custom category (community extensible)
    Custom {
        category: String,
        subcategory: String,
        metadata: HashMap<String, serde_json::Value>,
    },
}

/// Service capability definition
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ServiceCapability {
    /// Capability identifier
    pub capability_id: String,

    /// Capability name
    pub name: String,

    /// Capability version
    pub version: String,

    /// Capability description
    pub description: String,

    /// Input/output schema
    pub schema: CapabilitySchema,

    /// Performance characteristics
    pub performance: PerformanceCharacteristics,

    /// Security requirements
    pub security_requirements: SecurityRequirements,
}

/// Capability schema for API contract
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CapabilitySchema {
    /// Input schema (JSON Schema)
    pub input_schema: serde_json::Value,

    /// Output schema (JSON Schema)
    pub output_schema: serde_json::Value,

    /// Error schema (JSON Schema)
    pub error_schema: serde_json::Value,
}

/// Performance characteristics
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PerformanceCharacteristics {
    /// Expected latency in milliseconds
    pub expected_latency_ms: u64,

    /// Maximum throughput per second
    pub max_throughput_per_sec: u64,

    /// Resource requirements
    pub resource_requirements: ResourceSpec,

    /// Scalability characteristics
    pub scalability: ScalabilitySpec,
}

/// Resource specification
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ResourceSpec {
    /// CPU requirements (cores)
    pub cpu_cores: f64,

    /// Memory requirements (MB)
    pub memory_mb: u64,

    /// Storage requirements (MB)
    pub storage_mb: u64,

    /// Network bandwidth (Kbps)
    pub network_kbps: u64,

    /// Custom resource requirements
    pub custom: HashMap<String, serde_json::Value>,
}

/// Scalability specification
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ScalabilitySpec {
    /// Minimum instances
    pub min_instances: u32,

    /// Maximum instances
    pub max_instances: u32,

    /// Auto-scaling enabled
    pub auto_scaling: bool,

    /// Scaling triggers
    pub scaling_triggers: Vec<ScalingTrigger>,
}

/// Scaling trigger definition
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ScalingTrigger {
    /// Metric to monitor
    pub metric: String,

    /// Threshold value
    pub threshold: f64,

    /// Comparison operator
    pub operator: ComparisonOperator,

    /// Action to take
    pub action: ScalingAction,
}

/// Comparison operators for scaling
#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum ComparisonOperator {
    GreaterThan,
    LessThan,
    GreaterThanOrEqual,
    LessThanOrEqual,
    Equal,
    NotEqual,
}

/// Scaling actions
#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum ScalingAction {
    ScaleUp { instances: u32 },
    ScaleDown { instances: u32 },
    Custom { action: String },
}

/// Security requirements for capabilities
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SecurityRequirements {
    /// Authentication required
    pub authentication_required: bool,

    /// Authorization level required
    pub authorization_level: String,

    /// Encryption required
    pub encryption_required: bool,

    /// Audit logging required
    pub audit_logging: bool,

    /// Custom security requirements
    pub custom: HashMap<String, serde_json::Value>,
}

/// Service endpoint definition
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ServiceEndpoint {
    /// Endpoint ID
    pub endpoint_id: String,

    /// Endpoint URL
    pub url: String,

    /// HTTP method
    pub method: String,

    /// Capabilities provided by this endpoint
    pub capabilities: Vec<String>,

    /// Endpoint schema
    pub schema: CapabilitySchema,

    /// Health check configuration
    pub health_check: Option<HealthCheckConfig>,
}

/// Health check configuration
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct HealthCheckConfig {
    /// Health check URL
    pub url: String,

    /// Check interval in seconds
    pub interval_secs: u64,

    /// Timeout in seconds
    pub timeout_secs: u64,

    /// Expected response codes
    pub expected_status_codes: Vec<u16>,
}

/// Integration preferences
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct IntegrationPreferences {
    /// Preferred communication protocols
    pub preferred_protocols: Vec<String>,

    /// Load balancing preferences
    pub load_balancing: LoadBalancingPreferences,

    /// Retry configuration
    pub retry_config: RetryConfig,

    /// Circuit breaker configuration
    pub circuit_breaker: Option<CircuitBreakerConfig>,
}

/// Load balancing preferences
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct LoadBalancingPreferences {
    /// Load balancing algorithm
    pub algorithm: LoadBalancingAlgorithm,

    /// Weight for weighted algorithms
    pub weight: Option<u32>,

    /// Sticky sessions
    pub sticky_sessions: bool,
}

/// Load balancing algorithms
#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum LoadBalancingAlgorithm {
    RoundRobin,
    WeightedRoundRobin,
    LeastConnections,
    Random,
    IpHash,
    Custom { algorithm: String },
}

/// Retry configuration
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct RetryConfig {
    /// Maximum retries
    pub max_retries: u32,

    /// Base delay in milliseconds
    pub base_delay_ms: u64,

    /// Maximum delay in milliseconds
    pub max_delay_ms: u64,

    /// Backoff strategy
    pub backoff_strategy: BackoffStrategy,

    /// Retryable error codes
    pub retryable_errors: Vec<String>,
}

/// Backoff strategies
#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum BackoffStrategy {
    Fixed,
    Linear,
    Exponential { multiplier: f64 },
    Custom { strategy: String },
}

/// Circuit breaker configuration
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CircuitBreakerConfig {
    /// Failure threshold
    pub failure_threshold: u32,

    /// Recovery timeout in milliseconds
    pub recovery_timeout_ms: u64,

    /// Success threshold for recovery
    pub success_threshold: u32,
}

/// Universal trait for ecosystem integration
#[async_trait]
pub trait EcosystemIntegration: Send + Sync {
    /// Register this service in the ecosystem
    async fn register(&self) -> EcosystemResult<UniversalServiceRegistration>;

    /// Discover services by capability
    async fn discover_by_capability(
        &self,
        capability: &str,
    ) -> EcosystemResult<Vec<UniversalServiceRegistration>>;

    /// Get service health status
    async fn health_check(&self) -> EcosystemResult<HealthStatus>;

    /// Handle ecosystem requests
    async fn handle_request(
        &self,
        request: EcosystemRequest,
    ) -> EcosystemResult<AIFirstResponse<serde_json::Value>>;
}

/// Health status information
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct HealthStatus {
    /// Overall health
    pub status: HealthLevel,

    /// Health checks
    pub checks: Vec<HealthCheck>,

    /// Last updated timestamp
    pub last_updated: DateTime<Utc>,

    /// Version information
    pub version: String,
}

/// Health levels
#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum HealthLevel {
    Healthy,
    Degraded,
    Unhealthy,
    Unknown,
}

/// Individual health check
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct HealthCheck {
    /// Check name
    pub name: String,

    /// Check status
    pub status: HealthLevel,

    /// Check details
    pub details: Option<String>,

    /// Response time in milliseconds
    pub response_time_ms: Option<u64>,
}

/// Universal ecosystem request
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct EcosystemRequest {
    /// Request ID for tracing
    pub request_id: Uuid,

    /// Capability being requested
    pub capability: String,

    /// Request payload
    pub payload: serde_json::Value,

    /// Request metadata
    pub metadata: HashMap<String, serde_json::Value>,

    /// Security context
    pub security_context: Option<SecurityContext>,

    /// Timestamp
    pub timestamp: DateTime<Utc>,
}

/// Security context for requests
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SecurityContext {
    /// Authentication token
    pub auth_token: Option<String>,

    /// User identity
    pub user_id: Option<String>,

    /// Permission level
    pub permission_level: String,

    /// Request origin
    pub origin: Option<String>,
}
