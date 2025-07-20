//! Universal Primal Provider Implementation for BearDog
//!
//! Implements the ecosystem-standard UniversalPrimalProvider trait for BearDog,
//! enabling seamless integration with the ecoPrimals ecosystem through
//! standardized interfaces and communication protocols.

use async_trait::async_trait;
use beardog_errors::BearDogResult;
use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use uuid::Uuid;

/// Primal metadata for ecosystem integration
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PrimalMetadata {
    /// Primal name identifier
    pub name: String,
    /// Semantic version
    pub version: String,
    /// Type of primal in the ecosystem
    pub primal_type: PrimalType,
    /// Human-readable description
    pub description: String,
    /// Maintainer information
    pub maintainer: String,
    /// Role in ecosystem architecture
    pub ecosystem_role: EcosystemRole,
    /// AI-first design compliance score (0.0-1.0)
    pub ai_first_score: f64,
}

/// Types of primals in the ecosystem
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub enum PrimalType {
    /// Security and compliance management
    BearDog,
    /// Network orchestration and service mesh
    Songbird,
    /// Distributed storage and data management
    NestGate,
    /// Compute orchestration and resource management
    UniversalCompute,
    /// AI and machine learning capabilities
    AIFirstCitizen,
    /// Custom or third-party primal
    Custom(String),
}

/// Role in the ecosystem architecture
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub enum EcosystemRole {
    /// Provides security services to other primals
    SecurityProvider,
    /// Orchestrates network communication
    NetworkOrchestrator,
    /// Manages distributed data storage
    StorageProvider,
    /// Coordinates compute resources
    ComputeOrchestrator,
    /// Provides AI/ML capabilities
    AIProvider,
    /// Consumer of ecosystem services
    ServiceConsumer,
    /// Bridge to external systems
    ExternalBridge,
}

/// Capabilities that a primal can provide
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub enum PrimalCapability {
    /// Security and encryption services
    Security,
    /// AI and machine learning
    AI,
    /// Monitoring and observability
    Monitoring,
    /// Data storage and retrieval
    Storage,
    /// Network communication
    Networking,
    /// Compliance and audit
    Compliance,
    /// Workflow orchestration
    Workflow,
    /// Threat detection
    ThreatDetection,
    /// Key management
    KeyManagement,
    /// Custom capability
    Custom(String),
}

/// Service definition for ecosystem integration
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PrimalService {
    /// Service identifier
    pub id: String,
    /// Service name
    pub name: String,
    /// Service description
    pub description: String,
    /// Service endpoint configuration
    pub endpoint: ServiceEndpoint,
    /// Required capabilities for this service
    pub capabilities: Vec<PrimalCapability>,
    /// Health status
    pub health: ServiceHealth,
}

/// Service endpoint configuration
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ServiceEndpoint {
    /// Protocol (http, grpc, custom)
    pub protocol: String,
    /// Host address
    pub host: String,
    /// Port number
    pub port: u16,
    /// Path or route
    pub path: String,
    /// Security requirements
    pub security: EndpointSecurity,
}

/// Service health status
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub enum ServiceHealth {
    /// Service is healthy and operational
    Healthy,
    /// Service is degraded but functional
    Degraded,
    /// Service is unhealthy
    Unhealthy,
    /// Service status unknown
    Unknown,
}

/// Endpoint security configuration
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct EndpointSecurity {
    /// Require TLS encryption
    pub require_tls: bool,
    /// Client certificate required
    pub require_client_cert: bool,
    /// API key authentication
    pub require_api_key: bool,
    /// Custom authentication schemes
    pub custom_auth: Vec<String>,
}

/// Universal Primal Provider trait for ecosystem integration
#[async_trait]
pub trait UniversalPrimalProvider: Send + Sync {
    /// Get primal metadata for ecosystem registration
    fn metadata(&self) -> &PrimalMetadata;

    /// Get list of capabilities this primal provides
    fn capabilities(&self) -> &[PrimalCapability];

    /// Get list of services this primal exposes
    async fn services(&self) -> BearDogResult<Vec<PrimalService>>;

    /// Register with the ecosystem through Songbird
    async fn register_with_ecosystem(&self, songbird_endpoint: &str) -> BearDogResult<()>;

    /// Handle incoming service request from ecosystem
    async fn handle_service_request(
        &self,
        service_id: &str,
        request_data: Vec<u8>,
        context: ServiceContext,
    ) -> BearDogResult<Vec<u8>>;

    /// Health check for ecosystem monitoring
    async fn health_check(&self) -> BearDogResult<ServiceHealth>;

    /// Shutdown notification from ecosystem
    async fn shutdown(&self) -> BearDogResult<()>;
}

/// Service request context
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ServiceContext {
    /// Request ID for tracing
    pub request_id: Uuid,
    /// Source primal information
    pub source: PrimalIdentity,
    /// Security context
    pub security: SecurityContext,
    /// Additional metadata
    pub metadata: HashMap<String, String>,
}

/// Primal identity information
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PrimalIdentity {
    /// Primal name
    pub name: String,
    /// Primal type
    pub primal_type: PrimalType,
    /// Node ID in the ecosystem
    pub node_id: String,
}

/// Security context for service requests
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SecurityContext {
    /// Authentication token
    pub auth_token: Option<String>,
    /// Client certificate information
    pub client_cert: Option<String>,
    /// Security clearance level
    pub clearance_level: u8,
    /// Encryption requirements
    pub encryption_required: bool,
}

impl Default for PrimalMetadata {
    fn default() -> Self {
        Self {
            name: "BearDog".to_string(),
            version: env!("CARGO_PKG_VERSION").to_string(),
            primal_type: PrimalType::BearDog,
            description: "AI-First Security and Compliance Management".to_string(),
            maintainer: "BearDog Security Team".to_string(),
            ecosystem_role: EcosystemRole::SecurityProvider,
            ai_first_score: 0.95, // Gold standard compliance
        }
    }
}

impl Default for ServiceEndpoint {
    fn default() -> Self {
        Self {
            protocol: "https".to_string(),
            host: beardog_config::constants::network::DEFAULT_HOST.to_string(),
            port: 8443,
            path: "/api/v1".to_string(),
            security: EndpointSecurity {
                require_tls: true,
                require_client_cert: false,
                require_api_key: true,
                custom_auth: vec!["beardog-auth".to_string()],
            },
        }
    }
}
