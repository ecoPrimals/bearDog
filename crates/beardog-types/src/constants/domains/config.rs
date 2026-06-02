// SPDX-License-Identifier: AGPL-3.0-or-later

//! Configuration Constants Domain
//!
//! Centralized constants for configuration values to eliminate string allocations
//! and ensure consistency across the `BearDog` ecosystem.

/// **PEDANTIC PERFORMANCE OPTIMIZATION** - Configuration String Constants
///
/// These constants eliminate repeated string allocations in configuration
/// initialization and provide compile-time guarantees of consistency.
/// System configuration constants
pub mod system {
    /// Default system name
    pub const DEFAULT_SYSTEM_NAME: &str = "beardog";

    /// Default version string
    pub const DEFAULT_VERSION: &str = "3.6.0";

    /// Default environment designation
    pub const DEFAULT_ENVIRONMENT: &str = "development";

    /// Production environment designation
    pub const PRODUCTION_ENVIRONMENT: &str = "production";

    /// Default log level
    pub const DEFAULT_LOG_LEVEL: &str = "info";

    /// Debug log level
    pub const DEBUG_LOG_LEVEL: &str = "debug";

    /// Warning log level
    pub const WARN_LOG_LEVEL: &str = "warn";

    /// Error log level
    pub const ERROR_LOG_LEVEL: &str = "error";
}

/// Security configuration constants
pub mod security {
    /// Default encryption algorithm
    pub const DEFAULT_ENCRYPTION_ALGORITHM: &str = "AES-256-GCM";

    /// Default HSM provider
    pub const DEFAULT_HSM_PROVIDER: &str = "software";

    /// Default JWT issuer
    pub const DEFAULT_JWT_ISSUER: &str = "beardog";

    /// Default JWT audience
    pub const DEFAULT_JWT_AUDIENCE: &str = "beardog-api";

    /// Default rate limiting strategy
    pub const DEFAULT_RATE_LIMIT_STRATEGY: &str = "sliding_window";
}

/// AI and ML configuration constants
pub mod ai {
    /// Hybrid intelligence mode
    pub const HYBRID_MODE: &str = "hybrid";

    /// Default monitor metric
    pub const DEFAULT_MONITOR_METRIC: &str = "loss";

    /// Adam optimizer
    pub const ADAM_OPTIMIZER: &str = "adam";

    /// LRU eviction policy
    pub const LRU_EVICTION_POLICY: &str = "LRU";

    /// Dense layer type
    pub const DENSE_LAYER_TYPE: &str = "dense";

    /// `ReLU` activation
    pub const RELU_ACTIVATION: &str = "relu";

    /// Softmax activation
    pub const SOFTMAX_ACTIVATION: &str = "softmax";

    /// Float32 data type
    pub const FLOAT32_DATA_TYPE: &str = "float32";

    /// Batch normalization
    pub const BATCH_NORM_TYPE: &str = "batch_norm";

    /// Ensemble strategy
    pub const ENSEMBLE_STRATEGY: &str = "ensemble";

    /// Categorical crossentropy loss
    pub const CATEGORICAL_CROSSENTROPY_LOSS: &str = "categorical_crossentropy";
}

/// Network configuration constants
pub mod network {
    use beardog_config::env_keys;

    /// Default health check endpoint
    pub const DEFAULT_HEALTH_ENDPOINT: &str = "/health";

    /// Default readiness endpoint  
    pub const DEFAULT_READY_ENDPOINT: &str = "/ready";

    /// HTTP protocol
    pub const HTTP_PROTOCOL: &str = "http";

    /// HTTPS protocol
    pub const HTTPS_PROTOCOL: &str = "https";

    /// Get default auth callback URI (environment-aware)
    pub fn default_auth_callback() -> String {
        std::env::var(env_keys::ENV_AUTH_CALLBACK).unwrap_or_else(|_| {
            let network_config = crate::canonical::config::network::NetworkConfig::default();
            format!(
                "http://{}:{}/auth/callback",
                network_config.default_host, network_config.service_ports.api_port
            )
        })
    }

    /// Strict certificate validation
    pub const STRICT_CERT_VALIDATION: &str = "strict";
}

/// Storage and database constants
pub mod storage {
    use beardog_config::env_keys;

    /// Memory backend
    pub const MEMORY_BACKEND: &str = "memory";

    /// Memory URL
    pub const MEMORY_URL: &str = "memory://";

    /// File backend
    pub const FILE_BACKEND: &str = "file";

    /// `SQLite` backend
    pub const SQLITE_BACKEND: &str = "sqlite";

    /// `SQLite` memory URL
    pub const SQLITE_MEMORY_URL: &str = "sqlite::memory:";

    /// Redis backend
    pub const REDIS_BACKEND: &str = "redis";

    /// Get default Redis URL (environment-aware)
    pub fn default_redis_url() -> String {
        std::env::var(env_keys::ENV_REDIS_URL)
            .or_else(|_| std::env::var(env_keys::ENV_REDIS_URL_PREFIXED))
            .unwrap_or_else(|_| {
                let network_config = crate::canonical::config::network::NetworkConfig::default();
                let redis_port = std::env::var(env_keys::ENV_REDIS_PORT_UNPREFIXED)
                    .ok()
                    .and_then(|p| p.parse().ok())
                    .unwrap_or(6379);
                format!("redis://{}:{}/0", network_config.default_host, redis_port)
            })
    }
}

/// Workflow and scheduling constants
pub mod workflow {
    /// Default cron scheduler
    pub const CRON_SCHEDULER_TYPE: &str = "cron";

    /// Daily midnight schedule
    pub const DAILY_MIDNIGHT_SCHEDULE: &str = "0 0 * * *";

    /// Daily 2AM schedule for cleanup
    pub const DAILY_2AM_SCHEDULE: &str = "0 2 * * *";

    /// UTC timezone
    pub const UTC_TIMEZONE: &str = "UTC";

    /// Blue-green deployment strategy
    pub const BLUE_GREEN_STRATEGY: &str = "blue_green";

    /// Rolling update strategy
    pub const ROLLING_UPDATE_STRATEGY: &str = "rolling_update";
}

/// Monitoring and metrics constants
pub mod monitoring {
    /// Email notification channel
    pub const EMAIL_CHANNEL: &str = "email";

    /// Warning severity level
    pub const WARNING_SEVERITY: &str = "warning";

    /// Critical severity level
    pub const CRITICAL_SEVERITY: &str = "critical";

    /// Grid dashboard layout
    pub const GRID_LAYOUT: &str = "grid";

    /// Adaptive strategy
    pub const ADAPTIVE_STRATEGY: &str = "adaptive";

    /// Push strategy
    pub const PUSH_STRATEGY: &str = "push";

    /// JSON export format
    pub const JSON_FORMAT: &str = "json";

    /// GZIP compression algorithm
    pub const GZIP_ALGORITHM: &str = "gzip";
}

/// Compliance and security framework constants
pub mod compliance {
    /// GDPR framework
    pub const GDPR_FRAMEWORK: &str = "GDPR";

    /// HIPAA framework
    pub const HIPAA_FRAMEWORK: &str = "HIPAA";

    /// SOC2 framework
    pub const SOC2_FRAMEWORK: &str = "SOC2";

    /// PCI DSS framework
    pub const PCI_DSS_FRAMEWORK: &str = "PCI_DSS";
}

/// `OpenID` Connect scopes
pub mod oidc {
    /// `OpenID` scope
    pub const OPENID_SCOPE: &str = "openid";

    /// Profile scope
    pub const PROFILE_SCOPE: &str = "profile";

    /// Email scope
    pub const EMAIL_SCOPE: &str = "email";
}

/// Kubernetes resource specifications
pub mod k8s {
    /// Default CPU request
    pub const DEFAULT_CPU_REQUEST: &str = "100m";

    /// Default memory request
    pub const DEFAULT_MEMORY_REQUEST: &str = "128Mi";

    /// Default CPU limit
    pub const DEFAULT_CPU_LIMIT: &str = "500m";

    /// Default memory limit
    pub const DEFAULT_MEMORY_LIMIT: &str = "512Mi";

    /// Production CPU request
    pub const PRODUCTION_CPU_REQUEST: &str = "500m";

    /// Production memory request
    pub const PRODUCTION_MEMORY_REQUEST: &str = "1Gi";

    /// Production CPU limit
    pub const PRODUCTION_CPU_LIMIT: &str = "1";

    /// Production memory limit
    pub const PRODUCTION_MEMORY_LIMIT: &str = "2Gi";

    /// Default max unavailable percentage
    pub const DEFAULT_MAX_UNAVAILABLE: &str = "50%";

    /// Default max surge percentage
    pub const DEFAULT_MAX_SURGE: &str = "100%";
}

/// Default directory paths
pub mod paths {
    /// Default keys directory
    pub const DEFAULT_KEYS_DIR: &str = "./keys";

    /// Default exports directory
    pub const DEFAULT_EXPORTS_DIR: &str = "./exports";

    /// Default archive directory
    pub const DEFAULT_ARCHIVE_DIR: &str = "./archive";

    /// Default backups directory
    pub const DEFAULT_BACKUPS_DIR: &str = "./backups";

    /// Default migrations directory
    pub const DEFAULT_MIGRATIONS_DIR: &str = "./migrations";

    /// Default profiling directory
    pub const DEFAULT_PROFILING_DIR: &str = "./profiling";

    /// Default audit logs directory
    pub const DEFAULT_AUDIT_LOGS_DIR: &str = "./audit_logs";
}

/// Security warning constants for production
pub mod security_warnings {
    /// Default secret warning
    pub const CHANGE_DEFAULT_SECRET: &str = "change-me-in-production";
}

// **PEDANTIC SAFETY DOCUMENTATION**
//
// These constants are designed for:
// - **Zero allocation**: All `&'static str` for compile-time storage
// - **Consistency**: Single source of truth prevents typos
// - **Performance**: Eliminates repeated string allocations
// - **Maintainability**: Centralized configuration values
// - **Type safety**: Compile-time validation of constant usage
