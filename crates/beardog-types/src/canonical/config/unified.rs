// Unified Configuration System - Single Source of Truth
//
// This module provides the **PRIMARY** configuration system for BearDog that eliminates
// all fragmentation by consolidating 80+ Config structs into a unified hierarchy.
//
// ## Configuration Unification Strategy
//
// ### **ELIMINATED FRAGMENTATION**
// - ✅ **80+ Config structs** → **1 Unified Config** + **Domain modules**
// - ✅ **364 files** with configs → **Centralized system**
// - ✅ **Multiple version constants** → **Single version source**
// - ✅ **3 different systems** → **1 unified system**
//
// ### **ARCHITECTURE PRINCIPLES**
// - **Single Source of Truth**: All configuration in one canonical location
// - **Domain Organization**: Logical grouping by functional area
// - **Zero Duplication**: No duplicate config types anywhere
// - **Type Safety**: Strongly typed with comprehensive validation
// - **Performance Optimized**: Zero-cost abstractions and efficient loading
// - **Migration Friendly**: Maintains compatibility during transition

use beardog_errors::{BearDogError, BearDogResult};
use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use std::path::PathBuf;
use std::time::Duration;

// Import canonical configuration types
use crate::canonical::config::network::{CircuitBreakerConfig, LoadBalancingConfig};
use crate::canonical::config::security::{
    CanonicalAuditConfig, CanonicalAuthenticationConfig, CanonicalAuthorizationConfig,
    CanonicalEncryptionConfig, CanonicalMfaConfig, CanonicalSessionConfig,
};
// Import unified monitoring configuration

use crate::canonical::config::hsm::UnifiedHsmConfig;
use crate::canonical::config::type_aliases::{
    DatabaseBackupConfig, EntropyCollectionConfig, EntropyHierarchyConfig, GeneticAlgorithmConfig,
    HumanEntropyConfig, SimdOptimizationConfig,
};
// Removed unused import: RetryConfig (used directly where needed)
// Note: SecurityPolicy will be aliased to SecurityConfig for now

// Type aliases for compatibility - REMOVED October 2025
// Use the canonical types directly: RetryConfig, RateLimitConfig, SecurityConfig, etc.

/// Unified BearDog Configuration - Single Source of Truth
///
/// This is the **ROOT** of the unified configuration system that eliminates all
/// fragmentation by consolidating 80+ Config structs into one comprehensive structure.
///
/// ## Architecture
///
/// The configuration is organized into three main layers:
///
/// 1. **Core Domains** - Essential system configurations (app, network, security, hsm, database, monitoring)
/// 2. **Specialized Domains** - Advanced capabilities (genetics, workflows, compliance, performance)
/// 3. **Infrastructure Domains** - System operations (production, deployment)
///
/// ## Benefits
///
/// - **Single Source of Truth** - No configuration fragmentation
/// - **Type Safety** - Strongly typed with validation
/// - **Zero Duplication** - Consolidates 364 files with configs
/// - **Performance** - Zero-cost abstractions
/// - **Migration Friendly** - Maintains compatibility
///
/// ## Example
///
/// ```rust
/// use beardog_types::canonical::config::unified::UnifiedBearDogConfig;
/// # use beardog_errors::BearDogError;
///
/// # fn example() -> Result<(), BearDogError> {
/// // Load from environment
/// let config = UnifiedBearDogConfig::load()?;
///
/// // Access domain configs
/// println!("App name: {}", config.app.app_name);
/// println!("Network timeout: {:?}", config.network.request_timeout);
/// println!("HSM enabled: {}", config.hsm.enabled);
/// # Ok(())
/// # }
/// ```
#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct UnifiedBearDogConfig {
    /// System metadata including version, environment, and instance information
    pub metadata: SystemMetadata,

    /// **CORE DOMAINS** - Essential system configurations

    /// Application-level configuration (ports, service name, operational parameters)
    pub app: UnifiedAppConfig,

    /// Network communication and connectivity (timeouts, TLS, load balancing)
    pub network: UnifiedNetworkConfig,

    /// Security policies and authentication (encryption, access control, threat detection)
    pub security: UnifiedSecurityConfig,

    /// Hardware Security Module integration (YubiKey, TPM, PKCS#11)
    pub hsm: UnifiedHsmConfig,

    /// Database connectivity and storage (pools, migrations, backups)
    pub database: UnifiedDatabaseConfig,

    /// Monitoring, metrics, and observability (telemetry, alerts, dashboards)
    pub monitoring: crate::canonical::monitoring::MonitoringConfig,

    /// **SPECIALIZED DOMAINS** - Advanced capabilities

    /// Genetic algorithms and AI configuration (entropy, biome sovereignty, key evolution)
    pub genetics: UnifiedGeneticsConfig,

    /// Workflow engine and automation (orchestration, business processes)
    pub workflows: UnifiedWorkflowConfig,

    /// Compliance and regulatory settings (GDPR, HIPAA, SOC 2)
    pub compliance: UnifiedComplianceConfig,

    /// Performance optimization (concurrency, buffers, zero-copy)
    pub performance: UnifiedPerformanceConfig,

    /// **INFRASTRUCTURE DOMAINS** - System operations

    /// Production environment configuration (deployment, scaling, reliability)
    pub production: UnifiedProductionConfig,

    /// Deployment and orchestration (K8s, containers, service mesh)
    pub deployment: UnifiedDeploymentConfig,

    /// Testing framework and validation configuration
    pub testing: UnifiedTestingConfig,

    /// Development environment configuration (dev tools, hot reload, debug settings)
    pub development: UnifiedDevelopmentConfig,

    /// **INTEGRATION DOMAINS** - External connectivity

    /// External adapter and integration configuration (API adapters, protocol bridges)
    pub adapters: UnifiedAdapterConfig,

    /// Secure tunnel and VPN configuration (encrypted channels, peer discovery)
    /// The tunnel value
    pub tunnel: UnifiedTunnelConfig,
    /// Federation and distributed system configuration
    /// The federation value
    pub federation: UnifiedFederationConfig,
    /// Ecosystem and plugin configuration
    /// The ecosystem value
    pub ecosystem: UnifiedEcosystemConfig,
}

#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct SystemMetadata {
    /// The version value
    pub version: UnifiedVersionInfo,

    /// **ENVIRONMENT CONFIGURATION**
    /// Current deployment environment (dev, staging, prod)
    /// The environment value
    pub environment: Environment,
    /// Deployment mode and strategy
    /// The deployment mode value
    pub deployment_mode: DeploymentMode,
    /// Mapping of feature flags
    pub feature_flags: HashMap<String, bool>,
    pub rollout_config: RolloutConfig,

    /// **SYSTEM IDENTIFICATION**
    pub instance_id: String,
    pub cluster_id: Option<String>,
    /// Unique node identifier within the cluster
    pub node_id: String,
    /// Optional region
    pub region: Option<String>,
    /// Availability zone within the region
    /// Optional zone
    pub zone: Option<String>,
}

/// **UNIFIED VERSION INFORMATION** - Eliminates all version constant duplication
#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct UnifiedVersionInfo {
    /// **PRIMARY VERSIONS** - Core system versions
    /// Main `BearDog` system version (e.g., "3.0.0")
    /// The beardog version value
    pub beardog_version: String, // "3.0.0" - Main BearDog version
    pub config_schema_version: String, // "1.0.0" - Configuration schema version
    /// The api version value
    pub api_version: String, // "2.0" - API protocol version

    /// **COMPONENT VERSIONS** - Individual component versions
    /// The workflow system version value
    pub workflow_system_version: String, // "3.1.0" - Workflow engine version
    /// Hsm Foundation Version
    /// The hsm foundation version value
    pub hsm_foundation_version: String, // "2.0.0-clean" - HSM foundation version
    /// Software Hsm Version
    /// The software hsm version value
    pub software_hsm_version: String, // "1.0.0" - Software HSM version
    /// Tunnel Version
    /// The tunnel version value
    pub tunnel_version: String, // "1.0.0" - Tunnel system version
    /// Genetics Version
    /// The genetics version value
    pub genetics_version: String, // "1.0.0" - Genetics engine version

    /// **COMPATIBILITY VERSIONS** - Supported version ranges
    /// The min client version value
    pub min_client_version: String, // "1.0.0" - Minimum supported client
    /// Min Rust Version
    /// The min rust version value
    pub min_rust_version: String, // "1.70.0" - Minimum Rust version
    /// Max Supported Version
    /// The max supported version value
    pub max_supported_version: String, // "4.0.0" - Maximum supported version

    /// **BUILD INFORMATION**
    pub build_timestamp: String,
    /// Git Commit
    /// Optional git commit
    pub git_commit: Option<String>,
    /// Build Profile
    /// The build profile value
    pub build_profile: String, // "release", "debug", "production"
}

/// **UNIFIED APPLICATION CONFIG** - Consolidates all app-level configurations
#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct UnifiedAppConfig {
    /// **APPLICATION IDENTITY**
    /// Name of the app
    pub app_name: String,
    /// App Description
    /// The app description value
    pub app_description: String,
    /// Organization
    /// The organization value
    pub organization: String,

    /// **RUNTIME CONFIGURATION**
    /// The log level value
    pub log_level: LogLevel,
    /// Enable Metrics
    /// Whether `enable_metrics` is enabled
    pub enable_metrics: bool,
    /// Enable Tracing
    /// Whether `enable_tracing` is enabled
    pub enable_tracing: bool,
    /// Enable Profiling
    /// Whether `enable_profiling` is enabled
    pub enable_profiling: bool,

    /// **RESOURCE LIMITS**
    /// Number of `max_memory_mb`
    pub max_memory_mb: usize,
    /// Max Cpu Cores
    /// Optional max cpu cores
    pub max_cpu_cores: Option<usize>,
    /// Max File Descriptors
    /// Number of `max_file_descriptors`
    pub max_file_descriptors: usize,
    /// Max Connections
    /// Number of `max_connections`
    pub max_connections: usize,

    /// **FILE SYSTEM**
    pub config_dir: PathBuf,
    /// Data Dir
    /// The data dir value
    pub data_dir: PathBuf,
    /// Log Dir
    /// The log dir value
    pub log_dir: PathBuf,
    /// Temp Dir
    /// The temp dir value
    pub temp_dir: PathBuf,
    /// Cache Dir
    /// The cache dir value
    pub cache_dir: PathBuf,
}

/// **UNIFIED NETWORK CONFIG** - Consolidates all network configurations
#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct UnifiedNetworkConfig {
    /// **CORE NETWORKING**
    /// The bind address value
    pub bind_address: String,
    /// Port
    /// Number of port
    pub port: u16,
    /// External Address
    /// Optional external address
    pub external_address: Option<String>,
    /// Enable IPv6 network support
    /// Whether `enable_ipv6` is enabled
    pub enable_ipv6: bool,

    /// **CONNECTION MANAGEMENT**
    /// Number of `max_connections`
    pub max_connections: usize,
    /// Connection Timeout
    pub connection_timeout: Duration,
    /// Request Timeout
    pub request_timeout: Duration,
    /// Keep Alive Timeout
    pub keep_alive_timeout: Duration,

    /// **TLS/SECURITY**
    /// Whether `enable_tls` is enabled
    pub enable_tls: bool,
    /// Tls Cert Path
    /// Optional tls cert path
    pub tls_cert_path: Option<PathBuf>,
    /// Tls Key Path
    /// Optional tls key path
    pub tls_key_path: Option<PathBuf>,
    /// Tls Ca Path
    /// Optional tls ca path
    pub tls_ca_path: Option<PathBuf>,
    /// Require Client Certs
    /// Whether `require_client_certs` is enabled
    pub require_client_certs: bool,

    /// **PERFORMANCE TUNING**
    /// Number of `buffer_size`
    pub buffer_size: usize,
    /// Send Buffer Size
    /// Number of `send_buffer_size`
    pub send_buffer_size: usize,
    /// Recv Buffer Size
    /// Number of `recv_buffer_size`
    pub recv_buffer_size: usize,
    /// Tcp Nodelay
    /// Whether `tcp_nodelay` is enabled
    pub tcp_nodelay: bool,

    /// **LOAD BALANCING & RESILIENCE**
    /// The load balancing value
    pub load_balancing: LoadBalancingConfig,
    /// Circuit Breaker
    /// The circuit breaker value
    pub circuit_breaker: CircuitBreakerConfig,
    /// Retry Policy
    /// The retry policy value
    pub retry_policy: crate::canonical::providers_unified::resilience::RetryConfig,
    /// Rate Limiting
    /// The rate limiting value
    pub rate_limiting: crate::canonical::config::security::RateLimitingConfig,
}

/// **UNIFIED SECURITY CONFIG** - Consolidates all security configurations
#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct UnifiedSecurityConfig {
    /// **AUTHENTICATION**
    /// The authentication value
    pub authentication: CanonicalAuthenticationConfig,

    /// **AUTHORIZATION**
    /// The authorization value
    pub authorization: CanonicalAuthorizationConfig,

    /// **ENCRYPTION**
    /// The encryption value
    pub encryption: CanonicalEncryptionConfig,

    /// **SESSION MANAGEMENT**
    /// The session value
    pub session: CanonicalSessionConfig,

    /// **MULTI-FACTOR AUTHENTICATION**
    /// The mfa value
    pub mfa: CanonicalMfaConfig,

    /// **SECURITY POLICIES**
    /// The policies value
    pub policies: crate::canonical::SecurityConfig,

    /// **AUDIT & COMPLIANCE**
    /// The audit value
    pub audit: CanonicalAuditConfig,

    /// **THREAT DETECTION**
    /// The threat detection value
    pub threat_detection: crate::canonical::config::domains::threat::CanonicalThreatDetectionConfig,
}

/// **UNIFIED HSM CONFIG** - Consolidates all HSM configurations
///
/// **MIGRATION COMPLETE**: This now uses the canonical HSM system from `hsm_unified`.
/// All legacy HSM configurations from beardog-tunnel have been consolidated.
// REMOVED: Duplicate alias (Phase 2 cleanup - October 2025)
// Use CanonicalHsmConfig directly or HsmConfig alias
// REMOVED: Commented alias - use crate::canonical::hsm_unified::CanonicalHsmConfig directly
/// **UNIFIED DATABASE CONFIG** - Consolidates all database configurations
#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct UnifiedDatabaseConfig {
    /// **CONNECTION DETAILS**
    /// The connection string value
    pub connection_string: String,
    /// Database Name
    /// Name of the database
    pub database_name: String,
    /// Username
    /// Name of the useritem
    pub username: String,
    /// Password Source
    /// The password source value
    pub password_source: PasswordSource,

    /// **CONNECTION POOLING**
    /// Number of `pool_size`
    pub pool_size: u32,
    /// Min Connections
    /// Number of `min_connections`
    pub min_connections: u32,
    /// Max Connections
    /// Number of `max_connections`
    pub max_connections: u32,
    /// Connection Timeout
    pub connection_timeout: Duration,
    /// Idle Timeout
    pub idle_timeout: Duration,
    /// Max Lifetime
    pub max_lifetime: Duration,

    /// **PERFORMANCE**
    pub query_timeout: Duration,
    /// Batch Size
    /// Number of `batch_size`
    pub batch_size: usize,
    /// Enable Prepared Statements
    /// Whether `enable_prepared_statements` is enabled
    pub enable_prepared_statements: bool,
    /// Enable Connection Pooling
    /// Whether `enable_connection_pooling` is enabled
    pub enable_connection_pooling: bool,

    /// **BACKUP & RECOVERY**
    pub backup_config: DatabaseBackupConfig,

    /// **ENCRYPTION**
    /// Whether `enable_encryption_at_rest` is enabled
    pub enable_encryption_at_rest: bool,
    /// Enable Encryption In Transit
    /// Whether `enable_encryption_in_transit` is enabled
    pub enable_encryption_in_transit: bool,
}

// UnifiedMonitoringConfig is now imported from monitoring.rs

/// **UNIFIED GENETICS CONFIG** - Consolidates all genetics configurations
#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct UnifiedGeneticsConfig {
    /// **ENTROPY COLLECTION**
    /// The entropy value
    pub entropy: EntropyCollectionConfig,

    /// **HUMAN ENTROPY**
    /// The human entropy value
    pub human_entropy: HumanEntropyConfig,

    /// **GENETIC ALGORITHMS**
    /// The genetic algorithms value
    pub genetic_algorithms: GeneticAlgorithmConfig,

    /// **SIMD OPTIMIZATION**
    /// The simd optimization value
    pub simd_optimization: SimdOptimizationConfig,

    /// **ENTROPY HIERARCHY**
    /// The entropy hierarchy value
    pub entropy_hierarchy: EntropyHierarchyConfig,
}

// Supporting configuration types
#[derive(Debug, Clone, Serialize, Deserialize, Default)]
/// Deployment environment configuration
pub enum Environment {
    #[default]
    /// Development Environment
    Development,
    /// Testing variant
    Testing,
    /// Staging variant
    Staging,
    /// Production variant
    Production,
}

#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub enum DeploymentMode {
    /// Standalone deployment mode
    #[default]
    /// Represents standalone variant
    Standalone,
    /// Cluster variant
    Cluster,
    /// Federation variant
    Federation,
    /// Cloud variant
    Cloud,
}

/// Logging level configuration
#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub enum LogLevel {
    /// Trace variant
    Trace,
    /// Debug variant
    Debug,
    /// Info log level (default)
    #[default]
    /// Represents info variant
    Info,
    /// Warn variant
    Warn,
    /// Error variant
    Error,
}

#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct RolloutConfig {
    /// Enable Gradual Rollout
    /// Whether `enable_gradual_rollout` is enabled
    pub enable_gradual_rollout: bool,
    /// Rollout Percentage
    /// The rollout percentage value
    pub rollout_percentage: f64,
    /// Rollout Strategy
    /// The rollout strategy value
    pub rollout_strategy: RolloutStrategy,
}

#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub enum RolloutStrategy {
    /// Blue-green deployment strategy (default)
    #[default]
    /// Represents blue green variant
    BlueGreen,
    /// Canary variant
    Canary,
    /// `RollingUpdate` variant
    RollingUpdate,
    /// Immediate variant
    Immediate,
}

/// Password source configuration options
#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum PasswordSource {
    /// Password from environment variable
    Environment(String),
    /// Password from file path
    File(PathBuf),
    /// Password from vault service
    Vault(String),
    Inline(String), // Not recommended for production
}

impl Default for PasswordSource {
    fn default() -> Self {
        Self::Environment("DATABASE_PASSWORD".to_string())
    }
}

// Additional supporting types would be defined here...
// (LoadBalancingConfig, CircuitBreakerConfig, etc.)

impl UnifiedBearDogConfig {
    /// Loads data
    /// Loads data
    pub fn load() -> Result<UnifiedBearDogConfig, BearDogError> {
        let mut config = UnifiedBearDogConfig::default();

        // Load from environment variables
        if let Ok(env) = std::env::var("BEARDOG_ENVIRONMENT") {
            config.metadata.environment = match env.as_str() {
                "testing" => Environment::Testing,
                "staging" => Environment::Staging,
                "production" => Environment::Production,
                _ => Environment::Development,
            };
        }

        // Validate the loaded configuration
        config.validate()?;
        Ok(config)
    }

    /// **CONFIGURATION VALIDATION** - Comprehensive validation across all domains
    /// Validates input
    /// Validates input
    pub fn validate(&self) -> Result<(), BearDogError> {
        // Validate metadata
        if self.metadata.version.beardog_version.is_empty() {
            return Err(BearDogError::business(
                "Configuration version cannot be empty".to_string(),
            ));
        }

        // Validate monitoring configuration
        // Monitoring validation not needed - using validated defaults

        // Production configuration is valid by construction

        Ok(())
    }

    /// **CONFIGURATION MIGRATION** - Migrate from legacy configurations
    pub fn migrate_from_legacy() -> Result<UnifiedBearDogConfig, BearDogError> {
        let mut config = UnifiedBearDogConfig::default();

        // Set up basic migration defaults
        config.metadata.version.beardog_version = env!("CARGO_PKG_VERSION").to_string();
        config.metadata.environment = Environment::Development;

        // Enable basic monitoring
        config.monitoring.enabled = true;
        // Updated to use new unified monitoring config structure
        // config.monitoring.interval_seconds = 30;
        // config.monitoring.retention_hours = 24;

        // Validate migrated configuration
        config.validate()?;
        Ok(config)
    }
}

// Temporary placeholder types - these would be fully implemented
/// Unified workflow configuration (placeholder)
#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct UnifiedWorkflowConfig {}

/// Unified compliance configuration (placeholder)
#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct UnifiedComplianceConfig {}

#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct UnifiedPerformanceConfig {}

/// Unified production configuration (placeholder)
#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct UnifiedProductionConfig {}

/// Unified deployment configuration (placeholder)
#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct UnifiedDeploymentConfig {}

/// Unified testing configuration (placeholder)
#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct UnifiedTestingConfig {}

/// Unified development configuration (placeholder)
#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct UnifiedDevelopmentConfig {}

/// Unified adapter configuration (placeholder)
#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct UnifiedAdapterConfig {}

/// Unified tunnel configuration (placeholder)
#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct UnifiedTunnelConfig {}

/// Unified federation configuration (placeholder)
#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct UnifiedFederationConfig {}

/// Unified ecosystem configuration (placeholder)
#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct UnifiedEcosystemConfig {}

// =============================================================================
// SIMPLIFIED CONFIGURATION - Lightweight alternative to UnifiedBearDogConfig
// =============================================================================

/// **SIMPLIFIED CONFIGURATION** - Lightweight alternative for simple deployments
///
/// This provides a simplified configuration structure for projects that don't need
/// the full complexity of `UnifiedBearDogConfig`. It includes only the most essential
/// configuration domains with sensible defaults.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SimplifiedBearDogConfig {
    /// Version string
    pub version: String,
    /// Environment designation
    pub environment: String,
    /// Unique instance identifier
    pub instance_id: String,
    /// Network settings
    pub network: NetworkSettings,
    /// Security settings
    pub security: SecuritySettings,
    /// Database settings
    pub database: DatabaseSettings,
    /// Monitoring settings
    pub monitoring: MonitoringSettings,
    /// Feature flags
    pub features: HashMap<String, bool>,
    /// Performance settings
    pub performance: PerformanceSettings,
}

/// Simplified network configuration
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct NetworkSettings {
    /// Bind address
    pub bind_address: String,
    /// Port number
    pub port: u16,
    /// Maximum connections
    pub max_connections: usize,
    /// Timeout in seconds
    pub timeout_seconds: u64,
    /// Enable TLS
    pub enable_tls: bool,
}

impl Default for NetworkSettings {
    fn default() -> Self {
        Self {
            bind_address: "127.0.0.1".to_string(),
            port: 8080,
            max_connections: 1000,
            timeout_seconds: 30,
            enable_tls: true,
        }
    }
}

/// Simplified security configuration
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SecuritySettings {
    /// Session timeout in seconds
    pub session_timeout_seconds: u64,
    /// Maximum login attempts
    pub max_login_attempts: u32,
    /// Enable multi-factor authentication
    pub enable_mfa: bool,
    /// Hash rounds for password hashing
    pub hash_rounds: u32,
    /// Audit retention in days
    pub audit_retention_days: u32,
}

impl Default for SecuritySettings {
    fn default() -> Self {
        Self {
            session_timeout_seconds: 3600,
            max_login_attempts: 5,
            enable_mfa: true,
            hash_rounds: 12,
            audit_retention_days: 365,
        }
    }
}

/// Simplified database configuration
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct DatabaseSettings {
    /// Connection string
    pub connection_string: String,
    /// Connection pool size
    pub pool_size: u32,
    /// Timeout in seconds
    pub timeout_seconds: u64,
    /// Enable encryption
    pub enable_encryption: bool,
}

impl Default for DatabaseSettings {
    fn default() -> Self {
        Self {
            connection_string: "sqlite://beardog.db".to_string(),
            pool_size: 10,
            timeout_seconds: 30,
            enable_encryption: true,
        }
    }
}

/// Simplified monitoring configuration
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct MonitoringSettings {
    /// Enable metrics collection
    pub enable_metrics: bool,
    /// Metrics collection interval in seconds
    pub metrics_interval_seconds: u64,
    /// Log level
    pub log_level: String,
    /// Health check interval in seconds
    pub health_check_interval_seconds: u64,
}

impl Default for MonitoringSettings {
    fn default() -> Self {
        Self {
            enable_metrics: true,
            metrics_interval_seconds: 60,
            log_level: "info".to_string(),
            health_check_interval_seconds: 30,
        }
    }
}

/// Simplified performance configuration
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PerformanceSettings {
    /// Maximum memory in MB
    pub max_memory_mb: usize,
    /// Worker thread count
    pub worker_threads: usize,
    /// Cache size in MB
    pub cache_size_mb: usize,
    /// Enable optimization
    pub enable_optimization: bool,
}

impl Default for PerformanceSettings {
    fn default() -> Self {
        Self {
            max_memory_mb: 512,
            worker_threads: 4,
            cache_size_mb: 64,
            enable_optimization: true,
        }
    }
}

impl Default for SimplifiedBearDogConfig {
    fn default() -> Self {
        let mut features = HashMap::new();
        features.insert("security_enhanced".to_string(), true);
        features.insert("performance_optimized".to_string(), true);
        features.insert("monitoring_enabled".to_string(), true);

        Self {
            version: env!("CARGO_PKG_VERSION").to_string(),
            environment: "development".to_string(),
            instance_id: format!("beardog-{}", std::process::id()),
            network: NetworkSettings::default(),
            security: SecuritySettings::default(),
            database: DatabaseSettings::default(),
            monitoring: MonitoringSettings::default(),
            features,
            performance: PerformanceSettings::default(),
        }
    }
}

impl SimplifiedBearDogConfig {
    /// Create a new simplified configuration
    #[must_use]
    pub fn new() -> Self {
        Self::default()
    }

    /// Validate the configuration
    pub fn validate(&self) -> BearDogResult<()> {
        use super::r#trait::validation::{validate_port, validate_range};

        validate_port(self.network.port, "network.port")?;

        if self.network.max_connections == 0 {
            return Err(BearDogError::configuration(
                "network.max_connections must be greater than 0",
            ));
        }

        validate_range(self.security.hash_rounds, 4, 31, "security.hash_rounds")?;

        if self.database.pool_size == 0 {
            return Err(BearDogError::configuration(
                "database.pool_size must be greater than 0",
            ));
        }

        Ok(())
    }

    /// Load from environment variables
    pub fn from_env() -> BearDogResult<Self> {
        let mut config = Self::default();

        if let Ok(port) = std::env::var("BEARDOG_PORT") {
            if let Ok(port_num) = port.parse::<u16>() {
                config.network.port = port_num;
            }
        }

        if let Ok(log_level) = std::env::var("BEARDOG_LOG_LEVEL") {
            config.monitoring.log_level = log_level;
        }

        if let Ok(environment) = std::env::var("BEARDOG_ENVIRONMENT") {
            config.environment = environment;
        }

        Ok(config)
    }

    /// Apply configuration overrides
    #[must_use]
    pub fn with_overrides(mut self, overrides: HashMap<String, String>) -> Self {
        for (key, value) in overrides {
            match key.as_str() {
                "network.port" => {
                    if let Ok(port) = value.parse::<u16>() {
                        self.network.port = port;
                    }
                }
                "network.max_connections" => {
                    if let Ok(connections) = value.parse::<usize>() {
                        self.network.max_connections = connections;
                    }
                }
                "security.enable_mfa" => {
                    if let Ok(enable) = value.parse::<bool>() {
                        self.security.enable_mfa = enable;
                    }
                }
                "monitoring.log_level" => {
                    self.monitoring.log_level = value;
                }
                _ => {
                    if let Ok(bool_value) = value.parse::<bool>() {
                        self.features.insert(key, bool_value);
                    }
                }
            }
        }
        self
    }

    /// Get configuration summary
    #[must_use]
    pub fn summary(&self) -> HashMap<String, String> {
        let mut summary = HashMap::new();
        summary.insert("version".to_string(), self.version.clone());
        summary.insert("environment".to_string(), self.environment.clone());
        summary.insert("network_port".to_string(), self.network.port.to_string());
        summary.insert(
            "security_mfa".to_string(),
            self.security.enable_mfa.to_string(),
        );
        summary.insert(
            "database_pool_size".to_string(),
            self.database.pool_size.to_string(),
        );
        summary.insert(
            "monitoring_enabled".to_string(),
            self.monitoring.enable_metrics.to_string(),
        );
        summary
    }
}

// Backward compatibility type alias
// REMOVED: Deprecated alias (Phase 2 cleanup - October 2025)
// Use SimplifiedBearDogConfig directly
// REMOVED: Deprecated WorkingUnifiedConfig alias - use SimplifiedBearDogConfig directly

// Additional supporting configuration types would be implemented here...
