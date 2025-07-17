//! Comprehensive error handling for BearDog
//!
//! Provides detailed error types for all BearDog operations with proper context.

use thiserror::Error;

/// Result type alias for BearDog operations
pub type BearDogResult<T> = Result<T, BearDogError>;

/// Comprehensive error types for BearDog operations
#[derive(Error, Debug)]
pub enum BearDogError {
    /// Configuration-related errors
    #[error("Configuration error: {message}")]
    Configuration {
        /// Error message describing the configuration issue
        message: String,
    },

    /// Encryption/decryption operation errors
    #[error("Encryption error in {operation}: {message}")]
    Encryption {
        /// The encryption operation that failed
        operation: String,
        /// Error message describing the encryption issue
        message: String,
    },

    /// Key management errors
    #[error("Key management error: {message}")]
    KeyManagement {
        /// Error message describing the key management issue
        message: String,
    },

    /// Hardware Security Module (HSM) errors
    #[error("HSM error: {message}")]
    Hsm {
        /// Error message describing the HSM issue
        message: String,
    },

    /// Authentication errors
    #[error("Authentication error: {message}")]
    Authentication {
        /// Error message describing the authentication issue
        message: String,
    },

    /// Authorization errors
    #[error("Authorization error: {message}")]
    Authorization {
        /// Error message describing the authorization issue
        message: String,
    },

    /// Threat detection errors
    #[error("Threat detection error: {message}")]
    ThreatDetection {
        /// Error message describing the threat detection issue
        message: String,
    },

    /// Compliance-related errors
    #[error("Compliance error for {standard}: {message}")]
    Compliance {
        /// The compliance standard that failed
        standard: String,
        /// Error message describing the compliance issue
        message: String,
    },

    /// Audit-related errors
    #[error("Audit error: {message}")]
    Audit {
        /// Error message describing the audit issue
        message: String,
    },

    /// Workflow-related errors
    #[error("Workflow error: {message}")]
    Workflow {
        /// Error message describing the workflow issue
        message: String,
    },

    /// Network-related errors
    #[error("Network error: {message}")]
    Network {
        /// Error message describing the network issue
        message: String,
    },

    /// Storage-related errors
    #[error("Storage error: {message}")]
    Storage {
        /// Error message describing the storage issue
        message: String,
    },

    /// Parsing errors
    #[error("Parse error: {message}")]
    Parse {
        /// Error message describing the parsing issue
        message: String,
    },

    /// Validation errors
    #[error("Validation error: {message}")]
    Validation {
        /// Error message describing the validation issue
        message: String,
    },

    /// Rate limiting errors
    #[error("Rate limit exceeded: {message}")]
    RateLimit {
        /// Error message describing the rate limit issue
        message: String,
    },

    /// Permission errors
    #[error("Permission denied: {message}")]
    Permission {
        /// Error message describing the permission issue
        message: String,
    },

    /// Resource not found errors
    #[error("Resource not found: {message}")]
    NotFound {
        /// Error message describing what was not found
        message: String,
    },

    /// Resource already exists errors
    #[error("Resource already exists: {message}")]
    AlreadyExists {
        /// Error message describing what already exists
        message: String,
    },

    /// Resource conflict errors
    #[error("Resource conflict: {message}")]
    Conflict {
        /// Error message describing the conflict
        message: String,
    },

    /// Resource exhaustion errors
    #[error("Resource exhaustion: {message}")]
    ResourceExhaustion {
        /// Error message describing the resource exhaustion
        message: String,
    },

    /// Internal system errors
    #[error("Internal error: {message}")]
    Internal {
        /// Error message describing the internal issue
        message: String,
    },

    /// External system errors
    #[error("External system error: {message}")]
    External {
        /// Error message describing the external system issue
        message: String,
    },

    /// Timeout errors
    #[error("Timeout error: {message}")]
    Timeout {
        /// Error message describing the timeout
        message: String,
    },

    /// Cancellation errors
    #[error("Operation cancelled: {message}")]
    Cancelled {
        /// Error message describing the cancellation
        message: String,
    },

    /// Unavailable errors
    #[error("Service unavailable: {message}")]
    Unavailable {
        /// Error message describing the unavailability
        message: String,
    },

    /// Unimplemented errors
    #[error("Not implemented: {message}")]
    Unimplemented {
        /// Error message describing what's not implemented
        message: String,
    },

    /// Unknown errors
    #[error("Unknown error: {message}")]
    Unknown {
        /// Error message describing the unknown issue
        message: String,
    },

    /// Initialization errors
    #[error("Initialization error: {message}")]
    Initialization {
        /// Error message describing the initialization issue
        message: String,
    },

    /// Shutdown errors
    #[error("Shutdown error: {message}")]
    Shutdown {
        /// Error message describing the shutdown issue
        message: String,
    },

    /// Serialization errors
    #[error("Serialization error: {message}")]
    Serialization {
        /// Error message describing the serialization issue
        message: String,
    },

    /// Invalid input errors
    #[error("Invalid input: {message}")]
    InvalidInput {
        /// Error message describing the invalid input
        message: String,
    },

    /// Service unavailable errors
    #[error("Service '{service}' is unavailable: {message}")]
    ServiceUnavailable {
        /// The service that is unavailable
        service: String,
        /// Error message describing why the service is unavailable
        message: String,
    },

    /// Database-related errors
    #[error("Database error: {0}")]
    Database(#[from] sqlx::Error),

    /// I/O errors
    #[error("I/O error: {0}")]
    Io(#[from] std::io::Error),

    /// JSON serialization errors
    #[error("JSON error: {0}")]
    Json(#[from] serde_json::Error),

    /// TOML parsing errors
    #[error("TOML error: {0}")]
    Toml(#[from] toml::de::Error),

    /// TOML serialization errors
    #[error("TOML serialization error: {0}")]
    TomlSer(#[from] toml::ser::Error),

    /// HTTP client errors
    #[error("HTTP error: {0}")]
    Http(#[from] reqwest::Error),

    /// TLS errors
    #[error("TLS error: {0}")]
    Tls(#[from] rustls::Error),

    /// Cryptographic errors
    #[error("Cryptographic error")]
    Crypto {
        /// Error message describing the cryptographic issue
        message: String,
    },

    /// Key derivation errors
    #[error("Key derivation error")]
    KeyDerivation {
        /// Error message describing the key derivation issue
        message: String,
    },

    /// UUID parsing errors
    #[error("UUID error: {0}")]
    Uuid(#[from] uuid::Error),

    /// Time-related errors
    #[error("Time error: {0}")]
    Time(#[from] chrono::ParseError),

    /// Timeout errors from tokio
    #[error("Async timeout")]
    AsyncTimeout {
        /// Error message describing the timeout
        message: String,
    },

    // More specific errors continue...
    /// Error when attempting to create a workflow that already exists
    #[error("Workflow already exists: {0}")]
    WorkflowAlreadyExists(String),
    /// Error when a workflow is in an invalid state for the requested operation
    #[error("Workflow in invalid state: {0}")]
    WorkflowInvalidState(String),
    /// Error when workflow approval process fails
    #[error("Workflow approval failed: {0}")]
    WorkflowApprovalFailed(String),
    /// Error when workflow execution encounters a failure
    #[error("Workflow execution failed: {0}")]
    WorkflowExecutionFailed(String),
    /// Error when workflow validation fails due to invalid configuration or parameters
    #[error("Workflow validation failed: {0}")]
    WorkflowValidationFailed(String),
    /// Error when user lacks permission to perform workflow operation
    #[error("Workflow permission denied: {0}")]
    WorkflowPermissionDenied(String),

    // Network and system errors
    /// Network-related errors during communication or connectivity issues
    #[error("Network error: {0}")]
    NetworkError(String),

    // Additional missing variants
    /// Input/Output operation errors
    #[error("IO error: {0}")]
    IoError(String),
    /// Cryptographic key validation errors
    #[error("Invalid key: {0}")]
    InvalidKey(String),
    /// Error when a compliance standard is not supported by the system
    #[error("Compliance standard not supported: {0}")]
    ComplianceStandardNotSupported(String),

    /// Invalid request errors
    #[error("Invalid request: {0}")]
    InvalidRequest(String),

    /// Invalid data errors
    #[error("Invalid data: {message}")]
    InvalidData {
        /// Error message describing the invalid data issue
        message: String,
    },

    // Genetic spawning related errors
    /// Spawn operation was rejected
    #[error("Spawn rejected: {reason}")]
    SpawnRejected {
        /// Reason why the spawn was rejected
        reason: String,
    },

    /// Operation timed out
    #[error("Operation timeout: {operation}")]
    OperationTimeout {
        /// The operation that timed out
        operation: String,
    },

    /// System is in an unexpected state
    #[error("Unexpected state: {message}")]
    UnexpectedState {
        /// Description of the unexpected state
        message: String,
    },

    /// Resource cleanup failed
    #[error("Resource cleanup failed: {resource}")]
    ResourceCleanupFailed {
        /// The resource that failed to cleanup
        resource: String,
    },

    /// Error when genetic lineage integrity is compromised or violated
    #[error("Lineage integrity violation: {message}")]
    LineageIntegrityViolation {
        /// Description of the lineage integrity issue
        message: String,
    },

    /// Error when audit trail is incomplete or missing required entries
    #[error("Audit trail incomplete: {message}")]
    AuditTrailIncomplete {
        /// Description of what's missing from the audit trail
        message: String,
    },

    /// Error when audit integrity is violated or compromised
    #[error("Audit integrity violation: {message}")]
    AuditIntegrityViolation {
        /// Description of the audit integrity issue
        message: String,
    },

    /// Error when genetic data is invalid or corrupted
    #[error("Invalid genetics: {message}")]
    InvalidGenetics {
        /// Description of what's wrong with the genetics
        message: String,
    },

    // BSTP (BearDog Secure Tunnel Protocol) related errors
    /// Error when a secure tunnel session cannot be found
    #[error("Session not found")]
    SessionNotFound,

    /// Error when a peer node is not trusted for secure operations
    #[error("Peer not trusted")]
    PeerNotTrusted,

    /// Error when hardware acceleration is required but not available
    #[error("Hardware acceleration not available")]
    HardwareNotAvailable,

    // Cross-node authentication errors
    /// Verification process failed
    #[error("Verification failed: {message}")]
    VerificationFailed {
        /// Description of what verification failed
        message: String,
    },

    /// Node could not be found
    #[error("Node not found: {node_id}")]
    NodeNotFound {
        /// The node ID that was not found
        node_id: String,
    },

    /// Invalid state errors
    #[error("Invalid state: {message}")]
    InvalidState {
        /// Error message describing the invalid state
        message: String,
    },

    /// Unauthorized access errors
    #[error("Unauthorized: {message}")]
    Unauthorized {
        /// Error message describing the unauthorized access
        message: String,
    },

    /// Security violation errors
    #[error("Security violation: {0}")]
    SecurityViolation(String),

    /// Validation errors
    #[error("Validation error: {0}")]
    ValidationError(String),

    /// Not implemented errors
    #[error("Not implemented: {message}")]
    NotImplemented {
        /// Error message describing what's not implemented
        message: String,
    },

    // Human entropy collection errors
    /// Error when user consent for entropy collection has expired
    #[error("Consent expired")]
    ConsentExpired,

    /// Error when user consent is insufficient for the requested entropy collection operation
    #[error("Insufficient consent for requested collection")]
    InsufficientConsent,

    /// Error when entropy quality doesn't meet minimum requirements
    #[error("Insufficient entropy quality: required {required}, actual {actual}")]
    InsufficientEntropyQuality {
        /// Required quality score
        required: f64,
        /// Actual quality score
        actual: f64,
    },

    /// Error when entropy collection sampling rate is too high
    #[error("High sampling rate detected: {rate} samples/sec")]
    HighSamplingRate {
        /// Sampling rate in samples per second
        rate: f64,
    },

    /// Error when entropy source is not available
    #[error("Entropy source not available: {source_name}")]
    EntropySourceNotAvailable {
        /// The entropy source that is not available
        source_name: String,
    },

    /// Error when entropy collection violates privacy constraints
    #[error("Privacy violation in entropy collection: {violation}")]
    PrivacyViolation {
        /// Description of the privacy violation
        violation: String,
    },

    /// Error when entropy collection device is compromised
    #[error("Entropy collection device compromised: {device}")]
    DeviceCompromised {
        /// The device that is compromised
        device: String,
    },

    /// Error when entropy collection context is invalid
    #[error("Invalid entropy collection context: {context}")]
    InvalidContext {
        /// Description of the invalid context
        context: String,
    },

    /// Error when entropy collection has insufficient randomness
    #[error("Insufficient randomness in entropy collection")]
    InsufficientRandomness,

    /// Error when entropy collection has bias detected
    #[error("Bias detected in entropy collection: {bias}")]
    BiasDetected {
        /// Description of the bias detected
        bias: String,
    },

    /// Error when entropy collection has quality degradation
    #[error("Entropy quality degradation: {reason}")]
    QualityDegradation {
        /// Reason for quality degradation
        reason: String,
    },

    /// Error when entropy collection has temporal correlation
    #[error("Temporal correlation detected in entropy collection")]
    TemporalCorrelation,

    /// Error when entropy collection has spatial correlation
    #[error("Spatial correlation detected in entropy collection")]
    SpatialCorrelation,

    /// Error when entropy collection has been tampered with
    #[error("Entropy collection tampering detected")]
    TamperingDetected,

    /// Error when entropy collection has calibration issues
    #[error("Entropy collection calibration error: {error}")]
    CalibrationError {
        /// Description of the calibration error
        error: String,
    },

    /// Error when entropy collection has environmental interference
    #[error("Environmental interference in entropy collection: {interference}")]
    EnvironmentalInterference {
        /// Description of the environmental interference
        interference: String,
    },

    /// Error when entropy collection has hardware malfunction
    #[error("Hardware malfunction in entropy collection: {malfunction}")]
    HardwareMalfunction {
        /// Description of the hardware malfunction
        malfunction: String,
    },

    /// Error when entropy collection has software error
    #[error("Software error in entropy collection: {error}")]
    SoftwareError {
        /// Description of the software error
        error: String,
    },

    /// Error when entropy collection has network issues
    #[error("Network issues in entropy collection: {issue}")]
    NetworkIssues {
        /// Description of the network issue
        issue: String,
    },

    /// Error when entropy collection has synchronization issues
    #[error("Synchronization issues in entropy collection: {issue}")]
    SynchronizationIssues {
        /// Description of the synchronization issue
        issue: String,
    },

    /// Error when entropy collection has protocol violations
    #[error("Protocol violation in entropy collection: {violation}")]
    ProtocolViolation {
        /// Description of the protocol violation
        violation: String,
    },

    /// Error when entropy collection has compliance issues
    #[error("Compliance issues in entropy collection: {issue}")]
    ComplianceIssues {
        /// Description of the compliance issue
        issue: String,
    },

    /// Error when entropy collection has security issues
    #[error("Security issues in entropy collection: {issue}")]
    SecurityIssues {
        /// Description of the security issue
        issue: String,
    },

    /// Error when entropy collection has authentication issues
    #[error("Authentication issues in entropy collection: {issue}")]
    AuthenticationIssues {
        /// Description of the authentication issue
        issue: String,
    },

    /// Error when entropy collection has authorization issues
    #[error("Authorization issues in entropy collection: {issue}")]
    AuthorizationIssues {
        /// Description of the authorization issue
        issue: String,
    },

    /// Error when entropy collection has resource limitations
    #[error("Resource limitations in entropy collection: {limitation}")]
    ResourceLimitations {
        /// Description of the resource limitation
        limitation: String,
    },

    /// Error when entropy collection has configuration issues
    #[error("Configuration issues in entropy collection: {issue}")]
    ConfigurationIssues {
        /// Description of the configuration issue
        issue: String,
    },

    /// Error when entropy collection has monitoring issues
    #[error("Monitoring issues in entropy collection: {issue}")]
    MonitoringIssues {
        /// Description of the monitoring issue
        issue: String,
    },

    /// Error when entropy collection has logging issues
    #[error("Logging issues in entropy collection: {issue}")]
    LoggingIssues {
        /// Description of the logging issue
        issue: String,
    },

    /// Error when entropy collection has backup issues
    #[error("Backup issues in entropy collection: {issue}")]
    BackupIssues {
        /// Description of the backup issue
        issue: String,
    },

    /// Error when entropy collection has recovery issues
    #[error("Recovery issues in entropy collection: {issue}")]
    RecoveryIssues {
        /// Description of the recovery issue
        issue: String,
    },

    /// Error when entropy collection has maintenance issues
    #[error("Maintenance issues in entropy collection: {issue}")]
    MaintenanceIssues {
        /// Description of the maintenance issue
        issue: String,
    },

    /// Error when entropy collection has upgrade issues
    #[error("Upgrade issues in entropy collection: {issue}")]
    UpgradeIssues {
        /// Description of the upgrade issue
        issue: String,
    },

    /// Error when entropy collection has rollback issues
    #[error("Rollback issues in entropy collection: {issue}")]
    RollbackIssues {
        /// Description of the rollback issue
        issue: String,
    },

    /// Error when entropy collection has migration issues
    #[error("Migration issues in entropy collection: {issue}")]
    MigrationIssues {
        /// Description of the migration issue
        issue: String,
    },

    /// Error when entropy collection has scaling issues
    #[error("Scaling issues in entropy collection: {issue}")]
    ScalingIssues {
        /// Description of the scaling issue
        issue: String,
    },

    /// Error when entropy collection has performance issues
    #[error("Performance issues in entropy collection: {issue}")]
    PerformanceIssues {
        /// Description of the performance issue
        issue: String,
    },

    /// Error when entropy collection has reliability issues
    #[error("Reliability issues in entropy collection: {issue}")]
    ReliabilityIssues {
        /// Description of the reliability issue
        issue: String,
    },

    /// Error when entropy collection has availability issues
    #[error("Availability issues in entropy collection: {issue}")]
    AvailabilityIssues {
        /// Description of the availability issue
        issue: String,
    },

    /// Error when entropy collection has durability issues
    #[error("Durability issues in entropy collection: {issue}")]
    DurabilityIssues {
        /// Description of the durability issue
        issue: String,
    },

    /// Error when entropy collection has consistency issues
    #[error("Consistency issues in entropy collection: {issue}")]
    ConsistencyIssues {
        /// Description of the consistency issue
        issue: String,
    },

    /// Error when entropy collection has integrity issues
    #[error("Integrity issues in entropy collection: {issue}")]
    IntegrityIssues {
        /// Description of the integrity issue
        issue: String,
    },

    /// Error when entropy collection has confidentiality issues
    #[error("Confidentiality issues in entropy collection: {issue}")]
    ConfidentialityIssues {
        /// Description of the confidentiality issue
        issue: String,
    },

    /// Error when entropy collection has auditability issues
    #[error("Auditability issues in entropy collection: {issue}")]
    AuditabilityIssues {
        /// Description of the auditability issue
        issue: String,
    },

    /// Error when entropy collection has traceability issues
    #[error("Traceability issues in entropy collection: {issue}")]
    TraceabilityIssues {
        /// Description of the traceability issue
        issue: String,
    },

    /// Error when entropy collection has accountability issues
    #[error("Accountability issues in entropy collection: {issue}")]
    AccountabilityIssues {
        /// Description of the accountability issue
        issue: String,
    },

    /// Error when entropy collection has non-repudiation issues
    #[error("Non-repudiation issues in entropy collection: {issue}")]
    NonRepudiationIssues {
        /// Description of the non-repudiation issue
        issue: String,
    },

    /// Error when entropy collection has forensics issues
    #[error("Forensics issues in entropy collection: {issue}")]
    ForensicsIssues {
        /// Description of the forensics issue
        issue: String,
    },

    /// Error when an operation is not supported
    #[error("Unsupported operation: {operation}")]
    UnsupportedOperation {
        /// The operation that is not supported
        operation: String,
    },

    /// Error when no suitable provider is found
    #[error("No suitable provider found: {message}")]
    NoSuitableProvider {
        /// Error message describing why no suitable provider was found
        message: String,
    },
}

impl BearDogError {
    /// Create a new configuration error
    pub fn config(message: impl Into<String>) -> Self {
        Self::Configuration {
            message: message.into(),
        }
    }

    /// Create a new internal error
    pub fn internal(message: impl Into<String>) -> Self {
        Self::Internal {
            message: message.into(),
        }
    }

    /// Create a new validation error
    pub fn validation(message: impl Into<String>) -> Self {
        Self::Validation {
            message: message.into(),
        }
    }

    /// Create a new authentication error
    pub fn authentication(message: impl Into<String>) -> Self {
        Self::Authentication {
            message: message.into(),
        }
    }

    /// Create a new authorization error
    pub fn authorization(message: impl Into<String>) -> Self {
        Self::Authorization {
            message: message.into(),
        }
    }

    /// Create a new not found error
    pub fn not_found(message: impl Into<String>) -> Self {
        Self::NotFound {
            message: message.into(),
        }
    }

    /// Create a new already exists error
    pub fn already_exists(message: impl Into<String>) -> Self {
        Self::AlreadyExists {
            message: message.into(),
        }
    }

    /// Create a new timeout error
    pub fn timeout(message: impl Into<String>) -> Self {
        Self::Timeout {
            message: message.into(),
        }
    }

    /// Create a new service unavailable error
    pub fn service_unavailable(service: impl Into<String>, message: impl Into<String>) -> Self {
        Self::ServiceUnavailable {
            service: service.into(),
            message: message.into(),
        }
    }

    /// Create a new not implemented error
    pub fn not_implemented(message: impl Into<String>) -> Self {
        Self::Unimplemented {
            message: message.into(),
        }
    }
}

// External crate error conversions
impl From<argon2::Error> for BearDogError {
    fn from(err: argon2::Error) -> Self {
        BearDogError::Crypto {
            message: err.to_string(),
        }
    }
}

// TODO: Add these when the crates are available
// impl From<crate::adapters::nestgate::NestGateError> for BearDogError {
//     fn from(err: crate::adapters::nestgate::NestGateError) -> Self {
//         BearDogError::External {
//             message: err.to_string(),
//         }
//     }
// }

// impl From<crate::adapters::songbird::SongbirdError> for BearDogError {
//     fn from(err: crate::adapters::songbird::SongbirdError) -> Self {
//         BearDogError::External {
//             message: err.to_string(),
//         }
//     }
// }

// impl From<crate::adapters::universal::UniversalError> for BearDogError {
//     fn from(err: crate::adapters::universal::UniversalError) -> Self {
//         BearDogError::External {
//             message: err.to_string(),
//         }
//     }
// }

// impl From<crate::genetics::spawning::SpawningError> for BearDogError {
//     fn from(err: crate::genetics::spawning::SpawningError) -> Self {
//         BearDogError::Internal {
//             message: err.to_string(),
//         }
//     }
// }

// impl From<crate::tunnel::hsm::HsmError> for BearDogError {
//     fn from(err: crate::tunnel::hsm::HsmError) -> Self {
//         BearDogError::Hsm {
//             message: err.to_string(),
//         }
//     }
// }

// impl From<crate::tunnel::security_provider::SecurityProviderError> for BearDogError {
//     fn from(err: crate::tunnel::security_provider::SecurityProviderError) -> Self {
//         BearDogError::Internal {
//             message: err.to_string(),
//         }
//     }
// }

// impl From<crate::threat::types::ThreatError> for BearDogError {
//     fn from(err: crate::threat::types::ThreatError) -> Self {
//         BearDogError::ThreatDetection {
//             message: err.to_string(),
//         }
//     }
// }

// impl From<crate::workflows::types::WorkflowError> for BearDogError {
//     fn from(err: crate::workflows::types::WorkflowError) -> Self {
//         BearDogError::Workflow {
//             message: err.to_string(),
//         }
//     }
// }

// impl From<crate::node_registry::types::NodeRegistryError> for BearDogError {
//     fn from(err: crate::node_registry::types::NodeRegistryError) -> Self {
//         BearDogError::Internal {
//             message: err.to_string(),
//         }
//     }
// }

// impl From<crate::auth::types::AuthError> for BearDogError {
//     fn from(err: crate::auth::types::AuthError) -> Self {
//         BearDogError::Authentication {
//             message: err.to_string(),
//         }
//     }
// }

// impl From<crate::compliance::types::ComplianceError> for BearDogError {
//     fn from(err: crate::compliance::types::ComplianceError) -> Self {
//         BearDogError::Compliance {
//             standard: err.standard().unwrap_or_default(),
//             message: err.to_string(),
//         }
//     }
// }

// impl From<crate::api::types::ApiError> for BearDogError {
//     fn from(err: crate::api::types::ApiError) -> Self {
//         BearDogError::Internal {
//             message: err.to_string(),
//         }
//     }
// }

// impl From<crate::monitoring::types::MonitoringError> for BearDogError {
//     fn from(err: crate::monitoring::types::MonitoringError) -> Self {
//         BearDogError::Internal {
//             message: err.to_string(),
//         }
//     }
// }

// impl From<crate::production::types::ProductionError> for BearDogError {
//     fn from(err: crate::production::types::ProductionError) -> Self {
//         BearDogError::Internal {
//             message: err.to_string(),
//         }
//     }
// }

// impl From<crate::genetics::entropy_hierarchy::EntropyError> for BearDogError {
//     fn from(err: crate::genetics::entropy_hierarchy::EntropyError) -> Self {
//         BearDogError::Internal {
//             message: err.to_string(),
//         }
//     }
// }

// impl From<crate::genetics::human_entropy::HumanEntropyError> for BearDogError {
//     fn from(err: crate::genetics::human_entropy::HumanEntropyError) -> Self {
//         BearDogError::Internal {
//             message: err.to_string(),
//         }
//     }
// }

// impl From<crate::tunnel::hsm::android_strongbox::AndroidStrongboxError> for BearDogError {
//     fn from(err: crate::tunnel::hsm::android_strongbox::AndroidStrongboxError) -> Self {
//         BearDogError::Hsm {
//             message: err.to_string(),
//         }
//     }
// }

// impl From<crate::tunnel::hsm::software_hsm::SoftwareHsmError> for BearDogError {
//     fn from(err: crate::tunnel::hsm::software_hsm::SoftwareHsmError) -> Self {
//         BearDogError::Hsm {
//             message: err.to_string(),
//         }
//     }
// }

// impl From<crate::tunnel::events::EventError> for BearDogError {
//     fn from(err: crate::tunnel::events::EventError) -> Self {
//         BearDogError::Internal {
//             message: err.to_string(),
//         }
//     }
// }

// impl From<crate::licensing::LicensingError> for BearDogError {
//     fn from(err: crate::licensing::LicensingError) -> Self {
//         BearDogError::Internal {
//             message: err.to_string(),
//         }
//     }
// }

// impl From<crate::utils::UtilsError> for BearDogError {
//     fn from(err: crate::utils::UtilsError) -> Self {
//         BearDogError::Internal {
//             message: err.to_string(),
//         }
//     }
// }

// impl From<crate::ecosystem_integration::EcosystemError> for BearDogError {
//     fn from(err: crate::ecosystem_integration::EcosystemError) -> Self {
//         BearDogError::External {
//             message: err.to_string(),
//         }
//     }
// }

// impl From<crate::adapters::nestgate::types::NestGateError> for BearDogError {
//     fn from(err: crate::adapters::nestgate::types::NestGateError) -> Self {
//         BearDogError::External {
//             message: err.to_string(),
//         }
//     }
// }

// impl From<crate::adapters::songbird::types::SongbirdError> for BearDogError {
//     fn from(err: crate::adapters::songbird::types::SongbirdError) -> Self {
//         BearDogError::External {
//             message: err.to_string(),
//         }
//     }
// }

// impl From<crate::adapters::universal::types::UniversalError> for BearDogError {
//     fn from(err: crate::adapters::universal::types::UniversalError) -> Self {
//         BearDogError::External {
//             message: err.to_string(),
//         }
//     }
// }

// impl From<crate::security::types::SecurityError> for BearDogError {
//     fn from(err: crate::security::types::SecurityError) -> Self {
//         BearDogError::Internal {
//             message: err.to_string(),
//         }
//     }
// }

// impl From<crate::verification::types::VerificationError> for BearDogError {
//     fn from(err: crate::verification::types::VerificationError) -> Self {
//         BearDogError::Internal {
//             message: err.to_string(),
//         }
//     }
// }

// impl From<crate::config::types::ConfigError> for BearDogError {
//     fn from(err: crate::config::types::ConfigError) -> Self {
//         BearDogError::Configuration {
//             message: err.to_string(),
//         }
//     }
// }

// impl From<crate::crypto_utils::CryptoError> for BearDogError {
//     fn from(err: crate::crypto_utils::CryptoError) -> Self {
//         BearDogError::Crypto {
//             message: err.to_string(),
//         }
//     }
// }

// impl From<crate::encryption::EncryptionError> for BearDogError {
//     fn from(err: crate::encryption::EncryptionError) -> Self {
//         BearDogError::Encryption {
//             operation: err.operation().unwrap_or_default(),
//             message: err.to_string(),
//         }
//     }
// }

// impl From<crate::audit::AuditError> for BearDogError {
//     fn from(err: crate::audit::AuditError) -> Self {
//         BearDogError::Audit {
//             message: err.to_string(),
//         }
//     }
// }

// impl From<ring::error::Unspecified> for BearDogError {
//     fn from(_: ring::error::Unspecified) -> Self {
//         BearDogError::Crypto {
//             message: "Ring cryptography error".to_string(),
//         }
//     }
// }

// impl From<ring::error::KeyRejected> for BearDogError {
//     fn from(err: ring::error::KeyRejected) -> Self {
//         BearDogError::InvalidKey(err.to_string())
//     }
// }

// impl From<base64::DecodeError> for BearDogError {
//     fn from(err: base64::DecodeError) -> Self {
//         BearDogError::Parse {
//             message: format!("Base64 decode error: {}", err),
//         }
//     }
// }

// impl From<hex::FromHexError> for BearDogError {
//     fn from(err: hex::FromHexError) -> Self {
//         BearDogError::Parse {
//             message: format!("Hex decode error: {}", err),
//         }
//     }
// }

// impl<T> From<std::sync::PoisonError<T>> for BearDogError {
//     fn from(err: std::sync::PoisonError<T>) -> Self {
//         BearDogError::Internal {
//             message: format!("Mutex poison error: {}", err),
//         }
//     }
// }

// impl From<std::sync::mpsc::RecvError> for BearDogError {
//     fn from(err: std::sync::mpsc::RecvError) -> Self {
//         BearDogError::Internal {
//             message: format!("Channel receive error: {}", err),
//         }
//     }
// }

// impl<T> From<std::sync::mpsc::SendError<T>> for BearDogError {
//     fn from(err: std::sync::mpsc::SendError<T>) -> Self {
//         BearDogError::Internal {
//             message: format!("Channel send error: {}", err),
//         }
//     }
// }

// impl From<std::sync::mpsc::TryRecvError> for BearDogError {
//     fn from(err: std::sync::mpsc::TryRecvError) -> Self {
//         BearDogError::Internal {
//             message: format!("Channel try receive error: {}", err),
//         }
//     }
// }

// impl<T> From<std::sync::mpsc::TrySendError<T>> for BearDogError {
//     fn from(err: std::sync::mpsc::TrySendError<T>) -> Self {
//         BearDogError::Internal {
//             message: format!("Channel try send error: {}", err),
//         }
//     }
// }

// impl From<std::sync::mpsc::RecvTimeoutError> for BearDogError {
//     fn from(err: std::sync::mpsc::RecvTimeoutError) -> Self {
//         BearDogError::Timeout {
//             message: format!("Channel receive timeout: {}", err),
//         }
//     }
// }

// impl From<std::num::ParseIntError> for BearDogError {
//     fn from(err: std::num::ParseIntError) -> Self {
//         BearDogError::Parse {
//             message: format!("Integer parse error: {}", err),
//         }
//     }
// }

// impl From<std::num::ParseFloatError> for BearDogError {
//     fn from(err: std::num::ParseFloatError) -> Self {
//         BearDogError::Parse {
//             message: format!("Float parse error: {}", err),
//         }
//     }
// }

// impl From<std::str::ParseBoolError> for BearDogError {
//     fn from(err: std::str::ParseBoolError) -> Self {
//         BearDogError::Parse {
//             message: format!("Bool parse error: {}", err),
//         }
//     }
// }

// impl From<std::str::Utf8Error> for BearDogError {
//     fn from(err: std::str::Utf8Error) -> Self {
//         BearDogError::Parse {
//             message: format!("UTF-8 error: {}", err),
//         }
//     }
// }

// impl From<std::string::FromUtf8Error> for BearDogError {
//     fn from(err: std::string::FromUtf8Error) -> Self {
//         BearDogError::Parse {
//             message: format!("UTF-8 conversion error: {}", err),
//         }
//     }
// }

// impl From<std::ffi::NulError> for BearDogError {
//     fn from(err: std::ffi::NulError) -> Self {
//         BearDogError::Internal {
//             message: format!("Null byte error: {}", err),
//         }
//     }
// }

// impl From<std::ffi::IntoStringError> for BearDogError {
//     fn from(err: std::ffi::IntoStringError) -> Self {
//         BearDogError::Internal {
//             message: format!("String conversion error: {}", err),
//         }
//     }
// }

// impl From<std::net::AddrParseError> for BearDogError {
//     fn from(err: std::net::AddrParseError) -> Self {
//         BearDogError::Parse {
//             message: format!("Address parse error: {}", err),
//         }
//     }
// }

// impl From<std::env::VarError> for BearDogError {
//     fn from(err: std::env::VarError) -> Self {
//         BearDogError::Configuration {
//             message: format!("Environment variable error: {}", err),
//         }
//     }
// }

// impl From<std::path::StripPrefixError> for BearDogError {
//     fn from(err: std::path::StripPrefixError) -> Self {
//         BearDogError::Internal {
//             message: format!("Path strip prefix error: {}", err),
//         }
//     }
// }

// impl From<std::fmt::Error> for BearDogError {
//     fn from(err: std::fmt::Error) -> Self {
//         BearDogError::Internal {
//             message: format!("Format error: {}", err),
//         }
//     }
// }

// impl From<std::collections::TryReserveError> for BearDogError {
//     fn from(err: std::collections::TryReserveError) -> Self {
//         BearDogError::ResourceExhaustion {
//             message: format!("Memory allocation error: {}", err),
//         }
//     }
// }

// impl From<std::array::TryFromSliceError> for BearDogError {
//     fn from(err: std::array::TryFromSliceError) -> Self {
//         BearDogError::Internal {
//             message: format!("Array conversion error: {}", err),
//         }
//     }
// }

// impl From<std::char::CharTryFromError> for BearDogError {
//     fn from(err: std::char::CharTryFromError) -> Self {
//         BearDogError::Internal {
//             message: format!("Character conversion error: {}", err),
//         }
//     }
// }

// impl From<std::char::DecodeUtf16Error> for BearDogError {
//     fn from(err: std::char::DecodeUtf16Error) -> Self {
//         BearDogError::Parse {
//             message: format!("UTF-16 decode error: {}", err),
//         }
//     }
// }

// impl From<std::char::ParseCharError> for BearDogError {
//     fn from(err: std::char::ParseCharError) -> Self {
//         BearDogError::Parse {
//             message: format!("Character parse error: {}", err),
//         }
//     }
// }

// impl From<std::char::TryFromCharError> for BearDogError {
//     fn from(err: std::char::TryFromCharError) -> Self {
//         BearDogError::Internal {
//             message: format!("Character try from error: {}", err),
//         }
//     }
// }

// impl From<url::ParseError> for BearDogError {
//     fn from(err: url::ParseError) -> Self {
//         BearDogError::Parse {
//             message: format!("URL parse error: {}", err),
//         }
//     }
// }

// impl From<regex::Error> for BearDogError {
//     fn from(err: regex::Error) -> Self {
//         BearDogError::Parse {
//             message: format!("Regex error: {}", err),
//         }
//     }
// }

// impl From<flate2::DecompressError> for BearDogError {
//     fn from(err: flate2::DecompressError) -> Self {
//         BearDogError::Internal {
//             message: format!("Decompression error: {}", err),
//         }
//     }
// }

// impl From<flate2::CompressError> for BearDogError {
//     fn from(err: flate2::CompressError) -> Self {
//         BearDogError::Internal {
//             message: format!("Compression error: {}", err),
//         }
//     }
// }

// impl From<zip::result::ZipError> for BearDogError {
//     fn from(err: zip::result::ZipError) -> Self {
//         BearDogError::Internal {
//             message: format!("ZIP error: {}", err),
//         }
//     }
// }

// impl From<tar::Error> for BearDogError {
//     fn from(err: tar::Error) -> Self {
//         BearDogError::Internal {
//             message: format!("TAR error: {}", err),
//         }
//     }
// }

// impl From<tokio::sync::TryLockError> for BearDogError {
//     fn from(err: tokio::sync::TryLockError) -> Self {
//         BearDogError::Internal {
//             message: format!("Async try lock error: {}", err),
//         }
//     }
// }

// impl From<tokio::sync::oneshot::error::RecvError> for BearDogError {
//     fn from(err: tokio::sync::oneshot::error::RecvError) -> Self {
//         BearDogError::Internal {
//             message: format!("Async oneshot receive error: {}", err),
//         }
//     }
// }

// impl From<tokio::sync::broadcast::error::RecvError> for BearDogError {
//     fn from(err: tokio::sync::broadcast::error::RecvError) -> Self {
//         BearDogError::Internal {
//             message: format!("Async broadcast receive error: {}", err),
//         }
//     }
// }

// impl<T> From<tokio::sync::broadcast::error::SendError<T>> for BearDogError {
//     fn from(err: tokio::sync::broadcast::error::SendError<T>) -> Self {
//         BearDogError::Internal {
//             message: format!("Async broadcast send error: {}", err),
//         }
//     }
// }

// impl From<tokio::sync::broadcast::error::TryRecvError> for BearDogError {
//     fn from(err: tokio::sync::broadcast::error::TryRecvError) -> Self {
//         BearDogError::Internal {
//             message: format!("Async broadcast try receive error: {}", err),
//         }
//     }
// }

// impl<T> From<tokio::sync::mpsc::error::SendError<T>> for BearDogError {
//     fn from(err: tokio::sync::mpsc::error::SendError<T>) -> Self {
//         BearDogError::Internal {
//             message: format!("Async mpsc send error: {}", err),
//         }
//     }
// }

// impl From<tokio::sync::mpsc::error::TryRecvError> for BearDogError {
//     fn from(err: tokio::sync::mpsc::error::TryRecvError) -> Self {
//         BearDogError::Internal {
//             message: format!("Async mpsc try receive error: {}", err),
//         }
//     }
// }

// impl<T> From<tokio::sync::mpsc::error::TrySendError<T>> for BearDogError {
//     fn from(err: tokio::sync::mpsc::error::TrySendError<T>) -> Self {
//         BearDogError::Internal {
//             message: format!("Async mpsc try send error: {}", err),
//         }
//     }
// }

// impl From<tokio::sync::watch::error::RecvError> for BearDogError {
//     fn from(err: tokio::sync::watch::error::RecvError) -> Self {
//         BearDogError::Internal {
//             message: format!("Async watch receive error: {}", err),
//         }
//     }
// }

// impl<T> From<tokio::sync::watch::error::SendError<T>> for BearDogError {
//     fn from(err: tokio::sync::watch::error::SendError<T>) -> Self {
//         BearDogError::Internal {
//             message: format!("Async watch send error: {}", err),
//         }
//     }
// }

// impl From<tokio::time::error::Elapsed> for BearDogError {
//     fn from(err: tokio::time::error::Elapsed) -> Self {
//         BearDogError::Timeout {
//             message: format!("Async timeout: {}", err),
//         }
//     }
// }

// impl From<tokio::task::JoinError> for BearDogError {
//     fn from(err: tokio::task::JoinError) -> Self {
//         BearDogError::Internal {
//             message: format!("Async join error: {}", err),
//         }
//     }
// }

// impl From<futures::channel::oneshot::Canceled> for BearDogError {
//     fn from(err: futures::channel::oneshot::Canceled) -> Self {
//         BearDogError::Cancelled {
//             message: format!("Futures oneshot canceled: {}", err),
//         }
//     }
// }

// impl<T> From<futures::channel::mpsc::SendError> for BearDogError {
//     fn from(err: futures::channel::mpsc::SendError) -> Self {
//         BearDogError::Internal {
//             message: format!("Futures mpsc send error: {}", err),
//         }
//     }
// }

// impl From<futures::channel::mpsc::TryRecvError> for BearDogError {
//     fn from(err: futures::channel::mpsc::TryRecvError) -> Self {
//         BearDogError::Internal {
//             message: format!("Futures mpsc try receive error: {}", err),
//         }
//     }
// }

// impl<T> From<futures::channel::mpsc::TrySendError<T>> for BearDogError {
//     fn from(err: futures::channel::mpsc::TrySendError<T>) -> Self {
//         BearDogError::Internal {
//             message: format!("Futures mpsc try send error: {}", err),
//         }
//     }
// }

// impl From<hyper::Error> for BearDogError {
//     fn from(err: hyper::Error) -> Self {
//         BearDogError::Network {
//             message: format!("Hyper error: {}", err),
//         }
//     }
// }

// impl From<hyper::header::InvalidHeaderName> for BearDogError {
//     fn from(err: hyper::header::InvalidHeaderName) -> Self {
//         BearDogError::InvalidInput {
//             message: format!("Invalid header name: {}", err),
//         }
//     }
// }

// impl From<hyper::header::InvalidHeaderValue> for BearDogError {
//     fn from(err: hyper::header::InvalidHeaderValue) -> Self {
//         BearDogError::InvalidInput {
//             message: format!("Invalid header value: {}", err),
//         }
//     }
// }

// impl From<hyper::header::ToStrError> for BearDogError {
//     fn from(err: hyper::header::ToStrError) -> Self {
//         BearDogError::Internal {
//             message: format!("Header to string error: {}", err),
//         }
//     }
// }

// impl From<hyper::http::uri::InvalidUri> for BearDogError {
//     fn from(err: hyper::http::uri::InvalidUri) -> Self {
//         BearDogError::InvalidInput {
//             message: format!("Invalid URI: {}", err),
//         }
//     }
// }

// impl From<hyper::http::method::InvalidMethod> for BearDogError {
//     fn from(err: hyper::http::method::InvalidMethod) -> Self {
//         BearDogError::InvalidInput {
//             message: format!("Invalid method: {}", err),
//         }
//     }
// }

// impl From<hyper::http::status::InvalidStatusCode> for BearDogError {
//     fn from(err: hyper::http::status::InvalidStatusCode) -> Self {
//         BearDogError::InvalidInput {
//             message: format!("Invalid status code: {}", err),
//         }
//     }
// }

// impl From<hyper::http::version::InvalidVersion> for BearDogError {
//     fn from(err: hyper::http::version::InvalidVersion) -> Self {
//         BearDogError::InvalidInput {
//             message: format!("Invalid version: {}", err),
//         }
//     }
// }

// impl From<tower::timeout::error::Elapsed> for BearDogError {
//     fn from(err: tower::timeout::error::Elapsed) -> Self {
//         BearDogError::Timeout {
//             message: format!("Tower timeout: {}", err),
//         }
//     }
// }

// impl From<tower::load_shed::error::Overloaded> for BearDogError {
//     fn from(err: tower::load_shed::error::Overloaded) -> Self {
//         BearDogError::ResourceExhaustion {
//             message: format!("Tower load shed overloaded: {}", err),
//         }
//     }
// }

// impl From<tower::util::BoxError> for BearDogError {
//     fn from(err: tower::util::BoxError) -> Self {
//         BearDogError::Internal {
//             message: format!("Tower box error: {}", err),
//         }
//     }
// }

// impl From<axum::Error> for BearDogError {
//     fn from(err: axum::Error) -> Self {
//         BearDogError::Internal {
//             message: format!("Axum error: {}", err),
//         }
//     }
// }

// impl From<axum::extract::rejection::JsonRejection> for BearDogError {
//     fn from(err: axum::extract::rejection::JsonRejection) -> Self {
//         BearDogError::InvalidInput {
//             message: format!("JSON rejection: {}", err),
//         }
//     }
// }

// impl From<axum::extract::rejection::QueryRejection> for BearDogError {
//     fn from(err: axum::extract::rejection::QueryRejection) -> Self {
//         BearDogError::InvalidInput {
//             message: format!("Query rejection: {}", err),
//         }
//     }
// }

// impl From<axum::extract::rejection::PathRejection> for BearDogError {
//     fn from(err: axum::extract::rejection::PathRejection) -> Self {
//         BearDogError::InvalidInput {
//             message: format!("Path rejection: {}", err),
//         }
//     }
// }

// impl From<axum::extract::rejection::FormRejection> for BearDogError {
//     fn from(err: axum::extract::rejection::FormRejection) -> Self {
//         BearDogError::InvalidInput {
//             message: format!("Form rejection: {}", err),
//         }
//     }
// }

// impl From<axum::extract::rejection::BytesRejection> for BearDogError {
//     fn from(err: axum::extract::rejection::BytesRejection) -> Self {
//         BearDogError::InvalidInput {
//             message: format!("Bytes rejection: {}", err),
//         }
//     }
// }

// impl From<axum::extract::rejection::StringRejection> for BearDogError {
//     fn from(err: axum::extract::rejection::StringRejection) -> Self {
//         BearDogError::InvalidInput {
//             message: format!("String rejection: {}", err),
//         }
//     }
// }

// impl From<axum::extract::rejection::ExtensionRejection> for BearDogError {
//     fn from(err: axum::extract::rejection::ExtensionRejection) -> Self {
//         BearDogError::Internal {
//             message: format!("Extension rejection: {}", err),
//         }
//     }
// }

// impl From<axum::extract::rejection::HeadersRejection> for BearDogError {
//     fn from(err: axum::extract::rejection::HeadersRejection) -> Self {
//         BearDogError::InvalidInput {
//             message: format!("Headers rejection: {}", err),
//         }
//     }
// }

// impl From<axum::extract::rejection::TypedHeaderRejection> for BearDogError {
//     fn from(err: axum::extract::rejection::TypedHeaderRejection) -> Self {
//         BearDogError::InvalidInput {
//             message: format!("Typed header rejection: {}", err),
//         }
//     }
// }

// impl From<axum::extract::rejection::MatchedPathRejection> for BearDogError {
//     fn from(err: axum::extract::rejection::MatchedPathRejection) -> Self {
//         BearDogError::Internal {
//             message: format!("Matched path rejection: {}", err),
//         }
//     }
// }

// impl From<axum::extract::rejection::MethodRejection> for BearDogError {
//     fn from(err: axum::extract::rejection::MethodRejection) -> Self {
//         BearDogError::InvalidInput {
//             message: format!("Method rejection: {}", err),
//         }
//     }
// }

// impl From<axum::extract::rejection::HostRejection> for BearDogError {
//     fn from(err: axum::extract::rejection::HostRejection) -> Self {
//         BearDogError::InvalidInput {
//             message: format!("Host rejection: {}", err),
//         }
//     }
// }

// impl From<axum::extract::rejection::FailedToBufferBody> for BearDogError {
//     fn from(err: axum::extract::rejection::FailedToBufferBody) -> Self {
//         BearDogError::Internal {
//             message: format!("Failed to buffer body: {}", err),
//         }
//     }
// }

// impl From<axum::extract::rejection::InvalidContentType> for BearDogError {
//     fn from(err: axum::extract::rejection::InvalidContentType) -> Self {
//         BearDogError::InvalidInput {
//             message: format!("Invalid content type: {}", err),
//         }
//     }
// }

// impl From<axum::extract::rejection::LengthLimitError> for BearDogError {
//     fn from(err: axum::extract::rejection::LengthLimitError) -> Self {
//         BearDogError::InvalidInput {
//             message: format!("Length limit error: {}", err),
//         }
//     }
// }

// impl From<axum::extract::rejection::MissingContentType> for BearDogError {
//     fn from(err: axum::extract::rejection::MissingContentType) -> Self {
//         BearDogError::InvalidInput {
//             message: format!("Missing content type: {}", err),
//         }
//     }
// }

// impl From<axum::extract::rejection::InvalidJsonBody> for BearDogError {
//     fn from(err: axum::extract::rejection::InvalidJsonBody) -> Self {
//         BearDogError::InvalidInput {
//             message: format!("Invalid JSON body: {}", err),
//         }
//     }
// }

// impl From<axum::extract::rejection::JsonDataError> for BearDogError {
//     fn from(err: axum::extract::rejection::JsonDataError) -> Self {
//         BearDogError::InvalidInput {
//             message: format!("JSON data error: {}", err),
//         }
//     }
// }

// impl From<axum::extract::rejection::JsonSyntaxError> for BearDogError {
//     fn from(err: axum::extract::rejection::JsonSyntaxError) -> Self {
//         BearDogError::InvalidInput {
//             message: format!("JSON syntax error: {}", err),
//         }
//     }
// }

// impl From<axum::extract::rejection::MissingJsonContentType> for BearDogError {
//     fn from(err: axum::extract::rejection::MissingJsonContentType) -> Self {
//         BearDogError::InvalidInput {
//             message: format!("Missing JSON content type: {}", err),
//         }
//     }
// }

// impl From<axum::extract::rejection::InvalidFormBody> for BearDogError {
//     fn from(err: axum::extract::rejection::InvalidFormBody) -> Self {
//         BearDogError::InvalidInput {
//             message: format!("Invalid form body: {}", err),
//         }
//     }
// }

// impl From<axum::extract::rejection::InvalidUtf8InFormData> for BearDogError {
//     fn from(err: axum::extract::rejection::InvalidUtf8InFormData) -> Self {
//         BearDogError::InvalidInput {
//             message: format!("Invalid UTF-8 in form data: {}", err),
//         }
//     }
// }

// impl From<axum::extract::rejection::FormDataError> for BearDogError {
//     fn from(err: axum::extract::rejection::FormDataError) -> Self {
//         BearDogError::InvalidInput {
//             message: format!("Form data error: {}", err),
//         }
//     }
// }

// impl From<axum::extract::rejection::InvalidQuery> for BearDogError {
//     fn from(err: axum::extract::rejection::InvalidQuery) -> Self {
//         BearDogError::InvalidInput {
//             message: format!("Invalid query: {}", err),
//         }
//     }
// }

// impl From<axum::extract::rejection::FailedToDeserializeQueryString> for BearDogError {
//     fn from(err: axum::extract::rejection::FailedToDeserializeQueryString) -> Self {
//         BearDogError::InvalidInput {
//             message: format!("Failed to deserialize query string: {}", err),
//         }
//     }
// }

// impl From<axum::extract::rejection::InvalidPath> for BearDogError {
//     fn from(err: axum::extract::rejection::InvalidPath) -> Self {
//         BearDogError::InvalidInput {
//             message: format!("Invalid path: {}", err),
//         }
//     }
// }

// impl From<axum::extract::rejection::FailedToDeserializePathParams> for BearDogError {
//     fn from(err: axum::extract::rejection::FailedToDeserializePathParams) -> Self {
//         BearDogError::InvalidInput {
//             message: format!("Failed to deserialize path params: {}", err),
//         }
//     }
// }

// impl From<axum::extract::rejection::MissingRequestExtension> for BearDogError {
//     fn from(err: axum::extract::rejection::MissingRequestExtension) -> Self {
//         BearDogError::Internal {
//             message: format!("Missing request extension: {}", err),
//         }
//     }
// }

// impl From<axum::extract::rejection::InvalidWebSocketVersionHeader> for BearDogError {
//     fn from(err: axum::extract::rejection::InvalidWebSocketVersionHeader) -> Self {
//         BearDogError::InvalidInput {
//             message: format!("Invalid WebSocket version header: {}", err),
//         }
//     }
// }

// impl From<axum::extract::rejection::InvalidWebSocketKeyHeader> for BearDogError {
//     fn from(err: axum::extract::rejection::InvalidWebSocketKeyHeader) -> Self {
//         BearDogError::InvalidInput {
//             message: format!("Invalid WebSocket key header: {}", err),
//         }
//     }
// }

// impl From<axum::extract::rejection::InvalidConnectionHeader> for BearDogError {
//     fn from(err: axum::extract::rejection::InvalidConnectionHeader) -> Self {
//         BearDogError::InvalidInput {
//             message: format!("Invalid connection header: {}", err),
//         }
//     }
// }

// impl From<axum::extract::rejection::InvalidUpgradeHeader> for BearDogError {
//     fn from(err: axum::extract::rejection::InvalidUpgradeHeader) -> Self {
//         BearDogError::InvalidInput {
//             message: format!("Invalid upgrade header: {}", err),
//         }
//     }
// }

// impl From<axum::extract::rejection::WebSocketKeyHeaderMissing> for BearDogError {
//     fn from(err: axum::extract::rejection::WebSocketKeyHeaderMissing) -> Self {
//         BearDogError::InvalidInput {
//             message: format!("WebSocket key header missing: {}", err),
//         }
//     }
// }

// impl From<axum::extract::rejection::ConnectionNotUpgradable> for BearDogError {
//     fn from(err: axum::extract::rejection::ConnectionNotUpgradable) -> Self {
//         BearDogError::InvalidInput {
//             message: format!("Connection not upgradable: {}", err),
//         }
//     }
// }

// impl From<redis::RedisError> for BearDogError {
//     fn from(err: redis::RedisError) -> Self {
//         BearDogError::External {
//             message: format!("Redis error: {}", err),
//         }
//     }
// }

// impl From<clap::Error> for BearDogError {
//     fn from(err: clap::Error) -> Self {
//         BearDogError::InvalidInput {
//             message: format!("CLI error: {}", err),
//         }
//     }
// }

// impl From<config::ConfigError> for BearDogError {
//     fn from(err: config::ConfigError) -> Self {
//         BearDogError::Configuration {
//             message: format!("Config error: {}", err),
//         }
//     }
// }

// impl From<tracing::subscriber::SetGlobalDefaultError> for BearDogError {
//     fn from(err: tracing::subscriber::SetGlobalDefaultError) -> Self {
//         BearDogError::Initialization {
//             message: format!("Tracing initialization error: {}", err),
//         }
//     }
// }

// impl From<tracing_appender::non_blocking::WorkerGuard> for BearDogError {
//     fn from(_: tracing_appender::non_blocking::WorkerGuard) -> Self {
//         BearDogError::Internal {
//             message: "Tracing appender worker guard error".to_string(),
//         }
//     }
// }

// impl From<tracing_subscriber::filter::ParseError> for BearDogError {
//     fn from(err: tracing_subscriber::filter::ParseError) -> Self {
//         BearDogError::Configuration {
//             message: format!("Tracing filter parse error: {}", err),
//         }
//     }
// }

// impl From<tracing_subscriber::reload::Error> for BearDogError {
//     fn from(err: tracing_subscriber::reload::Error) -> Self {
//         BearDogError::Internal {
//             message: format!("Tracing reload error: {}", err),
//         }
//     }
// }

// impl From<tracing_subscriber::filter::FromEnvError> for BearDogError {
//     fn from(err: tracing_subscriber::filter::FromEnvError) -> Self {
//         BearDogError::Configuration {
//             message: format!("Tracing filter from env error: {}", err),
//         }
//     }
// }

// impl From<tracing_subscriber::util::TryInitError> for BearDogError {
//     fn from(err: tracing_subscriber::util::TryInitError) -> Self {
//         BearDogError::Initialization {
//             message: format!("Tracing try init error: {}", err),
//         }
//     }
// }

// impl From<tracing_subscriber::layer::LayerError> for BearDogError {
//     fn from(err: tracing_subscriber::layer::LayerError) -> Self {
//         BearDogError::Internal {
//             message: format!("Tracing layer error: {}", err),
//         }
//     }
// }

// impl From<tracing_subscriber::fmt::time::SystemTimeError> for BearDogError {
//     fn from(err: tracing_subscriber::fmt::time::SystemTimeError) -> Self {
//         BearDogError::Internal {
//             message: format!("Tracing system time error: {}", err),
//         }
//     }
// }

// impl From<tracing_subscriber::fmt::writer::TestWriterError> for BearDogError {
//     fn from(err: tracing_subscriber::fmt::writer::TestWriterError) -> Self {
//         BearDogError::Internal {
//             message: format!("Tracing test writer error: {}", err),
//         }
//     }
// }

// impl From<tracing_subscriber::fmt::format::DefaultFieldError> for BearDogError {
//     fn from(err: tracing_subscriber::fmt::format::DefaultFieldError) -> Self {
//         BearDogError::Internal {
//             message: format!("Tracing default field error: {}", err),
//         }
//     }
// }

// impl From<tracing_subscriber::fmt::format::PrettyFieldError> for BearDogError {
//     fn from(err: tracing_subscriber::fmt::format::PrettyFieldError) -> Self {
//         BearDogError::Internal {
//             message: format!("Tracing pretty field error: {}", err),
//         }
//     }
// }

// impl From<tracing_subscriber::fmt::format::JsonFieldError> for BearDogError {
//     fn from(err: tracing_subscriber::fmt::format::JsonFieldError) -> Self {
//         BearDogError::Internal {
//             message: format!("Tracing JSON field error: {}", err),
//         }
//     }
// }

// impl From<tracing_subscriber::fmt::format::JsonError> for BearDogError {
//     fn from(err: tracing_subscriber::fmt::format::JsonError) -> Self {
//         BearDogError::Internal {
//             message: format!("Tracing JSON error: {}", err),
//         }
//     }
// }
