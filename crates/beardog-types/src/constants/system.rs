

use std::time::Duration;

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

pub mod monitoring {

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
