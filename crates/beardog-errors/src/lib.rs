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

    /// Workflow-specific errors
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

    /// Network and system errors
    #[error("Network error: {0}")]
    NetworkError(String),

    #[error("IO error: {0}")]
    IoError(String),

    #[error("Invalid key: {0}")]
    InvalidKey(String),

    #[error("Compliance standard not supported: {0}")]
    ComplianceStandardNotSupported(String),

    #[error("Invalid request: {0}")]
    InvalidRequest(String),

    #[error("Invalid data: {message}")]
    InvalidData {
        /// Error message describing the invalid data issue
        message: String,
    },

    /// Genetic spawning related errors
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

    /// BSTP (BearDog Secure Tunnel Protocol) related errors
    #[error("Session not found")]
    SessionNotFound,

    #[error("Peer not trusted")]
    PeerNotTrusted,

    #[error("Hardware acceleration not available")]
    HardwareNotAvailable,

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

    #[error("Invalid state: {message}")]
    InvalidState {
        /// Error message describing the invalid state
        message: String,
    },

    #[error("Unauthorized: {message}")]
    Unauthorized {
        /// Error message describing the unauthorized access
        message: String,
    },

    #[error("Security violation: {0}")]
    SecurityViolation(String),

    #[error("Validation error: {0}")]
    ValidationError(String),

    #[error("Not implemented: {message}")]
    NotImplemented {
        /// Error message describing what's not implemented
        message: String,
    },

    /// Human entropy collection errors
    #[error("Consent expired")]
    ConsentExpired,

    #[error("Insufficient consent for requested collection")]
    InsufficientConsent,

    #[error("Insufficient entropy quality: required {required}, actual {actual}")]
    InsufficientEntropyQuality {
        /// Required quality score
        required: f64,
        /// Actual quality score
        actual: f64,
    },

    #[error("High sampling rate detected: {rate} samples/sec")]
    HighSamplingRate {
        /// Sampling rate in samples per second
        rate: f64,
    },

    #[error("Entropy source not available: {source_name}")]
    EntropySourceNotAvailable {
        /// The entropy source that is not available
        source_name: String,
    },

    #[error("Privacy violation in entropy collection: {violation}")]
    PrivacyViolation {
        /// Description of the privacy violation
        violation: String,
    },

    #[error("Entropy collection device compromised: {device}")]
    DeviceCompromised {
        /// The device that is compromised
        device: String,
    },

    #[error("Invalid entropy collection context: {context}")]
    InvalidContext {
        /// Description of the invalid context
        context: String,
    },

    #[error("Insufficient randomness in entropy collection")]
    InsufficientRandomness,

    #[error("Bias detected in entropy collection: {bias}")]
    BiasDetected {
        /// Description of the bias detected
        bias: String,
    },

    #[error("Entropy quality degradation: {reason}")]
    QualityDegradation {
        /// Reason for quality degradation
        reason: String,
    },

    #[error("Temporal correlation detected in entropy collection")]
    TemporalCorrelation,

    #[error("Spatial correlation detected in entropy collection")]
    SpatialCorrelation,

    #[error("Entropy collection tampering detected")]
    TamperingDetected,

    #[error("Entropy collection calibration error: {error}")]
    CalibrationError {
        /// Description of the calibration error
        error: String,
    },

    #[error("Environmental interference in entropy collection: {interference}")]
    EnvironmentalInterference {
        /// Description of the environmental interference
        interference: String,
    },

    #[error("Hardware malfunction in entropy collection: {malfunction}")]
    HardwareMalfunction {
        /// Description of the hardware malfunction
        malfunction: String,
    },

    #[error("Software error in entropy collection: {error}")]
    SoftwareError {
        /// Description of the software error
        error: String,
    },

    #[error("Unsupported operation: {operation}")]
    UnsupportedOperation {
        /// The operation that is not supported
        operation: String,
    },

    #[error("No suitable provider found: {message}")]
    NoSuitableProvider {
        /// Error message describing why no suitable provider was found
        message: String,
    },

    #[error("Deserialization failed: {message}")]
    DeserializationError {
        /// Error message describing the deserialization failure
        message: String,
    },

    #[error("Entropy generation failed: {message}")]
    Entropy {
        /// Message describing the entropy error
        message: String,
    },

    #[error("Unsupported key type: {key_type}")]
    UnsupportedKeyType {
        /// The key type that is not supported
        key_type: String,
    },

    #[error("Cryptographic operation failed: {operation}")]
    Cryptographic {
        /// The cryptographic operation that failed
        operation: String,
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

/// External crate error conversions
impl From<argon2::Error> for BearDogError {
    fn from(err: argon2::Error) -> Self {
        BearDogError::Crypto {
            message: err.to_string(),
        }
    }
}
