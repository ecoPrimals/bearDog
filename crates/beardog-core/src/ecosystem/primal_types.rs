// Ecosystem types for capability-based service discovery
//
// This module provides capability-based types that replace hardcoded primal references
// to maintain sovereignty compliance where primals only know themselves.

#![allow(deprecated)]

// MODERNIZATION NOTE: This file contains primal-specific references that should be migrated
// to universal adapter patterns. See migration guide: docs/guides/UNIVERSAL_ADAPTER_USAGE_GUIDE.md
// Target: Replace with capability-based discovery for vendor/primal agnosticism
// Removed unused compute types - these are handled by universal adapters
// Removed unused CapabilityType - using ServiceCapabilityType instead
pub use beardog_types::canonical::capabilities::ServiceCapabilityType;
use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use uuid::Uuid;

// ServiceCapabilityType methods are now provided by CapabilityType
// See beardog_types::canonical::capabilities::CapabilityType for implementation

/// `ServiceDependency` represents capability-based dependencies
/// instead of hardcoded primal dependencies
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub enum ServiceDependency {
    /// State indicating required
    /// Required service capability dependency
    Required {
        /// The capability type required
        capability: ServiceCapabilityType,
        /// Minimum version requirement
        min_version: String,
        /// Reason for the requirement
        reason: String,
    },
    /// Optional service capability dependency
    Optional {
        /// The capability type
        capability: ServiceCapabilityType,
        /// Minimum version if present
        min_version: String,
        /// Reason for the optional dependency
        reason: String,
    },
}

/// Service metadata describing capabilities without hardcoded references
///
/// Contains all information about a service's capabilities, dependencies,
/// and endpoints for dynamic discovery and integration.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ServiceMetadata {
    /// Unique service identifier
    pub service_id: Uuid,
    /// Collection of capabilities this service provides
    pub capabilities: Vec<ServiceCapabilityType>,
    /// Collection of dependencies this service requires
    pub dependencies: Vec<ServiceDependency>,
    /// Service version string
    pub version: String,
    /// Service endpoint configuration
    pub endpoints: ServiceEndpoints,
}

/// Service endpoint configuration
///
/// Defines all endpoints exposed by a service for health checks,
/// metrics, primary operations, and optional admin/websocket interfaces.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ServiceEndpoints {
    /// Health check endpoint URL
    pub health: String,
    /// Metrics/telemetry endpoint URL
    pub metrics: String,
    /// Primary service endpoint URL
    pub primary: String,
    /// Optional admin interface endpoint
    pub admin: Option<String>,
    /// Optional websocket endpoint for real-time communication
    pub websocket: Option<String>,
}

/// Capability integration configuration
///
/// Standard capability types for ecosystem integration
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub enum CapabilityType {
    /// Security capability integration
    Security,
    /// Storage capability integration
    Storage,
    /// Compute capability integration
    Compute,
    /// Networking capability integration
    Networking,
    /// AI capability integration
    AI,
}

/// Capability Integration Configuration
///
/// Uses capability-based configuration instead of hardcoded primal flags,
/// enabling dynamic service integration based on discovered capabilities.
///
/// Evolved from multiple boolean fields to a set-based approach for better
/// idiomatic Rust and easier runtime capability management.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CapabilityIntegrationConfig {
    /// Enabled standard capability types
    #[serde(default = "default_enabled_capabilities")]
    pub enabled_capabilities: std::collections::HashSet<CapabilityType>,
    /// Custom capability configurations mapping capability names to their config values
    pub custom_capabilities: HashMap<String, serde_json::Value>,
}

fn default_enabled_capabilities() -> std::collections::HashSet<CapabilityType> {
    [
        CapabilityType::Security,
        CapabilityType::Storage,
        CapabilityType::Networking,
    ]
    .into_iter()
    .collect()
}

impl Default for CapabilityIntegrationConfig {
    fn default() -> Self {
        Self {
            enabled_capabilities: default_enabled_capabilities(),
            custom_capabilities: HashMap::new(),
        }
    }
}

impl CapabilityIntegrationConfig {
    /// Check if security capability is enabled
    pub fn has_security_capability(&self) -> bool {
        self.enabled_capabilities
            .contains(&CapabilityType::Security)
    }

    /// Check if storage capability is enabled
    pub fn has_storage_capability(&self) -> bool {
        self.enabled_capabilities.contains(&CapabilityType::Storage)
    }

    /// Check if compute capability is enabled
    pub fn has_compute_capability(&self) -> bool {
        self.enabled_capabilities.contains(&CapabilityType::Compute)
    }

    /// Check if networking capability is enabled
    pub fn has_networking_capability(&self) -> bool {
        self.enabled_capabilities
            .contains(&CapabilityType::Networking)
    }

    /// Check if AI capability is enabled
    pub fn has_ai_capability(&self) -> bool {
        self.enabled_capabilities.contains(&CapabilityType::AI)
    }
}

/// Attestation verification result
///
/// Contains the result of verifying a service's attestation, including
/// whether verification succeeded, confidence level, and method used.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct AttestationVerificationResult {
    /// Whether the attestation was successfully verified
    pub verified: bool,
    /// Confidence level in the verification (0.0 to 1.0)
    pub confidence: f64,
    /// Method used for verification
    pub method: String,
    /// Timestamp when verification was performed
    pub timestamp: DateTime<Utc>,
}

/// Attestation verification chain
///
/// Represents a chain of attestations for verifying service authenticity
/// across multiple verification points with an overall trust level.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct AttestationVerificationChain {
    /// Whether the entire chain was successfully verified
    pub verified: bool,
    /// Chain of attestation identifiers forming the trust path
    pub chain: Vec<String>,
    /// Overall trust level for this verification chain
    pub trust_level: String,
}

/// Capability health status
///
/// Tracks health status for multiple capabilities, providing both
/// an overall status and individual status for each capability.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CapabilityHealthStatus {
    /// Overall health status across all capabilities
    pub overall_status: String,
    /// Health status for each individual capability
    pub individual_status: HashMap<String, String>,
}

pub use beardog_types::canonical::HealthStatus;

/// Authentication result
///
/// Contains the outcome of an authentication attempt, including
/// user identity, access token, expiration, and granted permissions.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct AuthenticationResult {
    /// Whether authentication succeeded
    pub success: bool,
    /// Authenticated user ID if successful
    pub user_id: Option<String>,
    /// Access token for authenticated sessions
    pub token: Option<String>,
    /// Token expiration timestamp
    pub expires_at: Option<DateTime<Utc>>,
    /// List of permissions granted to this user
    pub permissions: Vec<String>,
}

/// Primal health status
///
/// Comprehensive health information for a primal service, including
/// overall status, detailed health check results, and timing information.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PrimalHealth {
    /// Current health status
    pub status: HealthStatus,
    /// Timestamp of last health check
    pub last_check: DateTime<Utc>,
    /// Detailed health information for debugging
    pub details: HashMap<String, serde_json::Value>,
    /// Results of individual health checks
    pub checks: HashMap<String, bool>,
    /// Timestamp when this health report was generated
    pub timestamp: DateTime<Utc>,
}

impl Default for PrimalHealth {
    fn default() -> Self {
        Self {
            status: HealthStatus::Healthy,
            last_check: Utc::now(),
            details: HashMap::new(),
            checks: HashMap::new(),
            timestamp: Utc::now(),
        }
    }
}

/// Key operation status
///
/// Status information for cryptographic key operations, tracking
/// health, tested operations, and performance metrics.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct KeyOperationStatus {
    /// Whether key operations are healthy
    pub healthy: bool,
    /// List of operations that were tested
    pub operations_tested: Vec<String>,
    /// Response time metrics for operations
    pub response_times: serde_json::Value,
}

/// Endpoint health status
///
/// Health information for a specific service endpoint.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct EndpointHealth {
    /// Name of the endpoint
    pub name: String,
    /// Current health status of the endpoint
    pub status: String,
}

/// Response time metrics
///
/// Statistical metrics for measuring response time performance,
/// including average and percentile measurements.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize, Default)]
pub struct ResponseTimeMetrics {
    /// Average response time in milliseconds
    pub average: f64,
    /// 95th percentile response time
    pub p95: f64,
    /// 99th percentile response time
    pub p99: f64,
}

/// Note: `PrimalType` enum was removed as it violated primal sovereignty
///
/// Migration completed: Use capability-based discovery instead
/// ✅ SOVEREIGNTY COMPLIANCE: Use capability-based discovery instead of hardcoded primal names
/// - Replace `PrimalType::BiomeOS` with `ServiceCapabilityType::ContainerOrchestration` discovery
///
/// Each primal now only knows itself and discovers others through universal adapter.
/// NEW: Capability-based primal identification (replaces `PrimalType`)
///
/// This represents a discovered primal in the ecosystem without hardcoding
/// its name or making assumptions about its identity.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct DiscoveredPrimal {
    /// Unique identifier discovered during bootstrap
    pub primal_id: String,
    /// Self-reported capabilities (what this primal can do)
    /// Collection of capabilities
    pub capabilities: Vec<ServiceCapabilityType>,
    /// Communication endpoint
    /// The endpoint value
    pub endpoint: UniversalEndpoint,
    /// Self-reported metadata
    /// The metadata value
    pub metadata: PrimalMetadata,
    /// Discovery timestamp
    /// The discovered at value
    pub discovered_at: std::time::SystemTime,
    /// The metrics value
    pub metrics: PrimalMetrics,
}

/// Primal metadata discovered during ecosystem bootstrap
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct PrimalMetadata {
    /// Name of the display
    pub display_name: Option<String>,
    /// The version value
    pub version: String,
    /// Supported protocol versions
    /// Collection of protocol versions
    pub protocol_versions: Vec<String>,
    /// Security level and attestations
    /// Collection of security attestations
    pub security_attestations: Vec<SecurityAttestation>,
    /// Custom metadata fields
    /// Mapping of custom fields
    pub custom_fields: HashMap<String, String>,
    /// Capabilities provided by this primal (discovered dynamically)
    /// Collection of capabilities
    pub capabilities: Vec<ServiceCapabilityType>,
    /// Dependencies on other capabilities (not hardcoded primal names)
    /// Collection of dependencies
    pub dependencies: Vec<ServiceDependency>,
    /// Supported communication protocols
    /// Collection of supported protocols
    pub supported_protocols: Vec<String>,
    /// Health check endpoint path
    /// The health check endpoint value
    pub health_check_endpoint: String,
    /// Metrics endpoint path
    /// The metrics endpoint value
    pub metrics_endpoint: String,
}

/// Universal endpoint configuration for primal services
///
/// Defines a dynamically discovered service endpoint with protocol support,
/// authentication requirements, and security configuration.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct UniversalEndpoint {
    /// Base URL (discovered dynamically, not hardcoded)
    pub url: String,
    /// Supported communication protocols (HTTP, gRPC, WebSocket, etc.)
    pub protocols: Vec<String>,
    /// Authentication requirements for accessing this endpoint
    pub auth_requirements: AuthRequirements,
    /// TLS and security configuration for secure communication
    pub security_config: EndpointSecurityConfig,
}

/// Performance and health metrics for primal services
///
/// Tracks key operational metrics including response times, availability,
/// load/capacity, and error rates for monitoring and optimization.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct PrimalMetrics {
    /// Response time percentiles (p50, p95, p99)
    pub response_times: ResponseTimeMetrics,
    /// Service availability percentage (0.0-100.0)
    pub availability: f64,
    /// Current load and capacity utilization
    pub load_metrics: LoadMetrics,
    /// Error rates across different categories
    pub error_rates: ErrorRateMetrics,
}

/// Core capabilities that primals can provide in the ecosystem
///
/// Defines the functional categories of services that primals offer,
/// enabling capability-based service discovery and routing.
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq, Hash)]
pub enum PrimalCapability {
    /// Security services (encryption, authentication, HSM integration)
    Security,
    /// Storage services (distributed storage, caching, persistence)
    Storage,
    /// Compute services (processing, analysis, transformation)
    Compute,
    /// Networking services (routing, discovery, tunneling)
    Networking,
    /// AI/ML services (inference, training, optimization)
    AI,
    /// Represents custom variant
    Custom(String),
}

// PrimalDependency removed Nov 7, 2025 - use ServiceDependency instead
// Migration complete - all code now uses capability-based dependencies

// PrimalIntegrationConfig removed Nov 7, 2025 - use UniversalIntegrationConfig instead
// Migration complete - all code now uses capability-based discovery

///
/// Represents errors that occur during primal component operations,
/// including initialization failures, communication errors, and
/// with error codes, messages, and additional context details.
#[derive(Debug, Clone)]
pub struct PrimalError {
    /// Error code identifying the type of error
    /// The code value
    pub code: String,
    /// Human-readable error message
    /// The message value
    pub message: String,
    /// Mapping of details
    pub details: HashMap<String, serde_json::Value>,
}

impl PrimalError {
    /// Create a new primal error
    ///
    /// Creates a new error with the specified error code and message.
    /// The details map is initialized as empty and can be populated later.
    ///
    /// # Arguments
    /// * `code` - Error code identifying the type of error
    /// * `message` - Human-readable error message
    ///
    /// # Returns
    /// A new `PrimalError` instance
    /// Creates a new instance
    pub fn new(code: impl Into<String>, message: impl Into<String>) -> Self {
        Self {
            code: code.into(),
            message: message.into(),
            details: HashMap::new(),
        }
    }

    /// Create an initialization failure error
    ///
    ///
    /// # Arguments
    /// * `message` - Description of the initialization failure
    ///
    /// # Returns
    /// A `PrimalError` with "`InitializationFailed`" error code
    /// Initializes `componentialization_failed`
    /// Initializes `componentialization_failed`
    pub fn initialization_failed(message: impl Into<String>) -> Self {
        Self::new("InitializationFailed", message)
    }

    /// Create a health check failure error
    ///
    ///
    /// # Arguments
    /// * `message` - Description of the health check failure
    ///
    /// # Returns
    /// A `PrimalError` with "`HealthCheckFailed`" error code
    pub fn health_check_failed(message: impl Into<String>) -> Self {
        Self::new("HealthCheckFailed", message)
    }

    /// Create an unsupported operation error
    ///
    /// by the current primal component configuration.
    ///
    /// # Arguments
    /// * `message` - Description of the unsupported operation
    ///
    /// # Returns
    /// A `PrimalError` with "`UnsupportedOperation`" error code
    pub fn unsupported_operation(message: impl Into<String>) -> Self {
        Self::new("UnsupportedOperation", message)
    }
}

impl From<beardog_errors::BearDogError> for PrimalError {
    fn from(err: beardog_errors::BearDogError) -> Self {
        Self::new("BearDogError", format!("{err}"))
    }
}

/// Request to a primal service
///
/// Represents a standardized request format for primal-to-primal communication,
/// including operation specification, parameters, and contextual metadata.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PrimalRequest {
    /// Unique identifier for this primal instance
    pub id: String,
    /// Unique identifier for this specific request
    pub request_id: String,
    /// Type of operation being requested
    pub operation_type: String,
    /// Operation parameters as JSON value
    pub params: serde_json::Value,
    /// Additional contextual metadata
    pub metadata: HashMap<String, serde_json::Value>,
    /// Request creation timestamp
    pub timestamp: DateTime<Utc>,
}

impl Default for PrimalRequest {
    fn default() -> Self {
        Self {
            id: Uuid::new_v4().to_string(),
            request_id: Uuid::new_v4().to_string(),
            operation_type: "default".to_string(),
            params: serde_json::Value::Null,
            metadata: HashMap::new(),
            timestamp: Utc::now(),
        }
    }
}

/// Response from a primal service
///
/// Represents a standardized response format for primal-to-primal communication,
/// including status, success indication, result data, and metadata.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PrimalResponse {
    /// Unique identifier for the responding primal
    pub id: String,
    /// Identifier matching the original request
    pub request_id: String,
    /// Status description of the operation
    pub status: String,
    /// Whether the operation succeeded
    pub success: bool,
    /// Operation result data as JSON value
    pub data: serde_json::Value,
    /// Additional response metadata
    pub metadata: HashMap<String, serde_json::Value>,
    /// Response timestamp
    pub timestamp: DateTime<Utc>,
}

impl Default for PrimalResponse {
    fn default() -> Self {
        Self {
            id: Uuid::new_v4().to_string(),
            request_id: Uuid::new_v4().to_string(),
            status: "pending".to_string(),
            success: false,
            data: serde_json::Value::Null,
            metadata: HashMap::new(),
            timestamp: Utc::now(),
        }
    }
}

/// Health status information for a primal service
///
/// Tracks overall health and component-level health for comprehensive monitoring.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PrimalHealthStatus {
    /// Overall health status of the primal
    pub status: HealthStatus,
    /// Health status of individual components
    pub components: HashMap<String, HealthStatus>,
    /// Timestamp of last health check
    pub last_check: DateTime<Utc>,
    /// Scheduled timestamp for next health check
    pub next_check: DateTime<Utc>,
}

/// Resource usage information for monitoring
///
/// Tracks CPU, memory, network, and disk utilization metrics.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ResourceUsageInfo {
    /// CPU utilization as percentage (0.0-100.0)
    pub cpu_percent: f64,
    /// Memory usage in bytes
    pub memory_bytes: u64,
    /// Network throughput in bytes per second
    pub network_bytes_per_sec: u64,
    /// Disk I/O throughput in bytes per second
    pub disk_bytes_per_sec: u64,
}

/// Configuration data for a primal service
///
/// Flexible key-value configuration storage using JSON values.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PrimalConfig {
    /// Configuration key-value pairs
    pub data: HashMap<String, serde_json::Value>,
}

/// Universal Integration Configuration
/// Replaces hardcoded primal integration flags with capability-based discovery
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct UniversalIntegrationConfig {
    /// Enable capability-based service discovery
    /// Whether `enable_capability_discovery` is enabled
    pub enable_capability_discovery: bool,
    /// Collection of required capabilities
    pub required_capabilities: Vec<ServiceCapabilityType>,
    /// Optional capabilities that enhance functionality
    /// Collection of optional capabilities
    pub optional_capabilities: Vec<ServiceCapabilityType>,
    /// Collection of discovery endpoints
    pub discovery_endpoints: Vec<String>,
    /// Custom capability configurations
    pub custom_config: HashMap<String, serde_json::Value>,
    /// Fallback to environment-based discovery if explicit discovery fails
    /// Whether `enable_environment_discovery` is enabled
    pub enable_environment_discovery: bool,
}

impl Default for UniversalIntegrationConfig {
    fn default() -> Self {
        Self {
            enable_capability_discovery: true,
            required_capabilities: vec![
                ServiceCapabilityType::Security,
                ServiceCapabilityType::Storage,
            ],
            optional_capabilities: vec![
                ServiceCapabilityType::Compute,
                ServiceCapabilityType::ArtificialIntelligence,
                ServiceCapabilityType::Networking,
                ServiceCapabilityType::Orchestration,
            ],
            discovery_endpoints: vec![],
            custom_config: HashMap::new(),
            enable_environment_discovery: true,
        }
    }
}

/// Helper for migrating from hardcoded primal types to capability-based discovery
///
/// Deprecated: Use `CapabilityBasedEcosystem` for capability-based service discovery.
#[deprecated = "Use CapabilityBasedEcosystem instead"]
pub struct PrimalTypeMigrationHelper;

impl PrimalTypeMigrationHelper {
    /// Gets `migration_guidance`
    /// Gets `migration_guidance`
    #[must_use]
    pub const fn get_migration_guidance() -> &'static str {
        r#"
🔄 PRIMAL SOVEREIGNTY MIGRATION GUIDE

STEP 1: Replace hardcoded primal types with capability discovery
❌ OLD (VIOLATES SOVEREIGNTY):
    let primal = PrimalType::SpecificPrimal;
    let client = create_hardcoded_client();

✅ NEW (ACHIEVES SOVEREIGNTY):
    let universal_adapter = UniversalAdapter::new()?;
    let compute_providers = universal_adapter
        .discover_capability(ServiceCapabilityType::ComputeIntelligence)
        ?;

STEP 2: Use discovered capabilities instead of hardcoded names
❌ OLD (VIOLATES SOVEREIGNTY):
    if primal == PrimalType::SpecificPrimal {
        // Hardcoded primal-specific logic
    }

✅ NEW (ACHIEVES SOVEREIGNTY):
    for provider in compute_providers {
        if provider.capabilities.contains(&ServiceCapabilityType::ComputeIntelligence) {
            // Universal capability-based logic
        }
    }

STEP 3: Remove hardcoded endpoint assumptions
❌ OLD (VIOLATES SOVEREIGNTY):
    let endpoint = "http://hardcoded-service:8081";

✅ NEW (ACHIEVES SOVEREIGNTY):
    let endpoint = provider.endpoint.url; // Discovered dynamically

✅ BENEFITS:
- Works with ANY compute provider (no hardcoded assumptions)
- Scales to infinite ecosystem size (no 2^n hardcoding problem)
- True primal sovereignty (each knows only itself)
- Automatic failover and load balancing
- Zero vendor lock-in

📖 Full guide: HARDCODING_ELIMINATION_PLAN_2025.md
"#
    }
}

/// Security attestation for service verification
///
/// Cryptographic proof of service identity, security posture, and compliance.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct SecurityAttestation {
    /// Type of attestation (TPM, SGX, etc.)
    pub attestation_type: String,
    /// Cryptographic signature proving attestation
    pub signature: String,
    /// Timestamp when attestation was created
    pub timestamp: DateTime<Utc>,
    /// Entity that issued the attestation
    pub issuer: String,
}

/// Authentication requirements for service access
///
/// Specifies authentication method, required permissions, and token settings.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize, Default)]
pub struct AuthRequirements {
    /// Authentication type required (JWT, `OAuth2`, etc.)
    #[serde(default)]
    pub auth_type: String,
    /// Required permission scopes for access
    #[serde(default)]
    pub required_scopes: Vec<String>,
    /// Optional token lifetime in seconds
    pub token_lifetime: Option<u64>,
}

/// Security configuration for service endpoints
///
/// Defines TLS requirements, certificate validation, cipher suites, and authentication.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize, Default)]
pub struct EndpointSecurityConfig {
    /// Whether TLS/HTTPS is required
    pub tls_required: bool,
    /// Whether to validate server certificates
    pub cert_validation: bool,
    /// Allowed TLS cipher suites
    pub allowed_ciphers: Vec<String>,
    /// Authentication requirements
    pub auth_requirements: AuthRequirements,
}

/// Load and capacity metrics for services
///
/// Tracks resource utilization and capacity for load balancing decisions.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize, Default)]
pub struct LoadMetrics {
    /// Current CPU usage percentage
    pub cpu_usage: f64,
    /// The memory usage value
    pub memory_usage: f64,
    /// Number of `active_connections`
    pub active_connections: u32,
    /// The requests per second value
    pub requests_per_second: f64,
}

/// Error rate metrics for service monitoring
///
/// Tracks error rates, timeout rates, and categorizes failures for analysis.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize, Default)]
pub struct ErrorRateMetrics {
    /// Overall error rate as percentage (0.0-100.0)
    pub error_rate: f64,
    /// Timeout rate as percentage (0.0-100.0)
    pub timeout_rate: f64,
    /// Count of failures by category
    #[serde(default)]
    pub failure_categories: HashMap<String, u32>,
}
