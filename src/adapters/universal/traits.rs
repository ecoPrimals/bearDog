//! Universal Ecosystem Adapter Traits
//!
//! **Universal, domain-agnostic traits for ecosystem integration**
//!
//! This module provides truly universal patterns that any ecosystem component
//! can implement, regardless of their domain (security, compute, storage, AI, etc.).
//! It follows SongBird's established universal patterns rather than creating
//! BearDog-centric interfaces.

use async_trait::async_trait;
use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use uuid::Uuid;

use crate::BearDogResult;

/// Universal trait that any ecosystem component can implement
/// Based on SongBird's PrimalProvider pattern
#[async_trait]
pub trait PrimalProvider: Send + Sync {
    /// Unique ecosystem identifier (toadstool, songbird, nestgate, beardog, squirrel, biomeos)
    fn ecosystem_id(&self) -> &str;

    /// Instance identifier for multi-instance support
    fn instance_id(&self) -> &str;

    /// Human-readable service name
    fn service_name(&self) -> &str;

    /// Service version
    fn service_version(&self) -> &str;

    /// Capabilities this provider offers to the ecosystem
    fn capabilities(&self) -> Vec<Capability>;

    /// Dependencies this provider requires from other ecosystem components
    fn dependencies(&self) -> Vec<Dependency>;

    /// Service endpoints for communication
    fn endpoints(&self) -> ServiceEndpoints;

    /// Current health status
    async fn health_check(&self) -> HealthStatus;

    /// Handle a generic service request
    async fn handle_request(&self, request: ServiceRequest) -> BearDogResult<ServiceResponse>;

    /// Register with ecosystem discovery (SongBird)
    async fn register_with_ecosystem(&self) -> BearDogResult<EcosystemRegistration>;

    /// Initialize the provider with configuration
    async fn initialize(&mut self, config: ProviderConfig) -> BearDogResult<()>;

    /// Graceful shutdown
    async fn shutdown(&mut self) -> BearDogResult<()>;

    /// Check if this provider can handle the given request
    fn can_handle_request(&self, request: &ServiceRequest) -> bool;

    /// Get metadata about this provider
    fn metadata(&self) -> ProviderMetadata;
}

/// Universal capability that any ecosystem component can advertise
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct Capability {
    /// Capability identifier (e.g., "storage.provision", "compute.execute", "security.encrypt")
    pub id: String,
    /// Human-readable name
    pub name: String,
    /// Capability description
    pub description: String,
    /// Capability category (compute, storage, security, ai, communication, etc.)
    pub category: CapabilityCategory,
    /// Capability attributes and parameters
    pub attributes: HashMap<String, CapabilityAttribute>,
    /// Quality of service metrics
    pub qos: QualityOfService,
    /// Resource requirements
    pub resource_requirements: ResourceRequirements,
}

/// Universal capability categories
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub enum CapabilityCategory {
    /// Compute capabilities (execution, processing, orchestration)
    Compute,
    /// Storage capabilities (persist, cache, backup)
    Storage,
    /// Security capabilities (encrypt, authorize, audit)
    Security,
    /// AI capabilities (inference, training, analysis)
    AI,
    /// Communication capabilities (messaging, discovery, routing)
    Communication,
    /// Monitoring capabilities (metrics, logging, alerting)
    Monitoring,
    /// Integration capabilities (adaptation, transformation)
    Integration,
    /// Custom domain-specific capability
    Custom(String),
}

/// Capability attribute with type information
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct CapabilityAttribute {
    /// Attribute value
    pub value: String,
    /// Attribute data type
    pub data_type: AttributeDataType,
    /// Whether this attribute is required
    pub required: bool,
    /// Human-readable description
    pub description: Option<String>,
}

/// Data types for capability attributes
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub enum AttributeDataType {
    /// String data type
    String,
    /// Integer data type
    Integer,
    /// Float data type
    Float,
    /// Boolean data type
    Boolean,
    /// Array data type
    Array,
    /// Object data type
    Object,
    /// Duration data type
    Duration,
    /// Bytes data type
    Bytes,
}

/// Quality of service metrics
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct QualityOfService {
    /// Average response time in milliseconds
    pub avg_response_time_ms: u64,
    /// Availability percentage (0.0-100.0)
    pub availability_percent: f64,
    /// Throughput capacity
    pub throughput: Option<ThroughputMetric>,
    /// Scalability information
    pub scalability: ScalabilityInfo,
}

/// Throughput measurement
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct ThroughputMetric {
    /// Throughput value
    pub value: u64,
    /// Throughput unit (requests/sec, MB/sec, etc.)
    pub unit: String,
}

/// Scalability information
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct ScalabilityInfo {
    /// Minimum instances
    pub min_instances: u32,
    /// Maximum instances
    pub max_instances: u32,
    /// Auto-scaling supported
    pub auto_scaling: bool,
}

/// Resource requirements for a capability
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
#[derive(Default)]
pub struct ResourceRequirements {
    /// CPU requirements
    pub cpu: Option<ResourceRequirement>,
    /// Memory requirements
    pub memory: Option<ResourceRequirement>,
    /// Storage requirements
    pub storage: Option<ResourceRequirement>,
    /// Network requirements
    pub network: Option<ResourceRequirement>,
    /// Custom resource requirements
    pub custom: HashMap<String, ResourceRequirement>,
}

/// Individual resource requirement
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct ResourceRequirement {
    /// Minimum required amount
    pub min: u64,
    /// Maximum required amount
    pub max: Option<u64>,
    /// Unit of measurement (cores, MB, GB/sec, etc.)
    pub unit: String,
}

/// Universal dependency that any ecosystem component can require
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct Dependency {
    /// Dependency identifier
    pub id: String,
    /// Required capability
    pub required_capability: String,
    /// Minimum version requirement
    pub min_version: Option<String>,
    /// Whether this dependency is optional
    pub optional: bool,
    /// Dependency attributes
    pub attributes: HashMap<String, String>,
}

/// Service endpoints for communication
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct ServiceEndpoints {
    /// Primary service endpoint
    pub primary: String,
    /// Health check endpoint
    pub health: String,
    /// Metrics endpoint
    pub metrics: Option<String>,
    /// Admin/management endpoint
    pub admin: Option<String>,
    /// Event streaming endpoint
    pub events: Option<String>,
    /// Custom endpoints
    pub custom: HashMap<String, String>,
}

/// Health status of a provider
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub enum HealthStatus {
    /// Provider is healthy and operational
    Healthy,
    /// Provider is degraded but functional
    Degraded {
        /// List of issues causing degradation
        issues: Vec<String>,
        /// Impact level of the degradation
        impact: HealthImpact,
    },
    /// Provider is unhealthy and requires attention
    Unhealthy {
        /// Reason for unhealthy status
        reason: String,
        /// Estimated recovery time
        recovery_time: Option<DateTime<Utc>>,
    },
    /// Provider is starting up
    Starting,
    /// Provider is shutting down
    Stopping,
    /// Provider is unknown state
    Unknown,
}

/// Health impact levels
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub enum HealthImpact {
    /// No impact on service
    None,
    /// Low impact on service
    Low,
    /// Medium impact on service
    Medium,
    /// High impact on service
    High,
    /// Critical impact on service
    Critical,
}

/// Generic service request
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ServiceRequest {
    /// Unique request identifier
    pub request_id: Uuid,
    /// Request type/action
    pub request_type: String,
    /// Request payload
    pub payload: serde_json::Value,
    /// Request timestamp
    pub timestamp: DateTime<Utc>,
    /// Request priority
    pub priority: RequestPriority,
    /// Request metadata
    pub metadata: HashMap<String, String>,
    /// Request context
    pub context: RequestContext,
}

/// Request priority levels
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub enum RequestPriority {
    /// Low priority request
    Low,
    /// Normal priority request
    Normal,
    /// High priority request
    High,
    /// Critical priority request
    Critical,
    /// Emergency priority request
    Emergency,
}

/// Request context for routing and processing
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct RequestContext {
    /// User/caller identifier
    pub user_id: Option<String>,
    /// Session identifier
    pub session_id: Option<String>,
    /// Transaction identifier
    pub transaction_id: Option<String>,
    /// Source ecosystem component
    pub source_ecosystem: String,
    /// Target ecosystem component (if specific)
    pub target_ecosystem: Option<String>,
    /// Context metadata
    pub metadata: HashMap<String, String>,
}

/// Generic service response
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ServiceResponse {
    /// Request identifier this response is for
    pub request_id: Uuid,
    /// Response success status
    pub success: bool,
    /// Response payload
    pub payload: serde_json::Value,
    /// Response timestamp
    pub timestamp: DateTime<Utc>,
    /// Response metadata
    pub metadata: HashMap<String, String>,
    /// Error information if unsuccessful
    pub error: Option<ServiceError>,
}

/// Service error information
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ServiceError {
    /// Error code
    pub code: String,
    /// Error message
    pub message: String,
    /// Error details
    pub details: Option<serde_json::Value>,
    /// Whether error is retryable
    pub retryable: bool,
}

/// Ecosystem registration information
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct EcosystemRegistration {
    /// Registration identifier
    pub registration_id: Uuid,
    /// Ecosystem component identifier
    pub ecosystem_id: String,
    /// Instance identifier
    pub instance_id: String,
    /// Service endpoints
    pub endpoints: ServiceEndpoints,
    /// Provider capabilities
    pub capabilities: Vec<Capability>,
    /// Registration timestamp
    pub registration_time: DateTime<Utc>,
    /// Registration status
    pub status: RegistrationStatus,
}

/// Registration status
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub enum RegistrationStatus {
    /// Registration is pending approval
    Pending,
    /// Registration is active
    Active,
    /// Registration is inactive
    Inactive,
    /// Registration is temporarily suspended
    Suspended,
    /// Registration has been revoked
    Revoked,
}

/// Provider configuration
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ProviderConfig {
    /// Provider-specific configuration
    pub provider_config: HashMap<String, serde_json::Value>,
    /// Ecosystem-wide configuration
    pub ecosystem_config: HashMap<String, serde_json::Value>,
    /// Network configuration
    pub network_config: NetworkConfig,
    /// Monitoring configuration
    pub monitoring_config: MonitoringConfig,
}

/// Network configuration
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct NetworkConfig {
    /// Listen address
    pub listen_address: String,
    /// Port number
    pub port: u16,
    /// TLS enabled
    pub tls_enabled: bool,
    /// Timeout settings
    pub timeout_seconds: u64,
    /// Connection pool settings
    pub connection_pool: ConnectionPoolConfig,
}

/// Connection pool configuration
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ConnectionPoolConfig {
    /// Maximum connections
    pub max_connections: u32,
    /// Connection timeout
    pub connection_timeout_seconds: u64,
    /// Idle timeout
    pub idle_timeout_seconds: u64,
}

/// Monitoring configuration
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct MonitoringConfig {
    /// Metrics collection enabled
    pub metrics_enabled: bool,
    /// Logging level
    pub log_level: String,
    /// Health check interval
    pub health_check_interval_seconds: u64,
}

/// Provider metadata
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ProviderMetadata {
    /// Provider name
    pub name: String,
    /// Provider version
    pub version: String,
    /// Provider description
    pub description: String,
    /// Provider author/maintainer
    pub author: String,
    /// Provider website
    pub website: Option<String>,
    /// Provider license
    pub license: String,
    /// Provider tags
    pub tags: Vec<String>,
    /// Custom metadata
    pub custom: HashMap<String, String>,
}

/// Default implementations for common patterns
impl Default for ServiceEndpoints {
    fn default() -> Self {
        Self {
            primary: "http://localhost:8080".to_string(),
            health: "http://localhost:8080/health".to_string(),
            metrics: Some("http://localhost:8080/metrics".to_string()),
            admin: None,
            events: None,
            custom: HashMap::new(),
        }
    }
}

impl Default for QualityOfService {
    fn default() -> Self {
        Self {
            avg_response_time_ms: 100,
            availability_percent: 99.9,
            throughput: None,
            scalability: ScalabilityInfo {
                min_instances: 1,
                max_instances: 10,
                auto_scaling: false,
            },
        }
    }
}


impl Default for NetworkConfig {
    fn default() -> Self {
        Self {
            listen_address: "0.0.0.0".to_string(),
            port: 8080,
            tls_enabled: false,
            timeout_seconds: 30,
            connection_pool: ConnectionPoolConfig {
                max_connections: 100,
                connection_timeout_seconds: 10,
                idle_timeout_seconds: 60,
            },
        }
    }
}

impl Default for MonitoringConfig {
    fn default() -> Self {
        Self {
            metrics_enabled: true,
            log_level: "info".to_string(),
            health_check_interval_seconds: 30,
        }
    }
}
