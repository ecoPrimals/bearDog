//! Configuration constants for BearDog
//!
//! This module contains all hardcoded values extracted into named constants
//! to make them configurable and maintainable.

/// Network configuration constants
pub mod network {
    /// Default SongBird endpoint
    pub const SONGBIRD_ENDPOINT: &str = "https://songbird.beardog.local:8443";
    
    /// Default trusted hostnames
    pub const TRUSTED_HOSTS: &[&str] = &[
        "localhost",
        "127.0.0.1",
        "beardog.local",
        "songbird.beardog.local",
        "nestgate.beardog.local",
        "squirrel.beardog.local",
        "toadstool.beardog.local",
        "biomeos.beardog.local",
    ];
    
    /// Default HTTP port for plain text connections
    pub const DEFAULT_HTTP_PORT: u16 = 8080;
    /// Default HTTPS port for secure connections
    pub const DEFAULT_HTTPS_PORT: u16 = 8443;
    /// Default gRPC port for RPC communications
    pub const DEFAULT_GRPC_PORT: u16 = 9090;
    /// Default metrics port for monitoring endpoints
    pub const DEFAULT_METRICS_PORT: u16 = 9091;
    /// Default admin port for administrative interfaces
    pub const DEFAULT_ADMIN_PORT: u16 = 9092;
    /// Default API port for REST API endpoints
    pub const DEFAULT_API_PORT: u16 = 8080;
    
    /// Default host
    pub const DEFAULT_HOST: &str = "localhost";
    
    /// Default bind address
    pub const DEFAULT_BIND_ADDRESS: &str = "0.0.0.0";
    
    /// Trusted IP ranges
    pub const TRUSTED_IP_RANGES: &[&str] = &[
        "127.0.0.0/8",
        "10.0.0.0/8", 
        "172.16.0.0/12",
        "192.168.0.0/16",
        "::1/128",
        "fc00::/7",
    ];
    
    /// Private IP ranges for threat detection
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
    
    /// CORS origins
    pub const CORS_ORIGINS: &[&str] = &[
        "https://beardog.local",
        "https://songbird.beardog.local",
        "https://nestgate.beardog.local",
        "https://squirrel.beardog.local",
        "https://toadstool.beardog.local",
        "https://biomeos.beardog.local",
    ];
}

/// Service endpoints configuration
pub mod endpoints {
    /// Default SMTP server
    pub const DEFAULT_SMTP_SERVER: &str = "localhost";
    
    /// Default SMTP port
    pub const DEFAULT_SMTP_PORT: u16 = 587;
    
    /// Default localhost URL with port
    pub const DEFAULT_LOCALHOST_URL: &str = "http://localhost:8080";
    
    /// Default webhook URL
    pub const DEFAULT_WEBHOOK_URL: &str = "http://localhost:8080/webhook";
    
    /// Default health URL
    pub const DEFAULT_HEALTH_URL: &str = "http://localhost:8080/health";
    
    /// Default metrics URL
    pub const DEFAULT_METRICS_URL: &str = "http://localhost:8080/metrics";
    
    /// Default admin URL
    pub const DEFAULT_ADMIN_URL: &str = "http://localhost:8080/admin";
    
    /// Default listen address
    pub const DEFAULT_LISTEN_ADDRESS: &str = "localhost:8080";
    
    /// Default API endpoint
    pub const DEFAULT_API_ENDPOINT: &str = "http://localhost:8080";
    
    /// Default SongBird endpoint
    pub const DEFAULT_SONGBIRD_ENDPOINT: &str = "https://songbird.ecosystem.internal";
    
    /// Default NestGate endpoint
    pub const DEFAULT_NESTGATE_ENDPOINT: &str = "https://nestgate.ecosystem.internal";
    
    /// Default Squirrel endpoint
    pub const DEFAULT_SQUIRREL_ENDPOINT: &str = "https://squirrel.ecosystem.internal";
    
    /// Default ToadStool endpoint
    pub const DEFAULT_TOADSTOOL_ENDPOINT: &str = "https://toadstool.ecosystem.internal";
}

/// Security configuration constants
pub mod security {
    use std::time::Duration;
    
    /// Default session timeout
    pub const DEFAULT_SESSION_TIMEOUT: Duration = Duration::from_secs(3600); // 1 hour
    
    /// Default authentication timeout
    pub const DEFAULT_AUTH_TIMEOUT: Duration = Duration::from_secs(300); // 5 minutes
    
    /// Default max failed attempts
    pub const DEFAULT_MAX_FAILED_ATTEMPTS: u32 = 5;
    
    /// Default lockout duration
    pub const DEFAULT_LOCKOUT_DURATION: Duration = Duration::from_secs(1800); // 30 minutes
    
    /// Default token expiry
    pub const DEFAULT_TOKEN_EXPIRY: Duration = Duration::from_secs(3600); // 1 hour
    
    /// Default key rotation interval
    pub const DEFAULT_KEY_ROTATION_INTERVAL: Duration = Duration::from_secs(86400); // 24 hours
    
    /// Default encryption key size
    pub const DEFAULT_ENCRYPTION_KEY_SIZE: usize = 32; // 256 bits
    
    /// Default nonce size
    pub const DEFAULT_NONCE_SIZE: usize = 12; // 96 bits for AES-GCM
    
    /// Default signature size
    pub const DEFAULT_SIGNATURE_SIZE: usize = 64; // Ed25519 signature size
    
    /// Default public key size
    pub const DEFAULT_PUBLIC_KEY_SIZE: usize = 32; // Ed25519 public key size
    
    /// Default private key size
    pub const DEFAULT_PRIVATE_KEY_SIZE: usize = 32; // Ed25519 private key size
    
    /// Default hash size
    pub const DEFAULT_HASH_SIZE: usize = 32; // SHA-256 hash size
}

/// Performance configuration constants
pub mod performance {
    use std::time::Duration;
    
    /// Default request timeout
    pub const DEFAULT_REQUEST_TIMEOUT: Duration = Duration::from_secs(30);
    
    /// Default connection timeout
    pub const DEFAULT_CONNECTION_TIMEOUT: Duration = Duration::from_secs(10);
    
    /// Default keepalive timeout
    pub const DEFAULT_KEEPALIVE_TIMEOUT: Duration = Duration::from_secs(75);
    
    /// Default max connections
    pub const DEFAULT_MAX_CONNECTIONS: u32 = 100;
    
    /// Default max request size
    pub const DEFAULT_MAX_REQUEST_SIZE: usize = 1024 * 1024; // 1MB
    
    /// Default max response size
    pub const DEFAULT_MAX_RESPONSE_SIZE: usize = 1024 * 1024; // 1MB
    
    /// Default worker threads
    pub const DEFAULT_WORKER_THREADS: usize = 4;
    
    /// Default queue size
    pub const DEFAULT_QUEUE_SIZE: usize = 1000;
    
    /// Default buffer size
    pub const DEFAULT_BUFFER_SIZE: usize = 4096;
    
    /// Default batch size
    pub const DEFAULT_BATCH_SIZE: usize = 100;
    
    /// Default rate limit per minute
    pub const DEFAULT_RATE_LIMIT_PER_MINUTE: u32 = 60;
    
    /// Default rate limit window
    pub const DEFAULT_RATE_LIMIT_WINDOW: Duration = Duration::from_secs(60);
    
    /// Default cache size
    pub const DEFAULT_CACHE_SIZE: usize = 1000;
    
    /// Default response time threshold in milliseconds
    pub const DEFAULT_RESPONSE_TIME_THRESHOLD: u64 = 1000;
}

/// Database configuration constants
pub mod database {
    use std::time::Duration;
    
    /// Default database URL
    pub const DEFAULT_DATABASE_URL: &str = "sqlite://beardog.db";
    
    /// Default max connections
    pub const DEFAULT_MAX_CONNECTIONS: u32 = 10;
    
    /// Default connection timeout
    pub const DEFAULT_CONNECTION_TIMEOUT: Duration = Duration::from_secs(30);
    
    /// Default idle timeout
    pub const DEFAULT_IDLE_TIMEOUT: Duration = Duration::from_secs(300); // 5 minutes
    
    /// Default max lifetime
    pub const DEFAULT_MAX_LIFETIME: Duration = Duration::from_secs(3600); // 1 hour
}

/// Health check configuration constants
pub mod health {
    use std::time::Duration;
    
    /// Default health check interval
    pub const DEFAULT_HEALTH_CHECK_INTERVAL: Duration = Duration::from_secs(30);
    
    /// Default health check timeout
    pub const DEFAULT_HEALTH_CHECK_TIMEOUT: Duration = Duration::from_secs(5);
    
    /// Default failure threshold
    pub const DEFAULT_FAILURE_THRESHOLD: u32 = 3;
    
    /// Default recovery threshold
    pub const DEFAULT_RECOVERY_THRESHOLD: u32 = 2;
    
    /// Default heartbeat interval
    pub const DEFAULT_HEARTBEAT_INTERVAL: Duration = Duration::from_secs(60);
}

/// Monitoring configuration constants
pub mod monitoring {
    use std::time::Duration;
    
    /// Default metrics collection interval
    pub const DEFAULT_METRICS_INTERVAL: Duration = Duration::from_secs(60);
    
    /// Default metrics retention
    pub const DEFAULT_METRICS_RETENTION: Duration = Duration::from_secs(86400); // 24 hours
    
    /// Default log level
    pub const DEFAULT_LOG_LEVEL: &str = "info";
    
    /// Default log format
    pub const DEFAULT_LOG_FORMAT: &str = "json";
    
    /// Default max log files
    pub const DEFAULT_MAX_LOG_FILES: usize = 10;
    
    /// Default max log file size
    pub const DEFAULT_MAX_LOG_FILE_SIZE: usize = 1024 * 1024 * 10; // 10MB
}

/// HSM configuration constants
pub mod hsm {
    /// Default HSM type
    pub const DEFAULT_HSM_TYPE: &str = "software";
    
    /// Default HSM library path
    pub const DEFAULT_HSM_LIBRARY_PATH: &str = "/usr/lib/pkcs11/opensc-pkcs11.so";
    
    /// Default HSM slot
    pub const DEFAULT_HSM_SLOT: u32 = 0;
    
    /// Default HSM PIN
    pub const DEFAULT_HSM_PIN: &str = "1234";
    
    /// Default key cache size
    pub const DEFAULT_KEY_CACHE_SIZE: usize = 1000;
    
    /// Default key cache TTL
    pub const DEFAULT_KEY_CACHE_TTL: std::time::Duration = std::time::Duration::from_secs(300); // 5 minutes
}

/// Compliance configuration constants
pub mod compliance {
    /// Default compliance standards
    pub const DEFAULT_COMPLIANCE_STANDARDS: &[&str] = &[
        "GDPR",
        "HIPAA",
        "SOC2",
        "PCI-DSS",
        "ISO-27001",
    ];
    
    /// Default audit retention
    pub const DEFAULT_AUDIT_RETENTION: std::time::Duration = std::time::Duration::from_secs(31536000); // 1 year
    
    /// Default compliance check interval
    pub const DEFAULT_COMPLIANCE_CHECK_INTERVAL: std::time::Duration = std::time::Duration::from_secs(3600); // 1 hour
}

/// Threat detection configuration constants
pub mod threat_detection {
    /// Default threat scan interval
    pub const DEFAULT_THREAT_SCAN_INTERVAL: std::time::Duration = std::time::Duration::from_secs(300); // 5 minutes
    
    /// Default threat threshold
    pub const DEFAULT_THREAT_THRESHOLD: f64 = 0.8;
    
    /// Default ML model path
    pub const DEFAULT_ML_MODEL_PATH: &str = "/etc/beardog/models/threat_detection.model";
    
    /// Default anomaly threshold
    pub const DEFAULT_ANOMALY_THRESHOLD: f64 = 0.9;
}

/// Genetic spawning configuration constants
pub mod genetic_spawning {
    /// Default mutation rate
    pub const DEFAULT_MUTATION_RATE: f64 = 0.01;
    
    /// Default crossover rate
    pub const DEFAULT_CROSSOVER_RATE: f64 = 0.8;
    
    /// Default population size
    pub const DEFAULT_POPULATION_SIZE: usize = 100;
    
    /// Default max generations
    pub const DEFAULT_MAX_GENERATIONS: usize = 1000;
    
    /// Default fitness threshold
    pub const DEFAULT_FITNESS_THRESHOLD: f64 = 0.95;
}

/// Workflow configuration constants
pub mod workflow {
    /// Default approval timeout
    pub const DEFAULT_APPROVAL_TIMEOUT: std::time::Duration = std::time::Duration::from_secs(3600); // 1 hour
    
    /// Default max approvers
    pub const DEFAULT_MAX_APPROVERS: usize = 10;
    
    /// Default min approvers
    pub const DEFAULT_MIN_APPROVERS: usize = 2;
    
    /// Default workflow timeout
    pub const DEFAULT_WORKFLOW_TIMEOUT: std::time::Duration = std::time::Duration::from_secs(86400); // 24 hours
}

/// Environment variable names
pub mod env_vars {
    /// BearDog configuration file path
    pub const BEARDOG_CONFIG_FILE: &str = "BEARDOG_CONFIG_FILE";
    
    /// BearDog bind address
    pub const BEARDOG_BIND_ADDRESS: &str = "BEARDOG_BIND_ADDRESS";
    
    /// BearDog API port
    pub const BEARDOG_API_PORT: &str = "BEARDOG_API_PORT";
    
    /// BearDog database URL
    pub const BEARDOG_DATABASE_URL: &str = "BEARDOG_DATABASE_URL";
    
    /// BearDog log level
    pub const BEARDOG_LOG_LEVEL: &str = "BEARDOG_LOG_LEVEL";
    
    /// BearDog security level
    pub const BEARDOG_SECURITY_LEVEL: &str = "BEARDOG_SECURITY_LEVEL";
    
    /// SongBird endpoint
    pub const SONGBIRD_ENDPOINT: &str = "SONGBIRD_ENDPOINT";
    
    /// SongBird API key
    pub const SONGBIRD_API_KEY: &str = "SONGBIRD_API_KEY";
    
    /// HSM library path
    pub const HSM_LIBRARY_PATH: &str = "HSM_LIBRARY_PATH";
    
    /// HSM slot
    pub const HSM_SLOT: &str = "HSM_SLOT";
    
    /// HSM PIN
    pub const HSM_PIN: &str = "HSM_PIN";
}

/// Version information
pub mod version {
    /// BearDog version
    pub const VERSION: &str = env!("CARGO_PKG_VERSION");
    
    /// BearDog mission statement
    pub const MISSION: &str = "Democratizing enterprise-grade security for everyone";
    
    /// BearDog description
    pub const DESCRIPTION: &str = env!("CARGO_PKG_DESCRIPTION");
    
    /// BearDog homepage
    pub const HOMEPAGE: &str = "https://beardog.security";
    
    /// BearDog repository
    pub const REPOSITORY: &str = "https://github.com/ecoprimal/beardog";
}

/// Feature flags
pub mod features {
    /// Enable HSM support
    pub const ENABLE_HSM: bool = cfg!(feature = "hsm");
    
    /// Enable ML threat detection
    pub const ENABLE_ML_THREAT_DETECTION: bool = cfg!(feature = "ml-threat-detection");
    
    /// Enable genetic spawning
    pub const ENABLE_GENETIC_SPAWNING: bool = cfg!(feature = "genetic-spawning");
    
    /// Enable compliance
    pub const ENABLE_COMPLIANCE: bool = cfg!(feature = "compliance");
    
    /// Enable federation
    pub const ENABLE_FEDERATION: bool = cfg!(feature = "federation");
}

/// Storage configuration constants
pub mod storage {
    /// Default rules path for threat detection
    pub const DEFAULT_RULES_PATH: &str = "/etc/beardog/rules";
    
    /// Default monitor path for threat detection
    pub const DEFAULT_MONITOR_PATH: &str = "/var/log/beardog/monitor";
    
    /// Default data directory
    pub const DEFAULT_DATA_DIR: &str = "/var/lib/beardog";
    
    /// Default log directory
    pub const DEFAULT_LOG_DIR: &str = "/var/log/beardog";
    
    /// Default config directory
    pub const DEFAULT_CONFIG_DIR: &str = "/etc/beardog";
    
    /// Default cache directory
    pub const DEFAULT_CACHE_DIR: &str = "/var/cache/beardog";
}