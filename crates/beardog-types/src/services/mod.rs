// Service definitions and management types
//
// This module provides comprehensive service management types including service discovery,
// health monitoring, capability tracking, and endpoint management. All service operations
// maintain sovereignty compliance and zero hardcoded assumptions.

use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use uuid::Uuid;

/// Service definition with comprehensive metadata
///
/// Represents a complete service definition including capabilities, endpoints,
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ServiceDefinition {
    /// Unique service identifier
    pub id: Uuid,
    /// Human-readable service name
    /// Name of the item
    pub name: String,
    /// Detailed service description
    /// The description value
    pub description: String,
    /// Service version identifier
    /// The version value
    pub version: String,
    /// List of capabilities this service provides
    /// Collection of capabilities
    pub capabilities: Vec<ServiceCapability>,
    /// Available service endpoints
    /// Collection of endpoints
    pub endpoints: Vec<ServiceEndpoint>,
    /// Current health status of the service
    /// Current status of the health
    pub health_status: ServiceHealth,
    /// The category value
    pub category: ServiceCategory,
    /// Security domain classification
    /// The security domain value
    pub security_domain: SecurityDomain,
    /// Timestamp when service was registered
    /// The created at value
    pub created_at: DateTime<Utc>,
    /// Timestamp when service was last updated
    /// The updated at value
    pub updated_at: DateTime<Utc>,
    /// Additional metadata key-value pairs
    /// Mapping of metadata
    pub metadata: HashMap<String, String>,
}

/// Service capability definition
///
/// Defines a specific capability that a service provides, including
/// permissions required and operational details.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ServiceCapability {
    /// Capability name identifier
    /// Name of the item
    pub name: String,
    /// Detailed capability description
    /// The description value
    pub description: String,
    /// Required permissions to use this capability
    /// Collection of required permissions
    pub required_permissions: Vec<String>,
}

/// Service endpoint definition
///
/// Defines how to connect to and interact with a service endpoint
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ServiceEndpoint {
    /// Endpoint name identifier
    /// Name of the item
    pub name: String,
    /// The url value
    pub url: String,
    /// Communication protocol used
    /// The protocol value
    pub protocol: EndpointProtocol,
    /// Supported HTTP methods or operations
    /// Collection of methods
    pub methods: Vec<String>,
    /// Whether authentication is required
    /// Whether `authentication_required` is enabled
    pub authentication_required: bool,
}

/// Supported endpoint protocols
///
/// Defines the communication protocols that service endpoints can use
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
pub enum EndpointProtocol {
    /// Standard HTTP protocol
    Http,
    /// Secure HTTPS protocol
    Https,
    Grpc,
    WebSocket,
    /// Custom protocol with string identifier
    Custom(String),
}

/// Service dependency definition
///
/// Defines a dependency relationship between services including
/// version requirements and health check preferences.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ServiceDependency {
    /// Name of the required service
    /// Name of the service
    pub service_name: String,
    /// Version requirement specification
    /// The version requirement value
    pub version_requirement: String,
    /// Whether this dependency is optional
    /// Whether optional is enabled
    pub optional: bool,
    /// Whether `health_check_required` is enabled
    pub health_check_required: bool,
}

/// Service health status enumeration
///
/// Represents the current operational status of a service
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
pub enum ServiceHealth {
    /// Service is fully operational
    Healthy,
    Degraded,
    /// Service is not operational
    Unhealthy,
    /// Service health status cannot be determined
    Unknown,
}

/// Service category classification
///
/// organization and discovery purposes.
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
pub enum ServiceCategory {
    /// Core system services
    Core,
    /// Security and authentication services
    Security,
    /// Monitoring and observability services
    Monitoring,
    /// Workflow and orchestration services
    Workflow,
    /// Integration and adapter services
    Integration,
    /// Custom service category with string identifier
    Custom(String),
}

/// Security domain classification
///
/// Classifies services by their security clearance level
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
pub enum SecurityDomain {
    /// Publicly accessible services
    Public,
    /// Internal organization services
    Internal,
    /// Restricted access services
    Restricted,
    /// Classified or highly sensitive services
    Classified,
}

impl Default for ServiceDefinition {
    fn default() -> Self {
        Self {
            id: Uuid::new_v4(),
            name: "unknown".to_string(),
            description: "Unknown service".to_string(),
            version: "0.1.0".to_string(),
            capabilities: vec![],
            endpoints: vec![],
            health_status: ServiceHealth::Unknown,
            category: ServiceCategory::Custom("unknown".to_string()),
            security_domain: SecurityDomain::Internal,
            created_at: Utc::now(),
            updated_at: Utc::now(),
            metadata: HashMap::new(),
        }
    }
}

impl Default for ServiceCapability {
    fn default() -> Self {
        Self {
            name: "unknown".to_string(),
            description: "Unknown capability".to_string(),
            required_permissions: vec![],
        }
    }
}

impl Default for ServiceEndpoint {
    fn default() -> Self {
        Self {
            name: "unknown".to_string(),
            url: std::env::var("BEARDOG_SERVICE_URL")
                .unwrap_or_else(|_| "http://service.discovery.local:8080".to_string()),
            protocol: EndpointProtocol::Http,
            methods: vec!["GET".to_string()],
            authentication_required: false,
        }
    }
}

impl Default for EndpointProtocol {
    fn default() -> Self {
        Self::Http
    }
}

impl Default for ServiceCategory {
    fn default() -> Self {
        Self::Custom("unknown".to_string())
    }
}

impl Default for SecurityDomain {
    fn default() -> Self {
        Self::Internal
    }
}
