// SPDX-License-Identifier: AGPL-3.0-only

// Security Domain Constants
//
// This module provides security-related constants consolidated from the large unified.rs file.
// It includes cryptographic settings, authentication parameters, HSM configuration, and security limits.

use std::time::Duration;

/// **AUTHENTICATION CONSTANTS** - Authentication and authorization settings
pub mod auth {
    use super::Duration;

    /// Authentication timeouts
    /// Lifetime of authentication tokens (1 hour)
    pub const AUTH_TOKEN_LIFETIME: Duration = Duration::from_secs(3600); // 1 hour
    /// Lifetime of refresh tokens (7 days)
    pub const REFRESH_TOKEN_LIFETIME: Duration = Duration::from_secs(86400 * 7); // 7 days
    /// Idle session timeout before re-authentication (30 minutes)
    pub const SESSION_TIMEOUT: Duration = Duration::from_secs(1800); // 30 minutes
    /// Lifetime of password reset tokens (15 minutes)
    pub const PASSWORD_RESET_TOKEN_LIFETIME: Duration = Duration::from_secs(900); // 15 minutes

    /// Authentication limits
    pub const MAX_AUTH_ATTEMPTS: u32 = 5;
    /// Duration of authentication lockout (15 minutes)
    pub const AUTH_LOCKOUT_DURATION: Duration = Duration::from_secs(900); // 15 minutes
    /// Maximum concurrent sessions per user
    pub const MAX_CONCURRENT_SESSIONS: u32 = 10;
    /// Minimum required password length
    pub const MIN_PASSWORD_LENGTH: usize = 12;
    /// Maximum allowed password length
    pub const MAX_PASSWORD_LENGTH: usize = 128;

    /// MFA settings
    pub const TOTP_WINDOW_SIZE: u32 = 1;
    /// TOTP time step size in seconds
    pub const TOTP_STEP_SIZE: u32 = 30;
    /// Number of backup codes to generate
    pub const BACKUP_CODES_COUNT: usize = 10;
    /// Length of each backup code
    pub const BACKUP_CODE_LENGTH: usize = 8;

    /// JWT settings
    /// JWT signing algorithm (RS256 - RSA with SHA-256)
    pub const JWT_ALGORITHM: &str = "RS256";
    /// JWT token issuer identifier
    pub const JWT_ISSUER: &str = "beardog ";
    /// JWT token audience identifier
    pub const JWT_AUDIENCE: &str = "beardog-api";
    /// JWT clock skew tolerance
    pub const JWT_LEEWAY: Duration = Duration::from_secs(60);

    /// API key settings
    /// Length of generated API keys in bytes
    pub const API_KEY_LENGTH: usize = 32;
    /// Prefix for generated BearDog API keys
    pub const API_KEY_PREFIX: &str = "bd_";
    /// Default lifetime of API keys (1 year)
    pub const API_KEY_LIFETIME: Duration = Duration::from_secs(86400 * 365); // 1 year

    /// Biometric authentication
    pub const BIOMETRIC_MATCH_THRESHOLD: f64 = 0.95;
    /// Size of biometric templates in bytes
    pub const BIOMETRIC_TEMPLATE_SIZE: usize = 512;
    /// Maximum biometric templates per user
    pub const MAX_BIOMETRIC_TEMPLATES: usize = 5;
}

/// **CRYPTOGRAPHIC CONSTANTS** - Cryptographic algorithms and parameters
pub mod crypto {
    /// Key lengths (in bytes)
    /// AES-128 key length in bytes
    pub const AES_128_KEY_LENGTH: usize = 16;
    /// AES-192 key length in bytes
    pub const AES_192_KEY_LENGTH: usize = 24;
    /// AES-256 key length in bytes
    pub const AES_256_KEY_LENGTH: usize = 32;
    /// Default symmetric key length (AES-256)
    pub const DEFAULT_KEY_LENGTH: usize = AES_256_KEY_LENGTH;

    /// RSA key sizes (in bits)
    /// RSA 2048-bit key size
    pub const RSA_2048_KEY_SIZE: usize = 2048;
    /// RSA 3072-bit key size (recommended minimum)
    pub const RSA_3072_KEY_SIZE: usize = 3072;
    /// RSA 4096-bit key size (high security)
    pub const RSA_4096_KEY_SIZE: usize = 4096;
    /// Default RSA key size (3072-bit)
    pub const DEFAULT_RSA_KEY_SIZE: usize = RSA_3072_KEY_SIZE;

    /// Elliptic curve parameters
    pub const ED25519_KEY_SIZE: usize = 32;
    /// Ed25519 signature size in bytes
    pub const ED25519_SIGNATURE_SIZE: usize = 64;
    /// SECP256R1 key size in bytes
    pub const SECP256R1_KEY_SIZE: usize = 32;
    /// SECP384R1 key size in bytes
    pub const SECP384R1_KEY_SIZE: usize = 48;
    /// SECP521R1 key size in bytes
    pub const SECP521R1_KEY_SIZE: usize = 66;

    /// Hash algorithm parameters
    pub const SHA256_HASH_SIZE: usize = 32;
    /// SHA384 hash size in bytes
    pub const SHA384_HASH_SIZE: usize = 48;
    /// SHA512 hash size in bytes
    pub const SHA512_HASH_SIZE: usize = 64;
    /// BLAKE3 hash size in bytes
    pub const BLAKE3_HASH_SIZE: usize = 32;

    /// PBKDF2 parameters
    pub const PBKDF2_MIN_ITERATIONS: u32 = 100_000;
    /// Default PBKDF2 iteration count (OWASP-aligned default)
    pub const PBKDF2_DEFAULT_ITERATIONS: u32 = 600_000;
    /// PBKDF2 salt length in bytes - cryptographically secure length
    pub const PBKDF2_SALT_LENGTH: usize = 32;

    /// Argon2 parameters
    pub const ARGON2_MEMORY_COST: u32 = 65536; // 64MB
    /// Argon2 time cost parameter - number of iterations
    pub const ARGON2_TIME_COST: u32 = 3;
    /// Argon2 parallelism parameter - number of threads
    pub const ARGON2_PARALLELISM: u32 = 4;
    /// Argon2 salt length in bytes
    pub const ARGON2_SALT_LENGTH: usize = 32;
    /// Argon2 hash output length in bytes
    pub const ARGON2_HASH_LENGTH: usize = 32;

    /// Random number generation
    pub const SECURE_RANDOM_SEED_SIZE: usize = 32;
    /// Configuration constant: nonce size
    pub const NONCE_SIZE: usize = 12;
    /// Configuration constant: iv size
    pub const IV_SIZE: usize = 16;
    /// Configuration constant: salt size
    pub const SALT_SIZE: usize = 32;

    /// Encryption parameters
    pub const AES_GCM_NONCE_SIZE: usize = 12;
    /// Configuration constant: aes gcm tag size
    pub const AES_GCM_TAG_SIZE: usize = 16;
    /// Nonce length for ChaCha20-Poly1305 (RFC 8439)
    pub const CHACHA20_NONCE_SIZE: usize = 12;
    /// `ChaCha20` key size in bytes - 256-bit key length
    pub const CHACHA20_KEY_SIZE: usize = 32;

    /// Algorithm identifiers
    pub const AES_256_GCM: &str = "AES-256-GCM";
    /// ChaCha20-Poly1305 authenticated encryption algorithm identifier
    pub const CHACHA20_POLY1305: &str = "ChaCha20-Poly1305";
    /// Ed25519 signature algorithm identifier - quantum-resistant
    pub const ED25519: &str = "Ed25519";
    /// Configuration constant: rsa oaep
    pub const RSA_OAEP: &str = "RSA-OAEP";
    /// Configuration constant: rsa pss
    pub const RSA_PSS: &str = "RSA-PSS";
    /// ECDSA P-256 signature algorithm identifier
    pub const ECDSA_P256: &str = "ECDSA-P256";
    /// ECDSA P-384 signature algorithm identifier - higher security curve
    pub const ECDSA_P384: &str = "ECDSA-P384";
}

/// **HSM CONSTANTS** - Hardware Security Module configuration
pub mod hsm {
    use super::Duration;

    /// HSM connection settings
    pub const HSM_CONNECTION_TIMEOUT: Duration = Duration::from_secs(30);
    /// Configuration constant: hsm operation timeout
    pub const HSM_OPERATION_TIMEOUT: Duration = Duration::from_secs(60);
    /// Configuration constant: hsm keepalive interval
    pub const HSM_KEEPALIVE_INTERVAL: Duration = Duration::from_secs(30);
    /// Configuration constant: hsm health check interval
    pub const HSM_HEALTH_CHECK_INTERVAL: Duration = Duration::from_secs(60);

    /// HSM limits
    pub const MAX_HSM_CONNECTIONS: usize = 10;
    /// Configuration constant: max hsm operations per second
    pub const MAX_HSM_OPERATIONS_PER_SECOND: u32 = 1000;
    /// Configuration constant: max key label length
    pub const MAX_KEY_LABEL_LENGTH: usize = 255;
    /// Configuration constant: max hsm session count
    pub const MAX_HSM_SESSION_COUNT: usize = 100;

    /// HSM key management
    pub const KEY_GENERATION_TIMEOUT: Duration = Duration::from_secs(300);
    /// Configuration constant: key rotation interval
    pub const KEY_ROTATION_INTERVAL: Duration = Duration::from_secs(86400 * 90); // 90 days
    /// Configuration constant: key backup interval
    pub const KEY_BACKUP_INTERVAL: Duration = Duration::from_secs(86400); // 24 hours

    /// HSM device types
    pub const SOFTWARE_HSM: &str = "software";
    /// Configuration constant: hardware hsm
    pub const HARDWARE_HSM: &str = "hardware";
    /// Configuration constant: cloud hsm
    pub const CLOUD_HSM: &str = "cloud";
    /// Configuration constant: network hsm
    pub const NETWORK_HSM: &str = "network";

    /// PKCS#11 constants
    pub const PKCS11_USER_PIN_MIN_LENGTH: usize = 4;
    /// PKCS#11 maximum user PIN length - standard specification limit
    pub const PKCS11_USER_PIN_MAX_LENGTH: usize = 255;
    /// PKCS#11 minimum Security Officer PIN length
    pub const PKCS11_SO_PIN_MIN_LENGTH: usize = 4;
    /// PKCS#11 maximum Security Officer PIN length - standard specification limit
    pub const PKCS11_SO_PIN_MAX_LENGTH: usize = 255;
    /// Maximum number of concurrent PKCS#11 sessions
    pub const PKCS11_MAX_SESSION_COUNT: usize = 64;

    /// Default batch size for batched HSM operations
    pub const HSM_BATCH_SIZE: usize = 100;
    /// Configuration constant: hsm queue size
    pub const HSM_QUEUE_SIZE: usize = 1000;
    /// Configuration constant: hsm worker threads
    pub const HSM_WORKER_THREADS: usize = 4;
    /// Configuration constant: hsm retry attempts
    pub const HSM_RETRY_ATTEMPTS: u32 = 3;
    /// Configuration constant: hsm retry delay
    pub const HSM_RETRY_DELAY: Duration = Duration::from_millis(100);
}

/// **TLS/SSL CONSTANTS** - Transport Layer Security settings
pub mod tls {
    use super::Duration;

    /// TLS version 1.2 identifier
    pub const TLS_1_2: &str = "TLSv1.2";
    /// TLS version 1.3 identifier (recommended)
    pub const TLS_1_3: &str = "TLSv1.3";
    /// Configuration constant: default tls version
    pub const DEFAULT_TLS_VERSION: &str = TLS_1_3;
    /// Configuration constant: min tls version
    pub const MIN_TLS_VERSION: &str = TLS_1_2;

    /// TLS timeouts
    pub const TLS_HANDSHAKE_TIMEOUT: Duration = Duration::from_secs(30);
    /// Configuration constant: tls session timeout
    pub const TLS_SESSION_TIMEOUT: Duration = Duration::from_secs(86400); // 24 hours
    /// Configuration constant: tls ticket lifetime
    pub const TLS_TICKET_LIFETIME: Duration = Duration::from_secs(86400); // 24 hours

    /// Certificate settings
    pub const CERT_VALIDITY_PERIOD: Duration = Duration::from_secs(86400 * 365); // 1 year
    /// Configuration constant: cert renewal threshold
    pub const CERT_RENEWAL_THRESHOLD: Duration = Duration::from_secs(86400 * 30); // 30 days
    /// Configuration constant: max cert chain length
    pub const MAX_CERT_CHAIN_LENGTH: usize = 10;
    /// Configuration constant: cert key size
    pub const CERT_KEY_SIZE: usize = 2048;

    /// Cipher suites (preference order)
    pub const PREFERRED_CIPHER_SUITES: &[&str] = &[
        "TLS_AES_256_GCM_SHA384",
        "TLS_CHACHA20_POLY1305_SHA256",
        "TLS_AES_128_GCM_SHA256",
        "TLS_ECDHE_RSA_WITH_AES_256_GCM_SHA384",
        "TLS_ECDHE_RSA_WITH_CHACHA20_POLY1305_SHA256",
        "TLS_ECDHE_RSA_WITH_AES_128_GCM_SHA256",
    ];

    /// OCSP settings
    pub const OCSP_RESPONSE_TIMEOUT: Duration = Duration::from_secs(10);
    /// Configuration constant: ocsp cache duration
    pub const OCSP_CACHE_DURATION: Duration = Duration::from_secs(3600); // 1 hour
    /// Configuration constant: ocsp max response size
    pub const OCSP_MAX_RESPONSE_SIZE: usize = 1024 * 1024; // 1MB

    /// Certificate transparency
    pub const CT_LOG_TIMEOUT: Duration = Duration::from_secs(30);
    /// Configuration constant: ct max response size
    pub const CT_MAX_RESPONSE_SIZE: usize = 10 * 1024 * 1024; // 10MB
    /// Configuration constant: ct required scts
    pub const CT_REQUIRED_SCTS: usize = 2;
}

/// **ACCESS CONTROL CONSTANTS** - Access control and authorization
pub mod access_control {
    use super::Duration;

    /// Permission levels
    pub const PERMISSION_READ: &str = "read";
    /// Configuration constant: permission write
    pub const PERMISSION_WRITE: &str = "write";
    /// Configuration constant: permission delete
    pub const PERMISSION_DELETE: &str = "delete";
    /// Configuration constant: permission admin
    pub const PERMISSION_ADMIN: &str = "admin";
    /// Configuration constant: permission super admin
    pub const PERMISSION_SUPER_ADMIN: &str = "super_admin";

    /// Role hierarchies
    pub const ROLE_USER: &str = "user";
    /// Configuration constant: role operator
    pub const ROLE_OPERATOR: &str = "operator";
    /// Configuration constant: role admin
    pub const ROLE_ADMIN: &str = "admin";
    /// Configuration constant: role super admin
    pub const ROLE_SUPER_ADMIN: &str = "super_admin";
    /// Configuration constant: role system
    pub const ROLE_SYSTEM: &str = "system";

    /// Access control limits
    pub const MAX_ROLES_PER_USER: usize = 10;
    /// Configuration constant: max permissions per role
    pub const MAX_PERMISSIONS_PER_ROLE: usize = 100;
    /// Configuration constant: max resource name length
    pub const MAX_RESOURCE_NAME_LENGTH: usize = 255;
    /// Configuration constant: max role name length
    pub const MAX_ROLE_NAME_LENGTH: usize = 64;

    /// Policy evaluation
    pub const POLICY_EVALUATION_TIMEOUT: Duration = Duration::from_millis(100);
    /// Configuration constant: max policy size
    pub const MAX_POLICY_SIZE: usize = 1024 * 1024; // 1MB
    /// Configuration constant: max policy rules
    pub const MAX_POLICY_RULES: usize = 1000;
    /// Configuration constant: policy cache ttl
    pub const POLICY_CACHE_TTL: Duration = Duration::from_secs(300); // 5 minutes

    /// Capability-based access
    pub const CAPABILITY_TOKEN_SIZE: usize = 32;
    /// Configuration constant: capability lifetime
    pub const CAPABILITY_LIFETIME: Duration = Duration::from_secs(3600); // 1 hour
    /// Configuration constant: max capabilities per token
    pub const MAX_CAPABILITIES_PER_TOKEN: usize = 50;
}

/// **AUDIT CONSTANTS** - Security auditing and logging
pub mod audit {
    use super::Duration;

    /// Audit event types
    pub const EVENT_LOGIN: &str = "login";
    /// Configuration constant: event logout
    pub const EVENT_LOGOUT: &str = "logout";
    /// Configuration constant: event permission grant
    pub const EVENT_PERMISSION_GRANT: &str = "permission_grant";
    /// Configuration constant: event permission revoke
    pub const EVENT_PERMISSION_REVOKE: &str = "permission_revoke";
    /// Configuration constant: event key generation
    pub const EVENT_KEY_GENERATION: &str = "key_generation";
    /// Configuration constant: event key deletion
    pub const EVENT_KEY_DELETION: &str = "key_deletion";
    /// Configuration constant: event encryption
    pub const EVENT_ENCRYPTION: &str = "encryption ";
    /// Configuration constant: event decryption
    pub const EVENT_DECRYPTION: &str = "decryption";
    /// Configuration constant: event signature
    pub const EVENT_SIGNATURE: &str = "signature";
    /// Configuration constant: event verification
    pub const EVENT_VERIFICATION: &str = "verification";

    /// Audit log settings
    pub const AUDIT_LOG_RETENTION: Duration = Duration::from_secs(86400 * 365 * 7); // 7 years
    /// Configuration constant: audit log max size
    pub const AUDIT_LOG_MAX_SIZE: u64 = 1024 * 1024 * 1024; // 1GB
    /// Configuration constant: audit log rotation interval
    pub const AUDIT_LOG_ROTATION_INTERVAL: Duration = Duration::from_secs(86400); // 24 hours
    /// Configuration constant: max audit entry size
    pub const MAX_AUDIT_ENTRY_SIZE: usize = 64 * 1024; // 64KB

    /// Audit batch processing
    pub const AUDIT_BATCH_SIZE: usize = 1000;
    /// Configuration constant: audit flush interval
    pub const AUDIT_FLUSH_INTERVAL: Duration = Duration::from_secs(60);
    /// Configuration constant: audit queue size
    pub const AUDIT_QUEUE_SIZE: usize = 10000;

    /// Compliance requirements
    pub const GDPR_DATA_RETENTION: Duration = Duration::from_secs(86400 * 365 * 3); // 3 years
    /// Configuration constant: hipaa audit retention
    pub const HIPAA_AUDIT_RETENTION: Duration = Duration::from_secs(86400 * 365 * 6); // 6 years
    /// Configuration constant: sox audit retention
    pub const SOX_AUDIT_RETENTION: Duration = Duration::from_secs(86400 * 365 * 7); // 7 years
    /// Configuration constant: pci log retention
    pub const PCI_LOG_RETENTION: Duration = Duration::from_secs(86400 * 365); // 1 year
}

/// **THREAT DETECTION CONSTANTS** - Security monitoring and threat detection
pub mod threat_detection {
    use super::Duration;

    /// Anomaly detection thresholds
    pub const ANOMALY_SCORE_THRESHOLD: f64 = 0.8;
    /// Configuration constant: failed login threshold
    pub const FAILED_LOGIN_THRESHOLD: u32 = 5;
    /// Configuration constant: suspicious activity threshold
    pub const SUSPICIOUS_ACTIVITY_THRESHOLD: u32 = 10;
    /// Configuration constant: rate limit violation threshold
    pub const RATE_LIMIT_VIOLATION_THRESHOLD: u32 = 3;

    /// Detection timeframes
    pub const DETECTION_WINDOW: Duration = Duration::from_secs(300); // 5 minutes
    /// Configuration constant: analysis interval
    pub const ANALYSIS_INTERVAL: Duration = Duration::from_secs(60); // 1 minute
    /// Configuration constant: alert cooldown
    pub const ALERT_COOLDOWN: Duration = Duration::from_secs(300); // 5 minutes

    /// Machine learning parameters
    pub const ML_MODEL_UPDATE_INTERVAL: Duration = Duration::from_secs(3600); // 1 hour
    /// Configuration constant: ml training data retention
    pub const ML_TRAINING_DATA_RETENTION: Duration = Duration::from_secs(86400 * 30); // 30 days
    /// Configuration constant: ml confidence threshold
    pub const ML_CONFIDENCE_THRESHOLD: f64 = 0.85;
    /// Configuration constant: ml feature count
    pub const ML_FEATURE_COUNT: usize = 50;

    /// Threat intelligence
    pub const IOC_UPDATE_INTERVAL: Duration = Duration::from_secs(3600); // 1 hour
    /// Configuration constant: ioc cache size
    pub const IOC_CACHE_SIZE: usize = 100_000;
    /// Configuration constant: ioc retention period
    pub const IOC_RETENTION_PERIOD: Duration = Duration::from_secs(86400 * 90); // 90 days

    /// Response actions
    pub const AUTO_BLOCK_THRESHOLD: f64 = 0.95;
    /// Configuration constant: quarantine duration
    pub const QUARANTINE_DURATION: Duration = Duration::from_secs(86400); // 24 hours
    /// Configuration constant: incident escalation delay
    pub const INCIDENT_ESCALATION_DELAY: Duration = Duration::from_secs(300); // 5 minutes
}

/// **ENCRYPTION CONSTANTS** - Data encryption and protection
pub mod encryption {
    /// Data classification levels
    pub const CLASSIFICATION_PUBLIC: &str = "public";
    /// Configuration constant: classification internal
    pub const CLASSIFICATION_INTERNAL: &str = "internal";
    /// Configuration constant: classification confidential
    pub const CLASSIFICATION_CONFIDENTIAL: &str = "confidential";
    /// Configuration constant: classification secret
    pub const CLASSIFICATION_SECRET: &str = "secret";
    /// Configuration constant: classification top secret
    pub const CLASSIFICATION_TOP_SECRET: &str = "top_secret";

    /// Encryption requirements by classification
    pub const PUBLIC_ENCRYPTION_REQUIRED: bool = false;
    /// Configuration constant: internal encryption required
    pub const INTERNAL_ENCRYPTION_REQUIRED: bool = true;
    /// Configuration constant: confidential encryption required
    pub const CONFIDENTIAL_ENCRYPTION_REQUIRED: bool = true;
    /// Configuration constant: secret encryption required
    pub const SECRET_ENCRYPTION_REQUIRED: bool = true;
    /// Configuration constant: top secret encryption required
    pub const TOP_SECRET_ENCRYPTION_REQUIRED: bool = true;

    /// Key derivation
    pub const KDF_ITERATIONS: u32 = 100_000;
    /// Configuration constant: kdf salt size
    pub const KDF_SALT_SIZE: usize = 32;
    /// Configuration constant: kdf output size
    pub const KDF_OUTPUT_SIZE: usize = 32;

    /// Envelope encryption
    pub const DEK_SIZE: usize = 32; // Data Encryption Key
    /// Configuration constant: kek size
    pub const KEK_SIZE: usize = 32; // Key Encryption Key
    /// Configuration constant: envelope header size
    pub const ENVELOPE_HEADER_SIZE: usize = 256;

    /// Field-level encryption
    pub const FIELD_ENCRYPTION_KEY_SIZE: usize = 32;
    /// Configuration constant: field encryption nonce size
    pub const FIELD_ENCRYPTION_NONCE_SIZE: usize = 12;
    /// Configuration constant: max encrypted field size
    pub const MAX_ENCRYPTED_FIELD_SIZE: usize = 1024 * 1024; // 1MB
}

/// **COMPLIANCE CONSTANTS** - Regulatory compliance settings
pub mod compliance {
    use super::Duration;

    /// GDPR requirements
    pub const GDPR_DATA_RETENTION_MAX: Duration = Duration::from_secs(86400 * 365 * 3); // 3 years
    /// Configuration constant: gdpr breach notification time
    pub const GDPR_BREACH_NOTIFICATION_TIME: Duration = Duration::from_secs(3600 * 72); // 72 hours
    /// Configuration constant: gdpr consent expiry
    pub const GDPR_CONSENT_EXPIRY: Duration = Duration::from_secs(86400 * 365); // 1 year

    /// HIPAA requirements
    pub const HIPAA_AUDIT_LOG_RETENTION: Duration = Duration::from_secs(86400 * 365 * 6); // 6 years
    /// Configuration constant: hipaa encryption required
    pub const HIPAA_ENCRYPTION_REQUIRED: bool = true;
    /// Configuration constant: hipaa access log detail level
    pub const HIPAA_ACCESS_LOG_DETAIL_LEVEL: &str = "detailed";

    /// SOX requirements
    pub const SOX_CONTROL_TESTING_FREQUENCY: Duration = Duration::from_secs(86400 * 90); // 90 days
    /// Configuration constant: sox audit trail retention
    pub const SOX_AUDIT_TRAIL_RETENTION: Duration = Duration::from_secs(86400 * 365 * 7); // 7 years
    /// Configuration constant: sox segregation of duties
    pub const SOX_SEGREGATION_OF_DUTIES: bool = true;

    /// PCI DSS requirements
    pub const PCI_KEY_ROTATION_FREQUENCY: Duration = Duration::from_secs(86400 * 365); // 1 year
    /// Configuration constant: pci log monitoring frequency
    pub const PCI_LOG_MONITORING_FREQUENCY: Duration = Duration::from_secs(86400); // 24 hours
    /// Configuration constant: pci vulnerability scan frequency
    pub const PCI_VULNERABILITY_SCAN_FREQUENCY: Duration = Duration::from_secs(86400 * 90); // 90 days
    /// Configuration constant: pci min key length
    pub const PCI_MIN_KEY_LENGTH: usize = 128; // bits

    /// ISO 27001 requirements
    pub const ISO27001_RISK_ASSESSMENT_FREQUENCY: Duration = Duration::from_secs(86400 * 365); // 1 year
    /// ISO 27001 security review frequency (90 days)
    pub const ISO27001_SECURITY_REVIEW_FREQUENCY: Duration = Duration::from_secs(86400 * 90); // 90 days
    /// ISO 27001 maximum incident response time (4 hours)
    pub const ISO27001_INCIDENT_RESPONSE_TIME: Duration = Duration::from_secs(3600 * 4);
}

/// **SECURITY LIMITS** - Security-related limits and thresholds
pub mod limits {
    use std::time::Duration;
    /// Password requirements
    pub const MIN_PASSWORD_LENGTH: usize = 12;
    /// Configuration constant: max password length
    pub const MAX_PASSWORD_LENGTH: usize = 128;
    /// Configuration constant: password history count
    pub const PASSWORD_HISTORY_COUNT: usize = 12;
    /// Configuration constant: password complexity score
    pub const PASSWORD_COMPLEXITY_SCORE: u32 = 3;

    /// Session limits
    pub const MAX_CONCURRENT_SESSIONS: u32 = 10;
    /// Configuration constant: max session duration
    pub const MAX_SESSION_DURATION: Duration = Duration::from_secs(86400); // 24 hours
    /// Configuration constant: session idle timeout
    pub const SESSION_IDLE_TIMEOUT: Duration = Duration::from_secs(1800); // 30 minutes

    /// Authentication limits
    pub const MAX_LOGIN_ATTEMPTS: u32 = 5;
    /// Configuration constant: login lockout duration
    pub const LOGIN_LOCKOUT_DURATION: Duration = Duration::from_secs(900); // 15 minutes
    /// Configuration constant: max failed mfa attempts
    pub const MAX_FAILED_MFA_ATTEMPTS: u32 = 3;
    /// Configuration constant: mfa lockout duration
    pub const MFA_LOCKOUT_DURATION: Duration = Duration::from_secs(1800); // 30 minutes

    /// API security limits
    pub const MAX_API_KEYS_PER_USER: usize = 10;
    /// Configuration constant: api rate limit per minute
    pub const API_RATE_LIMIT_PER_MINUTE: u32 = 1000;
    /// Configuration constant: max request size
    pub const MAX_REQUEST_SIZE: usize = 10 * 1024 * 1024; // 10MB
    /// Configuration constant: max response size
    pub const MAX_RESPONSE_SIZE: usize = 100 * 1024 * 1024; // 100MB

    /// Cryptographic limits
    pub const MAX_KEY_SIZE: usize = 4096; // bits
    /// Configuration constant: min key size
    pub const MIN_KEY_SIZE: usize = 2048; // bits
    /// Configuration constant: max signature size
    pub const MAX_SIGNATURE_SIZE: usize = 1024; // bytes
    /// Configuration constant: max encrypted data size
    pub const MAX_ENCRYPTED_DATA_SIZE: usize = 100 * 1024 * 1024; // 100MB

    /// Certificate limits
    pub const MAX_CERT_VALIDITY_DAYS: u32 = 365;
    /// Configuration constant: min cert validity days
    pub const MIN_CERT_VALIDITY_DAYS: u32 = 1;
    /// Configuration constant: max cert chain depth
    pub const MAX_CERT_CHAIN_DEPTH: usize = 10;
    /// Configuration constant: max cert size
    pub const MAX_CERT_SIZE: usize = 16 * 1024; // 16KB
}

// Re-export commonly used constants for convenience
pub use auth::{MAX_AUTH_ATTEMPTS, MIN_PASSWORD_LENGTH, SESSION_TIMEOUT};
pub use crypto::{AES_256_GCM, DEFAULT_KEY_LENGTH};
pub use hsm::{HSM_CONNECTION_TIMEOUT, MAX_HSM_CONNECTIONS};

// **SECURITY CONSTANTS MIGRATION COMPLETE** - All constants now organized in domain modules
