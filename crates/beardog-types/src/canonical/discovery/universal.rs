// Universal Discovery Types - Zero Hardcoding Architecture
//
// This module defines discovery types that work with any service, vendor, or primal
// through capability-based interfaces. No hardcoded names, endpoints, or assumptions.

use serde::{Deserialize, Serialize};
use std::collections::HashMap;

/// Universal capability that any service can provide
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
/// Types of universal capability
pub enum UniversalCapabilityType {
    /// Compute capabilities (AI, processing, analysis)
    Compute {
        /// Specific compute abilities
        abilities: Vec<ComputeAbility>,
    },
    /// Storage capabilities (data persistence, retrieval)
    Storage {
        /// Storage characteristics
        characteristics: Vec<StorageCharacteristic>,
    },
    /// Network capabilities (mesh, routing, discovery)
    Network {
        /// Network functions
        functions: Vec<NetworkFunction>,
    },
    /// Security capabilities (encryption, authentication)
    Security {
        /// Security services
        services: Vec<SecurityService>,
    },
    /// Orchestration capabilities (container management, deployment)
    Orchestration {
        /// Orchestration features
        features: Vec<OrchestrationFeature>,
    },
    /// Collaboration capabilities (template storage, lineage tracking, community features)
    Collaboration {
        /// Collaboration functions
        functions: Vec<CollaborationFunction>,
    },
}

/// Compute abilities that any compute provider can offer
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
pub enum ComputeAbility {
    /// Machine learning inference
    MachineLearning,
    /// Data analysis and processing
    DataAnalysis,
    /// Mathematical computations
    MathematicalComputation,
    /// Pattern recognition
    PatternRecognition,
    /// Natural language processing
    NaturalLanguageProcessing,
    /// Image processing
    ImageProcessing,
    /// Custom computation with specific requirements
    Custom {
        /// Custom computation requirements specification
        requirements: HashMap<String, String>,
    },
}

/// Storage characteristics
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq, Hash)]
pub enum StorageCharacteristic {
    /// Persistent data storage
    Persistent,
    /// Temporary/cache storage
    Temporary,
    /// Encrypted storage
    Encrypted,
    /// Distributed storage
    Distributed,
    HighPerformance,
    /// Archival storage
    Archival,
}

/// Network functions
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq, Hash)]
pub enum NetworkFunction {
    /// Service mesh connectivity
    ServiceMesh,
    /// Load balancing
    LoadBalancing,
    /// Service discovery
    ServiceDiscovery,
    /// Traffic routing
    TrafficRouting,
    /// Network security
    NetworkSecurity,
    /// Protocol translation
    ProtocolTranslation,
}

/// Security services
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq, Hash)]
pub enum SecurityService {
    /// Encryption/decryption
    Encryption,
    /// Authentication
    Authentication,
    /// Authorization
    Authorization,
    /// Key management
    KeyManagement,
    /// Certificate management
    CertificateManagement,
    /// Threat detection
    ThreatDetection,
}

/// Orchestration features
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq, Hash)]
pub enum OrchestrationFeature {
    /// Container management
    ContainerManagement,
    /// Service deployment
    ServiceDeployment,
    /// Resource scaling
    ResourceScaling,
    /// Health monitoring
    HealthMonitoring,
    /// Configuration management
    ConfigurationManagement,
    /// Workflow orchestration
    WorkflowOrchestration,
}

/// Collaboration functions for template storage, user management, and community features
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq, Hash)]
pub enum CollaborationFunction {
    /// Template storage and retrieval
    TemplateStorage,
    /// User authentication and identity
    UserAuthentication,
    /// Template lineage and version tracking
    LineageTracking,
    /// Community metrics and usage statistics
    CommunityMetrics,
    /// Security assessment and vulnerability scanning
    SecurityAssessment,
    /// Collaborator and permission management
    PermissionManagement,
    /// Template rating and feedback
    RatingSystem,
    /// Template search and discovery
    TemplateDiscovery,
}

/// Discovery request that doesn't specify vendors or primals
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct UniversalDiscoveryRequest {
    /// Collection of required capabilities
    pub required_capabilities: Vec<UniversalCapabilityType>,
    /// Optional capabilities that would be nice to have
    /// Collection of optional capabilities
    pub optional_capabilities: Vec<UniversalCapabilityType>,
    pub performance_requirements: PerformanceRequirements,
    /// Security requirements
    /// The security requirements value
    pub security_requirements: SecurityRequirements,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PerformanceRequirements {
    /// Maximum acceptable latency (ms)
    /// Optional max latency ms
    pub max_latency_ms: Option<u64>,
    /// Minimum required throughput (ops/sec)
    /// Optional min throughput ops per sec
    pub min_throughput_ops_per_sec: Option<f64>,
    /// Required availability (0.0 to 1.0)
    /// Optional required availability
    pub required_availability: Option<f64>,
}

impl Default for PerformanceRequirements {
    fn default() -> Self {
        Self {
            max_latency_ms: Some(5000),
            min_throughput_ops_per_sec: Some(10.0),
            required_availability: Some(0.95),
        }
    }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SecurityRequirements {
    /// Require encrypted communication
    /// Whether `require_encryption` is enabled
    pub require_encryption: bool,
    /// Require mutual authentication
    /// Whether `require_mutual_auth` is enabled
    pub require_mutual_auth: bool,
    /// Required security level
    /// The min security level value
    pub min_security_level: SecurityLevel,
}

impl Default for SecurityRequirements {
    fn default() -> Self {
        Self {
            require_encryption: true,
            require_mutual_auth: true,
            min_security_level: SecurityLevel::High,
        }
    }
}

/// Security level classification
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq, PartialOrd, Ord)]
pub enum SecurityLevel {
    /// Basic security
    Basic,
    /// Standard security
    Standard,
    /// High security
    High,
    /// Critical security (HSM-backed)
    Critical,
}

/// Universal service descriptor - no vendor/primal names
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct UniversalServiceDescriptor {
    /// Service identifier (generated, not hardcoded)
    pub service_id: String,
    /// Capabilities this service provides
    /// Collection of capabilities
    pub capabilities: Vec<UniversalCapabilityType>,
    /// Communication endpoint (discovered dynamically)
    /// The endpoint value
    pub endpoint: ServiceEndpoint,
    /// Authentication method
    /// The auth method value
    pub auth_method: AuthenticationMethod,
    pub performance_profile: PerformanceProfile,
    /// Trust score (earned through interaction)
    /// The trust score value
    pub trust_score: f64,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ServiceEndpoint {
    /// Protocol (http, grpc, websocket, etc.)
    /// The protocol value
    pub protocol: String,
    /// Host (discovered, not hardcoded)
    /// The host value
    pub host: String,
    /// Port (discovered)
    /// Number of port
    pub port: u16,
    /// Path or service name
    /// Optional path
    pub path: Option<String>,
    /// Additional connection parameters
    /// Mapping of parameters
    pub parameters: HashMap<String, String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum AuthenticationMethod {
    /// No authentication required
    None,
    /// API key authentication
    ApiKey {
        /// Location where the API key is stored or retrieved
        key_location: String,
    },
    /// JWT token authentication
    JwtToken { token_source: String },
    /// Mutual TLS authentication
    MutualTls {
        /// Path to certificate file
        cert_path: String,
        /// Path to private key file
        key_path: String,
    },
    /// Custom authentication
    Custom {
        /// Authentication method name
        method: String,
        /// Authentication configuration parameters
        config: HashMap<String, String>,
    },
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PerformanceProfile {
    /// Average response time (ms)
    pub avg_response_time_ms: f64,
    /// 95th percentile response time (ms)
    pub p95_response_time_ms: f64,
    /// Success rate (0.0 to 1.0)
    /// The success rate value
    pub success_rate: f64,
    /// Throughput (operations per second)
    /// The throughput ops per sec value
    pub throughput_ops_per_sec: f64,
    /// Availability (0.0 to 1.0)
    /// The availability value
    pub availability: f64,
}

impl Default for PerformanceProfile {
    fn default() -> Self {
        Self {
            avg_response_time_ms: 0.0,
            p95_response_time_ms: 0.0,
            success_rate: 1.0,
            throughput_ops_per_sec: 0.0,
            availability: 1.0,
        }
    }
}

/// Discovery response with found services
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct UniversalDiscoveryResponse {
    /// Services that match the requirements
    /// Collection of matching services
    pub matching_services: Vec<UniversalServiceDescriptor>,
    /// Services that partially match
    /// Collection of partial matches
    pub partial_matches: Vec<UniversalServiceDescriptor>,
    /// Discovery metadata
    /// The metadata value
    pub metadata: DiscoveryMetadata,
}

/// Metadata about the discovery process
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct DiscoveryMetadata {
    pub discovery_timestamp: u64,
    /// How long discovery took (ms)
    /// Number of `discovery_duration_ms`
    pub discovery_duration_ms: u64,
    /// Number of services discovered
    /// Number of `services_discovered`
    pub services_discovered: usize,
    /// Discovery strategy used
    /// The strategy used value
    pub strategy_used: String,
    /// Any warnings or issues
    /// Collection of warnings
    pub warnings: Vec<String>,
}
