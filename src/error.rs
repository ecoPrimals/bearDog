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

    /// Audit system errors
    #[error("Audit error: {message}")]
    Audit {
        /// Error message describing the audit issue
        message: String,
    },

    /// Workflow execution errors
    #[error("Workflow error in {workflow_type}: {message}")]
    Workflow {
        /// The type of workflow that failed
        workflow_type: String,
        /// Error message describing the workflow issue
        message: String,
    },

    /// Workflow not found errors
    #[error("Workflow with ID '{0}' not found")]
    WorkflowNotFound(String),

    /// Unauthorized approver errors
    #[error("Unauthorized approver: {0}")]
    UnauthorizedApprover(String),

    /// Duplicate approval errors
    #[error("Duplicate approval: {0}")]
    DuplicateApproval(String),

    /// Workflow not accepting approvals
    #[error("Workflow not accepting approvals: {0}")]
    WorkflowNotAcceptingApprovals(String),

    /// Workflow expired errors
    #[error("Workflow expired: {0}")]
    WorkflowExpired(String),

    /// Unsupported workflow type errors
    #[error("Unsupported workflow type: {0}")]
    UnsupportedWorkflowType(String),

    /// Invalid workflow request errors
    #[error("Invalid workflow request: {0}")]
    InvalidWorkflowRequest(String),

    /// Integration adapter errors
    #[error("Integration error with {system}: {message}")]
    Integration {
        /// The external system that failed to integrate
        system: String,
        /// Error message describing the integration issue
        message: String,
    },

    /// Storage-related errors
    #[error("Storage error: {message}")]
    Storage {
        /// Error message describing the storage issue
        message: String,
    },

    /// Network communication errors
    #[error("Network error: {message}")]
    Network {
        /// Error message describing the network issue
        message: String,
    },

    /// Data validation errors
    #[error("Validation error in field '{field}': {message}")]
    Validation {
        /// The field that failed validation
        field: String,
        /// Error message describing the validation issue
        message: String,
    },

    /// Rate limiting errors
    #[error("Rate limit exceeded: {message}")]
    RateLimit {
        /// Error message describing the rate limit issue
        message: String,
    },

    /// Resource not found errors
    #[error("{resource_type} with ID '{id}' not found")]
    NotFound {
        /// The type of resource that was not found
        resource_type: String,
        /// The ID of the resource that was not found
        id: String,
    },

    /// Resource already exists errors
    #[error("{resource_type} with ID '{id}' already exists")]
    AlreadyExists {
        /// The type of resource that already exists
        resource_type: String,
        /// The ID of the resource that already exists
        id: String,
    },

    /// Permission denied errors
    #[error("Permission denied for action '{action}' on resource '{resource}'")]
    PermissionDenied {
        /// The action that was denied
        action: String,
        /// The resource for which permission was denied
        resource: String,
    },

    /// Internal system errors
    #[error("Internal error: {message}")]
    Internal {
        /// Error message describing the internal issue
        message: String,
    },

    /// Timeout errors
    #[error("Operation '{operation}' timed out after {duration_ms}ms")]
    Timeout {
        /// The operation that timed out
        operation: String,
        /// The timeout duration in milliseconds
        duration_ms: u64,
    },

    /// Serialization/deserialization errors
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

    // Workflow-related errors
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

    /// Resource exhaustion errors
    #[error("Resource exhaustion: {0}")]
    ResourceExhaustion(String),

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

    /// Error when not enough entropy sources are available for multimodal collection
    #[error("Insufficient entropy sources for multimodal collection")]
    InsufficientEntropySourcesForMultimodal,

    /// Error when entropy components are empty or missing
    #[error("Empty entropy components")]
    EmptyEntropyComponents,

    // HSM-specific error variants
    /// Error when HSM is not available on the system
    #[error("HSM unavailable: {hsm_type} - {reason}")]
    HsmUnavailable {
        /// The type of HSM that is unavailable
        hsm_type: String,
        /// Reason why the HSM is unavailable
        reason: String,
    },

    /// Error when HSM operation fails
    #[error("HSM operation failed: {hsm_type} - {operation} - {error}")]
    HsmOperationFailed {
        /// The type of HSM where the operation failed
        hsm_type: String,
        /// The operation that failed
        operation: String,
        /// Error message describing the failure
        error: String,
    },

    /// Error when key generation fails in HSM
    #[error("HSM key generation failed: {hsm_type} - {error}")]
    HsmKeyGenerationFailed {
        /// The type of HSM where key generation failed
        hsm_type: String,
        /// Error message describing the failure
        error: String,
    },

    /// Error when encryption fails in HSM
    #[error("HSM encryption failed: {hsm_type} - {key_id} - {error}")]
    HsmEncryptionFailed {
        /// The type of HSM where encryption failed
        hsm_type: String,
        /// The key ID that was used
        key_id: String,
        /// Error message describing the failure
        error: String,
    },

    /// Error when decryption fails in HSM
    #[error("HSM decryption failed: {hsm_type} - {key_id} - {error}")]
    HsmDecryptionFailed {
        /// The type of HSM where decryption failed
        hsm_type: String,
        /// The key ID that was used
        key_id: String,
        /// Error message describing the failure
        error: String,
    },

    /// Error when signing fails in HSM
    #[error("HSM signing failed: {hsm_type} - {key_id} - {error}")]
    HsmSigningFailed {
        /// The type of HSM where signing failed
        hsm_type: String,
        /// The key ID that was used
        key_id: String,
        /// Error message describing the failure
        error: String,
    },

    /// Error when an unsupported key type is requested
    #[error("Unsupported key type: {key_type:?} for HSM type: {hsm_type}")]
    UnsupportedKeyType {
        /// The key type that is not supported
        key_type: crate::tunnel::hsm::types::KeyType,
        /// The HSM type that doesn't support it
        hsm_type: String,
    },

    /// Error when an unsupported HSM type is requested
    #[error("Unsupported HSM type: {hsm_type} - {reason}")]
    UnsupportedHsmType {
        /// The HSM type that is not supported
        hsm_type: String,
        /// Reason why the HSM type is not supported
        reason: String,
    },

    /// Error when an unsupported operation is requested
    #[error("Unsupported operation: {operation} for HSM type: {hsm_type} - {reason}")]
    UnsupportedOperation {
        /// The operation that is not supported
        operation: String,
        /// The HSM type that doesn't support it
        hsm_type: String,
        /// Reason why the operation is not supported
        reason: String,
    },

    /// Error when an unsupported crypto backend is requested
    #[error("Unsupported crypto backend: {backend}")]
    UnsupportedCryptoBackend {
        /// The crypto backend that is not supported
        backend: String,
    },

    /// Error when an unsupported storage type is requested
    #[error("Unsupported storage type: {storage_type}")]
    UnsupportedStorageType {
        /// The storage type that is not supported
        storage_type: String,
    },

    /// Error when HSM provider is not found
    #[error("HSM provider not found: {provider_id}")]
    ProviderNotFound {
        /// The provider ID that was not found
        provider_id: String,
    },

    /// Error when no suitable HSM provider is found for requirements
    #[error("No suitable HSM provider found for requirements: {requirements}")]
    NoSuitableProvider {
        /// The requirements that couldn't be satisfied
        requirements: String,
    },

    /// Error when all HSM providers are unhealthy
    #[error("All HSM providers are unhealthy (total: {total_providers})")]
    AllProvidersUnhealthy {
        /// Total number of providers that are unhealthy
        total_providers: usize,
    },

    /// Error when a key is not found
    #[error("Key not found: {key_id}")]
    KeyNotFound {
        /// The key ID that was not found
        key_id: String,
    },

    /// Error when HSM configuration is missing
    #[error("Missing HSM configuration: {component} - {missing_field}")]
    MissingConfiguration {
        /// The component that is missing configuration
        component: String,
        /// The specific field that is missing
        missing_field: String,
    },

    /// Error when key usage limit is exceeded
    #[error("Key usage exceeded: {key_id} - max: {max_usage}, current: {current_usage}")]
    KeyUsageExceeded {
        /// The key ID that exceeded usage
        key_id: String,
        /// Maximum allowed usage
        max_usage: u64,
        /// Current usage count
        current_usage: u64,
    },

    /// Error when data serialization fails
    #[error("Serialization error: {error}")]
    SerializationError {
        /// Error message describing the serialization failure
        error: String,
    },

    /// Error when data deserialization fails
    #[error("Deserialization error: {error}")]
    DeserializationError {
        /// Error message describing the deserialization failure
        error: String,
    },

    /// Error when unsupported format is requested
    #[error("Unsupported format: {format}")]
    UnsupportedFormat {
        /// The format that is not supported
        format: String,
    },

    /// Error when unsupported logger type is requested
    #[error("Unsupported logger type: {logger_type}")]
    UnsupportedLoggerType {
        /// The logger type that is not supported
        logger_type: String,
    },

    /// Error when invalid operation is attempted
    #[error("Invalid operation: {operation} - {reason}")]
    InvalidOperation {
        /// The operation that is invalid
        operation: String,
        /// Reason why the operation is invalid
        reason: String,
    },

    /// Error when invalid configuration is provided
    #[error("Invalid configuration: {field} - {message}")]
    InvalidConfig {
        /// The configuration field that is invalid
        field: String,
        /// Error message describing the configuration issue
        message: String,
    },
}

// Manual From implementations for types that don't have std::error::Error
impl From<ring::error::Unspecified> for BearDogError {
    fn from(_err: ring::error::Unspecified) -> Self {
        BearDogError::Crypto {
            message: "Cryptographic operation failed".to_string(),
        }
    }
}

impl From<argon2::Error> for BearDogError {
    fn from(err: argon2::Error) -> Self {
        BearDogError::KeyDerivation {
            message: format!("Key derivation failed: {err:?}"),
        }
    }
}

impl From<tokio::time::error::Elapsed> for BearDogError {
    fn from(_err: tokio::time::error::Elapsed) -> Self {
        BearDogError::AsyncTimeout {
            message: "Operation timed out".to_string(),
        }
    }
}

impl From<std::string::FromUtf8Error> for BearDogError {
    fn from(error: std::string::FromUtf8Error) -> Self {
        BearDogError::InvalidData {
            message: format!("UTF-8 conversion error: {error}"),
        }
    }
}

impl From<crate::adapters::nestgate::NestGateError> for BearDogError {
    fn from(err: crate::adapters::nestgate::NestGateError) -> Self {
        BearDogError::IoError(err.to_string())
    }
}

// Convenience constructors for common error patterns
impl BearDogError {
    /// Create a configuration error
    pub fn config<S: Into<String>>(message: S) -> Self {
        BearDogError::Configuration {
            message: message.into(),
        }
    }

    /// Create an encryption error
    pub fn encryption<S1: Into<String>, S2: Into<String>>(operation: S1, message: S2) -> Self {
        BearDogError::Encryption {
            operation: operation.into(),
            message: message.into(),
        }
    }

    /// Create an authentication error
    pub fn auth<S: Into<String>>(message: S) -> Self {
        BearDogError::Authentication {
            message: message.into(),
        }
    }

    /// Create an authorization error
    pub fn authz<S: Into<String>>(message: S) -> Self {
        BearDogError::Authorization {
            message: message.into(),
        }
    }

    /// Create a validation error
    pub fn validation<S1: Into<String>, S2: Into<String>>(field: S1, message: S2) -> Self {
        BearDogError::Validation {
            field: field.into(),
            message: message.into(),
        }
    }

    /// Create a not found error
    pub fn not_found<S1: Into<String>, S2: Into<String>>(resource_type: S1, id: S2) -> Self {
        BearDogError::NotFound {
            resource_type: resource_type.into(),
            id: id.into(),
        }
    }

    /// Create an internal error
    pub fn internal<S: Into<String>>(message: S) -> Self {
        BearDogError::Internal {
            message: message.into(),
        }
    }

    /// Create a verification failed error
    pub fn verification_failed<S: Into<String>>(message: S) -> Self {
        BearDogError::VerificationFailed {
            message: message.into(),
        }
    }

    /// Create a node not found error
    pub fn node_not_found<S: Into<String>>(node_id: S) -> Self {
        BearDogError::NodeNotFound {
            node_id: node_id.into(),
        }
    }
}
