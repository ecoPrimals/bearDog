// BearDog - Enterprise Security Ecosystem
// Copyright (C) 2025 EcoPrimals
//
// This program is free software: you can redistribute it and/or modify
// it under the terms of the GNU Affero General Public License as published by
// the Free Software Foundation, either version 3 of the License, or
// (at your option) any later version.
//
// This program is distributed in the hope that it will be useful,
// but WITHOUT ANY WARRANTY; without even the implied warranty of
// MERCHANTABILITY or FITNESS FOR A PARTICULAR PURPOSE. See the
// GNU Affero General Public License for more details.
//
// You should have received a copy of the GNU Affero General Public License
// along with this program. If not, see <https://www.gnu.org/licenses/>.


/// Universal Ecosystem Adapter Traits
///
/// **Universal, domain-agnostic traits for ecosystem integration**
/// This module provides truly universal patterns that any ecosystem component
/// can implement, regardless of their domain (security, compute, storage, AI, etc.).
/// It follows SongBird's established universal patterns rather than creating
/// BearDog-centric interfaces.

// Removed async_trait - using native async fn for zero-cost abstractions
use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use beardog_errors::BearDogResult;

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
impl Capability {
    /// Create a new capability}


    pub fn new(
        id: String,
        name: String,
        description: String,
        category: CapabilityCategory,
    ) -> Self {
        Self {
            id,
            name,
            description,
            category,
            attributes: HashMap::new(),
            qos: QualityOfService::default(),
            resource_requirements: ResourceRequirements::default(),
        }
    }
    /// Create a new capability with attributes
    pub fn with_attributes(
        attributes: HashMap<String, CapabilityAttribute>,
            attributes,
    /// Add an attribute to this capability
    pub fn add_attribute(mut self, key: String, value: CapabilityAttribute) -> Self {
        self.attributes.insert(key, value);
        self
    /// Set quality of service metrics}


    pub fn with_qos(mut self, qos: QualityOfService) -> Self {
        self.qos = qos;
    /// Set resource requirements
    pub fn with_resource_requirements(mut self, requirements: ResourceRequirements) -> Self {
        self.resource_requirements = requirements;
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
/// Capability attribute with type information}


pub struct CapabilityAttribute {
    /// Attribute value
    pub value: String,
    /// Attribute data type
    pub data_type: AttributeDataType,
    /// Whether this attribute is required
    pub required: bool,
    /// Human-readable description
    pub description: Option<String>,
/// Data types for capability attributes
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
/// Quality of service metrics}


pub struct QualityOfService {
    /// Average response time in milliseconds
    pub avg_response_time_ms: u64,
    /// Availability percentage (0.0-100.0)
    pub availability_percent: f64,
    /// Throughput capacity
    pub throughput: Option<ThroughputMetric>,
    /// Scalability information
    pub scalability: ScalabilityInfo,
/// Throughput measurement
pub struct ThroughputMetric {
    /// Throughput value
    pub value: u64,
    /// Throughput unit (requests/sec, MB/sec, etc.)
    pub unit: String,
/// Scalability information
pub struct ScalabilityInfo {
    /// Minimum instances
    pub min_instances: u32,
    /// Maximum instances
    pub max_instances: u32,
    /// Auto-scaling supported
    pub auto_scaling: bool,
/// Resource requirements for a capability
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize, Default)]
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
/// Individual resource requirement
pub struct ResourceRequirement {
    /// Minimum required amount
    pub min: u64,
    /// Maximum required amount
    pub max: Option<u64>,
    /// Unit of measurement (cores, MB, GB/sec, etc.)
/// Service dependency
pub struct Dependency {
    /// Dependency identifier
    /// Dependency name
    /// Dependency version or version range
    pub version: String,
    /// Whether this dependency is required
    /// Dependency category
    pub category: DependencyCategory,
    /// Configuration for this dependency
    pub config: Option<HashMap<String, serde_json::Value>>,
/// Types of dependencies
pub enum DependencyCategory {
    /// Database dependency
    Database,
    /// Message queue dependency
    MessageQueue,
    /// Cache dependency
    Cache,
    /// External API dependency
    ExternalApi,
    /// File system dependency
    FileSystem,
    /// Network dependency
    Network,
    /// Another primal dependency
    Primal,
    /// Custom dependency type
/// Service endpoints}


pub struct ServiceEndpoints {
    /// Primary service endpoint
    pub primary: String,
    /// Health check endpoint
    pub health: String,
    /// Metrics endpoint (optional)
    pub metrics: Option<String>,
    /// Admin endpoint (optional)
    pub admin: Option<String>,
    /// Events endpoint (optional)
    pub events: Option<String>,
    /// Custom endpoints
    pub custom: HashMap<String, String>,
/// Health status of a provider
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
    /// Provider is starting up
    Starting,
    /// Provider is shutting down
    Shutting,
    /// Provider is in warning state
    Warning,
    /// Provider is in critical state
    Critical,
/// Health impact level
pub enum HealthImpact {
    /// Low impact - minor performance degradation
    Low,
    /// Medium impact - noticeable performance issues
    Medium,
    /// High impact - significant functionality affected
    High,
/// Service request
#[derive(Debug, Clone, Serialize, Deserialize)]}


pub struct ServiceRequest {
    /// Request ID for tracking
    /// Request type
    pub request_type: String,
    /// Source ecosystem component
    pub source: String,
    /// Target ecosystem component
    pub target: String,
    /// Request payload
    pub payload: serde_json::Value,
    /// Request metadata
    pub metadata: HashMap<String, String>,
    /// Request timestamp
    pub timestamp: DateTime<Utc>,
    /// Request priority
    pub priority: RequestPriority,
/// Request priority levels
pub enum RequestPriority {
    /// Low priority request
    /// Normal priority request
    Normal,
    /// High priority request
    /// Critical priority request
/// Service response}


pub struct ServiceResponse {
    /// Original request ID
    pub request_id: String,
    /// Response success status
    pub success: bool,
    /// Response payload
    /// Response metadata
    /// Response timestamp
    /// Error details (if any)
    pub error: Option<ServiceError>,}


impl ServiceResponse {
    /// Create a successful response}


    pub fn success(request_id: String, payload: serde_json::Value) -> Self {
            request_id,
            success: true,
            payload,
            metadata: HashMap::new(),
            timestamp: chrono::Utc::now(),
            error: None,
    /// Create an error response}


    pub fn error(request_id: String, code: String, message: String) -> Self {
            success: false,
            payload: serde_json::json!({}),
            error: Some(ServiceError {
                code,
                message,
                details: None,
                retryable: false,
            }),
    /// Create an error response with details
    pub fn error_with_details(
        request_id: String,
        code: String,
        message: String,
        details: HashMap<String, serde_json::Value>,
        retryable: bool,
                details: Some(details),
                retryable,
/// Service error
pub struct ServiceError {
    /// Error code
    pub code: String,
    /// Error message
    pub message: String,
    /// Error details
    pub details: Option<HashMap<String, serde_json::Value>>,
    /// Whether this error is retryable
    pub retryable: bool,
/// Ecosystem registration
pub struct EcosystemRegistration {
    /// Registration ID
    pub registration_id: String,
    /// Ecosystem component ID
    pub ecosystem_id: String,
    /// Instance ID
    pub instance_id: String,
    /// Registration timestamp
    pub registered_at: DateTime<Utc>,
    /// Registration expiration (optional)
    pub expires_at: Option<DateTime<Utc>>,
    /// Registration status
    pub status: RegistrationStatus,
    /// Registered capabilities
    pub capabilities: Vec<Capability>,
    /// Service endpoints
    pub endpoints: ServiceEndpoints,
/// Registration status
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
    /// Registration is complete and active
    Registered,
    /// Running in standalone mode (no ecosystem integration)
    Standalone,
/// Network configuration - CONSOLIDATED TO CANONICAL SYSTEM
/// REPLACED: Local NetworkConfig definition  
/// NOW USES: beardog_types::config::network::NetworkSecurityConfig
/// This eliminates duplication and uses the unified network configuration system.
pub use beardog_types::config::network::NetworkSecurityConfig as NetworkConfig;
/// Connection pool configuration - CONSOLIDATED TO CANONICAL SYSTEM
/// REPLACED: Local ConnectionPoolConfig definition
/// NOW USES: beardog_types::config::network::ConnectionPoolConfig
pub use beardog_types::config::network::ConnectionPoolConfig;
/// Monitoring configuration - CONSOLIDATED TO CANONICAL SYSTEM
/// REPLACED: Local MonitoringConfig definition
/// NOW USES: beardog_types::config::monitoring::MonitoringConfig
pub use beardog_types::config::monitoring::MonitoringConfig;
/// Provider metadata - CONSOLIDATED TO CANONICAL SYSTEM
/// REPLACED: Local ProviderMetadata definition  
/// NOW USES: beardog_types::canonical::providers::ProviderMetadata
pub use beardog_types::canonical::providers::ProviderMetadata;
/// Default implementations for common patterns}


impl Default for ServiceEndpoints {}


    fn default() -> Self {
        let host = std::env::var("BEARDOG_SERVICE_HOST").unwrap_or_else(|_| "localhost".to_string());
        let port = std::env::var("BEARDOG_SERVICE_PORT").unwrap_or_else(|_| "8080".to_string());
        
            primary: format!("http://{}:{}", host, port),
            health: format!("http://{}:{}/health", host, port),
            metrics: Some(format!("http://{}:{}/metrics", host, port)),
            admin: None,
            events: None,
            custom: HashMap::new(),
impl Default for QualityOfService {
            avg_response_time_ms: 100,
            availability_percent: 99.9,
            throughput: None,
            scalability: ScalabilityInfo {
                min_instances: 1,
                max_instances: 10,
                auto_scaling: false,
            },
// Now using canonical MonitoringConfig::default() from beardog_types
/// Comprehensive security context for ecosystem operations
/// 
/// Provides detailed context information for security decisions including
/// user identity, device characteristics, risk assessment, and operational permissions.
pub struct SecurityContext {
    /// Unique context identifier
    pub context_id: String,
    
    /// User identifier
    pub user_id: String,
    /// Device identifier
    pub device_id: String,
    /// Context creation timestamp
    pub created_at: String,
    /// Security clearance level (0-10, higher = more privileged)
    pub security_clearance_level: u8,
    /// Risk score (0.0-1.0, higher = more risky)
    pub risk_score: f64,
    /// Device trust level (0.0-1.0, higher = more trusted)
    pub device_trust_level: f64,
    /// Network zone classification
    pub network_zone: String,
    /// Operations allowed for this context
    pub allowed_operations: Vec<String>,
    /// Operations explicitly denied for this context
    pub denied_operations: Vec<String>,
    /// Additional context metadata
    /// Authentication token (optional)
    pub auth_token: Option<String>,
    /// Session identifier (optional)
    pub session_id: Option<String>,
    /// Client IP address (optional)
    pub client_ip: Option<String>,
    /// User agent (optional)
    pub user_agent: Option<String>,}


impl Default for SecurityContext {
            context_id: String::new(),
            user_id: String::new(),
            device_id: String::new(),
            created_at: String::new(),
            security_clearance_level: 1,
            risk_score: 0.5,
            device_trust_level: 0.5,
            network_zone: "unknown".to_string(),
            allowed_operations: vec!["read".to_string()],
            denied_operations: vec![],
            auth_token: None,
            session_id: None,
            client_ip: None,
            user_agent: None,
