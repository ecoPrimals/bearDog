pub mod api {
    pub const VERSION: &str = "v1";
    pub const VERSION_HEADER: &str = "X-BearDog-API-Version";
    pub const PROJECT_VERSION: &str = env!("CARGO_PKG_VERSION");
    pub const PROJECT_NAME: &str = "BearDog";
    pub const MISSION: &str = "Democratizing enterprise-grade security for everyone";
    pub const DEFAULT_TIMEOUT_MS: u64 = 30000;
    pub const MAX_REQUEST_SIZE: usize = 1024 * 1024; // 1MB
}

// Network limits constants
pub mod network {

    pub mod ports {
        pub const API: u16 = 8080;
        pub const HTTPS: u16 = 8443;
        pub const GRPC: u16 = 9090;
        pub const METRICS: u16 = 9091;
        pub const HEALTH: u16 = 8081;
    }

    pub mod endpoints {
        pub const SONGBIRD_ENDPOINT: &str = "https://songbird.ecoprimals.com";
        pub const NESTGATE_ENDPOINT: &str = "https://nestgate.ecoprimals.com";
        pub const TOADSTOOL_ENDPOINT: &str = "https://toadstool.ecoprimals.com";
    }

    pub mod limits {
        pub const MAX_CONNECTIONS: usize = 1000;
        pub const CONNECTION_POOL_SIZE: usize = 100;
        pub const CONNECTION_TIMEOUT_MS: u64 = 30000;
        pub const REQUEST_TIMEOUT_MS: u64 = 60000;
        pub const MAX_RETRIES: u32 = 3;
        pub const BACKOFF_BASE_MS: u64 = 1000;
        pub const STANDARD_RATE_LIMIT: u32 = 100;
    }

    pub mod addresses {
        pub const DEFAULT_BIND: &str = "127.0.0.1";
        pub const LOCALHOST: &str = "127.0.0.1";
        pub const DEFAULT_HOST: &str = "127.0.0.1"; // Alias for compatibility
    }

    pub mod timeouts {
        use std::time::Duration;
        pub const CONNECTION: Duration = Duration::from_secs(30);
        pub const OPERATION: Duration = Duration::from_secs(30);
        pub const CRYPTO_OPERATION: Duration = Duration::from_secs(5);
        pub const NETWORK_OPERATION: Duration = Duration::from_secs(10);
        pub const HSM_OPERATION: Duration = Duration::from_secs(30);
        pub const KEY_ROTATION: Duration = Duration::from_secs(30);
        pub const DEFAULT_CONNECTION_TIMEOUT: Duration = Duration::from_secs(30);
        pub const DEFAULT_IDLE_TIMEOUT: Duration = Duration::from_secs(300);
    }

    pub const PRIVATE_IP_RANGES: &[&str] = &[
        "10.0.0.0/8",
        "172.16.0.0/12",
        "192.168.0.0/16",
        "127.0.0.0/8",
        "169.254.0.0/16",
        "::1/128",
        "fc00::/7",
        "fe80::/10",
    ];

    pub const DEFAULT_API_ENDPOINT: &str = "https://api.beardog.local";
}

pub mod cache {

    pub const SMALL_SIZE: usize = 1_000;
    pub const STANDARD_SIZE: usize = 10_000;
    pub const LARGE_SIZE: usize = 100_000;

    pub mod ttl {
        use std::time::Duration;

        pub const API_RESPONSE: Duration = Duration::from_secs(300); // 5 minutes
        pub const USER_SESSION: Duration = Duration::from_secs(3600); // 1 hour
        pub const THREAT_ANALYSIS: Duration = Duration::from_secs(600); // 10 minutes
        pub const COMPLIANCE_REPORT: Duration = Duration::from_secs(1800); // 30 minutes
        pub const NODE_STATUS: Duration = Duration::from_secs(120); // 2 minutes
        pub const CONFIG_DATA: Duration = Duration::from_secs(3600); // 1 hour
        pub const STATIC_CONTENT: Duration = Duration::from_secs(86400); // 24 hours
        pub const STANDARD: Duration = Duration::from_secs(300); // Default 5 minutes
    }
}

pub mod security {
    use std::time::Duration;

    pub const MAX_AUTH_ATTEMPTS: u32 = 5;
    pub const MAX_SESSIONS: u32 = 5;
    pub const MAX_OPERATION_ATTEMPTS: u32 = 3;

    pub const STANDARD_KEY_SIZE: u32 = 256;
    pub const STANDARD_KEY_SIZE_LIMIT: usize = 4096;
    pub const MIN_ENTROPY_BITS: u32 = 256;
    pub const AES_BLOCK_SIZE: usize = 16;

    pub const SESSION_TIMEOUT: Duration = Duration::from_secs(3600); // 1 hour
    pub const LOCKOUT_DURATION: Duration = Duration::from_secs(900); // 15 minutes
    pub const TOKEN_REFRESH: Duration = Duration::from_secs(900); // 15 minutes

    pub const SECURE_CHARSET: &[u8] =
        b"ABCDEFGHIJKLMNOPQRSTUVWXYZabcdefghijklmnopqrstuvwxyz0123456789";
    pub const TEST_CHARSET: &[u8] = b"ABCDEFGHIJKLMNOPQRSTUVWXYZ0123456789";
}

// Node configuration constants
pub mod nodes {
    use std::time::Duration;

    pub const SECURITY: &str = "security_node";
    pub const PHONEBOOK: &str = "phonebook_node";
    pub const FEDERATION: &str = "federation_node";
    pub const DISCOVERY: &str = "discovery_node";
    pub const REGISTRY: &str = "registry";
    pub const MONITORING: &str = "monitoring";
    pub const API_GATEWAY: &str = "api_gateway";
    pub const LOAD_BALANCER: &str = "load_balancer";
    pub const DATABASE: &str = "database";
    pub const CACHE: &str = "cache";
    pub const AUTHENTICATION: &str = "authentication";

    pub const MAX_NODE_METADATA_SIZE: usize = 4096; // 4KB
    pub const NODE_REGISTRATION_TIMEOUT: Duration = Duration::from_secs(30);
    pub const MAX_CONCURRENT_NODE_OPERATIONS: usize = 100;

    pub mod status {
        pub const STATUS_ACTIVE: &str = "active";
        pub const STATUS_INACTIVE: &str = "inactive";
        pub const STATUS_STARTING: &str = "starting";
        pub const STATUS_STOPPING: &str = "stopping";
        pub const STATUS_MAINTENANCE: &str = "maintenance";
        pub const STATUS_ERROR: &str = "error";
        pub const STATUS_UNKNOWN: &str = "unknown";

        pub const AVAILABILITY_AVAILABLE: &str = "available";
        pub const AVAILABILITY_BUSY: &str = "busy";
        pub const AVAILABILITY_OVERLOADED: &str = "overloaded";
        pub const AVAILABILITY_UNREACHABLE: &str = "unreachable";

        pub const HEALTH_HEALTHY: &str = "healthy";
        pub const HEALTH_DEGRADED: &str = "degraded";
        pub const HEALTH_UNHEALTHY: &str = "unhealthy";
        pub const HEALTH_CRITICAL: &str = "critical";

        pub const ROLE_PRIMARY: &str = "primary";
        pub const ROLE_SECONDARY: &str = "secondary";
        pub const ROLE_REPLICA: &str = "replica";
        pub const ROLE_STANDBY: &str = "standby";
    }

    pub mod federation {
        use std::time::Duration;

        pub const MAX_FEDERATION_SIZE: usize = 1000;
        pub const CONSENSUS_TIMEOUT: Duration = Duration::from_secs(30);
        pub const ELECTION_TIMEOUT: Duration = Duration::from_secs(60);
        pub const MIN_QUORUM_SIZE: usize = 3;
        pub const MAX_CLUSTER_DEPTH: u32 = 10;
        pub const PROTOCOL_VERSION: u32 = 1;
        pub const CROSS_FEDERATION_TIMEOUT: Duration = Duration::from_secs(120); // 2 minutes
        pub const METADATA_SYNC_INTERVAL: Duration = Duration::from_secs(300); // 5 minutes
        pub const MAX_MESSAGE_SIZE: usize = 1024 * 1024; // 1MB
        pub const PEER_DISCOVERY_TIMEOUT: Duration = Duration::from_secs(60);
        pub const MAX_PENDING_REQUESTS: usize = 1000;
    }
}

// Compliance module moved below - consolidated version

pub mod performance {
    pub const LIGHT_ITERATIONS: usize = 100;
    pub const STANDARD_ITERATIONS: usize = 1000;
    pub const HEAVY_ITERATIONS: usize = 10000;

    pub const CONCURRENT_TASKS: usize = 50;
    pub const CONCURRENT_USERS: usize = 1000;
    pub const OPERATIONS_PER_TASK: usize = 100;
    pub const TARGET_RPS: u32 = 1000;
    pub const TEST_DATA_SIZE: usize = 1024;
    pub const LOAD_TEST_DURATION_SECONDS: u64 = 10;
    pub const STANDARD_BATCH_SIZE: usize = 100;

    pub mod retry {
        pub const WORKFLOW_MAX_ATTEMPTS: u32 = 3;
    }

    pub mod batch {
        pub const KEY_ROTATION_BATCH_SIZE: usize = 100;
    }
}

// HSM module moved below - consolidated version

pub mod system {
    use std::time::Duration;

    // Core system limits
    pub const MAX_MEMORY_USAGE_PERCENT: u8 = 80;
    pub const MAX_CPU_USAGE_PERCENT: u8 = 90;
    pub const MAX_DISK_USAGE_PERCENT: u8 = 85;
    pub const HEALTH_CHECK_INTERVAL_SECONDS: u64 = 30;
    pub const STARTUP_TIMEOUT_SECONDS: u64 = 120;
    pub const SHUTDOWN_TIMEOUT_SECONDS: u64 = 60;
    pub const LOG_ROTATION_SIZE_MB: u64 = 100;
    pub const MAX_LOG_FILES: u32 = 10;

    // Environment variable names
    pub mod environment {
        pub const BEARDOG_LOG_LEVEL: &str = "BEARDOG_LOG_LEVEL";
        pub const BEARDOG_CONFIG_PATH: &str = "BEARDOG_CONFIG_PATH";
        pub const BEARDOG_DATA_DIR: &str = "BEARDOG_DATA_DIR";
        pub const BEARDOG_LOG_DIR: &str = "BEARDOG_LOG_DIR";
        pub const BEARDOG_KEYS_DIR: &str = "BEARDOG_KEYS_DIR";

        pub const DATABASE_URL: &str = "DATABASE_URL";
        pub const DATABASE_MAX_CONNECTIONS: &str = "DATABASE_MAX_CONNECTIONS";
        pub const DATABASE_CONNECTION_TIMEOUT: &str = "DATABASE_CONNECTION_TIMEOUT";

        pub const HTTP_PORT: &str = "HTTP_PORT";
        pub const HTTPS_PORT: &str = "HTTPS_PORT";
        pub const GRPC_PORT: &str = "GRPC_PORT";
        pub const BIND_ADDRESS: &str = "BIND_ADDRESS";

        pub const HSM_LIBRARY_PATH: &str = "HSM_LIBRARY_PATH";
        pub const HSM_SLOT_ID: &str = "HSM_SLOT_ID";
        pub const HSM_PIN: &str = "HSM_PIN";
        pub const JWT_SECRET: &str = "JWT_SECRET";
        pub const ENCRYPTION_KEY: &str = "ENCRYPTION_KEY";

        pub const THREAD_POOL_SIZE: &str = "THREAD_POOL_SIZE";
        pub const MAX_CONCURRENT_OPERATIONS: &str = "MAX_CONCURRENT_OPERATIONS";
        pub const CACHE_SIZE: &str = "CACHE_SIZE";

        pub const RUST_LOG: &str = "RUST_LOG";
        pub const RUST_BACKTRACE: &str = "RUST_BACKTRACE";
        pub const BEARDOG_DEBUG: &str = "BEARDOG_DEBUG";
        pub const BEARDOG_PROFILE: &str = "BEARDOG_PROFILE";
    }

    // Feature flags
    pub mod features {
        pub const ENABLE_HSM: &str = "enable_hsm";
        pub const ENABLE_BIOMETRICS: &str = "enable_biometrics";
        pub const ENABLE_CLOUD_SYNC: &str = "enable_cloud_sync";
        pub const ENABLE_FEDERATION: &str = "enable_federation";

        pub const ENABLE_MFA: &str = "enable_mfa";
        pub const ENABLE_AUDIT_LOGGING: &str = "enable_audit_logging";
        pub const ENABLE_KEY_ROTATION: &str = "enable_key_rotation";
        pub const ENABLE_SECURE_BOOT: &str = "enable_secure_boot";

        pub const ENABLE_SIMD: &str = "enable_simd";
        pub const ENABLE_ASYNC_CRYPTO: &str = "enable_async_crypto";
        pub const ENABLE_ZERO_COPY: &str = "enable_zero_copy";
        pub const ENABLE_BATCH_PROCESSING: &str = "enable_batch_processing";

        pub const ENABLE_DEBUG_MODE: &str = "enable_debug_mode";
        pub const ENABLE_PROFILING: &str = "enable_profiling";
        pub const ENABLE_METRICS: &str = "enable_metrics";
        pub const ENABLE_TRACING: &str = "enable_tracing";

        pub const ENABLE_QUANTUM_RESISTANCE: &str = "enable_quantum_resistance";
        pub const ENABLE_HOMOMORPHIC_ENCRYPTION: &str = "enable_homomorphic_encryption";
        pub const ENABLE_ZERO_KNOWLEDGE_PROOFS: &str = "enable_zkproofs";
    }

    // Health monitoring
    pub mod health {
        use super::Duration;

        pub const SYSTEM_HEALTH_CHECK_INTERVAL: Duration = Duration::from_secs(30);
        pub const SERVICE_HEALTH_CHECK_INTERVAL: Duration = Duration::from_secs(60);
        pub const DATABASE_HEALTH_CHECK_INTERVAL: Duration = Duration::from_secs(30);
        pub const HSM_HEALTH_CHECK_INTERVAL: Duration = Duration::from_secs(60);

        pub const HEALTH_CHECK_TIMEOUT: Duration = Duration::from_secs(10);
        pub const EXTENDED_HEALTH_CHECK_TIMEOUT: Duration = Duration::from_secs(30);

        pub const STATUS_HEALTHY: &str = "healthy";
        pub const STATUS_DEGRADED: &str = "degraded";
        pub const STATUS_UNHEALTHY: &str = "unhealthy";
        pub const STATUS_UNKNOWN: &str = "unknown";

        pub const CPU_USAGE_WARNING_THRESHOLD: f64 = 70.0;
        pub const CPU_USAGE_CRITICAL_THRESHOLD: f64 = 90.0;
        pub const MEMORY_USAGE_WARNING_THRESHOLD: f64 = 75.0;
        pub const MEMORY_USAGE_CRITICAL_THRESHOLD: f64 = 90.0;
        pub const DISK_USAGE_WARNING_THRESHOLD: f64 = 80.0;
        pub const DISK_USAGE_CRITICAL_THRESHOLD: f64 = 95.0;

        pub const RESPONSE_TIME_WARNING_MS: u64 = 1000;
        pub const RESPONSE_TIME_CRITICAL_MS: u64 = 5000;

        pub const ERROR_RATE_WARNING_PERCENT: f64 = 5.0;
        pub const ERROR_RATE_CRITICAL_PERCENT: f64 = 10.0;
    }

    // Monitoring and metrics
    pub mod monitoring {
        use super::Duration;

        pub const METRICS_COLLECTION_INTERVAL: Duration = Duration::from_secs(60);
        pub const PERFORMANCE_SAMPLING_INTERVAL: Duration = Duration::from_secs(5);
        pub const LOG_ROTATION_INTERVAL: Duration = Duration::from_secs(3600); // 1 hour

        pub const METRICS_RETENTION_PERIOD: Duration = Duration::from_secs(86400 * 30); // 30 days
        pub const PERFORMANCE_DATA_RETENTION: Duration = Duration::from_secs(86400 * 7); // 7 days
        pub const AUDIT_LOG_RETENTION_PERIOD: Duration = Duration::from_secs(86400 * 365); // 1 year

        pub const METRICS_BUFFER_SIZE: usize = 10000;
        pub const LOG_BUFFER_SIZE: usize = 1000;
        pub const EVENT_BUFFER_SIZE: usize = 5000;

        pub const HIGH_LOAD_THRESHOLD: f64 = 0.8;
        pub const CRITICAL_LOAD_THRESHOLD: f64 = 0.95;
        pub const CONNECTION_POOL_EXHAUSTION_THRESHOLD: f64 = 0.9;

        pub const METRICS_ENDPOINT: &str = "/metrics";
        pub const HEALTH_ENDPOINT: &str = "/health";
        pub const READY_ENDPOINT: &str = "/ready";
        pub const STATUS_ENDPOINT: &str = "/status";
    }
}

// Utility functions for dynamic configuration

#[must_use]
pub fn default_api_host() -> String {
    std::env::var("BEARDOG_API_HOST")
        .unwrap_or_else(|_| network::addresses::DEFAULT_HOST.to_string())
}

pub fn default_api_port() -> u16 {
    std::env::var("BEARDOG_API_PORT")
        .ok()
        .and_then(|p| p.parse().ok())
        .unwrap_or(network::ports::API)
}

pub fn default_api_bind_address() -> String {
    format!("{}:{}", default_api_host(), default_api_port())
}

pub fn default_timeout_ms() -> u64 {
    std::env::var("BEARDOG_TIMEOUT_MS")
        .map(|t| t.parse().unwrap_or(30000))
        .unwrap_or(30000) // 30 seconds default
}

pub fn default_health_check_interval_ms() -> u64 {
    std::env::var("BEARDOG_HEALTH_CHECK_INTERVAL_MS")
        .map(|i| i.parse().unwrap_or(60000))
        .unwrap_or(60000) // 60 seconds default
}

pub fn default_key_rotation_timeout_ms() -> u64 {
    std::env::var("BEARDOG_KEY_ROTATION_TIMEOUT_MS")
        .ok()
        .and_then(|t| t.parse().ok())
        .unwrap_or(30000) // 30 seconds default
}

// HSM Constants - consolidated from hsm.rs
pub mod hsm {
    use std::time::Duration;

    // HSM Operations
    pub const KEY_GENERATION: &str = "KeyGeneration";
    pub const SIGNING: &str = "Signing";
    pub const ENCRYPTION: &str = "Encryption";
    pub const DECRYPTION: &str = "Decryption";
    pub const KEY_ATTESTATION: &str = "KeyAttestation";
    pub const USER_PRESENCE_VALIDATION: &str = "UserPresenceValidation";

    // Capabilities
    pub const COMMUNICATION_MESH_CAPABILITY: &str = "communication_mesh";
    pub const STORAGE_SERVICES_CAPABILITY: &str = "storage_services";
    pub const COMPUTE_ORCHESTRATION_CAPABILITY: &str = "compute_orchestration";
    pub const AI_INTELLIGENCE_CAPABILITY: &str = "ai_intelligence";
    pub const SECURITY_PROVIDER_CAPABILITY: &str = "security_provider";
    pub const SYSTEM_INTEGRATION_CAPABILITY: &str = "system_integration";
    pub const HSM_CAPABILITY: &str = "hardware_security_module";
    pub const KEY_MANAGEMENT_CAPABILITY: &str = "key_management";
    pub const SECURE_ENCLAVE_CAPABILITY: &str = "secure_enclave";

    // Limits
    pub const MAX_HSM_KEYS: usize = 1000;
    pub const MAX_KEY_SIZE: usize = 4096;
    pub const MAX_HSM_AUTH_ATTEMPTS: u32 = 3;

    // Providers
    pub const ZERO_COST_SOFTWARE_HSM_PROVIDER: &str = "ZeroCostSoftwareHSM";
    pub const SOFTWARE_HSM_PROVIDER: &str = "software";
    pub const HARDWARE_HSM_PROVIDER: &str = "hardware";
    pub const MOBILE_HSM_PROVIDER: &str = "mobile";
    pub const CLOUD_HSM_PROVIDER: &str = "cloud";
    pub const NETWORK_HSM_PROVIDER: &str = "network";
    pub const USB_HSM_PROVIDER: &str = "usb";
    pub const TPM_PROVIDER: &str = "tpm";
    pub const SMARTCARD_PROVIDER: &str = "smartcard";

    // Timeouts
    pub const HSM_OPERATION_TIMEOUT: Duration = Duration::from_secs(30);
    pub const HSM_HEALTH_CHECK_INTERVAL: Duration = Duration::from_secs(60);
    pub const KEY_ROTATION_INTERVAL: Duration = Duration::from_secs(86400 * 30); // 30 days
    pub const HSM_CONNECTION_TIMEOUT: Duration = Duration::from_secs(10);
    pub const HSM_DISCOVERY_TIMEOUT: Duration = Duration::from_secs(5);
    pub const HSM_AUTH_TIMEOUT: Duration = Duration::from_secs(300); // 5 minutes

    // Platform-specific constants
    pub mod android {
        pub const VERSION: &str = "2.0.0";
        pub const SUPPORTED_ANDROID_VERSION: u32 = 9; // Minimum Android version for StrongBox
        pub const MAX_KEY_COUNT: usize = 1000;
        pub const MAX_CHALLENGE_SIZE: usize = 1024;
    }

    pub mod ios {
        pub const VERSION: &str = "2.0.0";
        pub const SUPPORTED_IOS_VERSION: &str = "13.0";
        pub const MAX_KEY_COUNT: usize = 1000;
        pub const MAX_CHALLENGE_SIZE: usize = 1024;
    }
}

// Compliance Constants - consolidated from compliance.rs
pub mod compliance {
    use std::time::Duration;

    // Error codes
    pub const AUDIT_RETENTION_EXCEEDED: &str = "AuditRetentionExceeded";
    pub const AUDIT_RETENTION_MSG: &str = "Event timestamp exceeds audit retention period";
    pub const MISSING_CONSENT: &str = "MissingConsent";
    pub const MISSING_CONSENT_MSG: &str = "Data access without explicit consent";
    pub const MISSING_PURPOSE: &str = "MissingPurpose";
    pub const MISSING_PURPOSE_MSG: &str = "Data access purpose not specified";
    pub const ILLEGAL_TRANSFER: &str = "IllegalTransfer";
    pub const ILLEGAL_TRANSFER_MSG: &str = "Data transfer without adequate protection";
    pub const UNAUTHORIZED_FINANCIAL: &str = "UnauthorizedFinancialAccess";
    pub const UNAUTHORIZED_FINANCIAL_MSG: &str =
        "Financial data access without proper authorization";
    pub const UNENCRYPTED_PAYMENT: &str = "UnencryptedPaymentData";
    pub const UNENCRYPTED_PAYMENT_MSG: &str = "Payment data processed without encryption";
    pub const MINIMUM_NECESSARY: &str = "MinimumNecessaryViolation";
    pub const MINIMUM_NECESSARY_MSG: &str = "PHI access not limited to minimum necessary";
    pub const MISSING_AUDIT_LOG: &str = "MissingAuditLog";
    pub const MISSING_AUDIT_LOG_MSG: &str = "PHI access not properly logged";

    // Intervals
    pub const COMPLIANCE_CHECK_INTERVAL: Duration = Duration::from_secs(300); // 5 minutes
    pub const AUDIT_RETENTION_PERIOD: Duration = Duration::from_secs(86400 * 365); // 1 year
    pub const DATA_ACCESS_MONITORING_WINDOW: Duration = Duration::from_secs(86400); // 24 hours
    pub const CONSENT_VALIDATION_PERIOD: Duration = Duration::from_secs(86400 * 30); // 30 days

    pub mod gdpr {
        use super::Duration;
        pub const REQUEST_RESPONSE_TIME: Duration = Duration::from_secs(72 * 3600);
        pub const BREACH_NOTIFICATION_TIME: Duration = Duration::from_secs(72 * 3600);
        pub const ERASURE_PROCESSING_TIME: Duration = Duration::from_secs(30 * 86400);
    }

    pub mod hipaa {
        use super::Duration;
        pub const PHI_AUDIT_RETENTION: Duration = Duration::from_secs(6 * 365 * 86400);
        pub const BREACH_NOTIFICATION_TIME: Duration = Duration::from_secs(60 * 86400);
        pub const RISK_ASSESSMENT_INTERVAL: Duration = Duration::from_secs(365 * 86400);
    }

    pub mod pci_dss {
        use super::Duration;
        pub const VULNERABILITY_SCAN_INTERVAL: Duration = Duration::from_secs(90 * 86400);
        pub const SECURITY_ASSESSMENT_INTERVAL: Duration = Duration::from_secs(365 * 86400);
        pub const LOG_RETENTION_PERIOD: Duration = Duration::from_secs(365 * 86400);
    }

    pub mod violations {
        pub const ILLEGAL_TRANSFER: &str = "ILLEGAL_DATA_TRANSFER";
        pub const UNAUTHORIZED_FINANCIAL: &str = "UNAUTHORIZED_FINANCIAL_ACCESS";
        pub const UNENCRYPTED_PAYMENT: &str = "UNENCRYPTED_PAYMENT_DATA";
        pub const MINIMUM_NECESSARY: &str = "MINIMUM_NECESSARY_VIOLATION";
        pub const MISSING_AUDIT_LOG: &str = "MISSING_AUDIT_LOG";
    }
}

/// Registry of all available constants for introspection
pub struct UnifiedConstantRegistry;

impl UnifiedConstantRegistry {
    #[must_use]
    pub fn constant_domains() -> Vec<&'static str> {
        vec![
            "api",
            "network",
            "cache",
            "security",
            "nodes",
            "compliance",
            "performance",
            "hsm",
            "system",
        ]
    }

    pub fn is_canonical_domain(domain: &str) -> bool {
        Self::constant_domains().contains(&domain)
    }

    pub fn common_constants() -> Vec<(&'static str, &'static str)> {
        vec![
            ("API_VERSION", "Current API version"),
            ("MAX_CONNECTIONS", "Maximum concurrent connections"),
            ("SESSION_TIMEOUT", "Default session timeout"),
            ("STANDARD_KEY_SIZE", "Standard cryptographic key size"),
            ("DEFAULT_HTTP_PORT", "Default HTTP port"),
            ("DEFAULT_HTTPS_PORT", "Default HTTPS port"),
            ("MAX_AUTH_ATTEMPTS", "Maximum authentication attempts"),
            ("STANDARD_RATE_LIMIT", "Standard rate limiting"),
        ]
    }
}
