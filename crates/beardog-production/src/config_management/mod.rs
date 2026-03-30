// SPDX-License-Identifier: AGPL-3.0-only

//! # Production Configuration Management
//!
//! This module provides production-grade configuration management for BearDog.
//! It unifies configuration loading from multiple sources with proper secrets management.
//!
//! ## Architecture
//!
//! - **Canonical Types**: Uses types from `beardog-types::canonical::config`
//! - **Universal Adapters**: Vendor-agnostic secrets and config management
//! - **Secure by Default**: Production-ready security configurations
//! - **Multi-Source**: Environment, files, secrets managers, container orchestration

use beardog_errors::BearDogError;
use beardog_types::canonical::config::type_aliases::Environment;
use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use std::env;

/// Result type for configuration and secrets operations in this module.
pub type Result<T> = std::result::Result<T, BearDogError>;

// Import canonical configuration types (Phase 1 Migration - Oct 2, 2025)
use beardog_types::canonical::config::domains::database::MigrationConfig as CanonicalMigrationConfig;

/// Production configuration manager - loads and validates production configs
///
/// **NOTE**: This is a **production-specific runtime type**, not a canonical configuration.
/// It manages configuration loading from multiple sources (environment, files, secrets managers,
/// container orchestration). This type belongs in `beardog-production` as it handles runtime
/// operations specific to production deployments.
pub struct ProductionConfigManager {
    environment: Environment,
    config_sources: Vec<ConfigSource>,
    secrets_manager: SecretsManager,
    _config_cache: HashMap<String, ConfigValue>,
}

/// Configuration sources for loading production settings
///
/// **NOTE**: This is a **production-specific runtime type** for config loading.
/// It defines where configurations are loaded from at runtime (environment variables,
/// files, secrets managers, K8s secrets). Keep in `beardog-production`.
#[derive(Debug, Clone)]
pub enum ConfigSource {
    /// Environment variables
    Environment,
    /// Local configuration file
    File {
        /// Path to the configuration file on disk.
        path: String,
    },
    /// Universal secrets management capability
    UniversalSecretsManagement {
        /// Backend endpoint URL or connection string.
        endpoint: String,
        /// Provider identifier (e.g. vault, cloud KMS).
        provider_type: String,
        /// Provider-specific authentication parameters.
        auth_config: HashMap<String, String>,
    },
    /// Universal container orchestration secrets
    UniversalContainerSecrets {
        /// Kubernetes or orchestrator namespace.
        namespace: String,
        /// Container platform (e.g. kubernetes, docker_swarm).
        provider_type: String,
    },
}

/// Production configuration structure
#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct ProductionConfig {
    /// Application configuration
    pub application: ApplicationConfig,
    /// Database configuration  
    pub database: DatabaseConfig,
    /// Security configuration
    pub security: SecurityConfig,
    /// Monitoring configuration
    pub monitoring: MonitoringConfig,
    /// Logging configuration
    pub logging: beardog_types::canonical::config::domains::system::LoggingConfig,
    /// Networking configuration
    pub networking: NetworkingConfig,
    /// Scaling configuration
    pub scaling: ScalingConfig,
    /// Compliance configuration
    pub compliance: ComplianceConfig,
}

/// Application-level configuration for production deployments
///
/// **NOTE**: This is a **production-specific runtime type** for application orchestration.
/// It combines application identity, network settings, thread management, and timeouts
/// for production deployments. This is a comprehensive production configuration that
/// composes multiple concerns for runtime deployment.
///
/// **EVALUATION**: This struct is comprehensive and production-specific. It combines:
/// - Application identity (name, version, environment, instance_id)
/// - Network settings (bind_address, port)
/// - Runtime settings (worker_threads, max_connections)
/// - Timeout policies (request_timeout, graceful_shutdown_timeout)
///
/// **Recommendation**: Keep in `beardog-production` as it's a production deployment
/// composition. Individual concerns already have canonical types in:
/// - Network: `beardog_types::canonical::config::domains::network`
/// - System: `beardog_types::canonical::config::domains::system`
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ApplicationConfig {
    /// Application name
    pub name: String,
    /// Application version
    pub version: String,
    /// Environment name (development, staging, production)
    pub environment: String,
    /// Unique instance identifier (for distributed deployments)
    pub instance_id: String,
    /// Bind address
    pub bind_address: String,
    /// Port number
    pub port: u16,
    /// Number of worker threads
    pub worker_threads: usize,
    /// Maximum concurrent connections
    pub max_connections: usize,
    /// Request timeout in seconds
    pub request_timeout: u64,
    /// Graceful shutdown timeout in seconds
    pub graceful_shutdown_timeout: u64,
}

/// Database configuration
#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct DatabaseConfig {
    /// Primary database connection
    pub primary: DatabaseConnection,
    /// Read replica connections
    pub read_replicas: Vec<DatabaseConnection>,
    /// Connection pool configuration
    pub connection_pool: beardog_types::canonical::config::domains::network::ConnectionPoolConfig,
    /// Migration configuration
    pub migration: beardog_types::canonical::config::domains::database::MigrationConfig,
    /// Backup configuration
    pub backup: beardog_types::canonical::config::production::operations::BackupConfig,
}

/// Database connection configuration
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct DatabaseConnection {
    /// Database host
    pub host: String,
    /// Database port
    pub port: u16,
    /// Database name
    pub database: String,
    /// Database username
    pub username: String,
    /// Database password (injected from secrets manager)
    #[serde(skip_serializing)]
    pub password: String,
    /// SSL mode
    pub ssl_mode: String,
    /// Connection timeout in seconds
    pub connection_timeout: u64,
}

/// Connection pool configuration (DEPRECATED - use canonical)
///
/// **MIGRATION**: Use `beardog_types::canonical::config::domains::network::ConnectionPoolConfig` instead.
///
/// This type alias will be removed in v3.3.0.
#[deprecated(
    since = "3.1.0",
    note = "Use beardog_types::canonical::config::domains::network::ConnectionPoolConfig instead"
)]
pub type ConnectionPoolConfig =
    beardog_types::canonical::config::domains::network::ConnectionPoolConfig;

/// Logging configuration (DEPRECATED - use canonical)
///
/// **MIGRATION**: Use `beardog_types::canonical::config::domains::system::LoggingConfig` instead.
///
/// This type alias will be removed in v3.3.0.
#[deprecated(
    since = "3.1.0",
    note = "Use beardog_types::canonical::config::domains::system::LoggingConfig instead"
)]
pub type LoggingConfig = beardog_types::canonical::config::domains::system::LoggingConfig;

/// Log format (DEPRECATED - use canonical)
///
/// **MIGRATION**: Use `beardog_types::canonical::config::domains::system::LogFormat` instead.
#[deprecated(
    since = "3.1.0",
    note = "Use beardog_types::canonical::config::domains::system::LogFormat instead"
)]
pub type LogFormat = beardog_types::canonical::config::domains::system::LogFormat;

/// Log level (canonical)
pub type LogLevel = beardog_types::canonical::config::domains::system::LogLevel;

/// Production security section placeholder (extend with canonical fields as needed).
#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct SecurityConfig {}

/// Production monitoring section placeholder (extend with canonical fields as needed).
#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct MonitoringConfig {}

/// Log output destination
#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum LogOutput {
    /// Standard output
    Stdout,
    /// Standard error
    Stderr,
    /// File with rotation
    File {
        /// Path to the log file.
        path: String,
        /// Rotation policy when the file rolls over.
        rotation: FileRotation,
    },
    /// Syslog endpoint
    Syslog {
        /// Syslog server host or socket path.
        endpoint: String,
    },
    /// ElasticSearch endpoint
    ElasticSearch {
        /// ElasticSearch cluster URL.
        endpoint: String,
        /// Index name for log documents.
        index: String,
    },
    /// Splunk endpoint
    Splunk {
        /// Splunk HEC or indexer URL.
        endpoint: String,
        /// Authentication token for the Splunk API.
        token: String,
    },
}

/// File rotation configuration
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct FileRotation {
    /// Maximum file size in bytes
    pub max_size: u64,
    /// Maximum number of files to keep
    pub max_files: u32,
    /// Enable compression
    pub compress: bool,
}

/// Networking configuration for production deployments
///
/// **NOTE**: This is a **production-specific runtime type** for network orchestration.
/// It combines TLS, service mesh, load balancer, and rate limiting configs for production.
/// This is a production deployment composition and belongs in `beardog-production`.
///
/// For general-purpose network configuration, see:
/// - `beardog_types::canonical::config::domains::network::ConsolidatedNetworkConfiguration`
#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct NetworkingConfig {
    /// TLS configuration
    pub tls: TlsConfig,
    /// Service mesh configuration
    pub service_mesh: ServiceMeshConfig,
    /// Load balancer configuration
    pub load_balancer:
        beardog_types::canonical::config::domains::network::connection::LoadBalancerConfiguration,
    /// Rate limiting configuration
    pub rate_limiting: RateLimitingConfig,
}

/// TLS configuration for production deployments
///
/// **NOTE**: This is a **production-specific TLS configuration** with enhanced security controls.
/// It includes production-grade features like minimum TLS version enforcement and cipher suite
/// restrictions that go beyond basic TLS configuration.
///
/// For general-purpose TLS configuration, see:
/// - `beardog_types::canonical::config::domains::network::security::TlsConfiguration`
///
/// **Keep in `beardog-production`** as it provides production-specific security hardening.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct TlsConfig {
    /// Enable TLS
    pub enabled: bool,
    /// Certificate path
    pub cert_path: String,
    /// Private key path
    pub key_path: String,
    /// CA certificate path (optional)
    pub ca_path: Option<String>,
    /// Minimum TLS version (e.g., "TLS1.2", "TLS1.3")
    pub min_version: String,
    /// Allowed cipher suites for enhanced security
    pub cipher_suites: Vec<String>,
}

/// Scaling configuration for production deployments
///
/// **NOTE**: This is a **production-specific runtime type** for Kubernetes scaling orchestration.
/// It combines horizontal (HPA), vertical (VPA), and auto-scaling configurations for
/// production K8s deployments. This is K8s-specific infrastructure configuration.
/// Keep in `beardog-production`.
#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct ScalingConfig {
    /// Horizontal scaling configuration (HPA)
    pub horizontal: HorizontalScalingConfig,
    /// Vertical scaling configuration (VPA)
    pub vertical: VerticalScalingConfig,
    /// Auto-scaling configuration
    pub auto_scaling: AutoScalingConfig,
}

/// Auto-scaling configuration for Kubernetes
///
/// **NOTE**: This is a **production-specific runtime type** for Kubernetes auto-scaling.
/// It provides comprehensive auto-scaling settings including CPU/memory targets and cooldowns.
/// This is K8s-specific infrastructure configuration for HPA/VPA and belongs in `beardog-production`.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct AutoScalingConfig {
    /// Enable auto-scaling
    pub enabled: bool,
    /// Minimum number of replicas
    pub min_replicas: u32,
    /// Maximum number of replicas
    pub max_replicas: u32,
    /// Target CPU utilization percentage
    pub target_cpu_utilization: f64,
    /// Target memory utilization percentage
    pub target_memory_utilization: f64,
    /// Scale-up cooldown in seconds
    pub scale_up_cooldown: u64,
    /// Scale-down cooldown in seconds
    pub scale_down_cooldown: u64,
}

/// Compliance configuration for production deployments
///
/// **NOTE**: This is a **production-specific runtime type** for compliance management.
/// It combines data retention, encryption, and audit logging settings for production
/// deployments (GDPR, HIPAA, SOC2, etc.). This is production compliance orchestration
/// and belongs in `beardog-production`.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ComplianceConfig {
    /// Data retention configuration
    pub data_retention: DataRetentionConfig,
    /// Enable encryption at rest
    pub encryption_at_rest: bool,
    /// Enable encryption in transit
    pub encryption_in_transit: bool,
    /// Enable access logging
    pub access_logging: bool,
    /// Compliance standards (GDPR, HIPAA, SOC2, etc.)
    pub compliance_standards: Vec<ComplianceStandard>,
}

/// Data retention configuration for production deployments
///
/// **NOTE**: This is a **production-specific runtime type** for data retention policies.
/// It defines retention periods by data type for compliance (GDPR, HIPAA, etc.).
/// This is production compliance configuration and belongs in `beardog-production`.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct DataRetentionConfig {
    /// Log retention in days
    pub logs: u64,
    /// Metrics retention in days
    pub metrics: u64,
    /// Audit trails retention in days (typically 2555 days/7 years for SOC2)
    pub audit_trails: u64,
    /// User data retention in days (90 days per GDPR default)
    pub user_data: u64,
}

/// Compliance standard enumeration
///
/// **NOTE**: This is a **production-specific runtime enum** for compliance standards.
/// It defines the compliance frameworks to enforce in production deployments.
/// Keep in `beardog-production` as it's production compliance configuration.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum ComplianceStandard {
    /// GDPR (General Data Protection Regulation) compliance
    Gdpr,
    /// HIPAA (Health Insurance Portability and Accountability Act) compliance
    Hipaa,
    /// SOC2 (Service Organization Control 2) compliance
    Soc2,
    /// PCI-DSS (Payment Card Industry Data Security Standard) compliance
    PciDss,
}

/// Secrets manager for secure credential handling
///
/// **NOTE**: This is a **production-specific runtime type** for managing secrets at runtime.
/// It handles credential loading, caching, and rotation from multiple secret backends
/// (Vault, K8s secrets, environment variables, etc.). Keep in `beardog-production`.
pub struct SecretsManager {
    providers: Vec<SecretsProvider>,
    cache: HashMap<String, SecretValue>,
    _cache_ttl: u64,
}

/// Secrets provider types
///
/// **NOTE**: This is a **production-specific runtime enum** defining secret backend types.
/// `Vault` refers to BearDog's local encrypted file store (not HashiCorp Vault).
/// This is production infrastructure logic and belongs in `beardog-production`.
#[derive(Debug, Clone)]
pub enum SecretsProvider {
    /// BearDog local encrypted file vault (legacy env `VAULT_*` names).
    Vault {
        /// Vault root path or endpoint string.
        endpoint: String,
        /// Token or passphrase used when deriving the master key.
        token: String,
        /// Logical mount path under the vault root.
        mount_path: String,
    },
    /// Universal Secrets Manager — uses [`SecretsBackend`]; default backend is the file vault.
    UniversalSecretsManagement {
        /// Backend endpoint URL or filesystem path.
        endpoint: String,
        /// Provider identifier for capability selection.
        provider_type: String,
        /// Provider-specific authentication parameters.
        auth_config: HashMap<String, String>,
    },
    /// Environment variables (fallback)
    EnvironmentVariables,
}

/// Secret value with metadata
///
/// **NOTE**: This is a **production-specific runtime type** for storing secrets with metadata.
/// It wraps secret values with expiration and metadata for runtime secret management.
/// Keep in `beardog-production`.
#[derive(Debug, Clone)]
pub struct SecretValue {
    /// Secret value
    pub value: String,
    /// Expiration time
    pub expires_at: Option<std::time::SystemTime>,
    /// Metadata
    pub metadata: HashMap<String, String>,
}

/// Configuration value wrapper
///
/// **NOTE**: This is a **production-specific runtime type** for cached config values.
/// It provides a simple wrapper for configuration values in the runtime cache.
/// Keep in `beardog-production`.
#[derive(Debug, Clone)]
pub struct ConfigValue {
    /// Value as string
    pub value: String,
}

// Production-specific runtime configuration types
// These types are specific to production deployment and orchestration,
// not general-purpose canonical configurations.

/// Service mesh configuration
///
/// **NOTE**: This is a **production-specific runtime type** for service mesh setup.
/// It configures service mesh integration (Istio, Linkerd, etc.) for production deployments.
/// Keep in `beardog-production`.
#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct ServiceMeshConfig {
    /// Enable service mesh
    pub enabled: bool,
    /// Service mesh provider
    pub provider: String,
}

/// Load balancer configuration
///
/// **DEPRECATED**: Use `beardog_types::canonical::config::domains::network::LoadBalancerConfiguration` instead.
///
/// This type alias will be removed in v3.3.0.
#[deprecated(
    since = "3.1.0",
    note = "Use beardog_types::canonical::config::domains::network::LoadBalancerConfiguration instead"
)]
pub type LoadBalancerConfig =
    beardog_types::canonical::config::domains::network::connection::LoadBalancerConfiguration;

/// Rate limiting configuration
///
/// **NOTE**: This is a **production-specific runtime type** for rate limiting.
/// Simple production rate limiting settings. For more comprehensive rate limiting,
/// see `beardog_types::canonical::config::domains::network::RateLimitConfig`.
/// Keep in `beardog-production` for simple production use cases.
#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct RateLimitingConfig {
    /// Requests per second limit
    pub requests_per_second: u32,
}

/// Database migration configuration (DEPRECATED - use canonical)
///
/// **MIGRATION**: Use `beardog_types::canonical::config::domains::database::MigrationConfig` instead.
///
/// The canonical version provides more comprehensive migration settings including
/// directory paths, version tracking, and rollback support.
///
/// This type alias will be removed in v3.3.0 (Q1 2026).
#[deprecated(
    since = "3.1.1",
    note = "Use beardog_types::canonical::config::domains::database::MigrationConfig instead. \
            Canonical version provides comprehensive migration management."
)]
pub type MigrationConfig = CanonicalMigrationConfig;

/// Backup configuration
///
/// **DEPRECATED**: Use `beardog_types::canonical::config::production::BackupConfig` instead.
///
/// This type alias will be removed in v3.3.0.
#[deprecated(
    since = "3.1.0",
    note = "Use beardog_types::canonical::config::production::operations::BackupConfig instead"
)]
pub type BackupConfig = beardog_types::canonical::config::production::operations::BackupConfig;

/// Horizontal scaling configuration (Kubernetes HPA)
///
/// **NOTE**: This is a **production-specific runtime type** for Kubernetes autoscaling.
/// It configures Horizontal Pod Autoscaler (HPA) settings for production deployments.
/// This is K8s-specific infrastructure configuration and belongs in `beardog-production`.
#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct HorizontalScalingConfig {
    /// Enable horizontal scaling
    pub enabled: bool,
    /// Minimum replicas
    pub min_replicas: u32,
}

/// Vertical scaling configuration (Kubernetes VPA)
///
/// **NOTE**: This is a **production-specific runtime type** for Kubernetes resource management.
/// It configures Vertical Pod Autoscaler (VPA) and resource limits for production pods.
/// This is K8s-specific infrastructure configuration and belongs in `beardog-production`.
#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct VerticalScalingConfig {
    /// Enable vertical scaling
    pub enabled: bool,
    /// Minimum CPU
    pub min_cpu: String,
    /// Maximum CPU
    pub max_cpu: String,
    /// Minimum memory
    pub min_memory: String,
    /// Maximum memory
    pub max_memory: String,
}

pub mod secrets_backend;

pub mod runtime;

pub use secrets_backend::{FileVaultBackend, SecretsBackend};
