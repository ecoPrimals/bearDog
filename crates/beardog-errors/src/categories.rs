use serde::{Deserialize, Serialize};

/// Categories of security errors for detailed classification and handling
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq, Default)]
pub enum SecurityErrorCategory {
    /// Authentication failure (login, credentials, identity verification)
    Authentication,

    /// Authorization failure (permissions, access control, role validation)
    Authorization,

    /// Encryption/decryption operation failure
    Encryption,

    /// Digital certificate validation or management failure
    Certificate,

    /// Access control policy or permission enforcement failure
    AccessControl,

    /// Security audit logging or compliance failure
    Audit,

    /// Token generation, validation, or expiration failure
    Token,

    /// Cryptographic key management operation failure
    KeyManagement,

    /// Hardware Security Module operation failure
    Hsm,

    /// Security policy validation or enforcement failure
    Policy,

    /// Session management or validation failure
    Session,

    /// Multi-factor authentication failure
    MultiFactorAuth,

    /// Biometric authentication or validation failure
    Biometric,

    /// Security configuration validation failure
    Configuration,

    #[default]
    /// General security error not covered by specific categories
    General,
}

/// Categories of system-level errors for detailed classification and handling
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq, Default)]
pub enum SystemErrorCategory {
    /// General system error
    #[default]
    /// Represents general variant
    General,

    /// Internal system error
    Internal,

    /// File system operation failure (read, write, permissions)
    FileSystem,

    /// System resource exhaustion or allocation failure
    Resource,

    /// Memory allocation, management, or corruption failure
    Memory,

    /// CPU processing or computation failure
    Processing,

    /// Data storage operation failure (database, disk)
    Storage,

    /// Operating system level failure
    OS,

    /// Process management or execution failure
    Process,

    /// Environment variable or configuration failure
    Environment,

    /// Hardware component failure or detection
    Hardware,

    /// System service operation failure
    Service,

    /// System initialization or startup failure
    Initialization,

    /// System shutdown or cleanup failure
    Shutdown,

    /// System performance degradation or optimization failure
    Performance,
}

/// Categories of business logic errors for detailed classification and handling
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq, Default)]
pub enum BusinessErrorCategory {
    /// Business rule validation failure
    Validation,

    /// Business rule processing or enforcement failure
    Rule,

    /// Workflow execution or state transition failure
    Workflow,

    /// Business process execution failure
    Processing,

    /// Mathematical calculation or computation failure
    Calculation,

    /// Application state management failure
    State,

    /// Transaction processing or rollback failure
    Transaction,

    /// Business configuration validation failure
    Configuration,

    /// Approval process or authorization failure
    Approval,

    /// Notification delivery or processing failure
    Notification,

    /// Report generation or data export failure
    Reporting,

    /// External system integration failure
    Integration,

    #[default]
    /// General business error not covered by specific categories
    General,
}

/// Categories of network-related errors for detailed classification and handling
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq, Default)]
pub enum NetworkErrorCategory {
    /// Network connection establishment or maintenance failure
    Connection,

    /// Operation timeout or response delay failure
    Timeout,

    /// DNS resolution or lookup failure
    Dns,

    /// SSL/TLS certificate or handshake failure
    Ssl,

    /// HTTP protocol or status code failure
    Http,

    /// Proxy server or intermediary failure
    Proxy,

    /// Firewall or security filtering failure
    Firewall,

    /// Network bandwidth or capacity limitation
    Bandwidth,

    /// Load balancing or traffic distribution failure
    LoadBalancing,

    /// Network protocol parsing or validation failure
    Protocol,

    /// Service discovery or registry failure
    ServiceDiscovery,

    /// Network configuration or routing failure
    Configuration,

    #[default]
    /// General network error not covered by specific categories
    General,
}

/// Categories of configuration-related errors for detailed classification and handling
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq, Default)]
pub enum ConfigurationErrorCategory {
    /// Configuration file parsing or syntax failure
    Parsing,

    /// Required configuration parameter missing
    Missing,

    /// Invalid configuration value or parameter
    Invalid,

    /// Configuration format or structure error
    Format,

    /// Environment-specific configuration failure
    Environment,

    /// Default configuration value application failure
    Default,

    /// Configuration validation or consistency failure
    Validation,

    /// Configuration migration or upgrade failure
    Migration,

    /// Configuration backup or restore failure
    Backup,

    /// Security-related configuration failure
    Security,

    /// Configuration versioning or compatibility failure
    Versioning,

    #[default]
    /// General configuration error not covered by specific categories
    General,
}

/// Categories of Hardware Security Module errors for detailed classification and handling
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq, Default)]
pub enum HsmErrorCategory {
    /// Cryptographic key generation failure
    KeyGeneration,

    /// Key storage or retrieval failure
    KeyStorage,

    /// Hardware component or connectivity failure
    Hardware,

    /// HSM authentication or access failure
    Authentication,

    /// Cryptographic operation execution failure
    Crypto,

    /// HSM configuration or setup failure
    Configuration,

    /// HSM capacity or resource limitation
    Capacity,

    /// HSM firmware or software failure
    Firmware,

    /// HSM network connectivity failure
    Network,

    /// HSM backup or recovery failure
    Backup,

    /// HSM compliance or certification failure
    Compliance,

    #[default]
    /// General HSM error not covered by specific categories
    General,
}

/// Categories of workflow execution errors for detailed classification and handling
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq, Default)]
pub enum WorkflowErrorCategory {
    /// Workflow step execution failure
    Execution,

    /// Workflow approval or authorization failure
    Approval,

    /// Workflow state transition failure
    StateTransition,

    /// Workflow timeout or deadline failure
    Timeout,

    /// Workflow input validation failure
    Validation,

    #[default]
    /// General workflow error not covered by specific categories
    General,
}

/// Categories of API-related errors for detailed classification and handling
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq, Default)]
pub enum ApiErrorCategory {
    #[default]
    /// General API error not covered by specific categories
    General,

    /// API authentication failure
    Authentication,

    /// API authorization or permission failure
    Authorization,

    /// API input validation failure
    Validation,

    /// API resource not found failure
    NotFound,

    /// API resource conflict or duplicate failure
    Conflict,

    /// API rate limiting or throttling failure
    RateLimit,

    /// API internal server error
    Internal,

    /// API service unavailable or maintenance failure
    ServiceUnavailable,
}

/// Categories of testing framework errors for detailed classification and handling
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq, Default)]
pub enum TestingErrorCategory {
    /// Property-based testing failure
    Property,

    /// Mutation testing failure
    Mutation,

    /// Invariant validation failure
    Invariant,

    /// Test monitoring and metrics failure
    Monitoring,

    /// Code coverage analysis failure
    Coverage,

    /// Boundary testing failure
    Boundary,

    /// Quantum security testing failure
    Quantum,

    /// Attack simulation failure
    Attack,

    /// System readiness testing failure
    Readiness,

    /// Resistance testing failure
    Resistance,

    /// Simulation framework failure
    Simulation,

    /// Test validation framework failure
    Validation,

    #[default]
    /// General testing error not covered by specific categories
    General,
}
