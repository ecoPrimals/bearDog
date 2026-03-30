// SPDX-License-Identifier: AGPL-3.0-only

//! Core Domain Configuration Modules
//!
//! This module contains the essential configuration domains for `BearDog`:
//! - Application configuration
//! - Network configuration
//! - Security configuration
//! - Database configuration
//! - Genetics configuration

use serde::{Deserialize, Serialize};
use std::path::PathBuf;
use std::time::Duration;

use crate::canonical::config::network::{CircuitBreakerConfig, LoadBalancingConfig};
use crate::canonical::config::security::{
    CanonicalAuditConfig, CanonicalAuthenticationConfig, CanonicalAuthorizationConfig,
    CanonicalEncryptionConfig, CanonicalMfaConfig, CanonicalSessionConfig,
};
use crate::canonical::config::type_aliases::{
    DatabaseBackupConfig, EntropyCollectionConfig, EntropyHierarchyConfig, GeneticAlgorithmConfig,
    HumanEntropyConfig, SimdOptimizationConfig,
};

use super::metadata::LogLevel;

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

/// Password source configuration options
#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum PasswordSource {
    /// Password from environment variable
    Environment(String),
    /// Password from file path
    File(PathBuf),
    /// Password from vault service
    Vault(String),
    /// Inline secret material (testing only; never commit or log this variant).
    Inline(String), // Not recommended for production
}

impl Default for PasswordSource {
    fn default() -> Self {
        Self::Environment("DATABASE_PASSWORD".to_string())
    }
}

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
