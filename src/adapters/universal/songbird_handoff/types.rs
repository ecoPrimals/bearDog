//! Universal SongBird Handoff Types
//!
//! **Universal data structures for ecosystem service discovery**
//!
//! This module provides universal types for integrating any ecosystem component
//! with SongBird's discovery and orchestration platform. The types are designed
//! to be domain-agnostic and work with any PrimalProvider implementation.

use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use uuid::Uuid;

use super::super::traits::*;

/// Universal primal types for ecosystem standardization
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub enum PrimalType {
    /// ToadStool compute orchestrator
    ToadStool,
    /// Songbird discovery service
    Songbird,
    /// BearDog security provider
    BearDog,
    /// NestGate data management
    NestGate,
    /// Squirrel storage service
    Squirrel,
    /// BiomeOS operating system
    BiomeOS,
}

impl PrimalType {
    pub fn as_str(&self) -> &'static str {
        match self {
            PrimalType::ToadStool => "toadstool",
            PrimalType::Songbird => "songbird",
            PrimalType::BearDog => "beardog",
            PrimalType::NestGate => "nestgate",
            PrimalType::Squirrel => "squirrel",
            PrimalType::BiomeOS => "biomeos",
        }
    }
}

/// Universal ecosystem service registration (standardized across all primals)
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct EcosystemServiceRegistration {
    /// Unique service identifier: "primal-{type}-{instance}"
    pub service_id: String,
    
    /// Primal type from standardized enum
    pub primal_type: PrimalType,
    
    /// Associated biome identifier (if applicable)
    pub biome_id: Option<String>,
    
    /// Service capabilities (standardized format)
    pub capabilities: ServiceCapabilities,
    
    /// API endpoints (standardized format)
    pub endpoints: ServiceEndpoints,
    
    /// Resource requirements
    pub resource_requirements: ResourceSpec,
    
    /// Security configuration
    pub security_config: SecurityConfig,
    
    /// Health check configuration
    pub health_check: HealthCheckConfig,
    
    /// Additional metadata
    pub metadata: HashMap<String, String>,
    
    /// Registration timestamp
    pub registered_at: DateTime<Utc>,
}

/// Universal service capabilities (works with any primal)
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ServiceCapabilities {
    /// Core capabilities (required)
    pub core: Vec<String>,
    /// Extended capabilities (optional)
    pub extended: Vec<String>,
    /// Cross-primal integrations supported
    pub integrations: Vec<String>,
    /// Performance characteristics
    pub performance: PerformanceCapabilities,
}

/// Universal service endpoints (standardized across all primals)
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ServiceEndpoints {
    /// Health check endpoint
    pub health: String,
    /// Metrics endpoint
    pub metrics: String,
    /// Admin/management endpoint
    pub admin: String,
    /// WebSocket endpoint (if supported)
    pub websocket: Option<String>,
    /// Primary API endpoint
    pub primary: String,
}

/// Universal resource specification
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ResourceSpec {
    /// CPU requirements (cores)
    pub cpu_cores: Option<f64>,
    /// Memory requirements (MB)
    pub memory_mb: Option<u64>,
    /// Storage requirements (MB)
    pub storage_mb: Option<u64>,
    /// Network bandwidth (Mbps)
    pub network_mbps: Option<u64>,
    /// GPU requirements (if applicable)
    pub gpu_units: Option<u32>,
}

/// Universal security configuration
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SecurityConfig {
    /// Authentication method
    pub auth_method: AuthMethod,
    /// Encryption requirements
    pub encryption_required: bool,
    /// Security level
    pub security_level: SecurityLevel,
    /// Compliance requirements
    pub compliance: Vec<String>,
}

/// Universal health check configuration
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct HealthCheckConfig {
    /// Health check endpoint path
    pub path: String,
    /// Check interval in seconds
    pub interval_seconds: u64,
    /// Timeout in seconds
    pub timeout_seconds: u64,
    /// Failure threshold
    pub failure_threshold: u32,
}

/// Universal performance capabilities
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PerformanceCapabilities {
    /// Latency characteristics (milliseconds)
    pub latency_ms: Option<u64>,
    /// Throughput characteristics (requests per second)
    pub throughput_rps: Option<u64>,
    /// Concurrency support
    pub max_concurrent_requests: Option<u32>,
}

/// Universal authentication methods
#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum AuthMethod {
    /// No authentication required
    None,
    /// API key authentication
    ApiKey,
    /// Bearer token authentication
    Bearer,
    /// OAuth2 authentication
    OAuth2,
    /// Mutual TLS authentication
    MutualTLS,
    /// Custom authentication method
    Custom(String),
}

/// Universal security levels
#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum SecurityLevel {
    /// Low security level
    Low,
    /// Medium security level
    Medium,
    /// High security level
    High,
    /// Ultimate security level
    Ultimate,
    /// Adaptive security level
    Adaptive,
    /// Optimized security level
    Optimized,
}

/// Universal request format for all ecosystem communication
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct EcosystemRequest {
    /// Unique request identifier
    pub request_id: Uuid,
    
    /// Source service identifier
    pub source_service: String,
    
    /// Target service identifier
    pub target_service: String,
    
    /// Request operation
    pub operation: String,
    
    /// Request payload
    pub payload: serde_json::Value,
    
    /// Security context
    pub security_context: SecurityContext,
    
    /// Request metadata
    pub metadata: HashMap<String, String>,
    
    /// Request timestamp
    pub timestamp: DateTime<Utc>,
}

/// Universal response format
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct EcosystemResponse {
    /// Request ID this response is for
    pub request_id: Uuid,
    
    /// Response status
    pub status: ResponseStatus,
    
    /// Response payload
    pub payload: serde_json::Value,
    
    /// Response metadata
    pub metadata: HashMap<String, String>,
    
    /// Response timestamp
    pub timestamp: DateTime<Utc>,
}

/// Universal response status
#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum ResponseStatus {
    /// Operation completed successfully
    Success,
    /// Operation failed with error
    Error { 
        /// Error code
        code: String, 
        /// Error message
        message: String 
    },
    /// Operation timed out
    Timeout,
    /// Service is unavailable
    ServiceUnavailable,
    /// Unauthorized access
    Unauthorized,
    /// Access forbidden
    Forbidden,
}

/// Universal security context for all requests
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SecurityContext {
    /// Authentication token
    pub auth_token: Option<String>,
    
    /// User/service identity
    pub identity: String,
    
    /// Permissions/capabilities
    pub permissions: Vec<String>,
    
    /// Session information
    pub session_id: Option<String>,
}

/// Universal SongBird handoff configuration
///
/// Configuration for integrating any ecosystem component with SongBird's
/// discovery and orchestration platform.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SongBirdHandoffConfig {
    /// SongBird orchestrator endpoint
    pub songbird_endpoint: String,

    /// API authentication key
    pub api_key: String,

    /// Registration timeout in seconds
    pub registration_timeout_seconds: u64,

    /// Heartbeat interval in seconds
    pub heartbeat_interval_seconds: u64,

    /// Maximum registration retry attempts
    pub max_registration_retries: u32,

    /// Enable automatic re-registration on failure
    pub enable_auto_reregistration: bool,

    /// Service discovery tags
    pub discovery_tags: Vec<String>,

    /// Load balancer algorithm preference
    pub load_balancer_algorithm: LoadBalancingAlgorithm,

    /// Enable circuit breaker for fault tolerance
    pub enable_circuit_breaker: bool,

    /// Health check interval in seconds
    pub health_check_interval_seconds: u64,
}

impl Default for SongBirdHandoffConfig {
    fn default() -> Self {
        Self {
            songbird_endpoint: std::env::var("SONGBIRD_ENDPOINT")
                .unwrap_or_else(|_| "https://songbird.ecosystem.internal".to_string()),
            api_key: std::env::var("SONGBIRD_API_KEY")
                .unwrap_or_else(|_| "default-api-key".to_string()),
            registration_timeout_seconds: 30,
            heartbeat_interval_seconds: 60,
            max_registration_retries: 3,
            enable_auto_reregistration: true,
            discovery_tags: vec!["ecosystem".to_string(), "universal".to_string()],
            load_balancer_algorithm: LoadBalancingAlgorithm::RoundRobin,
            enable_circuit_breaker: true,
            health_check_interval_seconds: 30,
        }
    }
}

/// Universal service registration status
///
/// Tracks the registration state of any ecosystem component with SongBird.
#[derive(Debug, Clone)]
pub struct RegistrationStatus {
    /// Unique registration identifier
    pub registration_id: String,

    /// Current registration state
    pub status: RegistrationState,

    /// Last successful registration timestamp
    pub last_registration: DateTime<Utc>,

    /// Last heartbeat timestamp
    pub last_heartbeat: DateTime<Utc>,

    /// Number of consecutive failures
    pub consecutive_failures: u32,

    /// Next retry attempt time
    pub next_retry: Option<DateTime<Utc>>,
}

/// Universal registration state enumeration
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum RegistrationState {
    /// Not yet registered with SongBird
    NotRegistered,

    /// Registration in progress
    Registering,

    /// Successfully registered and active
    Active,

    /// Registration failed
    Failed,

    /// Temporarily deregistered
    Deregistered,

    /// Circuit breaker opened due to failures
    CircuitBreakerOpen,
}

/// Universal load balancing algorithms
#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum LoadBalancingAlgorithm {
    /// Round-robin load balancing
    RoundRobin,
    /// Least connections load balancing
    LeastConnections,
    /// Weighted round-robin load balancing
    WeightedRoundRobin,
    /// Resource-based load balancing
    ResourceBased,
    /// Latency-based load balancing
    LatencyBased,
}

/// Universal service registration result
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ServiceRegistrationResult {
    /// Registration success status
    pub success: bool,
    
    /// Assigned service identifier
    pub service_id: String,
    
    /// Registration details
    pub registration_details: EcosystemServiceRegistration,
    
    /// Error message if registration failed
    pub error_message: Option<String>,
}

/// Universal advertised service structure
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct AdvertisedService {
    /// Service registration information
    pub registration: EcosystemServiceRegistration,
    
    /// Current service health
    pub health: ServiceHealth,
    
    /// Load balancer configuration
    pub load_balancer_config: LoadBalancerConfig,
    
    /// Orchestration metadata
    pub orchestration: OrchestrationMetadata,
}

/// Universal service health status
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ServiceHealth {
    /// Overall health status
    pub status: HealthStatus,
    
    /// Health check timestamp
    pub last_check: DateTime<Utc>,
    
    /// Performance metrics
    pub metrics: PerformanceMetrics,
    
    /// Error details if unhealthy
    pub error_details: Option<String>,
}

/// Universal health status enumeration
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub enum HealthStatus {
    /// Service is healthy
    Healthy,
    /// Service is degraded but functional
    Degraded,
    /// Service is unhealthy
    Unhealthy,
    /// Health status is unknown
    Unknown,
}

/// Universal performance metrics
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PerformanceMetrics {
    /// CPU utilization percentage
    pub cpu_percent: f64,
    
    /// Memory utilization percentage
    pub memory_percent: f64,
    
    /// Request latency in milliseconds
    pub latency_ms: u64,
    
    /// Requests per second
    pub requests_per_second: f64,
    
    /// Error rate percentage
    pub error_rate_percent: f64,
}

/// Universal load balancer configuration
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct LoadBalancerConfig {
    /// Load balancing algorithm
    pub algorithm: LoadBalancingAlgorithm,
    
    /// Service weight for weighted algorithms
    pub weight: u32,
    
    /// Maximum concurrent requests
    pub max_requests: u32,
    
    /// Circuit breaker settings
    pub circuit_breaker: CircuitBreakerConfig,
}

/// Universal circuit breaker configuration
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CircuitBreakerConfig {
    /// Failure threshold to open circuit
    pub failure_threshold: u32,
    
    /// Timeout before trying to close circuit
    pub timeout_seconds: u64,
    
    /// Success threshold to close circuit
    pub success_threshold: u32,
}

/// Universal orchestration metadata
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct OrchestrationMetadata {
    /// Routing rules
    pub routing_rules: Vec<RoutingRule>,
    
    /// Scaling policies
    pub scaling_policies: Vec<ScalingPolicy>,
    
    /// Affinity rules
    pub affinity_rules: Vec<AffinityRule>,
}

/// Universal routing rule
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct RoutingRule {
    /// Rule condition
    pub condition: String,
    
    /// Target service
    pub target: String,
    
    /// Rule weight
    pub weight: u32,
}

/// Universal scaling policy
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ScalingPolicy {
    /// Metric to scale on
    pub metric: String,
    
    /// Threshold value
    pub threshold: f64,
    
    /// Scaling action
    pub action: ScalingAction,
}

/// Universal scaling action
#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum ScalingAction {
    /// Scale up by specified number of instances
    ScaleUp(u32),
    /// Scale down by specified number of instances
    ScaleDown(u32),
    /// Scale to zero instances
    ScaleToZero,
}

/// Universal affinity rule
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct AffinityRule {
    /// Affinity type
    pub affinity_type: AffinityType,
    
    /// Target selection
    pub target: String,
    
    /// Rule weight
    pub weight: u32,
}

/// Universal affinity type
#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum AffinityType {
    /// Node affinity
    NodeAffinity,
    /// Service affinity
    ServiceAffinity,
    /// Anti-affinity
    AntiAffinity,
}

/// Universal service endpoint
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ServiceEndpoint {
    /// Endpoint URL
    pub url: String,
    
    /// Endpoint type
    pub endpoint_type: EndpointType,
    
    /// Protocol used
    pub protocol: String,
    
    /// Port number
    pub port: u16,
}

/// Universal endpoint type
#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum EndpointType {
    /// Primary API endpoint
    Primary,
    /// Health check endpoint
    Health,
    /// Metrics endpoint
    Metrics,
    /// Admin/management endpoint
    Admin,
    /// WebSocket endpoint
    WebSocket,
    /// Custom endpoint type
    Custom(String),
}

/// Universal health summary
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct HealthSummary {
    /// Overall health status
    pub overall_status: HealthStatus,
    
    /// Component health details
    pub components: HashMap<String, ComponentHealth>,
    
    /// System performance metrics
    pub performance: PerformanceMetrics,
    
    /// Health check timestamp
    pub timestamp: DateTime<Utc>,
}

/// Universal component health
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ComponentHealth {
    /// Component name
    pub name: String,
    
    /// Health status
    pub status: HealthStatus,
    
    /// Health details
    pub details: Option<String>,
    
    /// Last check timestamp
    pub last_check: DateTime<Utc>,
}

/// Universal health monitor configuration
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct HealthMonitorConfig {
    /// Health check interval
    pub check_interval_seconds: u64,
    
    /// Health check timeout
    pub timeout_seconds: u64,
    
    /// Failure threshold
    pub failure_threshold: u32,
    
    /// Recovery threshold
    pub recovery_threshold: u32,
}

impl Default for HealthMonitorConfig {
    fn default() -> Self {
        Self {
            check_interval_seconds: 30,
            timeout_seconds: 5,
            failure_threshold: 3,
            recovery_threshold: 2,
        }
    }
}
