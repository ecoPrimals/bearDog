use serde::{Deserialize, Serialize};

/// Security-related error categories for authentication, authorization, and cryptographic operations
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq, Default)]
pub enum SecurityErrorCategory {
    /// Authentication failures and credential issues
    Authentication,
    /// Authorization and permission denied errors
    Authorization,
    /// Cryptographic operation failures
    Encryption,
    /// Certificate validation and management errors
    Certificate,
    /// Access control and policy violations
    AccessControl,
    /// Security audit and compliance failures
    Audit,
    /// Token validation and management errors
    Token,
    /// Key management and rotation errors
    KeyManagement,
    /// Hardware Security Module errors
    Hsm,
    /// Security policy violations
    Policy,
    /// Session management errors
    Session,
    /// Multi-factor authentication errors
    MultiFactorAuth,
    /// Biometric authentication errors
    Biometric,
    /// Security configuration errors
    Configuration,
    /// General security errors not covered by specific categories
    #[default]
    General,
}

/// System-level error categories for infrastructure and resource management
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq, Default)]
pub enum SystemErrorCategory {
    /// File system operations and permissions
    FileSystem,
    /// System resource exhaustion and limits
    Resource,
    /// Memory allocation and management errors
    Memory,
    /// CPU and processing errors
    Processing,
    /// Storage and disk-related errors
    Storage,
    /// Operating system interface errors
    OS,
    /// Process and thread management errors
    Process,
    /// Environment variable and configuration errors
    Environment,
    /// Hardware interface and driver errors
    Hardware,
    /// Service and daemon management errors
    Service,
    /// System initialization and startup errors
    Initialization,
    /// System shutdown and cleanup errors
    Shutdown,
    /// Performance and monitoring errors
    Performance,
    /// General system errors not covered by specific categories
    #[default]
    General,
}

/// Business logic and application-level error categories
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq, Default)]
pub enum BusinessErrorCategory {
    /// Data validation and format errors
    Validation,
    /// Business rule and constraint violations
    Rule,
    /// Workflow and process execution errors
    Workflow,
    /// Data processing and transformation errors
    Processing,
    /// Business logic calculation errors
    Calculation,
    /// State management and consistency errors
    State,
    /// Transaction and atomicity errors
    Transaction,
    /// Business configuration errors
    Configuration,
    /// Approval and authorization workflow errors
    Approval,
    /// Notification and communication errors
    Notification,
    /// Reporting and analytics errors
    Reporting,
    /// Integration and external service errors
    Integration,
    /// General business logic errors
    #[default]
    General,
}

/// Network communication and connectivity error categories
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq, Default)]
pub enum NetworkErrorCategory {
    /// Network connection establishment errors
    Connection,
    /// Request timeout errors
    Timeout,
    /// DNS resolution errors
    Dns,
    /// SSL/TLS certificate and handshake errors
    Ssl,
    /// HTTP protocol errors
    Http,
    /// Proxy and gateway errors
    Proxy,
    /// Firewall and network security errors
    Firewall,
    /// Bandwidth and throughput errors
    Bandwidth,
    /// Load balancing and routing errors
    LoadBalancing,
    /// Network protocol errors
    Protocol,
    /// Service discovery errors
    ServiceDiscovery,
    /// Network configuration errors
    Configuration,
    /// General network errors
    #[default]
    General,
}

/// Configuration and setup error categories
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq, Default)]
pub enum ConfigurationErrorCategory {
    /// Configuration file parsing errors
    Parsing,
    /// Missing required configuration
    Missing,
    /// Invalid configuration values
    Invalid,
    /// Configuration format errors
    Format,
    /// Environment-specific configuration errors
    Environment,
    /// Default configuration loading errors
    Default,
    /// Configuration validation errors
    Validation,
    /// Configuration migration errors
    Migration,
    /// Configuration backup and restore errors
    Backup,
    /// Configuration security errors
    Security,
    /// Configuration versioning errors
    Versioning,
    /// General configuration errors
    #[default]
    General,
}

/// Hardware Security Module (HSM) specific error categories
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq, Default)]
pub enum HsmErrorCategory {
    /// Key generation errors
    KeyGeneration,
    /// Key storage and retrieval errors
    KeyStorage,
    /// Hardware device errors
    Hardware,
    /// HSM authentication errors
    Authentication,
    /// Cryptographic operation errors
    Crypto,
    /// HSM configuration errors
    Configuration,
    /// HSM capacity and limit errors
    Capacity,
    /// HSM firmware and software errors
    Firmware,
    /// HSM network connectivity errors
    Network,
    /// HSM backup and recovery errors
    Backup,
    /// HSM compliance and audit errors
    Compliance,
    /// General HSM errors
    #[default]
    General,
}

/// Workflow execution and management error categories
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq, Default)]
pub enum WorkflowErrorCategory {
    /// Workflow execution errors
    Execution,
    /// Workflow approval process errors
    Approval,
    /// Workflow state transition errors
    StateTransition,
    /// Workflow timeout errors
    Timeout,
    /// Workflow validation errors
    Validation,
    /// General workflow errors
    #[default]
    General,
}

/// API and service interface error categories
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq, Default)]
pub enum ApiErrorCategory {
    /// General API errors
    #[default]
    General,
    /// API authentication errors
    Authentication,
    /// API authorization errors
    Authorization,
    /// Request validation errors
    Validation,
    /// Resource not found errors
    NotFound,
    /// Resource conflict errors
    Conflict,
    /// Rate limiting errors
    RateLimit,
    /// Internal server errors
    Internal,
    /// Service unavailable errors
    ServiceUnavailable,
}
