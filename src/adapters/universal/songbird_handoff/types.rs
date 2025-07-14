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
    
    /// Currently attempting registration
    Registering,
    
    /// Successfully registered
    Registered,
    
    /// Registration failed
    Failed,
    
    /// Registration expired
    Expired,
}

/// Universal advertised service structure
///
/// Represents how any ecosystem component advertises its capabilities to SongBird.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct AdvertisedService {
    /// Unique service identifier
    pub service_id: String,
    
    /// Human-readable service name
    pub service_name: String,
    
    /// Service capabilities offered to the ecosystem
    pub capabilities: Vec<Capability>,
    
    /// Service endpoints for communication
    pub endpoints: Vec<ServiceEndpoint>,
    
    /// Health check endpoint URL
    pub health_check_url: String,
    
    /// Discovery tags for service categorization
    pub discovery_tags: Vec<String>,
    
    /// Load balancer configuration
    pub load_balancer_config: LoadBalancerConfig,
    
    /// Orchestration metadata
    pub orchestration_metadata: OrchestrationMetadata,
}

/// Universal service endpoint
///
/// Represents a communication endpoint for any ecosystem component.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ServiceEndpoint {
    /// Protocol (http, https, grpc, etc.)
    pub protocol: String,
    
    /// Network address
    pub address: String,
    
    /// Port number
    pub port: u16,
    
    /// Optional path for HTTP endpoints
    pub path: Option<String>,
    
    /// Load balancing weight
    pub weight: u32,
    
    /// Whether this endpoint supports health checks
    pub health_check: bool,
}

/// Universal load balancer configuration
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct LoadBalancerConfig {
    /// Load balancing algorithm
    pub algorithm: LoadBalancingAlgorithm,
    
    /// Health check interval in seconds
    pub health_check_interval_seconds: u64,
    
    /// Maximum retry attempts
    pub max_retries: u32,
    
    /// Request timeout in seconds
    pub timeout_seconds: u64,
    
    /// Enable circuit breaker pattern
    pub circuit_breaker_enabled: bool,
}

impl Default for LoadBalancerConfig {
    fn default() -> Self {
        Self {
            algorithm: LoadBalancingAlgorithm::RoundRobin,
            health_check_interval_seconds: 30,
            max_retries: 3,
            timeout_seconds: 30,
            circuit_breaker_enabled: true,
        }
    }
}

/// Universal load balancing algorithms
#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum LoadBalancingAlgorithm {
    /// Round-robin distribution
    RoundRobin,
    
    /// Weighted round-robin distribution
    WeightedRoundRobin,
    
    /// Least connections algorithm
    LeastConnections,
    
    /// IP hash-based routing
    IpHash,
    
    /// Random selection
    Random,
    
    /// Performance-based routing
    PerformanceBased,
}

/// Universal orchestration metadata
///
/// Provides orchestration configuration for any ecosystem component.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct OrchestrationMetadata {
    /// Request routing rules
    pub routing_rules: Vec<RoutingRule>,
    
    /// Auto-scaling policies
    pub scaling_policies: Vec<ScalingPolicy>,
    
    /// Service affinity rules
    pub affinity_rules: Vec<AffinityRule>,
    
    /// Security policy identifiers
    pub security_policies: Vec<String>,
    
    /// Monitoring configuration
    pub monitoring_config: MonitoringConfig,
}

/// Universal routing rule
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct RoutingRule {
    /// Unique rule identifier
    pub rule_id: String,
    
    /// Routing condition
    pub condition: RoutingCondition,
    
    /// Routing action
    pub action: RoutingAction,
    
    /// Rule priority (higher number = higher priority)
    pub priority: u32,
}

/// Universal routing conditions
#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum RoutingCondition {
    /// Path prefix matching
    PathPrefix(String),
    
    /// Header matching
    Header { name: String, value: String },
    
    /// Query parameter matching
    QueryParam { name: String, value: String },
    
    /// Client IP matching
    ClientIp(String),
    
    /// Request size constraints
    RequestSize { min: Option<u64>, max: Option<u64> },
    
    /// Capability-based routing
    Capability(String),
}

/// Universal routing actions
#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum RoutingAction {
    /// Route to specific endpoint
    RouteToEndpoint(String),
    
    /// Route to capability provider
    RouteToCapability(String),
    
    /// Load balance across endpoints
    LoadBalance(Vec<String>),
    
    /// Reject request
    Reject { code: u16, message: String },
    
    /// Apply rate limiting
    RateLimitApply { requests_per_second: u64 },
}

/// Universal scaling policy
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ScalingPolicy {
    /// Unique policy identifier
    pub policy_id: String,
    
    /// Scaling metric
    pub metric: ScalingMetric,
    
    /// Threshold value
    pub threshold: f64,
    
    /// Scaling action
    pub action: ScalingAction,
    
    /// Cooldown period in seconds
    pub cooldown_seconds: u64,
}

/// Universal scaling metrics
#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum ScalingMetric {
    /// CPU utilization percentage
    CpuUtilization,
    
    /// Memory utilization percentage
    MemoryUtilization,
    
    /// Request rate (requests per second)
    RequestRate,
    
    /// Response time (milliseconds)
    ResponseTime,
    
    /// Error rate percentage
    ErrorRate,
    
    /// Queue length
    QueueLength,
}

/// Universal scaling actions
#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum ScalingAction {
    /// Scale up by number of instances
    ScaleUp { instances: u32 },
    
    /// Scale down by number of instances
    ScaleDown { instances: u32 },
    
    /// Auto-scale within limits
    AutoScale { min: u32, max: u32 },
}

/// Universal affinity rule
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct AffinityRule {
    /// Unique rule identifier
    pub rule_id: String,
    
    /// Affinity type
    pub affinity_type: AffinityType,
    
    /// Target identifier
    pub target: String,
    
    /// Affinity weight
    pub weight: f64,
}

/// Universal affinity types
#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum AffinityType {
    /// Node affinity
    NodeAffinity,
    
    /// Service affinity
    ServiceAffinity,
    
    /// Anti-affinity
    AntiAffinity,
    
    /// Zone affinity
    ZoneAffinity,
    
    /// Region affinity
    RegionAffinity,
}

/// Universal monitoring configuration
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct MonitoringConfig {
    /// Enable metrics collection
    pub metrics_enabled: bool,
    
    /// Enable distributed tracing
    pub tracing_enabled: bool,
    
    /// Logging level
    pub logging_level: String,
    
    /// Custom metrics configuration
    pub custom_metrics: Vec<CustomMetric>,
    
    /// Alert rules
    pub alert_rules: Vec<AlertRule>,
}

/// Universal custom metric
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CustomMetric {
    /// Metric name
    pub name: String,
    
    /// Metric type
    pub metric_type: MetricType,
    
    /// Metric description
    pub description: String,
    
    /// Metric tags
    pub tags: HashMap<String, String>,
}

/// Universal metric types
#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum MetricType {
    /// Counter metric
    Counter,
    
    /// Gauge metric
    Gauge,
    
    /// Histogram metric
    Histogram,
    
    /// Summary metric
    Summary,
}

/// Universal alert rule
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct AlertRule {
    /// Unique rule identifier
    pub rule_id: String,
    
    /// Alert condition
    pub condition: String,
    
    /// Threshold value
    pub threshold: f64,
    
    /// Alert severity
    pub severity: AlertSeverity,
    
    /// Notification channels
    pub notification_channels: Vec<String>,
}

/// Universal alert severity levels
#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum AlertSeverity {
    /// Informational alert
    Info,
    
    /// Warning alert
    Warning,
    
    /// Critical alert
    Critical,
    
    /// Emergency alert
    Emergency,
}

/// Universal service health status
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ServiceHealth {
    /// Health status
    pub status: HealthStatus,
    
    /// Last health check timestamp
    pub last_check: DateTime<Utc>,
    
    /// Response time in milliseconds
    pub response_time_ms: u64,
    
    /// Error count
    pub error_count: u64,
    
    /// Uptime percentage
    pub uptime_percentage: f64,
}

/// Universal service registration result
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ServiceRegistrationResult {
    /// Registration identifier
    pub registration_id: String,
    
    /// Service discovery URL
    pub service_discovery_url: String,
    
    /// Load balancer endpoints
    pub load_balancer_endpoints: Vec<String>,
    
    /// Monitoring dashboard URL
    pub monitoring_dashboard_url: String,
    
    /// Applied orchestration policies
    pub orchestration_policies_applied: Vec<String>,
} 