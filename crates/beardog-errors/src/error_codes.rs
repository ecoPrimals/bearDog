//! Structured Error Code System for BearDog
//!
//! This module provides a comprehensive, structured error code system that enables:
//! - Consistent error identification across the ecosystem
//! - Machine-readable error codes for monitoring and alerting
//! - Easy error categorization and tracking
//! - Internationalization support (codes are language-independent)
//!
//! ## Code Structure
//!
//! Error codes follow the format: `CATEGORY_NNNN_DESCRIPTION`
//!
//! **Categories**:
//! - 1000-1999: Security errors
//! - 2000-2999: Network errors
//! - 3000-3999: HSM/Hardware errors
//! - 4000-4999: Storage errors
//! - 5000-5999: Configuration errors
//! - 6000-6999: Service Discovery errors
//! - 7000-7999: AI/ML errors
//! - 8000-8999: System errors
//! - 9000-9999: Application/Business Logic errors
//!
//! ## Examples
//!
//! ```rust
//! use beardog_errors::{BearDogError, BearDogErrorCode};
//!
//! // Create error with code
//! let error = BearDogError::security(
//!     "Unauthorized access",
//!     None,
//! ).with_code(BearDogErrorCode::SEC_1001_UNAUTHORIZED);
//!
//! // Check error code
//! if error.code() == Some(&BearDogErrorCode::SEC_1001_UNAUTHORIZED) {
//!     // Handle unauthorized access
//! }
//! ```

use std::fmt;

/// Structured error codes for the BearDog ecosystem
///
/// Error codes are organized by category (1000s digit) and provide
/// machine-readable identifiers for error tracking, monitoring, and
/// internationalization.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
#[non_exhaustive]
pub enum BearDogErrorCode {
    // ============================================
    // SECURITY ERRORS (1000-1999)
    // ============================================
    
    /// Unauthorized access attempt (missing or invalid credentials)
    SEC_1001_UNAUTHORIZED,
    
    /// Forbidden operation (valid credentials but insufficient permissions)
    SEC_1002_FORBIDDEN,
    
    /// Authentication failed (invalid username/password)
    SEC_1003_AUTHENTICATION_FAILED,
    
    /// Token expired (JWT or session token no longer valid)
    SEC_1004_TOKEN_EXPIRED,
    
    /// Token invalid (malformed or tampered token)
    SEC_1005_TOKEN_INVALID,
    
    /// Encryption failed (cipher operation error)
    SEC_1010_ENCRYPTION_FAILED,
    
    /// Decryption failed (cipher operation error or wrong key)
    SEC_1011_DECRYPTION_FAILED,
    
    /// Key generation failed (RNG or key derivation error)
    SEC_1012_KEY_GENERATION_FAILED,
    
    /// Certificate validation failed (invalid cert chain)
    SEC_1020_CERT_VALIDATION_FAILED,
    
    /// Certificate expired
    SEC_1021_CERT_EXPIRED,
    
    /// TLS handshake failed
    SEC_1022_TLS_HANDSHAKE_FAILED,
    
    /// Rate limit exceeded (too many requests)
    SEC_1030_RATE_LIMIT_EXCEEDED,
    
    /// Malicious input detected (potential injection attack)
    SEC_1040_MALICIOUS_INPUT_DETECTED,
    
    // ============================================
    // NETWORK ERRORS (2000-2999)
    // ============================================
    
    /// Connection refused (target host unreachable or port closed)
    NET_2001_CONNECTION_REFUSED,
    
    /// Connection timeout (target host not responding)
    NET_2002_CONNECTION_TIMEOUT,
    
    /// Connection reset (connection dropped unexpectedly)
    NET_2003_CONNECTION_RESET,
    
    /// Request timeout (no response within timeout period)
    NET_2010_REQUEST_TIMEOUT,
    
    /// DNS resolution failed (hostname lookup error)
    NET_2020_DNS_RESOLUTION_FAILED,
    
    /// HTTP error 400 (Bad Request)
    NET_2400_BAD_REQUEST,
    
    /// HTTP error 404 (Not Found)
    NET_2404_NOT_FOUND,
    
    /// HTTP error 500 (Internal Server Error)
    NET_2500_INTERNAL_SERVER_ERROR,
    
    /// HTTP error 502 (Bad Gateway)
    NET_2502_BAD_GATEWAY,
    
    /// HTTP error 503 (Service Unavailable)
    NET_2503_SERVICE_UNAVAILABLE,
    
    /// HTTP error 504 (Gateway Timeout)
    NET_2504_GATEWAY_TIMEOUT,
    
    /// Circuit breaker open (too many failures, circuit tripped)
    NET_2600_CIRCUIT_BREAKER_OPEN,
    
    /// Load balancer error (no healthy backends)
    NET_2610_NO_HEALTHY_BACKENDS,
    
    // ============================================
    // HSM/HARDWARE ERRORS (3000-3999)
    // ============================================
    
    /// HSM device not found (no hardware detected)
    HSM_3001_DEVICE_NOT_FOUND,
    
    /// HSM operation failed (generic hardware error)
    HSM_3002_OPERATION_FAILED,
    
    /// HSM authentication required (PIN/passphrase needed)
    HSM_3003_AUTHENTICATION_REQUIRED,
    
    /// HSM authentication failed (wrong PIN/passphrase)
    HSM_3004_AUTHENTICATION_FAILED,
    
    /// HSM PIN blocked (too many failed attempts)
    HSM_3005_PIN_BLOCKED,
    
    /// PKCS#11 initialization failed
    HSM_3010_PKCS11_INIT_FAILED,
    
    /// PKCS#11 session error
    HSM_3011_PKCS11_SESSION_ERROR,
    
    /// PKCS#11 token not present
    HSM_3012_PKCS11_TOKEN_NOT_PRESENT,
    
    /// FIDO2 device not found
    HSM_3020_FIDO2_DEVICE_NOT_FOUND,
    
    /// FIDO2 operation canceled (user declined)
    HSM_3021_FIDO2_OPERATION_CANCELED,
    
    /// FIDO2 timeout (no user interaction)
    HSM_3022_FIDO2_TIMEOUT,
    
    /// TPM 2.0 not available
    HSM_3030_TPM_NOT_AVAILABLE,
    
    /// TPM 2.0 operation failed
    HSM_3031_TPM_OPERATION_FAILED,
    
    /// Android StrongBox not available
    HSM_3040_STRONGBOX_NOT_AVAILABLE,
    
    /// Android StrongBox operation failed
    HSM_3041_STRONGBOX_OPERATION_FAILED,
    
    // ============================================
    // STORAGE ERRORS (4000-4999)
    // ============================================
    
    /// File not found
    STORAGE_4001_FILE_NOT_FOUND,
    
    /// Permission denied (cannot read/write file)
    STORAGE_4002_PERMISSION_DENIED,
    
    /// Disk full (no space left on device)
    STORAGE_4003_DISK_FULL,
    
    /// I/O error (read/write operation failed)
    STORAGE_4004_IO_ERROR,
    
    /// Database connection failed
    STORAGE_4010_DB_CONNECTION_FAILED,
    
    /// Database query failed
    STORAGE_4011_DB_QUERY_FAILED,
    
    /// Database transaction failed
    STORAGE_4012_DB_TRANSACTION_FAILED,
    
    /// Cache miss (requested key not in cache)
    STORAGE_4020_CACHE_MISS,
    
    /// Cache eviction failed
    STORAGE_4021_CACHE_EVICTION_FAILED,
    
    /// Serialization failed (cannot encode data)
    STORAGE_4030_SERIALIZATION_FAILED,
    
    /// Deserialization failed (cannot decode data)
    STORAGE_4031_DESERIALIZATION_FAILED,
    
    // ============================================
    // CONFIGURATION ERRORS (5000-5999)
    // ============================================
    
    /// Configuration file not found
    CONFIG_5001_FILE_NOT_FOUND,
    
    /// Configuration parse error (invalid format)
    CONFIG_5002_PARSE_ERROR,
    
    /// Configuration validation failed (invalid values)
    CONFIG_5003_VALIDATION_FAILED,
    
    /// Missing required configuration field
    CONFIG_5004_MISSING_REQUIRED_FIELD,
    
    /// Invalid configuration value
    CONFIG_5005_INVALID_VALUE,
    
    /// Environment variable not set
    CONFIG_5010_ENV_VAR_NOT_SET,
    
    /// Environment variable invalid
    CONFIG_5011_ENV_VAR_INVALID,
    
    // ============================================
    // SERVICE DISCOVERY ERRORS (6000-6999)
    // ============================================
    
    /// Service not found (no instances discovered)
    DISCOVERY_6001_SERVICE_NOT_FOUND,
    
    /// Service registration failed
    DISCOVERY_6002_REGISTRATION_FAILED,
    
    /// Service deregistration failed
    DISCOVERY_6003_DEREGISTRATION_FAILED,
    
    /// Health check failed (service unhealthy)
    DISCOVERY_6010_HEALTH_CHECK_FAILED,
    
    /// Service discovery timeout
    DISCOVERY_6020_TIMEOUT,
    
    /// No healthy instances available
    DISCOVERY_6030_NO_HEALTHY_INSTANCES,
    
    // ============================================
    // AI/ML ERRORS (7000-7999)
    // ============================================
    
    /// Model not found
    AI_7001_MODEL_NOT_FOUND,
    
    /// Model loading failed
    AI_7002_MODEL_LOAD_FAILED,
    
    /// Inference failed (prediction error)
    AI_7010_INFERENCE_FAILED,
    
    /// Inference timeout
    AI_7011_INFERENCE_TIMEOUT,
    
    /// Training failed
    AI_7020_TRAINING_FAILED,
    
    /// Invalid input shape (tensor dimension mismatch)
    AI_7030_INVALID_INPUT_SHAPE,
    
    /// GPU not available (CUDA error)
    AI_7040_GPU_NOT_AVAILABLE,
    
    // ============================================
    // SYSTEM ERRORS (8000-8999)
    // ============================================
    
    /// Out of memory
    SYS_8001_OUT_OF_MEMORY,
    
    /// Thread pool exhausted
    SYS_8002_THREAD_POOL_EXHAUSTED,
    
    /// Resource limit exceeded
    SYS_8003_RESOURCE_LIMIT_EXCEEDED,
    
    /// Deadlock detected
    SYS_8010_DEADLOCK_DETECTED,
    
    /// Panic/crash (unrecoverable error)
    SYS_8020_PANIC,
    
    /// Invalid state (operation called in wrong state)
    SYS_8030_INVALID_STATE,
    
    /// Initialization failed
    SYS_8040_INITIALIZATION_FAILED,
    
    /// Shutdown timeout
    SYS_8041_SHUTDOWN_TIMEOUT,
    
    // ============================================
    // APPLICATION/BUSINESS LOGIC ERRORS (9000-9999)
    // ============================================
    
    /// Invalid request (malformed input)
    APP_9001_INVALID_REQUEST,
    
    /// Resource not found (business entity not found)
    APP_9002_RESOURCE_NOT_FOUND,
    
    /// Resource already exists (duplicate creation attempt)
    APP_9003_RESOURCE_ALREADY_EXISTS,
    
    /// Operation not permitted (business rule violation)
    APP_9004_OPERATION_NOT_PERMITTED,
    
    /// Validation failed (business logic validation)
    APP_9005_VALIDATION_FAILED,
    
    /// Workflow timeout
    APP_9010_WORKFLOW_TIMEOUT,
    
    /// Workflow canceled
    APP_9011_WORKFLOW_CANCELED,
}

impl BearDogErrorCode {
    /// Get the numeric code value
    ///
    /// # Examples
    ///
    /// ```
    /// use beardog_errors::BearDogErrorCode;
    ///
    /// assert_eq!(BearDogErrorCode::SEC_1001_UNAUTHORIZED.code(), 1001);
    /// assert_eq!(BearDogErrorCode::NET_2001_CONNECTION_REFUSED.code(), 2001);
    /// ```
    #[must_use]
    pub const fn code(&self) -> u16 {
        match self {
            // Security (1000-1999)
            Self::SEC_1001_UNAUTHORIZED => 1001,
            Self::SEC_1002_FORBIDDEN => 1002,
            Self::SEC_1003_AUTHENTICATION_FAILED => 1003,
            Self::SEC_1004_TOKEN_EXPIRED => 1004,
            Self::SEC_1005_TOKEN_INVALID => 1005,
            Self::SEC_1010_ENCRYPTION_FAILED => 1010,
            Self::SEC_1011_DECRYPTION_FAILED => 1011,
            Self::SEC_1012_KEY_GENERATION_FAILED => 1012,
            Self::SEC_1020_CERT_VALIDATION_FAILED => 1020,
            Self::SEC_1021_CERT_EXPIRED => 1021,
            Self::SEC_1022_TLS_HANDSHAKE_FAILED => 1022,
            Self::SEC_1030_RATE_LIMIT_EXCEEDED => 1030,
            Self::SEC_1040_MALICIOUS_INPUT_DETECTED => 1040,
            
            // Network (2000-2999)
            Self::NET_2001_CONNECTION_REFUSED => 2001,
            Self::NET_2002_CONNECTION_TIMEOUT => 2002,
            Self::NET_2003_CONNECTION_RESET => 2003,
            Self::NET_2010_REQUEST_TIMEOUT => 2010,
            Self::NET_2020_DNS_RESOLUTION_FAILED => 2020,
            Self::NET_2400_BAD_REQUEST => 2400,
            Self::NET_2404_NOT_FOUND => 2404,
            Self::NET_2500_INTERNAL_SERVER_ERROR => 2500,
            Self::NET_2502_BAD_GATEWAY => 2502,
            Self::NET_2503_SERVICE_UNAVAILABLE => 2503,
            Self::NET_2504_GATEWAY_TIMEOUT => 2504,
            Self::NET_2600_CIRCUIT_BREAKER_OPEN => 2600,
            Self::NET_2610_NO_HEALTHY_BACKENDS => 2610,
            
            // HSM (3000-3999)
            Self::HSM_3001_DEVICE_NOT_FOUND => 3001,
            Self::HSM_3002_OPERATION_FAILED => 3002,
            Self::HSM_3003_AUTHENTICATION_REQUIRED => 3003,
            Self::HSM_3004_AUTHENTICATION_FAILED => 3004,
            Self::HSM_3005_PIN_BLOCKED => 3005,
            Self::HSM_3010_PKCS11_INIT_FAILED => 3010,
            Self::HSM_3011_PKCS11_SESSION_ERROR => 3011,
            Self::HSM_3012_PKCS11_TOKEN_NOT_PRESENT => 3012,
            Self::HSM_3020_FIDO2_DEVICE_NOT_FOUND => 3020,
            Self::HSM_3021_FIDO2_OPERATION_CANCELED => 3021,
            Self::HSM_3022_FIDO2_TIMEOUT => 3022,
            Self::HSM_3030_TPM_NOT_AVAILABLE => 3030,
            Self::HSM_3031_TPM_OPERATION_FAILED => 3031,
            Self::HSM_3040_STRONGBOX_NOT_AVAILABLE => 3040,
            Self::HSM_3041_STRONGBOX_OPERATION_FAILED => 3041,
            
            // Storage (4000-4999)
            Self::STORAGE_4001_FILE_NOT_FOUND => 4001,
            Self::STORAGE_4002_PERMISSION_DENIED => 4002,
            Self::STORAGE_4003_DISK_FULL => 4003,
            Self::STORAGE_4004_IO_ERROR => 4004,
            Self::STORAGE_4010_DB_CONNECTION_FAILED => 4010,
            Self::STORAGE_4011_DB_QUERY_FAILED => 4011,
            Self::STORAGE_4012_DB_TRANSACTION_FAILED => 4012,
            Self::STORAGE_4020_CACHE_MISS => 4020,
            Self::STORAGE_4021_CACHE_EVICTION_FAILED => 4021,
            Self::STORAGE_4030_SERIALIZATION_FAILED => 4030,
            Self::STORAGE_4031_DESERIALIZATION_FAILED => 4031,
            
            // Configuration (5000-5999)
            Self::CONFIG_5001_FILE_NOT_FOUND => 5001,
            Self::CONFIG_5002_PARSE_ERROR => 5002,
            Self::CONFIG_5003_VALIDATION_FAILED => 5003,
            Self::CONFIG_5004_MISSING_REQUIRED_FIELD => 5004,
            Self::CONFIG_5005_INVALID_VALUE => 5005,
            Self::CONFIG_5010_ENV_VAR_NOT_SET => 5010,
            Self::CONFIG_5011_ENV_VAR_INVALID => 5011,
            
            // Service Discovery (6000-6999)
            Self::DISCOVERY_6001_SERVICE_NOT_FOUND => 6001,
            Self::DISCOVERY_6002_REGISTRATION_FAILED => 6002,
            Self::DISCOVERY_6003_DEREGISTRATION_FAILED => 6003,
            Self::DISCOVERY_6010_HEALTH_CHECK_FAILED => 6010,
            Self::DISCOVERY_6020_TIMEOUT => 6020,
            Self::DISCOVERY_6030_NO_HEALTHY_INSTANCES => 6030,
            
            // AI/ML (7000-7999)
            Self::AI_7001_MODEL_NOT_FOUND => 7001,
            Self::AI_7002_MODEL_LOAD_FAILED => 7002,
            Self::AI_7010_INFERENCE_FAILED => 7010,
            Self::AI_7011_INFERENCE_TIMEOUT => 7011,
            Self::AI_7020_TRAINING_FAILED => 7020,
            Self::AI_7030_INVALID_INPUT_SHAPE => 7030,
            Self::AI_7040_GPU_NOT_AVAILABLE => 7040,
            
            // System (8000-8999)
            Self::SYS_8001_OUT_OF_MEMORY => 8001,
            Self::SYS_8002_THREAD_POOL_EXHAUSTED => 8002,
            Self::SYS_8003_RESOURCE_LIMIT_EXCEEDED => 8003,
            Self::SYS_8010_DEADLOCK_DETECTED => 8010,
            Self::SYS_8020_PANIC => 8020,
            Self::SYS_8030_INVALID_STATE => 8030,
            Self::SYS_8040_INITIALIZATION_FAILED => 8040,
            Self::SYS_8041_SHUTDOWN_TIMEOUT => 8041,
            
            // Application (9000-9999)
            Self::APP_9001_INVALID_REQUEST => 9001,
            Self::APP_9002_RESOURCE_NOT_FOUND => 9002,
            Self::APP_9003_RESOURCE_ALREADY_EXISTS => 9003,
            Self::APP_9004_OPERATION_NOT_PERMITTED => 9004,
            Self::APP_9005_VALIDATION_FAILED => 9005,
            Self::APP_9010_WORKFLOW_TIMEOUT => 9010,
            Self::APP_9011_WORKFLOW_CANCELED => 9011,
        }
    }
    
    /// Get the error category
    ///
    /// # Examples
    ///
    /// ```
    /// use beardog_errors::BearDogErrorCode;
    ///
    /// assert_eq!(BearDogErrorCode::SEC_1001_UNAUTHORIZED.category(), "Security");
    /// assert_eq!(BearDogErrorCode::NET_2001_CONNECTION_REFUSED.category(), "Network");
    /// ```
    #[must_use]
    pub const fn category(&self) -> &'static str {
        match self.code() {
            1000..=1999 => "Security",
            2000..=2999 => "Network",
            3000..=3999 => "HSM/Hardware",
            4000..=4999 => "Storage",
            5000..=5999 => "Configuration",
            6000..=6999 => "Service Discovery",
            7000..=7999 => "AI/ML",
            8000..=8999 => "System",
            9000..=9999 => "Application",
            _ => "Unknown",
        }
    }
    
    /// Get a human-readable description
    #[must_use]
    pub const fn description(&self) -> &'static str {
        match self {
            Self::SEC_1001_UNAUTHORIZED => "Unauthorized access attempt",
            Self::SEC_1002_FORBIDDEN => "Forbidden operation",
            Self::SEC_1003_AUTHENTICATION_FAILED => "Authentication failed",
            Self::SEC_1004_TOKEN_EXPIRED => "Token expired",
            Self::SEC_1005_TOKEN_INVALID => "Token invalid",
            Self::SEC_1010_ENCRYPTION_FAILED => "Encryption failed",
            Self::SEC_1011_DECRYPTION_FAILED => "Decryption failed",
            Self::SEC_1012_KEY_GENERATION_FAILED => "Key generation failed",
            Self::SEC_1020_CERT_VALIDATION_FAILED => "Certificate validation failed",
            Self::SEC_1021_CERT_EXPIRED => "Certificate expired",
            Self::SEC_1022_TLS_HANDSHAKE_FAILED => "TLS handshake failed",
            Self::SEC_1030_RATE_LIMIT_EXCEEDED => "Rate limit exceeded",
            Self::SEC_1040_MALICIOUS_INPUT_DETECTED => "Malicious input detected",
            
            Self::NET_2001_CONNECTION_REFUSED => "Connection refused",
            Self::NET_2002_CONNECTION_TIMEOUT => "Connection timeout",
            Self::NET_2003_CONNECTION_RESET => "Connection reset",
            Self::NET_2010_REQUEST_TIMEOUT => "Request timeout",
            Self::NET_2020_DNS_RESOLUTION_FAILED => "DNS resolution failed",
            Self::NET_2400_BAD_REQUEST => "Bad request (HTTP 400)",
            Self::NET_2404_NOT_FOUND => "Not found (HTTP 404)",
            Self::NET_2500_INTERNAL_SERVER_ERROR => "Internal server error (HTTP 500)",
            Self::NET_2502_BAD_GATEWAY => "Bad gateway (HTTP 502)",
            Self::NET_2503_SERVICE_UNAVAILABLE => "Service unavailable (HTTP 503)",
            Self::NET_2504_GATEWAY_TIMEOUT => "Gateway timeout (HTTP 504)",
            Self::NET_2600_CIRCUIT_BREAKER_OPEN => "Circuit breaker open",
            Self::NET_2610_NO_HEALTHY_BACKENDS => "No healthy backends",
            
            Self::HSM_3001_DEVICE_NOT_FOUND => "HSM device not found",
            Self::HSM_3002_OPERATION_FAILED => "HSM operation failed",
            Self::HSM_3003_AUTHENTICATION_REQUIRED => "HSM authentication required",
            Self::HSM_3004_AUTHENTICATION_FAILED => "HSM authentication failed",
            Self::HSM_3005_PIN_BLOCKED => "HSM PIN blocked",
            Self::HSM_3010_PKCS11_INIT_FAILED => "PKCS#11 initialization failed",
            Self::HSM_3011_PKCS11_SESSION_ERROR => "PKCS#11 session error",
            Self::HSM_3012_PKCS11_TOKEN_NOT_PRESENT => "PKCS#11 token not present",
            Self::HSM_3020_FIDO2_DEVICE_NOT_FOUND => "FIDO2 device not found",
            Self::HSM_3021_FIDO2_OPERATION_CANCELED => "FIDO2 operation canceled",
            Self::HSM_3022_FIDO2_TIMEOUT => "FIDO2 timeout",
            Self::HSM_3030_TPM_NOT_AVAILABLE => "TPM 2.0 not available",
            Self::HSM_3031_TPM_OPERATION_FAILED => "TPM 2.0 operation failed",
            Self::HSM_3040_STRONGBOX_NOT_AVAILABLE => "Android StrongBox not available",
            Self::HSM_3041_STRONGBOX_OPERATION_FAILED => "Android StrongBox operation failed",
            
            Self::STORAGE_4001_FILE_NOT_FOUND => "File not found",
            Self::STORAGE_4002_PERMISSION_DENIED => "Permission denied",
            Self::STORAGE_4003_DISK_FULL => "Disk full",
            Self::STORAGE_4004_IO_ERROR => "I/O error",
            Self::STORAGE_4010_DB_CONNECTION_FAILED => "Database connection failed",
            Self::STORAGE_4011_DB_QUERY_FAILED => "Database query failed",
            Self::STORAGE_4012_DB_TRANSACTION_FAILED => "Database transaction failed",
            Self::STORAGE_4020_CACHE_MISS => "Cache miss",
            Self::STORAGE_4021_CACHE_EVICTION_FAILED => "Cache eviction failed",
            Self::STORAGE_4030_SERIALIZATION_FAILED => "Serialization failed",
            Self::STORAGE_4031_DESERIALIZATION_FAILED => "Deserialization failed",
            
            Self::CONFIG_5001_FILE_NOT_FOUND => "Configuration file not found",
            Self::CONFIG_5002_PARSE_ERROR => "Configuration parse error",
            Self::CONFIG_5003_VALIDATION_FAILED => "Configuration validation failed",
            Self::CONFIG_5004_MISSING_REQUIRED_FIELD => "Missing required configuration field",
            Self::CONFIG_5005_INVALID_VALUE => "Invalid configuration value",
            Self::CONFIG_5010_ENV_VAR_NOT_SET => "Environment variable not set",
            Self::CONFIG_5011_ENV_VAR_INVALID => "Environment variable invalid",
            
            Self::DISCOVERY_6001_SERVICE_NOT_FOUND => "Service not found",
            Self::DISCOVERY_6002_REGISTRATION_FAILED => "Service registration failed",
            Self::DISCOVERY_6003_DEREGISTRATION_FAILED => "Service deregistration failed",
            Self::DISCOVERY_6010_HEALTH_CHECK_FAILED => "Health check failed",
            Self::DISCOVERY_6020_TIMEOUT => "Service discovery timeout",
            Self::DISCOVERY_6030_NO_HEALTHY_INSTANCES => "No healthy instances available",
            
            Self::AI_7001_MODEL_NOT_FOUND => "Model not found",
            Self::AI_7002_MODEL_LOAD_FAILED => "Model loading failed",
            Self::AI_7010_INFERENCE_FAILED => "Inference failed",
            Self::AI_7011_INFERENCE_TIMEOUT => "Inference timeout",
            Self::AI_7020_TRAINING_FAILED => "Training failed",
            Self::AI_7030_INVALID_INPUT_SHAPE => "Invalid input shape",
            Self::AI_7040_GPU_NOT_AVAILABLE => "GPU not available",
            
            Self::SYS_8001_OUT_OF_MEMORY => "Out of memory",
            Self::SYS_8002_THREAD_POOL_EXHAUSTED => "Thread pool exhausted",
            Self::SYS_8003_RESOURCE_LIMIT_EXCEEDED => "Resource limit exceeded",
            Self::SYS_8010_DEADLOCK_DETECTED => "Deadlock detected",
            Self::SYS_8020_PANIC => "Panic/crash",
            Self::SYS_8030_INVALID_STATE => "Invalid state",
            Self::SYS_8040_INITIALIZATION_FAILED => "Initialization failed",
            Self::SYS_8041_SHUTDOWN_TIMEOUT => "Shutdown timeout",
            
            Self::APP_9001_INVALID_REQUEST => "Invalid request",
            Self::APP_9002_RESOURCE_NOT_FOUND => "Resource not found",
            Self::APP_9003_RESOURCE_ALREADY_EXISTS => "Resource already exists",
            Self::APP_9004_OPERATION_NOT_PERMITTED => "Operation not permitted",
            Self::APP_9005_VALIDATION_FAILED => "Validation failed",
            Self::APP_9010_WORKFLOW_TIMEOUT => "Workflow timeout",
            Self::APP_9011_WORKFLOW_CANCELED => "Workflow canceled",
        }
    }
}

impl fmt::Display for BearDogErrorCode {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "[{}] {} - {}", self.code(), self.category(), self.description())
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_error_code_values() {
        assert_eq!(BearDogErrorCode::SEC_1001_UNAUTHORIZED.code(), 1001);
        assert_eq!(BearDogErrorCode::NET_2001_CONNECTION_REFUSED.code(), 2001);
        assert_eq!(BearDogErrorCode::HSM_3001_DEVICE_NOT_FOUND.code(), 3001);
        assert_eq!(BearDogErrorCode::STORAGE_4001_FILE_NOT_FOUND.code(), 4001);
    }

    #[test]
    fn test_error_categories() {
        assert_eq!(BearDogErrorCode::SEC_1001_UNAUTHORIZED.category(), "Security");
        assert_eq!(BearDogErrorCode::NET_2001_CONNECTION_REFUSED.category(), "Network");
        assert_eq!(BearDogErrorCode::HSM_3001_DEVICE_NOT_FOUND.category(), "HSM/Hardware");
        assert_eq!(BearDogErrorCode::STORAGE_4001_FILE_NOT_FOUND.category(), "Storage");
    }

    #[test]
    fn test_error_display() {
        let code = BearDogErrorCode::SEC_1001_UNAUTHORIZED;
        let display = format!("{}", code);
        assert!(display.contains("1001"));
        assert!(display.contains("Security"));
        assert!(display.contains("Unauthorized"));
    }
}

