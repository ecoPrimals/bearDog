// SPDX-License-Identifier: AGPL-3.0-only

//! Canonical Capability Definitions for BearDog
//!
//! Provides structured capability types for security, performance, compliance,
//! and ecosystem integration. This module defines the capability-based discovery
//! system that eliminates hardcoded dependencies.
//!
//! ## Capability-Based Architecture
//!
//! Instead of hardcoding which services exist, BearDog discovers capabilities
//! dynamically through:
//!
//! 1. **Vendor Capabilities** - External services (AWS KMS, Azure KeyVault, etc.)
//! 2. **Primal Capabilities** - Other primals in the ecosystem
//! 3. **Cross-Cutting Capabilities** - Monitoring, logging, metrics
//! 4. **Specialized Capabilities** - Biometrics, quantum crypto, zero-knowledge
//!
//! ## Example
//!
//! ```rust
//! use beardog_types::canonical::capabilities::CapabilityType;
//!
//! // Define a capability type
//! let capability = CapabilityType::KeyManagement;
//! println!("Capability: {:?}", capability);
//! ```

use serde::{Deserialize, Serialize};
use std::collections::HashMap;
// Removed incorrect imports added by migration script
// These types are defined in this file, not imported from elsewhere

#[cfg(test)]
#[path = "capabilities_tests.rs"]
mod capabilities_tests;

/// Capability Type - Types of capabilities in the BearDog ecosystem
///
/// Defines all discoverable capabilities that can be provided by:
/// - External vendors (AWS, Azure, Google Cloud)
/// - Other primals in the ecosystem
/// - Internal BearDog services
///
/// ## Categories
///
/// - **Vendor Capabilities**: External cloud services
/// - **Primal Capabilities**: Ecosystem services
/// - **Cross-Cutting**: Monitoring, logging, metrics
/// - **Specialized**: Advanced cryptographic and AI features
///
/// ## Usage
///
/// Capabilities are discovered dynamically rather than hardcoded:
///
/// ```rust
/// # use beardog_types::canonical::capabilities::CapabilityType;
/// // Query for capability providers
/// let cap = CapabilityType::HardwareSecurityModule;
/// println!("Looking for: {}", cap.name());
/// ```
#[derive(Debug, Clone, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub enum CapabilityType {
    // === VENDOR CAPABILITIES ===
    /// Key Management Service (discovered through capability-based detection)
    KeyManagement,
    /// Hardware Security Module (discovered through capability-based detection)
    HardwareSecurityModule,
    /// Secrets Management (discovered through capability-based detection)
    SecretsManagement,
    /// Authentication Service (OAuth, SAML, OIDC providers)
    Authentication,
    /// Cloud Storage (discovered through capability-based detection)
    CloudStorage,
    /// Database Service (discovered through capability-based detection)
    DatabaseService,
    /// Load Balancing (discovered through capability-based detection)
    LoadBalancing,
    /// Content Delivery Network (discovered through capability-based detection)
    ContentDeliveryNetwork,

    // === PRIMAL CAPABILITIES ===
    /// Service Mesh and Network Routing
    ServiceMesh,
    /// Compute and AI Intelligence
    ComputeIntelligence,
    /// Data Storage and Management
    DataStorage,
    /// Distributed AI and ML
    DistributedIntelligence,
    /// Container Orchestration
    ContainerOrchestration,
    /// Security and Cryptography
    Security,

    // === CROSS-CUTTING CAPABILITIES ===
    /// Monitoring and Observability
    Monitoring,
    /// Centralized Logging
    Logging,
    /// Metrics Collection and Analysis
    Metrics,
    /// Health Checking and Status
    HealthChecking,
    /// Configuration Management
    Configuration,
    /// Network Communication
    Network,
    /// Message Queuing
    MessageQueue,
    /// Event Streaming
    EventStreaming,
    /// Workflow Orchestration
    WorkflowOrchestration,

    // === COMPATIBILITY ALIASES ===
    Storage,
    Compute,
    Networking,
    ArtificialIntelligence,
    Orchestration,

    // === SPECIALIZED CAPABILITIES ===
    /// Biometric Authentication
    BiometricAuth,
    /// Quantum-Resistant Cryptography
    QuantumCrypto,
    /// Zero-Knowledge Proofs
    ZeroKnowledgeProofs,
    /// Genetic Algorithm Processing
    GeneticAlgorithms,
    /// Threat Detection and Response
    ThreatDetection,
    /// Compliance and Audit
    ComplianceAudit,

    /// Custom capability with name
    Custom(String),
}

impl CapabilityType {
    #[must_use]
    pub fn name(&self) -> String {
        match self {
            Self::KeyManagement => "Key Management".to_string(),
            Self::HardwareSecurityModule => "Hardware Security Module".to_string(),
            Self::SecretsManagement => "Secrets Management".to_string(),
            Self::Authentication => "Authentication".to_string(),
            Self::CloudStorage => "Cloud Storage".to_string(),
            Self::DatabaseService => "Database Service".to_string(),
            Self::LoadBalancing => "Load Balancing".to_string(),
            Self::ContentDeliveryNetwork => "Content Delivery Network".to_string(),
            Self::ServiceMesh => "Service Mesh".to_string(),
            Self::ComputeIntelligence => "Compute Intelligence".to_string(),
            Self::DataStorage => "Data Storage".to_string(),
            Self::DistributedIntelligence => "Distributed Intelligence".to_string(),
            Self::ContainerOrchestration => "Container Orchestration".to_string(),
            Self::Security => "Security".to_string(),
            Self::Monitoring => "Monitoring".to_string(),
            Self::Logging => "Logging".to_string(),
            Self::Metrics => "Metrics".to_string(),
            Self::HealthChecking => "Health Checking".to_string(),
            Self::Configuration => "Configuration".to_string(),
            Self::Network => "Network".to_string(),
            Self::MessageQueue => "Message Queue".to_string(),
            Self::EventStreaming => "Event Streaming".to_string(),
            Self::WorkflowOrchestration => "Workflow Orchestration".to_string(),
            Self::Storage => "Storage".to_string(),
            Self::Compute => "Compute".to_string(),
            Self::Networking => "Networking".to_string(),
            Self::ArtificialIntelligence => "Artificial Intelligence".to_string(),
            Self::Orchestration => "Orchestration".to_string(),
            Self::BiometricAuth => "Biometric Authentication".to_string(),
            Self::QuantumCrypto => "Quantum-Resistant Cryptography".to_string(),
            Self::ZeroKnowledgeProofs => "Zero-Knowledge Proofs".to_string(),
            Self::GeneticAlgorithms => "Genetic Algorithms".to_string(),
            Self::ThreatDetection => "Threat Detection".to_string(),
            Self::ComplianceAudit => "Compliance and Audit".to_string(),
            Self::Custom(name) => name.clone(),
        }
    }

    #[must_use]
    /// Returns as capability id
    pub fn as_capability_id(&self) -> String {
        match self {
            Self::Security => "capability:security".to_string(),
            Self::Storage => "capability:storage".to_string(),
            Self::Compute => "capability:compute".to_string(),
            Self::Networking => "capability:networking".to_string(),
            Self::ArtificialIntelligence => "capability:ai".to_string(),
            Self::Orchestration => "capability:orchestration".to_string(),
            Self::Monitoring => "capability:monitoring".to_string(),
            Self::ComputeIntelligence => "capability:compute-intelligence".to_string(),
            Self::DistributedIntelligence => "capability:distributed-intelligence".to_string(),
            Self::ServiceMesh => "capability:service-mesh".to_string(),
            Self::DataStorage => "capability:data-storage".to_string(),
            Self::Custom(name) => format!("capability:custom:{name}"),
            _ => format!(
                "capability:{}",
                self.name().to_lowercase().replace(' ', "-")
            ),
        }
    }

    /// Check if this is a vendor capability (external service)
    #[must_use]
    /// Checks if vendor capability
    /// Checks if vendor capability
    #[inline]
    pub const fn is_vendor_capability(&self) -> bool {
        matches!(
            self,
            Self::KeyManagement
                | Self::HardwareSecurityModule
                | Self::SecretsManagement
                | Self::Authentication
                | Self::CloudStorage
                | Self::DatabaseService
                | Self::LoadBalancing
                | Self::ContentDeliveryNetwork
        )
    }

    /// Check if this is a primal capability (ecosystem service)
    #[must_use]
    /// Checks if primal capability
    /// Checks if primal capability
    #[inline]
    pub const fn is_primal_capability(&self) -> bool {
        matches!(
            self,
            Self::ServiceMesh
                | Self::ComputeIntelligence
                | Self::DataStorage
                | Self::DistributedIntelligence
                | Self::ContainerOrchestration
                | Self::Security
        )
    }

    /// Get the primal name associated with this capability (if any)
    /// DEPRECATED: Removed hardcoded primal mappings to achieve true capability-based discovery
    #[deprecated(note = "Use capability-based discovery instead of hardcoded primal names")]
    #[must_use]
    #[inline]
    pub const fn associated_primal(&self) -> Option<&'static str> {
        // EVOLUTION: No longer return hardcoded primal names
        // Each primal should discover capabilities dynamically through universal adapter
        // No capability available
        None
    }
}

impl std::fmt::Display for CapabilityType {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.name())
    }
}

impl From<CapabilityType> for String {
    fn from(capability: CapabilityType) -> Self {
        capability.as_capability_id()
    }
}

/// Use `CapabilityType` directly in new code
pub type ServiceCapabilityType = CapabilityType;

#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord, Serialize, Deserialize)]
pub enum SecurityLevel {
    /// Basic security level
    Basic,
    /// Standard security level
    Standard,
    /// High security level
    High,
    /// Critical security level
    Critical,
}

impl Default for SecurityLevel {
    fn default() -> Self {
        Self::Standard
    }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct UniversalCapability {
    /// The capability type
    /// The capability type value
    pub capability_type: CapabilityType,
    pub provider: ProviderInfo,
    /// Endpoint configuration
    /// The endpoint value
    pub endpoint: EndpointConfig,
    /// Authentication configuration
    pub auth_config: AuthConfig,
    /// Health status
    /// Current status of the health
    pub health_status: HealthStatus,
    pub performance: PerformanceMetrics,
    /// Security level
    /// The security level value
    pub security_level: SecurityLevel,
    /// Additional metadata
    /// Mapping of metadata
    pub metadata: HashMap<String, serde_json::Value>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ProviderInfo {
    /// Provider identifier (discovered dynamically via capability-based discovery)
    pub provider_id: String,
    /// Human-readable provider name
    pub provider_name: String,
    /// Provider type (vendor, primal, custom)
    pub provider_type: crate::canonical::providers_unified::core::ProviderType,
    /// Provider version
    /// The version value
    pub version: String,
    /// Provider region or location
    /// Optional region
    pub region: Option<String>,
}

/// Type of capability provider
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
// Removed duplicate derive - using the one above
/// `EndpointConfig` data structure
pub struct EndpointConfig {
    /// The base url value
    pub base_url: String,
    /// API version
    /// Optional api version
    pub api_version: Option<String>,
    /// Timeout configuration
    pub timeout_ms: u64,
    /// Retry configuration
    /// Number of `max_retries`
    pub max_retries: u32,
    /// Circuit breaker configuration
    /// The circuit breaker value
    pub circuit_breaker: CircuitBreakerConfig,
}

/// Circuit breaker configuration
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct CircuitBreakerConfig {
    /// Number of `failure_threshold`
    pub failure_threshold: u32,
    pub timeout_ms: u64,
    /// Number of `success_threshold`
    pub success_threshold: u32,
}

impl Default for CircuitBreakerConfig {
    /// Pure defaults without environment variable access
    /// Concurrent-safe and suitable for testing
    fn default() -> Self {
        Self {
            failure_threshold: 5,
            timeout_ms: 60000, // 60 seconds
            success_threshold: 3,
        }
    }
}

impl CircuitBreakerConfig {
    /// Load from environment or use defaults (production use)
    pub fn from_env() -> Self {
        Self {
            failure_threshold: std::env::var("BEARDOG_CIRCUIT_BREAKER_FAILURE_THRESHOLD")
                .ok()
                .and_then(|s| s.parse().ok())
                .unwrap_or(5),
            timeout_ms: std::env::var("BEARDOG_CIRCUIT_BREAKER_TIMEOUT_MS")
                .ok()
                .and_then(|s| s.parse().ok())
                .unwrap_or(60000), // 60 seconds
            success_threshold: std::env::var("BEARDOG_CIRCUIT_BREAKER_SUCCESS_THRESHOLD")
                .ok()
                .and_then(|s| s.parse().ok())
                .unwrap_or(3),
        }
    }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct AuthConfig {
    /// Authentication type
    /// The auth type value
    pub auth_type: AuthType,
    /// API key (if applicable)
    /// Optional api key
    pub api_key: Option<String>,
    /// Bearer token (if applicable)
    /// Optional bearer token
    pub bearer_token: Option<String>,
    /// Certificate path (if applicable)
    /// Optional cert path
    pub cert_path: Option<String>,
    /// Additional auth parameters
    /// Mapping of custom params
    pub custom_params: HashMap<String, String>,
}

/// Authentication type
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
/// Types of auth
pub enum AuthType {
    /// No authentication required
    None,
    /// API key authentication
    ApiKey,
    /// Bearer token authentication
    Bearer,
    /// Mutual TLS authentication
    MutualTLS,
    /// OAuth 2.0 authentication
    OAuth2,
    /// Custom authentication method
    Custom(String),
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub enum HealthStatus {
    /// Capability is healthy and available
    Healthy,
    /// Capability is degraded but functional
    Degraded,
    /// Capability is unhealthy
    Unhealthy,
    /// Health status is unknown
    Unknown,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PerformanceMetrics {
    /// Average response time in milliseconds
    pub avg_response_time_ms: f64,
    /// Success rate (0.0 to 1.0)
    /// The success rate value
    pub success_rate: f64,
    /// Throughput (requests per second)
    /// The throughput rps value
    pub throughput_rps: f64,
    /// Current load (0.0 to 1.0)
    /// The current load value
    pub current_load: f64,
    /// Last updated timestamp
    /// The last updated value
    pub last_updated: chrono::DateTime<chrono::Utc>,
}

impl Default for PerformanceMetrics {
    fn default() -> Self {
        Self {
            avg_response_time_ms: 0.0,
            success_rate: 1.0,
            throughput_rps: 0.0,
            current_load: 0.0,
            last_updated: chrono::Utc::now(),
        }
    }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SecurityCapabilities {
    /// Authentication methods supported
    /// Collection of authentication methods
    pub authentication_methods: Vec<String>,
    /// Role-based access control enabled
    /// Whether rbac is enabled
    pub rbac: bool,
    /// Audit logging enabled
    /// Whether `audit_logging` is enabled
    pub audit_logging: bool,
    /// Secure protocols supported
    /// Collection of secure protocols
    pub secure_protocols: Vec<String>,
    /// Compliance certifications held
    /// Collection of compliance certifications
    pub compliance_certifications: Vec<String>,
    /// Security level classification
    /// The security level value
    pub security_level: SecurityLevel,
}

impl Default for SecurityCapabilities {
    fn default() -> Self {
        Self {
            authentication_methods: vec![
                "password".to_string(),
                "biometric".to_string(),
                "hardware_key".to_string(),
            ],
            rbac: true,
            audit_logging: true,
            secure_protocols: vec!["TLS".to_string(), "HTTPS".to_string()],
            compliance_certifications: vec!["ISO27001".to_string()],
            security_level: SecurityLevel::High,
        }
    }
}

/// Compliance requirements and certifications
#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct ComplianceCapabilities {
    /// Regulatory frameworks supported
    /// Collection of frameworks
    pub frameworks: Vec<String>,
    /// Compliance level achieved
    /// The compliance level value
    pub compliance_level: ComplianceLevel,
    /// Audit requirements
    /// Collection of audit requirements
    pub audit_requirements: Vec<String>,
    /// Data residency requirements
    pub data_residency: Vec<String>,
    /// Encryption requirements
    /// The encryption requirements value
    pub encryption_requirements: EncryptionRequirements,
}

/// Compliance level classification
#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord, Serialize, Deserialize)]
pub enum ComplianceLevel {
    /// Basic compliance
    Basic,
    /// Standard compliance (SOC 2 Type I)
    Standard,
    /// High compliance (SOC 2 Type II, ISO 27001)
    High,
    /// Critical compliance (`FedRAMP`, FIPS 140-2)
    Critical,
}

impl Default for ComplianceLevel {
    fn default() -> Self {
        Self::Basic
    }
}

#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct EncryptionRequirements {
    /// Minimum encryption strength in bits
    /// Number of `min_key_size`
    pub min_key_size: u32,
    /// Required encryption algorithms
    /// Collection of required algorithms
    pub required_algorithms: Vec<String>,
    /// Key management requirements
    /// Collection of key management
    pub key_management: Vec<String>,
    /// Data-at-rest encryption required
    /// Whether `data_at_rest` is enabled
    pub data_at_rest: bool,
    /// Data-in-transit encryption required
    /// Whether `data_in_transit` is enabled
    pub data_in_transit: bool,
}

/// Capability discovery request
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CapabilityDiscoveryRequest {
    /// Capabilities to discover
    /// Collection of capability types
    pub capability_types: Vec<CapabilityType>,
    /// Minimum security level required
    /// Optional min security level
    pub min_security_level: Option<SecurityLevel>,
    /// Maximum response time requirement (ms)
    pub max_response_time_ms: Option<u64>,
    /// Minimum success rate required (0.0 to 1.0)
    /// Optional min success rate
    pub min_success_rate: Option<f64>,
    /// Preferred regions
    /// Collection of preferred regions
    pub preferred_regions: Vec<String>,
    /// Required compliance levels
    /// Collection of required compliance
    pub required_compliance: Vec<ComplianceLevel>,
}

/// Capability discovery response
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CapabilityDiscoveryResponse {
    /// Discovered capabilities
    /// Collection of capabilities
    pub capabilities: Vec<UniversalCapability>,
    /// Discovery metadata
    /// The metadata value
    pub metadata: DiscoveryMetadata,
}

/// Discovery metadata
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct DiscoveryMetadata {
    /// Discovery timestamp
    pub timestamp: chrono::DateTime<chrono::Utc>,
    /// Discovery duration in milliseconds
    /// Number of `discovery_duration_ms`
    pub discovery_duration_ms: u64,
    /// Number of providers queried
    pub providers_queried: u32,
    /// Number of capabilities found
    /// Number of `capabilities_found`
    pub capabilities_found: u32,
}

/// Network capabilities and connectivity options
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct NetworkCapabilities {
    /// Supported protocols
    /// Collection of protocols
    pub protocols: Vec<String>,
    /// Maximum bandwidth in Mbps
    pub max_bandwidth: f64,
    /// Encryption support
    /// Whether `encryption_support` is enabled
    pub encryption_support: bool,
    /// VPN support enabled
    /// Whether `vpn_support` is enabled
    pub vpn_support: bool,
}

impl Default for NetworkCapabilities {
    fn default() -> Self {
        Self {
            protocols: vec![
                "HTTP".to_string(),
                "HTTPS".to_string(),
                "WebSocket".to_string(),
            ],
            max_bandwidth: std::env::var("BEARDOG_MAX_BANDWIDTH_MBPS")
                .ok()
                .and_then(|v| v.parse().ok())
                .unwrap_or(1000.0),
            encryption_support: true,
            vpn_support: true,
        }
    }
}

/// Storage capabilities and data management
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct StorageCapabilities {
    /// Storage types supported
    /// Collection of storage types
    pub storage_types: Vec<String>,
    /// Maximum storage capacity in GB
    /// The max capacity value
    pub max_capacity: f64,
    /// Encryption at rest
    /// Whether `encryption_at_rest` is enabled
    pub encryption_at_rest: bool,
    /// Backup capabilities
    /// Whether `backup_support` is enabled
    pub backup_support: bool,
}

impl Default for StorageCapabilities {
    fn default() -> Self {
        Self {
            storage_types: vec!["SSD".to_string(), "NVMe".to_string(), "Cloud".to_string()],
            max_capacity: std::env::var("BEARDOG_STORAGE_MAX_CAPACITY")
                .ok()
                .and_then(|s| s.parse().ok())
                .unwrap_or(10000.0),
            encryption_at_rest: true,
            backup_support: true,
        }
    }
}

/// Compute capabilities and processing power
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ComputeCapabilities {
    /// CPU architectures supported
    /// Collection of cpu architectures
    pub cpu_architectures: Vec<String>,
    /// Number of cores available
    /// Number of core
    pub core_count: u32,
    /// Memory capacity in GB
    /// The memory gb value
    pub memory_gb: f64,
    /// GPU acceleration available
    /// Whether `gpu_acceleration` is enabled
    pub gpu_acceleration: bool,
}

impl Default for ComputeCapabilities {
    fn default() -> Self {
        Self {
            cpu_architectures: vec!["x86_64".to_string(), "ARM64".to_string()],
            core_count: std::env::var("BEARDOG_DEFAULT_CORE_COUNT")
                .ok()
                .and_then(|s| s.parse().ok())
                .unwrap_or(8),
            memory_gb: std::env::var("BEARDOG_DEFAULT_MEMORY_GB")
                .ok()
                .and_then(|s| s.parse().ok())
                .unwrap_or(32.0),
            gpu_acceleration: false,
        }
    }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PerformanceCapabilities {
    /// Maximum operations per second
    /// The max ops per second value
    pub max_ops_per_second: f64,
    /// Average response time in milliseconds
    pub avg_response_time_ms: f64,
    /// Throughput optimization enabled
    /// Whether `throughput_optimization` is enabled
    pub throughput_optimization: bool,
    /// Load balancing support
    /// Whether `load_balancing` is enabled
    pub load_balancing: bool,
}

impl Default for PerformanceCapabilities {
    fn default() -> Self {
        Self {
            max_ops_per_second: std::env::var("BEARDOG_PERF_MAX_OPS_PER_SECOND")
                .ok()
                .and_then(|v| v.parse().ok())
                .unwrap_or(10000.0),
            avg_response_time_ms: std::env::var("BEARDOG_PERF_AVG_RESPONSE_TIME_MS")
                .ok()
                .and_then(|v| v.parse().ok())
                .unwrap_or(50.0),
            throughput_optimization: true,
            load_balancing: true,
        }
    }
}

/// Environmental capabilities and sustainability
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct EnvironmentalCapabilities {
    /// Power consumption in watts
    /// The power consumption watts value
    pub power_consumption_watts: f64,
    /// Carbon footprint optimization
    /// Whether `carbon_optimization` is enabled
    pub carbon_optimization: bool,
    /// Renewable energy usage
    /// Whether `renewable_energy` is enabled
    pub renewable_energy: bool,
    /// Environmental certifications
    /// Collection of certifications
    pub certifications: Vec<String>,
}

impl Default for EnvironmentalCapabilities {
    fn default() -> Self {
        Self {
            power_consumption_watts: std::env::var("BEARDOG_POWER_CONSUMPTION_WATTS")
                .ok()
                .and_then(|v| v.parse().ok())
                .unwrap_or(500.0),
            carbon_optimization: true,
            renewable_energy: true,
            certifications: vec!["Energy Star".to_string(), "Green Computing".to_string()],
        }
    }
}

#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct SystemCapabilities {
    /// Security capabilities
    /// The security capabilities value
    pub security_capabilities: SecurityCapabilities,
    /// Compliance capabilities
    /// The compliance capabilities value
    pub compliance_capabilities: ComplianceCapabilities,
    /// Network capabilities
    /// The network capabilities value
    pub network_capabilities: NetworkCapabilities,
    /// Storage capabilities
    /// The storage capabilities value
    pub storage_capabilities: StorageCapabilities,
    /// Compute capabilities
    /// The compute capabilities value
    pub compute_capabilities: ComputeCapabilities,
    pub performance_capabilities: PerformanceCapabilities,
    /// Environmental capabilities
    /// The environmental capabilities value
    pub environmental_capabilities: EnvironmentalCapabilities,
}

impl SystemCapabilities {
    /// Create new system capabilities with default values
    #[must_use]
    /// Creates a new instance
    pub fn new() -> Self {
        Self::default()
    }

    /// Get the overall security level of the system
    #[must_use]
    pub fn security_level(&self) -> &SecurityLevel {
        &self.security_capabilities.security_level
    }

    /// Check if the system meets minimum security requirements
    #[must_use]
    pub fn meets_security_requirements(&self) -> bool {
        self.security_capabilities.rbac
            && self.security_capabilities.audit_logging
            && !self.security_capabilities.secure_protocols.is_empty()
    }

    /// Get total storage capacity
    #[must_use]
    pub fn total_storage_capacity(&self) -> f64 {
        self.storage_capabilities.max_capacity
    }

    /// Check if environmental optimization is enabled
    #[must_use]
    /// Checks if environmentally optimized
    /// Checks if environmentally optimized
    pub fn is_environmentally_optimized(&self) -> bool {
        self.environmental_capabilities.carbon_optimization
            && self.environmental_capabilities.renewable_energy
    }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CapabilityRequirements {
    /// Minimum security level required
    /// The min security level value
    pub min_security_level: SecurityLevel,
    /// Required security capabilities
    /// The security requirements value
    pub security_requirements: SecurityCapabilities,
    pub performance_requirements: PerformanceCapabilities,
    /// Required compliance standards
    /// Collection of compliance requirements
    pub compliance_requirements: Vec<String>,
    /// Hardware requirements
    /// Mapping of hardware requirements
    pub hardware_requirements: HashMap<String, String>,
    /// Software requirements
    /// Mapping of software requirements
    pub software_requirements: HashMap<String, String>,
    /// Network requirements
    /// Mapping of network requirements
    pub network_requirements: HashMap<String, String>,
    /// Optional features
    /// Collection of optional features
    pub optional_features: Vec<String>,
}

impl Default for CapabilityRequirements {
    fn default() -> Self {
        Self {
            min_security_level: SecurityLevel::Standard,
            security_requirements: SecurityCapabilities::default(),
            performance_requirements: PerformanceCapabilities::default(),
            compliance_requirements: vec!["SOC2".to_string()],
            hardware_requirements: HashMap::new(),
            software_requirements: HashMap::new(),
            network_requirements: HashMap::new(),
            optional_features: Vec::new(),
        }
    }
}

/// Human entropy generation capabilities
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct HumanEntropyCapabilities {
    /// Supported input methods
    /// Collection of input methods
    pub input_methods: Vec<String>,
    pub min_interaction_time_ms: u32,
    pub max_interaction_time_ms: u32,
    /// Entropy quality scoring
    /// Whether `quality_scoring` is enabled
    pub quality_scoring: bool,
    /// Biometric integration
    /// Whether `biometric_integration` is enabled
    pub biometric_integration: bool,
    /// Mouse movement tracking
    /// Whether `mouse_tracking` is enabled
    pub mouse_tracking: bool,
    /// Keyboard timing analysis
    /// Whether `keyboard_timing` is enabled
    pub keyboard_timing: bool,
    /// Touch pattern analysis
    /// Whether `touch_patterns` is enabled
    pub touch_patterns: bool,
    /// Voice pattern analysis
    /// Whether `voice_patterns` is enabled
    pub voice_patterns: bool,
    /// Behavioral analysis
    /// Whether `behavioral_analysis` is enabled
    pub behavioral_analysis: bool,
}

impl Default for HumanEntropyCapabilities {
    fn default() -> Self {
        Self {
            input_methods: vec![
                "mouse".to_string(),
                "keyboard".to_string(),
                "touch".to_string(),
            ],
            min_interaction_time_ms: std::env::var("BEARDOG_MIN_INTERACTION_TIME_MS")
                .ok()
                .and_then(|s| s.parse().ok())
                .unwrap_or(100),
            max_interaction_time_ms: std::env::var("BEARDOG_MAX_INTERACTION_TIME_MS")
                .ok()
                .and_then(|s| s.parse().ok())
                .unwrap_or(30000),
            quality_scoring: true,
            biometric_integration: false,
            mouse_tracking: true,
            keyboard_timing: true,
            touch_patterns: true,
            voice_patterns: false,
            behavioral_analysis: true,
        }
    }
}
