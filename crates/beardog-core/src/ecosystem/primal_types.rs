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
    Required {
        capability: ServiceCapabilityType,
        min_version: String,
        reason: String,
    },
    Optional {
        capability: ServiceCapabilityType,
        min_version: String,
        reason: String,
    },
}

/// `ServiceMetadata` describes service capabilities without hardcoded references
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ServiceMetadata {
    pub service_id: Uuid,
    /// Collection of capabilities
    pub capabilities: Vec<ServiceCapabilityType>,
    /// Collection of dependencies
    pub dependencies: Vec<ServiceDependency>,
    /// The version value
    pub version: String,
    /// The endpoints value
    pub endpoints: ServiceEndpoints,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ServiceEndpoints {
    /// The health value
    pub health: String,
    /// The metrics value
    pub metrics: String,
    /// The primary value
    pub primary: String,
    /// Optional admin
    pub admin: Option<String>,
    /// Optional websocket
    pub websocket: Option<String>,
}

/// `CapabilityIntegrationConfig` uses capability-based configuration
/// instead of hardcoded primal flags
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CapabilityIntegrationConfig {
    /// Enable security capability integration
    /// Whether `enable_security_capability` is enabled
    pub enable_security_capability: bool,
    /// Enable storage capability integration\
    /// Whether `enable_storage_capability` is enabled
    pub enable_storage_capability: bool,
    /// Enable compute capability integration
    /// Whether `enable_compute_capability` is enabled
    pub enable_compute_capability: bool,
    /// Enable networking capability integration
    /// Whether `enable_networking_capability` is enabled
    pub enable_networking_capability: bool,
    /// Enable AI capability integration
    /// Whether `enable_ai_capability` is enabled
    pub enable_ai_capability: bool,
    /// Custom capability configurations
    /// Mapping of custom capabilities
    pub custom_capabilities: HashMap<String, serde_json::Value>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct AttestationVerificationResult {
    /// Whether verified is enabled
    pub verified: bool,
    pub confidence: f64,
    /// The method value
    pub method: String,
    pub timestamp: DateTime<Utc>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct AttestationVerificationChain {
    /// Whether verified is enabled
    pub verified: bool,
    /// Collection of chain
    pub chain: Vec<String>,
    /// The trust level value
    pub trust_level: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CapabilityHealthStatus {
    /// Current status of the overall
    pub overall_status: String,
    pub individual_status: HashMap<String, String>,
}

pub use beardog_types::canonical::HealthStatus;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct AuthenticationResult {
    /// Whether success is enabled
    pub success: bool,
    pub user_id: Option<String>,
    /// Optional token
    pub token: Option<String>,
    /// Optional expires at
    pub expires_at: Option<DateTime<Utc>>,
    /// Collection of permissions
    pub permissions: Vec<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PrimalHealth {
    /// Current status of the component
    pub status: HealthStatus,
    /// The last check value
    pub last_check: DateTime<Utc>,
    /// Mapping of details
    pub details: HashMap<String, serde_json::Value>,
    /// Mapping of checks
    pub checks: HashMap<String, bool>,
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

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct KeyOperationStatus {
    /// Whether healthy is enabled
    pub healthy: bool,
    /// Collection of operations tested
    pub operations_tested: Vec<String>,
    pub response_times: serde_json::Value,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct EndpointHealth {
    /// Name of the item
    pub name: String,
    /// Current status of the component
    pub status: String,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize, Default)]
pub struct ResponseTimeMetrics {
    /// The average value
    pub average: f64,
    /// The p95 value
    pub p95: f64,
    /// The p99 value
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

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct UniversalEndpoint {
    /// Base URL (discovered, not hardcoded)
    /// The url value
    pub url: String,
    /// Supported protocols (HTTP, gRPC, WebSocket, etc.)
    /// Collection of protocols
    pub protocols: Vec<String>,
    /// Authentication requirements
    /// The auth requirements value
    pub auth_requirements: AuthRequirements,
    /// TLS/security configuration
    pub security_config: EndpointSecurityConfig,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct PrimalMetrics {
    /// Response time percentiles
    pub response_times: ResponseTimeMetrics,
    /// Availability percentage
    /// The availability value
    pub availability: f64,
    /// Current load/capacity
    /// The load metrics value
    pub load_metrics: LoadMetrics,
    /// Error rates
    /// The error rates value
    pub error_rates: ErrorRateMetrics,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq, Hash)]
pub enum PrimalCapability {
    /// Represents security variant
    Security,
    /// Represents storage variant
    Storage,
    /// Represents compute variant
    Compute,
    /// Currently networking
    Networking,
    /// Represents a i variant
    AI,
    /// Represents custom variant
    Custom(String),
}

/// Deprecated: Primal dependency specification (use `ServiceDependency` instead)
#[derive(Debug, Clone, Serialize, Deserialize)]
#[deprecated(note = "Use ServiceDependency for capability-based dependencies")]
#[allow(deprecated)]
pub enum PrimalDependency {
    /// State indicating required
    Required {
        capability: ServiceCapabilityType, // Migrated from PrimalType to capability-based
        min_version: String,
        reason: String,
    },
    Optional {
        capability: ServiceCapabilityType, // Migrated from PrimalType to capability-based
        min_version: String,
        reason: String,
    },
}

#[allow(deprecated)]
impl PrimalDependency {
    /// Convert to capability-based dependency
    /// Converts to service dependency
    pub fn to_service_dependency(&self) -> ServiceDependency {
        match self {
            Self::Required {
                capability,
                min_version,
                reason,
            } => ServiceDependency::Required {
                capability: capability.clone(), // Already a ServiceCapabilityType
                min_version: min_version.clone(),
                reason: reason.clone(),
            },
            Self::Optional {
                capability,
                min_version,
                reason,
            } => ServiceDependency::Optional {
                capability: capability.clone(), // Already a ServiceCapabilityType
                min_version: min_version.clone(),
                reason: reason.clone(),
            },
        }
    }
}

/// DEPRECATED: Use `UniversalIntegrationConfig` instead
#[derive(Debug, Clone, Serialize, Deserialize)]
#[deprecated(note = "Use UniversalIntegrationConfig for capability-based integration")]
pub struct PrimalIntegrationConfig {
    /// DEPRECATED: Use capability-based discovery instead
    #[deprecated(note = "Use ServiceCapabilityType::ComputeIntelligence discovery")]
    /// Whether `enable_compute_capability` is enabled
    pub enable_compute_capability: bool,
    /// DEPRECATED: Use capability-based discovery instead
    #[deprecated(
        note = "Use ServiceCapabilityType::DistributedIntelligence discovery instead of hardcoded AI integration"
    )]
    /// Whether `enable_ai_capability` is enabled
    pub enable_ai_capability: bool,
    /// DEPRECATED: Use capability-based discovery instead
    #[deprecated(note = "Use ServiceCapabilityType::DataStorage discovery")]
    /// Whether `enable_storage_capability` is enabled
    pub enable_storage_capability: bool,
    /// DEPRECATED: Use capability-based discovery instead
    #[deprecated(note = "Use ServiceCapabilityType::DistributedIntelligence discovery")]
    /// Whether `enable_ai_api` is enabled
    pub enable_ai_api: bool,
    pub custom_config: HashMap<String, serde_json::Value>,
}

impl PrimalIntegrationConfig {
    /// Convert to modern universal integration config
    /// Converts to universal config
    pub fn to_universal_config(&self) -> UniversalIntegrationConfig {
        let mut required_capabilities = vec![ServiceCapabilityType::Security];
        let optional_capabilities = vec![];

        #[allow(deprecated)]
        {
            if self.enable_compute_capability {
                required_capabilities.push(ServiceCapabilityType::ComputeIntelligence);
            }
            if self.enable_ai_capability || self.enable_ai_api {
                required_capabilities.push(ServiceCapabilityType::DistributedIntelligence);
            }
            if self.enable_storage_capability {
                required_capabilities.push(ServiceCapabilityType::DataStorage);
            }
        }

        UniversalIntegrationConfig {
            enable_capability_discovery: true,
            required_capabilities,
            optional_capabilities,
            discovery_endpoints: vec![],
            custom_config: self.custom_config.clone(),
            enable_environment_discovery: true,
        }
    }
}

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

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PrimalRequest {
    pub id: String,
    pub request_id: String,
    /// The operation type value
    pub operation_type: String,
    /// The params value
    pub params: serde_json::Value,
    /// Mapping of metadata
    pub metadata: HashMap<String, serde_json::Value>,
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

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PrimalResponse {
    pub id: String,
    pub request_id: String,
    /// Current status of the component
    pub status: String,
    /// Whether success is enabled
    pub success: bool,
    /// The data value
    pub data: serde_json::Value,
    /// Mapping of metadata
    pub metadata: HashMap<String, serde_json::Value>,
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

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PrimalHealthStatus {
    /// Current status of the component
    pub status: HealthStatus,
    /// Mapping of components
    pub components: HashMap<String, HealthStatus>,
    /// The last check value
    pub last_check: DateTime<Utc>,
    /// The next check value
    pub next_check: DateTime<Utc>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ResourceUsageInfo {
    /// The cpu percent value
    pub cpu_percent: f64,
    /// Number of `memory_bytes`
    pub memory_bytes: u64,
    /// Number of `network_bytes_per_sec`
    pub network_bytes_per_sec: u64,
    /// Number of `disk_bytes_per_sec`
    pub disk_bytes_per_sec: u64,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PrimalConfig {
    /// Mapping of data
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

#[deprecated = "Use CapabilityBasedEcosystem instead"]
pub struct PrimalTypeMigrationHelper;

impl PrimalTypeMigrationHelper {
    /// Gets `migration_guidance`
    /// Gets `migration_guidance`
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

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct SecurityAttestation {
    /// The attestation type value
    pub attestation_type: String,
    /// The signature value
    pub signature: String,
    pub timestamp: DateTime<Utc>,
    /// The issuer value
    pub issuer: String,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize, Default)]
pub struct AuthRequirements {
    /// The auth type value
    #[serde(default)]
    pub auth_type: String,
    /// Collection of required scopes
    #[serde(default)]
    pub required_scopes: Vec<String>,
    pub token_lifetime: Option<u64>,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize, Default)]
pub struct EndpointSecurityConfig {
    /// Whether `tls_required` is enabled
    pub tls_required: bool,
    pub cert_validation: bool,
    /// Collection of allowed ciphers
    pub allowed_ciphers: Vec<String>,
    /// The auth requirements value
    pub auth_requirements: AuthRequirements,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize, Default)]
pub struct LoadMetrics {
    /// The cpu usage value
    pub cpu_usage: f64,
    /// The memory usage value
    pub memory_usage: f64,
    /// Number of `active_connections`
    pub active_connections: u32,
    /// The requests per second value
    pub requests_per_second: f64,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize, Default)]
pub struct ErrorRateMetrics {
    /// The error rate value
    pub error_rate: f64,
    pub timeout_rate: f64,
    /// Mapping of failure categories
    #[serde(default)]
    pub failure_categories: HashMap<String, u32>,
}
