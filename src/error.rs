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
    #[error("Workflow already exists: {0}")]
    WorkflowAlreadyExists(String),
    #[error("Workflow in invalid state: {0}")]
    WorkflowInvalidState(String),
    #[error("Workflow approval failed: {0}")]
    WorkflowApprovalFailed(String),
    #[error("Workflow execution failed: {0}")]
    WorkflowExecutionFailed(String),
    #[error("Workflow validation failed: {0}")]
    WorkflowValidationFailed(String),
    #[error("Workflow permission denied: {0}")]
    WorkflowPermissionDenied(String),

    // Network and system errors
    #[error("Network error: {0}")]
    NetworkError(String),

    // Additional missing variants
    #[error("IO error: {0}")]
    IoError(String),
    #[error("Invalid key: {0}")]
    InvalidKey(String),
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
    #[error("Spawn rejected: {reason}")]
    SpawnRejected {
        /// Reason why the spawn was rejected
        reason: String,
    },

    #[error("Operation timeout: {operation}")]
    OperationTimeout {
        /// The operation that timed out
        operation: String,
    },

    #[error("Unexpected state: {message}")]
    UnexpectedState {
        /// Description of the unexpected state
        message: String,
    },

    #[error("Resource cleanup failed: {resource}")]
    ResourceCleanupFailed {
        /// The resource that failed to cleanup
        resource: String,
    },

    #[error("Lineage integrity violation: {message}")]
    LineageIntegrityViolation {
        /// Description of the lineage integrity issue
        message: String,
    },

    #[error("Audit trail incomplete: {message}")]
    AuditTrailIncomplete {
        /// Description of what's missing from the audit trail
        message: String,
    },

    #[error("Audit integrity violation: {message}")]
    AuditIntegrityViolation {
        /// Description of the audit integrity issue
        message: String,
    },

    #[error("Invalid genetics: {message}")]
    InvalidGenetics {
        /// Description of what's wrong with the genetics
        message: String,
    },

    // BSTP (BearDog Secure Tunnel Protocol) related errors
    #[error("Session not found")]
    SessionNotFound,

    #[error("Peer not trusted")]
    PeerNotTrusted,

    #[error("Hardware acceleration not available")]
    HardwareNotAvailable,

    // Cross-node authentication errors
    #[error("Verification failed: {message}")]
    VerificationFailed {
        /// Description of what verification failed
        message: String,
    },

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
            message: format!("Key derivation failed: {:?}", err),
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
            message: format!("UTF-8 conversion error: {}", error),
        }
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
